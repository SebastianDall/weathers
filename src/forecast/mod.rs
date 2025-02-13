use anyhow::Result;
use log::info;
use reqwest::{self, Client};

pub mod forecastargs;
pub use forecastargs::ForecastArgs;
use weathers::domain::yr::weatherapi::YrApi;

pub async fn forecast(args: ForecastArgs) -> Result<()> {
    info!("Requested coordinates are: {:?}", args.coordinates);

    let yr = YrApi::new()?;
    let client = Client::new();

    let json = yr.get_forecast(args.coordinates, client).await?;

    let symbol = &json.properties.timeseries[0]
        .data
        .next_1_hours
        .as_ref()
        .and_then(|next| next.summary.as_ref())
        .map(|code| &code.symbol_code);

    if let Some(sym) = symbol {
        println!("The code is: {}", sym.to_string());
    } else {
        println!("No code available");
    }
    Ok(())
}
