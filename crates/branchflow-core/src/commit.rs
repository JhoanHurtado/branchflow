/// Represents a commit in the domain model.
pub struct Commit {
    pub id: String,
    pub message: String,
    pub author: String,
    pub timestamp: i64,
}
