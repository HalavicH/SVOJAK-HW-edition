// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;

#[derive(Debug, Serialize)]
enum HubStatus {
    Detected,
    Error
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn discover_terminals(channel_id: u8) -> Vec<u8> {
    println!("Got channel id: {channel_id}");

    vec![1, 2, 3, 4]
}

#[tauri::command]
fn discover_serial_ports() -> Vec<String> {
    let ports = serialport::available_ports().expect("No ports found!");
    let mut ports_vec = Vec::new();

    for p in ports {
        println!("{}", p.port_name);

        ports_vec.push(p.port_name.clone());
    }

    ports_vec
}

#[tauri::command]
fn open_selected_port(path: String) -> HubStatus {
    println!("Pretend opening port: {path}");

    HubStatus::Detected
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
                        greet, discover_terminals, discover_serial_ports,
                        open_selected_port])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    /*
     *  Pack loader usage example
     *
     * let package: Package = load_pack("path/to/content.xml");
     *
     * println!("{:#?}", package);
     *
     * Before using such modules should be included:
     *
     * use svoyak_tauri_app::game_pack::pack_entities::Package;
     * use svoyak_tauri_app::game_pack::pack_loader::load_pack;
     */
}
