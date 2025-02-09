use serde::Deserialize;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Longitude(f64);

impl Longitude {
    pub fn new(value: f64) -> Result<Self> {
        if value < -180.0 || value > 180.0 {
            return Err(anyhow!("Longitude must be in [-180, 180]"));
        }

        let rounded = (value * 10_000.0).round() / 10_000.0;

        Ok(Self(rounded))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Latitude(f64);

impl Latitude {
    pub fn new(value: f64) -> Result<Self> {
        if value < -90.0 || value > 90.0 {
            return Err(anyhow!("Latitude must be in [-90, 90]"));
        }

        let rounded = (value * 10_000.0).round() / 10_000.0;

        Ok(Self(rounded))
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Altitude tuple struct.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Altitude(f64);

impl Altitude {
    pub fn new(value: f64) -> Result<Self> {
        if value < -500.0 || value > 9000.0 {
            return Err(anyhow!("Altitude outside of realistic range"));
        }

        Ok(Self(value))
    }
}

/// Struct for representing geo-coordinates.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LonLatAlt {
    pub longitude: Longitude,
    pub latitude: Latitude,
    /// Optional altitude in meters
    pub altitude: Option<Altitude>,
}

impl LonLatAlt {
    /// Create new LonLatAlt Coordinates
    ///
    /// ```
    /// use weathers::domain::{LonLatAlt};
    ///
    /// let coord = LonLat::new(0.0, 60, None).unwrap();
    /// assert_eq!(coord.latitude.value(), 60.0);
    /// ```
    pub fn new<L, T>(lon: L, lat: T, alt: Option<f64>) -> Result<Self>
    where
        L: Into<f64>,
        T: Into<f64>,
    {
        let lat = lat.into();
        let lon = lon.into();
        Ok(Self {
            latitude: Latitude::new(lat)?,
            longitude: Longitude::new(lon)?,
            altitude: alt.map(|meters| Altitude::new(meters)).transpose()?,
        })
    }
}

impl FromStr for LonLatAlt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = s.split(",").collect();

        if parts.len() != 2 {
            return Err(anyhow!(
                "Invalid coordinates {}. Coordinates must be in the format 'lat,lon' (e.g. 60.1234,10.5678)", s.to_string()
            ));
        }

        let lon: f64 = parts[0]
            .parse()
            .context("Longitude could not be parsed to float.")?;
        let lat: f64 = parts[1]
            .parse()
            .context("Latitude could not be parsed to float.")?;

        LonLatAlt::new(lat, lon, None).context("Error creating coordinates.")
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::coordinates::Latitude;

    use super::{LonLatAlt, Longitude};

    #[test]
    fn test_boundary_longitude() {
        let upper = Longitude::new(180.0).unwrap();
        assert_eq!(upper.value(), 180.0);

        let lower = Longitude::new(-180.0).unwrap();
        assert_eq!(lower.value(), -180.0);

        let upper_err = Longitude::new(181.0);
        assert!(upper_err.is_err());

        let lower_err = Longitude::new(-181.0);
        assert!(lower_err.is_err());
    }

    #[test]
    fn test_boundary_latitude() {
        let upper = Latitude::new(90.0).unwrap();
        assert_eq!(upper.value(), 90.0);

        let lower = Latitude::new(-90.0).unwrap();
        assert_eq!(lower.value(), -90.0);

        let upper_err = Latitude::new(91.0);
        assert!(upper_err.is_err());

        let lower_err = Latitude::new(-91.0);
        assert!(lower_err.is_err());
    }

    #[test]
    fn test_decimal_rounding() {
        let coord = LonLatAlt::new(51.00056, 60.10002, None).unwrap();
        assert_eq!(coord.latitude.value(), 60.1000);
        assert_eq!(coord.longitude.value(), 51.0006);
    }
}
