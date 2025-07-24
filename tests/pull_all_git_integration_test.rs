use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

// 🔴 RED: 実際のgit pull処理のテスト（まだ仮実装なので期待される動作をしない）

#[test]
fn test_pull_all_actually_pulls_changes() {
    let temp_dir = TempDir::new().unwrap();
    
    // メインリポジトリをセットアップ
    setup_git_repo(&temp_dir);
    
    // ベアリポジトリを作成（リモートとして使用）
    let bare_repo_dir = TempDir::new().unwrap();
    Command::new("git")
        .args(&["clone", "--bare", temp_dir.path().to_str().unwrap(), "."])
        .current_dir(&bare_repo_dir)
        .assert()
        .success();
    
    // メインリポジトリにリモートを設定
    Command::new("git")
        .args(&["remote", "add", "origin", bare_repo_dir.path().to_str().unwrap()])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // ワーキングツリーを作成（一意なブランチ名を使用）
    let unique_branch = format!("feature/pull-test-{}", std::process::id());
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["add", "-b", &unique_branch, "-c"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // メインブランチで新しいコミットを作成
    fs::write(temp_dir.path().join("new_file.txt"), "New content").unwrap();
    Command::new("git")
        .args(&["add", "new_file.txt"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    Command::new("git")
        .args(&["commit", "-m", "Add new file"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // リモートにプッシュ
    Command::new("git")
        .args(&["push", "origin", "main"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // ワーキングツリーでfeatureブランチをmainから作成し直してリモートから最新を取得する準備
    let temp_parent = temp_dir.path().parent().unwrap_or(temp_dir.path());
    let worktree_path = temp_parent.join(".gardener").join(&unique_branch.replace('/', "-"));
    
    if worktree_path.exists() {
        // 既存のリモートを削除（存在する場合）
        Command::new("git")
            .args(&["remote", "remove", "origin"])
            .current_dir(&worktree_path)
            .output()  // エラーを無視
            .unwrap();
        
        // ワーキングツリーのリモートを設定
        Command::new("git")
            .args(&["remote", "add", "origin", bare_repo_dir.path().to_str().unwrap()])
            .current_dir(&worktree_path)
            .assert()
            .success();
        
        // まず、main ブランチの情報を取得
        Command::new("git")
            .args(&["fetch", "origin", "main"])
            .current_dir(&worktree_path)
            .assert()
            .success();
    }
    
    // pull-allコマンドを実行
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    let output = cmd.args(&["pull-all"])
        .current_dir(&temp_dir)
        .output()
        .unwrap();
    
    println!("Pull-all stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Pull-all stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    // コマンドが成功することを確認
    assert!(output.status.success(), "pull-all command should succeed");
    
    // 実際にgit pullが実行されていることを確認
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pulled"), "Should show pull progress");
    
    // 🔴 RED: 現在の仮実装では、実際のpullは行われないため、この確認は失敗する
    // ワーキングツリーに最新の変更が反映されていることを確認
    if worktree_path.exists() {
        let new_file_in_worktree = worktree_path.join("new_file.txt");
        assert!(new_file_in_worktree.exists(), 
               "New file should be pulled to worktree - this will fail until real git pull is implemented");
    }
}

// 三角測量のための2つ目のテスト：複数ワーキングツリーでのpull
#[test]
fn test_pull_all_multiple_worktrees() {
    let temp_dir = TempDir::new().unwrap();
    
    // メインリポジトリをセットアップ
    setup_git_repo(&temp_dir);
    
    // ベアリポジトリを作成（リモートとして使用）
    let bare_repo_dir = TempDir::new().unwrap();
    Command::new("git")
        .args(&["clone", "--bare", temp_dir.path().to_str().unwrap(), "."])
        .current_dir(&bare_repo_dir)
        .assert()
        .success();
    
    // メインリポジトリにリモートを設定
    Command::new("git")
        .args(&["remote", "add", "origin", bare_repo_dir.path().to_str().unwrap()])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // 複数のワーキングツリーを作成
    let unique_branch1 = format!("feature/test1-{}", std::process::id());
    let unique_branch2 = format!("feature/test2-{}", std::process::id());
    
    for branch in &[&unique_branch1, &unique_branch2] {
        let mut cmd = Command::cargo_bin("git-gardener").unwrap();
        cmd.args(&["add", "-b", branch, "-c"])
            .current_dir(&temp_dir)
            .assert()
            .success();
    }
    
    // メインブランチで新しいコミットを作成
    fs::write(temp_dir.path().join("multi_test.txt"), "Multi worktree test").unwrap();
    Command::new("git")
        .args(&["add", "multi_test.txt"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    Command::new("git")
        .args(&["commit", "-m", "Add multi test file"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // リモートにプッシュ
    Command::new("git")
        .args(&["push", "origin", "main"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // pull-allコマンドを実行
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    let output = cmd.args(&["pull-all"])
        .current_dir(&temp_dir)
        .output()
        .unwrap();
    
    println!("Multi pull stdout: {}", String::from_utf8_lossy(&output.stdout));
    
    // コマンドが成功することを確認
    assert!(output.status.success(), "pull-all command should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pulling 2 worktrees"), "Should show correct worktree count");
    assert!(stdout.contains("Successfully pulled 2 worktrees"), "Should pull all worktrees");
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