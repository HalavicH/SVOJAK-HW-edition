use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

use crate::game_pack::pack_entities::Package;

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
