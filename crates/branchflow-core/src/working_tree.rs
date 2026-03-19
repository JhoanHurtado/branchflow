/// Represents the status of a file in the working tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileStatus {
    New,
    Modified,
    Deleted,
    Renamed,
    Typechange,
}
