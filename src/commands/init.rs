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
        let config_path = Config::get_config_path(&repo_root);
        
        if config_path.exists() && !self.force {
            return Err(GitGardenerError::Custom(
                format!(
                    "Config file already exists at {}. Use --force to overwrite.",
                    config_path.display()
                )
            ));
        }
        
        // 🟢 GREEN: .gardenerフォルダを作成
        let gardener_dir = repo_root.join(".gardener");
        if !gardener_dir.exists() {
            fs::create_dir_all(&gardener_dir)?;
            println!("✓ Created .gardener directory at: {}", gardener_dir.display());
        }
        
        // 🟢 GREEN: .gitignoreに.gardener/を追加
        self.update_gitignore(&repo_root)?;
        
        let config_path = Config::create_default_config_file(&repo_root)?;
        
        println!("✓ Created config file at: {}", config_path.display());
        println!("  You can now customize your worktree settings in this file.");
        
        Ok(())
    }
    
    // 🟢 GREEN: .gitignoreを更新する最小実装
    fn update_gitignore(&self, repo_root: &Path) -> Result<()> {
        let gitignore_path = repo_root.join(".gitignore");
        
        let mut content = if gitignore_path.exists() {
            fs::read_to_string(&gitignore_path)?
        } else {
            String::new()
        };
        
        // .gardener/がまだ含まれていない場合のみ追加
        if !content.contains(".gardener/") {
            if !content.is_empty() && !content.ends_with('\n') {
                content.push('\n');
            }
            content.push_str(".gardener/\n");
            fs::write(&gitignore_path, content)?;
            println!("✓ Added .gardener/ to .gitignore");
        }
        
        Ok(())
    }
}