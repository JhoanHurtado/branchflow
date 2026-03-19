


/*
Este módulo maneja las referencias del repositorio, específicamente HEAD.

HEAD es el punto central de Git: indica en qué commit y en qué rama
se encuentra actualmente el repositorio.

Este archivo permite:

- Saber en qué rama está el repositorio (refs/heads/*)
- Detectar si está en estado detached HEAD
- Obtener el commit actual (SHA)
- Manejar estados especiales o inconsistentes

Esto es fundamental para:
- Mostrar información en CLI (status, branch actual)
- Navegar el historial (log)
- Validar operaciones (ej: evitar commits en detached HEAD)

En resumen:
Este módulo traduce el estado interno de Git (HEAD)
a una forma usable para el resto del sistema.
*/

use crate::errors::GitError;
use crate::repository::GitRepository;

/// Representa la referencia HEAD del repositorio
#[derive(Debug, Clone)]
pub struct HeadReference {
    pub name: Option<String>,   // Ej: "refs/heads/main"
    pub target: Option<String>, // SHA del commit si está resuelto
    pub is_detached: bool,
}

/// Obtiene información completa del HEAD
pub fn get_head(repo: &GitRepository) -> Result<HeadReference, GitError> {
    let head = repo.inner.head()?;

    let is_detached = head.is_detached();

    let name = head.name().map(|s| s.to_string());

    let target = match head.target() {
        Some(oid) => Some(oid.to_string()),
        None => None,
    };

    Ok(HeadReference {
        name,
        target,
        is_detached,
    })
}

/// Obtiene el commit al que apunta HEAD (si existe)
pub fn get_head_commit_id(repo: &GitRepository) -> Result<String, GitError> {
    let head = repo.inner.head()?;

    let oid = head
        .target()
        .ok_or_else(|| GitError::InvalidReference("HEAD does not point to a commit".into()))?;

    Ok(oid.to_string())
}