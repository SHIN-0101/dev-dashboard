use crate::data::models::QualityMetrics;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame, area: Rect, metrics: &Option<QualityMetrics>, is_active: bool) {
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(" Quality ");

    match metrics {
        Some(metrics) => {
            let inner = block.inner(area);
            frame.render_widget(block, area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2), // coverage gauge
                    Constraint::Length(1), // spacer
                    Constraint::Min(0),    // stats
                ])
                .split(inner);

            // Coverage gauge
            let coverage_color = if metrics.test_coverage > 80.0 {
                Color::Green
            } else if metrics.test_coverage > 60.0 {
                Color::Yellow
            } else {
                Color::Red
            };

            let gauge = Gauge::default()
                .block(Block::default().title("Coverage"))
                .gauge_style(Style::default().fg(coverage_color))
                .percent(metrics.test_coverage.min(100.0) as u16)
                .label(format!("{:.1}%", metrics.test_coverage));

            frame.render_widget(gauge, chunks[0]);

            // Stats
            let warning_color = if metrics.lint_warnings > 0 {
                Color::Yellow
            } else {
                Color::Green
            };

            let error_color = if metrics.lint_errors > 0 {
                Color::Red
            } else {
                Color::Green
            };

            let security_color = if metrics.security_issues > 0 {
                Color::Red
            } else {
                Color::Green
            };

            let stats = vec![
                Line::from(vec![
                    Span::styled("Lint warnings: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        metrics.lint_warnings.to_string(),
                        Style::default().fg(warning_color),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Lint errors:   ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        metrics.lint_errors.to_string(),
                        Style::default().fg(error_color),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Security:      ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        metrics.security_issues.to_string(),
                        Style::default().fg(security_color),
                    ),
                ]),
            ];

            frame.render_widget(Paragraph::new(stats), chunks[2]);
        }
        None => {
            let loading = Paragraph::new("Fetching quality metrics...")
                .block(block)
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(loading, area);
        }
    }
}
