use anyhow::Result;
use log::info;
use reqwest::{self, Client};

pub mod forecastargs;
pub use forecastargs::ForecastArgs;
use weathers::domain::yr::{response::YrForecastType, weatherapi::YrApi};

pub async fn forecast(args: ForecastArgs) -> Result<()> {
    info!("Requested coordinates are: {:?}", args.coordinates);

    let yr = YrApi::new()?;
    let client = Client::new();

    let response = yr.get_forecast(args.coordinates, client).await?;

    info!("Data expires: {}", &response.headers.expires);

    match &response.forecast {
        YrForecastType::CompactForecastResponse(cfr) => {
            let symbol = &cfr.properties.timeseries[0]
                .data
                .next_1_hours
                .as_ref()
                .and_then(|next| next.summary.as_ref())
                .map(|code| &code.symbol_code);

            if let Some(sym) = symbol {
                info!("The code is: {}", sym.to_string());
            } else {
                info!("No code available");
            }
        }
    }

    Ok(())
}
