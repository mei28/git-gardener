use crate::config::Config;
use crate::error::{GitGardenerError, Result};
use crate::git::GitWorktree;
use crate::hooks::HookExecutor;

pub struct AddCommand {
    pub branch: String,
    pub new_branch: bool,
    pub commit: Option<String>,
}

impl AddCommand {
    pub fn new(
        branch: String,
        new_branch: bool,
        commit: Option<String>,
    ) -> Self {
        Self {
            branch,
            new_branch,
            commit,
        }
    }
    
    pub fn execute(&self) -> Result<()> {
        let git_worktree = GitWorktree::new()?;
        let repo_root = git_worktree.get_repository_root()?;
        
        // 設定ファイルを読み込む（存在しない場合はデフォルト設定を使用）
        let config_path = repo_root.join(".gardener.yml");
        let config = if config_path.exists() {
            Config::load_from_file(&config_path)?
        } else {
            Config::default()
        };
        
        // ブランチが既に存在するかチェック
        if !self.new_branch && !git_worktree.branch_exists(&self.branch)? {
            return Err(GitGardenerError::Custom(
                format!(
                    "Branch '{}' does not exist. Use -b flag to create a new branch.",
                    self.branch
                )
            ));
        }
        
        // worktreeのパスを決定（wtpスタイル）
        let base_dir = config.defaults.root_dir.unwrap_or_else(|| ".gardener".to_string());
        let worktree_path = repo_root
            .join(&base_dir)
            .join(&self.branch);
        
        // worktreeの名前を決定（パスのベース名）
        let worktree_name = self.branch.clone();
        
        // 既存のworktreeをチェック
        let existing_worktrees = git_worktree.list_worktrees()?;
        if existing_worktrees.iter().any(|w| w.name == worktree_name || w.path == worktree_path) {
            return Err(GitGardenerError::WorktreeExists {
                name: worktree_name,
            });
        }
        
        // worktreeを作成
        println!("Creating worktree for branch '{}'...", self.branch);
        
        // 親ディレクトリを作成
        if let Some(parent) = worktree_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        git_worktree.create_worktree_with_commit(
            &worktree_name,
            &worktree_path,
            &self.branch,
            self.new_branch,
            self.commit.as_deref(),
        )?;
        
        println!("✓ Created worktree at {}", worktree_path.display());
        
        // post_createフックの実行
        if let Some(ref hooks) = config.hooks {
            if let Some(ref post_create) = hooks.post_create {
                let hook_executor = HookExecutor::new();
                hook_executor.execute_hooks(&worktree_path, &self.branch, post_create)?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::process::Command;

    fn setup_git_repo() -> tempfile::TempDir {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path();
        
        // Git リポジトリを初期化
        Command::new("git")
            .args(&["init"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to init git repo");
        
        // 設定
        Command::new("git")
            .args(&["config", "user.name", "Test User"])
            .current_dir(repo_path)
            .output()
            .unwrap();
        
        Command::new("git")
            .args(&["config", "user.email", "test@example.com"])
            .current_dir(repo_path)
            .output()
            .unwrap();
        
        // 初期ファイルとコミットを作成
        fs::write(repo_path.join("README.md"), "# Test Repo").unwrap();
        
        Command::new("git")
            .args(&["add", "."])
            .current_dir(repo_path)
            .output()
            .unwrap();
        
        Command::new("git")
            .args(&["commit", "-m", "Initial commit"])
            .current_dir(repo_path)
            .output()
            .unwrap();
        
        temp_dir
    }

    #[test]
    fn test_add_command_new_creates_instance() {
        // What: AddCommand::newが正しくインスタンスを作成するかテスト
        let cmd = AddCommand::new("test-branch".to_string(), true, None);
        
        assert_eq!(cmd.branch, "test-branch");
        assert_eq!(cmd.new_branch, true);
        assert_eq!(cmd.commit, None);
    }

    #[test]
    fn test_add_command_new_with_commit() {
        // What: AddCommand::newがcommitオプション付きでインスタンスを作成するかテスト
        let cmd = AddCommand::new(
            "feature-branch".to_string(), 
            false, 
            Some("abc123".to_string())
        );
        
        assert_eq!(cmd.branch, "feature-branch");
        assert_eq!(cmd.new_branch, false);
        assert_eq!(cmd.commit, Some("abc123".to_string()));
    }

    #[test]
    fn test_add_command_fails_without_git_repo() {
        // What: Gitリポジトリでない場所でAddCommandが失敗するかテスト
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = AddCommand::new("test".to_string(), true, None);
        let result = cmd.execute();
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GitGardenerError::NotInRepository));
    }

    #[test]
    fn test_add_command_fails_for_nonexistent_branch() {
        // What: 存在しないブランチに対してnew_branch=falseの場合に失敗するかテスト
        let temp_dir = setup_git_repo();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = AddCommand::new("nonexistent-branch".to_string(), false, None);
        let result = cmd.execute();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("does not exist"));
        assert!(error_msg.contains("Use -b flag"));
    }
}