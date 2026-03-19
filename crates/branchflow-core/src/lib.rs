/*!
BranchFlow Core

This crate contains the **domain model** for BranchFlow. It defines the
core data structures and abstractions used across the application.

Important architectural rules:

- This crate must remain **independent from Git implementations**
- It must **not depend on libgit2 or filesystem access**
- It only defines **domain models, traits, and errors**
*/

pub mod branch;
pub mod commit;
pub mod repository;
pub mod working_tree;
pub mod graph;
pub mod errors;

// Re-export commonly used domain types
pub use branch::Branch;
pub use commit::Commit;
pub use repository::Repository;
pub use working_tree::FileStatus;
pub use errors::CoreError;
