use std::str::Utf8Error;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid file size: expected {expected} bytes, found {actual} bytes")]
    InvalidFileSize { expected: u32, actual: u64 },

    #[error("unexpected end of file while reading {context}")]
    UnexpectedEof { context: &'static str },

    #[error("offset {offset} is out of bounds for {context} (section length {length})")]
    OffsetOutOfBounds {
        context: &'static str,
        offset: usize,
        length: usize,
    },

    #[error("{context} contains invalid UTF-8 at offset {offset}")]
    InvalidUtf8 {
        context: &'static str,
        offset: usize,
        source: Option<Utf8Error>,
    },

    #[error("serialization produced {actual} bytes but computed file_size was {expected}")]
    SerializationMismatch { expected: u32, actual: usize },

    #[error("{context}: value {value} exceeds u32::MAX ({})", u32::MAX)]
    ValueTooLarge { context: &'static str, value: usize },

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("{context}: expected boolean encoded as 0 or 1, found {value}")]
    InvalidBoolean { context: &'static str, value: u8 },
}
