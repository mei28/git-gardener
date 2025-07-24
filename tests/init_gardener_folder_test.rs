use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_init_creates_gardener_folder() {
    // 🔴 RED: initコマンドが.gardenerフォルダを作成することをテスト
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
    
    // .gardenerフォルダが作成されていることを確認
    let gardener_dir = temp_dir.path().join(".gardener");
    assert!(gardener_dir.exists(), ".gardener directory should be created");
    assert!(gardener_dir.is_dir(), ".gardener should be a directory");
}

#[test]
fn test_init_adds_gardener_to_gitignore() {
    // 🔴 RED: initコマンドが.gitignoreに.gardener/を追加することをテスト
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
    
    // .gitignoreファイルの内容を確認
    let gitignore_path = temp_dir.path().join(".gitignore");
    assert!(gitignore_path.exists(), ".gitignore file should be created or modified");
    
    let gitignore_content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(
        gitignore_content.contains(".gardener/"),
        ".gitignore should contain .gardener/ entry"
    );
}

#[test]
fn test_init_does_not_duplicate_gitignore_entry() {
    // 🔴 RED: 既に.gardener/が.gitignoreにある場合、重複して追加しないことをテスト
    let temp_dir = tempdir().unwrap();
    
    // gitリポジトリを初期化
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // 既に.gitignoreに.gardener/を含むファイルを作成
    let gitignore_path = temp_dir.path().join(".gitignore");
    fs::write(&gitignore_path, ".gardener/\nnode_modules/\n").unwrap();
    
    // git-gardener initを実行
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // .gitignoreの内容を確認
    let gitignore_content = fs::read_to_string(&gitignore_path).unwrap();
    let gardener_count = gitignore_content.matches(".gardener/").count();
    assert_eq!(
        gardener_count, 1,
        ".gardener/ should appear only once in .gitignore"
    );
}

#[test]
fn test_init_appends_to_existing_gitignore() {
    // 🔴 RED: 既存の.gitignoreがある場合、内容を保持して追記することをテスト
    let temp_dir = tempdir().unwrap();
    
    // gitリポジトリを初期化
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // 既存の.gitignoreを作成
    let gitignore_path = temp_dir.path().join(".gitignore");
    fs::write(&gitignore_path, "node_modules/\n*.log\n").unwrap();
    
    // git-gardener initを実行
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // .gitignoreの内容を確認
    let gitignore_content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(gitignore_content.contains("node_modules/"), "Existing content should be preserved");
    assert!(gitignore_content.contains("*.log"), "Existing content should be preserved");
    assert!(gitignore_content.contains(".gardener/"), ".gardener/ should be added");
}

#[test]
fn test_init_force_overwrites_config_but_preserves_folder() {
    // 🔴 RED: --forceオプションでも.gardenerフォルダは削除されないことをテスト
    let temp_dir = tempdir().unwrap();
    
    // gitリポジトリを初期化
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // 最初のinit実行
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // .gardenerフォルダ内にファイルを作成
    let gardener_dir = temp_dir.path().join(".gardener");
    let test_file = gardener_dir.join("test.txt");
    fs::write(&test_file, "test content").unwrap();
    
    // --forceでinit実行
    Command::cargo_bin("git-gardener")
        .unwrap()
        .args(&["init", "--force"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // .gardenerフォルダとその中のファイルが保持されていることを確認
    assert!(gardener_dir.exists(), ".gardener directory should still exist");
    assert!(test_file.exists(), "Files in .gardener should be preserved");
}