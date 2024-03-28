use crate::{
    point, size, Bounds, DevicePixels, Font, FontFeatures, FontId, FontMetrics, FontRun, FontStyle,
    FontWeight, GlyphId, LineLayout, Pixels, PlatformTextSystem, Point, RenderGlyphParams,
    ShapedGlyph, SharedString, Size,
};
use anyhow::{anyhow, Context, Ok, Result};
use collections::HashMap;
use cosmic_text::{
    Attrs, AttrsList, BufferLine, CacheKey, Family, Font as CosmicTextFont, FontSystem, SwashCache,
};

use itertools::Itertools;
use parking_lot::RwLock;
use pathfinder_geometry::{
    rect::{RectF, RectI},
    vector::{Vector2F, Vector2I},
};
use smallvec::SmallVec;
use std::{borrow::Cow, sync::Arc};

pub(crate) struct LinuxTextSystem(RwLock<LinuxTextSystemState>);

struct LinuxTextSystemState {
    swash_cache: SwashCache,
    font_system: FontSystem,
    /// Contains all already loaded fonts, including all faces. Indexed by `FontId`.
    loaded_fonts_store: Vec<Arc<CosmicTextFont>>,
    /// Caches the `FontId`s associated with a specific family to avoid iterating the font database
    /// for every font face in a family.
    font_ids_by_family_cache: HashMap<SharedString, SmallVec<[FontId; 4]>>,
    /// The name of each font associated with the given font id
    postscript_names: HashMap<FontId, String>,
}

impl LinuxTextSystem {
    pub(crate) fn new() -> Self {
        let mut font_system = FontSystem::new();

        // todo(linux) make font loading non-blocking
        font_system.db_mut().load_system_fonts();

        Self(RwLock::new(LinuxTextSystemState {
            font_system,
            swash_cache: SwashCache::new(),
            loaded_fonts_store: Vec::new(),
            font_ids_by_family_cache: HashMap::default(),
            postscript_names: HashMap::default(),
        }))
    }
}

impl Default for LinuxTextSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformTextSystem for LinuxTextSystem {
    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        self.0.write().add_fonts(fonts)
    }

    // todo(linux) ensure that this integrates with platform font loading
    // do we need to do more than call load_system_fonts()?
    fn all_font_names(&self) -> Vec<String> {
        self.0
            .read()
            .font_system
            .db()
            .faces()
            .map(|face| face.post_script_name.clone())
            .collect()
    }

    fn all_font_families(&self) -> Vec<String> {
        self.0
            .read()
            .font_system
            .db()
            .faces()
            // todo(linux) this will list the same font family multiple times
            .filter_map(|face| face.families.first().map(|family| family.0.clone()))
            .collect_vec()
    }

    fn font_id(&self, font: &Font) -> Result<FontId> {
        // todo(linux): Do we need to use CosmicText's Font APIs? Can we consolidate this to use font_kit?
        let mut state = self.0.write();

        let candidates = if let Some(font_ids) = state.font_ids_by_family_cache.get(&font.family) {
            font_ids.as_slice()
        } else {
            let font_ids = state.load_family(&font.family, font.features)?;
            state
                .font_ids_by_family_cache
                .insert(font.family.clone(), font_ids);
            state.font_ids_by_family_cache[&font.family].as_ref()
        };

        // todo(linux) ideally we would make fontdb's `find_best_match` pub instead of using font-kit here
        let candidate_properties = candidates
            .iter()
            .map(|font_id| {
                let database_id = state.loaded_fonts_store[font_id.0].id();
                let face_info = state.font_system.db().face(database_id).expect("");
                face_info_into_properties(face_info)
            })
            .collect::<SmallVec<[_; 4]>>();

        let ix =
            font_kit::matching::find_best_match(&candidate_properties, &font_into_properties(font))
                .context("requested font family contains no font matching the other parameters")?;

        Ok(candidates[ix])
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        let metrics = self.0.read().loaded_fonts_store[font_id.0]
            .as_swash()
            .metrics(&[]);

        FontMetrics {
            units_per_em: metrics.units_per_em as u32,
            ascent: metrics.ascent,
            descent: -metrics.descent, // todo(linux) confirm this is correct
            line_gap: metrics.leading,
            underline_position: metrics.underline_offset,
            underline_thickness: metrics.stroke_size,
            cap_height: metrics.cap_height,
            x_height: metrics.x_height,
            // todo(linux): Compute this correctly
            bounding_box: Bounds {
                origin: point(0.0, 0.0),
                size: size(metrics.max_width, metrics.ascent + metrics.descent),
            },
        }
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        let lock = self.0.read();
        let glyph_metrics = lock.loaded_fonts_store[font_id.0]
            .as_swash()
            .glyph_metrics(&[]);
        let glyph_id = glyph_id.0 as u16;
        // todo(linux): Compute this correctly
        // see https://github.com/servo/font-kit/blob/master/src/loaders/freetype.rs#L614-L620
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

    // todo(linux) Confirm that this has been superseded by the LineWrapper
    fn wrap_line(
        &self,
        _text: &str,
        _font_id: FontId,
        _font_size: Pixels,
        _width: Pixels,
    ) -> Vec<usize> {
        unimplemented!()
    }
}

impl LinuxTextSystemState {
    #[profiling::function]
    fn add_fonts(&mut self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        let db = self.font_system.db_mut();
        for bytes in fonts {
            match bytes {
                Cow::Borrowed(embedded_font) => {
                    db.load_font_data(embedded_font.to_vec());
                }
                Cow::Owned(bytes) => {
                    db.load_font_data(bytes);
                }
            }
        }
        Ok(())
    }

    // todo(linux) handle `FontFeatures`
    #[profiling::function]
    fn load_family(
        &mut self,
        name: &SharedString,
        _features: FontFeatures,
    ) -> Result<SmallVec<[FontId; 4]>> {
        let mut font_ids = SmallVec::new();
        let families = self
            .font_system
            .db()
            .faces()
            .filter(|face| face.families.iter().any(|family| *name == family.0))
            .map(|face| (face.id, face.post_script_name.clone()))
            .collect::<SmallVec<[_; 4]>>();

        for (font_id, postscript_name) in families {
            let font = self
                .font_system
                .get_font(font_id)
                .ok_or_else(|| anyhow!("Could not load font"))?;

            // HACK: to let the storybook run, we should actually do better font fallback
            if font.as_swash().charmap().map('m') == 0 || postscript_name == "Segoe Fluent Icons" {
                self.font_system.db_mut().remove_face(font.id());
                continue;
            };

            let font_id = FontId(self.loaded_fonts_store.len());
            font_ids.push(font_id);
            self.loaded_fonts_store.push(font);
            self.postscript_names.insert(font_id, postscript_name);
        }

        Ok(font_ids)
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        let width = self.loaded_fonts_store[font_id.0]
            .as_swash()
            .glyph_metrics(&[])
            .advance_width(glyph_id.0 as u16);
        let height = self.loaded_fonts_store[font_id.0]
            .as_swash()
            .glyph_metrics(&[])
            .advance_height(glyph_id.0 as u16);
        Ok(Size { width, height })
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        let glyph_id = self.loaded_fonts_store[font_id.0]
            .as_swash()
            .charmap()
            .map(ch);
        if glyph_id == 0 {
            None
        } else {
            Some(GlyphId(glyph_id.into()))
        }
    }

    fn is_emoji(&self, font_id: FontId) -> bool {
        // TODO: Include other common emoji fonts
        self.postscript_names
            .get(&font_id)
            .map_or(false, |postscript_name| postscript_name == "NotoColorEmoji")
    }

    fn raster_bounds(&mut self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        let font = &self.loaded_fonts_store[params.font_id.0];
        let font_system = &mut self.font_system;
        let image = self
            .swash_cache
            .get_image(
                font_system,
                CacheKey::new(
                    font.id(),
                    params.glyph_id.0 as u16,
                    (params.font_size * params.scale_factor).into(),
                    (0.0, 0.0),
                    cosmic_text::CacheKeyFlags::empty(),
                )
                .0,
            )
            .clone()
            .unwrap();
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
            Err(anyhow!("glyph bounds are empty"))
        } else {
            // todo(linux) handle subpixel variants
            let bitmap_size = glyph_bounds.size;
            let font = &self.loaded_fonts_store[params.font_id.0];
            let font_system = &mut self.font_system;
            let image = self
                .swash_cache
                .get_image(
                    font_system,
                    CacheKey::new(
                        font.id(),
                        params.glyph_id.0 as u16,
                        (params.font_size * params.scale_factor).into(),
                        (0.0, 0.0),
                        cosmic_text::CacheKeyFlags::empty(),
                    )
                    .0,
                )
                .clone()
                .unwrap();

            Ok((bitmap_size, image.data))
        }
    }

    fn font_id_for_cosmic_id(&mut self, id: cosmic_text::fontdb::ID) -> FontId {
        if let Some(ix) = self
            .loaded_fonts_store
            .iter()
            .position(|font| font.id() == id)
        {
            FontId(ix)
        } else {
            // This matches the behavior of the mac text system
            let font = self.font_system.get_font(id).unwrap();
            let face = self
                .font_system
                .db()
                .faces()
                .find(|info| info.id == id)
                .unwrap();

            let font_id = FontId(self.loaded_fonts_store.len());
            self.loaded_fonts_store.push(font);
            self.postscript_names
                .insert(font_id, face.post_script_name.clone());

            font_id
        }
    }

    // todo(linux) This is all a quick first pass, maybe we should be using cosmic_text::Buffer
    #[profiling::function]
    fn layout_line(&mut self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        let mut attrs_list = AttrsList::new(Attrs::new());
        let mut offs = 0;
        for run in font_runs {
            // todo(linux) We need to check we are doing utf properly
            let font = &self.loaded_fonts_store[run.font_id.0];
            let font = self.font_system.db().face(font.id()).unwrap();
            attrs_list.add_span(
                offs..(offs + run.len),
                Attrs::new()
                    .family(Family::Name(&font.families.first().unwrap().0))
                    .stretch(font.stretch)
                    .style(font.style)
                    .weight(font.weight),
            );
            offs += run.len;
        }
        let mut line = BufferLine::new(text, attrs_list, cosmic_text::Shaping::Advanced);

        let layout = line.layout(
            &mut self.font_system,
            font_size.0,
            f32::MAX, // We do our own wrapping
            cosmic_text::Wrap::None,
            None,
        );
        let mut runs = Vec::new();

        let layout = layout.first().unwrap();
        for glyph in &layout.glyphs {
            let font_id = glyph.font_id;
            let font_id = self.font_id_for_cosmic_id(font_id);
            let mut glyphs = SmallVec::new();
            // todo(linux) this is definitely wrong, each glyph in glyphs from cosmic-text is a cluster with one glyph, ShapedRun takes a run of glyphs with the same font and direction
            glyphs.push(ShapedGlyph {
                id: GlyphId(glyph.glyph_id as u32),
                position: point((glyph.x).into(), glyph.y.into()),
                index: glyph.start,
                is_emoji: self.is_emoji(font_id),
            });

            runs.push(crate::ShapedRun { font_id, glyphs });
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

impl From<RectF> for Bounds<f32> {
    fn from(rect: RectF) -> Self {
        Bounds {
            origin: point(rect.origin_x(), rect.origin_y()),
            size: size(rect.width(), rect.height()),
        }
    }
}

impl From<RectI> for Bounds<DevicePixels> {
    fn from(rect: RectI) -> Self {
        Bounds {
            origin: point(DevicePixels(rect.origin_x()), DevicePixels(rect.origin_y())),
            size: size(DevicePixels(rect.width()), DevicePixels(rect.height())),
        }
    }
}

impl From<Vector2I> for Size<DevicePixels> {
    fn from(value: Vector2I) -> Self {
        size(value.x().into(), value.y().into())
    }
}

impl From<RectI> for Bounds<i32> {
    fn from(rect: RectI) -> Self {
        Bounds {
            origin: point(rect.origin_x(), rect.origin_y()),
            size: size(rect.width(), rect.height()),
        }
    }
}

impl From<Point<u32>> for Vector2I {
    fn from(size: Point<u32>) -> Self {
        Vector2I::new(size.x as i32, size.y as i32)
    }
}

impl From<Vector2F> for Size<f32> {
    fn from(vec: Vector2F) -> Self {
        size(vec.x(), vec.y())
    }
}

impl From<FontWeight> for cosmic_text::Weight {
    fn from(value: FontWeight) -> Self {
        cosmic_text::Weight(value.0 as u16)
    }
}

impl From<FontStyle> for cosmic_text::Style {
    fn from(style: FontStyle) -> Self {
        match style {
            FontStyle::Normal => cosmic_text::Style::Normal,
            FontStyle::Italic => cosmic_text::Style::Italic,
            FontStyle::Oblique => cosmic_text::Style::Oblique,
        }
    }
}

fn font_into_properties(font: &crate::Font) -> font_kit::properties::Properties {
    font_kit::properties::Properties {
        style: match font.style {
            crate::FontStyle::Normal => font_kit::properties::Style::Normal,
            crate::FontStyle::Italic => font_kit::properties::Style::Italic,
            crate::FontStyle::Oblique => font_kit::properties::Style::Oblique,
        },
        weight: font_kit::properties::Weight(font.weight.0),
        stretch: Default::default(),
    }
}

fn face_info_into_properties(
    face_info: &cosmic_text::fontdb::FaceInfo,
) -> font_kit::properties::Properties {
    font_kit::properties::Properties {
        style: match face_info.style {
            cosmic_text::Style::Normal => font_kit::properties::Style::Normal,
            cosmic_text::Style::Italic => font_kit::properties::Style::Italic,
            cosmic_text::Style::Oblique => font_kit::properties::Style::Oblique,
        },
        // both libs use the same values for weight
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
