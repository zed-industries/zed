// SPDX-License-Identifier: MIT OR Apache-2.0

use core::fmt::Display;

use core::ops::Range;

use crate::{math, CacheKey, CacheKeyFlags, Color, GlyphDecorationData};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use core_maths::CoreFloat;

/// A laid out glyph
#[derive(Clone, Debug)]
pub struct LayoutGlyph {
    /// Start index of cluster in original line
    pub start: usize,
    /// End index of cluster in original line
    pub end: usize,
    /// Font size of the glyph
    pub font_size: f32,
    /// Font weight of the glyph
    pub font_weight: fontdb::Weight,
    /// Line height of the glyph, will override buffer setting
    pub line_height_opt: Option<f32>,
    /// Font id of the glyph
    pub font_id: fontdb::ID,
    /// Font id of the glyph
    pub glyph_id: u16,
    /// X offset of hitbox
    pub x: f32,
    /// Y offset of hitbox
    pub y: f32,
    /// Width of hitbox
    pub w: f32,
    /// Unicode `BiDi` embedding level, character is left-to-right if `level` is divisible by 2
    pub level: unicode_bidi::Level,
    /// X offset in line
    ///
    /// If you are dealing with physical coordinates, use [`Self::physical`] to obtain a
    /// [`PhysicalGlyph`] for rendering.
    ///
    /// This offset is useful when you are dealing with logical units and you do not care or
    /// cannot guarantee pixel grid alignment. For instance, when you want to use the glyphs
    /// for vectorial text, apply linear transformations to the layout, etc.
    pub x_offset: f32,
    /// Y offset in line
    ///
    /// If you are dealing with physical coordinates, use [`Self::physical`] to obtain a
    /// [`PhysicalGlyph`] for rendering.
    ///
    /// This offset is useful when you are dealing with logical units and you do not care or
    /// cannot guarantee pixel grid alignment. For instance, when you want to use the glyphs
    /// for vectorial text, apply linear transformations to the layout, etc.
    pub y_offset: f32,
    /// Optional color override
    pub color_opt: Option<Color>,
    /// Metadata from `Attrs`
    pub metadata: usize,
    /// [`CacheKeyFlags`]
    pub cache_key_flags: CacheKeyFlags,
}

/// A span of consecutive glyphs sharing the same text decoration.
#[derive(Clone, Debug, PartialEq)]
pub struct DecorationSpan {
    /// Range of glyph indices in `LayoutLine::glyphs` covered by this span
    pub glyph_range: Range<usize>,
    /// The decoration config and metrics
    pub data: GlyphDecorationData,
    /// Fallback color from the first glyph's `color_opt`
    pub color_opt: Option<Color>,
    /// Font size from the first glyph (used to scale EM-unit metrics)
    pub font_size: f32,
}

#[derive(Clone, Debug)]
pub struct PhysicalGlyph {
    /// Cache key, see [`CacheKey`]
    pub cache_key: CacheKey,
    /// Integer component of X offset in line
    pub x: i32,
    /// Integer component of Y offset in line
    pub y: i32,
}

impl LayoutGlyph {
    pub fn physical(&self, offset: (f32, f32), scale: f32) -> PhysicalGlyph {
        let x_offset = self.font_size * self.x_offset;
        let y_offset = self.font_size * self.y_offset;

        let (cache_key, x, y) = CacheKey::new(
            self.font_id,
            self.glyph_id,
            self.font_size * scale,
            (
                (self.x + x_offset).mul_add(scale, offset.0),
                math::truncf((self.y - y_offset).mul_add(scale, offset.1)), // Hinting in Y axis
            ),
            self.font_weight,
            self.cache_key_flags,
        );

        PhysicalGlyph { cache_key, x, y }
    }
}

/// A line of laid out glyphs
#[derive(Clone, Debug)]
pub struct LayoutLine {
    /// Width of the line
    pub w: f32,
    /// Maximum ascent of the glyphs in line
    pub max_ascent: f32,
    /// Maximum descent of the glyphs in line
    pub max_descent: f32,
    /// Maximum line height of any spans in line
    pub line_height_opt: Option<f32>,
    /// Glyphs in line
    pub glyphs: Vec<LayoutGlyph>,
    /// Text decoration spans covering ranges of glyphs
    pub decorations: Vec<DecorationSpan>,
}

/// Wrapping mode
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Wrap {
    /// No wrapping
    None,
    /// Wraps at a glyph level
    Glyph,
    /// Wraps at the word level
    Word,
    /// Wraps at the word level, or fallback to glyph level if a word can't fit on a line by itself
    WordOrGlyph,
}

impl Display for Wrap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::None => write!(f, "No Wrap"),
            Self::Word => write!(f, "Word Wrap"),
            Self::WordOrGlyph => write!(f, "Word Wrap or Character"),
            Self::Glyph => write!(f, "Character"),
        }
    }
}

/// Align or justify
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Align {
    Left,
    Right,
    Center,
    Justified,
    End,
}

impl Display for Align {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Left => write!(f, "Left"),
            Self::Right => write!(f, "Right"),
            Self::Center => write!(f, "Center"),
            Self::Justified => write!(f, "Justified"),
            Self::End => write!(f, "End"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Ellipsize {
    /// No Ellipsizing
    #[default]
    None,
    /// Ellipsizes the start of the last visual line that fits within the `EllipsizeHeightLimit`
    Start(EllipsizeHeightLimit),
    /// Ellipsizes the middle of the last visual line that fits within the `EllipsizeHeightLimit`.
    Middle(EllipsizeHeightLimit),
    /// Ellipsizes the end of the last visual line that fits within the `EllipsizeHeightLimit`.
    End(EllipsizeHeightLimit),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EllipsizeHeightLimit {
    /// Number of lines to show before ellipsizing the rest. Only works if `Wrap` is NOT set to
    /// `Wrap::None`. Otherwise, it will be ignored and the behavior will be the same as `Lines(1)`
    Lines(usize),
    /// Ellipsizes the last line that fits within the given height limit. If `Wrap` is set to
    /// `Wrap::None`, the behavior will be the same as `Lines(1)`
    Height(f32),
}

/// Metrics hinting strategy
#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub enum Hinting {
    /// No metrics hinting.
    ///
    /// Glyphs will have subpixel coordinates.
    ///
    /// This is the default.
    #[default]
    Disabled,

    /// Metrics hinting.
    ///
    /// Glyphs will be snapped to integral coordinates in the X-axis during layout.
    /// This can improve readability for smaller text and/or low-DPI screens.
    ///
    /// However, in order to get the right effect, you must use physical coordinates
    /// during layout and avoid further scaling when rendering. Otherwise, the rounding
    /// errors can accumulate and glyph distances may look erratic.
    ///
    /// In other words, metrics hinting makes layouting dependent of the target
    /// resolution.
    Enabled,
}
