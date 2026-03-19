use branchflow_git::repository::GitRepository;
use tempfile::tempdir;

#[test]
fn test_init_repository() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();
    assert!(!repo.is_bare());
    assert!(repo.path().ends_with(".git"));
    let repo_path = repo.workdir().unwrap().canonicalize().unwrap();
    let dir_path = dir.path().canonicalize().unwrap();
    assert_eq!(repo_path, dir_path);
}

#[test]
fn test_open_repository() {
    let dir = tempdir().unwrap();
    let _ = GitRepository::init(dir.path()).unwrap();
    let repo = GitRepository::open(dir.path()).unwrap();
    assert!(!repo.is_bare());
}

#[test]
fn test_open_non_existent_repo_fails() {
    let dir = tempdir().unwrap();
    let result = GitRepository::open(dir.path());
    assert!(result.is_err());
}

#[test]
fn test_clone_repository() {
    // 1. Create a "remote" bare repository
    let remote_dir = tempdir().unwrap();
    let _remote_repo = git2::Repository::init_bare(remote_dir.path()).unwrap();

    // 2. Create a local directory to clone into
    let local_dir = tempdir().unwrap();

    // 3. Clone the repository
    let local_repo =
        GitRepository::clone(remote_dir.path().to_str().unwrap(), local_dir.path()).unwrap();

    assert!(!local_repo.is_bare());
    let repo_path = local_repo.workdir().unwrap().canonicalize().unwrap();
    let dir_path = local_dir.path().canonicalize().unwrap();
    assert_eq!(repo_path, dir_path);
}

#[test]
fn test_bare_repository() {
    let dir = tempdir().unwrap();
    let _ = git2::Repository::init_bare(dir.path()).unwrap();
    let repo = GitRepository::open(dir.path()).unwrap();

    assert!(repo.is_bare());
    assert!(repo.workdir().is_none());
}

use std::path::Path;

/// Helper: crea commit con archivo
fn create_commit(repo: &git2::Repository, file_name: &str, content: &str, message: &str) {
    let workdir = repo.workdir().unwrap();
    let file_path = workdir.join(file_name);

    std::fs::write(&file_path, content).unwrap();

    let mut index = repo.index().unwrap();
    index.add_path(Path::new(file_name)).unwrap();
    index.write().unwrap();

    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    let sig = repo.signature().unwrap();

    let parent = repo.head().ok()
        .and_then(|h| h.target())
        .and_then(|oid| repo.find_commit(oid).ok());

    match parent {
        Some(ref p) => {
            repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[p]).unwrap();
        }
        None => {
            repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[]).unwrap();
        }
    }
}

#[test]
fn test_repository_with_multiple_commits() {
    let dir = tempdir().unwrap();
    let repo = git2::Repository::init(dir.path()).unwrap();

    create_commit(&repo, "file1.txt", "hello", "first commit");
    create_commit(&repo, "file2.txt", "world", "second commit");

    let repo = GitRepository::open(dir.path()).unwrap();
    let head = repo.head().unwrap();

    assert!(head.is_branch());
}

#[test]
fn test_repository_with_branches() {
    let dir = tempdir().unwrap();
    let repo = git2::Repository::init(dir.path()).unwrap();

    create_commit(&repo, "file.txt", "base", "initial");

    let head_commit = repo.head().unwrap().peel_to_commit().unwrap();

    repo.branch("feature/test", &head_commit, false).unwrap();

    let branches = repo.branches(None).unwrap().count();

    assert!(branches >= 2);
}

#[test]
fn test_clone_repository_with_real_data() {
    let remote_dir = tempdir().unwrap();
    let repo = git2::Repository::init(remote_dir.path()).unwrap();

    create_commit(&repo, "file.txt", "content", "initial commit");

    let local_dir = tempdir().unwrap();

    let cloned = GitRepository::clone(
        remote_dir.path().to_str().unwrap(),
        local_dir.path(),
    ).unwrap();

    let _head = cloned.head().unwrap();
    let commit = cloned.head_commit().unwrap();

    assert_eq!(commit.message().unwrap(), "initial commit");
}

#[test]
fn test_repository_with_multiple_branches_and_commits() {
    let dir = tempdir().unwrap();
    let repo = git2::Repository::init(dir.path()).unwrap();

    create_commit(&repo, "file.txt", "base", "initial");

    let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
    repo.branch("feature/a", &head_commit, false).unwrap();
    repo.branch("feature/b", &head_commit, false).unwrap();

    let branches: Vec<_> = repo.branches(None).unwrap().collect();

    assert!(branches.len() >= 3);
}