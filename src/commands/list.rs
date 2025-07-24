use crate::error::Result;
use crate::git::GitWorktree;

pub struct ListCommand {
    pub all: bool,
    pub names_only: bool,
}

impl ListCommand {
    pub fn new(all: bool, names_only: bool) -> Self {
        Self { all, names_only }
    }
    
    pub fn execute(&self) -> Result<()> {
        let git_worktree = GitWorktree::new()?;
        let worktrees = git_worktree.list_worktrees()?;
        
        if worktrees.is_empty() {
            if !self.names_only {
                println!("No worktrees found.");
            }
            return Ok(());
        }
        
        if self.names_only {
            // Shell completion用にworktree名のみを出力
            for worktree in worktrees {
                if !self.all && worktree.is_prunable {
                    continue;
                }
                println!("{}", worktree.name);
            }
        } else {
            // 通常の表形式表示
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
        }
        
        Ok(())
    }
}