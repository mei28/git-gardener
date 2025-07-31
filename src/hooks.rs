use crate::config::Hook;
use crate::error::{GitGardenerError, Result};
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;

pub struct HookExecutor;

impl HookExecutor {
    pub fn new() -> Self {
        Self
    }
    
    pub fn execute_hooks(&self, worktree_path: &Path, branch: &str, hooks: &[Hook]) -> Result<()> {
        for hook in hooks {
            match &hook.hook_type {
                crate::config::HookType::Copy => {
                    self.execute_copy_hook(hook, worktree_path)?;
                }
                crate::config::HookType::Command => {
                    self.execute_command_hook(hook, worktree_path, branch)?;
                }
            }
        }
        
        Ok(())
    }
    
    fn execute_copy_hook(&self, hook: &Hook, worktree_path: &Path) -> Result<()> {
        let from = hook.from.as_ref()
            .ok_or_else(|| GitGardenerError::Custom("Copy hook requires 'from' field".to_string()))?;
        let to = hook.to.as_ref()
            .ok_or_else(|| GitGardenerError::Custom("Copy hook requires 'to' field".to_string()))?;
        
        let source = Path::new(from);
        let dest = worktree_path.join(to);
        
        if !source.exists() {
            return Err(GitGardenerError::Custom(
                format!("Source file does not exist: {}", source.display())
            ));
        }
        
        // 宛先ディレクトリを作成
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::copy(source, &dest)?;
        println!("✓ Copied {} to {}", source.display(), dest.display());
        
        Ok(())
    }
    
    fn execute_command_hook(&self, hook: &Hook, worktree_path: &Path, branch: &str) -> Result<()> {
        let command = hook.command.as_ref()
            .ok_or_else(|| GitGardenerError::Custom("Command hook requires 'command' field".to_string()))?;
        
        let expanded_command = self.expand_variables(command, worktree_path, branch);
        
        let mut env = HashMap::new();
        if let Some(hook_env) = &hook.env {
            for (key, value) in hook_env {
                env.insert(key.clone(), self.expand_variables(value, worktree_path, branch));
            }
        }
        
        match self.execute_shell_command(&expanded_command, worktree_path, &env) {
            Ok(_) => {
                println!("✓ Executed: {}", expanded_command);
            }
            Err(e) => {
                return Err(GitGardenerError::Custom(
                    format!("Command failed: {}", e)
                ));
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
    
    fn execute_shell_command(&self, command: &str, working_dir: &Path, env: &HashMap<String, String>) -> Result<()> {
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
        
        // 環境変数を設定
        cmd.envs(env);
        
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