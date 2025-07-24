use crate::error::{GitGardenerError, Result};
use crate::git::GitWorktree;
use std::path::PathBuf;

pub struct MoveCommand {
    pub worktree_name: String,
    pub new_path: PathBuf,
}

impl MoveCommand {
    pub fn new(worktree_name: String, new_path: PathBuf) -> Self {
        Self {
            worktree_name,
            new_path,
        }
    }
    
    pub fn execute(&self) -> Result<()> {
        // 🟢 GREEN: moveコマンドの最小実装
        let git_worktree = GitWorktree::new()?;
        
        // 現在のworktreeを確認
        let worktrees = git_worktree.list_worktrees()?;
        let current_worktree = worktrees
            .iter()
            .find(|w| w.name == self.worktree_name)
            .ok_or_else(|| GitGardenerError::WorktreeNotFound {
                name: self.worktree_name.clone(),
            })?;
        
        // git worktree moveを実行
        std::process::Command::new("git")
            .args(&["worktree", "move", &current_worktree.path.to_string_lossy(), &self.new_path.to_string_lossy()])
            .output()
            .map_err(|e| GitGardenerError::Custom(format!("Failed to execute git worktree move: {}", e)))?;
        
        println!("✓ Successfully moved worktree '{}' to {}", self.worktree_name, self.new_path.display());
        
        Ok(())
    }
}