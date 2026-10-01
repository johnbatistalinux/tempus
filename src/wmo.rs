/// Maps a WMO weather interpretation code to a Portuguese description and emoji icon.
///
/// Reference: <https://open-meteo.com/en/docs> — WMO Weather interpretation codes (WW).
pub fn wmo_description(code: u8, is_day: bool) -> (&'static str, &'static str) {
    match code {
        0 => {
            if is_day {
                ("Céu limpo", "☀️")
            } else {
                ("Céu limpo", "🌙")
            }
        }
        1 => {
            if is_day {
                ("Quase limpo", "🌤️")
            } else {
                ("Quase limpo", "🌙")
            }
        }
        2 => {
            if is_day {
                ("Parc. nublado", "⛅")
            } else {
                ("Parc. nublado", "☁️")
            }
        }
        3 => ("Nublado", "☁️"),
        45 | 48 => ("Neblina", "🌫️"),
        51 => ("Garoa leve", "🌦️"),
        53 => ("Garoa moderada", "🌦️"),
        55 => ("Garoa intensa", "🌧️"),
        56 | 57 => ("Garoa congelante", "🌧️"),
        61 => ("Chuva leve", "🌧️"),
        63 => ("Chuva moderada", "🌧️"),
        65 => ("Chuva forte", "🌧️"),
        66 | 67 => ("Chuva congelante", "🌧️"),
        71 => ("Neve leve", "🌨️"),
        73 => ("Neve moderada", "🌨️"),
        75 => ("Neve forte", "❄️"),
        77 => ("Granizo fino", "🌨️"),
        80 => ("Pancadas leves", "🌦️"),
        81 => ("Pancadas moderadas", "🌧️"),
        82 => ("Pancadas fortes", "🌧️"),
        85 | 86 => ("Pancadas de neve", "🌨️"),
        95 => ("Tempestade", "⛈️"),
        96 | 99 => ("Tempestade c/ granizo", "⛈️"),
        _ => ("Desconhecido", "•"),
    }
}

/// Converts wind direction in degrees to a cardinal abbreviation (e.g. "NE", "S").
pub fn wind_direction_label(degrees: f64) -> &'static str {
    let dirs = ["N", "NE", "L", "SE", "S", "SO", "O", "NO"];
    let index = ((degrees + 22.5) / 45.0) as usize % 8;
    dirs[index]
}

/// Describes UV index severity in Portuguese.
pub fn uv_label(uv: f64) -> &'static str {
    match uv as u8 {
        0..=2 => "Baixo",
        3..=5 => "Moderado",
        6..=7 => "Alto",
        8..=10 => "Muito alto",
        _ => "Extremo",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wmo_clear_day() {
        let (desc, icon) = wmo_description(0, true);
        assert_eq!(desc, "Céu limpo");
        assert_eq!(icon, "☀️");
    }

    #[test]
    fn test_wmo_clear_night() {
        let (desc, icon) = wmo_description(0, false);
        assert_eq!(desc, "Céu limpo");
        assert_eq!(icon, "🌙");
    }

    #[test]
    fn test_wmo_thunderstorm() {
        let (desc, _) = wmo_description(95, true);
        assert_eq!(desc, "Tempestade");
    }

    #[test]
    fn test_wind_direction() {
        assert_eq!(wind_direction_label(0.0), "N");
        assert_eq!(wind_direction_label(90.0), "L");
        assert_eq!(wind_direction_label(180.0), "S");
        assert_eq!(wind_direction_label(270.0), "O");
    }

    #[test]
    fn test_uv_label() {
        assert_eq!(uv_label(1.0), "Baixo");
        assert_eq!(uv_label(5.0), "Moderado");
        assert_eq!(uv_label(8.0), "Muito alto");
        assert_eq!(uv_label(12.0), "Extremo");
    }
}
