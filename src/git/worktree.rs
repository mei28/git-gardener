use git2::{Repository, BranchType};
use std::path::{Path, PathBuf};
use crate::error::{GitGardenerError, Result};
use super::status::{GitStatus, WorktreeStatus};

#[derive(Clone)]
pub struct WorktreeInfo {
    pub name: String,
    pub path: PathBuf,
    pub branch: String,
    pub is_prunable: bool,
    pub status: Option<GitStatus>,
}

// 🔴 RED: ステータス情報付きWorktreeInfo（まだ基本実装のみ）
#[derive(Clone)]
pub struct WorktreeInfoWithStatus {
    pub name: String,
    pub path: PathBuf,
    pub branch: String,
    pub is_prunable: bool,
    pub status: GitStatus,
}

pub struct GitWorktree {
    repo: Repository,
}

impl GitWorktree {
    pub fn new() -> Result<Self> {
        let repo = Repository::open_from_env()
            .map_err(|_| GitGardenerError::NotInRepository)?;
        Ok(Self { repo })
    }
    
    pub fn from_path(path: &Path) -> Result<Self> {
        let repo = Repository::open(path)
            .map_err(|_| GitGardenerError::NotInRepository)?;
        Ok(Self { repo })
    }
    
    pub fn create_worktree(
        &self,
        _name: &str,
        path: &Path,
        branch_name: &str,
        create_branch: bool,
    ) -> Result<()> {
        // 🟢 GREEN: worktree作成の修正版
        if create_branch && !self.branch_exists(branch_name)? {
            // 新しいブランチを作成
            let head = self.repo.head()?;
            let commit = head.peel_to_commit()?;
            let _branch = self.repo.branch(branch_name, &commit, false)?;
        }
        
        // gitコマンドを使用してworktreeを作成（より安定した方法）
        let output = std::process::Command::new("git")
            .args(&["worktree", "add", &path.to_string_lossy(), branch_name])
            .output()
            .map_err(|e| GitGardenerError::Custom(format!("Failed to execute git worktree add: {}", e)))?;
        
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitGardenerError::Custom(format!("git worktree add failed: {}", error_msg)));
        }
        
        Ok(())
    }
    
    pub fn create_worktree_with_commit(
        &self,
        _name: &str,
        path: &Path,
        branch_name: &str,
        create_branch: bool,
        commit: Option<&str>,
    ) -> Result<()> {
        // gitコマンドを使用してworktreeを作成
        let mut args = vec!["worktree", "add"];
        
        if create_branch {
            args.push("-b");
            args.push(branch_name);
        }
        
        let path_str = path.to_string_lossy();
        args.push(&path_str);
        
        if !create_branch {
            args.push(branch_name);
        } else if let Some(commit) = commit {
            args.push(commit);
        }
        
        let output = std::process::Command::new("git")
            .args(&args)
            .output()
            .map_err(|e| GitGardenerError::Custom(format!("Failed to execute git worktree add: {}", e)))?;
        
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitGardenerError::Custom(format!("git worktree add failed: {}", error_msg)));
        }
        
        Ok(())
    }
    
    pub fn list_worktrees(&self) -> Result<Vec<WorktreeInfo>> {
        let worktrees = self.repo.worktrees()?;
        let mut infos = Vec::new();
        
        for worktree_name in worktrees.iter().flatten() {
            if let Ok(worktree) = self.repo.find_worktree(worktree_name) {
                let path = worktree.path();
                let is_prunable = worktree.is_prunable(None).unwrap_or(false);
                
                let branch = self.get_worktree_branch(&worktree)?;
                
                // ステータス情報を取得（エラーの場合はNone）
                let status = GitStatus::from_path(&path).ok();
                
                infos.push(WorktreeInfo {
                    name: worktree_name.to_string(),
                    path: path.to_path_buf(),
                    branch,
                    is_prunable,
                    status,
                });
            }
        }
        
        Ok(infos)
    }
    
    pub fn remove_worktree(&self, name: &str, force: bool) -> Result<()> {
        // 🟢 GREEN: git worktree removeを使用する修正版
        let worktrees = self.list_worktrees()?;
        let worktree_info = worktrees
            .iter()
            .find(|w| w.name == name)
            .ok_or_else(|| GitGardenerError::WorktreeNotFound { 
                name: name.to_string() 
            })?;
        
        // gitコマンドを使用してworktreeを削除
        let path_str = worktree_info.path.to_string_lossy();
        let mut args = vec!["worktree", "remove"];
        if force {
            args.push("--force");
        }
        args.push(&path_str);
        
        let output = std::process::Command::new("git")
            .args(&args)
            .output()
            .map_err(|e| GitGardenerError::Custom(format!("Failed to execute git worktree remove: {}", e)))?;
        
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitGardenerError::Custom(format!("git worktree remove failed: {}", error_msg)));
        }
        
        Ok(())
    }
    
    pub fn get_repository_root(&self) -> Result<PathBuf> {
        Ok(self.repo.workdir()
            .ok_or_else(|| GitGardenerError::Custom(
                "Could not determine repository root".to_string()
            ))?
            .to_path_buf())
    }
    
    fn get_worktree_branch(&self, worktree: &git2::Worktree) -> Result<String> {
        let worktree_repo = Repository::open(worktree.path())?;
        
        if let Ok(head) = worktree_repo.head() {
            if let Some(name) = head.shorthand() {
                return Ok(name.to_string());
            }
        }
        
        Ok("(unknown)".to_string())
    }
    
    pub fn branch_exists(&self, branch_name: &str) -> Result<bool> {
        let branches = self.repo.branches(Some(BranchType::Local))?;
        
        for branch_result in branches {
            if let Ok((branch, _)) = branch_result {
                if let Some(name) = branch.name()? {
                    if name == branch_name {
                        return Ok(true);
                    }
                }
            }
        }
        
        Ok(false)
    }
    
    // 🟢 GREEN: マージ済みブランチの判定ロジック実装
    pub fn is_branch_merged(&self, branch_name: &str, base_branch: &str) -> Result<bool> {
        // ブランチの参照を取得
        let branch_ref = format!("refs/heads/{}", branch_name);
        let base_ref = format!("refs/heads/{}", base_branch);
        
        let branch_commit = match self.repo.find_reference(&branch_ref) {
            Ok(reference) => {
                let oid = reference.target().ok_or_else(|| {
                    GitGardenerError::Custom(format!("Branch {} has no target", branch_name))
                })?;
                self.repo.find_commit(oid)?
            }
            Err(_) => {
                return Ok(false); // ブランチが存在しない場合はfalse
            }
        };
        
        let base_commit = match self.repo.find_reference(&base_ref) {
            Ok(reference) => {
                let oid = reference.target().ok_or_else(|| {
                    GitGardenerError::Custom(format!("Base branch {} has no target", base_branch))
                })?;
                self.repo.find_commit(oid)?
            }
            Err(_) => {
                return Ok(false); // ベースブランチが存在しない場合はfalse
            }
        };
        
        // ブランチのコミットがベースブランチから到達可能かチェック
        let is_ancestor = self.repo.graph_descendant_of(base_commit.id(), branch_commit.id())?;
        
        Ok(is_ancestor)
    }
    
    // 🟢 GREEN: 古いworktreeかどうかの判定（実装）
    pub fn is_worktree_stale(&self, branch_name: &str, days: u32) -> Result<bool> {
        // ブランチの参照を取得
        let branch_ref = format!("refs/heads/{}", branch_name);
        
        let branch_commit = match self.repo.find_reference(&branch_ref) {
            Ok(reference) => {
                let oid = reference.target().ok_or_else(|| {
                    GitGardenerError::Custom(format!("Branch {} has no target", branch_name))
                })?;
                self.repo.find_commit(oid)?
            }
            Err(_) => {
                // ブランチが存在しない場合はfalse（削除できない）
                return Ok(false);
            }
        };
        
        // 最後のコミット日時を取得
        let commit_time = branch_commit.time().seconds();
        
        // 現在時刻から指定日数前の時刻を計算
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let threshold_time = now - (days as i64 * 24 * 60 * 60);
        
        // コミット時刻が閾値より古いかどうかを判定
        Ok(commit_time < threshold_time)
    }
    
    // 🔴 RED: ステータス情報付きワーキングツリー一覧取得（まだ基本実装のみ）
    pub fn list_worktrees_with_status(&self) -> Result<Vec<WorktreeInfoWithStatus>> {
        let worktrees = self.list_worktrees()?;
        let mut result = Vec::new();
        
        for worktree in worktrees {
            // 各ワーキングツリーのステータスを取得
            let status = GitStatus::from_path(&worktree.path).unwrap_or_else(|_| {
                // エラーの場合はデフォルトステータスを使用
                GitStatus {
                    working_tree_status: WorktreeStatus::Clean,
                    has_staged_changes: false,
                    has_unstaged_changes: false,
                    last_commit_time: None,
                    ahead_count: 0,
                    behind_count: 0,
                }
            });
            
            result.push(WorktreeInfoWithStatus {
                name: worktree.name,
                path: worktree.path,
                branch: worktree.branch,
                is_prunable: worktree.is_prunable,
                status,
            });
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_not_in_repository() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();
        
        let result = GitWorktree::new();
        assert!(matches!(result, Err(GitGardenerError::NotInRepository)));
    }
}