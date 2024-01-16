//! # Color
//!
//! The `color` crate provides a set utilities for working with colors. It is a wrapper around the [`palette`](https://docs.rs/palette) crate with some additional functionality.
//!
//! It is used to create a manipulate colors when building themes.
//!
//! **Note:** This crate does not depend on `gpui`, so it does not provide any
//! interfaces for converting to `gpui` style colors.

use palette::{
    blend::Blend, convert::FromColorUnclamped, encoding, rgb::Rgb, Clamp, Mix, Srgb, WithAlpha,
};

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

/// Converts a hexadecimal color string to a `palette::Hsla` color.
///
/// This function supports the following hex formats:
/// `#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`.
pub fn hex_to_hsla(s: &str) -> Result<Color, String> {
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

    let color = Color { r, g, b, a };

    Ok(color)
}

// This implements conversion to and from all Palette colors.
#[derive(FromColorUnclamped, WithAlpha, Debug, Clone)]
// We have to tell Palette that we will take care of converting to/from sRGB.
#[palette(skip_derives(Rgb), rgb_standard = "encoding::Srgb")]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    // Let Palette know this is our alpha channel.
    #[palette(alpha)]
    a: f32,
}

// There's no blanket implementation for Self -> Self, unlike the From trait.
// This is to better allow cases like Self<A> -> Self<B>.
impl FromColorUnclamped<Color> for Color {
    fn from_color_unclamped(color: Color) -> Color {
        color
    }
}

// Convert from any kind of f32 sRGB.
impl<S> FromColorUnclamped<Rgb<S, f32>> for Color
where
    Srgb: FromColorUnclamped<Rgb<S, f32>>,
{
    fn from_color_unclamped(color: Rgb<S, f32>) -> Color {
        let srgb = Srgb::from_color_unclamped(color);
        Color {
            r: srgb.red,
            g: srgb.green,
            b: srgb.blue,
            a: 1.0,
        }
    }
}

// Convert into any kind of f32 sRGB.
impl<S> FromColorUnclamped<Color> for Rgb<S, f32>
where
    Rgb<S, f32>: FromColorUnclamped<Srgb>,
{
    fn from_color_unclamped(color: Color) -> Self {
        let srgb = Srgb::new(color.r, color.g, color.b);
        Self::from_color_unclamped(srgb)
    }
}

// Add the required clamping.
impl Clamp for Color {
    fn clamp(self) -> Self {
        Color {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
            a: self.a.min(1.0).max(0.0),
        }
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    /// Returns a set of states for this color.
    pub fn states(self, is_light: bool) -> ColorStates {
        states_for_color(self, is_light)
    }

    /// Mixes this color with another [`palette::Hsl`] color at the given `mix_ratio`.
    pub fn mixed(&self, other: Color, mix_ratio: f32) -> Self {
        let srgb_self = Srgb::new(self.r, self.g, self.b);
        let srgb_other = Srgb::new(other.r, other.g, other.b);

        // Directly mix the colors as sRGB values
        let mixed = srgb_self.mix(srgb_other, mix_ratio);
        Color::from_color_unclamped(mixed)
    }

    pub fn blend(&self, other: Color, blend_mode: BlendMode) -> Self {
        let srgb_self = Srgb::new(self.r, self.g, self.b);
        let srgb_other = Srgb::new(other.r, other.g, other.b);

        let blended = match blend_mode {
            // replace hsl methods with the respective sRGB methods
            BlendMode::Multiply => srgb_self.multiply(srgb_other),
            _ => unimplemented!(),
        };

        Self {
            r: blended.red,
            g: blended.green,
            b: blended.blue,
            a: self.a,
        }
    }
}

/// A set of colors for different states of an element.
#[derive(Debug, Clone)]
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
    let adjustment_factor = if is_light { 0.1 } else { -0.1 };
    let hover_adjustment = 1.0 - adjustment_factor;
    let active_adjustment = 1.0 - 2.0 * adjustment_factor;
    let focused_adjustment = 1.0 - 3.0 * adjustment_factor;
    let disabled_adjustment = 1.0 - 4.0 * adjustment_factor;

    let make_adjustment = |color: Color, adjustment: f32| -> Color {
        // Adjust lightness for each state
        // Note: Adjustment logic may differ; simplify as needed for sRGB
        Color::new(
            color.r * adjustment,
            color.g * adjustment,
            color.b * adjustment,
            color.a,
        )
    };

    let color = color.clamp();

    ColorStates {
        default: color.clone(),
        hover: make_adjustment(color.clone(), hover_adjustment),
        active: make_adjustment(color.clone(), active_adjustment),
        focused: make_adjustment(color.clone(), focused_adjustment),
        disabled: make_adjustment(color.clone(), disabled_adjustment),
    }
}
