use anyhow::Result;
use clap::Parser;
use tokio::sync::mpsc;

mod app;
mod data;
mod ui;
mod web;

use app::cli::Cli;
use app::event::handle_events;
use app::state::App;
use data::fetcher::{self, DataMessage};
use data::github_config::GitHubConfig;
use ui::layout;
use ui::terminal;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let github_config = GitHubConfig::resolve(
        &cli.path,
        cli.owner.as_deref(),
        cli.repo.as_deref(),
        cli.token.as_deref(),
    )
    .ok();

    if cli.web {
        // Web mode: start API server
        web::api::run_server(cli.path, cli.refresh, github_config).await?;
    } else {
        // TUI mode (existing behavior)
        let mut terminal = terminal::init()?;
        let mut app = App::new();

        let (tx, mut rx) = mpsc::channel::<DataMessage>(32);

        fetcher::spawn_git_fetcher(tx.clone(), cli.path.clone(), cli.refresh);

        if let Some(ref config) = github_config {
            fetcher::spawn_ci_fetcher(
                tx.clone(),
                config.owner.clone(),
                config.repo.clone(),
                config.token.clone(),
                cli.refresh,
            );
            fetcher::spawn_task_fetcher(
                tx.clone(),
                config.owner.clone(),
                config.repo.clone(),
                config.token.clone(),
                cli.refresh,
            );
        }

        fetcher::spawn_quality_fetcher(tx.clone(), cli.path.clone(), cli.refresh);

        loop {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    DataMessage::GitUpdated(status) => app.git_status = status,
                    DataMessage::CiUpdated(status) => app.ci_status = status,
                    DataMessage::TasksUpdated(status) => app.tasks_status = status,
                    DataMessage::QualityUpdated(metrics) => app.quality_metrics = metrics,
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
    }

    Ok(())
}
