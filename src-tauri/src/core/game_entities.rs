#[allow(unused_imports, unused_variables)]

use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::game_pack::pack_entities::Package;

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
    pub players: HashMap<u8, Player>,
    pub pack: Package,
    pub hub: HubManager,
}

lazy_static::lazy_static! {
    static ref CONTEXT: Arc<Mutex<GameContext>> = Arc::new(Mutex::new(GameContext::default()));
}

pub fn game_ctx() -> std::sync::MutexGuard<'static, GameContext> {
    CONTEXT.lock().unwrap()
}
