use crate::api::dto::{PlayerGameDto, PlayerScoreDto, PlayerStatsDto, QuestionDataDto, RoundDto, RoundStatsDto};
use tauri::command;
use crate::api::dto::QuestionType::Normal;
use crate::api::mapper::*;
use crate::core::game_entities::{game_ctx};
use crate::game_pack::pack_content_entities::Question;

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

    map_question_to_question_dto(topic, price, question, q_num)
}

#[command]
pub fn has_next_question() -> bool {
    game_ctx().has_next_question()
}

#[command]
pub fn get_fastest_click() -> i32 {
    game_ctx().get_fastest_click()
}

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
pub fn allow_answer() {}

#[command]
pub fn wait_for_first_click() -> i32 {
    4
}

#[command]
pub fn fetch_round_stats() -> Option<RoundStatsDto> {
    Some(
        RoundStatsDto {
            roundNumber: 1,
            questionNumber: 30,
            normalQuestionNum: 27,
            pigInPokeQuestionNum: 3,
            totalCorrectAnswers: 25,
            totalWrongAnswers: 5,
            roundTime: "13:54".to_owned(),
            players: vec![
                PlayerStatsDto {
                    id: 1,
                    name: "HalavicH".to_owned(),
                    score: 400,
                    playerIconPath: "".to_owned(),
                    totalAnswers: 5,
                    answeredCorrectly: 3,
                    answeredWrong: 2,
                },
                PlayerStatsDto {
                    id: 2,
                    name: "Button".to_owned(),
                    score: 300,
                    playerIconPath: "".to_owned(),
                    totalAnswers: 5,
                    answeredCorrectly: 3,
                    answeredWrong: 2,
                },
                PlayerStatsDto {
                    id: 3,
                    name: "Minty".to_owned(),
                    score: 200,
                    playerIconPath: "".to_owned(),
                    totalAnswers: 5,
                    answeredCorrectly: 3,
                    answeredWrong: 2,
                },
            ],
        }
    )
}
