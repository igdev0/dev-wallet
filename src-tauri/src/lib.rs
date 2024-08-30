pub mod account;
pub mod config;
pub mod path_builder;
pub mod storage;
pub mod transaction;
pub mod utils;
pub mod wallet;
use thiserror::Error;

#[macro_use]
extern crate dotenv_codegen;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("File not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Failed to authenticate, incorrect password: {0}")]
    AuthenticationFailed(String),
    #[error("Unable to create wallet")]
    WalletCreationError,
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] std::io::Error),
}
