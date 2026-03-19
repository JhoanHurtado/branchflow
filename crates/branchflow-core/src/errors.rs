use thiserror::Error;

/// Core errors for the BranchFlow domain.
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("An unknown core error occurred")]
    Unknown,
}
