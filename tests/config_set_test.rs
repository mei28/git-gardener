use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

// 🔴 RED: config setコマンドの基本動作テスト
#[test]
fn test_config_set_basic_execution() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["config", "set", "defaults.root_dir", ".worktrees"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Set").or(predicate::str::contains("Updated")));
}

// 🔴 RED: config setで実際に値が変更されるかのテスト
#[test]
fn test_config_set_changes_value() {
    // 一時ディレクトリを作成
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".git").join("gardener.toml");
    
    // .gitディレクトリを作成してgitリポジトリを初期化
    fs::create_dir_all(temp_dir.path().join(".git")).unwrap();
    
    // 最小限のgitリポジトリ構造を作成
    let git_dir = temp_dir.path().join(".git");
    fs::write(git_dir.join("HEAD"), "ref: refs/heads/main\n").unwrap();
    fs::create_dir_all(git_dir.join("refs").join("heads")).unwrap();
    fs::create_dir_all(git_dir.join("objects")).unwrap();
    
    // 初期設定ファイルを作成
    fs::write(&config_path, r#"[defaults]
root_dir = ".gardener"
post_create = []

[branches]
"#).unwrap();

    // 現在のディレクトリを一時ディレクトリに変更
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    // config set コマンドを実行
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["config", "set", "defaults.root_dir", ".worktrees"])
        .assert()
        .success();

    // 設定ファイルの内容を確認
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains(r#"root_dir = ".worktrees""#));

    // 元のディレクトリに戻す
    std::env::set_current_dir(original_dir).unwrap();
}

// 🔴 RED: 存在しないキーに対するエラーハンドリングテスト
#[test]
fn test_config_set_invalid_key() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["config", "set", "invalid.key", "value"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid").or(predicate::str::contains("Unknown")));
}