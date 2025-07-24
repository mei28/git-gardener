use assert_cmd::Command;
use tempfile::tempdir;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_remove_command_removes_worktree() {
    // 🔴 RED: removeコマンドがworktreeを削除することをテスト
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
    
    // ユニークなブランチ名を生成
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let branch_name = format!("test-branch-{}", timestamp);
    
    // worktreeを作成
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["add", "-b", &branch_name, "-c"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // worktreeが存在することを確認
    let worktree_name = branch_name.replace('/', "-");
    let worktree_path = temp_dir.path().join(".gardener").join(&worktree_name);
    assert!(worktree_path.exists(), "Worktree should exist before removal");
    
    // removeコマンドを実行
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["remove", &worktree_name])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // worktreeが削除されたことを確認
    assert!(!worktree_path.exists(), "Worktree should be removed");
}

#[test]
fn test_remove_command_with_force_flag() {
    // 🔴 RED: --forceフラグ付きのremoveコマンドをテスト
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
    
    // ユニークなブランチ名を生成
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let branch_name = format!("force-test-{}", timestamp);
    
    // worktreeを作成
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["add", "-b", &branch_name, "-c"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // worktree内にファイルを作成（変更を加える）
    let worktree_name = branch_name.replace('/', "-");
    let worktree_path = temp_dir.path().join(".gardener").join(&worktree_name);
    let dirty_file = worktree_path.join("dirty.txt");
    fs::write(&dirty_file, "uncommitted changes").unwrap();
    
    // --forceフラグ付きでremoveコマンドを実行
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["remove", &worktree_name, "--force"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // worktreeが削除されたことを確認
    assert!(!worktree_path.exists(), "Worktree should be removed with force flag");
}

#[test]
fn test_remove_command_nonexistent_worktree() {
    // 🔴 RED: 存在しないworktreeを削除しようとした場合のエラーテスト
    let temp_dir = tempdir().unwrap();
    
    // gitリポジトリを初期化
    Command::new("git")
        .args(&["init"])
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
    
    // 存在しないworktreeを削除しようとする
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["remove", "nonexistent"])
        .current_dir(&temp_dir)
        .assert()
        .failure();
}

#[test]
fn test_remove_command_tab_completion() {
    // 🔴 RED: removeコマンドでworktree名のタブ補完をテスト
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
    
    // ユニークなブランチ名を生成
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let branch_one = format!("feature-one-{}", timestamp);
    let branch_two = format!("feature-two-{}", timestamp + 1);
    
    // 複数のworktreeを作成
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["add", "-b", &branch_one, "-c"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["add", "-b", &branch_two, "-c"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // listコマンドで--names-onlyオプションを使用して補完用のworktree名を取得
    let output = Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["list", "--names-only"])
        .current_dir(&temp_dir)
        .output()
        .unwrap();
    
    let worktree_names = String::from_utf8(output.stdout).unwrap();
    let worktree_one_name = branch_one.replace('/', "-");
    let worktree_two_name = branch_two.replace('/', "-");
    assert!(worktree_names.contains(&worktree_one_name), "Should list first worktree");
    assert!(worktree_names.contains(&worktree_two_name), "Should list second worktree");
}