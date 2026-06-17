use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid file size: header declares {expected} bytes but the file is {actual} bytes")]
    InvalidFileSize { expected: u32, actual: u64 },

    #[error("unexpected end of file while reading {context}")]
    UnexpectedEof { context: &'static str },

    #[error(
        "table `{table}` starting at address 0x{root_address:08x} has no terminator within the DATA segment"
    )]
    MissingTerminator { table: String, root_address: u32 },

    #[error("no table named `{0}` was discovered in this file")]
    TableNotFound(String),

    #[error("table `{table}` has no entry at index {index}")]
    IndexOutOfRange { table: String, index: usize },

    #[error(
        "field `{field}` on `{table}[{index}]` cannot be set this way (unknown, or pointer-typed)"
    )]
    TypeMismatch {
        table: String,
        index: usize,
        field: String,
    },

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    BinRw(#[from] binrw::Error),
}
