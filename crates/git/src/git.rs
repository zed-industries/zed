mod hosting_provider;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fmt;
use std::str::FromStr;

pub use git2 as libgit;
pub use lazy_static::lazy_static;

pub use crate::hosting_provider::*;

pub mod blame;
pub mod commit;
pub mod diff;
pub mod repository;
pub mod status;

lazy_static! {
    pub static ref DOT_GIT: &'static OsStr = OsStr::new(".git");
    pub static ref GITIGNORE: &'static OsStr = OsStr::new(".gitignore");
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Oid(libgit::Oid);

impl Oid {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let oid = libgit::Oid::from_bytes(bytes).context("failed to parse bytes into git oid")?;
        Ok(Self(oid))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Returns this [`Oid`] as a short SHA.
    pub fn display_short(&self) -> String {
        self.to_string().chars().take(7).collect()
    }
}

impl FromStr for Oid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        libgit::Oid::from_str(s)
            .map_err(|error| anyhow!("failed to parse git oid: {}", error))
            .map(|oid| Self(oid))
    }
}

impl fmt::Debug for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for Oid {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Oid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<Oid>().map_err(serde::de::Error::custom)
    }
}

impl Default for Oid {
    fn default() -> Self {
        Self(libgit::Oid::zero())
    }
}

impl From<Oid> for u32 {
    fn from(oid: Oid) -> Self {
        let bytes = oid.0.as_bytes();
        debug_assert!(bytes.len() > 4);

        let mut u32_bytes: [u8; 4] = [0; 4];
        u32_bytes.copy_from_slice(&bytes[..4]);

        u32::from_ne_bytes(u32_bytes)
    }
}

impl From<Oid> for usize {
    fn from(oid: Oid) -> Self {
        let bytes = oid.0.as_bytes();
        debug_assert!(bytes.len() > 8);

        let mut u64_bytes: [u8; 8] = [0; 8];
        u64_bytes.copy_from_slice(&bytes[..8]);

        u64::from_ne_bytes(u64_bytes) as usize
    }
}
