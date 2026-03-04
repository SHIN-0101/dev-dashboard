use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct GitHubConfig {
    pub owner: String,
    pub repo: String,
    pub token: String,
}

impl GitHubConfig {
    pub fn resolve(
        path: &Path,
        owner: Option<&str>,
        repo: Option<&str>,
        token: Option<&str>,
    ) -> Result<Self> {
        let token = match token {
            Some(t) => t.to_string(),
            None => Self::detect_token()?,
        };

        let (detected_owner, detected_repo) = Self::detect_from_remote(path)?;

        Ok(Self {
            owner: owner.unwrap_or(&detected_owner).to_string(),
            repo: repo.unwrap_or(&detected_repo).to_string(),
            token,
        })
    }

    fn detect_token() -> Result<String> {
        // Try GITHUB_TOKEN env var first
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            if !token.is_empty() {
                return Ok(token);
            }
        }

        // Fall back to `gh auth token`
        let output = Command::new("gh")
            .args(["auth", "token"])
            .output()
            .context("Failed to run `gh auth token`. Install GitHub CLI or set GITHUB_TOKEN")?;

        let token = String::from_utf8(output.stdout)
            .context("Invalid token output")?
            .trim()
            .to_string();

        if token.is_empty() {
            anyhow::bail!("No GitHub token found. Run `gh auth login` or set GITHUB_TOKEN");
        }

        Ok(token)
    }

    fn detect_from_remote(path: &Path) -> Result<(String, String)> {
        let repo = git2::Repository::open(path).context("Not a git repository")?;
        let remote = repo
            .find_remote("origin")
            .context("No 'origin' remote found")?;
        let url = remote.url().context("Remote URL is not valid UTF-8")?;

        Self::parse_github_url(url)
    }

    fn parse_github_url(url: &str) -> Result<(String, String)> {
        // Handle: https://github.com/owner/repo.git
        //         git@github.com:owner/repo.git
        let url = url.trim_end_matches(".git");

        if let Some(path) = url.strip_prefix("https://github.com/") {
            let parts: Vec<&str> = path.splitn(2, '/').collect();
            if parts.len() == 2 {
                return Ok((parts[0].to_string(), parts[1].to_string()));
            }
        }

        if let Some(path) = url.strip_prefix("git@github.com:") {
            let parts: Vec<&str> = path.splitn(2, '/').collect();
            if parts.len() == 2 {
                return Ok((parts[0].to_string(), parts[1].to_string()));
            }
        }

        anyhow::bail!("Could not parse GitHub owner/repo from remote URL: {}", url)
    }
}
