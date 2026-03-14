


# BranchFlow Architecture

## Overview

BranchFlow is designed with a modular architecture that separates the Git engine from the user interfaces and external integrations. This separation allows the project to support multiple interfaces (CLI and GUI) while keeping the core logic stable, testable, and independent.

The architecture is built around four main layers:

1. Core Domain
2. Git Backend
3. Interfaces
4. Integrations

This structure ensures that the Git logic remains reusable and that new interfaces or integrations can be added without rewriting the core system.

---

## High-Level Architecture

```
                +----------------------+
                |        UI (Tauri)    |
                +----------+-----------+
                           |
                    +------+------+
                    |     CLI     |
                    +------+------+
                           |
                           |
                    +------+------+
                    | branchflow- |
                    |    core     |
                    +------+------+
                           |
                           |
                    +------+------+
                    | branchflow- |
                    |     git     |
                    +------+------+
                           |
                      +----+----+
                      | libgit2 |
                      +----+----+

External integrations (GitHub API, etc.) interact with the system through dedicated integration modules.
```

---

## Core Domain (branchflow-core)

The **Core Domain** crate (`branchflow-core`) contains the central data models and application logic used across the system.

Responsibilities:

- Repository management
- Commit history retrieval
- Branch management
- Working directory state
- Staging and committing changes

Key characteristics:

- Written entirely in Rust
- Independent from UI frameworks
- Independent from external services
- Fully testable

The Core Domain should not depend on the CLI, GUI, or external APIs.

---

## Git Backend (branchflow-git)

The Git backend crate (`branchflow-git`) is responsible for interacting directly with Git repositories through `libgit2`.

Responsibilities:

- Opening repositories
- Reading commits
- Reading branch references
- Accessing repository metadata
- Reading working tree state
- Accessing the Git index (staging area)

This crate acts as a bridge between the Git storage layer (`libgit2`) and the domain logic defined in `branchflow-core`.

---

## Application Layer

The Application Layer acts as an orchestration layer between the Core Engine and the interfaces.

Responsibilities:

- Coordinating operations between components
- Exposing high-level commands
- Managing application state
- Handling workflows such as commits, branch changes, and history navigation

Example operations handled by this layer:

- Commit workflow
- Branch switching
- History queries
- Repository initialization

---

## Interfaces

BranchFlow will support multiple interfaces.

### CLI Interface

Implemented in the `branchflow-cli` crate.

The CLI provides a lightweight and fast way to interact with repositories.

Examples:

```
branchflow status
branchflow commit -m "message"
branchflow log
branchflow checkout <branch>
```

The CLI communicates with the Application Layer, not directly with the Core Engine.

---

### Graphical Interface (GUI)

Implemented in the `branchflow-ui` crate.

The graphical interface will be implemented using Tauri.

Responsibilities:

- Visualizing repository state
- Displaying commit graphs
- Providing intuitive workflows
- Managing user interactions

The GUI should remain thin and delegate logic to the Application Layer.

---

## Integrations

External integrations allow BranchFlow to interact with hosting platforms such as GitHub.

Initial integration target:

- GitHub Pull Requests

Possible responsibilities:

- Authentication
- Pull Request creation
- Pull Request review
- Comment management

These integrations should remain separate from the Git Core Engine.

---

## Design Principles

### Separation of Concerns

Each layer has a clearly defined responsibility. This reduces coupling and improves maintainability.

### Interface Independence

The Git engine should work independently of the UI. New interfaces can be added without modifying the core.

### Testability

The architecture should allow the Core Engine and Application Layer to be tested independently.

### Extensibility

Future integrations and features should be added through new modules without modifying the core architecture.

---

## Suggested Project Structure

```
branchflow
│
├─ Cargo.toml
│
├─ crates
│   ├─ branchflow-core
│   │   └─ Core domain models and repository logic
│   │
│   ├─ branchflow-git
│   │   └─ Git backend built on top of libgit2
│   │
│   ├─ branchflow-cli
│   │   └─ Command line interface
│   │
│   └─ branchflow-ui
│       └─ Backend layer for the graphical interface
│
├─ docs
│   ├─ vision.md
│   ├─ architecture.md
│   └─ git-model.md
│
└─ integrations
    └─ github
```

---

## Future Evolution

As BranchFlow grows, the architecture may expand to support:

- Advanced commit graph rendering
- Performance optimizations for large repositories
- Additional hosting integrations
- Plugin or extension systems

However, the Core Engine should remain stable and independent of these features.