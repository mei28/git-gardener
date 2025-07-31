use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "git-gardener",
    version,
    author,
    about = "Simple Git worktree management tool",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize git-gardener in the current repository
    Init {
        /// Force initialization even if already initialized
        #[arg(short, long)]
        force: bool,
    },
    
    /// Create a new worktree
    Add {
        /// Branch name
        branch: String,
        
        /// Create a new branch
        #[arg(short = 'b', long)]
        new_branch: bool,
        
        /// Create from specific commit
        #[arg(short = 'c', long)]
        commit: Option<String>,
    },
    
    /// List all worktrees
    List {
        /// Output only worktree names (for shell completion)
        #[arg(long = "names-only")]
        names_only: bool,
    },
    
    /// Change to worktree directory
    Cd {
        /// Worktree name to change to (use @ for main worktree)
        worktree: String,
    },
    
    /// Remove a worktree
    Remove {
        /// Name of the worktree to remove
        worktree: String,
        
        /// Also remove the branch
        #[arg(long = "with-branch")]
        with_branch: bool,
    },
    
    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completion for
        #[arg(value_enum)]
        shell: crate::commands::completion::CompletionShell,
    },
}

