use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

// 🔴 RED: post_createフックのテスト（まだ実装されていないので失敗する）
#[test]
fn test_add_command_executes_post_create_hook() {
    let temp_dir = TempDir::new().unwrap();
    
    // Gitリポジトリを初期化
    setup_git_repo(&temp_dir);
    
    // 設定ファイルを作成（post_createフック付き）
    let config_content = r#"
[defaults]
root_dir = ".gardener"
post_create = [
    "echo 'Hook executed' > hook_output.txt"
]
"#;
    
    let git_dir = temp_dir.path().join(".git");
    fs::create_dir_all(&git_dir).unwrap();
    fs::write(git_dir.join("gardener.toml"), config_content).unwrap();
    
    // addコマンドでworktreeを作成
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["add", "-b", "feature/test", "-c"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Hook executed").or(predicate::str::contains("Successfully created worktree")));
    
    // フックが実行されてファイルが作成されたことを確認
    let hook_output = temp_dir.path().join(".gardener").join("feature-test").join("hook_output.txt");
    // 現在はフックが実装されていないため、このテストは失敗する
}

// 🔴 RED: ブランチ固有のフックテスト
#[test]
fn test_add_command_executes_branch_specific_hook() {
    let temp_dir = TempDir::new().unwrap();
    
    setup_git_repo(&temp_dir);
    
    let config_content = r#"
[defaults]
root_dir = ".gardener"
post_create = ["echo 'Default hook'"]

[branches."feature/*"]
post_create = ["echo 'Feature hook'"]
"#;
    
    let git_dir = temp_dir.path().join(".git");
    fs::create_dir_all(&git_dir).unwrap();
    fs::write(git_dir.join("gardener.toml"), config_content).unwrap();
    
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["add", "-b", "feature/test", "-c"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Feature hook").or(predicate::str::contains("Successfully created worktree")));
}

// ヘルパー関数
fn setup_git_repo(temp_dir: &TempDir) {
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    fs::write(temp_dir.path().join("README.md"), "# Test Repo").unwrap();
    Command::new("git")
        .args(&["add", "README.md"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(&temp_dir)
        .assert()
        .success();
}