# git-gardener

[🇺🇸 English](README.md)

Git worktreeの管理を簡単にする強力なツールです。並行開発ワークフローを効率化します。

## 概要

`git-gardener`は、Git worktreeの管理を簡単にします。複数の機能を同時に開発したり、異なるブランチをテストしたり、並行開発環境を維持したりする場合に、git-gardenerがワークフロー全体を効率化します。

## 特徴

- **簡単なWorktree作成**: 最小限のコマンドでworktreeを作成、.gardenerフォルダの自動セットアップ
- **インテリジェントなクリーンアップ**: マージ済みや古いworktreeを自動検出・削除
- **完全なWorktree管理**: 専用コマンドでworktreeの削除、プルーン、移動が可能
- **スマートタブ補完**: `-b`フラグ使用時のブランチ名補完とコンテキスト補完
- **並列操作**: 並列処理で全worktreeを同時にpull
- **対話式TUI**: フル機能のターミナルインターフェースでディレクトリナビゲーション
- **柔軟な設定**: TOMLファイルによるリポジトリごとのカスタマイズ、.gitignore自動管理
- **スマート検出**: マージ済みブランチや古いコミットを自動識別

## インストール

### Nixを使用（推奨）

Nixフレークを使用した再現可能なビルド:

```bash
# リポジトリから直接インストール
nix profile install github:mei28/git-gardener

# またはクローンしてローカルでビルド
git clone https://github.com/mei28/git-gardener.git
cd git-gardener
nix build
./result/bin/git-gardener --help
```

### Nixでの開発環境

```bash
# 依存関係を含む開発シェルに入る
nix develop

# またはdirenvで自動環境読み込み
echo "use flake" > .envrc
direnv allow
```

### ソースからビルド

```bash
git clone https://github.com/your-username/git-gardener.git
cd git-gardener
cargo build --release
cp target/release/git-gardener /usr/local/bin/
```

### シェル補完
コマンドとworktree名のタブ補完を有効にします：

#### オプション1: 内蔵completionコマンドを使用（Nixユーザー推奨）

```bash
# シェル用の補完を生成・インストール
git-gardener completion bash > ~/.local/share/bash-completion/completions/git-gardener
git-gardener completion zsh > ~/.local/share/zsh/site-functions/_git-gardener
git-gardener completion fish > ~/.config/fish/completions/git-gardener.fish

# またはパイプで直接インストール
# Bash用:
git-gardener completion bash | sudo tee /etc/bash_completion.d/git-gardener

# Zsh用（.zshrcに追加）:
mkdir -p ~/.local/share/zsh/site-functions
git-gardener completion zsh > ~/.local/share/zsh/site-functions/_git-gardener
echo "fpath=(~/.local/share/zsh/site-functions \$fpath)" >> ~/.zshrc
echo "autoload -U compinit && compinit" >> ~/.zshrc

# Fish用:
mkdir -p ~/.config/fish/completions
git-gardener completion fish > ~/.config/fish/completions/git-gardener.fish
```

#### オプション2: インストールスクリプトを使用（開発・ソースビルド用）

```bash
# シェル用の補完をインストール
./scripts/install-completions.sh

# または特定のシェル用にインストール
./scripts/install-completions.sh --bash   # Bash用
./scripts/install-completions.sh --zsh    # Zsh用
./scripts/install-completions.sh --fish   # Fish用
./scripts/install-completions.sh --all    # 全シェル用
```

**手動インストール:**
- **Bash**: `completions/git-gardener.bash`を`.bashrc`でsource
- **Zsh**: `completions/git-gardener.zsh`を`_git-gardener`として`fpath`にコピー
- **Fish**: `completions/git-gardener.fish`を`~/.config/fish/completions/`にコピー

**機能:**
- 全コマンドとオプションのタブ補完
- `-b <TAB>`フラグ使用時のブランチ名自動補完
- `git-gardener cd <TAB>`でworktree名の自動補完
- コマンドごとのスマートなコンテキスト補完

### 必要な環境

- Nix（推奨、フレーク機能有効）、または
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
git-gardener list --all         # プルーンされたworktreeも含む
git-gardener list --names-only  # worktree名のみを出力（シェル補完用）
```

**オプション:**
- `-a, --all`: プルーン可能なworktreeも含めてすべて表示
- `--names-only`: worktree名のみを出力（シェル補完に便利）

### `git-gardener cd`
特定のworktreeのパスを出力します（シェルナビゲーションに便利）。

```bash
# worktreeのパスを取得
git-gardener cd feature-auth

# シェルと組み合わせて移動
cd $(git-gardener cd feature-auth)
```

**シェルエイリアスでの使用例:**
```bash
# .bashrcや.zshrcに追加
alias gcd='cd $(git-gardener cd "$1")'

# その後、以下のように使用
gcd feature-auth
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
ビジュアルworktree管理のための対話式ターミナルインターフェースを起動します。

```bash
git-gardener tui
```

**TUI機能:**
- **ビジュアルナビゲーション**: `j/k`キーまたは矢印キーでworktreeを移動
- **クイックアクション**: 
  - `a` - ブランチ入力で新規worktreeを作成
  - `d` - 選択されたworktreeを削除（確認あり）
  - `p` - 選択されたworktreeで最新変更をpull
  - `c` - worktreeをクリーンアップ（merged/staleオプション選択）
  - `n` - 選択されたworktreeに移動（cdパスを表示）
  - `Enter` - 選択されたworktreeディレクトリに移動（cdパスを出力）
- **リアルタイムステータス**: worktreeの状態を表示（Clean、Dirty、Ahead、Behind、Diverged）
- **スマートクリーンアップ**: クリーンアップ条件の対話的選択

### `git-gardener remove`
特定のworktreeを削除します。

```bash
# worktreeを安全に削除
git-gardener remove feature-auth

# 未コミットの変更があっても強制削除
git-gardener remove feature-auth --force
```

**オプション:**
- `-f, --force`: 未コミットの変更があっても強制削除

### `git-gardener prune`
削除されたディレクトリのworktreeレコードを削除します。

```bash
git-gardener prune
```

### `git-gardener move`
worktreeを新しい場所に移動します。

```bash
# worktreeを新しいパスに移動
git-gardener move feature-auth ../new-location/feature-auth
```

### `git-gardener completion`
シェル補完スクリプトを生成します。

```bash
# 特定のシェル用の補完を生成
git-gardener completion bash
git-gardener completion zsh
git-gardener completion fish
```

**使用方法:**
- 補完スクリプトを標準出力に出力
- 適切な補完ディレクトリにリダイレクト可能
- 主要シェル（Bash、Zsh、Fish）に対応

### `git-gardener init`
git-gardener設定ファイルを初期化し、.gardenerフォルダを作成します。

```bash
git-gardener init
git-gardener init --force  # 既存の設定を上書き
```

**機能:**
- worktree用の`.gardener`ディレクトリを作成
- `.gardener/`を`.gitignore`に自動追加
- デフォルト設定ファイルを生成

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

# またはTUIでビジュアル管理
git-gardener tui
# 'a'キーを押して対話的に新しいworktreeを作成
```

### リリース管理
```bash
# 異なるバージョンの個別環境を維持
git-gardener add -b release/v1.2 --path ../release-v1.2
git-gardener add -b hotfix/security --path ../hotfix
git-gardener add -b develop --path ../develop

# 全環境を最新に保つ
git-gardener pull-all

# またはTUIで全worktreeをビジュアル管理・更新
git-gardener tui
# 各worktreeで'p'キーを押すか、pull-allを使用
```

### クリーンアップワークフロー
```bash
# 定期メンテナンス
git-gardener clean --merged  # マージ済み機能ブランチを削除
git-gardener clean --stale 7  # 1週間非アクティブなブランチを削除

# TUIでの対話的クリーンアップ
git-gardener tui
# 'c'キーを押してクリーンアップオプションを対話的に選択
# 'merged'や'stale'条件を選択
```

### エディタ連携ワークフロー
```bash
# お好みのエディタを設定
git-gardener config set defaults.editor "code ${WORKTREE_PATH}"

# TUIでworktreeを素早く開く
git-gardener tui
# 任意のworktreeに移動してEnterキーでエディタで開く
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