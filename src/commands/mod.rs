pub mod init;
pub mod add;
pub mod list;
pub mod config;
pub mod clean;
pub mod pull_all;
pub mod tui;
pub mod cd;
pub mod remove;
pub mod prune;
pub mod r#move;

use crate::error::Result;

pub trait Command {
    fn execute(&self) -> Result<()>;
}