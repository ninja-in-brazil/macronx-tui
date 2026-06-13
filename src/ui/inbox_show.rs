use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph, Wrap},
};

use crate::app::App;
use crate::models::Inbox;

pub fn render(f: &mut Frame, app: &App) {
    let inbox = match &app.current_inbox {
        Some(i) => i,
        None => return,
    };

    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(10),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    // Breadcrumb header
    let header = Paragraph::new(Line::from(vec![
        Span::styled("← Inbox", Style::default().fg(Color::DarkGray)),
        Span::raw(" / "),
        Span::styled(
            inbox.name.as_str(),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(header, chunks[0]);

    // Details block
    render_details(f, inbox, chunks[1]);

    // Payload + Metadata side by side
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    render_json_block(f, "Payload", &inbox.payload, bottom_chunks[0]);
    render_json_block(f, "Metadata", &inbox.metadata, bottom_chunks[1]);

    // Status bar
    let status_text = if let Some((msg, is_error)) = &app.status {
        let style = if *is_error {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Green)
        };
        Line::from(Span::styled(format!(" {}", msg), style))
    } else {
        Line::from(vec![
            Span::styled(" [Esc/q] ", Style::default().fg(Color::Cyan)),
            Span::raw("Back to list"),
        ])
    };

    let status_bar = Paragraph::new(status_text).style(Style::default().bg(Color::Black));
    f.render_widget(status_bar, chunks[3]);
}

fn render_details(f: &mut Frame, inbox: &Inbox, area: ratatui::layout::Rect) {
    let label_style = Style::default().fg(Color::DarkGray);
    let value_style = Style::default().fg(Color::White);

    let lines = vec![
        Line::from(vec![
            Span::styled("  Name       ", label_style),
            Span::styled(inbox.name.as_str(), value_style.add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  Source     ", label_style),
            Span::styled(inbox.source.as_str(), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  Summary    ", label_style),
            Span::styled(inbox.summary.as_deref().unwrap_or("—"), value_style),
        ]),
        Line::from(vec![
            Span::styled("  Created    ", label_style),
            Span::styled(format_datetime(&inbox.created_at), value_style),
        ]),
        Line::from(vec![
            Span::styled("  Updated    ", label_style),
            Span::styled(format_datetime(&inbox.updated_at), value_style),
        ]),
    ];

    let details = Paragraph::new(lines).block(
        Block::default()
            .title(" Details ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(details, area);
}

fn render_json_block(
    f: &mut Frame,
    title: &str,
    value: &serde_json::Value,
    area: ratatui::layout::Rect,
) {
    let json_str = serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string());
    let paragraph = Paragraph::new(json_str)
        .style(Style::default().fg(Color::Green))
        .block(
            Block::default()
                .title(format!(" {} ", title))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}

fn format_datetime(s: &str) -> String {
    // "2026-06-12T20:40:00.000Z" → "June 12, 2026 at 20:40 UTC"
    if s.len() >= 16 {
        let date = &s[..10];
        let time = &s[11..16];
        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() == 3 {
            let month = match parts[1] {
                "01" => "January",
                "02" => "February",
                "03" => "March",
                "04" => "April",
                "05" => "May",
                "06" => "June",
                "07" => "July",
                "08" => "August",
                "09" => "September",
                "10" => "October",
                "11" => "November",
                "12" => "December",
                _ => parts[1],
            };
            let day = parts[2].trim_start_matches('0');
            return format!("{} {}, {} at {} UTC", month, day, parts[0], time);
        }
    }
    s.to_string()
}
