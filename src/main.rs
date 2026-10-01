mod api;
mod app;
mod config;
mod models;
mod ui;
mod widgets;
mod wmo;

use std::io;
use std::panic;
use std::time::Duration;

use crossterm::{
    cursor::Show,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use app::{App, AppMode};
use models::{City, ForecastResponse};

/// Messages sent from background asynchronous network tasks to the main UI loop.
enum AsyncMsg {
    Forecast(Box<Result<ForecastResponse, String>>),
    Search(Result<Vec<City>, String>),
    InitialSearch {
        query: String,
        result: Result<Vec<City>, String>,
    },
}

/// RAII Guard ensuring the terminal is always returned to normal mode when dropped.
struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);
    }
}

/// Installs a panic hook that resets the terminal state before printing the panic info.
fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);
        original_hook(panic_info);
    }));
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Process command-line arguments prior to terminal takeover
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        let first = &args[0];
        if first == "-h" || first == "--help" {
            println!("🌦️  Tempus - Previsão do tempo moderna no terminal");
            println!();
            println!("Uso:");
            println!("  tempus               Inicia com a última cidade consultada (ou Curitiba)");
            println!("  tempus <cidade>      Busca e abre diretamente a previsão da cidade informada");
            println!("  tempus -h, --help    Exibe esta mensagem de ajuda");
            println!();
            println!("Exemplos:");
            println!("  tempus               # Abre a cidade salva");
            println!("  tempus Florianopolis # Abre Florianópolis");
            println!("  tempus \"Sao Paulo\"   # Abre São Paulo");
            println!("  tempus Paris         # Abre Paris");
            println!();
            println!("Atalhos no aplicativo:");
            println!("  [←/→] ou [↑/↓] Navegar entre os 14 dias da previsão (atualiza o gráfico de 24h)");
            println!("  [0] / Home     Retornar para a previsão de hoje");
            println!("  [s]            Buscar nova cidade interativamente");
            println!("  [r]            Recarregar previsão atual");
            println!("  [q] / Esc      Sair do aplicativo");
            println!("  Ctrl+C         Encerramento seguro a qualquer momento");
            return Ok(());
        }
    }

    let initial_query = if !args.is_empty() {
        Some(args.join(" "))
    } else {
        None
    };

    setup_panic_hook();

    // Setup terminal with RAII cleanup guard
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let _guard = TerminalGuard;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Init with persisted city (or default)
    let initial_city = config::load_last_city().unwrap_or_else(config::default_city);
    let mut app = App::new(initial_city);

    let (tx, mut rx) = mpsc::unbounded_channel::<AsyncMsg>();

    if let Some(query) = initial_query {
        app.mode = AppMode::Loading;
        app.is_loading = true;
        let tx_clone = tx.clone();
        let q_clone = query.clone();
        tokio::spawn(async move {
            let res = api::search_city(&q_clone).await.map_err(|e| format!("{e:#}"));
            let _ = tx_clone.send(AsyncMsg::InitialSearch {
                query: q_clone,
                result: res,
            });
        });
    } else {
        // Initial non-blocking background fetch
        trigger_fetch_forecast(&mut app, tx.clone());
    }

    // Main event loop
    loop {
        // Drain any incoming async messages
        while let Ok(msg) = rx.try_recv() {
            match msg {
                AsyncMsg::Forecast(res) => app.handle_forecast_result(*res),
                AsyncMsg::Search(res) => app.handle_search_result(res),
                AsyncMsg::InitialSearch { query, result } => {
                    app.is_loading = false;
                    match result {
                        Ok(cities) if !cities.is_empty() => {
                            let chosen = cities[0].clone();
                            app.city = chosen;
                            app.selected_day = 0;
                            config::save_last_city(&app.city);
                            trigger_fetch_forecast(&mut app, tx.clone());
                        }
                        Ok(_) => {
                            app.error = Some(format!("Cidade \"{query}\" não encontrada."));
                            trigger_fetch_forecast(&mut app, tx.clone());
                        }
                        Err(e) => {
                            app.error = Some(format!("Erro ao buscar \"{query}\": {e}"));
                            trigger_fetch_forecast(&mut app, tx.clone());
                        }
                    }
                }
            }
        }

        terminal.draw(|f| ui::render(f, &app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Global exit shortcut: Ctrl+C always safely quits
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    app.should_quit = true;
                    break;
                }

                match app.mode {
                    AppMode::Normal | AppMode::Loading => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.should_quit = true;
                        }
                        KeyCode::Char('s') => {
                            app.enter_search();
                        }
                        KeyCode::Char('r') => {
                            trigger_fetch_forecast(&mut app, tx.clone());
                        }
                        KeyCode::Right | KeyCode::Down | KeyCode::Char('l') | KeyCode::Char('j') => {
                            app.select_next_day();
                        }
                        KeyCode::Left | KeyCode::Up | KeyCode::Char('h') | KeyCode::Char('k') => {
                            app.select_previous_day();
                        }
                        KeyCode::Home | KeyCode::Char('0') => {
                            app.select_today();
                        }
                        _ => {}
                    },
                    AppMode::SearchInput => match key.code {
                        KeyCode::Esc => {
                            app.cancel_search();
                        }
                        KeyCode::Enter => {
                            let query = app.search_input.trim().to_string();
                            if !query.is_empty() {
                                app.mode = AppMode::Loading;
                                app.is_loading = true;
                                app.error = None;
                                let tx_clone = tx.clone();
                                tokio::spawn(async move {
                                    let res = api::search_city(&query)
                                        .await
                                        .map_err(|e| format!("{e:#}"));
                                    let _ = tx_clone.send(AsyncMsg::Search(res));
                                });
                            }
                        }
                        KeyCode::Backspace => {
                            app.search_input.pop();
                        }
                        KeyCode::Char(c) => {
                            app.search_input.push(c);
                        }
                        _ => {}
                    },
                    AppMode::SearchResults => match key.code {
                        KeyCode::Esc => {
                            app.cancel_search();
                        }
                        KeyCode::Up => {
                            app.select_previous();
                        }
                        KeyCode::Down => {
                            app.select_next();
                        }
                        KeyCode::Enter => {
                            if let Some(city) = app.search_results.get(app.selected_result).cloned() {
                                app.city = city;
                                app.selected_day = 0;
                                config::save_last_city(&app.city);
                                app.search_input.clear();
                                app.search_results.clear();
                                trigger_fetch_forecast(&mut app, tx.clone());
                            }
                        }
                        _ => {}
                    },
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn trigger_fetch_forecast(app: &mut App, tx: mpsc::UnboundedSender<AsyncMsg>) {
    app.mode = AppMode::Loading;
    app.is_loading = true;
    app.error = None;

    let lat = app.city.latitude;
    let lon = app.city.longitude;

    tokio::spawn(async move {
        let res = api::fetch_forecast(lat, lon)
            .await
            .map_err(|e| format!("{e:#}"));
        let _ = tx.send(AsyncMsg::Forecast(Box::new(res)));
    });
}
