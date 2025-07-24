# git-gardener

[🇺🇸 English](README.md)

Git worktreeの管理を簡単にする強力なツールです。並行開発ワークフローを効率化します。

## 概要

`git-gardener`は、Git worktreeの管理を簡単にします。複数の機能を同時に開発したり、異なるブランチをテストしたり、並行開発環境を維持したりする場合に、git-gardenerがワークフロー全体を効率化します。

## 特徴

- **簡単なWorktree作成**: 最小限のコマンドでworktreeを作成
- **インテリジェントなクリーンアップ**: マージ済みや古いworktreeを自動検出・削除
- **並列操作**: 並列処理で全worktreeを同時にpull
- **対話式UI**: ビジュアルなworktree管理のためのターミナルベースのインターフェース
- **柔軟な設定**: TOMLファイルによるリポジトリごとのカスタマイズ
- **スマート検出**: マージ済みブランチや古いコミットを自動識別

## インストール

### ソースからビルド

```bash
git clone https://github.com/your-username/git-gardener.git
cd git-gardener
cargo build --release
cp target/release/git-gardener /usr/local/bin/
```

### 必要な環境

- Rust 1.75以降
- Git

## クイックスタート

1. **リポジトリでgit-gardenerを初期化:**
   ```bash
   git-gardener init
   ```

2. **新しいworktreeを作成:**
   ```bash
   git-gardener add -b feature/new-feature
   ```

3. **全worktreeを一覧表示:**
   ```bash
   git-gardener list
   ```

4. **全worktreeを並列でpull:**
   ```bash
   git-gardener pull-all
   ```

5. **マージ済みworktreeをクリーンアップ:**
   ```bash
   git-gardener clean --merged
   ```

## コマンド

### `git-gardener add`
指定したブランチの新しいworktreeを作成します。

```bash
# 既存ブランチのworktreeを作成
git-gardener add -b feature/auth

# 新しいブランチでworktreeを作成
git-gardener add -b feature/new-feature --create-branch

# カスタムパスを指定
git-gardener add -b hotfix/urgent --path ../hotfix-urgent
```

**オプション:**
- `-b, --branch <BRANCH>`: ブランチ名（必須）
- `-p, --path <PATH>`: カスタムworktreeパス（デフォルト: `.gardener/<branch>`）
- `-c, --create-branch`: 新しいブランチを作成
- `--upstream <UPSTREAM>`: 上流リモートを設定

### `git-gardener list`
全worktreeをステータス情報とともに表示します。

```bash
git-gardener list
git-gardener list --all  # プルーンされたworktreeも含む
```

### `git-gardener clean`
指定した条件に基づいてworktreeを削除します。

```bash
# マージ済みworktreeを削除
git-gardener clean --merged

# 過去30日間コミットがないworktreeを削除
git-gardener clean --stale 30

# 全worktreeを強制削除（危険！）
git-gardener clean --force
```

**オプション:**
- `--merged`: マージ済みブランチのworktreeを削除
- `--stale <DAYS>`: N日より古いworktreeを削除
- `--force`: 全worktreeを強制削除

### `git-gardener pull-all`
全worktreeの最新変更を同時にpullします。

```bash
git-gardener pull-all
git-gardener pull-all --parallel 8  # 8つの並列ジョブを使用
```

**オプション:**
- `--parallel <N>`: 並列ジョブ数（デフォルト: CPUコア数）

### `git-gardener config`
設定の表示・変更を行います。

```bash
# 現在の設定を表示
git-gardener config view

# 設定値を変更
git-gardener config set defaults.root_dir .worktrees
git-gardener config set defaults.editor "code"
```

### `git-gardener tui`
対話式ターミナルインターフェースを起動します。

```bash
git-gardener tui
```

### `git-gardener init`
git-gardener設定ファイルを初期化します。

```bash
git-gardener init
git-gardener init --force  # 既存の設定を上書き
```

## 設定

git-gardenerは、リポジトリごとの設定に`.git/gardener.toml`設定ファイルを使用します。

### 設定例

```toml
[defaults]
# worktreeのルートディレクトリ
root_dir = ".gardener"

# worktree作成後に実行するコマンド
post_create = [
    "cp .env.example ${WORKTREE_PATH}/.env",
    "npm install"
]

# デフォルトエディタコマンド
editor = "code ${WORKTREE_PATH}"

[branches]
# ブランチ固有の設定をここに追加できます
```

### 設定オプション

- `defaults.root_dir`: worktree作成時のデフォルトディレクトリ
- `defaults.post_create`: worktree作成後に実行するシェルコマンド
- `defaults.editor`: worktreeを開くためのエディタコマンド

## 使用例

### 並行機能開発
```bash
# 複数の機能ブランチをセットアップ
git-gardener add -b feature/auth --create-branch
git-gardener add -b feature/payment --create-branch
git-gardener add -b feature/dashboard --create-branch

# 異なる機能を同時に作業
# 各worktreeは独自の作業ディレクトリで分離されています
```

### リリース管理
```bash
# 異なるバージョンの個別環境を維持
git-gardener add -b release/v1.2 --path ../release-v1.2
git-gardener add -b hotfix/security --path ../hotfix
git-gardener add -b develop --path ../develop

# 全環境を最新に保つ
git-gardener pull-all
```

### クリーンアップワークフロー
```bash
# 定期メンテナンス
git-gardener clean --merged  # マージ済み機能ブランチを削除
git-gardener clean --stale 7  # 1週間非アクティブなブランチを削除
```

## 開発

### ソースからビルド

```bash
git clone https://github.com/your-username/git-gardener.git
cd git-gardener
cargo build --release
```

### テスト実行

```bash
cargo test
```

### プロジェクト構造

```
git-gardener/
├── src/
│   ├── main.rs          # エントリーポイント
│   ├── cli.rs           # CLIコマンド定義
│   ├── commands/        # コマンド実装
│   ├── git/             # Git操作
│   ├── config.rs        # 設定処理
│   └── error.rs         # エラー型
├── tests/               # 統合テスト
└── docs/                # ドキュメント
```

## コントリビューション

1. リポジトリをフォーク
2. 機能ブランチを作成: `git checkout -b feature/new-feature`
3. 変更を加えてテストを追加
4. テストを実行: `cargo test`
5. リントを実行: `cargo clippy`
6. コードをフォーマット: `cargo fmt`
7. 変更をコミット: `git commit -m 'Add new feature'`
8. ブランチにプッシュ: `git push origin feature/new-feature`
9. プルリクエストを送信

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細は[LICENSE](LICENSE)ファイルを参照してください。

## 謝辞

- [Rust](https://www.rust-lang.org/)で構築
- CLIは[clap](https://github.com/clap-rs/clap)を使用
- TUIは[ratatui](https://github.com/ratatui-org/ratatui)で構築
- Git操作は[git2](https://github.com/rust-lang/git2-rs)経由