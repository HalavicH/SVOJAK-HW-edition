use crate::api::dto::{PlayerGameDto, PlayerScoreDto, PlayerStatsDto, QuestionDataDto, QuestionType, RoundDto, RoundStatsDto};
use tauri::command;
use crate::api::mapper::*;
use crate::core::game_entities::{game, GameplayError};
use crate::core::hub_manager::HubManagerError;

#[command]
pub fn fetch_players() -> Vec<PlayerGameDto> {
    let vec = map_players_to_player_game_dto(game().fetch_players());
    log::info!("Players: {:#?}", vec);
    vec
}

#[command]
pub fn fetch_round() -> RoundDto {
    let round_dto = map_round_to_dto(game().get_current_round());
    log::info!("{round_dto:#?}");
    round_dto
}

#[command]
pub fn get_question_data(topic: String, price: i32) -> QuestionDataDto {
    let (question, q_num) = game()
        .process_question_obtaining(&topic, &price).unwrap();

    map_question_to_question_dto(topic, question, q_num)
}

#[command]
pub fn allow_answer() -> Result<(), HubManagerError> {
    game().allow_answer()
        .map_err(|e| {
            log::error!("{:?}", e);
            e.current_context().clone()
        })
}

#[command]
pub fn get_fastest_click() -> Result<i32, GameplayError> {
    let id = game().get_fastest_click_player_id()
        .map_err(|e| {
            log::error!("{:?}", e);
            e.current_context().clone()
        })?;
    Ok(id as i32)
}

#[command]
pub fn answer_question(answered_correctly: bool) -> Result<bool, GameplayError> {
    log::info!("Answered correctly: {answered_correctly}");

    game().answer_question(answered_correctly).map_err(|e| {
        log::error!("Failed to answer question: {:?}", e);
        e.current_context().clone()
    })
}

#[command]
pub fn has_next_question() -> bool {
    game().has_next_question()
}

#[command]
pub fn send_pip_victim(victim_id: i32) {
    log::info!("Victim id is: {}", victim_id);
}

#[command]
pub fn get_active_player_id() -> i32 {
    game().get_active_player_id() as i32
}

#[command]
pub fn is_allow_answer_required() -> bool {
    game().current.question_type == QuestionType::Normal
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
