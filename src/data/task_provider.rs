use anyhow::Result;

use super::models::{TaskItem, TaskStatus, TasksStatus};

pub struct GitHubIssuesProvider {
    owner: String,
    repo: String,
    token: String,
}

impl GitHubIssuesProvider {
    pub fn new(owner: String, repo: String, token: String) -> Self {
        Self { owner, repo, token }
    }

    pub async fn fetch_status(&self) -> Result<TasksStatus> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues?state=all&per_page=15&sort=updated",
            self.owner, self.repo
        );

        let response: Vec<serde_json::Value> = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "dev-dashboard")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?
            .json()
            .await?;

        let tasks: Vec<TaskItem> = response
            .into_iter()
            .filter(|issue| issue.get("pull_request").is_none()) // Skip PRs
            .map(|issue| {
                let state = issue["state"].as_str().unwrap_or("open");
                let empty_vec = vec![];
                let labels: Vec<String> = issue["labels"]
                    .as_array()
                    .unwrap_or(&empty_vec)
                    .iter()
                    .filter_map(|l| l["name"].as_str().map(String::from))
                    .collect();

                let status = if state == "closed" {
                    TaskStatus::Done
                } else if labels.iter().any(|l| l.to_lowercase().contains("block")) {
                    TaskStatus::Blocked
                } else if labels.iter().any(|l| {
                    l.to_lowercase().contains("progress") || l.to_lowercase().contains("wip")
                }) {
                    TaskStatus::InProgress
                } else {
                    TaskStatus::Todo
                };

                let assignee = issue["assignee"]["login"].as_str().map(String::from);

                TaskItem {
                    id: format!("#{}", issue["number"]),
                    title: issue["title"].as_str().unwrap_or("").to_string(),
                    status,
                    assignee,
                }
            })
            .collect();

        Ok(TasksStatus { tasks })
    }
}
