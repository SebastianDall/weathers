use anyhow::{anyhow, Result};
use chrono::prelude::*;
use reqwest::header::HeaderMap;
use serde::Deserialize;

use super::LonLatAlt;

fn parse_header_to_rfc2822(headers: &HeaderMap, key: &str) -> anyhow::Result<DateTime<Utc>> {
    let value = headers
        .get(key)
        .ok_or_else(|| anyhow!("Missing key '{}' in headers", key))?;
    let rfc2822 = value.to_str()?;

    let utc = DateTime::parse_from_rfc2822(&rfc2822)?.with_timezone(&Utc);
    Ok(utc)
}
#[derive(Debug)]
pub struct YrResponseHeaders {
    pub last_modified: DateTime<Utc>,
    pub expires: DateTime<Utc>,
}

impl YrResponseHeaders {
    pub fn from_headermap(headers: &HeaderMap) -> Result<Self> {
        let last_modified = parse_header_to_rfc2822(headers, "last-modified")?;
        let expires = parse_header_to_rfc2822(headers, "expires")?;

        Ok(Self {
            last_modified,
            expires,
        })
    }
}

#[derive(Debug)]
pub struct YrResponseBody {
    pub date_requested: DateTime<Utc>,
    pub headers: YrResponseHeaders,
    pub forecast: YrForecastType,
}

impl YrResponseBody {
    pub fn new(headers: YrResponseHeaders, forecast: YrForecastType) -> Self {
        let datetime: DateTime<Utc> = Utc::now();

        Self {
            date_requested: datetime,
            headers,
            forecast,
        }
    }
}

#[derive(Debug)]
pub enum YrForecastType {
    CompactForecastResponse(CompactForecastResponse),
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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
    pub time: DateTime<Utc>,
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
