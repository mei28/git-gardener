use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "git-gardener",
    version,
    author,
    about = "A Git worktree management tool",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(global = true, long, short = 'v', action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize git-gardener configuration
    Init {
        /// Overwrite existing config file
        #[arg(long, short = 'f')]
        force: bool,
    },
    
    /// Create a new worktree
    Add {
        /// Branch name
        #[arg(short = 'b', long = "branch")]
        branch: String,
        
        /// Path where worktree will be created (default: .gardener/<branch>)
        #[arg(short = 'p', long = "path")]
        path: Option<PathBuf>,
        
        /// Set upstream remote
        #[arg(long = "upstream")]
        upstream: Option<String>,
        
        /// Create a new branch
        #[arg(short = 'c', long = "create-branch")]
        create_branch: bool,
    },
    
    /// List all worktrees
    List {
        /// Show all worktrees including prunable ones
        #[arg(short = 'a', long = "all")]
        all: bool,
        
        /// Output only worktree names (for shell completion)
        #[arg(long = "names-only")]
        names_only: bool,
    },
    
    /// View or edit configuration
    Config {
        #[command(subcommand)]
        subcommand: ConfigSubcommands,
    },
    
    /// Clean up worktrees based on conditions
    Clean {
        /// Remove merged worktrees
        #[arg(long)]
        merged: bool,
        
        /// Remove stale worktrees older than N days
        #[arg(long)]
        stale: Option<u32>,
        
        /// Force removal of all worktrees
        #[arg(long)]
        force: bool,
    },
    
    /// Pull all worktrees
    PullAll {
        /// Number of parallel operations
        #[arg(long, short = 'j')]
        parallel: Option<u32>,
    },
    
    /// Launch interactive TUI
    Tui {
        /// Run in fullscreen mode
        #[arg(long)]
        fullscreen: bool,
        
        /// Disable mouse support
        #[arg(long)]
        no_mouse: bool,
    },
    
    /// Change to worktree directory
    Cd {
        /// Worktree name to change to
        worktree_name: String,
    },
    
    /// Remove a worktree
    Remove {
        /// Name of the worktree to remove
        worktree_name: String,
        
        /// Force removal even if worktree has uncommitted changes
        #[arg(short = 'f', long = "force")]
        force: bool,
    },
    
    /// Prune worktree information
    Prune {
        /// Only show what would be removed
        #[arg(long = "dry-run")]
        dry_run: bool,
    },
    
    /// Move a worktree to a new location
    Move {
        /// Name of the worktree to move
        worktree_name: String,
        
        /// New path for the worktree
        new_path: PathBuf,
    },
    
    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completion for
        #[arg(value_enum)]
        shell: crate::commands::completion::CompletionShell,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigSubcommands {
    /// View current configuration
    View,
    
    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,
        
        /// Configuration value
        value: String,
    },
}

