use std::str::FromStr;

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

pub fn optional_naive_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    // If the string is empty, return None
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.trim() == "" {
        return Ok(None);
    }
    match NaiveDate::from_str(&s) {
        Ok(date) => Ok(Some(date)),
        Err(_) => Err(serde::de::Error::custom("Invalid date format")),
    }
}
