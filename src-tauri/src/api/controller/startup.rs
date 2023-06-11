use tauri::{command};
use crate::api::dto::{ConfigDto, PackInfoDto};
use crate::api::mapper::{get_config_dto, map_package_to_pack_info_dto, update_players};
use crate::core::game_entities::{game, GameplayError, HubStatus, Player, PlayerState};

use crate::api::dto::PlayerSetupDto;
use crate::core::hub_manager::HubManagerError;
use crate::game_pack::game_pack_loader::{GamePackLoadingError, load_game_pack};

/// Provide saved game configuration
#[command]
pub fn fetch_configuration() -> ConfigDto {
    log::info!("Fetching config");

    let config = get_config_dto();
    log::info!("Config: {:#?}", config);

    config
}

/// Tries to detect hub at given serial port. If successful saves port name
#[command]
pub fn discover_hub(path: String) -> Result<HubStatus, HubManagerError> {
    let result = game().hub.probe(&path);
    match result {
        Ok(status) => {
            log::info!("Hub status: {:?}", status);
            Ok(status)
        }
        Err(error_stack) => {
            log::error!("Can't open port: {:?}", error_stack);
            Err(error_stack.current_context().clone())
        }
    }
}

/// Calls HUB to set specific radio channel, pings all devices on that channel, devices which
/// replied considered as available and returned as vector
#[command]
pub fn discover_terminals(channel_id: i32) -> Result<Vec<u8>, HubManagerError> {
    log::info!("Got channel id: {channel_id}");

    if !game().hub.is_alive() {
        return Err(HubManagerError::NoResponseFromHub);
    }

    Ok(game().hub.discover_terminals(channel_id))
}

/// Saves configuration to game context
#[command]
pub fn save_players(players: Vec<PlayerSetupDto>) {
    log::debug!("Updating game context with new config: {players:#?}");

    let player_entities = players
        .iter()
        .map(|player| {
            return Player {
                icon: player.icon.clone(),
                name: player.name.clone(),
                term_id: player.termId,
                is_used: player.isUsed,
                state: PlayerState::Idle,
                stats: Default::default(),
            };
        })
        .collect();

    log::info!("Converted players: {:#?}", player_entities);

    update_players(&player_entities)
}

/// Load game pack into the game
#[command]
pub fn get_pack_info(path: String) -> Result<PackInfoDto, GamePackLoadingError> {
    log::info!("Obtained package path: {}", path);

    let result = load_game_pack(path.as_str());

    match result {
        Ok(pack) => {
            game().pack = pack.content;

            let pack_info_dto = map_package_to_pack_info_dto(&game().pack);
            log::info!("Pack info: {:#?}", pack_info_dto);
            Ok(pack_info_dto)
        }
        Err(err) => {
            log::error!("\n{err:?}");
            Err(err.current_context().clone())
        }
    }
}

#[command]
pub fn save_round_duration(round_minutes: i32) {
    log::info!("Round duration is {round_minutes}");
}

#[command]
pub fn start_the_game() -> Result<(), GameplayError> {
    log::info!("Triggered the game start");
    game().start_the_game().map_err(|e|{
        log::error!("{:#?}", e);
        e.current_context().clone()
    })
}
