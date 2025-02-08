use anyhow::{Context, Result};
use log::info;
use reqwest;
use url::Url;

pub mod forecastargs;
pub use forecastargs::ForecastArgs;

pub async fn forecast(args: ForecastArgs) -> Result<()> {
    info!("Requested coordinates are: {:?}", args.coordinates);

    let mut base_url = Url::parse("https://api.met.no/weatherapi/locationforecast/2.0/compact")?;

    base_url
        .query_pairs_mut()
        .append_pair("lat", &args.coordinates.latitude.value().to_string())
        .append_pair("lon", &args.coordinates.longitude.value().to_string());

    let sitename = "weathers/0.1.0 (https://github.com/SebastianDall/weathers)";
    let client = reqwest::Client::new();
    let response = client
        .get(base_url)
        .header("User-Agent", sitename)
        .send()
        .await
        .context("Failed to send request")?;

    let body = response.text().await.context("Failed to get response")?;

    println!("body: {body:?}");
    Ok(())
}
