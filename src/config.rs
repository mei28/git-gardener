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
    
    #[test]
    fn test_config_load_fails_for_nonexistent_file() {
        // What: 存在しないファイルを読み込もうとした場合にエラーが返されるかテスト
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("nonexistent.yml");
        
        let result = Config::load_from_file(&config_path);
        
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GitGardenerError::ConfigNotFound { .. }
        ));
    }
    
    #[test]
    fn test_config_with_hooks_serialization() {
        // What: フック設定を含むconfigの直列化・逆直列化が正しく動作するかテスト
        use std::collections::HashMap;
        
        let mut env = HashMap::new();
        env.insert("NODE_ENV".to_string(), "development".to_string());
        
        let hook = Hook {
            hook_type: HookType::Command,
            from: None,
            to: None,
            command: Some("npm install".to_string()),
            env: Some(env),
        };
        
        let hooks = Hooks {
            post_create: Some(vec![hook]),
        };
        
        let config = Config {
            version: "1.0".to_string(),
            defaults: DefaultConfig {
                root_dir: Some(".gardener".to_string()),
            },
            hooks: Some(hooks),
        };
        
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("hooks_config.yml");
        
        config.save_to_file(&config_path).unwrap();
        let loaded_config = Config::load_from_file(&config_path).unwrap();
        
        assert_eq!(loaded_config.version, config.version);
        assert!(loaded_config.hooks.is_some());
        
        let loaded_hooks = loaded_config.hooks.unwrap();
        assert!(loaded_hooks.post_create.is_some());
        
        let hooks_vec = loaded_hooks.post_create.unwrap();
        assert_eq!(hooks_vec.len(), 1);
        assert_eq!(hooks_vec[0].hook_type, HookType::Command);
        assert_eq!(hooks_vec[0].command, Some("npm install".to_string()));
    }
    
    #[test]
    fn test_config_with_copy_hook_serialization() {
        // What: copyフックの直列化・逆直列化が正しく動作するかテスト
        let hook = Hook {
            hook_type: HookType::Copy,
            from: Some("README.md".to_string()),
            to: Some("README.md".to_string()),
            command: None,
            env: None,
        };
        
        let hooks = Hooks {
            post_create: Some(vec![hook]),
        };
        
        let config = Config {
            version: "1.0".to_string(),
            defaults: DefaultConfig::default(),
            hooks: Some(hooks),
        };
        
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("copy_config.yml");
        
        config.save_to_file(&config_path).unwrap();
        let loaded_config = Config::load_from_file(&config_path).unwrap();
        
        let loaded_hooks = loaded_config.hooks.unwrap();
        let hooks_vec = loaded_hooks.post_create.unwrap();
        
        assert_eq!(hooks_vec[0].hook_type, HookType::Copy);
        assert_eq!(hooks_vec[0].from, Some("README.md".to_string()));
        assert_eq!(hooks_vec[0].to, Some("README.md".to_string()));
        assert!(hooks_vec[0].command.is_none());
    }
    
    #[test]
    fn test_config_fails_with_invalid_yaml() {
        // What: 不正なYAMLファイルの読み込みでエラーが返されるかテスト
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("invalid.yml");
        
        // 不正なYAMLを書き込み
        std::fs::write(&config_path, "invalid: yaml: content: [").unwrap();
        
        let result = Config::load_from_file(&config_path);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GitGardenerError::Custom(_)));
    }
}