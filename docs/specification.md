# git-gardener 仕様書

## 1. 目的・背景

| 項目 | 内容 |
|------|------|
| 解決したい課題 | - git worktree コマンドの冗長さ<br>- 作成後の環境構築・IDE 起動など手作業の重複<br>- どの worktree がどこにあるか把握しづらい<br>- 使い終わったワークツリーの安全な掃除 |
| 提供価値 | - 最小入力でワークツリー作成・移動・削除<br>- TUI で一覧を視覚化（状態・ブランチ・更新日など）<br>- フック機構により各開発者／リポジトリ固有の初期化を自動化<br>- 並列処理で複数 worktree への git pull や掃除を高速化 |

## 2. 想定ユーザー・ユースケース

- 複数機能開発を並行する Web/モバイルエンジニア
- モノレポでブランチ切替コストが大きいチーム
- CI/CD 検証用に短命ブランチを大量に作成する運用担当

## 3. コマンドラインインターフェース

### 3.1 ベース構造（clap v4）

```
git‑gardener <SUBCOMMAND> [OPTIONS]
```

| サブコマンド | 役割 | 主要オプション |
|-------------|------|----------------|
| `add` | 新規 worktree 作成 | `-b, --branch <name>`（既存 or 新規）<br>`-p, --path <dir>`（省略時は `.gardener/<branch>`）<br>`--upstream <remote>`（自動 push 先設定） |
| `go` | (実装はあとでやる、低優先度)既存 worktree へ移動 or IDE 起動 | `--code` (VS Code 起動)<br>`--shell` (cd して $SHELL) |
| `list` | 登録 worktree 一覧表示 | `-a/--all`（prune 済みを含む） |
| `clean` | 条件付き一括削除 | `--merged` / `--stale <days>` / `--force` |
| `pull-all` | 全 worktree で git pull | `--parallel <N>`（デフォルト CPU コア数） |
| `config` | 設定確認・編集 | `config set <key> <value>` / `config view` |
| `tui` | 対話式 UI 起動 | `--fullscreen` / `--no-mouse` |
| `init` | 初期化(configfileの作成) | (未定) |

**注**: `tui` サブコマンドは add/go/list/clean の機能をすべて内包した対話式画面を提供

## 4. TUI 仕様（ratatui + crossterm）

```
┌─────────────────────────────────────────────────────────────┐
│ git‑gardener — Worktree Dashboard               [q] quit   │
├─────────────────────────────────────────────────────────────┤
│   BRANCH        PATH                     STATUS   UPDATED   │
│ > feature/auth  .gardener/feature/auth   ✔ Clean  3 h ago   │
│   bugfix/login  .gardener/bugfix/login   ✗ Dirty  10 m ago  │
│   release/1.2   ../release–1.2           ▲ Ahead  1 d ago   │
├─────────────────────────────────────────────────────────────┤
│ [a] add   [Enter] open   [d] delete   [p] pull   [c] clean │
└─────────────────────────────────────────────────────────────┘
```

**ナビゲーション**: j/k or ↓/↑ で移動、g/G で先頭/末尾

**アクションキー**:
- `a`: 新規作成ダイアログ（ブランチ入力 → パス自動補完）
- `Enter`: cd + オプションでエディタ起動 (`config.editor`)
- `d`: ゴミ箱アイコン表示 → y で削除確定
- `p`: 選択行または Shift+p で全 worktree pull
- `c`: clean 条件を対話設定（merged/stale など）

**ステータス色分け**:
- ✔ Clean: 緑
- ✗ Dirty: 黄
- ▲ Ahead / ▼ Behind: 青/赤

リサイズ対応・マウス scroll 対応（任意）

## 5. 設定ファイル (`.git/gardener.toml` 1 リポジトリ 1 枚)

```toml
[defaults]
# worktree の格納パスルート
root_dir = ".gardener"

# 作成後に実行するスクリプト（posix shell）
post_create = [
  "cp .env.example ${WORKTREE_PATH}/.env",
  "pnpm install --frozen-lockfile"
]

# go 時に IDE を開く場合のコマンド
editor = "code ${WORKTREE_PATH}"

[branches."release/*"]
post_create = [
  "make db:migrate",
  "docker compose up -d"
]
```

**環境変数**: `WORKTREE_PATH`, `BRANCH`, `REPO_ROOT` などを注入

**ワイルドカードセクション**でブランチパターン別のフックを上書き

## 6. 内部アーキテクチャ

```
src/
├─ cli.rs         # clap 定義 → Command enum
├─ tui/
│   ├─ app.rs     # 状態管理 (worktree list, selected idx, mode)
│   ├─ ui.rs      # ratatui 描画
│   └─ handler.rs # キーバインド → Cmd
├─ git/
│   ├─ worktree.rs# git2-rs を薄くラップ
│   └─ status.rs  # Dirty/Ahead 等を高速判定 (libgit2 status flags)
├─ hooks.rs       # post_create 等シェル呼び出し＋ログ
├─ config.rs      # serde + figment で TOML ロード
├─ tasks.rs       # 並列 pull/clean 実装 (rayon / tokio blocking)
└─ util.rs
```

- **git 操作**: git2 crate（libgit2 バインディング）
- **並列実行**: CPU バウンドは rayon; IO/CPU 混合は tokio + indicatif 進捗バー
- **ログ**: tracing + tracing‑subscriber（レベル可変）

## 7. 依存クレート候補

| 分類 | クレート | 理由 |
|------|---------|------|
| CLI | clap v4, clap_complete | サブコマンド + シェル補完生成 |
| Git | git2 / gix | パフォーマンス観点で比較検討 |
| TUI | ratatui, crossterm | ノンブロッキング入力・カラースタイリング |
| 並列 | rayon, tokio, indicatif | 並列 pull・プログレスバー |
| 設定 | serde, toml_edit, figment | TOML パースと動的マージ |
| テスト | assert_cmd, predicates, rstest | CLI テストとスナップショット |
| 配布 | cargo‑dist | GitHub Releases + Homebrew Tap 自動生成 |

## 8. エラーハンドリング方針

| レイヤ | 例 | 振る舞い |
|--------|---|----------|
| CLI 引数 | 無効オプション | clap の built‑in help + exit = 2 |
| Git 操作 | ブランチ存在しない | ユーザー向け赤字メッセージ；ログ DEBUG |
| フック実行 | スクリプト失敗 | exit code 伝搬・stderr をログ／TUI モーダル |
| 並列 pull | 部分的失敗 | 失敗数を要約し exit = 1; --fail-fast で即終了可 |

## 9. テスト戦略

**ユニットテスト**
- git 仮想リポジトリを tempdir + git2::Repository::init で生成

**統合テスト (assert_cmd)**
- `git‑gardener add/go/clean` を実行し副作用を検証

**TUI スナップショット**
- ratatui-test で描画バッファを比較

**CI**: GitHub Actions (ubuntu-latest, macos-latest, windows-latest)

## 10. ビルド・配布

**ターゲット**: x86_64-unknown-linux-gnu, aarch64-apple-darwin, x86_64-pc-windows-msvc

**インストール方法**:
- `cargo install git-gardener`
- Homebrew: `brew install <tap>/git-gardener`
- Pre‑built binaries（GH Release）

**バージョニング**: SemVer + conventional commits

## 11. 今後拡張アイデア（Backlog）

| 優先 | 機能 | 説明 |
|------|------|------|
| ★★★ | Fuzzy 式 branch picker | skim/obliterate 風で add 入力を快適に |
| ★★☆ | GitHub PR 作成 | 新規 branch から PR 雛形を自動オープン |
| ★★☆ | グローバル設定 | $HOME/.config/git-gardener/config.toml |
| ★☆☆ | プラグイン SDK | post_create 以外に Rust で書けるフック |

## 12. ロードマップ（概算）

| フェーズ | 期間 | マイルストーン |
|----------|------|----------------|
| 0. PoC | 1 週 | add/go/list の CLI 動作 & 基本 config 読込 |
| 1. CLI α | 2–3 週 | clean/pull-all 実装・単体テスト網羅 |
| 2. TUI α | 2 週 | ratatui 画面・基本操作 (list, go) |
| 3. β版 | 3 週 | フック・並列処理 |
| 4. v1.0 | 1 週 | ドキュメント整備・配布パイプライン |
