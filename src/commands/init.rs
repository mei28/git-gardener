use crate::config::Config;
use crate::error::{GitGardenerError, Result};
use crate::git::GitWorktree;
use std::fs;
use std::path::Path;

pub struct InitCommand {
    pub force: bool,
}

impl InitCommand {
    pub fn new(force: bool) -> Self {
        Self { force }
    }
    
    pub fn execute(&self) -> Result<()> {
        let git_worktree = GitWorktree::new()?;
        let repo_root = git_worktree.get_repository_root()?;
        
        let config_path = repo_root.join(".gardener.yml");
        let gardener_dir = repo_root.join(".gardener");
        let gitignore_path = repo_root.join(".gitignore");
        
        // 既に初期化されているかチェック
        if config_path.exists() && !self.force {
            return Err(GitGardenerError::Custom(
                "git-gardener is already initialized. Use --force to reinitialize.".to_string()
            ));
        }
        
        // .gardenerディレクトリを作成
        if !gardener_dir.exists() {
            fs::create_dir_all(&gardener_dir)?;
            println!("✓ Created .gardener directory");
        }
        
        // .gitignoreに.gardenerを追加
        self.update_gitignore(&gitignore_path)?;
        
        // .gardener.ymlファイルを作成
        let config = Config::default();
        config.save_to_file(&config_path)?;
        println!("✓ Created .gardener.yml configuration file");
        
        println!("git-gardener initialized successfully!");
        Ok(())
    }
    
    fn update_gitignore(&self, gitignore_path: &Path) -> Result<()> {
        let gardener_entry = ".gardener/";
        
        // .gitignoreファイルが存在するかチェック
        if gitignore_path.exists() {
            let content = fs::read_to_string(gitignore_path)?;
            
            // 既に.gardenerエントリがあるかチェック
            if content.lines().any(|line| line.trim() == gardener_entry) {
                return Ok(());
            }
            
            // .gardenerエントリを追加
            let new_content = if content.ends_with('\n') {
                format!("{}{}\n", content, gardener_entry)
            } else {
                format!("{}\n{}\n", content, gardener_entry)
            };
            
            fs::write(gitignore_path, new_content)?;
        } else {
            // .gitignoreファイルを新規作成
            fs::write(gitignore_path, format!("{}\n", gardener_entry))?;
        }
        
        println!("✓ Updated .gitignore");
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
    fn test_init_command_new_creates_instance() {
        // What: InitCommand::newが正しくインスタンスを作成するかテスト
        let cmd = InitCommand::new(false);
        assert_eq!(cmd.force, false);
        
        let cmd = InitCommand::new(true);
        assert_eq!(cmd.force, true);
    }

    #[test]
    fn test_init_command_fails_without_git_repo() {
        // What: Gitリポジトリでない場所でInitCommandが失敗するかテスト
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = InitCommand::new(false);
        let result = cmd.execute();
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GitGardenerError::NotInRepository));
    }

    #[test]
    fn test_init_command_creates_gardener_directory() {
        // What: initコマンドが.gardenerディレクトリを作成するかテスト
        let temp_dir = setup_git_repo();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = InitCommand::new(false);
        let result = cmd.execute();
        
        assert!(result.is_ok());
        
        // .gardenerディレクトリが作成されたことを確認
        let gardener_dir = temp_dir.path().join(".gardener");
        assert!(gardener_dir.exists());
        assert!(gardener_dir.is_dir());
    }

    #[test]
    fn test_init_command_creates_config_file() {
        // What: initコマンドが.gardener.ymlファイルを作成するかテスト
        let temp_dir = setup_git_repo();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = InitCommand::new(false);
        let result = cmd.execute();
        
        assert!(result.is_ok());
        
        // .gardener.ymlファイルが作成されたことを確認
        let config_path = temp_dir.path().join(".gardener.yml");
        assert!(config_path.exists());
        
        // ファイルが有効なYAMLであることを確認
        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.version, "1.0");
    }

    #[test]
    fn test_init_command_updates_gitignore() {
        // What: initコマンドが.gitignoreを正しく更新するかテスト
        let temp_dir = setup_git_repo();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let cmd = InitCommand::new(false);
        let result = cmd.execute();
        
        assert!(result.is_ok());
        
        // .gitignoreファイルが作成/更新されたことを確認
        let gitignore_path = temp_dir.path().join(".gitignore");
        assert!(gitignore_path.exists());
        
        let content = fs::read_to_string(gitignore_path).unwrap();
        assert!(content.contains(".gardener/"));
    }

    #[test]
    fn test_init_command_updates_existing_gitignore() {
        // What: 既存の.gitignoreファイルを正しく更新するかテスト
        let temp_dir = setup_git_repo();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        // 既存の.gitignoreファイルを作成
        let gitignore_path = temp_dir.path().join(".gitignore");
        fs::write(&gitignore_path, "*.log\n/target/\n").unwrap();
        
        let cmd = InitCommand::new(false);
        let result = cmd.execute();
        
        assert!(result.is_ok());
        
        let content = fs::read_to_string(gitignore_path).unwrap();
        assert!(content.contains("*.log"));
        assert!(content.contains("/target/"));
        assert!(content.contains(".gardener/"));
    }

    #[test]
    fn test_init_command_fails_when_already_initialized() {
        // What: 既に初期化されている場合にエラーが返されるかテスト
        let temp_dir = setup_git_repo();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        // 最初の初期化
        let cmd = InitCommand::new(false);
        let result = cmd.execute();
        assert!(result.is_ok());
        
        // 2回目の初期化（forceフラグなし）
        let cmd = InitCommand::new(false);
        let result = cmd.execute();
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GitGardenerError::Custom(_)));
    }

    #[test]
    fn test_init_command_force_reinitializes() {
        // What: --forceフラグで再初期化できるかテスト
        let temp_dir = setup_git_repo();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        // 最初の初期化
        let cmd = InitCommand::new(false);
        let result = cmd.execute();
        assert!(result.is_ok());
        
        // 2回目の初期化（forceフラグあり）
        let cmd = InitCommand::new(true);
        let result = cmd.execute();
        
        assert!(result.is_ok());
    }
}