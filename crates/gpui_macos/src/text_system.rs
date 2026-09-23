use anyhow::anyhow;
use cocoa::appkit::CGFloat;
use collections::{HashMap, HashSet};
use core_foundation::{
    array::{CFArray, CFArrayRef},
    attributed_string::CFMutableAttributedString,
    base::{CFRange, CFType, TCFType},
    data::CFData,
    number::CFNumber,
    string::CFString,
};
use core_graphics::{
    base::{CGGlyph, kCGImageAlphaPremultipliedLast},
    color_space::CGColorSpace,
    context::{CGContext, CGTextDrawingMode},
    display::CGPoint,
};
use core_text::{
    font::CTFont,
    font_collection::CTFontCollectionRef,
    font_descriptor::{
        CTFontDescriptor, kCTFontSlantTrait, kCTFontSymbolicTrait, kCTFontWeightTrait,
        kCTFontWidthTrait,
    },
    line::CTLine,
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
    Bounds, DevicePixels, Font, FontFallbacks, FontFeatures, FontId, FontMetrics, FontRun,
    FontStyle, FontWeight, GlyphId, Hsla, LineLayout, Pixels, PlatformTextSystem,
    RenderGlyphParams, Result, Rgba, SUBPIXEL_VARIANTS_X, ShapedGlyph, ShapedRun, SharedString,
    Size, TextRenderingMode, point, px, size, swap_rgba_pa_to_bgra,
};
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use pathfinder_geometry::{
    rect::{RectF, RectI},
    transform2d::Transform2F,
    vector::Vector2F,
};
use smallvec::SmallVec;
use std::{borrow::Cow, char, convert::TryFrom, sync::Arc, sync::OnceLock};

use crate::open_type::apply_features_and_fallbacks;

#[allow(non_upper_case_globals)]
const kCGImageAlphaOnly: u32 = 7;

/// macOS text system using CoreText for font shaping.
pub struct MacTextSystem(RwLock<MacTextSystemState>);

#[derive(Clone, PartialEq, Eq, Hash)]
struct FontKey {
    font_family: SharedString,
    font_features: FontFeatures,
    font_fallbacks: Option<FontFallbacks>,
}

struct MacTextSystemState {
    memory_source: MemSource,
    system_source: SystemSource,
    fonts: Vec<FontKitFont>,
    font_selections: HashMap<Font, FontId>,
    font_ids_by_postscript_name: HashMap<String, FontId>,
    font_ids_by_font_key: HashMap<FontKey, SmallVec<[FontId; 4]>>,
    postscript_names_by_font_id: HashMap<FontId, String>,
}

impl MacTextSystem {
    /// Create a new MacTextSystem.
    pub fn new() -> Self {
        Self(RwLock::new(MacTextSystemState {
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

impl Default for MacTextSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformTextSystem for MacTextSystem {
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
        self.0.write().layout_line(text, font_size, font_runs)
    }

    fn recommended_rendering_mode(
        &self,
        _font_id: FontId,
        _font_size: Pixels,
    ) -> TextRenderingMode {
        TextRenderingMode::Grayscale
    }

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

    fn ascii_shaping_preserves_advances(&self, font_id: FontId, features: &FontFeatures) -> bool {
        let lock = self.0.read();
        let Some(font) = lock.fonts.get(font_id.0) else {
            return false;
        };
        let native = font.native_font();
        let table = |tag: &[u8; 4]| native.get_font_table(u32::from_be_bytes(*tag));
        let (Some(head), Some(hhea), Some(maxp)) = (table(b"head"), table(b"hhea"), table(b"maxp"))
        else {
            return false;
        };
        let (hmtx, gsub, gpos, morx, kern, kerx) = (
            table(b"hmtx"),
            table(b"GSUB"),
            table(b"GPOS"),
            table(b"morx"),
            table(b"kern"),
            table(b"kerx"),
        );
        let Some(ascii_glyphs) = (0x20u8..=0x7E)
            .map(|byte| font.glyph_for_char(byte as char).map(|glyph| glyph as u16))
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        gpui::ascii_shaping_preserves_advances(
            gpui::ShapingTables {
                head: head.bytes(),
                hhea: hhea.bytes(),
                maxp: maxp.bytes(),
                hmtx: hmtx.as_ref().map(CFData::bytes),
                gsub: gsub.as_ref().map(CFData::bytes),
                gpos: gpos.as_ref().map(CFData::bytes),
                morx: morx.as_ref().map(CFData::bytes),
                kern: kern.as_ref().map(CFData::bytes),
                kerx: kerx.as_ref().map(CFData::bytes),
            },
            &ascii_glyphs,
            features.tag_value_list(),
            |glyph| {
                font.advance(u32::from(glyph))
                    .ok()
                    .map(|advance| advance.x())
            },
        )
    }
}

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

impl MacTextSystemState {
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
        Ok(bounds.dilate(DevicePixels(1)))
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        glyph_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        if glyph_bounds.size.width.0 == 0 || glyph_bounds.size.height.0 == 0 {
            anyhow::bail!("glyph bounds are empty");
        } else {
            // Add an extra pixel when the subpixel variant isn't zero to make room for anti-aliasing.
            let mut bitmap_size = glyph_bounds.size;
            if params.subpixel_variant.x > 0 {
                bitmap_size.width += DevicePixels(1);
            }
            if params.subpixel_variant.y > 0 {
                bitmap_size.height += DevicePixels(1);
            }
            let bitmap_size = bitmap_size;

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
                (glyph_bounds.origin.y.0 + glyph_bounds.size.height.0) as CGFloat,
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

            if params.dilation > 0 {
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

    fn layout_line(&mut self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        // Construct the attributed string, converting UTF8 ranges to UTF16 ranges.
        let mut string = CFMutableAttributedString::new();
        let mut max_ascent = 0.0f32;
        let mut max_descent = 0.0f32;

        {
            let mut text = text;
            let mut break_ligature = true;
            for run in font_runs {
                let text_run;
                (text_run, text) = text.split_at(run.len);

                let utf16_start = string.char_len(); // insert at end of string
                // note: replace_str may silently ignore codepoints it dislikes (e.g., BOM at start of string)
                string.replace_str(&CFString::new(text_run), CFRange::init(utf16_start, 0));
                let utf16_end = string.char_len();

                let length = utf16_end - utf16_start;
                let cf_range = CFRange::init(utf16_start, length);
                let font = &self.fonts[run.font_id.0];

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
        // Retrieve the glyphs from the shaped line, converting UTF16 offsets to UTF8 offsets.
        let line = CTLine::new_with_attributed_string(string.as_concrete_TypeRef());
        let glyph_runs = line.glyph_runs();
        let mut runs = <Vec<ShapedRun>>::with_capacity(glyph_runs.len() as usize);
        let mut ix_converter = StringIndexConverter::new(text);
        for run in glyph_runs.into_iter() {
            let attributes = run.attributes().unwrap();
            let font = unsafe {
                attributes
                    .get(kCTFontAttributeName)
                    .downcast::<CTFont>()
                    .unwrap()
            };
            let font_id = self.id_for_native_font(font);

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
                if ix_converter.utf16_ix > glyph_utf16_ix {
                    // We cannot reuse current index converter, as it can only seek forward. Restart the search.
                    ix_converter = StringIndexConverter::new(text);
                }
                ix_converter.advance_to_utf16_ix(glyph_utf16_ix);
                glyphs.push(ShapedGlyph {
                    id: GlyphId(glyph_id as u32),
                    position: point(position.x as f32, position.y as f32).map(px),
                    index: ix_converter.utf8_ix,
                    is_emoji: self.is_emoji(font_id),
                });
            }
        }
        let typographic_bounds = line.get_typographic_bounds();
        LineLayout {
            runs,
            font_size,
            width: typographic_bounds.width.into(),
            ascent: max_ascent.into(),
            descent: max_descent.into(),
            len: text.len(),
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
}

impl<'a> StringIndexConverter<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            utf8_ix: 0,
            utf16_ix: 0,
        }
    }

    fn advance_to_utf16_ix(&mut self, utf16_target: usize) {
        for (ix, c) in self.text[self.utf8_ix..].char_indices() {
            if self.utf16_ix >= utf16_target {
                self.utf8_ix += ix;
                return;
            }
            self.utf16_ix += c.len_utf16();
        }
        self.utf8_ix = self.text.len();
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
    use crate::MacTextSystem;
    use gpui::{Font, FontFeatures, FontRun, FontWeight, GlyphId, PlatformTextSystem, font, px};
    use std::sync::Arc;

    #[test]
    fn test_monospace_advances_are_style_independent_and_cell_aligned() {
        let fonts = MacTextSystem::new();
        let regular = fonts.font_id(&font("Menlo")).unwrap();
        let bold = fonts
            .font_id(&Font {
                weight: FontWeight::BOLD,
                ..font("Menlo")
            })
            .unwrap();
        let width = |font_id, text: &str| {
            fonts
                .layout_line(
                    text,
                    px(14.),
                    &[FontRun {
                        font_id,
                        len: text.len(),
                    }],
                )
                .width
        };
        let cell = width(regular, "m");
        for sample in ["x", "W", "i", "const value = 12345;", "界", "漢字仮名"] {
            let regular_width = width(regular, sample);
            let bold_width = width(bold, sample);
            assert!(
                (regular_width - bold_width).abs() < px(0.01),
                "{sample:?}: regular {regular_width:?} vs bold {bold_width:?}"
            );
        }
        for sample in ["x", "W", "i", "const value = 12345;"] {
            let cells = width(regular, sample) / cell;
            assert!(
                (cells - cells.round()).abs() < 0.01,
                "{sample:?} spans {cells} cells"
            );
        }
        let cjk_cells = width(regular, "界") / cell;
        assert!(
            (cjk_cells - cjk_cells.round()).abs() > 0.1,
            "CJK fallback glyphs span {cjk_cells} cells; the grid path must keep excluding them"
        );
    }

    #[test]
    fn test_context_shaped_chunk_widths_are_boundary_independent() {
        let fonts = MacTextSystem::new();
        let font_id = fonts.font_id(&font("Times")).unwrap();
        let layout = |text: &str| {
            fonts.layout_line(
                text,
                px(14.),
                &[FontRun {
                    font_id,
                    len: text.len(),
                }],
            )
        };
        let text = format!("a{}", "fi".repeat(4_096));
        let full_width = layout(&text).width;
        let context = 64;
        let chunk_len = 2_048;
        let mut standalone_sums = Vec::new();
        for first_boundary in [chunk_len, chunk_len + 1] {
            let mut context_sum = px(0.);
            let mut standalone_sum = px(0.);
            let mut start = 0;
            let mut boundary = first_boundary;
            while start < text.len() {
                let end = boundary.min(text.len());
                let context_start = start.saturating_sub(context);
                let context_end = (end + context).min(text.len());
                let shaped = layout(&text[context_start..context_end]);
                context_sum += shaped.x_for_index(end - context_start)
                    - shaped.x_for_index(start - context_start);
                standalone_sum += layout(&text[start..end]).width;
                start = end;
                boundary += chunk_len;
            }
            assert!(
                (context_sum - full_width).abs() < px(0.01),
                "chunks from {first_boundary}: {context_sum:?} vs {full_width:?}"
            );
            standalone_sums.push(standalone_sum);
        }
        assert!(
            (standalone_sums[0] - standalone_sums[1]).abs() > px(1.),
            "standalone chunks must expose the broken ligatures this test guards against: {standalone_sums:?}"
        );
    }

    #[test]
    fn test_grid_exactness_needs_every_printable_ascii_glyph() {
        let fonts = MacTextSystem::new();
        let width = |font_id, text: &str| {
            fonts
                .layout_line(
                    text,
                    px(14.),
                    &[FontRun {
                        font_id,
                        len: text.len(),
                    }],
                )
                .width
        };
        let ayuthaya = fonts.font_id(&font("Ayuthaya")).unwrap();
        let cell = width(ayuthaya, "m");
        assert_eq!(width(ayuthaya, "i"), cell);
        assert_eq!(width(ayuthaya, "W"), cell);
        assert_ne!(
            width(ayuthaya, " "),
            cell,
            "Ayuthaya's space must differ from its letter cell for this test to be meaningful"
        );
        let spaces = " ".repeat(2_048);
        assert!((width(ayuthaya, &spaces) - cell * 2_048.).abs() > px(100.));
        for family in ["Menlo"] {
            for weight in [FontWeight::NORMAL, FontWeight::BOLD] {
                let font_id = fonts
                    .font_id(&Font {
                        weight,
                        ..font(family)
                    })
                    .unwrap();
                let cell = width(font_id, "m");
                for byte in 0x20u8..=0x7E {
                    let glyph = (byte as char).to_string();
                    assert_eq!(
                        width(font_id, &glyph),
                        cell,
                        "{family} {weight:?} {glyph:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_chunks_cut_beside_isolated_chars_telescope_across_ligature_chains() {
        let fonts = MacTextSystem::new();
        let layout = |font_id, text: &str| {
            fonts.layout_line(
                text,
                px(14.),
                &[FontRun {
                    font_id,
                    len: text.len(),
                }],
            )
        };
        let context = 64;
        let chunk_len = 2_048;
        let cases = [
            (
                "Hoefler Text",
                format!("a{}", "ffi fl ff ".repeat(1_000)),
                true,
            ),
            ("Hoefler Text", format!("a{}", "f".repeat(8_192)), false),
            ("Helvetica", "\u{628}\u{644}\u{62f} ".repeat(1_100), true),
        ];
        for (family, text, expect_exact) in cases {
            let font_id = fonts.font_id(&font(family)).unwrap();
            let full_width = layout(font_id, &text).width;
            let rtl = text.starts_with('\u{628}');
            let mut start = 0;
            let mut sum = px(0.);
            let mut cuts = 0;
            while start < text.len() {
                let mut end = (start + chunk_len).min(text.len());
                while end < text.len()
                    && !(text.is_char_boundary(end)
                        && (text[..end].ends_with(' ') || text[end..].starts_with(' ')))
                {
                    end += 1;
                }
                if end < text.len() {
                    cuts += 1;
                }
                if rtl {
                    sum += layout(font_id, &text[start..end]).width;
                } else {
                    let context_start = start.saturating_sub(context);
                    let context_end = (end + context).min(text.len());
                    let shaped = layout(font_id, &text[context_start..context_end]);
                    sum += shaped.x_for_index(end - context_start)
                        - shaped.x_for_index(start - context_start);
                }
                start = end;
            }
            assert_eq!(cuts > 0, expect_exact, "{family}: {cuts} cuts");
            assert!(
                (sum - full_width).abs() < px(0.05),
                "{family}: chunk sum {sum:?} vs full width {full_width:?}"
            );
        }
    }

    #[test]
    fn test_sliced_pieces_cut_without_context_never_duplicate_glyphs() {
        let text_system = Arc::new(gpui::TextSystem::new(Arc::new(MacTextSystem::new())));
        let window_text_system = gpui::WindowTextSystem::new(text_system);
        let text = format!("a{}", "f".repeat(12_000));
        let font = font("Hoefler Text");
        let run = |len: usize| gpui::TextRun {
            len,
            font: font.clone(),
            color: gpui::black(),
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let cut = 6_144usize;
        let shape_piece = |shaped_range: std::ops::Range<usize>, kept: std::ops::Range<usize>| {
            let piece = &text[shaped_range.clone()];
            window_text_system
                .shape_line(piece.to_string().into(), px(14.), &[run(piece.len())], None)
                .slice(
                    kept.start - shaped_range.start..kept.end - shaped_range.start,
                    None,
                )
        };
        let glyph_indices = |line: &gpui::ShapedLine, base: usize| {
            line.runs
                .iter()
                .flat_map(|run| run.glyphs.iter().map(move |glyph| glyph.index + base))
                .collect::<Vec<_>>()
        };

        let first = shape_piece(0..cut, 0..cut);
        let second = shape_piece(cut..text.len(), cut..text.len());
        let mut indices = glyph_indices(&first, 0);
        indices.extend(glyph_indices(&second, cut));
        assert!(indices.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(glyph_indices(&second, cut).first(), Some(&cut));
        assert!(
            glyph_indices(&first, 0)
                .last()
                .is_some_and(|index| *index < cut)
        );
        assert!(
            (first.width() + second.width()
                - window_text_system
                    .shape_line(text.clone().into(), px(14.), &[run(text.len())], None)
                    .width())
            .abs()
                > px(1.),
            "a 12,001-byte line exceeds CoreText's shaping limit, so whole-line width is not the reference"
        );

        let last_glyph_id = |line: &gpui::ShapedLine| {
            line.runs
                .last()
                .and_then(|run| run.glyphs.last())
                .map(|glyph| glyph.id)
        };
        let single_f = window_text_system.shape_line("f".into(), px(14.), &[run(1)], None);
        assert_eq!(last_glyph_id(&first), last_glyph_id(&single_f));

        let context = 64;
        let first_with_context = shape_piece(0..cut + context, 0..cut);
        let second_with_context = shape_piece(cut - context..text.len(), cut..text.len());
        assert_ne!(last_glyph_id(&first_with_context), last_glyph_id(&single_f));
        assert_eq!(glyph_indices(&second_with_context, cut).first(), Some(&cut));
    }

    #[test]
    fn test_coretext_shapes_ligatures_and_kerning_up_to_10240_code_units() {
        let fonts = MacTextSystem::new();
        let font_id = fonts.font_id(&font("Times")).unwrap();
        let layout = |text: &str| {
            fonts.layout_line(
                text,
                px(14.),
                &[FontRun {
                    font_id,
                    len: text.len(),
                }],
            )
        };
        let glyph_count = |text: &str| {
            layout(text)
                .runs
                .iter()
                .map(|run| run.glyphs.len())
                .sum::<usize>()
        };
        assert_eq!(glyph_count(&"fi".repeat(5_120)), 5_120);
        assert_eq!(glyph_count(&"fi".repeat(5_121)), 10_242);
        let kerned_pair = layout(&"AV".repeat(5_120)).width / 5_120.;
        let unkerned_pair = layout(&"AV".repeat(5_121)).width / 5_121.;
        assert!(
            unkerned_pair - kerned_pair > px(1.),
            "kerning must stop past 10240 code units: {kerned_pair:?} vs {unkerned_pair:?}"
        );
    }

    #[test]
    fn test_layout_line_bom_char() {
        let fonts = MacTextSystem::new();
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
    fn test_layout_line_zwnj_insertion() {
        let fonts = MacTextSystem::new();
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
        let fonts = MacTextSystem::new();
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

    #[test]
    fn test_code_ligature_ink_survives_cuts_beside_whitespace_and_hard_breaks_only() {
        let fonts = MacTextSystem::new();
        fonts
            .add_fonts(vec![
                include_bytes!("../../../assets/fonts/lilex/Lilex-Regular.ttf")
                    .as_slice()
                    .into(),
            ])
            .unwrap();
        let font_id = fonts.font_id(&font("Lilex")).unwrap();
        let font_size = px(14.);
        let scale = f32::from(font_size) / fonts.font_metrics(font_id).units_per_em as f32;
        let layout = |text: &str| {
            fonts.layout_line(
                text,
                font_size,
                &[FontRun {
                    font_id,
                    len: text.len(),
                }],
            )
        };
        let cell = layout("x").width;
        let glyphs = |layout: &gpui::LineLayout| {
            layout
                .runs
                .iter()
                .flat_map(|run| run.glyphs.iter().cloned())
                .collect::<Vec<_>>()
        };
        let inked_cells = |glyphs: &[gpui::ShapedGlyph]| {
            let mut cells = std::collections::BTreeSet::new();
            for glyph in glyphs {
                let bounds = fonts.typographic_bounds(font_id, glyph.id).unwrap();
                if bounds.size.width <= 0. {
                    continue;
                }
                let start = f32::from(glyph.position.x) + bounds.origin.x * scale;
                let end = start + bounds.size.width * scale;
                let first = ((start + 0.01) / f32::from(cell)).floor() as usize;
                let last = ((end - 0.01) / f32::from(cell)).ceil() as usize;
                cells.extend(first..last);
            }
            cells
        };
        let ids = |glyphs: &[gpui::ShapedGlyph]| {
            glyphs
                .iter()
                .map(|glyph| (glyph.index, glyph.id))
                .collect::<Vec<_>>()
        };
        let compose = |text: &str, cut: usize, context: usize| {
            let mut composed = Vec::new();
            for (piece, shaped) in [
                (0..cut, 0..(cut + context).min(text.len())),
                (cut..text.len(), cut.saturating_sub(context)..text.len()),
            ] {
                let shaped_layout = layout(&text[shaped.clone()]);
                let origin = shaped_layout.x_for_index(piece.start - shaped.start);
                let piece_x = cell * piece.start as f32;
                composed.extend(
                    glyphs(&shaped_layout)
                        .into_iter()
                        .filter(|glyph| piece.contains(&(glyph.index + shaped.start)))
                        .map(|glyph| gpui::ShapedGlyph {
                            index: glyph.index + shaped.start,
                            position: gpui::point(
                                glyph.position.x - origin + piece_x,
                                glyph.position.y,
                            ),
                            ..glyph
                        }),
                );
            }
            composed
        };

        let spaced = format!("{} {}", "!=".repeat(1_000), "!=".repeat(1_000));
        let whole = glyphs(&layout(&spaced));
        assert_eq!(inked_cells(&whole).len(), spaced.len() - 1);
        for cut in [2_000, 2_001] {
            let composed = compose(&spaced, cut, 64);
            assert_eq!(ids(&composed), ids(&whole), "cut at {cut}");
            assert_eq!(inked_cells(&composed), inked_cells(&whole), "cut at {cut}");
        }

        let unspaced = format!("a{}", "!=".repeat(1_050));
        let whole = glyphs(&layout(&unspaced));
        let all_cells = (0..unspaced.len()).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(inked_cells(&whole), all_cells);
        let cut = 2_048;
        assert_eq!(&unspaced[cut - 1..=cut], "!=");
        let with_context = compose(&unspaced, cut, 64);
        let missing = all_cells
            .difference(&inked_cells(&with_context))
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(missing, vec![cut - 2, cut - 1]);

        let hard_break = compose(&unspaced, cut, 0);
        assert_eq!(inked_cells(&hard_break), all_cells);
        assert_ne!(ids(&hard_break), ids(&whole));
        let positions = |glyphs: &[gpui::ShapedGlyph]| {
            glyphs
                .iter()
                .map(|glyph| (glyph.index, glyph.position.x))
                .collect::<Vec<_>>()
        };
        assert_eq!(positions(&hard_break), positions(&whole));
    }

    #[test]
    fn test_bidi_chunks_cut_before_whitespace_runs_compose_in_visual_order() {
        let fonts = MacTextSystem::new();
        let font_id = fonts.font_id(&font("Helvetica")).unwrap();
        let layout = |text: &str| {
            fonts.layout_line(
                text,
                px(14.),
                &[FontRun {
                    font_id,
                    len: text.len(),
                }],
            )
        };
        let positions = |layout: &gpui::LineLayout, base: usize, x: f32| {
            layout
                .runs
                .iter()
                .flat_map(|run| {
                    run.glyphs
                        .iter()
                        .map(move |glyph| (glyph.index + base, f32::from(glyph.position.x) + x))
                })
                .collect::<std::collections::BTreeMap<_, _>>()
        };
        let compose = |text: &str, cuts: &[usize], rtl: bool| {
            let mut bounds = vec![0];
            bounds.extend_from_slice(cuts);
            bounds.push(text.len());
            let chunks = bounds
                .windows(2)
                .map(|pair| (pair[0], layout(&text[pair[0]..pair[1]])))
                .collect::<Vec<_>>();
            let total = chunks
                .iter()
                .map(|(_, chunk)| f32::from(chunk.width))
                .sum::<f32>();
            let mut composed = std::collections::BTreeMap::new();
            let mut cumulative = 0.;
            for (start, chunk) in &chunks {
                let width = f32::from(chunk.width);
                let x = if rtl {
                    total - cumulative - width
                } else {
                    cumulative
                };
                composed.extend(positions(chunk, *start, x));
                cumulative += width;
            }
            composed
        };
        let mismatches = |text: &str, cuts: &[usize], rtl: bool| {
            let reference = positions(&layout(text), 0, 0.);
            let composed = compose(text, cuts, rtl);
            assert_eq!(composed.len(), reference.len());
            reference
                .iter()
                .filter(|(index, x)| (composed[index] - **x).abs() > 0.05)
                .count()
        };

        let hebrew = format!(
            "{} {} {}",
            "\u{5d0}".repeat(300),
            "\u{5d1}".repeat(100),
            "\u{5d2}".repeat(100)
        );
        assert_eq!(mismatches(&hebrew, &[600, 801], true), 0);
        assert_ne!(mismatches(&hebrew, &[601, 802], true), 0);
        assert_ne!(mismatches(&hebrew, &[600, 801], false), 0);

        let double_space = format!("{}  {}", "\u{5d0}".repeat(300), "\u{5d1}".repeat(100));
        assert_eq!(mismatches(&double_space, &[600], true), 0);
        assert_ne!(mismatches(&double_space, &[601], true), 0);

        let mixed = "\u{5d0}\u{5d1}\u{5d2} abc 123 \u{5d3}\u{5d4}\u{5d5} (x) \u{5d6}\u{5d7}\u{5d8} 45,6 \u{5d0}\u{5d1} end \u{5d2}\u{5d3}";
        let before_hebrew_words = [14, 25, 37, 46];
        for cut in before_hebrew_words {
            assert_eq!(&mixed[cut..cut + 1], " ");
            assert_eq!(mismatches(mixed, &[cut], true), 0, "cut at {cut}");
        }
        assert_eq!(mismatches(mixed, &before_hebrew_words, true), 0);
        for (cut, following) in [(6, " abc"), (10, " 123"), (21, " (x)"), (42, " end")] {
            assert!(mixed[cut..].starts_with(following));
            assert_ne!(
                mismatches(mixed, &[cut], true),
                0,
                "cut before {following:?}"
            );
        }

        let embedded = "abc \u{5d0}\u{5d1}\u{5d2} \u{5d3}\u{5d4} def ghi \u{5d5}\u{5d6} jkl";
        let before_latin_words = ["def", "ghi", "jkl"]
            .map(|word| embedded.find(word).unwrap() - 1)
            .to_vec();
        assert_eq!(mismatches(embedded, &before_latin_words, false), 0);
        assert_ne!(
            mismatches(embedded, &[embedded.find("\u{5d3}").unwrap() - 1], false),
            0
        );

        let numbers_first = "123 456 \u{5d0}\u{5d1}\u{5d2} \u{5d3}\u{5d4}";
        assert_ne!(mismatches(numbers_first, &[7], true), 0);
        assert_eq!(mismatches(numbers_first, &[14], true), 0);

        let arabic = format!(
            "{} {} {}",
            "\u{628}".repeat(200),
            "\u{644}".repeat(100),
            "\u{62f}".repeat(100)
        );
        assert_eq!(mismatches(&arabic, &[400, 601], true), 0);
    }

    #[test]
    fn test_ascii_shaping_invariance_analysis_matches_native_shaping() {
        let fonts = MacTextSystem::new();
        fonts
            .add_fonts(vec![
                include_bytes!("../../../assets/fonts/lilex/Lilex-Regular.ttf")
                    .as_slice()
                    .into(),
            ])
            .unwrap();
        let with_features = |family: &str, features: &[(&str, u32)]| Font {
            features: FontFeatures(Arc::new(
                features
                    .iter()
                    .map(|(tag, value)| (tag.to_string(), *value))
                    .collect(),
            )),
            ..font(family)
        };
        let probes = [
            "1/2 ".repeat(512),
            "fi ".repeat(512),
            "ffi fl ff ".repeat(200),
            "!= -> === <= >= :: <!-- --> www ".repeat(64),
            (0x20u8..=0x7E).map(|byte| byte as char).collect::<String>(),
        ];
        let cases = [
            ("Lilex", &[][..], true),
            ("Lilex", &[("frac", 1)][..], false),
            ("Lilex", &[("calt", 0)][..], true),
            ("Zed Plex Mono", &[][..], true),
            ("Zed Plex Mono", &[("frac", 1)][..], false),
            ("Menlo", &[][..], true),
            ("Menlo", &[("liga", 1)][..], false),
            ("Menlo", &[("liga", 0)][..], true),
            ("Monaco", &[][..], true),
        ];
        for (family, features, expected) in cases {
            let font = with_features(family, features);
            let font_id = fonts.font_id(&font).unwrap();
            let analysis = fonts.ascii_shaping_preserves_advances(font_id, &font.features);
            assert_eq!(analysis, expected, "{family} {features:?}");
            let layout = |text: &str| {
                fonts.layout_line(
                    text,
                    px(14.),
                    &[FontRun {
                        font_id,
                        len: text.len(),
                    }],
                )
            };
            let cell = layout("x").width;
            let grid_exact = probes
                .iter()
                .all(|probe| (layout(probe).width - cell * probe.len() as f32).abs() < px(0.01));
            if analysis {
                assert!(
                    grid_exact,
                    "{family} {features:?}: analysis passed but shaping drifts"
                );
            } else {
                assert!(
                    !grid_exact || features.is_empty(),
                    "{family} {features:?}: analysis rejected a font that shaped on the grid"
                );
            }
        }
        let times = fonts.font_id(&font("Times")).unwrap();
        assert!(!fonts.ascii_shaping_preserves_advances(times, &FontFeatures::default()));
    }

    #[test]
    fn test_glyph_boundaries_exclude_positions_inside_ligatures() {
        let fonts = MacTextSystem::new();
        let font_id = fonts.font_id(&font("Times")).unwrap();
        let text = format!("{}fi{}", "x".repeat(100), "x".repeat(100));
        let layout = fonts.layout_line(
            &text,
            px(14.),
            &[FontRun {
                font_id,
                len: text.len(),
            }],
        );
        assert!(layout.is_glyph_boundary(100));
        assert!(!layout.is_glyph_boundary(101));
        assert!(layout.is_glyph_boundary(102));
        assert!(layout.is_glyph_boundary(0));
        assert!(layout.is_glyph_boundary(text.len()));
        let glyph_count = layout
            .runs
            .iter()
            .map(|run| run.glyphs.len())
            .sum::<usize>();
        assert_eq!(glyph_count, text.len() - 1);
    }
}
