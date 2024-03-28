use crate::{
    Bounds, DevicePixels, Font, FontId, FontMetrics, FontRun, GlyphId, LineLayout, Pixels,
    PlatformTextSystem, RenderGlyphParams, Size,
};
use anyhow::Result;
use std::borrow::Cow;

pub(crate) struct TestTextSystem {}

// todo(linux)
#[allow(unused)]
impl PlatformTextSystem for TestTextSystem {
    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        unimplemented!()
    }
    fn all_font_names(&self) -> Vec<String> {
        unimplemented!()
    }
    fn all_font_families(&self) -> Vec<String> {
        unimplemented!()
    }
    fn font_id(&self, descriptor: &Font) -> Result<FontId> {
        unimplemented!()
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
        unimplemented!()
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
        unimplemented!()
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
