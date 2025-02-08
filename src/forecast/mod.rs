use anyhow::{Context, Result};
use log::info;
// use reqwest;

pub mod forecastargs;
pub use forecastargs::ForecastArgs;

pub async fn forecast(args: ForecastArgs) -> Result<()> {
    info!("Requested coordinates are: {:?}", args.coordinates);
    // let body = reqwest::get("https://google.com").await?.text().await?;
    // println!("body: {body:?}");

    Ok(())
}
