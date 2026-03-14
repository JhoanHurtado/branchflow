

# Contributing to BranchFlow

Thank you for your interest in contributing to BranchFlow.

This document describes the workflow, guidelines, and expectations for contributing to the project. The goal is to keep development organized and maintain a high code quality standard.

---

## Project Goals

BranchFlow aims to be a fast and intuitive Git client with a strong internal architecture. Contributions should prioritize:

- clarity of code
- maintainability
- performance
- alignment with the existing architecture

Before implementing large changes, it is recommended to open an issue to discuss the proposal.

---

## Development Setup

Clone the repository:

```bash
git clone https://github.com/jhoanhurtado/branchflow.git
cd branchflow
```

Make sure you have the following installed:

- Rust (stable toolchain)
- Cargo
- Git

Build the project:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

BranchFlow is organized as a Rust **workspace** composed of multiple crates located in the `crates/` directory.  
Each crate represents a different part of the system (core logic, git backend, CLI, etc.).  
Running `cargo build` or `cargo test` from the repository root builds and tests the entire workspace.

---

## Branching Workflow

This repository follows a structured branching strategy.

Main branches:

```
main
develop
```

- `main` contains stable releases.
- `develop` is the integration branch for ongoing development.

Feature and fix branches should be created from `develop`.

Examples:

```
feature/commit-graph
feature/github-integration
fix/status-detection
```

After completing the work, open a Pull Request targeting `develop`.

---

## Pull Request Guidelines

Before submitting a Pull Request:

1. Ensure the project builds successfully.
2. Run all tests.
3. Ensure the CI pipeline passes.
4. Follow the project's code style.
5. Keep commits focused and descriptive.

A good Pull Request should:

- explain **what changed**
- explain **why the change was needed**
- reference any related issue

Example PR title:

```
Add commit graph traversal support
```

---

## Commit Messages

Commit messages should be clear and concise.
Use the imperative mood when writing commit messages (e.g., "add feature" instead of "added feature").

Recommended format:

```
type: short description
```

Examples:

```
feat: add commit graph model
fix: correct branch detection
docs: update architecture documentation
```

Common commit types:

```
feat
fix
docs
refactor
test
chore
```

---

## Code Style

General guidelines:

- keep functions small and focused
- avoid unnecessary abstractions
- prefer clear code over clever code
- follow standard Rust formatting

Format code before committing:

```bash
cargo fmt
```

Run lints:

```bash
cargo clippy
```

---

## Issues

Issues should describe a clear problem or proposal.

When creating an issue include:

- a clear description
- steps to reproduce (if applicable)
- expected behavior
- current behavior

---

## Large Changes

If you want to introduce significant architectural changes:

1. Open an issue first.
2. Discuss the proposal with maintainers.
3. Agree on the design before implementation.

This prevents wasted work and keeps the project consistent.

---

## Code of Conduct

Contributors are expected to interact respectfully and professionally.

Constructive feedback and collaboration are encouraged.