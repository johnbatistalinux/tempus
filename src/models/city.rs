use serde::{Deserialize, Serialize};

/// Response from the Open-Meteo Geocoding API.
#[derive(Debug, Deserialize)]
pub struct GeocodingResponse {
    pub results: Option<Vec<City>>,
}

/// A geocoded city with coordinates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country: String,
    /// State or administrative region (e.g. "Paraná").
    #[serde(default)]
    pub admin1: Option<String>,
}

impl City {
    /// Human-readable display label, e.g. "Curitiba, Paraná, Brasil".
    pub fn display_label(&self) -> String {
        match &self.admin1 {
            Some(region) => format!("{}, {}, {}", self.name, region, self.country),
            None => format!("{}, {}", self.name, self.country),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_label_with_admin() {
        let city = City {
            name: "Curitiba".into(),
            latitude: -25.4284,
            longitude: -49.2733,
            country: "Brasil".into(),
            admin1: Some("Paraná".into()),
        };
        assert_eq!(city.display_label(), "Curitiba, Paraná, Brasil");
    }

    #[test]
    fn test_display_label_without_admin() {
        let city = City {
            name: "Vaticano".into(),
            latitude: 41.9029,
            longitude: 12.4534,
            country: "Vaticano".into(),
            admin1: None,
        };
        assert_eq!(city.display_label(), "Vaticano, Vaticano");
    }
}
