use crate::errors::GitError;
use crate::repository::GitRepository;
use git2::BranchType;

pub struct BranchInfo {
    pub name: String,
    pub is_head: bool,
}

pub fn list_local_branches(repo: &GitRepository) -> Result<Vec<BranchInfo>, GitError> {
    let git_repo = &repo.inner;
    let branches = git_repo.branches(Some(BranchType::Local))?;
    
    let mut branch_list = Vec::new();

    for branch_result in branches {
        let (branch, branch_type) = branch_result?;

        // Obtener nombre válido del branch o fallar explícitamente
        let name = branch
            .name()?
            .ok_or_else(|| GitError::InvalidReference("Branch has no valid name".into()))?
            .to_string();

        let is_head = branch.is_head();

        // Actualmente solo trabajamos con branches locales (fase 1),
        // pero dejamos explícito el tipo para futura extensión
        match branch_type {
            BranchType::Local => {
                branch_list.push(BranchInfo { name, is_head });
            }
            BranchType::Remote => {
                // Ignorados por ahora (fuera de alcance de la fase 1)
            }
        }
    }

    Ok(branch_list)
}