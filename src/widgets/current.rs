use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph},
    Frame,
};

use crate::models::CurrentWeather;
use crate::wmo;

pub fn temp_color(temp: f64) -> Color {
    if temp < 10.0 {
        Color::LightBlue
    } else if temp < 20.0 {
        Color::Cyan
    } else if temp < 25.0 {
        Color::Green
    } else if temp < 30.0 {
        Color::Yellow
    } else if temp < 35.0 {
        Color::LightRed
    } else {
        Color::Red
    }
}

/// Handcrafted clean 5-line block numerals for crisp terminal readability.
fn digit_glyph(c: char) -> [&'static str; 5] {
    match c {
        '0' => [
            "████",
            "█  █",
            "█  █",
            "█  █",
            "████",
        ],
        '1' => [
            "  ██",
            "  ██",
            "  ██",
            "  ██",
            "  ██",
        ],
        '2' => [
            "████",
            "   █",
            "████",
            "█   ",
            "████",
        ],
        '3' => [
            "████",
            "   █",
            "████",
            "   █",
            "████",
        ],
        '4' => [
            "█  █",
            "█  █",
            "████",
            "   █",
            "   █",
        ],
        '5' => [
            "████",
            "█   ",
            "████",
            "   █",
            "████",
        ],
        '6' => [
            "████",
            "█   ",
            "████",
            "█  █",
            "████",
        ],
        '7' => [
            "████",
            "   █",
            "   █",
            "   █",
            "   █",
        ],
        '8' => [
            "████",
            "█  █",
            "████",
            "█  █",
            "████",
        ],
        '9' => [
            "████",
            "█  █",
            "████",
            "   █",
            "████",
        ],
        '-' => [
            "    ",
            "    ",
            "████",
            "    ",
            "    ",
        ],
        _ => [
            "    ",
            "    ",
            "    ",
            "    ",
            "    ",
        ],
    }
}

pub fn render(f: &mut Frame, area: Rect, current: &CurrentWeather) {
    let is_day = current.is_day != 0;
    let (desc, icon) = wmo::wmo_description(current.weather_code, is_day);

    let block = Block::default()
        .title(" CLIMA ATUAL ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // If height is tight (< 7 lines), fallback to single-line crisp temperature
    if inner_area.height < 7 {
        render_compact(f, inner_area, current, icon, desc);
        return;
    }

    let chunks = Layout::vertical([
        Constraint::Length(1), // Condição (topo)
        Constraint::Length(5), // Dígitos em Bloco (5 linhas)
        Constraint::Length(1), // Sensação (base)
    ])
    .split(inner_area);

    // 1. Condição
    let color = temp_color(current.temperature_2m);
    let condition = Paragraph::new(Line::from(vec![
        Span::styled(format!("{} ", icon), Style::default().fg(color)),
        Span::styled(
            desc.to_uppercase(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(condition, chunks[0]);

    // 2. Dígitos em bloco nítidos com sobrescrito "°C"
    let temp_rounded = current.temperature_2m.round() as i32;
    let temp_str = temp_rounded.to_string();
    let color = temp_color(current.temperature_2m);

    let glyphs: Vec<[&'static str; 5]> = temp_str.chars().map(digit_glyph).collect();

    let mut lines = Vec::with_capacity(5);
    for row in 0..5 {
        let mut spans = Vec::new();

        for (i, glyph) in glyphs.iter().enumerate() {
            spans.push(Span::styled(
                glyph[row],
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ));
            if i + 1 < glyphs.len() {
                spans.push(Span::raw(" "));
            }
        }

        // Adiciona "°C" no topo direito dos dígitos para legibilidade cristalina
        if row == 0 {
            spans.push(Span::raw(" "));
            spans.push(Span::styled(
                "°C",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::raw("   "));
        }

        lines.push(Line::from(spans));
    }

    let digits_p = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(digits_p, chunks[1]);

    // 3. Sensação
    let apparent = Paragraph::new(format!("Sensação: {:.0}°C", current.apparent_temperature))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(apparent, chunks[2]);
}

fn render_compact(f: &mut Frame, area: Rect, current: &CurrentWeather, icon: &str, desc: &str) {
    let color = temp_color(current.temperature_2m);
    let lines = vec![
        Line::from(vec![
            Span::styled(format!("{} ", icon), Style::default()),
            Span::styled(
                desc.to_uppercase(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
        .alignment(Alignment::Center),
        Line::from(vec![
            Span::styled(
                format!("{:.0}°C", current.temperature_2m),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  (Sensação: {:.0}°C)", current.apparent_temperature),
                Style::default().fg(Color::DarkGray),
            ),
        ])
        .alignment(Alignment::Center),
    ];

    f.render_widget(Paragraph::new(lines), area);
}
