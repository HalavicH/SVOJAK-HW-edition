use std::{fmt, fs, io};
use std::error::Error;
use std::sync::Arc;
use error_stack::{IntoReport, Report, Result, ResultExt};
use serde::Serialize;
use tempfile::TempDir;
use zip::ZipArchive;
use crate::game_pack::game_pack_entites::*;
use crate::game_pack::pack_content_loader::load_pack_content;

#[derive(Debug, Clone, Serialize)]
pub enum GamePackLoadingError {
    InvalidPathToPack(String),
    InvalidPackFileExtension(String),
    CorruptedPack(String),
    InternalError,
}

impl fmt::Display for GamePackLoadingError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to load game pack:")
    }
}

impl Error for GamePackLoadingError {}

/// Accepts path to pack, returns Result with GamePack or GamePackLoadingError
pub fn load_game_pack(game_archive_path: &str) -> Result<GamePack, GamePackLoadingError> {
    validate_pack_path(game_archive_path)?;

    let temp_dir = create_temp_directory()
        .change_context(GamePackLoadingError::InternalError)
        .attach_printable("Can't create temp directory")?;

    let temp_dir_path = temp_dir.path();
    let tmp_dir_path_str = temp_dir_path.to_str().ok_or(GamePackLoadingError::InternalError)?;

    unarchive_zip(game_archive_path, tmp_dir_path_str)?;

    let locations = PackLocationData {
        base_dir: temp_dir.clone(),
        content_file_path: temp_dir_path.join(PACKAGE_CONTENT_FILE_NAME),
        audio_path: temp_dir_path.join(PACKAGE_AUDIO_DIR_NAME),
        images_path: temp_dir_path.join(PACKAGE_IMAGES_DIR_NAME),
        video_path: temp_dir_path.join(PACKAGE_VIDEO_DIR_NAME),
    };

    // TODO: Update media with full path
    let err_message = format!("Can't load pack {game_archive_path}");
    let game_package = load_pack_content(&locations)
        .change_context(GamePackLoadingError::CorruptedPack(err_message.clone()))
        .attach_printable(err_message)?;

    Ok(GamePack {
        location: locations,
        content: game_package,
    })
}

fn validate_pack_path(game_archive_path: &str) -> Result<(), GamePackLoadingError> {
    let file_exists = std::path::Path::new(game_archive_path).exists();
    if !file_exists {
        let err_msg = format!("No pack found at: {}", game_archive_path);
        log::error!("{}", err_msg);
        return Err(Report::new(GamePackLoadingError::InvalidPathToPack(game_archive_path.to_string()))
            .attach_printable(err_msg));
    }

    if !game_archive_path.ends_with(".siq") {
        let file_name = game_archive_path.split("/").last().unwrap_or_default();
        let err_msg = format!("Provided file doesn't have '.siq' file extension. Yot file: {}", file_name);
        log::error!("{}", err_msg);
        return Err(Report::new(GamePackLoadingError::InvalidPackFileExtension(game_archive_path.to_string()))
            .attach_printable(err_msg));
    }

    Ok(())
}

fn create_temp_directory() -> Result<Arc<TempDir>, io::Error> {
    let tmp_dir = TempDir::new()?;
    let temp_dir = Arc::new(tmp_dir);

    Ok(temp_dir)
}

fn unarchive_zip(archive_path: &str, directory_path: &str) -> Result<(), GamePackLoadingError> {
    let file = fs::File::open(archive_path)
        .into_report()
        .change_context(GamePackLoadingError::InternalError)
        .attach_printable("Failed to open pack archive file")?;

    let mut archive = ZipArchive::new(file)
        .into_report()
        .attach_printable(format!("Failed to read archive {archive_path:?}"))
        .change_context(GamePackLoadingError::InternalError)?;

    archive.extract(directory_path)
        .into_report()
        .attach_printable("Failed to unpack archive")
        .change_context(GamePackLoadingError::InternalError)?;

    Ok(())
}
