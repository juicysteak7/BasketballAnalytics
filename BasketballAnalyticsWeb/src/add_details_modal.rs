use yew::prelude::*;
use crate::{PlayerSeason, Player};

#[derive(Properties, PartialEq)]
pub struct AddDetailsModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<PlayerSeason>,
    pub is_open: bool,
    pub season_id: usize,
    pub player: Player,
}

pub enum Msg {
    Submit,
    Close,
    Open,
    UpdateSeason(i32),
    UpdateTeamName(String),
    UpdatePoints(f64),
    UpdateAssists(f64),
    UpdateRebounds(f64),
}

pub struct AddDetailsModal {
    is_open: bool,
    season: PlayerSeason,
}

impl Component for AddDetailsModal {
    type Properties = AddDetailsModalProps;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        Self { is_open: ctx.props().is_open, season: PlayerSeason {
            player_id: ctx.props().player.player_id.clone(),
            season_number: 0,
            team_name: "".to_string(),
            season_id: ctx.props().season_id.to_string(),
            points: 0.0,
            assists: 0.0,
            rebounds: 0.0,
        }}
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                ctx.props().on_submit.emit(self.season.clone());
                ctx.props().on_close.emit(());
                self.is_open = false;
                self.season.team_name.clear();
                self.season.points = 0.0;
                self.season.assists = 0.0;
                self.season.rebounds = 0.0;
                self.season.season_number = 0;
                true
            },
            Msg::Close => {
                ctx.props().on_close.emit(());
                self.is_open = false;
                true
            },
            Msg::Open => {
                self.is_open = true;
                true
            },
            Msg::UpdateSeason(number) => {
                self.season.season_number = number;
                true
            }
            Msg::UpdateTeamName(team_name) => {
                self.season.team_name = team_name;
                true
            },
            Msg::UpdatePoints(points) => {
                self.season.points = points;
                true
            },
            Msg::UpdateAssists(assists) => {
                self.season.assists = assists;
                true
            },
            Msg::UpdateRebounds(rebounds) => {
                self.season.rebounds = rebounds;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html!{
            <div>
                <div>
                    <label for="Season">{"Season"}</label>
                    <input
                        name="Season"
                        id="Season"
                        placeholder=0
                        type="number"
                        label="Season Number"
                        pattern=r"[0-9]+([,\.][0-9]+)?"
                        value={self.season.season_number.to_string()}
                        oninput={link.callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            let parsed = value.parse::<i32>();
                            match parsed {
                                Ok(value) => {
                                    Msg::UpdateSeason(value)
                                },
                                Err(e) => {
                                    log::info!("Error: {:?}", e);
                                    Msg::UpdateSeason(0)
                                }
                            }
                            //Msg::UpdateSeason(value.parse::<i32>().expect("Failed to parse into i32."))
                        })}
                    />
                </div>
                <div>
                    <label for="TeamName">{"Team Name"}</label>
                    <input
                        name="TeamName"
                        id="TeamName"
                        placeholder="Team Name"
                        label="Team Name"
                        value={self.season.team_name.clone()}
                        oninput={link.callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            Msg::UpdateTeamName(value)
                        })}
                    />
                </div>
                <div>
                    <label for="Points">{"Points"}</label>
                    <input
                        name="Points"
                        id="Points"
                        placeholder=0.00
                        type="number"
                        label="Points"
                        step=".01"
                        pattern=r"[0-9]+([,\.][0-9]+)?"
                        value={self.season.points.to_string()}
                        oninput={link.callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            Msg::UpdatePoints(value.parse::<f64>().expect("Failed to parse into f64."))
                        })}
                    />
                </div>
                <div>
                    <label for="Assists">{"Assists"}</label>
                    <input
                        name="Assists"
                        id="Assists"
                        placeholder=0.00
                        type="number"
                        label="Assists"
                        step=".01"
                        pattern=r"[0-9]+([,\.][0-9]+)?"
                        value={self.season.assists.to_string()}
                        oninput={link.callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            Msg::UpdateAssists(value.parse::<f64>().expect("Failed to parse into f64."))
                        })}
                    />
                </div>
                <div>
                    <label for="Rebounds">{"Rebounds"}</label>
                    <input
                        name="Rebounds"
                        id="Rebounds"
                        placeholder=0.00
                        type="number"
                        label="Rebounds"
                        step=".01"
                        pattern=r"[0-9]+([,\.][0-9]+)?"
                        value={self.season.rebounds.to_string()}
                        oninput={link.callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            Msg::UpdateRebounds(value.parse::<f64>().expect("Failed to parse into f64."))
                        })}
                    />
                </div>
                <div>
                    <button class="btn-primary" onclick={link.callback(|_| Msg::Close)}>{"Close"}</button>
                    <button class="btn-primary" onclick={link.callback(|_| Msg::Submit)}>{"Submit"}</button>
                </div>
            </div>
        }
    }
}
