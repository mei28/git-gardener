pub mod init;
pub mod add;
pub mod list;
pub mod config;
pub mod clean;
pub mod pull_all;
pub mod tui;
pub mod cd;

use crate::error::Result;

pub trait Command {
    fn execute(&self) -> Result<()>;
}