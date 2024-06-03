use core::fmt;
use std::{fmt::Display, str::FromStr};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Ratio eg. 1/1000
pub struct Ratio((u64, u64));

impl Ratio {
    pub fn numerator(&self) -> u64 {
        self.0 .0
    }
    pub fn denominator(&self) -> u64 {
        self.0 .1
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}:{}", self.0 .0, self.0 .1))
    }
}

impl FromStr for Ratio {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(|c| c == ':' || c == '/').collect();
        if parts.len() != 2 {
            return Err("Invalid ratio format".to_string());
        }
        let numerator = parts[0].parse::<u64>().map_err(|_| "Invalid number")?;
        let denominator = parts[1].parse::<u64>().map_err(|_| "Invalid number")?;
        Ok(Ratio((numerator, denominator)))
    }
}

impl Serialize for Ratio {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Ratio {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RatioVisitor)
    }
}

struct RatioVisitor;

impl<'de> Visitor<'de> for RatioVisitor {
    type Value = Ratio;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "a ratio in the format of 'numerator:denominator' or 'numerator/denominator'",
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Ratio, E>
    where
        E: de::Error,
    {
        Ratio::from_str(value).map_err(de::Error::custom)
    }
}
