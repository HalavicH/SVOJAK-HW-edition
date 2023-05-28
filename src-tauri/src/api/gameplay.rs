use serde::Serialize;
use tauri::command;
use crate::core::game_entities::Player;

// Score methods
/// Sends bool if answer is correct (todo: rethink the logic)
#[command]
pub fn get_question(category: String, price: i32) {
    todo!("Implement logic")
}

/// Sends bool if answer is correct (todo: rethink the logic)
#[command]
pub fn send_answer_status(correct_answer: bool) {
    todo!("Do logic by result")
}

/// Obtains players from internal storage and sets score (host override)
#[command]
pub fn set_player_score(name: String, score: i32) {
    todo!("Get player and set score")
}
