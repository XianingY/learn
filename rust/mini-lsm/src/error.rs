use thiserror::Error;

#[derive(Error, Debug)]
pub enum LsmError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Block checksum mismatch: expected {expected}, actual {actual}")]
    ChecksumMismatch { expected: u32, actual: u32 },

    #[error("Key not found")]
    KeyNotFound,

    #[error("Invalid data format: {0}")]
    Format(String),
}

pub type Result<T> = std::result::Result<T, LsmError>;
