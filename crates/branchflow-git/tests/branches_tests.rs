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

    create_commit(&repo, "Initial", "Test", "test@example.com").unwrap();

    (dir, repo)
}

#[test]
fn test_list_branches_in_new_repo() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    // A repo without commits has no branches
    let branches = list_local_branches(&repo).unwrap();
    assert!(branches.is_empty());
}

#[test]
fn test_list_branches_single_branch() {
    let (_dir, repo) = setup_repo_with_commit();

    let branches = list_local_branches(&repo).unwrap();
    assert_eq!(branches.len(), 1);
    assert_eq!(branches[0].name, "main"); // git2 defaults to 'main'
    assert!(branches[0].is_head);
}

#[test]
fn test_list_branches_multiple_branches() {
    let (_dir, repo) = setup_repo_with_commit();
    let git_repo = &repo.inner;

    // Create a new branch "develop"
    let head_commit = git_repo.head().unwrap().peel_to_commit().unwrap();
    git_repo.branch("develop", &head_commit, false).unwrap();

    let branches = list_local_branches(&repo).unwrap();
    assert_eq!(branches.len(), 2);

    let mut names: Vec<_> = branches.iter().map(|b| b.name.clone()).collect();
    names.sort();
    assert_eq!(names, vec!["develop", "main"]);

    // 'main' should still be HEAD
    let main_branch = branches.iter().find(|b| b.name == "main").unwrap();
    assert!(main_branch.is_head);
}