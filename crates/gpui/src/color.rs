use std::{
    borrow::Cow,
    fmt,
    ops::{Deref, DerefMut},
};

use crate::json::ToJson;
use pathfinder_color::{ColorF, ColorU};
use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};
use serde_json::json;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    pub fn red() -> Self {
        Self(ColorU::from_u32(0xff0000ff))
    }

    pub fn green() -> Self {
        Self(ColorU::from_u32(0x00ff00ff))
    }

    pub fn blue() -> Self {
        Self(ColorU::from_u32(0x0000ffff))
    }

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(ColorU::new(r, g, b, a))
    }

    pub fn from_u32(rgba: u32) -> Self {
        Self(ColorU::from_u32(rgba))
    }

    pub fn blend(source: Color, dest: Color) -> Color {
        // Skip blending if we don't need it.
        if source.a == 255 {
            return source;
        } else if source.a == 0 {
            return dest;
        }

        let source = source.0.to_f32();
        let dest = dest.0.to_f32();

        let a = source.a() + (dest.a() * (1. - source.a()));
        let r = ((source.r() * source.a()) + (dest.r() * dest.a() * (1. - source.a()))) / a;
        let g = ((source.g() * source.a()) + (dest.g() * dest.a() * (1. - source.a()))) / a;
        let b = ((source.b() * source.a()) + (dest.b() * dest.a() * (1. - source.a()))) / a;

        Self(ColorF::new(r, g, b, a).to_u8())
    }

    pub fn fade_out(&mut self, fade: f32) {
        let fade = fade.clamp(0., 1.);
        self.0.a = (self.0.a as f32 * (1. - fade)) as u8;
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let literal: Cow<str> = Deserialize::deserialize(deserializer)?;
        if let Some(digits) = literal.strip_prefix('#') {
            if let Ok(value) = u32::from_str_radix(digits, 16) {
                if digits.len() == 6 {
                    return Ok(Color::from_u32((value << 8) | 0xFF));
                } else if digits.len() == 8 {
                    return Ok(Color::from_u32(value));
                }
            }
        }
        Err(de::Error::invalid_value(
            Unexpected::Str(literal.as_ref()),
            &"#RRGGBB[AA]",
        ))
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
