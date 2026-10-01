use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, AppMode};
use crate::widgets;

pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();

    // Layout vertical adaptativo:
    // Header (1) + Espaço (1) + Topo em cartelas (9) + Gráficos (10) + Tabela (Min 16) + Footer (1)
    let chunks = Layout::vertical([
        Constraint::Length(1),  // Header moderno
        Constraint::Length(1),  // Espaçador
        Constraint::Length(9),  // Clima Atual + Detalhes (ambos com moldura simétrica)
        Constraint::Length(10), // Gráficos (Curva 24h + Chuva 14 dias)
        Constraint::Min(16),    // Tabela 14 Dias
        Constraint::Length(1),  // Footer
    ])
    .split(size);

    render_header(f, chunks[0], app);

    if let Some(ref forecast) = app.forecast {
        let main_area = Rect {
            x: chunks[2].x + 1,
            width: chunks[2].width.saturating_sub(2),
            ..chunks[2]
        };

        let top_chunks = Layout::horizontal([
            Constraint::Percentage(36),
            Constraint::Percentage(64),
        ])
        .spacing(1)
        .split(main_area);

        widgets::current::render(f, top_chunks[0], &forecast.current);
        widgets::details::render(f, top_chunks[1], &forecast.current, &forecast.daily);

        let charts_area = Rect {
            x: chunks[3].x + 1,
            width: chunks[3].width.saturating_sub(2),
            ..chunks[3]
        };
        widgets::chart::render(f, charts_area, &forecast.hourly, &forecast.daily, app.selected_day);

        let table_area = Rect {
            x: chunks[4].x + 1,
            width: chunks[4].width.saturating_sub(2),
            ..chunks[4]
        };
        widgets::forecast_table::render(f, table_area, &forecast.daily, app.selected_day);
    } else {
        let msg = if app.is_loading || app.mode == AppMode::Loading {
            "Carregando dados meteorológicos..."
        } else {
            "Sem dados. Pressione [r] para atualizar."
        };
        let paragraph = Paragraph::new(msg)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(paragraph, chunks[2]);
    }

    render_footer(f, chunks[5], app);

    match app.mode {
        AppMode::SearchInput => render_search_input(f, size, app),
        AppMode::SearchResults => render_search_results(f, size, app),
        _ => {}
    }
}

/// Sophisticated dual-tone status bar header.
fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let city_label = app.city.display_label();

    let spans = vec![
        Span::styled(
            " 🌦️ TEMPUS ",
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  📍 {}  ", city_label.to_uppercase()),
            Style::default()
                .bg(Color::Rgb(25, 35, 55))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    let p = Paragraph::new(Line::from(spans))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(p, area);
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let mut spans = vec![
        Span::styled(" [←/→] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("Mudar Dia   "),
    ];

    if app.selected_day > 0 {
        spans.push(Span::styled("[0] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
        spans.push(Span::raw("Hoje   "));
    }

    spans.extend(vec![
        Span::styled("[s] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("Buscar   "),
        Span::styled("[r] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("Atualizar   "),
        Span::styled("[q] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("Sair"),
    ]);

    if let Some(ref err) = app.error {
        spans.push(Span::raw("  │  "));
        spans.push(Span::styled(
            err.clone(),
            Style::default().fg(Color::Red).bg(Color::DarkGray).add_modifier(Modifier::BOLD),
        ));
    }

    if app.is_loading || app.mode == AppMode::Loading {
        spans.push(Span::raw("  │  "));
        spans.push(Span::styled(
            "⏳ Carregando...",
            Style::default().fg(Color::Cyan).bg(Color::DarkGray).add_modifier(Modifier::BOLD),
        ));
    }

    let paragraph = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    f.render_widget(paragraph, area);
}

fn render_search_input(f: &mut Frame, area: Rect, app: &App) {
    let popup = centered_rect(50, 3, area);
    f.render_widget(Clear, popup);

    let input_text = format!(" {}_", app.search_input);
    let paragraph = Paragraph::new(input_text).block(
        Block::default()
            .title(" 🔍 Buscar cidade (Enter confirmar, Esc cancelar) ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow)),
    );

    f.render_widget(paragraph, popup);
}

fn render_search_results(f: &mut Frame, area: Rect, app: &App) {
    let height = (app.search_results.len() as u16 + 2).min(10);
    let popup = centered_rect(60, height, area);
    f.render_widget(Clear, popup);

    let items: Vec<ListItem> = app
        .search_results
        .iter()
        .enumerate()
        .map(|(i, city)| {
            let style = if i == app.selected_result {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let prefix = if i == app.selected_result {
                " ▸ "
            } else {
                "   "
            };
            ListItem::new(format!("{}{}", prefix, city.display_label())).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" Selecione a cidade (↑↓ navegar, Enter selecionar) ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow)),
    );

    f.render_widget(list, popup);
}

fn centered_rect(percent_x: u16, height: u16, area: Rect) -> Rect {
    let popup_width = area.width * percent_x / 100;
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = area.height / 3;
    Rect::new(
        area.x + x,
        area.y + y,
        popup_width.min(area.width),
        height.min(area.height.saturating_sub(y)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use crate::models::{CurrentWeather, DailyForecast};

    #[test]
    fn test_top_row_alignment() {
        let current = CurrentWeather {
            temperature_2m: 13.0,
            apparent_temperature: 13.0,
            relative_humidity_2m: 99.0,
            weather_code: 3, // Nublado (☁)
            wind_speed_10m: 9.6,
            wind_direction_10m: 45.0,
            is_day: 1,
        };
        let daily = DailyForecast {
            time: vec!["2026-09-25".into()],
            weather_code: vec![3],
            temperature_2m_max: vec![25.0],
            temperature_2m_min: vec![12.0],
            precipitation_sum: vec![0.0],
            wind_speed_10m_max: vec![15.0],
            sunrise: vec!["2026-09-25T06:03".into()],
            sunset: vec!["2026-09-25T18:14".into()],
            uv_index_max: vec![8.2],
            relative_humidity_2m_mean: vec![80.0],
        };

        let backend = TestBackend::new(100, 15);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                let main_area = Rect::new(1, 1, 98, 9);
                let top_chunks = Layout::horizontal([Constraint::Percentage(36), Constraint::Percentage(64)])
                    .spacing(1)
                    .split(main_area);
                widgets::current::render(f, top_chunks[0], &current);
                widgets::details::render(f, top_chunks[1], &current, &daily);
            })
            .unwrap();

        let backend = terminal.backend();
        // Verify that on all content rows (y=2..=7), vertical borders are at identical x coordinates
        let c1_right = top_chunks_right(0, 100);
        let c2_left = top_chunks_left(1, 100);

        for y in 2..8 {
            let cell_c1 = &backend.buffer()[(c1_right, y)];
            assert_eq!(cell_c1.symbol(), "│", "CLIMA ATUAL right border misaligned at row {y}");

            let cell_c2 = &backend.buffer()[(c2_left, y)];
            assert_eq!(cell_c2.symbol(), "│", "DETALHES left border misaligned at row {y}");
        }
    }

    fn top_chunks_right(idx: usize, width: u16) -> u16 {
        let main_area = Rect::new(1, 1, width.saturating_sub(2), 9);
        let top_chunks = Layout::horizontal([Constraint::Percentage(36), Constraint::Percentage(64)])
            .spacing(1)
            .split(main_area);
        top_chunks[idx].x + top_chunks[idx].width - 1
    }

    fn top_chunks_left(idx: usize, width: u16) -> u16 {
        let main_area = Rect::new(1, 1, width.saturating_sub(2), 9);
        let top_chunks = Layout::horizontal([Constraint::Percentage(36), Constraint::Percentage(64)])
            .spacing(1)
            .split(main_area);
        top_chunks[idx].x
    }
}

