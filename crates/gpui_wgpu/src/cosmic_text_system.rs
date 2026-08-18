use anyhow::{Context as _, Ok, Result};
use collections::HashMap;
use cosmic_text::{
    Attrs, AttrsList, Ellipsize, Family, Font as CosmicTextFont,
    FontFeatures as CosmicFontFeatures, FontSystem, ShapeBuffer, ShapeLine,
};
use gpui::{
    Bounds, DevicePixels, Font, FontFallbacks, FontFeatures, FontId, FontMetrics, FontRun, GlyphId,
    LineLayout, Pixels, PlatformTextSystem, RenderGlyphParams, SUBPIXEL_VARIANTS_X,
    SUBPIXEL_VARIANTS_Y, ShapedGlyph, ShapedRun, SharedString, Size, TextRenderingMode, point,
    size,
};

use itertools::Itertools;
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::{borrow::Cow, ops::Range, sync::Arc};
use swash::{
    NormalizedCoord,
    scale::{Render, ScaleContext, Source, StrikeWith},
    tag_from_bytes,
    zeno::{Format, Vector},
};
use unicode_segmentation::UnicodeSegmentation;

pub struct CosmicTextSystem(RwLock<CosmicTextSystemState>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FontKey {
    family: SharedString,
    features: FontFeatures,
    fallbacks: Option<FontFallbacks>,
}

impl FontKey {
    fn new(family: SharedString, features: FontFeatures, fallbacks: Option<FontFallbacks>) -> Self {
        Self {
            family,
            features,
            fallbacks,
        }
    }
}

struct CosmicTextSystemState {
    font_system: FontSystem,
    scratch: ShapeBuffer,
    swash_scale_context: ScaleContext,
    /// Contains all already loaded fonts, including all faces. Indexed by `FontId`.
    loaded_fonts: Vec<LoadedFont>,
    /// Caches the `FontId`s associated with a specific family to avoid iterating the font database
    /// for every font face in a family.
    font_ids_by_family_cache: HashMap<FontKey, SmallVec<[FontId; 4]>>,
    /// Caches the derived `FontId` for a (variable face, requested weight) pair. Only
    /// variable faces get entries, so static families cost nothing.
    variable_instances: HashMap<(FontId, u32), FontId>,
    system_font_fallback: String,
}

struct LoadedFont {
    font: Arc<CosmicTextFont>,
    features: CosmicFontFeatures,
    is_known_emoji_font: bool,
    /// resolved at load time so `layout_line` shares one chain across faces.
    /// `Arc` keeps clone cheap on the per-run hot path.
    user_fallback_chain: Arc<[(FontId, SharedString)]>,
    /// The weight this entry represents. For a static face it is the face's own
    /// weight; for a variable face it is the weight that was *requested*, which
    /// fontdb cannot express because it reports the whole file as one face at its
    /// default `usWeightClass`.
    weight: gpui::FontWeight,
    /// Normalized `wght` coordinate for `weight`, empty when the face is static or
    /// already sits at the requested weight. Every swash call that measures or
    /// rasterizes outlines has to pass this, or the glyphs come out at the face
    /// default while the layout uses the requested weight.
    coords: SmallVec<[NormalizedCoord; 1]>,
}

impl CosmicTextSystem {
    pub fn new(system_font_fallback: &str) -> Self {
        let font_system = FontSystem::new();

        Self(RwLock::new(CosmicTextSystemState {
            font_system,
            scratch: ShapeBuffer::default(),
            swash_scale_context: ScaleContext::new(),
            loaded_fonts: Vec::new(),
            font_ids_by_family_cache: HashMap::default(),
            variable_instances: HashMap::default(),
            system_font_fallback: system_font_fallback.to_string(),
        }))
    }

    pub fn new_without_system_fonts(system_font_fallback: &str) -> Self {
        let font_system = FontSystem::new_with_locale_and_db(
            "en-US".to_string(),
            cosmic_text::fontdb::Database::new(),
        );

        Self(RwLock::new(CosmicTextSystemState {
            font_system,
            scratch: ShapeBuffer::default(),
            swash_scale_context: ScaleContext::new(),
            loaded_fonts: Vec::new(),
            font_ids_by_family_cache: HashMap::default(),
            variable_instances: HashMap::default(),
            system_font_fallback: system_font_fallback.to_string(),
        }))
    }
}

impl PlatformTextSystem for CosmicTextSystem {
    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        self.0.write().add_fonts(fonts)
    }

    fn all_font_names(&self) -> Vec<String> {
        let mut result = self
            .0
            .read()
            .font_system
            .db()
            .faces()
            .filter_map(|face| face.families.first().map(|family| family.0.clone()))
            .collect_vec();
        result.sort_unstable();
        result.dedup();
        result
    }

    fn font_id(&self, font: &Font) -> Result<FontId> {
        let mut state = self.0.write();
        let key = FontKey::new(
            font.family.clone(),
            font.features.clone(),
            font.fallbacks.clone(),
        );
        let candidates = if let Some(font_ids) = state.font_ids_by_family_cache.get(&key) {
            font_ids.as_slice()
        } else {
            let font_ids =
                state.load_family(&font.family, &font.features, font.fallbacks.as_ref())?;
            state.font_ids_by_family_cache.insert(key.clone(), font_ids);
            state.font_ids_by_family_cache[&key].as_ref()
        };

        let ix = find_best_match(font, candidates, &state)?;
        let base = candidates[ix];

        // `find_best_match` picked a FACE. For a variable file that face is the whole
        // family, so the requested weight is still unrepresented; derive an instance
        // that carries it. Static faces return `base` unchanged.
        state.instance_for_weight(base, font.weight)
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        let lock = self.0.read();
        let loaded_font = lock.loaded_font(font_id);
        let metrics = loaded_font.font.as_swash().metrics(&loaded_font.coords);

        FontMetrics {
            units_per_em: metrics.units_per_em as u32,
            ascent: metrics.ascent,
            descent: -metrics.descent,
            line_gap: metrics.leading,
            underline_position: metrics.underline_offset,
            underline_thickness: metrics.stroke_size,
            cap_height: metrics.cap_height,
            x_height: metrics.x_height,
            bounding_box: Bounds {
                origin: point(0.0, 0.0),
                size: size(metrics.max_width, metrics.ascent + metrics.descent),
            },
        }
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        let lock = self.0.read();
        let loaded_font = lock.loaded_font(font_id);
        let glyph_metrics = loaded_font
            .font
            .as_swash()
            .glyph_metrics(&loaded_font.coords);
        let glyph_id = glyph_id.0 as u16;
        Ok(Bounds {
            origin: point(0.0, 0.0),
            size: size(
                glyph_metrics.advance_width(glyph_id),
                glyph_metrics.advance_height(glyph_id),
            ),
        })
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        self.0.read().advance(font_id, glyph_id)
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.0.read().glyph_for_char(font_id, ch)
    }

    fn glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        self.0.write().raster_bounds(params)
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        self.0.write().rasterize_glyph(params, raster_bounds)
    }

    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout {
        self.0.write().layout_line(text, font_size, runs)
    }

    fn recommended_rendering_mode(
        &self,
        _font_id: FontId,
        _font_size: Pixels,
    ) -> TextRenderingMode {
        TextRenderingMode::Subpixel
    }
}

impl CosmicTextSystemState {
    fn loaded_font(&self, font_id: FontId) -> &LoadedFont {
        &self.loaded_fonts[font_id.0]
    }

    #[profiling::function]
    fn add_fonts(&mut self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        let db = self.font_system.db_mut();
        for bytes in fonts {
            db.load_font_source(cosmic_text::fontdb::Source::Binary(Arc::new(bytes)));
        }
        Ok(())
    }

    #[profiling::function]
    fn load_family(
        &mut self,
        name: &str,
        features: &FontFeatures,
        fallbacks: Option<&FontFallbacks>,
    ) -> Result<SmallVec<[FontId; 4]>> {
        // recurse with `fallbacks = None` so a fallback family cannot pull in
        // another chain. missing fallback families are dropped so a typo in
        // settings still lets the primary family load.
        let user_fallback_chain: Arc<[(FontId, SharedString)]> = match fallbacks {
            Some(fallbacks) if !fallbacks.fallback_list().is_empty() => {
                let mut chain: Vec<(FontId, SharedString)> = Vec::new();
                for fallback_name in fallbacks.fallback_list() {
                    let fb_key = FontKey::new(
                        SharedString::from(fallback_name.clone()),
                        features.clone(),
                        None,
                    );
                    let fb_ids = if let Some(cached) = self.font_ids_by_family_cache.get(&fb_key) {
                        cached.clone()
                    } else {
                        let loaded = self.load_family(fallback_name, features, None)?;
                        self.font_ids_by_family_cache
                            .insert(fb_key.clone(), loaded.clone());
                        loaded
                    };
                    let Some(&fb_id) = fb_ids.first() else {
                        continue;
                    };
                    let db_id = self.loaded_fonts[fb_id.0].font.id();
                    if let Some(face) = self.font_system.db().face(db_id)
                        && let Some(family) = face.families.first()
                    {
                        chain.push((fb_id, SharedString::from(family.0.clone())));
                    }
                }
                Arc::from(chain)
            }
            _ => Arc::from(Vec::new()),
        };

        let name = gpui::font_name_with_fallbacks(name, &self.system_font_fallback);

        let families = self
            .font_system
            .db()
            .faces()
            .filter(|face| face.families.iter().any(|family| *name == family.0))
            .map(|face| {
                (
                    face.id,
                    face.post_script_name.clone(),
                    gpui::FontWeight(face.weight.0 as f32),
                )
            })
            .collect::<SmallVec<[_; 4]>>();

        let cosmic_features = cosmic_font_features(features)?;

        let mut loaded_font_ids = SmallVec::new();
        for (font_id, postscript_name, face_weight) in families {
            let font = self
                .font_system
                .get_font(font_id, cosmic_text::Weight::NORMAL)
                .context("Could not load font")?;

            // HACK: To let the storybook run and render Windows caption icons. We should actually do better font fallback.
            let allowed_bad_font_names = [
                "SegoeFluentIcons", // NOTE: Segoe fluent icons postscript name is inconsistent
                "Segoe Fluent Icons",
            ];

            if font.as_swash().charmap().map('m') == 0
                && !allowed_bad_font_names.contains(&postscript_name.as_str())
            {
                self.font_system.db_mut().remove_face(font.id());
                continue;
            };

            let font_id = FontId(self.loaded_fonts.len());
            loaded_font_ids.push(font_id);
            self.loaded_fonts.push(LoadedFont {
                font,
                features: cosmic_features.clone(),
                is_known_emoji_font: check_is_known_emoji_font(&postscript_name),
                user_fallback_chain: Arc::clone(&user_fallback_chain),
                weight: face_weight,
                coords: SmallVec::new(),
            });
        }

        Ok(loaded_font_ids)
    }

    /// Returns a `FontId` whose `LoadedFont` represents `weight`.
    ///
    /// `find_best_match` picks a FACE, and a face is not a weight: fontdb reports a
    /// variable file as a single face at its default `usWeightClass`, and a static
    /// family clamps a request to the nearest weight it ships. Either way the number
    /// the caller asked for is gone by the time layout and rasterization run, so it is
    /// recorded here, together with the normalized coordinate that carries it into
    /// swash.
    ///
    /// The user fallback chain is instanced too. It is resolved at load time, before
    /// any weight is known, so without this a variable fallback family would keep
    /// rendering at its default weight no matter what the primary asked for, which is
    /// the shape of the original report.
    fn instance_for_weight(&mut self, base: FontId, weight: gpui::FontWeight) -> Result<FontId> {
        if self.loaded_fonts[base.0].weight == weight {
            return Ok(base);
        }
        let key = (base, weight.0.to_bits());
        if let Some(&existing) = self.variable_instances.get(&key) {
            return Ok(existing);
        }

        let chain = Arc::clone(&self.loaded_fonts[base.0].user_fallback_chain);
        let mut instanced_chain: Vec<(FontId, SharedString)> = Vec::with_capacity(chain.len());
        for (fallback_id, fallback_name) in chain.iter() {
            // Fallback entries carry an empty chain of their own (`load_family` recurses
            // with `fallbacks = None`), so this cannot recurse further.
            let instanced = self.instance_for_weight(*fallback_id, weight)?;
            instanced_chain.push((instanced, fallback_name.clone()));
        }

        let base_font = &self.loaded_fonts[base.0];
        let database_id = base_font.font.id();
        let features = base_font.features.clone();
        let is_known_emoji_font = base_font.is_known_emoji_font;
        let coords = weight_coords(&base_font.font, weight);

        // cosmic-text builds its own `wght` location from this argument, so passing the
        // real weight is what keeps shaping agreeing with the coords used for raster.
        let font = self
            .font_system
            .get_font(database_id, cosmic_text::Weight(weight.0 as u16))
            .context("could not instantiate font at the requested weight")?;

        let font_id = FontId(self.loaded_fonts.len());
        self.loaded_fonts.push(LoadedFont {
            font,
            features,
            is_known_emoji_font,
            user_fallback_chain: Arc::from(instanced_chain),
            weight,
            coords,
        });
        self.variable_instances.insert(key, font_id);

        Ok(font_id)
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        let loaded_font = self.loaded_font(font_id);
        let glyph_metrics = loaded_font
            .font
            .as_swash()
            .glyph_metrics(&loaded_font.coords);
        Ok(Size {
            width: glyph_metrics.advance_width(glyph_id.0 as u16),
            height: glyph_metrics.advance_height(glyph_id.0 as u16),
        })
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        let glyph_id = self.loaded_font(font_id).font.as_swash().charmap().map(ch);
        if glyph_id == 0 {
            None
        } else {
            Some(GlyphId(glyph_id.into()))
        }
    }

    fn raster_bounds(&mut self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        let image = self.render_glyph_image(params)?;
        Ok(Bounds {
            origin: point(image.placement.left.into(), (-image.placement.top).into()),
            size: size(image.placement.width.into(), image.placement.height.into()),
        })
    }

    #[profiling::function]
    fn rasterize_glyph(
        &mut self,
        params: &RenderGlyphParams,
        glyph_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        if glyph_bounds.size.width.0 == 0 || glyph_bounds.size.height.0 == 0 {
            anyhow::bail!("glyph bounds are empty");
        }

        let mut image = self.render_glyph_image(params)?;
        let bitmap_size = glyph_bounds.size;
        match image.content {
            swash::scale::image::Content::Color | swash::scale::image::Content::SubpixelMask => {
                // Convert from RGBA to BGRA.
                for pixel in image.data.chunks_exact_mut(4) {
                    pixel.swap(0, 2);
                }
                Ok((bitmap_size, image.data))
            }
            swash::scale::image::Content::Mask => {
                if params.subpixel_rendering {
                    // We must always return RGBA data when subpixel rendering is requested.
                    let expanded = image.data.iter().flat_map(|&a| [a, a, a, a]).collect();
                    Ok((bitmap_size, expanded))
                } else {
                    Ok((bitmap_size, image.data))
                }
            }
        }
    }

    fn render_glyph_image(
        &mut self,
        params: &RenderGlyphParams,
    ) -> Result<swash::scale::image::Image> {
        let loaded_font = &self.loaded_fonts[params.font_id.0];
        let font_ref = loaded_font.font.as_swash();
        let pixel_size = f32::from(params.font_size);

        let subpixel_offset = Vector::new(
            params.subpixel_variant.x as f32 / SUBPIXEL_VARIANTS_X as f32 / params.scale_factor,
            params.subpixel_variant.y as f32 / SUBPIXEL_VARIANTS_Y as f32 / params.scale_factor,
        );

        let coords = loaded_font.coords.clone();
        let mut scaler = self
            .swash_scale_context
            .builder(font_ref)
            .normalized_coords(coords.iter().copied())
            .size(pixel_size * params.scale_factor)
            .hint(true)
            .build();

        let sources: &[Source] = if params.is_emoji {
            &[
                Source::ColorOutline(0),
                Source::ColorBitmap(StrikeWith::BestFit),
                Source::Outline,
            ]
        } else {
            &[Source::Bitmap(StrikeWith::ExactSize), Source::Outline]
        };

        let mut renderer = Render::new(sources);
        if params.subpixel_rendering {
            // There seems to be a bug in Swash where the B and R values are swapped.
            renderer
                .format(Format::subpixel_bgra())
                .offset(subpixel_offset);
        } else {
            renderer.format(Format::Alpha).offset(subpixel_offset);
        }

        let glyph_id: u16 = params.glyph_id.0.try_into()?;
        renderer
            .render(&mut scaler, glyph_id)
            .with_context(|| format!("unable to render glyph via swash for {params:?}"))
    }

    /// This is used when cosmic_text has chosen a fallback font instead of using the requested
    /// font, typically to handle some unicode characters. When this happens, `loaded_fonts` may not
    /// yet have an entry for this fallback font, and so one is added.
    ///
    /// Note that callers shouldn't use this `FontId` somewhere that will retrieve the corresponding
    /// `LoadedFont.features`, as it will have an arbitrarily chosen or empty value. The only
    /// current use of this field is for the *input* of `layout_line`, and so it's fine to use
    /// `font_id_for_cosmic_id` when computing the *output* of `layout_line`.
    fn font_id_for_cosmic_id(
        &mut self,
        id: cosmic_text::fontdb::ID,
        weight: gpui::FontWeight,
    ) -> Result<FontId> {
        // Matched on weight as well as face: a variable face is one fontdb id covering
        // every weight, so keying on the id alone would hand back whichever weight was
        // instantiated first.
        if let Some(ix) = self
            .loaded_fonts
            .iter()
            .position(|loaded_font| loaded_font.font.id() == id && loaded_font.weight == weight)
        {
            Ok(FontId(ix))
        } else {
            let font = self
                .font_system
                .get_font(id, cosmic_text::Weight(weight.0 as u16))
                .context("failed to get fallback font from cosmic-text font system")?;
            let face = self
                .font_system
                .db()
                .face(id)
                .context("fallback font face not found in cosmic-text database")?;

            let font_id = FontId(self.loaded_fonts.len());
            let post_script_name = face.post_script_name.clone();
            let coords = weight_coords(&font, weight);
            self.loaded_fonts.push(LoadedFont {
                font,
                features: CosmicFontFeatures::new(),
                is_known_emoji_font: check_is_known_emoji_font(&post_script_name),
                user_fallback_chain: Arc::from(Vec::new()),
                weight,
                coords,
            });

            Ok(font_id)
        }
    }

    #[profiling::function]
    fn layout_line(&mut self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        if contains_paragraph_separator(text) {
            self.layout_line_with_separators(text, font_size, font_runs)
        } else {
            self.layout_line_no_separators(text, font_size, font_runs)
        }
    }

    fn layout_line_with_separators(
        &mut self,
        text: &str,
        font_size: Pixels,
        font_runs: &[FontRun],
    ) -> LineLayout {
        let mut layout = LineLayout {
            font_size,
            len: text.len(),
            ..Default::default()
        };
        let mut paragraph_start = 0;

        for (separator_start, separator) in text
            .char_indices()
            .filter(|(_, character)| is_paragraph_separator(*character))
        {
            let separator_end = separator_start + separator.len_utf8();
            self.shape_segment(
                text,
                paragraph_start..separator_start,
                font_size,
                font_runs,
                &mut layout,
            );
            self.shape_segment(
                text,
                separator_start..separator_end,
                font_size,
                font_runs,
                &mut layout,
            );
            paragraph_start = separator_end;
        }

        self.shape_segment(
            text,
            paragraph_start..text.len(),
            font_size,
            font_runs,
            &mut layout,
        );

        layout
    }

    fn shape_segment(
        &mut self,
        text: &str,
        range: Range<usize>,
        font_size: Pixels,
        font_runs: &[FontRun],
        layout: &mut LineLayout,
    ) {
        if range.is_empty() {
            return;
        }

        let segment_font_runs = clip_font_runs(font_runs, range.clone());
        let segment =
            self.layout_line_no_separators(&text[range.clone()], font_size, &segment_font_runs);

        let mut segment_runs = segment.runs;
        for run in &mut segment_runs {
            for glyph in &mut run.glyphs {
                glyph.index += range.start;
                glyph.position.x += layout.width;
            }
        }

        for mut run in segment_runs {
            if let Some(same_run) = layout
                .runs
                .last_mut()
                .filter(|last| last.font_id == run.font_id)
            {
                same_run.glyphs.append(&mut run.glyphs);
            } else {
                layout.runs.push(run);
            }
        }

        layout.width += segment.width;
        layout.ascent = layout.ascent.max(segment.ascent);
        layout.descent = layout.descent.max(segment.descent);
    }

    fn layout_line_no_separators(
        &mut self,
        text: &str,
        font_size: Pixels,
        font_runs: &[FontRun],
    ) -> LineLayout {
        let mut attrs_list = AttrsList::new(&Attrs::new());
        let mut offs = 0;
        for run in font_runs {
            let run_end = offs + run.len;

            let loaded_font = self.loaded_font(run.font_id);
            let Some(face) = self.font_system.db().face(loaded_font.font.id()) else {
                log::warn!(
                    "font face not found in database for font_id {:?}",
                    run.font_id
                );
                offs = run_end;
                continue;
            };
            let Some(first_family) = face.families.first() else {
                log::warn!(
                    "font face has no family names for font_id {:?}",
                    run.font_id
                );
                offs = run_end;
                continue;
            };

            let primary_family_name: SharedString = first_family.0.clone().into();
            let primary_stretch = face.stretch;
            let primary_style = face.style;
            // The primary span keeps the matched face's weight: that face was already
            // chosen for this request, so its attrs should describe it.
            let primary_weight = face.weight;
            // The fallback spans must not. They are separate families that nothing has
            // matched yet, and reusing the primary's weight silently clamps them to
            // whatever the primary family happens to ship, so a fallback with a 900 face
            // is unreachable behind a primary that stops at 800.
            let requested_weight = cosmic_text::Weight(loaded_font.weight.0 as u16);
            let primary_features = loaded_font.features.clone();
            let fallback_chain = Arc::clone(&loaded_font.user_fallback_chain);

            // build one `Attrs` per slot up front. each clone of span attrs
            // would otherwise re-allocate the `font_features` Vec.
            let primary_attrs = Attrs::new()
                .metadata(run.font_id.0)
                .family(Family::Name(&primary_family_name))
                .stretch(primary_stretch)
                .style(primary_style)
                .weight(primary_weight)
                .font_features(primary_features.clone());
            let fallback_attrs: SmallVec<[Attrs<'_>; 4]> = fallback_chain
                .iter()
                .map(|(fb_id, fb_name)| {
                    Attrs::new()
                        .metadata(fb_id.0)
                        .family(Family::Name(fb_name))
                        .stretch(primary_stretch)
                        .style(primary_style)
                        .weight(requested_weight)
                        .font_features(primary_features.clone())
                })
                .collect();

            let spans = if fallback_chain.is_empty() {
                let mut spans = SmallVec::<[RunSpan; 4]>::new();
                spans.push(RunSpan {
                    start: offs,
                    end: run_end,
                    slot: None,
                    font_id: run.font_id,
                });
                spans
            } else {
                let loaded_fonts = &self.loaded_fonts;
                let covers = |id: FontId, ch: char| charmap_covers(loaded_fonts, id, ch);
                compute_run_spans(text, offs, run.len, run.font_id, &fallback_chain, &covers)
            };

            for span in spans {
                let attrs = match span.slot {
                    None => &primary_attrs,
                    Some(ix) => &fallback_attrs[ix],
                };
                attrs_list.add_span(span.start..span.end, attrs);
            }
            offs = run_end;
        }

        let line = ShapeLine::new(
            &mut self.font_system,
            text,
            &attrs_list,
            cosmic_text::Shaping::Advanced,
            4,
        );
        let mut layout_lines = Vec::with_capacity(1);
        line.layout_to_buffer(
            &mut self.scratch,
            f32::from(font_size),
            None, // We do our own wrapping
            cosmic_text::Wrap::None,
            Ellipsize::None,
            None,
            &mut layout_lines,
            None,
            cosmic_text::Hinting::Disabled,
        );

        let Some(layout) = layout_lines.first() else {
            return LineLayout {
                font_size,
                width: Pixels::ZERO,
                ascent: Pixels::ZERO,
                descent: Pixels::ZERO,
                runs: Vec::new(),
                len: text.len(),
            };
        };

        let mut runs: Vec<ShapedRun> = Vec::new();
        for glyph in &layout.glyphs {
            let mut font_id = FontId(glyph.metadata);
            let mut loaded_font = self.loaded_font(font_id);
            if loaded_font.font.id() != glyph.font_id {
                let requested_weight = loaded_font.weight;
                match self.font_id_for_cosmic_id(glyph.font_id, requested_weight) {
                    std::result::Result::Ok(resolved_id) => {
                        font_id = resolved_id;
                        loaded_font = self.loaded_font(font_id);
                    }
                    Err(error) => {
                        log::warn!(
                            "failed to resolve cosmic font id {:?}: {error:#}",
                            glyph.font_id
                        );
                        continue;
                    }
                }
            }
            let is_emoji = loaded_font.is_known_emoji_font;

            // HACK: Prevent crash caused by variation selectors.
            if glyph.glyph_id == 3 && is_emoji {
                continue;
            }

            let shaped_glyph = ShapedGlyph {
                id: GlyphId(glyph.glyph_id as u32),
                position: point(glyph.x.into(), glyph.y.into()),
                index: glyph.start,
                is_emoji,
            };

            if let Some(last_run) = runs
                .last_mut()
                .filter(|last_run| last_run.font_id == font_id)
            {
                last_run.glyphs.push(shaped_glyph);
            } else {
                runs.push(ShapedRun {
                    font_id,
                    glyphs: vec![shaped_glyph],
                });
            }
        }

        LineLayout {
            font_size,
            width: layout.w.into(),
            ascent: layout.max_ascent.into(),
            descent: layout.max_descent.into(),
            runs,
            len: text.len(),
        }
    }
}

#[inline(always)]
fn is_paragraph_separator(character: char) -> bool {
    unicode_bidi::bidi_class(character) == unicode_bidi::BidiClass::B
}

fn contains_paragraph_separator(text: &str) -> bool {
    if text
        .bytes()
        .any(|byte| matches!(byte, b'\n' | b'\r' | 0x1c | 0x1d | 0x1e))
    {
        return true;
    }

    !text.is_ascii() && text.chars().any(is_paragraph_separator)
}

fn clip_font_runs(font_runs: &[FontRun], range: Range<usize>) -> SmallVec<[FontRun; 4]> {
    let mut clipped = SmallVec::new();
    let mut offs = 0;
    for run in font_runs {
        let run_start = offs;
        offs += run.len;
        if offs <= range.start {
            continue;
        }
        if run_start >= range.end {
            break;
        }
        let start = run_start.max(range.start);
        let end = offs.min(range.end);
        if start < end {
            clipped.push(FontRun {
                len: end - start,
                font_id: run.font_id,
            });
        }
    }
    clipped
}

#[cfg(feature = "font-kit")]
fn find_best_match(
    font: &Font,
    candidates: &[FontId],
    state: &CosmicTextSystemState,
) -> Result<usize> {
    let candidate_properties = candidates
        .iter()
        .map(|font_id| {
            let database_id = state.loaded_font(*font_id).font.id();
            let face_info = state
                .font_system
                .db()
                .face(database_id)
                .context("font face not found in database")?;
            Ok(face_info_into_properties(face_info))
        })
        .collect::<Result<SmallVec<[_; 4]>>>()?;

    let ix =
        font_kit::matching::find_best_match(&candidate_properties, &font_into_properties(font))
            .context("requested font family contains no font matching the other parameters")?;

    Ok(ix)
}

#[cfg(not(feature = "font-kit"))]
fn find_best_match(
    font: &Font,
    candidates: &[FontId],
    state: &CosmicTextSystemState,
) -> Result<usize> {
    if candidates.is_empty() {
        anyhow::bail!("requested font family contains no font matching the other parameters");
    }
    if candidates.len() == 1 {
        return Ok(0);
    }

    let target_weight = font.weight.0;
    let target_italic = matches!(
        font.style,
        gpui::FontStyle::Italic | gpui::FontStyle::Oblique
    );

    let mut best_index = 0;
    let mut best_score = u32::MAX;

    for (index, font_id) in candidates.iter().enumerate() {
        let database_id = state.loaded_font(*font_id).font.id();
        let face_info = state
            .font_system
            .db()
            .face(database_id)
            .context("font face not found in database")?;

        let is_italic = matches!(
            face_info.style,
            cosmic_text::Style::Italic | cosmic_text::Style::Oblique
        );
        let style_penalty: u32 = if is_italic == target_italic { 0 } else { 1000 };
        let weight_diff = (face_info.weight.0 as i32 - target_weight as i32).unsigned_abs();
        let score = style_penalty + weight_diff;

        if score < best_score {
            best_score = score;
            best_index = index;
        }
    }

    Ok(best_index)
}

/// one contiguous slice of a `FontRun` that maps to a single slot. `slot` is
/// `None` for the primary font and `Some(ix)` for `fallback_chain[ix]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RunSpan {
    start: usize,
    end: usize,
    slot: Option<usize>,
    font_id: FontId,
}

/// walks `text[run_offset..run_offset + run_len]` and groups codepoints into
/// spans. inheriting codepoints stay in the current span so shaping clusters
/// like emoji zwj sequences and combining marks are not torn apart.
fn compute_run_spans(
    text: &str,
    run_offset: usize,
    run_len: usize,
    primary: FontId,
    fallback_chain: &[(FontId, SharedString)],
    covers: &impl Fn(FontId, char) -> bool,
) -> SmallVec<[RunSpan; 4]> {
    let mut spans = SmallVec::new();
    let run_end = run_offset + run_len;
    if run_end <= run_offset {
        return spans;
    }
    if fallback_chain.is_empty() {
        spans.push(RunSpan {
            start: run_offset,
            end: run_end,
            slot: None,
            font_id: primary,
        });
        return spans;
    }
    let run_text = &text[run_offset..run_end];
    let mut span_start = run_offset;
    let mut span_slot: Option<usize> = None;
    let mut span_font_id = primary;
    for (grapheme_idx, grapheme) in run_text.grapheme_indices(true) {
        let abs = run_offset + grapheme_idx;
        let ch = grapheme.chars().next().unwrap_or('\0');
        let next_slot = pick_covering_slot(ch, span_slot, primary, fallback_chain, covers);
        if next_slot == span_slot {
            continue;
        }
        if abs > span_start {
            spans.push(RunSpan {
                start: span_start,
                end: abs,
                slot: span_slot,
                font_id: span_font_id,
            });
        }
        span_start = abs;
        span_slot = next_slot;
        span_font_id = slot_font_id(next_slot, primary, fallback_chain);
    }
    if span_start < run_end {
        spans.push(RunSpan {
            start: span_start,
            end: run_end,
            slot: span_slot,
            font_id: span_font_id,
        });
    }
    spans
}

fn slot_font_id(
    slot: Option<usize>,
    primary: FontId,
    fallback_chain: &[(FontId, SharedString)],
) -> FontId {
    match slot {
        None => primary,
        Some(ix) => fallback_chain[ix].0,
    }
}

fn pick_covering_slot(
    ch: char,
    current: Option<usize>,
    primary: FontId,
    fallback_chain: &[(FontId, SharedString)],
    covers: &impl Fn(FontId, char) -> bool,
) -> Option<usize> {
    if (ch as u32) <= 0x7F {
        return None;
    }
    if covers(primary, ch) {
        return None;
    }
    let current_id = slot_font_id(current, primary, fallback_chain);
    if covers(current_id, ch) {
        return current;
    }

    fallback_chain
        .iter()
        .position(|(fb_id, _)| covers(*fb_id, ch))
}

fn charmap_covers(loaded_fonts: &[LoadedFont], id: FontId, ch: char) -> bool {
    loaded_fonts
        .get(id.0)
        .is_some_and(|loaded| loaded.font.as_swash().charmap().map(ch) != 0)
}

fn cosmic_font_features(features: &FontFeatures) -> Result<CosmicFontFeatures> {
    let mut result = CosmicFontFeatures::new();
    for feature in features.0.iter() {
        let name_bytes: [u8; 4] = feature
            .0
            .as_bytes()
            .try_into()
            .context("Incorrect feature flag format")?;

        let tag = cosmic_text::FeatureTag::new(&name_bytes);

        result.set(tag, feature.1);
    }
    Ok(result)
}

/// Normalized `wght` coordinates for `weight`, or empty when the face has no weight
/// axis or already sits at that weight. Empty means "nothing to vary", which keeps
/// static families on the single shared `LoadedFont` they have always used.
fn weight_coords(
    font: &CosmicTextFont,
    weight: gpui::FontWeight,
) -> SmallVec<[NormalizedCoord; 1]> {
    let font_ref = font.as_swash();
    let variations = font_ref.variations();
    if variations.find_by_tag(tag_from_bytes(b"wght")).is_none() {
        return SmallVec::new();
    }

    let coords: SmallVec<[NormalizedCoord; 1]> =
        variations.normalized_coords([("wght", weight.0)]).collect();
    if coords.iter().all(|coord| *coord == 0) {
        return SmallVec::new();
    }
    coords
}

#[cfg(feature = "font-kit")]
fn font_into_properties(font: &gpui::Font) -> font_kit::properties::Properties {
    font_kit::properties::Properties {
        style: match font.style {
            gpui::FontStyle::Normal => font_kit::properties::Style::Normal,
            gpui::FontStyle::Italic => font_kit::properties::Style::Italic,
            gpui::FontStyle::Oblique => font_kit::properties::Style::Oblique,
        },
        weight: font_kit::properties::Weight(font.weight.0),
        stretch: Default::default(),
    }
}

#[cfg(feature = "font-kit")]
fn face_info_into_properties(
    face_info: &cosmic_text::fontdb::FaceInfo,
) -> font_kit::properties::Properties {
    font_kit::properties::Properties {
        style: match face_info.style {
            cosmic_text::Style::Normal => font_kit::properties::Style::Normal,
            cosmic_text::Style::Italic => font_kit::properties::Style::Italic,
            cosmic_text::Style::Oblique => font_kit::properties::Style::Oblique,
        },
        weight: font_kit::properties::Weight(face_info.weight.0.into()),
        stretch: match face_info.stretch {
            cosmic_text::Stretch::Condensed => font_kit::properties::Stretch::CONDENSED,
            cosmic_text::Stretch::Expanded => font_kit::properties::Stretch::EXPANDED,
            cosmic_text::Stretch::ExtraCondensed => font_kit::properties::Stretch::EXTRA_CONDENSED,
            cosmic_text::Stretch::ExtraExpanded => font_kit::properties::Stretch::EXTRA_EXPANDED,
            cosmic_text::Stretch::Normal => font_kit::properties::Stretch::NORMAL,
            cosmic_text::Stretch::SemiCondensed => font_kit::properties::Stretch::SEMI_CONDENSED,
            cosmic_text::Stretch::SemiExpanded => font_kit::properties::Stretch::SEMI_EXPANDED,
            cosmic_text::Stretch::UltraCondensed => font_kit::properties::Stretch::ULTRA_CONDENSED,
            cosmic_text::Stretch::UltraExpanded => font_kit::properties::Stretch::ULTRA_EXPANDED,
        },
    }
}

fn check_is_known_emoji_font(postscript_name: &str) -> bool {
    // TODO: Include other common emoji fonts
    postscript_name == "NotoColorEmoji"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fid(i: usize) -> FontId {
        FontId(i)
    }

    fn chain(ids: &[usize]) -> SmallVec<[(FontId, SharedString); 4]> {
        ids.iter()
            .map(|&i| (fid(i), SharedString::from(format!("fb{i}"))))
            .collect()
    }

    fn span(start: usize, end: usize, slot: Option<usize>, font_id: FontId) -> RunSpan {
        RunSpan {
            start,
            end,
            slot,
            font_id,
        }
    }

    /// A 1KB generated variable font with a single `wght` axis; see
    /// `crates/gpui_wgpu/test_data/README.md`. Its one glyph, `A`, is a rectangle
    /// whose width and advance both grow with the weight, so a weight that fails
    /// to reach the rasterizer shows up as an identical bitmap.
    const VAR_TEST: &[u8] = include_bytes!("../test_data/gpui-var-test.ttf");

    const IBM_PLEX: &[u8] =
        include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf");

    /// Every code point of `Bidi_Class=B`, each of which starts a new bidi
    /// paragraph and so can split one line into mixed-direction paragraphs.
    const SEPARATORS: &[char] = &[
        '\u{000a}', '\u{000d}', '\u{001c}', '\u{001d}', '\u{001e}', '\u{0085}', '\u{2029}',
    ];

    fn text_system() -> Result<CosmicTextSystem> {
        let text_system = CosmicTextSystem::new_without_system_fonts("IBM Plex Sans");
        text_system.add_fonts(vec![Cow::Borrowed(IBM_PLEX)])?;
        Ok(text_system)
    }

    fn layout_text(text_system: &CosmicTextSystem, text: &str) -> Result<LineLayout> {
        let font_id = text_system.font_id(&gpui::font("IBM Plex Sans"))?;
        let runs = [FontRun {
            len: text.len(),
            font_id,
        }];
        Ok(text_system.layout_line(text, gpui::px(14.0), &runs))
    }

    /// Mirrors the original crash: mixed-direction text reaching the shaper
    /// through `shape_text`, which only splits lines on `\n`.
    #[test]
    fn shape_text_with_mixed_direction_paragraphs() -> Result<()> {
        let platform_text_system = Arc::new(text_system()?);
        let text_system = Arc::new(gpui::TextSystem::new(platform_text_system));
        let window_text_system = gpui::WindowTextSystem::new(text_system);

        let text: SharedString = "first line\n\u{05d0}\u{001c}A".into();
        let runs = [gpui::TextRun {
            len: text.len(),
            font: gpui::font("IBM Plex Sans"),
            ..Default::default()
        }];

        let lines = window_text_system.shape_text(text, gpui::px(14.0), &runs, None, None)?;

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[1].len(), "\u{05d0}\u{001c}A".len());
        assert!(lines[1].width() > Pixels::ZERO);
        Ok(())
    }

    #[test]
    fn layout_line_with_mixed_direction_paragraphs() -> Result<()> {
        let text_system = text_system()?;

        for separator in SEPARATORS {
            for text in [
                format!("\u{05d0}{separator}A"),
                format!("A{separator}\u{05d0}"),
            ] {
                let layout = layout_text(&text_system, &text)?;

                assert_eq!(layout.len, text.len(), "{text:?}");
                assert!(layout.width > Pixels::ZERO, "{text:?}");
                assert!(
                    layout.runs.iter().any(|run| !run.glyphs.is_empty()),
                    "{text:?}"
                );
            }
        }

        Ok(())
    }

    #[test]
    fn layout_line_with_separators_at_line_edges() -> Result<()> {
        let text_system = text_system()?;

        for text in [
            "\u{001c}",
            "\u{001c}\u{001c}",
            "\u{001c}\u{05d0}",
            "\u{05d0}\u{001c}",
            "\u{05d0}\u{001c}\u{001c}A",
            "\u{001c}\u{05d0}\u{001c}A\u{001c}",
        ] {
            let layout = layout_text(&text_system, text)?;
            assert_eq!(layout.len, text.len(), "{text:?}");
        }

        Ok(())
    }

    /// Glyph indices must stay absolute and positions ordered across segment
    /// boundaries, otherwise cursor placement and hit testing desync. Uses
    /// single-direction text so visual order matches logical order.
    #[test]
    fn layout_line_keeps_indices_and_positions_ordered_across_paragraphs() -> Result<()> {
        let text_system = text_system()?;
        let text = "ab\u{001c}cd\u{2029}ef";
        let layout = layout_text(&text_system, text)?;

        let glyphs: Vec<_> = layout.runs.iter().flat_map(|run| &run.glyphs).collect();
        assert!(!glyphs.is_empty());

        for glyph in &glyphs {
            assert!(glyph.index < text.len(), "{:?}", glyph.index);
            assert!(text.is_char_boundary(glyph.index), "{:?}", glyph.index);
        }
        for pair in glyphs.windows(2) {
            assert!(pair[0].index < pair[1].index);
            assert!(pair[0].position.x <= pair[1].position.x);
        }

        // Every segment contributes width, so the whole line is wider than its
        // leading paragraph alone.
        assert!(layout.width > layout_text(&text_system, "ab")?.width);
        Ok(())
    }

    /// A font run boundary that does not line up with a paragraph boundary must
    /// still be clipped to the right segments.
    #[test]
    fn layout_line_with_font_run_straddling_a_separator() -> Result<()> {
        let text_system = text_system()?;
        let font_id = text_system.font_id(&gpui::font("IBM Plex Sans"))?;
        let text = "ab\u{001c}\u{05d0}\u{05d1}";

        // The run boundary falls inside the trailing RTL paragraph.
        let runs = [
            FontRun {
                len: "ab\u{001c}\u{05d0}".len(),
                font_id,
            },
            FontRun {
                len: "\u{05d1}".len(),
                font_id,
            },
        ];
        let layout = text_system.layout_line(text, gpui::px(14.0), &runs);

        assert_eq!(layout.len, text.len());
        assert!(layout.width > Pixels::ZERO);
        Ok(())
    }

    /// Lines with no separator take the fast path and must be shaped exactly as
    /// they were before paragraph splitting existed.
    #[test]
    fn layout_line_without_separators_takes_fast_path() -> Result<()> {
        let text_system = text_system()?;

        for text in [
            "hello world",
            "\u{05d0}\u{05d1}\u{05d2}",
            "mixed \u{05d0}\u{05d1}",
        ] {
            assert!(!contains_paragraph_separator(text), "{text:?}");
            let layout = layout_text(&text_system, text)?;
            assert_eq!(layout.len, text.len(), "{text:?}");
            assert!(layout.width > Pixels::ZERO, "{text:?}");
        }

        Ok(())
    }

    #[test]
    fn paragraph_separator_detection() {
        for separator in SEPARATORS {
            assert!(is_paragraph_separator(*separator), "{separator:?}");
            assert!(contains_paragraph_separator(&format!("a{separator}b")));
        }

        for text in [
            "",
            "plain ascii",
            "\u{05d0}",
            "tab\there",
            "emoji \u{1f600}",
        ] {
            assert!(!contains_paragraph_separator(text), "{text:?}");
        }
    }

    #[test]
    fn font_runs_are_clipped_to_segment() {
        let runs = [
            FontRun {
                len: 3,
                font_id: fid(1),
            },
            FontRun {
                len: 4,
                font_id: fid(2),
            },
        ];

        assert_eq!(clip_font_runs(&runs, 0..7).as_slice(), &runs);
        assert_eq!(
            clip_font_runs(&runs, 2..5).as_slice(),
            &[
                FontRun {
                    len: 1,
                    font_id: fid(1)
                },
                FontRun {
                    len: 2,
                    font_id: fid(2)
                },
            ]
        );
        assert_eq!(
            clip_font_runs(&runs, 3..7).as_slice(),
            &[FontRun {
                len: 4,
                font_id: fid(2)
            }]
        );
        assert!(clip_font_runs(&runs, 5..5).is_empty());
    }

    #[test]
    fn primary_wins_over_current_fallback_when_primary_covers() {
        let primary = fid(0);
        let fb = chain(&[1, 2]);
        let covers = |id: FontId, _: char| id == fid(0) || id == fid(1);
        assert_eq!(
            pick_covering_slot('a', Some(0), primary, &fb, &covers),
            None
        );
    }

    #[test]
    fn primary_preferred_over_fallback_when_both_cover() {
        let primary = fid(0);
        let fb = chain(&[1]);
        let covers = |_: FontId, _: char| true;
        assert_eq!(pick_covering_slot('a', None, primary, &fb, &covers), None);
    }

    #[test]
    fn falls_through_chain_in_order() {
        let primary = fid(0);
        let fb = chain(&[1, 2, 3]);
        // only fallback 2 at index 1 covers.
        let covers = |id: FontId, _: char| id == fid(2);
        assert_eq!(
            pick_covering_slot('字', None, primary, &fb, &covers),
            Some(1)
        );
    }

    #[test]
    fn no_coverage_returns_primary() {
        let primary = fid(0);
        let fb = chain(&[1, 2]);
        let covers = |_: FontId, _: char| false;
        // nothing covers. return `None` so the `cosmic-text` built in script
        // fallback can take over during shaping.
        assert_eq!(
            pick_covering_slot('\u{1F600}', Some(1), primary, &fb, &covers),
            None
        );
    }

    #[test]
    fn empty_chain_always_returns_primary() {
        let primary = fid(0);
        let fb: SmallVec<[(FontId, SharedString); 4]> = SmallVec::new();
        let covers = |_: FontId, _: char| false;
        assert_eq!(pick_covering_slot('a', None, primary, &fb, &covers), None);
    }

    #[test]
    fn slot_font_id_resolution() {
        let primary = fid(7);
        let fb = chain(&[10, 20]);
        assert_eq!(slot_font_id(None, primary, &fb), fid(7));
        assert_eq!(slot_font_id(Some(0), primary, &fb), fid(10));
        assert_eq!(slot_font_id(Some(1), primary, &fb), fid(20));
    }

    #[test]
    fn run_spans_with_no_chain_emit_one_primary_span() {
        let primary = fid(0);
        let fb: SmallVec<[(FontId, SharedString); 4]> = SmallVec::new();
        let covers = |_: FontId, _: char| false;
        let text = "hello";
        let spans = compute_run_spans(text, 0, text.len(), primary, &fb, &covers);
        assert_eq!(spans.as_slice(), &[span(0, text.len(), None, primary)]);
    }

    #[test]
    fn run_spans_use_byte_offsets_for_multibyte_chars() {
        let primary = fid(0);
        let fb = chain(&[1]);
        // primary covers ascii. fallback covers cjk.
        let covers = |id: FontId, ch: char| {
            if id == primary {
                ch.is_ascii()
            } else {
                !ch.is_ascii()
            }
        };
        let text = "a字b";
        let spans = compute_run_spans(text, 0, text.len(), primary, &fb, &covers);
        // '字' is 3 bytes so split is at 1 then 4.
        assert_eq!(
            spans.as_slice(),
            &[
                span(0, 1, None, primary),
                span(1, 4, Some(0), fid(1)),
                span(4, 5, None, primary),
            ]
        );
    }

    #[test]
    fn run_spans_respect_run_offset() {
        let primary = fid(0);
        let fb = chain(&[1]);
        let covers = |id: FontId, ch: char| {
            if id == primary {
                ch.is_ascii()
            } else {
                !ch.is_ascii()
            }
        };
        // outer text has a prefix that is not part of this run.
        let text = "xx字y";
        let run_offset = 2;
        let run_len = text.len() - run_offset;
        let spans = compute_run_spans(text, run_offset, run_len, primary, &fb, &covers);
        assert_eq!(
            spans.as_slice(),
            &[span(2, 5, Some(0), fid(1)), span(5, 6, None, primary)]
        );
    }

    #[test]
    fn run_spans_keep_combining_marks_with_base_in_fallback() {
        let primary = fid(0);
        let fb = chain(&[1]);
        // primary covers ascii only. fallback covers the base char.
        // combining mark must stay in the fallback span even when fallback
        // does not advertise coverage of it.
        let covers = |id: FontId, ch: char| {
            if id == primary {
                ch.is_ascii()
            } else {
                ch == '\u{0905}'
            }
        };
        // \u{0905} devanagari short a + \u{0902} candrabindu mark.
        let text = "\u{0905}\u{0902}";
        let spans = compute_run_spans(text, 0, text.len(), primary, &fb, &covers);
        assert_eq!(spans.as_slice(), &[span(0, text.len(), Some(0), fid(1))]);
    }

    #[test]
    fn run_spans_keep_zwj_inside_emoji_cluster() {
        let primary = fid(0);
        let fb = chain(&[1]);
        // only fallback covers the emoji codepoints. zwj must not split.
        let covers = |id: FontId, ch: char| id == fid(1) && ch != '\u{200D}';
        // family zwj sequence woman zwj girl.
        let text = "\u{1F469}\u{200D}\u{1F467}";
        let spans = compute_run_spans(text, 0, text.len(), primary, &fb, &covers);
        assert_eq!(spans.as_slice(), &[span(0, text.len(), Some(0), fid(1))]);
    }

    #[test]
    fn run_spans_collapse_adjacent_same_slot() {
        let primary = fid(0);
        let fb = chain(&[1]);
        let covers = |id: FontId, ch: char| {
            if id == primary {
                ch.is_ascii()
            } else {
                !ch.is_ascii()
            }
        };
        let text = "字字字";
        let spans = compute_run_spans(text, 0, text.len(), primary, &fb, &covers);
        assert_eq!(spans.as_slice(), &[span(0, text.len(), Some(0), fid(1))]);
    }

    #[test]
    fn run_spans_empty_run_returns_no_spans() {
        let primary = fid(0);
        let fb = chain(&[1]);
        let covers = |_: FontId, _: char| true;
        let spans = compute_run_spans("anything", 3, 0, primary, &fb, &covers);
        assert!(spans.is_empty());
    }
    fn weighted(family: &str, weight: gpui::FontWeight) -> Font {
        let mut font = gpui::font(family);
        font.weight = weight;
        font
    }

    fn rasterize_a(
        text_system: &CosmicTextSystem,
        font_id: FontId,
    ) -> (Size<DevicePixels>, Vec<u8>) {
        let glyph_id = text_system
            .glyph_for_char(font_id, 'A')
            .expect("test font has an 'A'");
        let params = RenderGlyphParams {
            font_id,
            glyph_id,
            font_size: gpui::px(32.0),
            subpixel_variant: point(0, 0),
            scale_factor: 1.0,
            is_emoji: false,
            subpixel_rendering: false,
            dilation: 0,
        };
        let bounds = text_system.glyph_raster_bounds(&params).unwrap();
        text_system.rasterize_glyph(&params, bounds).unwrap()
    }

    /// A variable font exposes one face at its default weight, so the requested
    /// weight has to reach the rasterizer as a variation setting. Without that,
    /// every weight renders the 400 outline.
    #[test]
    fn variable_font_rasterizes_at_the_requested_weight() {
        let text_system = CosmicTextSystem::new_without_system_fonts("GpuiVarTest");
        text_system
            .add_fonts(vec![Cow::Borrowed(VAR_TEST)])
            .unwrap();

        let thin = text_system
            .font_id(&weighted("GpuiVarTest", gpui::FontWeight::THIN))
            .unwrap();
        let black = text_system
            .font_id(&weighted("GpuiVarTest", gpui::FontWeight::BLACK))
            .unwrap();

        let (thin_size, thin_bytes) = rasterize_a(&text_system, thin);
        let (black_size, black_bytes) = rasterize_a(&text_system, black);

        let thin_ink: u64 = thin_bytes.iter().map(|b| *b as u64).sum();
        let black_ink: u64 = black_bytes.iter().map(|b| *b as u64).sum();

        assert!(
            black_ink > thin_ink,
            "wght=900 should paint more ink than wght=100, \
             got {black_ink} vs {thin_ink} (sizes {black_size:?} and {thin_size:?})"
        );
    }

    /// The reported shape of #60155: a static primary with a variable family in
    /// `*_font_fallbacks`. The fallback chain is resolved at load time, before any
    /// weight exists, so its glyphs used to render at the file's default weight no
    /// matter what was configured.
    #[test]
    fn variable_fallback_font_follows_the_requested_weight() {
        fn fallback_ink(weight: gpui::FontWeight) -> u64 {
            let text_system = CosmicTextSystem::new_without_system_fonts("IBM Plex Sans");
            text_system
                .add_fonts(vec![Cow::Borrowed(IBM_PLEX), Cow::Borrowed(VAR_TEST)])
                .unwrap();

            let mut font = weighted("IBM Plex Sans", weight);
            font.fallbacks = Some(FontFallbacks::from_fonts(vec!["GpuiVarTest".into()]));
            let font_id = text_system.font_id(&font).unwrap();

            // U+4E2D is absent from IBM Plex Sans and present in the test font, so the
            // run has to come out of the fallback chain.
            let layout = text_system.layout_line(
                "\u{4e2d}",
                gpui::px(32.0),
                &[FontRun {
                    len: "\u{4e2d}".len(),
                    font_id,
                }],
            );

            let run = layout
                .runs
                .iter()
                .find(|run| run.font_id != font_id)
                .expect("U+4E2D must be shaped by the fallback, not the primary");
            let glyph = run.glyphs.first().expect("fallback run has a glyph");

            let params = RenderGlyphParams {
                font_id: run.font_id,
                glyph_id: glyph.id,
                font_size: gpui::px(32.0),
                subpixel_variant: point(0, 0),
                scale_factor: 1.0,
                is_emoji: false,
                subpixel_rendering: false,
                dilation: 0,
            };
            let bounds = text_system.glyph_raster_bounds(&params).unwrap();
            let (_, bytes) = text_system.rasterize_glyph(&params, bounds).unwrap();
            bytes.iter().map(|b| *b as u64).sum()
        }

        let thin = fallback_ink(gpui::FontWeight::THIN);
        let black = fallback_ink(gpui::FontWeight::BLACK);
        assert!(
            black > thin,
            "a variable fallback must follow the requested weight, got {black} ink at \
             wght=900 against {thin} at wght=100"
        );
    }
}
