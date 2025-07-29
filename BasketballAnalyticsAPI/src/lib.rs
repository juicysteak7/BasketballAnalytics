use serde::Serialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Error;

pub struct DataBase{
        db: Surreal<Client>,
            _signed_in: bool,
}

impl DataBase {
        pub async fn sign_in(username: &str, password: &str) -> Result<DataBase, Error>{
            let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
            db.signin(Root {
                username,
                password
            }).await?;

            db.use_ns("test").use_db("test").await?;
            Ok(DataBase { db, _signed_in: true })
        }

        pub async fn add_player(&self, player:Player) -> Result<Option<Player>, Error>{
            let created: Option<Player> = self.db.create(("player", player.player_id.clone())).content(player).await?;
            println!("created: {:?}",created);
            Ok(created)
        }

        pub async fn get_all_players(&self) -> Result<Players, Error>{
            let players:Vec<Player> = self.db.select("player").await?;
            Ok(Players { players })
        }

        pub async fn update_player(&self, player:Player, old_id: String) -> Result<Option<Player>, Error>{
            let updated:Option<Player> = self.db.update(("player", old_id)).content(player).await?;
            Ok(updated)
        }

        pub async fn delete_player(&self, player:Player) -> Result<Option<Player>, Error>{
            let delete = self.db.delete(("player", player.player_id)).await?;
            Ok(delete)
        }
}

#[derive(Debug, Serialize, serde::Deserialize, Clone)]
pub struct Player {
    player_id:String,
    name:String,
    points:f64,
    assists:f64,
    rebounds:f64,
}

impl Player {
    pub fn new(player_id:String, name:String, points:f64, assists:f64, rebounds:f64) -> Player {
        Player {player_id, name, points, assists, rebounds}
    }
}

#[derive(Debug, Serialize, serde::Deserialize, Clone)]
pub struct Players {
    players: Vec<Player>,
}

impl Players {
    pub fn new() -> Players {
        let players:Vec<Player> = Vec::new();
        Players { players }
    }
    pub fn add(&mut self, player: Player) {
        self.players.push(player);
    }
}
