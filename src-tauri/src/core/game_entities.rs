use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use serde::{Serialize, Deserialize};
use crate::core::hub_manager::HubManager;
use crate::game_pack::pack_content_entities::{PackContent};

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlayerState {
    #[default]
    Idle,
    Target,
    FirstResponse,
    Inactive,
    Dead,
    AnsweredCorrectly,
    AnsweredWrong
}

#[derive(Default, Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerStats {
    pub score: i32,
    pub correct_num: i32,
    pub wrong_num: i32,
    pub total_tries: i32,
}

#[derive(Debug, Default, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub icon: String,
    // todo: make actual image
    pub term_id: u8,
    pub is_used: bool,
    pub state: PlayerState,
    pub stats: PlayerStats,
}

impl Player {
    pub fn new(term_id: u8) -> Self {
        Self {
            term_id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize)]
pub enum HubStatus {
    Detected,
    NoDevice,
}

impl Default for HubStatus {
    fn default() -> Self {
        HubStatus::NoDevice
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum GamePackError {
    ThemeNotPresent,
    QuestionNotPresent,
}

impl fmt::Display for GamePackError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to retrieve pack item:")
    }
}

impl Error for GamePackError {}

#[derive(Default, Debug)]
pub struct GameContext {
    pub players: HashMap<u8, Player>,
    pub pack: PackContent,
    pub hub: HubManager,
    pub current: CurrentContext,
}

#[derive(Default, Debug)]
pub struct CurrentContext {
    pub round_index: i32,
    pub player_id: u8,
    pub answer_allowed: bool,
    pub question_theme: String,
    pub question_price: i32,
}

#[derive(Debug, Clone, Serialize)]
pub enum GameplayError {
    PackElementNotPresent,
    PlayerNotPresent,
    HubOperationError,
}

impl fmt::Display for GameplayError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed perform game operation:")
    }
}

impl std::error::Error for GameplayError {}

lazy_static::lazy_static! {
    static ref CONTEXT: Arc<Mutex<GameContext>> = Arc::new(Mutex::new(GameContext::default()));
}

pub fn game_ctx() -> std::sync::MutexGuard<'static, GameContext> {
    CONTEXT.lock().unwrap()
}

#[cfg(test)]
mod game_entities_test {
    use crate::core::game_entities::GameContext;

    #[test]
    fn test_fastest_click() {
        let i = GameContext::default().get_fastest_click();
        log::info!("Fastest click from: {i}");
    }
}