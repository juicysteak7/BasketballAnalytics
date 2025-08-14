use yew::prelude::*;
use serde_json;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;

pub enum Msg {
    Delete,
    Select,
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct PlayerProps {
    pub player: Player,
    pub delete: Callback<Player>,
    pub select: Callback<Player>,
}

#[derive(Properties, PartialEq, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub player_id: String,
    pub name: String,
    pub points: f64,
    pub assists: f64,
    pub rebounds: f64,
}

impl Component for Player {
    type Message = Msg;
    type Properties = PlayerProps;

    fn create(ctx: &Context<Self>) -> Self{
        let player = ctx.props().player.clone();
        Self { player_id: player.player_id, name: player.name, points: player.points, assists: player.assists, rebounds: player.rebounds }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Delete => {
                ctx.props().delete.emit(Player {
                    player_id: self.player_id.clone(), 
                    name: self.name.clone(), 
                    points: self.points,
                    assists: self.assists,
                    rebounds: self.rebounds,
                });
                true
            },
            Msg::Select => {
                ctx.props().select.emit(Player {
                    player_id: self.player_id.clone(),
                    name: self.name.clone(),
                    points: self.points,
                    assists: self.assists,
                    rebounds: self.rebounds,
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let player = self.clone();
        let link = ctx.link();
        html!{
            <tr>
                <td onclick={link.callback(|_| Msg::Select)}>
                    {player.name}
                </td>
                <td>{player.points}</td>
                <td>{player.assists}</td>
                <td>{player.rebounds}</td>
                <td>
                    <button aria-label="Close" onclick={link.callback(|_| Msg::Delete)}>{"X"}</button>
                </td>
            </tr>
        }
    }
}
