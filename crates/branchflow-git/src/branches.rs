use crate::errors::GitError;
use crate::repository::GitRepository;
use git2::BranchType;

pub struct BranchInfo {
    pub name: String,
    pub is_head: bool,
}

pub fn list_local_branches(repo: &GitRepository) -> Result<Vec<BranchInfo>, GitError> {
    let git_repo = repo.get_inner();
    let branches = git_repo.branches(Some(BranchType::Local))?;
    
    let mut branch_list = Vec::new();

    for branch_result in branches {
        let (branch, _) = branch_result?;
        let name = branch.name()?.unwrap_or("unknown").to_string();
        let is_head = branch.is_head();
        
        branch_list.push(BranchInfo { name, is_head });
    }

    Ok(branch_list)
}