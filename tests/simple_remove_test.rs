use assert_cmd::Command;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_simple_remove_worktree() {
    let temp_dir = tempdir().unwrap();
    
    // gitリポジトリを初期化
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // 初期コミットを作成
    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test").unwrap();
    
    Command::new("git")
        .args(&["add", "."])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // git-gardener initを実行
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // 既存のブランチを使ってworktreeを作成（-cフラグなし）
    Command::new("git")
        .args(&["branch", "test-branch"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["add", "-b", "test-branch"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // worktreeが存在することを確認
    let worktree_path = temp_dir.path().join(".gardener").join("test-branch");
    assert!(worktree_path.exists(), "Worktree should exist before removal");
    
    // removeコマンドを実行
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["remove", "test-branch"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // worktreeが削除されたことを確認
    assert!(!worktree_path.exists(), "Worktree should be removed");
}