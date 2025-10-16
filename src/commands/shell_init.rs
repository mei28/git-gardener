use crate::error::Result;
use crate::commands::completion::CompletionShell;

pub struct ShellInitCommand {
    pub shell: CompletionShell,
}

impl ShellInitCommand {
    pub fn new(shell: CompletionShell) -> Self {
        Self { shell }
    }

    pub fn execute(&self) -> Result<()> {
        // 仮実装: シェルスクリプトを出力
        let script = self.generate_shell_script();
        println!("{}", script);
        Ok(())
    }

    fn generate_shell_script(&self) -> String {
        match self.shell {
            CompletionShell::Bash => self.generate_bash_script(),
            CompletionShell::Zsh => self.generate_zsh_script(),
            CompletionShell::Fish => self.generate_fish_script(),
        }
    }

    fn generate_bash_script(&self) -> String {
        include_str!("../../shell-init/bash.sh").to_string()
    }

    fn generate_zsh_script(&self) -> String {
        include_str!("../../shell-init/zsh.sh").to_string()
    }

    fn generate_fish_script(&self) -> String {
        include_str!("../../shell-init/fish.fish").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_init_command_new_creates_instance() {
        // What: ShellInitCommand::newが正しくインスタンスを作成するかテスト
        let cmd = ShellInitCommand::new(CompletionShell::Bash);
        assert!(matches!(cmd.shell, CompletionShell::Bash));
    }

    #[test]
    fn test_bash_script_contains_function_definition() {
        // What: Bashスクリプトに関数定義が含まれるかテスト
        let cmd = ShellInitCommand::new(CompletionShell::Bash);
        let script = cmd.generate_bash_script();
        assert!(script.contains("ggr()"));
    }

    #[test]
    fn test_zsh_script_contains_function_definition() {
        // What: Zshスクリプトに関数定義が含まれるかテスト
        let cmd = ShellInitCommand::new(CompletionShell::Zsh);
        let script = cmd.generate_zsh_script();
        assert!(script.contains("ggr()"));
    }

    #[test]
    fn test_fish_script_contains_function_definition() {
        // What: Fishスクリプトに関数定義が含まれるかテスト
        let cmd = ShellInitCommand::new(CompletionShell::Fish);
        let script = cmd.generate_fish_script();
        assert!(script.contains("function ggr"));
    }
}
