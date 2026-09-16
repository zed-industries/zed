//! A Parley-backed [`TextSystem`] for GPUI.
//!
//! This crate proves that an out-of-tree crate can implement GPUI's text SPI on
//! top of a different shaping and line-layout engine. It depends on `gpui_engine`
//! (the SPI surface) and `parley`, not on `gpui_engine_default`.
//!
//! The shaping and line layout go through Parley; glyph rasterization uses
//! `skrifa` for outline extraction and hinting and `tiny-skia` for coverage
//! rasterization.

#![warn(missing_docs)]

use std::collections::HashMap;
use std::ops::Range;
use std::sync::{Arc, Mutex};

use anyhow::{Context as _, Result};
use gpui_engine::{
    Font, FontId, FontMetrics, FontRun, GlyphId, LineLayout, LineLayoutIndex, LineWrapper,
    LineWrapperHandle, RenderGlyphParams, ShapedGlyph, ShapedRun, TextRenderingMode, WrapBoundary,
    WrappedLineLayout, font,
};

pub use gpui_engine::{PlatformTextSystem, TextSystem};
use gpui_shared_string::SharedString;
use gpui_types::{Bounds, DevicePixels, Hsla, Pixels, Point, Size, px};
use parley::{
    Alignment, AlignmentOptions, FontContext, FontFamily, FontStyle, FontWeight, IndentOptions,
    LayoutContext, PositionedLayoutItem, StyleProperty, YieldData,
};
use skrifa::{
    FontRef, GlyphId as SkrifaGlyphId, MetadataProvider,
    instance::{LocationRef, Size as SkrifaSize},
    outline::{DrawSettings, HintingInstance, HintingOptions, OutlinePen},
    raw::TableProvider,
};
use smallvec::SmallVec;
use tiny_skia::{FillRule, Mask, PathBuilder, Transform};

/// The embedded regular font.
const FONT_DATA: &[u8] =
    include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf");
/// The embedded italic font.
const FONT_DATA_ITALIC: &[u8] =
    include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Italic.ttf");
/// The embedded semibold font.
const FONT_DATA_SEMIBOLD: &[u8] =
    include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-SemiBold.ttf");
/// The embedded semibold italic font.
const FONT_DATA_SEMIBOLD_ITALIC: &[u8] =
    include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-SemiBoldItalic.ttf");
/// The family name shared by the embedded fonts.
pub const FONT_FAMILY: &str = "IBM Plex Sans";

/// Builds a [`parley::FontContext`] pre-loaded with the embedded fonts.
///
/// This is exposed so examples and consumers can use Parley's advanced layout
/// features directly, outside the shared [`TextSystem`] shaping boundary.
pub fn font_context() -> parley::FontContext {
    let mut collection = parley::fontique::Collection::new(parley::fontique::CollectionOptions {
        shared: false,
        system_fonts: false,
    });
    for data in [
        FONT_DATA,
        FONT_DATA_ITALIC,
        FONT_DATA_SEMIBOLD,
        FONT_DATA_SEMIBOLD_ITALIC,
    ] {
        collection.register_fonts(parley::fontique::Blob::new(Arc::new(data.to_vec())), None);
    }
    parley::FontContext {
        collection,
        source_cache: parley::fontique::SourceCache::default(),
    }
}

/// Maps GPUI's font weight onto Parley's.
fn map_weight(weight: gpui_engine::FontWeight) -> FontWeight {
    FontWeight::new(weight.0)
}

/// Maps GPUI's font style onto Parley's.
fn map_style(style: gpui_engine::FontStyle) -> FontStyle {
    match style {
        gpui_engine::FontStyle::Normal => FontStyle::Normal,
        gpui_engine::FontStyle::Italic | gpui_engine::FontStyle::Oblique => FontStyle::Italic,
    }
}

/// Returns the embedded font bytes and index for a GPUI font.
fn font_data_for(font: &Font) -> (&'static [u8], usize) {
    let semi_bold = font.weight.0 >= 550.0;
    let italic = matches!(
        font.style,
        gpui_engine::FontStyle::Italic | gpui_engine::FontStyle::Oblique
    );
    match (semi_bold, italic) {
        (false, false) => (FONT_DATA, 0),
        (false, true) => (FONT_DATA_ITALIC, 0),
        (true, false) => (FONT_DATA_SEMIBOLD, 0),
        (true, true) => (FONT_DATA_SEMIBOLD_ITALIC, 0),
    }
}

/// Returns the `font_id` of the run that covers `byte`, if any.
fn font_id_for_byte(runs: &[FontRun], byte: usize) -> Option<FontId> {
    let mut offset = 0;
    for run in runs {
        if byte < offset + run.len {
            return Some(run.font_id);
        }
        offset += run.len;
    }
    runs.last().map(|run| run.font_id)
}

/// Converts a Parley layout into a GPUI [`LineLayout`].
fn convert_layout(
    layout: &parley::Layout<[u8; 4]>,
    runs: &[FontRun],
    default_font_id: FontId,
    font_size: Pixels,
    len: usize,
) -> LineLayout {
    let mut result = LineLayout {
        font_size,
        width: px(layout.width()),
        ascent: px(0.0),
        descent: px(0.0),
        runs: Vec::new(),
        len,
    };

    for line in layout.lines() {
        let metrics = line.metrics();
        result.ascent = px(metrics.ascent);
        result.descent = px(metrics.descent);

        for item in line.items() {
            if let PositionedLayoutItem::GlyphRun(glyph_run) = item {
                let run_byte = glyph_run.run().text_range().start;
                let font_id = font_id_for_byte(runs, run_byte).unwrap_or(default_font_id);
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
                result.runs.push(ShapedRun { font_id, glyphs });
            }
        }
    }

    result
}

/// Returns the wrap boundary whose glyph carries the given byte index.
fn wrap_boundary_for_byte(unwrapped: &LineLayout, byte: usize) -> Option<WrapBoundary> {
    for (run_ix, run) in unwrapped.runs.iter().enumerate() {
        for (glyph_ix, glyph) in run.glyphs.iter().enumerate() {
            if glyph.index == byte {
                return Some(WrapBoundary { run_ix, glyph_ix });
            }
        }
    }
    None
}

/// Collects a glyph outline into a [`tiny_skia::Path`], flipping the y-axis so
/// the font's y-up outline coordinates become the y-down coordinates
/// `tiny_skia` rasterizes in.
struct GlyphPathBuilder {
    builder: PathBuilder,
}

impl GlyphPathBuilder {
    fn new() -> Self {
        Self {
            builder: PathBuilder::new(),
        }
    }

    fn build(self) -> Option<tiny_skia::Path> {
        self.builder.finish()
    }
}

impl OutlinePen for GlyphPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.move_to(x, -y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(x, -y);
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.builder.quad_to(cx0, -cy0, x, -y);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.builder.cubic_to(cx0, -cy0, cx1, -cy1, x, -y);
    }

    fn close(&mut self) {
        self.builder.close();
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
    font_runs_pool: Mutex<Vec<Vec<FontRun>>>,
    wrapper_pool: Mutex<Vec<LineWrapper>>,
}

impl ParleyTextSystem {
    /// Creates a text system that shapes with the embedded IBM Plex Sans fonts.
    pub fn new() -> Arc<Self> {
        let platform = Arc::new(ParleyPlatformTextSystem::new());
        let platform_dyn: Arc<dyn PlatformTextSystem> = platform.clone();
        Arc::new(Self {
            platform,
            platform_dyn,
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
        self.platform.font_for_id(id)
    }

    fn resolve_font(&self, font: &Font) -> FontId {
        self.platform.resolve_font(font)
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
        let width = wrap_width.unwrap_or(Pixels::MAX);
        let (unwrapped_layout, wrap_boundaries) =
            self.platform.layout_wrapped(text, font_size, runs, width);
        Arc::new(WrappedLineLayout {
            unwrapped_layout: Arc::new(unwrapped_layout),
            wrap_boundaries,
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
    font_registry: Mutex<FontRegistry>,
}

/// Maps GPUI [`Font`]s to stable [`FontId`]s and back.
#[derive(Default)]
struct FontRegistry {
    ids_by_font: HashMap<Font, FontId>,
    fonts_by_id: HashMap<FontId, Font>,
    next_id: usize,
}

impl FontRegistry {
    fn resolve(&mut self, font: &Font) -> FontId {
        if let Some(id) = self.ids_by_font.get(font) {
            return *id;
        }
        let id = FontId(self.next_id);
        self.next_id += 1;
        self.ids_by_font.insert(font.clone(), id);
        self.fonts_by_id.insert(id, font.clone());
        id
    }

    fn font_for_id(&self, id: FontId) -> Option<Font> {
        self.fonts_by_id.get(&id).cloned()
    }
}

impl ParleyPlatformTextSystem {
    fn new() -> Self {
        Self {
            font_context: Mutex::new(font_context()),
            layout_context: Mutex::new(LayoutContext::new()),
            font_registry: Mutex::new(FontRegistry::default()),
        }
    }

    fn resolve_font(&self, font: &Font) -> FontId {
        self.font_registry.lock().unwrap().resolve(font)
    }

    fn font_for_id(&self, id: FontId) -> Option<Font> {
        self.font_registry.lock().unwrap().font_for_id(id)
    }

    fn font_data_for_id(&self, id: FontId) -> Option<(&'static [u8], usize)> {
        self.font_for_id(id).map(|font| font_data_for(&font))
    }

    fn rasterize_outline(
        &self,
        params: &RenderGlyphParams,
    ) -> Result<(Bounds<DevicePixels>, Vec<u8>)> {
        let (static_data, index) = self
            .font_data_for_id(params.font_id)
            .context("unknown font")?;
        let data: &[u8] = static_data;
        let font_ref = FontRef::from_index(data, index as u32).context("invalid font data")?;

        let size = SkrifaSize::new(params.font_size.0 * params.scale_factor);
        let outlines = font_ref.outline_glyphs();
        let hinting = HintingInstance::new(
            &outlines,
            size,
            LocationRef::default(),
            HintingOptions::default(),
        )
        .context("unable to create hinting instance")?;
        let glyph_id = SkrifaGlyphId::new(params.glyph_id.0);
        let glyph = outlines.get(glyph_id).context("missing glyph outline")?;

        let mut path_builder = GlyphPathBuilder::new();
        glyph
            .draw(DrawSettings::hinted(&hinting, false), &mut path_builder)
            .context("unable to draw glyph outline")?;
        let Some(path) = path_builder.build() else {
            return Ok((Bounds::default(), Vec::new()));
        };

        let bounds = path.bounds();
        let left = bounds.left().floor();
        let top = bounds.top().floor();
        let right = bounds.right().ceil();
        let bottom = bounds.bottom().ceil();
        let width = (right - left).max(0.0) as u32;
        let height = (bottom - top).max(0.0) as u32;
        if width == 0 || height == 0 {
            return Ok((Bounds::default(), Vec::new()));
        }

        let mut mask = Mask::new(width, height).context("unable to allocate glyph mask")?;
        mask.fill_path(
            &path,
            FillRule::Winding,
            true,
            Transform::from_translate(-left, -top),
        );

        let bounds = Bounds {
            origin: Point {
                x: DevicePixels(left as i32),
                y: DevicePixels(top as i32),
            },
            size: Size {
                width: DevicePixels(width as i32),
                height: DevicePixels(height as i32),
            },
        };
        Ok((bounds, mask.data().to_vec()))
    }

    fn build_layout(
        &self,
        text: &str,
        font_size: Pixels,
        runs: &[FontRun],
    ) -> parley::Layout<[u8; 4]> {
        let mut font_context = self.font_context.lock().unwrap();
        let mut layout_context = self.layout_context.lock().unwrap();

        let mut builder = layout_context.ranged_builder(&mut font_context, text, 1.0, true);
        builder.push_default(StyleProperty::FontFamily(FontFamily::from(FONT_FAMILY)));
        builder.push_default(StyleProperty::FontSize(font_size.0));

        let mut byte = 0;
        for run in runs {
            let end = byte + run.len;
            if let Some(font) = self.font_for_id(run.font_id) {
                builder.push(
                    StyleProperty::FontWeight(map_weight(font.weight)),
                    byte..end,
                );
                builder.push(StyleProperty::FontStyle(map_style(font.style)), byte..end);
            }
            byte = end;
        }

        builder.build(text)
    }

    fn layout_wrapped(
        &self,
        text: &str,
        font_size: Pixels,
        runs: &[FontRun],
        wrap_width: Pixels,
    ) -> (LineLayout, SmallVec<[WrapBoundary; 1]>) {
        let default_font_id = self.resolve_font(&font(FONT_FAMILY));
        let mut layout = self.build_layout(text, font_size, runs);

        layout.break_all_lines(None);
        layout.align(Alignment::Start, AlignmentOptions::default());
        let unwrapped = convert_layout(&layout, runs, default_font_id, font_size, text.len());

        layout.break_all_lines(Some(wrap_width.0));
        let mut boundaries = SmallVec::new();
        for (i, line) in layout.lines().enumerate() {
            if i == 0 {
                continue;
            }
            if let Some(boundary) = wrap_boundary_for_byte(&unwrapped, line.text_range().start) {
                boundaries.push(boundary);
            }
        }

        (unwrapped, boundaries)
    }
}

impl PlatformTextSystem for ParleyPlatformTextSystem {
    fn add_fonts(&self, _fonts: Vec<std::borrow::Cow<'static, [u8]>>) -> Result<()> {
        Ok(())
    }

    fn all_font_names(&self) -> Vec<String> {
        vec![FONT_FAMILY.to_string()]
    }

    fn font_id(&self, descriptor: &Font) -> Result<FontId> {
        Ok(self.resolve_font(descriptor))
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

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        let (data, index) = self.font_data_for_id(font_id).context("unknown font")?;
        let font_ref = FontRef::from_index(data, index as u32).context("invalid font data")?;
        let units_per_em = font_ref.head().map(|head| head.units_per_em())?;
        let glyph_metrics =
            font_ref.glyph_metrics(SkrifaSize::new(units_per_em as f32), LocationRef::default());
        let glyph_id = SkrifaGlyphId::new(glyph_id.0);
        let width = glyph_metrics
            .advance_width(glyph_id)
            .context("glyph out of range")?;
        Ok(Size { width, height: 0.0 })
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        let (data, index) = self.font_data_for_id(font_id)?;
        let font_ref = FontRef::from_index(data, index as u32).ok()?;
        let glyph_id = font_ref.charmap().map(ch)?;
        (glyph_id.to_u32() != 0).then_some(GlyphId(glyph_id.to_u32()))
    }

    fn glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        self.rasterize_outline(params).map(|(bounds, _)| bounds)
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        _raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        self.rasterize_outline(params)
            .map(|(bounds, data)| (bounds.size, data))
    }

    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout {
        let default_font_id = self.resolve_font(&font(FONT_FAMILY));
        let mut layout = self.build_layout(text, font_size, runs);
        layout.break_all_lines(None);
        layout.align(Alignment::Start, AlignmentOptions::default());
        convert_layout(&layout, runs, default_font_id, font_size, text.len())
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
    use gpui_engine::{FontRun, RenderGlyphParams, font};
    use gpui_types::{Point, px};

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

    #[test]
    fn resolves_distinct_fonts_to_distinct_ids_and_back() {
        let text_system = ParleyTextSystem::new();
        let regular = font("IBM Plex Sans");
        let bold = font("IBM Plex Sans").bold();
        let regular_id = text_system.resolve_font(&regular);
        let bold_id = text_system.resolve_font(&bold);

        assert_ne!(regular_id, bold_id);
        assert_eq!(
            text_system.get_font_for_id(regular_id),
            Some(regular.clone())
        );
        assert_eq!(text_system.get_font_for_id(bold_id), Some(bold.clone()));
        assert_eq!(text_system.resolve_font(&regular), regular_id);
    }

    #[test]
    fn shapes_mixed_weight_runs() {
        let text_system = ParleyTextSystem::new();
        let regular = font("IBM Plex Sans");
        let bold = font("IBM Plex Sans").bold();
        let regular_id = text_system.resolve_font(&regular);
        let bold_id = text_system.resolve_font(&bold);

        let runs = [
            FontRun {
                len: 7,
                font_id: regular_id,
            },
            FontRun {
                len: 4,
                font_id: bold_id,
            },
        ];
        let layout = text_system.layout_line("regularbold", px(16.0), &runs, None);

        assert_eq!(layout.len, 11);
        assert!(layout.width > px(0.0));
        assert!(!layout.runs.is_empty());
        assert!(layout.runs.iter().any(|run| run.font_id == regular_id));
        assert!(layout.runs.iter().any(|run| run.font_id == bold_id));
    }

    #[test]
    fn wraps_a_line_into_multiple_lines() {
        let text_system = ParleyTextSystem::new();
        let wrapped = text_system.layout_wrapped_line(
            "hello world this is a long line that should wrap",
            px(16.0),
            &[],
            Some(px(100.0)),
            None,
        );

        assert_eq!(wrapped.wrap_width, Some(px(100.0)));
        assert!(
            wrapped.wrap_boundaries.len() >= 1,
            "expected at least one wrap boundary"
        );
        assert!(wrapped.width() <= px(100.0));
    }

    #[test]
    fn rasterizes_a_glyph() {
        let text_system = ParleyTextSystem::new();
        let font_id = text_system.resolve_font(&font("IBM Plex Sans"));
        let glyph_id = text_system
            .platform_text_system()
            .glyph_for_char(font_id, 'A')
            .expect("the embedded font should map 'A'");
        let params = RenderGlyphParams {
            font_id,
            glyph_id,
            font_size: px(16.0),
            subpixel_variant: Point { x: 0, y: 0 },
            scale_factor: 1.0,
            is_emoji: false,
            subpixel_rendering: false,
            dilation: 0,
        };

        let bounds = text_system.raster_bounds(&params).unwrap();
        assert!(bounds.size.width.0 > 0);
        assert!(bounds.size.height.0 > 0);

        let (size, data) = text_system.rasterize_glyph(&params).unwrap();
        assert_eq!(size.width, bounds.size.width);
        assert_eq!(size.height, bounds.size.height);
        assert!(!data.is_empty());
    }
}
