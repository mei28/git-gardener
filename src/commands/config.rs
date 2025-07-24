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
    
    fn set_config(&self, key: &str, value: &str) -> Result<()> {
        // 🟢 GREEN: 実際の設定変更実装
        let git_worktree = GitWorktree::new()?;
        let repo_root = git_worktree.get_repository_root()?;
        let config_path = Config::get_config_path(&repo_root);
        
        // 設定ファイルが存在しない場合は作成
        if !config_path.exists() {
            Config::create_default_config_file(&repo_root)?;
        }
        
        // 設定を読み込み
        let mut config = Config::load_from_file(&config_path)?;
        
        // キーに基づいて値を設定
        match key {
            "defaults.root_dir" => {
                config.defaults.root_dir = value.to_string();
            }
            "defaults.editor" => {
                config.defaults.editor = Some(value.to_string());
            }
            _ => {
                return Err(GitGardenerError::Custom(
                    format!("Unknown config key: {}", key)
                ));
            }
        }
        
        // 設定を保存
        config.save_to_file(&config_path)?;
        
        println!("Set {}: {}", key, value);
        Ok(())
    }
}