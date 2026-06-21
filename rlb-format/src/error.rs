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
        offset: u32,
        length: u32,
    },

    #[error("serialization produced {actual} bytes but computed file_size was {expected}")]
    SerializationMismatch { expected: u32, actual: usize },
    #[error("")]
    ValueTooLarge { context: &'static str, value: usize },
}
