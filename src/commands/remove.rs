use crate::error::Result;
use crate::git::GitWorktree;

pub struct RemoveCommand {
    pub worktree_name: String,
    pub force: bool,
}

impl RemoveCommand {
    pub fn new(worktree_name: String, force: bool) -> Self {
        Self {
            worktree_name,
            force,
        }
    }
    
    pub fn execute(&self) -> Result<()> {
        // 🟢 GREEN: removeコマンドの最小実装
        let git_worktree = GitWorktree::new()?;
        
        // worktreeを削除
        git_worktree.remove_worktree(&self.worktree_name, self.force)?;
        
        println!("✓ Successfully removed worktree '{}'", self.worktree_name);
        
        Ok(())
    }
}