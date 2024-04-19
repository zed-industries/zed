use std::{borrow::Cow, sync::Arc};

use ::util::ResultExt;
use anyhow::{anyhow, Result};
use collections::HashMap;
use itertools::Itertools;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use windows::{
    core::*,
    Foundation::Numerics::Matrix3x2,
    Win32::{
        Foundation::*,
        Globalization::GetUserDefaultLocaleName,
        Graphics::{
            Direct2D::{Common::*, *},
            DirectWrite::*,
            Dxgi::Common::*,
            Imaging::{D2D::IWICImagingFactory2, *},
        },
        System::{Com::*, SystemServices::LOCALE_NAME_MAX_LENGTH},
    },
};

use crate::*;

#[derive(Debug)]
struct FontInfo {
    font_family: String,
    font_face: IDWriteFontFace3,
    features: IDWriteTypography,
    is_system_font: bool,
    is_emoji: bool,
}

pub(crate) struct DirectWriteTextSystem(RwLock<DirectWriteState>);

struct DirectWriteComponent {
    locale: String,
    factory: IDWriteFactory5,
    bitmap_factory: IWICImagingFactory2,
    d2d1_factory: ID2D1Factory,
    in_memory_loader: IDWriteInMemoryFontFileLoader,
    builder: IDWriteFontSetBuilder1,
    text_renderer: Arc<TextRendererWrapper>,
}

// All use of the IUnknown methods should be "thread-safe".
unsafe impl Sync for DirectWriteComponent {}
unsafe impl Send for DirectWriteComponent {}

struct DirectWriteState {
    components: DirectWriteComponent,
    system_font_collection: IDWriteFontCollection1,
    custom_font_collection: IDWriteFontCollection1,
    fonts: Vec<FontInfo>,
    font_selections: HashMap<Font, FontId>,
    font_id_by_postscript_name: HashMap<String, FontId>,
}

impl DirectWriteComponent {
    pub fn new() -> Result<Self> {
        unsafe {
            let factory: IDWriteFactory5 = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;
            let bitmap_factory: IWICImagingFactory2 =
                CoCreateInstance(&CLSID_WICImagingFactory2, None, CLSCTX_INPROC_SERVER)?;
            let d2d1_factory: ID2D1Factory =
                D2D1CreateFactory(D2D1_FACTORY_TYPE_MULTI_THREADED, None)?;
            // The `IDWriteInMemoryFontFileLoader` here is supported starting from
            // Windows 10 Creators Update, which consequently requires the entire
            // `DirectWriteTextSystem` to run on `win10 1703`+.
            let in_memory_loader = factory.CreateInMemoryFontFileLoader()?;
            factory.RegisterFontFileLoader(&in_memory_loader)?;
            let builder = factory.CreateFontSetBuilder2()?;
            let mut locale_vec = vec![0u16; LOCALE_NAME_MAX_LENGTH as usize];
            GetUserDefaultLocaleName(&mut locale_vec);
            let locale = String::from_utf16_lossy(&locale_vec);
            let text_renderer = Arc::new(TextRendererWrapper::new(&locale));

            Ok(DirectWriteComponent {
                locale,
                factory,
                bitmap_factory,
                d2d1_factory,
                in_memory_loader,
                builder,
                text_renderer,
            })
        }
    }
}

impl DirectWriteTextSystem {
    pub(crate) fn new() -> Result<Self> {
        let components = DirectWriteComponent::new()?;
        let system_font_collection = unsafe {
            let mut result = std::mem::zeroed();
            components
                .factory
                .GetSystemFontCollection2(false, &mut result, true)?;
            result.unwrap()
        };
        let custom_font_set = unsafe { components.builder.CreateFontSet()? };
        let custom_font_collection = unsafe {
            components
                .factory
                .CreateFontCollectionFromFontSet(&custom_font_set)?
        };

        Ok(Self(RwLock::new(DirectWriteState {
            components,
            system_font_collection,
            custom_font_collection,
            fonts: Vec::new(),
            font_selections: HashMap::default(),
            font_id_by_postscript_name: HashMap::default(),
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

    fn all_font_families(&self) -> Vec<String> {
        self.0.read().all_font_families()
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
        self.0.write().layout_line(text, font_size, runs)
    }

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

    unsafe fn generate_font_features(
        &self,
        font_features: &FontFeatures,
    ) -> Result<IDWriteTypography> {
        let direct_write_features = self.components.factory.CreateTypography()?;
        apply_font_features(&direct_write_features, font_features)?;
        Ok(direct_write_features)
    }

    unsafe fn get_font_id_from_font_collection(
        &mut self,
        family_name: &str,
        font_weight: FontWeight,
        font_style: FontStyle,
        font_features: &FontFeatures,
        is_system_font: bool,
    ) -> Option<FontId> {
        let collection = if is_system_font {
            &self.system_font_collection
        } else {
            &self.custom_font_collection
        };
        let Some(fontset) = collection.GetFontSet().log_err() else {
            return None;
        };
        let Some(font) = fontset
            .GetMatchingFonts(
                &HSTRING::from(family_name),
                font_weight.into(),
                DWRITE_FONT_STRETCH_NORMAL,
                font_style.into(),
            )
            .log_err()
        else {
            return None;
        };
        let total_number = font.GetFontCount();
        for index in 0..total_number {
            let Some(font_face_ref) = font.GetFontFaceReference(index).log_err() else {
                continue;
            };
            let Some(font_face) = font_face_ref.CreateFontFace().log_err() else {
                continue;
            };
            let Some(postscript_name) = get_postscript_name(&font_face, &self.components.locale)
            else {
                continue;
            };
            let is_emoji = font_face.IsColorFont().as_bool();
            let Some(direct_write_features) = self.generate_font_features(font_features).log_err()
            else {
                continue;
            };
            let font_info = FontInfo {
                font_family: family_name.to_owned(),
                font_face,
                is_system_font,
                features: direct_write_features,
                is_emoji,
            };
            let font_id = FontId(self.fonts.len());
            self.fonts.push(font_info);
            self.font_id_by_postscript_name
                .insert(postscript_name, font_id);
            return Some(font_id);
        }
        None
    }

    unsafe fn update_system_font_collection(&mut self) {
        let mut collection = std::mem::zeroed();
        self.components
            .factory
            .GetSystemFontCollection2(false, &mut collection, true)
            .unwrap();
        self.system_font_collection = collection.unwrap();
    }

    fn select_font(&mut self, target_font: &Font) -> FontId {
        let family_name = if target_font.family == ".SystemUIFont" {
            // https://learn.microsoft.com/en-us/windows/win32/uxguide/vis-fonts
            // Segoe UI is the Windows font intended for user interface text strings.
            "Segoe UI"
        } else {
            target_font.family.as_ref()
        };
        unsafe {
            // try to find target font in custom font collection first
            self.get_font_id_from_font_collection(
                family_name,
                target_font.weight,
                target_font.style,
                &target_font.features,
                false,
            )
            .or_else(|| {
                self.get_font_id_from_font_collection(
                    family_name,
                    target_font.weight,
                    target_font.style,
                    &target_font.features,
                    true,
                )
            })
            .or_else(|| {
                self.update_system_font_collection();
                self.get_font_id_from_font_collection(
                    family_name,
                    target_font.weight,
                    target_font.style,
                    &target_font.features,
                    true,
                )
            })
            .or_else(|| {
                log::error!("{} not found, use Arial instead.", family_name);
                self.get_font_id_from_font_collection(
                    "Arial",
                    target_font.weight,
                    target_font.style,
                    &target_font.features,
                    false,
                )
            })
            .unwrap()
        }
    }

    fn layout_line(&mut self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        if font_runs.is_empty() {
            return LineLayout::default();
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
                let format = self
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
                    )
                    .unwrap();

                let layout = self
                    .components
                    .factory
                    .CreateTextLayout(&text_wide, &format, f32::INFINITY, f32::INFINITY)
                    .unwrap();
                let current_text = &text[utf8_offset..(utf8_offset + first_run.len)];
                utf8_offset += first_run.len;
                let current_text_utf16_length = current_text.encode_utf16().count() as u32;
                let text_range = DWRITE_TEXT_RANGE {
                    startPosition: utf16_offset,
                    length: current_text_utf16_length,
                };
                layout
                    .SetTypography(&font_info.features, text_range)
                    .unwrap();
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
                    text_layout
                        .GetLineMetrics(Some(&mut metrics), &mut line_count as _)
                        .unwrap();
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
                text_layout
                    .SetFontCollection(collection, text_range)
                    .unwrap();
                text_layout
                    .SetFontFamilyName(&HSTRING::from(&font_info.font_family), text_range)
                    .unwrap();
                text_layout.SetFontSize(font_size.0, text_range).unwrap();
                text_layout
                    .SetFontStyle(font_info.font_face.GetStyle(), text_range)
                    .unwrap();
                text_layout
                    .SetFontWeight(font_info.font_face.GetWeight(), text_range)
                    .unwrap();
                text_layout
                    .SetTypography(&font_info.features, text_range)
                    .unwrap();
            }

            let mut runs = Vec::new();
            let renderer_context = RendererContext {
                text_system: self,
                index_converter: StringIndexConverter::new(text),
                runs: &mut runs,
                utf16_index: 0,
                width: 0.0,
            };
            text_layout
                .Draw(
                    Some(&renderer_context as *const _ as _),
                    &text_renderer.0,
                    0.0,
                    0.0,
                )
                .unwrap();
            let width = px(renderer_context.width);

            LineLayout {
                font_size,
                width,
                ascent,
                descent,
                runs,
                len: text.len(),
            }
        }
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        unsafe {
            let font_info = &self.fonts[font_id.0];
            let mut metrics = std::mem::zeroed();
            font_info.font_face.GetMetrics2(&mut metrics);

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

    unsafe fn get_glyphrun_analysis(
        &self,
        params: &RenderGlyphParams,
    ) -> windows::core::Result<IDWriteGlyphRunAnalysis> {
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
        let transform = DWRITE_MATRIX {
            m11: params.scale_factor,
            m12: 0.0,
            m21: 0.0,
            m22: params.scale_factor,
            dx: 0.0,
            dy: 0.0,
        };
        self.components.factory.CreateGlyphRunAnalysis(
            &glyph_run as _,
            1.0,
            Some(&transform as _),
            DWRITE_RENDERING_MODE_NATURAL,
            DWRITE_MEASURING_MODE_NATURAL,
            0.0,
            0.0,
        )
    }

    fn raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        unsafe {
            let glyph_run_analysis = self.get_glyphrun_analysis(params)?;
            let bounds = glyph_run_analysis.GetAlphaTextureBounds(DWRITE_TEXTURE_CLEARTYPE_3x1)?;

            Ok(Bounds {
                origin: Point {
                    x: DevicePixels(bounds.left),
                    y: DevicePixels(bounds.top),
                },
                size: Size {
                    width: DevicePixels(bounds.right - bounds.left),
                    height: DevicePixels(bounds.bottom - bounds.top),
                },
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
            return Err(anyhow!("glyph bounds are empty"));
        }

        let font_info = &self.fonts[params.font_id.0];
        let glyph_id = [params.glyph_id.0 as u16];
        let advance = [glyph_bounds.size.width.0 as f32];
        let offset = [DWRITE_GLYPH_OFFSET {
            advanceOffset: -glyph_bounds.origin.x.0 as f32 / params.scale_factor,
            ascenderOffset: glyph_bounds.origin.y.0 as f32 / params.scale_factor,
        }];
        let glyph_run = DWRITE_GLYPH_RUN {
            fontFace: unsafe { std::mem::transmute_copy(&font_info.font_face) },
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
        let transform = DWRITE_MATRIX {
            m11: params.scale_factor,
            m12: 0.0,
            m21: 0.0,
            m22: params.scale_factor,
            dx: 0.0,
            dy: 0.0,
        };
        let brush_property = D2D1_BRUSH_PROPERTIES {
            opacity: 1.0,
            transform: Matrix3x2 {
                M11: params.scale_factor,
                M12: 0.0,
                M21: 0.0,
                M22: params.scale_factor,
                M31: 0.0,
                M32: 0.0,
            },
        };

        let total_bytes;
        let bitmap_format;
        let render_target_property;
        let bitmap_stride;
        if params.is_emoji {
            total_bytes = bitmap_size.height.0 as usize * bitmap_size.width.0 as usize * 4;
            bitmap_format = &GUID_WICPixelFormat32bppPRGBA;
            render_target_property = D2D1_RENDER_TARGET_PROPERTIES {
                r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_R8G8B8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: params.scale_factor * 96.0,
                dpiY: params.scale_factor * 96.0,
                usage: D2D1_RENDER_TARGET_USAGE_NONE,
                minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
            };
            bitmap_stride = bitmap_size.width.0 as u32 * 4;
        } else {
            total_bytes = bitmap_size.height.0 as usize * bitmap_size.width.0 as usize;
            bitmap_format = &GUID_WICPixelFormat8bppAlpha;
            render_target_property = D2D1_RENDER_TARGET_PROPERTIES {
                r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_STRAIGHT,
                },
                dpiX: params.scale_factor * 96.0,
                dpiY: params.scale_factor * 96.0,
                usage: D2D1_RENDER_TARGET_USAGE_NONE,
                minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
            };
            bitmap_stride = bitmap_size.width.0 as u32;
        }

        unsafe {
            let bitmap = self.components.bitmap_factory.CreateBitmap(
                bitmap_size.width.0 as u32,
                bitmap_size.height.0 as u32,
                bitmap_format,
                WICBitmapCacheOnLoad,
            )?;
            let render_target = self
                .components
                .d2d1_factory
                .CreateWicBitmapRenderTarget(&bitmap, &render_target_property)?;
            let brush = render_target.CreateSolidColorBrush(&BRUSH_COLOR, Some(&brush_property))?;
            let subpixel_shift = params
                .subpixel_variant
                .map(|v| v as f32 / SUBPIXEL_VARIANTS as f32);
            let baseline_origin = D2D_POINT_2F {
                x: subpixel_shift.x / params.scale_factor,
                y: subpixel_shift.y / params.scale_factor,
            };

            // This `cast()` action here should never fail since we are running on Win10+, and
            // ID2D1DeviceContext4 requires Win8+
            let render_target = render_target.cast::<ID2D1DeviceContext4>().unwrap();
            render_target.BeginDraw();
            if params.is_emoji {
                // WARN: only DWRITE_GLYPH_IMAGE_FORMATS_COLR has been tested
                let enumerator = self.components.factory.TranslateColorGlyphRun2(
                    baseline_origin,
                    &glyph_run as _,
                    None,
                    DWRITE_GLYPH_IMAGE_FORMATS_COLR
                        | DWRITE_GLYPH_IMAGE_FORMATS_SVG
                        | DWRITE_GLYPH_IMAGE_FORMATS_PNG
                        | DWRITE_GLYPH_IMAGE_FORMATS_JPEG
                        | DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8,
                    DWRITE_MEASURING_MODE_NATURAL,
                    Some(&transform as _),
                    0,
                )?;
                while enumerator.MoveNext().is_ok() {
                    let Ok(color_glyph) = enumerator.GetCurrentRun2() else {
                        break;
                    };
                    let color_glyph = &*color_glyph;
                    let brush_color = translate_color(&color_glyph.Base.runColor);
                    brush.SetColor(&brush_color);
                    match color_glyph.glyphImageFormat {
                        DWRITE_GLYPH_IMAGE_FORMATS_PNG
                        | DWRITE_GLYPH_IMAGE_FORMATS_JPEG
                        | DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8 => render_target
                            .DrawColorBitmapGlyphRun(
                                color_glyph.glyphImageFormat,
                                baseline_origin,
                                &color_glyph.Base.glyphRun,
                                color_glyph.measuringMode,
                                D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DEFAULT,
                            ),
                        DWRITE_GLYPH_IMAGE_FORMATS_SVG => render_target.DrawSvgGlyphRun(
                            baseline_origin,
                            &color_glyph.Base.glyphRun,
                            &brush,
                            None,
                            color_glyph.Base.paletteIndex as u32,
                            color_glyph.measuringMode,
                        ),
                        _ => render_target.DrawGlyphRun2(
                            baseline_origin,
                            &color_glyph.Base.glyphRun,
                            Some(color_glyph.Base.glyphRunDescription as *const _),
                            &brush,
                            color_glyph.measuringMode,
                        ),
                    }
                }
            } else {
                render_target.DrawGlyphRun(
                    baseline_origin,
                    &glyph_run,
                    &brush,
                    DWRITE_MEASURING_MODE_NATURAL,
                );
            }
            render_target.EndDraw(None, None)?;
            let mut raw_data = vec![0u8; total_bytes];
            bitmap.CopyPixels(std::ptr::null() as _, bitmap_stride, &mut raw_data)?;
            if params.is_emoji {
                // Convert from BGRA with premultiplied alpha to BGRA with straight alpha.
                for pixel in raw_data.chunks_exact_mut(4) {
                    let a = pixel[3] as f32 / 255.;
                    pixel[0] = (pixel[0] as f32 / a) as u8;
                    pixel[1] = (pixel[1] as f32 / a) as u8;
                    pixel[2] = (pixel[2] as f32 / a) as u8;
                }
            }
            Ok((bitmap_size, raw_data))
        }
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

    fn all_font_families(&self) -> Vec<String> {
        get_font_names_from_collection(&self.system_font_collection, &self.components.locale)
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
    utf16_index: usize,
    width: f32,
}

#[allow(non_snake_case)]
impl IDWritePixelSnapping_Impl for TextRenderer {
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
impl IDWriteTextRenderer_Impl for TextRenderer {
    fn DrawGlyphRun(
        &self,
        clientdrawingcontext: *const ::core::ffi::c_void,
        _baselineoriginx: f32,
        _baselineoriginy: f32,
        _measuringmode: DWRITE_MEASURING_MODE,
        glyphrun: *const DWRITE_GLYPH_RUN,
        glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        _clientdrawingeffect: Option<&windows::core::IUnknown>,
    ) -> windows::core::Result<()> {
        unsafe {
            let glyphrun = &*glyphrun;
            let glyph_count = glyphrun.glyphCount as usize;
            if glyph_count == 0 {
                return Ok(());
            }
            let desc = &*glyphrundescription;
            let utf16_length_per_glyph = desc.stringLength as usize / glyph_count;
            let context =
                &mut *(clientdrawingcontext as *const RendererContext as *mut RendererContext);

            if glyphrun.fontFace.is_none() {
                return Ok(());
            }

            let font_face = glyphrun.fontFace.as_ref().unwrap();
            // This `cast()` action here should never fail since we are running on Win10+, and
            // `IDWriteFontFace3` requires Win10
            let font_face = &font_face.cast::<IDWriteFontFace3>().unwrap();
            let Some((postscript_name, font_struct, is_emoji)) =
                get_postscript_name_and_font(font_face, &self.locale)
            else {
                log::error!("none postscript name found");
                return Ok(());
            };

            let font_id = if let Some(id) = context
                .text_system
                .font_id_by_postscript_name
                .get(&postscript_name)
            {
                *id
            } else {
                context.text_system.select_font(&font_struct)
            };
            let mut glyphs = SmallVec::new();
            for index in 0..glyph_count {
                let id = GlyphId(*glyphrun.glyphIndices.add(index) as u32);
                context
                    .index_converter
                    .advance_to_utf16_ix(context.utf16_index);
                glyphs.push(ShapedGlyph {
                    id,
                    position: point(px(context.width), px(0.0)),
                    index: context.index_converter.utf8_ix,
                    is_emoji,
                });
                context.utf16_index += utf16_length_per_glyph;
                context.width += *glyphrun.glyphAdvances.add(index);
            }
            context.runs.push(ShapedRun { font_id, glyphs });
        }
        Ok(())
    }

    fn DrawUnderline(
        &self,
        _clientdrawingcontext: *const ::core::ffi::c_void,
        _baselineoriginx: f32,
        _baselineoriginy: f32,
        _underline: *const DWRITE_UNDERLINE,
        _clientdrawingeffect: Option<&windows::core::IUnknown>,
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
        _clientdrawingeffect: Option<&windows::core::IUnknown>,
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
        _inlineobject: Option<&IDWriteInlineObject>,
        _issideways: BOOL,
        _isrighttoleft: BOOL,
        _clientdrawingeffect: Option<&windows::core::IUnknown>,
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
            let Some(family_name) = get_name(localized_family_name, locale) else {
                continue;
            };
            result.push(family_name);
        }

        result
    }
}

unsafe fn get_postscript_name_and_font(
    font_face: &IDWriteFontFace3,
    locale: &str,
) -> Option<(String, Font, bool)> {
    let Some(postscript_name) = get_postscript_name(font_face, locale) else {
        return None;
    };
    let Some(localized_family_name) = font_face.GetFamilyNames().log_err() else {
        return None;
    };
    let Some(family_name) = get_name(localized_family_name, locale) else {
        return None;
    };
    let font_struct = Font {
        family: family_name.into(),
        features: FontFeatures::default(),
        weight: font_face.GetWeight().into(),
        style: font_face.GetStyle().into(),
    };
    let is_emoji = font_face.IsColorFont().as_bool();
    Some((postscript_name, font_struct, is_emoji))
}

unsafe fn get_postscript_name(font_face: &IDWriteFontFace3, locale: &str) -> Option<String> {
    let mut info = std::mem::zeroed();
    let mut exists = BOOL(0);
    font_face
        .GetInformationalStrings(
            DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME,
            &mut info,
            &mut exists,
        )
        .log_err();
    if !exists.as_bool() || info.is_none() {
        return None;
    }

    get_name(info.unwrap(), locale)
}

// https://learn.microsoft.com/en-us/windows/win32/api/dwrite/ne-dwrite-dwrite_font_feature_tag
unsafe fn apply_font_features(
    direct_write_features: &IDWriteTypography,
    features: &FontFeatures,
) -> Result<()> {
    let tag_values = features.tag_value_list();
    if tag_values.is_empty() {
        return Ok(());
    }

    // All of these features are enabled by default by DirectWrite.
    // If you want to (and can) peek into the source of DirectWrite
    let mut feature_liga = make_direct_write_feature("liga", true);
    let mut feature_clig = make_direct_write_feature("clig", true);
    let mut feature_calt = make_direct_write_feature("calt", true);

    for (tag, enable) in tag_values {
        if tag == *"liga" && !enable {
            feature_liga.parameter = 0;
            continue;
        }
        if tag == *"clig" && !enable {
            feature_clig.parameter = 0;
            continue;
        }
        if tag == *"calt" && !enable {
            feature_calt.parameter = 0;
            continue;
        }

        direct_write_features.AddFontFeature(make_direct_write_feature(&tag, enable))?;
    }
    direct_write_features.AddFontFeature(feature_liga)?;
    direct_write_features.AddFontFeature(feature_clig)?;
    direct_write_features.AddFontFeature(feature_calt)?;

    Ok(())
}

#[inline]
fn make_direct_write_feature(feature_name: &str, enable: bool) -> DWRITE_FONT_FEATURE {
    let tag = make_direct_write_tag(feature_name);
    if enable {
        DWRITE_FONT_FEATURE {
            nameTag: tag,
            parameter: 1,
        }
    } else {
        DWRITE_FONT_FEATURE {
            nameTag: tag,
            parameter: 0,
        }
    }
}

#[inline]
fn make_open_type_tag(tag_name: &str) -> u32 {
    assert_eq!(tag_name.chars().count(), 4);
    let bytes = tag_name.bytes().collect_vec();
    ((bytes[3] as u32) << 24)
        | ((bytes[2] as u32) << 16)
        | ((bytes[1] as u32) << 8)
        | (bytes[0] as u32)
}

#[inline]
fn make_direct_write_tag(tag_name: &str) -> DWRITE_FONT_FEATURE_TAG {
    DWRITE_FONT_FEATURE_TAG(make_open_type_tag(tag_name))
}

unsafe fn get_name(string: IDWriteLocalizedStrings, locale: &str) -> Option<String> {
    let mut locale_name_index = 0u32;
    let mut exists = BOOL(0);
    string
        .FindLocaleName(
            &HSTRING::from(locale),
            &mut locale_name_index,
            &mut exists as _,
        )
        .log_err();
    if !exists.as_bool() {
        string
            .FindLocaleName(
                DEFAULT_LOCALE_NAME,
                &mut locale_name_index as _,
                &mut exists as _,
            )
            .log_err();
        if !exists.as_bool() {
            return None;
        }
    }

    let name_length = string.GetStringLength(locale_name_index).unwrap() as usize;
    let mut name_vec = vec![0u16; name_length + 1];
    string.GetString(locale_name_index, &mut name_vec).unwrap();

    Some(String::from_utf16_lossy(&name_vec[..name_length]))
}

#[inline]
fn translate_color(color: &DWRITE_COLOR_F) -> D2D1_COLOR_F {
    D2D1_COLOR_F {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
    }
}

const DEFAULT_LOCALE_NAME: PCWSTR = windows::core::w!("en-US");
const BRUSH_COLOR: D2D1_COLOR_F = D2D1_COLOR_F {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
