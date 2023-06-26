// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[allow(unused_imports)]

use svoyak_tauri_app::api::controller::startup::*;
use svoyak_tauri_app::api::controller::gameplay::*;
use svoyak_tauri_app::core::game_entities::*;

fn main() {
    env_logger::init();

    log_ctx_content();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Startup API
            fetch_configuration,
            discover_hub,
            discover_terminals,
            save_players,
            get_pack_info,
            save_round_duration,
            get_pack_info,
            start_the_game,
            // Debug API
            send_raw_request_frame,
            setup_hub_connection,
            // Gameplay API
            fetch_players,
            fetch_round,
            get_question_data,
            allow_answer,
            get_fastest_click,
            answer_question,
            has_next_question,
            init_next_round,
            send_pip_victim,
            get_active_player_id,
            is_allow_answer_required,
            fetch_round_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    /*
     *  Game loader usage example
     *
     * let game: GameInstance = load_game("path/to/game_package.siq");
     *
     * log::info!("{:#?}", game);
     *
     * Before using such modules should be included:
     *
     * use svoyak_tauri_app::game_process::game_processor::load_game;
     * use svoyak_tauri_app::game_process::game_info::GameInstance;
     *
     * Out example:
     *
     * GameInstance {
     *     information: GameInfo {
     *         pack_content_dir: TempDir {
     *             path: "/var/folders/s2/99s5p5054xz0kgpp1y4f0slm0000gn/T/.tmpha0THA",
     *         },
     *         pack_content_file_path: "/var/folders/s2/99s5p5054xz0kgpp1y4f0slm0000gn/T/.tmpha0THA/content.xml",
     *         pack_video_path: "/var/folders/s2/99s5p5054xz0kgpp1y4f0slm0000gn/T/.tmpha0THA/Video",
     *         pack_images_path: "/var/folders/s2/99s5p5054xz0kgpp1y4f0slm0000gn/T/.tmpha0THA/Images",
     *         pack_audio_path: "/var/folders/s2/99s5p5054xz0kgpp1y4f0slm0000gn/T/.tmpha0THA/Audio",
     *     },
     *     package: Package {
     *         ...
     *     }
     * }
     */
}

fn log_ctx_content() {
    let context = game();
    log::info!("default context: {context:#?}");
}
