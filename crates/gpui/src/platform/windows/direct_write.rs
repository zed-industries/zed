use std::{arch::x86_64::CpuidResult, borrow::Cow, cell::Cell, mem::ManuallyDrop, sync::Arc};

use anyhow::{anyhow, Result};
use collections::HashMap;
use itertools::Itertools;
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use util::ResultExt;
use windows::{
    core::{implement, HRESULT, HSTRING, PCWSTR},
    Win32::{
        Foundation::{BOOL, RECT},
        Globalization::GetUserDefaultLocaleName,
        Graphics::DirectWrite::*,
    },
};

use crate::{
    px, Bounds, DevicePixels, Font, FontFeatures, FontId, FontMetrics, FontRun, GlobalPixels,
    GlyphId, LineLayout, Pixels, PlatformTextSystem, Point, RenderGlyphParams, ShapedGlyph,
    ShapedRun, SharedString, Size,
};

struct FontInfo {
    font_family: String,
    font_face: IDWriteFontFace3,
    font_set_index: usize,
    // features: FontFeatures,
    features: Vec<*const DWRITE_TYPOGRAPHIC_FEATURES>,
    raw_features: FontFeatures,
}

unsafe impl Send for FontInfo {}
unsafe impl Sync for FontInfo {}

pub(crate) struct DirectWriteTextSystem(RwLock<DirectWriteState>);

struct DirectWriteComponent {
    locale: String,
    factory: IDWriteFactory5,
    in_memory_loader: IDWriteInMemoryFontFileLoader,
    builder: IDWriteFontSetBuilder1,
    analyzer: IDWriteTextAnalyzer,
}

struct DirectWriteState {
    components: DirectWriteComponent,
    font_sets: Vec<IDWriteFontSet>,
    fonts: Vec<FontInfo>,
    font_selections: HashMap<Font, FontId>,
    postscript_names_by_font_id: HashMap<FontId, String>,
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
            let analyzer = factory.CreateTextAnalyzer().unwrap();

            DirectWriteComponent {
                locale,
                factory,
                in_memory_loader,
                builder,
                analyzer,
            }
        }
    }
}

impl DirectWriteTextSystem {
    pub(crate) fn new() -> Self {
        let components = DirectWriteComponent::new();
        let system_set = unsafe { components.factory.GetSystemFontSet().unwrap() };
        let mut res = unsafe { std::mem::zeroed() };
        let x = unsafe { components.factory.GetSystemFontCollection(&mut res, false) };
        Self(RwLock::new(DirectWriteState {
            components: DirectWriteComponent::new(),
            font_sets: vec![system_set],
            fonts: Vec::new(),
            font_selections: HashMap::default(),
            postscript_names_by_font_id: HashMap::default(),
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
        todo!()
    }

    fn all_font_families(&self) -> Vec<String> {
        todo!()
    }

    fn font_id(&self, font: &Font) -> Result<FontId> {
        let lock = self.0.upgradable_read();
        if let Some(font_id) = lock.font_selections.get(font) {
            Ok(*font_id)
        } else {
            let mut lock = RwLockUpgradableReadGuard::upgrade(lock);
            let font_id = lock.select_font(font).unwrap();
            lock.font_selections.insert(font.clone(), font_id);
            println!("Get font id: {:#?}", font_id);
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
        // self.0.read().wrap_line(text, font_id, font_size, width)
        unimplemented!()
    }
}

impl DirectWriteState {
    fn add_fonts(&mut self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        println!("Adding fonts");
        for font_data in fonts {
            match font_data {
                Cow::Borrowed(data) => unsafe {
                    println!("Add borrowed font");
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
                    println!("Add owned font");
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

    fn select_font(&mut self, target_font: &Font) -> Option<FontId> {
        unsafe {
            for (fontset_index, fontset) in self.font_sets.iter().enumerate() {
                println!(
                    "Checking fontsets: {}/{}",
                    fontset_index + 1,
                    self.font_sets.len()
                );
                let font = fontset
                    .GetMatchingFonts(
                        &HSTRING::from(target_font.family.to_string()),
                        // DWRITE_FONT_WEIGHT(target_font.weight.0 as _),
                        DWRITE_FONT_WEIGHT_NORMAL,
                        DWRITE_FONT_STRETCH_NORMAL,
                        DWRITE_FONT_STYLE_NORMAL,
                    )
                    .unwrap();
                let total_number = font.GetFontCount();
                for sub_index in 0..total_number {
                    println!("   Checking sub fonts: {}/{}", sub_index + 1, total_number);
                    let font_face_ref = font.GetFontFaceReference(0).unwrap();
                    let Ok(font_face) = font_face_ref
                        .CreateFontFace()
                        .inspect_err(|e| println!("        Error: {}", e))
                    else {
                        continue;
                    };
                    let font_info = FontInfo {
                        font_family: target_font.family.to_string(),
                        font_face,
                        font_set_index: fontset_index,
                        features: direct_write_features(&target_font.features),
                        raw_features: target_font.features,
                    };
                    let font_id = FontId(self.fonts.len());
                    self.fonts.push(font_info);
                    return Some(font_id);
                }
            }
            None
        }
    }

    fn layout_line(&mut self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        unsafe {
            let locale_wide = self
                .components
                .locale
                .encode_utf16()
                .chain(Some(0))
                .collect_vec();
            let locale_string = PCWSTR::from_raw(locale_wide.as_ptr());

            let mut offset = 0usize;
            let mut shaped_runs_vec = Vec::new();
            let mut glyph_position = 0.0f32;
            let text_wide = text.encode_utf16().collect_vec();
            println!(
                "==> text: {}, raw: {:?}, raw_wide: {:?}",
                text,
                text.as_bytes(),
                text_wide
            );
            for run in font_runs {
                println!("fontrun: {:?}", run);
                let run_len = run.len;
                if run_len == 0 {
                    continue;
                }
                let font_info = &self.fonts[run.font_id.0];
                let local_str = &text[offset..(offset + run_len)];
                println!("text: {}", local_str);
                let local_wide = local_str.encode_utf16().collect_vec();
                // let local_wide = text_wide[offset..(offset + run_len)].to_vec();
                let local_length = local_wide.len();
                let local_wstring = PCWSTR::from_raw(local_wide.as_ptr());
                let analysis = Analysis::new(
                    PCWSTR::from_raw(locale_wide.as_ptr()),
                    local_wide,
                    local_length as u32,
                );

                let Some(analysis_result) = analysis.generate_result(&self.components.analyzer)
                else {
                    println!("None analysis result");
                    continue;
                };
                let list_capacity = local_length * 2;
                let mut cluster_map = vec![0u16; list_capacity];
                let mut text_props = vec![DWRITE_SHAPING_TEXT_PROPERTIES::default(); list_capacity];
                let mut glyph_indeices = vec![0u16; list_capacity];
                let mut glyph_props =
                    vec![DWRITE_SHAPING_GLYPH_PROPERTIES::default(); list_capacity];
                let mut glyph_count = 0u32;
                let featurelenght = [local_length as u32];
                println!("Getting glyphs");
                // let features = direct_write_features(&font_info.raw_features);
                let features = temp_features(&font_info.raw_features);
                self.components
                    .analyzer
                    .GetGlyphs(
                        local_wstring,
                        local_length as _,
                        &font_info.font_face,
                        false,
                        false,
                        &analysis_result as _,
                        locale_string,
                        None,
                        // Some(font_info.features.as_ptr()),
                        Some(features.as_ptr()),
                        Some(featurelenght.as_ptr()),
                        1,
                        // None,
                        // None,
                        // 0,
                        list_capacity as u32, // TODO:
                        cluster_map.as_mut_ptr(),
                        text_props.as_mut_ptr(),
                        glyph_indeices.as_mut_ptr(),
                        glyph_props.as_mut_ptr(),
                        &mut glyph_count,
                    )
                    .unwrap();

                cluster_map.truncate(glyph_count as usize);
                text_props.truncate(glyph_count as usize);
                glyph_indeices.truncate(glyph_count as usize);
                glyph_props.truncate(glyph_count as usize);
                let mut glyph_advances = vec![0.0f32; glyph_count as usize];
                let mut glyph_offsets = vec![DWRITE_GLYPH_OFFSET::default(); glyph_count as usize];
                // let mut glyph_advances = vec![0.0f32; list_capacity as usize];
                // let mut glyph_offsets =
                //     vec![DWRITE_GLYPH_OFFSET::default(); list_capacity as usize];
                println!("Getting glyphs placement");
                self.components
                    .analyzer
                    .GetGlyphPlacements(
                        local_wstring,
                        cluster_map.as_ptr(),
                        text_props.as_mut_ptr(),
                        local_length as _,
                        glyph_indeices.as_ptr(),
                        glyph_props.as_ptr(),
                        glyph_count,
                        &font_info.font_face,
                        font_size.0,
                        false,
                        false,
                        &analysis_result,
                        locale_string,
                        // Some(font_info.features.as_ptr()),
                        Some(features.as_ptr()),
                        Some(featurelenght.as_ptr()),
                        1,
                        // None,
                        // None,
                        // 0,
                        glyph_advances.as_mut_ptr(),
                        glyph_offsets.as_mut_ptr(),
                    )
                    .unwrap();

                let mut glyphs = SmallVec::new();
                for (pos, glyph) in glyph_indeices.iter().enumerate() {
                    let shaped_gylph = ShapedGlyph {
                        id: GlyphId(*glyph as u32),
                        // TODO:
                        position: Point {
                            x: px(glyph_position),
                            y: px(0.),
                        },
                        index: offset + pos,
                        // TODO:
                        is_emoji: false,
                    };
                    glyph_position += glyph_advances[pos];
                    glyphs.push(shaped_gylph);
                }
                // println!("runs: {:#?}", glyphs);
                let shaped_run = ShapedRun {
                    font_id: run.font_id,
                    glyphs,
                };
                offset += run_len;
                shaped_runs_vec.push(shaped_run);
            }
            // TODO:
            LineLayout {
                font_size,
                width: px(glyph_position),
                ascent: px(0.),
                descent: px(0.),
                runs: shaped_runs_vec,
                len: text.len(),
            }
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
            println!("Font metrics: {:#?}", res);
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
        // TODO: is this right?
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
        unsafe {
            // TODO:
            // let mut bitmap_size = glyph_bounds.size;
            // if params.subpixel_variant.x > 0 {
            //     bitmap_size.width += DevicePixels(1);
            // }
            // if params.subpixel_variant.y > 0 {
            //     bitmap_size.height += DevicePixels(1);
            // }
            // let bitmap_size = bitmap_size;
            let bitmap_size = glyph_bounds.size;

            let glyph_run_analysis = self.get_glyphrun_analysis(params)?;
            let total_bytes = bitmap_size.height.0 * bitmap_size.width.0 * 3;
            // let total_bytes = bitmap_size.height.0 * bitmap_size.width.0;
            let texture_bounds = RECT {
                left: glyph_bounds.left().0,
                top: glyph_bounds.top().0,
                right: glyph_bounds.left().0 + bitmap_size.width.0,
                bottom: glyph_bounds.top().0 + bitmap_size.height.0,
            };
            let mut result = vec![0u8; total_bytes as usize];
            glyph_run_analysis.CreateAlphaTexture(
                DWRITE_TEXTURE_CLEARTYPE_3x1,
                &texture_bounds as _,
                &mut result,
            )?;
            let mut bitmap_rawdata =
                vec![0u8; (bitmap_size.height.0 * bitmap_size.width.0) as usize];
            for (chunk, num) in result.chunks_exact(3).zip(bitmap_rawdata.iter_mut()) {
                let sum: u32 = chunk.iter().map(|&x| x as u32).sum();
                *num = (sum / 3) as u8;
            }
            Ok((bitmap_size, bitmap_rawdata))
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

// #[implement(IDWriteTextAnalysisSource, IDWriteTextAnalysisSink)]
struct Analysis {
    source: IDWriteTextAnalysisSource,
    sink: IDWriteTextAnalysisSink,
    inner: Arc<RwLock<AnalysisInner>>,
    length: u32,
}

#[implement(IDWriteTextAnalysisSource)]
struct AnalysisSource {
    inner: Arc<RwLock<AnalysisInner>>,
}

#[implement(IDWriteTextAnalysisSink)]
struct AnalysisSink {
    inner: Arc<RwLock<AnalysisInner>>,
}

struct AnalysisInner {
    locale: PCWSTR,
    text: Vec<u16>,
    text_length: u32,
    substitution: Option<IDWriteNumberSubstitution>,
    script_analysis: Option<DWRITE_SCRIPT_ANALYSIS>,
}

impl AnalysisSource {
    pub fn new(inner: Arc<RwLock<AnalysisInner>>) -> Self {
        AnalysisSource { inner }
    }
}

impl AnalysisSink {
    pub fn new(inner: Arc<RwLock<AnalysisInner>>) -> Self {
        AnalysisSink { inner }
    }

    pub fn get_result(&self) -> DWRITE_SCRIPT_ANALYSIS {
        self.inner.read().script_analysis.unwrap()
    }
}

impl AnalysisInner {
    pub fn new(locale: PCWSTR, text: Vec<u16>, text_length: u32) -> Self {
        AnalysisInner {
            locale,
            text,
            text_length,
            substitution: None,
            script_analysis: None,
        }
    }

    pub fn get_result(&self) -> Option<DWRITE_SCRIPT_ANALYSIS> {
        self.script_analysis.clone()
    }
}

impl Analysis {
    pub fn new(locale: PCWSTR, text: Vec<u16>, text_length: u32) -> Self {
        let inner = Arc::new(RwLock::new(AnalysisInner::new(locale, text, text_length)));
        let source_struct = AnalysisSource::new(inner.clone());
        let sink_struct = AnalysisSink::new(inner.clone());
        let source: IDWriteTextAnalysisSource = source_struct.into();
        let sink: IDWriteTextAnalysisSink = sink_struct.into();
        Analysis {
            source,
            sink,
            inner,
            length: text_length,
        }
    }

    // https://learn.microsoft.com/en-us/windows/win32/api/dwrite/nf-dwrite-idwritetextanalyzer-getglyphs
    pub unsafe fn generate_result(
        &self,
        analyzer: &IDWriteTextAnalyzer,
    ) -> Option<DWRITE_SCRIPT_ANALYSIS> {
        analyzer
            .AnalyzeScript(&self.source, 0, self.length, &self.sink)
            .unwrap();
        self.inner.read().get_result()
    }
}

// https://github.com/microsoft/Windows-classic-samples/blob/main/Samples/Win7Samples/multimedia/DirectWrite/CustomLayout/TextAnalysis.cpp
impl IDWriteTextAnalysisSource_Impl for AnalysisSource {
    fn GetTextAtPosition(
        &self,
        textposition: u32,
        textstring: *mut *mut u16,
        textlength: *mut u32,
    ) -> windows::core::Result<()> {
        println!("GetTextAtPosition");
        let lock = self.inner.read();
        if textposition >= lock.text_length {
            unsafe {
                *textstring = std::ptr::null_mut() as _;
                *textlength = 0;
            }
        } else {
            unsafe {
                // *textstring = self.text.as_wide()[textposition as usize..].as_ptr() as *mut u16;
                *textstring = lock.text.as_ptr().add(textposition as usize) as _;
                *textlength = lock.text_length - textposition;
            }
        }
        Ok(())
    }

    fn GetTextBeforePosition(
        &self,
        textposition: u32,
        textstring: *mut *mut u16,
        textlength: *mut u32,
    ) -> windows::core::Result<()> {
        println!("GetTextBeforePosition");
        let inner = self.inner.read();
        if textposition == 0 || textposition >= inner.text_length {
            unsafe {
                *textstring = 0 as _;
                *textlength = 0;
            }
        } else {
            unsafe {
                *textstring = inner.text.as_ptr() as *mut u16;
                *textlength = textposition - 0;
            }
        }
        Ok(())
    }

    fn GetParagraphReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        println!("GetParagraphReadingDirection");
        DWRITE_READING_DIRECTION_LEFT_TO_RIGHT
    }

    fn GetLocaleName(
        &self,
        textposition: u32,
        textlength: *mut u32,
        localename: *mut *mut u16,
    ) -> windows::core::Result<()> {
        println!("GetLocaleName");
        let inner = self.inner.read();
        unsafe {
            *localename = inner.locale.as_ptr() as *mut u16;
            *textlength = inner.text_length - textposition;
        }
        Ok(())
    }

    fn GetNumberSubstitution(
        &self,
        textposition: u32,
        textlength: *mut u32,
        numbersubstitution: *mut Option<IDWriteNumberSubstitution>,
    ) -> windows::core::Result<()> {
        println!("GetNumberSubstitution");
        let inner = self.inner.read();
        unsafe {
            *numbersubstitution = inner.substitution.clone();
            *textlength = inner.text_length - textposition;
        }
        Ok(())
    }
}

impl IDWriteTextAnalysisSink_Impl for AnalysisSink {
    fn SetScriptAnalysis(
        &self,
        textposition: u32,
        textlength: u32,
        scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
    ) -> windows::core::Result<()> {
        println!("SetScriptAnalysis");
        let mut inner = self.inner.write();
        unsafe {
            // (*scriptanalysis).shapes
            inner.script_analysis = Some(*scriptanalysis);
        }
        Ok(())
    }

    fn SetLineBreakpoints(
        &self,
        textposition: u32,
        textlength: u32,
        linebreakpoints: *const DWRITE_LINE_BREAKPOINT,
    ) -> windows::core::Result<()> {
        println!("SetLineBreakpoints");
        Err(windows::core::Error::new(HRESULT(-1), "SetLineBreakpoints"))
    }

    fn SetBidiLevel(
        &self,
        textposition: u32,
        textlength: u32,
        explicitlevel: u8,
        resolvedlevel: u8,
    ) -> windows::core::Result<()> {
        println!("SetBidiLevel");
        Err(windows::core::Error::new(HRESULT(-1), "SetBidiLevel"))
    }

    fn SetNumberSubstitution(
        &self,
        textposition: u32,
        textlength: u32,
        numbersubstitution: Option<&IDWriteNumberSubstitution>,
    ) -> windows::core::Result<()> {
        println!("SetNumberSubstitution");
        Err(windows::core::Error::new(
            HRESULT(-1),
            "SetNumberSubstitution",
        ))
    }
}

// https://learn.microsoft.com/en-us/windows/win32/api/dwrite/ne-dwrite-dwrite_font_feature_tag
fn direct_write_features(features: &FontFeatures) -> Vec<*const DWRITE_TYPOGRAPHIC_FEATURES> {
    let mut result = Vec::new();
    add_feature(
        &mut result,
        DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_ALTERNATES,
        features.calt(),
    );
    add_feature(
        &mut result,
        DWRITE_FONT_FEATURE_TAG_CASE_SENSITIVE_FORMS,
        features.case(),
    );
    add_feature(
        &mut result,
        DWRITE_FONT_FEATURE_TAG_CAPITAL_SPACING,
        features.cpsp(),
    );
    add_feature(
        &mut result,
        DWRITE_FONT_FEATURE_TAG_FRACTIONS,
        features.frac(),
    );
    add_feature(
        &mut result,
        DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES,
        features.liga(),
    );
    result
}

fn add_feature(
    feature_list: &mut Vec<*const DWRITE_TYPOGRAPHIC_FEATURES>,
    feature: DWRITE_FONT_FEATURE_TAG,
    enable: Option<bool>,
) {
    let Some(enable) = enable else {
        return;
    };
    let font_feature = if enable {
        Arc::new(DWRITE_FONT_FEATURE {
            nameTag: feature,
            parameter: 1,
        })
    } else {
        Arc::new(DWRITE_FONT_FEATURE {
            nameTag: feature,
            parameter: 0,
        })
    };
    let result = Arc::new(DWRITE_TYPOGRAPHIC_FEATURES {
        features: Arc::into_raw(font_feature) as _,
        featureCount: 1,
    });
    feature_list.push(Arc::into_raw(result));
}

fn temp_features(features: &FontFeatures) -> Vec<*const DWRITE_TYPOGRAPHIC_FEATURES> {
    let mut result = Vec::new();
    add_feature(
        &mut result,
        DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_ALTERNATES,
        features.calt(),
    );
    add_feature(
        &mut result,
        DWRITE_FONT_FEATURE_TAG_CASE_SENSITIVE_FORMS,
        features.case(),
    );
    add_feature(
        &mut result,
        DWRITE_FONT_FEATURE_TAG_CAPITAL_SPACING,
        features.cpsp(),
    );
    add_feature(
        &mut result,
        DWRITE_FONT_FEATURE_TAG_FRACTIONS,
        features.frac(),
    );
    add_feature(
        &mut result,
        DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES,
        features.liga(),
    );
    result
}

fn temp_add_feature(
    feature_list: &mut Vec<*const DWRITE_TYPOGRAPHIC_FEATURES>,
    feature: DWRITE_FONT_FEATURE_TAG,
    enable: Option<bool>,
) {
    let Some(enable) = enable else {
        return;
    };
    let font_feature = if enable {
        Box::new(DWRITE_FONT_FEATURE {
            nameTag: feature,
            parameter: 1,
        })
    } else {
        Box::new(DWRITE_FONT_FEATURE {
            nameTag: feature,
            parameter: 0,
        })
    };
    let result = Box::new(DWRITE_TYPOGRAPHIC_FEATURES {
        features: Box::into_raw(font_feature),
        featureCount: 1,
    });
    feature_list.push(Box::into_raw(result) as *const _);
}
