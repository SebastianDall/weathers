use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::Client;
use url::Url;

use crate::domain::{yr::response::CompactForecastResponse, LonLatAlt};

use super::response::{YrForecastType, YrResponseBody, YrResponseHeaders};

pub struct YrApi {
    url: Url,
    sitename: String,
}

impl YrApi {
    pub fn new() -> Result<Self> {
        let url = Url::parse("https://api.met.no/weatherapi/locationforecast/2.0/compact")?;
        let sitename = "weathers/0.1.0 (https://github.com/SebastianDall/weathers)".to_string();

        Ok(Self { url, sitename })
    }

    pub async fn get_forecast(self, coords: LonLatAlt, client: Client) -> Result<YrResponseBody> {
        let mut query = self.url;
        query
            .query_pairs_mut()
            .append_pair("lat", &coords.latitude.value().to_string())
            .append_pair("lon", &coords.longitude.value().to_string());

        let response = client
            .get(query)
            .header("User-Agent", &self.sitename)
            .send()
            .await
            .context("Failed to send request")?;

        let headers = YrResponseHeaders::from_headermap(response.headers())?;

        let body = response.text().await.context("Failed to get response")?;
        let compact_forecast: CompactForecastResponse =
            serde_json::from_str(&body).with_context(|| {
                format!(
                    "Could not parse response to CompactForecastResponse: {}",
                    &body
                )
            })?;

        let forecast = YrForecastType::CompactForecastResponse(compact_forecast);

        Ok(YrResponseBody {
            date_requested: Utc::now(),
            headers,
            forecast,
        })
    }
}
