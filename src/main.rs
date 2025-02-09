use anyhow::Result;
use clap::Parser;
use log::info;
use std::fs;
use std::path::Path;

mod argparser;
mod forecast;

use crate::forecast::forecast;
use argparser::Args;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cache_dir = Path::new("./cache");

    if !cache_dir.is_dir() {
        info!("Creating cache dir at: {:?}", &cache_dir);
        fs::create_dir(cache_dir);
    }

    let args = Args::parse();

    match &args.command {
        argparser::Commands::Forecast(forecast_args) => {
            let f_args = forecast_args.clone();
            forecast(f_args).await?;
        }
    }

    Ok(())
}
