use crate::api::dto::{ConfigDto, RoundDto, TopicDto};
use crate::core::game_entities::{game_ctx, Player};
use crate::hw_comm::api::discover_serial_ports;
use std::collections::HashMap;
use crate::api::dto::{PackInfoDto, PlayerGameDto, QuestionDto};
use crate::game_pack::pack_content_entities::{PackContent, Question, Round, RoundType};

use super::dto::PlayerSetupDto;

/// Takes whole game context and maps to config which contains only required elements
pub fn get_config_dto() -> ConfigDto {
    let context = game_ctx();
    ConfigDto {
        available_ports: discover_serial_ports(),
        hub_port: context.hub.port_name.clone(),
        radio_channel: context.hub.radio_channel,
        players: context
            .players
            .iter()
            .map(|p| PlayerSetupDto {
                icon: p.1.icon.clone(),
                isUsed: p.1.is_used,
                name: p.1.name.clone(),
                termId: p.1.term_id,
            })
            .collect(),
    }
}

/// Takes whole game context and maps to config which contains only required elements
// pub fn update_game_context(config: &ConfigDto) {
//     let mut context = game_ctx();
//     context.players = config.players.clone();
//     context.hub.port = config.hub_port.clone();
//     context.hub.radio_channel = config.radio_channel;
// }

/// Takes whole game context and maps to config which contains only required elements
pub fn update_players(players: &Vec<Player>) {
    let mut context = game_ctx();

    context.players = players.iter()
        .fold(HashMap::new(), |mut map, player| {
            map.insert(player.term_id, player.clone());
            map
        });
}

pub fn map_package_to_pack_info_dto(package: &PackContent) -> PackInfoDto {
    let author = match package.info.authors.first() {
        Some(author) => author.name.clone(),
        None => String::new(),
    };

    let num_rounds = package.rounds.len() as i32;
    let num_topics = package.rounds.iter().map(|round| round.themes.len()).sum::<usize>() as i32;
    let num_questions = package
        .rounds
        .iter()
        .flat_map(|round| round.themes.iter())
        .map(|(_, theme)| theme.questions.len())
        .sum::<usize>() as i32;

    let topic_list: Vec<String> = package
        .rounds
        .iter()
        .flat_map(|round| round.themes.iter().map(|(_, theme)| theme.name.clone()))
        .collect();

    PackInfoDto {
        packName: package.name.clone(),
        packAuthor: author,
        packRounds: num_rounds,
        packTopics: num_topics,
        packQuestions: num_questions,
        packTopicList: topic_list,
    }
}

pub fn map_players_to_player_game_dto(players: &HashMap<u8, Player>) -> Vec<PlayerGameDto> {
    players.values()
        .map(|player| {
            PlayerGameDto {
                id: player.term_id as i32,
                playerIconPath: player.icon.clone(),
                playerName: player.name.clone(),
                score: player.score,
            }
        })
        .collect()
}


/// Converts a `Round` struct to a `RoundDto` struct.
///
/// # Arguments
///
/// * `round` - A reference to the `Round` struct to be converted.
///
/// # Returns
///
/// A `RoundDto` struct representing the converted `Round` struct.
///
/// # Examples
///
/// ```
/// use serde::Serialize;
/// use svoyak_tauri_app::api::mapper::map_round_to_dto;
/// use svoyak_tauri_app::game_pack::pack_content_entities::{RoundType, Theme};
///
/// #[derive(Debug, Serialize)]
/// #[allow(non_snake_case)]
/// pub struct Round {
///     pub name: String,
///     pub round_type: RoundType,
///     pub themes: Vec<Theme>,
/// }
///
/// // Assume proper implementations for RoundType and Theme structs.
///
/// let round = Round {
///     name: "1".to_string(),
///     round_type: RoundType::Normal,
///     themes: vec![
///         // ... populate the themes with your data
///     ],
/// };
///
/// let round_dto = map_round_to_dto(&round);
/// ```
pub fn map_round_to_dto(round: &Round) -> RoundDto {
    let round_type = match round.round_type {
        RoundType::Normal => "Normal",
        RoundType::Final => "Final",
    };

    let round_topics: Vec<TopicDto> = round
        .themes
        .iter()
        .map(|(_, theme)| {
            println!("{theme:#?}");
            let mut game_questions: Vec<Question> = theme.questions
                .values().cloned().collect::<Vec<Question>>();
            game_questions.sort_by(|q1, q2| { q1.price.cmp(&q2.price) });

            let mut questions = Vec::new();
            game_questions.iter()
                .enumerate()
                .for_each(|(i, q)| {
                    questions.push(
                        QuestionDto {
                            index: i,
                            price: q.price,
                        });
                });

            TopicDto {
                topicName: theme.name.clone(),
                questions,
            }
        })
        .collect();

    RoundDto {
        roundName: round.name.clone(),
        roundType: round_type.to_string(),
        roundTopics: round_topics,
    }
}
