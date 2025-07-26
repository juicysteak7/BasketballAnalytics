use yew::prelude::*;
use serde_json;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct PlayerProps {
    pub player: Player
}

#[derive(Properties, PartialEq, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub player_id: String,
    pub name: String,
    pub points: u64,
    pub assists: u64,
    pub rebounds: u64,
}

impl Component for Player {
    type Message = ();
    type Properties = PlayerProps;

    fn create(ctx: &Context<Self>) -> Self{
        let player = ctx.props().player.clone();
        Self { player_id: player.player_id, name: player.name, points: player.points, assists: player.assists, rebounds: player.rebounds }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let player = self.clone();
        let _link = ctx.link();
        html!{
            <tr>
                <td style="border: 1px solid black; padding: 8px;">{player.name}</td>
                <td style="border: 1px solid black; padding: 8px;">{player.points}</td>
                <td style="border: 1px solid black; padding: 8px;">{player.assists}</td>
                <td style="border: 1px solid black; padding: 8px;">{player.rebounds}</td>
                <td style="border: 1px solid black; padding: 8px;">
                    <button aria-label="Close">{"X"}</button>
                </td>
            </tr>
        }
    }
}
