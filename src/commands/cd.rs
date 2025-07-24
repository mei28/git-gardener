use crate::error::{GitGardenerError, Result};
use crate::git::GitWorktree;
use crate::config::Config;

// 🟢 GREEN: cdコマンドの実装
pub struct CdCommand {
    pub worktree_name: String,
}

impl CdCommand {
    pub fn new(worktree_name: String) -> Self {
        Self { worktree_name }
    }
    
    // 🟢 GREEN: executeメソッドの最小実装
    pub fn execute(&self) -> Result<String> {
        // GitWorktreeを初期化してリポジトリの存在をチェック
        let git_worktree = GitWorktree::new()?;
        let repo_root = git_worktree.get_repository_root()?;
        
        // 設定ファイルを読み込む（現在は使用していないが将来的に拡張可能）
        let _config_path = Config::get_config_path(&repo_root);
        
        // worktreeの一覧を取得
        let worktrees = git_worktree.list_worktrees()?;
        
        // 指定されたworktreeを検索
        let target_worktree = worktrees.iter()
            .find(|w| w.name == self.worktree_name)
            .ok_or_else(|| GitGardenerError::Custom(
                format!("Worktree '{}' not found", self.worktree_name)
            ))?;
        
        // worktreeのパスを返す
        Ok(target_worktree.path.to_string_lossy().to_string())
    }
}