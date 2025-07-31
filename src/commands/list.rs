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
    fn test_list_command_new_creates_instance() {
        // What: ListCommand::newが正しくインスタンスを作成するかテスト
        let cmd = ListCommand::new(true);
        assert_eq!(cmd.names_only, true);
        
        let cmd = ListCommand::new(false);
        assert_eq!(cmd.names_only, false);
    }

    #[test]
    fn test_list_command_fails_without_git_repo() {
        // What: Gitリポジトリでない場所でListCommandが失敗するかテスト
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = ListCommand::new(false);
        let result = cmd.execute();
        
        assert!(result.is_err());
    }

    #[test]  
    fn test_list_command_empty_repo_shows_no_worktrees() {
        // What: worktreeがない場合に適切なメッセージを表示するかテスト
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path();
        
        // Git リポジトリを初期化（worktreeなし）
        Command::new("git")
            .args(&["init"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to init git repo");
        
        std::env::set_current_dir(repo_path).unwrap();
        
        let cmd = ListCommand::new(false);
        let result = cmd.execute();
        
        // worktreeが見つからない場合は成功するが出力は空
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_command_names_only_flag() {
        // What: names_onlyフラグの動作をテスト
        let temp_dir = setup_git_repo_with_worktree();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        // names_only = true の場合
        let cmd = ListCommand::new(true);
        let result = cmd.execute();
        assert!(result.is_ok());
        
        // names_only = false の場合
        let cmd = ListCommand::new(false);
        let result = cmd.execute();
        assert!(result.is_ok());
    }
}