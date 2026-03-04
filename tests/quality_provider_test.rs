use dev_dashboard::data::models::QualityMetrics;

#[test]
fn test_quality_metrics() {
    let metrics = QualityMetrics {
        test_coverage: 85.5,
        lint_warnings: 3,
        lint_errors: 0,
        security_issues: 0,
    };
    assert!(metrics.test_coverage > 80.0);
    assert_eq!(metrics.lint_errors, 0);
}
