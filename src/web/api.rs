use axum::{extract::State, response::Json, routing::get, Router};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::{ServeDir, ServeFile};

use crate::data::ci_provider::GitHubActionsProvider;
use crate::data::git_provider::GitProvider;
use crate::data::github_config::GitHubConfig;
use crate::data::models::{CiStatus, GitStatus, QualityMetrics, TasksStatus};
use crate::data::quality_provider::LocalQualityProvider;
use crate::data::task_provider::GitHubIssuesProvider;

#[derive(Clone)]
pub struct AppState {
    pub git_status: Arc<RwLock<Option<GitStatus>>>,
    pub ci_status: Arc<RwLock<Option<CiStatus>>>,
    pub tasks_status: Arc<RwLock<Option<TasksStatus>>>,
    pub quality_metrics: Arc<RwLock<Option<QualityMetrics>>>,
    pub config_info: Arc<ConfigInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfigInfo {
    pub owner: Option<String>,
    pub repo: Option<String>,
    pub path: String,
}

pub fn create_router(state: AppState) -> Router {
    let api = Router::new()
        .route("/api/git", get(get_git))
        .route("/api/ci", get(get_ci))
        .route("/api/tasks", get(get_tasks))
        .route("/api/quality", get(get_quality))
        .route("/api/config", get(get_config))
        .with_state(state);

    // Serve static SPA files, fallback to index.html for client-side routing
    let static_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("../../../static")))
        .unwrap_or_else(|| PathBuf::from("static"));

    let spa = ServeDir::new(&static_dir)
        .not_found_service(ServeFile::new(static_dir.join("index.html")));

    let cors = tower_http::cors::CorsLayer::permissive();

    api.fallback_service(spa).layer(cors)
}

async fn get_git(State(state): State<AppState>) -> Json<serde_json::Value> {
    let data = state.git_status.read().await;
    match &*data {
        Some(status) => Json(serde_json::to_value(status).unwrap_or(serde_json::Value::Null)),
        None => Json(serde_json::Value::Null),
    }
}

async fn get_ci(State(state): State<AppState>) -> Json<serde_json::Value> {
    let data = state.ci_status.read().await;
    match &*data {
        Some(status) => Json(serde_json::to_value(status).unwrap_or(serde_json::Value::Null)),
        None => Json(serde_json::Value::Null),
    }
}

async fn get_tasks(State(state): State<AppState>) -> Json<serde_json::Value> {
    let data = state.tasks_status.read().await;
    match &*data {
        Some(status) => Json(serde_json::to_value(status).unwrap_or(serde_json::Value::Null)),
        None => Json(serde_json::Value::Null),
    }
}

async fn get_quality(State(state): State<AppState>) -> Json<serde_json::Value> {
    let data = state.quality_metrics.read().await;
    match &*data {
        Some(metrics) => Json(serde_json::to_value(metrics).unwrap_or(serde_json::Value::Null)),
        None => Json(serde_json::Value::Null),
    }
}

async fn get_config(State(state): State<AppState>) -> Json<ConfigInfo> {
    Json((*state.config_info).clone())
}

pub async fn run_server(
    path: PathBuf,
    refresh_secs: u64,
    github_config: Option<GitHubConfig>,
) -> anyhow::Result<()> {
    let state = AppState {
        git_status: Arc::new(RwLock::new(None)),
        ci_status: Arc::new(RwLock::new(None)),
        tasks_status: Arc::new(RwLock::new(None)),
        quality_metrics: Arc::new(RwLock::new(None)),
        config_info: Arc::new(ConfigInfo {
            owner: github_config.as_ref().map(|c| c.owner.clone()),
            repo: github_config.as_ref().map(|c| c.repo.clone()),
            path: path.display().to_string(),
        }),
    };

    // Spawn background data fetchers
    spawn_web_fetchers(state.clone(), path.clone(), refresh_secs, github_config);

    let app = create_router(state);

    let addr = "0.0.0.0:3000";
    println!("dev-dashboard web server running at http://localhost:3000");
    println!("API endpoints:");
    println!("  GET /api/git");
    println!("  GET /api/ci");
    println!("  GET /api/tasks");
    println!("  GET /api/quality");
    println!("  GET /api/config");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn spawn_web_fetchers(
    state: AppState,
    path: PathBuf,
    refresh_secs: u64,
    github_config: Option<GitHubConfig>,
) {
    // Git fetcher
    {
        let state = state.clone();
        let path = path.clone();
        tokio::spawn(async move {
            let provider = GitProvider::new(path);
            loop {
                if let Ok(status) = provider.fetch_status() {
                    *state.git_status.write().await = Some(status);
                }
                tokio::time::sleep(std::time::Duration::from_secs(refresh_secs)).await;
            }
        });
    }

    // CI fetcher
    if let Some(ref config) = github_config {
        let state = state.clone();
        let owner = config.owner.clone();
        let repo = config.repo.clone();
        let token = config.token.clone();
        tokio::spawn(async move {
            let provider = GitHubActionsProvider::new(owner, repo, token);
            loop {
                if let Ok(status) = provider.fetch_status().await {
                    *state.ci_status.write().await = Some(status);
                }
                tokio::time::sleep(std::time::Duration::from_secs(refresh_secs)).await;
            }
        });
    }

    // Tasks fetcher
    if let Some(ref config) = github_config {
        let state = state.clone();
        let owner = config.owner.clone();
        let repo = config.repo.clone();
        let token = config.token.clone();
        tokio::spawn(async move {
            let provider = GitHubIssuesProvider::new(owner, repo, token);
            loop {
                if let Ok(status) = provider.fetch_status().await {
                    *state.tasks_status.write().await = Some(status);
                }
                tokio::time::sleep(std::time::Duration::from_secs(refresh_secs)).await;
            }
        });
    }

    // Quality fetcher
    {
        let state = state.clone();
        let path = path.clone();
        tokio::spawn(async move {
            loop {
                let p = path.clone();
                let metrics = tokio::task::spawn_blocking(move || {
                    let provider = LocalQualityProvider::new(p);
                    provider.fetch_metrics().ok()
                })
                .await
                .ok()
                .flatten();
                if let Some(m) = metrics {
                    *state.quality_metrics.write().await = Some(m);
                }
                tokio::time::sleep(std::time::Duration::from_secs(refresh_secs * 6)).await;
            }
        });
    }
}
