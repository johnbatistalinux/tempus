pub mod forecast;
pub mod geocoding;

use std::sync::LazyLock;
use std::time::Duration;
use reqwest::Client;

/// Reusable HTTP client with 10s timeout and connection safety.
pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap_or_default()
});

pub use forecast::fetch_forecast;
pub use geocoding::search_city;
