// 🔴 RED: TUIナビゲーション機能の単体テスト（まだ実装されていない機能をテスト）

#[cfg(test)]
mod tests {
    use git_gardener::commands::tui::{TuiState, TuiAction};
    use git_gardener::git::WorktreeInfo;
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
            WorktreeInfo {
                name: "bugfix-fix".to_string(),
                path: PathBuf::from("/test/bugfix-fix"),
                branch: "bugfix/fix".to_string(),
                is_prunable: false,
                status: None,
            },
        ]
    }
    
    #[test]
    fn test_tui_state_initialization() {
        // 🔴 RED: TuiStateが初期化できることをテスト（まだ実装されていない）
        let worktrees = create_test_worktrees();
        let state = TuiState::new(worktrees.clone());
        
        assert_eq!(state.worktrees.len(), 3);
        assert_eq!(state.selected_index, 0);
        assert_eq!(state.worktrees[0].name, "main");
    }
    
    #[test]
    fn test_tui_navigation_down() {
        // 🔴 RED: 下方向ナビゲーションのテスト（まだ実装されていない）
        let worktrees = create_test_worktrees();
        let mut state = TuiState::new(worktrees);
        
        // 初期状態
        assert_eq!(state.selected_index, 0);
        
        // 下に移動
        state.move_down();
        assert_eq!(state.selected_index, 1);
        
        state.move_down();
        assert_eq!(state.selected_index, 2);
        
        // 最後の要素でもう一度下に移動しても変わらない
        state.move_down();
        assert_eq!(state.selected_index, 2);
    }
    
    #[test]
    fn test_tui_navigation_up() {
        // 🔴 RED: 上方向ナビゲーションのテスト（まだ実装されていない）
        let worktrees = create_test_worktrees();
        let mut state = TuiState::new(worktrees);
        
        // 最後の要素に移動
        state.selected_index = 2;
        
        // 上に移動
        state.move_up();
        assert_eq!(state.selected_index, 1);
        
        state.move_up();
        assert_eq!(state.selected_index, 0);
        
        // 最初の要素でもう一度上に移動しても変わらない
        state.move_up();
        assert_eq!(state.selected_index, 0);
    }
    
    // 三角測量のための3つ目のテスト：先頭・末尾への移動
    #[test]
    fn test_tui_navigation_home_end() {
        // 🔴 RED: g/Gキーでの先頭・末尾移動のテスト（まだ実装されていない）
        let worktrees = create_test_worktrees();
        let mut state = TuiState::new(worktrees);
        
        // 真ん中に移動
        state.selected_index = 1;
        
        // Gで末尾に移動
        state.move_to_end();
        assert_eq!(state.selected_index, 2);
        
        // gで先頭に移動
        state.move_to_start();
        assert_eq!(state.selected_index, 0);
    }
    
    #[test]
    fn test_tui_get_selected_worktree() {
        // 🔴 RED: 選択されたワーキングツリーの取得テスト（まだ実装されていない）
        let worktrees = create_test_worktrees();
        let mut state = TuiState::new(worktrees);
        
        // 初期選択
        let selected = state.get_selected();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().name, "main");
        
        // 選択を変更
        state.move_down();
        let selected = state.get_selected();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().name, "feature-test");
    }
    
    // 🔵 REFACTOR: アクションキー機能のテスト
    #[test]
    fn test_tui_action_state() {
        let worktrees = create_test_worktrees();
        let mut state = TuiState::new(worktrees);
        
        // アクション状態の初期値
        assert_eq!(state.get_current_action(), None);
        
        // アクションを設定
        state.set_action(Some(TuiAction::Add));
        assert_eq!(state.get_current_action(), Some(TuiAction::Add));
        
        // アクションをクリア
        state.clear_action();
        assert_eq!(state.get_current_action(), None);
    }
    
    #[test]
    fn test_tui_action_help_text() {
        let worktrees = create_test_worktrees();
        let state = TuiState::new(worktrees);
        
        // ヘルプテキストが含まれることを確認
        let help = state.get_help_text();
        assert!(help.contains("[a] add"));
        assert!(help.contains("[d] delete"));
        assert!(help.contains("[p] pull"));
        assert!(help.contains("[c] clean"));
        assert!(help.contains("[q] quit"));
    }
}