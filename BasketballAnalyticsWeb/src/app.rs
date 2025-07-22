use yew::prelude::*;
use crate::{Player, AddPlayerModal};
// use reqwest::Client;
// use wasm_bindgen_futures::spawn_local;
// use wasm_bindgen::JsCast;

pub enum Msg {
    OpenModal,
    CloseModal,
    OnSubmit(Player)
}
pub struct App {
    players: Vec<Player>,
    modal_open: bool,
    len:usize,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self { modal_open: false, len:0, players: Vec::new() }
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
                log::info!("player:",player);
                self.players.push(player);
                self.modal_open = false;
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
            id: "1".to_string(),
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
                    </tr>
                    </thead>
                    <tbody>
                    {
                        for players.iter().map(|player| html!{
                            <Player player={player.clone()}/>
                        })
                    }
                    </tbody>
                </table>

                <AddPlayerModal
                    is_open={self.modal_open}
                    on_close={link.callback(|_| Msg::CloseModal)}
                    on_submit={link.callback(|_| Msg::CloseModal)}
                    player_id={self.len}
                />
            </div>
            
        }
    }
}
