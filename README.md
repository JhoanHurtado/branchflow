# BranchFlow

Branchy is a fast and intuitive Git client designed to make
working with repositories easier and more visual.

Built with Rust and powered by libgit2, Branchy aims to provide
a consistent experience across Linux, macOS and Windows.

## Goals

- Clear and intuitive interface
- Fast performance
- Cross-platform consistency
- Visual commit history

## Planned Features

- Repository status management
- Staging and committing changes
- Branch management
- Interactive commit graph
- GitHub Pull Request integration
# BranchFlow

BranchFlow is a fast and intuitive Git client designed to make working with repositories easier and more visual.

Built with Rust and powered by libgit2, BranchFlow aims to provide a consistent and reliable experience across Linux, macOS, and Windows.

## Vision

BranchFlow focuses on simplifying common Git workflows while providing a clear visual understanding of repository history. The goal is to combine the simplicity of modern Git clients with powerful visualization tools.

## Goals

- Clear and intuitive interface
- Fast performance
- Cross-platform consistency
- Visual and interactive commit history
- Minimal and predictable workflows

## Planned Features

### Core Git Features

- Open and manage repositories
- View repository status
- Stage and unstage files
- Create commits
- Branch management
- Checkout branches
- Commit history exploration

### Visualization

- Interactive commit graph
- Clear representation of branches and merges
- Efficient navigation of large histories

### Interfaces

BranchFlow will provide two ways to interact with repositories:

**CLI**

A lightweight command-line interface for fast workflows.

Example:

```
branchflow status
branchflow commit -m "message"
branchflow log
```

**Graphical Interface**

A cross‑platform UI built with Tauri that focuses on clarity and usability.

Supported platforms:

- Linux
- macOS
- Windows

### Future Features

Planned for later phases:

- GitHub Pull Request management
- Pull Request creation
- Review approvals and comments
- Repository insights and history exploration

## Technology Stack

- Rust
- libgit2
- Tauri

## Project Status

BranchFlow is currently in early development. The first phase focuses on building a stable Git core and CLI interface before introducing the graphical interface.

## Branching Strategy

This repository follows a structured branching model to organize development, testing, and releases.

### Main Branches

```
main
develop
```

- **main**  
  Represents stable production-ready code. All official releases are tagged from this branch.

- **develop**  
  The primary integration branch where new development work is merged.

### Development Branches

```
feature/*
fix/*
```

- **feature/** branches are used for new features.

Example:

```
feature/commit-graph
feature/github-integration
```

- **fix/** branches are used for bug fixes discovered during development.

Example:

```
fix/status-detection
```

These branches are typically created from `develop` and merged back into `develop`.

### Release Branches

```
release/*
```

Release branches prepare a new version for production.

Example:

```
release/0.1.0
```

During this phase:

- final fixes are applied
- documentation is updated
- the version is stabilized

Once ready, the branch is merged into:

```
main
develop
```

### Hotfix Branches

```
hotfix/*
```

Hotfix branches are created from `main` to quickly patch production issues.

Example:

```
hotfix/0.1.1
```

After the fix:

- merge into `main`
- merge back into `develop`

### Tags

Official releases are marked using semantic version tags.

Example:

```
v0.1.0
v0.2.0
v1.0.0
```

Tags are always created from the `main` branch.

## License

TBD