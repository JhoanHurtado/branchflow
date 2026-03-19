use branchflow_git::commits::{get_commit, get_log};
use branchflow_git::operations::{create_commit, stage_path};
use branchflow_git::repository::GitRepository;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

// Helper to create a repo with one commit
fn setup_repo_with_commit() -> (tempfile::TempDir, GitRepository, String) {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    let file_path = dir.path().join("test.txt");
    File::create(&file_path)
        .unwrap()
        .write_all(b"hello")
        .unwrap();
    stage_path(&repo, "test.txt").unwrap();

    let commit_id = create_commit(&repo, "Test Commit", "Author", "author@test.com").unwrap();

    (dir, repo, commit_id)
}

#[test]
fn test_get_commit_valid() {
    let (_dir, repo, commit_id) = setup_repo_with_commit();

    let commit_data = get_commit(&repo, &commit_id).unwrap();

    assert_eq!(commit_data.id, commit_id);
    assert_eq!(commit_data.message, "Test Commit");
    assert_eq!(commit_data.author, "Author");
    assert!(commit_data.parents.is_empty());
}

#[test]
fn test_get_commit_invalid() {
    let (_dir, repo, _) = setup_repo_with_commit();
    let invalid_hash = "0123456789abcdef0123456789abcdef01234567";

    let result = get_commit(&repo, invalid_hash);
    assert!(result.is_err());
}

#[test]
fn test_get_log_empty_repo() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    // Should fail because HEAD has no target
    let result = get_log(&repo, 10);
    assert!(result.is_err());
}

#[test]
fn test_get_log_single_commit() {
    let (_dir, repo, commit_id) = setup_repo_with_commit();

    let log = get_log(&repo, 10).unwrap();
    assert_eq!(log.len(), 1);
    assert_eq!(log[0].id, commit_id);
}

#[test]
fn test_get_log_multiple_commits_and_limit() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    // Commit 1
    let file_path1 = dir.path().join("file1.txt");
    File::create(&file_path1).unwrap().write_all(b"one").unwrap();
    stage_path(&repo, "file1.txt").unwrap();
    let id1 = create_commit(&repo, "First", "Test", "test@example.com").unwrap();

    // Commit 2
    let file_path2 = dir.path().join("file2.txt");
    File::create(&file_path2).unwrap().write_all(b"two").unwrap();
    stage_path(&repo, "file2.txt").unwrap();
    let id2 = create_commit(&repo, "Second", "Test", "test@example.com").unwrap();

    // Test full log
    let log_full = get_log(&repo, 10).unwrap();
    assert_eq!(log_full.len(), 2);
    assert_eq!(log_full[0].id, id2);
    assert_eq!(log_full[1].id, id1);

    // Test limit
    let log_limited = get_log(&repo, 1).unwrap();
    assert_eq!(log_limited.len(), 1);
    assert_eq!(log_limited[0].id, id2);
}