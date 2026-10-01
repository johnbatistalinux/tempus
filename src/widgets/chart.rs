use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, BorderType},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::models::{DailyForecast, HourlyForecast};
use crate::widgets::current::temp_color;
use crate::wmo;

/// Renders the hourly temperature chart for the selected day (left) and 14-day precipitation chart (right).
pub fn render(
    f: &mut Frame,
    area: Rect,
    hourly: &HourlyForecast,
    daily: &DailyForecast,
    selected_day: usize,
) {
    let chunks = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_chunk = Rect {
        width: chunks[0].width.saturating_sub(1),
        ..chunks[0]
    };
    let right_chunk = Rect {
        x: chunks[1].x + 1,
        width: chunks[1].width.saturating_sub(1),
        ..chunks[1]
    };

    render_temp_columns(f, left_chunk, hourly, daily, selected_day);
    render_precip_columns(f, right_chunk, daily, selected_day);
}

/// Renders a beautifully spaced, high-contrast column chart for the 24 hours of the selected day.
fn render_temp_columns(
    f: &mut Frame,
    area: Rect,
    hourly: &HourlyForecast,
    daily: &DailyForecast,
    selected_day: usize,
) {
    let day_idx = selected_day.min(daily.time.len().saturating_sub(1));
    let start = (day_idx * 24).min(hourly.temperature_2m.len());
    let end = (start + 24).min(hourly.temperature_2m.len());

    let (day_temps, day_times, day_codes, day_is_days) = if start < end {
        (
            &hourly.temperature_2m[start..end],
            &hourly.time[start..end],
            hourly.weather_code.get(start..end).unwrap_or(&[]),
            hourly.is_day.get(start..end).unwrap_or(&[]),
        )
    } else {
        (
            hourly.temperature_2m.as_slice(),
            hourly.time.as_slice(),
            hourly.weather_code.as_slice(),
            hourly.is_day.as_slice(),
        )
    };

    let min_label = day_temps.iter().copied().fold(f64::INFINITY, f64::min);
    let max_label = day_temps.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    let date_str = daily.time.get(day_idx).map(|s| s.as_str()).unwrap_or("");
    let day_title = if day_idx == 0 {
        "HOJE".to_string()
    } else if day_idx == 1 {
        "AMANHÃ".to_string()
    } else {
        format_weekday_date(date_str)
    };

    let weather_code = daily.weather_code.get(day_idx).copied().unwrap_or(0);
    let (_, w_icon) = wmo::wmo_description(weather_code, true);

    let title = if day_temps.is_empty() {
        format!(" TEMPERATURA 24H • {day_title} ")
    } else {
        format!(
            " TEMPERATURA 24H • {day_title} ({w_icon} Mín: {:.0}° | Máx: {:.0}°) ",
            min_label, max_label
        )
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if day_temps.is_empty() || inner.height < 5 || inner.width < 10 {
        return;
    }

    // Escolhe o número de colunas (12 se largo, 8 se médio, 6 se estreito)
    let n_bars: usize = if inner.width >= 56 {
        12
    } else if inner.width >= 36 {
        8
    } else {
        6
    };

    let step = (day_temps.len() / n_bars).max(1);
    let sample_indices: Vec<usize> = (0..n_bars)
        .map(|idx| (idx * step).min(day_temps.len().saturating_sub(1)))
        .collect();

    // Altura disponível:
    // y_label: linha do rótulo da hora na base
    // y_emoji: linha do emoji da condição logo acima da hora
    // bar_base_y: base inferior da barra térmica
    let y_label = inner.bottom().saturating_sub(1);
    let y_emoji = y_label.saturating_sub(1);
    let bar_base_y = y_emoji.saturating_sub(1);
    let max_bar_h = (bar_base_y.saturating_sub(inner.y).saturating_sub(1) as f64).max(1.0);

    let range = (max_label - min_label).max(1.0);
    let slot_w = inner.width as f32 / n_bars as f32;

    let buf = f.buffer_mut();

    for (i, &idx) in sample_indices.iter().enumerate() {
        let temp = day_temps[idx];
        let color = temp_color(temp);

        // Centro horizontal da coluna
        let center_x = inner.x as f32 + (i as f32 + 0.5) * slot_w;
        let bar_x = (center_x - 1.0).round().max(inner.x as f32) as u16;

        // Altura proporcional da barra (mínimo 1)
        let ratio = ((temp - min_label) / range).clamp(0.0, 1.0);
        let bar_h = (1.0 + ratio * (max_bar_h - 1.0)).round() as u16;

        // 1. Desenha a barra vertical sólida com "██"
        let bar_top_y = bar_base_y.saturating_sub(bar_h.saturating_sub(1));
        for y in bar_top_y..=bar_base_y {
            buf.set_string(bar_x, y, "██", Style::default().fg(color));
        }

        // 2. Número exato impresso com alto contraste ACIMA da barra
        let val_str = format!("{:.0}°", temp);
        let val_x = ((center_x - (val_str.len() as f32 / 2.0)).round() as u16)
            .max(inner.x)
            .min(inner.right().saturating_sub(val_str.len() as u16));
        let val_y = bar_top_y.saturating_sub(1).max(inner.y);
        buf.set_string(
            val_x,
            val_y,
            &val_str,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        );

        // 3. Emoji da condição do céu no momento (em cima da hora)
        let code = day_codes.get(idx).copied().unwrap_or(0);
        let is_day = day_is_days.get(idx).copied().unwrap_or(1) == 1;
        let (_, icon) = wmo::wmo_description(code, is_day);

        let icon_w = UnicodeWidthStr::width(icon) as f32;
        let icon_x = ((center_x - (icon_w / 2.0)).round() as u16)
            .max(inner.x)
            .min(inner.right().saturating_sub(icon_w.max(1.0) as u16));

        let icon_style = match code {
            0 if is_day => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            0 => Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD),
            1 | 2 if is_day => Style::default().fg(Color::Yellow),
            1 | 2 => Style::default().fg(Color::LightYellow),
            3 => Style::default().fg(Color::Gray),
            45 | 48 => Style::default().fg(Color::DarkGray),
            51..=67 | 80..=82 => Style::default().fg(Color::LightBlue),
            71..=77 | 85 | 86 => Style::default().fg(Color::White),
            95..=99 => Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD),
            _ => Style::default(),
        };
        buf.set_string(icon_x, y_emoji, icon, icon_style);

        // 4. Rótulo da hora impresso ABAIXO do emoji
        let hour_str = if let Some(t) = day_times.get(idx) {
            t.get(11..13).map(|h| format!("{h}h")).unwrap_or_else(|| format!("{idx}h"))
        } else {
            format!("{idx}h")
        };
        let lbl_x = ((center_x - (hour_str.len() as f32 / 2.0)).round() as u16)
            .max(inner.x)
            .min(inner.right().saturating_sub(hour_str.len() as u16));
        buf.set_string(
            lbl_x,
            y_label,
            &hour_str,
            Style::default().fg(Color::DarkGray),
        );
    }
}

/// Renders a clean, high-contrast column chart for 14-day rain forecast.
/// Highlights the currently selected day.
fn render_precip_columns(
    f: &mut Frame,
    area: Rect,
    daily: &DailyForecast,
    selected_day: usize,
) {
    let day_idx = selected_day.min(daily.time.len().saturating_sub(1));
    let title = " CHUVA (14 DIAS) ".to_string();

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let n_days = daily.time.len();
    if n_days == 0 || inner.height < 4 || inner.width < 10 {
        return;
    }

    let max_rain = daily
        .precipitation_sum
        .iter()
        .copied()
        .fold(0.0_f64, f64::max);

    let y_label = inner.bottom().saturating_sub(1);
    let max_bar_h = inner.height.saturating_sub(2) as f64;
    let slot_w = inner.width as f32 / n_days as f32;

    let buf = f.buffer_mut();

    for i in 0..n_days {
        let precip = daily.precipitation_sum.get(i).copied().unwrap_or(0.0);
        let is_selected = i == day_idx;

        let center_x = inner.x as f32 + (i as f32 + 0.5) * slot_w;
        let bar_x = (center_x - 1.0).round().max(inner.x as f32) as u16;

        if precip > 0.0 && max_rain > 0.0 {
            // Altura proporcional
            let ratio = (precip / max_rain).clamp(0.0, 1.0);
            let bar_h = (1.0 + ratio * (max_bar_h - 1.0)).round() as u16;
            let bar_top_y = y_label.saturating_sub(bar_h);

            let bar_color = if is_selected {
                Color::Yellow
            } else {
                Color::LightBlue
            };

            for y in bar_top_y..y_label {
                buf.set_string(bar_x, y, "██", Style::default().fg(bar_color));
            }

            // Valor da chuva impresso ACIMA da barra
            let val_str = if precip >= 10.0 {
                format!("{:.0}", precip)
            } else {
                format!("{:.1}", precip)
            };
            let val_x = (center_x - (val_str.len() as f32 / 2.0)).round().max(inner.x as f32) as u16;
            let val_y = bar_top_y.saturating_sub(1).max(inner.y);
            let val_style = if is_selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            };
            buf.set_string(val_x, val_y, &val_str, val_style);
        } else {
            // Dia sem chuva: ponto destacado se selecionado
            let dot_x = center_x.round().max(inner.x as f32) as u16;
            let dot_y = y_label.saturating_sub(1).max(inner.y);
            let (symbol, style) = if is_selected {
                ("●", Style::default().fg(Color::Yellow))
            } else {
                ("·", Style::default().fg(Color::DarkGray))
            };
            buf.set_string(dot_x, dot_y, symbol, style);
        }

        // Dia do mês (DD) na base
        let day_str = daily
            .time
            .get(i)
            .and_then(|d| d.get(8..10))
            .unwrap_or("??");

        let lbl_x = (center_x - 1.0).round().max(inner.x as f32) as u16;
        let lbl_style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if precip > 0.0 {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        buf.set_string(lbl_x, y_label, day_str, lbl_style);
    }
}

fn format_weekday_date(iso_date: &str) -> String {
    use chrono::NaiveDate;
    if let Ok(date) = NaiveDate::parse_from_str(iso_date, "%Y-%m-%d") {
        let weekday = match date.format("%a").to_string().as_str() {
            "Mon" => "SEG",
            "Tue" => "TER",
            "Wed" => "QUA",
            "Thu" => "QUI",
            "Fri" => "SEX",
            "Sat" => "SÁB",
            "Sun" => "DOM",
            _ => "",
        };
        format!("{weekday}, {}", date.format("%d/%m"))
    } else {
        iso_date.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_chart_render_with_emojis() {
        let hourly = HourlyForecast {
            time: (0..24).map(|h| format!("2026-09-27T{:02}:00", h)).collect(),
            temperature_2m: (0..24).map(|h| 15.0 + (h as f64)).collect(),
            weather_code: vec![0; 24],
            is_day: vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
        };

        let daily = DailyForecast {
            time: vec!["2026-09-27".into()],
            weather_code: vec![0],
            temperature_2m_max: vec![30.0],
            temperature_2m_min: vec![15.0],
            precipitation_sum: vec![2.0],
            wind_speed_10m_max: vec![10.0],
            sunrise: vec!["2026-09-27T06:00".into()],
            sunset: vec!["2026-09-27T18:00".into()],
            uv_index_max: vec![5.0],
            relative_humidity_2m_mean: vec![70.0],
        };

        let backend = TestBackend::new(100, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                render(f, Rect::new(0, 0, 100, 10), &hourly, &daily, 0);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        let found_symbol = (0..100).any(|x| {
            (0..10).any(|y| {
                let cell = &buffer[(x, y)];
                cell.symbol() == "☀️" || cell.symbol() == "🌙" || cell.symbol().contains('☀') || cell.symbol().contains('🌙')
            })
        });
        assert!(found_symbol, "Expected sky condition symbols rendered above hours");
    }
}
