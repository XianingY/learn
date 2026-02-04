use thiserror::Error;

#[derive(Error, Debug)]
pub enum MiniRedisError {
    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Incomplete frame")]
    Incomplete,

    #[error("Command execution failed: {0}")]
    CommandExecution(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Connection reset by peer")]
    ConnectionReset,

    #[error("Protocol error: {0}")]
    Protocol(String),
}

pub type Result<T> = std::result::Result<T, MiniRedisError>;
