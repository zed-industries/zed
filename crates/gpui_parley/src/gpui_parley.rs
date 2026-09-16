//! A Parley-backed [`TextSystem`] for GPUI.
//!
//! This crate proves that an out-of-tree crate can implement GPUI's text SPI on
//! top of a different shaping and line-layout engine. It depends on `gpui_engine`
//! (the SPI surface) and `parley`, not on `gpui_engine_default`.
//!
//! The shaping and line layout go through Parley; rasterization is intentionally
//! left as a stub because the point of this crate is the layout boundary, not a
//! second glyph rasterizer.

#![warn(missing_docs)]

use std::ops::Range;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use gpui_engine::{
    Font, FontId, FontMetrics, FontRun, GlyphId, LineLayout, LineLayoutIndex, LineWrapper,
    LineWrapperHandle, RenderGlyphParams, ShapedGlyph, ShapedRun, TextRenderingMode,
    WrappedLineLayout, font,
};

pub use gpui_engine::{PlatformTextSystem, TextSystem};
use gpui_shared_string::SharedString;
use gpui_types::{Bounds, DevicePixels, Hsla, Pixels, Point, Size, px};
use parley::{
    Alignment, AlignmentOptions, FontContext, FontFamily, IndentOptions, LayoutContext,
    PositionedLayoutItem, StyleProperty, YieldData,
};
use smallvec::SmallVec;

/// The embedded fallback font used for shaping.
const FONT_DATA: &[u8] =
    include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf");
/// The family name of the embedded font.
pub const FONT_FAMILY: &str = "IBM Plex Sans";

/// Builds a [`parley::FontContext`] pre-loaded with the embedded font.
///
/// This is exposed so examples and consumers can use Parley's advanced layout
/// features directly, outside the shared [`TextSystem`] shaping boundary.
pub fn font_context() -> parley::FontContext {
    let mut collection = parley::fontique::Collection::new(parley::fontique::CollectionOptions {
        shared: false,
        system_fonts: false,
    });
    collection.register_fonts(
        parley::fontique::Blob::new(Arc::new(FONT_DATA.to_vec())),
        None,
    );
    parley::FontContext {
        collection,
        source_cache: parley::fontique::SourceCache::default(),
    }
}

/// A per-line layout box used by [`ParleyTextSystem::layout_with_boxes`].
#[derive(Clone, Copy, Debug)]
pub struct LineBox {
    /// The x offset of the line's start edge.
    pub x: f32,
    /// The maximum advance (width) of the line.
    pub width: f32,
}

/// A [`TextSystem`] that shapes and lays out text through Parley.
pub struct ParleyTextSystem {
    platform: Arc<ParleyPlatformTextSystem>,
    platform_dyn: Arc<dyn PlatformTextSystem>,
    font: Font,
    font_id: FontId,
    font_runs_pool: Mutex<Vec<Vec<FontRun>>>,
    wrapper_pool: Mutex<Vec<LineWrapper>>,
}

impl ParleyTextSystem {
    /// Creates a text system that shapes with the embedded IBM Plex Sans font.
    pub fn new() -> Arc<Self> {
        let font = font(FONT_FAMILY);
        let font_id = FontId(0);
        let platform = Arc::new(ParleyPlatformTextSystem::new(font_id));
        let platform_dyn: Arc<dyn PlatformTextSystem> = platform.clone();
        Arc::new(Self {
            platform,
            platform_dyn,
            font,
            font_id,
            font_runs_pool: Mutex::new(Vec::new()),
            wrapper_pool: Mutex::new(Vec::new()),
        })
    }

    /// Lays out `text` with a first-line indent, using Parley's CSS
    /// `text-indent` support.
    pub fn layout_indented(
        &self,
        text: &str,
        size: f32,
        indent: f32,
        max_width: f32,
    ) -> parley::Layout<[u8; 4]> {
        let mut font_context = self.platform.font_context.lock().unwrap();
        let mut layout_context = self.platform.layout_context.lock().unwrap();
        let mut builder = layout_context.ranged_builder(&mut font_context, text, 1.0, true);
        builder.push_default(StyleProperty::FontFamily(FontFamily::from(FONT_FAMILY)));
        builder.push_default(StyleProperty::FontSize(size));
        let mut layout = builder.build(text);
        layout.set_text_indent(indent, IndentOptions::default());
        layout.break_all_lines(Some(max_width));
        layout.align(Alignment::Start, AlignmentOptions::default());
        layout
    }

    /// Lays out `text` with a character-count limit per line, using Parley's
    /// `break_next_with_length`.
    pub fn layout_with_char_count(
        &self,
        text: &str,
        size: f32,
        max_chars: u32,
    ) -> parley::Layout<[u8; 4]> {
        let mut font_context = self.platform.font_context.lock().unwrap();
        let mut layout_context = self.platform.layout_context.lock().unwrap();
        let mut builder = layout_context.ranged_builder(&mut font_context, text, 1.0, true);
        builder.push_default(StyleProperty::FontFamily(FontFamily::from(FONT_FAMILY)));
        builder.push_default(StyleProperty::FontSize(size));
        let mut layout = builder.build(text);
        {
            let mut breaker = layout.break_lines();
            while breaker.break_next_with_length(max_chars).is_some() {}
        }
        layout.align(Alignment::Start, AlignmentOptions::default());
        layout
    }

    /// Lays out `text` into per-line boxes (e.g. flowing around excluded
    /// regions). Each box constrains the x offset and maximum advance of one
    /// line; lines past the end of `boxes` use the full width.
    pub fn layout_with_boxes(
        &self,
        text: &str,
        size: f32,
        boxes: &[LineBox],
    ) -> parley::Layout<[u8; 4]> {
        let mut font_context = self.platform.font_context.lock().unwrap();
        let mut layout_context = self.platform.layout_context.lock().unwrap();
        let mut builder = layout_context.ranged_builder(&mut font_context, text, 1.0, true);
        builder.push_default(StyleProperty::FontFamily(FontFamily::from(FONT_FAMILY)));
        builder.push_default(StyleProperty::FontSize(size));
        let mut layout = builder.build(text);
        {
            let mut breaker = layout.break_lines();
            breaker.state_mut().set_layout_max_advance(f32::MAX);
            let mut line = 0;
            loop {
                let box_ = boxes.get(line);
                breaker
                    .state_mut()
                    .set_line_max_advance(box_.map_or(f32::MAX, |b| b.width));
                breaker.state_mut().set_line_x(box_.map_or(0.0, |b| b.x));
                match breaker.break_next() {
                    Some(YieldData::LineBreak(_)) => line += 1,
                    Some(_) => {}
                    None => break,
                }
            }
        }
        layout.align(Alignment::Start, AlignmentOptions::default());
        layout
    }
}

impl TextSystem for ParleyTextSystem {
    fn platform_text_system(&self) -> &Arc<dyn PlatformTextSystem> {
        &self.platform_dyn
    }

    fn all_font_names(&self) -> Vec<String> {
        vec![FONT_FAMILY.to_string()]
    }

    fn add_fonts(&self, _fonts: Vec<std::borrow::Cow<'static, [u8]>>) -> Result<()> {
        Ok(())
    }

    fn get_font_for_id(&self, id: FontId) -> Option<Font> {
        (id == self.font_id).then(|| self.font.clone())
    }

    fn resolve_font(&self, _font: &Font) -> FontId {
        self.font_id
    }

    fn prewarm_fonts(&self, _fonts: &[Font]) {}

    fn bounding_box(&self, font_id: FontId, font_size: Pixels) -> Bounds<Pixels> {
        self.platform.font_metrics(font_id).bounding_box(font_size)
    }

    fn typographic_bounds(
        &self,
        font_id: FontId,
        font_size: Pixels,
        character: char,
    ) -> Result<Bounds<Pixels>> {
        let glyph_id = self
            .platform
            .glyph_for_char(font_id, character)
            .unwrap_or(GlyphId(0));
        let bounds = self.platform.typographic_bounds(font_id, glyph_id)?;
        let scale = font_size.0 / self.platform.font_metrics(font_id).units_per_em as f32;
        Ok((bounds * scale).map(px))
    }

    fn advance(&self, font_id: FontId, font_size: Pixels, ch: char) -> Result<Size<Pixels>> {
        let glyph_id = self
            .platform
            .glyph_for_char(font_id, ch)
            .unwrap_or(GlyphId(0));
        let units = self.platform.advance(font_id, glyph_id)?;
        let scale = font_size.0 / self.platform.font_metrics(font_id).units_per_em as f32;
        Ok(Size {
            width: px(units.width * scale),
            height: px(units.height * scale),
        })
    }

    fn layout_width(&self, font_id: FontId, font_size: Pixels, ch: char) -> Pixels {
        self.advance(font_id, font_size, ch)
            .map(|size| size.width)
            .unwrap_or(font_size)
    }

    fn em_width(&self, font_id: FontId, font_size: Pixels) -> Result<Pixels> {
        Ok(self.layout_width(font_id, font_size, 'm'))
    }

    fn em_advance(&self, font_id: FontId, font_size: Pixels) -> Result<Pixels> {
        self.advance(font_id, font_size, 'm').map(|size| size.width)
    }

    fn ch_width(&self, font_id: FontId, font_size: Pixels) -> Result<Pixels> {
        Ok(self.layout_width(font_id, font_size, '0'))
    }

    fn ch_advance(&self, font_id: FontId, font_size: Pixels) -> Result<Pixels> {
        self.advance(font_id, font_size, '0').map(|size| size.width)
    }

    fn units_per_em(&self, font_id: FontId) -> u32 {
        self.platform.font_metrics(font_id).units_per_em
    }

    fn cap_height(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.platform.font_metrics(font_id).cap_height(font_size)
    }

    fn x_height(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.platform.font_metrics(font_id).x_height(font_size)
    }

    fn ascent(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.platform.font_metrics(font_id).ascent(font_size)
    }

    fn descent(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.platform.font_metrics(font_id).descent(font_size)
    }

    fn baseline_offset(&self, font_id: FontId, font_size: Pixels, line_height: Pixels) -> Pixels {
        let ascent = self.ascent(font_id, font_size);
        let descent = self.descent(font_id, font_size);
        (line_height - (ascent - descent)) / 2.0 + ascent
    }

    fn take_font_runs(&self) -> Vec<FontRun> {
        self.font_runs_pool
            .lock()
            .unwrap()
            .pop()
            .unwrap_or_default()
    }

    fn recycle_font_runs(&self, font_runs: Vec<FontRun>) {
        self.font_runs_pool.lock().unwrap().push(font_runs);
    }

    fn line_wrapper(self: Arc<Self>, font: Font, font_size: Pixels) -> LineWrapperHandle {
        let font_id = self.resolve_font(&font);
        let wrapper = self
            .wrapper_pool
            .lock()
            .unwrap()
            .pop()
            .unwrap_or_else(|| LineWrapper::new(font_id, font_size, self.clone()));
        let this = self.clone();
        LineWrapperHandle::new(wrapper, move |wrapper| {
            this.wrapper_pool.lock().unwrap().push(wrapper);
        })
    }

    fn raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        self.platform.glyph_raster_bounds(params)
    }

    fn rasterize_glyph(&self, params: &RenderGlyphParams) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        let bounds = self.platform.glyph_raster_bounds(params)?;
        self.platform.rasterize_glyph(params, bounds)
    }

    fn glyph_dilation_for_color(&self, _color: Hsla) -> u8 {
        0
    }

    fn recommended_rendering_mode(
        &self,
        _font_id: FontId,
        _font_size: Pixels,
    ) -> TextRenderingMode {
        TextRenderingMode::PlatformDefault
    }

    fn layout_index(&self) -> LineLayoutIndex {
        LineLayoutIndex {
            lines_index: 0,
            wrapped_lines_index: 0,
            lines_by_hash_index: 0,
            wrapped_lines_by_hash_index: 0,
        }
    }

    fn reuse_layouts(&self, _range: Range<LineLayoutIndex>) {}

    fn truncate_layouts(&self, _index: LineLayoutIndex) {}

    fn finish_frame(&self) {}

    fn layout_wrapped_line(
        &self,
        text: &str,
        font_size: Pixels,
        runs: &[FontRun],
        wrap_width: Option<Pixels>,
        _max_lines: Option<usize>,
    ) -> Arc<WrappedLineLayout> {
        let unwrapped_layout = self.layout_line(text, font_size, runs, None);
        Arc::new(WrappedLineLayout {
            unwrapped_layout,
            wrap_boundaries: SmallVec::new(),
            wrap_width,
        })
    }

    fn layout_line(
        &self,
        text: &str,
        font_size: Pixels,
        runs: &[FontRun],
        _force_width: Option<Pixels>,
    ) -> Arc<LineLayout> {
        Arc::new(self.platform.layout_line(text, font_size, runs))
    }

    fn try_layout_line_by_hash(
        &self,
        _text_hash: u64,
        _text_len: usize,
        _font_size: Pixels,
        _runs: &[FontRun],
        _force_width: Option<Pixels>,
    ) -> Option<Arc<LineLayout>> {
        None
    }

    fn layout_line_by_hash(
        &self,
        _text_hash: u64,
        _text_len: usize,
        font_size: Pixels,
        runs: &[FontRun],
        force_width: Option<Pixels>,
        materialize_text: Box<dyn FnOnce() -> SharedString>,
    ) -> Arc<LineLayout> {
        let text = materialize_text();
        self.layout_line(&text, font_size, runs, force_width)
    }
}

/// The Parley shaping backend behind [`ParleyTextSystem`].
struct ParleyPlatformTextSystem {
    font_context: Mutex<FontContext>,
    layout_context: Mutex<LayoutContext>,
    font_id: FontId,
}

impl ParleyPlatformTextSystem {
    fn new(font_id: FontId) -> Self {
        Self {
            font_context: Mutex::new(font_context()),
            layout_context: Mutex::new(LayoutContext::new()),
            font_id,
        }
    }
}

impl PlatformTextSystem for ParleyPlatformTextSystem {
    fn add_fonts(&self, _fonts: Vec<std::borrow::Cow<'static, [u8]>>) -> Result<()> {
        Ok(())
    }

    fn all_font_names(&self) -> Vec<String> {
        vec![FONT_FAMILY.to_string()]
    }

    fn font_id(&self, _descriptor: &Font) -> Result<FontId> {
        Ok(self.font_id)
    }

    fn font_metrics(&self, _font_id: FontId) -> FontMetrics {
        FontMetrics {
            units_per_em: 1000,
            ascent: 1025.0,
            descent: -275.0,
            line_gap: 0.0,
            underline_position: -95.0,
            underline_thickness: 60.0,
            cap_height: 698.0,
            x_height: 516.0,
            bounding_box: Bounds {
                origin: Point {
                    x: -260.0,
                    y: -245.0,
                },
                size: Size {
                    width: 1501.0,
                    height: 1364.0,
                },
            },
        }
    }

    fn typographic_bounds(&self, _font_id: FontId, _glyph_id: GlyphId) -> Result<Bounds<f32>> {
        Ok(Bounds {
            origin: Point { x: 54.0, y: 0.0 },
            size: Size {
                width: 392.0,
                height: 528.0,
            },
        })
    }

    fn advance(&self, _font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        Ok(Size {
            width: 600.0 * glyph_id.0 as f32,
            height: 0.0,
        })
    }

    fn glyph_for_char(&self, _font_id: FontId, ch: char) -> Option<GlyphId> {
        Some(GlyphId(ch.len_utf16() as u32))
    }

    fn glyph_raster_bounds(&self, _params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        Ok(Default::default())
    }

    fn rasterize_glyph(
        &self,
        _params: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        Ok((raster_bounds.size, Vec::new()))
    }

    fn layout_line(&self, text: &str, font_size: Pixels, _runs: &[FontRun]) -> LineLayout {
        let mut font_context = self.font_context.lock().unwrap();
        let mut layout_context = self.layout_context.lock().unwrap();

        let mut builder = layout_context.ranged_builder(&mut font_context, text, 1.0, true);
        builder.push_default(StyleProperty::FontFamily(FontFamily::from(FONT_FAMILY)));
        builder.push_default(StyleProperty::FontSize(font_size.0));

        let mut layout = builder.build(text);
        layout.break_all_lines(None);
        layout.align(Alignment::Start, AlignmentOptions::default());

        let mut result = LineLayout {
            font_size,
            width: px(layout.width()),
            ascent: px(0.0),
            descent: px(0.0),
            runs: Vec::new(),
            len: text.len(),
        };

        for line in layout.lines() {
            let metrics = line.metrics();
            result.ascent = px(metrics.ascent);
            result.descent = px(metrics.descent);

            for item in line.items() {
                if let PositionedLayoutItem::GlyphRun(glyph_run) = item {
                    let mut glyphs = Vec::new();
                    let mut offset = glyph_run.offset();
                    let baseline = glyph_run.baseline();
                    for cluster in glyph_run.run().visual_clusters() {
                        let byte_index = cluster.text_range().start;
                        let is_emoji = cluster.is_emoji();
                        for mut glyph in cluster.glyphs() {
                            glyph.x += offset;
                            glyph.y += baseline;
                            offset += glyph.advance;
                            glyphs.push(ShapedGlyph {
                                id: GlyphId(glyph.id),
                                position: Point {
                                    x: px(glyph.x),
                                    y: px(glyph.y),
                                },
                                index: byte_index,
                                is_emoji,
                            });
                        }
                    }
                    result.runs.push(ShapedRun {
                        font_id: self.font_id,
                        glyphs,
                    });
                }
            }
        }

        result
    }

    fn recommended_rendering_mode(
        &self,
        _font_id: FontId,
        _font_size: Pixels,
    ) -> TextRenderingMode {
        TextRenderingMode::PlatformDefault
    }
}

#[cfg(test)]
mod tests {
    use crate::{ParleyTextSystem, TextSystem};
    use gpui_types::px;

    #[test]
    fn shapes_a_line_through_parley() {
        let text_system = ParleyTextSystem::new();
        let layout = text_system.layout_line("hello", px(16.0), &[], None);

        assert_eq!(layout.len, 5);
        assert!(layout.width > px(0.0));
        assert!(!layout.runs.is_empty());
        assert!(!layout.runs[0].glyphs.is_empty());
    }

    #[test]
    fn parley_width_grows_with_font_size() {
        let text_system = ParleyTextSystem::new();
        let small = text_system.layout_line("hello", px(12.0), &[], None);
        let large = text_system.layout_line("hello", px(24.0), &[], None);

        assert!(large.width > small.width);
    }

    #[test]
    fn parley_native_layout_is_reachable_through_downcast() {
        let text_system = ParleyTextSystem::new();
        let text_system: &dyn TextSystem = &*text_system;
        let parley = text_system
            .as_any()
            .downcast_ref::<ParleyTextSystem>()
            .expect("should downcast to the concrete ParleyTextSystem");

        let layout =
            parley.layout_indented("hello world this is a longer string", 16.0, 32.0, 120.0);
        assert!(layout.lines().len() > 1);
    }
}
