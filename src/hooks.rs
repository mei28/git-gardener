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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Hook, HookType};
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_hook_executor_new_creates_instance() {
        // What: HookExecutor::newが正しくインスタンスを作成するかテスト
        let _executor = HookExecutor::new();
    }

    #[test]
    fn test_execute_copy_hook_copies_file() {
        // What: copyフックがファイルを正しくコピーするかテスト
        let temp_dir = tempdir().unwrap();
        let worktree_path = temp_dir.path().join("worktree");
        fs::create_dir_all(&worktree_path).unwrap();
        
        // ソースファイルを作成
        let source_file = temp_dir.path().join("source.txt");
        fs::write(&source_file, "test content").unwrap();
        
        let hook = Hook {
            hook_type: HookType::Copy,
            from: Some(source_file.to_string_lossy().to_string()),
            to: Some("dest.txt".to_string()),
            command: None,
            env: None,
        };
        
        let executor = HookExecutor::new();
        let result = executor.execute_copy_hook(&hook, &worktree_path);
        
        assert!(result.is_ok());
        
        // ファイルがコピーされたことを確認
        let dest_file = worktree_path.join("dest.txt");
        assert!(dest_file.exists());
        let content = fs::read_to_string(dest_file).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_execute_copy_hook_fails_without_from_field() {
        // What: fromフィールドがないcopyフックがエラーになるかテスト
        let temp_dir = tempdir().unwrap();
        let worktree_path = temp_dir.path().join("worktree");
        
        let hook = Hook {
            hook_type: HookType::Copy,
            from: None,
            to: Some("dest.txt".to_string()),
            command: None,
            env: None,
        };
        
        let executor = HookExecutor::new();
        let result = executor.execute_copy_hook(&hook, &worktree_path);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GitGardenerError::Custom(_)));
    }

    #[test]
    fn test_execute_copy_hook_fails_without_to_field() {
        // What: toフィールドがないcopyフックがエラーになるかテスト
        let temp_dir = tempdir().unwrap();
        let worktree_path = temp_dir.path().join("worktree");
        
        let hook = Hook {
            hook_type: HookType::Copy,
            from: Some("source.txt".to_string()),
            to: None,
            command: None,
            env: None,
        };
        
        let executor = HookExecutor::new();
        let result = executor.execute_copy_hook(&hook, &worktree_path);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GitGardenerError::Custom(_)));
    }

    #[test]
    fn test_execute_copy_hook_fails_for_nonexistent_source() {
        // What: 存在しないソースファイルでcopyフックがエラーになるかテスト
        let temp_dir = tempdir().unwrap();
        let worktree_path = temp_dir.path().join("worktree");
        
        let hook = Hook {
            hook_type: HookType::Copy,
            from: Some("nonexistent.txt".to_string()),
            to: Some("dest.txt".to_string()),
            command: None,
            env: None,
        };
        
        let executor = HookExecutor::new();
        let result = executor.execute_copy_hook(&hook, &worktree_path);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GitGardenerError::Custom(_)));
    }

    #[test]
    fn test_execute_command_hook_runs_command() {
        // What: commandフックがコマンドを正しく実行するかテスト
        let temp_dir = tempdir().unwrap();
        let worktree_path = temp_dir.path().join("worktree");
        fs::create_dir_all(&worktree_path).unwrap();
        
        let hook = Hook {
            hook_type: HookType::Command,
            from: None,
            to: None,
            command: Some("echo 'test' > test.txt".to_string()),
            env: None,
        };
        
        let executor = HookExecutor::new();
        let result = executor.execute_command_hook(&hook, &worktree_path, "test-branch");
        
        assert!(result.is_ok());
        
        // コマンドが実行されたことを確認
        let test_file = worktree_path.join("test.txt");
        assert!(test_file.exists());
    }

    #[test]
    fn test_execute_command_hook_fails_without_command_field() {
        // What: commandフィールドがないcommandフックがエラーになるかテスト
        let temp_dir = tempdir().unwrap();
        let worktree_path = temp_dir.path().join("worktree");
        
        let hook = Hook {
            hook_type: HookType::Command,
            from: None,
            to: None,
            command: None,
            env: None,
        };
        
        let executor = HookExecutor::new();
        let result = executor.execute_command_hook(&hook, &worktree_path, "test-branch");
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GitGardenerError::Custom(_)));
    }

    #[test]
    fn test_expand_variables_replaces_placeholders() {
        // What: 環境変数のプレースホルダが正しく置換されるかテスト
        let temp_dir = tempdir().unwrap();
        let worktree_path = temp_dir.path().join("worktree");
        
        let executor = HookExecutor::new();
        let command = "echo '${BRANCH}' '${WORKTREE_PATH}'";
        let expanded = executor.expand_variables(command, &worktree_path, "feature-test");
        
        assert!(expanded.contains("feature-test"));
        assert!(expanded.contains(&worktree_path.display().to_string()));
    }

    #[test]
    fn test_execute_hooks_runs_multiple_hooks() {
        // What: 複数のフックが順番に実行されるかテスト
        let temp_dir = tempdir().unwrap();
        let worktree_path = temp_dir.path().join("worktree");
        fs::create_dir_all(&worktree_path).unwrap();
        
        // ソースファイルを作成
        let source_file = temp_dir.path().join("source.txt");
        fs::write(&source_file, "test content").unwrap();
        
        let hooks = vec![
            Hook {
                hook_type: HookType::Copy,
                from: Some(source_file.to_string_lossy().to_string()),
                to: Some("copied.txt".to_string()),
                command: None,
                env: None,
            },
            Hook {
                hook_type: HookType::Command,
                from: None,
                to: None,
                command: Some("echo 'command executed' > executed.txt".to_string()),
                env: None,
            },
        ];
        
        let executor = HookExecutor::new();
        let result = executor.execute_hooks(&worktree_path, "test-branch", &hooks);
        
        assert!(result.is_ok());
        
        // 両方のフックが実行されたことを確認
        assert!(worktree_path.join("copied.txt").exists());
        assert!(worktree_path.join("executed.txt").exists());
    }
}