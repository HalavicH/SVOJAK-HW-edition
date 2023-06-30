#![allow(unused)]

use rocket::{routes, post};
use rocket::serde::json::{Json, Value};
use rocket::fairing::AdHoc;
use rocket::serde::json::serde_json::json;
use crate::hub_comm::hw::hw_hub_manager::get_epoch_ms;
use crate::hub_comm::web::web_hub::server::{Persistence, PlayerEvent, PlayerIdentityDto};

#[post("/register", data = "<name>")]
fn register_player(name: Json<PlayerIdentityDto>, state: Persistence) -> Value {
    log::info!("Got new? player: {:?}", name);

    let mut guard = state.lock().expect("Poisoned");
    // if guard.has_ip(&player.ip) {
    //     log::info!("Ip collision. Skip for now :D");
    // }

    let id = guard.add_player(&name.name);

    json!({
        "playerId": id,
        "baseTimestamp": guard.base_timestamp,
    })
}

#[post("/event", format = "application/json", data = "<event>")]
fn process_event(event: Json<PlayerEvent>, state: Persistence) -> Value {
    log::info!("Received event {:?}", event);

    let mut guard = state.lock().expect("Poisoned");
    // TODO: Move to the gameplay
    let color = if event.state == true {
        "#00FFFF"
    } else {
        "#000000"
    };

    let e = PlayerEvent {
        id: event.id,
        timestamp: get_epoch_ms().expect("Can't get epoch"),
        state: event.state,
    };

    // TODO: Add check for player existense
    guard.push_event(e);

    json!({"color": color})
}

pub fn setup() -> AdHoc {
    AdHoc::on_ignite("Player-API", |rocket| async {
        rocket
            .mount("/", routes![
                register_player,
                process_event
            ])
    })
}