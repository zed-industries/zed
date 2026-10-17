use anyhow::anyhow;
use collections::{HashMap, HashSet};
use core_foundation::{
    array::{CFArray, CFArrayRef},
    attributed_string::CFMutableAttributedString,
    base::{CFRange, TCFType},
    number::CFNumber,
    string::CFString,
};
use core_graphics::{
    base::{CGFloat, CGGlyph, kCGImageAlphaPremultipliedLast},
    color_space::CGColorSpace,
    context::{CGContext, CGTextDrawingMode},
    geometry::CGPoint,
};
use core_text::{
    font::CTFont,
    font_collection::CTFontCollectionRef,
    font_descriptor::{
        CTFontDescriptor, kCTFontSlantTrait, kCTFontSymbolicTrait, kCTFontWeightTrait,
        kCTFontWidthTrait,
    },
    line::CTLine,
    run::CTRun,
    string_attributes::kCTFontAttributeName,
};
use font_kit::{
    font::Font as FontKitFont,
    handle::Handle,
    hinting::HintingOptions,
    metrics::Metrics,
    properties::{Style as FontkitStyle, Weight as FontkitWeight},
    source::SystemSource,
    sources::mem::MemSource,
};
use gpui::{
    Bounds, CompactGlyphPositions, CompactLineLayout, DevicePixels, Font, FontFallbacks,
    FontFeatures, FontId, FontMetrics, FontRun, FontStyle, FontWeight, GlyphId, LineLayout, Pixels,
    PlatformTextSystem, RenderGlyphParams, Result, RetainedLineLayout, SUBPIXEL_VARIANTS_X,
    ShapedGlyph, ShapedRun, SharedString, Size, TextRenderingMode, point, px, size,
    swap_rgba_pa_to_bgra,
};
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use pathfinder_geometry::{
    rect::{RectF, RectI},
    transform2d::Transform2F,
    vector::Vector2F,
};
use smallvec::SmallVec;
use std::{borrow::Cow, char, convert::TryFrom, ptr::NonNull, sync::Arc};
use unicode_segmentation::UnicodeSegmentation;

#[cfg(target_os = "macos")]
use {
    core_foundation::base::CFType,
    gpui::{Hsla, Rgba},
    std::sync::OnceLock,
};

use crate::open_type::apply_features_and_fallbacks;

#[allow(non_upper_case_globals)]
const kCGImageAlphaOnly: u32 = 7;

/// Apple text system using CoreText for font shaping.
pub struct AppleTextSystem(RwLock<AppleTextSystemState>);

#[derive(Clone, PartialEq, Eq, Hash)]
struct FontKey {
    font_family: SharedString,
    font_features: FontFeatures,
    font_fallbacks: Option<FontFallbacks>,
}

struct AppleTextSystemState {
    memory_source: MemSource,
    system_source: SystemSource,
    fonts: Vec<FontKitFont>,
    font_selections: HashMap<Font, FontId>,
    font_ids_by_postscript_name: HashMap<String, FontId>,
    font_ids_by_font_key: HashMap<FontKey, SmallVec<[FontId; 4]>>,
    postscript_names_by_font_id: HashMap<FontId, String>,
}

impl AppleTextSystem {
    /// Create a new AppleTextSystem.
    pub fn new() -> Self {
        Self(RwLock::new(AppleTextSystemState {
            memory_source: MemSource::empty(),
            system_source: SystemSource::new(),
            fonts: Vec::new(),
            font_selections: HashMap::default(),
            font_ids_by_postscript_name: HashMap::default(),
            font_ids_by_font_key: HashMap::default(),
            postscript_names_by_font_id: HashMap::default(),
        }))
    }
}

impl Default for AppleTextSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformTextSystem for AppleTextSystem {
    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        self.0.write().add_fonts(fonts)
    }

    fn all_font_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        let collection = core_text::font_collection::create_for_all_families();
        // NOTE: We intentionally avoid using `collection.get_descriptors()` here because
        // it has a memory leak bug in core-text v21.0.0. The upstream code uses
        // `wrap_under_get_rule` but `CTFontCollectionCreateMatchingFontDescriptors`
        // follows the Create Rule (caller owns the result), so it should use
        // `wrap_under_create_rule`. We call the function directly with correct memory management.
        unsafe extern "C" {
            fn CTFontCollectionCreateMatchingFontDescriptors(
                collection: CTFontCollectionRef,
            ) -> CFArrayRef;
        }
        let descriptors: Option<CFArray<CTFontDescriptor>> = unsafe {
            let array_ref =
                CTFontCollectionCreateMatchingFontDescriptors(collection.as_concrete_TypeRef());
            if array_ref.is_null() {
                None
            } else {
                Some(CFArray::wrap_under_create_rule(array_ref))
            }
        };
        let Some(descriptors) = descriptors else {
            return names;
        };
        for descriptor in descriptors.into_iter() {
            names.extend(lenient_font_attributes::family_name(&descriptor));
        }
        if let Ok(fonts_in_memory) = self.0.read().memory_source.all_families() {
            names.extend(fonts_in_memory);
        }
        names
    }

    fn font_id(&self, font: &Font) -> Result<FontId> {
        let lock = self.0.upgradable_read();
        if let Some(font_id) = lock.font_selections.get(font) {
            Ok(*font_id)
        } else {
            let mut lock = RwLockUpgradableReadGuard::upgrade(lock);
            let font_key = FontKey {
                font_family: font.family.clone(),
                font_features: font.features.clone(),
                font_fallbacks: font.fallbacks.clone(),
            };
            let candidates = if let Some(font_ids) = lock.font_ids_by_font_key.get(&font_key) {
                font_ids.as_slice()
            } else {
                let font_ids =
                    lock.load_family(&font.family, &font.features, font.fallbacks.as_ref())?;
                lock.font_ids_by_font_key.insert(font_key.clone(), font_ids);
                lock.font_ids_by_font_key[&font_key].as_ref()
            };

            let candidate_properties = candidates
                .iter()
                .map(|font_id| lock.fonts[font_id.0].properties())
                .collect::<SmallVec<[_; 4]>>();

            let ix = font_kit::matching::find_best_match(
                &candidate_properties,
                &font_kit::properties::Properties {
                    style: fontkit_style(font.style),
                    weight: fontkit_weight(font.weight),
                    stretch: Default::default(),
                },
            )?;

            let font_id = candidates[ix];
            lock.font_selections.insert(font.clone(), font_id);
            Ok(font_id)
        }
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        font_kit_metrics_to_metrics(self.0.read().fonts[font_id.0].metrics())
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        Ok(bounds_from_rect(
            self.0.read().fonts[font_id.0].typographic_bounds(glyph_id.0)?,
        ))
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        self.0.read().advance(font_id, glyph_id)
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.0.read().glyph_for_char(font_id, ch)
    }

    fn glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        self.0.read().raster_bounds(params)
    }

    fn rasterize_glyph(
        &self,
        glyph_id: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        self.0.read().rasterize_glyph(glyph_id, raster_bounds)
    }

    fn layout_line(&self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        self.layout_line_inner(text, font_size, font_runs, None).0
    }

    fn layout_line_for_retention(
        &self,
        text: &str,
        font_size: Pixels,
        runs: &[FontRun],
    ) -> RetainedLineLayout {
        let (line, layout, width, omitted_boms) = self.native_line(text, font_size, runs);
        if let [run] = runs
            && run.len == text.len()
            && let Some(compact) = self.compact_line(&line, &layout, width, text)
        {
            return RetainedLineLayout::Compact(compact);
        }
        let mut positions = Vec::new();
        let layout = self.extract_line(
            &line,
            layout,
            text,
            width,
            &omitted_boms,
            Some(&mut positions),
        );
        RetainedLineLayout::Dense(layout, positions, width)
    }

    fn layout_line_with_precise_positions(
        &self,
        text: &str,
        font_size: Pixels,
        font_runs: &[FontRun],
    ) -> (LineLayout, Vec<f64>, f64) {
        let mut glyph_x_positions = Vec::new();
        let (layout, width) =
            self.layout_line_inner(text, font_size, font_runs, Some(&mut glyph_x_positions));
        (layout, glyph_x_positions, width)
    }

    fn recommended_rendering_mode(
        &self,
        _font_id: FontId,
        _font_size: Pixels,
    ) -> TextRenderingMode {
        TextRenderingMode::Grayscale
    }

    #[cfg(target_os = "macos")]
    fn glyph_dilation_for_color(&self, color: Hsla) -> u8 {
        // When font smoothing is enabled, CoreGraphics thickens glyph strokes by an amount that
        // depends on the foreground color's luminance. We replicate the logic used by CoreGraphics
        // to select between the different levels of dilation.
        if !font_smoothing_allowed_by_user() {
            return 0;
        }
        let rgba: Rgba = color.into();
        let luminance = 0.2126 * rgba.r + 0.7152 * rgba.g + 0.0722 * rgba.b;
        let level = ((4.0 * luminance) + 0.5).floor() as i32;
        level.clamp(0, 4) as u8
    }
}

#[cfg(target_os = "macos")]
fn font_smoothing_allowed_by_user() -> bool {
    static ALLOWED: OnceLock<bool> = OnceLock::new();
    *ALLOWED.get_or_init(|| {
        use core_foundation_sys::preferences::{
            CFPreferencesCopyAppValue, kCFPreferencesCurrentApplication,
        };

        let key = CFString::new("AppleFontSmoothing");
        let value_ref = unsafe {
            CFPreferencesCopyAppValue(key.as_concrete_TypeRef(), kCFPreferencesCurrentApplication)
        };
        if value_ref.is_null() {
            return true;
        }
        let value = unsafe { CFType::wrap_under_create_rule(value_ref) };
        let Some(number) = value.downcast_into::<CFNumber>() else {
            return true;
        };
        // Only an explicit value of `0` means that font smoothing is disabled.
        number.to_i64() != Some(0)
    })
}

impl AppleTextSystemState {
    fn add_fonts(&mut self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        let fonts = fonts
            .into_iter()
            .map(|bytes| match bytes {
                Cow::Borrowed(embedded_font) => {
                    let data_provider = unsafe {
                        core_graphics::data_provider::CGDataProvider::from_slice(embedded_font)
                    };
                    let font = core_graphics::font::CGFont::from_data_provider(data_provider)
                        .map_err(|()| anyhow!("Could not load an embedded font."))?;
                    let font = font_kit::loaders::core_text::Font::from_core_graphics_font(font);
                    Ok(Handle::from_native(&font))
                }
                Cow::Owned(bytes) => Ok(Handle::from_memory(Arc::new(bytes), 0)),
            })
            .collect::<Result<Vec<_>>>()?;
        self.memory_source.add_fonts(fonts.into_iter())?;
        Ok(())
    }

    fn load_family(
        &mut self,
        name: &str,
        features: &FontFeatures,
        fallbacks: Option<&FontFallbacks>,
    ) -> Result<SmallVec<[FontId; 4]>> {
        let name = gpui::font_name_with_fallbacks(name, ".AppleSystemUIFont");

        let mut font_ids = SmallVec::new();
        let mut postscript_names_seen = HashSet::default();
        let family = self
            .memory_source
            .select_family_by_name(name)
            .or_else(|_| self.system_source.select_family_by_name(name))?;
        for font in family.fonts() {
            let mut font = font.load()?;

            apply_features_and_fallbacks(&mut font, features, fallbacks)?;
            // This block contains a precautionary fix to guard against loading fonts
            // that might cause panics due to `.unwrap()`s up the chain.
            {
                // We use the 'm' character for text measurements in various spots
                // (e.g., the editor). However, at time of writing some of those usages
                // will panic if the font has no 'm' glyph.
                //
                // Therefore, we check up front that the font has the necessary glyph.
                let has_m_glyph = font.glyph_for_char('m').is_some();

                // HACK: The 'Segoe Fluent Icons' font does not have an 'm' glyph,
                // but we need to be able to load it for rendering Windows icons in
                // the Storybook (on macOS).
                let is_segoe_fluent_icons = font.full_name() == "Segoe Fluent Icons";

                if !has_m_glyph && !is_segoe_fluent_icons {
                    // I spent far too long trying to track down why a font missing the 'm'
                    // character wasn't loading. This log statement will hopefully save
                    // someone else from suffering the same fate.
                    log::warn!(
                        "font '{}' has no 'm' character and was not loaded",
                        font.full_name()
                    );
                    continue;
                }
            }

            // We've seen a number of panics in production caused by calling font.properties()
            // which unwraps a downcast to CFNumber. This is an attempt to avoid the panic,
            // and to try and identify the incalcitrant font.
            let traits = font.native_font().all_traits();
            if unsafe {
                !(traits
                    .get(kCTFontSymbolicTrait)
                    .downcast::<CFNumber>()
                    .is_some()
                    && traits
                        .get(kCTFontWidthTrait)
                        .downcast::<CFNumber>()
                        .is_some()
                    && traits
                        .get(kCTFontWeightTrait)
                        .downcast::<CFNumber>()
                        .is_some()
                    && traits
                        .get(kCTFontSlantTrait)
                        .downcast::<CFNumber>()
                        .is_some())
            } {
                log::error!(
                    "Failed to read traits for font {:?} (PostScript name {:?})",
                    font.full_name(),
                    font.postscript_name(),
                );
                continue;
            }

            let Some(postscript_name) = font.postscript_name() else {
                log::warn!(
                    "font {:?} in family {:?} has no PostScript name; skipping",
                    font.full_name(),
                    name,
                );
                continue;
            };
            // Dedup is scoped to this single `load_family` call (issue #55472).
            // The same family can be reloaded later under a different `FontKey`
            // (different features/fallbacks); a global check against
            // `font_ids_by_postscript_name` would skip every already-registered
            // font and leave the second call's `font_ids` empty.
            if !postscript_names_seen.insert(postscript_name.clone()) {
                log::warn!(
                    "skipping duplicate font {:?} with PostScript name {:?} \
                     in family {:?}",
                    font.full_name(),
                    postscript_name,
                    name,
                );
                continue;
            }
            let font_id = FontId(self.fonts.len());
            font_ids.push(font_id);
            self.font_ids_by_postscript_name
                .insert(postscript_name.clone(), font_id);
            self.postscript_names_by_font_id
                .insert(font_id, postscript_name);
            self.fonts.push(font);
        }
        Ok(font_ids)
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        Ok(size_from_vector2f(
            self.fonts[font_id.0].advance(glyph_id.0)?,
        ))
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.fonts[font_id.0].glyph_for_char(ch).map(GlyphId)
    }

    fn id_for_native_font(&mut self, requested_font: CTFont) -> FontId {
        let postscript_name = requested_font.postscript_name();
        if let Some(font_id) = self.font_ids_by_postscript_name.get(&postscript_name) {
            *font_id
        } else {
            let font_id = FontId(self.fonts.len());
            self.font_ids_by_postscript_name
                .insert(postscript_name.clone(), font_id);
            self.postscript_names_by_font_id
                .insert(font_id, postscript_name);
            self.fonts
                .push(font_kit::font::Font::from_core_graphics_font(
                    requested_font.copy_to_CGFont(),
                ));
            font_id
        }
    }

    fn is_emoji(&self, font_id: FontId) -> bool {
        self.postscript_names_by_font_id
            .get(&font_id)
            .is_some_and(|postscript_name| {
                postscript_name == "AppleColorEmoji" || postscript_name == ".AppleColorEmojiUI"
            })
    }

    fn raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        let font = &self.fonts[params.font_id.0];
        let scale = Transform2F::from_scale(params.scale_factor);
        let bounds: Bounds<DevicePixels> = bounds_from_rect_i(font.raster_bounds(
            params.glyph_id.0,
            params.font_size.into(),
            scale,
            HintingOptions::None,
            font_kit::canvas::RasterizationOptions::GrayscaleAa,
        )?);

        // Expand the bounds by 1 pixel on each side to give CG room for anti-aliasing.
        let mut bounds = bounds.dilate(DevicePixels(1));
        // Add an extra pixel when the subpixel variant isn't zero to make room for anti-aliasing.
        bounds.size.width += DevicePixels(i32::from(params.subpixel_variant.x > 0));
        bounds.size.height += DevicePixels(i32::from(params.subpixel_variant.y > 0));
        Ok(bounds)
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        glyph_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        if glyph_bounds.size.width.0 == 0 || glyph_bounds.size.height.0 == 0 {
            anyhow::bail!("glyph bounds are empty");
        } else {
            let bitmap_size = glyph_bounds.size;

            let mut bytes;
            let cx;
            if params.is_emoji {
                bytes = vec![0; bitmap_size.width.0 as usize * 4 * bitmap_size.height.0 as usize];
                cx = CGContext::create_bitmap_context(
                    Some(bytes.as_mut_ptr() as *mut _),
                    bitmap_size.width.0 as usize,
                    bitmap_size.height.0 as usize,
                    8,
                    bitmap_size.width.0 as usize * 4,
                    &CGColorSpace::create_device_rgb(),
                    kCGImageAlphaPremultipliedLast,
                );
            } else {
                bytes = vec![0; bitmap_size.width.0 as usize * bitmap_size.height.0 as usize];
                cx = CGContext::create_bitmap_context(
                    Some(bytes.as_mut_ptr() as *mut _),
                    bitmap_size.width.0 as usize,
                    bitmap_size.height.0 as usize,
                    8,
                    bitmap_size.width.0 as usize,
                    &CGColorSpace::create_device_gray(),
                    kCGImageAlphaOnly,
                );
            }

            // Move the origin to bottom left and account for scaling, this
            // makes drawing text consistent with the font-kit's raster_bounds.
            cx.translate(
                -glyph_bounds.origin.x.0 as CGFloat,
                (glyph_bounds.origin.y.0 + glyph_bounds.size.height.0
                    - i32::from(params.subpixel_variant.y > 0)) as CGFloat,
            );
            cx.scale(
                params.scale_factor as CGFloat,
                params.scale_factor as CGFloat,
            );

            let subpixel_shift = params
                .subpixel_variant
                .map(|v| v as f32 / SUBPIXEL_VARIANTS_X as f32);
            cx.set_text_drawing_mode(CGTextDrawingMode::CGTextFill);
            cx.set_allows_antialiasing(true);
            cx.set_should_antialias(true);
            cx.set_allows_font_subpixel_positioning(true);
            cx.set_should_subpixel_position_fonts(true);
            cx.set_allows_font_subpixel_quantization(false);
            cx.set_should_subpixel_quantize_fonts(false);

            if cfg!(target_os = "macos") && params.dilation > 0 {
                let luminance = params.dilation as f64 * 0.25;
                cx.set_should_smooth_fonts(true);
                cx.set_gray_fill_color(luminance, 1.0);
            } else {
                cx.set_gray_fill_color(0.0, 1.0);
            }
            self.fonts[params.font_id.0]
                .native_font()
                .clone_with_font_size(f32::from(params.font_size) as CGFloat)
                .draw_glyphs(
                    &[params.glyph_id.0 as CGGlyph],
                    &[CGPoint::new(
                        (subpixel_shift.x / params.scale_factor) as CGFloat,
                        (subpixel_shift.y / params.scale_factor) as CGFloat,
                    )],
                    cx,
                );

            if params.is_emoji {
                // Convert from RGBA with premultiplied alpha to BGRA with straight alpha.
                for pixel in bytes.chunks_exact_mut(4) {
                    swap_rgba_pa_to_bgra(pixel);
                }
            }

            Ok((bitmap_size, bytes))
        }
    }
}

impl AppleTextSystem {
    fn layout_line_inner(
        &self,
        text: &str,
        font_size: Pixels,
        font_runs: &[FontRun],
        glyph_x_positions: Option<&mut Vec<f64>>,
    ) -> (LineLayout, f64) {
        let (line, layout, width, omitted_boms) = self.native_line(text, font_size, font_runs);
        (
            self.extract_line(&line, layout, text, width, &omitted_boms, glyph_x_positions),
            width,
        )
    }

    fn native_line(
        &self,
        text: &str,
        font_size: Pixels,
        font_runs: &[FontRun],
    ) -> (CTLine, LineLayout, f64, Vec<usize>) {
        // Construct the attributed string, converting UTF8 ranges to UTF16 ranges.
        let mut string = CFMutableAttributedString::new();
        let mut omitted_boms = Vec::new();
        let mut max_ascent = 0.0f32;
        let mut max_descent = 0.0f32;

        {
            let mut text = text;
            let mut break_ligature = true;
            for run in font_runs {
                let text_run;
                (text_run, text) = text.split_at(run.len);

                let utf16_start = string.char_len(); // insert at end of string
                if text_run.starts_with('\u{feff}') {
                    omitted_boms.push(utf16_start as usize);
                }
                // note: replace_str may silently ignore codepoints it dislikes (e.g., BOM at start of string)
                string.replace_str(&CFString::new(text_run), CFRange::init(utf16_start, 0));
                let utf16_end = string.char_len();

                let length = utf16_end - utf16_start;
                let cf_range = CFRange::init(utf16_start, length);
                let font = self.0.read().fonts[run.font_id.0].clone();

                let font_metrics = font.metrics();
                let font_scale = f32::from(font_size) / font_metrics.units_per_em as f32;
                max_ascent = max_ascent.max(font_metrics.ascent * font_scale);
                max_descent = max_descent.max(-font_metrics.descent * font_scale);

                let font_size = if break_ligature {
                    px(f32::from(font_size).next_up())
                } else {
                    font_size
                };
                unsafe {
                    string.set_attribute(
                        cf_range,
                        kCTFontAttributeName,
                        &font.native_font().clone_with_font_size(font_size.into()),
                    );
                }
                break_ligature = !break_ligature;
            }
        }
        let line = CTLine::new_with_attributed_string(string.as_concrete_TypeRef());
        let width = line.get_typographic_bounds().width;
        let layout = LineLayout {
            font_size,
            width: px(width as f32),
            ascent: px(max_ascent),
            descent: px(max_descent),
            len: text.len(),
            ..LineLayout::default()
        };
        (line, layout, width, omitted_boms)
    }

    fn extract_line(
        &self,
        line: &CTLine,
        mut layout: LineLayout,
        text: &str,
        width: f64,
        omitted_boms: &[usize],
        mut glyph_x_positions: Option<&mut Vec<f64>>,
    ) -> LineLayout {
        // Retrieve the glyphs from the shaped line, converting UTF16 offsets to UTF8 offsets.
        let glyph_runs = line.glyph_runs();
        let glyph_run_count = glyph_runs.len();
        let mut runs = <Vec<ShapedRun>>::with_capacity(glyph_run_count as usize);
        let mut ix_converter = StringIndexConverter::new(text);
        let mut source_starts = vec![0_u64; text.len().div_ceil(64)];
        let mut font_identities = HashMap::<_, (FontId, bool)>::default();
        for run in glyph_runs.iter() {
            let attributes = run.attributes().unwrap();
            let font = unsafe {
                attributes
                    .get(kCTFontAttributeName)
                    .downcast::<CTFont>()
                    .unwrap()
            };
            let (font_id, is_emoji) = if glyph_run_count == 1 {
                self.identify_font(&font)
            } else {
                *font_identities
                    .entry(font.as_concrete_TypeRef())
                    .or_insert_with(|| self.identify_font(&font))
            };

            if let Some(glyph_x_positions) = glyph_x_positions.as_mut() {
                glyph_x_positions.reserve(run.glyph_count().try_into().unwrap_or(0));
            }
            let glyphs = match runs.last_mut() {
                Some(run) if run.font_id == font_id => &mut run.glyphs,
                _ => {
                    runs.push(ShapedRun {
                        font_id,
                        glyphs: Vec::with_capacity(run.glyph_count().try_into().unwrap_or(0)),
                    });
                    &mut runs.last_mut().unwrap().glyphs
                }
            };
            for ((&glyph_id, position), &glyph_utf16_ix) in run
                .glyphs()
                .iter()
                .zip(run.positions().iter())
                .zip(run.string_indices().iter())
            {
                let glyph_utf16_ix = usize::try_from(glyph_utf16_ix).unwrap();
                ix_converter.advance_to_utf16_ix(
                    glyph_utf16_ix
                        + omitted_boms.partition_point(|offset| *offset <= glyph_utf16_ix),
                );
                if let Some(starts) = source_starts.get_mut(ix_converter.utf8_ix / 64) {
                    *starts |= 1 << (ix_converter.utf8_ix % 64);
                }
                if let Some(glyph_x_positions) = glyph_x_positions.as_mut() {
                    glyph_x_positions.push(position.x);
                }
                glyphs.push(ShapedGlyph {
                    id: GlyphId(glyph_id as u32),
                    position: point(position.x as f32, -position.y as f32).map(px),
                    index: ix_converter.utf8_ix,
                    is_emoji,
                });
            }
        }
        for (index, _) in text.grapheme_indices(true) {
            if source_starts[index / 64] & (1 << (index % 64)) != 0 {
                layout.caret_positions.push((index, 0.));
            }
        }
        let mut advances = Vec::new();
        let mut glyphs = runs.iter().flat_map(|run| &run.glyphs);
        let mut pen_x = 0.;
        for run in glyph_runs.iter() {
            Self::read_base_advances(&run, &mut advances);
            for (advance, glyph) in advances.iter().zip(glyphs.by_ref()) {
                if let Some(caret) = layout
                    .caret_positions
                    .partition_point(|(index, _)| *index <= glyph.index)
                    .checked_sub(1)
                    .and_then(|index| layout.caret_positions.get_mut(index))
                {
                    let starts = &mut source_starts[caret.0 / 64];
                    let mask = 1 << (caret.0 % 64);
                    if *starts & mask != 0 {
                        caret.1 = pen_x;
                        *starts &= !mask;
                    }
                }
                pen_x += advance.width;
            }
        }
        if !text.is_empty() {
            layout.caret_positions.push((text.len(), width));
        }
        layout.runs = runs;
        layout
    }

    fn compact_line(
        &self,
        line: &CTLine,
        layout: &LineLayout,
        width: f64,
        text: &str,
    ) -> Option<CompactLineLayout> {
        if text.is_empty()
            || !text.bytes().all(|byte| (b' '..=b'~').contains(&byte))
            || !width.is_finite()
            || width < 0.
        {
            return None;
        }
        let runs = line.glyph_runs();
        if runs.len() != 1 {
            return None;
        }
        let run = runs.iter().next()?;
        if usize::try_from(run.glyph_count()).ok()? != text.len() {
            return None;
        }
        let attributes = run.attributes()?;
        let font = unsafe { attributes.get(kCTFontAttributeName).downcast::<CTFont>()? };
        let (font_id, is_emoji) = self.identify_font(&font);
        let mut glyphs = vec![0_u16; text.len()];
        let mut native_positions =
            [objc2_core_foundation::CGPoint::new(0., 0.); CompactGlyphPositions::BLOCK_LEN];
        let mut indices = [0_isize; CompactGlyphPositions::BLOCK_LEN];
        let positions = CompactGlyphPositions::new(text.len(), |range, values| {
            let count = range.len();
            let native_range = objc2_core_foundation::CFRange::new(
                isize::try_from(range.start).ok()?,
                isize::try_from(count).ok()?,
            );
            unsafe {
                let native = &*run.as_concrete_TypeRef().cast::<objc2_core_text::CTRun>();
                native.glyphs(
                    native_range,
                    NonNull::new(glyphs.get_mut(range.clone())?.as_mut_ptr())?,
                );
                native.positions(native_range, NonNull::new(native_positions.as_mut_ptr())?);
                native.string_indices(native_range, NonNull::new(indices.as_mut_ptr())?);
            }
            for (offset, value) in values.iter_mut().enumerate() {
                let position = native_positions[offset];
                let x = position.x;
                if usize::try_from(indices[offset]).ok()? != range.start + offset
                    || position.y.to_bits() != 0
                    || !x.is_finite()
                    || x < 0.
                    || x > width
                    || (offset > 0 && position.x < native_positions[offset - 1].x)
                {
                    return None;
                }
                *value = x;
            }
            Some(())
        })?;
        if positions.get(0)?.to_bits() != 0
            || !(CompactGlyphPositions::BLOCK_LEN..text.len())
                .step_by(CompactGlyphPositions::BLOCK_LEN)
                .all(|index| {
                    positions
                        .get(index - 1)
                        .zip(positions.get(index))
                        .is_some_and(|(previous, next)| previous <= next)
                })
        {
            return None;
        }
        let mut advances = Vec::new();
        Self::read_base_advances(&run, &mut advances);
        let mut pen_x = 0_f64;
        for (index, advance) in advances.iter().enumerate() {
            if positions.get(index)?.to_bits() != pen_x.to_bits() {
                return None;
            }
            pen_x += advance.width;
        }
        Some(CompactLineLayout {
            font_id,
            font_size: layout.font_size,
            ascent: layout.ascent,
            descent: layout.descent,
            width,
            is_emoji,
            glyphs,
            positions,
        })
    }

    fn identify_font(&self, font: &CTFont) -> (FontId, bool) {
        let postscript_name = font.postscript_name();
        let state = self.0.upgradable_read();
        if let Some(&font_id) = state.font_ids_by_postscript_name.get(&postscript_name) {
            (font_id, state.is_emoji(font_id))
        } else {
            let mut state = RwLockUpgradableReadGuard::upgrade(state);
            let font_id = state.id_for_native_font(font.clone());
            (font_id, state.is_emoji(font_id))
        }
    }

    fn read_base_advances(run: &CTRun, advances: &mut Vec<objc2_core_foundation::CGSize>) {
        advances.resize(
            run.glyph_count().try_into().unwrap_or(0),
            objc2_core_foundation::CGSize::new(0., 0.),
        );
        unsafe {
            let native = &*run.as_concrete_TypeRef().cast::<objc2_core_text::CTRun>();
            native.base_advances_and_origins(
                objc2_core_foundation::CFRange::new(0, 0),
                advances.as_mut_ptr(),
                std::ptr::null_mut(),
            );
        }
    }
}

#[derive(Debug, Clone)]
struct StringIndexConverter<'a> {
    text: &'a str,
    /// Index in UTF-8 bytes
    utf8_ix: usize,
    /// Index in UTF-16 code units
    utf16_ix: usize,
    checkpoints: Option<Vec<(usize, usize)>>,
}

impl<'a> StringIndexConverter<'a> {
    const CHECKPOINT_INTERVAL: usize = 64;

    fn new(text: &'a str) -> Self {
        Self {
            text,
            utf8_ix: 0,
            utf16_ix: 0,
            checkpoints: None,
        }
    }

    fn advance_to_utf16_ix(&mut self, utf16_target: usize) {
        let backward = self.utf16_ix > utf16_target;
        if backward && self.checkpoints.is_none() {
            self.checkpoints = Some(Self::build_checkpoints(self.text));
        }
        if (backward || utf16_target - self.utf16_ix > Self::CHECKPOINT_INTERVAL)
            && let Some(checkpoints) = &self.checkpoints
            && let Some(&(utf16_ix, utf8_ix)) = checkpoints
                .partition_point(|&(utf16_ix, _)| utf16_ix <= utf16_target)
                .checked_sub(1)
                .and_then(|index| checkpoints.get(index))
            && (backward || utf16_ix > self.utf16_ix)
        {
            self.utf16_ix = utf16_ix;
            self.utf8_ix = utf8_ix;
        }
        for (ix, c) in self.text[self.utf8_ix..].char_indices() {
            if self.utf16_ix >= utf16_target {
                self.utf8_ix += ix;
                return;
            }
            self.utf16_ix += c.len_utf16();
        }
        self.utf8_ix = self.text.len();
    }

    fn build_checkpoints(text: &str) -> Vec<(usize, usize)> {
        let mut checkpoints = Vec::with_capacity(text.len() / Self::CHECKPOINT_INTERVAL + 2);
        let mut utf16_ix = 0;
        let mut next_checkpoint = 0;
        for (utf8_ix, character) in text.char_indices() {
            if utf16_ix >= next_checkpoint {
                checkpoints.push((utf16_ix, utf8_ix));
                next_checkpoint =
                    (utf16_ix / Self::CHECKPOINT_INTERVAL + 1) * Self::CHECKPOINT_INTERVAL;
            }
            utf16_ix += character.len_utf16();
        }
        if utf16_ix >= next_checkpoint {
            checkpoints.push((utf16_ix, text.len()));
        }
        checkpoints
    }
}

fn font_kit_metrics_to_metrics(metrics: Metrics) -> FontMetrics {
    FontMetrics {
        units_per_em: metrics.units_per_em,
        ascent: metrics.ascent,
        descent: metrics.descent,
        line_gap: metrics.line_gap,
        underline_position: metrics.underline_position,
        underline_thickness: metrics.underline_thickness,
        cap_height: metrics.cap_height,
        x_height: metrics.x_height,
        bounding_box: bounds_from_rect(metrics.bounding_box),
    }
}

fn bounds_from_rect(rect: RectF) -> Bounds<f32> {
    Bounds {
        origin: point(rect.origin_x(), rect.origin_y()),
        size: size(rect.width(), rect.height()),
    }
}

fn bounds_from_rect_i(rect: RectI) -> Bounds<DevicePixels> {
    Bounds {
        origin: point(DevicePixels(rect.origin_x()), DevicePixels(rect.origin_y())),
        size: size(DevicePixels(rect.width()), DevicePixels(rect.height())),
    }
}

// impl From<Vector2I> for Size<DevicePixels> {
//     fn from(value: Vector2I) -> Self {
//         size(value.x().into(), value.y().into())
//     }
// }

// impl From<RectI> for Bounds<i32> {
//     fn from(rect: RectI) -> Self {
//         Bounds {
//             origin: point(rect.origin_x(), rect.origin_y()),
//             size: size(rect.width(), rect.height()),
//         }
//     }
// }

// impl From<Point<u32>> for Vector2I {
//     fn from(size: Point<u32>) -> Self {
//         Vector2I::new(size.x as i32, size.y as i32)
//     }
// }

fn size_from_vector2f(vec: Vector2F) -> Size<f32> {
    size(vec.x(), vec.y())
}

fn fontkit_weight(value: FontWeight) -> FontkitWeight {
    FontkitWeight(value.0)
}

fn fontkit_style(style: FontStyle) -> FontkitStyle {
    match style {
        FontStyle::Normal => FontkitStyle::Normal,
        FontStyle::Italic => FontkitStyle::Italic,
        FontStyle::Oblique => FontkitStyle::Oblique,
    }
}

// Some fonts may have no attributes despite `core_text` requiring them (and panicking).
// This is the same version as `core_text` has without `expect` calls.
mod lenient_font_attributes {
    use core_foundation::{
        base::{CFRetain, CFType, TCFType},
        string::{CFString, CFStringRef},
    };
    use core_text::font_descriptor::{
        CTFontDescriptor, CTFontDescriptorCopyAttribute, kCTFontFamilyNameAttribute,
    };

    pub fn family_name(descriptor: &CTFontDescriptor) -> Option<String> {
        unsafe { get_string_attribute(descriptor, kCTFontFamilyNameAttribute) }
    }

    fn get_string_attribute(
        descriptor: &CTFontDescriptor,
        attribute: CFStringRef,
    ) -> Option<String> {
        unsafe {
            let value = CTFontDescriptorCopyAttribute(descriptor.as_concrete_TypeRef(), attribute);
            if value.is_null() {
                return None;
            }

            let value = CFType::wrap_under_create_rule(value);
            assert!(value.instance_of::<CFString>());
            let s = wrap_under_get_rule(value.as_CFTypeRef() as CFStringRef);
            Some(s.to_string())
        }
    }

    unsafe fn wrap_under_get_rule(reference: CFStringRef) -> CFString {
        unsafe {
            assert!(!reference.is_null(), "Attempted to create a NULL object.");
            let reference = CFRetain(reference as *const ::std::os::raw::c_void) as CFStringRef;
            TCFType::wrap_under_create_rule(reference)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AppleTextSystem, StringIndexConverter};
    use anyhow::Result;
    use core_foundation::{
        attributed_string::CFMutableAttributedString,
        base::{CFRange, TCFType},
        string::{CFString, CFStringCreateWithCharacters},
    };
    use core_text::{
        font_descriptor::kCTFontOrientationHorizontal, line::CTLine,
        string_attributes::kCTFontAttributeName,
    };
    use gpui::{
        FontFallbacks, FontFeatures, FontRun, FontStyle, FontWeight, GlyphId, Pixels,
        PlatformTextSystem, RenderGlyphParams, RetainedLineLayout, SUBPIXEL_VARIANTS_X, font,
        point, px,
    };
    use std::{borrow::Cow, sync::Arc};
    use unicode_segmentation::UnicodeSegmentation;

    #[test]
    fn test_retained_layout_matches_native() {
        let fonts = test_fonts();
        for (style, weight) in [
            (FontStyle::Normal, FontWeight::NORMAL),
            (FontStyle::Normal, FontWeight::BOLD),
            (FontStyle::Italic, FontWeight::NORMAL),
            (FontStyle::Italic, FontWeight::BOLD),
        ] {
            let mut descriptor = font("Lilex");
            descriptor.style = style;
            descriptor.weight = weight;
            let font_id = fonts.font_id(&descriptor).unwrap();
            for font_size in [
                13.25_f32,
                14.,
                15.,
                16.,
                f32::from_bits(1),
                f32::MIN_POSITIVE,
                0.125,
                16_777_216.,
                f32::MAX.next_down(),
            ] {
                let mut texts = vec![
                    "x".repeat(20_000),
                    format!("{}!x!x", "x".repeat(19_998)),
                    "xy".repeat(10_001),
                    " x!x ".repeat(4_001),
                    "hello world".repeat(1_819),
                    "hello 0123456789, (world)! [a] {b}; c + d - e * f / g = h.".repeat(401),
                ];
                if style == FontStyle::Normal && weight == FontWeight::NORMAL {
                    texts.extend(
                        [1, 1_023, 1_024, 1_025, 65_535, 65_536, 65_537].map(|len| "x".repeat(len)),
                    );
                    if font_size == 15. {
                        texts.push("x".repeat(1_048_577));
                    }
                }
                for text in texts {
                    let runs = [FontRun {
                        font_id,
                        len: text.len(),
                    }];
                    assert_retained_matches_native(&fonts, &text, px(font_size), &runs, true);
                }
            }
        }
        let font_id = fonts.font_id(&font("Lilex")).unwrap();
        for text in [
            "0xA",
            "1x2",
            "x!!x",
            "//->...++",
            "office",
            "",
            "x\t",
            "x\u{301}",
            "नमस्ते hello",
            "x\u{202e}abc",
        ] {
            assert_retained_matches_native(
                &fonts,
                text,
                px(15.),
                &[FontRun {
                    font_id,
                    len: text.len(),
                }],
                false,
            );
        }
        assert_retained_matches_native(
            &fonts,
            "xx",
            px(15.),
            &[FontRun { font_id, len: 1 }; 2],
            false,
        );
        for features in [
            vec![("aalt".to_string(), 1)],
            vec![("calt".to_string(), 0)],
            vec![("ordn".to_string(), 1)],
            vec![("cv01".to_string(), 1)],
            vec![("onum".to_string(), 1)],
        ] {
            let mut descriptor = font("Lilex");
            descriptor.features = FontFeatures(Arc::new(features));
            let font_id = fonts.font_id(&descriptor).unwrap();
            let text = "hello world 0123456789 0xA 1x2 x!!x //->...++";
            assert_retained_matches_native(
                &fonts,
                text,
                px(15.),
                &[FontRun {
                    font_id,
                    len: text.len(),
                }],
                false,
            );
        }
    }

    #[test]
    fn test_raster_bounds_match_bitmap_size() {
        let fonts = test_fonts();
        let font_id = fonts.font_id(&font("Lilex")).expect("test font");
        let text = "af\u{0361}😀";
        let layout = fonts.layout_line(
            text,
            px(16.),
            &[FontRun {
                font_id,
                len: text.len(),
            }],
        );
        for scale_factor in [1., 1.25, 1.5, 2., 3.] {
            for run in &layout.runs {
                for glyph in &run.glyphs {
                    for phase in 0..if glyph.is_emoji {
                        1
                    } else {
                        SUBPIXEL_VARIANTS_X
                    } {
                        for dilation in 0..=if glyph.is_emoji { 0 } else { 4 } {
                            let params = RenderGlyphParams {
                                font_id: run.font_id,
                                glyph_id: glyph.id,
                                font_size: layout.font_size,
                                subpixel_variant: point(phase, 0),
                                scale_factor,
                                is_emoji: glyph.is_emoji,
                                subpixel_rendering: false,
                                dilation,
                            };
                            let bounds = fonts.glyph_raster_bounds(&params).expect("raster bounds");
                            let (size, bytes) = fonts
                                .rasterize_glyph(&params, bounds)
                                .expect("glyph bitmap");
                            assert_eq!(size, bounds.size, "{params:?}");
                            let channels = if glyph.is_emoji { 4 } else { 1 };
                            assert_eq!(
                                bytes.len(),
                                size.width.0 as usize * size.height.0 as usize * channels
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_reload_family_with_features_and_fallbacks() {
        let fonts = AppleTextSystem::new();
        let mut font = font("Helvetica");
        let original_id = fonts.font_id(&font).unwrap();
        font.features = FontFeatures::disable_ligatures();
        font.fallbacks = Some(FontFallbacks(Arc::new(vec!["Times".into()])));
        let configured_id = fonts.font_id(&font).unwrap();
        assert_ne!(original_id, configured_id);
        assert_eq!(fonts.font_id(&font).unwrap(), configured_id);
        assert!(fonts.glyph_for_char(configured_id, 'm').is_some());
        let layout = fonts.layout_line(
            "office",
            px(16.),
            &[FontRun {
                font_id: configured_id,
                len: 6,
            }],
        );
        assert_eq!(layout.len, 6);
        assert!(!layout.runs.is_empty());
    }

    #[test]
    fn test_layout_line_with_precise_positions() {
        let fonts = AppleTextSystem::new();
        let font_id = fonts.font_id(&font("Helvetica")).unwrap();
        let glyph_id = fonts.glyph_for_char(font_id, 'm').unwrap().0 as u16;
        for font_size in [16.0f32, 16_777_216.0] {
            let native_font = fonts.0.read().fonts[font_id.0]
                .native_font()
                .clone_with_font_size(f64::from(font_size.next_up()));
            let advance = unsafe {
                native_font.get_advances_for_glyphs(
                    kCTFontOrientationHorizontal,
                    &glyph_id,
                    std::ptr::null_mut(),
                    1,
                )
            };
            let (layout, positions, width) = fonts.layout_line_with_precise_positions(
                "mmmm",
                px(font_size),
                &[FontRun { font_id, len: 4 }],
            );
            assert_eq!(positions, [0., advance, advance * 2., advance * 3.]);
            assert_eq!(width, advance * 4.);
            assert_ne!(width, f64::from(layout.width));
        }

        let text = "mक".repeat(3);
        let layout = fonts.layout_line(
            &text,
            px(16.),
            &[FontRun {
                font_id,
                len: text.len(),
            }],
        );
        let fallback = layout.runs[1].font_id;
        assert_ne!(fallback, font_id);
        assert_eq!(
            layout
                .runs
                .iter()
                .map(|run| (run.font_id, run.glyphs[0].index, run.glyphs[0].is_emoji))
                .collect::<Vec<_>>(),
            [
                (font_id, 0, false),
                (fallback, 1, false),
                (font_id, 4, false),
                (fallback, 5, false),
                (font_id, 8, false),
                (fallback, 9, false),
            ]
        );
    }

    #[test]
    fn test_combining_mark_offsets_match_core_text() -> Result<()> {
        let fonts = test_fonts();
        fonts.add_fonts(vec![Cow::Borrowed(include_bytes!(
            "../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf"
        ))])?;
        let font_size = 16_f32;
        let cases: &[(&str, &str, &[usize], &[(usize, Option<usize>)])] = &[
            (
                "IBM Plex Sans",
                "WX\u{0301}i",
                &[5],
                &[(0, Some(0)), (1, Some(1)), (4, Some(3))],
            ),
            (
                "Helvetica",
                "TóAVậ",
                &[11],
                &[
                    (0, Some(0)),
                    (1, Some(1)),
                    (4, Some(2)),
                    (5, Some(3)),
                    (6, Some(4)),
                ],
            ),
            ("Helvetica", "́a", &[3], &[(0, None), (2, Some(1))]),
            ("Menlo", "́a", &[3], &[(0, None), (2, Some(1))]),
            ("Helvetica", "किX", &[7], &[(0, Some(0)), (6, Some(2))]),
            (
                "Helvetica",
                "दुनियाX",
                &[19],
                &[(0, Some(0)), (6, Some(2)), (12, Some(4)), (18, Some(6))],
            ),
            ("Helvetica", "क्षिX", &[13], &[(0, Some(0)), (12, Some(2))]),
            ("Helvetica", "fiX", &[3], &[(0, Some(0)), (2, Some(1))]),
            (
                "Helvetica",
                "😀कि👩‍💻x",
                &[22],
                &[(0, Some(0)), (4, Some(1)), (10, Some(3)), (21, Some(4))],
            ),
            ("Lilex", "क́x", &[6], &[(0, Some(0)), (5, Some(2))]),
            (
                "Helvetica",
                "किX",
                &[3, 3, 1],
                &[(0, Some(0)), (6, Some(3))],
            ),
        ];
        for &(family, text, lengths, boundaries) in cases {
            let font_id = fonts.font_id(&font(family))?;
            let runs = lengths
                .iter()
                .map(|&len| FontRun { font_id, len })
                .collect::<Vec<_>>();
            assert_eq!(runs.iter().map(|run| run.len).sum::<usize>(), text.len());
            let mut string = CFMutableAttributedString::new();
            string.replace_str(&CFString::new(text), CFRange::init(0, 0));
            let mut byte_offset = 0;
            let mut utf16_offset = 0;
            for (index, run) in runs.iter().enumerate() {
                let utf16_len = text[byte_offset..byte_offset + run.len]
                    .encode_utf16()
                    .count();
                let size = if index % 2 == 0 {
                    font_size.next_up()
                } else {
                    font_size
                };
                let native_font = fonts.0.read().fonts[font_id.0]
                    .native_font()
                    .clone_with_font_size(f64::from(size));
                unsafe {
                    string.set_attribute(
                        CFRange::init(utf16_offset as isize, utf16_len as isize),
                        kCTFontAttributeName,
                        &native_font,
                    );
                }
                byte_offset += run.len;
                utf16_offset += utf16_len;
            }
            let native = CTLine::new_with_attributed_string(string.as_concrete_TypeRef());
            let byte_indices = text
                .char_indices()
                .flat_map(|(index, character)| std::iter::repeat_n(index, character.len_utf16()))
                .collect::<Vec<_>>();
            let mut expected = Vec::new();
            let mut expected_x = Vec::new();
            let mut origins = Vec::new();
            for run in native.glyph_runs().iter() {
                let start = origins.len();
                origins.resize(
                    start + run.glyph_count() as usize,
                    objc2_core_foundation::CGPoint::new(0., 0.),
                );
                unsafe {
                    let native_run = &*run.as_concrete_TypeRef().cast::<objc2_core_text::CTRun>();
                    native_run.base_advances_and_origins(
                        objc2_core_foundation::CFRange::new(0, 0),
                        std::ptr::null_mut(),
                        origins[start..].as_mut_ptr(),
                    );
                }
                for ((&glyph, position), &index) in run
                    .glyphs()
                    .iter()
                    .zip(run.positions().iter())
                    .zip(run.string_indices().iter())
                {
                    expected.push((
                        GlyphId(u32::from(glyph)),
                        byte_indices[index as usize],
                        point(px(position.x as f32), px(-position.y as f32)),
                    ));
                    expected_x.push(position.x);
                }
            }
            if family == "IBM Plex Sans" {
                assert_eq!(
                    expected
                        .iter()
                        .filter(|(_, _, position)| f32::from(position.y) < 0.)
                        .count(),
                    1
                );
            }
            let expected_width = native.get_typographic_bounds().width;
            let expected_carets = boundaries
                .iter()
                .map(|&(index, ordinal)| {
                    let position = ordinal.map_or(0., |ordinal| {
                        assert_eq!(origins[ordinal].x, 0., "{family}: {text:?}");
                        expected_x[ordinal]
                    });
                    (index, position)
                })
                .chain([(text.len(), expected_width)])
                .collect::<Vec<_>>();
            let (precise, positions, width) =
                fonts.layout_line_with_precise_positions(text, px(font_size), &runs);
            assert_eq!((positions, width), (expected_x.clone(), expected_width));
            let RetainedLineLayout::Dense(retained, positions, width) =
                fonts.layout_line_for_retention(text, px(font_size), &runs)
            else {
                panic!("these clusters require dense geometry");
            };
            assert_eq!((positions, width), (expected_x, expected_width));
            for layout in [
                fonts.layout_line(text, px(font_size), &runs),
                precise,
                retained,
            ] {
                assert_eq!(layout.width, Pixels::from(expected_width));
                assert_eq!(layout.len, text.len());
                assert_eq!(
                    layout.caret_positions, expected_carets,
                    "{family}: {text:?}"
                );
                for &(index, position) in &expected_carets {
                    assert_eq!(layout.caret_x_for_index(index), position);
                    assert_eq!(layout.closest_index_for_x(Pixels::from(position)), index);
                }
                assert_eq!(
                    layout
                        .runs
                        .iter()
                        .flat_map(|run| &run.glyphs)
                        .map(|glyph| (glyph.id, glyph.index, glyph.position))
                        .collect::<Vec<_>>(),
                    expected
                );
                if family == "IBM Plex Sans" {
                    assert_eq!(layout.x_for_index(2), Pixels::from(expected_carets[2].1));
                    let query = (expected_carets[1].1 + expected_carets[2].1) / 2. + 0.25;
                    assert_eq!(layout.closest_index_for_x(Pixels::from(query)), 4);
                }
            }
        }

        let flag_pairs = 65_536;
        let text = format!("{}🇺", "🇺🇸".repeat(flag_pairs));
        let boundary_work = text.grapheme_indices(true).enumerate().fold(
            (0, 0),
            |(visits, bytes), (ordinal, (index, grapheme))| {
                assert_eq!(index, ordinal * 8);
                (visits + 1, bytes + grapheme.len())
            },
        );
        assert_eq!(boundary_work, (flag_pairs + 1, text.len()));
        let font_id = fonts.font_id(&font("Helvetica"))?;
        let native_font = fonts.0.read().fonts[font_id.0]
            .native_font()
            .clone_with_font_size(f64::from(font_size.next_up()));
        let mut string = CFMutableAttributedString::new();
        string.replace_str(&CFString::new(&text), CFRange::init(0, 0));
        unsafe {
            string.set_attribute(
                CFRange::init(0, string.char_len()),
                kCTFontAttributeName,
                &native_font,
            );
        }
        let native = CTLine::new_with_attributed_string(string.as_concrete_TypeRef());
        let mut expected_carets = Vec::new();
        for run in native.glyph_runs().iter() {
            let mut origins =
                vec![objc2_core_foundation::CGPoint::new(0., 0.); run.glyph_count() as usize];
            unsafe {
                let native_run = &*run.as_concrete_TypeRef().cast::<objc2_core_text::CTRun>();
                native_run.base_advances_and_origins(
                    objc2_core_foundation::CFRange::new(0, 0),
                    std::ptr::null_mut(),
                    origins.as_mut_ptr(),
                );
            }
            for ((&index, position), origin) in run
                .string_indices()
                .iter()
                .zip(run.positions().iter())
                .zip(origins)
            {
                if index % 4 == 0 {
                    assert_eq!(origin.x, 0.);
                    expected_carets.push((index as usize * 2, position.x));
                }
            }
        }
        expected_carets.dedup_by_key(|(index, _)| *index);
        assert_eq!(expected_carets.len(), flag_pairs + 1);
        for (ordinal, &(index, _)) in expected_carets.iter().enumerate() {
            assert_eq!(index, ordinal * 8);
        }
        let expected_width = native.get_typographic_bounds().width;
        expected_carets.push((text.len(), expected_width));
        let RetainedLineLayout::Dense(layout, _, width) = fonts.layout_line_for_retention(
            &text,
            px(font_size),
            &[FontRun {
                font_id,
                len: text.len(),
            }],
        ) else {
            panic!("regional indicators require dense geometry");
        };
        assert_eq!(width, expected_width);
        assert_eq!(layout.caret_positions.len(), flag_pairs + 2);
        for (actual, expected) in layout.caret_positions.iter().zip(&expected_carets) {
            assert_eq!(actual, expected);
        }
        Ok(())
    }

    #[test]
    fn test_reordered_string_indices() {
        let text = "😀किa".repeat(1_024);
        let mut converter = StringIndexConverter::new(&text);
        for repetition in 0..1_024 {
            for (utf16, utf8) in [(0, 0), (3, 7), (2, 4), (1, 4), (4, 10), (5, 11)] {
                converter.advance_to_utf16_ix(repetition * 5 + utf16);
                assert_eq!(converter.utf8_ix, repetition * 11 + utf8);
            }
        }

        let text = format!(
            "{}😀{}{}a",
            "a".repeat(63),
            "कि".repeat(31),
            "😀".repeat(40)
        );
        let utf16_len = text.encode_utf16().count();
        let expected = |target: usize| {
            let mut utf16_ix = 0;
            text.char_indices()
                .find_map(|(utf8_ix, character)| {
                    if utf16_ix >= target {
                        return Some(utf8_ix);
                    }
                    utf16_ix += character.len_utf16();
                    None
                })
                .unwrap_or(text.len())
        };
        let mut converter = StringIndexConverter::new(&text);
        for target in 0..=utf16_len + 2 {
            converter.advance_to_utf16_ix(target);
            assert_eq!(converter.utf8_ix, expected(target), "monotone {target}");
        }
        assert_eq!(converter.checkpoints, None);
        for target in 0..=utf16_len + 2 {
            for target in [target, 0, utf16_len + 2, target, utf16_len + 2 - target] {
                converter.advance_to_utf16_ix(target);
                assert_eq!(converter.utf8_ix, expected(target), "adversarial {target}");
            }
        }
        assert_eq!(
            converter.checkpoints,
            Some(Vec::from([(0, 0), (65, 67), (129, 257), (193, 385)]))
        );
    }

    #[test]
    fn test_layout_line_bom_char() {
        let fonts = AppleTextSystem::new();
        let font_id = fonts.font_id(&font("Helvetica")).unwrap();
        let line = "\u{feff}";
        let mut style = FontRun {
            font_id,
            len: line.len(),
        };

        let layout = fonts.layout_line(line, px(16.), &[style]);
        assert_eq!(layout.len, line.len());
        assert!(layout.runs.is_empty());

        let line = "a\u{feff}b";
        style.len = line.len();
        let layout = fonts.layout_line(line, px(16.), &[style]);
        assert_eq!(layout.len, line.len());
        assert_eq!(layout.runs.len(), 1);
        assert_eq!(layout.runs[0].glyphs.len(), 2);
        assert_eq!(layout.runs[0].glyphs[0].id, GlyphId(68u32)); // a
        // There's no glyph for \u{feff}
        assert_eq!(layout.runs[0].glyphs[1].id, GlyphId(69u32)); // b

        let line = "\u{feff}ab";
        let font_runs = &[
            FontRun {
                len: "\u{feff}".len(),
                font_id,
            },
            FontRun {
                len: "ab".len(),
                font_id,
            },
        ];
        let layout = fonts.layout_line(line, px(16.), font_runs);
        assert_eq!(layout.len, line.len());
        assert_eq!(layout.runs.len(), 1);
        assert_eq!(layout.runs[0].glyphs.len(), 2);
        // There's no glyph for \u{feff}
        assert_eq!(layout.runs[0].glyphs[0].id, GlyphId(68u32)); // a
        assert_eq!(layout.runs[0].glyphs[1].id, GlyphId(69u32)); // b
    }

    #[test]
    fn test_bom_source_indices_match_native_pens() -> Result<()> {
        let fonts = AppleTextSystem::new();
        let font_id = fonts.font_id(&font("Helvetica"))?;
        let font_size = 16_f32;
        for (text, lengths) in [
            ("\u{feff}ab", vec![5]),
            ("\u{feff}ab", vec![3, 2]),
            ("a\u{feff}b", vec![1, 4]),
            ("\u{feff}\u{feff}ab", vec![3, 3, 2]),
            ("a\u{feff}\u{feff}b", vec![1, 3, 4]),
            ("\u{feff}😀ab", vec![9]),
        ] {
            let runs = lengths
                .into_iter()
                .map(|len| FontRun { len, font_id })
                .collect::<Vec<_>>();
            assert_eq!(runs.iter().map(|run| run.len).sum::<usize>(), text.len());
            let characters = text.encode_utf16().collect::<Vec<_>>();
            let native_string = unsafe {
                CFString::wrap_under_create_rule(CFStringCreateWithCharacters(
                    std::ptr::null(),
                    characters.as_ptr(),
                    characters.len() as isize,
                ))
            };
            let mut string = CFMutableAttributedString::new();
            string.replace_str(&native_string, CFRange::init(0, 0));
            assert_eq!(string.char_len() as usize, characters.len());
            let mut byte_offset = 0;
            let mut utf16_offset = 0;
            for (index, run) in runs.iter().enumerate() {
                let utf16_len = text[byte_offset..byte_offset + run.len]
                    .encode_utf16()
                    .count();
                let size = if index % 2 == 0 {
                    font_size.next_up()
                } else {
                    font_size
                };
                let native_font = fonts.0.read().fonts[font_id.0]
                    .native_font()
                    .clone_with_font_size(f64::from(size));
                unsafe {
                    string.set_attribute(
                        CFRange::init(utf16_offset as isize, utf16_len as isize),
                        kCTFontAttributeName,
                        &native_font,
                    );
                }
                byte_offset += run.len;
                utf16_offset += utf16_len;
            }
            let native = CTLine::new_with_attributed_string(string.as_concrete_TypeRef());
            let expected_indices = text
                .char_indices()
                .filter_map(|(index, character)| (character != '\u{feff}').then_some(index))
                .collect::<Vec<_>>();
            let native_positions = native
                .glyph_runs()
                .iter()
                .flat_map(|run| {
                    run.string_indices()
                        .iter()
                        .copied()
                        .zip(run.positions().iter().map(|position| position.x))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let expected_carets = expected_indices
                .iter()
                .map(|&index| {
                    let utf16_index = text[..index].encode_utf16().count() as isize;
                    let position = native_positions
                        .iter()
                        .find(|(source, _)| *source == utf16_index)
                        .expect("native source glyph")
                        .1;
                    (index, position)
                })
                .chain([(text.len(), native.get_typographic_bounds().width)])
                .collect::<Vec<_>>();
            let RetainedLineLayout::Dense(retained, _, _) =
                fonts.layout_line_for_retention(text, px(font_size), &runs)
            else {
                panic!("BOMs require dense geometry");
            };
            for layout in [
                fonts.layout_line(text, px(font_size), &runs),
                fonts
                    .layout_line_with_precise_positions(text, px(font_size), &runs)
                    .0,
                retained,
            ] {
                assert_eq!(layout.len, text.len());
                assert_eq!(
                    layout
                        .runs
                        .iter()
                        .flat_map(|run| &run.glyphs)
                        .map(|glyph| glyph.index)
                        .collect::<Vec<_>>(),
                    expected_indices,
                    "{text:?}",
                );
                for index in text
                    .char_indices()
                    .map(|(index, _)| index)
                    .chain([text.len()])
                {
                    let expected = expected_carets
                        .iter()
                        .find(|(boundary, _)| *boundary >= index)
                        .expect("native boundary")
                        .1;
                    assert_eq!(
                        layout.caret_x_for_index(index),
                        expected,
                        "{text:?} at {index}"
                    );
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_layout_line_zwnj_insertion() {
        let fonts = AppleTextSystem::new();
        let font_id = fonts.font_id(&font("Helvetica")).unwrap();

        let text = "hello world";
        let font_runs = &[
            FontRun { font_id, len: 5 }, // "hello"
            FontRun { font_id, len: 6 }, // " world"
        ];

        let layout = fonts.layout_line(text, px(16.), font_runs);
        assert_eq!(layout.len, text.len());

        for run in &layout.runs {
            for glyph in &run.glyphs {
                assert!(
                    glyph.index < text.len(),
                    "Glyph index {} is out of bounds for text length {}",
                    glyph.index,
                    text.len()
                );
            }
        }

        // Test with different font runs - should not insert ZWNJ
        let font_id2 = fonts.font_id(&font("Times")).unwrap_or(font_id);
        let font_runs_different = &[
            FontRun { font_id, len: 5 }, // "hello"
            // " world"
            FontRun {
                font_id: font_id2,
                len: 6,
            },
        ];

        let layout2 = fonts.layout_line(text, px(16.), font_runs_different);
        assert_eq!(layout2.len, text.len());

        for run in &layout2.runs {
            for glyph in &run.glyphs {
                assert!(
                    glyph.index < text.len(),
                    "Glyph index {} is out of bounds for text length {}",
                    glyph.index,
                    text.len()
                );
            }
        }
    }

    #[test]
    fn test_layout_line_zwnj_edge_cases() {
        let fonts = AppleTextSystem::new();
        let font_id = fonts.font_id(&font("Helvetica")).unwrap();

        let text = "hello";
        let font_runs = &[FontRun { font_id, len: 5 }];
        let layout = fonts.layout_line(text, px(16.), font_runs);
        assert_eq!(layout.len, text.len());

        let text = "abc";
        let font_runs = &[
            FontRun { font_id, len: 1 }, // "a"
            FontRun { font_id, len: 1 }, // "b"
            FontRun { font_id, len: 1 }, // "c"
        ];
        let layout = fonts.layout_line(text, px(16.), font_runs);
        assert_eq!(layout.len, text.len());

        for run in &layout.runs {
            for glyph in &run.glyphs {
                assert!(
                    glyph.index < text.len(),
                    "Glyph index {} is out of bounds for text length {}",
                    glyph.index,
                    text.len()
                );
            }
        }

        // Test with empty text
        let text = "";
        let font_runs = &[];
        let layout = fonts.layout_line(text, px(16.), font_runs);
        assert_eq!(layout.len, 0);
        assert!(layout.runs.is_empty());
    }

    fn assert_retained_matches_native(
        fonts: &AppleTextSystem,
        text: &str,
        font_size: Pixels,
        runs: &[FontRun],
        require_compact: bool,
    ) {
        let (expected, positions, width) =
            fonts.layout_line_with_precise_positions(text, font_size, runs);
        let retained = fonts.layout_line_for_retention(text, font_size, runs);
        let (actual, actual_positions, actual_width) = match retained {
            RetainedLineLayout::Dense(layout, positions, width) => {
                assert!(
                    !require_compact,
                    "expected compact output for {} bytes at {font_size:?}",
                    text.len()
                );
                (layout, positions, width)
            }
            RetainedLineLayout::Compact(compact) => {
                assert_eq!(expected.runs.len(), 1);
                assert_eq!(compact.font_id, expected.runs[0].font_id);
                assert_eq!(compact.font_size, expected.font_size);
                assert_eq!(compact.ascent, expected.ascent);
                assert_eq!(compact.descent, expected.descent);
                assert_eq!(compact.width.to_bits(), width.to_bits());
                assert_eq!(compact.glyphs.len(), text.len());
                assert_eq!(compact.positions.len(), positions.len());
                assert_eq!(expected.caret_positions.len(), text.len() + 1);
                for (index, glyph) in expected.runs[0].glyphs.iter().enumerate() {
                    assert_eq!(GlyphId(u32::from(compact.glyphs[index])), glyph.id);
                    assert_eq!(glyph.index, index);
                    assert_eq!(glyph.position.y, Pixels::ZERO);
                    assert_eq!(compact.is_emoji, glyph.is_emoji);
                    assert_eq!(
                        compact.positions.get(index).unwrap().to_bits(),
                        positions[index].to_bits()
                    );
                    assert_eq!(expected.caret_positions[index], (index, positions[index]));
                }
                assert_eq!(
                    expected.caret_positions.last(),
                    Some(&(text.len(), compact.width))
                );
                return;
            }
        };
        assert_eq!(actual_width.to_bits(), width.to_bits());
        assert_eq!(actual.caret_positions, expected.caret_positions);
        assert_eq!(
            actual_positions
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            positions
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            (
                actual.font_size,
                actual.width,
                actual.ascent,
                actual.descent,
                actual.len
            ),
            (
                expected.font_size,
                expected.width,
                expected.ascent,
                expected.descent,
                expected.len
            )
        );
        assert_eq!(actual.runs.len(), expected.runs.len());
        for (actual, expected) in actual.runs.iter().zip(&expected.runs) {
            assert_eq!(actual.font_id, expected.font_id);
            assert_eq!(actual.glyphs.len(), expected.glyphs.len());
            for (actual, expected) in actual.glyphs.iter().zip(&expected.glyphs) {
                assert_eq!(
                    (actual.id, actual.index, actual.position, actual.is_emoji),
                    (
                        expected.id,
                        expected.index,
                        expected.position,
                        expected.is_emoji
                    )
                );
            }
        }
    }

    fn test_fonts() -> AppleTextSystem {
        let fonts = AppleTextSystem::new();
        fonts
            .add_fonts(vec![
                Cow::Borrowed(
                    include_bytes!("../../../assets/fonts/lilex/Lilex-Regular.ttf").as_slice(),
                ),
                Cow::Borrowed(
                    include_bytes!("../../../assets/fonts/lilex/Lilex-Bold.ttf").as_slice(),
                ),
                Cow::Borrowed(
                    include_bytes!("../../../assets/fonts/lilex/Lilex-Italic.ttf").as_slice(),
                ),
                Cow::Borrowed(
                    include_bytes!("../../../assets/fonts/lilex/Lilex-BoldItalic.ttf").as_slice(),
                ),
            ])
            .unwrap();
        fonts
    }
}
