use branchflow_git::GitRepository;
use branchflow_git::commits::get_commit;

#[test]
fn test_read_commit_from_simple_repo() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut path = std::path::PathBuf::from(manifest_dir);
    path.push("../../tests/repositories/simple_repo");

    // 1. Si el repo no es un repo de git, lo inicializamos para el test
    if !path.join(".git").exists() {
        std::fs::create_dir_all(&path).unwrap();
        let repo = git2::Repository::init(&path).expect("Falló init");
        
        // Creamos un commit real para poder leerlo
        let sig = repo.signature().unwrap();
        let tree_id = repo.index().unwrap().write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[]).unwrap();
    }

    // 2. Ahora intentamos abrirlo con TU abstracción
    let repo = GitRepository::open(&path).expect("No se pudo abrir el repo");
    
    // 3. Obtenemos el HEAD y verificamos
    let head_id = repo.get_head_id().expect("No se pudo obtener HEAD");
    let commit = get_commit(&repo, &head_id).unwrap();

    assert_eq!(commit.message.trim(), "Initial commit");
    println!("✅ Test exitoso! Commit: {} - {}", commit.id, commit.message);
}