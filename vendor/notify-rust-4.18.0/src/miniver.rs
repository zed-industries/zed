use crate::error::*;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, Debug)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
}

impl Version {
    #[allow(dead_code)]
    pub fn new(major: u64, minor: u64) -> Self {
        Self { major, minor }
    }
}

impl FromStr for Version {
    type Err = Error;
    fn from_str(s: &str) -> Result<Version> {
        let vv = s.split('.').collect::<Vec<&str>>();
        match (vv.first(), vv.get(1)) {
            (Some(maj), Some(min)) => Ok(Version {
                major: maj.parse()?,
                minor: min.parse()?,
            }),
            _ => Err(ErrorKind::SpecVersion(s.into()).into()),
        }
    }
}

use std::cmp;

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.major == other.major && self.minor == other.minor
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> cmp::Ordering {
        match self.major.cmp(&other.major) {
            cmp::Ordering::Equal => {}
            r => return r,
        }
        match self.minor.cmp(&other.minor) {
            cmp::Ordering::Equal => {}
            r => return r,
        }
        cmp::Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_parsing() {
        assert_eq!("1.3".parse::<Version>().unwrap(), Version::new(1, 3));
    }

    #[test]
    fn version_comparison() {
        assert!(Version::new(1, 3) >= Version::new(1, 2));
        assert!(Version::new(1, 2) >= Version::new(1, 2));
        assert!(Version::new(1, 2) == Version::new(1, 2));
        assert!(Version::new(1, 1) <= Version::new(1, 2));
    }
}
