use branchflow_git::operations::{create_commit, stage_path};
use branchflow_git::repository::GitRepository;
use branchflow_git::working_tree::status;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

// Helper to create a repo with one committed file
fn setup_repo_with_committed_file(
    filename: &str,
    content: &str,
) -> (tempfile::TempDir, GitRepository) {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    let file_path = dir.path().join(filename);
    File::create(&file_path)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
    stage_path(&repo, filename).unwrap();

    create_commit(&repo, "Initial", "Test", "test@example.com").unwrap();

    (dir, repo)
}

#[test]
fn test_status_clean() {
    let (_dir, repo) = setup_repo_with_committed_file("file.txt", "clean");
    let statuses = status(&repo).unwrap();
    assert!(statuses.is_empty());
}

#[test]
fn test_status_new_untracked_file() {
    let (dir, repo) = setup_repo_with_committed_file("file.txt", "clean");

    let new_file_path = dir.path().join("new.txt");
    File::create(new_file_path).unwrap().write_all(b"new").unwrap();

    let statuses = status(&repo).unwrap();
    assert_eq!(statuses.len(), 1);
    let s = &statuses[0];
    assert_eq!(s.path, "new.txt");
    assert!(s.is_new);
    assert!(!s.is_staged);
}

#[test]
fn test_status_modified_file() {
    let (dir, repo) = setup_repo_with_committed_file("file.txt", "clean");

    let file_path = dir.path().join("file.txt");
    fs::write(file_path, "modified").unwrap();

    let statuses = status(&repo).unwrap();
    assert_eq!(statuses.len(), 1);
    let s = &statuses[0];
    assert_eq!(s.path, "file.txt");
    assert!(s.is_modified);
    assert!(!s.is_staged);
}

#[test]
fn test_status_deleted_file() {
    let (dir, repo) = setup_repo_with_committed_file("file.txt", "clean");

    let file_path = dir.path().join("file.txt");
    fs::remove_file(file_path).unwrap();

    let statuses = status(&repo).unwrap();
    assert_eq!(statuses.len(), 1);
    let s = &statuses[0];
    assert_eq!(s.path, "file.txt");
    assert!(s.is_deleted);
    assert!(!s.is_staged);
}

#[test]
fn test_status_staged_new_file() {
    let (dir, repo) = setup_repo_with_committed_file("file.txt", "clean");

    let new_file_path = dir.path().join("new.txt");
    File::create(new_file_path).unwrap().write_all(b"new").unwrap();
    stage_path(&repo, "new.txt").unwrap();

    let statuses = status(&repo).unwrap();
    assert_eq!(statuses.len(), 1);
    let s = &statuses[0];
    assert_eq!(s.path, "new.txt");
    assert!(s.is_new);
    assert!(s.is_staged);
}