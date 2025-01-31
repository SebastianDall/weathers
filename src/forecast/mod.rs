use anyhow::Result;
use reqwest;
pub mod forecastargs;

pub use forecastargs::ForecastArgs;

pub async fn forecast(args: ForecastArgs) -> Result<()> {
    let body = reqwest::get("https://google.com").await?.text().await?;
    println!("body: {body:?}");

    Ok(())
}
