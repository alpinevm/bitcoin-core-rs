use thiserror::Error;

#[derive(Error, Debug)]
pub enum BitcoinError {
    #[error("Failed to compute block hash")]
    BlockHashError,

    #[error("Failed to compute next work required")]
    WorkRequirementError,

    #[error("Failed to deserialize block header")]
    DeserializeError,
}
