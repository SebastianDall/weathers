use anyhow::{anyhow, Result};

#[derive(Clone, Debug, PartialEq)]
struct Longitude(f64);

impl Longitude {
    pub fn new(value: f64) -> Result<Self> {
        if value < -180.0 || value > 180.0 {
            return Err(anyhow!("Longitude must be in [-180, 180]"));
        }

        let rounded = (value * 10_000.0).round() / 10_000.0;

        Ok(Self(rounded))
    }
}

// impl fmt::Debug for Longitude {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

#[derive(Clone, Debug, PartialEq)]
struct Latitude(f64);

impl Latitude {
    pub fn new(value: f64) -> Result<Self> {
        if value < -90.0 || value > 90.0 {
            return Err(anyhow!("Latitude must be in [-90, 90]"));
        }

        let rounded = (value * 10_000.0).round() / 10_000.0;

        Ok(Self(rounded))
    }
}

// impl fmt::Debug for Latitude {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

/// A domain type representing latitude/longitude coordinates.
///
/// # Constraints
/// - Maximum 4 decimals
#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    pub longitude: Longitude,
    pub latitude: Latitude,
}

impl Coordinates {
    /// Create new coordinate
    ///
    /// ```
    /// use domain::Coordinates;
    ///
    /// let coord = Coordinates::new(60, 0).unwrap();
    /// assert_eq!(coord.longitude, 60.0);
    ///
    /// let coord_err = Coordinates::new(0, 91);
    /// assert!(coord_err.is_err());
    /// ```
    pub fn new(lon: f64, lat: f64) -> Result<Self> {
        Ok(Self {
            latitude: Latitude::new(lat)?,
            longitude: Longitude::new(lon)?,
        })
    }
}
