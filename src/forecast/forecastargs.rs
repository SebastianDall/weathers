use clap::Parser;
use weathers::domain::Coordinates;

#[derive(Parser, Debug, Clone)]
pub struct ForecastArgs {
    #[arg(
        short,
        long,
        required = true,
        help = "Coordinates to request weather data 'lat,lon'"
    )]
    pub coordinates: Coordinates,
}
