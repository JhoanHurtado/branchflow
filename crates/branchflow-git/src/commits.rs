use crate::errors::GitError;
use crate::repository::GitRepository;
use git2::Oid;

pub struct CommitData {
    pub id: String,
    pub author: String,
    pub message: String,
    pub timestamp: i64,
}

pub fn get_commit(repo: &GitRepository, hash: &str) -> Result<CommitData, GitError> {
    let oid = Oid::from_str(hash)?;
    let commit = repo.get_inner().find_commit(oid)?;

    // Extrae la firma del autor a una variable para extender su vida útil.
    // Esto evita el error de lifetime E0597.
    let author_signature = commit.author();
    let author_name = author_signature.name().unwrap_or("Unknown");
    let message = commit.message().unwrap_or("");

    Ok(CommitData {
        id: commit.id().to_string(),
        author: author_name.to_string(),
        message: message.to_string(),
        timestamp: commit.time().seconds(),
    })
}