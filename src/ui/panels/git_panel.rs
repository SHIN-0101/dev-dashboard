use crate::data::models::GitStatus;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame, area: Rect, status: &Option<GitStatus>, is_active: bool) {
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(" Git ");

    match status {
        Some(status) => {
            let inner = block.inner(area);
            frame.render_widget(block, area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(2), Constraint::Min(0)])
                .split(inner);

            let branch_line = Line::from(vec![
                Span::styled("branch: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    &status.branch,
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("  "),
                Span::styled(
                    format!("{}M {}S", status.changed_files, status.staged_files),
                    Style::default().fg(Color::Yellow),
                ),
            ]);
            frame.render_widget(Paragraph::new(branch_line), chunks[0]);

            let rows: Vec<Row> = status
                .commits
                .iter()
                .take(10)
                .map(|c| {
                    Row::new(vec![
                        Cell::from(c.hash.clone()).style(Style::default().fg(Color::Yellow)),
                        Cell::from(c.message.clone()),
                        Cell::from(c.author.clone()).style(Style::default().fg(Color::Blue)),
                    ])
                })
                .collect();

            let table = Table::new(
                rows,
                [
                    Constraint::Length(8),
                    Constraint::Min(20),
                    Constraint::Length(12),
                ],
            )
            .header(
                Row::new(vec!["Hash", "Message", "Author"])
                    .style(Style::default().fg(Color::DarkGray)),
            );

            frame.render_widget(table, chunks[1]);
        }
        None => {
            let loading = Paragraph::new("Fetching git data...")
                .block(block)
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(loading, area);
        }
    }
}
