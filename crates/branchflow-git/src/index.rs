/*
Este módulo maneja el acceso directo al index (staging area) de Git.

El index es la capa intermedia entre el working tree y los commits.
Representa los archivos que serán incluidos en el próximo commit.

Este archivo permite:

- Listar archivos en staging
- Detectar qué está staged actualmente
- Limpiar el staging (unstage)

Esto es fundamental para:
- Implementar `status` correctamente
- Mostrar qué archivos serán parte del próximo commit
- Separar cambios staged vs no staged

En resumen:
Este módulo expone el estado del index de forma controlada
sin mezclarlo con lógica de commits u operaciones.
*/

use crate::errors::GitError;
use crate::repository::GitRepository;
use git2::Repository;

/// Representa un archivo en el index (staging)
#[derive(Debug, Clone)]
pub struct IndexFile {
    pub path: String,
}

/// Lista todos los archivos actualmente en el index
pub fn list_staged(repo: &GitRepository) -> Result<Vec<IndexFile>, GitError> {
    let repo: &Repository = &repo.inner;
    let index = repo.index()?;

    let mut result = Vec::new();

    for entry in index.iter() {
        let path = std::str::from_utf8(&entry.path)
            .map_err(|_| GitError::InvalidRepositoryState("Invalid UTF-8 path in index".into()))?
            .to_string();

        result.push(IndexFile { path });
    }

    Ok(result)
}

/// Elimina un archivo del staging (unstage)
pub fn unstage_path(repo: &GitRepository, path: &str) -> Result<(), GitError> {
    let repo: &Repository = &repo.inner;
    let mut index = repo.index()?;

    index.remove_path(std::path::Path::new(path))?;
    index.write()?;

    Ok(())
}

/// Limpia completamente el index (unstage all)
pub fn clear(repo: &GitRepository) -> Result<(), GitError> {
    let repo: &Repository = &repo.inner;
    let mut index = repo.index()?;

    index.clear()?;
    index.write()?;

    Ok(())
}
