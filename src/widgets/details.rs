use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph},
    Frame,
};

use crate::models::{CurrentWeather, DailyForecast};
use crate::wmo;

/// Renders the "details" panel with surgical grid alignment and rock-solid borders.
pub fn render(f: &mut Frame, area: Rect, current: &CurrentWeather, daily: &DailyForecast) {
    let block = Block::default()
        .title(" DETALHES ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if inner.height < 4 || inner.width < 25 {
        return;
    }

    let wind_dir = wmo::wind_direction_label(current.wind_direction_10m);
    let uv = daily.uv_index_max.first().copied().unwrap_or(0.0);
    let uv_text = wmo::uv_label(uv);

    let sunrise = daily
        .sunrise
        .first()
        .map(|s| extract_time(s))
        .unwrap_or_else(|| "--:--".into());
    let sunset = daily
        .sunset
        .first()
        .map(|s| extract_time(s))
        .unwrap_or_else(|| "--:--".into());

    let humidity = current.relative_humidity_2m.clamp(0.0, 100.0);

    // Todos os rótulos têm exatamente o mesmo comprimento visual (14 caracteres)
    // Todos os valores têm exatamente 6 caracteres
    // Garantindo que colchetes, barras e textos comecem rigorosamente na mesma coluna!
    let mut lines = Vec::new();

    // 1. Vento
    lines.push(Line::from(vec![
        Span::styled("  ● ", Style::default().fg(Color::Cyan)),
        Span::styled("Vento:       ", Style::default().fg(Color::White)),
        Span::styled(
            format!("{:>4.1} km/h", current.wind_speed_10m),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ("),
        Span::styled(wind_dir, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(")"),
    ]));

    // 2. Umidade
    let mut hum_spans = vec![
        Span::styled("  ● ", Style::default().fg(Color::LightBlue)),
        Span::styled("Umidade:     ", Style::default().fg(Color::White)),
        Span::styled(
            format!("{:>4.0}%  ", humidity),
            Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD),
        ),
    ];
    hum_spans.extend(render_mini_bar(humidity, 100.0, 10, Color::LightBlue));
    lines.push(Line::from(hum_spans));

    // 3. Índice UV
    let uv_c = uv_color(uv);
    let mut uv_spans = vec![
        Span::styled("  ● ", Style::default().fg(uv_c)),
        Span::styled("UV:          ", Style::default().fg(Color::White)),
        Span::styled(
            format!("{:>4.1}   ", uv),
            Style::default().fg(uv_c).add_modifier(Modifier::BOLD),
        ),
    ];
    uv_spans.extend(render_mini_bar(uv, 12.0, 10, uv_c));
    if inner.width >= 48 {
        uv_spans.push(Span::raw(" "));
        uv_spans.push(Span::styled(
            format!("({uv_text})"),
            Style::default().fg(uv_c).add_modifier(Modifier::BOLD),
        ));
    }
    lines.push(Line::from(uv_spans));

    // 4. Sol (Nascer e Pôr)
    let mut sol_spans = vec![
        Span::styled("  ● ", Style::default().fg(Color::Yellow)),
        Span::styled("Sol:         ", Style::default().fg(Color::White)),
        Span::styled(sunrise, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ];
    if inner.width >= 46 {
        sol_spans.push(Span::raw(" (Nascer)  •  "));
        sol_spans.push(Span::styled(sunset, Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)));
        sol_spans.push(Span::raw(" (Pôr)"));
    } else {
        sol_spans.push(Span::raw(" ↑  •  "));
        sol_spans.push(Span::styled(sunset, Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)));
        sol_spans.push(Span::raw(" ↓"));
    }
    lines.push(Line::from(sol_spans));

    let p = Paragraph::new(lines);
    f.render_widget(p, inner);
}

/// Creates a sleek, high-contrast terminal progress bar like `[████████░░]`
fn render_mini_bar(value: f64, max: f64, len: usize, color: Color) -> Vec<Span<'static>> {
    let ratio = (value / max).clamp(0.0, 1.0);
    let filled = (ratio * len as f64).round() as usize;
    let empty = len.saturating_sub(filled);

    vec![
        Span::styled("[", Style::default().fg(Color::DarkGray)),
        Span::styled("█".repeat(filled), Style::default().fg(color)),
        Span::styled("░".repeat(empty), Style::default().fg(Color::DarkGray)),
        Span::styled("]", Style::default().fg(Color::DarkGray)),
    ]
}

fn extract_time(iso: &str) -> String {
    iso.split('T')
        .nth(1)
        .unwrap_or("--:--")
        .chars()
        .take(5)
        .collect()
}

fn uv_color(uv: f64) -> Color {
    match uv as u8 {
        0..=2 => Color::Green,
        3..=5 => Color::Yellow,
        6..=7 => Color::LightRed,
        _ => Color::Red,
    }
}
