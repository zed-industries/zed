//! # Color
//!
//! The `color` crate provides a set utilities for working with colors. It is a wrapper around the [`palette`](https://docs.rs/palette) crate with some additional functionality.
//!
//! It is used to create a manipulate colors when building themes.
//!
//! **Note:** This crate does not depend on `gpui`, so it does not provide any
//! interfaces for converting to `gpui` style colors.

use palette::{FromColor, Hsl, Hsla, Mix, Srgba, WithAlpha};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BlendMode {
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    Dodge,
    Burn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
}

/// Creates a new [`palette::Hsl`] color.
pub fn hsl(h: f32, s: f32, l: f32) -> Hsl {
    Hsl::new_srgb(h, s, l)
}

/// Converts a hexadecimal color string to a `palette::Hsla` color.
///
/// This function supports the following hex formats:
/// `#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`.
pub fn hex_to_hsla(s: &str) -> Result<Hsla, String> {
    let hex = s.trim_start_matches('#');

    // Expand shorthand formats #RGB and #RGBA to #RRGGBB and #RRGGBBAA
    let hex = match hex.len() {
        3 => hex
            .chars()
            .map(|c| c.to_string().repeat(2))
            .collect::<String>(),
        4 => {
            let (rgb, alpha) = hex.split_at(3);
            let rgb = rgb
                .chars()
                .map(|c| c.to_string().repeat(2))
                .collect::<String>();
            let alpha = alpha.chars().next().unwrap().to_string().repeat(2);
            format!("{}{}", rgb, alpha)
        }
        6 => format!("{}ff", hex), // Add alpha if missing
        8 => hex.to_string(),      // Already in full format
        _ => return Err("Invalid hexadecimal string length".to_string()),
    };

    let hex_val =
        u32::from_str_radix(&hex, 16).map_err(|_| format!("Invalid hexadecimal string: {}", s))?;

    let r = ((hex_val >> 24) & 0xFF) as f32 / 255.0;
    let g = ((hex_val >> 16) & 0xFF) as f32 / 255.0;
    let b = ((hex_val >> 8) & 0xFF) as f32 / 255.0;
    let a = (hex_val & 0xFF) as f32 / 255.0;

    let srgba = Srgba::new(r, g, b, a);
    let hsl = Hsl::from_color(srgba);
    let hsla = Hsla::from(hsl).with_alpha(a);

    Ok(hsla)
}

/// Mixes two [`palette::Hsl`] colors at the given `mix_ratio`.
pub fn hsl_mix(hsla_1: Hsl, hsla_2: Hsl, mix_ratio: f32) -> Hsl {
    hsla_1.mix(hsla_2, mix_ratio).into()
}

/// Represents a color
/// An interstitial state used to provide a consistent API for colors
/// with additional functionality like color mixing, blending, etc.
///
/// Does not return [gpui] colors as the `color` crate does not
/// depend on [gpui].
#[derive(Debug, Copy, Clone)]
pub struct Color {
    value: Hsla,
}

impl Color {
    /// Creates a new [`Color`] with an alpha value.
    pub fn new(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        Self {
            value: Hsla::new(hue, saturation, lightness, alpha),
        }
    }

    /// Creates a new [`Color`] with an alpha value of `1.0`.
    pub fn hsl(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self::new(hue, saturation, lightness, 1.0)
    }

    /// Returns the [`palette::Hsla`] value of this color.
    pub fn value(&self) -> Hsla {
        self.value
    }

    /// Returns a set of states for this color.
    pub fn states(&self, is_light: bool) -> ColorStates {
        states_for_color(*self, is_light)
    }

    /// Mixes this color with another [`palette::Hsl`] color at the given `mix_ratio`.
    pub fn mix(&self, other: Hsl, mix_ratio: f32) -> Self {
        let mixed = self.value.mix(other.into(), mix_ratio);

        Self {
            value: mixed.into(),
        }
    }
}

/// A set of colors for different states of an element.
#[derive(Debug, Copy, Clone)]
pub struct ColorStates {
    /// The default color.
    pub default: Color,
    /// The color when the mouse is hovering over the element.
    pub hover: Color,
    /// The color when the mouse button is held down on the element.
    pub active: Color,
    /// The color when the element is focused with the keyboard.
    pub focused: Color,
    /// The color when the element is disabled.
    pub disabled: Color,
}

/// Returns a set of colors for different states of an element.
///
/// todo!("Test and improve this function")
pub fn states_for_color(color: Color, is_light: bool) -> ColorStates {
    let hover_lightness = if is_light { 0.9 } else { 0.1 };
    let active_lightness = if is_light { 0.8 } else { 0.2 };
    let focused_lightness = if is_light { 0.7 } else { 0.3 };
    let disabled_lightness = if is_light { 0.6 } else { 0.5 };

    let hover = color.mix(hsl(0.0, 0.0, hover_lightness), 0.1);
    let active = color.mix(hsl(0.0, 0.0, active_lightness), 0.1);
    let focused = color.mix(hsl(0.0, 0.0, focused_lightness), 0.1);
    let disabled = color.mix(hsl(0.0, 0.0, disabled_lightness), 0.1);

    ColorStates {
        default: color,
        hover,
        active,
        focused,
        disabled,
    }
}
