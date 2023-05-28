use serde::Serialize;
use crate::core::game_entities::Player;

#[derive(Serialize)]
pub struct ConfigDto {
    players: Vec<Player>,
    serial_port: String,
    radio_channel: i32,
}

impl ConfigDto {
    pub fn new(players: Vec<Player>, serial_port: String, radio_channel: i32) -> Self {
        Self { players, serial_port, radio_channel }
    }
}