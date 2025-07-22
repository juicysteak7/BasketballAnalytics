use yew::prelude::*;
use crate::{ Player };

#[derive(Properties, PartialEq)]
pub struct AddPlayerModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<Player>,
    pub is_open: bool,
    pub player_id: usize,
}

pub enum Msg {
    Submit,
    Close,
    Open,
    UpdateName(String),
    UpdatePoints(u64),
    UpdateAssists(u64),
    UpdateRebounds(u64),
}

pub struct AddPlayerModal {
    is_open: bool,
    player: Player,
}

impl Component for AddPlayerModal {
    type Properties = AddPlayerModalProps;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self {is_open: props.is_open, player: Player {id: props.player_id.to_string(), name: "".to_string(), points: 0, assists: 0, rebounds: 0 }}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Open => {
                true
            },
            Msg::Close => {
                ctx.props().on_close.emit(());
                true
            },
            Msg::Submit => {
                ctx.props().on_submit.emit(self.player.clone());
                ctx.props().on_close.emit(());
                true
            },
            Msg::UpdateName(name) => {
                self.player.name = name;
                true
            },
            Msg::UpdatePoints(points) => {
                self.player.points = points;
                true
            },
            Msg::UpdateAssists(assists) => {
                self.player.assists = assists;
                true
            }
            Msg::UpdateRebounds(rebounds) => {
                self.player.rebounds = rebounds;
                true
            }
        }
    }

    fn changed (&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        let props = ctx.props();
        self.is_open = props.is_open;
        self.player.id = props.player_id.to_string();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.is_open {
            return html! {}
        }

        let link = ctx.link();

        html! {
            <div>
                <div>
                    <input
                        placeholder="Name"
                        label="Name"
                        value={self.player.name.clone()}
                        oninput={link.callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            Msg::UpdateName(value)
                        })}
                    />
                </div>
                <div>
                    <input
                        placeholder=0
                        type="number"
                        label="Points"
                        value={self.player.points.to_string()}
                        oninput={link.callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            Msg::UpdatePoints(value)
                        })}
                    />
                </div>
                <div>
                    <input
                        placeholder=0
                        type="number"
                        label="Assists"
                        value={self.player.assists.to_string()}
                        oninput={link.callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            Msg::UpdateAssists(value)
                        })}
                    />
                </div>
                <div>
                    <input
                        placeholder=0
                        type="number"
                        label="Rebounds"
                        value={self.player.rebounds.to_string()}
                        oninput={link.callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            Msg::UpdateRebounds(value)
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
