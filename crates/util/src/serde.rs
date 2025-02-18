use std::str::FromStr;

use serde::{de::DeserializeOwned, Deserialize, Deserializer, de::Error};

pub const fn default_true() -> bool {
    true
}

pub fn deserialize_enum_fromstr<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: DeserializeOwned + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(s.as_str()).map_err(D::Error::custom)
}

pub fn deserialize_option_enum_fromstr<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: DeserializeOwned + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    if let Some(s) = Option::<String>::deserialize(deserializer)? {
        T::from_str(s.as_str()).map(Some).map_err(D::Error::custom)
    } else {
        Ok(None)
    }
}
