use git_gardener::commands::cd::CdCommand;
use git_gardener::error::GitGardenerError;
use git_gardener::git::GitWorktree;
use tempfile::TempDir;
use std::process::Command;
use std::path::PathBuf;

// 🔴 RED: cdコマンドのテスト（まだ実装されていない）

#[cfg(test)]
mod cd_tests {
    use super::*;

    fn setup_test_repo_with_worktree(temp_dir: &TempDir) -> String {
        // Gitリポジトリの初期化
        Command::new("git")
            .args(&["init"])
            .current_dir(&temp_dir)
            .output()
            .expect("Failed to initialize git repo");
        
        Command::new("git")
            .args(&["config", "user.email", "test@example.com"])
            .current_dir(&temp_dir)
            .output()
            .expect("Failed to set git config");
        
        Command::new("git")
            .args(&["config", "user.name", "Test User"])
            .current_dir(&temp_dir)
            .output()
            .expect("Failed to set git config");
        
        // 初期コミットを作成
        std::fs::write(temp_dir.path().join("README.md"), "# Test Repo").unwrap();
        Command::new("git")
            .args(&["add", "README.md"])
            .current_dir(&temp_dir)
            .output()
            .expect("Failed to add file");
        
        Command::new("git")
            .args(&["commit", "-m", "Initial commit"])
            .current_dir(&temp_dir)
            .output()
            .expect("Failed to create initial commit");
        
        // テスト用ブランチとworktreeを作成
        let branch_name = "feature/test";
        let worktree_name = branch_name.replace('/', "-");
        
        Command::new("git")
            .args(&["checkout", "-b", branch_name])
            .current_dir(&temp_dir)
            .output()
            .expect("Failed to create branch");
        
        Command::new("git")
            .args(&["checkout", "main"])
            .current_dir(&temp_dir)
            .output()
            .ok(); // mainブランチがない場合もある
        
        // .gardenerディレクトリとworktreeを作成
        let gardener_dir = temp_dir.path().join(".gardener");
        std::fs::create_dir_all(&gardener_dir).unwrap();
        
        let worktree_path = gardener_dir.join(&worktree_name);
        
        Command::new("git")
            .args(&["worktree", "add", worktree_path.to_str().unwrap(), branch_name])
            .current_dir(&temp_dir)
            .output()
            .expect("Failed to create worktree");
        
        worktree_name
    }

    #[test]
    fn test_cd_existing_worktree() {
        // 🔴 RED: 存在するworktreeへのcd（まだ実装されていない）
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();
        
        let worktree_name = setup_test_repo_with_worktree(&temp_dir);
        
        let cd_command = CdCommand::new(worktree_name.clone());
        let result = cd_command.execute();
        
        assert!(result.is_ok());
        let path = result.unwrap();
        
        // 期待されるパス（macOSの/privateプレフィックスを考慮）
        let expected_path = temp_dir.path().join(".gardener").join(&worktree_name);
        let actual_path = PathBuf::from(path);
        
        // パスの正規化を行って比較（macOSの/privateプレフィックス対応）
        let normalized_actual = actual_path.canonicalize().unwrap_or(actual_path);
        let normalized_expected = expected_path.canonicalize().unwrap_or(expected_path);
        
        assert_eq!(normalized_actual, normalized_expected);
    }

    #[test]
    fn test_cd_nonexistent_worktree() {
        // 🔴 RED: 存在しないworktreeへのcd（エラーが返されることをテスト）
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();
        
        setup_test_repo_with_worktree(&temp_dir);
        
        let cd_command = CdCommand::new("nonexistent-worktree".to_string());
        let result = cd_command.execute();
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Worktree 'nonexistent-worktree' not found"));
    }

    #[test]
    fn test_cd_without_git_repo() {
        // 🔴 RED: gitリポジトリ外でのcd（エラーが返されることをテスト）
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();
        
        let cd_command = CdCommand::new("test-worktree".to_string());
        let result = cd_command.execute();
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error, GitGardenerError::NotInRepository));
    }
}