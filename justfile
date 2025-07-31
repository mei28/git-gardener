# git-gardener justfile
# タスクランナーとしてjustを使用

# デフォルトレシピを表示
default:
    just --list

# ビルド
build:
    cargo build

# リリースビルド
build-release:
    cargo build --release

# テスト実行
test:
    cargo test

# 特定のテストを実行
test-module module:
    cargo test {{module}} -- --nocapture

# フォーマット
fmt:
    cargo fmt

# リント
lint:
    cargo clippy

# フォーマット + リント + テスト
check: fmt lint test

# 開発用の監視モード
watch:
    cargo watch -x test

# クリーンビルド
clean:
    cargo clean

# ドキュメント生成
doc:
    cargo doc --open

# インストール（ローカル）
install:
    cargo install --path .

# 全てのコマンドテストを実行
test-commands:
    cargo test commands -- --nocapture

# TDD用 - 失敗するテストを実行
test-red pattern:
    cargo test {{pattern}} -- --nocapture

# TDD用 - グリーンフェーズ（テストが通るまで実行）
test-green pattern:
    cargo test {{pattern}} -- --nocapture

# TDD用 - リファクタリング後のテスト
test-refactor:
    just check

# git-gardener コマンドをテスト
demo:
    cargo build
    ./target/debug/git-gardener --help

# プロジェクトの初期化をテスト
demo-init:
    cargo build
    ./target/debug/git-gardener init --help
    
# worktreeの作成をテスト（test-branchを作成）
demo-add:
    cargo build
    ./target/debug/git-gardener add test-branch -b

# worktree一覧を表示
demo-list:
    cargo build
    ./target/debug/git-gardener list

# テスト用worktreeを削除
demo-clean:
    cargo build
    ./target/debug/git-gardener remove test-branch || true
    git branch -D test-branch || true