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

/// Calculate the relative luminance of an [`Hsla`] color according to WCAG 2.1 specifications.
pub fn relative_luminance(color: Hsla) -> f32 {
    let rgba = Rgba::from(color);
    let to_linear = |c: f32| -> f32 {
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let r = to_linear(rgba.r);
    let g = to_linear(rgba.g);
    let b = to_linear(rgba.b);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Calculate WCAG 2.1 contrast ratio between foreground and background colors.
/// Returns a ratio in the range `1.0..=21.0`.
pub fn contrast_ratio(fg: Hsla, bg: Hsla) -> f32 {
    let l1 = relative_luminance(fg);
    let l2 = relative_luminance(bg);
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    (lighter + 0.05) / (darker + 0.05)
}

/// Verify if a color pair meets WCAG AA standard (4.5:1 for normal text, 3.0:1 for large text).
pub fn meets_wcag_aa(fg: Hsla, bg: Hsla, is_large_text: bool) -> bool {
    let ratio = contrast_ratio(fg, bg);
    if is_large_text {
        ratio >= 3.0
    } else {
        ratio >= 4.5
    }
}
