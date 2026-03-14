

# Git Model

## Overview

This document defines the internal Git model used by BranchFlow.  
The goal is to provide a clear abstraction over Git concepts while keeping the implementation close enough to Git's real data model.

BranchFlow does **not re‑implement Git**. Instead, it builds a structured internal representation of a repository using `libgit2`.

The model focuses on the following core entities:

- Repository
- Commit
- Branch
- Reference
- HEAD
- Index
- Working Tree

These entities represent the essential components required to understand and manipulate a Git repository.

---

# Repository

The **Repository** is the root object representing a Git project.

Responsibilities:

- Opening and validating a Git repository
- Accessing commits
- Accessing branches
- Managing references
- Reading repository configuration

Conceptually:

```
Repository
 ├─ HEAD
 ├─ Branches
 ├─ Commits
 ├─ Index
 └─ WorkingTree
```

Possible Rust representation:

```
struct Repository {
    path: PathBuf,
}
```

The repository acts as the entry point for all operations.

---

# Commit

A **Commit** represents a snapshot of the project at a specific point in time.

Each commit contains:

- Tree (snapshot of files)
- Parent commit(s)
- Author
- Committer
- Message
- Timestamp

Commit relationships form a **Directed Acyclic Graph (DAG)**.

Example:

```
A --- B --- C --- D
        \
         E --- F
```

Possible Rust representation:

```
struct Commit {
    id: Oid,
    message: String,
    parents: Vec<Oid>,
}
```

BranchFlow should treat commits primarily as nodes in a graph.

---

# Branch

A **Branch** is simply a movable reference pointing to a commit.

For example:

```
main → D
feature → F
```

Branches allow parallel lines of development.

Possible representation:

```
struct Branch {
    name: String,
    target: Oid
}
```

Important notes:

- Branches do not contain commits
- They only point to commits
- Moving a branch changes its target commit

---

# Reference

A **Reference** is a pointer to a Git object.

Common references:

- `refs/heads/*` (branches)
- `refs/tags/*` (tags)
- `HEAD`

Examples:

```
refs/heads/main
refs/heads/feature/login
refs/tags/v1.0
```

Possible model:

```
struct Reference {
    name: String,
    target: Oid
}
```

Branches are a specialized form of reference.

---

# HEAD

`HEAD` is a special reference that points to the currently checked-out branch or commit.

Two modes exist:

### Normal Mode

```
HEAD → refs/heads/main
```

### Detached HEAD

```
HEAD → commit
```

BranchFlow must handle both cases correctly.

Possible representation:

```
enum Head {
    Branch(String),
    Detached(Oid),
}
```

---

# Index (Staging Area)

The **Index** represents the staging area between the working directory and the next commit.

Workflow:

```
Working Tree → Index → Commit
```

Operations:

- Add files
- Remove files
- Prepare staged snapshot

Possible abstraction:

```
struct Index {
    entries: Vec<IndexEntry>
}
```

---

# Working Tree

The **Working Tree** represents the actual files on disk.

It reflects:

- Current checkout
- Uncommitted changes
- Untracked files

BranchFlow should be able to detect:

- Modified files
- New files
- Deleted files
- Staged changes

Example state:

```
src/main.rs      modified
README.md        staged
notes.txt        untracked
```

---

# Repository State

A repository can be in multiple states:

- Clean
- Modified
- Staged changes
- Merge in progress
- Rebase in progress

BranchFlow should expose a clear representation of the current state.

Example:

```
enum RepoState {
    Clean,
    Modified,
    Staged,
    Merging,
    Rebasing
}
```

---

# Commit Graph

The commit history is not a list.  
It is a **graph**.

BranchFlow should treat the commit history as a DAG.

Example:

```
        C
       /
A --- B --- D
       \
        E
```

Graph operations required:

- Traverse parents
- Traverse children
- Find merge base
- Build commit history views

---

# Key Principles

### Stay Close to Git

The internal model should reflect Git's real structure.

### Avoid Over-Abstraction

Do not hide Git concepts behind unnecessary abstractions.

### Graph-Oriented Thinking

Commits are nodes in a graph, not a simple history list.

### Clear Separation of Layers

The Git model belongs to the **Core Engine**, not to the UI or CLI.

---

# Future Extensions

The model may later include:

- Tags
- Remote tracking branches
- Reflog
- Submodules
- Git hooks

These features should extend the model without breaking the core entities defined here.