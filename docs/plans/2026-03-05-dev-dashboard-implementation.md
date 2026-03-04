# Dev Dashboard Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** ターミナルでGit・CI/CD・タスク管理・コード品質を一画面で可視化するダッシュボードCLIツールを構築する。

**Architecture:** Rust + Ratatui による即時モードレンダリングTUI。tokioベースの非同期データフェッチ層がmpscチャネル経由でUI層にデータを流す。各データソース（Git, CI/CD, Tasks, Quality）はtrait抽象化で差し替え可能。

**Tech Stack:** Rust, Ratatui, crossterm, tokio, git2, octocrab, reqwest, clap, serde, rusqlite

---

## Task 1: Project Scaffolding

**Owner:** Zen（バックエンド）
**Reviewer:** Sora（アーキテクト）

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `src/lib.rs`

**Step 1: Initialize Cargo project**

```bash
cd /Users/shin/Desktop/dev-projects/dev-dashboard
cargo init --name dev-dashboard
```

**Step 2: Configure Cargo.toml with dependencies**

```toml
[package]
name = "dev-dashboard"
version = "0.1.0"
edition = "2021"
description = "Terminal dashboard for development status visualization"

[dependencies]
ratatui = "0.29"
crossterm = "0.28"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
octocrab = "0.44"
git2 = "0.19"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
clap = { version = "4", features = ["derive"] }
anyhow = "1"
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
tokio-test = "0.4"
mockall = "0.13"
assert_cmd = "2"
predicates = "3"
```

**Step 3: Create minimal src/main.rs**

```rust
use anyhow::Result;

mod app;
mod ui;
mod data;

#[tokio::main]
async fn main() -> Result<()> {
    println!("dev-dashboard v0.1.0");
    Ok(())
}
```

**Step 4: Create src/lib.rs**

```rust
pub mod app;
pub mod ui;
pub mod data;
```

**Step 5: Create module directories**

```bash
mkdir -p src/{app,ui,data}
touch src/app/mod.rs src/ui/mod.rs src/data/mod.rs
```

**Step 6: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 7: Commit**

```bash
git add Cargo.toml src/
git commit -m "feat: scaffold project with Rust + Ratatui dependencies"
```

---

## Task 2: App State & Event Loop

**Owner:** Zen（バックエンド）
**Reviewer:** Sora（アーキテクト）

**Files:**
- Create: `src/app/mod.rs`
- Create: `src/app/state.rs`
- Create: `src/app/event.rs`
- Modify: `src/main.rs`
- Test: `tests/app_test.rs`

**Step 1: Write failing test for App state**

```rust
// tests/app_test.rs
use dev_dashboard::app::state::{App, ActivePanel};

#[test]
fn test_app_initial_state() {
    let app = App::new();
    assert!(!app.should_quit);
    assert_eq!(app.active_panel, ActivePanel::Git);
}

#[test]
fn test_app_quit() {
    let mut app = App::new();
    app.quit();
    assert!(app.should_quit);
}

#[test]
fn test_app_cycle_panel() {
    let mut app = App::new();
    assert_eq!(app.active_panel, ActivePanel::Git);
    app.next_panel();
    assert_eq!(app.active_panel, ActivePanel::CiCd);
    app.next_panel();
    assert_eq!(app.active_panel, ActivePanel::Tasks);
    app.next_panel();
    assert_eq!(app.active_panel, ActivePanel::Quality);
    app.next_panel();
    assert_eq!(app.active_panel, ActivePanel::Git);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test app_test`
Expected: FAIL — module not found

**Step 3: Implement App state**

```rust
// src/app/state.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    Git,
    CiCd,
    Tasks,
    Quality,
}

impl ActivePanel {
    pub fn next(self) -> Self {
        match self {
            Self::Git => Self::CiCd,
            Self::CiCd => Self::Tasks,
            Self::Tasks => Self::Quality,
            Self::Quality => Self::Git,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Git => Self::Quality,
            Self::CiCd => Self::Git,
            Self::Tasks => Self::CiCd,
            Self::Quality => Self::Tasks,
        }
    }
}

pub struct App {
    pub should_quit: bool,
    pub active_panel: ActivePanel,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            active_panel: ActivePanel::Git,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next_panel(&mut self) {
        self.active_panel = self.active_panel.next();
    }

    pub fn prev_panel(&mut self) {
        self.active_panel = self.active_panel.prev();
    }
}
```

**Step 4: Update src/app/mod.rs**

```rust
pub mod state;
pub mod event;
```

**Step 5: Create event handler**

```rust
// src/app/event.rs
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;
use anyhow::Result;

use super::state::App;

pub fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            handle_key(app, key);
        }
    }
    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),
        KeyCode::Tab => app.next_panel(),
        KeyCode::BackTab => app.prev_panel(),
        KeyCode::Char('1') => app.active_panel = super::state::ActivePanel::Git,
        KeyCode::Char('2') => app.active_panel = super::state::ActivePanel::CiCd,
        KeyCode::Char('3') => app.active_panel = super::state::ActivePanel::Tasks,
        KeyCode::Char('4') => app.active_panel = super::state::ActivePanel::Quality,
        _ => {}
    }
}
```

**Step 6: Run tests**

Run: `cargo test --test app_test`
Expected: PASS

**Step 7: Commit**

```bash
git add src/app/ tests/
git commit -m "feat: add App state management and event handling"
```

---

## Task 3: Terminal Setup & Render Loop

**Owner:** Rin（デザイナー） + Zen（バックエンド）
**Reviewer:** Sora（アーキテクト）

**Files:**
- Create: `src/ui/terminal.rs`
- Create: `src/ui/layout.rs`
- Modify: `src/ui/mod.rs`
- Modify: `src/main.rs`

**Step 1: Create terminal wrapper**

```rust
// src/ui/terminal.rs
use std::io::{self, Stdout};
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> Result<Tui> {
    execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;
    Ok(terminal)
}

pub fn restore() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
```

**Step 2: Create dashboard layout**

```rust
// src/ui/layout.rs
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::state::{App, ActivePanel};

pub fn render(frame: &mut Frame, app: &App) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header
            Constraint::Min(0),    // main content
            Constraint::Length(1), // footer
        ])
        .split(frame.area());

    render_header(frame, outer[0], app);
    render_panels(frame, outer[1], app);
    render_footer(frame, outer[2]);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let titles = vec!["Git", "CI/CD", "Tasks", "Quality"];
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" dev-dashboard "))
        .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .select(match app.active_panel {
            ActivePanel::Git => 0,
            ActivePanel::CiCd => 1,
            ActivePanel::Tasks => 2,
            ActivePanel::Quality => 3,
        });
    frame.render_widget(tabs, area);
}

fn render_panels(frame: &mut Frame, area: Rect, app: &App) {
    let grid = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(grid[0]);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(grid[1]);

    let panels = [
        ("Git", ActivePanel::Git, left[0]),
        ("CI/CD", ActivePanel::CiCd, right[0]),
        ("Tasks", ActivePanel::Tasks, left[1]),
        ("Quality", ActivePanel::Quality, right[1]),
    ];

    for (title, panel, rect) in panels {
        let is_active = app.active_panel == panel;
        let border_style = if is_active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(format!(" {} ", title));

        let placeholder = Paragraph::new("Loading...")
            .block(block)
            .style(Style::default().fg(Color::DarkGray));

        frame.render_widget(placeholder, rect);
    }
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new(" [1-4] Panel  [Tab] Next  [q] Quit")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, area);
}
```

**Step 3: Update src/ui/mod.rs**

```rust
pub mod terminal;
pub mod layout;
```

**Step 4: Update src/main.rs with render loop**

```rust
// src/main.rs
use anyhow::Result;

mod app;
mod ui;
mod data;

use app::state::App;
use app::event::handle_events;
use ui::terminal;
use ui::layout;

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = terminal::init()?;
    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            layout::render(frame, &app);
        })?;

        handle_events(&mut app)?;

        if app.should_quit {
            break;
        }
    }

    terminal::restore()?;
    Ok(())
}
```

**Step 5: Run and verify visually**

Run: `cargo run`
Expected: 4-panel dashboard with tab navigation, quit with 'q'

**Step 6: Commit**

```bash
git add src/
git commit -m "feat: add terminal UI with 4-panel dashboard layout"
```

---

## Task 4: Data Layer — Git Provider

**Owner:** Zen（バックエンド）
**Reviewer:** Sora（アーキテクト）, Ryu（QA）

**Files:**
- Create: `src/data/mod.rs`
- Create: `src/data/git_provider.rs`
- Create: `src/data/models.rs`
- Test: `tests/git_provider_test.rs`

**Step 1: Define data models**

```rust
// src/data/models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,      // short hash (7 chars)
    pub message: String,
    pub author: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub is_head: bool,
    pub last_commit: String,  // short hash
}

#[derive(Debug, Clone)]
pub struct GitStatus {
    pub branch: String,
    pub commits: Vec<CommitInfo>,
    pub branches: Vec<BranchInfo>,
    pub changed_files: usize,
    pub staged_files: usize,
}
```

**Step 2: Write failing test for git provider**

```rust
// tests/git_provider_test.rs
use dev_dashboard::data::git_provider::GitProvider;
use std::path::PathBuf;

#[test]
fn test_git_provider_reads_current_repo() {
    let provider = GitProvider::new(PathBuf::from("."));
    let status = provider.fetch_status();
    assert!(status.is_ok());
    let status = status.unwrap();
    assert!(!status.branch.is_empty());
    assert!(!status.commits.is_empty());
}

#[test]
fn test_git_provider_invalid_path() {
    let provider = GitProvider::new(PathBuf::from("/tmp/nonexistent-repo-xyz"));
    let status = provider.fetch_status();
    assert!(status.is_err());
}
```

**Step 3: Run test to verify it fails**

Run: `cargo test --test git_provider_test`
Expected: FAIL

**Step 4: Implement GitProvider**

```rust
// src/data/git_provider.rs
use std::path::PathBuf;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc, TimeZone};
use git2::Repository;

use super::models::{CommitInfo, BranchInfo, GitStatus};

pub struct GitProvider {
    path: PathBuf,
}

impl GitProvider {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn fetch_status(&self) -> Result<GitStatus> {
        let repo = Repository::open(&self.path)
            .context("Failed to open git repository")?;

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
                let timestamp = Utc.timestamp_opt(time.seconds(), 0)
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
        let head_oid = repo.head().ok().and_then(|h| h.target());

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
```

**Step 5: Update src/data/mod.rs**

```rust
pub mod models;
pub mod git_provider;
```

**Step 6: Run tests**

Run: `cargo test --test git_provider_test`
Expected: PASS

**Step 7: Commit**

```bash
git add src/data/ tests/
git commit -m "feat: add Git data provider with local repo reading"
```

---

## Task 5: Git Panel UI

**Owner:** Rin（デザイナー）
**Reviewer:** Mio（DevRel）for help text

**Files:**
- Create: `src/ui/panels/mod.rs`
- Create: `src/ui/panels/git_panel.rs`
- Modify: `src/ui/layout.rs`
- Modify: `src/ui/mod.rs`

**Step 1: Create git panel renderer**

```rust
// src/ui/panels/git_panel.rs
use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::data::models::GitStatus;

pub fn render(frame: &mut Frame, area: Rect, status: &Option<GitStatus>, is_active: bool) {
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(format!(" Git "));

    match status {
        Some(status) => {
            let inner = block.inner(area);
            frame.render_widget(block, area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2), // branch + file status
                    Constraint::Min(0),    // commit list
                ])
                .split(inner);

            // Branch and file status
            let branch_line = Line::from(vec![
                Span::styled("branch: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&status.branch, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled(
                    format!("{}M {}S", status.changed_files, status.staged_files),
                    Style::default().fg(Color::Yellow),
                ),
            ]);
            frame.render_widget(Paragraph::new(branch_line), chunks[0]);

            // Commit list
            let rows: Vec<Row> = status
                .commits
                .iter()
                .take(10)
                .map(|c| {
                    Row::new(vec![
                        Cell::from(c.hash.clone()).style(Style::default().fg(Color::Yellow)),
                        Cell::from(c.message.clone()),
                        Cell::from(c.author.clone()).style(Style::default().fg(Color::Blue)),
                    ])
                })
                .collect();

            let table = Table::new(
                rows,
                [
                    Constraint::Length(8),
                    Constraint::Min(20),
                    Constraint::Length(12),
                ],
            )
            .header(
                Row::new(vec!["Hash", "Message", "Author"])
                    .style(Style::default().fg(Color::DarkGray))
            );

            frame.render_widget(table, chunks[1]);
        }
        None => {
            let loading = Paragraph::new("Fetching git data...")
                .block(block)
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(loading, area);
        }
    }
}
```

**Step 2: Create panels module**

```rust
// src/ui/panels/mod.rs
pub mod git_panel;
```

**Step 3: Update src/ui/mod.rs**

```rust
pub mod terminal;
pub mod layout;
pub mod panels;
```

**Step 4: Integrate git data into App state and layout**

Update `src/app/state.rs` to hold git data:

```rust
// Add to App struct
use crate::data::models::GitStatus;

pub struct App {
    pub should_quit: bool,
    pub active_panel: ActivePanel,
    pub git_status: Option<GitStatus>,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            active_panel: ActivePanel::Git,
            git_status: None,
        }
    }
    // ... existing methods unchanged
}
```

Update `src/ui/layout.rs` to use the git panel:

Replace the Git placeholder block in `render_panels` with:

```rust
// In render_panels, replace the git placeholder:
use super::panels::git_panel;

// For the Git panel area:
git_panel::render(frame, left[0], &app.git_status, app.active_panel == ActivePanel::Git);
```

**Step 5: Load git data on startup in main.rs**

```rust
// In main.rs, after App::new():
use data::git_provider::GitProvider;
use std::path::PathBuf;

let git_provider = GitProvider::new(PathBuf::from("."));
if let Ok(status) = git_provider.fetch_status() {
    app.git_status = Some(status);
}
```

**Step 6: Run and verify**

Run: `cargo run`
Expected: Git panel shows branch, file counts, and recent commits

**Step 7: Commit**

```bash
git add src/
git commit -m "feat: add Git panel with commit history and branch status"
```

---

## Task 6: Async Data Refresh Loop

**Owner:** Zen（バックエンド）
**Reviewer:** Sora（アーキテクト）, Ryu（QA）

**Files:**
- Create: `src/data/fetcher.rs`
- Modify: `src/main.rs`
- Modify: `src/app/state.rs`
- Test: `tests/fetcher_test.rs`

**Step 1: Write failing test for data message channel**

```rust
// tests/fetcher_test.rs
use dev_dashboard::data::fetcher::DataMessage;

#[tokio::test]
async fn test_data_message_channel() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<DataMessage>(32);
    tx.send(DataMessage::GitUpdated(None)).await.unwrap();
    let msg = rx.recv().await.unwrap();
    assert!(matches!(msg, DataMessage::GitUpdated(_)));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test fetcher_test`
Expected: FAIL

**Step 3: Implement fetcher**

```rust
// src/data/fetcher.rs
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;

use super::git_provider::GitProvider;
use super::models::GitStatus;

#[derive(Debug)]
pub enum DataMessage {
    GitUpdated(Option<GitStatus>),
}

pub fn spawn_git_fetcher(tx: mpsc::Sender<DataMessage>, path: PathBuf) {
    tokio::spawn(async move {
        let provider = GitProvider::new(path);
        loop {
            let status = provider.fetch_status().ok();
            let _ = tx.send(DataMessage::GitUpdated(status)).await;
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
}
```

**Step 4: Update src/data/mod.rs**

```rust
pub mod models;
pub mod git_provider;
pub mod fetcher;
```

**Step 5: Integrate into main.rs**

```rust
// src/main.rs
use anyhow::Result;
use std::path::PathBuf;
use tokio::sync::mpsc;

mod app;
mod ui;
mod data;

use app::state::App;
use app::event::handle_events;
use ui::terminal;
use ui::layout;
use data::fetcher::{self, DataMessage};

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = terminal::init()?;
    let mut app = App::new();

    let (tx, mut rx) = mpsc::channel::<DataMessage>(32);

    fetcher::spawn_git_fetcher(tx.clone(), PathBuf::from("."));

    loop {
        // Drain all pending data updates
        while let Ok(msg) = rx.try_recv() {
            match msg {
                DataMessage::GitUpdated(status) => {
                    app.git_status = status;
                }
            }
        }

        terminal.draw(|frame| {
            layout::render(frame, &app);
        })?;

        handle_events(&mut app)?;

        if app.should_quit {
            break;
        }
    }

    terminal::restore()?;
    Ok(())
}
```

**Step 6: Run tests and verify**

Run: `cargo test --test fetcher_test`
Expected: PASS

Run: `cargo run`
Expected: Dashboard with auto-refreshing git data every 5 seconds

**Step 7: Commit**

```bash
git add src/ tests/
git commit -m "feat: add async data fetcher with mpsc channel architecture"
```

---

## Task 7: CI/CD Panel (GitHub Actions)

**Owner:** Zen（バックエンド）
**Reviewer:** Sora（アーキテクト）

**Files:**
- Create: `src/data/ci_provider.rs`
- Create: `src/ui/panels/ci_panel.rs`
- Modify: `src/data/models.rs`
- Modify: `src/data/fetcher.rs`
- Modify: `src/data/mod.rs`
- Modify: `src/ui/panels/mod.rs`
- Modify: `src/app/state.rs`
- Test: `tests/ci_provider_test.rs`

**Step 1: Add CI/CD data models**

```rust
// Add to src/data/models.rs
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

#[derive(Debug, Clone)]
pub struct CiStatus {
    pub runs: Vec<PipelineRun>,
}
```

**Step 2: Write failing test**

```rust
// tests/ci_provider_test.rs
use dev_dashboard::data::models::{PipelineStatus, PipelineRun, CiStatus};

#[test]
fn test_pipeline_status_display() {
    let run = PipelineRun {
        id: "123".to_string(),
        name: "CI".to_string(),
        status: PipelineStatus::Success,
        branch: "main".to_string(),
        duration_secs: Some(120),
        started_at: None,
    };
    assert!(matches!(run.status, PipelineStatus::Success));
}
```

**Step 3: Implement CI provider trait**

```rust
// src/data/ci_provider.rs
use anyhow::Result;
use super::models::CiStatus;

#[async_trait::async_trait]
pub trait CiProvider: Send + Sync {
    async fn fetch_status(&self) -> Result<CiStatus>;
}

// Mock provider for initial development
pub struct MockCiProvider;

#[async_trait::async_trait]
impl CiProvider for MockCiProvider {
    async fn fetch_status(&self) -> Result<CiStatus> {
        use super::models::{PipelineRun, PipelineStatus};
        Ok(CiStatus {
            runs: vec![
                PipelineRun {
                    id: "1".to_string(),
                    name: "Build & Test".to_string(),
                    status: PipelineStatus::Success,
                    branch: "main".to_string(),
                    duration_secs: Some(94),
                    started_at: None,
                },
                PipelineRun {
                    id: "2".to_string(),
                    name: "Deploy".to_string(),
                    status: PipelineStatus::Running,
                    branch: "main".to_string(),
                    duration_secs: None,
                    started_at: None,
                },
            ],
        })
    }
}
```

**Step 4: Create CI panel UI**

```rust
// src/ui/panels/ci_panel.rs
use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::data::models::{CiStatus, PipelineStatus};

pub fn render(frame: &mut Frame, area: Rect, status: &Option<CiStatus>, is_active: bool) {
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(" CI/CD ");

    match status {
        Some(status) => {
            let inner = block.inner(area);
            frame.render_widget(block, area);

            let rows: Vec<Row> = status
                .runs
                .iter()
                .map(|run| {
                    let (icon, color) = match run.status {
                        PipelineStatus::Success => ("✓", Color::Green),
                        PipelineStatus::Failure => ("✗", Color::Red),
                        PipelineStatus::Running => ("●", Color::Yellow),
                        PipelineStatus::Pending => ("○", Color::DarkGray),
                        PipelineStatus::Cancelled => ("—", Color::DarkGray),
                    };

                    let duration = run
                        .duration_secs
                        .map(|d| format!("{}s", d))
                        .unwrap_or_else(|| "—".to_string());

                    Row::new(vec![
                        Cell::from(icon).style(Style::default().fg(color)),
                        Cell::from(run.name.clone()),
                        Cell::from(run.branch.clone()).style(Style::default().fg(Color::Blue)),
                        Cell::from(duration),
                    ])
                })
                .collect();

            let table = Table::new(
                rows,
                [
                    Constraint::Length(2),
                    Constraint::Min(15),
                    Constraint::Length(12),
                    Constraint::Length(6),
                ],
            )
            .header(
                Row::new(vec!["", "Pipeline", "Branch", "Time"])
                    .style(Style::default().fg(Color::DarkGray)),
            );

            frame.render_widget(table, inner);
        }
        None => {
            let loading = Paragraph::new("Fetching CI/CD data...")
                .block(block)
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(loading, area);
        }
    }
}
```

**Step 5: Wire into App state, fetcher, and layout (follow same pattern as Git)**

Add `ci_status: Option<CiStatus>` to App, add `DataMessage::CiUpdated`, update layout to call `ci_panel::render`.

**Step 6: Run tests and verify**

Run: `cargo test`
Expected: All PASS

**Step 7: Commit**

```bash
git add src/ tests/
git commit -m "feat: add CI/CD panel with pipeline status display"
```

---

## Task 8: Tasks Panel & Quality Panel

**Owner:** Zen（バックエンド）+ Rin（デザイナー）
**Reviewer:** Ryu（QA）

Same pattern as Tasks 4-7. Create:
- `src/data/task_provider.rs` — mock provider with trait
- `src/data/quality_provider.rs` — reads local coverage/lint files
- `src/ui/panels/task_panel.rs` — task list with status icons
- `src/ui/panels/quality_panel.rs` — coverage gauge + lint count
- Models: `TaskItem`, `TaskStatus`, `QualityMetrics`
- Tests for each provider

**Step 1-7:** Follow TDD pattern from Tasks 4-7.

**Commit:**

```bash
git commit -m "feat: add Tasks and Quality panels with mock data"
```

---

## Task 9: CLI Arguments & Configuration

**Owner:** Zen（バックエンド）
**Reviewer:** Mio（DevRel）for help text

**Files:**
- Create: `src/app/cli.rs`
- Modify: `src/main.rs`

**Step 1: Define CLI with clap**

```rust
// src/app/cli.rs
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "dev-dashboard")]
#[command(about = "Terminal dashboard for development status")]
#[command(version)]
pub struct Cli {
    /// Path to git repository (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,

    /// Refresh interval in seconds
    #[arg(short, long, default_value = "5")]
    pub refresh: u64,
}
```

**Step 2: Wire into main.rs**

**Step 3: Commit**

```bash
git commit -m "feat: add CLI argument parsing with clap"
```

---

## Task 10: README & Documentation

**Owner:** Mio（DevRel）
**Reviewer:** Kai（PM）

**Files:**
- Create: `README.md`

**Step 1: Write README**

Include: project description, screenshot placeholder, install instructions, usage, keybindings, configuration, contributing guide.

Mio's rule: install → first success in 3 minutes.

**Step 2: Commit**

```bash
git commit -m "docs: add README with installation and usage guide"
```

---

## Task 11: Integration Tests & QA Pass

**Owner:** Ryu（QA）
**Reviewer:** Zen（バックエンド）

**Files:**
- Create: `tests/integration_test.rs`

**Step 1: Test CLI starts and exits cleanly**

```rust
use assert_cmd::Command;

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("dev-dashboard").unwrap();
    cmd.arg("--version").assert().success();
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("dev-dashboard").unwrap();
    cmd.arg("--help").assert().success();
}
```

**Step 2: Run full test suite**

Run: `cargo test`
Expected: All PASS

**Step 3: Ryu's QA checklist**

- [ ] All tests pass
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Format check: `cargo fmt --check`
- [ ] Invalid repo path → graceful error
- [ ] Terminal resize → layout adapts
- [ ] Ctrl+C → clean exit

**Step 4: Commit**

```bash
git commit -m "test: add integration tests and QA verification"
```
