# git-gardener development tasks

# デフォルトタスク
default:
    @just --list

# ビルド
build:
    cargo build

# リリースビルド
release:
    cargo build --release

# テスト実行
test:
    cargo test

# フォーマット
fmt:
    cargo fmt

# リント
lint:
    cargo clippy -- -D warnings

# フォーマットとリントを実行
check: fmt lint

# 開発モードで実行
run *args:
    cargo run -- {{args}}

# ドキュメント生成
doc:
    cargo doc --open

# クリーンアップ
clean:
    cargo clean
    rm -rf .gardener/
    rm -rf test-repo/

# インストール（ローカル）
install:
    cargo install --path .

# アンインストール
uninstall:
    cargo uninstall git-gardener