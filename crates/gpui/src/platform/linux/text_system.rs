use crate::{
    Bounds, DevicePixels, Font, FontId, FontMetrics, FontRun, GlyphId, LineLayout, Pixels,
    PlatformTextSystem, RenderGlyphParams, SharedString, Size,
};
use anyhow::Result;
use collections::HashMap;
use font_kit::{
    font::Font as FontKitFont,
    handle::Handle,
    hinting::HintingOptions,
    metrics::Metrics,
    properties::{Style as FontkitStyle, Weight as FontkitWeight},
    source::SystemSource,
    sources::mem::MemSource,
};
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::borrow::Cow;

pub(crate) struct LinuxTextSystem(RwLock<LinuxTextSystemState>);

struct LinuxTextSystemState {
    memory_source: MemSource,
    system_source: SystemSource,
    fonts: Vec<FontKitFont>,
    font_selections: HashMap<Font, FontId>,
    font_ids_by_postscript_name: HashMap<String, FontId>,
    font_ids_by_family_name: HashMap<SharedString, SmallVec<[FontId; 4]>>,
    postscript_names_by_font_id: HashMap<FontId, String>,
}

unsafe impl Send for LinuxTextSystemState {}
unsafe impl Sync for LinuxTextSystemState {}

impl LinuxTextSystem {
    pub(crate) fn new() -> Self {
        Self(RwLock::new(LinuxTextSystemState {
            memory_source: MemSource::empty(),
            system_source: SystemSource::new(),
            fonts: Vec::new(),
            font_selections: HashMap::default(),
            font_ids_by_postscript_name: HashMap::default(),
            font_ids_by_family_name: HashMap::default(),
            postscript_names_by_font_id: HashMap::default(),
        }))
    }
}

impl Default for LinuxTextSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused)]
impl PlatformTextSystem for LinuxTextSystem {
    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        Ok(()) //TODO
    }
    fn all_font_names(&self) -> Vec<String> {
        Vec::new()
    }
    fn all_font_families(&self) -> Vec<String> {
        Vec::new()
    }
    fn font_id(&self, descriptor: &Font) -> Result<FontId> {
        Ok(FontId(0)) //TODO
    }
    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        unimplemented!()
    }
    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        unimplemented!()
    }
    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        unimplemented!()
    }
    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        None
    }
    fn glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        unimplemented!()
    }
    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        unimplemented!()
    }
    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout {
        LineLayout::default() //TODO
    }
    fn wrap_line(
        &self,
        text: &str,
        font_id: FontId,
        font_size: Pixels,
        width: Pixels,
    ) -> Vec<usize> {
        unimplemented!()
    }
}
