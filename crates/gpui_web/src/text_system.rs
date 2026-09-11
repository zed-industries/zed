use crate::canvas_fallback::classify_canvas_fallback;
use crate::canvas_text::{self, CanvasTextMetrics};
use anyhow::{Context as _, Result, ensure};
use gpui::{
    Bounds, DevicePixels, Font, FontId, FontMetrics, FontRun, FontStyle, GlyphId, Hsla, LineLayout,
    Pixels, PlatformTextSystem, RenderGlyphParams, SUBPIXEL_VARIANTS_X, SUBPIXEL_VARIANTS_Y,
    ShapedGlyph, ShapedRun, SharedString, Size, TextRenderingMode, point, px, size,
};
use gpui_wgpu::CosmicTextSystem;
use parking_lot::RwLock;
use std::{borrow::Cow, collections::HashMap, ops::Range, sync::Arc};
use unicode_segmentation::UnicodeSegmentation;

// Cosmic's font IDs index its loaded-font vector. Keep browser IDs in a disjoint namespace.
const CANVAS_FONT_BIT: usize = 1 << (usize::BITS - 1);
const MAX_MEASUREMENTS: usize = 4096;

pub(crate) struct WebTextSystem {
    native: CosmicTextSystem,
    state: RwLock<State>,
}

#[derive(Default)]
struct State {
    descriptors: HashMap<FontId, Font>,
    canvas_fonts: Vec<Arc<CanvasFont>>,
    canvas_font_ids: HashMap<FontId, FontId>,
    glyphs: Vec<CanvasGlyph>,
    glyph_ids: HashMap<(FontId, SharedString, bool), GlyphId>,
    measurements: HashMap<(FontId, GlyphId, Pixels), CanvasTextMetrics>,
}

struct CanvasFont {
    native_id: FontId,
    descriptor: Font,
    monospace: bool,
}

#[derive(Clone)]
struct CanvasGlyph {
    font_id: FontId,
    text: SharedString,
    color: bool,
}

struct Candidate {
    source: Range<usize>,
    font_id: FontId,
    color: bool,
    glyph_range: Option<Range<usize>>,
    contiguous: bool,
    missing: bool,
    native_color: bool,
}

struct Replacement {
    glyph_range: Range<usize>,
    font_id: FontId,
    glyph: ShapedGlyph,
    width_delta: Pixels,
}

impl WebTextSystem {
    pub(crate) fn new(system_font_fallback: &str) -> Self {
        Self {
            native: CosmicTextSystem::new_without_system_fonts(system_font_fallback),
            state: RwLock::new(State::default()),
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
        let key = (font_id, SharedString::from(text.to_owned()), color);
        let mut state = self.state.write();
        if let Some(glyph_id) = state.glyph_ids.get(&key) {
            return Ok(*glyph_id);
        }
        let glyph_id = GlyphId(u32::try_from(state.glyphs.len())?);
        state.glyphs.push(CanvasGlyph {
            font_id,
            text: key.1.clone(),
            color,
        });
        state.glyph_ids.insert(key, glyph_id);
        Ok(glyph_id)
    }

    fn canvas_glyph(&self, font_id: FontId, glyph_id: GlyphId) -> Result<CanvasGlyph> {
        self.state
            .read()
            .glyphs
            .get(glyph_id.0 as usize)
            .filter(|glyph| glyph.font_id == font_id)
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

    fn apply_fallback(&self, text: &str, font_runs: &[FontRun], layout: &mut LineLayout) {
        if text.is_ascii() || !f32::from(layout.font_size).is_finite() || layout.font_size <= px(0.)
        {
            return;
        }
        let mut font_runs = font_runs.iter();
        let mut font_run = font_runs.next();
        let mut run_end = font_run.map_or(0, |run| run.len);
        let mut candidates = Vec::new();
        for (start, grapheme) in text.grapheme_indices(true) {
            while start >= run_end {
                font_run = font_runs.next();
                let Some(run) = font_run else { break };
                run_end += run.len;
            }
            let Some(run) = font_run else { break };
            let end = start + grapheme.len();
            if end > run_end {
                continue;
            }
            let Some(fallback) = classify_canvas_fallback(grapheme) else {
                continue;
            };
            candidates.push(Candidate {
                source: start..end,
                font_id: run.font_id,
                color: fallback.emoji_presentation,
                glyph_range: None,
                contiguous: true,
                missing: false,
                native_color: true,
            });
        }
        if candidates.is_empty() {
            return;
        }

        let glyphs: Vec<_> = layout
            .runs
            .iter()
            .flat_map(|run| run.glyphs.iter().map(move |glyph| (run.font_id, glyph)))
            .collect();
        for (index, (_, glyph)) in glyphs.iter().enumerate() {
            let candidate_index =
                candidates.partition_point(|candidate| candidate.source.end <= glyph.index);
            let Some(candidate) = candidates
                .get_mut(candidate_index)
                .filter(|candidate| candidate.source.contains(&glyph.index))
            else {
                continue;
            };
            match &mut candidate.glyph_range {
                Some(range) => {
                    candidate.contiguous &= range.end == index;
                    range.end = index + 1;
                }
                range => *range = Some(index..index + 1),
            }
            candidate.missing |= glyph.id.0 == 0;
            candidate.native_color &= glyph.is_emoji;
        }

        let mut replacements = Vec::new();
        for candidate in candidates {
            if !candidate.contiguous
                || !(candidate.missing || (candidate.color && !candidate.native_color))
            {
                continue;
            }
            let Some(glyph_range) = candidate.glyph_range else {
                continue;
            };
            let Some((_, first)) = glyphs.get(glyph_range.start) else {
                continue;
            };
            let end_x = glyphs
                .get(glyph_range.end)
                .map_or(layout.width, |(_, glyph)| glyph.position.x);
            let old_width = end_x - first.position.x;
            if old_width < px(0.) {
                continue;
            }
            let replacement = (|| -> Result<Replacement> {
                let text = text
                    .get(candidate.source.clone())
                    .context("invalid Canvas fallback source range")?;
                let font_id = self.canvas_font_id(candidate.font_id)?;
                let glyph_id = self.register_glyph(font_id, text, candidate.color)?;
                let metrics = self.measure(font_id, glyph_id, layout.font_size)?;
                Ok(Replacement {
                    glyph_range,
                    font_id,
                    glyph: ShapedGlyph {
                        id: glyph_id,
                        position: first.position,
                        index: candidate.source.start,
                        is_emoji: candidate.color,
                    },
                    width_delta: px(metrics.advance) - old_width,
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
        drop(glyphs);

        let mut replacements = replacements.into_iter().peekable();
        let mut glyphs = std::mem::take(&mut layout.runs)
            .into_iter()
            .flat_map(|run| {
                run.glyphs
                    .into_iter()
                    .map(move |glyph| (run.font_id, glyph))
            })
            .enumerate();
        let mut shift = Pixels::ZERO;
        while let Some((index, (font_id, mut glyph))) = glyphs.next() {
            if replacements
                .peek()
                .is_some_and(|replacement| replacement.glyph_range.start == index)
            {
                let Some(mut replacement) = replacements.next() else {
                    break;
                };
                for _ in index + 1..replacement.glyph_range.end {
                    glyphs.next();
                }
                replacement.glyph.position.x += shift;
                shift += replacement.width_delta;
                push_glyph(&mut layout.runs, replacement.font_id, replacement.glyph);
            } else {
                glyph.position.x += shift;
                push_glyph(&mut layout.runs, font_id, glyph);
            }
        }
        layout.width += shift;
        // Keep the primary font's baseline and line spacing stable. Browser ink
        // extents affect raster bounds, not the surrounding editor's line metrics.
    }
}

impl PlatformTextSystem for WebTextSystem {
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
        let mut native_runs: Vec<_> = runs
            .iter()
            .map(|run| FontRun {
                len: run.len,
                font_id: self.native_font_id(run.font_id),
            })
            .collect();
        let mut layout = self.native.layout_line(text, font_size, &native_runs);
        // Preserve native shaping boundaries, but do not let paint-only run splits
        // prevent fallback for one complete grapheme in the same font.
        native_runs.dedup_by(|run, previous| {
            if run.font_id != previous.font_id {
                return false;
            }
            previous.len += run.len;
            true
        });
        self.apply_fallback(text, &native_runs, &mut layout);
        layout
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

fn push_glyph(runs: &mut Vec<ShapedRun>, font_id: FontId, glyph: ShapedGlyph) {
    if let Some(run) = runs.last_mut().filter(|run| run.font_id == font_id) {
        run.glyphs.push(glyph);
    } else {
        runs.push(ShapedRun {
            font_id,
            glyphs: vec![glyph],
        });
    }
}
