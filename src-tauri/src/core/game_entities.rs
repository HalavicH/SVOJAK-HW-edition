use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::core::hub_manager::HubManager;
use crate::game_pack::pack_content_entities::{PackContent};

#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub icon: String,
    // todo: make actual image
    pub term_id: u8,
    pub score: i32,
    pub is_used: bool,
}

impl Player {
    pub fn new(term_id: u8) -> Self {
        Self {
            term_id,
            ..Default::default()
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self { name: "".to_string(), icon: "".to_string(), term_id: 0, score: 0, is_used: false }
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

#[derive(Default, Debug)]
pub struct GameContext {
    pub players: HashMap<u8, Player>,
    pub pack: PackContent,
    pub current_round_index: i32,
    pub hub: HubManager,
}

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