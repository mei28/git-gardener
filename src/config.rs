use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::error::{GitGardenerError, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_version")]
    pub version: String,
    
    #[serde(default)]
    pub defaults: DefaultConfig,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Hooks>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DefaultConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hooks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_create: Option<Vec<Hook>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hook {
    #[serde(rename = "type")]
    pub hook_type: HookType,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HookType {
    Copy,
    Command,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: default_version(),
            defaults: DefaultConfig::default(),
            hooks: None,
        }
    }
}

fn default_version() -> String {
    "1.0".to_string()
}

impl Config {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(GitGardenerError::ConfigNotFound {
                path: path.display().to_string(),
            });
        }
        
        let contents = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)
            .map_err(|e| GitGardenerError::Custom(format!("Failed to parse YAML config: {}", e)))?;
        Ok(config)
    }
    
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let contents = serde_yaml::to_string(self)
            .map_err(|e| GitGardenerError::Custom(format!("Failed to serialize config: {}", e)))?;
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(path, contents)?;
        Ok(())
    }
    
    pub fn get_config_path(repo_path: &Path) -> PathBuf {
        repo_path.join(".gardener.yml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.version, "1.0");
        assert!(config.defaults.root_dir.is_none());
        assert!(config.hooks.is_none());
    }
    
    #[test]
    fn test_save_and_load_config() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.yml");
        
        let config = Config::default();
        config.save_to_file(&config_path).unwrap();
        
        let loaded_config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(loaded_config.version, config.version);
    }
}