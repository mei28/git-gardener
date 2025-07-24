use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::error::{GitGardenerError, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub defaults: DefaultConfig,
    
    #[serde(default)]
    pub branches: HashMap<String, BranchConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DefaultConfig {
    #[serde(default = "default_root_dir")]
    pub root_dir: String,
    
    #[serde(default)]
    pub post_create: Vec<String>,
    
    #[serde(default)]
    pub editor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BranchConfig {
    #[serde(default)]
    pub post_create: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            defaults: DefaultConfig::default(),
            branches: HashMap::new(),
        }
    }
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            root_dir: default_root_dir(),
            post_create: Vec::new(),
            editor: None,
        }
    }
}

fn default_root_dir() -> String {
    ".gardener".to_string()
}

impl Config {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(GitGardenerError::ConfigNotFound {
                path: path.display().to_string(),
            });
        }
        
        let contents = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
    
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)?;
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(path, contents)?;
        Ok(())
    }
    
    pub fn get_config_path(repo_path: &Path) -> PathBuf {
        repo_path.join(".git").join("gardener.toml")
    }
    
    pub fn create_default_config_file(repo_path: &Path) -> Result<PathBuf> {
        let config_path = Self::get_config_path(repo_path);
        let default_config = Config::default();
        default_config.save_to_file(&config_path)?;
        Ok(config_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.defaults.root_dir, ".gardener");
        assert!(config.defaults.post_create.is_empty());
        assert!(config.defaults.editor.is_none());
        assert!(config.branches.is_empty());
    }
    
    #[test]
    fn test_save_and_load_config() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        
        let config = Config::default();
        config.save_to_file(&config_path).unwrap();
        
        let loaded_config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(loaded_config.defaults.root_dir, config.defaults.root_dir);
    }
}