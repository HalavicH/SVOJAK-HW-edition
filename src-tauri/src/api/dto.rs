use serde::{Serialize, Deserialize};
use crate::game_pack::pack_content_entities::QuestionMediaType;

////////// Config ///////////
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct ConfigDto {
    pub hub_port: String,
    pub available_ports: Vec<String>,
    pub radio_channel: i32,
    pub players: Vec<PlayerSetupDto>,
}

////////// Players ///////////
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PlayerSetupDto {
    pub termId: u8,
    pub icon: String,
    pub name: String,
    pub isUsed: bool,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct PlayerGameDto {
    pub id: i32,
    pub playerIconPath: String,
    pub playerName: String,
    pub score: i32,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct PlayerScoreDto {
    pub id: i32,
    pub score: i32,
}

////////// Pack info ///////////
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct PackInfoDto {
    pub packName: String,
    pub packAuthor: String,
    pub packRounds: i32,
    pub packTopics: i32,
    pub packQuestions: i32,
    pub packTopicList: Vec<String>,
}

////////// Round ///////////
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct RoundDto {
    pub roundName: String,
    pub roundType: String,
    pub roundTopics: Vec<TopicDto>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct TopicDto {
    pub topicName: String,
    pub questions: Vec<QuestionDto>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct QuestionDto {
    pub index: usize,
    pub price: i32,
}

////////// Round ///////////
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub enum QuestionType {
    Normal,
    PigInPoke,
    Auction,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct QuestionDataDto {
    pub number: i32,
    pub category: String,
    pub price: i32,
    pub questionType: QuestionType,
    pub mediaType: QuestionMediaType,
    pub content: String,
}