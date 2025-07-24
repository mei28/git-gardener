use assert_cmd::Command;
use predicates::prelude::*;

// 🔴 RED: pull-allコマンドのテスト（まだ実装されていないので失敗する）
#[test]
fn test_pull_all_command_exists() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["pull-all", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Pull all worktrees"));
}

#[test]
fn test_pull_all_command_basic_execution() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["pull-all"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Pulled").or(predicate::str::contains("No worktrees")));
}

// 三角測量のための2つ目のテスト
#[test]
fn test_pull_all_command_with_parallel_option() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["pull-all", "--parallel", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Pulled").or(predicate::str::contains("No worktrees")));
}

// 三角測量の3つ目のテスト：短縮形オプション
#[test]
fn test_pull_all_command_with_short_parallel_option() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["pull-all", "-j", "4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Pulled").or(predicate::str::contains("No worktrees")));
}