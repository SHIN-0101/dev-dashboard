use dev_dashboard::app::state::{ActivePanel, App};

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
