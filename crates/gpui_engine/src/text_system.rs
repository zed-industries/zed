//! The text-shaping, wrapping, and line-layout surface the authoring layer drives.
//!
//! [`PlatformTextSystem`] is the lower-level shaping SPI; [`TextSystem`] adds the
//! engine's font-resolution, metric, wrapping, and per-frame line-layout caching
//! on top of it. `gpui_engine_default` ships the standard implementation.

use anyhow::Result;
use gpui_shared_string::SharedString;
use gpui_types::{Bounds, DevicePixels, Hsla, Pixels, Size};
use std::borrow::Cow;
use std::ops::Range;
use std::sync::Arc;

use crate::{
    Font, FontId, FontRun, LineLayout, LineLayoutIndex, PlatformTextSystem, RenderGlyphParams,
    TextRenderingMode, WrappedLineLayout,
};

/// The text shaping, metric, wrapping, and line-layout surface.
///
/// Implementations wrap a [`PlatformTextSystem`] with the engine's font-id,
/// metric, raster-bounds, and line-wrapper pools, and with the per-frame
/// line-layout cache the authoring layer reuses.
pub trait TextSystem: Send + Sync {
    /// The platform text system this engine wraps.
    fn platform_text_system(&self) -> &Arc<dyn PlatformTextSystem>;

    /// All font names known to the text system, including fallbacks.
    fn all_font_names(&self) -> Vec<String>;

    /// Add font data to the text system.
    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()>;

    /// The font for a font id, if it was resolved through this system.
    fn get_font_for_id(&self, id: FontId) -> Option<Font>;

    /// Resolve a font to its id, falling back to the default stack.
    fn resolve_font(&self, font: &Font) -> FontId;

    /// Prewarm any system font caches needed to shape text.
    fn prewarm_fonts(&self, fonts: &[Font]);

    /// The bounding box for a font, at a size.
    fn bounding_box(&self, font_id: FontId, font_size: Pixels) -> Bounds<Pixels>;

    /// The typographic bounds for a character, in a font and size.
    fn typographic_bounds(
        &self,
        font_id: FontId,
        font_size: Pixels,
        character: char,
    ) -> Result<Bounds<Pixels>>;

    /// The advance width for a character, in a font and size.
    fn advance(&self, font_id: FontId, font_size: Pixels, ch: char) -> Result<Size<Pixels>>;

    /// The shaped layout width of a character, in a font and size.
    fn layout_width(&self, font_id: FontId, font_size: Pixels, ch: char) -> Pixels;

    /// The width of an em, in a font and size.
    fn em_width(&self, font_id: FontId, font_size: Pixels) -> Result<Pixels>;

    /// The advance width of an em, in a font and size.
    fn em_advance(&self, font_id: FontId, font_size: Pixels) -> Result<Pixels>;

    /// The width of the `0` character, in a font and size.
    fn ch_width(&self, font_id: FontId, font_size: Pixels) -> Result<Pixels>;

    /// The advance width of the `0` character, in a font and size.
    fn ch_advance(&self, font_id: FontId, font_size: Pixels) -> Result<Pixels>;

    /// The number of font units per em square.
    fn units_per_em(&self, font_id: FontId) -> u32;

    /// The cap height of a font, at a size.
    fn cap_height(&self, font_id: FontId, font_size: Pixels) -> Pixels;

    /// The x-height of a font, at a size.
    fn x_height(&self, font_id: FontId, font_size: Pixels) -> Pixels;

    /// The ascent of a font, at a size.
    fn ascent(&self, font_id: FontId, font_size: Pixels) -> Pixels;

    /// The descent of a font, at a size.
    fn descent(&self, font_id: FontId, font_size: Pixels) -> Pixels;

    /// The recommended baseline offset for a font, size, and line height.
    fn baseline_offset(&self, font_id: FontId, font_size: Pixels, line_height: Pixels) -> Pixels;

    /// Take a pooled font-run buffer, or an empty one when the pool is dry.
    fn take_font_runs(&self) -> Vec<FontRun>;

    /// Return a font-run buffer to the pool for reuse.
    fn recycle_font_runs(&self, font_runs: Vec<FontRun>);

    /// The rasterized size and location of a glyph.
    fn raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>>;

    /// Rasterize a glyph, returning its size and coverage bitmap.
    fn rasterize_glyph(&self, params: &RenderGlyphParams) -> Result<(Size<DevicePixels>, Vec<u8>)>;

    /// The dilation level to use for a glyph painted in the given color.
    fn glyph_dilation_for_color(&self, color: Hsla) -> u8;

    /// The rendering mode recommended for a font and size.
    fn recommended_rendering_mode(&self, font_id: FontId, font_size: Pixels) -> TextRenderingMode;

    /// Saves the current cache position so it can be reused or truncated later.
    fn layout_index(&self) -> LineLayoutIndex;

    /// Re-inserts layouts from the previous frame created before `range`.
    fn reuse_layouts(&self, range: Range<LineLayoutIndex>);

    /// Drops layouts created after `index`.
    fn truncate_layouts(&self, index: LineLayoutIndex);

    /// Ages the current frame's layouts into the previous frame.
    fn finish_frame(&self);

    /// Shapes and wraps `text`, reusing a cached layout when the inputs match.
    fn layout_wrapped_line(
        &self,
        text: &str,
        font_size: Pixels,
        runs: &[FontRun],
        wrap_width: Option<Pixels>,
        max_lines: Option<usize>,
    ) -> Arc<WrappedLineLayout>;

    /// Shapes a single line of `text`, reusing a cached layout when the inputs match.
    fn layout_line(
        &self,
        text: &str,
        font_size: Pixels,
        runs: &[FontRun],
        force_width: Option<Pixels>,
    ) -> Arc<LineLayout>;

    /// Probe the line-layout cache using a caller-provided content hash.
    fn try_layout_line_by_hash(
        &self,
        text_hash: u64,
        text_len: usize,
        font_size: Pixels,
        runs: &[FontRun],
        force_width: Option<Pixels>,
    ) -> Option<Arc<LineLayout>>;

    /// Layout a line using a caller-provided content hash as the cache key.
    fn layout_line_by_hash(
        &self,
        text_hash: u64,
        text_len: usize,
        font_size: Pixels,
        runs: &[FontRun],
        force_width: Option<Pixels>,
        materialize_text: Box<dyn FnOnce() -> SharedString>,
    ) -> Arc<LineLayout>;
}
