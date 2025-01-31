use anyhow::Result;
use clap::Parser;

mod argparser;
mod forecast;
use crate::forecast::forecast;
use argparser::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        argparser::Commands::Forecast(forecast_args) => {
            let f_args = forecast_args.clone();
            forecast(f_args).await?;
        }
    }

    Ok(())
}
