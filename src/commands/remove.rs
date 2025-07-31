use crate::error::Result;
use crate::git::GitWorktree;
use std::process::Command;

pub struct RemoveCommand {
    pub worktree: String,
    pub with_branch: bool,
}

impl RemoveCommand {
    pub fn new(worktree: String, with_branch: bool) -> Self {
        Self {
            worktree,
            with_branch,
        }
    }
    
    pub fn execute(&self) -> Result<()> {
        let git_worktree = GitWorktree::new()?;
        
        // worktreeの情報を取得
        let worktrees = git_worktree.list_worktrees()?;
        let worktree_info = worktrees
            .iter()
            .find(|w| w.name == self.worktree || w.branch == self.worktree)
            .ok_or_else(|| crate::error::GitGardenerError::WorktreeNotFound { 
                name: self.worktree.clone() 
            })?;
        
        let branch_name = worktree_info.branch.clone();
        
        // worktreeを削除
        git_worktree.remove_worktree(&worktree_info.name, false)?;
        
        println!("✓ Removed worktree '{}'", self.worktree);
        
        // --with-branchが指定されていればブランチも削除
        if self.with_branch {
            let output = Command::new("git")
                .args(&["branch", "-D", &branch_name])
                .output()?;
            
            if output.status.success() {
                println!("✓ Removed branch '{}'", branch_name);
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to remove branch '{}': {}", branch_name, error_msg);
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
    fn test_remove_command_new_creates_instance() {
        // What: RemoveCommand::newが正しくインスタンスを作成するかテスト
        let cmd = RemoveCommand::new("test-branch".to_string(), false);
        assert_eq!(cmd.worktree, "test-branch");
        assert_eq!(cmd.with_branch, false);
        
        let cmd = RemoveCommand::new("test-branch".to_string(), true);
        assert_eq!(cmd.worktree, "test-branch");
        assert_eq!(cmd.with_branch, true);
    }

    #[test]
    fn test_remove_command_fails_without_git_repo() {
        // What: Gitリポジトリでない場所でRemoveCommandが失敗するかテスト
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = RemoveCommand::new("test".to_string(), false);
        let result = cmd.execute();
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), crate::error::GitGardenerError::NotInRepository));
    }

    #[test]
    fn test_remove_command_fails_for_nonexistent_worktree() {
        // What: 存在しないワークツリーに対して失敗するかテスト
        let temp_dir = setup_git_repo_with_worktree();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = RemoveCommand::new("nonexistent-worktree".to_string(), false);
        let result = cmd.execute();
        
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(), 
            crate::error::GitGardenerError::WorktreeNotFound { .. }
        ));
    }

    #[test]
    fn test_remove_command_removes_worktree_by_branch_name() {
        // What: ブランチ名でワークツリーを削除できるかテスト
        let temp_dir = setup_git_repo_with_worktree();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        // worktreeが存在することを確認
        let git_worktree = GitWorktree::new().unwrap();
        let worktrees_before = git_worktree.list_worktrees().unwrap();
        assert!(worktrees_before.iter().any(|w| w.branch == "feature-test"));
        
        let cmd = RemoveCommand::new("feature-test".to_string(), false);
        let result = cmd.execute();
        
        assert!(result.is_ok());
        
        // worktreeが削除されたことを確認
        let worktrees_after = git_worktree.list_worktrees().unwrap();
        assert!(!worktrees_after.iter().any(|w| w.branch == "feature-test"));
    }

    #[test]
    fn test_remove_command_with_branch_flag() {
        // What: --with-branchフラグでブランチも削除されるかテスト
        let temp_dir = setup_git_repo_with_worktree();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        // ブランチが存在することを確認
        let output = Command::new("git")
            .args(&["branch", "--list", "feature-test"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stdout).contains("feature-test"));
        
        let cmd = RemoveCommand::new("feature-test".to_string(), true);
        let result = cmd.execute();
        
        assert!(result.is_ok());
        
        // ブランチが削除されたことを確認
        let output = Command::new("git")
            .args(&["branch", "--list", "feature-test"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();
        assert!(!String::from_utf8_lossy(&output.stdout).contains("feature-test"));
    }
}