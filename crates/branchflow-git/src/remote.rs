/*
Este módulo maneja la interacción con repositorios remotos.

Un repositorio remoto representa una copia externa del repositorio local,
generalmente alojada en plataformas como GitHub.

Este archivo permite:

- Listar remotos configurados (origin, upstream, etc.)
- Obtener URLs de remotos
- Clonar repositorios
- Consultar referencias remotas (ramas remotas)

Esto es fundamental para:
- Integraciones con GitHub u otros servicios
- Operaciones como clone, fetch y pull (a futuro)
- Visualizar estado remoto en CLI/UI

En resumen:
Este módulo encapsula el acceso a remotos sin mezclarlo con la lógica local.
*/

use git2::{Repository, Direction};

use crate::errors::GitError;
use crate::repository::GitRepository;

/// Representa un remoto configurado
#[derive(Debug, Clone)]
pub struct RemoteInfo {
    pub name: String,
    pub url: String,
}

/// Lista todos los remotos configurados en el repositorio
pub fn list_remotes(repo: &GitRepository) -> Result<Vec<RemoteInfo>, GitError> {
    let remotes = repo.inner.remotes()?;

    let mut result = Vec::new();

    for name in remotes.iter().flatten() {
        let remote = repo.inner.find_remote(name)?;

        let url = remote
            .url()
            .ok_or_else(|| GitError::InvalidRepositoryState("Remote has no URL".into()))?;

        result.push(RemoteInfo {
            name: name.to_string(),
            url: url.to_string(),
        });
    }

    Ok(result)
}

/// Clona un repositorio remoto en una ruta local
pub fn clone_repository(url: &str, path: &str) -> Result<GitRepository, GitError> {
    let repo = Repository::clone(url, path)?;

    Ok(GitRepository { inner: repo })
}

/// Obtiene las referencias (ramas) de un remoto
pub fn list_remote_branches(
    repo: &GitRepository,
    remote_name: &str,
) -> Result<Vec<String>, GitError> {
    let mut remote = repo.inner.find_remote(remote_name)?;

    remote.connect(Direction::Fetch)?;

    let list = remote.list()?;

    let mut branches = Vec::new();

    for head in list {
        let name = head.name();
        branches.push(name.to_string());
    }

    remote.disconnect()?;

    Ok(branches)
}
