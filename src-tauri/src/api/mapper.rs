use crate::api::dto::ConfigDto;
use crate::core::game_entities::{game_ctx, Player};
use crate::hw_comm::api::discover_serial_ports;
use std::collections::HashMap;
#[allow(unused_imports, unused_variables)]
use std::ops::Deref;

use super::dto::PlayerSetupDto;

/// Takes whole game context and maps to config which contains only required elements
pub fn get_config_dto() -> ConfigDto {
    let context = game_ctx();
    ConfigDto {
        available_ports: discover_serial_ports(),
        hub_port: context.hub.port.clone(),
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

    let players_map: HashMap<String, Player> = HashMap::new();

    context.players = players.iter()
        .fold(HashMap::new(), |mut map, player| {
        map.insert(player.name.clone(), player.clone());
        map
    });
}
