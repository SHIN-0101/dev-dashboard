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
    #[arg(short = 'i', long, default_value = "5")]
    pub refresh: u64,

    /// GitHub repository owner (auto-detected from git remote if not specified)
    #[arg(short, long)]
    pub owner: Option<String>,

    /// GitHub repository name (auto-detected from git remote if not specified)
    #[arg(short = 'n', long)]
    pub repo: Option<String>,

    /// GitHub token (defaults to `gh auth token` output or GITHUB_TOKEN env var)
    #[arg(long)]
    pub token: Option<String>,

    /// Run in web mode (starts API server instead of TUI)
    #[arg(short, long)]
    pub web: bool,
}
