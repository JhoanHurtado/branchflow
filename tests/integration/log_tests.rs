use branchflow_git::GitRepository;
use branchflow_git::commits::get_commit;
use std::path::PathBuf;

#[test]
fn test_read_commit_from_simple_repo() {
    // Apuntamos a tu carpeta de repositorios de prueba
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../tests/repositories/simple_repo");

    let repo = GitRepository::open(path).expect("No se pudo abrir el repo de prueba");
    
    // Aquí deberías poner un hash que sepas que existe en tu simple_repo
    // Por ahora, intentamos resolver HEAD
    let head_id = repo.get_head_id().expect("No se pudo obtener HEAD");
    let commit = get_commit(&repo, &head_id).unwrap();

    assert!(!commit.id.is_empty());
    println!("Commit leído: {} - {}", commit.id, commit.message);
}