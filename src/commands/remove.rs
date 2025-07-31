use crate::error::Result;
use crate::git::GitWorktree;
use std::process::Command;

pub struct RemoveCommand {
    pub worktree: String,
    pub with_branch: bool,
}

impl RemoveCommand {
    pub fn new(worktree: String, with_branch: bool) -> Self {
        Self {
            worktree,
            with_branch,
        }
    }
    
    pub fn execute(&self) -> Result<()> {
        let git_worktree = GitWorktree::new()?;
        
        // worktreeの情報を取得
        let worktrees = git_worktree.list_worktrees()?;
        let worktree_info = worktrees
            .iter()
            .find(|w| w.name == self.worktree || w.branch == self.worktree)
            .ok_or_else(|| crate::error::GitGardenerError::WorktreeNotFound { 
                name: self.worktree.clone() 
            })?;
        
        let branch_name = worktree_info.branch.clone();
        
        // worktreeを削除
        git_worktree.remove_worktree(&worktree_info.name, false)?;
        
        println!("✓ Removed worktree '{}'", self.worktree);
        
        // --with-branchが指定されていればブランチも削除
        if self.with_branch {
            let output = Command::new("git")
                .args(&["branch", "-D", &branch_name])
                .output()?;
            
            if output.status.success() {
                println!("✓ Removed branch '{}'", branch_name);
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to remove branch '{}': {}", branch_name, error_msg);
            }
        }
        
        Ok(())
    }
}