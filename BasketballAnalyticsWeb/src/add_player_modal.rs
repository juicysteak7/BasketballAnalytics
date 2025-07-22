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
                    <button onclick={link.callback(|_| Msg::Close)}>{"Close"}</button>
                    <button onclick={link.callback(|_| Msg::Submit)}>{"Submit"}</button>
                </div>
            </div>
        }
    }
}
