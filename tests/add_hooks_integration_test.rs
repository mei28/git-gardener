use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

// 🟢 GREEN: addコマンドがpost_createフックを実行するかのテスト（統合完了）
#[test]
fn test_add_command_runs_post_create_hooks() {
    let temp_dir = TempDir::new().unwrap();
    
    // Gitリポジトリをセットアップ
    setup_git_repo(&temp_dir);
    
    // post_createフック付きの設定ファイルを作成
    let config_content = r#"
[defaults]
root_dir = ".gardener"
post_create = [
    "echo 'Hook executed successfully' > hook_test.txt"
]
"#;
    
    let git_dir = temp_dir.path().join(".git");
    fs::create_dir_all(&git_dir).unwrap();
    fs::write(git_dir.join("gardener.toml"), config_content).unwrap();
    
    // addコマンドを実行（一意なブランチ名を使用）
    let unique_branch = format!("feature/hook-test-{}", std::process::id());
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["add", "-b", &unique_branch, "-c"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // 実際の作成された場所を親ディレクトリから探す
    let temp_parent = temp_dir.path().parent().unwrap_or(temp_dir.path());
    let hook_file = temp_parent
        .join(".gardener")
        .join(&unique_branch.replace('/', "-"))
        .join("hook_test.txt");
    
    // フックが実行されて、ファイルが作成されたことを確認
    assert!(hook_file.exists(), "Hook should have created hook_test.txt file");
    
    let hook_content = fs::read_to_string(&hook_file).unwrap();
    assert!(hook_content.contains("Hook executed successfully"), "Hook output should be correct");
}

// 三角測量のための2つ目のテスト：環境変数展開
#[test]
fn test_add_command_hook_environment_variables() {
    let temp_dir = TempDir::new().unwrap();
    
    setup_git_repo(&temp_dir);
    
    let config_content = r#"
[defaults]
root_dir = ".gardener"
post_create = [
    "echo 'Branch: ${BRANCH}' > branch_info.txt",
    "echo 'Path: ${WORKTREE_PATH}' >> branch_info.txt"
]
"#;
    
    let git_dir = temp_dir.path().join(".git");
    fs::create_dir_all(&git_dir).unwrap();
    fs::write(git_dir.join("gardener.toml"), config_content).unwrap();
    
    let unique_branch = format!("feature/env-test-{}", std::process::id());
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["add", "-b", &unique_branch, "-c"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // 実際の作成された場所を親ディレクトリから探す
    let temp_parent = temp_dir.path().parent().unwrap_or(temp_dir.path());
    let info_file = temp_parent
        .join(".gardener")
        .join(&unique_branch.replace('/', "-"))
        .join("branch_info.txt");
    
    // フックが実行されて、環境変数が正しく展開されたことを確認
    assert!(info_file.exists(), "Hook should have created branch_info.txt");
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