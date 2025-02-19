pub mod error_code;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("system error: {0}")]
    SystemError(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("db error: {0}")]
    DbError(#[from] surrealdb::Error),

    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("error message: {0}")]
    ErrorMessage(String),

    #[error("{message:} ({line:}, {column})")]
    CustomError {
        message: String,
        line: u32,
        column: u32,
    },

    #[error("error code: {0}")]
    ErrorCode(u16),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
