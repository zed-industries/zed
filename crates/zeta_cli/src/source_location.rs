use std::{fmt, fmt::Display, path::Path, str::FromStr, sync::Arc};

use ::util::{paths::PathStyle, rel_path::RelPath};
use anyhow::{Result, anyhow};
use language::Point;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SourceLocation {
    pub path: Arc<RelPath>,
    pub point: Point,
}

impl Serialize for SourceLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SourceLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.path.display(PathStyle::Posix),
            self.point.row + 1,
            self.point.column + 1
        )
    }
}

impl FromStr for SourceLocation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow!(
                "Invalid source location. Expected 'file.rs:line:column', got '{}'",
                s
            ));
        }

        let path = RelPath::new(Path::new(&parts[0]), PathStyle::local())?.into_arc();
        let line: u32 = parts[1]
            .parse()
            .map_err(|_| anyhow!("Invalid line number: '{}'", parts[1]))?;
        let column: u32 = parts[2]
            .parse()
            .map_err(|_| anyhow!("Invalid column number: '{}'", parts[2]))?;

        // Convert from 1-based to 0-based indexing
        let point = Point::new(line.saturating_sub(1), column.saturating_sub(1));

        Ok(SourceLocation { path, point })
    }
}
