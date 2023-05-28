use serde::Serialize;
use tauri::{command};
use crate::api::dto::ConfigDto;
use crate::api::mapper::{get_config_dto, update_game_context};
use crate::core::game_entities::{game_ctx, HubStatus, Player};
use crate::game_pack::pack_loader;
use crate::game_pack::pack_entities::Round;

/// Provide saved game configuration
#[command]
pub fn fetch_configuration() -> ConfigDto {
    println!("Fetching config");

    get_config_dto()
}

/// Queries OS for all available serial ports
#[command]
pub fn discover_serial_ports() -> Vec<String> {
    let ports = serialport::available_ports().expect("No ports found!");
    let mut ports_vec = Vec::new();

    for p in ports {
        println!("{}", p.port_name);

        ports_vec.push(p.port_name.clone());
    }

    ports_vec
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
pub fn save_config(config: ConfigDto) {
    println!("Updating game context with new config: {config:#?}");

    update_game_context(&config)
}

/// Load game pack into the game
#[command]
pub fn load_pack(path: String) {
    println!("Try to load pack from: {pack}");
    game_ctx().pack = pack_loader::load_pack(&path);
    println!("Pack contains: {:#?}", game_ctx().pack)
}
