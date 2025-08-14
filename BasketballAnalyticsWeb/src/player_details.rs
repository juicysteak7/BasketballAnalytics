use yew::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use crate::{ Player, AddDetailsModal, Plotters, add_player_season, delete_player_season };
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;

pub enum Msg {
    Back,
    Submit(PlayerSeason),
    CloseModal,
    OpenModal,
    Delete(PlayerSeason),
}

pub struct PlayerDetails {
    player: Player,
    seasons: Vec<PlayerSeason>,
    len: usize,
    modal_open: bool
}

#[derive(Properties, PartialEq, Debug)]
pub struct PlayerDetailsProps {
    pub player: Player,
    pub back: Callback<()>,
    pub seasons: Vec<PlayerSeason>,
}

impl Component for PlayerDetails {
    type Properties = PlayerDetailsProps;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        Self { player: ctx.props().player.clone(), 
            seasons: ctx.props().seasons.clone(), 
            len: ctx.props().seasons.len(), 
            modal_open: false }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.seasons = ctx.props().seasons.clone();
        self.len = ctx.props().seasons.len();
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Back => {
                ctx.props().back.emit(());
                true
            },
            Msg::Submit(season) => {
                log::info!("season to add: {:?}", season);
                self.seasons.push(season.clone());
                self.len = self.seasons.len();
                self.modal_open = false;
                spawn_local(async move {
                    match add_player_season(season).await {
                        Ok(result) => {
                            log::info!("Season added: {:?}", result);
                        }
                        Err(e) => {
                            eprintln!("{}",e);
                        }
                    }
                });
                true
            },
            Msg::CloseModal => {
                self.modal_open = false;
                true
            },
            Msg::OpenModal => {
                self.modal_open = true;
                true
            },
            Msg::Delete(to_delete) => {
                self.seasons.retain(|season| season.season_id != to_delete.season_id);
                spawn_local(async move {
                    match delete_player_season(to_delete).await {
                        Ok(result) => {
                            log::info!("Season deleted: {:?}", result);
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
        let link = ctx.link();
        html!{
            <div>
                <button class="btn-primary" onclick={link.callback(|_| Msg::Back)}>{"Back"}</button>
                <button class="btn-primary" onclick={link.callback(|_| Msg::OpenModal)}>{"Add Season"}</button>
                if self.modal_open {
                    <div class="modal">
                        <AddDetailsModal
                            on_submit={link.callback(|season| Msg::Submit(season))}
                            on_close={link.callback(|_| Msg::CloseModal)}
                            is_open={self.modal_open}
                            season_id={self.len}
                            player={self.player.clone()}
                        />
                    </div>
                }
                <table class="stats-table">
                    <thead>
                        <tr>
                        <th>{"Season"}</th>
                        <th>{"Team"}</th>
                        <th>{"Points"}</th>
                        <th>{"Assists"}</th>
                        <th>{"Rebounds"}</th>
                        <th>{"Remove"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                        <td>{"All"}</td>
                        <td>{"All"}</td>
                        <td>{self.player.points}</td>
                        <td>{self.player.assists}</td>
                        <td>{self.player.rebounds}</td>
                        <td></td>
                        </tr>
                        {
                            for self.seasons.iter().map(|season| {
                                let this_season = PlayerSeason {
                                    player_id: season.player_id.clone(),
                                    season_id: season.season_id.clone(),
                                    season_number: season.season_number,
                                    team_name: season.team_name.clone(),
                                    points: season.points,
                                    assists: season.assists,
                                    rebounds: season.rebounds,
                                };

                                html!{
                                <tr>
                                    <td>{season.season_number}</td>
                                    <td>{season.team_name.clone()}</td>
                                    <td>{season.points}</td>
                                    <td>{season.assists}</td>
                                    <td>{season.rebounds}</td>
                                    <td>
                                        <button onclick={link.callback(move |_| Msg::Delete(this_season.clone()))}>{"X"}</button>
                                    </td>
                                </tr>
                                }
                            })
                        }
                    </tbody>
                </table>
                <Plotters 
                    data={self.seasons.clone()}
                />
            </div>
        }
    }
}

#[derive(Properties, PartialEq, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PlayerSeason {
    pub player_id: String,
    pub season_id: String,
    pub team_name: String,
    pub season_number: i32,
    pub points: f64,
    pub assists: f64,
    pub rebounds: f64,
}
