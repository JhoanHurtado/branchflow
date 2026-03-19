use branchflow_git::operations::{create_commit, stage_path};
use branchflow_git::references::{get_head, get_head_commit_id};
use branchflow_git::repository::GitRepository;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_get_head_on_new_repo() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    let head_ref = get_head(&repo).unwrap();

    assert_eq!(head_ref.name, Some("refs/heads/main".to_string()));
    assert!(head_ref.target.is_none()); // Unborn branch
    assert!(!head_ref.is_detached);
}

#[test]
fn test_get_head_commit_id_on_new_repo() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    // Fails because HEAD is unborn and has no target OID
    let result = get_head_commit_id(&repo);
    assert!(result.is_err());
}

#[test]
fn test_get_head_after_commit() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    let file_path = dir.path().join("test.txt");
    File::create(&file_path).unwrap().write_all(b"hello").unwrap();
    stage_path(&repo, "test.txt").unwrap();
    let commit_id = create_commit(&repo, "Initial", "Test", "test@example.com").unwrap();

    let head_ref = get_head(&repo).unwrap();
    assert_eq!(head_ref.name, Some("refs/heads/main".to_string()));
    assert_eq!(head_ref.target, Some(commit_id.clone()));
    assert!(!head_ref.is_detached);

    let head_commit_id = get_head_commit_id(&repo).unwrap();
    assert_eq!(head_commit_id, commit_id);
}

#[test]
fn test_get_head_detached() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    let file_path = dir.path().join("test.txt");
    File::create(&file_path).unwrap().write_all(b"hello").unwrap();
    stage_path(&repo, "test.txt").unwrap();
    let commit_id = create_commit(&repo, "Initial", "Test", "test@example.com").unwrap();

    // Detach HEAD
    repo.inner.set_head(&commit_id).unwrap();

    let head_ref = get_head(&repo).unwrap();
    assert!(head_ref.name.is_none()); // Detached HEAD has no symbolic name
    assert_eq!(head_ref.target, Some(commit_id.clone()));
    assert!(head_ref.is_detached);
}