use dev_dashboard::data::models::{PipelineRun, PipelineStatus};

#[test]
fn test_pipeline_status_variants() {
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
