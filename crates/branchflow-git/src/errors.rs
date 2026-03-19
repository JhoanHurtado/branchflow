use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Target not found: {0}")]
    NotFound(String),
}
