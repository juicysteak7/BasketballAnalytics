use yew::prelude::*;
use crate::{PlayerSeason};

#[derive(Properties, PartialEq)]
pub struct AddDetailsModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<PlayerSeason>,
    pub is_open: bool,
    pub season_id: usize,
}

pub enum Msg {
    Submit,
    Close,
    Open,
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
                    <input
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
                    <input
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
                    <input
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
                    <input
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
                    <button onclick={link.callback(|_| Msg::Close)}>{"Close"}</button>
                    <button onclick={link.callback(|_| Msg::Submit)}>{"Submit"}</button>
                </div>
            </div>
        }
    }
}
