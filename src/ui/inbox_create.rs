use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Clear, Paragraph, Wrap},
};

use crate::app::{App, FormField};

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    // Centered popup: 70% wide, 80% tall
    let popup_area = centered_rect(70, 80, area);

    // Clear background behind popup
    f.render_widget(Clear, popup_area);

    let outer_block = Block::default()
        .title(" New Inbox ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));
    f.render_widget(outer_block, popup_area);

    // Inner area with margin
    let inner = Rect {
        x: popup_area.x + 2,
        y: popup_area.y + 1,
        width: popup_area.width.saturating_sub(4),
        height: popup_area.height.saturating_sub(3),
    };

    // Build per-field constraints: label(1) + input(3) for 5 fields + status(1)
    let fields = FormField::all();
    let mut constraints: Vec<Constraint> = fields
        .iter()
        .flat_map(|_| [Constraint::Length(1), Constraint::Length(3)])
        .collect();
    constraints.push(Constraint::Length(1)); // status line
    constraints.push(Constraint::Min(0));    // padding

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    for (i, field) in fields.iter().enumerate() {
        let label_idx = i * 2;
        let input_idx = i * 2 + 1;
        let is_active = app.form.active_field == i;

        // Label
        let label_style = if is_active {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let label = Paragraph::new(field.label()).style(label_style);
        f.render_widget(label, chunks[label_idx]);

        // Input box
        let value = app.form.value_for(field);
        let display = if value.is_empty() && !is_active {
            Span::styled("(empty)", Style::default().fg(Color::DarkGray))
        } else if is_active {
            // Show cursor indicator
            Span::styled(
                format!("{}█", value),
                Style::default().fg(Color::White),
            )
        } else {
            Span::styled(value, Style::default().fg(Color::White))
        };

        let border_style = if is_active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let input = Paragraph::new(Line::from(display))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(border_style),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(input, chunks[input_idx]);
    }

    // Status / error line
    let status_idx = fields.len() * 2;
    let status_text = if let Some((msg, is_error)) = &app.status {
        let style = if *is_error {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Green)
        };
        Line::from(Span::styled(msg.as_str(), style))
    } else {
        Line::from(vec![
            Span::styled("[Tab] ", Style::default().fg(Color::Cyan)),
            Span::raw("Next field  "),
            Span::styled("[Shift+Tab] ", Style::default().fg(Color::Cyan)),
            Span::raw("Prev  "),
            Span::styled("[Enter] ", Style::default().fg(Color::Cyan)),
            Span::raw("Submit  "),
            Span::styled("[Esc] ", Style::default().fg(Color::Cyan)),
            Span::raw("Cancel"),
        ])
    };
    let status = Paragraph::new(status_text);
    f.render_widget(status, chunks[status_idx]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
