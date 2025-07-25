use yew::prelude::*;
use crate::{Player, AddPlayerModal};
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;

pub enum Msg {
    OpenModal,
    CloseModal,
    OnSubmit(Player)
}
#[derive(Properties, PartialEq, Debug, Default)]
pub struct AppProps {
    pub players: Vec<Player>,
}
pub struct App {
    players: Vec<Player>,
    modal_open: bool,
    len:usize,
}
impl Component for App {
    type Message = Msg;
    type Properties = AppProps;
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        log::info!("props players: {:?}",props.players.clone());
        Self { modal_open: false, len:0, players: props.players.clone() }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();
        self.players = props.players.clone();
        self.len = self.players.len();
        log::info!("props players: {:?}",props.players.clone());
        true
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let sample_player = Player {
            name: "Lebron".to_string(),
            points: 25,
            assists: 8,
            rebounds: 7,
            player_id: "1".to_string(),
        };
        //self.players.push(sample_player);
        let link = ctx.link();
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
                            <Player player={player.clone()} key={player.player_id.clone()}/>
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
    }
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
