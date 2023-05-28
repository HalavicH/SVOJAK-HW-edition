use std::ops::Deref;
use crate::api::dto::ConfigDto;
use crate::core::game_entities::game_ctx;

/// Takes whole game context and maps to config which contains only required elements
pub fn get_config_dto() -> ConfigDto {
    let context = game_ctx();
    ConfigDto::new(
        context.players.clone(),
        context.hub.port.clone(),
        context.hub.radio_channel
    )
}

/// Takes whole game context and maps to config which contains only required elements
pub fn update_game_context(config: &ConfigDto) {
    let mut context = game_ctx();
    context.players = context.players.clone();
    context.hub.port = context.hub.port.clone();
    context.hub.radio_channel = context.hub.radio_channel;
}