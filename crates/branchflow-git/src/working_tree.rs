

/*
Este módulo maneja el estado del working tree (árbol de trabajo).

El working tree representa los archivos del repositorio en disco y su relación
con el último commit (HEAD) y el área de staging (index).

Este archivo permite:

- Detectar archivos modificados
- Detectar archivos nuevos (untracked)
- Detectar archivos eliminados
- Identificar cambios en staging (index) vs no staged

Esto es fundamental para:
- Implementar `status` en CLI/UI
- Saber qué archivos serán incluidos en el próximo commit
- Validar operaciones antes de hacer commit

En resumen:
Este módulo traduce el estado del sistema de archivos y el index
a una estructura usable por la aplicación.
*/

use git2::{Repository, Status, StatusOptions};

use crate::errors::GitError;
use crate::repository::GitRepository;

/// Representa el estado de un archivo dentro del working tree
#[derive(Debug, Clone)]
pub struct FileStatus {
    pub path: String,
    pub is_new: bool,
    pub is_modified: bool,
    pub is_deleted: bool,
    pub is_staged: bool,
}

/// Obtiene el estado actual del working tree
pub fn status(repo: &GitRepository) -> Result<Vec<FileStatus>, GitError> {
    let repo: &Repository = &repo.inner;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true)
        .include_ignored(false)
        .recurse_untracked_dirs(true);

    let statuses = repo.statuses(Some(&mut opts))?;

    let mut result = Vec::new();

    for entry in statuses.iter() {
        let status = entry.status();

        let path = entry
            .path()
            .ok_or_else(|| GitError::InvalidRepositoryState("Invalid file path".into()))?
            .to_string();

        let file_status = FileStatus {
            path,
            is_new: status.contains(Status::WT_NEW) || status.contains(Status::INDEX_NEW),
            is_modified: status.contains(Status::WT_MODIFIED) || status.contains(Status::INDEX_MODIFIED),
            is_deleted: status.contains(Status::WT_DELETED) || status.contains(Status::INDEX_DELETED),
            is_staged: status.intersects(
                Status::INDEX_NEW | Status::INDEX_MODIFIED | Status::INDEX_DELETED,
            ),
        };

        result.push(file_status);
    }

    Ok(result)
}