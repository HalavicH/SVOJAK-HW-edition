#![allow(unused)]

use rocket::{routes, get};
use rocket::serde::json::{Value};
use rocket::serde::json::serde_json::json;
use crate::hub_comm::web::web_hub::server::Persistence;

#[get("/events", format = "application/json")]
fn get_events(state: Persistence) -> Value {
    let guard = state.lock().expect("Poisoned");
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