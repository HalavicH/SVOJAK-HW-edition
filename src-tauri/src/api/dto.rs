use serde::Serialize;
use crate::core::game_entities::Player;

#[derive(Debug, Serialize)]
pub struct ConfigDto {
    pub hub_port: String,
    pub available_ports: Vec<String>,
    pub radio_channel: i32,
    pub players: Vec<Player>,
}

impl ConfigDto {
    pub fn new(hub_port: String, available_ports: Vec<String>, radio_channel: i32, players: Vec<Player>) -> Self {
        Self { hub_port, available_ports, radio_channel, players }
    }
}

