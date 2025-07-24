mod cli;
mod commands;
mod config;
mod error;
mod git;
mod hooks;

use clap::Parser;
use cli::{Cli, Commands, ConfigSubcommands};
use commands::{add::AddCommand, cd::CdCommand, clean::CleanCommand, config::{ConfigCommand, ConfigSubcommand}, init::InitCommand, list::ListCommand, pull_all::PullAllCommand, tui::TuiCommand, remove::RemoveCommand, prune::PruneCommand, r#move::MoveCommand};
use error::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    
    // ログレベルの設定
    let log_level = match cli.verbose {
        0 => Level::ERROR,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    
    match cli.command {
        Commands::Init { force } => {
            info!("Running init command");
            let cmd = InitCommand::new(force);
            cmd.execute()
        }
        Commands::Add {
            branch,
            path,
            upstream,
            create_branch,
        } => {
            info!("Running add command for branch: {}", branch);
            let cmd = AddCommand::new(branch, path, upstream, create_branch);
            cmd.execute()
        }
        Commands::List { all, names_only } => {
            info!("Running list command");
            let cmd = ListCommand::new(all, names_only);
            cmd.execute()
        }
        Commands::Config { subcommand } => {
            info!("Running config command");
            let subcmd = match subcommand {
                ConfigSubcommands::View => ConfigSubcommand::View,
                ConfigSubcommands::Set { key, value } => ConfigSubcommand::Set { key, value },
            };
            let cmd = ConfigCommand::new(subcmd);
            cmd.execute()
        }
        Commands::Clean { merged, stale, force } => {
            info!("Running clean command");
            let cmd = CleanCommand::new(merged, stale, force);
            cmd.execute()
        }
        Commands::PullAll { parallel } => {
            info!("Running pull-all command");
            let cmd = PullAllCommand::new(parallel);
            cmd.execute()
        }
        Commands::Tui { fullscreen, no_mouse } => {
            info!("Running TUI command");
            let cmd = TuiCommand::new(fullscreen, no_mouse);
            cmd.execute()
        }
        Commands::Cd { worktree_name } => {
            info!("Running cd command for worktree: {}", worktree_name);
            let cmd = CdCommand::new(worktree_name);
            let path = cmd.execute()?;
            println!("{}", path);
            Ok(())
        }
        Commands::Remove { worktree_name, force } => {
            info!("Running remove command for worktree: {}", worktree_name);
            let cmd = RemoveCommand::new(worktree_name, force);
            cmd.execute()
        }
        Commands::Prune { dry_run } => {
            info!("Running prune command");
            let cmd = PruneCommand::new(dry_run);
            cmd.execute()
        }
        Commands::Move { worktree_name, new_path } => {
            info!("Running move command for worktree: {}", worktree_name);
            let cmd = MoveCommand::new(worktree_name, new_path);
            cmd.execute()
        }
    }
}
