use serde::Serialize;
use tauri::{command};
use crate::api::dto::ConfigDto;
use crate::api::mapper::{get_config_dto, update_players};
use crate::core::game_entities::{game_ctx, HubStatus, Player};
use crate::game_pack::pack_loader;
use crate::game_pack::pack_entities::Round;
use tauri::api::dialog::FileDialogBuilder;

/// Provide saved game configuration
#[command]
pub fn fetch_configuration() -> ConfigDto {
    println!("Fetching config");

    get_config_dto()
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
pub fn save_players(players: Vec<Player>) {
    println!("Updating game context with new config: {config:#?}");

    update_players(&players)
}

/// Load game pack into the game
#[command]
pub fn load_pack() {
    // println!("Try to load pack from: {path}");
    // game_ctx().pack = pack_loader::load_pack(&path);
    // println!("Pack contains: {:#?}", game_ctx().pack)
}

// #[tauri::command]
// fn open_file_dialog() -> Result<String, String> {
//     let path = FileDialogBuilder::new()
//         .pick_files(|f| {
//             f.allowed_types(&["*"])
//         })
//         .show()
//         .unwrap()
//         .into_single()
//         .ok_or("No file selected")?;
//
//     let path_str = path.to_string_lossy().to_string();
//     Ok(path_str)
// }