use std::fs;
use std::sync::Arc;
use tempfile::TempDir;
use zip::ZipArchive;

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
