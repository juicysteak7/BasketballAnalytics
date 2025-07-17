use yew::prelude::*;
// use reqwest::Client;
// use wasm_bindgen_futures::spawn_local;
// use wasm_bindgen::JsCast;

pub struct App {

}
impl Component for App {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
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
                 <tr>
                 <td style="border: 1px solid black; padding: 8px;">{"LeBron James"}</td>
                 <td style="border: 1px solid black; padding: 8px;">{"25"}</td>
                 <td style="border: 1px solid black; padding: 8px;">{"8"}</td>
                 <td style="border: 1px solid black; padding: 8px;">{"7"}</td>
                 </tr>
                 <tr>
                 <td style="border: 1px solid black; padding: 8px;">{"Steph Curry"}</td>
                 <td style="border: 1px solid black; padding: 8px;">{"30"}</td>
                 <td style="border: 1px solid black; padding: 8px;">{"6"}</td>
                 <td style="border: 1px solid black; padding: 8px;">{"5"}</td>
                 </tr>
                 </tbody>
                 </table>
        }
    }
}
