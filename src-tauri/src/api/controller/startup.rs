use tauri::{command};
use crate::api::dto::{ConfigDto, PackInfoDto};
use crate::api::mapper::{get_config_dto, map_package_to_pack_info_dto, update_players};
use crate::core::game_entities::{game_ctx, HubStatus, Player};

use crate::api::dto::PlayerSetupDto;
use crate::game_pack::game_pack_loader::load_game_pack;

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
pub fn get_pack_info() -> PackInfoDto {
    game_ctx().pack = load_game_pack("/Users/okholiavko/IdeaProjects/rust/svoyak-tauri-app/src-tauri/content.xml").content;

    let pack_info_dto = map_package_to_pack_info_dto(&game_ctx().pack);
    println!("Pack info: {:#?}", pack_info_dto);
    pack_info_dto
    // PackInfoDto {
    //     packName: "Ракування 2023".to_string(),
    //     packAuthor: "Злий репер зеник".to_string(),
    //     packRounds: 3,
    //     packTopics: 21,
    //     packQuestions: 67,
    //     packTopicList: vec![
    //         "Кавуни".to_string(),
    //         "Мемарня".to_string(),
    //         "Лесь подерв'янський".to_string(),
    //         "ГМО".to_string(),
    //         "Срала мазала ліпила".to_string(),
    //         "Іспанія".to_string(),
    //         "Орги бронукона".to_string(),
    //         "Металісти".to_string(),
    //         "Баба з хуйом".to_string(),
    //         "Пиво".to_string(),
    //         "Коломийки".to_string(),
    //         "Каракулєва шуба".to_string(),
    //         "Дослідники калу".to_string(),
    //         "Гамлєт".to_string(),
    //         "Совок".to_string(),
    //         "Табуретка".to_string(),
    //         "Нова Луняшна республіка".to_string(),
    //         "Кефір - не ряженка".to_string(),
    //         "Колеса".to_string(),
    //         "Скайрім".to_string(),
    //         "Угу".to_string(),
    //         ],
    // }
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