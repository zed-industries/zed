use std::{arch::x86_64::CpuidResult, borrow::Cow, cell::Cell, sync::Arc};

use anyhow::{anyhow, Result};
use collections::HashMap;
use itertools::Itertools;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use windows::{
    core::{implement, HRESULT, HSTRING, PCWSTR},
    Win32::{Foundation::BOOL, Globalization::GetUserDefaultLocaleName, Graphics::DirectWrite::*},
};

use crate::{
    Bounds, DevicePixels, Font, FontFeatures, FontId, FontMetrics, FontRun, GlyphId, LineLayout,
    Pixels, PlatformTextSystem, RenderGlyphParams, ShapedGlyph, ShapedRun, SharedString, Size,
};

struct FontInfo {
    font_family: String,
    font_face: IDWriteFontFace3,
    font_set_index: usize,
}

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
        todo!()
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        todo!()
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<Size<f32>> {
        todo!()
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        todo!()
    }

    fn glyph_raster_bounds(
        &self,
        params: &RenderGlyphParams,
    ) -> anyhow::Result<Bounds<DevicePixels>> {
        todo!()
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> anyhow::Result<(Size<DevicePixels>, Vec<u8>)> {
        todo!()
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
                    let font_id = FontId(self.fonts.len());
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
                    };
                    self.fonts.push(font_info);
                    return Some(font_id);
                }
            }
            None
        }
    }

    fn layout_line(&mut self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        unsafe {
            let string = text.encode_utf16().collect_vec();
            let mut offset = 0usize;
            // let mut shaped_runs_vec = Vec::new();
            for run in font_runs {
                let run_len = run.len;
                let font_info = &self.fonts[run.font_id.0];
                let locale_wide = self
                    .components
                    .locale
                    .encode_utf16()
                    .chain(Some(0))
                    .collect_vec();
                let locale_string = PCWSTR::from_raw(locale_wide.as_ptr());
                let cur_str_wide = &string[offset..(offset + run_len)];
                let cur_string = PCWSTR::from_raw(cur_str_wide.as_ptr());
                let analysis = Analysis::new(
                    PCWSTR::from_raw(locale_wide.as_ptr()),
                    cur_str_wide.to_vec(),
                    run_len as u32,
                );
                println!("Generating res");
                let x = analysis.generate_result(&self.components.analyzer);
                println!("Generated res");
                // let mut res = std::mem::zeroed();
                // self.components.analyzer.AnalyzeScript(
                //     PCWSTR::from_raw(cur_str.as_ptr()),
                //     offset as _,
                //     run_len as _,
                //     &mut res,
                // );
                let list_capacity = run_len * 2;
                let mut cluster_map = 0u16;
                let mut text_props = vec![DWRITE_SHAPING_TEXT_PROPERTIES::default(); list_capacity];
                let mut glyph_indeices = vec![0u16; list_capacity];
                let mut glyph_props =
                    vec![DWRITE_SHAPING_GLYPH_PROPERTIES::default(); list_capacity];
                let mut glyph_count = 0u32;
                println!("Getting glyphs");
                self.components
                    .analyzer
                    .GetGlyphs(
                        cur_string,
                        run_len as _,
                        &font_info.font_face,
                        false,
                        false,
                        &x as _,
                        locale_string,
                        None,
                        None,
                        None,
                        0,
                        100,
                        &mut cluster_map as _,
                        text_props.as_mut_ptr(),
                        glyph_indeices.as_mut_ptr(),
                        glyph_props.as_mut_ptr(),
                        &mut glyph_count,
                    )
                    .unwrap();

                println!(
                    "Res: len: {}, {} glyphs, indices: {:?}",
                    run_len, glyph_count, glyph_indeices
                );

                let shaped_gylph = ShapedGlyph {
                    id: todo!(),
                    position: todo!(),
                    index: todo!(),
                    is_emoji: todo!(),
                };
                let shaped_run = ShapedRun {
                    font_id: run.font_id,
                    glyphs: todo!(),
                };
            }
        }
        unimplemented!();
        LineLayout::default()
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

    pub fn get_result(&self) -> DWRITE_SCRIPT_ANALYSIS {
        self.script_analysis.unwrap()
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
    pub unsafe fn generate_result(&self, analyzer: &IDWriteTextAnalyzer) -> DWRITE_SCRIPT_ANALYSIS {
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
