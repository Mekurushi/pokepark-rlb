use rlb_domain::RLBFile;
use std::path::{Path, PathBuf};

const FILTER_NAME: &str = "RLB files";
const FILTER_EXTENSIONS: &[&str] = &["rlb"];

pub(crate) fn pick_open_path() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter(FILTER_NAME, FILTER_EXTENSIONS)
        .pick_file()
}

pub(crate) fn pick_save_path(suggested_name: &str) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter(FILTER_NAME, FILTER_EXTENSIONS)
        .set_file_name(suggested_name)
        .save_file()
}

pub(crate) fn load_file(path: &Path) -> Result<RLBFile, String> {
    let bytes =
        std::fs::read(path).map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    RLBFile::parse(&bytes).map_err(|e| format!("failed to parse {}: {e}", path.display()))
}

pub(crate) fn save_file(file: &RLBFile, path: &Path) -> Result<(), String> {
    let bytes = file
        .clone()
        .write()
        .map_err(|e| format!("failed to serialize file: {e}"))?;
    std::fs::write(path, bytes).map_err(|e| format!("failed to write {}: {e}", path.display()))
}
