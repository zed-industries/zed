use std::{arch::x86_64::CpuidResult, borrow::Cow, cell::Cell};

use anyhow::{anyhow, Result};
use collections::HashMap;
use itertools::Itertools;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use windows::{
    core::{implement, HSTRING, PCWSTR},
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
                            &self.components.factory,
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
                            &self.components.factory,
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
                let font = fontset
                    .GetMatchingFonts(
                        &HSTRING::from(target_font.family.to_string()),
                        DWRITE_FONT_WEIGHT(target_font.weight.0 as _),
                        DWRITE_FONT_STRETCH_NORMAL,
                        DWRITE_FONT_STYLE_NORMAL,
                    )
                    .unwrap();
                let total_number = font.GetFontCount();
                for _ in 0..total_number {
                    let font_id = FontId(self.fonts.len());
                    let font_face_ref = font.GetFontFaceReference(0).unwrap();
                    let Ok(font_face) = font_face_ref.CreateFontFace() else {
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
                let analysis = Analysis::new(
                    PCWSTR::from_raw(locale_wide.as_ptr()),
                    PCWSTR::from_raw(string[offset..(offset + run_len)].as_ptr()),
                    run_len as u32,
                );
                analysis.generate_result(&self.components.analyzer);
                // let cur_str = &string[offset..offset + run_len];
                // let mut res = std::mem::zeroed();
                // self.components.analyzer.AnalyzeScript(
                //     PCWSTR::from_raw(cur_str.as_ptr()),
                //     offset as _,
                //     run_len as _,
                //     &mut res,
                // );
                // self.components.analyzer.GetGlyphs(
                //     PCWSTR::from_raw(cur_str.as_ptr()),
                //     run_len as _,
                //     &font_info.font_face,
                //     false,
                //     false,
                //     None,
                //     localename,
                //     numbersubstitution,
                //     features,
                //     featurerangelengths,
                //     featureranges,
                //     maxglyphcount,
                //     clustermap,
                //     textprops,
                //     glyphindices,
                //     glyphprops,
                //     actualglyphcount,
                // );
                let collection = self
                    .components
                    .factory
                    .CreateFontCollectionFromFontSet(&self.font_sets[font_info.font_set_index])
                    .unwrap();
                let format = self
                    .components
                    .factory
                    .CreateTextFormat(
                        &HSTRING::from(&font_info.font_family),
                        &collection,
                        font_info.font_face.GetWeight(),
                        font_info.font_face.GetStyle(),
                        font_info.font_face.GetStretch(),
                        font_size.0,
                        &HSTRING::from(&self.components.locale),
                    )
                    .unwrap();
                let encoded_string = &string[offset..offset + run_len];
                let layout = self
                    .components
                    .factory
                    .CreateTextLayout(encoded_string, &format, f32::MAX, f32::MAX)
                    .unwrap();
                offset += run_len;
                let mut detail = std::mem::zeroed();
                layout.GetMetrics(&mut detail).unwrap();
                // let x = layout.SetFontStyle(fontstyle, textrange)
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

#[implement(IDWriteTextAnalysisSource, IDWriteTextAnalysisSink)]
struct Analysis {
    locale: PCWSTR,
    text: PCWSTR,
    text_length: u32,
    substitution: Option<IDWriteNumberSubstitution>,
    script_analysis: Cell<Option<DWRITE_SCRIPT_ANALYSIS>>,
}

impl Analysis {
    pub fn new(locale: PCWSTR, text: PCWSTR, text_length: u32) -> Self {
        Analysis {
            locale,
            text,
            text_length,
            substitution: None,
            script_analysis: Cell::new(None),
        }
    }

    pub unsafe fn generate_result(&self, analyzer: &IDWriteTextAnalyzer) -> DWRITE_SCRIPT_ANALYSIS {
        analyzer
            .AnalyzeScript(
                &self.cast::<IDWriteTextAnalysisSource>().unwrap(),
                0,
                self.text_length,
                &self.cast::<IDWriteTextAnalysisSink>().unwrap(),
            )
            .unwrap();
        self.script_analysis.get().unwrap()
    }
}

// https://github.com/microsoft/Windows-classic-samples/blob/main/Samples/Win7Samples/multimedia/DirectWrite/CustomLayout/TextAnalysis.cpp
impl IDWriteTextAnalysisSource_Impl for Analysis {
    fn GetTextAtPosition(
        &self,
        textposition: u32,
        textstring: *mut *mut u16,
        textlength: *mut u32,
    ) -> windows::core::Result<()> {
        if textposition >= self.text_length {
            unsafe {
                *textstring = 0 as _;
                *textlength = 0;
            }
        } else {
            unsafe {
                *textstring = self.text.as_wide()[textposition as usize..].as_ptr() as *mut u16;
                *textlength = self.text_length - textposition;
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
        if textposition == 0 || textposition >= self.text_length {
            unsafe {
                *textstring = 0 as _;
                *textlength = 0;
            }
        } else {
            unsafe {
                *textstring = self.text.as_ptr() as *mut u16;
                *textlength = textposition - 0;
            }
        }
        Ok(())
    }

    fn GetParagraphReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        DWRITE_READING_DIRECTION_LEFT_TO_RIGHT
    }

    fn GetLocaleName(
        &self,
        textposition: u32,
        textlength: *mut u32,
        localename: *mut *mut u16,
    ) -> windows::core::Result<()> {
        unsafe {
            *localename = self.locale.as_ptr() as *mut u16;
            *textlength = self.text_length - textposition;
        }
        Ok(())
    }

    fn GetNumberSubstitution(
        &self,
        textposition: u32,
        textlength: *mut u32,
        numbersubstitution: *mut Option<IDWriteNumberSubstitution>,
    ) -> windows::core::Result<()> {
        unsafe {
            *numbersubstitution = self.substitution.clone();
            *textlength = self.text_length - textposition;
        }
        Ok(())
    }
}

impl IDWriteTextAnalysisSink_Impl for Analysis {
    fn SetScriptAnalysis(
        &self,
        textposition: u32,
        textlength: u32,
        scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
    ) -> windows::core::Result<()> {
        unsafe {
            self.script_analysis.set(Some(*scriptanalysis));
        }
        Ok(())
    }

    fn SetLineBreakpoints(
        &self,
        textposition: u32,
        textlength: u32,
        linebreakpoints: *const DWRITE_LINE_BREAKPOINT,
    ) -> windows::core::Result<()> {
        todo!()
    }

    fn SetBidiLevel(
        &self,
        textposition: u32,
        textlength: u32,
        explicitlevel: u8,
        resolvedlevel: u8,
    ) -> windows::core::Result<()> {
        todo!()
    }

    fn SetNumberSubstitution(
        &self,
        textposition: u32,
        textlength: u32,
        numbersubstitution: Option<&IDWriteNumberSubstitution>,
    ) -> windows::core::Result<()> {
        todo!()
    }
}
