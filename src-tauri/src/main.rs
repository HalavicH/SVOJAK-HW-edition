// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[allow(unused_imports)]

use svoyak_tauri_app::api::setup::*;
use svoyak_tauri_app::api::gameplay::*;
use svoyak_tauri_app::core::game_entities::*;

fn main() {
    log_ctx_content();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Setup API
            fetch_configuration,
            discover_hub,
            discover_terminals,
            save_players,
            // Gameplay API
            get_question,
        ])
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

fn log_ctx_content() {
    let mut context = game_ctx();

    // context.players.push(Player::new(1));
    // context.players.push(Player::new(2));
    // context.players.push(Player::new(3));
    // context.players.push(Player::new(4));
    // context.players.push(Player::new(5));

    println!("default context: {context:#?}");
}
