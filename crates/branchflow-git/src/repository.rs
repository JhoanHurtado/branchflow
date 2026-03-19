use crate::errors::GitError;
use git2::Repository;
use std::path::Path;

pub struct GitRepository {
    inner: Repository,
}

impl GitRepository {
    /// Abre un repositorio existente en la ruta dada.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, GitError> {
        let repo = Repository::open(path)?;
        Ok(Self { inner: repo })
    }

    /// Inicializa un nuevo repositorio.
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self, GitError> {
        let repo = Repository::init(path)?;
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

    /// Devuelve el ID (SHA) del commit al que apunta HEAD.
    /// Útil para tests y para obtener el punto de partida.
    pub fn get_head_id(&self) -> Result<String, GitError> {
        let head = self.inner.head()?;
        let target = head.target().ok_or_else(|| GitError::NotFound("HEAD has no target".into()))?;
        Ok(target.to_string())
    }

    /// Acceso interno para otros módulos de la misma crate
    pub(crate) fn get_inner(&self) -> &Repository {
        &self.inner
    }
}