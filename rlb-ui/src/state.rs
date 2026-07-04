use rlb_domain::{RLBFile, TableId};
use std::path::PathBuf;

pub struct LoadedFile {
    pub file: RLBFile,
    pub path: PathBuf,
    pub selected_table: Option<TableId>,
    pub dirty: bool,
}

pub struct Status {
    pub message: String,
    pub is_error: bool,
}

impl Status {
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            is_error: false,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            is_error: true,
        }
    }
}

pub enum PendingAction {
    Open(PathBuf),
    Exit,
}

#[derive(Default)]
pub struct AppState {
    pub loaded: Option<LoadedFile>,
    pub status: Option<Status>,
    pub pending_action: Option<PendingAction>,
}
