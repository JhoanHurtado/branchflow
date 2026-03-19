use branchflow_git::commits::get_log;
use branchflow_git::index::list_staged;
use branchflow_git::operations::{create_commit, stage_all, stage_path};
use branchflow_git::repository::GitRepository;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_stage_path() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    let file_path = dir.path().join("test.txt");
    File::create(&file_path).unwrap().write_all(b"hello").unwrap();

    stage_path(&repo, "test.txt").unwrap();

    let staged_files = list_staged(&repo).unwrap();
    assert_eq!(staged_files.len(), 1);
    assert_eq!(staged_files[0].path, "test.txt");
}

#[test]
fn test_stage_all() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    let file_path1 = dir.path().join("test1.txt");
    File::create(&file_path1).unwrap().write_all(b"hello").unwrap();

    let file_path2 = dir.path().join("test2.txt");
    File::create(&file_path2).unwrap().write_all(b"world").unwrap();

    stage_all(&repo).unwrap();

    let staged_files = list_staged(&repo).unwrap();
    assert_eq!(staged_files.len(), 2);
    let mut paths: Vec<_> = staged_files.iter().map(|f| f.path.clone()).collect();
    paths.sort();
    assert_eq!(paths, vec!["test1.txt", "test2.txt"]);
}

#[test]
fn test_create_first_commit() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    let file_path = dir.path().join("test.txt");
    File::create(&file_path)
        .unwrap()
        .write_all(b"initial commit")
        .unwrap();
    stage_path(&repo, "test.txt").unwrap();

    let commit_id =
        create_commit(&repo, "Initial commit", "Test Author", "test@example.com").unwrap();

    assert!(!commit_id.is_empty());

    let log = get_log(&repo, 1).unwrap();
    assert_eq!(log.len(), 1);
    assert_eq!(log[0].id, commit_id);
}

#[test]
fn test_create_subsequent_commit() {
    let dir = tempdir().unwrap();
    let repo = GitRepository::init(dir.path()).unwrap();

    // First commit
    let file_path1 = dir.path().join("file1.txt");
    File::create(&file_path1).unwrap().write_all(b"one").unwrap();
    stage_path(&repo, "file1.txt").unwrap();
    let first_commit_id = create_commit(&repo, "First", "Test", "test@example.com").unwrap();

    // Second commit
    let file_path2 = dir.path().join("file2.txt");
    File::create(&file_path2).unwrap().write_all(b"two").unwrap();
    stage_path(&repo, "file2.txt").unwrap();
    let second_commit_id = create_commit(&repo, "Second", "Test", "test@example.com").unwrap();

    let log = get_log(&repo, 2).unwrap();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0].parents[0], first_commit_id);
}