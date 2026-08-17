use super::EndpointError;

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

/// The type of transport used by a given endpoint
#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Transport {
    /// TCP transport
    Tcp,
    Ipc,
}

impl Transport {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Transport::Tcp => "tcp",
            Transport::Ipc => "ipc",
        }
    }
}

impl FromStr for Transport {
    type Err = EndpointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "tcp" => Transport::Tcp,
            "ipc" => Transport::Ipc,
            _ => return Err(EndpointError::UnknownTransport(s.to_string())),
        };
        Ok(result)
    }
}

impl TryFrom<&str> for Transport {
    type Error = EndpointError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl fmt::Display for Transport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(self.as_str())
    }
}
