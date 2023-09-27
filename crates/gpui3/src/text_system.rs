mod font_features;
mod line_wrapper;
mod text_layout_cache;

pub use font_features::*;
use line_wrapper::*;
pub use text_layout_cache::*;

use crate::{
    px, Bounds, Hsla, Pixels, PlatformTextSystem, Point, Result, SharedString, Size, UnderlineStyle,
};
use collections::HashMap;
use core::fmt;
use parking_lot::Mutex;
use std::{
    borrow::BorrowMut,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    sync::Arc,
};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct FontId(pub usize);

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct FontFamilyId(pub usize);

pub struct TextSystem {
    text_layout_cache: Arc<TextLayoutCache>,
    platform_text_system: Arc<dyn PlatformTextSystem>,
    wrapper_pool: Mutex<HashMap<(Font, Pixels), Vec<LineWrapper>>>,
}

impl TextSystem {
    pub fn new(platform_text_system: Arc<dyn PlatformTextSystem>) -> Self {
        TextSystem {
            text_layout_cache: Arc::new(TextLayoutCache::new(platform_text_system.clone())),
            wrapper_pool: Mutex::new(HashMap::default()),
            platform_text_system,
        }
    }

    pub fn select_font(&self, descriptor: impl Into<Font>) -> Result<FontId> {
        self.platform_text_system.select_font(descriptor.into())
    }

    pub fn bounding_box(&self, font_id: FontId, font_size: Pixels) -> Size<Pixels> {
        let metrics = self.platform_text_system.font_metrics(font_id);
        metrics.bounding_box(font_size);

        todo!()
        // self.font_cache.bounding_box(font_id, font_size)
    }

    pub fn em_width(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        todo!()
        // self.font_cache.em_width(font_id, font_size)
    }

    pub fn em_advance(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        todo!()
        // self.font_cache.em_advance(font_id, font_size)
    }

    pub fn line_height(&self, font_size: Pixels) -> Pixels {
        todo!()
        // self.font_cache.line_height(font_size)
    }

    pub fn cap_height(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        todo!()
        // self.font_cache.cap_height(font_id, font_size)
    }

    pub fn x_height(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        todo!()
        // self.font_cache.x_height(font_id, font_size)
    }

    pub fn ascent(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        todo!()
        // self.font_cache.ascent(font_id, font_size)
    }

    pub fn descent(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        todo!()
        // self.font_cache.descent(font_id, font_size)
    }

    pub fn em_size(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        todo!()
        // self.font_cache.em_size(font_id, font_size)
    }

    pub fn baseline_offset(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        todo!()
        // self.font_cache.baseline_offset(font_id, font_size)
    }

    pub fn layout_str<'a>(
        &'a self,
        text: &'a str,
        font_size: Pixels,
        runs: &'a [(usize, RunStyle)],
    ) -> Line {
        self.text_layout_cache.layout_str(text, font_size, runs)
    }

    pub fn finish_frame(&self) {
        self.text_layout_cache.finish_frame()
    }

    pub fn line_wrapper(self: &Arc<Self>, font: Font, font_size: Pixels) -> LineWrapperHandle {
        let lock = &mut self.wrapper_pool.lock();
        let wrappers = lock.entry((font.clone(), font_size)).or_default();
        let wrapper = wrappers.pop().unwrap_or_else(|| {
            LineWrapper::new(font, font_size, self.platform_text_system.clone())
        });

        LineWrapperHandle {
            wrapper: Some(wrapper),
            text_system: self.clone(),
        }
    }
}

pub struct LineWrapperHandle {
    wrapper: Option<LineWrapper>,
    text_system: Arc<TextSystem>,
}

impl Drop for LineWrapperHandle {
    fn drop(&mut self) {
        let mut state = self.text_system.wrapper_pool.lock();
        let wrapper = self.wrapper.take().unwrap();
        state
            .get_mut(&(wrapper.font.clone(), wrapper.font_size))
            .unwrap()
            .push(wrapper);
    }
}

impl Deref for LineWrapperHandle {
    type Target = LineWrapper;

    fn deref(&self) -> &Self::Target {
        self.wrapper.as_ref().unwrap()
    }
}

impl DerefMut for LineWrapperHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.wrapper.as_mut().unwrap()
    }
}

/// The degree of blackness or stroke thickness of a font. This value ranges from 100.0 to 900.0,
/// with 400.0 as normal.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct FontWeight(pub f32);

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
}

/// Allows italic or oblique faces to be selected.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum FontStyle {
    /// A face that is neither italic not obliqued.
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

impl Default for FontStyle {
    fn default() -> FontStyle {
        FontStyle::Normal
    }
}

impl Display for FontStyle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunStyle {
    pub font: Font,
    pub color: Hsla,
    pub underline: Option<UnderlineStyle>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GlyphId(u32);

impl From<GlyphId> for u32 {
    fn from(value: GlyphId) -> Self {
        value.0
    }
}

impl From<u16> for GlyphId {
    fn from(num: u16) -> Self {
        GlyphId(num as u32)
    }
}

impl From<u32> for GlyphId {
    fn from(num: u32) -> Self {
        GlyphId(num)
    }
}

#[derive(Clone, Debug)]
pub struct Glyph {
    pub id: GlyphId,
    pub position: Point<Pixels>,
    pub index: usize,
    pub is_emoji: bool,
}

#[derive(Default, Debug)]
pub struct LineLayout {
    pub font_size: Pixels,
    pub width: Pixels,
    pub ascent: Pixels,
    pub descent: Pixels,
    pub runs: Vec<Run>,
    pub len: usize,
}

#[derive(Debug)]
pub struct Run {
    pub font_id: FontId,
    pub glyphs: Vec<Glyph>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Font {
    pub family: SharedString,
    pub features: FontFeatures,
    pub weight: FontWeight,
    pub style: FontStyle,
}

pub fn font(family: impl Into<SharedString>) -> Font {
    Font {
        family: family.into(),
        features: FontFeatures::default(),
        weight: FontWeight::default(),
        style: FontStyle::default(),
    }
}

impl Font {
    pub fn bold(mut self) -> Self {
        self.weight = FontWeight::BOLD;
        self
    }
}

/// A struct for storing font metrics.
/// It is used to define the measurements of a typeface.
#[derive(Clone, Copy, Debug)]
pub struct FontMetrics {
    /// The number of font units that make up the "em square",
    /// a scalable grid for determining the size of a typeface.
    pub(crate) units_per_em: u32,

    /// The vertical distance from the baseline of the font to the top of the glyph covers.
    pub(crate) ascent: f32,

    /// The vertical distance from the baseline of the font to the bottom of the glyph covers.
    pub(crate) descent: f32,

    /// The recommended additional space to add between lines of type.
    pub(crate) line_gap: f32,

    /// The suggested position of the underline.
    pub(crate) underline_position: f32,

    /// The suggested thickness of the underline.
    pub(crate) underline_thickness: f32,

    /// The height of a capital letter measured from the baseline of the font.
    pub(crate) cap_height: f32,

    /// The height of a lowercase x.
    pub(crate) x_height: f32,

    /// The outer limits of the area that the font covers.
    pub(crate) bounding_box: Bounds<f32>,
}

impl FontMetrics {
    /// Returns the number of pixels that make up the "em square",
    /// a scalable grid for determining the size of a typeface.
    pub fn units_per_em(&self, font_size: Pixels) -> Pixels {
        Pixels((self.units_per_em as f32 / font_size.0).ceil())
    }

    /// Returns the vertical distance from the baseline of the font to the top of the glyph covers in pixels.
    pub fn ascent(&self, font_size: Pixels) -> Pixels {
        Pixels((self.ascent / font_size.0).ceil() as f32)
    }

    /// Returns the vertical distance from the baseline of the font to the bottom of the glyph covers in pixels.
    pub fn descent(&self, font_size: Pixels) -> Pixels {
        Pixels((self.descent / font_size.0).ceil() as f32)
    }

    /// Returns the recommended additional space to add between lines of type in pixels.
    pub fn line_gap(&self, font_size: Pixels) -> Pixels {
        Pixels((self.line_gap / font_size.0).ceil() as f32)
    }

    /// Returns the suggested position of the underline in pixels.
    pub fn underline_position(&self, font_size: Pixels) -> Pixels {
        Pixels((self.underline_position / font_size.0).ceil() as f32)
    }

    /// Returns the suggested thickness of the underline in pixels.
    pub fn underline_thickness(&self, font_size: Pixels) -> Pixels {
        Pixels((self.underline_thickness / font_size.0).ceil() as f32)
    }

    /// Returns the height of a capital letter measured from the baseline of the font in pixels.
    pub fn cap_height(&self, font_size: Pixels) -> Pixels {
        Pixels((self.cap_height / font_size.0).ceil() as f32)
    }

    /// Returns the height of a lowercase x in pixels.
    pub fn x_height(&self, font_size: Pixels) -> Pixels {
        Pixels((self.x_height / font_size.0).ceil() as f32)
    }

    /// Returns the outer limits of the area that the font covers in pixels.
    pub fn bounding_box(&self, font_size: Pixels) -> Bounds<Pixels> {
        (self.bounding_box / font_size.0).map(px)
    }
}
