

/*!
BranchFlow UI

This crate defines the **user interface layer** for BranchFlow.

Responsibilities:

- Provide view models and UI state
- Call the application layer (`branchflow-app`)
- Translate application data into UI-friendly structures

Architectural constraints:

- The UI must depend on `branchflow-app`, not directly on `branchflow-core`
- No direct Git logic should exist here
*/

pub mod state;
pub mod view_models;
pub mod commands;
pub mod errors;

// Re-export commonly used UI types
pub use state::UiState;
pub use view_models::RepositoryView;