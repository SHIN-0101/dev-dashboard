use anyhow::Result;
use chrono::{DateTime, Utc};

use super::models::{CiStatus, PipelineRun, PipelineStatus};

pub struct GitHubActionsProvider {
    owner: String,
    repo: String,
    token: String,
}

impl GitHubActionsProvider {
    pub fn new(owner: String, repo: String, token: String) -> Self {
        Self { owner, repo, token }
    }

    pub async fn fetch_status(&self) -> Result<CiStatus> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://api.github.com/repos/{}/{}/actions/runs?per_page=10",
            self.owner, self.repo
        );

        let response: serde_json::Value = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "dev-dashboard")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?
            .json()
            .await?;

        let empty_vec = vec![];
        let runs = response["workflow_runs"]
            .as_array()
            .unwrap_or(&empty_vec)
            .iter()
            .take(10)
            .map(|run| {
                let conclusion = run["conclusion"].as_str().unwrap_or("");
                let status_str = run["status"].as_str().unwrap_or("");

                let status = match conclusion {
                    "success" => PipelineStatus::Success,
                    "failure" => PipelineStatus::Failure,
                    "cancelled" => PipelineStatus::Cancelled,
                    _ => {
                        if status_str == "in_progress" || status_str == "queued" {
                            PipelineStatus::Running
                        } else {
                            PipelineStatus::Pending
                        }
                    }
                };

                PipelineRun {
                    id: run["id"].to_string(),
                    name: run["name"].as_str().unwrap_or("Unknown").to_string(),
                    status,
                    branch: run["head_branch"].as_str().unwrap_or("unknown").to_string(),
                    duration_secs: None,
                    started_at: run["created_at"]
                        .as_str()
                        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                        .map(|dt| dt.with_timezone(&Utc)),
                }
            })
            .collect();

        Ok(CiStatus { runs })
    }
}
