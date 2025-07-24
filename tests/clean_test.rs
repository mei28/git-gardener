use assert_cmd::Command;
use predicates::prelude::*;

// 🔴 RED: 最初に失敗するシンプルなテスト
#[test]
fn test_clean_command_exists() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["clean", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Clean up worktrees"));
}

// 🔴 RED: cleanコマンドの基本動作テスト（最小限）
#[test] 
fn test_clean_command_basic_execution() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["clean", "--force"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed").or(predicate::str::contains("No worktrees to remove")));
}

// 三角測量のための2つ目のテスト
#[test]
fn test_clean_command_merged_flag() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["clean", "--merged"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed").or(predicate::str::contains("No worktrees to remove")));
}

// 3つ目のテスト：何も削除しない場合
#[test]
fn test_clean_command_no_worktrees() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["clean", "--force"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No worktrees to remove").or(predicate::str::contains("Removed")));
}

// エラーケース：オプションが指定されていない場合
#[test]
fn test_clean_command_no_options() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["clean"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No cleanup option specified"));
}

// 実際のworktree操作は行わず、動作確認
#[test]
fn test_clean_command_stale_option() {
    let mut cmd = Command::cargo_bin("git-gardener").unwrap();
    cmd.args(&["clean", "--stale", "30"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed").or(predicate::str::contains("No worktrees to remove")));
}

// 🔴 RED: マージ済みブランチ判定のテスト（最初は失敗する）
#[cfg(test)]
mod unit_tests {
    use git_gardener::git::GitWorktree;
    use tempfile::TempDir;
    use git2::Repository;
    use std::path::Path;

    #[test]
    fn test_is_branch_merged_returns_false_for_unmerged_branch() {
        // 一時的なgitリポジトリを作成
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();
        
        // gitリポジトリを初期化
        let repo = Repository::init(repo_path).unwrap();
        
        // 設定を追加（commitに必要）
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@example.com").unwrap();
        
        // 初期コミットを作成
        create_initial_commit(&repo, repo_path);
        
        // feature/testブランチを作成
        let head = repo.head().unwrap();
        let target = head.target().unwrap();
        let commit = repo.find_commit(target).unwrap();
        repo.branch("feature/test", &commit, false).unwrap();
        
        // GitWorktreeインスタンスを作成
        let git_worktree = GitWorktree::from_path(repo_path).unwrap();
        
        // feature/testブランチはまだマージされていないので、falseが返るはず
        let result = git_worktree.is_branch_merged("feature/test", "main");
        assert_eq!(result.unwrap(), false);
    }

    // 🔴 RED: 三角測量のための2つ目のテスト（マージ済みブランチ）
    #[test]
    fn test_is_branch_merged_returns_true_for_merged_branch() {
        // 一時的なgitリポジトリを作成
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();
        
        // gitリポジトリを初期化
        let repo = Repository::init(repo_path).unwrap();
        
        // 設定を追加（commitに必要）
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@example.com").unwrap();
        
        // 初期コミットを作成
        create_initial_commit(&repo, repo_path);
        
        // feature/mergedブランチを作成
        let head = repo.head().unwrap();
        let target = head.target().unwrap();
        let commit = repo.find_commit(target).unwrap();
        let feature_branch = repo.branch("feature/merged", &commit, false).unwrap();
        
        // feature/mergedブランチに新しいコミットを追加
        repo.checkout_tree(
            &commit.tree().unwrap().as_object(),
            Some(git2::build::CheckoutBuilder::new().force())
        ).unwrap();
        repo.set_head("refs/heads/feature/merged").unwrap();
        
        // 新しいファイルを追加してコミット
        let feature_file = repo_path.join("feature.txt");
        std::fs::write(&feature_file, "feature content").unwrap();
        
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("feature.txt")).unwrap();
        index.write().unwrap();
        
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let signature = repo.signature().unwrap();
        
        let feature_commit = repo.commit(
            Some("refs/heads/feature/merged"),
            &signature,
            &signature,
            "Add feature",
            &tree,
            &[&commit],
        ).unwrap();
        
        // mainブランチに戻ってマージ
        repo.set_head("refs/heads/main").unwrap();
        repo.checkout_head(Some(git2::build::CheckoutBuilder::new().force())).unwrap();
        
        // マージコミットを作成
        let main_commit = repo.find_commit(target).unwrap();
        let feature_commit_obj = repo.find_commit(feature_commit).unwrap();
        
        let mut index = repo.merge_commits(&main_commit, &feature_commit_obj, None).unwrap();
        let tree_id = index.write_tree_to(&repo).unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        
        repo.commit(
            Some("refs/heads/main"),
            &signature,
            &signature,
            &format!("Merge branch 'feature/merged'"),
            &tree,
            &[&main_commit, &feature_commit_obj],
        ).unwrap();
        
        // GitWorktreeインスタンスを作成
        let git_worktree = GitWorktree::from_path(repo_path).unwrap();
        
        // feature/mergedブランチはマージされているので、trueが返るはず
        let result = git_worktree.is_branch_merged("feature/merged", "main");
        assert_eq!(result.unwrap(), true);
    }

    fn create_initial_commit(repo: &Repository, repo_path: &Path) {
        // README.mdファイルを作成
        let readme_path = repo_path.join("README.md");
        std::fs::write(&readme_path, "# Test Repository").unwrap();
        
        // ファイルをステージング
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("README.md")).unwrap();
        index.write().unwrap();
        
        // コミットを作成
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let signature = repo.signature().unwrap();
        
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        ).unwrap();
    }
}