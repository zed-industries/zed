// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the content of dwrite_2.h
use ctypes::{c_void, wchar_t};
use shared::basetsd::{UINT16, UINT32, UINT8};
use shared::d3d9types::D3DCOLORVALUE;
use shared::minwindef::{BOOL, FLOAT};
use um::dcommon::DWRITE_MEASURING_MODE;
use um::dwrite::{
    DWRITE_FONT_FEATURE_TAG, DWRITE_FONT_STRETCH, DWRITE_FONT_STYLE, DWRITE_FONT_WEIGHT,
    DWRITE_GLYPH_RUN, DWRITE_GLYPH_RUN_DESCRIPTION, DWRITE_MATRIX, DWRITE_PIXEL_GEOMETRY,
    DWRITE_RENDERING_MODE, DWRITE_SCRIPT_ANALYSIS, DWRITE_STRIKETHROUGH, DWRITE_UNDERLINE,
    IDWriteFont, IDWriteFontCollection, IDWriteFontFace, IDWriteGlyphRunAnalysis,
    IDWriteInlineObject, IDWriteRenderingParams, IDWriteTextAnalysisSource, IDWriteTextFormat,
    IDWriteTextFormatVtbl, IDWriteTextRenderer, IDWriteTextRendererVtbl,
};
use um::dwrite_1::{
    DWRITE_GLYPH_ORIENTATION_ANGLE, DWRITE_OUTLINE_THRESHOLD, DWRITE_TEXT_ANTIALIAS_MODE,
    DWRITE_UNICODE_RANGE, DWRITE_VERTICAL_GLYPH_ORIENTATION, IDWriteFactory1,
    IDWriteFactory1Vtbl, IDWriteFont1, IDWriteFont1Vtbl, IDWriteFontFace1, IDWriteFontFace1Vtbl,
    IDWriteRenderingParams1, IDWriteRenderingParams1Vtbl, IDWriteTextAnalyzer1,
    IDWriteTextAnalyzer1Vtbl, IDWriteTextLayout1, IDWriteTextLayout1Vtbl,
};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, WCHAR};
ENUM!{enum DWRITE_OPTICAL_ALIGNMENT {
    DWRITE_OPTICAL_ALIGNMENT_NONE = 0x0, // 0
    DWRITE_OPTICAL_ALIGNMENT_NO_SIDE_BEARINGS = 0x1, // 1
}}
ENUM!{enum DWRITE_GRID_FIT_MODE {
    DWRITE_GRID_FIT_MODE_DEFAULT = 0x0, // 0
    DWRITE_GRID_FIT_MODE_DISABLED = 0x1, // 1
    DWRITE_GRID_FIT_MODE_ENABLED = 0x2, // 2
}}
STRUCT!{struct DWRITE_TEXT_METRICS1 {
    left: FLOAT,
    top: FLOAT,
    width: FLOAT,
    widthIncludingTrailingWhitespace: FLOAT,
    height: FLOAT,
    layoutWidth: FLOAT,
    layoutHeight: FLOAT,
    maxBidiReorderingDepth: UINT32,
    lineCount: UINT32,
    heightIncludingTrailingWhitespace: FLOAT,
}}
RIDL!{#[uuid(0xd3e0e934, 0x22a0, 0x427e, 0xaa, 0xe4, 0x7d, 0x95, 0x74, 0xb5, 0x9d, 0xb1)]
interface IDWriteTextRenderer1(IDWriteTextRenderer1Vtbl):
    IDWriteTextRenderer(IDWriteTextRendererVtbl) {
    fn DrawGlyphRun(
        clientDrawingContext: *mut c_void,
        baselineOriginX: FLOAT,
        baselineOriginY: FLOAT,
        orientationAngle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        measuringMode: DWRITE_MEASURING_MODE,
        glyphRun: *const DWRITE_GLYPH_RUN,
        glyphRunDescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        clientDrawingEffect: *mut IUnknown,
    ) -> HRESULT,
    fn DrawUnderline(
        clientDrawingContext: *mut c_void,
        baselineOriginX: FLOAT,
        baselineOriginY: FLOAT,
        orientationAngle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        underline: *const DWRITE_UNDERLINE,
        clientDrawingEffect: *mut IUnknown,
    ) -> HRESULT,
    fn DrawStrikethrough(
        clientDrawingContext: *mut c_void,
        baselineOriginX: FLOAT,
        baselineOriginY: FLOAT,
        orientationAngle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        strikethrough: *const DWRITE_STRIKETHROUGH,
        clientDrawingEffect: *mut IUnknown,
    ) -> HRESULT,
    fn DrawInlineObject(
        clientDrawingContext: *mut c_void,
        originX: FLOAT,
        originY: FLOAT,
        orientationAngle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        inlineObject: *mut IDWriteInlineObject,
        isSideways: BOOL,
        isRightToLeft: BOOL,
        clientDrawingEffect: *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5f174b49, 0x0d8b, 0x4cfb, 0x8b, 0xca, 0xf1, 0xcc, 0xe9, 0xd0, 0x6c, 0x67)]
interface IDWriteTextFormat1(IDWriteTextFormat1Vtbl):
    IDWriteTextFormat(IDWriteTextFormatVtbl) {
    fn SetVerticalGlyphOrientation(
        glyphOrientation: DWRITE_VERTICAL_GLYPH_ORIENTATION,
    ) -> HRESULT,
    fn GetVerticalGlyphOrientation() -> DWRITE_VERTICAL_GLYPH_ORIENTATION,
    fn SetLastLineWrapping(
        isLastLineWrappingEnabled: BOOL,
    ) -> HRESULT,
    fn GetLastLineWrapping() -> BOOL,
    fn SetOpticalAlignment(
        opticalAlignment: DWRITE_OPTICAL_ALIGNMENT,
    ) -> HRESULT,
    fn GetOpticalAlignment() -> DWRITE_OPTICAL_ALIGNMENT,
    fn SetFontFallback(
        fontFallback: *mut IDWriteFontFallback,
    ) -> HRESULT,
    fn GetFontFallback(
        fontFallback: *mut *mut IDWriteFontFallback,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1093c18f, 0x8d5e, 0x43f0, 0xb0, 0x64, 0x09, 0x17, 0x31, 0x1b, 0x52, 0x5e)]
interface IDWriteTextLayout2(IDWriteTextLayout2Vtbl):
    IDWriteTextLayout1(IDWriteTextLayout1Vtbl) {
    fn GetMetrics(
        textMetrics: *mut DWRITE_TEXT_METRICS1,
    ) -> HRESULT,
    fn SetVerticalGlyphOrientation(
        glyphOrientation: DWRITE_VERTICAL_GLYPH_ORIENTATION,
    ) -> HRESULT,
    fn GetVerticalGlyphOrientation() -> DWRITE_VERTICAL_GLYPH_ORIENTATION,
    fn SetLastLineWrapping(
        isLastLineWrappingEnabled: BOOL,
    ) -> HRESULT,
    fn GetLastLineWrapping() -> BOOL,
    fn SetOpticalAlignment(
        opticalAlignment: DWRITE_OPTICAL_ALIGNMENT,
    ) -> HRESULT,
    fn GetOpticalAlignment() -> DWRITE_OPTICAL_ALIGNMENT,
    fn SetFontFallback(
        fontFallback: *mut IDWriteFontFallback,
    ) -> HRESULT,
    fn GetFontFallback(
        fontFallback: *mut *mut IDWriteFontFallback,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x553a9ff3, 0x5693, 0x4df7, 0xb5, 0x2b, 0x74, 0x80, 0x6f, 0x7f, 0x2e, 0xb9)]
interface IDWriteTextAnalyzer2(IDWriteTextAnalyzer2Vtbl):
    IDWriteTextAnalyzer1(IDWriteTextAnalyzer1Vtbl) {
    fn GetGlyphOrientationTransform(
        glyphOrientationAngle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        isSideways: BOOL,
        originX: FLOAT,
        originY: FLOAT,
        transform: *mut DWRITE_MATRIX,
    ) -> HRESULT,
    fn GetTypographicFeatures(
        fontFace: *mut IDWriteFontFace,
        scriptAnalysis: DWRITE_SCRIPT_ANALYSIS,
        localeName: *const WCHAR,
        maxTagCount: UINT32,
        actualTagCount: *mut UINT32,
        tags: *mut DWRITE_FONT_FEATURE_TAG,
    ) -> HRESULT,
    fn CheckTypographicFeature(
        fontFace: *mut IDWriteFontFace,
        scriptAnalysis: DWRITE_SCRIPT_ANALYSIS,
        localeName: *const WCHAR,
        featureTag: DWRITE_FONT_FEATURE_TAG,
        glyphCount: UINT32,
        glyphIndices: *const UINT16,
        featureApplies: *mut UINT8,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xefa008f9, 0xf7a1, 0x48bf, 0xb0, 0x5c, 0xf2, 0x24, 0x71, 0x3c, 0xc0, 0xff)]
interface IDWriteFontFallback(IDWriteFontFallbackVtbl): IUnknown(IUnknownVtbl) {
    fn MapCharacters(
        analysisSource: *mut IDWriteTextAnalysisSource,
        textPosition: UINT32,
        textLength: UINT32,
        baseFontCollection: *mut IDWriteFontCollection,
        baseFamilyName: *mut wchar_t,
        baseWeight: DWRITE_FONT_WEIGHT,
        baseStyle: DWRITE_FONT_STYLE,
        baseStretch: DWRITE_FONT_STRETCH,
        mappedLength: *mut UINT32,
        mappedFont: *mut *mut IDWriteFont,
        scale: *mut FLOAT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xfd882d06, 0x8aba, 0x4fb8, 0xb8, 0x49, 0x8b, 0xe8, 0xb7, 0x3e, 0x14, 0xde)]
interface IDWriteFontFallbackBuilder(IDWriteFontFallbackBuilderVtbl):
    IUnknown(IUnknownVtbl) {
    fn AddMapping(
        ranges: *const DWRITE_UNICODE_RANGE,
        rangesCount: UINT32,
        targetFamilyNames: *mut *const WCHAR,
        targetFamilyNamesCount: UINT32,
        fontCollection: *mut IDWriteFontCollection,
        localeName: *const WCHAR,
        baseFamilyName: *const WCHAR,
        scale: FLOAT,
    ) -> HRESULT,
    fn AddMappings(
        fontFallback: *mut IDWriteFontFallback,
    ) -> HRESULT,
    fn CreateFontFallback(
        fontFallback: *mut *mut IDWriteFontFallback,
    ) -> HRESULT,
}}
pub type DWRITE_COLOR_F = D3DCOLORVALUE;
RIDL!{#[uuid(0x29748ed6, 0x8c9c, 0x4a6a, 0xbe, 0x0b, 0xd9, 0x12, 0xe8, 0x53, 0x89, 0x44)]
interface IDWriteFont2(IDWriteFont2Vtbl): IDWriteFont1(IDWriteFont1Vtbl) {
    fn IsColorFont() -> BOOL,
}}
RIDL!{#[uuid(0xd8b768ff, 0x64bc, 0x4e66, 0x98, 0x2b, 0xec, 0x8e, 0x87, 0xf6, 0x93, 0xf7)]
interface IDWriteFontFace2(IDWriteFontFace2Vtbl):
    IDWriteFontFace1(IDWriteFontFace1Vtbl) {
    fn IsColorFont() -> BOOL,
    fn GetColorPaletteCount() -> UINT32,
    fn GetPaletteEntryCount() -> UINT32,
    fn GetPaletteEntries(
        colorPaletteIndex: UINT32,
        firstEntryIndex: UINT32,
        entryCount: UINT32,
        paletteEntries: *mut DWRITE_COLOR_F,
    ) -> HRESULT,
    fn GetRecommendedRenderingMode(
        fontEmSize: FLOAT,
        dpiX: FLOAT,
        dpiY: FLOAT,
        transform: *const DWRITE_MATRIX,
        isSideways: BOOL,
        outlineThreshold: DWRITE_OUTLINE_THRESHOLD,
        measuringMode: DWRITE_MEASURING_MODE,
        renderingParams: *mut IDWriteRenderingParams,
        renderingMode: *mut DWRITE_RENDERING_MODE,
        gridFitMode: *mut DWRITE_GRID_FIT_MODE,
    ) -> HRESULT,
}}
STRUCT!{struct DWRITE_COLOR_GLYPH_RUN {
    glyphRun: DWRITE_GLYPH_RUN,
    glyphRunDescription: *mut DWRITE_GLYPH_RUN_DESCRIPTION,
    baselineOriginX: FLOAT,
    baselineOriginY: FLOAT,
    runColor: DWRITE_COLOR_F,
    paletteIndex: UINT16,
}}
RIDL!{#[uuid(0xd31fbe17, 0xf157, 0x41a2, 0x8d, 0x24, 0xcb, 0x77, 0x9e, 0x05, 0x60, 0xe8)]
interface IDWriteColorGlyphRunEnumerator(IDWriteColorGlyphRunEnumeratorVtbl):
    IUnknown(IUnknownVtbl) {
    fn MoveNext(
        hasRun: *mut BOOL,
    ) -> HRESULT,
    fn GetCurrentRun(
        colorGlyphRun: *mut *const DWRITE_COLOR_GLYPH_RUN,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf9d711c3, 0x9777, 0x40ae, 0x87, 0xe8, 0x3e, 0x5a, 0xf9, 0xbf, 0x09, 0x48)]
interface IDWriteRenderingParams2(IDWriteRenderingParams2Vtbl):
    IDWriteRenderingParams1(IDWriteRenderingParams1Vtbl) {
    fn GetGridFitMode() -> DWRITE_GRID_FIT_MODE,
}}
RIDL!{#[uuid(0x0439fc60, 0xca44, 0x4994, 0x8d, 0xee, 0x3a, 0x9a, 0xf7, 0xb7, 0x32, 0xec)]
interface IDWriteFactory2(IDWriteFactory2Vtbl): IDWriteFactory1(IDWriteFactory1Vtbl) {
    fn GetSystemFontFallback(
        fontFallback: *mut *mut IDWriteFontFallback,
    ) -> HRESULT,
    fn CreateFontFallbackBuilder(
        fontFallbackBuilder: *mut *mut IDWriteFontFallbackBuilder,
    ) -> HRESULT,
    fn TranslateColorGlyphRun(
        baselineOriginX: FLOAT,
        baselineOriginY: FLOAT,
        glyphRun: *const DWRITE_GLYPH_RUN,
        glyphRunDescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        measuringMode: DWRITE_MEASURING_MODE,
        worldToDeviceTransform: *const DWRITE_MATRIX,
        colorPaletteIndex: UINT32,
        colorLayers: *mut *mut IDWriteColorGlyphRunEnumerator,
    ) -> HRESULT,
    fn CreateCustomRenderingParams(
        gamma: FLOAT,
        enhancedContrast: FLOAT,
        grayscaleEnhancedContrast: FLOAT,
        clearTypeLevel: FLOAT,
        pixelGeometry: DWRITE_PIXEL_GEOMETRY,
        renderingMode: DWRITE_RENDERING_MODE,
        gridFitMode: DWRITE_GRID_FIT_MODE,
        renderingParams: *mut *mut IDWriteRenderingParams2,
    ) -> HRESULT,
    fn CreateGlyphRunAnalysis(
        glyphRun: *const DWRITE_GLYPH_RUN,
        transform: *const DWRITE_MATRIX,
        renderingMode: DWRITE_RENDERING_MODE,
        measuringMode: DWRITE_MEASURING_MODE,
        gridFitMode: DWRITE_GRID_FIT_MODE,
        antialiasMode: DWRITE_TEXT_ANTIALIAS_MODE,
        baselineOriginX: FLOAT,
        baselineOriginY: FLOAT,
        glyphRunAnalysis: *mut *mut IDWriteGlyphRunAnalysis,
    ) -> HRESULT,
}}
