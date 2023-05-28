use std::ops::Deref;
use crate::api::dto::ConfigDto;
use crate::core::game_entities::{game_ctx, Player};
use crate::hw_comm::api::discover_serial_ports;

/// Takes whole game context and maps to config which contains only required elements
pub fn get_config_dto() -> ConfigDto {
    let context = game_ctx();
    ConfigDto::new(
        context.hub.port.clone(),
        discover_serial_ports(),
        context.hub.radio_channel,
        context.players.clone(),
    )
}

/// Takes whole game context and maps to config which contains only required elements
pub fn update_game_context(config: &ConfigDto) {
    let mut context = game_ctx();
    context.players = config.players.clone();
    context.hub.port = config.hub.port.clone();
    context.hub.radio_channel = config.hub.radio_channel;
}

/// Takes whole game context and maps to config which contains only required elements
pub fn update_players(players: &Vec<Player>) {
    let mut context = game_ctx();
    context.players = players.clone();
}