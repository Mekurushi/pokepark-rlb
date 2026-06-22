mod header;
#[allow(clippy::map_err_ignore)]
mod raw_file;
mod relocation;

pub use header::{HEADER_SIZE, Header};
pub use raw_file::{RawFile, TableRecord};
pub use relocation::RelocationTable;
