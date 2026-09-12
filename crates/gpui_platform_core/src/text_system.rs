//! Font identity, metrics, and rendering parameter types shared by `gpui` and its platform backends.

use derive_more::{Add, FromStr, Sub};
use gpui_shared_string::SharedString;
use gpui_types::{Bounds, Pixels, Point, px};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

use crate::{FontFallbacks, FontFeatures};

/// An opaque identifier for a specific font.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(C)]
pub struct FontId(pub usize);

/// An opaque identifier for a specific font family.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct FontFamilyId(pub usize);

/// An identifier for a specific glyph, as returned by the platform text system.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct GlyphId(pub u32);

/// The degree of blackness or stroke thickness of a font. This value ranges from 100.0 to 900.0,
/// with 400.0 as normal.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sub, FromStr)]
#[serde(transparent)]
pub struct FontWeight(pub f32);

impl Display for FontWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<f32> for FontWeight {
    fn from(weight: f32) -> Self {
        FontWeight(weight)
    }
}

impl Default for FontWeight {
    #[inline]
    fn default() -> FontWeight {
        FontWeight::NORMAL
    }
}

impl Hash for FontWeight {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(u32::from_be_bytes(self.0.to_be_bytes()));
    }
}

impl Eq for FontWeight {}

impl FontWeight {
    /// Thin weight (100), the thinnest value.
    pub const THIN: FontWeight = FontWeight(100.0);
    /// Extra light weight (200).
    pub const EXTRA_LIGHT: FontWeight = FontWeight(200.0);
    /// Light weight (300).
    pub const LIGHT: FontWeight = FontWeight(300.0);
    /// Normal (400).
    pub const NORMAL: FontWeight = FontWeight(400.0);
    /// Medium weight (500, higher than normal).
    pub const MEDIUM: FontWeight = FontWeight(500.0);
    /// Semibold weight (600).
    pub const SEMIBOLD: FontWeight = FontWeight(600.0);
    /// Bold weight (700).
    pub const BOLD: FontWeight = FontWeight(700.0);
    /// Extra-bold weight (800).
    pub const EXTRA_BOLD: FontWeight = FontWeight(800.0);
    /// Black weight (900), the thickest value.
    pub const BLACK: FontWeight = FontWeight(900.0);

    /// All of the font weights, in order from thinnest to thickest.
    pub const ALL: [FontWeight; 9] = [
        Self::THIN,
        Self::EXTRA_LIGHT,
        Self::LIGHT,
        Self::NORMAL,
        Self::MEDIUM,
        Self::SEMIBOLD,
        Self::BOLD,
        Self::EXTRA_BOLD,
        Self::BLACK,
    ];
}

impl schemars::JsonSchema for FontWeight {
    fn schema_name() -> Cow<'static, str> {
        "FontWeight".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        use schemars::json_schema;
        json_schema!({
            "type": "number",
            "minimum": Self::THIN,
            "maximum": Self::BLACK,
            "default": Self::default(),
            "description": "Font weight value between 100 (thin) and 900 (black)"
        })
    }
}

/// Allows italic or oblique faces to be selected.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub enum FontStyle {
    /// A face that is neither italic not obliqued.
    #[default]
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

impl Display for FontStyle {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

/// The configuration details for identifying a specific font.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Font {
    /// The font family name.
    ///
    /// The special name ".SystemUIFont" is used to identify the system UI font, which varies based on platform.
    pub family: SharedString,

    /// The font features to use.
    pub features: FontFeatures,

    /// The fallbacks fonts to use.
    pub fallbacks: Option<FontFallbacks>,

    /// The font weight.
    pub weight: FontWeight,

    /// The font style.
    pub style: FontStyle,
}

impl Default for Font {
    fn default() -> Self {
        font(".SystemUIFont")
    }
}

/// Get a [`Font`] for a given name.
pub fn font(family: impl Into<SharedString>) -> Font {
    Font {
        family: family.into(),
        features: FontFeatures::default(),
        weight: FontWeight::default(),
        style: FontStyle::default(),
        fallbacks: None,
    }
}

impl Font {
    /// Set this Font to be bold
    pub fn bold(mut self) -> Self {
        self.weight = FontWeight::BOLD;
        self
    }

    /// Set this Font to be italic
    pub fn italic(mut self) -> Self {
        self.style = FontStyle::Italic;
        self
    }
}

/// A struct for storing font metrics.
/// It is used to define the measurements of a typeface.
#[derive(Clone, Copy, Debug)]
pub struct FontMetrics {
    /// The number of font units that make up the "em square",
    /// a scalable grid for determining the size of a typeface.
    pub units_per_em: u32,

    /// The vertical distance from the baseline of the font to the top of the glyph covers.
    pub ascent: f32,

    /// The vertical distance from the baseline of the font to the bottom of the glyph covers.
    pub descent: f32,

    /// The recommended additional space to add between lines of type.
    pub line_gap: f32,

    /// The suggested position of the underline.
    pub underline_position: f32,

    /// The suggested thickness of the underline.
    pub underline_thickness: f32,

    /// The height of a capital letter measured from the baseline of the font.
    pub cap_height: f32,

    /// The height of a lowercase x.
    pub x_height: f32,

    /// The outer limits of the area that the font covers.
    /// Corresponds to the xMin / xMax / yMin / yMax values in the OpenType `head` table
    pub bounding_box: Bounds<f32>,
}

impl FontMetrics {
    /// Returns the vertical distance from the baseline of the font to the top of the glyph covers in pixels.
    pub fn ascent(&self, font_size: Pixels) -> Pixels {
        Pixels((self.ascent / self.units_per_em as f32) * font_size.0)
    }

    /// Returns the vertical distance from the baseline of the font to the bottom of the glyph covers in pixels.
    pub fn descent(&self, font_size: Pixels) -> Pixels {
        Pixels((self.descent / self.units_per_em as f32) * font_size.0)
    }

    /// Returns the recommended additional space to add between lines of type in pixels.
    pub fn line_gap(&self, font_size: Pixels) -> Pixels {
        Pixels((self.line_gap / self.units_per_em as f32) * font_size.0)
    }

    /// Returns the suggested position of the underline in pixels.
    pub fn underline_position(&self, font_size: Pixels) -> Pixels {
        Pixels((self.underline_position / self.units_per_em as f32) * font_size.0)
    }

    /// Returns the suggested thickness of the underline in pixels.
    pub fn underline_thickness(&self, font_size: Pixels) -> Pixels {
        Pixels((self.underline_thickness / self.units_per_em as f32) * font_size.0)
    }

    /// Returns the height of a capital letter measured from the baseline of the font in pixels.
    pub fn cap_height(&self, font_size: Pixels) -> Pixels {
        Pixels((self.cap_height / self.units_per_em as f32) * font_size.0)
    }

    /// Returns the height of a lowercase x in pixels.
    pub fn x_height(&self, font_size: Pixels) -> Pixels {
        Pixels((self.x_height / self.units_per_em as f32) * font_size.0)
    }

    /// Returns the outer limits of the area that the font covers in pixels.
    pub fn bounding_box(&self, font_size: Pixels) -> Bounds<Pixels> {
        (self.bounding_box / self.units_per_em as f32 * font_size.0).map(px)
    }
}

/// Parameters for rendering a glyph, used as cache keys for raster bounds.
///
/// This struct identifies a specific glyph rendering configuration including
/// font, size, subpixel positioning, and scale factor. It's used to look up
/// cached raster bounds and sprite atlas entries.
#[derive(Clone, Debug, PartialEq)]
#[expect(missing_docs)]
pub struct RenderGlyphParams {
    pub font_id: FontId,
    pub glyph_id: GlyphId,
    pub font_size: Pixels,
    pub subpixel_variant: Point<u8>,
    pub scale_factor: f32,
    pub is_emoji: bool,
    pub subpixel_rendering: bool,
    pub dilation: u8,
}

impl Eq for RenderGlyphParams {}

impl Hash for RenderGlyphParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.font_id.0.hash(state);
        self.glyph_id.0.hash(state);
        self.font_size.0.to_bits().hash(state);
        self.subpixel_variant.hash(state);
        self.scale_factor.to_bits().hash(state);
        self.is_emoji.hash(state);
        self.subpixel_rendering.hash(state);
        self.dilation.hash(state);
    }
}

/// A run of text with a single font.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[expect(missing_docs)]
pub struct FontRun {
    pub len: usize,
    pub font_id: FontId,
}

/// The text rendering mode to use for drawing glyphs.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum TextRenderingMode {
    /// Use the platform's default text rendering mode.
    #[default]
    PlatformDefault,
    /// Use subpixel (ClearType-style) text rendering.
    Subpixel,
    /// Use grayscale text rendering.
    Grayscale,
}
