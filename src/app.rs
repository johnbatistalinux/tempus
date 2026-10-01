use crate::models::{City, ForecastResponse};

/// Application mode / screen state.
#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    /// Displaying weather forecast normally.
    Normal,
    /// User is typing a city name.
    SearchInput,
    /// User is selecting from search results.
    SearchResults,
    /// Waiting for API response.
    Loading,
}

/// Main application state.
pub struct App {
    pub mode: AppMode,
    pub city: City,
    pub forecast: Option<ForecastResponse>,
    pub search_input: String,
    pub search_results: Vec<City>,
    pub selected_result: usize,
    pub selected_day: usize,
    pub error: Option<String>,
    pub should_quit: bool,
    pub is_loading: bool,
}

impl App {
    /// Creates a new App with the given default city.
    pub fn new(city: City) -> Self {
        Self {
            mode: AppMode::Normal,
            city,
            forecast: None,
            search_input: String::new(),
            search_results: Vec::new(),
            selected_result: 0,
            selected_day: 0,
            error: None,
            should_quit: false,
            is_loading: false,
        }
    }

    /// Enters search mode.
    pub fn enter_search(&mut self) {
        self.search_input.clear();
        self.search_results.clear();
        self.error = None;
        self.mode = AppMode::SearchInput;
    }

    /// Cancels search and returns to normal mode.
    pub fn cancel_search(&mut self) {
        self.search_input.clear();
        self.search_results.clear();
        self.error = None;
        self.mode = AppMode::Normal;
    }

    /// Moves selection up in search results.
    pub fn select_previous(&mut self) {
        if self.selected_result > 0 {
            self.selected_result -= 1;
        }
    }

    /// Moves selection down in search results.
    pub fn select_next(&mut self) {
        if self.selected_result + 1 < self.search_results.len() {
            self.selected_result += 1;
        }
    }

    /// Advances the selected day by 1, up to the maximum available days in the forecast.
    pub fn select_next_day(&mut self) {
        let max_days = self
            .forecast
            .as_ref()
            .map(|f| f.daily.time.len().saturating_sub(1))
            .unwrap_or(13);

        if self.selected_day < max_days {
            self.selected_day += 1;
        }
    }

    /// Moves the selected day back by 1 (minimum 0 = today).
    pub fn select_previous_day(&mut self) {
        if self.selected_day > 0 {
            self.selected_day -= 1;
        }
    }

    /// Resets the selected day back to today (0).
    pub fn select_today(&mut self) {
        self.selected_day = 0;
    }

    /// Updates app state upon receiving forecast data from background task.
    pub fn handle_forecast_result(&mut self, result: Result<ForecastResponse, String>) {
        self.is_loading = false;
        if self.mode == AppMode::Loading {
            self.mode = AppMode::Normal;
        }
        match result {
            Ok(data) => {
                self.forecast = Some(data);
                self.error = None;
            }
            Err(err) => {
                self.error = Some(err);
            }
        }
    }

    /// Updates app state upon receiving city search results from background task.
    pub fn handle_search_result(&mut self, result: Result<Vec<City>, String>) {
        self.is_loading = false;
        match result {
            Ok(results) => {
                if results.is_empty() {
                    self.error = Some("Nenhuma cidade encontrada".into());
                    self.mode = AppMode::SearchInput;
                } else {
                    self.search_results = results;
                    self.selected_result = 0;
                    self.mode = AppMode::SearchResults;
                }
            }
            Err(err) => {
                self.error = Some(err);
                self.mode = AppMode::SearchInput;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_city() -> City {
        City {
            name: "São Paulo".into(),
            latitude: -23.55,
            longitude: -46.63,
            country: "Brasil".into(),
            admin1: Some("São Paulo".into()),
        }
    }

    #[test]
    fn test_day_navigation_bounds() {
        let mut app = App::new(dummy_city());
        assert_eq!(app.selected_day, 0);

        // Previous on day 0 does not underflow
        app.select_previous_day();
        assert_eq!(app.selected_day, 0);

        // Advance days
        app.select_next_day();
        assert_eq!(app.selected_day, 1);

        app.select_next_day();
        assert_eq!(app.selected_day, 2);

        app.select_previous_day();
        assert_eq!(app.selected_day, 1);

        // Reset to today
        app.select_today();
        assert_eq!(app.selected_day, 0);
    }
}
