use thiserror::Error;

/// Defines errors that can occur within the UI layer.
#[derive(Error, Debug)]
pub enum UiError {
    #[error("An unknown UI error occurred")]
    Unknown,
}