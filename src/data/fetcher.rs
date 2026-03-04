use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;

use super::ci_provider::GitHubActionsProvider;
use super::git_provider::GitProvider;
use super::models::{CiStatus, GitStatus, QualityMetrics, TasksStatus};
use super::quality_provider::LocalQualityProvider;
use super::task_provider::GitHubIssuesProvider;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum DataMessage {
    GitUpdated(Option<GitStatus>),
    CiUpdated(Option<CiStatus>),
    TasksUpdated(Option<TasksStatus>),
    QualityUpdated(Option<QualityMetrics>),
}

pub fn spawn_git_fetcher(tx: mpsc::Sender<DataMessage>, path: PathBuf, refresh_secs: u64) {
    tokio::spawn(async move {
        let provider = GitProvider::new(path);
        loop {
            let status = provider.fetch_status().ok();
            let _ = tx.send(DataMessage::GitUpdated(status)).await;
            tokio::time::sleep(Duration::from_secs(refresh_secs)).await;
        }
    });
}

pub fn spawn_ci_fetcher(
    tx: mpsc::Sender<DataMessage>,
    owner: String,
    repo: String,
    token: String,
    refresh_secs: u64,
) {
    tokio::spawn(async move {
        let provider = GitHubActionsProvider::new(owner, repo, token);
        loop {
            let status = provider.fetch_status().await.ok();
            let _ = tx.send(DataMessage::CiUpdated(status)).await;
            tokio::time::sleep(Duration::from_secs(refresh_secs)).await;
        }
    });
}

pub fn spawn_task_fetcher(
    tx: mpsc::Sender<DataMessage>,
    owner: String,
    repo: String,
    token: String,
    refresh_secs: u64,
) {
    tokio::spawn(async move {
        let provider = GitHubIssuesProvider::new(owner, repo, token);
        loop {
            let status = provider.fetch_status().await.ok();
            let _ = tx.send(DataMessage::TasksUpdated(status)).await;
            tokio::time::sleep(Duration::from_secs(refresh_secs)).await;
        }
    });
}

pub fn spawn_quality_fetcher(tx: mpsc::Sender<DataMessage>, path: PathBuf, refresh_secs: u64) {
    tokio::spawn(async move {
        loop {
            let path_clone = path.clone();
            let metrics = tokio::task::spawn_blocking(move || {
                let p = LocalQualityProvider::new(path_clone);
                p.fetch_metrics().ok()
            })
            .await
            .ok()
            .flatten();
            let _ = tx.send(DataMessage::QualityUpdated(metrics)).await;
            // Quality checks are expensive, run less frequently
            tokio::time::sleep(Duration::from_secs(refresh_secs * 6)).await;
        }
    });
}
