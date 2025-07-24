use git2::{Repository, Status};
use std::path::Path;
use crate::error::{GitGardenerError, Result};

// 🔴 RED: Gitワーキングツリーのステータス（まだ基本実装のみ）
#[derive(Debug, Clone, PartialEq)]
pub enum WorktreeStatus {
    Clean,
    Dirty,
    Ahead,
    Behind,
    Diverged,
}

// 🔴 RED: Gitステータス情報を保持する構造体（まだ基本実装のみ）
#[derive(Debug, Clone)]
pub struct GitStatus {
    pub working_tree_status: WorktreeStatus,
    pub has_staged_changes: bool,
    pub has_unstaged_changes: bool,
    pub last_commit_time: Option<i64>,
    pub ahead_count: u32,
    pub behind_count: u32,
}

impl GitStatus {
    pub fn from_path(path: &Path) -> Result<Self> {
        let repo = Repository::open(path).map_err(|e| {
            GitGardenerError::Custom(format!("Failed to open repository: {}", e))
        })?;
        
        Self::from_repository(&repo)
    }
    
    pub fn from_repository(repo: &Repository) -> Result<Self> {
        // ワーキングツリーの状態をチェック
        let statuses = repo.statuses(None).map_err(|e| {
            GitGardenerError::Custom(format!("Failed to get repository status: {}", e))
        })?;
        
        let mut has_staged_changes = false;
        let mut has_unstaged_changes = false;
        
        for status_entry in statuses.iter() {
            let flags = status_entry.status();
            
            if flags.intersects(
                Status::INDEX_NEW | Status::INDEX_MODIFIED | Status::INDEX_DELETED | Status::INDEX_RENAMED | Status::INDEX_TYPECHANGE
            ) {
                has_staged_changes = true;
            }
            
            if flags.intersects(
                Status::WT_MODIFIED | Status::WT_DELETED | Status::WT_TYPECHANGE | Status::WT_RENAMED | Status::WT_NEW
            ) {
                has_unstaged_changes = true;
            }
        }
        
        // ワーキングツリーのステータスを決定
        let working_tree_status = if has_staged_changes || has_unstaged_changes {
            WorktreeStatus::Dirty
        } else {
            WorktreeStatus::Clean
        };
        
        // 最終コミット時刻を取得
        let last_commit_time = Self::get_last_commit_time(repo);
        
        // 今後の実装: Ahead/Behind の計算
        let ahead_count = 0;
        let behind_count = 0;
        
        Ok(GitStatus {
            working_tree_status,
            has_staged_changes,
            has_unstaged_changes,
            last_commit_time,
            ahead_count,
            behind_count,
        })
    }
    
    fn get_last_commit_time(repo: &Repository) -> Option<i64> {
        // HEADコミットの時刻を取得
        repo.head().ok()
            .and_then(|reference| reference.target())
            .and_then(|oid| repo.find_commit(oid).ok())
            .map(|commit| commit.time().seconds())
    }
}