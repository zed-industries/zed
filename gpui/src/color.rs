use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use crate::json::ToJson;
use pathfinder_color::ColorU;
use serde::{Deserialize, Deserializer};
use serde_json::json;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Color(ColorU);

impl Color {
    pub fn transparent_black() -> Self {
        Self(ColorU::transparent_black())
    }

    pub fn black() -> Self {
        Self(ColorU::black())
    }

    pub fn white() -> Self {
        Self(ColorU::white())
    }

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(ColorU::new(r, g, b, a))
    }

    pub fn from_u32(rgba: u32) -> Self {
        Self(ColorU::from_u32(rgba))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut rgba = u32::deserialize(deserializer)?;

        if rgba <= 0xFFFFFF {
            rgba = (rgba << 8) + 0xFF;
        }

        Ok(Self::from_u32(rgba))
    }
}

impl ToJson for Color {
    fn to_json(&self) -> serde_json::Value {
        json!(format!(
            "0x{:x}{:x}{:x}{:x}",
            self.0.r, self.0.g, self.0.b, self.0.a
        ))
    }
}

impl Deref for Color {
    type Target = ColorU;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
