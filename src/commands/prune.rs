use crate::error::Result;
use crate::git::GitWorktree;

pub struct PruneCommand {
    pub dry_run: bool,
}

impl PruneCommand {
    pub fn new(dry_run: bool) -> Self {
        Self { dry_run }
    }
    
    pub fn execute(&self) -> Result<()> {
        // 🟢 GREEN: pruneコマンドの最小実装
        let git_worktree = GitWorktree::new()?;
        
        let worktrees = git_worktree.list_worktrees()?;
        let prunable = worktrees.iter().filter(|w| w.is_prunable).collect::<Vec<_>>();
        
        if prunable.is_empty() {
            println!("No worktrees to prune");
            return Ok(());
        }
        
        if self.dry_run {
            println!("Would prune the following worktrees:");
            for worktree in &prunable {
                println!("  - {} ({})", worktree.name, worktree.path.display());
            }
        } else {
            for worktree in &prunable {
                println!("Pruning worktree '{}' at {}", worktree.name, worktree.path.display());
                git_worktree.remove_worktree(&worktree.name, true)?;
            }
            println!("✓ Successfully pruned {} worktree(s)", prunable.len());
        }
        
        Ok(())
    }
}