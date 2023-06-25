use std::sync::{Arc, MutexGuard};
use std::sync::Mutex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use serde::{Serialize, Deserialize};
use crate::api::dto::QuestionType;
use crate::core::hub_manager::HubManager;
use crate::game_pack::game_pack_entites::GamePack;

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlayerState {
    #[default]
    Idle,
    QuestionChooser,
    Target,
    FirstResponse,
    Inactive,
    Dead,
    AnsweredCorrectly,
    AnsweredWrong,
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

    pub fn allowed_to_click(&self) -> bool {
        self.state != PlayerState::Dead && self.state != PlayerState::Inactive
    }
}

#[derive(Debug, Serialize, PartialEq)]
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
    pub game_pack: GamePack,
    pub hub: HubManager,
    pub current: CurrentContext,
}

#[derive(Default, Debug)]
pub struct CurrentContext {
    pub round_index: usize,
    active_player_id: u8,
    game_state: GameState,
    pub click_for_answer_allowed: bool,
    pub answer_allowed: bool,
    pub question_theme: String,
    pub question_price: i32,
    pub question_type: QuestionType,
    pub total_correct_answers: i32,
    pub total_wrong_answers: i32,
    pub total_tries: i32,
}

impl CurrentContext {
    pub fn active_player_id(&self) -> u8 {
        self.active_player_id
    }

    pub fn set_active_player_id(&mut self, new_id: u8) {
        self.active_player_id = new_id
    }
    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }
    pub fn set_game_state(&mut self, game_state: GameState) {
        self.game_state = game_state;
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum GameplayError {
    AnswerForbidden,
    PackElementNotPresent,
    PlayerNotPresent,
    HubOperationError,
    OperationForbidden,
}

impl fmt::Display for GameplayError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            GameplayError::PackElementNotPresent => { "Pack element not present" }
            GameplayError::PlayerNotPresent => { "Player is not present" }
            GameplayError::HubOperationError => { "HUB operation failed" }
            GameplayError::AnswerForbidden => { "Answer forbidden" }
            GameplayError::OperationForbidden => { "Operation forbidden" }
        };
        fmt.write_str(&format!("Gameplay error: {}", error))
    }
}

impl Error for GameplayError {}

lazy_static::lazy_static! {
    static ref CONTEXT: Arc<Mutex<GameContext>> = Arc::new(Mutex::new(GameContext::new()));
}

pub fn game() -> MutexGuard<'static, GameContext> {
    CONTEXT.lock().unwrap()
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameState {
    #[default]
    SetupAndLoading,
    QuestionChoosing,
    QuestionSelected,
    AnswerAllowed,
    AnswerRequested,
    AnswerWrong,
    AnswerCorrect,
    NoPlayersToAnswerLeft,
}

#[cfg(test)]
mod game_entities_test {
    use crate::core::game_entities::{GameContext, Player};

    #[test]
    fn test_fastest_click() {
        let mut ctx = GameContext::default();
        ctx.players.insert(1, Player::default());
        ctx.players.insert(2, Player::default());
        ctx.players.insert(3, Player::default());
        ctx.players.insert(4, Player::default());
        let i = ctx.get_fastest_click_player_id().unwrap();
        log::info!("Fastest click from: {i}");
    }
}