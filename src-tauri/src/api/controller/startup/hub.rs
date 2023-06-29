

use tauri::{command};


use crate::core::game_entities::{game, HubStatus};
use crate::hub_comm::common::hub_api::HubManager;




use crate::hub_comm::hw::hw_hub_manager::{HubManagerError};


/// Tries to detect hub at given serial port. If successful saves port name
#[command]
pub fn discover_hub(path: String) -> Result<HubStatus, HubManagerError> {
    let guard = game();
    let result = guard.get_locked_hub_mut().probe(&path);
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

/// Calls HUB to ping all devices on selected channel, devices which
/// replied considered as available. All available devices are returned as vector
#[command]
pub fn discover_terminals() -> Result<Vec<u8>, HubManagerError> {
    log::info!("Discovering terminals");
    let guard = game();
    let mut hub_guard = guard.get_locked_hub_mut();

    hub_guard.discover_terminals()
        .map_err(|e| {
            log::error!("{:#?}", e);
            e.current_context().clone()
        })
}
