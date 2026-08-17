use std::str::FromStr;

use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

/// A simple URI wrapper that stores the URI as a string with minimal
/// validation.
///
/// This is a lightweight alternative to `url::Url` for cases where we just need
/// to store and pass URIs without extensive parsing or manipulation.
///
/// URIs must contain `://` to be valid (e.g., `file://`, `http://`, `https://`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Type)]
#[zvariant(signature = "s")]
pub struct Uri(String);

impl Uri {
    /// Parses the URI, doing only two validations:
    ///
    /// - Ensuring it is not an empty string
    /// - That it contains the schema part by checking for `://`
    pub fn parse(uri: &str) -> Result<Self, ParseError> {
        Self::from_str(uri)
    }

    /// Get the URI as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Uri {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseError::Empty);
        }
        if !s.contains("://") {
            return Err(ParseError::MissingScheme);
        }
        Ok(Self(s.to_string()))
    }
}

impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse()
            .map_err(|e: ParseError| serde::de::Error::custom(e.to_string()))
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Error type for URI parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The URI string was empty.
    Empty,
    /// The URI is missing a scheme (must contain `://`).
    MissingScheme,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "URI cannot be empty"),
            Self::MissingScheme => write!(f, "URI must contain a scheme (e.g., file://, http://)"),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(feature = "glib")]
impl TryFrom<Uri> for glib::Uri {
    type Error = glib::Error;

    fn try_from(uri: Uri) -> Result<Self, Self::Error> {
        glib::Uri::parse(uri.as_str(), glib::UriFlags::NONE)
    }
}

#[cfg(feature = "glib")]
impl TryFrom<&Uri> for glib::Uri {
    type Error = glib::Error;

    fn try_from(uri: &Uri) -> Result<Self, Self::Error> {
        glib::Uri::parse(uri.as_str(), glib::UriFlags::NONE)
    }
}
