use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, BorderType, Cell, Row, Table},
    Frame,
};

use crate::models::DailyForecast;
use crate::wmo;
use crate::widgets::current::temp_color;

pub fn render(f: &mut Frame, area: Rect, daily: &DailyForecast, selected_day: usize) {
    let header = Row::new(vec![
        Cell::from("   Dia"),
        Cell::from("Condição"),
        Cell::from("Mín"),
        Cell::from("Máx"),
        Cell::from("Chuva"),
        Cell::from("Vento"),
    ])
    .style(
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    )
    .bottom_margin(1);

    let rows: Vec<Row> = (0..daily.time.len())
        .map(|i| {
            let is_selected = i == selected_day;
            let date_label = if is_selected {
                format!(" ▸ {}", format_date(&daily.time[i]))
            } else {
                format!("   {}", format_date(&daily.time[i]))
            };
            let weather_code = daily.weather_code.get(i).copied().unwrap_or(0);
            let (desc, icon) = wmo::wmo_description(weather_code, true);
            let condition = format!("{icon} {desc}");
            let min_t = daily.temperature_2m_min.get(i).copied().unwrap_or(0.0);
            let max_t = daily.temperature_2m_max.get(i).copied().unwrap_or(0.0);
            let precip = daily.precipitation_sum.get(i).copied().unwrap_or(0.0);
            let wind = daily.wind_speed_10m_max.get(i).copied().unwrap_or(0.0);

            let precip_str = if precip > 0.0 {
                format!("{:.1} mm", precip)
            } else {
                "-".to_string()
            };

            let date_style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            };

            let row = Row::new(vec![
                Cell::from(date_label).style(date_style),
                Cell::from(condition),
                Cell::from(format!("{:.0}°", min_t)).style(Style::default().fg(temp_color(min_t))),
                Cell::from(format!("{:.0}°", max_t)).style(Style::default().fg(temp_color(max_t))),
                Cell::from(precip_str).style(Style::default().fg(Color::LightBlue)),
                Cell::from(format!("{:.0} km/h", wind)).style(Style::default().fg(Color::DarkGray)),
            ]);

            if is_selected {
                row.style(
                    Style::default()
                        .bg(Color::Rgb(25, 45, 75))
                        .add_modifier(Modifier::BOLD),
                )
            } else if i % 2 == 0 {
                row.style(Style::default().bg(Color::Rgb(30, 30, 30)))
            } else {
                row
            }
        })
        .collect();

    // Colunas proporcionais que ocupam 100% da largura do terminal (sem vácuo na direita)
    let widths = [
        Constraint::Fill(2), // Dia (~14%)
        Constraint::Fill(4), // Condição (~28%)
        Constraint::Fill(2), // Mínima (~14%)
        Constraint::Fill(2), // Máxima (~14%)
        Constraint::Fill(2), // Chuva (~15%)
        Constraint::Fill(2), // Vento (~15%)
    ];

    let title = format!(" PREVISÃO {} DIAS • [↑↓/←→] Navegar ", daily.time.len());

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .column_spacing(2);

    f.render_widget(table, area);
}

fn format_date(iso_date: &str) -> String {
    use chrono::NaiveDate;

    let Ok(date) = NaiveDate::parse_from_str(iso_date, "%Y-%m-%d") else {
        return iso_date.to_string();
    };

    let weekday = match date.format("%a").to_string().as_str() {
        "Mon" => "Seg",
        "Tue" => "Ter",
        "Wed" => "Qua",
        "Thu" => "Qui",
        "Fri" => "Sex",
        "Sat" => "Sáb",
        "Sun" => "Dom",
        other => return other.to_string(),
    };

    format!("{} {:02}", weekday, date.format("%d"))
}
