//! Conversions between gpui's [`Hsla`] and the OKLab / OKLCh perceptual color
//! spaces, backed by the `palette` crate.
//!
//! These are exposed so consumers can reason about perceptual color distance
//! (e.g. bracket colorization) without taking a direct dependency on `palette`.

use gpui::{Hsla, Rgba};
use palette::{
    FromColor, OklabHue,
    rgb::{LinSrgba, Srgba},
};

/// A color in the OKLab perceptual color space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Oklab {
    /// Perceptual lightness, in `0.0..=1.0`.
    pub l: f32,
    /// Green/red opponent axis.
    pub a: f32,
    /// Blue/yellow opponent axis.
    pub b: f32,
}

/// A color in the OKLCh perceptual color space (the cylindrical form of OKLab).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Oklch {
    /// Perceptual lightness, in `0.0..=1.0`.
    pub l: f32,
    /// Chroma (colorfulness).
    pub chroma: f32,
    /// Hue, in degrees (`0.0..360.0`).
    pub hue: f32,
}

/// Converts an [`Hsla`] color into the OKLab color space.
pub fn hsla_to_oklab(color: Hsla) -> Oklab {
    let oklab = palette::Oklab::from_color(hsla_to_linear(color));
    Oklab {
        l: oklab.l,
        a: oklab.a,
        b: oklab.b,
    }
}

/// Converts an [`Hsla`] color into the OKLCh color space.
pub fn hsla_to_oklch(color: Hsla) -> Oklch {
    let oklch = palette::Oklch::from_color(hsla_to_linear(color));
    Oklch {
        l: oklch.l,
        chroma: oklch.chroma,
        hue: oklch.hue.into_positive_degrees(),
    }
}

/// Converts an [`Oklch`] color back into [`Hsla`], using `alpha` for the
/// resulting alpha channel. Channels outside the sRGB gamut are clamped.
pub fn oklch_to_hsla(color: Oklch, alpha: f32) -> Hsla {
    let oklch = palette::Oklch {
        l: color.l,
        chroma: color.chroma,
        hue: OklabHue::from_degrees(color.hue),
    };
    let rgba: Srgba = Srgba::from_linear(LinSrgba::from_color(oklch));
    let (red, green, blue, _) = rgba.into_components();
    Hsla::from(Rgba {
        r: red.clamp(0.0, 1.0),
        g: green.clamp(0.0, 1.0),
        b: blue.clamp(0.0, 1.0),
        a: alpha,
    })
}

fn hsla_to_linear(color: Hsla) -> LinSrgba {
    let rgba = Rgba::from(color);
    Srgba::new(rgba.r, rgba.g, rgba.b, rgba.a).into_linear()
}
