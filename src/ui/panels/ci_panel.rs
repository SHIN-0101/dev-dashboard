use crate::data::models::{CiStatus, PipelineStatus};
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame, area: Rect, status: &Option<CiStatus>, is_active: bool) {
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(" CI/CD ");

    match status {
        Some(status) => {
            let inner = block.inner(area);
            frame.render_widget(block, area);

            let rows: Vec<Row> = status
                .runs
                .iter()
                .map(|run| {
                    let (icon, color) = match run.status {
                        PipelineStatus::Success => ("✓", Color::Green),
                        PipelineStatus::Failure => ("✗", Color::Red),
                        PipelineStatus::Running => ("●", Color::Yellow),
                        PipelineStatus::Pending => ("○", Color::DarkGray),
                        PipelineStatus::Cancelled => ("—", Color::DarkGray),
                    };

                    let duration = run
                        .duration_secs
                        .map(|d| format!("{}s", d))
                        .unwrap_or_else(|| "—".to_string());

                    Row::new(vec![
                        Cell::from(icon).style(Style::default().fg(color)),
                        Cell::from(run.name.clone()),
                        Cell::from(run.branch.clone()).style(Style::default().fg(Color::Blue)),
                        Cell::from(duration),
                    ])
                })
                .collect();

            let table = Table::new(
                rows,
                [
                    Constraint::Length(2),
                    Constraint::Min(15),
                    Constraint::Length(12),
                    Constraint::Length(6),
                ],
            )
            .header(
                Row::new(vec!["", "Pipeline", "Branch", "Time"])
                    .style(Style::default().fg(Color::DarkGray)),
            );

            frame.render_widget(table, inner);
        }
        None => {
            let loading = Paragraph::new("Fetching CI/CD data...")
                .block(block)
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(loading, area);
        }
    }
}
