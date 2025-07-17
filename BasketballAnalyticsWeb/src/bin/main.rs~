use yew::prelude::*;
use BasketballAnalyticsWeb::App;
// use wasm_bindgen_futures::spawn_local;
//
//
// Function component to do any pre-render work to pass to App
#[function_component(StartApp)]
fn start_app() -> Html {
html! {
    <div>
        <App/>
    </div>
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<StartApp>::new().render();
}
