//! # Color
//!
//! The `color` crate provides a set utilities for working with colors. It is a wrapper around the [`palette`](https://docs.rs/palette) crate with some additional functionality.
//!
//! It is used to create a manipulate colors when building themes.
//!
//! === In development note ===
//!
//! This crate is meant to sit between gpui and the theme/ui for all the color related stuff.
//!
//! It could be folded into gpui, ui or theme potentially but for now we'll continue
//! to develop it in isolation.
//!
//! Once we have a good idea of the needs of the theme system and color in gpui in general I see 3 paths:
//! 1. Use `palette` (or another color library) directly in gpui and everywhere else, rather than rolling our own color system.
//! 2. Keep this crate as a thin wrapper around `palette` and use it everywhere except gpui, and convert to gpui's color system when needed.
//! 3. Build the needed functionality into gpui and keep using its color system everywhere.
//!
//! I'm leaning towards 2 in the short term and 1 in the long term, but we'll need to discuss it more.
//!
//! === End development note ===
use palette::{
    blend::Blend, convert::FromColorUnclamped, encoding, rgb::Rgb, Clamp, Mix, Srgb, WithAlpha,
};

/// The types of blend modes supported
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BlendMode {
    /// Multiplies the colors, resulting in a darker color. This mode is useful for creating shadows.
    Multiply,
    /// Lightens the color by adding the source and destination colors. It results in a lighter color.
    Screen,
    /// Combines Multiply and Screen blend modes. Parts of the image that are lighter than 50% gray are lightened, and parts that are darker are darkened.
    Overlay,
    /// Selects the darker of the base or blend color as the resulting color. Useful for darkening images without affecting the overall contrast.
    Darken,
    /// Selects the lighter of the base or blend color as the resulting color. Useful for lightening images without affecting the overall contrast.
    Lighten,
    /// Brightens the base color to reflect the blend color. The result is a lightened image.
    Dodge,
    /// Darkens the base color to reflect the blend color. The result is a darkened image.
    Burn,
    /// Similar to Overlay, but with a stronger effect. Hard Light can either multiply or screen colors, depending on the blend color.
    HardLight,
    /// A softer version of Hard Light. Soft Light either darkens or lightens colors, depending on the blend color.
    SoftLight,
    /// Subtracts the darker of the two constituent colors from the lighter color. Difference mode is useful for creating more vivid colors.
    Difference,
    /// Similar to Difference, but with a lower contrast. Exclusion mode produces an effect similar to Difference but with less intensity.
    Exclusion,
}

/// Converts a hexadecimal color string to a `palette::Hsla` color.
///
/// This function supports the following hex formats:
/// `#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`.
pub fn hex_to_hsla(s: &str) -> Result<RGBAColor, String> {
    let hex = s.trim_start_matches('#');

    // Expand shorthand formats #RGB and #RGBA to #RRGGBB and #RRGGBBAA
    let hex = match hex.len() {
        3 => hex.chars().map(|c| c.to_string().repeat(2)).collect(),
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

    Ok(RGBAColor {
        r: ((hex_val >> 24) & 0xFF) as f32 / 255.0,
        g: ((hex_val >> 16) & 0xFF) as f32 / 255.0,
        b: ((hex_val >> 8) & 0xFF) as f32 / 255.0,
        a: (hex_val & 0xFF) as f32 / 255.0,
    })
}

// These derives implement to and from palette's color types.
#[derive(FromColorUnclamped, WithAlpha, Debug, Clone)]
#[palette(skip_derives(Rgb), rgb_standard = "encoding::Srgb")]
pub struct RGBAColor {
    r: f32,
    g: f32,
    b: f32,
    // Let Palette know this is our alpha channel.
    #[palette(alpha)]
    a: f32,
}

impl FromColorUnclamped<RGBAColor> for RGBAColor {
    fn from_color_unclamped(color: RGBAColor) -> RGBAColor {
        color
    }
}

impl<S> FromColorUnclamped<Rgb<S, f32>> for RGBAColor
where
    Srgb: FromColorUnclamped<Rgb<S, f32>>,
{
    fn from_color_unclamped(color: Rgb<S, f32>) -> RGBAColor {
        let srgb = Srgb::from_color_unclamped(color);
        RGBAColor {
            r: srgb.red,
            g: srgb.green,
            b: srgb.blue,
            a: 1.0,
        }
    }
}

impl<S> FromColorUnclamped<RGBAColor> for Rgb<S, f32>
where
    Rgb<S, f32>: FromColorUnclamped<Srgb>,
{
    fn from_color_unclamped(color: RGBAColor) -> Self {
        Self::from_color_unclamped(Srgb::new(color.r, color.g, color.b))
    }
}

impl Clamp for RGBAColor {
    fn clamp(self) -> Self {
        RGBAColor {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
            a: self.a.min(1.0).max(0.0),
        }
    }
}

impl RGBAColor {
    /// Creates a new color from the given RGBA values.
    ///
    /// This color can be used to convert to any [`palette::Color`] type.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        RGBAColor { r, g, b, a }
    }

    /// Returns a set of states for this color.
    pub fn states(self, is_light: bool) -> ColorStates {
        states_for_color(self, is_light)
    }

    /// Mixes this color with another [`palette::Hsl`] color at the given `mix_ratio`.
    pub fn mixed(&self, other: RGBAColor, mix_ratio: f32) -> Self {
        let srgb_self = Srgb::new(self.r, self.g, self.b);
        let srgb_other = Srgb::new(other.r, other.g, other.b);

        // Directly mix the colors as sRGB values
        let mixed = srgb_self.mix(srgb_other, mix_ratio);
        RGBAColor::from_color_unclamped(mixed)
    }

    pub fn blend(&self, other: RGBAColor, blend_mode: BlendMode) -> Self {
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
    pub default: RGBAColor,
    /// The color when the mouse is hovering over the element.
    pub hover: RGBAColor,
    /// The color when the mouse button is held down on the element.
    pub active: RGBAColor,
    /// The color when the element is focused with the keyboard.
    pub focused: RGBAColor,
    /// The color when the element is disabled.
    pub disabled: RGBAColor,
}

/// Returns a set of colors for different states of an element.
///
/// todo("This should take a theme and use appropriate colors from it")
pub fn states_for_color(color: RGBAColor, is_light: bool) -> ColorStates {
    let adjustment_factor = if is_light { 0.1 } else { -0.1 };
    let hover_adjustment = 1.0 - adjustment_factor;
    let active_adjustment = 1.0 - 2.0 * adjustment_factor;
    let focused_adjustment = 1.0 - 3.0 * adjustment_factor;
    let disabled_adjustment = 1.0 - 4.0 * adjustment_factor;

    let make_adjustment = |color: RGBAColor, adjustment: f32| -> RGBAColor {
        // Adjust lightness for each state
        // Note: Adjustment logic may differ; simplify as needed for sRGB
        RGBAColor::new(
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
