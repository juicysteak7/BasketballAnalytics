use BasketballAnalyticsAPI::{ Player, DataBase, Players };
use serde::Serialize;
use tokio;
use tokio::net::TcpListener;
use axum::{ extract::Json, routing::put, routing::get, Router };
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins (*)
        .allow_methods(Any) // Allow GET, PUT, DELETE, POST, etc.
        .allow_headers(Any); // Allow all headers
                
    let app = Router::new()
        .route("/api/add_player", put(add_player))
        .route("/api/get_all_players", get(get_all_players))
        .route("/api/update_player", put(update_player))
        .route("/api/delete_player", put(delete_player))
        .layer(cors);
    let addr:SocketAddr = "127.0.0.1:6969".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();
    
    axum::serve(listener,app).await.unwrap();
    
    println!("Server running on {}", addr);
    Ok(())
}

async fn add_player(Json(payload):Json<Player>) -> impl axum::response::IntoResponse {
    use axum::Json;
    use serde::Serialize;

    let db = DataBase::sign_in("root", "root").await.unwrap();
    let result = db.add_player(payload).await.unwrap();

    #[derive(Serialize)]
    struct ResponseMessage {
        players: Players
    }

    let mut players = Players::new();

    match result { 
        Some(result) => {
            players.add(result);
            return Json(ResponseMessage {
                players
            })
        } 
        None => {
            return Json(ResponseMessage {
                players
            })
        }
    }
}

async fn update_player(Json(payload):Json<(Player, String)>) -> impl axum::response::IntoResponse {
    use axum::Json;
    use serde::Serialize;

    let db = DataBase::sign_in("root", "root").await.unwrap();
    let result = db.update_player(payload.0, payload.1).await.unwrap();

    #[derive(Serialize)]
    struct ResponseMessage {
        players: Players
    }

    let mut players = Players::new();

    if let Some(player) = result {
        players.add(player);
    }

    return Json(ResponseMessage { players })
}

async fn delete_player(Json(payload):Json<Player>) -> impl axum::response::IntoResponse {
    use axum::Json;
    use serde::Serialize;

    let db = DataBase::sign_in("root", "root").await.unwrap();
    let result = db.delete_player(payload).await.unwrap();

    #[derive(Serialize)]
    struct ResponseMessage {
        players: Players
    }

    let mut players = Players::new();

    if let Some(player) = result {
        players.add(player);
    }

    return Json(ResponseMessage { players })
}

async fn get_all_players() -> impl axum::response::IntoResponse {
    use axum::Json;
    use serde::Serialize;

    let db = DataBase::sign_in("root", "root").await.unwrap();
    let players = db.get_all_players().await;

    #[derive(Serialize)]
    struct ResponseMessage {
        players: Players,
    }
    match players {
        Ok(players) => {
            println!("Players: {:?}", players);
            Json(ResponseMessage { players: players, })
        }
        Err(e) => {
            eprintln!("{}",e);
            Json(ResponseMessage { players: Players::new(),})
        }
    }
}
