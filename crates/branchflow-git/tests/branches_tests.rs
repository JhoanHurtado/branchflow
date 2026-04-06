use branchflow_git::branches::list_local_branches;
use branchflow_git::operations::{create_commit, stage_path};
use branchflow_git::repository::GitRepository;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

// Helper to create a repo with one commit
fn setup_repo_with_commit() -> (tempfile::TempDir, GitRepository) {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    let file_path = dir.path().join("test.txt");
    File::create(&file_path)
        .unwrap()
        .write_all(b"hello")
        .unwrap();

    stage_path(&repo, "test.txt").unwrap();
    create_commit(&repo, "Initial commit", "Test", "test@example.com").unwrap();

    (dir, repo)
}

#[test]
fn test_list_branches_multiple_branches() {
    let (_dir, repo) = setup_repo_with_commit();

    // List branches after initial commit
    let branches = list_local_branches(&repo).unwrap();

    assert_eq!(branches.len(), 1);
    assert_eq!(branches[0].name, "main");
    assert!(branches[0].is_head);
}