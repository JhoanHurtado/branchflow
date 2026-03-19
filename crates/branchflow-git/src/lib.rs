/*!
BranchFlow Git Backend

This crate provides the **Git adapter layer** for BranchFlow. It is responsible
for interacting with Git repositories through `libgit2` (via the `git2` crate)
and translating those operations into the domain models defined in
`branchflow-core`.

Architectural responsibilities:

- Interact with Git repositories using `git2`
- Convert Git data into `branchflow-core` domain models
- Isolate the rest of the application from Git implementation details
*/

pub mod repository;
pub mod commits;
pub mod branches;
pub mod index;
pub mod working_tree;
pub mod errors;

// Re-export the main repository adapter
pub use repository::GitRepository;
pub use errors::GitError;