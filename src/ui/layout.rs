use ratatui::prelude::*;
use ratatui::widgets::*;

use super::panels::ci_panel;
use super::panels::git_panel;
use super::panels::quality_panel;
use super::panels::task_panel;
use crate::app::state::{ActivePanel, App};

pub fn render(frame: &mut Frame, app: &App) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" dev-dashboard "),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
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

    // Render Git panel with real data
    git_panel::render(
        frame,
        left[0],
        &app.git_status,
        app.active_panel == ActivePanel::Git,
    );

    // Render CI/CD panel with real data
    ci_panel::render(
        frame,
        right[0],
        &app.ci_status,
        app.active_panel == ActivePanel::CiCd,
    );

    // Render Tasks panel with real data
    task_panel::render(
        frame,
        left[1],
        &app.tasks_status,
        app.active_panel == ActivePanel::Tasks,
    );

    // Render Quality panel with real data
    quality_panel::render(
        frame,
        right[1],
        &app.quality_metrics,
        app.active_panel == ActivePanel::Quality,
    );
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new(" [1-4] Panel  [Tab] Next  [q] Quit")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, area);
}
