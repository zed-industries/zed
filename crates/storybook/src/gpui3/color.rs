#![allow(dead_code)]

use bytemuck::{Pod, Zeroable};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;
use std::num::ParseIntError;

pub fn rgb<C: From<Rgba>>(hex: u32) -> C {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;
    Rgba { r, g, b, a: 1.0 }.into()
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

struct RgbaVisitor;

impl<'de> Visitor<'de> for RgbaVisitor {
    type Value = Rgba;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string in the format #rrggbb or #rrggbbaa")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Rgba, E> {
        if value.len() == 7 || value.len() == 9 {
            let r = u8::from_str_radix(&value[1..3], 16).unwrap() as f32 / 255.0;
            let g = u8::from_str_radix(&value[3..5], 16).unwrap() as f32 / 255.0;
            let b = u8::from_str_radix(&value[5..7], 16).unwrap() as f32 / 255.0;
            let a = if value.len() == 9 {
                u8::from_str_radix(&value[7..9], 16).unwrap() as f32 / 255.0
            } else {
                1.0
            };
            Ok(Rgba { r, g, b, a })
        } else {
            Err(E::custom(
                "Bad format for RGBA. Expected #rrggbb or #rrggbbaa.",
            ))
        }
    }
}

impl<'de> Deserialize<'de> for Rgba {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(RgbaVisitor)
    }
}

impl From<Hsla> for Rgba {
    fn from(color: Hsla) -> Self {
        let h = color.h;
        let s = color.s;
        let l = color.l;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;
        let cm = c + m;
        let xm = x + m;

        let (r, g, b) = match (h * 6.0).floor() as i32 {
            0 | 6 => (cm, xm, m),
            1 => (xm, cm, m),
            2 => (m, cm, xm),
            3 => (m, xm, cm),
            4 => (xm, m, cm),
            _ => (cm, m, xm),
        };

        Rgba {
            r,
            g,
            b,
            a: color.a,
        }
    }
}

impl TryFrom<&'_ str> for Rgba {
    type Error = ParseIntError;

    fn try_from(value: &'_ str) -> Result<Self, Self::Error> {
        let r = u8::from_str_radix(&value[1..3], 16)? as f32 / 255.0;
        let g = u8::from_str_radix(&value[3..5], 16)? as f32 / 255.0;
        let b = u8::from_str_radix(&value[5..7], 16)? as f32 / 255.0;
        let a = if value.len() > 7 {
            u8::from_str_radix(&value[7..9], 16)? as f32 / 255.0
        } else {
            1.0
        };

        Ok(Rgba { r, g, b, a })
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Hsla {
    pub h: f32,
    pub s: f32,
    pub l: f32,
    pub a: f32,
}

unsafe impl Zeroable for Hsla {}
unsafe impl Pod for Hsla {}

pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
    Hsla {
        h: h.clamp(0., 1.),
        s: s.clamp(0., 1.),
        l: l.clamp(0., 1.),
        a: a.clamp(0., 1.),
    }
}

pub fn black() -> Hsla {
    Hsla {
        h: 0.,
        s: 0.,
        l: 0.,
        a: 1.,
    }
}

impl Hsla {
    /// Returns true if the HSLA color is fully transparent, false otherwise.
    pub fn is_transparent(&self) -> bool {
        self.a == 0.0
    }
}

impl From<Rgba> for Hsla {
    fn from(color: Rgba) -> Self {
        let r = color.r;
        let g = color.g;
        let b = color.b;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let l = (max + min) / 2.0;
        let s = if l == 0.0 || l == 1.0 {
            0.0
        } else if l < 0.5 {
            delta / (2.0 * l)
        } else {
            delta / (2.0 - 2.0 * l)
        };

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            ((g - b) / delta).rem_euclid(6.0) / 6.0
        } else if max == g {
            ((b - r) / delta + 2.0) / 6.0
        } else {
            ((r - g) / delta + 4.0) / 6.0
        };

        Hsla {
            h,
            s,
            l,
            a: color.a,
        }
    }
}

impl<'de> Deserialize<'de> for Hsla {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First, deserialize it into Rgba
        let rgba = Rgba::deserialize(deserializer)?;

        // Then, use the From<Rgba> for Hsla implementation to convert it
        Ok(Hsla::from(rgba))
    }
}
