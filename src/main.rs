mod cli;
mod commands;
mod config;
mod error;
mod git;
mod hooks;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{add::AddCommand, cd::CdCommand, completion::CompletionCommand, list::ListCommand, remove::RemoveCommand};
use error::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Add {
            branch,
            new_branch,
            commit,
        } => {
            let cmd = AddCommand::new(branch, new_branch, commit);
            cmd.execute()
        }
        Commands::List { names_only } => {
            let cmd = ListCommand::new(names_only);
            cmd.execute()
        }
        Commands::Cd { worktree } => {
            let cmd = CdCommand::new(worktree);
            let path = cmd.execute()?;
            println!("{}", path);
            Ok(())
        }
        Commands::Remove { worktree, with_branch } => {
            let cmd = RemoveCommand::new(worktree, with_branch);
            cmd.execute()
        }
        Commands::Completion { shell } => {
            let cmd = CompletionCommand::new(shell);
            cmd.execute()
        }
    }
}
