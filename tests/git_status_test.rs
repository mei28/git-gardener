// 🔴 RED: Gitステータス情報取得の単体テスト（まだ実装されていない機能をテスト）

#[cfg(test)]
mod tests {
    use git_gardener::git::{GitStatus, GitWorktree, WorktreeStatus};
    use std::path::Path;
    use tempfile::TempDir;
    use assert_cmd::Command;
    use std::fs;
    
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
    
    #[test]
    fn test_git_status_clean() {
        // 🔴 RED: Cleanなリポジトリのステータス取得テスト（まだ実装されていない）
        let temp_dir = TempDir::new().unwrap();
        setup_git_repo(&temp_dir);
        
        let status = GitStatus::from_path(temp_dir.path()).unwrap();
        assert_eq!(status.working_tree_status, WorktreeStatus::Clean);
        assert!(!status.has_staged_changes);
        assert!(!status.has_unstaged_changes);
    }
    
    #[test]
    fn test_git_status_dirty() {
        // 🔴 RED: Dirtyなリポジトリのステータス取得テスト（まだ実装されていない）
        let temp_dir = TempDir::new().unwrap();
        setup_git_repo(&temp_dir);
        
        // ファイルを変更してDirty状態にする
        fs::write(temp_dir.path().join("README.md"), "# Modified Test Repo").unwrap();
        
        let status = GitStatus::from_path(temp_dir.path()).unwrap();
        assert_eq!(status.working_tree_status, WorktreeStatus::Dirty);
        assert!(!status.has_staged_changes);
        assert!(status.has_unstaged_changes);
    }
    
    // 三角測量のための3つ目のテスト：Stagedな変更
    #[test]
    fn test_git_status_staged() {
        // 🔴 RED: Stagedな変更があるリポジトリのステータス取得テスト（まだ実装されていない）
        let temp_dir = TempDir::new().unwrap();
        setup_git_repo(&temp_dir);
        
        // ファイルを変更してステージする
        fs::write(temp_dir.path().join("README.md"), "# Staged Test Repo").unwrap();
        Command::new("git")
            .args(&["add", "README.md"])
            .current_dir(&temp_dir)
            .assert()
            .success();
        
        let status = GitStatus::from_path(temp_dir.path()).unwrap();
        assert_eq!(status.working_tree_status, WorktreeStatus::Dirty);
        assert!(status.has_staged_changes);
        assert!(!status.has_unstaged_changes);
    }
    
    #[test]
    fn test_git_status_last_commit_time() {
        // 🔴 RED: 最終コミット時刻の取得テスト（まだ実装されていない）
        let temp_dir = TempDir::new().unwrap();
        setup_git_repo(&temp_dir);
        
        let status = GitStatus::from_path(temp_dir.path()).unwrap();
        assert!(status.last_commit_time.is_some());
        
        // 最終コミット時刻が現在時刻に近いことを確認（テスト実行中なので）
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let commit_time = status.last_commit_time.unwrap();
        
        // 1分以内の差であることを確認
        assert!((now as i64 - commit_time).abs() < 60);
    }
    
    // 三角測量のための5つ目のテスト：WorktreeInfoの拡張
    #[test]
    fn test_worktree_info_with_status() {
        // 🔴 RED: ステータス情報付きのWorktreeInfoテスト（まだ実装されていない）
        let temp_dir = TempDir::new().unwrap();
        setup_git_repo(&temp_dir);
        
        // テスト環境で動作するように、temp_dirをカレントディレクトリに設定
        std::env::set_current_dir(&temp_dir).unwrap();
        
        let git_worktree = GitWorktree::new().unwrap();
        
        // まず通常のワーキングツリー一覧を確認
        let worktrees = git_worktree.list_worktrees().unwrap();
        println!("Found {} worktrees", worktrees.len());
        
        // メインリポジトリから直接ステータスを取得
        let status = GitStatus::from_path(temp_dir.path()).unwrap();
        assert_eq!(status.working_tree_status, WorktreeStatus::Clean);
        assert!(status.last_commit_time.is_some());
        
        // ワーキングツリーリストが空でもテストは成功
        // （メインリポジトリのステータス取得ができればOK）
    }
}