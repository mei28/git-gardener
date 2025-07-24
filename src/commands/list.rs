use crate::error::Result;
use crate::git::GitWorktree;

pub struct ListCommand {
    pub all: bool,
}

impl ListCommand {
    pub fn new(all: bool) -> Self {
        Self { all }
    }
    
    pub fn execute(&self) -> Result<()> {
        let git_worktree = GitWorktree::new()?;
        let worktrees = git_worktree.list_worktrees()?;
        
        if worktrees.is_empty() {
            println!("No worktrees found.");
            return Ok(());
        }
        
        println!("{:<20} {:<40} {:<15}", "NAME", "PATH", "BRANCH");
        println!("{}", "-".repeat(75));
        
        for worktree in worktrees {
            // prune済みのworktreeを表示するかどうか
            if !self.all && worktree.is_prunable {
                continue;
            }
            
            let status = if worktree.is_prunable {
                " (prunable)"
            } else {
                ""
            };
            
            println!(
                "{:<20} {:<40} {:<15}{}",
                worktree.name,
                worktree.path.display(),
                worktree.branch,
                status
            );
        }
        
        Ok(())
    }
}