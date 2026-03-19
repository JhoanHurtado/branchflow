/*
Este módulo maneja el acceso a commits del repositorio.

Un commit representa un snapshot del estado del proyecto en un punto en el tiempo.

Este archivo permite:

- Obtener un commit específico por su hash (SHA)
- Extraer información relevante del commit (autor, mensaje, timestamp)

Esto es fundamental para:
- Implementar `log`
- Navegar el historial del repositorio
- Mostrar información de commits en CLI/UI

En resumen:
Este módulo traduce los objetos commit de Git (libgit2)
a una estructura simple y usable por la aplicación.
*/
use crate::errors::GitError;
use crate::repository::GitRepository;
use git2::Oid;
use git2::Sort;

#[derive(Debug, Clone)]
pub struct CommitData {
    pub id: String,
    pub author: String,
    pub message: String,
    pub timestamp: i64,
    pub parents: Vec<String>,
}

pub fn get_commit(repo: &GitRepository, hash: &str) -> Result<CommitData, GitError> {
    let oid = Oid::from_str(hash)?;
    let commit = repo.inner.find_commit(oid)?;

    // Extrae la firma del autor a una variable para extender su vida útil.
    // Esto evita el error de lifetime E0597.
    let author_signature = commit.author();
    let author_name = author_signature
        .name()
        .ok_or_else(|| GitError::InvalidRepositoryState("Commit author has no valid name".into()))?;

    let message = commit
        .message()
        .ok_or_else(|| GitError::InvalidRepositoryState("Commit has no valid message".into()))?;

    Ok(CommitData {
        id: commit.id().to_string(),
        author: author_name.to_string(),
        message: message.to_string(),
        timestamp: commit.time().seconds(),
        parents: commit.parent_ids().map(|p| p.to_string()).collect(),
    })
}

/// Recorre el historial de commits (equivalente a `git log`)
pub fn get_log(
    repo: &GitRepository,
    limit: usize,
) -> Result<Vec<CommitData>, GitError> {
    let repo_inner = &repo.inner;

    let mut revwalk = repo_inner.revwalk()?;

    // Empezar desde HEAD
    let head = repo_inner.head()?;
    let head_oid = head
        .target()
        .ok_or_else(|| GitError::InvalidRepositoryState("HEAD has no target".into()))?;

    revwalk.push(head_oid)?;

    // Orden típico de git log
    revwalk.set_sorting(Sort::TIME | Sort::TOPOLOGICAL)?;

    let mut commits = Vec::new();

    for oid_result in revwalk.take(limit) {
        let oid = oid_result?;
        let commit = repo_inner.find_commit(oid)?;

        let author = commit.author();
        let author_name = author
            .name()
            .ok_or_else(|| GitError::InvalidRepositoryState("Invalid author name".into()))?
            .to_string();

        let message = commit
            .message()
            .ok_or_else(|| GitError::InvalidRepositoryState("Invalid commit message".into()))?
            .to_string();

        let timestamp = commit.time().seconds();

        let parents = commit.parent_ids().map(|p| p.to_string()).collect();

        commits.push(CommitData {
            id: oid.to_string(),
            author: author_name,
            message,
            timestamp,
            parents,
        });
    }

    Ok(commits)
}