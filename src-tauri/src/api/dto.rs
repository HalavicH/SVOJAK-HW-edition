#[allow(unused_imports, unused_variables)]

use serde::Serialize;
use crate::core::game_entities::Player;

#[derive(Debug, Serialize)]
pub struct ConfigDto {
    pub hub_port: String,
    pub available_ports: Vec<String>,
    pub radio_channel: i32,
    pub players: Vec<PlayerSetupDto>,
}

#[derive(Debug, Serialize)]
pub struct PlayerSetupDto {
    pub term_id: u8,
    pub icon: String,
    pub name: String,
    pub is_used: bool,
}

#[derive(Debug, Serialize)]
pub struct PlayerScoreDto {
    pub name: String,
    pub score: i32,
}

