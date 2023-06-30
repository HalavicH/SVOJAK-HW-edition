#![allow(unused)]

use rocket::{Build, Rocket, routes, get, post};
use rocket::serde::json::{Json, Value};
use rocket::State;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rgb::{RGB, RGB8};
use rocket::fairing::AdHoc;
use rocket::form::FromForm;
use rocket::fs::{FileServer, relative};
use rocket::futures::SinkExt;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::serde_json::json;
use crate::hub_comm::hw::internal::api_types::TermEvent;
use crate::hub_comm::web::web_hub::server::{Persistence, PlayerEvent};

#[post("/register", data = "<name>")]
fn register_player(name: String, state: Persistence) -> Value {
    log::info!("Got new? player: {:?}", name);

    let mut guard = state.lock().expect("Poisoned");
    // if guard.has_ip(&player.ip) {
    //     log::info!("Ip collision. Skip for now :D");
    // }

    let id = guard.add_player(&name);

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

    // TODO: Add check for player existanse
    guard.push_event(event.0);

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