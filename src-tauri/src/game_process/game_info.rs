use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

use crate::game_pack::pack_entities::Package;

pub static PACKAGE_AUDIO_DIR_NAME: &str = "Audio";
pub static PACKAGE_CONTENT_FILE_NAME: &str = "content.xml";
pub static PACKAGE_IMAGES_DIR_NAME: &str = "Images";
pub static PACKAGE_VIDEO_DIR_NAME: &str = "Video";

#[derive(Debug, Clone)]
pub struct GameInfo {
    // !warning: if you lose this pointer, temp directory will be deleted
    pub pack_content_dir: Arc<TempDir>,
    pub pack_content_file_path: PathBuf,
    pub pack_video_path: PathBuf,
    pub pack_images_path: PathBuf,
    pub pack_audio_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct GameInstance {
    pub information: GameInfo,
    pub package: Package,
}
