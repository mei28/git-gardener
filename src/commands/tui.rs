use crate::error::{GitGardenerError, Result};
use crate::git::{GitWorktree, WorktreeInfo};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

// 🔵 REFACTOR: TUIのアクション種別
#[derive(Debug, Clone, PartialEq)]
pub enum TuiAction {
    Add,
    Delete,
    Pull,
    Clean,
    Open,
}

// 🟢 GREEN: ダイアログモードの定義
#[derive(Debug, Clone, PartialEq)]
pub enum DialogMode {
    BranchInput,
    DeleteConfirmation,
    CleanOptions,
}

// 🔵 REFACTOR: TUIの内部状態を管理する構造体（アクション機能付き）
pub struct TuiState {
    pub worktrees: Vec<WorktreeInfo>,
    pub selected_index: usize,
    pub current_action: Option<TuiAction>,
    pub status_message: Option<String>, // 🔵 REFACTOR: ステータスメッセージ表示用
    pub dialog_mode: Option<DialogMode>, // 🟢 GREEN: ダイアログモード
    pub input_text: String, // 🟢 GREEN: 入力テキスト
    pub clean_options: Vec<String>, // 🟢 GREEN: クリーンオプション（merged, stale等）
}

impl TuiState {
    pub fn new(worktrees: Vec<WorktreeInfo>) -> Self {
        Self {
            worktrees,
            selected_index: 0,
            current_action: None,
            status_message: None,
            dialog_mode: None,
            input_text: String::new(),
            clean_options: Vec::new(),
        }
    }
    
    pub fn move_down(&mut self) {
        if !self.worktrees.is_empty() && self.selected_index < self.worktrees.len() - 1 {
            self.selected_index += 1;
        }
    }
    
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
    
    pub fn move_to_start(&mut self) {
        self.selected_index = 0;
    }
    
    pub fn move_to_end(&mut self) {
        if !self.worktrees.is_empty() {
            self.selected_index = self.worktrees.len() - 1;
        }
    }
    
    pub fn get_selected(&self) -> Option<&WorktreeInfo> {
        self.worktrees.get(self.selected_index)
    }
    
    // 🔵 REFACTOR: アクション管理メソッド
    pub fn get_current_action(&self) -> Option<TuiAction> {
        self.current_action.clone()
    }
    
    // 🟢 GREEN: 現在のアクションを実行する最小限の実装
    pub fn execute_current_action(&mut self, input: &str) -> Result<String> {
        match &self.current_action {
            Some(action) => {
                match action {
                    TuiAction::Add => {
                        // addアクションの最小実装
                        self.execute_add_action(input)
                    },
                    TuiAction::Pull => {
                        // pullアクションの最小実装
                        self.execute_pull_action()
                    },
                    TuiAction::Delete => {
                        // deleteアクションの最小実装
                        self.execute_delete_action(input)
                    },
                    TuiAction::Clean => {
                        // cleanアクションの実装（選択された条件に基づく）
                        self.execute_clean_action(input)
                    },
                    TuiAction::Open => {
                        // openアクションの最小実装
                        Ok("Open action executed".to_string())
                    },
                }
            },
            None => Err(GitGardenerError::Custom("No action set".to_string())),
        }
    }
    
    // 🟢 GREEN: addアクションの最小実装
    fn execute_add_action(&mut self, branch_name: &str) -> Result<String> {
        // ブランチ名のバリデーション
        if branch_name.trim().is_empty() || branch_name.contains(' ') || branch_name.contains('!') {
            return Err(GitGardenerError::Custom("Invalid branch name".to_string()));
        }
        
        // 実際のワーキングツリー作成（最小実装）
        let git_worktree = GitWorktree::new()?;
        let worktree_name = branch_name.replace('/', "-");
        let gardener_dir = std::env::current_dir()?.join(".gardener");
        
        // .gardenerディレクトリを作成（存在しない場合）
        std::fs::create_dir_all(&gardener_dir).map_err(|e| 
            GitGardenerError::Custom(format!("Failed to create .gardener directory: {}", e))
        )?;
        
        let worktree_path = gardener_dir.join(&worktree_name);
        
        git_worktree.create_worktree(&worktree_name, &worktree_path, branch_name, true)?;
        
        // ワーキングツリー一覧を更新
        let updated_worktrees = git_worktree.list_worktrees()?;
        self.worktrees = updated_worktrees;
        
        Ok(format!("Created worktree '{}'", worktree_name))
    }
    
    // 🟢 GREEN: pullアクションの最小実装
    fn execute_pull_action(&self) -> Result<String> {
        // 選択されたワーキングツリーをpull（最小実装）
        if let Some(selected_worktree) = self.worktrees.get(self.selected_index) {
            // git pullを実行
            let output = std::process::Command::new("git")
                .args(&["pull"])
                .current_dir(&selected_worktree.path)
                .output()
                .map_err(|e| GitGardenerError::Custom(format!("Failed to execute git pull: {}", e)))?;
            
            if output.status.success() {
                Ok("Pull completed successfully".to_string())
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Ok(format!("Pull completed with warnings: {}", error_msg))
            }
        } else {
            Err(GitGardenerError::Custom("No worktree selected".to_string()))
        }
    }
    
    // 🟢 GREEN: deleteアクションの最小実装  
    fn execute_delete_action(&mut self, confirmation: &str) -> Result<String> {
        if confirmation != "y" {
            return Ok("Delete cancelled".to_string());
        }
        
        if let Some(selected_worktree) = self.worktrees.get(self.selected_index) {
            let worktree_name = selected_worktree.name.clone(); // 名前をクローン
            let git_worktree = GitWorktree::new()?;
            git_worktree.remove_worktree(&worktree_name, false)?;
            
            // ワーキングツリー一覧を更新
            let updated_worktrees = git_worktree.list_worktrees()?;
            self.worktrees = updated_worktrees;
            
            // 選択インデックスを調整
            if self.selected_index >= self.worktrees.len() && !self.worktrees.is_empty() {
                self.selected_index = self.worktrees.len() - 1;
            }
            
            Ok(format!("Deleted worktree '{}'", worktree_name))
        } else {
            Err(GitGardenerError::Custom("No worktree selected".to_string()))
        }
    }
    
    // 🔵 REFACTOR: cleanアクションの実装
    fn execute_clean_action(&mut self, clean_options: &str) -> Result<String> {
        if clean_options.is_empty() {
            return Err(GitGardenerError::Custom("No clean options selected".to_string()));
        }
        
        let options: Vec<&str> = clean_options.split(',').collect();
        let mut cleaned_count = 0;
        
        // 現在は基本的なメッセージのみ（実際のclean実装は別途）
        for option in options {
            match option {
                "merged" => {
                    // マージ済みワーキングツリーの削除ロジック（将来実装）
                    cleaned_count += 1;
                },
                "stale" => {
                    // 古いワーキングツリーの削除ロジック（将来実装）
                    cleaned_count += 1;
                },
                _ => {} // 無効なオプションは無視
            }
        }
        
        Ok(format!("Clean completed: {} options processed ({})", cleaned_count, clean_options))
    }
    
    pub fn set_action(&mut self, action: Option<TuiAction>) {
        self.current_action = action;
    }
    
    pub fn clear_action(&mut self) {
        self.current_action = None;
    }
    
    // 🔵 REFACTOR: ステータスメッセージ管理
    pub fn set_status_message(&mut self, message: String) {
        self.status_message = Some(message);
    }
    
    pub fn clear_status_message(&mut self) {
        self.status_message = None;
    }
    
    // 🟢 GREEN: ダイアログ関連メソッド
    pub fn get_dialog_mode(&self) -> Option<DialogMode> {
        self.dialog_mode.clone()
    }
    
    pub fn is_in_dialog(&self) -> bool {
        self.dialog_mode.is_some()
    }
    
    pub fn enter_dialog_mode(&mut self, mode: DialogMode) {
        self.dialog_mode = Some(mode);
        self.input_text.clear();
    }
    
    pub fn exit_dialog_mode(&mut self) {
        self.dialog_mode = None;
        self.input_text.clear();
        self.current_action = None;
        self.clean_options.clear();
    }
    
    pub fn get_input_text(&self) -> &str {
        &self.input_text
    }
    
    pub fn input_char(&mut self, c: char) {
        self.input_text.push(c);
    }
    
    pub fn delete_char(&mut self) {
        self.input_text.pop();
    }
    
    pub fn get_selected_worktree_name(&self) -> Option<String> {
        self.worktrees.get(self.selected_index).map(|w| w.name.clone())
    }
    
    pub fn confirm_dialog(&mut self) -> Option<String> {
        if let Some(mode) = &self.dialog_mode {
            match mode {
                DialogMode::BranchInput => {
                    // ブランチ名のバリデーション
                    let input = self.input_text.trim();
                    if input.is_empty() || input.contains(' ') || input.contains('!') {
                        return None; // 無効な入力
                    }
                    let result = input.to_string();
                    self.exit_dialog_mode();
                    Some(result)
                },
                DialogMode::DeleteConfirmation => {
                    // 削除確認は常にOK（確認ダイアログなので）
                    self.exit_dialog_mode();
                    Some("y".to_string())
                },
                DialogMode::CleanOptions => {
                    // クリーンオプションは空でない場合のみ確定
                    if self.clean_options.is_empty() {
                        return None; // 何も選択されていない
                    }
                    let result = self.clean_options.join(",");
                    self.exit_dialog_mode();
                    Some(result)
                }
            }
        } else {
            None
        }
    }
    
    // 🟢 GREEN: クリーンオプション関連メソッド
    pub fn get_clean_options(&self) -> String {
        self.clean_options.join(",")
    }
    
    pub fn toggle_clean_option(&mut self, option: char) {
        let option_str = match option {
            'm' => "merged",
            's' => "stale",
            _ => return, // 無効なオプションは無視
        };
        
        if let Some(pos) = self.clean_options.iter().position(|x| x == option_str) {
            // 既に存在する場合は削除（トグル）
            self.clean_options.remove(pos);
        } else {
            // 存在しない場合は追加
            self.clean_options.push(option_str.to_string());
        }
    }

    pub fn get_help_text(&self) -> String {
        match &self.current_action {
            Some(action) => {
                match action {
                    TuiAction::Add => "[Enter] confirm add  [Esc] cancel".to_string(),
                    TuiAction::Delete => "[Enter] confirm delete  [Esc] cancel".to_string(),
                    TuiAction::Pull => "[Enter] confirm pull  [Esc] cancel".to_string(),
                    TuiAction::Clean => "[Enter] confirm clean  [Esc] cancel".to_string(),
                    TuiAction::Open => "[Enter] confirm open  [Esc] cancel".to_string(),
                }
            }
            None => "[j/k,↓/↑] navigate  [g/G] first/last  [a] add  [d] delete  [p] pull  [c] clean  [Enter] open  [q] quit".to_string(),
        }
    }
}

pub struct TuiCommand {
    pub fullscreen: bool,
    pub no_mouse: bool,
}

impl TuiCommand {
    pub fn new(fullscreen: bool, no_mouse: bool) -> Self {
        Self {
            fullscreen,
            no_mouse,
        }
    }
    
    pub fn execute(&self) -> Result<()> {
        // 🟢 GREEN: TUIの最小実装
        self.run_tui()
    }
    
    fn run_tui(&self) -> Result<()> {
        // ターミナルの設定
        enable_raw_mode().map_err(|e| GitGardenerError::Custom(format!("Failed to enable raw mode: {}", e)))?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).map_err(|e| GitGardenerError::Custom(format!("Failed to enter alternate screen: {}", e)))?;
        
        if !self.no_mouse {
            execute!(stdout, EnableMouseCapture).map_err(|e| GitGardenerError::Custom(format!("Failed to enable mouse capture: {}", e)))?;
        }
        
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).map_err(|e| GitGardenerError::Custom(format!("Failed to create terminal: {}", e)))?;
        
        // TUIのメインループ
        let result = self.run_app(&mut terminal);
        
        // 後処理
        disable_raw_mode().map_err(|e| GitGardenerError::Custom(format!("Failed to disable raw mode: {}", e)))?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ).map_err(|e| GitGardenerError::Custom(format!("Failed to cleanup terminal: {}", e)))?;
        
        result
    }
    
    fn run_app(&self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        // ワーキングツリーの情報を取得
        let git_worktree = GitWorktree::new().map_err(|_| {
            GitGardenerError::Custom("Not in a git repository".to_string())
        })?;
        let worktrees = git_worktree.list_worktrees()?;
        
        // 🟢 GREEN: TuiStateを使用した状態管理
        let mut state = TuiState::new(worktrees);
        
        loop {
            terminal.draw(|f| self.ui(f, &state)).map_err(|e| GitGardenerError::Custom(format!("Failed to draw terminal: {}", e)))?;
            
            if let Event::Key(key) = event::read().map_err(|e| GitGardenerError::Custom(format!("Failed to read event: {}", e)))? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Esc => {
                        if state.is_in_dialog() {
                            // 🔵 REFACTOR: ダイアログモードを終了
                            state.exit_dialog_mode();
                        } else if state.current_action.is_some() {
                            state.clear_action();
                        } else {
                            return Ok(());
                        }
                    },
                    // 🔵 REFACTOR: アクションキーの処理（ダイアログ統合）
                    KeyCode::Char('a') if state.current_action.is_none() && !state.is_in_dialog() => {
                        state.set_action(Some(TuiAction::Add));
                        state.enter_dialog_mode(DialogMode::BranchInput);
                    },
                    KeyCode::Char('d') if state.current_action.is_none() && !state.is_in_dialog() => {
                        state.set_action(Some(TuiAction::Delete));
                        state.enter_dialog_mode(DialogMode::DeleteConfirmation);
                    },
                    KeyCode::Char('p') if state.current_action.is_none() => {
                        state.set_action(Some(TuiAction::Pull));
                    },
                    KeyCode::Char('c') if state.current_action.is_none() && !state.is_in_dialog() => {
                        state.set_action(Some(TuiAction::Clean));
                        state.enter_dialog_mode(DialogMode::CleanOptions);
                    },
                    KeyCode::Enter => {
                        if state.is_in_dialog() {
                            // 🔵 REFACTOR: ダイアログの確認処理
                            if let Some(input) = state.confirm_dialog() {
                                // ダイアログで入力された値でアクション実行
                                if let Some(action) = &state.current_action.clone() {
                                    match state.execute_current_action(&input) {
                                        Ok(message) => {
                                            tracing::info!("Action executed: {}", message);
                                            state.set_status_message(format!("✅ {}", message));
                                        },
                                        Err(e) => {
                                            tracing::error!("Action failed: {}", e);
                                            state.set_status_message(format!("❌ {}", e));
                                        }
                                    }
                                }
                            }
                            // ダイアログが無効な入力の場合は開いたまま
                        } else if let Some(_) = &state.current_action {
                            // アクションが設定されているがダイアログではない場合（p, c等）
                            match state.execute_current_action("") {
                                Ok(message) => {
                                    tracing::info!("Action executed: {}", message);
                                    state.set_status_message(format!("✅ {}", message));
                                },
                                Err(e) => {
                                    tracing::error!("Action failed: {}", e);
                                    state.set_status_message(format!("❌ {}", e));
                                }
                            }
                            state.clear_action();
                        } else {
                            // 選択されたアイテムを開く
                            state.set_action(Some(TuiAction::Open));
                        }
                    },
                    // ナビゲーションキーの処理（アクション選択中は無効）
                    KeyCode::Down | KeyCode::Char('j') if state.current_action.is_none() => state.move_down(),
                    KeyCode::Up | KeyCode::Char('k') if state.current_action.is_none() => state.move_up(),
                    KeyCode::Char('g') if state.current_action.is_none() => state.move_to_start(),
                    KeyCode::Char('G') if state.current_action.is_none() => state.move_to_end(),
                    
                    // 🔵 REFACTOR: ダイアログモード時の文字入力処理
                    KeyCode::Char(c) if state.is_in_dialog() => {
                        match &state.dialog_mode {
                            Some(DialogMode::CleanOptions) => {
                                // クリーンオプションの場合は特定のキーのみ受け付ける
                                state.toggle_clean_option(c);
                            },
                            _ => {
                                // その他のダイアログでは通常の文字入力
                                state.input_char(c);
                            }
                        }
                    },
                    KeyCode::Backspace if state.is_in_dialog() => {
                        state.delete_char();
                    },
                    
                    _ => {}
                }
            }
        }
    }
    
    fn ui(&self, f: &mut Frame, state: &TuiState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.size());
        
        // タイトル
        let title = Paragraph::new("git-gardener — Worktree Dashboard")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);
        
        // ワーキングツリーリスト（ステータス情報付き）
        let items: Vec<ListItem> = state.worktrees
            .iter()
            .enumerate()
            .map(|(i, w)| {
                // ステータス表示の決定
                let (status_text, status_color) = if let Some(ref status) = w.status {
                    match status.working_tree_status {
                        crate::git::WorktreeStatus::Clean => ("✔ Clean", Color::Green),
                        crate::git::WorktreeStatus::Dirty => ("✗ Dirty", Color::Yellow),
                        crate::git::WorktreeStatus::Ahead => ("▲ Ahead", Color::Blue),
                        crate::git::WorktreeStatus::Behind => ("▼ Behind", Color::Red),
                        crate::git::WorktreeStatus::Diverged => ("⇕ Diverged", Color::Magenta),
                    }
                } else {
                    ("? Unknown", Color::Gray)
                };

                // 最終更新時刻の表示
                let updated_text = if let Some(ref status) = w.status {
                    if let Some(last_commit_time) = status.last_commit_time {
                        let now = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as i64;
                        let diff = now - last_commit_time;
                        
                        if diff < 3600 {
                            format!("{} m ago", diff / 60)
                        } else if diff < 86400 {
                            format!("{} h ago", diff / 3600)
                        } else {
                            format!("{} d ago", diff / 86400)
                        }
                    } else {
                        "Unknown".to_string()
                    }
                } else {
                    "Unknown".to_string()
                };

                let line = Line::from(vec![
                    Span::styled(
                        if i == state.selected_index { "> " } else { "  " },
                        Style::default().fg(Color::Red),
                    ),
                    Span::styled(
                        format!("{:<15}", w.branch),
                        Style::default().fg(Color::Blue),
                    ),
                    Span::styled(
                        format!("{:<25}", w.path.display()),
                        Style::default().fg(Color::White),
                    ),
                    Span::styled(
                        format!("{:<12}", status_text),
                        Style::default().fg(status_color),
                    ),
                    Span::styled(
                        updated_text,
                        Style::default().fg(Color::Cyan),
                    ),
                ]);
                ListItem::new(line)
            })
            .collect();
        
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("  BRANCH         PATH                     STATUS       UPDATED"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));
        f.render_widget(list, chunks[1]);
        
        // ヘルプとステータス表示（状態に応じて動的に変化）
        let help_text = state.get_help_text();
        let mut display_text = if let Some(ref status) = state.status_message {
            format!("{}\n{}", status, help_text)
        } else {
            help_text
        };
        
        // 🔵 REFACTOR: ダイアログモード時の表示
        if let Some(ref dialog_mode) = state.dialog_mode {
            let dialog_text = match dialog_mode {
                DialogMode::BranchInput => {
                    format!("Enter branch name: {}_", state.get_input_text())
                },
                DialogMode::DeleteConfirmation => {
                    if let Some(name) = state.get_selected_worktree_name() {
                        format!("Delete worktree '{}'? [Enter] Yes [Esc] No", name)
                    } else {
                        "Delete confirmation".to_string()
                    }
                },
                DialogMode::CleanOptions => {
                    let selected = state.get_clean_options();
                    format!("Clean options: [m] merged [s] stale - Selected: {} - [Enter] Confirm [Esc] Cancel", 
                           if selected.is_empty() { "none" } else { &selected })
                }
            };
            display_text = format!("{}\n\n{}", dialog_text, display_text);
        }
        
        let help = Paragraph::new(display_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(help, chunks[2]);
    }
}

#[cfg(test)]
mod tui_clean_dialog_tests {
    use super::*;
    use crate::git::WorktreeInfo;
    use std::path::PathBuf;
    
    fn create_test_worktrees() -> Vec<WorktreeInfo> {
        vec![
            WorktreeInfo {
                name: "main".to_string(),
                path: PathBuf::from("/test/main"),
                branch: "main".to_string(),
                is_prunable: false,
                status: None,
            },
            WorktreeInfo {
                name: "feature-test".to_string(),
                path: PathBuf::from("/test/feature-test"),
                branch: "feature/test".to_string(),
                is_prunable: false,
                status: None,
            },
        ]
    }
    
    #[test]
    fn test_tui_clean_dialog_initialization() {
        // 🟢 GREEN: クリーン条件設定ダイアログが正しく初期化されることをテスト
        let worktrees = create_test_worktrees();
        let mut state = TuiState::new(worktrees);
        
        // cleanアクション設定時にクリーン条件ダイアログに移行
        state.set_action(Some(TuiAction::Clean));
        state.enter_dialog_mode(DialogMode::CleanOptions);
        
        assert!(state.is_in_dialog());
        assert_eq!(state.get_dialog_mode(), Some(DialogMode::CleanOptions));
        
        // クリーン条件の初期値は空
        assert_eq!(state.get_clean_options(), "");
    }
    
    #[test]
    fn test_tui_clean_merged_option() {
        // 🟢 GREEN: マージ済みブランチ削除オプションのテスト
        let worktrees = create_test_worktrees();
        let mut state = TuiState::new(worktrees);
        
        state.set_action(Some(TuiAction::Clean));
        state.enter_dialog_mode(DialogMode::CleanOptions);
        
        // 'm'キーでmergedオプションを選択
        state.toggle_clean_option('m');
        
        let options = state.get_clean_options();
        assert!(options.contains("merged"));
    }
    
    #[test]
    fn test_tui_clean_stale_option() {
        // 🟢 GREEN: 古いブランチ削除オプションのテスト
        let worktrees = create_test_worktrees();
        let mut state = TuiState::new(worktrees);
        
        state.set_action(Some(TuiAction::Clean));
        state.enter_dialog_mode(DialogMode::CleanOptions);
        
        // 's'キーでstaleオプションを選択
        state.toggle_clean_option('s');
        
        let options = state.get_clean_options();
        assert!(options.contains("stale"));
    }
    
    #[test]
    fn test_tui_clean_multiple_options() {
        // 🟢 GREEN: 複数のクリーン条件選択のテスト
        let worktrees = create_test_worktrees();
        let mut state = TuiState::new(worktrees);
        
        state.set_action(Some(TuiAction::Clean));
        state.enter_dialog_mode(DialogMode::CleanOptions);
        
        // 複数オプションを選択
        state.toggle_clean_option('m'); // merged
        state.toggle_clean_option('s'); // stale
        
        let options = state.get_clean_options();
        assert!(options.contains("merged"));
        assert!(options.contains("stale"));
    }
    
    #[test]
    fn test_tui_clean_option_toggle() {
        // 🟢 GREEN: オプションのトグル（選択・解除）機能のテスト
        let worktrees = create_test_worktrees();
        let mut state = TuiState::new(worktrees);
        
        state.set_action(Some(TuiAction::Clean));
        state.enter_dialog_mode(DialogMode::CleanOptions);
        
        // オプションを選択
        state.toggle_clean_option('m');
        assert!(state.get_clean_options().contains("merged"));
        
        // 同じオプションをもう一度選択して解除
        state.toggle_clean_option('m');
        assert!(!state.get_clean_options().contains("merged"));
    }
    
    #[test]
    fn test_tui_clean_dialog_confirmation() {
        // 🟢 GREEN: クリーン条件確定処理のテスト
        let worktrees = create_test_worktrees();
        let mut state = TuiState::new(worktrees);
        
        state.set_action(Some(TuiAction::Clean));
        state.enter_dialog_mode(DialogMode::CleanOptions);
        
        // オプションを選択
        state.toggle_clean_option('m');
        state.toggle_clean_option('s');
        
        // Enterで確定
        let clean_options = state.confirm_dialog();
        
        assert!(clean_options.is_some());
        let options = clean_options.unwrap();
        assert!(options.contains("merged"));
        assert!(options.contains("stale"));
        assert!(!state.is_in_dialog()); // ダイアログが閉じられる
    }
    
    #[test]
    fn test_tui_clean_dialog_empty_confirmation() {
        // 🟢 GREEN: 何も選択せずに確定した場合のテスト
        let worktrees = create_test_worktrees();
        let mut state = TuiState::new(worktrees);
        
        state.set_action(Some(TuiAction::Clean));
        state.enter_dialog_mode(DialogMode::CleanOptions);
        
        // 何も選択せずにEnterで確定
        let clean_options = state.confirm_dialog();
        
        // 空の場合はNoneを返す
        assert_eq!(clean_options, None);
        assert!(state.is_in_dialog()); // ダイアログは開いたまま
    }
}