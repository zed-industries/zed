use crate::canvas_fallback::{CanvasFontFallback, classify_canvas_fallback};
use crate::canvas_text::{self, CanvasTextMetrics};
use crate::glyph_cache::{CanvasGlyph, GlyphCache};
use crate::run_replacements::{Replacement, apply_replacements, collect_candidates};
use anyhow::{Context as _, Result, ensure};
use gpui::{
    Bounds, DevicePixels, Font, FontId, FontMetrics, FontRun, FontStyle, GlyphId, Hsla, LineLayout,
    MissingGlyphSink, Pixels, PlatformTextSystem, RenderGlyphParams, RetainedLineLayout,
    SUBPIXEL_VARIANTS_X, SUBPIXEL_VARIANTS_Y, ShapedGlyph, Size, TextRenderingMode, point, px,
    size,
};
use gpui_wgpu::CosmicTextSystem;
use parking_lot::RwLock;
use std::{borrow::Cow, collections::HashMap, sync::Arc};

// Cosmic's font IDs index its loaded-font vector. Keep browser IDs in a disjoint namespace.
const CANVAS_FONT_BIT: usize = 1 << (usize::BITS - 1);
const MAX_MEASUREMENTS: usize = 4096;

pub(crate) struct WebTextSystem {
    native: CosmicTextSystem,
    state: RwLock<State>,
    canvas_font_fallback: CanvasFontFallback,
}

#[derive(Default)]
struct State {
    missing_glyph_sink: Option<Arc<dyn MissingGlyphSink>>,
    descriptors: HashMap<FontId, Font>,
    canvas_fonts: Vec<Arc<CanvasFont>>,
    canvas_font_ids: HashMap<FontId, FontId>,
    glyphs: GlyphCache,
    measurements: HashMap<(FontId, GlyphId, Pixels), CanvasTextMetrics>,
}

struct CanvasFont {
    native_id: FontId,
    descriptor: Font,
    monospace: bool,
}

struct PreciseLayout {
    glyph_x_positions: Vec<f64>,
    width: f64,
}

impl State {
    fn cached_fallback(
        &self,
        native_id: FontId,
        text: &str,
        color: bool,
        font_size: Pixels,
    ) -> Option<(FontId, GlyphId, CanvasTextMetrics)> {
        let font_id = *self.canvas_font_ids.get(&native_id)?;
        let glyph_id = self.glyphs.id_for(font_id, text, color)?;
        let metrics = *self.measurements.get(&(font_id, glyph_id, font_size))?;
        Some((font_id, glyph_id, metrics))
    }
}

impl WebTextSystem {
    pub(crate) fn new(
        system_font_fallback: &str,
        canvas_font_fallback: CanvasFontFallback,
    ) -> Self {
        Self {
            native: CosmicTextSystem::new_without_system_fonts(system_font_fallback),
            state: RwLock::new(State::default()),
            canvas_font_fallback,
        }
    }

    fn canvas_font(&self, font_id: FontId) -> Option<Arc<CanvasFont>> {
        if font_id.0 & CANVAS_FONT_BIT == 0 {
            return None;
        }
        self.state
            .read()
            .canvas_fonts
            .get(font_id.0 & !CANVAS_FONT_BIT)
            .cloned()
    }

    fn native_font_id(&self, font_id: FontId) -> FontId {
        self.canvas_font(font_id)
            .map_or(font_id, |font| font.native_id)
    }

    fn native_runs<'a>(&self, runs: &'a [FontRun]) -> Cow<'a, [FontRun]> {
        if runs.iter().any(|run| run.font_id.0 & CANVAS_FONT_BIT != 0) {
            Cow::Owned(
                runs.iter()
                    .map(|run| FontRun {
                        len: run.len,
                        font_id: self.native_font_id(run.font_id),
                    })
                    .collect(),
            )
        } else {
            Cow::Borrowed(runs)
        }
    }

    fn canvas_font_id(&self, native_id: FontId) -> Result<FontId> {
        let descriptor = {
            let state = self.state.read();
            if let Some(font_id) = state.canvas_font_ids.get(&native_id) {
                return Ok(*font_id);
            }
            state
                .descriptors
                .get(&native_id)
                .cloned()
                .context("missing source font descriptor for Canvas fallback")?
        };
        let monospace = match (
            self.native.glyph_for_char(native_id, 'i'),
            self.native.glyph_for_char(native_id, 'm'),
        ) {
            (Some(narrow), Some(wide)) => {
                self.native.advance(native_id, narrow)?.width
                    == self.native.advance(native_id, wide)?.width
            }
            _ => false,
        };
        let mut state = self.state.write();
        if let Some(font_id) = state.canvas_font_ids.get(&native_id) {
            return Ok(*font_id);
        }
        ensure!(
            state.canvas_fonts.len() < CANVAS_FONT_BIT,
            "Canvas font ID space exhausted"
        );
        let font_id = FontId(CANVAS_FONT_BIT | state.canvas_fonts.len());
        state.canvas_fonts.push(Arc::new(CanvasFont {
            native_id,
            descriptor,
            monospace,
        }));
        state.canvas_font_ids.insert(native_id, font_id);
        Ok(font_id)
    }

    fn register_glyph(&self, font_id: FontId, text: &str, color: bool) -> Result<GlyphId> {
        if let Some(glyph_id) = self.state.read().glyphs.id_for(font_id, text, color) {
            return Ok(glyph_id);
        }
        // Another thread may register the same glyph between the read and write locks.
        self.state
            .write()
            .glyphs
            .get_or_insert(font_id, text, color)
    }

    fn canvas_glyph(&self, font_id: FontId, glyph_id: GlyphId) -> Result<CanvasGlyph> {
        self.state
            .read()
            .glyphs
            .glyph(font_id, glyph_id)
            .cloned()
            .context("invalid Canvas glyph ID")
    }

    fn measure(
        &self,
        font_id: FontId,
        glyph_id: GlyphId,
        font_size: Pixels,
    ) -> Result<CanvasTextMetrics> {
        let key = (font_id, glyph_id, font_size);
        if let Some(metrics) = self.state.read().measurements.get(&key) {
            return Ok(*metrics);
        }
        let font = self
            .canvas_font(font_id)
            .context("invalid Canvas font ID")?;
        let glyph = self.canvas_glyph(font_id, glyph_id)?;
        let metrics = canvas_text::measure(&glyph.text, &font.css_font(font_size, glyph.color)?)?;
        ensure!(metrics.advance >= 0.0, "Canvas returned a negative advance");
        let mut state = self.state.write();
        if state.measurements.len() >= MAX_MEASUREMENTS {
            state.measurements.clear();
        }
        state.measurements.insert(key, metrics);
        Ok(metrics)
    }

    fn fallback_glyph(
        &self,
        native_id: FontId,
        text: &str,
        color: bool,
        font_size: Pixels,
    ) -> Result<(FontId, GlyphId, CanvasTextMetrics)> {
        if let Some(cached) = self
            .state
            .read()
            .cached_fallback(native_id, text, color, font_size)
        {
            return Ok(cached);
        }
        let font_id = self.canvas_font_id(native_id)?;
        let glyph_id = self.register_glyph(font_id, text, color)?;
        let metrics = self.measure(font_id, glyph_id, font_size)?;
        Ok((font_id, glyph_id, metrics))
    }

    fn uses_canvas_fallback(&self, text: &str, font_size: Pixels) -> bool {
        self.canvas_font_fallback != CanvasFontFallback::Disabled
            && !text.is_ascii()
            && f32::from(font_size).is_finite()
            && font_size > px(0.)
    }

    fn apply_fallback(
        &self,
        text: &str,
        font_runs: &[FontRun],
        layout: &mut LineLayout,
        mut precise: Option<&mut PreciseLayout>,
    ) {
        if !self.uses_canvas_fallback(text, layout.font_size) {
            return;
        }
        let mut replacements = Vec::new();
        for candidate in collect_candidates(text, font_runs, layout) {
            if !self.canvas_font_fallback.allows(candidate.is_emoji) {
                continue;
            }
            let Some(glyphs) = candidate.glyphs else {
                continue;
            };
            if !glyphs.contiguous || !(glyphs.missing || (candidate.color && !glyphs.native_color))
            {
                continue;
            }
            if !layout.caret_positions.is_empty()
                && [candidate.source.start, candidate.source.end]
                    .iter()
                    .any(|index| {
                        layout
                            .caret_positions
                            .binary_search_by_key(index, |(index, _)| *index)
                            .is_err()
                    })
            {
                continue;
            }
            let start_x = layout.caret_x_for_index(candidate.source.start);
            let old_width = layout.caret_x_for_index(candidate.source.end) - start_x;
            if old_width < 0.0 {
                continue;
            }
            let replacement = (|| -> Result<Replacement> {
                let text = text
                    .get(candidate.source.clone())
                    .context("invalid Canvas fallback source range")?;
                let (font_id, glyph_id, metrics) = self.fallback_glyph(
                    candidate.font_id,
                    text,
                    candidate.color,
                    layout.font_size,
                )?;
                Ok(Replacement {
                    source: candidate.source.clone(),
                    glyph_range: glyphs.range,
                    font_id,
                    glyph: ShapedGlyph {
                        id: glyph_id,
                        position: point(Pixels::from(start_x), px(0.)),
                        index: candidate.source.start,
                        is_emoji: candidate.color,
                    },
                    width_delta: f64::from(metrics.advance) - old_width,
                })
            })();
            match replacement {
                Ok(replacement) => replacements.push(replacement),
                Err(error) => log::warn!("Canvas font fallback failed: {error:#}"),
            }
        }
        if replacements.is_empty() {
            return;
        }
        replacements.sort_unstable_by_key(|replacement| replacement.glyph_range.start);
        if let Some(precise) = precise.as_deref_mut() {
            precise.apply_replacements(layout, &replacements);
        }
        // Keep the primary font's baseline and line spacing stable. Browser ink
        // extents affect raster bounds, not the surrounding editor's line metrics.
        apply_replacements(layout, replacements);
        if let Some(precise) = precise {
            for (glyph, position) in layout
                .runs
                .iter_mut()
                .flat_map(|run| &mut run.glyphs)
                .zip(&precise.glyph_x_positions)
            {
                glyph.position.x = Pixels::from(*position);
            }
            layout.width = Pixels::from(precise.width);
        }
    }

    fn finish_layout(
        &self,
        text: &str,
        native_runs: &[FontRun],
        layout: &mut LineLayout,
        precise: Option<&mut PreciseLayout>,
    ) {
        self.apply_fallback(text, native_runs, layout, precise);
        let sink = self.state.read().missing_glyph_sink.clone();
        if let Some(sink) = sink {
            let missing_indices = layout
                .runs
                .iter()
                .filter(|run| run.font_id.0 & CANVAS_FONT_BIT == 0)
                .flat_map(|run| &run.glyphs)
                .filter(|glyph| glyph.id.0 == 0)
                .map(|glyph| glyph.index);
            sink.report(
                self.native
                    .missing_glyphs(text, native_runs, missing_indices),
            );
        }
    }

    fn layout_line_precisely(
        &self,
        text: &str,
        font_size: Pixels,
        runs: &[FontRun],
        for_retention: bool,
    ) -> (LineLayout, Vec<f64>, f64) {
        let native_runs = self.native_runs(runs);
        let (mut layout, glyph_x_positions, width) = self.native.layout_line_with_precise_geometry(
            text,
            font_size,
            &native_runs,
            for_retention,
        );
        let mut precise = PreciseLayout {
            glyph_x_positions,
            width,
        };
        self.finish_layout(text, &native_runs, &mut layout, Some(&mut precise));
        (layout, precise.glyph_x_positions, precise.width)
    }
}

impl PreciseLayout {
    fn apply_replacements(&mut self, layout: &LineLayout, replacements: &[Replacement]) {
        let mut replacements = replacements.iter().peekable();
        let mut original_index = 0;
        let mut consumed_end = 0;
        let mut shift = 0.0;
        self.glyph_x_positions.retain_mut(|position| {
            let index = original_index;
            original_index += 1;
            if index < consumed_end {
                return false;
            }
            if let Some(replacement) =
                replacements.next_if(|replacement| replacement.glyph_range.start == index)
            {
                consumed_end = replacement.glyph_range.end;
                *position = layout.caret_x_for_index(replacement.source.start) + shift;
                shift += replacement.width_delta;
            } else {
                *position += shift;
            }
            true
        });
        self.width += shift;
    }
}

impl PlatformTextSystem for WebTextSystem {
    fn set_missing_glyph_sink(&self, sink: Option<Arc<dyn MissingGlyphSink>>) {
        // Cosmic must not report missing glyphs before Canvas has a chance to replace them.
        self.state.write().missing_glyph_sink = sink;
    }

    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        self.native.add_fonts(fonts)?;
        self.state.write().measurements.clear();
        Ok(())
    }

    fn all_font_names(&self) -> Vec<String> {
        self.native.all_font_names()
    }

    fn font_id(&self, font: &Font) -> Result<FontId> {
        let font_id = self.native.font_id(font)?;
        ensure!(
            font_id.0 & CANVAS_FONT_BIT == 0,
            "native font ID space exhausted"
        );
        let mut descriptor = font.clone();
        (descriptor.weight, descriptor.style) = self.native.font_weight_and_style(font_id)?;
        self.state
            .write()
            .descriptors
            .entry(font_id)
            .or_insert(descriptor);
        Ok(font_id)
    }

    fn prewarm_fonts(&self, font_ids: &[FontId]) {
        if font_ids
            .iter()
            .all(|font_id| font_id.0 & CANVAS_FONT_BIT == 0)
        {
            return self.native.prewarm_fonts(font_ids);
        }
        let native_ids: Vec<_> = font_ids
            .iter()
            .map(|font_id| self.native_font_id(*font_id))
            .collect();
        self.native.prewarm_fonts(&native_ids);
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        self.native.font_metrics(self.native_font_id(font_id))
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        let Some(font) = self.canvas_font(font_id) else {
            return self.native.typographic_bounds(font_id, glyph_id);
        };
        let font_size = px(self.native.font_metrics(font.native_id).units_per_em as f32);
        let metrics = self.measure(font_id, glyph_id, font_size)?;
        Ok(Bounds {
            origin: point(metrics.left, -metrics.ascent),
            size: size(
                metrics.right - metrics.left,
                metrics.ascent + metrics.descent,
            ),
        })
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        let Some(font) = self.canvas_font(font_id) else {
            return self.native.advance(font_id, glyph_id);
        };
        let font_size = px(self.native.font_metrics(font.native_id).units_per_em as f32);
        Ok(size(
            self.measure(font_id, glyph_id, font_size)?.advance,
            0.,
        ))
    }

    fn glyph_for_char(&self, font_id: FontId, character: char) -> Option<GlyphId> {
        if self.canvas_font(font_id).is_none() {
            return self.native.glyph_for_char(font_id, character);
        }
        let mut buffer = [0; 4];
        let text = character.encode_utf8(&mut buffer);
        let color =
            classify_canvas_fallback(text).is_some_and(|fallback| fallback.emoji_presentation);
        match self.register_glyph(font_id, text, color) {
            Ok(glyph_id) => Some(glyph_id),
            Err(error) => {
                log::warn!("registering Canvas glyph failed: {error:#}");
                None
            }
        }
    }

    fn glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        if self.canvas_font(params.font_id).is_none() {
            return self.native.glyph_raster_bounds(params);
        }
        let metrics = self.measure(
            params.font_id,
            params.glyph_id,
            params.font_size * params.scale_factor,
        )?;
        if metrics.right <= metrics.left || metrics.ascent + metrics.descent <= 0. {
            return Ok(Bounds::default());
        }
        let (offset_x, offset_y) = subpixel_offset(params);
        let left = (f64::from(metrics.left) + f64::from(offset_x)).floor() - 1.;
        let top = (-f64::from(metrics.ascent) + f64::from(offset_y)).floor() - 1.;
        let right = (f64::from(metrics.right) + f64::from(offset_x)).ceil() + 1.;
        let bottom = (f64::from(metrics.descent) + f64::from(offset_y)).ceil() + 1.;
        ensure!(
            [left, top, right, bottom]
                .into_iter()
                .all(|value| value >= f64::from(i32::MIN) && value <= f64::from(i32::MAX)),
            "Canvas text bounds exceed device coordinates"
        );
        let width = (right as i32)
            .checked_sub(left as i32)
            .context("Canvas text width overflow")?;
        let height = (bottom as i32)
            .checked_sub(top as i32)
            .context("Canvas text height overflow")?;
        Ok(Bounds {
            origin: point((left as i32).into(), (top as i32).into()),
            size: size(width.into(), height.into()),
        })
    }

    fn glyph_raster_bounds_with_image(
        &self,
        params: &RenderGlyphParams,
    ) -> Result<(Bounds<DevicePixels>, Option<Vec<u8>>)> {
        if self.canvas_font(params.font_id).is_none() {
            self.native.glyph_raster_bounds_with_image(params)
        } else {
            Ok((self.glyph_raster_bounds(params)?, None))
        }
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        let Some(font) = self.canvas_font(params.font_id) else {
            return self.native.rasterize_glyph(params, bounds);
        };
        let glyph = self.canvas_glyph(params.font_id, params.glyph_id)?;
        let css_font = font.css_font(params.font_size * params.scale_factor, glyph.color)?;
        let mut pixels = canvas_text::rasterize(
            &glyph.text,
            &css_font,
            bounds,
            subpixel_offset(params),
            glyph.color,
        )?;
        if params.subpixel_rendering && !glyph.color {
            pixels = pixels.into_iter().flat_map(|alpha| [alpha; 4]).collect();
        }
        Ok((bounds.size, pixels))
    }

    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout {
        let native_runs = self.native_runs(runs);
        let mut layout = self.native.layout_line(text, font_size, &native_runs);
        self.finish_layout(text, &native_runs, &mut layout, None);
        layout
    }

    fn layout_line_for_retention(
        &self,
        text: &str,
        font_size: Pixels,
        runs: &[FontRun],
    ) -> RetainedLineLayout {
        let (layout, positions, width) = self.layout_line_precisely(text, font_size, runs, true);
        RetainedLineLayout::Dense(layout, positions, width)
    }

    fn layout_line_with_precise_positions(
        &self,
        text: &str,
        font_size: Pixels,
        runs: &[FontRun],
    ) -> (LineLayout, Vec<f64>, f64) {
        self.layout_line_precisely(text, font_size, runs, false)
    }

    fn recommended_rendering_mode(&self, font_id: FontId, font_size: Pixels) -> TextRenderingMode {
        if self.canvas_font(font_id).is_some() {
            TextRenderingMode::Grayscale
        } else {
            self.native.recommended_rendering_mode(font_id, font_size)
        }
    }

    fn glyph_dilation_for_color(&self, color: Hsla) -> u8 {
        self.native.glyph_dilation_for_color(color)
    }
}

impl CanvasFont {
    fn css_font(&self, font_size: Pixels, color: bool) -> Result<String> {
        let font_size = f32::from(font_size);
        ensure!(
            font_size.is_finite() && font_size > 0.,
            "invalid Canvas font size"
        );
        let style = match self.descriptor.style {
            FontStyle::Normal => "normal",
            FontStyle::Italic => "italic",
            FontStyle::Oblique => "oblique",
        };
        let mut families = Vec::new();
        if !color && !self.descriptor.family.starts_with('.') {
            families.push(quote_family(&self.descriptor.family));
        }
        if let Some(fallbacks) = &self.descriptor.fallbacks {
            families.extend(
                fallbacks
                    .fallback_list()
                    .iter()
                    .map(|family| quote_family(family)),
            );
        }
        families.push(
            if color {
                "emoji"
            } else if self.monospace {
                "monospace"
            } else {
                "sans-serif"
            }
            .to_owned(),
        );
        Ok(format!(
            "{style} {} {font_size}px {}",
            self.descriptor.weight.0,
            families.join(", ")
        ))
    }
}

fn quote_family(family: &str) -> String {
    let mut quoted = String::from("\"");
    for character in family.chars() {
        match character {
            '\\' | '"' => {
                quoted.push('\\');
                quoted.push(character);
            }
            character if character.is_control() => quoted.push(' '),
            character => quoted.push(character),
        }
    }
    quoted.push('"');
    quoted
}

fn subpixel_offset(params: &RenderGlyphParams) -> (f32, f32) {
    (
        f32::from(params.subpixel_variant.x) / SUBPIXEL_VARIANTS_X as f32,
        f32::from(params.subpixel_variant.y) / SUBPIXEL_VARIANTS_Y as f32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::bail;
    use gpui::ShapedRun;

    #[test]
    fn precise_and_retained_layout_preserve_long_lines() -> Result<()> {
        let (text_system, font_id) = text_system()?;
        let font_size = px(16.1);
        let advance = f64::from(
            text_system
                .native
                .layout_line("m", font_size, &[FontRun { font_id, len: 1 }])
                .width,
        );
        let (canvas_font_id, canvas_glyph_id) =
            cache_fallback(&text_system, font_id, font_size, "界", 12.25)?;
        let count = 100_000;
        for fallback in [false, true] {
            let mut text = "m".repeat(count);
            if fallback {
                text.push_str("界m界");
            }
            let runs = [FontRun {
                font_id: if fallback { canvas_font_id } else { font_id },
                len: text.len(),
            }];
            let expected_width = if fallback {
                advance * (count + 1) as f64 + 24.5
            } else {
                advance * count as f64
            };
            for for_retention in [false, true] {
                let (layout, positions, width) = if for_retention {
                    let RetainedLineLayout::Dense(layout, positions, width) =
                        text_system.layout_line_for_retention(&text, font_size, &runs)
                    else {
                        bail!("expected a dense Web layout");
                    };
                    (layout, positions, width)
                } else {
                    text_system.layout_line_with_precise_positions(&text, font_size, &runs)
                };
                assert_eq!(layout.len, text.len());
                assert_eq!(width, expected_width);
                assert_eq!(positions.len(), count + if fallback { 3 } else { 0 });
                for (index, position) in positions.iter().take(count).enumerate() {
                    assert_eq!(*position, advance * index as f64, "glyph {index}");
                }
                if fallback {
                    assert_eq!(
                        &positions[count..],
                        [
                            advance * count as f64,
                            advance * count as f64 + 12.25,
                            advance * (count + 1) as f64 + 12.25,
                        ],
                    );
                    assert_eq!(
                        layout
                            .runs
                            .iter()
                            .filter(|run| run.font_id == canvas_font_id)
                            .flat_map(|run| &run.glyphs)
                            .map(|glyph| (glyph.id, glyph.index))
                            .collect::<Vec<_>>(),
                        [(canvas_glyph_id, count), (canvas_glyph_id, count + 4)],
                    );
                }
                assert_geometry(&layout, &positions, width);
            }
        }
        Ok(())
    }

    #[test]
    fn precise_fallback_uses_pen_boundaries() -> Result<()> {
        let (text_system, font_id) = text_system()?;
        let font_size = px(16.1);
        let (canvas_font_id, ideograph) =
            cache_fallback(&text_system, font_id, font_size, "界", 12.0)?;
        let text = "m界aか\u{3099}b界";
        let runs = [FontRun {
            font_id,
            len: text.len(),
        }];
        for (kana_advance, expected_positions, expected_width) in [
            (0.0, [0.0, 9.0, 22.0, 25.0, 32.0], 44.0),
            (4.5, [0.0, 9.0, 22.0, 29.5, 36.5], 48.5),
        ] {
            let (_, kana) =
                cache_fallback(&text_system, font_id, font_size, "か\u{3099}", kana_advance)?;
            for origin in [0.0, 67_108_864.25] {
                let mut precise = PreciseLayout {
                    glyph_x_positions: std::iter::once(0.0)
                        .chain([2.0, 7.0, 21.0, 22.0, 33.0, 42.0].map(|x| origin + x))
                        .collect(),
                    width: origin + 50.0,
                };
                let mut layout = LineLayout {
                    font_size,
                    width: Pixels::from(precise.width),
                    ascent: px(7.0),
                    descent: px(2.0),
                    len: text.len(),
                    caret_positions: std::iter::once((0, 0.0))
                        .chain(
                            [
                                (1, 0.0),
                                (4, 10.0),
                                (5, 20.0),
                                (11, 30.0),
                                (12, 40.0),
                                (15, 50.0),
                            ]
                            .map(|(index, position)| (index, origin + position)),
                        )
                        .collect(),
                    runs: vec![ShapedRun {
                        font_id,
                        glyphs: [(0, 1), (1, 0), (4, 1), (5, 0), (8, 0), (11, 1), (12, 0)]
                            .into_iter()
                            .zip(&precise.glyph_x_positions)
                            .map(|((index, id), position)| ShapedGlyph {
                                id: GlyphId(id),
                                index,
                                position: point(Pixels::from(*position), px(0.0)),
                                is_emoji: false,
                            })
                            .collect(),
                    }],
                };
                if origin == 0.0 {
                    let mut regular = LineLayout {
                        runs: layout.runs.clone(),
                        caret_positions: layout.caret_positions.clone(),
                        ..layout
                    };

                    text_system.apply_fallback(text, &runs, &mut regular, None);
                    assert_eq!(
                        regular.caret_positions,
                        [
                            (0, 0.0),
                            (1, 0.0),
                            (4, 12.0),
                            (5, 22.0),
                            (11, 22.0 + f64::from(kana_advance)),
                            (12, 32.0 + f64::from(kana_advance)),
                            (15, expected_width),
                        ]
                    );
                    assert_geometry(
                        &regular,
                        &std::iter::once(0.0)
                            .chain(expected_positions)
                            .collect::<Vec<_>>(),
                        expected_width,
                    );
                }
                text_system.apply_fallback(text, &runs, &mut layout, Some(&mut precise));
                assert_eq!(
                    precise.glyph_x_positions,
                    std::iter::once(0.0)
                        .chain(expected_positions.map(|position| origin + position))
                        .collect::<Vec<_>>(),
                );
                assert_eq!(precise.width, origin + expected_width);
                assert_eq!(
                    layout.caret_positions,
                    std::iter::once((0, 0.0))
                        .chain(
                            [
                                (1, 0.0),
                                (4, 12.0),
                                (5, 22.0),
                                (11, 22.0 + f64::from(kana_advance)),
                                (12, 32.0 + f64::from(kana_advance)),
                                (15, expected_width),
                            ]
                            .map(|(index, position)| (index, origin + position))
                        )
                        .collect::<Vec<_>>()
                );
                assert_eq!((layout.ascent, layout.descent), (px(7.0), px(2.0)));
                assert_eq!(
                    layout
                        .runs
                        .iter()
                        .flat_map(|run| run.glyphs.iter().map(move |glyph| (
                            run.font_id,
                            glyph.id,
                            glyph.index
                        )))
                        .collect::<Vec<_>>(),
                    [
                        (font_id, GlyphId(1), 0),
                        (canvas_font_id, ideograph, 1),
                        (font_id, GlyphId(1), 4),
                        (canvas_font_id, kana, 5),
                        (font_id, GlyphId(1), 11),
                        (canvas_font_id, ideograph, 12),
                    ],
                );
                assert_geometry(&layout, &precise.glyph_x_positions, precise.width);
            }
        }
        Ok(())
    }

    fn text_system() -> Result<(WebTextSystem, FontId)> {
        let text_system = WebTextSystem::new("Lilex", CanvasFontFallback::EmojiAndCjk);
        text_system.add_fonts(vec![Cow::Borrowed(include_bytes!(
            "../../../assets/fonts/lilex/Lilex-Regular.ttf"
        ))])?;
        let font_id = text_system.font_id(&gpui::font("Lilex"))?;
        Ok((text_system, font_id))
    }

    fn cache_fallback(
        text_system: &WebTextSystem,
        font_id: FontId,
        font_size: Pixels,
        text: &str,
        advance: f32,
    ) -> Result<(FontId, GlyphId)> {
        let font_id = text_system.canvas_font_id(font_id)?;
        let glyph_id = text_system.register_glyph(font_id, text, false)?;
        text_system.state.write().measurements.insert(
            (font_id, glyph_id, font_size),
            CanvasTextMetrics {
                advance,
                left: -1.0,
                right: advance + 1.0,
                ascent: 100.0,
                descent: 100.0,
            },
        );
        Ok((font_id, glyph_id))
    }

    fn assert_geometry(layout: &LineLayout, positions: &[f64], width: f64) {
        assert_eq!(layout.width, Pixels::from(width));
        assert_eq!(layout.caret_positions.last(), Some(&(layout.len, width)));
        assert_eq!(
            layout
                .runs
                .iter()
                .map(|run| run.glyphs.len())
                .sum::<usize>(),
            positions.len(),
        );
        for (glyph, position) in layout
            .runs
            .iter()
            .flat_map(|run| &run.glyphs)
            .zip(positions)
        {
            assert_eq!(glyph.position.x, Pixels::from(*position));
        }
    }
}
