#![allow(unused)]

use rgb::{RGB, RGB8};
use rocket::{routes, get};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use crate::hub_comm::web::web_hub::server::{Persistence, PlayerIdentityDto, PlayerId};
use rocket::serde::{Deserialize, Serialize};
use crate::api::dto::PlayerSetupDto;
use crate::hub_comm::web::web_hub::internal_api::INTERNAL_API::TAKE_EVENT_QUEUE;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Rgb8Dto {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<RGB8> for Rgb8Dto {
    fn from(value: RGB8) -> Self {
        Rgb8Dto {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

impl Rgb8Dto {
    pub fn into_rgb8(&self) -> RGB8 {
        RGB8 {
            r: self.r,
            g: self.g,
            b: self.g,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TimestampDto {
    pub timestamp: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TermLightColorDto {
    pub id: PlayerId,
    pub color: Rgb8Dto,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TermFeedbackColor {
    pub id: PlayerId,
    pub state: bool,
}

/// INTERNAL API ENDPOINTS
#[allow(non_snake_case)]
pub mod INTERNAL_API {
    pub const GET_PLAYERS: &str = "/players";
    pub const GET_TIMESTAMP: &str = "/timestamp";
    pub const SET_TIMESTAMP: &str = "/timestamp";
    pub const SET_TERM_COLOR: &str = "/term-color";
    pub const SET_FEEDBACK_COLOR: &str = "/feedback-color";
    pub const GET_EVENT_QUEUE: &str = "/get-event-queue";
    pub const TAKE_EVENT_QUEUE: &str = "/take-event-queue";
}

#[get("/players", format = "application/json")]
fn get_players(state: Persistence) -> Json<Vec<PlayerIdentityDto>> {
    let guard = state.lock().expect("Poisoned");
    let players = guard.players.clone();
    let players_dto: Vec<PlayerIdentityDto> = players.values().cloned().collect();
    log::info!("Players are: {:?}", players_dto);
    Json::from(players_dto)
}

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
                get_players,
                get_events
            ])
    })
}