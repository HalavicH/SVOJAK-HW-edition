#[allow(unused_imports, unused_variables)]

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct ConfigDto {
    pub hub_port: String,
    pub available_ports: Vec<String>,
    pub radio_channel: i32,
    pub players: Vec<PlayerSetupDto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PlayerSetupDto {
    pub termId: u8,
    pub icon: String,
    pub name: String,
    pub isUsed: bool,
}

#[derive(Debug, Serialize)]
pub struct PlayerScoreDto {
    pub name: String,
    pub score: i32,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct PackInfoDto {
    pub packName: String,
    pub packAuthor: String,
    pub packRounds: i32,
    pub packTopics: i32,
    pub packQuestion: i32,
    pub packTopicList: Vec<String>,
}
