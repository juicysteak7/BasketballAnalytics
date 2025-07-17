use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct PlayerData {
    pub name: String,
    pub points: u64,
    pub assists: u64,
    pub rebounds: u64,
}

#[derive(Properties, PartialEq)]
pub struct PlayerProps {
    pub player: PlayerData
}

pub struct Player {
    player: PlayerData
}

impl Component for Player {
    type Message = ();
    type Properties = PlayerProps;

    fn create(ctx: &Context<Self>) -> Self{
        Self { player: ctx.props().player.clone() }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let player = self.player.clone();
        let link = ctx.link();
        html!{
            <tr>
                <td style="border: 1px solid black; padding: 8px;">{player.name}</td>
                <td style="border: 1px solid black; padding: 8px;">{player.points}</td>
                <td style="border: 1px solid black; padding: 8px;">{player.assists}</td>
                <td style="border: 1px solid black; padding: 8px;">{player.rebounds}</td>
            </tr>
        }
    }
}
