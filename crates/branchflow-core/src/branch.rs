/// Represents a branch in the domain model.
/// This is independent of the Git implementation.
pub struct Branch {
    pub name: String,
    pub commit_id: String,
}
