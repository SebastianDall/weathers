use serde::Deserialize;
use url::Url;

use super::Coordinates;

enum YrReport {
    Compact,
    Complete,
}

pub struct Yr {
    url: Url,
    report_type: YrReport,
}

#[derive(Deserialize, Debug)]
pub struct ForecastResponse {
    pub kind: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Deserialize, Debug)]
pub struct Geometry {
    pub kind: String,
    pub coordinates: Coordinates,
}

#[derive(Deserialize, Debug)]
pub struct Properties {
    pub kind: Meta,
    pub timeseries: Vec<TimeSeries>,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub updated_at: String,
    pub units: Units,
}

#[derive(Deserialize, Debug)]
pub struct Units {
    pub air_temperature_unit: String,

    pub precipitation_amount_unit: String,
}

#[derive(Deserialize, Debug)]
pub struct TimeSeries {
    pub time: String,

    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub instant_details: InstantDetails,

    pub next_1_hours: Option<NextHours>,
    pub next_6_hours: Option<NextHours>,
    pub next_12_hours: Option<NextHours>,
}

#[derive(Deserialize, Debug)]
pub struct InstantDetails {
    #[serde(rename = "air_pressure_at_sea_level")]
    pub air_pressure_at_sea_level: f64,
    #[serde(rename = "air_temperature")]
    pub air_temperature: f64,
    #[serde(rename = "cloud_area_fraction")]
    pub cloud_area_fraction: f64,
    // ... skip or ignore the rest
    #[serde(rename = "relative_humidity")]
    pub relative_humidity: f64,
    #[serde(rename = "wind_from_direction")]
    pub wind_from_direction: f64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
}
#[derive(Debug, Deserialize)]
pub struct NextHours {
    pub summary: Option<SymbolCode>,
    pub details: Option<PrecipitationDetails>,
}

#[derive(Debug, Deserialize)]
pub struct SymbolCode {
    #[serde(rename = "symbol_code")]
    pub symbol_code: String,
}

#[derive(Debug, Deserialize)]
pub struct PrecipitationDetails {
    #[serde(rename = "precipitation_amount")]
    pub precipitation_amount: f64,
}
