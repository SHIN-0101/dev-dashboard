use crate::data::models::{TaskStatus, TasksStatus};
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame, area: Rect, status: &Option<TasksStatus>, is_active: bool) {
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(" Tasks ");

    match status {
        Some(status) => {
            let inner = block.inner(area);
            frame.render_widget(block, area);

            let rows: Vec<Row> = status
                .tasks
                .iter()
                .map(|task| {
                    let (icon, color) = match task.status {
                        TaskStatus::Todo => ("○", Color::DarkGray),
                        TaskStatus::InProgress => ("●", Color::Yellow),
                        TaskStatus::Done => ("✓", Color::Green),
                        TaskStatus::Blocked => ("✗", Color::Red),
                    };

                    let assignee = task.assignee.clone().unwrap_or_else(|| "—".to_string());

                    Row::new(vec![
                        Cell::from(icon).style(Style::default().fg(color)),
                        Cell::from(task.title.clone()),
                        Cell::from(assignee).style(Style::default().fg(Color::Blue)),
                    ])
                })
                .collect();

            let table = Table::new(
                rows,
                [
                    Constraint::Length(2),
                    Constraint::Min(20),
                    Constraint::Length(10),
                ],
            )
            .header(
                Row::new(vec!["", "Task", "Assignee"]).style(Style::default().fg(Color::DarkGray)),
            );

            frame.render_widget(table, inner);
        }
        None => {
            let loading = Paragraph::new("Fetching tasks...")
                .block(block)
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(loading, area);
        }
    }
}
