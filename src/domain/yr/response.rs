use serde::Deserialize;

use super::LonLatAlt;

#[derive(Deserialize, Debug)]
pub struct CompactForecastResponse {
    #[serde(rename = "type")]
    pub kind: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Deserialize, Debug)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub kind: String,
    pub coordinates: LonLatAlt,
}

#[derive(Deserialize, Debug)]
pub struct Properties {
    pub meta: Meta,
    pub timeseries: Vec<TimeSeries>,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub updated_at: String,
    pub units: Units,
}

#[derive(Deserialize, Debug)]
pub struct Units {
    pub air_pressure_at_sea_level: String,
    pub air_temperature: String,
    pub cloud_area_fraction: String,
    pub precipitation_amount: String,
    pub relative_humidity: String,
    pub wind_from_direction: String,
    pub wind_speed: String,
}

#[derive(Deserialize, Debug)]
pub struct TimeSeries {
    pub time: String,
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub instant: Instant,

    pub next_1_hours: Option<NextHours>,
    pub next_6_hours: Option<NextHours>,
    pub next_12_hours: Option<NextHours>,
}

#[derive(Deserialize, Debug)]
pub struct Instant {
    pub details: InstantDetails,
}

#[derive(Deserialize, Debug)]
pub struct InstantDetails {
    #[serde(rename = "air_pressure_at_sea_level")]
    pub air_pressure_at_sea_level: f64,
    #[serde(rename = "air_temperature")]
    pub air_temperature: f64,
    #[serde(rename = "cloud_area_fraction")]
    pub cloud_area_fraction: f64,
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
    pub precipitation_amount: Option<f64>,
}
