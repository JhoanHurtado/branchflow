use branchflow_git::remote::{clone_repository, list_remote_branches, list_remotes};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

// Helper to create a bare "remote" repo with a commit and a branch
fn setup_remote_repo() -> (tempfile::TempDir, String) {
    let remote_dir = tempdir().unwrap();
    let remote_path_str = remote_dir.path().to_str().unwrap().to_string();

    // Init a temporary normal repo to create a commit
    let temp_repo_dir = tempdir().unwrap();
    let repo = git2::Repository::init(temp_repo_dir.path()).unwrap();
    let mut opts = git2::RepositoryInitOptions::new();
    opts.initial_head("main");
    let repo = git2::Repository::init_opts(temp_repo_dir.path(), &opts).unwrap();

    // Create a commit
    let file_path = temp_repo_dir.path().join("file.txt");
    File::create(&file_path)
        .unwrap()
        .write_all(b"data")
        .unwrap();
    let mut index = repo.index().unwrap();
    index.add_path(std::path::Path::new("file.txt")).unwrap();
    index.write().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let sig = git2::Signature::now("test", "test@example.com").unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[])
        .unwrap();

    // Now, create the bare repo and push to it
    let bare_repo = git2::Repository::init_bare(&remote_path_str).unwrap();
    let mut remote = bare_repo
        .remote_anonymous(temp_repo_dir.path().to_str().unwrap())
        .unwrap();
    remote
        .push(&["refs/heads/main:refs/heads/main"], None)
        .unwrap();

    (remote_dir, remote_path_str)
}

#[test]
fn test_clone_repo() {
    let (_remote_dir, remote_path) = setup_remote_repo();
    let local_dir = tempdir().unwrap();

    let repo = clone_repository(&remote_path, local_dir.path().to_str().unwrap()).unwrap();

    assert!(local_dir.path().join(".git").exists());
    assert!(!repo.is_bare());
}

#[test]
fn test_list_remotes() {
    let (_remote_dir, remote_path) = setup_remote_repo();
    let local_dir = tempdir().unwrap();
    let repo = clone_repository(&remote_path, local_dir.path().to_str().unwrap()).unwrap();

    // After cloning, there should be an 'origin' remote
    let remotes = list_remotes(&repo).unwrap();
    assert_eq!(remotes.len(), 1);
    assert_eq!(remotes[0].name, "origin");

    // Add another remote
    let raw_repo = git2::Repository::open(local_dir.path()).unwrap();
    raw_repo.remote("upstream", "http://example.com/repo.git").unwrap();
    let remotes_after_add = list_remotes(&repo).unwrap();
    assert_eq!(remotes_after_add.len(), 2);
}

#[test]
fn test_list_remote_branches() {
    let (_remote_dir, remote_path) = setup_remote_repo();
    let local_dir = tempdir().unwrap();
    let repo = clone_repository(&remote_path, local_dir.path().to_str().unwrap()).unwrap();

    let branches = list_remote_branches(&repo, "origin").unwrap();
    assert!(branches.contains(&"refs/heads/main".to_string()));
}
