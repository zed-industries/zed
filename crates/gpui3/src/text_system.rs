mod font_cache;
mod line_wrapper;
mod text_layout_cache;

pub use font_cache::*;
use line_wrapper::*;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
pub use text_layout_cache::*;

use crate::{Hsla, Pixels, PlatformTextSystem, Point, Result, Size, UnderlineStyle};
use collections::HashMap;
use core::fmt;
use parking_lot::Mutex;
use std::{
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    sync::Arc,
};

pub struct TextSystem {
    font_cache: Arc<FontCache>,
    text_layout_cache: Arc<TextLayoutCache>,
    platform_text_system: Arc<dyn PlatformTextSystem>,
    wrapper_pool: Mutex<HashMap<(FontId, Pixels), Vec<LineWrapper>>>,
}

impl TextSystem {
    pub fn new(platform_text_system: Arc<dyn PlatformTextSystem>) -> Self {
        TextSystem {
            font_cache: Arc::new(FontCache::new(platform_text_system.clone())),
            text_layout_cache: Arc::new(TextLayoutCache::new(platform_text_system.clone())),
            platform_text_system,
            wrapper_pool: Mutex::new(HashMap::default()),
        }
    }

    pub fn font_family_name(&self, family_id: FontFamilyId) -> Result<Arc<str>> {
        self.font_cache.family_name(family_id)
    }

    pub fn load_font_family(
        &self,
        names: &[&str],
        features: &FontFeatures,
    ) -> Result<FontFamilyId> {
        self.font_cache.load_family(names, features)
    }

    /// Returns an arbitrary font family that is available on the system.
    pub fn known_existing_font_family(&self) -> FontFamilyId {
        self.font_cache.known_existing_family()
    }

    pub fn default_font(&self, family_id: FontFamilyId) -> FontId {
        self.font_cache.default_font(family_id)
    }

    pub fn select_font(
        &self,
        family_id: FontFamilyId,
        weight: FontWeight,
        style: FontStyle,
    ) -> Result<FontId> {
        self.font_cache.select_font(family_id, weight, style)
    }

    pub fn read_font_metric<F, T>(&self, font_id: FontId, f: F) -> T
    where
        F: FnOnce(&FontMetrics) -> T,
        T: 'static,
    {
        self.font_cache.read_metric(font_id, f)
    }

    pub fn bounding_box(&self, font_id: FontId, font_size: Pixels) -> Size<Pixels> {
        self.font_cache.bounding_box(font_id, font_size)
    }

    pub fn em_width(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.font_cache.em_width(font_id, font_size)
    }

    pub fn em_advance(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.font_cache.em_advance(font_id, font_size)
    }

    pub fn line_height(&self, font_size: Pixels) -> Pixels {
        self.font_cache.line_height(font_size)
    }

    pub fn cap_height(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.font_cache.cap_height(font_id, font_size)
    }

    pub fn x_height(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.font_cache.x_height(font_id, font_size)
    }

    pub fn ascent(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.font_cache.ascent(font_id, font_size)
    }

    pub fn descent(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.font_cache.descent(font_id, font_size)
    }

    pub fn em_size(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.font_cache.em_size(font_id, font_size)
    }

    pub fn baseline_offset(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.font_cache.baseline_offset(font_id, font_size)
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

    pub fn line_wrapper(self: &Arc<Self>, font_id: FontId, font_size: Pixels) -> LineWrapperHandle {
        let lock = &mut self.wrapper_pool.lock();
        let wrappers = lock.entry((font_id, font_size)).or_default();
        let wrapper = wrappers.pop().unwrap_or_else(|| {
            LineWrapper::new(font_id, font_size, self.platform_text_system.clone())
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
            .get_mut(&(wrapper.font_id, wrapper.font_size))
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

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FontFeatures {
    pub calt: Option<bool>,
    pub case: Option<bool>,
    pub cpsp: Option<bool>,
    pub frac: Option<bool>,
    pub liga: Option<bool>,
    pub onum: Option<bool>,
    pub ordn: Option<bool>,
    pub pnum: Option<bool>,
    pub ss01: Option<bool>,
    pub ss02: Option<bool>,
    pub ss03: Option<bool>,
    pub ss04: Option<bool>,
    pub ss05: Option<bool>,
    pub ss06: Option<bool>,
    pub ss07: Option<bool>,
    pub ss08: Option<bool>,
    pub ss09: Option<bool>,
    pub ss10: Option<bool>,
    pub ss11: Option<bool>,
    pub ss12: Option<bool>,
    pub ss13: Option<bool>,
    pub ss14: Option<bool>,
    pub ss15: Option<bool>,
    pub ss16: Option<bool>,
    pub ss17: Option<bool>,
    pub ss18: Option<bool>,
    pub ss19: Option<bool>,
    pub ss20: Option<bool>,
    pub subs: Option<bool>,
    pub sups: Option<bool>,
    pub swsh: Option<bool>,
    pub titl: Option<bool>,
    pub tnum: Option<bool>,
    pub zero: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunStyle {
    pub color: Hsla,
    pub font_id: FontId,
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
