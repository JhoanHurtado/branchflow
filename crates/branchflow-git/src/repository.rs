use crate::errors::GitError;
use git2::Repository;
use std::path::Path;

pub struct GitRepository {
    pub(crate) inner: Repository,
}

impl GitRepository {
    /// Abre un repositorio existente en la ruta dada.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, GitError> {
        let repo = Repository::open(path).map_err(GitError::from)?;
        Ok(Self { inner: repo })
    }

    /// Clona un repositorio remoto en la ruta dada.
    pub fn clone<P: AsRef<Path>>(url: &str, path: P) -> Result<Self, GitError> {
        let repo = Repository::clone(url, path).map_err(GitError::from)?;
        Ok(Self { inner: repo })
    }

    /// Inicializa un nuevo repositorio.
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self, GitError> {
        let repo = Repository::init(path).map_err(GitError::from)?;
        Ok(Self { inner: repo })
    }

    /// Devuelve true si el repositorio es "bare" (sin directorio de trabajo).
    pub fn is_bare(&self) -> bool {
        self.inner.is_bare()
    }

    /// Devuelve la ruta al directorio `.git`.
    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    /// Devuelve la ruta al directorio de trabajo (si existe).
    /// Retorna None si el repositorio es bare.
    pub fn workdir(&self) -> Option<&Path> {
        self.inner.workdir()
    }

    /// Devuelve la referencia HEAD del repositorio.
    pub fn head(&self) -> Result<git2::Reference<'_>, GitError> {
        self.inner.head().map_err(GitError::from)
    }

    /// Devuelve el commit actual apuntado por HEAD.
    pub fn head_commit(&self) -> Result<git2::Commit<'_>, GitError> {
        let head = self.inner.head().map_err(GitError::from)?;
        head.peel_to_commit().map_err(GitError::from)
    }
}