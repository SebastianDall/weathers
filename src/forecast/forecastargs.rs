use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct ForecastArgs {
    #[arg(
        short,
        long,
        required = true,
        help = "Coordinates to request weather data"
    )]
    pub coordinates: String,
}
