use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitGardenerError {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),
    
    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    
    #[error("Config file not found at {path}")]
    ConfigNotFound { path: String },
    
    #[error("Invalid configuration: {message}")]
    InvalidConfig { message: String },
    
    #[error("Worktree '{name}' already exists")]
    WorktreeExists { name: String },
    
    #[error("Worktree '{name}' not found")]
    WorktreeNotFound { name: String },
    
    #[error("Not in a git repository")]
    NotInRepository,
    
    #[error("{0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, GitGardenerError>;