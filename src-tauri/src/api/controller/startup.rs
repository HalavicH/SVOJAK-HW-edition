use std::sync::{mpsc, RwLockWriteGuard};
use error_stack::ResultExt;
use tauri::{command};
use crate::api::dto::{ConfigDto, HubRequestDto, HubResponseDto, PackInfoDto};
use crate::api::mapper::{get_config_dto, map_package_to_pack_info_dto, update_players};
use crate::core::game_entities::{game, GameplayError, HubStatus, Player, PlayerState};

use crate::api::dto::PlayerSetupDto;
use crate::core::game_logic::start_event_listener;
use crate::core::hub_manager::{HubManager, HubManagerError};
use crate::game_pack::game_pack_loader::{GamePackLoadingError, load_game_pack};
use crate::hw_comm::api_types::{HubIoError, HubRequest};

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
    let guard = game();
    let result = guard.get_unlocked_hub_mut().probe(&path);
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
    let guard = game();
    let mut hub_guard = guard.get_unlocked_hub_mut();

    if !hub_guard.is_hub_alive() {
        return Err(HubManagerError::NoResponseFromHub);
    }

    hub_guard.discover_terminals(channel_id)
        .map_err(|e| {
            log::error!("{:#?}", e);
            e.current_context().clone()
        })
}

/// Saves configuration to game context
#[command]
pub fn save_players(players: Vec<PlayerSetupDto>) {
    log::debug!("Updating game context with new config: {players:#?}");

    let player_entities = players
        .iter()
        .map(|player| {
            Player {
                icon: player.icon.clone(),
                name: player.name.clone(),
                term_id: player.termId,
                is_used: player.isUsed,
                state: PlayerState::Idle,
                stats: Default::default(),
            }
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
            game().game_pack = pack;

            let pack_info_dto = map_package_to_pack_info_dto(&game().game_pack.content);
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
    game().start_the_game().map_err(|e| {
        log::error!("{:#?}", e);
        e.current_context().clone()
    })
}

/// HUB Debug API
#[command]
pub fn setup_hub_connection(port_name: String) -> Result<(), HubManagerError> {
    log::info!("Trying to open HUB connection");
    let mut game_ctx = game();
    let mut hub = game_ctx.get_unlocked_hub_mut();
    hub.setup_hub_connection(&port_name)
        .map_err(|e| {
            log::error!("Operation failed: {:?}", e);
            e.current_context().clone()
        })
}

#[command]
pub fn send_raw_request_frame(request_frame: Vec<u8>) -> Result<Vec<u8>, HubIoError> {
    log::info!("Sending raw frame request to HUB");
    let guard = game();
    let mut hub_guard = guard.get_unlocked_hub_mut();
    let handler = hub_guard.hub_io_handler.as_mut()
        .ok_or(HubIoError::NotInitializedError)?;

    handler.send_raw_frame(request_frame).map_err(|e| {
        log::error!("Operation failed: {:?}", e);
        e.current_context().clone()
    })
}

// #[command]
// pub fn send_hub_command(request: HubRequestDto) -> Result<HubResponseDto, HubIoError> {
//     log::info!("Sending request to HUB.\n{:#?}", request);
//     let guard = game();
//     let mut hub_guard = guard.get_unlocked_hub_mut();
//     let handler = hub_guard.hub_io_handler.as_mut()
//         .ok_or(HubIoError::NotInitializedError)?;
//
//     let request_enum = HubRequest::from_debug_request(request);
//     let response = handler.send_command(request_enum)?;
//
//
//     let dto = HubResponseDto {
//         request_frame: "stuffed_frame from send_command()".to_string(),
//         response_frame: "response_frame from send_command()".to_string(),
//         generic_response_obj: "HubResponse from send_command()".to_string(),
//         response_obj: "response from any of high-level functions".to_string(),
//     };
//     Ok(dto)
// }

#[command]
pub fn send_hub_command(request: HubRequestDto) -> Result<HubResponseDto, HubManagerError> {
    log::info!("Sending request to HUB.\n{:#?}", request);
    let guard = game();
    let mut hub_guard = guard.get_unlocked_hub_mut();

    let request_enum = HubRequest::from_debug_request(request);
    let result = process_hub_command(&mut hub_guard, request_enum)
        .map_err(|e| e.current_context().clone())?;

    let dto = HubResponseDto {
        request_frame: "Watch logs (DEBUG)".to_string(),
        response_frame: "Watch logs (DEBUG)".to_string(),
        generic_response_obj: "".to_string(),
        response_obj: result,
    };
    Ok(dto)
}

fn process_hub_command(hub_guard: &mut RwLockWriteGuard<HubManager>, request_enum: HubRequest) -> error_stack::Result<String, HubManagerError> {
    match request_enum {
        HubRequest::SetTimestamp(timestamp) => {
            hub_guard.set_hub_timestamp(timestamp)?;
            Ok("".to_owned())
        }
        HubRequest::GetTimestamp => {
            let timestamp = hub_guard.get_hub_timestamp()?;
            Ok(format!("Hub timestamp: {}", timestamp))
        }
        HubRequest::SetHubRadioChannel(channel_num) => {
            hub_guard.set_hub_radio_channel(channel_num)?;
            Ok("Set hub radio channel successfully".to_owned())
        }
        HubRequest::SetTermRadioChannel(term_id, channel_num) => {
            hub_guard.set_term_radio_channel(term_id, channel_num)?;
            Ok(format!("Set terminal {} radio channel successfully", term_id))
        }
        HubRequest::PingDevice(term_id) => {
            hub_guard.ping_terminal(term_id)?;
            Ok(format!("Ping terminal {} successfully", term_id))
        }
        HubRequest::SetLightColor(term_id, color) => {
            hub_guard.set_term_light_color(term_id, color)?;
            Ok(format!("Set terminal {} light color successfully", term_id))
        }
        HubRequest::SetFeedbackLed(term_id, state) => {
            hub_guard.set_term_feedback_led(term_id, &state.into())?;
            Ok(format!("Set terminal {} feedback LED to {} successfully", term_id, state))
        }
        HubRequest::ReadEventQueue => {
            let events = hub_guard.read_event_queue()?;
            Ok(format!("Event queue: {:#?}", events))
        }
    }
}
