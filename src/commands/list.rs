use crate::error::Result;
use crate::git::GitWorktree;

pub struct ListCommand {
    pub names_only: bool,
}

impl ListCommand {
    pub fn new(names_only: bool) -> Self {
        Self { names_only }
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
                println!("{}", worktree.branch);
            }
        } else {
            // 通常の表形式表示
            println!("{:<30} {:<50}", "BRANCH", "PATH");
            println!("{}", "-".repeat(80));
            
            for worktree in worktrees {
                println!(
                    "{:<30} {:<50}",
                    worktree.branch,
                    worktree.path.display()
                );
            }
        }
        
        Ok(())
    }
}