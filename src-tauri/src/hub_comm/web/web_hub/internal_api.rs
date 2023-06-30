#![allow(unused)]

use rocket::{Build, Rocket, routes, get, post};
use rocket::serde::json::{Json, Value};
use rocket::State;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rgb::{RGB, RGB8};
use rocket::form::FromForm;
use rocket::fs::{FileServer, relative};
use rocket::futures::SinkExt;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::serde_json::json;
use crate::hub_comm::web::web_hub::server::Persistence;

#[get("/events", format = "application/json")]
fn get_events(state: Persistence) -> Value {
    let mut guard = state.lock().expect("Poisoned");
    let events = guard.events.clone();
    log::info!("Events are: {:?}", events);
    json!({"events": events})
}

pub fn setup() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Player-API", |rocket| async {
        rocket
            .mount("/", routes![
                get_events
            ])
    })
}