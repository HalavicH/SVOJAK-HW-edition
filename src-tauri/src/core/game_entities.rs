use std::sync::Arc;
use std::sync::Mutex;

use serde::{Serialize, Deserialize};
use crate::game_pack::pack_entities::Package;

#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    name: String,
    icon: String,
    // todo: make actual image
    term_id: u8,
    score: i32,
    is_used: bool,
}

impl Player {
    pub fn new(name: String, icon: String, term_id: u8) -> Self {
        Self { name, icon, term_id, score: 0, is_used: false }
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
    UnknownDevice,
    NoDevice,
}

impl Default for HubStatus {
    fn default() -> Self {
        HubStatus::NoDevice
    }
}

#[derive(Default, Debug)]
pub struct HubManager {
    pub port: String,
    pub status: HubStatus,
    pub radio_channel: i32,
    pub baudrate: i32,
    pub base_timestamp: u32,
}

impl HubManager {
    pub fn new(port: String, channel: i32, baudrate: i32, base_timestamp: u32) -> Self {
        Self { port, status: HubStatus::NoDevice, radio_channel: channel, baudrate, base_timestamp }
    }
}

#[derive(Default, Debug)]
pub struct GameContext {
    pub players: Vec<Player>,
    pub pack: Package,
    pub hub: HubManager,
}

lazy_static::lazy_static! {
    static ref CONTEXT: Arc<Mutex<GameContext>> = Arc::new(Mutex::new(GameContext::default()));
}

pub fn game_ctx() -> std::sync::MutexGuard<'static, GameContext> {
    CONTEXT.lock().unwrap()
}
