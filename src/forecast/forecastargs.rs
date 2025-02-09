use clap::Parser;
use weathers::domain::LonLatAlt;

#[derive(Parser, Debug, Clone)]
pub struct ForecastArgs {
    #[arg(
        short,
        long,
        required = true,
        help = "Coordinates to request weather data 'lat,lon'"
    )]
    pub coordinates: LonLatAlt,
}
