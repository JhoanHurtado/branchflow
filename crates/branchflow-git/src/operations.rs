/*
Este módulo maneja operaciones que MUTAN el repositorio.

A diferencia de otros módulos (commits, branches, references),
aquí se ejecutan acciones que cambian el estado del repositorio:

- Crear commits
- Agregar archivos al staging (index)
- Escribir el árbol (tree)
- Actualizar HEAD

Este módulo orquesta varias piezas:
- working tree
- index
- references (HEAD)
- commits

En resumen:
Este módulo implementa las operaciones de escritura del repositorio
manteniendo la lógica separada de los módulos de lectura.
*/

use git2::{IndexAddOption, Oid, Signature};

use crate::errors::GitError;
use crate::repository::GitRepository;

/// Agrega todos los archivos al staging area (equivalente a `git add .`)
pub fn stage_all(repo: &GitRepository) -> Result<(), GitError> {
    let mut index = repo.inner.index()?;

    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    Ok(())
}

/// Agrega un archivo específico al staging
pub fn stage_path(repo: &GitRepository, path: &str) -> Result<(), GitError> {
    let mut index = repo.inner.index()?;

    index.add_path(std::path::Path::new(path))?;
    index.write()?;

    Ok(())
}

/// Crea un commit usando el estado actual del index
pub fn create_commit(
    repo: &GitRepository,
    message: &str,
    author_name: &str,
    author_email: &str,
) -> Result<String, GitError> {
    let repo_inner = &repo.inner;

    // 1. Obtener index y escribir tree
    let mut index = repo_inner.index()?;
    let tree_oid = index.write_tree()?;
    let tree = repo_inner.find_tree(tree_oid)?;

    // 2. Obtener HEAD actual (si existe)
    let parent_commit = match repo_inner.head() {
        Ok(head) => {
            if let Some(oid) = head.target() {
                Some(repo_inner.find_commit(oid)?)
            } else {
                None
            }
        }
        Err(_) => None, // repo sin commits iniciales
    };

    // 3. Firma del autor/committer
    let signature = Signature::now(author_name, author_email)?;

    // 4. Crear commit
    let commit_oid: Oid = match parent_commit {
        Some(ref parent) => repo_inner.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[parent],
        )?,
        None => repo_inner.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[],
        )?,
    };

    Ok(commit_oid.to_string())
}
