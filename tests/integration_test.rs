use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A Git worktree management tool"));
}

#[test]
fn test_init_command_creates_config_file() {
    let temp_dir = TempDir::new().unwrap();
    
    // Gitリポジトリを初期化
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // git-gardener initを実行
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created config file"));
    
    // 設定ファイルが作成されたことを確認
    let config_path = temp_dir.path().join(".git").join("gardener.toml");
    assert!(config_path.exists());
    
    // 設定ファイルの内容を確認
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("[defaults]"));
    assert!(content.contains("root_dir = \".gardener\""));
}

#[test]
fn test_init_command_fails_without_git_repo() {
    let temp_dir = TempDir::new().unwrap();
    
    // Gitリポジトリではないディレクトリでinitを実行
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Not in a git repository"));
}

#[test]
fn test_init_command_with_existing_config() {
    let temp_dir = TempDir::new().unwrap();
    
    // Gitリポジトリを初期化
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // 1回目のinit
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // 2回目のinit（既存ファイルがある場合）
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn test_config_view_command() {
    let temp_dir = TempDir::new().unwrap();
    
    // Gitリポジトリを初期化
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // 設定ファイルを作成
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // config viewを実行
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["config", "view"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Config file:"))
        .stdout(predicate::str::contains("[defaults]"));
}

#[test]
fn test_list_command_no_worktrees() {
    let temp_dir = TempDir::new().unwrap();
    
    // Gitリポジトリを初期化
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // worktreeがない状態でlistを実行
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["list"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("No worktrees found"));
}

#[test]
fn test_add_command_with_initial_commit() {
    let temp_dir = TempDir::new().unwrap();
    
    // Gitリポジトリを初期化し、初期コミットを作成
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // 初期コミットを作成
    fs::write(temp_dir.path().join("README.md"), "# Test Repo").unwrap();
    Command::new("git")
        .args(&["add", "README.md"])
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
    
    Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // addコマンドで新しいブランチとworktreeを作成
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["add", "-b", "feature/test", "-c"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Successfully created worktree"));
    
    // listコマンドでworktreeが表示されることを確認
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["list"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("feature-test"))
        .stdout(predicate::str::contains("feature/test"));
}

#[test]
fn test_add_command_without_initial_commit() {
    let temp_dir = TempDir::new().unwrap();
    
    // Gitリポジトリを初期化（初期コミットなし）
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // addコマンドを実行（初期コミットがないため失敗するはず）
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["add", "-b", "feature/test", "-c"])
        .current_dir(&temp_dir)
        .assert()
        .failure();
}

// 🔴 RED: cleanコマンドのテスト（まだ実装されていないので失敗する）
#[test]
fn test_clean_command_removes_merged_worktrees() {
    let temp_dir = TempDir::new().unwrap();
    
    // Gitリポジトリを初期化し、初期コミットを作成
    setup_git_repo(&temp_dir);
    
    // worktreeを作成
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["add", "-b", "feature/test", "-c"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // マージ済みブランチをシミュレート（mainブランチに戻ってマージ）
    Command::new("git")
        .args(&["checkout", "main"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    Command::new("git")
        .args(&["merge", "feature/test"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // cleanコマンドでマージ済みworktreeを削除
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["clean", "--merged"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed"));
    
    // listコマンドでworktreeが削除されたことを確認
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["list"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("No worktrees found"));
}

#[test]
fn test_clean_command_with_force_flag() {
    let temp_dir = TempDir::new().unwrap();
    
    setup_git_repo(&temp_dir);
    
    // worktreeを作成
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["add", "-b", "feature/test", "-c"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // forceフラグで強制削除
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["clean", "--force"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed"));
}

// ヘルパー関数
fn setup_git_repo(temp_dir: &TempDir) {
    // Gitリポジトリを初期化
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // Git設定
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
    
    // 初期コミットを作成
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