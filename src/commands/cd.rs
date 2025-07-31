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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::process::Command;

    fn setup_git_repo_with_worktree() -> tempfile::TempDir {
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
        
        // テスト用のworktreeを作成
        let worktree_path = repo_path.join("feature-test");
        Command::new("git")
            .args(&["worktree", "add", "-b", "feature-test", &worktree_path.to_string_lossy()])
            .current_dir(repo_path)
            .output()
            .unwrap();
        
        temp_dir
    }

    #[test]
    fn test_cd_command_new_creates_instance() {
        // What: CdCommand::newが正しくインスタンスを作成するかテスト
        let cmd = CdCommand::new("test-branch".to_string());
        assert_eq!(cmd.worktree, "test-branch");
    }

    #[test]
    fn test_cd_command_fails_without_git_repo() {
        // What: Gitリポジトリでない場所でCdCommandが失敗するかテスト
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = CdCommand::new("test".to_string());
        let result = cmd.execute();
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GitGardenerError::NotInRepository));
    }

    #[test]
    fn test_cd_command_returns_main_worktree_for_at_symbol() {
        // What: @記号でメインワークツリーのパスを返すかテスト
        let temp_dir = setup_git_repo_with_worktree();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = CdCommand::new("@".to_string());
        let result = cmd.execute();
        
        assert!(result.is_ok());
        let path = result.unwrap();
        // メインリポジトリのパスが返されることを確認
        assert!(path.contains(temp_dir.path().to_str().unwrap()));
    }

    #[test]
    fn test_cd_command_finds_worktree_by_branch_name() {
        // What: ブランチ名でワークツリーを見つけられるかテスト
        let temp_dir = setup_git_repo_with_worktree();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = CdCommand::new("feature-test".to_string());
        let result = cmd.execute();
        
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.contains("feature-test"));
    }

    #[test]
    fn test_cd_command_fails_for_nonexistent_worktree() {
        // What: 存在しないワークツリーに対して失敗するかテスト
        let temp_dir = setup_git_repo_with_worktree();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = CdCommand::new("nonexistent-worktree".to_string());
        let result = cmd.execute();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("not found"));
    }
}