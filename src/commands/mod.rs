pub mod add;
pub mod cd;
pub mod completion;
pub mod list;
pub mod remove;

use crate::error::Result;

pub trait Command {
    fn execute(&self) -> Result<()>;
}