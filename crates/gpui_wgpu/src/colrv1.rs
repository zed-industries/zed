use skrifa::{
    color::{
        Brush, ColorGlyphFormat, ColorPainter, CompositeMode,
        Transform as SkrifaTransform,
    },
    instance::{LocationRef, Size},
    outline::OutlinePen,
    raw::{types::BoundingBox, FontRef, TableProvider},
    GlyphId as SkrifaGlyphId, MetadataProvider,
};
use tiny_skia::{
    BlendMode, Color, FillRule, GradientStop, LinearGradient, Mask, Paint, Path, PathBuilder,
    Pixmap, PixmapPaint, Point, RadialGradient, Rect, SpreadMode, Transform,
};

struct TinySkiaPen {
    builder: PathBuilder,
}

impl TinySkiaPen {
    fn new() -> Self {
        Self {
            builder: PathBuilder::new(),
        }
    }

    fn finish(self) -> Option<Path> {
        self.builder.finish()
    }
}

impl OutlinePen for TinySkiaPen {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.move_to(x, y);
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(x, y);
    }
    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.builder.quad_to(cx0, cy0, x, y);
    }
    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.builder.cubic_to(cx0, cy0, cx1, cy1, x, y);
    }
    fn close(&mut self) {
        self.builder.close();
    }
}

fn to_tiny_skia_blend_mode(mode: CompositeMode) -> BlendMode {
    match mode {
        CompositeMode::Clear => BlendMode::Clear,
        CompositeMode::Src => BlendMode::Source,
        CompositeMode::Dest => BlendMode::Destination,
        CompositeMode::SrcOver => BlendMode::SourceOver,
        CompositeMode::DestOver => BlendMode::DestinationOver,
        CompositeMode::SrcIn => BlendMode::SourceIn,
        CompositeMode::DestIn => BlendMode::DestinationIn,
        CompositeMode::SrcOut => BlendMode::SourceOut,
        CompositeMode::DestOut => BlendMode::DestinationOut,
        CompositeMode::SrcAtop => BlendMode::SourceAtop,
        CompositeMode::DestAtop => BlendMode::DestinationAtop,
        CompositeMode::Xor => BlendMode::Xor,
        CompositeMode::Plus => BlendMode::Plus,
        CompositeMode::Screen => BlendMode::Screen,
        CompositeMode::Overlay => BlendMode::Overlay,
        CompositeMode::Darken => BlendMode::Darken,
        CompositeMode::Lighten => BlendMode::Lighten,
        CompositeMode::ColorDodge => BlendMode::ColorDodge,
        CompositeMode::ColorBurn => BlendMode::ColorBurn,
        CompositeMode::HardLight => BlendMode::HardLight,
        CompositeMode::SoftLight => BlendMode::SoftLight,
        CompositeMode::Difference => BlendMode::Difference,
        CompositeMode::Exclusion => BlendMode::Exclusion,
        CompositeMode::Multiply => BlendMode::Multiply,
        CompositeMode::HslHue => BlendMode::Hue,
        CompositeMode::HslSaturation => BlendMode::Saturation,
        CompositeMode::HslColor => BlendMode::Color,
        CompositeMode::HslLuminosity => BlendMode::Luminosity,
        _ => BlendMode::SourceOver,
    }
}

fn to_tiny_skia_transform(t: SkrifaTransform) -> Transform {
    Transform::from_row(t.xx, t.yx, t.xy, t.yy, t.dx, t.dy)
}

fn extract_palette(font: &FontRef, palette_index: u16) -> Vec<Color> {
    let Ok(cpal) = font.cpal() else {
        return Vec::new();
    };
    let num_palettes = cpal.num_palettes() as usize;
    if num_palettes == 0 {
        return Vec::new();
    }
    let palette_idx = (palette_index as usize).min(num_palettes - 1);
    let indices = cpal.color_record_indices();
    if palette_idx >= indices.len() {
        return Vec::new();
    }
    let first_color_idx = indices[palette_idx].get() as usize;
    let num_entries = cpal.num_palette_entries() as usize;
    let Some(Ok(records)) = cpal.color_records_array() else {
        return Vec::new();
    };
    let end_idx = (first_color_idx + num_entries).min(records.len());
    if first_color_idx > end_idx {
        return Vec::new();
    }
    records[first_color_idx..end_idx]
        .iter()
        .map(|r| Color::from_rgba8(r.red, r.green, r.blue, r.alpha))
        .collect()
}

pub struct TinySkiaColorPainter<'a> {
    font: &'a FontRef<'a>,
    palette: &'a [Color],
    foreground_color: Color,
    layers: Vec<Pixmap>,
    modes: Vec<CompositeMode>,
    transforms: Vec<Transform>,
    clip_masks: Vec<Option<Mask>>,
}

impl<'a> TinySkiaColorPainter<'a> {
    fn resolve_color(&self, palette_index: u16, alpha: f32) -> Color {
        let base = if palette_index == 0xFFFF {
            self.foreground_color
        } else {
            self.palette
                .get(palette_index as usize)
                .copied()
                .unwrap_or(self.foreground_color)
        };
        Color::from_rgba(
            base.red(),
            base.green(),
            base.blue(),
            (base.alpha() * alpha).clamp(0.0, 1.0),
        )
        .unwrap_or(Color::TRANSPARENT)
    }

    fn glyph_path(&self, glyph_id: SkrifaGlyphId) -> Option<Path> {
        let outlines = self.font.outline_glyphs();
        let glyph = outlines.get(glyph_id)?;
        let mut pen = TinySkiaPen::new();
        let settings = skrifa::outline::DrawSettings::unhinted(
            skrifa::instance::Size::unscaled(),
            LocationRef::default(),
        );
        glyph.draw(settings, &mut pen).ok()?;
        pen.finish()
    }

    fn current_pixmap(&mut self) -> &mut Pixmap {
        self.layers.last_mut().expect("layers underflow")
    }

    fn current_transform(&self) -> Transform {
        *self.transforms.last().unwrap_or(&Transform::identity())
    }

    fn current_mask(&self) -> Option<&Mask> {
        self.clip_masks.last().and_then(|m| m.as_ref())
    }

    fn paint_with_brush(
        &mut self,
        path: &Path,
        fill_transform: Transform,
        brush: Brush<'_>,
        brush_transform: Option<Transform>,
    ) {
        let mut paint = Paint::default();
        paint.anti_alias = true;

        match brush {
            Brush::Solid {
                palette_index,
                alpha,
            } => {
                paint.set_color(self.resolve_color(palette_index, alpha));
            }
            Brush::LinearGradient {
                p0,
                p1,
                color_stops,
                extend,
            } => {
                let stops: Vec<GradientStop> = color_stops
                    .iter()
                    .map(|s| {
                        GradientStop::new(
                            s.offset,
                            self.resolve_color(s.palette_index, s.alpha),
                        )
                    })
                    .collect();
                let mode = match extend {
                    skrifa::raw::tables::colr::Extend::Pad => SpreadMode::Pad,
                    skrifa::raw::tables::colr::Extend::Repeat => SpreadMode::Repeat,
                    skrifa::raw::tables::colr::Extend::Reflect => SpreadMode::Reflect,
                    _ => SpreadMode::Pad,
                };
                let ts = brush_transform.unwrap_or(fill_transform);
                if let Some(shader) = LinearGradient::new(
                    Point::from_xy(p0.x, p0.y),
                    Point::from_xy(p1.x, p1.y),
                    stops,
                    mode,
                    ts,
                ) {
                    paint.shader = shader;
                } else if let Some(first) = color_stops.first() {
                    paint.set_color(self.resolve_color(first.palette_index, first.alpha));
                }
            }
            Brush::RadialGradient {
                c0,
                r0: _,
                c1,
                r1,
                color_stops,
                extend,
            } => {
                let stops: Vec<GradientStop> = color_stops
                    .iter()
                    .map(|s| {
                        GradientStop::new(
                            s.offset,
                            self.resolve_color(s.palette_index, s.alpha),
                        )
                    })
                    .collect();
                let mode = match extend {
                    skrifa::raw::tables::colr::Extend::Pad => SpreadMode::Pad,
                    skrifa::raw::tables::colr::Extend::Repeat => SpreadMode::Repeat,
                    skrifa::raw::tables::colr::Extend::Reflect => SpreadMode::Reflect,
                    _ => SpreadMode::Pad,
                };
                let ts = brush_transform.unwrap_or(fill_transform);
                if r1 > 0.0 {
                    if let Some(shader) = RadialGradient::new(
                        Point::from_xy(c0.x, c0.y),
                        Point::from_xy(c1.x, c1.y),
                        r1,
                        stops,
                        mode,
                        ts,
                    ) {
                        paint.shader = shader;
                    } else if let Some(first) = color_stops.first() {
                        paint.set_color(self.resolve_color(first.palette_index, first.alpha));
                    }
                } else if let Some(first) = color_stops.first() {
                    paint.set_color(self.resolve_color(first.palette_index, first.alpha));
                }
            }
            Brush::SweepGradient { color_stops, .. } => {
                if let Some(first) = color_stops.first() {
                    paint.set_color(self.resolve_color(first.palette_index, first.alpha));
                }
            }
        }

        let mask = self.current_mask().cloned();
        self.current_pixmap().fill_path(
            path,
            &paint,
            FillRule::Winding,
            fill_transform,
            mask.as_ref(),
        );
    }
}

impl<'a> ColorPainter for TinySkiaColorPainter<'a> {
    fn push_transform(&mut self, transform: SkrifaTransform) {
        let ts = to_tiny_skia_transform(transform);
        let next = self.current_transform().pre_concat(ts);
        self.transforms.push(next);
    }

    fn pop_transform(&mut self) {
        self.transforms.pop();
    }

    fn push_clip_glyph(&mut self, glyph_id: SkrifaGlyphId) {
        if let Some(path) = self.glyph_path(glyph_id) {
            let width = self.current_pixmap().width();
            let height = self.current_pixmap().height();
            let mask = match self.current_mask() {
                Some(parent) => {
                    let mut m = parent.clone();
                    m.intersect_path(&path, FillRule::Winding, true, self.current_transform());
                    m
                }
                None => {
                    if let Some(mut m) = Mask::new(width, height) {
                        m.fill_path(&path, FillRule::Winding, true, self.current_transform());
                        m
                    } else {
                        self.clip_masks.push(None);
                        return;
                    }
                }
            };
            self.clip_masks.push(Some(mask));
        } else {
            self.clip_masks.push(None);
        }
    }

    fn push_clip_box(&mut self, clip_box: BoundingBox<f32>) {
        let width = self.current_pixmap().width();
        let height = self.current_pixmap().height();
        let rect = Rect::from_ltrb(clip_box.x_min, clip_box.y_min, clip_box.x_max, clip_box.y_max);
        if let Some(r) = rect {
            let path = PathBuilder::from_rect(r);
            let mask = match self.current_mask() {
                Some(parent) => {
                    let mut m = parent.clone();
                    m.intersect_path(&path, FillRule::Winding, true, self.current_transform());
                    m
                }
                None => {
                    if let Some(mut m) = Mask::new(width, height) {
                        m.fill_path(&path, FillRule::Winding, true, self.current_transform());
                        m
                    } else {
                        self.clip_masks.push(None);
                        return;
                    }
                }
            };
            self.clip_masks.push(Some(mask));
        } else {
            self.clip_masks.push(None);
        }
    }

    fn pop_clip(&mut self) {
        self.clip_masks.pop();
    }

    fn fill(&mut self, brush: Brush<'_>) {
        let width = self.current_pixmap().width() as f32;
        let height = self.current_pixmap().height() as f32;
        if let Some(rect) = Rect::from_xywh(0.0, 0.0, width, height) {
            let path = PathBuilder::from_rect(rect);
            self.paint_with_brush(
                &path,
                Transform::identity(),
                brush,
                Some(self.current_transform()),
            );
        }
    }

    fn fill_glyph(
        &mut self,
        glyph_id: SkrifaGlyphId,
        brush_transform: Option<SkrifaTransform>,
        brush: Brush<'_>,
    ) {
        if let Some(path) = self.glyph_path(glyph_id) {
            let brush_ts = brush_transform.map(|bt| {
                let ts = to_tiny_skia_transform(bt);
                self.current_transform().pre_concat(ts)
            });
            self.paint_with_brush(&path, self.current_transform(), brush, brush_ts);
        }
    }

    fn push_layer(&mut self, composite_mode: CompositeMode) {
        let width = self.current_pixmap().width();
        let height = self.current_pixmap().height();
        if let Some(new_layer) = Pixmap::new(width, height) {
            self.layers.push(new_layer);
            self.modes.push(composite_mode);
        }
    }

    fn pop_layer(&mut self) {
        if self.layers.len() > 1 {
            let top = self.layers.pop().unwrap();
            let mode = self.modes.pop().unwrap_or(CompositeMode::SrcOver);
            let mut paint = PixmapPaint::default();
            paint.blend_mode = to_tiny_skia_blend_mode(mode);
            let mask = self.current_mask().cloned();
            self.current_pixmap().draw_pixmap(
                0,
                0,
                top.as_ref(),
                &paint,
                Transform::identity(),
                mask.as_ref(),
            );
        }
    }
}

/// Detects whether a font has a COLRv1 table.
pub fn font_has_colrv1(font_data: &[u8], face_index: u32) -> bool {
    if let Ok(font) = FontRef::from_index(font_data, face_index) {
        if let Ok(colr) = font.colr() {
            colr.version() == 1
        } else {
            false
        }
    } else {
        false
    }
}

/// Checks whether a specific glyph in the font is defined as a COLRv1 glyph.
pub fn has_colrv1_glyph(font_data: &[u8], face_index: u32, glyph_id: u16) -> bool {
    if let Ok(font) = FontRef::from_index(font_data, face_index) {
        font.color_glyphs()
            .get_with_format(SkrifaGlyphId::new(glyph_id.into()), ColorGlyphFormat::ColrV1)
            .is_some()
    } else {
        false
    }
}

/// Renders a COLRv1 glyph to an RGBA image buffer suitable for GPUI's sprite atlas.
pub fn render_colrv1_glyph(
    font_data: &[u8],
    face_index: u32,
    glyph_id: u16,
    font_size: f32,
    scale_factor: f32,
) -> Option<swash::scale::image::Image> {
    let font = FontRef::from_index(font_data, face_index).ok()?;
    let colr_glyph = font
        .color_glyphs()
        .get_with_format(SkrifaGlyphId::new(glyph_id.into()), ColorGlyphFormat::ColrV1)?;

    let head = font.head().ok()?;
    let upem = head.units_per_em() as f32;
    if upem <= 0.0 {
        return None;
    }

    let scale = (font_size * scale_factor) / upem;
    let location = LocationRef::default();
    let size = Size::new(font_size * scale_factor);

    let bbox = colr_glyph.bounding_box(location, size).unwrap_or_else(|| {
        let hhea = font.hhea().ok();
        let ascender =
            hhea.as_ref().map(|h| h.ascender().to_i16() as f32).unwrap_or(upem * 0.8) * scale;
        let descender =
            hhea.as_ref().map(|h| h.descender().to_i16() as f32).unwrap_or(-upem * 0.2) * scale;
        BoundingBox {
            x_min: 0.0,
            y_min: descender,
            x_max: font_size * scale_factor,
            y_max: ascender,
        }
    });

    let left = bbox.x_min.floor() as i32;
    let top = bbox.y_max.ceil() as i32;
    let width = (bbox.x_max.ceil() as i32 - left).max(1) as u32;
    let height = (top - bbox.y_min.floor() as i32).max(1) as u32;

    let base_pixmap = Pixmap::new(width, height)?;
    let palette = extract_palette(&font, 0);

    let base_transform =
        Transform::from_row(scale, 0.0, 0.0, -scale, -left as f32, top as f32);

    let mut painter = TinySkiaColorPainter {
        font: &font,
        palette: &palette,
        foreground_color: Color::from_rgba8(0, 0, 0, 255),
        layers: vec![base_pixmap],
        modes: Vec::new(),
        transforms: vec![base_transform],
        clip_masks: vec![None],
    };

    colr_glyph.paint(location, &mut painter).ok()?;

    let pixmap = painter.layers.pop()?;
    Some(swash::scale::image::Image {
        content: swash::scale::image::Content::Color,
        placement: swash::zeno::Placement {
            left,
            top,
            width,
            height,
        },
        source: swash::scale::Source::ColorOutline(0),
        data: pixmap.take(),
    })
}
