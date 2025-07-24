use git_gardener::commands::add::AddCommand;
use git_gardener::git::GitWorktree;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// テスト用のgitリポジトリを作成する
fn create_test_git_repo(temp_dir: &TempDir) -> PathBuf {
    use git2::{Repository, Signature, ObjectType};
    
    let repo_path = temp_dir.path().to_path_buf();
    
    // git2を使って適切なリポジトリを初期化
    let repo = Repository::init(&repo_path).unwrap();
    
    // 設定を追加
    let mut config = repo.config().unwrap();
    config.set_str("user.name", "Test User").unwrap();
    config.set_str("user.email", "test@example.com").unwrap();
    
    // 初期コミットを作成
    let signature = Signature::now("Test User", "test@example.com").unwrap();
    let tree_id = {
        let mut builder = repo.treebuilder(None).unwrap();
        let blob_id = repo.blob(b"# Test\n").unwrap();
        builder.insert("README.md", blob_id, 0o100644).unwrap();
        builder.write().unwrap()
    };
    let tree = repo.find_tree(tree_id).unwrap();
    
    let _commit = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    ).unwrap();
    
    repo_path
}

#[test]
fn test_worktree_created_at_same_level_as_git_directory() {
    let temp_dir = tempfile::tempdir().unwrap();
    let repo_path = create_test_git_repo(&temp_dir);
    
    // テスト前に.gardenerディレクトリをクリーンアップ
    if let Some(parent) = temp_dir.path().parent() {
        let old_gardener = parent.join(".gardener");
        if old_gardener.exists() {
            std::fs::remove_dir_all(&old_gardener).ok();
        }
    }
    
    // テスト対象のディレクトリに移動
    std::env::set_current_dir(&repo_path).unwrap();
    
    let add_command = AddCommand::new(
        "feature/test".to_string(),
        None,  // パスを指定しない（デフォルトの.gardenerを使用）
        Some("main".to_string()),
        true,  // create_branch = true
    );
    
    // GitWorktreeインスタンスを作成してrepo_rootを確認
    let git_worktree = git_gardener::git::GitWorktree::new().unwrap();
    let repo_root_from_git = git_worktree.get_repository_root().unwrap();
    println!("repo_root from GitWorktree: {}", repo_root_from_git.display());
    
    let result = add_command.execute();
    
    // デバッグ情報を表示
    println!("repo_path: {}", repo_path.display());
    if let Some(parent) = repo_path.parent() {
        println!("repo_path.parent(): {}", parent.display());
    }
    
    // 現在のディレクトリとその内容を確認
    if let Ok(entries) = std::fs::read_dir(&repo_path) {
        println!("Contents of repo_path:");
        for entry in entries.flatten() {
            println!("  {}", entry.file_name().to_string_lossy());
        }
    }
    
    if let Some(parent) = repo_path.parent() {
        if let Ok(entries) = std::fs::read_dir(parent) {
            println!("Contents of parent:");
            for entry in entries.flatten() {
                println!("  {}", entry.file_name().to_string_lossy());
            }
        }
    }
    
    // コマンドが成功することを確認
    if result.is_err() {
        // もし失敗した場合もディレクトリ状況を表示
        println!("Command failed: {:?}", result);
    }
    
    // .gardener ディレクトリが .git と同じ階層に作成されることを確認
    let expected_gardener_dir = repo_path.join(".gardener");
    println!("Checking for .gardener at: {}", expected_gardener_dir.display());
    
    // 一つ上の階層に作成されていないか確認
    if let Some(parent) = repo_path.parent() {
        let wrong_gardener_dir = parent.join(".gardener");
        println!("Checking wrong location: {}", wrong_gardener_dir.display());
        if wrong_gardener_dir.exists() {
            println!("ERROR: .gardener created at wrong location!");
        }
    }
    
    assert!(result.is_ok(), "Add command should succeed: {:?}", result);
    
    assert!(expected_gardener_dir.exists(), 
        ".gardener directory should exist at: {}", 
        expected_gardener_dir.display()
    );
    
    // worktree が .gardener 内に作成されることを確認
    let expected_worktree_path = expected_gardener_dir.join("feature-test");
    assert!(expected_worktree_path.exists(),
        "Worktree should be created at: {}",
        expected_worktree_path.display()
    );
    
    // .worktree ディレクトリが作成されていないことを確認
    let wrong_worktree_dir = repo_path.join(".worktree");
    assert!(!wrong_worktree_dir.exists(),
        ".worktree directory should NOT exist at: {}",
        wrong_worktree_dir.display()
    );
    
    // 一つ上の階層に .gardener が作成されていないことを確認
    if let Some(parent) = repo_path.parent() {
        let wrong_gardener_dir = parent.join(".gardener");
        assert!(!wrong_gardener_dir.exists(),
            ".gardener should NOT be created at parent level: {}",
            wrong_gardener_dir.display()
        );
    }
}

#[test]
fn test_worktree_directory_name_is_gardener_not_worktree() {
    let temp_dir = tempfile::tempdir().unwrap();
    let repo_path = create_test_git_repo(&temp_dir);
    
    // テスト対象のディレクトリに移動
    std::env::set_current_dir(&repo_path).unwrap();
    
    let add_command = AddCommand::new(
        "feature/unique-test".to_string(),
        None,
        Some("main".to_string()),
        true,
    );
    
    let result = add_command.execute();
    
    // デバッグ情報を出力
    if let Err(ref e) = result {
        println!("Add command failed with error: {:?}", e);
    }
    
    assert!(result.is_ok(), "Add command should succeed: {:?}", result);
    
    // .gardener ディレクトリが作成されることを確認
    let gardener_dir = repo_path.join(".gardener");
    assert!(gardener_dir.exists(), ".gardener directory should be created");
    
    // .worktree ディレクトリが作成されないことを確認
    let worktree_dir = repo_path.join(".worktree");
    assert!(!worktree_dir.exists(), ".worktree directory should NOT be created");
}