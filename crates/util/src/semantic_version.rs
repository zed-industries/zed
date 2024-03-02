use std::{
    fmt::{self, Display},
    str::FromStr,
};

use anyhow::{anyhow, Result};
use serde::Serialize;

/// A datastructure representing a semantic version number
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SemanticVersion {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}

impl SemanticVersion {
    pub fn new(major: usize, minor: usize, patch: usize) -> Self {
        Self {
            major,
            minor,
            patch,
        }
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
