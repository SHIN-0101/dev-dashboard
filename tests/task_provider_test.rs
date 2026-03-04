use dev_dashboard::data::models::{TaskItem, TaskStatus};

#[test]
fn test_task_status_variants() {
    let task = TaskItem {
        id: "1".to_string(),
        title: "Test task".to_string(),
        status: TaskStatus::InProgress,
        assignee: Some("Zen".to_string()),
    };
    assert!(matches!(task.status, TaskStatus::InProgress));
}
