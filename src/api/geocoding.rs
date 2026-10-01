use anyhow::{Context, Result};

use crate::api::CLIENT;
use crate::models::{City, GeocodingResponse};

const GEOCODING_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";

/// Searches for cities matching the given query string.
/// Uses native reqwest query serialization to properly handle UTF-8 accents and spaces.
pub async fn search_city(query: &str) -> Result<Vec<City>> {
    let resp = CLIENT
        .get(GEOCODING_URL)
        .query(&[
            ("name", query),
            ("count", "5"),
            ("language", "pt"),
        ])
        .send()
        .await
        .context("Falha na conexão com geocoding")?
        .error_for_status()
        .context("Serviço de geocoding retornou erro HTTP")?
        .json::<GeocodingResponse>()
        .await
        .context("Falha ao parsear resultados de busca")?;

    Ok(resp.results.unwrap_or_default())
}
