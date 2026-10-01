use std::fs;
use std::path::PathBuf;

use crate::models::City;

/// Returns the configuration directory for Tempus: `$XDG_CONFIG_HOME/tempus` or `~/.config/tempus`.
pub fn config_dir() -> PathBuf {
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        PathBuf::from(xdg).join("tempus")
    } else if let Some(home) = std::env::var_os("HOME") {
        PathBuf::from(home).join(".config").join("tempus")
    } else {
        PathBuf::from(".").join(".tempus")
    }
}

/// Returns the path to the `last_city.json` file.
fn last_city_path() -> PathBuf {
    config_dir().join("last_city.json")
}

/// Loads the last viewed city from disk, if available.
pub fn load_last_city() -> Option<City> {
    let path = last_city_path();
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Persists the selected city to disk so it can be automatically restored on next launch.
pub fn save_last_city(city: &City) {
    let dir = config_dir();
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Aviso: Não foi possível criar diretório de configuração {dir:?}: {e}");
        return;
    }
    let path = last_city_path();
    if let Ok(json) = serde_json::to_string_pretty(city) {
        let _ = fs::write(path, json);
    }
}

/// Default fallback city (Curitiba, PR, Brasil).
pub fn default_city() -> City {
    City {
        name: "Curitiba".into(),
        latitude: -25.4284,
        longitude: -49.2733,
        country: "Brasil".into(),
        admin1: Some("Paraná".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_city() {
        let city = default_city();
        assert_eq!(city.name, "Curitiba");
        assert_eq!(city.country, "Brasil");
    }

    #[test]
    fn test_city_serialization_roundtrip() {
        let city = default_city();
        let json = serde_json::to_string(&city).expect("serialization failed");
        let parsed: City = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(city, parsed);
    }
}
