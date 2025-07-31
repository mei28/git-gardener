# git-gardener

シンプルなGit worktree管理ツール

## 概要

**git-gardener**は、Git worktreeの管理を簡単にするRust製のCLIツールです。[wtp](https://github.com/satococoa/wtp)にインスパイアされ、直感的で使いやすいインターフェースを提供します。

## 特徴

- **シンプルな操作**: 5つの基本コマンドでworktreeを管理
- **自動パス生成**: `.gardener/branch-name` 形式で自動的にworktreeを配置
- **フック機能**: worktree作成後の自動化処理（ファイルコピー、コマンド実行）
- **@記号での移動**: `git-gardener cd @` でメインworktreeに瞬時に移動

## インストール

```bash
# Cargoから（予定）
cargo install git-gardener

# ソースからビルド
git clone https://github.com/username/git-gardener
cd git-gardener
cargo build --release
```

## 使用方法

### 基本コマンド

```bash
# 新しいブランチでworktreeを作成
git-gardener add feature/new-feature -b

# 既存ブランチからworktreeを作成
git-gardener add existing-branch

# worktree一覧を表示
git-gardener list

# worktreeに移動（パスを出力）
git-gardener cd feature/new-feature

# メインworktreeに移動
git-gardener cd @

# worktreeを削除
git-gardener remove feature/new-feature

# ブランチも一緒に削除
git-gardener remove feature/new-feature --with-branch
```

### 設定ファイル

プロジェクトルートに `.gardener.yml` を作成して、カスタム設定やフックを定義できます。

```yaml
version: "1.0"
defaults:
  base_dir: ".gardener"

hooks:
  post_create:
    - type: copy
      from: ".env.example"
      to: ".env"
    
    - type: command
      command: "npm install"
      env:
        NODE_ENV: "development"
```

### フック機能

#### copyフック
ファイルやディレクトリをworktreeに自動コピー

#### commandフック
worktree作成後に任意のコマンドを実行

環境変数も利用可能：
- `${WORKTREE_PATH}`: 作成されたworktreeのパス
- `${BRANCH}`: ブランチ名
- `${REPO_ROOT}`: リポジトリのルートパス

## 開発

### 必要なツール

- Rust 1.75以降
- Git
- [just](https://github.com/casey/just)（推奨）

### タスク実行

justを使用してタスクを実行：

```bash
# 利用可能なタスクを表示
just

# ビルド
just build

# テスト実行
just test

# フォーマット + リント + テスト
just check

# 特定のテストを実行
just test-module "commands::add"
```

### TDD（テスト駆動開発）

このプロジェクトはt-wada流TDDで開発されています：

```bash
# 🔴 Red: 失敗するテストを書く
just test-red "test_new_feature"

# 🟢 Green: テストを通す最小実装
just test-green "test_new_feature"

# 🔵 Refactor: リファクタリング
just test-refactor
```

### プロジェクト構造

```
git-gardener/
├── src/
│   ├── commands/        # コマンド実装
│   │   ├── add.rs
│   │   ├── cd.rs
│   │   ├── list.rs
│   │   └── remove.rs
│   ├── config.rs        # YAML設定ファイル処理
│   ├── git/             # Git操作
│   │   └── worktree.rs
│   ├── hooks.rs         # フック機能
│   └── error.rs         # エラー型定義
├── .gardener.yml        # 設定ファイル例
├── justfile             # タスクランナー
└── README.md
```

## 貢献

1. このリポジトリをフォーク
2. フィーチャーブランチを作成 (`git-gardener add feature/new-feature -b`)
3. テストを書いて実装
4. テストが通ることを確認 (`just check`)
5. プルリクエストを作成

## ライセンス

MIT License