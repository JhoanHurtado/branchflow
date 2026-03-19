use branchflow_git::index::{clear, list_staged, unstage_path};
use branchflow_git::operations::stage_path;
use branchflow_git::repository::GitRepository;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

fn setup_repo_with_file(filename: &str) -> (tempfile::TempDir, GitRepository) {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    let file_path = dir.path().join(filename);
    File::create(&file_path)
        .unwrap()
        .write_all(b"content")
        .unwrap();

    (dir, repo)
}

#[test]
fn test_list_staged_empty() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();
    let staged_files = list_staged(&repo).unwrap();
    assert!(staged_files.is_empty());
}

#[test]
fn test_list_staged_with_one_file() {
    let (_dir, repo) = setup_repo_with_file("file1.txt");
    stage_path(&repo, "file1.txt").unwrap();

    let staged_files = list_staged(&repo).unwrap();
    assert_eq!(staged_files.len(), 1);
    assert_eq!(staged_files[0].path, "file1.txt");
}

#[test]
fn test_unstage_path() {
    let (_dir, repo) = setup_repo_with_file("file1.txt");
    stage_path(&repo, "file1.txt").unwrap();

    let staged_before = list_staged(&repo).unwrap();
    assert_eq!(staged_before.len(), 1);

    unstage_path(&repo, "file1.txt").unwrap();

    let staged_after = list_staged(&repo).unwrap();
    assert!(staged_after.is_empty());
}

#[test]
fn test_clear_index() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    // Create and stage two files
    let file_path1 = dir.path().join("file1.txt");
    File::create(&file_path1).unwrap().write_all(b"one").unwrap();
    stage_path(&repo, "file1.txt").unwrap();

    let file_path2 = dir.path().join("file2.txt");
    File::create(&file_path2).unwrap().write_all(b"two").unwrap();
    stage_path(&repo, "file2.txt").unwrap();

    clear(&repo).unwrap();

    let staged_after = list_staged(&repo).unwrap();
    assert!(staged_after.is_empty());
}