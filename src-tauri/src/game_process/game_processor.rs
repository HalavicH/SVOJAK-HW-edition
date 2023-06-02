use std::fs;
use std::sync::Arc;
use tempfile::TempDir;
use zip::ZipArchive;

use crate::game_pack::pack_loader::load_pack;
use crate::game_process::game_info::*;

fn create_temp_directory() -> Arc<TempDir> {
    let temp_dir = Arc::new(TempDir::new().expect("Failed to create temporary directory"));

    temp_dir
}

fn unarchive_zip(archive_path: &str, directory_path: &str) {
    let file = fs::File::open(archive_path).expect("Failed to open pack archive file");

    let mut archive = ZipArchive::new(file).expect("Failed to create pack archive instance");

    archive
        .extract(directory_path)
        .expect("Failed to unpack archive");
}

pub fn load_game(game_archive_path: &str) -> GamePack {
    let temp_dir = create_temp_directory();
    let temp_dir_path = temp_dir.path();

    unarchive_zip(game_archive_path, temp_dir_path.to_str().unwrap());

    let locations = PackLocationData {
        base_dir: temp_dir.clone(),
        content_file_path: temp_dir_path.join(PACKAGE_CONTENT_FILE_NAME),
        audio_path: temp_dir_path.join(PACKAGE_AUDIO_DIR_NAME),
        images_path: temp_dir_path.join(PACKAGE_IMAGES_DIR_NAME),
        video_path: temp_dir_path.join(PACKAGE_VIDEO_DIR_NAME),
    };

    // TODO: Update media with full path
    let game_package = load_pack(&locations);

    GamePack {
        location: locations.clone(),
        content: game_package.clone(),
    }
}
