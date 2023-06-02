use crate::api::dto::{PlayerGameDto, PlayerScoreDto, QuestionDataDto, RoundDto};
use tauri::command;
use crate::api::dto::QuestionType::Normal;
use crate::api::mapper::{map_players_to_player_game_dto, map_round_to_dto};
use crate::core::game_entities::{game_ctx};

#[command]
pub fn fetch_players() -> Vec<PlayerGameDto> {
    let players = &game_ctx().players;
    map_players_to_player_game_dto(players)
}

#[command]
pub fn fetch_round() -> RoundDto {
    let round_dto = map_round_to_dto(game_ctx().get_current_round());
    println!("{round_dto:#?}");
    round_dto
}

#[command]
pub fn get_question_data(topic: String, price: i32) -> QuestionDataDto {
    let (question, q_num) = game_ctx().pop_question(&topic, &price).unwrap();

    // TODO: Update with flexible scenarios
    let first_atom = question.scenario.get(0).unwrap();
    QuestionDataDto {
        number: q_num,
        category: topic,
        price,
        questionType: Normal,
        mediaType: first_atom.atom_type.clone(),
        content: first_atom.content.clone(),
    }
}

#[command]
pub fn is_last_question() -> bool{
    game_ctx().is_last_question()
}

#[command]
pub fn get_fastest_click() {}

#[command]
pub fn answer_question(answered_correctly: bool) -> PlayerScoreDto {
    println!("Answered correctly: {answered_correctly}");
    PlayerScoreDto {
        id: 2,
        score: 777,
    }
}

#[command]
pub fn send_pip_victim(victim_id: i32) {
    println!("Victim id is: {}", victim_id);

}

#[command]
pub fn get_active_player_id() -> i32 {
    2
}

#[command]
pub fn allow_answer() {
}

#[command]
pub fn wait_for_first_click() -> i32 {
    4
}
