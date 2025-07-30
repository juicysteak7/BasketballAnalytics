use yew::prelude::*;
use crate::{Player, AddPlayerModal, PlayerDetails, PlayerSeason};
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;

pub enum Msg {
    OpenModal,
    CloseModal,
    OnSubmit(Player),
    Delete(Player),
    Select(Player),
    UnSelect,
    PlayerSeasons(Vec<PlayerSeason>),
}
#[derive(Properties, PartialEq, Debug, Default)]
pub struct AppProps {
    pub players: Vec<Player>,
}
pub struct App {
    players: Vec<Player>,
    modal_open: bool,
    len:usize,
    player_selected: bool,
    selected_player: Option<Player>,
    selected_player_seasons: Vec<PlayerSeason>,
}
impl Component for App {
    type Message = Msg;
    type Properties = AppProps;
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        log::info!("props players: {:?}",props.players.clone());
        Self { 
            modal_open: false, 
            len:0, 
            players: props.players.clone(), 
            player_selected: false, 
            selected_player:None,
            selected_player_seasons: Vec::new(),
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();
        self.players = props.players.clone();
        self.len = self.players.len();
        log::info!("props players: {:?}",props.players.clone());
        true
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OpenModal => {
                self.modal_open = true;
                true
            },
            Msg::CloseModal => {
                self.modal_open = false;
                true
            },
            Msg::OnSubmit(player) => {
                log::info!("player: {:?}",player);
                self.players.push(player.clone());
                self.len = self.players.len();
                self.modal_open = false;

                spawn_local(async move {
                    match add_player(player).await {
                        Ok(result) => {
                            log::info!("Application added: {:?}", result);
                        }
                        Err(e) => {
                            eprintln!("{}",e);
                        }
                    }
                });
                true
            },
            Msg::Delete(to_delete) => {
                self.players.retain(|player| player.player_id != to_delete.player_id);
                self.len = self.players.len();

                spawn_local(async move {
                    match delete_player(to_delete).await {
                        Ok(result) => {
                            log::info!("Application added: {:?}", result);
                        }
                        Err(e) => {
                            eprintln!("{}",e);
                        }
                    }
                });
                true
            },
            Msg::Select(player) => {
                self.player_selected = true;
                self.selected_player = Some(player.clone());
                log::info!("Selected player: {:?}", player);
                let link = ctx.link().clone();
                let selected_player = self.selected_player.clone().unwrap();
                spawn_local(async move {
                    match get_all_player_seasons(selected_player).await {
                        Ok(result) => {
                            link.send_message(Msg::PlayerSeasons(result));
                        },
                        Err(e) => {
                            log::info!("Error: {:?}",e);
                        }
                    }
                });
                true
            },
            Msg::UnSelect => {
                self.player_selected = false;
                self.selected_player = None;
                self.selected_player_seasons = Vec::new();
                true
            },
            Msg::PlayerSeasons(seasons) => {
                self.selected_player_seasons = seasons.clone();
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        if !self.player_selected {
            html! {
                <div>
                    <button onclick={link.callback(|_| Msg::OpenModal)}>{ "Add Player" }</button>
                    <table style="border-collapse: collapse; width: 100%;">
                        <thead>
                        <tr>
                        <th style="border: 1px solid black; padding: 8px;">{"Name"}</th>
                        <th style="border: 1px solid black; padding: 8px;">{"Points"}</th>
                        <th style="border: 1px solid black; padding: 8px;">{"Assists"}</th>
                        <th style="border: 1px solid black; padding: 8px;">{"Rebounds"}</th>
                        <th style="border: 1px solid black; padding: 8px;">{"Remove"}</th>
                        </tr>
                        </thead>
                        <tbody>
                        {
                            for self.players.iter().map(|player|
                            html!{
                                <Player 
                                    key={player.player_id.clone()}
                                    player={player.clone()} 
                                    delete={link.callback(|player| Msg::Delete(player))}
                                    select={link.callback(|player| Msg::Select(player))}
                                />
                            })
                        }
                        </tbody>
                    </table>

                    <AddPlayerModal
                        is_open={self.modal_open}
                        on_close={link.callback(|_| Msg::CloseModal)}
                        on_submit={link.callback(|player| Msg::OnSubmit(player))}
                        player_id={self.len}
                    />
                </div>
            }

        } else {
            html!{
                <PlayerDetails
                    player={self.selected_player.clone().unwrap()}
                    back={link.callback(|_| Msg::UnSelect)}
                    seasons={self.selected_player_seasons.clone()}
                />
            }
        }
    }
}


pub async fn get_all_player_seasons(player: Player) -> Result<Vec<PlayerSeason>, reqwest::Error> {
    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
    struct Response {
        seasons: Vec<PlayerSeason>,
    }

    let client = Client::new();
    let response = client.put("http://127.0.0.1:6969/api/get_all_player_seasons")
        .json(&player.player_id)
        .send()
        .await?;
    let data = response.json::<Response>().await?;
    Ok(data.seasons)
}

pub async fn add_player_season(season:PlayerSeason) -> Result<Vec<PlayerSeason>, reqwest::Error> {
    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
    struct Response {
        seasons: Vec<PlayerSeason>
    }

    let client = Client::new();
    let response = client.put("http://127.0.0.1:6969/api/add_player_season").json(&season).send().await?;
    let data = response.json::<Response>().await?;
    Ok(data.seasons)
}

pub async fn get_all_players() -> Result<Vec<Player>, reqwest::Error> {
    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
    struct PlayerResponse {
        players: Players
    }
    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
    struct Players {
        players: Vec<Player>
    }

    let client = Client::new();
    let response = client.get("http://127.0.0.1:6969/api/get_all_players").send().await?;
    let data = response.json::<PlayerResponse>().await?;
    Ok(data.players.players)
}

async fn add_player(player:Player) -> Result<Vec<Player>, reqwest::Error> {
    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
    struct PlayerResponse {
        players: Vec<Player>
    }

    let client = Client::new();
    let response = client.put("http://127.0.0.1:6969/api/add_player").json(&player).send().await?;
    let data = response.json::<PlayerResponse>().await?;
    Ok(data.players)
}

async fn delete_player(player:Player) -> Result<Vec<Player>, reqwest::Error> {
    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
    struct PlayerResponse {
        players: Vec<Player>
    }

    let client = Client::new();
    let response = client.put("http://127.0.0.1:6969/api/delete_player").json(&player).send().await?;
    let data = response.json::<PlayerResponse>().await?;
    Ok(data.players)
}
