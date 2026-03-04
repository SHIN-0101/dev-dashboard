use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use git2::Repository;
use std::path::PathBuf;

use super::models::{BranchInfo, CommitInfo, GitStatus};

pub struct GitProvider {
    path: PathBuf,
}

impl GitProvider {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn fetch_status(&self) -> Result<GitStatus> {
        let repo = Repository::open(&self.path).context("Failed to open git repository")?;

        let branch = self.current_branch(&repo)?;
        let commits = self.recent_commits(&repo, 20)?;
        let branches = self.list_branches(&repo)?;
        let (changed, staged) = self.file_counts(&repo)?;

        Ok(GitStatus {
            branch,
            commits,
            branches,
            changed_files: changed,
            staged_files: staged,
        })
    }

    fn current_branch(&self, repo: &Repository) -> Result<String> {
        let head = repo.head().context("Failed to get HEAD")?;
        Ok(head.shorthand().unwrap_or("detached").to_string())
    }

    fn recent_commits(&self, repo: &Repository, limit: usize) -> Result<Vec<CommitInfo>> {
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;

        let commits: Vec<CommitInfo> = revwalk
            .take(limit)
            .filter_map(|oid| oid.ok())
            .filter_map(|oid| {
                let commit = repo.find_commit(oid).ok()?;
                let hash = oid.to_string()[..7].to_string();
                let message = commit.summary().unwrap_or("").to_string();
                let author = commit.author().name().unwrap_or("unknown").to_string();
                let time = commit.time();
                let timestamp = Utc
                    .timestamp_opt(time.seconds(), 0)
                    .single()
                    .unwrap_or_else(Utc::now);

                Some(CommitInfo {
                    hash,
                    message,
                    author,
                    timestamp,
                })
            })
            .collect();

        Ok(commits)
    }

    fn list_branches(&self, repo: &Repository) -> Result<Vec<BranchInfo>> {
        let mut branches = Vec::new();

        for branch in repo.branches(Some(git2::BranchType::Local))? {
            let (branch, _) = branch?;
            let name = branch.name()?.unwrap_or("unknown").to_string();
            let is_head = branch.is_head();
            let last_commit = branch
                .get()
                .target()
                .map(|oid| oid.to_string()[..7].to_string())
                .unwrap_or_default();

            branches.push(BranchInfo {
                name,
                is_head,
                last_commit,
            });
        }

        Ok(branches)
    }

    fn file_counts(&self, repo: &Repository) -> Result<(usize, usize)> {
        let statuses = repo.statuses(None)?;
        let mut changed = 0;
        let mut staged = 0;

        for entry in statuses.iter() {
            let s = entry.status();
            if s.intersects(
                git2::Status::WT_MODIFIED
                    | git2::Status::WT_NEW
                    | git2::Status::WT_DELETED
                    | git2::Status::WT_RENAMED,
            ) {
                changed += 1;
            }
            if s.intersects(
                git2::Status::INDEX_NEW
                    | git2::Status::INDEX_MODIFIED
                    | git2::Status::INDEX_DELETED
                    | git2::Status::INDEX_RENAMED,
            ) {
                staged += 1;
            }
        }

        Ok((changed, staged))
    }
}
