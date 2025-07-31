use crate::error::{GitGardenerError, Result};
use crate::git::GitWorktree;

pub struct CdCommand {
    pub worktree: String,
}

impl CdCommand {
    pub fn new(worktree: String) -> Self {
        Self { worktree }
    }
    
    pub fn execute(&self) -> Result<String> {
        let git_worktree = GitWorktree::new()?;
        let repo_root = git_worktree.get_repository_root()?;
        
        // @でメインワークツリーに移動
        if self.worktree == "@" {
            return Ok(repo_root.to_string_lossy().to_string());
        }
        
        // worktreeの一覧を取得
        let worktrees = git_worktree.list_worktrees()?;
        
        // 指定されたworktreeを検索（ブランチ名またはworktree名で検索）
        let target_worktree = worktrees.iter()
            .find(|w| w.name == self.worktree || w.branch == self.worktree)
            .ok_or_else(|| GitGardenerError::Custom(
                format!("Worktree '{}' not found", self.worktree)
            ))?;
        
        // worktreeのパスを返す
        Ok(target_worktree.path.to_string_lossy().to_string())
    }
}