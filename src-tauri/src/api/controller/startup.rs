use tauri::{command};
use crate::api::dto::{ConfigDto, PackInfoDto};
use crate::api::mapper::{get_config_dto, map_package_to_pack_info_dto, update_players};
use crate::core::game_entities::{game_ctx, HubStatus, Player};

use crate::api::dto::PlayerSetupDto;
use crate::game_pack::game_pack_entites::GamePack;
use crate::game_pack::game_pack_loader::{GamePackLoadingError, load_game_pack};

/// Provide saved game configuration
#[command]
pub fn fetch_configuration() -> ConfigDto {
    println!("Fetching config");

    let config = get_config_dto();
    println!("Config: {:#?}", config);

    config
}

/// Tries to detect hub at given serial port. If successful saves port name
#[command]
pub fn discover_hub(path: String) -> HubStatus {
    println!("Pretend opening port: {path}");

    game_ctx().hub.probe(&path)
}

/// Calls HUB to set specific radio channel, pings all devices on that channel, devices which
/// replied considered as available and returned as vector
#[command]
pub fn discover_terminals(channel_id: i32) -> Vec<u8> {
    println!("Got channel id: {channel_id}");

    game_ctx().hub.discover_terminals(channel_id)
}

/// Saves configuration to game context
#[command]
pub fn save_players(players: Vec<PlayerSetupDto>) {
    println!("Updating game context with new config: {players:#?}");

    let player_entities = players
        .iter()
        .map(|player| {
            return Player {
                icon: player.icon.clone(),
                name: player.name.clone(),
                term_id: player.termId,
                is_used: player.isUsed,
                score: 0
            } 
        })
        .collect();

    println!("Converted players: {:#?}", player_entities);

    update_players(&player_entities)
}

/// Load game pack into the game
#[command]
pub fn get_pack_info(path: String) -> Result<PackInfoDto, GamePackLoadingError> {
    log::info!("Obtained package path: {}", path);

    let result = load_game_pack(path.as_str());

    match result {
        Ok(pack) => {
            game_ctx().pack = pack.content;

            let pack_info_dto = map_package_to_pack_info_dto(&game_ctx().pack);
            println!("Pack info: {:#?}", pack_info_dto);
            Ok(pack_info_dto)
        }
        Err(err) => {
            log::error!("\n{err:?}");
            Err((*err.current_context()).clone())
        }
    }
}
