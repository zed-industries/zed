use std::{borrow::Cow, mem::ManuallyDrop, sync::Arc};

use ::util::ResultExt;
use anyhow::Result;
use collections::HashMap;
use itertools::Itertools;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use windows::{
    Win32::{
        Foundation::*,
        Globalization::GetUserDefaultLocaleName,
        Graphics::{DirectWrite::*, Dxgi::Common::*, Gdi::LOGFONTW, Imaging::*},
        System::SystemServices::LOCALE_NAME_MAX_LENGTH,
        UI::WindowsAndMessaging::*,
    },
    core::*,
};
use windows_numerics::Vector2;

use crate::*;

#[derive(Debug)]
struct FontInfo {
    font_family: String,
    font_face: IDWriteFontFace3,
    features: IDWriteTypography,
    fallbacks: Option<IDWriteFontFallback>,
    is_system_font: bool,
}

pub(crate) struct DirectWriteTextSystem(RwLock<DirectWriteState>);

struct DirectWriteComponent {
    locale: String,
    factory: IDWriteFactory5,
    bitmap_factory: AgileReference<IWICImagingFactory>,
    in_memory_loader: IDWriteInMemoryFontFileLoader,
    builder: IDWriteFontSetBuilder1,
    text_renderer: Arc<TextRendererWrapper>,
    render_context: GlyphRenderContext,
}

struct GlyphRenderContext {
    params: IDWriteRenderingParams3,
}

struct DirectWriteState {
    components: DirectWriteComponent,
    system_ui_font_name: SharedString,
    system_font_collection: IDWriteFontCollection1,
    custom_font_collection: IDWriteFontCollection1,
    fonts: Vec<FontInfo>,
    font_selections: HashMap<Font, FontId>,
    font_id_by_identifier: HashMap<FontIdentifier, FontId>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct FontIdentifier {
    postscript_name: String,
    weight: i32,
    style: i32,
}

impl DirectWriteComponent {
    pub fn new(bitmap_factory: &IWICImagingFactory) -> Result<Self> {
        unsafe {
            let factory: IDWriteFactory5 = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;
            let bitmap_factory = AgileReference::new(bitmap_factory)?;
            // The `IDWriteInMemoryFontFileLoader` here is supported starting from
            // Windows 10 Creators Update, which consequently requires the entire
            // `DirectWriteTextSystem` to run on `win10 1703`+.
            let in_memory_loader = factory.CreateInMemoryFontFileLoader()?;
            factory.RegisterFontFileLoader(&in_memory_loader)?;
            let builder = factory.CreateFontSetBuilder()?;
            let mut locale_vec = vec![0u16; LOCALE_NAME_MAX_LENGTH as usize];
            GetUserDefaultLocaleName(&mut locale_vec);
            let locale = String::from_utf16_lossy(&locale_vec);
            let text_renderer = Arc::new(TextRendererWrapper::new(&locale));
            let render_context = GlyphRenderContext::new(&factory)?;

            Ok(DirectWriteComponent {
                locale,
                factory,
                bitmap_factory,
                in_memory_loader,
                builder,
                text_renderer,
                render_context,
            })
        }
    }
}

impl GlyphRenderContext {
    pub fn new(factory: &IDWriteFactory5) -> Result<Self> {
        unsafe {
            let default_params: IDWriteRenderingParams3 =
                factory.CreateRenderingParams()?.cast()?;
            let gamma = default_params.GetGamma();
            let enhanced_contrast = default_params.GetEnhancedContrast();
            let gray_contrast = default_params.GetGrayscaleEnhancedContrast();
            let cleartype_level = default_params.GetClearTypeLevel();
            let grid_fit_mode = default_params.GetGridFitMode();

            let params = factory.CreateCustomRenderingParams(
                gamma,
                enhanced_contrast,
                gray_contrast,
                cleartype_level,
                DWRITE_PIXEL_GEOMETRY_RGB,
                DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC,
                grid_fit_mode,
            )?;

            Ok(Self { params })
        }
    }
}

impl DirectWriteTextSystem {
    pub(crate) fn new(bitmap_factory: &IWICImagingFactory) -> Result<Self> {
        let components = DirectWriteComponent::new(bitmap_factory)?;
        let system_font_collection = unsafe {
            let mut result = std::mem::zeroed();
            components
                .factory
                .GetSystemFontCollection(false, &mut result, true)?;
            result.unwrap()
        };
        let custom_font_set = unsafe { components.builder.CreateFontSet()? };
        let custom_font_collection = unsafe {
            components
                .factory
                .CreateFontCollectionFromFontSet(&custom_font_set)?
        };
        let system_ui_font_name = get_system_ui_font_name();

        Ok(Self(RwLock::new(DirectWriteState {
            components,
            system_ui_font_name,
            system_font_collection,
            custom_font_collection,
            fonts: Vec::new(),
            font_selections: HashMap::default(),
            font_id_by_identifier: HashMap::default(),
        })))
    }
}

impl PlatformTextSystem for DirectWriteTextSystem {
    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        self.0.write().add_fonts(fonts)
    }

    fn all_font_names(&self) -> Vec<String> {
        self.0.read().all_font_names()
    }

    fn font_id(&self, font: &Font) -> Result<FontId> {
        let lock = self.0.upgradable_read();
        if let Some(font_id) = lock.font_selections.get(font) {
            Ok(*font_id)
        } else {
            let mut lock = RwLockUpgradableReadGuard::upgrade(lock);
            let font_id = lock.select_font(font);
            lock.font_selections.insert(font.clone(), font_id);
            Ok(font_id)
        }
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        self.0.read().font_metrics(font_id)
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        self.0.read().get_typographic_bounds(font_id, glyph_id)
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<Size<f32>> {
        self.0.read().get_advance(font_id, glyph_id)
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.0.read().glyph_for_char(font_id, ch)
    }

    fn glyph_raster_bounds(
        &self,
        params: &RenderGlyphParams,
    ) -> anyhow::Result<Bounds<DevicePixels>> {
        self.0.read().raster_bounds(params)
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> anyhow::Result<(Size<DevicePixels>, Vec<u8>)> {
        self.0.read().rasterize_glyph(params, raster_bounds)
    }

    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout {
        self.0
            .write()
            .layout_line(text, font_size, runs)
            .log_err()
            .unwrap_or(LineLayout {
                font_size,
                ..Default::default()
            })
    }
}

impl DirectWriteState {
    fn add_fonts(&mut self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        for font_data in fonts {
            match font_data {
                Cow::Borrowed(data) => unsafe {
                    let font_file = self
                        .components
                        .in_memory_loader
                        .CreateInMemoryFontFileReference(
                            &self.components.factory,
                            data.as_ptr() as _,
                            data.len() as _,
                            None,
                        )?;
                    self.components.builder.AddFontFile(&font_file)?;
                },
                Cow::Owned(data) => unsafe {
                    let font_file = self
                        .components
                        .in_memory_loader
                        .CreateInMemoryFontFileReference(
                            &self.components.factory,
                            data.as_ptr() as _,
                            data.len() as _,
                            None,
                        )?;
                    self.components.builder.AddFontFile(&font_file)?;
                },
            }
        }
        let set = unsafe { self.components.builder.CreateFontSet()? };
        let collection = unsafe {
            self.components
                .factory
                .CreateFontCollectionFromFontSet(&set)?
        };
        self.custom_font_collection = collection;

        Ok(())
    }

    fn generate_font_fallbacks(
        &self,
        fallbacks: &FontFallbacks,
    ) -> Result<Option<IDWriteFontFallback>> {
        if fallbacks.fallback_list().is_empty() {
            return Ok(None);
        }
        unsafe {
            let builder = self.components.factory.CreateFontFallbackBuilder()?;
            let font_set = &self.system_font_collection.GetFontSet()?;
            for family_name in fallbacks.fallback_list() {
                let Some(fonts) = font_set
                    .GetMatchingFonts(
                        &HSTRING::from(family_name),
                        DWRITE_FONT_WEIGHT_NORMAL,
                        DWRITE_FONT_STRETCH_NORMAL,
                        DWRITE_FONT_STYLE_NORMAL,
                    )
                    .log_err()
                else {
                    continue;
                };
                if fonts.GetFontCount() == 0 {
                    log::error!("No matching font found for {}", family_name);
                    continue;
                }
                let font = fonts.GetFontFaceReference(0)?.CreateFontFace()?;
                let mut count = 0;
                font.GetUnicodeRanges(None, &mut count).ok();
                if count == 0 {
                    continue;
                }
                let mut unicode_ranges = vec![DWRITE_UNICODE_RANGE::default(); count as usize];
                let Some(_) = font
                    .GetUnicodeRanges(Some(&mut unicode_ranges), &mut count)
                    .log_err()
                else {
                    continue;
                };
                let target_family_name = HSTRING::from(family_name);
                builder.AddMapping(
                    &unicode_ranges,
                    &[target_family_name.as_ptr()],
                    None,
                    None,
                    None,
                    1.0,
                )?;
            }
            let system_fallbacks = self.components.factory.GetSystemFontFallback()?;
            builder.AddMappings(&system_fallbacks)?;
            Ok(Some(builder.CreateFontFallback()?))
        }
    }

    unsafe fn generate_font_features(
        &self,
        font_features: &FontFeatures,
    ) -> Result<IDWriteTypography> {
        let direct_write_features = unsafe { self.components.factory.CreateTypography()? };
        apply_font_features(&direct_write_features, font_features)?;
        Ok(direct_write_features)
    }

    unsafe fn get_font_id_from_font_collection(
        &mut self,
        family_name: &str,
        font_weight: FontWeight,
        font_style: FontStyle,
        font_features: &FontFeatures,
        font_fallbacks: Option<&FontFallbacks>,
        is_system_font: bool,
    ) -> Option<FontId> {
        let collection = if is_system_font {
            &self.system_font_collection
        } else {
            &self.custom_font_collection
        };
        let fontset = unsafe { collection.GetFontSet().log_err()? };
        let font = unsafe {
            fontset
                .GetMatchingFonts(
                    &HSTRING::from(family_name),
                    font_weight.into(),
                    DWRITE_FONT_STRETCH_NORMAL,
                    font_style.into(),
                )
                .log_err()?
        };
        let total_number = unsafe { font.GetFontCount() };
        for index in 0..total_number {
            let Some(font_face_ref) = (unsafe { font.GetFontFaceReference(index).log_err() })
            else {
                continue;
            };
            let Some(font_face) = (unsafe { font_face_ref.CreateFontFace().log_err() }) else {
                continue;
            };
            let Some(identifier) = get_font_identifier(&font_face, &self.components.locale) else {
                continue;
            };
            let Some(direct_write_features) =
                (unsafe { self.generate_font_features(font_features).log_err() })
            else {
                continue;
            };
            let fallbacks = font_fallbacks
                .and_then(|fallbacks| self.generate_font_fallbacks(fallbacks).log_err().flatten());
            let font_info = FontInfo {
                font_family: family_name.to_owned(),
                font_face,
                features: direct_write_features,
                fallbacks,
                is_system_font,
            };
            let font_id = FontId(self.fonts.len());
            self.fonts.push(font_info);
            self.font_id_by_identifier.insert(identifier, font_id);
            return Some(font_id);
        }
        None
    }

    unsafe fn update_system_font_collection(&mut self) {
        let mut collection = unsafe { std::mem::zeroed() };
        if unsafe {
            self.components
                .factory
                .GetSystemFontCollection(false, &mut collection, true)
                .log_err()
                .is_some()
        } {
            self.system_font_collection = collection.unwrap();
        }
    }

    fn select_font(&mut self, target_font: &Font) -> FontId {
        unsafe {
            if target_font.family == ".SystemUIFont" {
                let family = self.system_ui_font_name.clone();
                self.find_font_id(
                    family.as_ref(),
                    target_font.weight,
                    target_font.style,
                    &target_font.features,
                    target_font.fallbacks.as_ref(),
                )
                .unwrap()
            } else {
                self.find_font_id(
                    target_font.family.as_ref(),
                    target_font.weight,
                    target_font.style,
                    &target_font.features,
                    target_font.fallbacks.as_ref(),
                )
                .unwrap_or_else(|| {
                    #[cfg(any(test, feature = "test-support"))]
                    {
                        panic!("ERROR: {} font not found!", target_font.family);
                    }
                    #[cfg(not(any(test, feature = "test-support")))]
                    {
                        let family = self.system_ui_font_name.clone();
                        log::error!("{} not found, use {} instead.", target_font.family, family);
                        self.get_font_id_from_font_collection(
                            family.as_ref(),
                            target_font.weight,
                            target_font.style,
                            &target_font.features,
                            target_font.fallbacks.as_ref(),
                            true,
                        )
                        .unwrap()
                    }
                })
            }
        }
    }

    unsafe fn find_font_id(
        &mut self,
        family_name: &str,
        weight: FontWeight,
        style: FontStyle,
        features: &FontFeatures,
        fallbacks: Option<&FontFallbacks>,
    ) -> Option<FontId> {
        // try to find target font in custom font collection first
        unsafe {
            self.get_font_id_from_font_collection(
                family_name,
                weight,
                style,
                features,
                fallbacks,
                false,
            )
            .or_else(|| {
                self.get_font_id_from_font_collection(
                    family_name,
                    weight,
                    style,
                    features,
                    fallbacks,
                    true,
                )
            })
            .or_else(|| {
                self.update_system_font_collection();
                self.get_font_id_from_font_collection(
                    family_name,
                    weight,
                    style,
                    features,
                    fallbacks,
                    true,
                )
            })
        }
    }

    fn layout_line(
        &mut self,
        text: &str,
        font_size: Pixels,
        font_runs: &[FontRun],
    ) -> Result<LineLayout> {
        if font_runs.is_empty() {
            return Ok(LineLayout {
                font_size,
                ..Default::default()
            });
        }
        unsafe {
            let text_renderer = self.components.text_renderer.clone();
            let text_wide = text.encode_utf16().collect_vec();

            let mut utf8_offset = 0usize;
            let mut utf16_offset = 0u32;
            let text_layout = {
                let first_run = &font_runs[0];
                let font_info = &self.fonts[first_run.font_id.0];
                let collection = if font_info.is_system_font {
                    &self.system_font_collection
                } else {
                    &self.custom_font_collection
                };
                let format: IDWriteTextFormat1 = self
                    .components
                    .factory
                    .CreateTextFormat(
                        &HSTRING::from(&font_info.font_family),
                        collection,
                        font_info.font_face.GetWeight(),
                        font_info.font_face.GetStyle(),
                        DWRITE_FONT_STRETCH_NORMAL,
                        font_size.0,
                        &HSTRING::from(&self.components.locale),
                    )?
                    .cast()?;
                if let Some(ref fallbacks) = font_info.fallbacks {
                    format.SetFontFallback(fallbacks)?;
                }

                let layout = self.components.factory.CreateTextLayout(
                    &text_wide,
                    &format,
                    f32::INFINITY,
                    f32::INFINITY,
                )?;
                let current_text = &text[utf8_offset..(utf8_offset + first_run.len)];
                utf8_offset += first_run.len;
                let current_text_utf16_length = current_text.encode_utf16().count() as u32;
                let text_range = DWRITE_TEXT_RANGE {
                    startPosition: utf16_offset,
                    length: current_text_utf16_length,
                };
                layout.SetTypography(&font_info.features, text_range)?;
                utf16_offset += current_text_utf16_length;

                layout
            };

            let mut first_run = true;
            let mut ascent = Pixels::default();
            let mut descent = Pixels::default();
            for run in font_runs {
                if first_run {
                    first_run = false;
                    let mut metrics = vec![DWRITE_LINE_METRICS::default(); 4];
                    let mut line_count = 0u32;
                    text_layout.GetLineMetrics(Some(&mut metrics), &mut line_count as _)?;
                    ascent = px(metrics[0].baseline);
                    descent = px(metrics[0].height - metrics[0].baseline);
                    continue;
                }
                let font_info = &self.fonts[run.font_id.0];
                let current_text = &text[utf8_offset..(utf8_offset + run.len)];
                utf8_offset += run.len;
                let current_text_utf16_length = current_text.encode_utf16().count() as u32;

                let collection = if font_info.is_system_font {
                    &self.system_font_collection
                } else {
                    &self.custom_font_collection
                };
                let text_range = DWRITE_TEXT_RANGE {
                    startPosition: utf16_offset,
                    length: current_text_utf16_length,
                };
                utf16_offset += current_text_utf16_length;
                text_layout.SetFontCollection(collection, text_range)?;
                text_layout
                    .SetFontFamilyName(&HSTRING::from(&font_info.font_family), text_range)?;
                text_layout.SetFontSize(font_size.0, text_range)?;
                text_layout.SetFontStyle(font_info.font_face.GetStyle(), text_range)?;
                text_layout.SetFontWeight(font_info.font_face.GetWeight(), text_range)?;
                text_layout.SetTypography(&font_info.features, text_range)?;
            }

            let mut runs = Vec::new();
            let renderer_context = RendererContext {
                text_system: self,
                index_converter: StringIndexConverter::new(text),
                runs: &mut runs,
                width: 0.0,
            };
            text_layout.Draw(
                Some(&renderer_context as *const _ as _),
                &text_renderer.0,
                0.0,
                0.0,
            )?;
            let width = px(renderer_context.width);

            Ok(LineLayout {
                font_size,
                width,
                ascent,
                descent,
                runs,
                len: text.len(),
            })
        }
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        unsafe {
            let font_info = &self.fonts[font_id.0];
            let mut metrics = std::mem::zeroed();
            font_info.font_face.GetMetrics(&mut metrics);

            FontMetrics {
                units_per_em: metrics.Base.designUnitsPerEm as _,
                ascent: metrics.Base.ascent as _,
                descent: -(metrics.Base.descent as f32),
                line_gap: metrics.Base.lineGap as _,
                underline_position: metrics.Base.underlinePosition as _,
                underline_thickness: metrics.Base.underlineThickness as _,
                cap_height: metrics.Base.capHeight as _,
                x_height: metrics.Base.xHeight as _,
                bounding_box: Bounds {
                    origin: Point {
                        x: metrics.glyphBoxLeft as _,
                        y: metrics.glyphBoxBottom as _,
                    },
                    size: Size {
                        width: (metrics.glyphBoxRight - metrics.glyphBoxLeft) as _,
                        height: (metrics.glyphBoxTop - metrics.glyphBoxBottom) as _,
                    },
                },
            }
        }
    }

    fn raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        let font = &self.fonts[params.font_id.0];
        let glyph_id = [params.glyph_id.0 as u16];
        let advance = [0.0f32];
        let offset = [DWRITE_GLYPH_OFFSET::default()];
        let glyph_run = DWRITE_GLYPH_RUN {
            fontFace: unsafe { std::mem::transmute_copy(&font.font_face) },
            fontEmSize: params.font_size.0,
            glyphCount: 1,
            glyphIndices: glyph_id.as_ptr(),
            glyphAdvances: advance.as_ptr(),
            glyphOffsets: offset.as_ptr(),
            isSideways: BOOL(0),
            bidiLevel: 0,
        };

        let rendering_mode = DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC;
        let measuring_mode = DWRITE_MEASURING_MODE_NATURAL;
        let baseline_origin_x = 0.0;
        let baseline_origin_y = 0.0;

        let glyph_analysis = unsafe {
            self.components.factory.CreateGlyphRunAnalysis(
                &glyph_run,
                None,
                rendering_mode,
                measuring_mode,
                DWRITE_GRID_FIT_MODE_DEFAULT,
                DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE,
                baseline_origin_x,
                baseline_origin_y,
            )?
        };

        let texture_type = DWRITE_TEXTURE_CLEARTYPE_3x1;
        let bounds = unsafe { glyph_analysis.GetAlphaTextureBounds(texture_type)? };

        // todo(windows)
        // This is a walkaround, deleted when figured out.
        let y_offset;
        let extra_height;
        if params.is_emoji {
            y_offset = 0;
            extra_height = 0;
        } else {
            // make some room for scaler.
            y_offset = -1;
            extra_height = 2;
        }

        if bounds.right < bounds.left {
            Ok(Bounds {
                origin: point(0.into(), 0.into()),
                size: size(0.into(), 0.into()),
            })
        } else {
            Ok(Bounds {
                origin: point(
                    ((bounds.left as f32 * params.scale_factor).ceil() as i32).into(),
                    ((bounds.top as f32 * params.scale_factor).ceil() as i32 + y_offset).into(),
                ),
                size: size(
                    (((bounds.right - bounds.left) as f32 * params.scale_factor).ceil() as i32)
                        .into(),
                    (((bounds.bottom - bounds.top) as f32 * params.scale_factor).ceil() as i32
                        + extra_height)
                        .into(),
                ),
            })
        }
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        let font_info = &self.fonts[font_id.0];
        let codepoints = [ch as u32];
        let mut glyph_indices = vec![0u16; 1];
        unsafe {
            font_info
                .font_face
                .GetGlyphIndices(codepoints.as_ptr(), 1, glyph_indices.as_mut_ptr())
                .log_err()
        }
        .map(|_| GlyphId(glyph_indices[0] as u32))
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        glyph_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        if glyph_bounds.size.width.0 == 0 || glyph_bounds.size.height.0 == 0 {
            anyhow::bail!("glyph bounds are empty");
        }

        let font_info = &self.fonts[params.font_id.0];
        let glyph_id = [params.glyph_id.0 as u16];
        let advance = [glyph_bounds.size.width.0 as f32];
        let offset = [DWRITE_GLYPH_OFFSET {
            advanceOffset: -glyph_bounds.origin.x.0 as f32 / params.scale_factor,
            ascenderOffset: glyph_bounds.origin.y.0 as f32 / params.scale_factor,
        }];
        let glyph_run = DWRITE_GLYPH_RUN {
            fontFace: ManuallyDrop::new(Some(font_info.font_face.cast()?)),
            fontEmSize: params.font_size.0,
            glyphCount: 1,
            glyphIndices: glyph_id.as_ptr(),
            glyphAdvances: advance.as_ptr(),
            glyphOffsets: offset.as_ptr(),
            isSideways: BOOL(0),
            bidiLevel: 0,
        };

        // Add an extra pixel when the subpixel variant isn't zero to make room for anti-aliasing.
        let mut bitmap_size = glyph_bounds.size;
        if params.subpixel_variant.x > 0 {
            bitmap_size.width += DevicePixels(1);
        }
        if params.subpixel_variant.y > 0 {
            bitmap_size.height += DevicePixels(1);
        }
        let bitmap_size = bitmap_size;

        let subpixel_shift = params
            .subpixel_variant
            .map(|v| v as f32 / SUBPIXEL_VARIANTS as f32);
        let baseline_origin_x = subpixel_shift.x / params.scale_factor;
        let baseline_origin_y = subpixel_shift.y / params.scale_factor;

        let transform = DWRITE_MATRIX {
            m11: params.scale_factor,
            m12: 0.0,
            m21: 0.0,
            m22: params.scale_factor,
            dx: 0.0,
            dy: 0.0,
        };

        let rendering_mode = if params.is_emoji {
            DWRITE_RENDERING_MODE1_NATURAL
        } else {
            DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC
        };

        let measuring_mode = DWRITE_MEASURING_MODE_NATURAL;

        let glyph_analysis = unsafe {
            self.components.factory.CreateGlyphRunAnalysis(
                &glyph_run,
                Some(&transform),
                rendering_mode,
                measuring_mode,
                DWRITE_GRID_FIT_MODE_DEFAULT,
                DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE,
                baseline_origin_x,
                baseline_origin_y,
            )?
        };

        let texture_type = DWRITE_TEXTURE_CLEARTYPE_3x1;
        let texture_bounds = unsafe { glyph_analysis.GetAlphaTextureBounds(texture_type)? };
        let texture_width = (texture_bounds.right - texture_bounds.left) as u32;
        let texture_height = (texture_bounds.bottom - texture_bounds.top) as u32;

        if texture_width == 0 || texture_height == 0 {
            return Ok((
                bitmap_size,
                vec![
                    0u8;
                    bitmap_size.width.0 as usize
                        * bitmap_size.height.0 as usize
                        * if params.is_emoji { 4 } else { 1 }
                ],
            ));
        }

        let mut bitmap_data;
        if params.is_emoji {
            // todo: support more glyph image formats for more exotic fonts, for now it should fallback to monochrome rendering
            let color_enumerator = unsafe {
                self.components.factory.TranslateColorGlyphRun(
                    Vector2::new(baseline_origin_x, baseline_origin_y),
                    &glyph_run,
                    None,
                    DWRITE_GLYPH_IMAGE_FORMATS_COLR
                        | DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8,
                    measuring_mode,
                    Some(&transform),
                    0,
                )
            };

            // if let Ok(color_enumerator) = color_enumerator {
            //     loop {
            //         let color_run = unsafe { color_enumerator.GetCurrentRun() };
            //         if let Ok(color_run) = color_run {
            //             let color_glyph_run = unsafe { &*color_run };
            //             let color_value = color_glyph_run.Base.runColor;

            //             // Create analysis for this color layer
            //             let color_analysis = unsafe {
            //                 self.components.factory.CreateGlyphRunAnalysis(
            //                     &color_glyph_run.Base.glyphRun as *const _,
            //                     Some(&transform),
            //                     rendering_mode,
            //                     measuring_mode,
            //                     DWRITE_GRID_FIT_MODE_DEFAULT,
            //                     DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE,
            //                     baseline_origin_x,
            //                     baseline_origin_y,
            //                 )
            //             };

            //             // todo: move this block completely to the gpu
            //             // this is important because fonts can bundle quite large icons
            //             // and compositing them on the cpu is quite expensive
            //             // also the code is ugly
            //             if let Ok(color_analysis) = color_analysis {
            //                 let color_bounds =
            //                     unsafe { color_analysis.GetAlphaTextureBounds(texture_type) };
            //                 if let Ok(color_bounds) = color_bounds {
            //                     let color_width = (color_bounds.right - color_bounds.left) as u32;
            //                     let color_height = (color_bounds.bottom - color_bounds.top) as u32;

            //                     if color_width > 0 && color_height > 0 {
            //                         let mut alpha_data =
            //                             vec![0u8; (color_width * color_height * 3) as usize];
            //                         if unsafe {
            //                             color_analysis.CreateAlphaTexture(
            //                                 texture_type,
            //                                 &color_bounds,
            //                                 &mut alpha_data,
            //                             )
            //                         }
            //                         .is_ok()
            //                         {
            //                             let r = (color_value.r * 255.0) as u8;
            //                             let g = (color_value.g * 255.0) as u8;
            //                             let b = (color_value.b * 255.0) as u8;
            //                             let a = (color_value.a * 255.0) as u8;

            //                             let offset_x = color_bounds.left.max(0) as usize;
            //                             let offset_y = color_bounds.top.max(0) as usize;

            //                             for y in 0..color_height as usize {
            //                                 for x in 0..color_width as usize {
            //                                     let bitmap_x = offset_x + x;
            //                                     let bitmap_y = offset_y + y;

            //                                     if bitmap_x < bitmap_size.width.0 as usize
            //                                         && bitmap_y < bitmap_size.height.0 as usize
            //                                     {
            //                                         let alpha_idx =
            //                                             (y * color_width as usize + x) * 3;
            //                                         let bitmap_idx = (bitmap_y
            //                                             * bitmap_size.width.0 as usize
            //                                             + bitmap_x)
            //                                             * 4;

            //                                         if alpha_idx + 2 < alpha_data.len()
            //                                             && bitmap_idx + 3 < bitmap_data.len()
            //                                         {
            //                                             let alpha_value = (alpha_data[alpha_idx]
            //                                                 as u32
            //                                                 + alpha_data[alpha_idx + 1] as u32
            //                                                 + alpha_data[alpha_idx + 2] as u32)
            //                                                 / 3;
            //                                             let final_alpha =
            //                                                 ((alpha_value * a as u32) / 255) as u8;

            //                                             if final_alpha > 0 {
            //                                                 let existing_r =
            //                                                     bitmap_data[bitmap_idx];
            //                                                 let existing_g =
            //                                                     bitmap_data[bitmap_idx + 1];
            //                                                 let existing_b =
            //                                                     bitmap_data[bitmap_idx + 2];
            //                                                 let existing_a =
            //                                                     bitmap_data[bitmap_idx + 3];

            //                                                 let src_alpha =
            //                                                     final_alpha as f32 / 255.0;
            //                                                 let dst_alpha =
            //                                                     existing_a as f32 / 255.0;
            //                                                 let out_alpha = src_alpha
            //                                                     + dst_alpha * (1.0 - src_alpha);

            //                                                 if out_alpha > 0.0 {
            //                                                     bitmap_data[bitmap_idx] =
            //                                                         ((r as f32 * src_alpha
            //                                                             + existing_r as f32
            //                                                                 * dst_alpha
            //                                                                 * (1.0 - src_alpha))
            //                                                             / out_alpha)
            //                                                             as u8;
            //                                                     bitmap_data[bitmap_idx + 1] =
            //                                                         ((g as f32 * src_alpha
            //                                                             + existing_g as f32
            //                                                                 * dst_alpha
            //                                                                 * (1.0 - src_alpha))
            //                                                             / out_alpha)
            //                                                             as u8;
            //                                                     bitmap_data[bitmap_idx + 2] =
            //                                                         ((b as f32 * src_alpha
            //                                                             + existing_b as f32
            //                                                                 * dst_alpha
            //                                                                 * (1.0 - src_alpha))
            //                                                             / out_alpha)
            //                                                             as u8;
            //                                                     bitmap_data[bitmap_idx + 3] =
            //                                                         (out_alpha * 255.0) as u8;
            //                                                 }
            //                                             }
            //                                         }
            //                                     }
            //                                 }
            //                             }
            //                         }
            //                     }
            //                 }
            //             }
            //         }

            //         if !unsafe { color_enumerator.MoveNext() }?.as_bool() {
            //             break;
            //         }
            //     }
            // } else {
            // }
            let monochrome_data = Self::rasterize_monochrome(
                &glyph_analysis,
                bitmap_size,
                size(texture_width, texture_height),
                &texture_bounds,
            )?;
            bitmap_data = monochrome_data
                .into_iter()
                .flat_map(|e| [e, e, e, 255])
                .collect::<Vec<u8>>();
        } else {
            bitmap_data = Self::rasterize_monochrome(
                &glyph_analysis,
                bitmap_size,
                size(texture_width, texture_height),
                &texture_bounds,
            )?;
        }

        Ok((bitmap_size, bitmap_data))
    }

    fn rasterize_monochrome(
        glyph_analysis: &IDWriteGlyphRunAnalysis,
        bitmap_size: Size<DevicePixels>,
        texture_size: Size<u32>,
        texture_bounds: &RECT,
    ) -> Result<Vec<u8>> {
        let mut bitmap_data =
            vec![0u8; bitmap_size.width.0 as usize * bitmap_size.height.0 as usize];

        let mut alpha_data = vec![0u8; (texture_size.width * texture_size.height * 3) as usize];
        unsafe {
            glyph_analysis.CreateAlphaTexture(
                DWRITE_TEXTURE_CLEARTYPE_3x1,
                texture_bounds,
                &mut alpha_data,
            )?;
        }

        // Convert ClearType RGB data to grayscale and place in bitmap
        let offset_x = texture_bounds.left.max(0) as usize;
        let offset_y = texture_bounds.top.max(0) as usize;

        for y in 0..texture_size.height as usize {
            for x in 0..texture_size.width as usize {
                let bitmap_x = offset_x + x;
                let bitmap_y = offset_y + y;

                if bitmap_x < bitmap_size.width.0 as usize
                    && bitmap_y < bitmap_size.height.0 as usize
                {
                    let texture_idx = (y * texture_size.width as usize + x) * 3;
                    let bitmap_idx = bitmap_y * bitmap_size.width.0 as usize + bitmap_x;

                    if texture_idx + 2 < alpha_data.len() && bitmap_idx < bitmap_data.len() {
                        let avg = (alpha_data[texture_idx] as u32
                            + alpha_data[texture_idx + 1] as u32
                            + alpha_data[texture_idx + 2] as u32)
                            / 3;
                        bitmap_data[bitmap_idx] = avg as u8;
                    }
                }
            }
        }

        Ok(bitmap_data)
    }

    fn get_typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        unsafe {
            let font = &self.fonts[font_id.0].font_face;
            let glyph_indices = [glyph_id.0 as u16];
            let mut metrics = [DWRITE_GLYPH_METRICS::default()];
            font.GetDesignGlyphMetrics(glyph_indices.as_ptr(), 1, metrics.as_mut_ptr(), false)?;

            let metrics = &metrics[0];
            let advance_width = metrics.advanceWidth as i32;
            let advance_height = metrics.advanceHeight as i32;
            let left_side_bearing = metrics.leftSideBearing;
            let right_side_bearing = metrics.rightSideBearing;
            let top_side_bearing = metrics.topSideBearing;
            let bottom_side_bearing = metrics.bottomSideBearing;
            let vertical_origin_y = metrics.verticalOriginY;

            let y_offset = vertical_origin_y + bottom_side_bearing - advance_height;
            let width = advance_width - (left_side_bearing + right_side_bearing);
            let height = advance_height - (top_side_bearing + bottom_side_bearing);

            Ok(Bounds {
                origin: Point {
                    x: left_side_bearing as f32,
                    y: y_offset as f32,
                },
                size: Size {
                    width: width as f32,
                    height: height as f32,
                },
            })
        }
    }

    fn get_advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        unsafe {
            let font = &self.fonts[font_id.0].font_face;
            let glyph_indices = [glyph_id.0 as u16];
            let mut metrics = [DWRITE_GLYPH_METRICS::default()];
            font.GetDesignGlyphMetrics(glyph_indices.as_ptr(), 1, metrics.as_mut_ptr(), false)?;

            let metrics = &metrics[0];

            Ok(Size {
                width: metrics.advanceWidth as f32,
                height: 0.0,
            })
        }
    }

    fn all_font_names(&self) -> Vec<String> {
        let mut result =
            get_font_names_from_collection(&self.system_font_collection, &self.components.locale);
        result.extend(get_font_names_from_collection(
            &self.custom_font_collection,
            &self.components.locale,
        ));
        result
    }
}

impl Drop for DirectWriteState {
    fn drop(&mut self) {
        unsafe {
            let _ = self
                .components
                .factory
                .UnregisterFontFileLoader(&self.components.in_memory_loader);
        }
    }
}

struct TextRendererWrapper(pub IDWriteTextRenderer);

impl TextRendererWrapper {
    pub fn new(locale_str: &str) -> Self {
        let inner = TextRenderer::new(locale_str);
        TextRendererWrapper(inner.into())
    }
}

#[implement(IDWriteTextRenderer)]
struct TextRenderer {
    locale: String,
}

impl TextRenderer {
    pub fn new(locale_str: &str) -> Self {
        TextRenderer {
            locale: locale_str.to_owned(),
        }
    }
}

struct RendererContext<'t, 'a, 'b> {
    text_system: &'t mut DirectWriteState,
    index_converter: StringIndexConverter<'a>,
    runs: &'b mut Vec<ShapedRun>,
    width: f32,
}

#[derive(Debug)]
struct ClusterAnalyzer<'t> {
    utf16_idx: usize,
    glyph_idx: usize,
    glyph_count: usize,
    cluster_map: &'t [u16],
}

impl<'t> ClusterAnalyzer<'t> {
    pub fn new(cluster_map: &'t [u16], glyph_count: usize) -> Self {
        ClusterAnalyzer {
            utf16_idx: 0,
            glyph_idx: 0,
            glyph_count,
            cluster_map,
        }
    }
}

impl Iterator for ClusterAnalyzer<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.utf16_idx >= self.cluster_map.len() {
            return None; // No more clusters
        }
        let start_utf16_idx = self.utf16_idx;
        let current_glyph = self.cluster_map[start_utf16_idx] as usize;

        // Find the end of current cluster (where glyph index changes)
        let mut end_utf16_idx = start_utf16_idx + 1;
        while end_utf16_idx < self.cluster_map.len()
            && self.cluster_map[end_utf16_idx] as usize == current_glyph
        {
            end_utf16_idx += 1;
        }

        let utf16_len = end_utf16_idx - start_utf16_idx;

        // Calculate glyph count for this cluster
        let next_glyph = if end_utf16_idx < self.cluster_map.len() {
            self.cluster_map[end_utf16_idx] as usize
        } else {
            self.glyph_count
        };

        let glyph_count = next_glyph - current_glyph;

        // Update state for next call
        self.utf16_idx = end_utf16_idx;
        self.glyph_idx = next_glyph;

        Some((utf16_len, glyph_count))
    }
}

#[allow(non_snake_case)]
impl IDWritePixelSnapping_Impl for TextRenderer_Impl {
    fn IsPixelSnappingDisabled(
        &self,
        _clientdrawingcontext: *const ::core::ffi::c_void,
    ) -> windows::core::Result<BOOL> {
        Ok(BOOL(0))
    }

    fn GetCurrentTransform(
        &self,
        _clientdrawingcontext: *const ::core::ffi::c_void,
        transform: *mut DWRITE_MATRIX,
    ) -> windows::core::Result<()> {
        unsafe {
            *transform = DWRITE_MATRIX {
                m11: 1.0,
                m12: 0.0,
                m21: 0.0,
                m22: 1.0,
                dx: 0.0,
                dy: 0.0,
            };
        }
        Ok(())
    }

    fn GetPixelsPerDip(
        &self,
        _clientdrawingcontext: *const ::core::ffi::c_void,
    ) -> windows::core::Result<f32> {
        Ok(1.0)
    }
}

#[allow(non_snake_case)]
impl IDWriteTextRenderer_Impl for TextRenderer_Impl {
    fn DrawGlyphRun(
        &self,
        clientdrawingcontext: *const ::core::ffi::c_void,
        _baselineoriginx: f32,
        _baselineoriginy: f32,
        _measuringmode: DWRITE_MEASURING_MODE,
        glyphrun: *const DWRITE_GLYPH_RUN,
        glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        _clientdrawingeffect: windows::core::Ref<windows::core::IUnknown>,
    ) -> windows::core::Result<()> {
        let glyphrun = unsafe { &*glyphrun };
        let glyph_count = glyphrun.glyphCount as usize;
        if glyph_count == 0 || glyphrun.fontFace.is_none() {
            return Ok(());
        }
        let desc = unsafe { &*glyphrundescription };
        let context = unsafe {
            &mut *(clientdrawingcontext as *const RendererContext as *mut RendererContext)
        };
        let font_face = glyphrun.fontFace.as_ref().unwrap();
        // This `cast()` action here should never fail since we are running on Win10+, and
        // `IDWriteFontFace3` requires Win10
        let font_face = &font_face.cast::<IDWriteFontFace3>().unwrap();
        let Some((font_identifier, font_struct, color_font)) =
            get_font_identifier_and_font_struct(font_face, &self.locale)
        else {
            return Ok(());
        };

        let font_id = if let Some(id) = context
            .text_system
            .font_id_by_identifier
            .get(&font_identifier)
        {
            *id
        } else {
            context.text_system.select_font(&font_struct)
        };

        let glyph_ids = unsafe { std::slice::from_raw_parts(glyphrun.glyphIndices, glyph_count) };
        let glyph_advances =
            unsafe { std::slice::from_raw_parts(glyphrun.glyphAdvances, glyph_count) };
        let glyph_offsets =
            unsafe { std::slice::from_raw_parts(glyphrun.glyphOffsets, glyph_count) };
        let cluster_map =
            unsafe { std::slice::from_raw_parts(desc.clusterMap, desc.stringLength as usize) };

        let mut cluster_analyzer = ClusterAnalyzer::new(cluster_map, glyph_count);
        let mut utf16_idx = desc.textPosition as usize;
        let mut glyph_idx = 0;
        let mut glyphs = Vec::with_capacity(glyph_count);
        for (cluster_utf16_len, cluster_glyph_count) in cluster_analyzer {
            context.index_converter.advance_to_utf16_ix(utf16_idx);
            utf16_idx += cluster_utf16_len;
            for (cluster_glyph_idx, glyph_id) in glyph_ids
                [glyph_idx..(glyph_idx + cluster_glyph_count)]
                .iter()
                .enumerate()
            {
                let id = GlyphId(*glyph_id as u32);
                let is_emoji = color_font
                    && is_color_glyph(font_face, id, &context.text_system.components.factory);
                let this_glyph_idx = glyph_idx + cluster_glyph_idx;
                glyphs.push(ShapedGlyph {
                    id,
                    position: point(
                        px(context.width + glyph_offsets[this_glyph_idx].advanceOffset),
                        px(0.0),
                    ),
                    index: context.index_converter.utf8_ix,
                    is_emoji,
                });
                context.width += glyph_advances[this_glyph_idx];
            }
            glyph_idx += cluster_glyph_count;
        }
        context.runs.push(ShapedRun { font_id, glyphs });
        Ok(())
    }

    fn DrawUnderline(
        &self,
        _clientdrawingcontext: *const ::core::ffi::c_void,
        _baselineoriginx: f32,
        _baselineoriginy: f32,
        _underline: *const DWRITE_UNDERLINE,
        _clientdrawingeffect: windows::core::Ref<windows::core::IUnknown>,
    ) -> windows::core::Result<()> {
        Err(windows::core::Error::new(
            E_NOTIMPL,
            "DrawUnderline unimplemented",
        ))
    }

    fn DrawStrikethrough(
        &self,
        _clientdrawingcontext: *const ::core::ffi::c_void,
        _baselineoriginx: f32,
        _baselineoriginy: f32,
        _strikethrough: *const DWRITE_STRIKETHROUGH,
        _clientdrawingeffect: windows::core::Ref<windows::core::IUnknown>,
    ) -> windows::core::Result<()> {
        Err(windows::core::Error::new(
            E_NOTIMPL,
            "DrawStrikethrough unimplemented",
        ))
    }

    fn DrawInlineObject(
        &self,
        _clientdrawingcontext: *const ::core::ffi::c_void,
        _originx: f32,
        _originy: f32,
        _inlineobject: windows::core::Ref<IDWriteInlineObject>,
        _issideways: BOOL,
        _isrighttoleft: BOOL,
        _clientdrawingeffect: windows::core::Ref<windows::core::IUnknown>,
    ) -> windows::core::Result<()> {
        Err(windows::core::Error::new(
            E_NOTIMPL,
            "DrawInlineObject unimplemented",
        ))
    }
}

struct StringIndexConverter<'a> {
    text: &'a str,
    utf8_ix: usize,
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

    #[allow(dead_code)]
    fn advance_to_utf8_ix(&mut self, utf8_target: usize) {
        for (ix, c) in self.text[self.utf8_ix..].char_indices() {
            if self.utf8_ix + ix >= utf8_target {
                self.utf8_ix += ix;
                return;
            }
            self.utf16_ix += c.len_utf16();
        }
        self.utf8_ix = self.text.len();
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

impl Into<DWRITE_FONT_STYLE> for FontStyle {
    fn into(self) -> DWRITE_FONT_STYLE {
        match self {
            FontStyle::Normal => DWRITE_FONT_STYLE_NORMAL,
            FontStyle::Italic => DWRITE_FONT_STYLE_ITALIC,
            FontStyle::Oblique => DWRITE_FONT_STYLE_OBLIQUE,
        }
    }
}

impl From<DWRITE_FONT_STYLE> for FontStyle {
    fn from(value: DWRITE_FONT_STYLE) -> Self {
        match value.0 {
            0 => FontStyle::Normal,
            1 => FontStyle::Italic,
            2 => FontStyle::Oblique,
            _ => unreachable!(),
        }
    }
}

impl Into<DWRITE_FONT_WEIGHT> for FontWeight {
    fn into(self) -> DWRITE_FONT_WEIGHT {
        DWRITE_FONT_WEIGHT(self.0 as i32)
    }
}

impl From<DWRITE_FONT_WEIGHT> for FontWeight {
    fn from(value: DWRITE_FONT_WEIGHT) -> Self {
        FontWeight(value.0 as f32)
    }
}

fn get_font_names_from_collection(
    collection: &IDWriteFontCollection1,
    locale: &str,
) -> Vec<String> {
    unsafe {
        let mut result = Vec::new();
        let family_count = collection.GetFontFamilyCount();
        for index in 0..family_count {
            let Some(font_family) = collection.GetFontFamily(index).log_err() else {
                continue;
            };
            let Some(localized_family_name) = font_family.GetFamilyNames().log_err() else {
                continue;
            };
            let Some(family_name) = get_name(localized_family_name, locale).log_err() else {
                continue;
            };
            result.push(family_name);
        }

        result
    }
}

fn get_font_identifier_and_font_struct(
    font_face: &IDWriteFontFace3,
    locale: &str,
) -> Option<(FontIdentifier, Font, bool)> {
    let postscript_name = get_postscript_name(font_face, locale).log_err()?;
    let localized_family_name = unsafe { font_face.GetFamilyNames().log_err() }?;
    let family_name = get_name(localized_family_name, locale).log_err()?;
    let weight = unsafe { font_face.GetWeight() };
    let style = unsafe { font_face.GetStyle() };
    let identifier = FontIdentifier {
        postscript_name,
        weight: weight.0,
        style: style.0,
    };
    let font_struct = Font {
        family: family_name.into(),
        features: FontFeatures::default(),
        weight: weight.into(),
        style: style.into(),
        fallbacks: None,
    };
    let is_emoji = unsafe { font_face.IsColorFont().as_bool() };
    Some((identifier, font_struct, is_emoji))
}

#[inline]
fn get_font_identifier(font_face: &IDWriteFontFace3, locale: &str) -> Option<FontIdentifier> {
    let weight = unsafe { font_face.GetWeight().0 };
    let style = unsafe { font_face.GetStyle().0 };
    get_postscript_name(font_face, locale)
        .log_err()
        .map(|postscript_name| FontIdentifier {
            postscript_name,
            weight,
            style,
        })
}

#[inline]
fn get_postscript_name(font_face: &IDWriteFontFace3, locale: &str) -> Result<String> {
    let mut info = None;
    let mut exists = BOOL(0);
    unsafe {
        font_face.GetInformationalStrings(
            DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME,
            &mut info,
            &mut exists,
        )?
    };
    if !exists.as_bool() || info.is_none() {
        anyhow::bail!("No postscript name found for font face");
    }

    get_name(info.unwrap(), locale)
}

// https://learn.microsoft.com/en-us/windows/win32/api/dwrite/ne-dwrite-dwrite_font_feature_tag
fn apply_font_features(
    direct_write_features: &IDWriteTypography,
    features: &FontFeatures,
) -> Result<()> {
    let tag_values = features.tag_value_list();
    if tag_values.is_empty() {
        return Ok(());
    }

    // All of these features are enabled by default by DirectWrite.
    // If you want to (and can) peek into the source of DirectWrite
    let mut feature_liga = make_direct_write_feature("liga", 1);
    let mut feature_clig = make_direct_write_feature("clig", 1);
    let mut feature_calt = make_direct_write_feature("calt", 1);

    for (tag, value) in tag_values {
        if tag.as_str() == "liga" && *value == 0 {
            feature_liga.parameter = 0;
            continue;
        }
        if tag.as_str() == "clig" && *value == 0 {
            feature_clig.parameter = 0;
            continue;
        }
        if tag.as_str() == "calt" && *value == 0 {
            feature_calt.parameter = 0;
            continue;
        }

        unsafe {
            direct_write_features.AddFontFeature(make_direct_write_feature(&tag, *value))?;
        }
    }
    unsafe {
        direct_write_features.AddFontFeature(feature_liga)?;
        direct_write_features.AddFontFeature(feature_clig)?;
        direct_write_features.AddFontFeature(feature_calt)?;
    }

    Ok(())
}

#[inline]
const fn make_direct_write_feature(feature_name: &str, parameter: u32) -> DWRITE_FONT_FEATURE {
    let tag = make_direct_write_tag(feature_name);
    DWRITE_FONT_FEATURE {
        nameTag: tag,
        parameter,
    }
}

#[inline]
const fn make_open_type_tag(tag_name: &str) -> u32 {
    let bytes = tag_name.as_bytes();
    debug_assert!(bytes.len() == 4);
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

#[inline]
const fn make_direct_write_tag(tag_name: &str) -> DWRITE_FONT_FEATURE_TAG {
    DWRITE_FONT_FEATURE_TAG(make_open_type_tag(tag_name))
}

#[inline]
fn get_name(string: IDWriteLocalizedStrings, locale: &str) -> Result<String> {
    let mut locale_name_index = 0u32;
    let mut exists = BOOL(0);
    unsafe {
        string.FindLocaleName(
            &HSTRING::from(locale),
            &mut locale_name_index,
            &mut exists as _,
        )?
    };
    if !exists.as_bool() {
        unsafe {
            string.FindLocaleName(
                DEFAULT_LOCALE_NAME,
                &mut locale_name_index as _,
                &mut exists as _,
            )?
        };
        anyhow::ensure!(exists.as_bool(), "No localised string for {locale}");
    }

    let name_length = unsafe { string.GetStringLength(locale_name_index) }? as usize;
    let mut name_vec = vec![0u16; name_length + 1];
    unsafe {
        string.GetString(locale_name_index, &mut name_vec)?;
    }

    Ok(String::from_utf16_lossy(&name_vec[..name_length]))
}

#[inline]
fn translate_color(color: &DWRITE_COLOR_F) -> [f32; 4] {
    [color.r, color.g, color.b, color.a]
}

fn get_system_ui_font_name() -> SharedString {
    unsafe {
        let mut info: LOGFONTW = std::mem::zeroed();
        let font_family = if SystemParametersInfoW(
            SPI_GETICONTITLELOGFONT,
            std::mem::size_of::<LOGFONTW>() as u32,
            Some(&mut info as *mut _ as _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
        .log_err()
        .is_none()
        {
            // https://learn.microsoft.com/en-us/windows/win32/uxguide/vis-fonts
            // Segoe UI is the Windows font intended for user interface text strings.
            "Segoe UI".into()
        } else {
            let font_name = String::from_utf16_lossy(&info.lfFaceName);
            font_name.trim_matches(char::from(0)).to_owned().into()
        };
        log::info!("Use {} as UI font.", font_family);
        font_family
    }
}

// One would think that with newer DirectWrite method: IDWriteFontFace4::GetGlyphImageFormats
// but that doesn't seem to work for some glyphs, say ❤
fn is_color_glyph(
    font_face: &IDWriteFontFace3,
    glyph_id: GlyphId,
    factory: &IDWriteFactory5,
) -> bool {
    let glyph_run = DWRITE_GLYPH_RUN {
        fontFace: unsafe { std::mem::transmute_copy(font_face) },
        fontEmSize: 14.0,
        glyphCount: 1,
        glyphIndices: &(glyph_id.0 as u16),
        glyphAdvances: &0.0,
        glyphOffsets: &DWRITE_GLYPH_OFFSET {
            advanceOffset: 0.0,
            ascenderOffset: 0.0,
        },
        isSideways: BOOL(0),
        bidiLevel: 0,
    };
    unsafe {
        factory.TranslateColorGlyphRun(
            Vector2::default(),
            &glyph_run as _,
            None,
            DWRITE_GLYPH_IMAGE_FORMATS_COLR
                | DWRITE_GLYPH_IMAGE_FORMATS_SVG
                | DWRITE_GLYPH_IMAGE_FORMATS_PNG
                | DWRITE_GLYPH_IMAGE_FORMATS_JPEG
                | DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8,
            DWRITE_MEASURING_MODE_NATURAL,
            None,
            0,
        )
    }
    .is_ok()
}

const DEFAULT_LOCALE_NAME: PCWSTR = windows::core::w!("en-US");

#[cfg(test)]
mod tests {
    use crate::platform::windows::direct_write::ClusterAnalyzer;

    #[test]
    fn test_cluster_map() {
        let cluster_map = [0];
        let mut analyzer = ClusterAnalyzer::new(&cluster_map, 1);
        let next = analyzer.next();
        assert_eq!(next, Some((1, 1)));
        let next = analyzer.next();
        assert_eq!(next, None);

        let cluster_map = [0, 1, 2];
        let mut analyzer = ClusterAnalyzer::new(&cluster_map, 3);
        let next = analyzer.next();
        assert_eq!(next, Some((1, 1)));
        let next = analyzer.next();
        assert_eq!(next, Some((1, 1)));
        let next = analyzer.next();
        assert_eq!(next, Some((1, 1)));
        let next = analyzer.next();
        assert_eq!(next, None);
        // 👨‍👩‍👧‍👦👩‍💻
        let cluster_map = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4];
        let mut analyzer = ClusterAnalyzer::new(&cluster_map, 5);
        let next = analyzer.next();
        assert_eq!(next, Some((11, 4)));
        let next = analyzer.next();
        assert_eq!(next, Some((5, 1)));
        let next = analyzer.next();
        assert_eq!(next, None);
        // 👩‍💻
        let cluster_map = [0, 0, 0, 0, 0];
        let mut analyzer = ClusterAnalyzer::new(&cluster_map, 1);
        let next = analyzer.next();
        assert_eq!(next, Some((5, 1)));
        let next = analyzer.next();
        assert_eq!(next, None);
    }
}
