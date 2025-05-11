//! Constructs for working with [semantic versions](https://semver.org/).

#![deny(missing_docs)]

use std::{
    fmt::{self, Display},
    str::FromStr,
};

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize, de::Error};

/// A [semantic version](https://semver.org/) number.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SemanticVersion {
    major: usize,
    minor: usize,
    patch: usize,
}

impl SemanticVersion {
    /// Returns a new [`SemanticVersion`] from the given components.
    pub const fn new(major: usize, minor: usize, patch: usize) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version number.
    #[inline(always)]
    pub fn major(&self) -> usize {
        self.major
    }

    /// Returns the minor version number.
    #[inline(always)]
    pub fn minor(&self) -> usize {
        self.minor
    }

    /// Returns the patch version number.
    #[inline(always)]
    pub fn patch(&self) -> usize {
        self.patch
    }
}

impl FromStr for SemanticVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut components = s.trim().split('.');
        let major = components
            .next()
            .ok_or_else(|| anyhow!("missing major version number"))?
            .parse()?;
        let minor = components
            .next()
            .ok_or_else(|| anyhow!("missing minor version number"))?
            .parse()?;
        let patch = components
            .next()
            .ok_or_else(|| anyhow!("missing patch version number"))?
            .parse()?;
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl Display for SemanticVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Serialize for SemanticVersion {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SemanticVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Self::from_str(&string)
            .map_err(|_| Error::custom(format!("Invalid version string \"{string}\"")))
    }
}
