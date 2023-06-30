#![allow(unused)]

use rocket::{Build, Rocket};
use rocket::State;
use std::collections::HashMap;
use std::sync::{Mutex};
use rocket::form::FromForm;
use rocket::fs::{FileServer, relative};
use rocket::serde::{Deserialize, Serialize};
use crate::hub_comm::web::web_hub::{internal_api, player_api};

pub type PlayerId = u8;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PlayerEvent {
    pub id: PlayerId,
    pub timestamp: u32,
    pub state: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PlayerIdentityDto {
    pub id: PlayerId,
    pub name: String,
    pub ip: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ServerState {
    pub base_timestamp: u32,
    pub players: HashMap<PlayerId, PlayerIdentityDto>,
    pub events: Vec<PlayerEvent>,
}

impl ServerState {
    pub fn has_player_name(&self, name: &String) -> bool {
        let players_with_such_name = self.players.values()
            .filter(|&p| {
                p.name.eq(name)
            })
            .count();
        players_with_such_name > 0
    }
    pub fn has_ip(&self, _ip: &String) -> bool {
        false
    }
    // pub fn add_player(&mut self, player: &Player) -> PlayerId {
    //     let id = self.players.len() as PlayerId;
    //     let p = Player {
    //         id,
    //         name: player.name.clone(),
    //         ip: player.ip.clone(),
    //     };
    //     self.players.insert(id, p);
    //     id
    // }
    pub fn add_player(&mut self, name: &String) -> PlayerId {
        let id = (self.players.len() + 1) as PlayerId;
        let p = PlayerIdentityDto {
            id,
            name: name.clone(),
            ip: "0.0.0.0".to_string(),
        };
        self.players.insert(id, p);
        id
    }
    pub fn push_event(&mut self, event: PlayerEvent) {
        self.events.push(event);
    }
}

pub type SharedServerState = Mutex<ServerState>;
pub type Persistence<'a> = &'a State<Mutex<ServerState>>;


pub fn setup() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Player-API", |rocket| async {
        rocket
            .manage(SharedServerState::default())
            .mount("/", FileServer::from(relative!("static")))
    })
}

#[rocket::launch]
pub fn launch() -> Rocket<Build> {
    rocket::build()
        .attach(setup())
        .attach(player_api::setup())
        .attach(internal_api::setup())
}
