use yew::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use crate::{ Player, AddDetailsModal, Plotters, add_player_season };
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;

pub enum Msg {
    Back,
    Submit(PlayerSeason),
    CloseModal,
    OpenModal,
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
        Self { player: ctx.props().player.clone(), seasons: ctx.props().seasons.clone(), len: 0, modal_open: false }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.seasons = ctx.props().seasons.clone();
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Back => {
                ctx.props().back.emit(());
                true
            },
            Msg::Submit(season) => {
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
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html!{
            <div>
                <table style="border-collapse: collapse; width: 100%;">
                    <thead>
                        <tr>
                        <th style="border: 1px solid black; padding: 8px;">{"Season"}</th>
                        <th style="border: 1px solid black; padding: 8px;">{"Team"}</th>
                        <th style="border: 1px solid black; padding: 8px;">{"Points"}</th>
                        <th style="border: 1px solid black; padding: 8px;">{"Assists"}</th>
                        <th style="border: 1px solid black; padding: 8px;">{"Rebounds"}</th>
                        <th style="border: 1px solid black; padding: 8px;">{"Remove"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                        <td style="border: 1px solid black; padding: 8px;">{"All"}</td>
                        <td style="border: 1px solid black; padding: 8px;">{"All"}</td>
                        <td style="border: 1px solid black; padding: 8px;">{self.player.points}</td>
                        <td style="border: 1px solid black; padding: 8px;">{self.player.assists}</td>
                        <td style="border: 1px solid black; padding: 8px;">{self.player.rebounds}</td>
                        //<td style="border: 1px solid black; padding: 8px;"><button>{"X"}</button></td>
                        </tr>
                        {
                            for self.seasons.iter().map(|season| html!{
                                <tr>
                                    <td style="border: 1px solid black; padding: 8px;">{season.season_number}</td>
                                    <td style="border: 1px solid black; padding: 8px;">{season.team_name.clone()}</td>
                                    <td style="border: 1px solid black; padding: 8px;">{season.points}</td>
                                    <td style="border: 1px solid black; padding: 8px;">{season.assists}</td>
                                    <td style="border: 1px solid black; padding: 8px;">{season.rebounds}</td>
                                    <td style="border: 1px solid black; padding: 8px;"><button>{"X"}</button></td>
                                </tr>
                            })
                        }
                    </tbody>
                </table>
                <button onclick={link.callback(|_| Msg::Back)}>{"Back"}</button>
                <button onclick={link.callback(|_| Msg::OpenModal)}>{"Add Season"}</button>
                if self.modal_open {
                    <AddDetailsModal
                        on_submit={link.callback(|season| Msg::Submit(season))}
                        on_close={link.callback(|_| Msg::CloseModal)}
                        is_open={self.modal_open}
                        season_id={self.len}
                        player={self.player.clone()}
                    />
                }

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
