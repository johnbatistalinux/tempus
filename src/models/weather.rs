use serde::Deserialize;

/// Top-level response from Open-Meteo `/v1/forecast`.
#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    #[allow(dead_code)]
    pub latitude: f64,
    #[allow(dead_code)]
    pub longitude: f64,
    #[allow(dead_code)]
    pub timezone: String,
    pub current: CurrentWeather,
    pub daily: DailyForecast,
    pub hourly: HourlyForecast,
}

/// Current weather conditions.
#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub relative_humidity_2m: f64,
    pub weather_code: u8,
    pub wind_speed_10m: f64,
    pub wind_direction_10m: f64,
    pub is_day: u8,
}

/// Hourly weather conditions (e.g. 24h curve).
#[derive(Debug, Deserialize)]
pub struct HourlyForecast {
    #[allow(dead_code)]
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    #[serde(default)]
    pub weather_code: Vec<u8>,
    #[serde(default)]
    pub is_day: Vec<u8>,
}

/// Daily aggregated forecast arrays (one entry per day, up to 14 days).
#[derive(Debug, Deserialize)]
pub struct DailyForecast {
    pub time: Vec<String>,
    pub weather_code: Vec<u8>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub precipitation_sum: Vec<f64>,
    pub wind_speed_10m_max: Vec<f64>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
    pub uv_index_max: Vec<f64>,
    #[allow(dead_code)]
    pub relative_humidity_2m_mean: Vec<f64>,
}
