use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::game_pack::pack_content_entities::{PackContent, Question, Round};

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
    pub pack: PackContent,
    pub current_round_index: i32,
    pub hub: HubManager,
}

impl GameContext {
    pub fn get_current_round(&self) -> &Round {
        let index = self.current_round_index as usize;
        let round = self.pack.rounds.get(index).unwrap();
        println!("Current round: #{index} name: {}", round.name);
        &round
    }

    pub fn pop_question(&mut self, theme: &String, price: &i32) -> Result<(Question, i32), String> {
        println!("Get question from category: {theme}, price: {price}");
        let round = self.get_current_round_mut();
        let theme = round.themes.get_mut(theme).ok_or("Theme not found".to_string())?;
        let question = theme.pop_question(price).ok_or("Question not found".to_string())?.clone();
        round.questions_left -= 1;
        println!("Question left: {}", round.questions_left);

        let question_number = round.question_count - round.questions_left;
        Ok((question, question_number))
    }

    pub(crate) fn is_last_question(&self) -> bool {
        self.get_current_round().questions_left <= 1
    }

    fn get_current_round_mut(&mut self) -> &mut Round {
        let index = self.current_round_index as usize;
        let round = self.pack.rounds.get_mut(index).unwrap();
        round
    }
}

lazy_static::lazy_static! {
    static ref CONTEXT: Arc<Mutex<GameContext>> = Arc::new(Mutex::new(GameContext::default()));
}

pub fn game_ctx() -> std::sync::MutexGuard<'static, GameContext> {
    CONTEXT.lock().unwrap()
}
