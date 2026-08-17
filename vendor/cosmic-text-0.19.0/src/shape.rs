// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::too_many_arguments)]

use crate::fallback::FontFallbackIter;
use crate::{
    math, Align, Attrs, AttrsList, CacheKeyFlags, Color, DecorationMetrics, DecorationSpan,
    Ellipsize, EllipsizeHeightLimit, Family, Font, FontSystem, GlyphDecorationData, Hinting,
    LayoutGlyph, LayoutLine, Metrics, Wrap,
};
#[cfg(not(feature = "std"))]
use alloc::{format, vec, vec::Vec};

use alloc::collections::VecDeque;
use core::cmp::{max, min};
use core::fmt;
use core::mem;
use core::ops::Range;

#[cfg(not(feature = "std"))]
use core_maths::CoreFloat;
use fontdb::Style;
use unicode_script::{Script, UnicodeScript};
use unicode_segmentation::UnicodeSegmentation;

/// The shaping strategy of some text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shaping {
    /// Basic shaping with no font fallback.
    ///
    /// This shaping strategy is very cheap, but it will not display complex
    /// scripts properly nor try to find missing glyphs in your system fonts.
    ///
    /// You should use this strategy when you have complete control of the text
    /// and the font you are displaying in your application.
    #[cfg(feature = "swash")]
    Basic,
    /// Advanced text shaping and font fallback.
    ///
    /// You will need to enable this strategy if the text contains a complex
    /// script, the font used needs it, and/or multiple fonts in your system
    /// may be needed to display all of the glyphs.
    Advanced,
}

impl Shaping {
    fn run(
        self,
        glyphs: &mut Vec<ShapeGlyph>,
        font_system: &mut FontSystem,
        line: &str,
        attrs_list: &AttrsList,
        start_run: usize,
        end_run: usize,
        span_rtl: bool,
    ) {
        match self {
            #[cfg(feature = "swash")]
            Self::Basic => shape_skip(font_system, glyphs, line, attrs_list, start_run, end_run),
            #[cfg(not(feature = "shape-run-cache"))]
            Self::Advanced => shape_run(
                glyphs,
                font_system,
                line,
                attrs_list,
                start_run,
                end_run,
                span_rtl,
            ),
            #[cfg(feature = "shape-run-cache")]
            Self::Advanced => shape_run_cached(
                glyphs,
                font_system,
                line,
                attrs_list,
                start_run,
                end_run,
                span_rtl,
            ),
        }
    }
}

const NUM_SHAPE_PLANS: usize = 6;

/// A set of buffers containing allocations for shaped text.
#[derive(Default)]
pub struct ShapeBuffer {
    /// Cache for harfrust shape plans. Stores up to [`NUM_SHAPE_PLANS`] plans at once. Inserting a new one past that
    /// will remove the one that was least recently added (not least recently used).
    shape_plan_cache: VecDeque<(fontdb::ID, harfrust::ShapePlan)>,

    /// Buffer for holding unicode text.
    harfrust_buffer: Option<harfrust::UnicodeBuffer>,

    /// Temporary buffers for scripts.
    scripts: Vec<Script>,

    /// Buffer for shape spans.
    spans: Vec<ShapeSpan>,

    /// Buffer for shape words.
    words: Vec<ShapeWord>,

    /// Buffers for visual lines.
    visual_lines: Vec<VisualLine>,
    cached_visual_lines: Vec<VisualLine>,

    /// Buffer for sets of layout glyphs.
    glyph_sets: Vec<Vec<LayoutGlyph>>,
}

impl fmt::Debug for ShapeBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("ShapeBuffer { .. }")
    }
}

fn shape_fallback(
    scratch: &mut ShapeBuffer,
    glyphs: &mut Vec<ShapeGlyph>,
    font: &Font,
    line: &str,
    attrs_list: &AttrsList,
    start_run: usize,
    end_run: usize,
    span_rtl: bool,
) -> Vec<usize> {
    let run = &line[start_run..end_run];

    let font_scale = font.metrics().units_per_em as f32;
    let ascent = font.metrics().ascent / font_scale;
    let descent = -font.metrics().descent / font_scale;

    let mut buffer = scratch.harfrust_buffer.take().unwrap_or_default();
    buffer.set_direction(if span_rtl {
        harfrust::Direction::RightToLeft
    } else {
        harfrust::Direction::LeftToRight
    });
    if run.contains('\t') {
        // Push string to buffer, replacing tabs with spaces
        //TODO: Find a way to do this with minimal allocating, calling
        // UnicodeBuffer::push_str multiple times causes issues and
        // UnicodeBuffer::add resizes the buffer with every character
        buffer.push_str(&run.replace('\t', " "));
    } else {
        buffer.push_str(run);
    }
    buffer.guess_segment_properties();

    let rtl = matches!(buffer.direction(), harfrust::Direction::RightToLeft);
    assert_eq!(rtl, span_rtl);

    let attrs = attrs_list.get_span(start_run);
    let mut rb_font_features = Vec::new();

    // Convert attrs::Feature to harfrust::Feature
    for feature in &attrs.font_features.features {
        rb_font_features.push(harfrust::Feature::new(
            harfrust::Tag::new(feature.tag.as_bytes()),
            feature.value,
            0..usize::MAX,
        ));
    }

    let language = buffer.language();
    let key = harfrust::ShapePlanKey::new(Some(buffer.script()), buffer.direction())
        .features(&rb_font_features)
        .instance(Some(font.shaper_instance()))
        .language(language.as_ref());

    let shape_plan = match scratch
        .shape_plan_cache
        .iter()
        .find(|(id, plan)| *id == font.id() && key.matches(plan))
    {
        Some((_font_id, plan)) => plan,
        None => {
            let plan = harfrust::ShapePlan::new(
                font.shaper(),
                buffer.direction(),
                Some(buffer.script()),
                buffer.language().as_ref(),
                &rb_font_features,
            );
            if scratch.shape_plan_cache.len() >= NUM_SHAPE_PLANS {
                scratch.shape_plan_cache.pop_front();
            }
            scratch.shape_plan_cache.push_back((font.id(), plan));
            &scratch
                .shape_plan_cache
                .back()
                .expect("we just pushed the shape plan")
                .1
        }
    };

    let glyph_buffer = font
        .shaper()
        .shape_with_plan(shape_plan, buffer, &rb_font_features);
    let glyph_infos = glyph_buffer.glyph_infos();
    let glyph_positions = glyph_buffer.glyph_positions();

    let mut missing = Vec::new();
    glyphs.reserve(glyph_infos.len());
    let glyph_start = glyphs.len();
    for (info, pos) in glyph_infos.iter().zip(glyph_positions.iter()) {
        let start_glyph = start_run + info.cluster as usize;

        if info.glyph_id == 0 {
            missing.push(start_glyph);
        }

        let attrs = attrs_list.get_span(start_glyph);
        let x_advance = pos.x_advance as f32 / font_scale
            + attrs.letter_spacing_opt.map_or(0.0, |spacing| spacing.0);
        let y_advance = pos.y_advance as f32 / font_scale;
        let x_offset = pos.x_offset as f32 / font_scale;
        let y_offset = pos.y_offset as f32 / font_scale;

        glyphs.push(ShapeGlyph {
            start: start_glyph,
            end: end_run, // Set later
            x_advance,
            y_advance,
            x_offset,
            y_offset,
            ascent,
            descent,
            font_monospace_em_width: font.monospace_em_width(),
            font_id: font.id(),
            font_weight: attrs.weight,
            glyph_id: info.glyph_id.try_into().expect("failed to cast glyph ID"),
            //TODO: color should not be related to shaping
            color_opt: attrs.color_opt,
            metadata: attrs.metadata,
            cache_key_flags: override_fake_italic(attrs.cache_key_flags, font, &attrs),
            metrics_opt: attrs.metrics_opt.map(Into::into),
        });
    }

    // Adjust end of glyphs
    if rtl {
        for i in glyph_start + 1..glyphs.len() {
            let next_start = glyphs[i - 1].start;
            let next_end = glyphs[i - 1].end;
            let prev = &mut glyphs[i];
            if prev.start == next_start {
                prev.end = next_end;
            } else {
                prev.end = next_start;
            }
        }
    } else {
        for i in (glyph_start + 1..glyphs.len()).rev() {
            let next_start = glyphs[i].start;
            let next_end = glyphs[i].end;
            let prev = &mut glyphs[i - 1];
            if prev.start == next_start {
                prev.end = next_end;
            } else {
                prev.end = next_start;
            }
        }
    }

    // Restore the buffer to save an allocation.
    scratch.harfrust_buffer = Some(glyph_buffer.clear());

    missing
}

fn shape_run(
    glyphs: &mut Vec<ShapeGlyph>,
    font_system: &mut FontSystem,
    line: &str,
    attrs_list: &AttrsList,
    start_run: usize,
    end_run: usize,
    span_rtl: bool,
) {
    // Re-use the previous script buffer if possible.
    let mut scripts = {
        let mut scripts = mem::take(&mut font_system.shape_buffer.scripts);
        scripts.clear();
        scripts
    };
    for c in line[start_run..end_run].chars() {
        match c.script() {
            Script::Common | Script::Inherited | Script::Latin | Script::Unknown => (),
            script => {
                if !scripts.contains(&script) {
                    scripts.push(script);
                }
            }
        }
    }

    log::trace!("      Run {:?}: '{}'", &scripts, &line[start_run..end_run],);

    let attrs = attrs_list.get_span(start_run);

    let fonts = font_system.get_font_matches(&attrs);

    let default_families = [&attrs.family];
    let mut font_iter = FontFallbackIter::new(
        font_system,
        &fonts,
        &default_families,
        &scripts,
        &line[start_run..end_run],
        attrs.weight,
    );

    let font = font_iter.next().expect("no default font found");

    let glyph_start = glyphs.len();
    let mut missing = {
        let scratch = font_iter.shape_caches();
        shape_fallback(
            scratch, glyphs, &font, line, attrs_list, start_run, end_run, span_rtl,
        )
    };

    //TODO: improve performance!
    while !missing.is_empty() {
        let Some(font) = font_iter.next() else {
            break;
        };

        log::trace!(
            "Evaluating fallback with font '{}'",
            font_iter.face_name(font.id())
        );
        let mut fb_glyphs = Vec::new();
        let scratch = font_iter.shape_caches();
        let fb_missing = shape_fallback(
            scratch,
            &mut fb_glyphs,
            &font,
            line,
            attrs_list,
            start_run,
            end_run,
            span_rtl,
        );

        // Insert all matching glyphs
        let mut fb_i = 0;
        while fb_i < fb_glyphs.len() {
            let start = fb_glyphs[fb_i].start;
            let end = fb_glyphs[fb_i].end;

            // Skip clusters that are not missing, or where the fallback font is missing
            if !missing.contains(&start) || fb_missing.contains(&start) {
                fb_i += 1;
                continue;
            }

            let mut missing_i = 0;
            while missing_i < missing.len() {
                if missing[missing_i] >= start && missing[missing_i] < end {
                    // println!("No longer missing {}", missing[missing_i]);
                    missing.remove(missing_i);
                } else {
                    missing_i += 1;
                }
            }

            // Find prior glyphs
            let mut i = glyph_start;
            while i < glyphs.len() {
                if glyphs[i].start >= start && glyphs[i].end <= end {
                    break;
                }
                i += 1;
            }

            // Remove prior glyphs
            while i < glyphs.len() {
                if glyphs[i].start >= start && glyphs[i].end <= end {
                    let _glyph = glyphs.remove(i);
                    // log::trace!("Removed {},{} from {}", _glyph.start, _glyph.end, i);
                } else {
                    break;
                }
            }

            while fb_i < fb_glyphs.len() {
                if fb_glyphs[fb_i].start >= start && fb_glyphs[fb_i].end <= end {
                    let fb_glyph = fb_glyphs.remove(fb_i);
                    // log::trace!("Insert {},{} from font {} at {}", fb_glyph.start, fb_glyph.end, font_i, i);
                    glyphs.insert(i, fb_glyph);
                    i += 1;
                } else {
                    break;
                }
            }
        }
    }

    // Debug missing font fallbacks
    font_iter.check_missing(&line[start_run..end_run]);

    /*
    for glyph in glyphs.iter() {
        log::trace!("'{}': {}, {}, {}, {}", &line[glyph.start..glyph.end], glyph.x_advance, glyph.y_advance, glyph.x_offset, glyph.y_offset);
    }
    */

    // Restore the scripts buffer.
    font_system.shape_buffer.scripts = scripts;
}

#[cfg(feature = "shape-run-cache")]
fn shape_run_cached(
    glyphs: &mut Vec<ShapeGlyph>,
    font_system: &mut FontSystem,
    line: &str,
    attrs_list: &AttrsList,
    start_run: usize,
    end_run: usize,
    span_rtl: bool,
) {
    use crate::{AttrsOwned, ShapeRunKey};

    let run_range = start_run..end_run;
    let mut key = ShapeRunKey {
        text: line[run_range.clone()].to_string(),
        default_attrs: AttrsOwned::new(&attrs_list.defaults()),
        attrs_spans: Vec::new(),
    };
    for (attrs_range, attrs) in attrs_list.spans.overlapping(&run_range) {
        if attrs == &key.default_attrs {
            // Skip if attrs matches default attrs
            continue;
        }
        let start = max(attrs_range.start, start_run).saturating_sub(start_run);
        let end = min(attrs_range.end, end_run).saturating_sub(start_run);
        if end > start {
            let range = start..end;
            key.attrs_spans.push((range, attrs.clone()));
        }
    }
    if let Some(cache_glyphs) = font_system.shape_run_cache.get(&key) {
        for mut glyph in cache_glyphs.iter().cloned() {
            // Adjust glyph start and end to match run position
            glyph.start += start_run;
            glyph.end += start_run;
            glyphs.push(glyph);
        }
        return;
    }

    // Fill in cache if not already set
    let mut cache_glyphs = Vec::new();
    shape_run(
        &mut cache_glyphs,
        font_system,
        line,
        attrs_list,
        start_run,
        end_run,
        span_rtl,
    );
    glyphs.extend_from_slice(&cache_glyphs);
    for glyph in cache_glyphs.iter_mut() {
        // Adjust glyph start and end to remove run position
        glyph.start -= start_run;
        glyph.end -= start_run;
    }
    font_system.shape_run_cache.insert(key, cache_glyphs);
}

#[cfg(feature = "swash")]
fn shape_skip(
    font_system: &mut FontSystem,
    glyphs: &mut Vec<ShapeGlyph>,
    line: &str,
    attrs_list: &AttrsList,
    start_run: usize,
    end_run: usize,
) {
    let attrs = attrs_list.get_span(start_run);
    let fonts = font_system.get_font_matches(&attrs);

    let default_families = [&attrs.family];
    let mut font_iter = FontFallbackIter::new(
        font_system,
        &fonts,
        &default_families,
        &[],
        "",
        attrs.weight,
    );

    let font = font_iter.next().expect("no default font found");
    let glyph_start = glyphs.len();

    shape_skip_glyphs(glyphs, &font, line, attrs_list, start_run, end_run);

    // If any glyphs are missing and the user has specified a font,
    // fall back to a default font (SansSerif or Monospace)
    if matches!(attrs.family, Family::Name(_))
        && glyphs[glyph_start..].iter().any(|g| g.glyph_id == 0)
    {
        let is_mono = font_system
            .db()
            .face(font.id())
            .is_some_and(|face| face.monospaced);
        let fb_family = if is_mono {
            Family::Monospace
        } else {
            Family::SansSerif
        };
        let fb_attrs = Attrs::new()
            .family(fb_family)
            .weight(attrs.weight)
            .style(attrs.style)
            .stretch(attrs.stretch);
        let fb_fonts = font_system.get_font_matches(&fb_attrs);
        let fb_families = [&fb_family];
        let mut fb_iter =
            FontFallbackIter::new(font_system, &fb_fonts, &fb_families, &[], "", attrs.weight);

        if let Some(fb_font) = fb_iter.next() {
            let fb_swash = fb_font.as_swash();
            let fb_charmap = fb_swash.charmap();
            let fb_metrics = fb_swash.metrics(&[]);
            let fb_glyph_metrics = fb_swash.glyph_metrics(&[]).scale(1.0);
            let fb_scale = f32::from(fb_metrics.units_per_em);

            for glyph in glyphs[glyph_start..].iter_mut() {
                if glyph.glyph_id != 0 {
                    continue;
                }
                let codepoint = line[glyph.start..glyph.end].chars().next().unwrap_or('\0');
                let glyph_id = fb_charmap.map(codepoint);
                if glyph_id != 0 {
                    let span_attrs = attrs_list.get_span(glyph.start);
                    glyph.glyph_id = glyph_id;
                    glyph.font_id = fb_font.id();
                    glyph.font_monospace_em_width = fb_font.monospace_em_width();
                    glyph.ascent = fb_metrics.ascent / fb_scale;
                    glyph.descent = fb_metrics.descent / fb_scale;
                    glyph.x_advance = fb_glyph_metrics.advance_width(glyph_id)
                        + span_attrs
                            .letter_spacing_opt
                            .map_or(0.0, |spacing| spacing.0);
                    glyph.cache_key_flags = override_fake_italic(
                        span_attrs.cache_key_flags,
                        fb_font.as_ref(),
                        &span_attrs,
                    );
                }
            }
        }
    }
}

#[cfg(feature = "swash")]
fn shape_skip_glyphs(
    glyphs: &mut Vec<ShapeGlyph>,
    font: &Font,
    line: &str,
    attrs_list: &AttrsList,
    start_run: usize,
    end_run: usize,
) {
    let font_id = font.id();
    let font_monospace_em_width = font.monospace_em_width();
    let swash_font = font.as_swash();

    let charmap = swash_font.charmap();
    let metrics = swash_font.metrics(&[]);
    let glyph_metrics = swash_font.glyph_metrics(&[]).scale(1.0);

    let ascent = metrics.ascent / f32::from(metrics.units_per_em);
    let descent = metrics.descent / f32::from(metrics.units_per_em);

    glyphs.extend(
        line[start_run..end_run]
            .char_indices()
            .map(|(chr_idx, codepoint)| {
                let glyph_id = charmap.map(codepoint);
                let x_advance = glyph_metrics.advance_width(glyph_id)
                    + attrs_list
                        .get_span(start_run + chr_idx)
                        .letter_spacing_opt
                        .map_or(0.0, |spacing| spacing.0);
                let attrs = attrs_list.get_span(start_run + chr_idx);

                ShapeGlyph {
                    start: chr_idx + start_run,
                    end: chr_idx + start_run + codepoint.len_utf8(),
                    x_advance,
                    y_advance: 0.0,
                    x_offset: 0.0,
                    y_offset: 0.0,
                    ascent,
                    descent,
                    font_monospace_em_width,
                    font_id,
                    font_weight: attrs.weight,
                    glyph_id,
                    color_opt: attrs.color_opt,
                    metadata: attrs.metadata,
                    cache_key_flags: override_fake_italic(attrs.cache_key_flags, font, &attrs),
                    metrics_opt: attrs.metrics_opt.map(Into::into),
                }
            }),
    );
}

fn override_fake_italic(
    cache_key_flags: CacheKeyFlags,
    font: &Font,
    attrs: &Attrs,
) -> CacheKeyFlags {
    if !font.italic_or_oblique && (attrs.style == Style::Italic || attrs.style == Style::Oblique) {
        cache_key_flags | CacheKeyFlags::FAKE_ITALIC
    } else {
        cache_key_flags
    }
}

/// A shaped glyph
#[derive(Clone, Debug)]
pub struct ShapeGlyph {
    pub start: usize,
    pub end: usize,
    pub x_advance: f32,
    pub y_advance: f32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub ascent: f32,
    pub descent: f32,
    pub font_monospace_em_width: Option<f32>,
    pub font_id: fontdb::ID,
    pub font_weight: fontdb::Weight,
    pub glyph_id: u16,
    pub color_opt: Option<Color>,
    pub metadata: usize,
    pub cache_key_flags: CacheKeyFlags,
    pub metrics_opt: Option<Metrics>,
}

impl ShapeGlyph {
    const fn layout(
        &self,
        font_size: f32,
        line_height_opt: Option<f32>,
        x: f32,
        y: f32,
        w: f32,
        level: unicode_bidi::Level,
    ) -> LayoutGlyph {
        LayoutGlyph {
            start: self.start,
            end: self.end,
            font_size,
            line_height_opt,
            font_id: self.font_id,
            font_weight: self.font_weight,
            glyph_id: self.glyph_id,
            x,
            y,
            w,
            level,
            x_offset: self.x_offset,
            y_offset: self.y_offset,
            color_opt: self.color_opt,
            metadata: self.metadata,
            cache_key_flags: self.cache_key_flags,
        }
    }

    /// Get the width of the [`ShapeGlyph`] in pixels, either using the provided font size
    /// or the [`ShapeGlyph::metrics_opt`] override.
    pub fn width(&self, font_size: f32) -> f32 {
        self.metrics_opt.map_or(font_size, |x| x.font_size) * self.x_advance
    }
}

fn decoration_metrics(font: &Font) -> (DecorationMetrics, DecorationMetrics, f32) {
    let metrics = font.metrics();
    let upem = metrics.units_per_em as f32;
    if upem == 0.0 {
        return (
            DecorationMetrics::default(),
            DecorationMetrics::default(),
            0.0,
        );
    }
    (
        DecorationMetrics {
            offset: metrics.underline.map_or(-0.125, |d| d.offset / upem),
            thickness: metrics.underline.map_or(1.0 / 14.0, |d| d.thickness / upem),
        },
        DecorationMetrics {
            offset: metrics.strikeout.map_or(0.3, |d| d.offset / upem),
            thickness: metrics.strikeout.map_or(1.0 / 14.0, |d| d.thickness / upem),
        },
        metrics.ascent / upem,
    )
}

/// span index used in `VlRange` to indicate this range is the ellipsis.
const ELLIPSIS_SPAN: usize = usize::MAX;

fn shape_ellipsis(
    font_system: &mut FontSystem,
    attrs: &Attrs,
    shaping: Shaping,
    span_rtl: bool,
) -> Vec<ShapeGlyph> {
    let attrs_list = AttrsList::new(attrs);
    let level = if span_rtl {
        unicode_bidi::Level::rtl()
    } else {
        unicode_bidi::Level::ltr()
    };
    let word = ShapeWord::new(
        font_system,
        "\u{2026}", // TODO: maybe do CJK ellipsis
        &attrs_list,
        0.."\u{2026}".len(),
        level,
        false,
        shaping,
    );
    let mut glyphs = word.glyphs;

    // did we fail to shape it?
    if glyphs.is_empty() || glyphs.iter().all(|g| g.glyph_id == 0) {
        let fallback = ShapeWord::new(
            font_system,
            "...",
            &attrs_list,
            0.."...".len(),
            level,
            false,
            shaping,
        );
        glyphs = fallback.glyphs;
    }
    glyphs
}

/// A shaped word (for word wrapping)
#[derive(Clone, Debug)]
pub struct ShapeWord {
    pub blank: bool,
    pub glyphs: Vec<ShapeGlyph>,
}

impl ShapeWord {
    /// Creates an empty word.
    ///
    /// The returned word is in an invalid state until [`Self::build_in_buffer`] is called.
    pub(crate) fn empty() -> Self {
        Self {
            blank: true,
            glyphs: Vec::default(),
        }
    }

    /// Shape a word into a set of glyphs.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        font_system: &mut FontSystem,
        line: &str,
        attrs_list: &AttrsList,
        word_range: Range<usize>,
        level: unicode_bidi::Level,
        blank: bool,
        shaping: Shaping,
    ) -> Self {
        let mut empty = Self::empty();
        empty.build(
            font_system,
            line,
            attrs_list,
            word_range,
            level,
            blank,
            shaping,
        );
        empty
    }

    /// See [`Self::new`].
    ///
    /// Reuses as much of the pre-existing internal allocations as possible.
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        &mut self,
        font_system: &mut FontSystem,
        line: &str,
        attrs_list: &AttrsList,
        word_range: Range<usize>,
        level: unicode_bidi::Level,
        blank: bool,
        shaping: Shaping,
    ) {
        let word = &line[word_range.clone()];

        log::trace!(
            "      Word{}: '{}'",
            if blank { " BLANK" } else { "" },
            word
        );

        let mut glyphs = mem::take(&mut self.glyphs);
        glyphs.clear();

        let span_rtl = level.is_rtl();

        // Fast path optimization: For simple ASCII words, skip expensive grapheme iteration
        let is_simple_ascii =
            word.is_ascii() && !word.chars().any(|c| c.is_ascii_control() && c != '\t');

        if is_simple_ascii && !word.is_empty() && {
            let attrs_start = attrs_list.get_span(word_range.start);
            attrs_list.spans_iter().all(|(other_range, other_attrs)| {
                word_range.end <= other_range.start
                    || other_range.end <= word_range.start
                    || attrs_start.compatible(&other_attrs.as_attrs())
            })
        } {
            shaping.run(
                &mut glyphs,
                font_system,
                line,
                attrs_list,
                word_range.start,
                word_range.end,
                span_rtl,
            );
        } else {
            // Complex text path: Full grapheme iteration and attribute processing
            let mut start_run = word_range.start;
            let mut attrs = attrs_list.defaults();
            for (egc_i, _egc) in word.grapheme_indices(true) {
                let start_egc = word_range.start + egc_i;
                let attrs_egc = attrs_list.get_span(start_egc);
                if !attrs.compatible(&attrs_egc) {
                    shaping.run(
                        &mut glyphs,
                        font_system,
                        line,
                        attrs_list,
                        start_run,
                        start_egc,
                        span_rtl,
                    );

                    start_run = start_egc;
                    attrs = attrs_egc;
                }
            }
            if start_run < word_range.end {
                shaping.run(
                    &mut glyphs,
                    font_system,
                    line,
                    attrs_list,
                    start_run,
                    word_range.end,
                    span_rtl,
                );
            }
        }

        self.blank = blank;
        self.glyphs = glyphs;
    }

    /// Get the width of the [`ShapeWord`] in pixels, using the [`ShapeGlyph::width`] function.
    pub fn width(&self, font_size: f32) -> f32 {
        let mut width = 0.0;
        for glyph in &self.glyphs {
            width += glyph.width(font_size);
        }
        width
    }
}

/// A shaped span (for bidirectional processing)
#[derive(Clone, Debug)]
pub struct ShapeSpan {
    pub level: unicode_bidi::Level,
    pub words: Vec<ShapeWord>,
    /// Decoration data per user-level attr span within this shape span.
    /// Each entry maps a byte range to its decoration config and font metrics.
    /// Empty when no decorations are active.
    pub decoration_spans: Vec<(Range<usize>, GlyphDecorationData)>,
}

impl ShapeSpan {
    /// Creates an empty span.
    ///
    /// The returned span is in an invalid state until [`Self::build_in_buffer`] is called.
    pub(crate) fn empty() -> Self {
        Self {
            level: unicode_bidi::Level::ltr(),
            words: Vec::default(),
            decoration_spans: Vec::new(),
        }
    }

    /// Shape a span into a set of words.
    pub fn new(
        font_system: &mut FontSystem,
        line: &str,
        attrs_list: &AttrsList,
        span_range: Range<usize>,
        line_rtl: bool,
        level: unicode_bidi::Level,
        shaping: Shaping,
    ) -> Self {
        let mut empty = Self::empty();
        empty.build(
            font_system,
            line,
            attrs_list,
            span_range,
            line_rtl,
            level,
            shaping,
        );
        empty
    }

    /// See [`Self::new`].
    ///
    /// Reuses as much of the pre-existing internal allocations as possible.
    pub fn build(
        &mut self,
        font_system: &mut FontSystem,
        line: &str,
        attrs_list: &AttrsList,
        span_range: Range<usize>,
        line_rtl: bool,
        level: unicode_bidi::Level,
        shaping: Shaping,
    ) {
        let span = &line[span_range.start..span_range.end];

        log::trace!(
            "  Span {}: '{}'",
            if level.is_rtl() { "RTL" } else { "LTR" },
            span
        );

        let mut words = mem::take(&mut self.words);

        // Cache the shape words in reverse order so they can be popped for reuse in the same order.
        let mut cached_words = mem::take(&mut font_system.shape_buffer.words);
        cached_words.clear();
        if line_rtl != level.is_rtl() {
            // Un-reverse previous words so the internal glyph counts match accurately when rewriting memory.
            cached_words.append(&mut words);
        } else {
            cached_words.extend(words.drain(..).rev());
        }

        let mut start_word = 0;
        for (end_lb, _) in unicode_linebreak::linebreaks(span) {
            // Check if this break opportunity splits a likely ligature (e.g. "|>" or "!=")
            if end_lb > 0 && end_lb < span.len() {
                let start_idx = span_range.start;
                let pre_char = span[..end_lb].chars().last();
                let post_char = span[end_lb..].chars().next();

                if let (Some(c1), Some(c2)) = (pre_char, post_char) {
                    // Only probe if both are punctuation (optimization for coding ligatures)
                    if c1.is_ascii_punctuation() && c2.is_ascii_punctuation() {
                        let probe_text = format!("{}{}", c1, c2);
                        let attrs = attrs_list.get_span(start_idx + end_lb);
                        let fonts = font_system.get_font_matches(&attrs);
                        let default_families = [&attrs.family];

                        let mut font_iter = FontFallbackIter::new(
                            font_system,
                            &fonts,
                            &default_families,
                            &[],
                            &probe_text,
                            attrs.weight,
                        );

                        if let Some(font) = font_iter.next() {
                            let mut glyphs = Vec::new();
                            let scratch = font_iter.shape_caches();
                            shape_fallback(
                                scratch,
                                &mut glyphs,
                                &font,
                                &probe_text,
                                attrs_list,
                                0,
                                probe_text.len(),
                                false,
                            );

                            // 1. If we have fewer glyphs than chars, it's definitely a ligature (e.g. -> becoming 1 arrow).
                            if glyphs.len() < probe_text.chars().count() {
                                continue;
                            }

                            // 2. If we have the same number of glyphs, they might be contextual alternates (e.g. |> becoming 2 special glyphs).
                            // Check if the glyphs match the standard "cmap" (character to glyph) mapping.
                            // If they differ, the shaper substituted them, so we should keep them together.
                            #[cfg(feature = "swash")]
                            if glyphs.len() == probe_text.chars().count() {
                                let charmap = font.as_swash().charmap();
                                let mut is_modified = false;
                                for (i, c) in probe_text.chars().enumerate() {
                                    let std_id = charmap.map(c);
                                    if glyphs[i].glyph_id != std_id {
                                        is_modified = true;
                                        break;
                                    }
                                }

                                if is_modified {
                                    // Ligature/Contextual Alternate detected!
                                    continue;
                                }
                            }
                        }
                    }
                }
            }

            let mut start_lb = end_lb;
            for (i, c) in span[start_word..end_lb].char_indices().rev() {
                // TODO: Not all whitespace characters are linebreakable, e.g. 00A0 (No-break
                // space)
                // https://www.unicode.org/reports/tr14/#GL
                // https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt
                if c.is_whitespace() {
                    start_lb = start_word + i;
                } else {
                    break;
                }
            }
            if start_word < start_lb {
                let mut word = cached_words.pop().unwrap_or_else(ShapeWord::empty);
                word.build(
                    font_system,
                    line,
                    attrs_list,
                    (span_range.start + start_word)..(span_range.start + start_lb),
                    level,
                    false,
                    shaping,
                );
                words.push(word);
            }
            if start_lb < end_lb {
                for (i, c) in span[start_lb..end_lb].char_indices() {
                    // assert!(c.is_whitespace());
                    let mut word = cached_words.pop().unwrap_or_else(ShapeWord::empty);
                    word.build(
                        font_system,
                        line,
                        attrs_list,
                        (span_range.start + start_lb + i)
                            ..(span_range.start + start_lb + i + c.len_utf8()),
                        level,
                        true,
                        shaping,
                    );
                    words.push(word);
                }
            }
            start_word = end_lb;
        }

        // Reverse glyphs in RTL lines
        if line_rtl {
            for word in &mut words {
                word.glyphs.reverse();
            }
        }

        // Reverse words in spans that do not match line direction
        if line_rtl != level.is_rtl() {
            words.reverse();
        }

        self.level = level;
        self.words = words;

        // Build decoration spans: one entry per user-level attr span that has
        // decorations within this shape span's byte range.  Font metrics come from
        // the primary font (first shaped glyph), following Pango convention.
        self.decoration_spans.clear();

        // Early-out: skip font lookup and span iteration when no decorations exist.
        // For plain text (the common case) this is a single bool check.
        let any_decoration = attrs_list.defaults().text_decoration.has_decoration()
            || attrs_list.spans_iter().any(|(range, attr_owned)| {
                let start = range.start.max(span_range.start);
                let end = range.end.min(span_range.end);
                start < end && attr_owned.as_attrs().text_decoration.has_decoration()
            });

        if any_decoration {
            // Get font metrics once from the primary glyph of this shape span
            let primary_metrics = self
                .words
                .iter()
                .flat_map(|w| w.glyphs.first())
                .next()
                .and_then(|glyph| {
                    font_system
                        .get_font(glyph.font_id, glyph.font_weight)
                        .map(|font| decoration_metrics(&font))
                });

            if let Some((ul_metrics, st_metrics, ascent)) = primary_metrics {
                // Track which sub-ranges of span_range are covered by explicit spans
                let mut covered_end = span_range.start;

                for (range, attr_owned) in attrs_list.spans_iter() {
                    // Compute intersection with our shape span's byte range
                    let start = range.start.max(span_range.start);
                    let end = range.end.min(span_range.end);
                    if start >= end {
                        continue;
                    }

                    // Check the gap before this span (covered by defaults)
                    if covered_end < start {
                        let default_attrs = attrs_list.defaults();
                        if default_attrs.text_decoration.has_decoration() {
                            self.decoration_spans.push((
                                covered_end..start,
                                GlyphDecorationData {
                                    text_decoration: default_attrs.text_decoration,
                                    underline_metrics: ul_metrics,
                                    strikethrough_metrics: st_metrics,
                                    ascent,
                                },
                            ));
                        }
                    }
                    covered_end = end;

                    let attrs = attr_owned.as_attrs();
                    if attrs.text_decoration.has_decoration() {
                        self.decoration_spans.push((
                            start..end,
                            GlyphDecorationData {
                                text_decoration: attrs.text_decoration,
                                underline_metrics: ul_metrics,
                                strikethrough_metrics: st_metrics,
                                ascent,
                            },
                        ));
                    }
                }

                // Check trailing gap (covered by defaults)
                if covered_end < span_range.end {
                    let default_attrs = attrs_list.defaults();
                    if default_attrs.text_decoration.has_decoration() {
                        self.decoration_spans.push((
                            covered_end..span_range.end,
                            GlyphDecorationData {
                                text_decoration: default_attrs.text_decoration,
                                underline_metrics: ul_metrics,
                                strikethrough_metrics: st_metrics,
                                ascent,
                            },
                        ));
                    }
                }
            }
        }

        // Cache buffer for future reuse.
        font_system.shape_buffer.words = cached_words;
    }
}

/// A shaped line (or paragraph)
#[derive(Clone, Debug)]
pub struct ShapeLine {
    pub rtl: bool,
    pub spans: Vec<ShapeSpan>,
    pub metrics_opt: Option<Metrics>,
    ellipsis_span: Option<ShapeSpan>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct WordGlyphPos {
    word: usize,
    glyph: usize,
}

impl WordGlyphPos {
    const ZERO: Self = Self { word: 0, glyph: 0 };
    fn new(word: usize, glyph: usize) -> Self {
        Self { word, glyph }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct SpanWordGlyphPos {
    span: usize,
    word: usize,
    glyph: usize,
}

impl SpanWordGlyphPos {
    const ZERO: Self = Self {
        span: 0,
        word: 0,
        glyph: 0,
    };
    fn word_glyph_pos(&self) -> WordGlyphPos {
        WordGlyphPos {
            word: self.word,
            glyph: self.glyph,
        }
    }
    fn with_wordglyph(span: usize, wordglyph: WordGlyphPos) -> Self {
        Self {
            span,
            word: wordglyph.word,
            glyph: wordglyph.glyph,
        }
    }
}

/// Controls whether we layout spans forward or backward.
/// Backward layout is used to improve efficiency
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum LayoutDirection {
    Forward,
    Backward,
}

// Visual Line Ranges
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct VlRange {
    span: usize,
    start: WordGlyphPos,
    end: WordGlyphPos,
    level: unicode_bidi::Level,
}

impl Default for VlRange {
    fn default() -> Self {
        Self {
            span: Default::default(),
            start: Default::default(),
            end: Default::default(),
            level: unicode_bidi::Level::ltr(),
        }
    }
}

#[derive(Default, Debug)]
struct VisualLine {
    ranges: Vec<VlRange>,
    spaces: u32,
    w: f32,
    ellipsized: bool,
    /// Byte range (start, end) of the original line text that was replaced by the ellipsis.
    /// Only set when `ellipsized` is true.
    elided_byte_range: Option<(usize, usize)>,
}

impl VisualLine {
    fn clear(&mut self) {
        self.ranges.clear();
        self.spaces = 0;
        self.w = 0.;
        self.ellipsized = false;
        self.elided_byte_range = None;
    }
}

impl ShapeLine {
    /// Creates an empty line.
    ///
    /// The returned line is in an invalid state until [`Self::build_in_buffer`] is called.
    pub(crate) fn empty() -> Self {
        Self {
            rtl: false,
            spans: Vec::default(),
            metrics_opt: None,
            ellipsis_span: None,
        }
    }

    /// Shape a line into a set of spans, using a scratch buffer. If [`unicode_bidi::BidiInfo`]
    /// detects multiple paragraphs, they will be joined.
    ///
    /// # Panics
    ///
    /// Will panic if `line` contains multiple paragraphs that do not have matching direction
    pub fn new(
        font_system: &mut FontSystem,
        line: &str,
        attrs_list: &AttrsList,
        shaping: Shaping,
        tab_width: u16,
    ) -> Self {
        let mut empty = Self::empty();
        empty.build(font_system, line, attrs_list, shaping, tab_width);
        empty
    }

    /// See [`Self::new`].
    ///
    /// Reuses as much of the pre-existing internal allocations as possible.
    ///
    /// # Panics
    ///
    /// Will panic if `line` contains multiple paragraphs that do not have matching direction
    pub fn build(
        &mut self,
        font_system: &mut FontSystem,
        line: &str,
        attrs_list: &AttrsList,
        shaping: Shaping,
        tab_width: u16,
    ) {
        // Clear stale ellipsis span so it gets recomputed with the current attrs.
        // Without this, reusing a ShapeLine from a previous text (via Cached::Unused)
        // would keep an ellipsis shaped with the old attrs.
        self.ellipsis_span = None;

        let mut spans = mem::take(&mut self.spans);

        // Cache the shape spans in reverse order so they can be popped for reuse in the same order.
        let mut cached_spans = mem::take(&mut font_system.shape_buffer.spans);
        cached_spans.clear();
        cached_spans.extend(spans.drain(..).rev());

        let bidi = unicode_bidi::BidiInfo::new(line, None);
        let rtl = if bidi.paragraphs.is_empty() {
            false
        } else {
            bidi.paragraphs[0].level.is_rtl()
        };

        log::trace!("Line {}: '{}'", if rtl { "RTL" } else { "LTR" }, line);

        for para_info in &bidi.paragraphs {
            let line_rtl = para_info.level.is_rtl();
            assert_eq!(line_rtl, rtl);

            let line_range = para_info.range.clone();
            let levels = Self::adjust_levels(&unicode_bidi::Paragraph::new(&bidi, para_info));

            // Find consecutive level runs. We use this to create Spans.
            // Each span is a set of characters with equal levels.
            let mut start = line_range.start;
            let mut run_level = levels[start];
            spans.reserve(line_range.end - start + 1);

            for (i, &new_level) in levels
                .iter()
                .enumerate()
                .take(line_range.end)
                .skip(start + 1)
            {
                if new_level != run_level {
                    // End of the previous run, start of a new one.
                    let mut span = cached_spans.pop().unwrap_or_else(ShapeSpan::empty);
                    span.build(
                        font_system,
                        line,
                        attrs_list,
                        start..i,
                        line_rtl,
                        run_level,
                        shaping,
                    );
                    spans.push(span);
                    start = i;
                    run_level = new_level;
                }
            }
            let mut span = cached_spans.pop().unwrap_or_else(ShapeSpan::empty);
            span.build(
                font_system,
                line,
                attrs_list,
                start..line_range.end,
                line_rtl,
                run_level,
                shaping,
            );
            spans.push(span);
        }

        // Adjust for tabs
        let mut x = 0.0;
        for span in &mut spans {
            for word in &mut span.words {
                for glyph in &mut word.glyphs {
                    if line.get(glyph.start..glyph.end) == Some("\t") {
                        // Tabs are shaped as spaces, so they will always have the x_advance of a space.
                        let tab_x_advance = f32::from(tab_width) * glyph.x_advance;
                        let tab_stop = (math::floorf(x / tab_x_advance) + 1.0) * tab_x_advance;
                        glyph.x_advance = tab_stop - x;
                    }
                    x += glyph.x_advance;
                }
            }
        }

        self.rtl = rtl;
        self.spans = spans;
        self.metrics_opt = attrs_list.defaults().metrics_opt.map(Into::into);

        self.ellipsis_span.get_or_insert_with(|| {
            let attrs = if attrs_list.spans.is_empty() {
                attrs_list.defaults()
            } else {
                attrs_list.get_span(0) // TODO: using the attrs from the first span for
                                       // ellipsis even if it's at the end. Which for rich text may look weird if the first
                                       // span has a different color or size than where ellipsizing is happening
            };
            let mut glyphs = shape_ellipsis(font_system, &attrs, shaping, rtl);
            if rtl {
                glyphs.reverse();
            }
            let word = ShapeWord {
                blank: false,
                glyphs,
            };
            // The level here is a placeholder; the actual level used for BiDi reordering
            // is set on the VlRange when the ellipsis is inserted during layout.
            let level = if rtl {
                unicode_bidi::Level::rtl()
            } else {
                unicode_bidi::Level::ltr()
            };
            ShapeSpan {
                level,
                words: vec![word],
                decoration_spans: Vec::new(),
            }
        });

        // Return the buffer for later reuse.
        font_system.shape_buffer.spans = cached_spans;
    }

    // A modified version of first part of unicode_bidi::bidi_info::visual_run
    fn adjust_levels(para: &unicode_bidi::Paragraph) -> Vec<unicode_bidi::Level> {
        use unicode_bidi::BidiClass::{B, BN, FSI, LRE, LRI, LRO, PDF, PDI, RLE, RLI, RLO, S, WS};
        let text = para.info.text;
        let levels = &para.info.levels;
        let original_classes = &para.info.original_classes;

        let mut levels = levels.clone();
        let line_classes = &original_classes[..];
        let line_levels = &mut levels[..];

        // Reset some whitespace chars to paragraph level.
        // <http://www.unicode.org/reports/tr9/#L1>
        let mut reset_from: Option<usize> = Some(0);
        let mut reset_to: Option<usize> = None;
        for (i, c) in text.char_indices() {
            match line_classes[i] {
                // Ignored by X9
                RLE | LRE | RLO | LRO | PDF | BN => {}
                // Segment separator, Paragraph separator
                B | S => {
                    assert_eq!(reset_to, None);
                    reset_to = Some(i + c.len_utf8());
                    if reset_from.is_none() {
                        reset_from = Some(i);
                    }
                }
                // Whitespace, isolate formatting
                WS | FSI | LRI | RLI | PDI => {
                    if reset_from.is_none() {
                        reset_from = Some(i);
                    }
                }
                _ => {
                    reset_from = None;
                }
            }
            if let (Some(from), Some(to)) = (reset_from, reset_to) {
                for level in &mut line_levels[from..to] {
                    *level = para.para.level;
                }
                reset_from = None;
                reset_to = None;
            }
        }
        if let Some(from) = reset_from {
            for level in &mut line_levels[from..] {
                *level = para.para.level;
            }
        }
        levels
    }

    // A modified version of second part of unicode_bidi::bidi_info::visual run
    fn reorder(&self, line_range: &[VlRange]) -> Vec<Range<usize>> {
        let line: Vec<unicode_bidi::Level> = line_range.iter().map(|range| range.level).collect();
        let count = line.len();
        if count == 0 {
            return Vec::new();
        }

        // Each VlRange is its own element for L2 reordering.
        // Using individual elements (not grouped runs) ensures that reversal
        // correctly reorders elements even when consecutive ranges share a level.
        let mut elements: Vec<Range<usize>> = (0..count).map(|i| i..i + 1).collect();

        let mut min_level = line[0];
        let mut max_level = line[0];
        for &level in &line[1..] {
            min_level = min(min_level, level);
            max_level = max(max_level, level);
        }

        // Re-order the odd runs.
        // <http://www.unicode.org/reports/tr9/#L2>

        // Stop at the lowest *odd* level.
        min_level = min_level.new_lowest_ge_rtl().expect("Level error");

        while max_level >= min_level {
            // Look for the start of a sequence of consecutive elements at max_level or higher.
            let mut seq_start = 0;
            while seq_start < count {
                if line[elements[seq_start].start] < max_level {
                    seq_start += 1;
                    continue;
                }

                // Found the start of a sequence. Now find the end.
                let mut seq_end = seq_start + 1;
                while seq_end < count {
                    if line[elements[seq_end].start] < max_level {
                        break;
                    }
                    seq_end += 1;
                }

                // Reverse the individual elements within this sequence.
                elements[seq_start..seq_end].reverse();

                seq_start = seq_end;
            }
            max_level
                .lower(1)
                .expect("Lowering embedding level below zero");
        }

        elements
    }

    pub fn layout(
        &self,
        font_size: f32,
        width_opt: Option<f32>,
        wrap: Wrap,
        align: Option<Align>,
        match_mono_width: Option<f32>,
        hinting: Hinting,
    ) -> Vec<LayoutLine> {
        let mut lines = Vec::with_capacity(1);
        let mut scratch = ShapeBuffer::default();
        self.layout_to_buffer(
            &mut scratch,
            font_size,
            width_opt,
            wrap,
            Ellipsize::None,
            align,
            &mut lines,
            match_mono_width,
            hinting,
        );
        lines
    }

    fn get_glyph_start_end(
        word: &ShapeWord,
        start: SpanWordGlyphPos,
        span_index: usize,
        word_idx: usize,
        _direction: LayoutDirection,
        congruent: bool,
    ) -> (usize, usize) {
        if span_index != start.span || word_idx != start.word {
            return (0, word.glyphs.len());
        }
        let (start_glyph_pos, end_glyph_pos) = if congruent {
            (start.glyph, word.glyphs.len())
        } else {
            (0, start.glyph)
        };
        (start_glyph_pos, end_glyph_pos)
    }

    fn fit_glyphs(
        word: &ShapeWord,
        font_size: f32,
        start: SpanWordGlyphPos,
        span_index: usize,
        word_idx: usize,
        direction: LayoutDirection,
        congruent: bool,
        currently_used_width: f32,
        total_available_width: f32,
        forward: bool,
    ) -> (usize, f32) {
        let mut glyphs_w = 0.0;
        let (start_glyph_pos, end_glyph_pos) =
            Self::get_glyph_start_end(word, start, span_index, word_idx, direction, congruent);

        if forward {
            let mut glyph_end = start_glyph_pos;
            for glyph_idx in start_glyph_pos..end_glyph_pos {
                let g_w = word.glyphs[glyph_idx].width(font_size);
                if currently_used_width + glyphs_w + g_w > total_available_width {
                    break;
                }
                glyphs_w += g_w;
                glyph_end = glyph_idx + 1;
            }
            (glyph_end, glyphs_w)
        } else {
            let mut glyph_end = word.glyphs.len();
            for glyph_idx in (start_glyph_pos..end_glyph_pos).rev() {
                let g_w = word.glyphs[glyph_idx].width(font_size);
                if currently_used_width + glyphs_w + g_w > total_available_width {
                    break;
                }
                glyphs_w += g_w;
                glyph_end = glyph_idx;
            }
            (glyph_end, glyphs_w)
        }
    }

    #[inline]
    fn add_to_visual_line(
        &self,
        vl: &mut VisualLine,
        span_index: usize,
        start: WordGlyphPos,
        end: WordGlyphPos,
        width: f32,
        number_of_blanks: u32,
    ) {
        if end == start {
            return;
        }

        vl.ranges.push(VlRange {
            span: span_index,
            start,
            end,
            level: self.spans[span_index].level,
        });
        vl.w += width;
        vl.spaces += number_of_blanks;
    }

    fn remaining_content_exceeds(
        spans: &[ShapeSpan],
        font_size: f32,
        span_index: usize,
        word_idx: usize,
        word_count: usize,
        starting_word_index: usize,
        direction: LayoutDirection,
        congruent: bool,
        start_span: usize,
        span_count: usize,
        threshold: f32,
    ) -> bool {
        let mut acc: f32 = 0.0;

        // Remaining words in the current span
        let word_range: Range<usize> = match (direction, congruent) {
            (LayoutDirection::Forward, true) => word_idx + 1..word_count,
            (LayoutDirection::Forward, false) => 0..word_idx,
            (LayoutDirection::Backward, true) => starting_word_index..word_idx,
            (LayoutDirection::Backward, false) => word_idx + 1..word_count,
        };
        for wi in word_range {
            acc += spans[span_index].words[wi].width(font_size);
            if acc > threshold {
                return true;
            }
        }

        // Remaining spans
        let span_range: Range<usize> = match direction {
            LayoutDirection::Forward => span_index + 1..span_count,
            LayoutDirection::Backward => start_span..span_index,
        };
        for si in span_range {
            for w in &spans[si].words {
                acc += w.width(font_size);
                if acc > threshold {
                    return true;
                }
            }
        }

        false
    }

    /// This will fit as much as possible in one line
    /// If forward is false, it will fit as much as possible from the end of the spans
    /// it will stop when it gets to "start".
    /// If forward is true, it will start from start and keep going to the end of the spans
    #[inline]
    fn layout_spans(
        &self,
        current_visual_line: &mut VisualLine,
        font_size: f32,
        spans: &[ShapeSpan],
        start_opt: Option<SpanWordGlyphPos>,
        rtl: bool,
        width_opt: Option<f32>,
        ellipsize: Ellipsize,
        ellipsis_w: f32,
        direction: LayoutDirection,
    ) {
        let check_ellipsizing = matches!(ellipsize, Ellipsize::Start(_) | Ellipsize::End(_))
            && width_opt.is_some_and(|w| w.is_finite());

        let max_width = width_opt.unwrap_or(f32::INFINITY);
        let span_count = spans.len();

        let mut total_w: f32 = 0.0;

        let start = if let Some(s) = start_opt {
            s
        } else {
            SpanWordGlyphPos::ZERO
        };

        let span_indices: Vec<usize> = if matches!(direction, LayoutDirection::Forward) {
            (start.span..spans.len()).collect()
        } else {
            (start.span..spans.len()).rev().collect()
        };

        'outer: for span_index in span_indices {
            let mut word_range_width = 0.;
            let mut number_of_blanks: u32 = 0;

            let span = &spans[span_index];
            let word_count = span.words.len();

            let starting_word_index = if span_index == start.span {
                start.word
            } else {
                0
            };

            let congruent = rtl == span.level.is_rtl();
            let word_forward: bool = congruent == (direction == LayoutDirection::Forward);

            let word_indices: Vec<usize> = match (direction, congruent, start_opt) {
                (LayoutDirection::Forward, true, _) => (starting_word_index..word_count).collect(),
                (LayoutDirection::Forward, false, Some(start)) => {
                    if span_index == start.span {
                        (0..start.word).rev().collect()
                    } else {
                        (0..word_count).rev().collect()
                    }
                }
                (LayoutDirection::Forward, false, None) => (0..word_count).rev().collect(),
                (LayoutDirection::Backward, true, _) => {
                    ((starting_word_index)..word_count).rev().collect()
                }
                (LayoutDirection::Backward, false, Some(start)) => {
                    if span_index == start.span {
                        if start.glyph > 0 {
                            (0..(start.word + 1)).collect()
                        } else {
                            (0..(start.word)).collect()
                        }
                    } else {
                        (0..word_count).collect()
                    }
                }
                (LayoutDirection::Backward, false, None) => (0..span.words.len()).collect(),
            };
            for word_idx in word_indices {
                let word = &span.words[word_idx];
                let word_width = if span_index == start.span && word_idx == start.word {
                    let (start_glyph_pos, end_glyph_pos) = Self::get_glyph_start_end(
                        word, start, span_index, word_idx, direction, congruent,
                    );
                    let mut w = 0.;
                    for glyph_idx in start_glyph_pos..end_glyph_pos {
                        w += word.glyphs[glyph_idx].width(font_size);
                    }
                    w
                } else {
                    word.width(font_size)
                };

                let overflowing = {
                    // only check this if we're ellipsizing
                    check_ellipsizing
                        && (
                            // if this  word doesn't fit, then we have an overflow
                            (total_w + word_range_width + word_width > max_width)
                                || (Self::remaining_content_exceeds(
                                    spans,
                                    font_size,
                                    span_index,
                                    word_idx,
                                    word_count,
                                    starting_word_index,
                                    direction,
                                    congruent,
                                    start.span,
                                    span_count,
                                    ellipsis_w,
                                ) && total_w + word_range_width + word_width + ellipsis_w
                                    > max_width)
                        )
                };

                if overflowing {
                    // overflow detected
                    let available = (max_width - ellipsis_w).max(0.0);

                    let (glyph_end, glyphs_w) = Self::fit_glyphs(
                        word,
                        font_size,
                        start,
                        span_index,
                        word_idx,
                        direction,
                        congruent,
                        total_w + word_range_width,
                        available,
                        word_forward,
                    );

                    let (start_pos, end_pos) = if word_forward {
                        if span_index == start.span {
                            if !congruent {
                                (WordGlyphPos::ZERO, WordGlyphPos::new(word_idx, glyph_end))
                            } else {
                                (
                                    start.word_glyph_pos(),
                                    WordGlyphPos::new(word_idx, glyph_end),
                                )
                            }
                        } else {
                            (WordGlyphPos::ZERO, WordGlyphPos::new(word_idx, glyph_end))
                        }
                    } else {
                        // For an incongruent span in the forward direction, the
                        // word indices are (0..start.word).rev(). Cap the VlRange
                        // end at start.word_glyph_pos() so it doesn't include
                        // words beyond start.word that belong to a previous line.
                        // For the backward direction (congruent span), the word
                        // indices are (start.word..word_count).rev() and
                        // span.words.len() is the correct end.
                        let range_end = if span_index == start.span && !congruent {
                            start.word_glyph_pos()
                        } else {
                            WordGlyphPos::new(span.words.len(), 0)
                        };
                        (WordGlyphPos::new(word_idx, glyph_end), range_end)
                    };
                    self.add_to_visual_line(
                        current_visual_line,
                        span_index,
                        start_pos,
                        end_pos,
                        word_range_width + glyphs_w,
                        number_of_blanks,
                    );

                    // don't iterate anymore since we overflowed
                    current_visual_line.ellipsized = true;
                    break 'outer;
                }

                word_range_width += word_width;
                if word.blank {
                    number_of_blanks += 1;
                }

                // Backward-only: if we've reached the starting point, commit and stop.
                if matches!(direction, LayoutDirection::Backward)
                    && word_idx == start.word
                    && span_index == start.span
                {
                    let (start_pos, end_pos) = if word_forward {
                        (WordGlyphPos::ZERO, start.word_glyph_pos())
                    } else {
                        (
                            start.word_glyph_pos(),
                            WordGlyphPos::new(span.words.len(), 0),
                        )
                    };

                    self.add_to_visual_line(
                        current_visual_line,
                        span_index,
                        start_pos,
                        end_pos,
                        word_range_width,
                        number_of_blanks,
                    );

                    break 'outer;
                }
            }

            // if we get to here that means we didn't ellipsize, so either the whole span fits,
            // or we don't really care
            total_w += word_range_width;
            let (start_pos, end_pos) = if congruent {
                if span_index == start.span {
                    (
                        start.word_glyph_pos(),
                        WordGlyphPos::new(span.words.len(), 0),
                    )
                } else {
                    (WordGlyphPos::ZERO, WordGlyphPos::new(span.words.len(), 0))
                }
            } else if span_index == start.span {
                (WordGlyphPos::ZERO, start.word_glyph_pos())
            } else {
                (WordGlyphPos::ZERO, WordGlyphPos::new(span.words.len(), 0))
            };

            self.add_to_visual_line(
                current_visual_line,
                span_index,
                start_pos,
                end_pos,
                word_range_width,
                number_of_blanks,
            );
        }

        if matches!(direction, LayoutDirection::Backward) {
            current_visual_line.ranges.reverse();
        }
    }

    fn layout_middle(
        &self,
        current_visual_line: &mut VisualLine,
        font_size: f32,
        spans: &[ShapeSpan],
        start_opt: Option<SpanWordGlyphPos>,
        rtl: bool,
        width: f32,
        ellipsize: Ellipsize,
        ellipsis_w: f32,
    ) {
        assert!(matches!(ellipsize, Ellipsize::Middle(_)));

        // First check if all content fits without any ellipsis.
        {
            let mut test_line = VisualLine::default();
            self.layout_spans(
                &mut test_line,
                font_size,
                spans,
                start_opt,
                rtl,
                Some(width),
                Ellipsize::End(EllipsizeHeightLimit::Lines(1)),
                ellipsis_w,
                LayoutDirection::Forward,
            );
            if !test_line.ellipsized && test_line.w <= width {
                *current_visual_line = test_line;
                return;
            }
        }

        let mut starting_line = VisualLine::default();
        self.layout_spans(
            &mut starting_line,
            font_size,
            spans,
            start_opt,
            rtl,
            Some(width / 2.0),
            Ellipsize::End(EllipsizeHeightLimit::Lines(1)),
            0., //pass 0 for ellipsis_w
            LayoutDirection::Forward,
        );
        let forward_pass_overflowed = starting_line.ellipsized;
        let end_range_opt = starting_line.ranges.last();
        match end_range_opt {
            Some(range) if forward_pass_overflowed => {
                let congruent = rtl == self.spans[range.span].level.is_rtl();
                // create a new range and do the other half
                let mut ending_line = VisualLine::default();
                let start = if congruent {
                    SpanWordGlyphPos {
                        span: range.span,
                        word: range.end.word,
                        glyph: range.end.glyph,
                    }
                } else {
                    SpanWordGlyphPos {
                        span: range.span,
                        word: range.start.word,
                        glyph: range.start.glyph,
                    }
                };
                self.layout_spans(
                    &mut ending_line,
                    font_size,
                    spans,
                    Some(start),
                    rtl,
                    Some((width - starting_line.w - ellipsis_w).max(0.0)),
                    Ellipsize::Start(EllipsizeHeightLimit::Lines(1)),
                    0., //pass 0 for ellipsis_w
                    LayoutDirection::Backward,
                );
                // Insert the ellipsis VlRange between the two halves.
                // Its BiDi level is determined by the adjacent ranges.
                let ellipsis_level = self.ellipsis_level_between(
                    starting_line.ranges.last(),
                    ending_line.ranges.first(),
                );
                starting_line
                    .ranges
                    .push(self.ellipsis_vlrange(ellipsis_level));
                starting_line.ranges.extend(ending_line.ranges);
                current_visual_line.ranges = starting_line.ranges;
                current_visual_line.ellipsized = true;
                current_visual_line.w = starting_line.w + ending_line.w + ellipsis_w;
                current_visual_line.spaces = starting_line.spaces + ending_line.spaces;
            }
            None if forward_pass_overflowed && width > ellipsis_w => {
                // buffer is small enough that the forward pass didn't fit
                // only show the ellipsis
                current_visual_line
                    .ranges
                    .push(self.ellipsis_vlrange(if self.rtl {
                        unicode_bidi::Level::rtl()
                    } else {
                        unicode_bidi::Level::ltr()
                    }));
                current_visual_line.ellipsized = true;
                current_visual_line.w = ellipsis_w;
                current_visual_line.spaces = 0;
            }
            _ => {
                // everything fit in the forward pass
                current_visual_line.ranges = starting_line.ranges;
                current_visual_line.w = starting_line.w;
                current_visual_line.spaces = starting_line.spaces;
                current_visual_line.ellipsized = false;
            }
        }
    }

    /// Returns the words for a given span index, handling the ellipsis sentinel.
    fn get_span_words(&self, span_index: usize) -> &[ShapeWord] {
        if span_index == ELLIPSIS_SPAN {
            &self
                .ellipsis_span
                .as_ref()
                .expect("ellipsis_span not set")
                .words
        } else {
            &self.spans[span_index].words
        }
    }

    fn byte_range_of_vlrange(&self, r: &VlRange) -> Option<(usize, usize)> {
        debug_assert_ne!(r.span, ELLIPSIS_SPAN);
        let words = self.get_span_words(r.span);
        let mut min_byte = usize::MAX;
        let mut max_byte = 0usize;
        let end_word = r.end.word + usize::from(r.end.glyph != 0);
        for (i, word) in words.iter().enumerate().take(end_word).skip(r.start.word) {
            let included_glyphs = match (i == r.start.word, i == r.end.word) {
                (false, false) => &word.glyphs[..],
                (true, false) => &word.glyphs[r.start.glyph..],
                (false, true) => &word.glyphs[..r.end.glyph],
                (true, true) => &word.glyphs[r.start.glyph..r.end.glyph],
            };
            for glyph in included_glyphs {
                min_byte = min_byte.min(glyph.start);
                max_byte = max_byte.max(glyph.end);
            }
        }
        if min_byte <= max_byte {
            Some((min_byte, max_byte))
        } else {
            None
        }
    }

    fn compute_elided_byte_range(
        &self,
        visual_line: &VisualLine,
        line_len: usize,
    ) -> Option<(usize, usize)> {
        if !visual_line.ellipsized {
            return None;
        }
        // Find the position of the ellipsis VlRange
        let ellipsis_idx = visual_line
            .ranges
            .iter()
            .position(|r| r.span == ELLIPSIS_SPAN)?;

        // Find the byte range of the visible content before the ellipsis
        let before_end = (0..ellipsis_idx)
            .rev()
            .find_map(|i| self.byte_range_of_vlrange(&visual_line.ranges[i]))
            .map(|(_, end)| end)
            .unwrap_or(0);

        // Find the byte range of the visible content after the ellipsis
        let after_start = (ellipsis_idx + 1..visual_line.ranges.len())
            .find_map(|i| self.byte_range_of_vlrange(&visual_line.ranges[i]))
            .map(|(start, _)| start)
            .unwrap_or(line_len);

        Some((before_end, after_start))
    }

    /// Returns the maximum byte offset across all glyphs in all non-ellipsis spans.
    /// This effectively gives the byte length of the original shaped text.
    fn max_byte_offset(&self) -> usize {
        self.spans
            .iter()
            .flat_map(|span| span.words.iter())
            .flat_map(|word| word.glyphs.iter())
            .map(|g| g.end)
            .max()
            .unwrap_or(0)
    }

    /// Returns the width of the ellipsis in the given font size.
    fn ellipsis_w(&self, font_size: f32) -> f32 {
        self.ellipsis_span
            .as_ref()
            .map_or(0.0, |s| s.words.iter().map(|w| w.width(font_size)).sum())
    }

    /// Creates a `VlRange` for the ellipsis with the give`BiDi`Di level.
    fn ellipsis_vlrange(&self, level: unicode_bidi::Level) -> VlRange {
        VlRange {
            span: ELLIPSIS_SPAN,
            start: WordGlyphPos::ZERO,
            end: WordGlyphPos::new(1, 0),
            level,
        }
    }

    /// Determines the appropriate `BiDi` level for the ellipsis based on the
    /// adjacent ranges, following UAX#9 N1/N2 rules for neutral characters.
    fn ellipsis_level_between(
        &self,
        before: Option<&VlRange>,
        after: Option<&VlRange>,
    ) -> unicode_bidi::Level {
        match (before, after) {
            (Some(a), Some(b)) if a.level == b.level => a.level,
            (Some(a), None) => a.level,
            (None, Some(b)) => b.level,
            _ => {
                if self.rtl {
                    unicode_bidi::Level::rtl()
                } else {
                    unicode_bidi::Level::ltr()
                }
            }
        }
    }

    fn layout_line(
        &self,
        current_visual_line: &mut VisualLine,
        font_size: f32,
        spans: &[ShapeSpan],
        start_opt: Option<SpanWordGlyphPos>,
        rtl: bool,
        width_opt: Option<f32>,
        ellipsize: Ellipsize,
    ) {
        let ellipsis_w = self.ellipsis_w(font_size);

        match (ellipsize, width_opt) {
            (Ellipsize::Start(_), Some(_)) => {
                self.layout_spans(
                    current_visual_line,
                    font_size,
                    spans,
                    start_opt,
                    rtl,
                    width_opt,
                    ellipsize,
                    ellipsis_w,
                    LayoutDirection::Backward,
                );
                // Insert ellipsis at the visual start (index 0, after backward reversal)
                if current_visual_line.ellipsized {
                    let level =
                        self.ellipsis_level_between(None, current_visual_line.ranges.first());
                    current_visual_line
                        .ranges
                        .insert(0, self.ellipsis_vlrange(level));
                    current_visual_line.w += ellipsis_w;
                }
            }
            (Ellipsize::Middle(_), Some(width)) => {
                self.layout_middle(
                    current_visual_line,
                    font_size,
                    spans,
                    start_opt,
                    rtl,
                    width,
                    ellipsize,
                    ellipsis_w,
                );
            }
            _ => {
                self.layout_spans(
                    current_visual_line,
                    font_size,
                    spans,
                    start_opt,
                    rtl,
                    width_opt,
                    ellipsize,
                    ellipsis_w,
                    LayoutDirection::Forward,
                );
                // Insert ellipsis at the visual end
                if current_visual_line.ellipsized {
                    let level =
                        self.ellipsis_level_between(current_visual_line.ranges.last(), None);
                    current_visual_line
                        .ranges
                        .push(self.ellipsis_vlrange(level));
                    current_visual_line.w += ellipsis_w;
                }
            }
        }

        // Compute the byte range of ellipsized text so the ellipsis LayoutGlyph
        // can have valid start/end indices into the original line text.
        if current_visual_line.ellipsized {
            let line_len = self.max_byte_offset();
            current_visual_line.elided_byte_range =
                self.compute_elided_byte_range(current_visual_line, line_len);
        }
    }

    pub fn layout_to_buffer(
        &self,
        scratch: &mut ShapeBuffer,
        font_size: f32,
        width_opt: Option<f32>,
        wrap: Wrap,
        ellipsize: Ellipsize,
        align: Option<Align>,
        layout_lines: &mut Vec<LayoutLine>,
        match_mono_width: Option<f32>,
        hinting: Hinting,
    ) {
        // For each visual line a list of  (span index,  and range of words in that span)
        // Note that a BiDi visual line could have multiple spans or parts of them
        // let mut vl_range_of_spans = Vec::with_capacity(1);
        let mut visual_lines = mem::take(&mut scratch.visual_lines);
        let mut cached_visual_lines = mem::take(&mut scratch.cached_visual_lines);
        cached_visual_lines.clear();
        cached_visual_lines.extend(visual_lines.drain(..).map(|mut l| {
            l.clear();
            l
        }));

        // Cache glyph sets in reverse order so they will ideally be reused in exactly the same lines.
        let mut cached_glyph_sets = mem::take(&mut scratch.glyph_sets);
        cached_glyph_sets.clear();
        cached_glyph_sets.extend(layout_lines.drain(..).rev().map(|mut v| {
            v.glyphs.clear();
            v.glyphs
        }));

        // This would keep the maximum number of spans that would fit on a visual line
        // If one span is too large, this variable will hold the range of words inside that span
        // that fits on a line.
        // let mut current_visual_line: Vec<VlRange> = Vec::with_capacity(1);
        let mut current_visual_line = cached_visual_lines.pop().unwrap_or_default();

        if wrap == Wrap::None {
            self.layout_line(
                &mut current_visual_line,
                font_size,
                &self.spans,
                None,
                self.rtl,
                width_opt,
                ellipsize,
            );
        } else {
            let mut total_line_height = 0.0;
            let mut total_line_count = 0;
            let max_line_count_opt = match ellipsize {
                Ellipsize::Start(EllipsizeHeightLimit::Lines(lines))
                | Ellipsize::Middle(EllipsizeHeightLimit::Lines(lines))
                | Ellipsize::End(EllipsizeHeightLimit::Lines(lines)) => Some(lines.max(1)),
                _ => None,
            };
            let max_height_opt = match ellipsize {
                Ellipsize::Start(EllipsizeHeightLimit::Height(height))
                | Ellipsize::Middle(EllipsizeHeightLimit::Height(height))
                | Ellipsize::End(EllipsizeHeightLimit::Height(height)) => Some(height),
                _ => None,
            };
            let line_height = self
                .metrics_opt
                .map_or_else(|| font_size, |m| m.line_height);

            let try_ellipsize_last_line = |total_line_count: usize,
                                           total_line_height: f32,
                                           current_visual_line: &mut VisualLine,
                                           font_size: f32,
                                           start_opt: Option<SpanWordGlyphPos>,
                                           width_opt: Option<f32>,
                                           ellipsize: Ellipsize|
             -> bool {
                // If Ellipsize::End, then how many lines can we fit or how much is the available height
                if max_line_count_opt == Some(total_line_count + 1)
                    || max_height_opt.is_some_and(|max_height| {
                        total_line_height + line_height * 2.0 > max_height
                    })
                {
                    self.layout_line(
                        current_visual_line,
                        font_size,
                        &self.spans,
                        start_opt,
                        self.rtl,
                        width_opt,
                        ellipsize,
                    );
                    return true;
                }
                false
            };

            if !try_ellipsize_last_line(
                total_line_count,
                total_line_height,
                &mut current_visual_line,
                font_size,
                None,
                width_opt,
                ellipsize,
            ) {
                'outer: for (span_index, span) in self.spans.iter().enumerate() {
                    let mut word_range_width = 0.;
                    let mut width_before_last_blank = 0.;
                    let mut number_of_blanks: u32 = 0;

                    // Create the word ranges that fits in a visual line
                    if self.rtl != span.level.is_rtl() {
                        // incongruent directions
                        let mut fitting_start = WordGlyphPos::new(span.words.len(), 0);
                        for (i, word) in span.words.iter().enumerate().rev() {
                            let word_width = word.width(font_size);
                            // Addition in the same order used to compute the final width, so that
                            // relayouts with that width as the `line_width` will produce the same
                            // wrapping results.
                            if current_visual_line.w + (word_range_width + word_width)
                            <= width_opt.unwrap_or(f32::INFINITY)
                            // Include one blank word over the width limit since it won't be
                            // counted in the final width
                            || (word.blank
                                && (current_visual_line.w + word_range_width) <= width_opt.unwrap_or(f32::INFINITY))
                            {
                                // fits
                                if word.blank {
                                    number_of_blanks += 1;
                                    width_before_last_blank = word_range_width;
                                }
                                word_range_width += word_width;
                            } else if wrap == Wrap::Glyph
                            // Make sure that the word is able to fit on it's own line, if not, fall back to Glyph wrapping.
                            || (wrap == Wrap::WordOrGlyph && word_width > width_opt.unwrap_or(f32::INFINITY))
                            {
                                // Commit the current line so that the word starts on the next line.
                                if word_range_width > 0.
                                    && wrap == Wrap::WordOrGlyph
                                    && word_width > width_opt.unwrap_or(f32::INFINITY)
                                {
                                    self.add_to_visual_line(
                                        &mut current_visual_line,
                                        span_index,
                                        WordGlyphPos::new(i + 1, 0),
                                        fitting_start,
                                        word_range_width,
                                        number_of_blanks,
                                    );

                                    visual_lines.push(current_visual_line);
                                    current_visual_line =
                                        cached_visual_lines.pop().unwrap_or_default();

                                    number_of_blanks = 0;
                                    word_range_width = 0.;

                                    fitting_start = WordGlyphPos::new(i, 0);
                                    total_line_count += 1;
                                    total_line_height += line_height;
                                    if try_ellipsize_last_line(
                                        total_line_count,
                                        total_line_height,
                                        &mut current_visual_line,
                                        font_size,
                                        Some(SpanWordGlyphPos::with_wordglyph(
                                            span_index,
                                            fitting_start,
                                        )),
                                        width_opt,
                                        ellipsize,
                                    ) {
                                        break 'outer;
                                    }
                                }

                                for (glyph_i, glyph) in word.glyphs.iter().enumerate().rev() {
                                    let glyph_width = glyph.width(font_size);
                                    if current_visual_line.w + (word_range_width + glyph_width)
                                        <= width_opt.unwrap_or(f32::INFINITY)
                                    {
                                        word_range_width += glyph_width;
                                    } else {
                                        self.add_to_visual_line(
                                            &mut current_visual_line,
                                            span_index,
                                            WordGlyphPos::new(i, glyph_i + 1),
                                            fitting_start,
                                            word_range_width,
                                            number_of_blanks,
                                        );
                                        visual_lines.push(current_visual_line);
                                        current_visual_line =
                                            cached_visual_lines.pop().unwrap_or_default();

                                        number_of_blanks = 0;
                                        word_range_width = glyph_width;
                                        fitting_start = WordGlyphPos::new(i, glyph_i + 1);
                                        total_line_count += 1;
                                        total_line_height += line_height;
                                        if try_ellipsize_last_line(
                                            total_line_count,
                                            total_line_height,
                                            &mut current_visual_line,
                                            font_size,
                                            Some(SpanWordGlyphPos::with_wordglyph(
                                                span_index,
                                                fitting_start,
                                            )),
                                            width_opt,
                                            ellipsize,
                                        ) {
                                            break 'outer;
                                        }
                                    }
                                }
                            } else {
                                // Wrap::Word, Wrap::WordOrGlyph

                                // If we had a previous range, commit that line before the next word.
                                if word_range_width > 0. {
                                    // Current word causing a wrap is not whitespace, so we ignore the
                                    // previous word if it's a whitespace
                                    let trailing_blank = span
                                        .words
                                        .get(i + 1)
                                        .is_some_and(|previous_word| previous_word.blank);

                                    if trailing_blank {
                                        number_of_blanks = number_of_blanks.saturating_sub(1);
                                        self.add_to_visual_line(
                                            &mut current_visual_line,
                                            span_index,
                                            WordGlyphPos::new(i + 2, 0),
                                            fitting_start,
                                            width_before_last_blank,
                                            number_of_blanks,
                                        );
                                    } else {
                                        self.add_to_visual_line(
                                            &mut current_visual_line,
                                            span_index,
                                            WordGlyphPos::new(i + 1, 0),
                                            fitting_start,
                                            word_range_width,
                                            number_of_blanks,
                                        );
                                    }
                                }

                                // This fixes a bug that a long first word at the boundary of
                                // was overflowing
                                if !current_visual_line.ranges.is_empty() {
                                    visual_lines.push(current_visual_line);
                                    current_visual_line =
                                        cached_visual_lines.pop().unwrap_or_default();
                                    number_of_blanks = 0;
                                    total_line_count += 1;
                                    total_line_height += line_height;

                                    if try_ellipsize_last_line(
                                        total_line_count,
                                        total_line_height,
                                        &mut current_visual_line,
                                        font_size,
                                        Some(SpanWordGlyphPos::with_wordglyph(
                                            span_index,
                                            if word.blank {
                                                WordGlyphPos::new(i, 0)
                                            } else {
                                                WordGlyphPos::new(i + 1, 0)
                                            },
                                        )),
                                        width_opt,
                                        ellipsize,
                                    ) {
                                        break 'outer;
                                    }
                                }

                                if word.blank {
                                    word_range_width = 0.;
                                    fitting_start = WordGlyphPos::new(i, 0);
                                } else {
                                    word_range_width = word_width;
                                    fitting_start = WordGlyphPos::new(i + 1, 0);
                                }
                            }
                        }
                        self.add_to_visual_line(
                            &mut current_visual_line,
                            span_index,
                            WordGlyphPos::new(0, 0),
                            fitting_start,
                            word_range_width,
                            number_of_blanks,
                        );
                    } else {
                        // congruent direction
                        let mut fitting_start = WordGlyphPos::ZERO;
                        for (i, word) in span.words.iter().enumerate() {
                            let word_width = word.width(font_size);
                            if current_visual_line.w + (word_range_width + word_width)
                            <= width_opt.unwrap_or(f32::INFINITY)
                            // Include one blank word over the width limit since it won't be
                            // counted in the final width.
                            || (word.blank
                                && (current_visual_line.w + word_range_width) <= width_opt.unwrap_or(f32::INFINITY))
                            {
                                // fits
                                if word.blank {
                                    number_of_blanks += 1;
                                    width_before_last_blank = word_range_width;
                                }
                                word_range_width += word_width;
                            } else if wrap == Wrap::Glyph
                            // Make sure that the word is able to fit on it's own line, if not, fall back to Glyph wrapping.
                            || (wrap == Wrap::WordOrGlyph && word_width > width_opt.unwrap_or(f32::INFINITY))
                            {
                                // Commit the current line so that the word starts on the next line.
                                if word_range_width > 0.
                                    && wrap == Wrap::WordOrGlyph
                                    && word_width > width_opt.unwrap_or(f32::INFINITY)
                                {
                                    self.add_to_visual_line(
                                        &mut current_visual_line,
                                        span_index,
                                        fitting_start,
                                        WordGlyphPos::new(i, 0),
                                        word_range_width,
                                        number_of_blanks,
                                    );

                                    visual_lines.push(current_visual_line);
                                    current_visual_line =
                                        cached_visual_lines.pop().unwrap_or_default();

                                    number_of_blanks = 0;
                                    word_range_width = 0.;

                                    fitting_start = WordGlyphPos::new(i, 0);
                                    total_line_count += 1;
                                    total_line_height += line_height;
                                    if try_ellipsize_last_line(
                                        total_line_count,
                                        total_line_height,
                                        &mut current_visual_line,
                                        font_size,
                                        Some(SpanWordGlyphPos::with_wordglyph(
                                            span_index,
                                            fitting_start,
                                        )),
                                        width_opt,
                                        ellipsize,
                                    ) {
                                        break 'outer;
                                    }
                                }

                                for (glyph_i, glyph) in word.glyphs.iter().enumerate() {
                                    let glyph_width = glyph.width(font_size);
                                    if current_visual_line.w + (word_range_width + glyph_width)
                                        <= width_opt.unwrap_or(f32::INFINITY)
                                    {
                                        word_range_width += glyph_width;
                                    } else {
                                        self.add_to_visual_line(
                                            &mut current_visual_line,
                                            span_index,
                                            fitting_start,
                                            WordGlyphPos::new(i, glyph_i),
                                            word_range_width,
                                            number_of_blanks,
                                        );
                                        visual_lines.push(current_visual_line);
                                        current_visual_line =
                                            cached_visual_lines.pop().unwrap_or_default();

                                        number_of_blanks = 0;
                                        word_range_width = glyph_width;
                                        fitting_start = WordGlyphPos::new(i, glyph_i);
                                        total_line_count += 1;
                                        total_line_height += line_height;
                                        if try_ellipsize_last_line(
                                            total_line_count,
                                            total_line_height,
                                            &mut current_visual_line,
                                            font_size,
                                            Some(SpanWordGlyphPos::with_wordglyph(
                                                span_index,
                                                fitting_start,
                                            )),
                                            width_opt,
                                            ellipsize,
                                        ) {
                                            break 'outer;
                                        }
                                    }
                                }
                            } else {
                                // Wrap::Word, Wrap::WordOrGlyph

                                // If we had a previous range, commit that line before the next word.
                                if word_range_width > 0. {
                                    // Current word causing a wrap is not whitespace, so we ignore the
                                    // previous word if it's a whitespace.
                                    let trailing_blank = i > 0 && span.words[i - 1].blank;

                                    if trailing_blank {
                                        number_of_blanks = number_of_blanks.saturating_sub(1);
                                        self.add_to_visual_line(
                                            &mut current_visual_line,
                                            span_index,
                                            fitting_start,
                                            WordGlyphPos::new(i - 1, 0),
                                            width_before_last_blank,
                                            number_of_blanks,
                                        );
                                    } else {
                                        self.add_to_visual_line(
                                            &mut current_visual_line,
                                            span_index,
                                            fitting_start,
                                            WordGlyphPos::new(i, 0),
                                            word_range_width,
                                            number_of_blanks,
                                        );
                                    }
                                }

                                if !current_visual_line.ranges.is_empty() {
                                    visual_lines.push(current_visual_line);
                                    current_visual_line =
                                        cached_visual_lines.pop().unwrap_or_default();
                                    number_of_blanks = 0;
                                    total_line_count += 1;
                                    total_line_height += line_height;
                                    if try_ellipsize_last_line(
                                        total_line_count,
                                        total_line_height,
                                        &mut current_visual_line,
                                        font_size,
                                        Some(SpanWordGlyphPos::with_wordglyph(
                                            span_index,
                                            if i > 0 && span.words[i - 1].blank {
                                                WordGlyphPos::new(i - 1, 0)
                                            } else {
                                                WordGlyphPos::new(i, 0)
                                            },
                                        )),
                                        width_opt,
                                        ellipsize,
                                    ) {
                                        break 'outer;
                                    }
                                }

                                if word.blank {
                                    word_range_width = 0.;
                                    fitting_start = WordGlyphPos::new(i + 1, 0);
                                } else {
                                    word_range_width = word_width;
                                    fitting_start = WordGlyphPos::new(i, 0);
                                }
                            }
                        }
                        self.add_to_visual_line(
                            &mut current_visual_line,
                            span_index,
                            fitting_start,
                            WordGlyphPos::new(span.words.len(), 0),
                            word_range_width,
                            number_of_blanks,
                        );
                    }
                }
            }
        }

        if current_visual_line.ranges.is_empty() {
            current_visual_line.clear();
            cached_visual_lines.push(current_visual_line);
        } else {
            visual_lines.push(current_visual_line);
        }

        // Create the LayoutLines using the ranges inside visual lines
        let align = align.unwrap_or(if self.rtl { Align::Right } else { Align::Left });

        let line_width = width_opt.unwrap_or_else(|| {
            let mut width: f32 = 0.0;
            for visual_line in &visual_lines {
                width = width.max(visual_line.w);
            }
            width
        });

        let start_x = if self.rtl { line_width } else { 0.0 };

        let number_of_visual_lines = visual_lines.len();
        for (index, visual_line) in visual_lines.iter().enumerate() {
            if visual_line.ranges.is_empty() {
                continue;
            }

            let new_order = self.reorder(&visual_line.ranges);

            let mut glyphs = cached_glyph_sets
                .pop()
                .unwrap_or_else(|| Vec::with_capacity(1));
            let mut x = start_x;
            let mut y = 0.;
            let mut max_ascent: f32 = 0.;
            let mut max_descent: f32 = 0.;
            let alignment_correction = match (align, self.rtl) {
                (Align::Left, true) => (line_width - visual_line.w).max(0.),
                (Align::Left, false) => 0.,
                (Align::Right, true) => 0.,
                (Align::Right, false) => (line_width - visual_line.w).max(0.),
                (Align::Center, _) => (line_width - visual_line.w).max(0.) / 2.0,
                (Align::End, _) => (line_width - visual_line.w).max(0.),
                (Align::Justified, _) => 0.,
            };

            if self.rtl {
                x -= alignment_correction;
            } else {
                x += alignment_correction;
            }

            if hinting == Hinting::Enabled {
                x = x.round();
            }

            // TODO: Only certain `is_whitespace` chars are typically expanded but this is what is
            // currently used to compute `visual_line.spaces`.
            //
            // https://www.unicode.org/reports/tr14/#Introduction
            // > When expanding or compressing interword space according to common
            // > typographical practice, only the spaces marked by U+0020 SPACE and U+00A0
            // > NO-BREAK SPACE are subject to compression, and only spaces marked by U+0020
            // > SPACE, U+00A0 NO-BREAK SPACE, and occasionally spaces marked by U+2009 THIN
            // > SPACE are subject to expansion. All other space characters normally have
            // > fixed width.
            //
            // (also some spaces aren't followed by potential linebreaks but they could
            //  still be expanded)

            // Amount of extra width added to each blank space within a line.
            let justification_expansion = if matches!(align, Align::Justified)
                && visual_line.spaces > 0
                // Don't justify the last line in a paragraph.
                && index != number_of_visual_lines - 1
            {
                (line_width - visual_line.w) / visual_line.spaces as f32
            } else {
                0.
            };

            let elided_byte_range = if visual_line.ellipsized {
                visual_line.elided_byte_range
            } else {
                None
            };

            let mut decorations: Vec<DecorationSpan> = Vec::new();

            let process_range = |range: Range<usize>,
                                 x: &mut f32,
                                 y: &mut f32,
                                 glyphs: &mut Vec<LayoutGlyph>,
                                 decorations: &mut Vec<DecorationSpan>,
                                 max_ascent: &mut f32,
                                 max_descent: &mut f32| {
                for r in visual_line.ranges[range.clone()].iter() {
                    let is_ellipsis = r.span == ELLIPSIS_SPAN;
                    let span_words = self.get_span_words(r.span);
                    let deco_spans: &[(Range<usize>, GlyphDecorationData)] = if is_ellipsis {
                        &[]
                    } else {
                        &self.spans[r.span].decoration_spans
                    };
                    // Cursor into deco_spans — advances forward as glyphs are
                    // emitted in byte order, giving amortized O(1) lookup.
                    let mut deco_cursor: usize = 0;
                    // If ending_glyph is not 0 we need to include glyphs from the ending_word
                    for i in r.start.word..r.end.word + usize::from(r.end.glyph != 0) {
                        let word = &span_words[i];
                        let included_glyphs = match (i == r.start.word, i == r.end.word) {
                            (false, false) => &word.glyphs[..],
                            (true, false) => &word.glyphs[r.start.glyph..],
                            (false, true) => &word.glyphs[..r.end.glyph],
                            (true, true) => &word.glyphs[r.start.glyph..r.end.glyph],
                        };

                        for glyph in included_glyphs {
                            // Use overridden font size
                            let font_size = glyph.metrics_opt.map_or(font_size, |x| x.font_size);

                            let match_mono_em_width = match_mono_width.map(|w| w / font_size);

                            let glyph_font_size = match (
                                match_mono_em_width,
                                glyph.font_monospace_em_width,
                            ) {
                                (Some(match_em_width), Some(glyph_em_width))
                                    if glyph_em_width != match_em_width =>
                                {
                                    let glyph_to_match_factor = glyph_em_width / match_em_width;
                                    let glyph_font_size = math::roundf(glyph_to_match_factor)
                                        .max(1.0)
                                        / glyph_to_match_factor
                                        * font_size;
                                    log::trace!(
                                        "Adjusted glyph font size ({font_size} => {glyph_font_size})"
                                    );
                                    glyph_font_size
                                }
                                _ => font_size,
                            };

                            let mut x_advance = glyph_font_size.mul_add(
                                glyph.x_advance,
                                if word.blank {
                                    justification_expansion
                                } else {
                                    0.0
                                },
                            );
                            if let Some(match_em_width) = match_mono_em_width {
                                // Round to nearest monospace width
                                x_advance = ((x_advance / match_em_width).round()) * match_em_width;
                            }
                            if hinting == Hinting::Enabled {
                                x_advance = x_advance.round();
                            }
                            if self.rtl {
                                *x -= x_advance;
                            }
                            let y_advance = glyph_font_size * glyph.y_advance;
                            let mut layout_glyph = glyph.layout(
                                glyph_font_size,
                                glyph.metrics_opt.map(|x| x.line_height),
                                *x,
                                *y,
                                x_advance,
                                r.level,
                            );
                            // Fix ellipsis glyph indices: point both start and
                            // end to the elision boundary so that hit-detection
                            // places the cursor at the seam between visible and
                            // elided text instead of selecting invisible content.
                            if is_ellipsis {
                                if let Some((elided_start, elided_end)) = elided_byte_range {
                                    // Use the boundary closest to the visible
                                    // content that is adjacent to this ellipsis:
                                    //   Start:  …|visible  → boundary = elided_end
                                    //   End:    visible|…  → boundary = elided_start
                                    //   Middle: vis|…|vis  → boundary = elided_start
                                    let boundary = if elided_start == 0 {
                                        elided_end
                                    } else {
                                        elided_start
                                    };
                                    layout_glyph.start = boundary;
                                    layout_glyph.end = boundary;
                                }
                            }
                            glyphs.push(layout_glyph);

                            if deco_cursor >= deco_spans.len()
                                || glyph.start < deco_spans[deco_cursor].0.start
                            {
                                deco_cursor = 0;
                            }
                            while deco_cursor < deco_spans.len()
                                && deco_spans[deco_cursor].0.end <= glyph.start
                            {
                                deco_cursor += 1;
                            }
                            let glyph_deco = deco_spans
                                .get(deco_cursor)
                                .filter(|(range, _)| glyph.start >= range.start);
                            let glyph_idx = glyphs.len() - 1;
                            let extends = matches!(
                                (decorations.last(), &glyph_deco),
                                (Some(span), Some((_, d))) if span.data == *d
                            );
                            if extends {
                                if let Some(last) = decorations.last_mut() {
                                    last.glyph_range.end = glyph_idx + 1;
                                }
                            } else if let Some((_, d)) = glyph_deco {
                                decorations.push(DecorationSpan {
                                    glyph_range: glyph_idx..glyph_idx + 1,
                                    data: d.clone(),
                                    color_opt: glyphs[glyph_idx].color_opt,
                                    font_size: glyphs[glyph_idx].font_size,
                                });
                            }
                            if !self.rtl {
                                *x += x_advance;
                            }
                            *y += y_advance;
                            *max_ascent = max_ascent.max(glyph_font_size * glyph.ascent);
                            *max_descent = max_descent.max(glyph_font_size * glyph.descent);
                        }
                    }
                }
            };

            if self.rtl {
                for range in new_order.into_iter().rev() {
                    process_range(
                        range,
                        &mut x,
                        &mut y,
                        &mut glyphs,
                        &mut decorations,
                        &mut max_ascent,
                        &mut max_descent,
                    );
                }
            } else {
                /* LTR */
                for range in new_order {
                    process_range(
                        range,
                        &mut x,
                        &mut y,
                        &mut glyphs,
                        &mut decorations,
                        &mut max_ascent,
                        &mut max_descent,
                    );
                }
            }

            let mut line_height_opt: Option<f32> = None;
            for glyph in &glyphs {
                if let Some(glyph_line_height) = glyph.line_height_opt {
                    line_height_opt = line_height_opt
                        .map_or(Some(glyph_line_height), |line_height| {
                            Some(line_height.max(glyph_line_height))
                        });
                }
            }

            layout_lines.push(LayoutLine {
                w: if align != Align::Justified {
                    visual_line.w
                } else if self.rtl {
                    start_x - x
                } else {
                    x
                },
                max_ascent,
                max_descent,
                line_height_opt,
                glyphs,
                decorations,
            });
        }

        // This is used to create a visual line for empty lines (e.g. lines with only a <CR>)
        if layout_lines.is_empty() {
            layout_lines.push(LayoutLine {
                w: 0.0,
                max_ascent: 0.0,
                max_descent: 0.0,
                line_height_opt: self.metrics_opt.map(|x| x.line_height),
                glyphs: Vec::default(),
                decorations: Vec::new(),
            });
        }

        // Restore the buffer to the scratch set to prevent reallocations.
        scratch.visual_lines = visual_lines;
        scratch.visual_lines.append(&mut cached_visual_lines);
        scratch.cached_visual_lines = cached_visual_lines;
        scratch.glyph_sets = cached_glyph_sets;
    }
}
