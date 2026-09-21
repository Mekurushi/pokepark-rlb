use rlb_domain::{RLBFile, TableId};
use std::path::PathBuf;

pub(crate) struct LoadedFile {
    pub file: RLBFile,
    pub path: PathBuf,
    pub selected_table: Option<TableId>,
    pub dirty: bool,
}

pub(crate) struct Status {
    pub message: String,
    pub is_error: bool,
}

impl Status {
    pub(crate) fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            is_error: false,
        }
    }

    pub(crate) fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            is_error: true,
        }
    }
}

pub(crate) enum PendingAction {
    Open(PathBuf),
    Exit,
}

#[derive(Default)]
pub(crate) struct AppState {
    pub loaded: Option<LoadedFile>,
    pub status: Option<Status>,
    pub pending_action: Option<PendingAction>,
}
