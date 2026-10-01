use anyhow::{Context, Result};

use crate::api::CLIENT;
use crate::models::ForecastResponse;

const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";

/// Fetches 14-day forecast + 24h hourly curve + current weather for the given coordinates.
pub async fn fetch_forecast(lat: f64, lon: f64) -> Result<ForecastResponse> {
    let lat_str = lat.to_string();
    let lon_str = lon.to_string();

    let resp = CLIENT
        .get(FORECAST_URL)
        .query(&[
            ("latitude", lat_str.as_str()),
            ("longitude", lon_str.as_str()),
            ("timezone", "auto"),
            (
                "current",
                "temperature_2m,apparent_temperature,relative_humidity_2m,\
                 weather_code,wind_speed_10m,wind_direction_10m,is_day",
            ),
            (
                "hourly",
                "temperature_2m,weather_code,is_day",
            ),
            (
                "daily",
                "weather_code,temperature_2m_max,temperature_2m_min,\
                 precipitation_sum,wind_speed_10m_max,sunrise,sunset,\
                 uv_index_max,relative_humidity_2m_mean",
            ),
            ("forecast_days", "14"),
        ])
        .send()
        .await
        .context("Falha na conexão com Open-Meteo (tempo limite ou rede)")?
        .error_for_status()
        .context("Open-Meteo retornou código de erro HTTP")?
        .json::<ForecastResponse>()
        .await
        .context("Falha ao parsear resposta da API")?;

    Ok(resp)
}
