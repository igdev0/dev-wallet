mod account;
pub mod path_builder;
pub mod storage;
pub mod utils;
pub mod wallet;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("File not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Unable to create wallet")]
    WalletCreationError,
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] std::io::Error),
}
