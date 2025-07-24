use crate::config::Config;
use crate::error::{GitGardenerError, Result};
use crate::git::GitWorktree;
use crate::hooks::HookExecutor;
use std::path::PathBuf;

pub struct AddCommand {
    pub branch: String,
    pub path: Option<PathBuf>,
    pub upstream: Option<String>,
    pub create_branch: bool,
}

impl AddCommand {
    pub fn new(
        branch: String,
        path: Option<PathBuf>,
        upstream: Option<String>,
        create_branch: bool,
    ) -> Self {
        Self {
            branch,
            path,
            upstream,
            create_branch,
        }
    }
    
    pub fn execute(&self) -> Result<()> {
        let git_worktree = GitWorktree::new()?;
        let repo_root = git_worktree.get_repository_root()?;
        
        // 設定ファイルを読み込む（存在しない場合はデフォルト設定を使用）
        let config_path = Config::get_config_path(&repo_root);
        let config = if config_path.exists() {
            Config::load_from_file(&config_path)?
        } else {
            Config::default()
        };
        
        // ブランチが既に存在するかチェック
        if !self.create_branch && !git_worktree.branch_exists(&self.branch)? {
            return Err(GitGardenerError::Custom(
                format!(
                    "Branch '{}' does not exist. Use -c flag to create a new branch.",
                    self.branch
                )
            ));
        }
        
        // worktreeのパスを決定
        let worktree_path = if let Some(ref path) = self.path {
            path.clone()
        } else {
            repo_root.parent()
                .unwrap_or(&repo_root)
                .join(&config.defaults.root_dir)
                .join(&self.branch.replace('/', "-"))
        };
        
        // worktreeの名前を決定（ブランチ名から/を-に置換）
        let worktree_name = self.branch.replace('/', "-");
        
        // 既存のworktreeをチェック
        let existing_worktrees = git_worktree.list_worktrees()?;
        if existing_worktrees.iter().any(|w| w.name == worktree_name) {
            return Err(GitGardenerError::WorktreeExists {
                name: worktree_name,
            });
        }
        
        // worktreeを作成
        println!("Creating worktree '{}' at '{}'...", worktree_name, worktree_path.display());
        
        // 親ディレクトリを作成
        if let Some(parent) = worktree_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        git_worktree.create_worktree(
            &worktree_name,
            &worktree_path,
            &self.branch,
            self.create_branch,
        )?;
        
        println!("✓ Successfully created worktree '{}' at '{}'", worktree_name, worktree_path.display());
        
        // 🟢 GREEN: post_createフックの実行（最小実装）
        let hook_executor = HookExecutor::new();
        hook_executor.execute_post_create(&worktree_path, &self.branch, &config.defaults.post_create)?;
        
        Ok(())
    }
}