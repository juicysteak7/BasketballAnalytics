use yew::prelude::*;
use BasketballAnalyticsWeb::{ App, get_all_players, Player };
use wasm_bindgen_futures::spawn_local;

// Function component to do any pre-render work to pass to App
#[function_component(StartApp)]
fn start_app() -> Html {
    let init = use_state(|| Players { players: vec![] });

    {
        let init = init.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                match get_all_players().await {
                    Ok(data) => {
                        // let players:Players = Players { players: data };
                        // println!("Got all players: {:?}", players);
                        // log::info!("got all players: {:?}", players);
                        init.set(Players {players: data});
                                                                                                                    
                    }
                    Err(e) => {
                        eprintln!("{}",e);
                    }
                                                                                                                    
                };
            });
            || ()
        }, ());
    }


html! {
    <div>
        <App players={(*init).players.clone()}/>
    </div>
    }
}
#[derive(Debug)]
struct Players {
    players: Vec<Player>,
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    //yew::Renderer::<StartApp>::new().render();
    yew::start_app::<StartApp>();
}
