use anyhow::{Context, Result};
use reqwest::Client;
use url::Url;

use crate::domain::LonLatAlt;

use super::response::CompactForecastResponse;

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

    pub async fn get_forecast(
        self,
        coords: LonLatAlt,
        client: Client,
    ) -> Result<CompactForecastResponse> {
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

        // let headers = response.headers().await.context("Failed to get response headers");

        let body = response.text().await.context("Failed to get response")?;

        let json: CompactForecastResponse =
            serde_json::from_str(&body).context("Unable to parse request to json")?;

        Ok(json)
    }
}
