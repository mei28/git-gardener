use crate::error::{GitGardenerError, Result};
use std::path::Path;
use std::process::Command;

pub struct HookExecutor;

impl HookExecutor {
    pub fn new() -> Self {
        Self
    }
    
    pub fn execute_post_create(&self, worktree_path: &Path, branch: &str, commands: &[String]) -> Result<()> {
        // 🔵 REFACTOR: 三角測量による一般化と実際の機能実装
        
        if commands.is_empty() {
            return Ok(());
        }
        
        for command in commands {
            // 環境変数を設定してコマンドを実行
            let expanded_command = self.expand_variables(command, worktree_path, branch);
            
            match self.execute_shell_command(&expanded_command, worktree_path) {
                Ok(_) => {
                    tracing::debug!("Hook command succeeded: {}", expanded_command);
                }
                Err(e) => {
                    tracing::error!("Hook command failed: {} - {}", expanded_command, e);
                    return Err(GitGardenerError::Custom(
                        format!("Hook command failed: {}", e)
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    fn expand_variables(&self, command: &str, worktree_path: &Path, branch: &str) -> String {
        // 環境変数を展開
        command
            .replace("${WORKTREE_PATH}", &worktree_path.display().to_string())
            .replace("${BRANCH}", branch)
            .replace("${REPO_ROOT}", &worktree_path.parent().unwrap_or(worktree_path).display().to_string())
    }
    
    fn execute_shell_command(&self, command: &str, working_dir: &Path) -> Result<()> {
        // POSIXシェルでコマンドを実行
        let mut cmd = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(&["/C", command]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(&["-c", command]);
            cmd
        };
        
        // ワーキングディレクトリが存在する場合のみ設定
        if working_dir.exists() {
            cmd.current_dir(working_dir);
        }
        
        let output = cmd.output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitGardenerError::Custom(
                format!("Command failed with exit code {:?}: {}", output.status.code(), stderr)
            ));
        }
        
        Ok(())
    }
    
    // テスト用の関数
    pub fn mock_execute() -> bool {
        true
    }
}