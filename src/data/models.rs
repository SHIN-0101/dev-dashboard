use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub is_head: bool,
    pub last_commit: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GitStatus {
    pub branch: String,
    pub commits: Vec<CommitInfo>,
    #[allow(dead_code)] // Reserved for future branch list UI
    pub branches: Vec<BranchInfo>,
    pub changed_files: usize,
    pub staged_files: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineStatus {
    Success,
    Failure,
    Running,
    Pending,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineRun {
    pub id: String,
    pub name: String,
    pub status: PipelineStatus,
    pub branch: String,
    pub duration_secs: Option<u64>,
    pub started_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CiStatus {
    pub runs: Vec<PipelineRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskItem {
    pub id: String,
    pub title: String,
    pub status: TaskStatus,
    pub assignee: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TasksStatus {
    pub tasks: Vec<TaskItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct QualityMetrics {
    pub test_coverage: f64, // 0.0 to 100.0
    pub lint_warnings: usize,
    pub lint_errors: usize,
    pub security_issues: usize,
}
