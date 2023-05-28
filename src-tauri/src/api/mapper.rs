#[allow(unused_imports, unused_variables)]

use std::ops::Deref;
use crate::api::dto::ConfigDto;
use crate::core::game_entities::{game_ctx, Player};
use crate::hw_comm::api::discover_serial_ports;

use super::dto::PlayerSetupDto;

/// Takes whole game context and maps to config which contains only required elements
pub fn get_config_dto() -> ConfigDto {
    let context = game_ctx();
    ConfigDto {
        available_ports: discover_serial_ports(),
        hub_port: context.hub.port.clone(),
        radio_channel: context.hub.radio_channel,
        players: context.players
            .iter()
            .map(|p| {
                PlayerSetupDto {
                    icon: p.icon.clone(),
                    is_used: p.is_used,
                    name: p.name.clone(),
                    term_id: p.term_id
                }
            }).collect(),
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
    context.players = players.clone();
}