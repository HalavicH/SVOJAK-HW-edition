use rand::Rng;

use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::core::hub_manager::HubManager;
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

impl GameContext {
    pub fn get_current_round(&self) -> &Round {
        let index = self.current_round_index as usize;
        let round = self.pack.rounds.get(index).unwrap();
        &round
    }

    fn get_current_round_mut(&mut self) -> &mut Round {
        let index = self.current_round_index as usize;
        let round = self.pack.rounds.get_mut(index).unwrap();
        round
    }

    pub fn pop_question(&mut self, theme: &String, price: &i32) -> Result<(Question, i32), String> {
        log::info!("Get question from category: {theme}, price: {price}");
        let round = self.get_current_round_mut();
        let theme = round.themes.get_mut(theme).ok_or("Theme not found".to_string())?;
        let question = theme.pop_question(price).ok_or("Question not found".to_string())?.clone();
        round.questions_left -= 1;
        log::info!("Question left: {}", round.questions_left);

        let question_number = round.question_count - round.questions_left;
        Ok((question, question_number))
    }

    pub fn has_next_question(&self) -> bool {
        self.get_current_round().questions_left > 0
    }

    pub fn get_fastest_click(&self) -> i32 {
        // TODO: Add logic for fastest click

        let random_int = rand::thread_rng().gen_range(1..100);
        log::info!("Fastest click from user: {}", random_int);
        random_int
    }
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