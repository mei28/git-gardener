use crate::config::Config;
use crate::error::{GitGardenerError, Result};
use crate::git::GitWorktree;

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
        
        let config_path = Config::create_default_config_file(&repo_root)?;
        
        println!("✓ Created config file at: {}", config_path.display());
        println!("  You can now customize your worktree settings in this file.");
        
        Ok(())
    }
}