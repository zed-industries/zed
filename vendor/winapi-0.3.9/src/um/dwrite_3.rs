// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the content of dwrite_3.h
use ctypes::c_void;
use shared::basetsd::{UINT16, UINT32, UINT64};
use shared::minwindef::{BOOL, FILETIME, FLOAT};
use um::dcommon::{DWRITE_GLYPH_IMAGE_DATA, DWRITE_GLYPH_IMAGE_FORMATS, DWRITE_MEASURING_MODE};
use um::dwrite::{
    DWRITE_FONT_SIMULATIONS, DWRITE_FONT_STRETCH, DWRITE_FONT_STYLE, DWRITE_FONT_WEIGHT,
    DWRITE_GLYPH_RUN, DWRITE_INFORMATIONAL_STRING_ID, DWRITE_LINE_SPACING_METHOD, DWRITE_MATRIX,
    DWRITE_PIXEL_GEOMETRY, DWRITE_RENDERING_MODE_ALIASED, DWRITE_RENDERING_MODE_DEFAULT,
    DWRITE_RENDERING_MODE_GDI_CLASSIC, DWRITE_RENDERING_MODE_GDI_NATURAL,
    DWRITE_RENDERING_MODE_NATURAL, DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC,
    DWRITE_RENDERING_MODE_OUTLINE, IDWriteFont, IDWriteFontCollection, IDWriteFontCollectionVtbl,
    IDWriteFontFace, IDWriteFontFamily, IDWriteFontFamilyVtbl, IDWriteFontFile, IDWriteFontList,
    IDWriteFontListVtbl, IDWriteGdiInterop, IDWriteGdiInteropVtbl, IDWriteGlyphRunAnalysis,
    IDWriteLocalizedStrings, IDWriteRenderingParams,
};
use um::dwrite_1::{DWRITE_OUTLINE_THRESHOLD, DWRITE_PANOSE, DWRITE_TEXT_ANTIALIAS_MODE};
use um::dwrite_2::{
    DWRITE_GRID_FIT_MODE, IDWriteFactory2, IDWriteFactory2Vtbl, IDWriteFont2, IDWriteFont2Vtbl,
    IDWriteFontFace2, IDWriteFontFace2Vtbl, IDWriteRenderingParams2, IDWriteRenderingParams2Vtbl,
    IDWriteTextFormat1, IDWriteTextFormat1Vtbl, IDWriteTextLayout2, IDWriteTextLayout2Vtbl,
};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wingdi::{FONTSIGNATURE, LOGFONTW};
use um::winnt::{HRESULT, WCHAR};
pub const DWRITE_E_REMOTEFONT: HRESULT = 0x8898500D;
pub const DWRITE_E_DOWNLOADCANCELLED: HRESULT = 0x8898500E;
pub const DWRITE_E_DOWNLOADFAILED: HRESULT = 0x8898500F;
pub const DWRITE_E_TOOMANYDOWNLOADS: HRESULT = 0x88985010;
ENUM!{enum DWRITE_FONT_PROPERTY_ID {
    DWRITE_FONT_PROPERTY_ID_NONE,
    DWRITE_FONT_PROPERTY_ID_FAMILY_NAME,
    DWRITE_FONT_PROPERTY_ID_PREFERRED_FAMILY_NAME,
    DWRITE_FONT_PROPERTY_ID_FACE_NAME,
    DWRITE_FONT_PROPERTY_ID_FULL_NAME,
    DWRITE_FONT_PROPERTY_ID_WIN32_FAMILY_NAME,
    DWRITE_FONT_PROPERTY_ID_POSTSCRIPT_NAME,
    DWRITE_FONT_PROPERTY_ID_DESIGN_SCRIPT_LANGUAGE_TAG,
    DWRITE_FONT_PROPERTY_ID_SUPPORTED_SCRIPT_LANGUAGE_TAG,
    DWRITE_FONT_PROPERTY_ID_SEMANTIC_TAG,
    DWRITE_FONT_PROPERTY_ID_WEIGHT ,
    DWRITE_FONT_PROPERTY_ID_STRETCH,
    DWRITE_FONT_PROPERTY_ID_STYLE,
    DWRITE_FONT_PROPERTY_ID_TOTAL,
}}
STRUCT!{struct DWRITE_FONT_PROPERTY {
    propertyId: DWRITE_FONT_PROPERTY_ID,
    propertyValue: *const WCHAR,
    localeName: *const WCHAR,
}}
ENUM!{enum DWRITE_LOCALITY {
    DWRITE_LOCALITY_REMOTE,
    DWRITE_LOCALITY_PARTIAL,
    DWRITE_LOCALITY_LOCAL,
}}
ENUM!{enum DWRITE_RENDERING_MODE1 {
    DWRITE_RENDERING_MODE1_DEFAULT = DWRITE_RENDERING_MODE_DEFAULT,
    DWRITE_RENDERING_MODE1_ALIASED = DWRITE_RENDERING_MODE_ALIASED,
    DWRITE_RENDERING_MODE1_GDI_CLASSIC = DWRITE_RENDERING_MODE_GDI_CLASSIC,
    DWRITE_RENDERING_MODE1_GDI_NATURAL = DWRITE_RENDERING_MODE_GDI_NATURAL,
    DWRITE_RENDERING_MODE1_NATURAL = DWRITE_RENDERING_MODE_NATURAL,
    DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC = DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC,
    DWRITE_RENDERING_MODE1_OUTLINE = DWRITE_RENDERING_MODE_OUTLINE,
    DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC_DOWNSAMPLED,
}}
RIDL!{#[uuid(0xb7924baa, 0x391b, 0x412a, 0x8c, 0x5c, 0xe4, 0x4c, 0xc2, 0xd8, 0x67, 0xdc)]
interface IDWriteRenderingParams3(IDWriteRenderingParams3Vtbl):
    IDWriteRenderingParams2(IDWriteRenderingParams2Vtbl) {
    fn GetRenderingMode1() -> DWRITE_RENDERING_MODE1,
}}
RIDL!{#[uuid(0x9a1b41c3, 0xd3bb, 0x466a, 0x87, 0xfc, 0xfe, 0x67, 0x55, 0x6a, 0x3b, 0x65)]
interface IDWriteFactory3(IDWriteFactory3Vtbl): IDWriteFactory2(IDWriteFactory2Vtbl) {
    fn CreateGlyphRunAnalysis(
        glyphRun: *const DWRITE_GLYPH_RUN,
        transform: *const DWRITE_MATRIX,
        renderingMode: DWRITE_RENDERING_MODE1,
        measuringMode: DWRITE_MEASURING_MODE,
        gridFitMode: DWRITE_GRID_FIT_MODE,
        antialiasMode: DWRITE_TEXT_ANTIALIAS_MODE,
        baselineOriginX: FLOAT,
        baselineOriginY: FLOAT,
        glyphRunAnalysis: *mut *mut IDWriteGlyphRunAnalysis,
    ) -> HRESULT,
    fn CreateCustomRenderingParams(
        gamma: FLOAT,
        enhancedContrast: FLOAT,
        grayscaleEnhancedContrast: FLOAT,
        clearTypeLevel: FLOAT,
        pixelGeometry: DWRITE_PIXEL_GEOMETRY,
        renderingMode: DWRITE_RENDERING_MODE1,
        gridFitMode: DWRITE_GRID_FIT_MODE,
        renderingParams: *mut *mut IDWriteRenderingParams3,
    ) -> HRESULT,
    fn CreateFontFaceReference_2(
        fontFile: *mut IDWriteFontFile,
        faceIndex: UINT32,
        fontSimulations: DWRITE_FONT_SIMULATIONS,
        fontFaceReference: *mut *mut IDWriteFontFaceReference,
    ) -> HRESULT,
    fn CreateFontFaceReference_1(
        filePath: *const WCHAR,
        lastWriteTime: *const FILETIME,
        faceIndex: UINT32,
        fontSimulations: DWRITE_FONT_SIMULATIONS,
        fontFaceReference: *mut *mut IDWriteFontFaceReference,
    ) -> HRESULT,
    fn GetSystemFontSet(
        fontSet: *mut *mut IDWriteFontSet,
    ) -> HRESULT,
    fn CreateFontSetBuilder(
        fontSetBuilder: *mut *mut IDWriteFontSetBuilder,
    ) -> HRESULT,
    fn CreateFontCollectionFromFontSet(
        fontSet: *mut IDWriteFontSet,
        fontCollection: *mut *mut IDWriteFontCollection1,
    ) -> HRESULT,
    fn GetSystemFontCollection(
        includeDownloadableFonts: BOOL,
        fontCollection: *mut *mut IDWriteFontCollection1,
        checkForUpdates: BOOL,
    ) -> HRESULT,
    fn GetFontDownloadQueue(
        fontDownloadQueue: *mut *mut IDWriteFontDownloadQueue,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x53585141, 0xd9f8, 0x4095, 0x83, 0x21, 0xd7, 0x3c, 0xf6, 0xbd, 0x11, 0x6b)]
interface IDWriteFontSet(IDWriteFontSetVtbl): IUnknown(IUnknownVtbl) {
    fn GetFontCount() -> UINT32,
    fn GetFontFaceReference(
        listIndex: UINT32,
        fontFaceReference: *mut *mut IDWriteFontFaceReference,
    ) -> HRESULT,
    fn FindFontFaceReference(
        fontFaceReference: *mut IDWriteFontFaceReference,
        listIndex: *mut UINT32,
        exists: *mut BOOL,
    ) -> HRESULT,
    fn FindFontFace(
        fontFace: *mut IDWriteFontFace,
        listIndex: *mut UINT32,
        exists: *mut BOOL,
    ) -> HRESULT,
    fn GetPropertyValues_3(
        propertyID: DWRITE_FONT_PROPERTY_ID,
        values: *mut *mut IDWriteStringList,
    ) -> HRESULT,
    fn GetPropertyValues_2(
        propertyID: DWRITE_FONT_PROPERTY_ID,
        preferredLocaleNames: *const WCHAR,
        values: *mut *mut IDWriteStringList,
    ) -> HRESULT,
    fn GetPropertyValues_1(
        listIndex: UINT32,
        propertyId: DWRITE_FONT_PROPERTY_ID,
        exists: *mut BOOL,
        values: *mut *mut IDWriteLocalizedStrings,
    ) -> HRESULT,
    fn GetPropertyOccurrenceCount(
        property: *const DWRITE_FONT_PROPERTY,
        propertyOccurrenceCount: *mut UINT32,
    ) -> HRESULT,
    fn GetMatchingFonts_2(
        familyName: *const WCHAR,
        fontWeight: DWRITE_FONT_WEIGHT,
        fontStretch: DWRITE_FONT_STRETCH,
        fontStyle: DWRITE_FONT_STYLE,
        filteredSet: *mut *mut IDWriteFontSet,
    ) -> HRESULT,
    fn GetMatchingFonts_1(
        properties: *const DWRITE_FONT_PROPERTY,
        propertyCount: UINT32,
        filteredSet: *mut *mut IDWriteFontSet,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2f642afe, 0x9c68, 0x4f40, 0xb8, 0xbe, 0x45, 0x74, 0x01, 0xaf, 0xcb, 0x3d)]
interface IDWriteFontSetBuilder(IDWriteFontSetBuilderVtbl): IUnknown(IUnknownVtbl) {
    fn AddFontFaceReference_2(
        fontFaceReference: *mut IDWriteFontFaceReference,
        properties: *const DWRITE_FONT_PROPERTY,
        propertyCount: UINT32,
    ) -> HRESULT,
    fn AddFontFaceReference_1(
        fontFaceReference: *mut IDWriteFontFaceReference,
    ) -> HRESULT,
    fn AddFontSet(
        fontSet: *mut IDWriteFontSet,
    ) -> HRESULT,
    fn CreateFontSet(
        fontSet: *mut *mut IDWriteFontSet,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x53585141, 0xd9f8, 0x4095, 0x83, 0x21, 0xd7, 0x3c, 0xf6, 0xbd, 0x11, 0x6c)]
interface IDWriteFontCollection1(IDWriteFontCollection1Vtbl):
    IDWriteFontCollection(IDWriteFontCollectionVtbl) {
    fn GetFontSet(
        fontSet: *mut *mut IDWriteFontSet,
    ) -> HRESULT,
    fn GetFontFamily(
        index: UINT32,
        fontFamily: *mut *mut IDWriteFontFamily1,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xda20d8ef, 0x812a, 0x4c43, 0x98, 0x02, 0x62, 0xec, 0x4a, 0xbd, 0x7a, 0xdf)]
interface IDWriteFontFamily1(IDWriteFontFamily1Vtbl):
    IDWriteFontFamily(IDWriteFontFamilyVtbl) {
    fn GetFontLocality(
        listIndex: UINT32,
    ) -> DWRITE_LOCALITY,
    fn GetFont(
        listIndex: UINT32,
        font: *mut *mut IDWriteFont3,
    ) -> HRESULT,
    fn GetFontFaceReference(
        listIndex: UINT32,
        fontFaceReference: *mut *mut IDWriteFontFaceReference,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xda20d8ef, 0x812a, 0x4c43, 0x98, 0x02, 0x62, 0xec, 0x4a, 0xbd, 0x7a, 0xde)]
interface IDWriteFontList1(IDWriteFontList1Vtbl): IDWriteFontList(IDWriteFontListVtbl) {
    fn GetFontLocality(
        listIndex: UINT32,
    ) -> DWRITE_LOCALITY,
    fn GetFont(
        listIndex: UINT32,
        font: *mut *mut IDWriteFont3,
    ) -> HRESULT,
    fn GetFontFaceReference(
        listIndex: UINT32,
        fontFaceReference: *mut *mut IDWriteFontFaceReference,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5e7fa7ca, 0xdde3, 0x424c, 0x89, 0xf0, 0x9f, 0xcd, 0x6f, 0xed, 0x58, 0xcd)]
interface IDWriteFontFaceReference(IDWriteFontFaceReferenceVtbl):
    IUnknown(IUnknownVtbl) {
    fn CreateFontFace(
        fontFace: *mut *mut IDWriteFontFace3,
    ) -> HRESULT,
    fn CreateFontFaceWithSimulations(
        fontFaceSimulationFlags: DWRITE_FONT_SIMULATIONS,
        fontFace: *mut *mut IDWriteFontFace3,
    ) -> HRESULT,
    fn Equals(
        fontFaceReference: *mut IDWriteFontFaceReference,
    ) -> BOOL,
    fn GetFontFaceIndex() -> UINT32,
    fn GetSimulations() -> DWRITE_FONT_SIMULATIONS,
    fn GetFontFile(
        fontFile: *mut *mut IDWriteFontFile,
    ) -> HRESULT,
    fn GetLocalFileSize() -> UINT64,
    fn GetFileSize() -> UINT64,
    fn GetFileTime(
        lastWriteTime: *mut FILETIME,
    ) -> HRESULT,
    fn GetLocality() -> DWRITE_LOCALITY,
    fn EnqueueFontDownloadRequest() -> HRESULT,
    fn EnqueueCharacterDownloadRequest(
        characters: *const WCHAR,
        characterCount: UINT32,
    ) -> HRESULT,
    fn EnqueueGlyphDownloadRequest(
        glyphIndices: *const UINT16,
        glyphCount: UINT32,
    ) -> HRESULT,
    fn EnqueueFileFragmentDownloadRequest(
        fileOffset: UINT64,
        fragmentSize: UINT64,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x29748ed6, 0x8c9c, 0x4a6a, 0xbe, 0x0b, 0xd9, 0x12, 0xe8, 0x53, 0x89, 0x44)]
interface IDWriteFont3(IDWriteFont3Vtbl): IDWriteFont2(IDWriteFont2Vtbl) {
    fn CreateFontFace(
        fontFace: *mut *mut IDWriteFontFace3,
    ) -> HRESULT,
    fn Equals(
        font: *mut IDWriteFont,
    ) -> BOOL,
    fn GetFontFaceReference(
        fontFaceReference: *mut *mut IDWriteFontFaceReference,
    ) -> HRESULT,
    fn HasCharacter(
        unicodeValue: UINT32,
    ) -> BOOL,
    fn GetLocality() -> DWRITE_LOCALITY,
}}
RIDL!{#[uuid(0xd37d7598, 0x09be, 0x4222, 0xa2, 0x36, 0x20, 0x81, 0x34, 0x1c, 0xc1, 0xf2)]
interface IDWriteFontFace3(IDWriteFontFace3Vtbl):
    IDWriteFontFace2(IDWriteFontFace2Vtbl) {
    fn GetFontFaceReference(
        fontFaceReference: *mut *mut IDWriteFontFaceReference,
    ) -> HRESULT,
    fn GetPanose(
        panose: *mut DWRITE_PANOSE,
    ) -> (),
    fn GetWeight() -> DWRITE_FONT_WEIGHT,
    fn GetStretch() -> DWRITE_FONT_STRETCH,
    fn GetStyle() -> DWRITE_FONT_STYLE,
    fn GetFamilyNames(
        names: *mut *mut IDWriteLocalizedStrings,
    ) -> HRESULT,
    fn GetFaceNames(
        names: *mut *mut IDWriteLocalizedStrings,
    ) -> HRESULT,
    fn GetInformationalStrings(
        informationalStringID: DWRITE_INFORMATIONAL_STRING_ID,
        informationalStrings: *mut *mut IDWriteLocalizedStrings,
        exists: *mut BOOL,
    ) -> HRESULT,
    fn HasCharacter(
        unicodeValue: UINT32,
    ) -> BOOL,
    fn GetRecommendedRenderingMode(
        fontEmSize: FLOAT,
        dpiX: FLOAT,
        dpiY: FLOAT,
        transform: *const DWRITE_MATRIX,
        isSideways: BOOL,
        outlineThreshold: DWRITE_OUTLINE_THRESHOLD,
        measuringMode: DWRITE_MEASURING_MODE,
        renderingParams: *mut IDWriteRenderingParams,
        renderingMode: *mut DWRITE_RENDERING_MODE1,
        gridFitMode: *mut DWRITE_GRID_FIT_MODE,
    ) -> HRESULT,
    fn IsCharacterLocal(
        unicodeValue: UINT32,
    ) -> BOOL,
    fn IsGlyphLocal(
        glyphId: UINT16,
    ) -> BOOL,
    fn AreCharactersLocal(
        characters: *const WCHAR,
        characterCount: UINT32,
        enqueueIfNotLocal: BOOL,
        isLocal: *mut BOOL,
    ) -> HRESULT,
    fn AreGlyphsLocal(
        glyphIndices: *const UINT16,
        glyphCount: UINT32,
        enqueueIfNotLocal: BOOL,
        isLocal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xcfee3140, 0x1157, 0x47ca, 0x8b, 0x85, 0x31, 0xbf, 0xcf, 0x3f, 0x2d, 0x0e)]
interface IDWriteStringList(IDWriteStringListVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount() -> UINT32,
    fn GetLocaleNameLength(
        listIndex: UINT32,
        length: *mut UINT32,
    ) -> HRESULT,
    fn GetLocaleName(
        listIndex: UINT32,
        localeName: *mut WCHAR,
        size: UINT32,
    ) -> HRESULT,
    fn GetStringLength(
        listIndex: UINT32,
        length: *mut UINT32,
    ) -> HRESULT,
    fn GetString(
        listIndex: UINT32,
        stringBuffer: *mut WCHAR,
        stringBufferSize: UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb06fe5b9, 0x43ec, 0x4393, 0x88, 0x1b, 0xdb, 0xe4, 0xdc, 0x72, 0xfd, 0xa7)]
interface IDWriteFontDownloadListener(IDWriteFontDownloadListenerVtbl):
    IUnknown(IUnknownVtbl) {
    fn DownloadCompleted(
        downloadQueue: *mut IDWriteFontDownloadQueue,
        context: *mut IUnknown,
        downloadResult: HRESULT,
    ) -> (),
}}
RIDL!{#[uuid(0xb71e6052, 0x5aea, 0x4fa3, 0x83, 0x2e, 0xf6, 0x0d, 0x43, 0x1f, 0x7e, 0x91)]
interface IDWriteFontDownloadQueue(IDWriteFontDownloadQueueVtbl):
    IUnknown(IUnknownVtbl) {
    fn AddListener(
        listener: *mut IDWriteFontDownloadListener,
        token: *mut UINT32,
    ) -> HRESULT,
    fn RemoveListener(
        token: UINT32,
    ) -> HRESULT,
    fn IsEmpty() -> BOOL,
    fn BeginDownload(
        context: *mut IUnknown,
    ) -> HRESULT,
    fn CancelDownload() -> HRESULT,
    fn GetGenerationCount() -> UINT64,
}}
RIDL!{#[uuid(0x4556be70, 0x3abd, 0x4f70, 0x90, 0xbe, 0x42, 0x17, 0x80, 0xa6, 0xf5, 0x15)]
interface IDWriteGdiInterop1(IDWriteGdiInterop1Vtbl):
    IDWriteGdiInterop(IDWriteGdiInteropVtbl) {
    fn CreateFontFromLOGFONT(
        logFont: *const LOGFONTW,
        fontCollection: *mut IDWriteFontCollection,
        font: *mut *mut IDWriteFont,
    ) -> HRESULT,
    fn GetFontSignature_2(
        fontFace: *mut IDWriteFontFace,
        fontSignature: *mut FONTSIGNATURE,
    ) -> HRESULT,
    fn GetFontSignature_1(
        font: *mut IDWriteFont,
        fontSignature: *mut FONTSIGNATURE,
    ) -> HRESULT,
    fn GetMatchingFontsByLOGFONT(
        logFont: *const LOGFONTW,
        fontSet: *mut IDWriteFontSet,
        filteredSet: *mut *mut IDWriteFontSet,
    ) -> HRESULT,
}}
STRUCT!{struct DWRITE_LINE_METRICS1 {
    length: UINT32,
    trailingWhitespaceLength: UINT32,
    newlineLength: UINT32,
    height: FLOAT,
    baseline: FLOAT,
    isTrimmed: BOOL,
    leadingBefore: FLOAT,
    leadingAfter: FLOAT,
}}
ENUM!{enum DWRITE_FONT_LINE_GAP_USAGE {
    DWRITE_FONT_LINE_GAP_USAGE_DEFAULT,
    DWRITE_FONT_LINE_GAP_USAGE_DISABLED,
    DWRITE_FONT_LINE_GAP_USAGE_ENABLED,
}}
STRUCT!{struct DWRITE_LINE_SPACING {
    method: DWRITE_LINE_SPACING_METHOD,
    height: FLOAT,
    baseline: FLOAT,
    leadingBefore: FLOAT,
    fontLineGapUsage: DWRITE_FONT_LINE_GAP_USAGE,
}}
RIDL!{#[uuid(0xf67e0edd, 0x9e3d, 0x4ecc, 0x8c, 0x32, 0x41, 0x83, 0x25, 0x3d, 0xfe, 0x70)]
interface IDWriteTextFormat2(IDWriteTextFormat2Vtbl):
    IDWriteTextFormat1(IDWriteTextFormat1Vtbl) {
    fn SetLineSpacing(
        lineSpacingOptions: *const DWRITE_LINE_SPACING,
    ) -> HRESULT,
    fn GetLineSpacing(
        lineSpacingOptions: *mut DWRITE_LINE_SPACING,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x07ddcd52, 0x020e, 0x4de8, 0xac, 0x33, 0x6c, 0x95, 0x3d, 0x83, 0xf9, 0x2d)]
interface IDWriteTextLayout3(IDWriteTextLayout3Vtbl):
    IDWriteTextLayout2(IDWriteTextLayout2Vtbl) {
    fn InvalidateLayout() -> HRESULT,
    fn SetLineSpacing(
        lineSpacingOptions: *const DWRITE_LINE_SPACING,
    ) -> HRESULT,
    fn GetLineSpacing(
        lineSpacingOptions: *mut DWRITE_LINE_SPACING,
    ) -> HRESULT,
    fn GetLineMetrics(
        lineMetrics: *mut DWRITE_LINE_METRICS1,
        maxLineCount: UINT32,
        actualLineCount: *mut UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x27f2a904, 0x4eb8, 0x441d, 0x96, 0x78, 0x05, 0x63, 0xf5, 0x3e, 0x3e, 0x2f)]
interface IDWriteFontFace4(IDWriteFontFace4Vtbl): IDWriteFontFace3(IDWriteFontFace3Vtbl) {
    fn GetGlyphImageFormats_2(
        glyph: UINT16,
        ppemFirst: UINT32,
        ppemLast: UINT32,
        formats: *mut DWRITE_GLYPH_IMAGE_FORMATS,
    ) -> HRESULT,
    fn GetGlyphImageFormats_1() -> DWRITE_GLYPH_IMAGE_FORMATS,
    fn GetGlyphImageData(
        glyph: UINT16,
        ppem: UINT32,
        format: DWRITE_GLYPH_IMAGE_FORMATS,
        data: *mut DWRITE_GLYPH_IMAGE_DATA,
        context: *mut *mut c_void,
    ) -> HRESULT,
    fn ReleaseGlyphImageData(
        context: *mut c_void,
    ) -> (),
}}
ENUM!{enum DWRITE_FONT_AXIS_TAG {
    DWRITE_FONT_AXIS_TAG_WEIGHT = 0x74686777,
    DWRITE_FONT_AXIS_TAG_WIDTH = 0x68746477,
    DWRITE_FONT_AXIS_TAG_SLANT = 0x746e6c73,
    DWRITE_FONT_AXIS_TAG_OPTICAL_SIZE = 0x7a73706f,
    DWRITE_FONT_AXIS_TAG_ITALIC = 0x6c617469,
}}
STRUCT!{struct DWRITE_FONT_AXIS_VALUE {
    axisTag: DWRITE_FONT_AXIS_TAG,
    value: FLOAT,
}}
STRUCT!{struct DWRITE_FONT_AXIS_RANGE {
    axisTag: DWRITE_FONT_AXIS_TAG,
    minValue: FLOAT,
    maxValue: FLOAT,
}}
ENUM!{enum DWRITE_FONT_AXIS_ATTRIBUTES {
    DWRITE_FONT_AXIS_ATTRIBUTES_NONE,
    DWRITE_FONT_AXIS_ATTRIBUTES_VARIABLE,
    DWRITE_FONT_AXIS_ATTRIBUTES_HIDDEN,
}}
RIDL!{#[uuid(0x98eff3a5, 0xb667, 0x479a, 0xb1, 0x45, 0xe2, 0xfa, 0x5b, 0x9f, 0xdc, 0x29)]
interface IDWriteFontFace5(IDWriteFontFace5Vtbl): IDWriteFontFace4(IDWriteFontFace4Vtbl) {
    fn GetFontAxisValueCount() -> UINT32,
    fn GetFontAxisValues(
        values: *mut DWRITE_FONT_AXIS_VALUE,
        valueCount: UINT32,
    ) -> HRESULT,
    fn HasVariations() -> BOOL,
    fn GetFontResource(
        resource: *mut *mut IDWriteFontResource,
    ) -> HRESULT,
    fn Equals(
        fontFace: *mut IDWriteFontFace,
    ) -> BOOL,
}}
RIDL!{#[uuid(0xc081fe77, 0x2fd1, 0x41ac, 0xa5, 0xa3, 0x34, 0x98, 0x3c, 0x4b, 0xa6, 0x1a)]
interface IDWriteFontFaceReference1(IDWriteFontFaceReference1Vtbl):
    IDWriteFontFaceReference(IDWriteFontFaceReferenceVtbl) {
    fn CreateFontFace(
        fontFace: *mut *mut IDWriteFontFace5,
    ) -> HRESULT,
    fn GetFontAxisValueCount() -> UINT32,
    fn GetFontAxisValues(
        values: *mut DWRITE_FONT_AXIS_VALUE,
        numValues: UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1f803a76, 0x6871, 0x48e8, 0x98, 0x7f, 0xb9, 0x75, 0x55, 0x1c, 0x50, 0xf2)]
interface IDWriteFontResource(IDWriteFontResourceVtbl): IUnknown(IUnknownVtbl) {
    fn GetFontFile(
        fontFile: *mut *mut IDWriteFontFile,
    ) -> HRESULT,
    fn GetFontFaceIndex() -> UINT32,
    fn GetFontAxisCount() -> UINT32,
    fn GetDefaultFontAxisValues(
        values: *const DWRITE_FONT_AXIS_VALUE,
        numValues: UINT32,
    ) -> HRESULT,
    fn GetFontAxisRanges(
        ranges: *const DWRITE_FONT_AXIS_RANGE,
        numRanges: UINT32,
    ) -> HRESULT,
    fn GetFontAxisAttributes(
        axis: UINT32,
    ) -> DWRITE_FONT_AXIS_ATTRIBUTES,
    fn GetAxisNames(
        axis: UINT32,
        names: *mut *mut IDWriteLocalizedStrings,
    ) -> HRESULT,
    fn GetAxisValueNameCount(
        axis: UINT32,
    ) -> UINT32,
    fn GetAxisValueNames(
        axis: UINT32,
        axisValue: UINT32,
        axisRange: *mut DWRITE_FONT_AXIS_RANGE,
        names: *mut *mut IDWriteLocalizedStrings,
    ) -> HRESULT,
    fn HasVariations() -> BOOL,
    fn CreateFontFace(
        simulations: DWRITE_FONT_SIMULATIONS,
        axisValues: *const DWRITE_FONT_AXIS_VALUE,
        numValues: UINT32,
        fontFace: *mut *mut IDWriteFontFace5,
    ) -> HRESULT,
    fn CreateFontFaceReference(
        simulations: DWRITE_FONT_SIMULATIONS,
        axisValues: *const DWRITE_FONT_AXIS_VALUE,
        numValues: UINT32,
        reference: *mut *mut IDWriteFontFaceReference1,
    ) -> HRESULT,
}}
