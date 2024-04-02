use std::{
    borrow::{Borrow, Cow},
    mem::ManuallyDrop,
    sync::Arc,
};

use anyhow::{anyhow, Result};
use collections::HashMap;
use itertools::Itertools;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use util::ResultExt;
use windows::{
    core::{implement, HRESULT, HSTRING, PCWSTR},
    Foundation::Numerics::Matrix3x2,
    Win32::{
        Foundation::{BOOL, COLORREF, DWRITE_E_NOCOLOR, RECT},
        Globalization::GetUserDefaultLocaleName,
        Graphics::{
            Direct2D::{
                Common::{
                    D2D1_ALPHA_MODE_IGNORE, D2D1_ALPHA_MODE_PREMULTIPLIED,
                    D2D1_ALPHA_MODE_STRAIGHT, D2D1_COLOR_F, D2D1_PIXEL_FORMAT, D2D_POINT_2F,
                    D2D_SIZE_F, D2D_SIZE_U,
                },
                D2D1CreateFactory, ID2D1Bitmap1, ID2D1Factory, D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
                D2D1_BITMAP_OPTIONS_CPU_READ, D2D1_BITMAP_PROPERTIES, D2D1_BITMAP_PROPERTIES1,
                D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS,
                D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_GDI_COMPATIBLE,
                D2D1_FACTORY_TYPE_MULTI_THREADED, D2D1_FEATURE_LEVEL_DEFAULT,
                D2D1_MAP_OPTIONS_READ, D2D1_RENDER_TARGET_PROPERTIES,
                D2D1_RENDER_TARGET_TYPE_DEFAULT, D2D1_RENDER_TARGET_TYPE_SOFTWARE,
                D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE,
            },
            DirectWrite::*,
            Dxgi::Common::{DXGI_FORMAT_A8_UNORM, DXGI_FORMAT_B8G8R8A8_UNORM},
            Gdi::{
                CreateBitmap, CreateCompatibleBitmap, CreateCompatibleDC, DeleteObject,
                GetCurrentObject, GetDIBits, GetObjectW, GetStockObject, Rectangle, ReleaseDC,
                SelectObject, SetBoundsRect, SetDCBrushColor, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
                DCB_ENABLE, DCB_RESET, DC_BRUSH, DIBSECTION, DIB_RGB_COLORS, HDC, NULL_PEN,
                OBJ_BITMAP, SET_BOUNDS_RECT_FLAGS,
            },
        },
    },
};

use crate::{
    point, px, Bounds, DevicePixels, Font, FontFeatures, FontId, FontMetrics, FontRun, FontStyle,
    FontWeight, GlyphId, LineLayout, Pixels, PlatformTextSystem, Point, RenderGlyphParams,
    ShapedGlyph, ShapedRun, Size, SUBPIXEL_VARIANTS,
};

#[derive(Debug)]
struct FontInfo {
    font_family: String,
    font_face: IDWriteFontFace3,
    font_set_index: usize,
    features: Vec<DWRITE_FONT_FEATURE>,
    is_emoji: bool,
}

pub(crate) struct DirectWriteTextSystem(RwLock<DirectWriteState>);

struct DirectWriteComponent {
    locale: String,
    factory: IDWriteFactory5,
    in_memory_loader: IDWriteInMemoryFontFileLoader,
    builder: IDWriteFontSetBuilder1,
    gdi: IDWriteGdiInterop,
}

struct DirectWriteState {
    components: DirectWriteComponent,
    font_sets: Vec<IDWriteFontSet>,
    fonts: Vec<FontInfo>,
    font_selections: HashMap<Font, FontId>,
    font_id_by_postscript_name: HashMap<String, FontId>,
}

impl DirectWriteComponent {
    pub fn new() -> Self {
        unsafe {
            let factory: IDWriteFactory5 = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED).unwrap();
            let in_memory_loader = factory.CreateInMemoryFontFileLoader().unwrap();
            factory.RegisterFontFileLoader(&in_memory_loader).unwrap();
            let builder = factory.CreateFontSetBuilder2().unwrap();
            let mut locale_vec = vec![0u16; 512];
            GetUserDefaultLocaleName(&mut locale_vec);
            let locale = String::from_utf16_lossy(&locale_vec);
            let gdi = factory.GetGdiInterop().unwrap();

            DirectWriteComponent {
                locale,
                factory,
                in_memory_loader,
                builder,
                gdi,
            }
        }
    }
}

impl DirectWriteTextSystem {
    pub(crate) fn new() -> Self {
        let components = DirectWriteComponent::new();
        let system_set = unsafe { components.factory.GetSystemFontSet().unwrap() };

        Self(RwLock::new(DirectWriteState {
            components: DirectWriteComponent::new(),
            font_sets: vec![system_set],
            fonts: Vec::new(),
            font_selections: HashMap::default(),
            font_id_by_postscript_name: HashMap::default(),
        }))
    }
}

impl Default for DirectWriteTextSystem {
    fn default() -> Self {
        Self::new()
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
            let font_id = lock.select_font(font).unwrap();
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
        println!("===========================================================");
        println!("fonts: {:?}", self.0.read().font_selections);
        println!("fonts: {:?}", self.0.read().fonts);
        println!("fonts: {:?}", self.0.read().font_id_by_postscript_name);
        println!("fonts: {:?}", self.0.read().font_sets);
        println!("===========================================================");
        self.0.write().layout_line(text, font_size, runs)
    }

    fn wrap_line(
        &self,
        _text: &str,
        _font_id: FontId,
        _font_size: Pixels,
        _width: Pixels,
    ) -> Vec<usize> {
        // self.0.read().wrap_line(text, font_id, font_size, width)
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
        self.font_sets.push(set);

        Ok(())
    }

    unsafe fn match_font_from_font_sets(
        &mut self,
        family_name: String,
        font_weight: FontWeight,
        font_style: FontStyle,
        features: &FontFeatures,
    ) -> Option<FontId> {
        for (fontset_index, fontset) in self.font_sets.iter().enumerate() {
            let font = fontset
                .GetMatchingFonts(
                    &HSTRING::from(&family_name),
                    DWRITE_FONT_WEIGHT(font_weight.0 as i32),
                    DWRITE_FONT_STRETCH_NORMAL,
                    DWRITE_FONT_STYLE_NORMAL,
                )
                .unwrap();
            let total_number = font.GetFontCount();
            for _ in 0..total_number {
                let font_face_ref = font.GetFontFaceReference(0).unwrap();
                let Some(font_face) = font_face_ref.CreateFontFace().log_err() else {
                    continue;
                };
                let Some(postscript_name) = get_postscript_name(&font_face) else {
                    continue;
                };
                let is_emoji = font_face.IsColorFont().as_bool();
                let font_info = FontInfo {
                    font_family: family_name,
                    font_face,
                    font_set_index: fontset_index,
                    features: direct_write_features(features),
                    is_emoji,
                };
                let font_id = FontId(self.fonts.len());
                self.fonts.push(font_info);
                self.font_id_by_postscript_name
                    .insert(postscript_name, font_id);
                return Some(font_id);
            }
        }
        None
    }

    fn select_font(&mut self, target_font: &Font) -> Option<FontId> {
        unsafe {
            self.match_font_from_font_sets(
                target_font.family.to_string(),
                target_font.weight,
                target_font.style,
                &target_font.features,
            )
        }
    }

    fn select_font_by_family(&mut self, family: String) -> Option<FontId> {
        unsafe {
            self.match_font_from_font_sets(
                family,
                FontWeight::NORMAL,
                FontStyle::Normal,
                &FontFeatures::default(),
            )
        }
    }

    fn layout_line(&mut self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        if font_runs.is_empty() {
            return LineLayout::default();
        }
        unsafe {
            let locale_wide = self
                .components
                .locale
                .encode_utf16()
                .chain(Some(0))
                .collect_vec();
            let locale_name = PCWSTR::from_raw(locale_wide.as_ptr());
            let text_wide = text.encode_utf16().collect_vec();

            let mut offset = 0usize;
            let mut wstring_offset = 0u32;
            let (text_format, text_layout) = {
                let first_run = &font_runs[0];
                let font_info = &self.fonts[first_run.font_id.0];
                let collection = {
                    let font_set = &self.font_sets[font_info.font_set_index];
                    self.components
                        .factory
                        .CreateFontCollectionFromFontSet(font_set)
                        .unwrap()
                };
                let font_family_name = font_info.font_family.clone();
                let font_weight = font_info.font_face.GetWeight();
                let font_style = font_info.font_face.GetStyle();
                let format = self
                    .components
                    .factory
                    .CreateTextFormat(
                        &HSTRING::from(&font_family_name),
                        &collection,
                        font_weight,
                        font_style,
                        DWRITE_FONT_STRETCH_NORMAL,
                        font_size.0,
                        locale_name,
                    )
                    .unwrap();
                let features = self.components.factory.CreateTypography().unwrap();
                for x in font_info.features.iter() {
                    features.AddFontFeature(*x).unwrap();
                }
                let layout = self
                    .components
                    .factory
                    .CreateTextLayout(&text_wide, &format, f32::INFINITY, f32::INFINITY)
                    .unwrap();
                let first_str = &text[offset..(offset + first_run.len)];
                offset += first_run.len;
                let first_wstring = first_str.encode_utf16().collect_vec();
                let local_length = first_wstring.len() as u32;
                let text_range = DWRITE_TEXT_RANGE {
                    startPosition: wstring_offset,
                    length: local_length,
                };
                layout.SetTypography(&features, text_range);
                wstring_offset += local_length;
                (format, layout)
            };

            let mut first_run = true;
            for run in font_runs {
                if first_run {
                    first_run = false;
                    continue;
                }
                let font_info = &self.fonts[run.font_id.0];
                let local_str = &text[offset..(offset + run.len)];
                offset += run.len;
                let local_wide = local_str.encode_utf16().collect_vec();
                let local_length = local_wide.len() as u32;

                let collection = {
                    let font_set = &self.font_sets[font_info.font_set_index];
                    self.components
                        .factory
                        .CreateFontCollectionFromFontSet(font_set)
                        .unwrap()
                };
                let text_range = DWRITE_TEXT_RANGE {
                    startPosition: wstring_offset,
                    length: local_length,
                };
                wstring_offset += local_length;
                text_layout
                    .SetFontCollection(&collection, text_range)
                    .unwrap();
                text_layout
                    .SetFontFamilyName(&HSTRING::from(&font_info.font_family), text_range)
                    .unwrap();
                let features = self.components.factory.CreateTypography().unwrap();
                for x in font_info.features.iter() {
                    features.AddFontFeature(*x).unwrap();
                }
                text_layout.SetTypography(&features, text_range).unwrap();
            }

            let renderer_inner = Arc::new(RwLock::new(TextRendererInner::new()));
            let renderer: IDWriteTextRenderer =
                TextRenderer::new(renderer_inner.clone(), locale_name).into();
            text_layout.Draw(None, &renderer, 0.0, 0.0).unwrap();

            let mut ix_converter = StringIndexConverter::new(text);
            let runs = {
                let mut vec = Vec::new();
                for result in renderer_inner.read().runs.iter() {
                    let font_id;
                    if let Some(id) = self.font_id_by_postscript_name.get(&result.postscript) {
                        font_id = *id;
                    } else {
                        font_id = self.select_font_by_family(result.family.clone()).unwrap();
                    }
                    let mut glyphs = SmallVec::new();
                    for glyph in result.glyphs.iter() {
                        ix_converter.advance_to_utf16_ix(glyph.index);
                        glyphs.push(ShapedGlyph {
                            id: glyph.id,
                            position: glyph.position,
                            index: ix_converter.utf8_ix,
                            is_emoji: result.is_emoji,
                        });
                    }
                    vec.push(ShapedRun { font_id, glyphs });
                }
                vec
            };

            let mut metrics = vec![DWRITE_LINE_METRICS::default(); 4];
            let mut line_count = 0u32;
            text_layout
                .GetLineMetrics(Some(&mut metrics), &mut line_count as _)
                .unwrap();
            let width = renderer_inner.read().width;
            let ascent = px(metrics[0].baseline);
            let descent = px(metrics[0].height - metrics[0].baseline);

            let result = LineLayout {
                font_size,
                width: px(width),
                ascent,
                descent,
                runs,
                len: text.len(),
            };
            println!("Linelayout: {:#?}", result);
            result
        }
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        unsafe {
            let font_info = &self.fonts[font_id.0];
            let mut metrics = std::mem::zeroed();
            font_info.font_face.GetMetrics2(&mut metrics);

            let res = FontMetrics {
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
            };

            res
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
            fontFace: ManuallyDrop::new(Some(
                // TODO: remove this clone
                <IDWriteFontFace3 as Clone>::clone(&font.font_face).into(),
            )),
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
            // None,
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
        println!(
            "rastering: {}, is emoji {}",
            font_info.font_family, params.is_emoji
        );
        let glyph_id = [params.glyph_id.0 as u16];
        let advance = [0.0f32];
        let offset = [DWRITE_GLYPH_OFFSET::default()];
        let glyph_run = DWRITE_GLYPH_RUN {
            fontFace: ManuallyDrop::new(Some(
                // TODO: remove this clone😀
                <IDWriteFontFace3 as Clone>::clone(&font_info.font_face).into(),
            )),
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

        unsafe {
            if params.is_emoji {
                let bitmap_size = glyph_bounds.size;
                let total_bytes = bitmap_size.height.0 as usize * bitmap_size.width.0 as usize * 4;

                let enumerator = self
                    .components
                    .factory
                    .TranslateColorGlyphRun2(
                        D2D_POINT_2F { x: 0.0, y: 0.0 },
                        &glyph_run as _,
                        None,
                        DWRITE_GLYPH_IMAGE_FORMATS_COLR,
                        DWRITE_MEASURING_MODE_NATURAL,
                        Some(&transform as _),
                        0,
                    )
                    .unwrap();

                let current_transform = DWRITE_MATRIX {
                    m11: 1.0,
                    m12: 0.0,
                    m21: 0.0,
                    m22: 1.0,
                    dx: (-glyph_bounds.origin.x.0 as f32) / 2.0 - 1.0,
                    dy: ((glyph_bounds.origin.y.0 + glyph_bounds.size.height.0) / 2) as f32,
                };
                let bitmap_render_target = self
                    .components
                    .gdi
                    .CreateBitmapRenderTarget(
                        None,
                        bitmap_size.width.0 as u32,
                        bitmap_size.height.0 as u32,
                    )
                    .unwrap();
                let bitmap_render_target: IDWriteBitmapRenderTarget3 =
                    std::mem::transmute(bitmap_render_target);
                bitmap_render_target
                    .SetCurrentTransform(Some(&current_transform))
                    .unwrap();

                let render_params = self.components.factory.CreateRenderingParams()?;
                // bitmap_render_target
                //     .DrawGlyphRunWithColorSupport(
                //         0.0,
                //         0.0,
                //         DWRITE_MEASURING_MODE_NATURAL,
                //         &glyph_run,
                //         &render_params,
                //         COLORREF(0x77777777),
                //         0,
                //         None,
                //     )
                //     .inspect_err(|e| {
                //         println!("Error: {}, msg: {}", e, std::io::Error::last_os_error())
                //     });

                while enumerator.MoveNext().is_ok() {
                    let Ok(run) = enumerator.GetCurrentRun2() else {
                        break;
                    };
                    let emoji = &*run;
                    bitmap_render_target
                        .DrawGlyphRun(
                            0.0,
                            (glyph_bounds.size.height.0 / 2) as f32,
                            DWRITE_MEASURING_MODE_NATURAL,
                            &emoji.Base.glyphRun,
                            &render_params,
                            translate_color(&emoji.Base.runColor),
                            None,
                        )
                        .unwrap();
                }

                let mut raw_bytes = vec![0u8; total_bytes];
                let bitmap_data = bitmap_render_target.GetBitmapData().unwrap();
                let raw_u32 = std::slice::from_raw_parts(bitmap_data.pixels, total_bytes / 4);
                for (bytes, color) in raw_bytes.chunks_exact_mut(4).zip(raw_u32.iter()) {
                    bytes[3] = 0xFF;
                    if *color == 0 {
                        continue;
                    }
                    bytes[0] = (color >> 16 & 0xFF) as u8;
                    bytes[1] = (color >> 8 & 0xFF) as u8;
                    bytes[2] = (color & 0xFF) as u8;
                }
                Ok((bitmap_size, raw_bytes))
            } else {
                let bitmap_size = glyph_bounds.size;

                let glyph_run_analysis = self.get_glyphrun_analysis(params)?;
                let total_bytes = bitmap_size.height.0 as usize * bitmap_size.width.0 as usize * 3;
                let texture_bounds = RECT {
                    left: glyph_bounds.left().0,
                    top: glyph_bounds.top().0,
                    right: glyph_bounds.right().0,
                    bottom: glyph_bounds.bottom().0,
                };
                let mut result = vec![0u8; total_bytes];
                glyph_run_analysis.CreateAlphaTexture(
                    DWRITE_TEXTURE_CLEARTYPE_3x1,
                    &texture_bounds as _,
                    &mut result,
                )?;
                let mut bitmap_rawdata =
                    vec![0u8; bitmap_size.height.0 as usize * bitmap_size.width.0 as usize];
                for (chunk, num) in result.chunks_exact(3).zip(bitmap_rawdata.iter_mut()) {
                    let sum: u32 = chunk.iter().map(|&x| x as u32).sum();
                    *num = (sum / 3) as u8;
                }
                Ok((bitmap_size, bitmap_rawdata))
            }
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
            let left_side_bearing = metrics.leftSideBearing as i32;
            let right_side_bearing = metrics.rightSideBearing as i32;
            let top_side_bearing = metrics.topSideBearing as i32;
            let bottom_side_bearing = metrics.bottomSideBearing as i32;
            let vertical_origin_y = metrics.verticalOriginY as i32;

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
        unsafe {
            let mut result = Vec::new();
            let mut system_collection = std::mem::zeroed();
            self.components
                .factory
                .GetSystemFontCollection(&mut system_collection, false)
                .unwrap();
            if system_collection.is_none() {
                return result;
            }
            let system_collection = system_collection.unwrap();
            let locale_name_wide = self
                .components
                .locale
                .encode_utf16()
                .chain(Some(0))
                .collect_vec();
            let locale_name = PCWSTR::from_raw(locale_name_wide.as_ptr());
            let family_count = system_collection.GetFontFamilyCount();
            for index in 0..family_count {
                let font_family = system_collection.GetFontFamily(index).unwrap();
                let font_count = font_family.GetFontCount();
                for font_index in 0..font_count {
                    let font = font_family.GetFont(font_index).unwrap();
                    let mut font_name_localized_string: Option<IDWriteLocalizedStrings> = {
                        let mut string: Option<IDWriteLocalizedStrings> = std::mem::zeroed();
                        let mut exists = BOOL(0);
                        font.GetInformationalStrings(
                            DWRITE_INFORMATIONAL_STRING_FULL_NAME,
                            &mut string as _,
                            &mut exists as _,
                        )
                        .unwrap();
                        if exists.as_bool() {
                            string
                        } else {
                            continue;
                        }
                    };
                    let Some(localized_font_name) = font_name_localized_string else {
                        continue;
                    };
                    let Some(font_name) = get_name(localized_font_name, locale_name) else {
                        continue;
                    };
                    result.push(font_name);
                }
            }

            result
        }
    }

    fn all_font_families(&self) -> Vec<String> {
        unsafe {
            let mut result = Vec::new();
            let mut system_collection = std::mem::zeroed();
            self.components
                .factory
                .GetSystemFontCollection(&mut system_collection, false)
                .unwrap();
            if system_collection.is_none() {
                return result;
            }
            let system_collection = system_collection.unwrap();
            let locale_name_wide = self
                .components
                .locale
                .encode_utf16()
                .chain(Some(0))
                .collect_vec();
            let locale_name = PCWSTR::from_raw(locale_name_wide.as_ptr());
            let family_count = system_collection.GetFontFamilyCount();
            for index in 0..family_count {
                let Some(font_family) = system_collection.GetFontFamily(index).log_err() else {
                    continue;
                };
                let Some(localized_family_name) = font_family.GetFamilyNames().log_err() else {
                    continue;
                };
                let Some(family_name) = get_name(localized_family_name, locale_name) else {
                    continue;
                };
                result.push(family_name);
            }

            result
        }
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

#[implement(IDWriteTextRenderer)]
struct TextRenderer {
    inner: Arc<RwLock<TextRendererInner>>,
    locale: PCWSTR,
}

impl TextRenderer {
    pub fn new(inner: Arc<RwLock<TextRendererInner>>, locale: PCWSTR) -> Self {
        TextRenderer { inner, locale }
    }
}

struct RendererShapedGlyph {
    id: GlyphId,
    position: Point<Pixels>,
    index: usize,
}

struct RendererShapedRun {
    postscript: String,
    family: String,
    is_emoji: bool,
    glyphs: SmallVec<[RendererShapedGlyph; 8]>,
}

struct TextRendererInner {
    index: usize,
    width: f32,
    runs: Vec<RendererShapedRun>,
}

impl TextRendererInner {
    pub fn new() -> Self {
        TextRendererInner {
            index: 0,
            width: 0.0,
            runs: Vec::new(),
        }
    }
}

struct GlyphRunResult {
    id: GlyphId,
    advance: f32,
    index: usize,
}

impl IDWritePixelSnapping_Impl for TextRenderer {
    fn IsPixelSnappingDisabled(
        &self,
        _clientdrawingcontext: *const ::core::ffi::c_void,
    ) -> windows::core::Result<BOOL> {
        Ok(BOOL(1))
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

impl IDWriteTextRenderer_Impl for TextRenderer {
    fn DrawGlyphRun(
        &self,
        _clientdrawingcontext: *const ::core::ffi::c_void,
        _baselineoriginx: f32,
        _baselineoriginy: f32,
        _measuringmode: DWRITE_MEASURING_MODE,
        glyphrun: *const DWRITE_GLYPH_RUN,
        _glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        _clientdrawingeffect: Option<&windows::core::IUnknown>,
    ) -> windows::core::Result<()> {
        unsafe {
            let glyphrun = &*glyphrun;
            if glyphrun.fontFace.is_none() {
                return Ok(());
            }
            let font = glyphrun.fontFace.as_ref().unwrap();
            let Some((postscript_name, family_name, is_emoji)) =
                get_postscript_and_family_name(font, self.locale)
            else {
                log::error!("none postscript name found");
                return Ok(());
            };

            let mut global_index = self.inner.read().index;
            let mut position = self.inner.read().width;
            let mut glyphs = SmallVec::new();
            for index in 0..glyphrun.glyphCount {
                let id = GlyphId(*glyphrun.glyphIndices.add(index as _) as u32);
                glyphs.push(RendererShapedGlyph {
                    id,
                    position: point(px(position), px(0.0)),
                    index: global_index,
                });
                position += *glyphrun.glyphAdvances.add(index as _);
                if is_emoji {
                    global_index += 2;
                } else {
                    global_index += 1;
                }
            }
            self.inner.write().index = global_index;
            self.inner.write().width = position;
            self.inner.write().runs.push(RendererShapedRun {
                postscript: postscript_name,
                family: family_name,
                is_emoji,
                glyphs,
            });
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
            HRESULT(-1),
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
            HRESULT(-1),
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
            HRESULT(-1),
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

unsafe fn get_postscript_and_family_name(
    font_face: &IDWriteFontFace,
    locale: PCWSTR,
) -> Option<(String, String, bool)> {
    let font_face_pointer = font_face as *const IDWriteFontFace;
    let font_face_3_pointer: *const IDWriteFontFace3 = std::mem::transmute(font_face_pointer);
    let font_face_3 = &*font_face_3_pointer;
    let Some(postscript_name) = get_postscript_name(font_face_3) else {
        return None;
    };
    let Some(localized_family_name) = font_face_3.GetFamilyNames().log_err() else {
        return None;
    };
    Some((
        postscript_name,
        get_name(localized_family_name, locale).unwrap(),
        font_face_3.IsColorFont().as_bool(),
    ))
}

unsafe fn get_postscript_name(font_face: &IDWriteFontFace3) -> Option<String> {
    let mut info = std::mem::zeroed();
    let mut exists = BOOL(0);
    font_face
        .GetInformationalStrings(
            DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME,
            &mut info,
            &mut exists,
        )
        .unwrap();
    if !exists.as_bool() || info.is_none() {
        return None;
    }

    get_name(info.unwrap(), DEFAULT_LOCALE_NAME)
}

// https://learn.microsoft.com/en-us/windows/win32/api/dwrite/ne-dwrite-dwrite_font_feature_tag
fn direct_write_features(features: &FontFeatures) -> Vec<DWRITE_FONT_FEATURE> {
    let mut feature_list = Vec::new();
    let tag_values = features.tag_value_list();
    if tag_values.is_empty() {
        return feature_list;
    }
    // All of these features are enabled by default by DirectWrite.
    // If you want to (and can) peek into the source of DirectWrite
    add_feature(&mut feature_list, "liga", true);
    add_feature(&mut feature_list, "clig", true);
    add_feature(&mut feature_list, "calt", true);

    for (tag, enable) in tag_values {
        if tag == "liga".to_string() && !enable {
            feature_list[0].parameter = 0;
            continue;
        }
        if tag == "clig".to_string() && !enable {
            feature_list[1].parameter = 0;
            continue;
        }
        if tag == "calt".to_string() && !enable {
            feature_list[2].parameter = 0;
            continue;
        }
        add_feature(&mut feature_list, &tag, enable);
    }

    feature_list
}

fn add_feature(feature_list: &mut Vec<DWRITE_FONT_FEATURE>, feature_name: &str, enable: bool) {
    let tag = make_direct_write_tag(feature_name);
    let font_feature = if enable {
        DWRITE_FONT_FEATURE {
            nameTag: tag,
            parameter: 1,
        }
    } else {
        DWRITE_FONT_FEATURE {
            nameTag: tag,
            parameter: 0,
        }
    };
    feature_list.push(font_feature);
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

unsafe fn get_name(string: IDWriteLocalizedStrings, locale: PCWSTR) -> Option<String> {
    let mut locale_name_index = 0u32;
    let mut exists = BOOL(0);
    string
        .FindLocaleName(locale, &mut locale_name_index, &mut exists as _)
        .unwrap();
    if !exists.as_bool() {
        string
            .FindLocaleName(
                DEFAULT_LOCALE_NAME,
                &mut locale_name_index as _,
                &mut exists as _,
            )
            .unwrap();
    }
    if !exists.as_bool() {
        return None;
    }

    let name_length = string.GetStringLength(locale_name_index).unwrap() as usize;
    let mut name_vec = vec![0u16; name_length + 1];
    string.GetString(locale_name_index, &mut name_vec).unwrap();

    Some(String::from_utf16_lossy(&name_vec[..name_length]))
}

fn translate_color(color: &DWRITE_COLOR_F) -> COLORREF {
    let r_int = (color.r * 255.0) as u32;
    let g_int = (color.g * 255.0) as u32;
    let b_int = (color.b * 255.0) as u32;
    let a_int = (color.a * 255.0) as u32;

    COLORREF((a_int << 24) | (b_int << 16) | (g_int << 8) | r_int)
}

const DEFAULT_LOCALE_NAME: PCWSTR = windows::core::w!("en-US");
