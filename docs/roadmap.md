# BranchFlow Roadmap

## Overview

This document outlines the planned development stages for BranchFlow.

The goal of the roadmap is to guide development from a minimal usable Git client to a more complete developer tool. Each phase focuses on delivering a functional milestone rather than attempting to implement all Git features at once.

BranchFlow is organized as a Rust workspace composed of several crates:

- `branchflow-core` – domain models and repository abstractions
- `branchflow-git` – Git backend built on top of libgit2
- `branchflow-app` – application layer coordinating workflows
- `branchflow-cli` – command line interface
- `branchflow-ui` – backend layer for the graphical interface

The roadmap is divided into the following phases:

1. Git Backend Foundation
2. Read‑Only Repository Model
3. Commit Graph Engine
4. Application Layer
5. CLI (Read‑Only)
6. Repository Mutations
7. Graphical Interface
8. Advanced Git Workflows
9. Remote Git Support
10. Hosting Platform Integrations

---

# Phase 1 — Git Backend Foundation

Goal: Build the low‑level Git access layer.

This phase focuses on implementing the Git backend that interacts directly with `libgit2`.

Crate involved:

```
branchflow-git
```

Key features:

- Open existing repositories
- Initialize repositories
- Access repository metadata
- Read commit objects
- Read branch references
- Read repository HEAD
- Access working tree state
- Access index (staging area)

Milestone:

BranchFlow can open a repository and inspect its Git data through the backend layer.

---

# Phase 2 — Read‑Only Repository Model

Goal: Implement the core domain model for inspecting repositories.

This phase focuses strictly on **reading repository data**, not modifying it.

Crate involved:

```
branchflow-core
```

Core modules:

```
repository
commits
branches
working_tree
```

Key features:

- Repository abstraction
- Commit object model
- Branch modeling
- HEAD resolution
- Working tree inspection
- Index inspection

Milestone:

BranchFlow can inspect a repository programmatically through a stable domain model.

---

# Phase 3 — Commit Graph Engine

Goal: Build the internal commit graph representation.

The commit graph is a core component used by both the CLI and GUI to traverse history efficiently.

Crate involved:

```
branchflow-core
```

Responsibilities:

- commit history traversal
- DAG representation of commits
- branch decorations
- topological ordering
- pagination of large histories

Performance considerations:

- lazy loading of commits
- caching strategies
- handling repositories with large histories

Milestone:

BranchFlow can efficiently traverse and visualize repository history.

---

# Phase 4 — Application Layer

Goal: Introduce the orchestration layer that connects the domain model, Git backend, and external interfaces.

Crate involved:

```
branchflow-app
```

Responsibilities:

- Coordinate operations between `branchflow-core` and `branchflow-git`
- Expose high-level workflows used by CLI and GUI
- Provide a stable API for interfaces
- Handle repository-level operations

Example operations:

- open_repository
- get_status
- get_log
- list_branches
- resolve_head

Milestone:

A stable application service layer exists that both CLI and GUI can use without directly depending on Git internals.

---

# Phase 5 — CLI (Read‑Only)

Goal: Provide a minimal CLI for inspecting repositories.

The CLI acts as an early integration layer and a testing surface for the core engine.

Crate involved:

```
branchflow-cli
```

Initial commands:

```
branchflow status
branchflow log
branchflow branch
branchflow show
```

Milestone:

BranchFlow CLI can inspect repositories and display history information.

---

# Phase 6 — Repository Mutations

Goal: Allow BranchFlow to modify repositories.

Crates involved:

```
branchflow-app
branchflow-core
branchflow-git
```

Features:

- stage files
- unstage files
- create commits
- amend commits
- basic branch creation and deletion

Design constraints:

- mutations must go through the application layer
- the core domain must remain independent from libgit2
- operations should be transactional where possible

Milestone:

BranchFlow can perform basic Git write operations such as staging files and creating commits.

---

# Phase 7 — Graphical Interface

Goal: Provide a graphical interface for visualizing repositories.

Technology:

- Tauri
- Web-based UI (React, Svelte, or similar)

Crate involved:

```
branchflow-ui
```

Initial GUI features:

- repository dashboard
- commit history viewer
- commit graph visualization
- branch list
- file changes view
- staging interface
- commit creation

Milestone:

Users can explore repositories visually and create commits through the GUI.

---

# Phase 8 — Advanced Git Workflows

Goal: Support real-world Git workflows.

Features:

- merge operations
- rebase workflows
- conflict detection
- conflict resolution UI
- amend commits
- advanced history navigation

Graph improvements:

- optimized commit graph traversal
- large repository performance

Milestone:

BranchFlow supports typical development workflows.

---

# Phase 9 — Remote Git Support

Goal: Support interaction with remote repositories.

Features:

- fetch
- push
- pull
- remote management
- authentication handling

Milestone:

BranchFlow can synchronize repositories with remote Git servers.

---

# Phase 10 — Hosting Platform Integrations

Goal: Integrate with Git hosting platforms.

Initial target:

- GitHub

Possible features:

- authentication via OAuth
- pull request creation
- pull request review
- issue linking
- branch synchronization

Milestone:

BranchFlow supports collaboration workflows with hosted repositories.

---

# Development Principles

Throughout all phases, the following principles should guide development.

### Ship Small Iterations

Focus on completing small, usable milestones rather than implementing many unfinished features.

### Protect the Core Engine

The core logic must remain independent from CLI, GUI, and integrations.

### Avoid Feature Creep

Not every Git feature needs to be implemented immediately.

### Maintain Clear Boundaries

Each crate should have a clear responsibility and minimal coupling.

### Testing Strategy

Development should include strong automated testing:

- integration tests using real Git repositories
- repository fixtures for edge cases
- compatibility testing against common Git workflows
- testing with large repositories

Reliable Git behavior is critical for user trust.

---

# First Target Release

Version **0.1.0**

Minimum feature set:

- open repository
- view commit history
- view branches
- stage files
- create commits
- CLI interface

This version focuses on validating the architecture and core engine.