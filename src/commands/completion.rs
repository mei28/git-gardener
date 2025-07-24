use crate::error::Result;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum CompletionShell {
    Bash,
    Zsh,
    Fish,
}

pub struct CompletionCommand {
    pub shell: CompletionShell,
}

impl CompletionCommand {
    pub fn new(shell: CompletionShell) -> Self {
        Self { shell }
    }

    pub fn execute(&self) -> Result<()> {
        let completion_content = match self.shell {
            CompletionShell::Bash => include_str!("../../completions/git-gardener.bash"),
            CompletionShell::Zsh => include_str!("../../completions/git-gardener.zsh"),
            CompletionShell::Fish => include_str!("../../completions/git-gardener.fish"),
        };

        println!("{}", completion_content);
        Ok(())
    }
}