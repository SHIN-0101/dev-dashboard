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
