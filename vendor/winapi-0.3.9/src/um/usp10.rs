// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Unicode Complex Script processor API declarations
use ctypes::{c_int, c_long, c_void};
use shared::minwindef::{BOOL, BYTE, DWORD, UINT, ULONG, WORD};
use shared::ntdef::LCID;
use shared::windef::{HDC, RECT, SIZE};
use shared::winerror::{FACILITY_ITF, SEVERITY_ERROR};
use um::wingdi::ABC;
use um::winnt::{HRESULT, LONG, WCHAR};
pub const SCRIPT_UNDEFINED: WORD = 0;
pub const USP_E_SCRIPT_NOT_IN_FONT: HRESULT = MAKE_HRESULT!(SEVERITY_ERROR, FACILITY_ITF, 0x200);
DECLARE_HANDLE!{SCRIPT_CACHE, SCRIPT_CACHE__}
extern "system" {
    pub fn ScriptFreeCache(
        psc: *mut SCRIPT_CACHE,
    ) -> HRESULT;
}
STRUCT!{struct SCRIPT_CONTROL {
    bit_fields: DWORD,
}}
BITFIELD!{SCRIPT_CONTROL bit_fields: DWORD [
    uDefaultLanguage set_uDefaultLanguage[0..16],
    fContextDigits set_fContextDigits[16..17],
    fInvertPreBoundDir set_fInvertPreBoundDir[17..18],
    fInvertPostBoundDir set_fInvertPostBoundDir[18..19],
    fLinkStringBefore set_fLinkStringBefore[19..20],
    fLinkStringAfter set_fLinkStringAfter[20..21],
    fNeutralOverride set_fNeutralOverride[21..22],
    fNumericOverride set_fNumericOverride[22..23],
    fLegacyBidiClass set_fLegacyBidiClass[23..24],
    fMergeNeutralItems set_fMergeNeutralItems[24..25],
    fReserved set_fReserved[25..32],
]}
STRUCT!{struct SCRIPT_STATE {
    bit_fields: WORD,
}}
BITFIELD!{SCRIPT_STATE bit_fields: WORD [
    uBidiLevel set_uBidiLevel[0..5],
    fOverrideDirection set_fOverrideDirection[5..6],
    fInhibitSymSwap set_fInhibitSymSwap[6..7],
    fCharShape set_fCharShape[7..8],
    fDigitSubstitute set_fDigitSubstitute[8..9],
    fInhibitLigate set_fInhibitLigate[9..10],
    fDisplayZWG set_fDisplayZWG[10..11],
    fArabicNumContext set_fArabicNumContext[11..12],
    fGcpClusters set_fGcpClusters[12..13],
    fReserved set_fReserved[13..14],
    fEngineReserved set_fEngineReserved[14..16],
]}
STRUCT!{struct SCRIPT_ANALYSIS {
    bit_fields: WORD,
    s: SCRIPT_STATE,
}}
BITFIELD!{SCRIPT_ANALYSIS bit_fields: WORD [
    eScript set_eScript[0..10],
    fRTL set_fRTL[10..11],
    fLayoutRTL set_fLayoutRTL[11..12],
    fLinkBefore set_fLinkBefore[12..13],
    fLinkAfter set_fLinkAfter[13..14],
    fLogicalOrder set_fLogicalOrder[14..15],
    fNoGlyphIndex set_fNoGlyphIndex[15..16],
]}
STRUCT!{struct SCRIPT_ITEM {
    iCharPos: c_int,
    a: SCRIPT_ANALYSIS,
}}
extern "system" {
    pub fn ScriptItemize(
        pwcInChars: *const WCHAR,
        cInChars: c_int,
        cMaxItems: c_int,
        psControl: *const SCRIPT_CONTROL,
        psState: *const SCRIPT_STATE,
        pItems: *mut SCRIPT_ITEM,
        pcItems: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptLayout(
        cRuns: c_int,
        pbLevel: *const BYTE,
        piVisualToLogical: *mut c_int,
        piLogicalToVisual: *mut c_int,
    ) -> HRESULT;
}
pub const SCRIPT_JUSTIFY_NONE: WORD = 0;
pub const SCRIPT_JUSTIFY_ARABIC_BLANK: WORD = 1;
pub const SCRIPT_JUSTIFY_CHARACTER: WORD = 2;
pub const SCRIPT_JUSTIFY_RESERVED1: WORD = 3;
pub const SCRIPT_JUSTIFY_BLANK: WORD = 4;
pub const SCRIPT_JUSTIFY_RESERVED2: WORD = 5;
pub const SCRIPT_JUSTIFY_RESERVED3: WORD = 6;
pub const SCRIPT_JUSTIFY_ARABIC_NORMAL: WORD = 7;
pub const SCRIPT_JUSTIFY_ARABIC_KASHIDA: WORD = 8;
pub const SCRIPT_JUSTIFY_ARABIC_ALEF: WORD = 9;
pub const SCRIPT_JUSTIFY_ARABIC_HA: WORD = 10;
pub const SCRIPT_JUSTIFY_ARABIC_RA: WORD = 11;
pub const SCRIPT_JUSTIFY_ARABIC_BA: WORD = 12;
pub const SCRIPT_JUSTIFY_ARABIC_BARA: WORD = 13;
pub const SCRIPT_JUSTIFY_ARABIC_SEEN: WORD = 14;
pub const SCRIPT_JUSTIFY_ARABIC_SEEN_M: WORD = 15;
STRUCT!{struct SCRIPT_VISATTR {
    bit_fields: WORD,
}}
BITFIELD!{SCRIPT_VISATTR bit_fields: WORD [
    uJustification set_uJustification[0..4],
    fClusterStart set_fClusterStart[4..5],
    fDiacritic set_fDiacritic[5..6],
    fZeroWidth set_fZeroWidth[6..7],
    fReserved set_fReserved[7..8],
    fShapeReserved set_fShapeReserved[8..16],
]}
extern "system" {
    pub fn ScriptShape(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        pwcChars: *const WCHAR,
        cChars: c_int,
        cMaxGlyphs: c_int,
        psa: *mut SCRIPT_ANALYSIS,
        pwOutGlyphs: *mut WORD,
        pwLogClust: *mut WORD,
        psva: *mut SCRIPT_VISATTR,
        pcGlyphs: *mut c_int,
    ) -> HRESULT;
}
STRUCT!{struct GOFFSET {
    du: LONG,
    dv: LONG,
}}
extern "system" {
    pub fn ScriptPlace(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        pwGlyphs: *const WORD,
        cGlyphs: c_int,
        psva: *const SCRIPT_VISATTR,
        psa: *mut SCRIPT_ANALYSIS,
        piAdvance: *mut c_int,
        pGoffset: *mut GOFFSET,
        pABC: *mut ABC,
    ) -> HRESULT;
    pub fn ScriptTextOut(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        x: c_int,
        y: c_int,
        fuOptions: UINT,
        lprc: *const RECT,
        psa: *const SCRIPT_ANALYSIS,
        pwcReserved: *const WCHAR,
        iReserved: c_int,
        pwGlyphs: *const WORD,
        cGlyphs: c_int,
        piAdvance: *const c_int,
        piJustify: *const c_int,
        pGoffset: *const GOFFSET,
    ) -> HRESULT;
    pub fn ScriptJustify(
        psva: *const SCRIPT_VISATTR,
        piAdvance: *const c_int,
        cGlyphs: c_int,
        iDx: c_int,
        iMinKashida: c_int,
        piJustify: *mut c_int,
    ) -> HRESULT;
}
STRUCT!{struct SCRIPT_LOGATTR {
    bit_fields: BYTE,
}}
BITFIELD!{SCRIPT_LOGATTR bit_fields: BYTE [
    fSoftBreak set_fSoftBreak[0..1],
    fWhiteSpace set_fWhiteSpace[1..2],
    fCharStop set_fCharStop[2..3],
    fWordStop set_fWordStop[3..4],
    fInvalid set_fInvalid[4..5],
    fReserved set_fReserved[5..8],
]}
extern "system" {
    pub fn ScriptBreak(
        pwcChars: *const WCHAR,
        cChars: c_int,
        psa: *const SCRIPT_ANALYSIS,
        psla: *mut SCRIPT_LOGATTR,
    ) -> HRESULT;
    pub fn ScriptCPtoX(
        iCP: c_int,
        fTrailing: BOOL,
        cChars: c_int,
        cGlyphs: c_int,
        pwLogClust: *const WORD,
        psva: *const SCRIPT_VISATTR,
        piAdvance: *const c_int,
        psa: *const SCRIPT_ANALYSIS,
        piX: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptXtoCP(
        iX: c_int,
        cChars: c_int,
        cGlyphs: c_int,
        pwLogClust: *const WORD,
        psva: *const SCRIPT_VISATTR,
        piAdvance: *const c_int,
        psa: *const SCRIPT_ANALYSIS,
        piCP: *mut c_int,
        piTrailing: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptGetLogicalWidths(
        psa: *const SCRIPT_ANALYSIS,
        cChars: c_int,
        cGlyphs: c_int,
        piGlyphWidth: *const c_int,
        pwLogClust: *const WORD,
        psva: *const SCRIPT_VISATTR,
        piDx: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptApplyLogicalWidth(
        piDx: *const c_int,
        cChars: c_int,
        cGlyphs: c_int,
        pwLogClust: *const WORD,
        psva: *const SCRIPT_VISATTR,
        piAdvance: *const c_int,
        psa: *const SCRIPT_ANALYSIS,
        pABC: *mut ABC,
        piJustify: *mut c_int,
    ) -> HRESULT;
}
pub const SGCM_RTL: DWORD = 0x00000001;
extern "system" {
    pub fn ScriptGetCMap(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        pwcInChars: *const WCHAR,
        cChars: c_int,
        dwFlags: DWORD,
        pwOutGlyphs: *mut WORD,
    ) -> HRESULT;
    pub fn ScriptGetGlyphABCWidth(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        wGlyph: WORD,
        pABC: *mut ABC,
    ) -> HRESULT;
}
STRUCT!{struct SCRIPT_PROPERTIES {
    bit_fields1: DWORD,
    bit_fields2: DWORD,
}}
BITFIELD!{SCRIPT_PROPERTIES bit_fields1: DWORD [
    langid set_langid[0..16],
    fNumeric set_fNumeric[16..17],
    fComplex set_fComplex[17..18],
    fNeedsWordBreaking set_fNeedsWordBreaking[18..19],
    fNeedsCaretInfo set_fNeedsCaretInfo[19..20],
    bCharSet set_bCharSet[20..28],
    fControl set_fControl[28..29],
    fPrivateUseArea set_fPrivateUseArea[29..30],
    fNeedsCharacterJustify set_fNeedsCharacterJustify[30..31],
    fInvalidGlyph set_fInvalidGlyph[31..32],
]}
BITFIELD!{SCRIPT_PROPERTIES bit_fields2: DWORD [
    fInvalidLogAttr set_fInvalidLogAttr[0..1],
    fCDM set_fCDM[1..2],
    fAmbiguousCharSet set_fAmbiguousCharSet[2..3],
    fClusterSizeVaries set_fClusterSizeVaries[3..4],
    fRejectInvalid set_fRejectInvalid[4..5],
]}
extern "system" {
    pub fn ScriptGetProperties(
        ppSp: *mut *mut *const SCRIPT_PROPERTIES,
        piNumScripts: *mut c_int,
    ) -> HRESULT;
}
STRUCT!{struct SCRIPT_FONTPROPERTIES {
    cBytes: c_int,
    wgBlank: WORD,
    wgDefault: WORD,
    wgInvalid: WORD,
    wgKashida: WORD,
    iKashidaWidth: c_int,
}}
extern "system" {
    pub fn ScriptGetFontProperties(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        sfp: *mut SCRIPT_FONTPROPERTIES,
    ) -> HRESULT;
    pub fn ScriptCacheGetHeight(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        tmHeight: *mut c_long,
    ) -> HRESULT;
}
pub const SSA_PASSWORD: DWORD = 0x00000001;
pub const SSA_TAB: DWORD = 0x00000002;
pub const SSA_CLIP: DWORD = 0x00000004;
pub const SSA_FIT: DWORD = 0x00000008;
pub const SSA_DZWG: DWORD = 0x00000010;
pub const SSA_FALLBACK: DWORD = 0x00000020;
pub const SSA_BREAK: DWORD = 0x00000040;
pub const SSA_GLYPHS: DWORD = 0x00000080;
pub const SSA_RTL: DWORD = 0x00000100;
pub const SSA_GCP: DWORD = 0x00000200;
pub const SSA_HOTKEY: DWORD = 0x00000400;
pub const SSA_METAFILE: DWORD = 0x00000800;
pub const SSA_LINK: DWORD = 0x00001000;
pub const SSA_HIDEHOTKEY: DWORD = 0x00002000;
pub const SSA_HOTKEYONLY: DWORD = 0x00002400;
pub const SSA_FULLMEASURE: DWORD = 0x04000000;
pub const SSA_LPKANSIFALLBACK: DWORD = 0x08000000;
pub const SSA_PIDX: DWORD = 0x10000000;
pub const SSA_LAYOUTRTL: DWORD = 0x20000000;
pub const SSA_DONTGLYPH: DWORD = 0x40000000;
pub const SSA_NOKASHIDA: DWORD = 0x80000000;
STRUCT!{struct SCRIPT_TABDEF {
    cTabStops: c_int,
    iScale: c_int,
    pTabStops: *mut c_int,
    iTabOrigin: c_int,
}}
DECLARE_HANDLE!{SCRIPT_STRING_ANALYSIS, SCRIPT_STRING_ANALYSIS__}
extern "system" {
    pub fn ScriptStringAnalyse(
        hdc: HDC,
        pString: *const c_void,
        cString: c_int,
        cGlyphs: c_int,
        iCharset: c_int,
        dwFlags: DWORD,
        iReqWidth: c_int,
        psControl: *mut SCRIPT_CONTROL,
        psState: *mut SCRIPT_STATE,
        piDx: *const c_int,
        pTabdef: *mut SCRIPT_TABDEF,
        pbInClass: *const BYTE,
        pssa: *mut SCRIPT_STRING_ANALYSIS,
    ) -> HRESULT;
    pub fn ScriptStringFree(
        pssa: *mut SCRIPT_STRING_ANALYSIS,
    ) -> HRESULT;
    pub fn ScriptString_pSize(
        ssa: SCRIPT_STRING_ANALYSIS,
    ) -> *const SIZE;
    pub fn ScriptString_pcOutChars(
        ssa: SCRIPT_STRING_ANALYSIS,
    ) -> *const c_int;
    pub fn ScriptString_pLogAttr(
        ssa: SCRIPT_STRING_ANALYSIS,
    ) -> *const SCRIPT_LOGATTR;
    pub fn ScriptStringGetOrder(
        ssa: SCRIPT_STRING_ANALYSIS,
        puOrder: *mut UINT,
    ) -> HRESULT;
    pub fn ScriptStringCPtoX(
        ssa: SCRIPT_STRING_ANALYSIS,
        icp: c_int,
        fTrailing: BOOL,
        pX: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptStringXtoCP(
        ssa: SCRIPT_STRING_ANALYSIS,
        iX: c_int,
        piCh: *mut c_int,
        piTrailing: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptStringGetLogicalWidths(
        ssa: SCRIPT_STRING_ANALYSIS,
        dpiDx: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptStringValidate(
        ssa: SCRIPT_STRING_ANALYSIS,
    ) -> HRESULT;
    pub fn ScriptStringOut(
        ssa: SCRIPT_STRING_ANALYSIS,
        iX: c_int,
        iY: c_int,
        uOptions: UINT,
        prc: *const RECT,
        iMinSel: c_int,
        iMaxSel: c_int,
        fDisabled: BOOL,
    ) -> HRESULT;
}
pub const SIC_COMPLEX: DWORD = 1;
pub const SIC_ASCIIDIGIT: DWORD = 2;
pub const SIC_NEUTRAL: DWORD = 4;
extern "system" {
    pub fn ScriptIsComplex(
        pwcInChars: *const WCHAR,
        cInChars: c_int,
        dwFlags: DWORD,
    ) -> HRESULT;
}
STRUCT!{struct SCRIPT_DIGITSUBSTITUTE {
    bit_fields1: DWORD,
    bit_fields2: DWORD,
    dwReserved: DWORD,
}}
BITFIELD!{SCRIPT_DIGITSUBSTITUTE bit_fields1: DWORD [
    NationalDigitLanguage set_NationalDigitLanguage[0..16],
    TraditionalDigitLanguage set_TraditionalDigitLanguage[16..32],
]}
BITFIELD!{SCRIPT_DIGITSUBSTITUTE bit_fields2: DWORD [
    DigitSubstitute set_DigitSubstitute[0..8],
]}
extern "system" {
    pub fn ScriptRecordDigitSubstitution(
        Locale: LCID,
        psds: *mut SCRIPT_DIGITSUBSTITUTE,
    ) -> HRESULT;
}
pub const SCRIPT_DIGITSUBSTITUTE_CONTEXT: BYTE = 0;
pub const SCRIPT_DIGITSUBSTITUTE_NONE: BYTE = 1;
pub const SCRIPT_DIGITSUBSTITUTE_NATIONAL: BYTE = 2;
pub const SCRIPT_DIGITSUBSTITUTE_TRADITIONAL: BYTE = 3;
extern "system" {
    pub fn ScriptApplyDigitSubstitution(
        psds: *const SCRIPT_DIGITSUBSTITUTE,
        psc: *mut SCRIPT_CONTROL,
        pss: *mut SCRIPT_STATE,
    ) -> HRESULT;
}
pub type OPENTYPE_TAG = ULONG;
pub const SCRIPT_TAG_UNKNOWN: OPENTYPE_TAG = 0x00000000;
STRUCT!{struct OPENTYPE_FEATURE_RECORD {
    tagFeature: OPENTYPE_TAG,
    lParameter: LONG,
}}
STRUCT!{struct TEXTRANGE_PROPERTIES {
    potfRecords: *mut OPENTYPE_FEATURE_RECORD,
    cotfRecords: c_int,
}}
STRUCT!{struct SCRIPT_CHARPROP {
    bit_fields: WORD,
}}
BITFIELD!{SCRIPT_CHARPROP bit_fields: WORD [
    fCanGlyphAlone set_fCanGlyphAlone[0..1],
    reserved set_reserved[1..16],
]}
STRUCT!{struct SCRIPT_GLYPHPROP {
    sva: SCRIPT_VISATTR,
    reserved: WORD,
}}
extern "system" {
    pub fn ScriptShapeOpenType(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        psa: *mut SCRIPT_ANALYSIS,
        tagScript: OPENTYPE_TAG,
        tagLangSys: OPENTYPE_TAG,
        rcRangeChars: *mut c_int,
        rpRangeProperties: *mut *mut TEXTRANGE_PROPERTIES,
        cRanges: c_int,
        pwcChars: *const WCHAR,
        cChars: c_int,
        cMaxGlyphs: c_int,
        pwLogClust: *mut WORD,
        pCharProps: *mut SCRIPT_CHARPROP,
        pwOutGlyphs: *mut WORD,
        pOutGlyphProps: *mut SCRIPT_GLYPHPROP,
        pcGlyphs: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptPlaceOpenType(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        psa: *mut SCRIPT_ANALYSIS,
        tagScript: OPENTYPE_TAG,
        tagLangSys: OPENTYPE_TAG,
        rcRangeChars: *mut c_int,
        rpRangeProperties: *mut *mut TEXTRANGE_PROPERTIES,
        cRanges: c_int,
        pwcChars: *const WCHAR,
        pwLogClust: *mut WORD,
        pCharProps: *mut SCRIPT_CHARPROP,
        cChars: c_int,
        pwGlyphs: *const WORD,
        pGlyphProps: *const SCRIPT_GLYPHPROP,
        cGlyphs: c_int,
        piAdvance: *mut c_int,
        pGoffset: *mut GOFFSET,
        pABC: *mut ABC,
    ) -> HRESULT;
    pub fn ScriptItemizeOpenType(
        pwcInChars: *const WCHAR,
        cInChars: c_int,
        cMaxItems: c_int,
        psControl: *const SCRIPT_CONTROL,
        psState: *const SCRIPT_STATE,
        pItems: *mut SCRIPT_ITEM,
        pScriptTags: *mut OPENTYPE_TAG,
        pcItems: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptGetFontScriptTags(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        psa: *mut SCRIPT_ANALYSIS,
        cMaxTags: c_int,
        pScriptTags: *mut OPENTYPE_TAG,
        pcTags: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptGetFontLanguageTags(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        psa: *mut SCRIPT_ANALYSIS,
        tagScript: OPENTYPE_TAG,
        cMaxTags: c_int,
        pLangsysTags: *mut OPENTYPE_TAG,
        pcTags: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptGetFontFeatureTags(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        psa: *mut SCRIPT_ANALYSIS,
        tagScript: OPENTYPE_TAG,
        tagLangSys: OPENTYPE_TAG,
        cMaxTags: c_int,
        pFeatureTags: *mut OPENTYPE_TAG,
        pcTags: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptGetFontAlternateGlyphs(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        psa: *mut SCRIPT_ANALYSIS,
        tagScript: OPENTYPE_TAG,
        tagLangSys: OPENTYPE_TAG,
        tagFeature: OPENTYPE_TAG,
        wGlyphId: WORD,
        cMaxAlternates: c_int,
        pAlternateGlyphs: *mut WORD,
        pcAlternates: *mut c_int,
    ) -> HRESULT;
    pub fn ScriptSubstituteSingleGlyph(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        psa: *mut SCRIPT_ANALYSIS,
        tagScript: OPENTYPE_TAG,
        tagLangSys: OPENTYPE_TAG,
        tagFeature: OPENTYPE_TAG,
        lParameter: LONG,
        wGlyphId: WORD,
        pwOutGlyphId: *mut WORD,
    ) -> HRESULT;
    pub fn ScriptPositionSingleGlyph(
        hdc: HDC,
        psc: *mut SCRIPT_CACHE,
        psa: *mut SCRIPT_ANALYSIS,
        tagScript: OPENTYPE_TAG,
        tagLangSys: OPENTYPE_TAG,
        tagFeature: OPENTYPE_TAG,
        lParameter: LONG,
        wGlyphId: WORD,
        iAdvance: c_int,
        GOffset: GOFFSET,
        piOutAdvance: *mut c_int,
        pOutGoffset: *mut GOFFSET,
    ) -> HRESULT;
}
