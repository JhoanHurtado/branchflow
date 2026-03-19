use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Target not found: {0}")]
    NotFound(String),

    #[error("Invalid reference: {0}")]
    InvalidReference(String),

    #[error("Repository is in an invalid state: {0}")]
    InvalidRepositoryState(String),

    #[error("Operation not supported: {0}")]
    Unsupported(String),
}
