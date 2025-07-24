use crate::config::Config;
use crate::error::{GitGardenerError, Result};
use crate::git::GitWorktree;

pub enum ConfigSubcommand {
    View,
    Set { key: String, value: String },
}

pub struct ConfigCommand {
    pub subcommand: ConfigSubcommand,
}

impl ConfigCommand {
    pub fn new(subcommand: ConfigSubcommand) -> Self {
        Self { subcommand }
    }
    
    pub fn execute(&self) -> Result<()> {
        match &self.subcommand {
            ConfigSubcommand::View => self.view_config(),
            ConfigSubcommand::Set { key, value } => self.set_config(key, value),
        }
    }
    
    fn view_config(&self) -> Result<()> {
        let git_worktree = GitWorktree::new()?;
        let repo_root = git_worktree.get_repository_root()?;
        let config_path = Config::get_config_path(&repo_root);
        
        if !config_path.exists() {
            println!("No config file found. Run 'git-gardener init' to create one.");
            return Ok(());
        }
        
        let config = Config::load_from_file(&config_path)?;
        let config_toml = toml::to_string_pretty(&config)?;
        
        println!("Config file: {}", config_path.display());
        println!("{}", "-".repeat(50));
        println!("{}", config_toml);
        
        Ok(())
    }
    
    fn set_config(&self, _key: &str, _value: &str) -> Result<()> {
        // TODO: set機能の実装は後のフェーズで
        Err(GitGardenerError::Custom(
            "Config set feature is not implemented yet.".to_string()
        ))
    }
}