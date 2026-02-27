pub mod label;
pub mod todo;
pub mod user;

use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
    #[error("Not Found, id is {0}")]
    NotFound(i32),
    #[error("Duplicate data, id is {0}")]
    Duplicate(i32),
}