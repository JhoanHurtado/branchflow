/*
Módulo: references

Responsabilidad:
Gestionar el estado de HEAD del repositorio.

HEAD es una referencia especial en Git que indica:
- La rama actual (si no está en detached)
- El commit actual (SHA)
- Si el repositorio está en estado detached HEAD

Este módulo expone funciones de lectura (query) que traducen
el estado interno de git2 a estructuras propias del dominio.

NO debe contener lógica de escritura.
*/

use crate::errors::GitError;
use crate::repository::GitRepository;

/// Representa la referencia HEAD del repositorio
#[derive(Debug, Clone)]
pub struct HeadReference {
    /// Nombre de la referencia (ej: "refs/heads/main")
    pub name: Option<String>,

    /// SHA del commit actual (si existe)
    pub target: Option<String>,

    /// Indica si el HEAD está desacoplado (detached)
    pub is_detached: bool,
}

impl HeadReference {
    /// Retorna el nombre corto de la rama (ej: "main")
    pub fn branch_name(&self) -> Option<String> {
        self.name
            .as_ref()
            .and_then(|n| n.strip_prefix("refs/heads/"))
            .map(|s| s.to_string())
    }

    /// Indica si HEAD está apuntando a una rama válida
    pub fn is_on_branch(&self) -> bool {
        !self.is_detached && self.name.is_some()
    }
}

/// Obtiene información completa del HEAD
pub fn get_head(repo: &GitRepository) -> Result<HeadReference, GitError> {
    let head = repo.inner.head()?;

    let is_detached = repo.inner.head_detached()?;

    let name = if is_detached {
        None
    } else {
        head.name().map(|s| s.to_string())
    };

    let target = head.target().map(|oid| oid.to_string());

    Ok(HeadReference {
        name,
        target,
        is_detached,
    })
}

/// Obtiene el SHA del commit al que apunta HEAD
pub fn get_head_commit_id(repo: &GitRepository) -> Result<String, GitError> {
    let head = get_head(repo)?;

    head.target.ok_or_else(|| {
        GitError::InvalidReference(
            "HEAD no apunta a un commit válido".to_string()
        )
    })
}