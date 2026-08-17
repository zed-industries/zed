// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::array::CFArrayRef;
use crate::base::{
    Boolean, CFAllocatorRef, CFComparisonResult, CFIndex, CFOptionFlags, CFRange, CFTypeID,
    ConstStr255Param, ConstStringPtr, SInt32, StringPtr, UInt32, UInt8, UTF32Char,
};
use crate::characterset::CFCharacterSetRef;
use crate::data::CFDataRef;
use crate::dictionary::CFDictionaryRef;
use crate::locale::CFLocaleRef;
use std::os::raw::{c_char, c_double, c_ulong, c_ushort, c_void};

pub type CFStringCompareFlags = CFOptionFlags;
pub const kCFCompareCaseInsensitive: CFStringCompareFlags = 1;
pub const kCFCompareBackwards: CFStringCompareFlags = 4;
pub const kCFCompareAnchored: CFStringCompareFlags = 8;
pub const kCFCompareNonliteral: CFStringCompareFlags = 16;
pub const kCFCompareLocalized: CFStringCompareFlags = 32;
pub const kCFCompareNumerically: CFStringCompareFlags = 64;
pub const kCFCompareDiacriticInsensitive: CFStringCompareFlags = 128;
pub const kCFCompareWidthInsensitive: CFStringCompareFlags = 256;
pub const kCFCompareForcedOrdering: CFStringCompareFlags = 512;

pub type CFStringEncoding = UInt32;
pub type UniChar = c_ushort;

// macOS built-in encodings.

pub const kCFStringEncodingMacRoman: CFStringEncoding = 0;
pub const kCFStringEncodingWindowsLatin1: CFStringEncoding = 0x0500;
pub const kCFStringEncodingISOLatin1: CFStringEncoding = 0x0201;
pub const kCFStringEncodingNextStepLatin: CFStringEncoding = 0x0B01;
pub const kCFStringEncodingASCII: CFStringEncoding = 0x0600;
pub const kCFStringEncodingUnicode: CFStringEncoding = 0x0100;
pub const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;
pub const kCFStringEncodingNonLossyASCII: CFStringEncoding = 0x0BFF;

pub const kCFStringEncodingUTF16: CFStringEncoding = 0x0100;
pub const kCFStringEncodingUTF16BE: CFStringEncoding = 0x10000100;
pub const kCFStringEncodingUTF16LE: CFStringEncoding = 0x14000100;
pub const kCFStringEncodingUTF32: CFStringEncoding = 0x0c000100;
pub const kCFStringEncodingUTF32BE: CFStringEncoding = 0x18000100;
pub const kCFStringEncodingUTF32LE: CFStringEncoding = 0x1c000100;

// CFStringEncodingExt.h
// External encodings, except those defined above.
pub const kCFStringEncodingMacJapanese: CFStringEncoding = 1;
pub const kCFStringEncodingMacChineseTrad: CFStringEncoding = 2;
pub const kCFStringEncodingMacKorean: CFStringEncoding = 3;
pub const kCFStringEncodingMacArabic: CFStringEncoding = 4;
pub const kCFStringEncodingMacHebrew: CFStringEncoding = 5;
pub const kCFStringEncodingMacGreek: CFStringEncoding = 6;
pub const kCFStringEncodingMacCyrillic: CFStringEncoding = 7;
pub const kCFStringEncodingMacDevanagari: CFStringEncoding = 9;
pub const kCFStringEncodingMacGurmukhi: CFStringEncoding = 10;
pub const kCFStringEncodingMacGujarati: CFStringEncoding = 11;
pub const kCFStringEncodingMacOriya: CFStringEncoding = 12;
pub const kCFStringEncodingMacBengali: CFStringEncoding = 13;
pub const kCFStringEncodingMacTamil: CFStringEncoding = 14;
pub const kCFStringEncodingMacTelugu: CFStringEncoding = 15;
pub const kCFStringEncodingMacKannada: CFStringEncoding = 16;
pub const kCFStringEncodingMacMalayalam: CFStringEncoding = 17;
pub const kCFStringEncodingMacSinhalese: CFStringEncoding = 18;
pub const kCFStringEncodingMacBurmese: CFStringEncoding = 19;
pub const kCFStringEncodingMacKhmer: CFStringEncoding = 20;
pub const kCFStringEncodingMacThai: CFStringEncoding = 21;
pub const kCFStringEncodingMacLaotian: CFStringEncoding = 22;
pub const kCFStringEncodingMacGeorgian: CFStringEncoding = 23;
pub const kCFStringEncodingMacArmenian: CFStringEncoding = 24;
pub const kCFStringEncodingMacChineseSimp: CFStringEncoding = 25;
pub const kCFStringEncodingMacTibetan: CFStringEncoding = 26;
pub const kCFStringEncodingMacMongolian: CFStringEncoding = 27;
pub const kCFStringEncodingMacEthiopic: CFStringEncoding = 28;
pub const kCFStringEncodingMacCentralEurRoman: CFStringEncoding = 29;
pub const kCFStringEncodingMacVietnamese: CFStringEncoding = 30;
pub const kCFStringEncodingMacExtArabic: CFStringEncoding = 31;
pub const kCFStringEncodingMacSymbol: CFStringEncoding = 33;
pub const kCFStringEncodingMacDingbats: CFStringEncoding = 34;
pub const kCFStringEncodingMacTurkish: CFStringEncoding = 35;
pub const kCFStringEncodingMacCroatian: CFStringEncoding = 36;
pub const kCFStringEncodingMacIcelandic: CFStringEncoding = 37;
pub const kCFStringEncodingMacRomanian: CFStringEncoding = 38;
pub const kCFStringEncodingMacCeltic: CFStringEncoding = 39;
pub const kCFStringEncodingMacGaelic: CFStringEncoding = 40;
pub const kCFStringEncodingMacFarsi: CFStringEncoding = 0x8C;
pub const kCFStringEncodingMacUkrainian: CFStringEncoding = 0x98;
pub const kCFStringEncodingMacInuit: CFStringEncoding = 0xEC;
pub const kCFStringEncodingMacVT100: CFStringEncoding = 0xFC;
pub const kCFStringEncodingMacHFS: CFStringEncoding = 0xFF;
pub const kCFStringEncodingISOLatin2: CFStringEncoding = 0x0202;
pub const kCFStringEncodingISOLatin3: CFStringEncoding = 0x0203;
pub const kCFStringEncodingISOLatin4: CFStringEncoding = 0x0204;
pub const kCFStringEncodingISOLatinCyrillic: CFStringEncoding = 0x0205;
pub const kCFStringEncodingISOLatinArabic: CFStringEncoding = 0x0206;
pub const kCFStringEncodingISOLatinGreek: CFStringEncoding = 0x0207;
pub const kCFStringEncodingISOLatinHebrew: CFStringEncoding = 0x0208;
pub const kCFStringEncodingISOLatin5: CFStringEncoding = 0x0209;
pub const kCFStringEncodingISOLatin6: CFStringEncoding = 0x020A;
pub const kCFStringEncodingISOLatinThai: CFStringEncoding = 0x020B;
pub const kCFStringEncodingISOLatin7: CFStringEncoding = 0x020D;
pub const kCFStringEncodingISOLatin8: CFStringEncoding = 0x020E;
pub const kCFStringEncodingISOLatin9: CFStringEncoding = 0x020F;
pub const kCFStringEncodingISOLatin10: CFStringEncoding = 0x0210;
pub const kCFStringEncodingDOSLatinUS: CFStringEncoding = 0x0400;
pub const kCFStringEncodingDOSGreek: CFStringEncoding = 0x0405;
pub const kCFStringEncodingDOSBalticRim: CFStringEncoding = 0x0406;
pub const kCFStringEncodingDOSLatin1: CFStringEncoding = 0x0410;
pub const kCFStringEncodingDOSGreek1: CFStringEncoding = 0x0411;
pub const kCFStringEncodingDOSLatin2: CFStringEncoding = 0x0412;
pub const kCFStringEncodingDOSCyrillic: CFStringEncoding = 0x0413;
pub const kCFStringEncodingDOSTurkish: CFStringEncoding = 0x0414;
pub const kCFStringEncodingDOSPortuguese: CFStringEncoding = 0x0415;
pub const kCFStringEncodingDOSIcelandic: CFStringEncoding = 0x0416;
pub const kCFStringEncodingDOSHebrew: CFStringEncoding = 0x0417;
pub const kCFStringEncodingDOSCanadianFrench: CFStringEncoding = 0x0418;
pub const kCFStringEncodingDOSArabic: CFStringEncoding = 0x0419;
pub const kCFStringEncodingDOSNordic: CFStringEncoding = 0x041A;
pub const kCFStringEncodingDOSRussian: CFStringEncoding = 0x041B;
pub const kCFStringEncodingDOSGreek2: CFStringEncoding = 0x041C;
pub const kCFStringEncodingDOSThai: CFStringEncoding = 0x041D;
pub const kCFStringEncodingDOSJapanese: CFStringEncoding = 0x0420;
pub const kCFStringEncodingDOSChineseSimplif: CFStringEncoding = 0x0421;
pub const kCFStringEncodingDOSKorean: CFStringEncoding = 0x0422;
pub const kCFStringEncodingDOSChineseTrad: CFStringEncoding = 0x0423;
pub const kCFStringEncodingWindowsLatin2: CFStringEncoding = 0x0501;
pub const kCFStringEncodingWindowsCyrillic: CFStringEncoding = 0x0502;
pub const kCFStringEncodingWindowsGreek: CFStringEncoding = 0x0503;
pub const kCFStringEncodingWindowsLatin5: CFStringEncoding = 0x0504;
pub const kCFStringEncodingWindowsHebrew: CFStringEncoding = 0x0505;
pub const kCFStringEncodingWindowsArabic: CFStringEncoding = 0x0506;
pub const kCFStringEncodingWindowsBalticRim: CFStringEncoding = 0x0507;
pub const kCFStringEncodingWindowsVietnamese: CFStringEncoding = 0x0508;
pub const kCFStringEncodingWindowsKoreanJohab: CFStringEncoding = 0x0510;
pub const kCFStringEncodingANSEL: CFStringEncoding = 0x0601;
pub const kCFStringEncodingJIS_X0201_76: CFStringEncoding = 0x0620;
pub const kCFStringEncodingJIS_X0208_83: CFStringEncoding = 0x0621;
pub const kCFStringEncodingJIS_X0208_90: CFStringEncoding = 0x0622;
pub const kCFStringEncodingJIS_X0212_90: CFStringEncoding = 0x0623;
pub const kCFStringEncodingJIS_C6226_78: CFStringEncoding = 0x0624;
pub const kCFStringEncodingShiftJIS_X0213: CFStringEncoding = 0x0628;
pub const kCFStringEncodingShiftJIS_X0213_MenKuTen: CFStringEncoding = 0x0629;
pub const kCFStringEncodingGB_2312_80: CFStringEncoding = 0x0630;
pub const kCFStringEncodingGBK_95: CFStringEncoding = 0x0631;
pub const kCFStringEncodingGB_18030_2000: CFStringEncoding = 0x0632;
pub const kCFStringEncodingKSC_5601_87: CFStringEncoding = 0x0640;
pub const kCFStringEncodingKSC_5601_92_Johab: CFStringEncoding = 0x0641;
pub const kCFStringEncodingCNS_11643_92_P1: CFStringEncoding = 0x0651;
pub const kCFStringEncodingCNS_11643_92_P2: CFStringEncoding = 0x0652;
pub const kCFStringEncodingCNS_11643_92_P3: CFStringEncoding = 0x0653;
pub const kCFStringEncodingISO_2022_JP: CFStringEncoding = 0x0820;
pub const kCFStringEncodingISO_2022_JP_2: CFStringEncoding = 0x0821;
pub const kCFStringEncodingISO_2022_JP_1: CFStringEncoding = 0x0822;
pub const kCFStringEncodingISO_2022_JP_3: CFStringEncoding = 0x0823;
pub const kCFStringEncodingISO_2022_CN: CFStringEncoding = 0x0830;
pub const kCFStringEncodingISO_2022_CN_EXT: CFStringEncoding = 0x0831;
pub const kCFStringEncodingISO_2022_KR: CFStringEncoding = 0x0840;
pub const kCFStringEncodingEUC_JP: CFStringEncoding = 0x0920;
pub const kCFStringEncodingEUC_CN: CFStringEncoding = 0x0930;
pub const kCFStringEncodingEUC_TW: CFStringEncoding = 0x0931;
pub const kCFStringEncodingEUC_KR: CFStringEncoding = 0x0940;
pub const kCFStringEncodingShiftJIS: CFStringEncoding = 0x0A01;
pub const kCFStringEncodingKOI8_R: CFStringEncoding = 0x0A02;
pub const kCFStringEncodingBig5: CFStringEncoding = 0x0A03;
pub const kCFStringEncodingMacRomanLatin1: CFStringEncoding = 0x0A04;
pub const kCFStringEncodingHZ_GB_2312: CFStringEncoding = 0x0A05;
pub const kCFStringEncodingBig5_HKSCS_1999: CFStringEncoding = 0x0A06;
pub const kCFStringEncodingVISCII: CFStringEncoding = 0x0A07;
pub const kCFStringEncodingKOI8_U: CFStringEncoding = 0x0A08;
pub const kCFStringEncodingBig5_E: CFStringEncoding = 0x0A09;
pub const kCFStringEncodingNextStepJapanese: CFStringEncoding = 0x0B02;
pub const kCFStringEncodingEBCDIC_US: CFStringEncoding = 0x0C01;
pub const kCFStringEncodingEBCDIC_CP037: CFStringEncoding = 0x0C02;
pub const kCFStringEncodingUTF7: CFStringEncoding = 0x04000100;
pub const kCFStringEncodingUTF7_IMAP: CFStringEncoding = 0x0A10;
pub const kCFStringEncodingShiftJIS_X0213_00: CFStringEncoding = 0x0628; /* Deprecated */

pub const kCFStringEncodingInvalidId: u32 = 0xffffffff;

pub type CFStringNormalizationForm = CFIndex;
pub const kCFStringNormalizationFormD: CFStringNormalizationForm = 0;
pub const kCFStringNormalizationFormKD: CFStringNormalizationForm = 1;
pub const kCFStringNormalizationFormC: CFStringNormalizationForm = 2;
pub const kCFStringNormalizationFormKC: CFStringNormalizationForm = 3;

#[repr(C)]
pub struct __CFString(c_void);

pub type CFStringRef = *const __CFString;
pub type CFMutableStringRef = *mut __CFString;

/* todo: The source code of the following functions is right in CFString.h */
/*
pub fn CFStringGetLongCharacterForSurrogatePair(surrogateHigh: UniChar, surrogateLow: UniChar) -> UTF32Char;
pub fn CFStringGetSurrogatePairForLongCharacter(character: UTF32Char, surrogates: *mut UniChar) -> Boolean;
pub fn CFStringIsSurrogateHighCharacter(character: UniChar) -> Boolean;
pub fn CFStringIsSurrogateLowCharacter(character: UniChar) -> Boolean;
pub fn CFStringInitInlineBuffer(str: CFStringRef, buf: *mut CFStringInlineBuffer, range: CFRange);
pub fn CFStringGetCharacterFromInlineBuffer(buf: *mut CFStringInlineBuffer, idx: CFIndex) -> UniChar;
*/

extern "C" {
    /*
     * CFString.h
     */

    // N.B. organized according to "Functions by task" in docs

    /* CFString */
    /* Creating a CFString */
    //fn CFSTR
    pub fn CFStringCreateArrayBySeparatingStrings(
        alloc: CFAllocatorRef,
        theString: CFStringRef,
        separatorString: CFStringRef,
    ) -> CFArrayRef;
    pub fn CFStringCreateByCombiningStrings(
        alloc: CFAllocatorRef,
        theArray: CFArrayRef,
        separatorString: CFStringRef,
    ) -> CFStringRef;
    pub fn CFStringCreateCopy(alloc: CFAllocatorRef, theString: CFStringRef) -> CFStringRef;
    pub fn CFStringCreateFromExternalRepresentation(
        alloc: CFAllocatorRef,
        data: CFDataRef,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
    pub fn CFStringCreateWithBytes(
        alloc: CFAllocatorRef,
        bytes: *const UInt8,
        numBytes: CFIndex,
        encoding: CFStringEncoding,
        isExternalRepresentation: Boolean,
    ) -> CFStringRef;
    pub fn CFStringCreateWithBytesNoCopy(
        alloc: CFAllocatorRef,
        bytes: *const UInt8,
        numBytes: CFIndex,
        encoding: CFStringEncoding,
        isExternalRepresentation: Boolean,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
    pub fn CFStringCreateWithCharacters(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        numChars: CFIndex,
    ) -> CFStringRef;
    pub fn CFStringCreateWithCharactersNoCopy(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        numChars: CFIndex,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
    pub fn CFStringCreateWithCString(
        alloc: CFAllocatorRef,
        cStr: *const c_char,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
    pub fn CFStringCreateWithCStringNoCopy(
        alloc: CFAllocatorRef,
        cStr: *const c_char,
        encoding: CFStringEncoding,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
    pub fn CFStringCreateWithFormat(
        alloc: CFAllocatorRef,
        formatOptions: CFDictionaryRef,
        format: CFStringRef,
        ...
    ) -> CFStringRef;
    //pub fn CFStringCreateWithFormatAndArguments(alloc: CFAllocatorRef, formatOptions: CFDictionaryRef, format: CFStringRef, arguments: va_list) -> CFStringRef;
    pub fn CFStringCreateWithPascalString(
        alloc: CFAllocatorRef,
        pStr: ConstStr255Param,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
    pub fn CFStringCreateWithPascalStringNoCopy(
        alloc: CFAllocatorRef,
        pStr: ConstStr255Param,
        encoding: CFStringEncoding,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
    pub fn CFStringCreateWithSubstring(
        alloc: CFAllocatorRef,
        str: CFStringRef,
        range: CFRange,
    ) -> CFStringRef;

    /* Searching Strings */
    pub fn CFStringCreateArrayWithFindResults(
        alloc: CFAllocatorRef,
        theString: CFStringRef,
        stringToFind: CFStringRef,
        rangeToSearch: CFRange,
        compareOptions: CFStringCompareFlags,
    ) -> CFArrayRef;
    pub fn CFStringFind(
        theString: CFStringRef,
        stringToFind: CFStringRef,
        compareOptions: CFStringCompareFlags,
    ) -> CFRange;
    pub fn CFStringFindCharacterFromSet(
        theString: CFStringRef,
        theSet: CFCharacterSetRef,
        rangeToSearch: CFRange,
        searchOptions: CFStringCompareFlags,
        result: *mut CFRange,
    ) -> Boolean;
    pub fn CFStringFindWithOptions(
        theString: CFStringRef,
        stringToFind: CFStringRef,
        rangeToSearch: CFRange,
        searchOptions: CFStringCompareFlags,
        result: *mut CFRange,
    ) -> Boolean;
    pub fn CFStringFindWithOptionsAndLocale(
        theString: CFStringRef,
        stringToFind: CFStringRef,
        rangeToSearch: CFRange,
        searchOptions: CFStringCompareFlags,
        locale: CFLocaleRef,
        result: *mut CFRange,
    ) -> Boolean;
    pub fn CFStringGetLineBounds(
        theString: CFStringRef,
        range: CFRange,
        lineBeginIndex: *mut CFIndex,
        lineEndIndex: *mut CFIndex,
        contentsEndIndex: *mut CFIndex,
    );

    /* Comparing Strings */
    pub fn CFStringCompare(
        theString1: CFStringRef,
        theString2: CFStringRef,
        compareOptions: CFStringCompareFlags,
    ) -> CFComparisonResult;
    pub fn CFStringCompareWithOptions(
        theString1: CFStringRef,
        theString2: CFStringRef,
        rangeToCompare: CFRange,
        compareOptions: CFStringCompareFlags,
    ) -> CFComparisonResult;
    pub fn CFStringCompareWithOptionsAndLocale(
        theString1: CFStringRef,
        theString2: CFStringRef,
        rangeToCompare: CFRange,
        compareOptions: CFStringCompareFlags,
        locale: CFLocaleRef,
    ) -> CFComparisonResult;
    pub fn CFStringHasPrefix(theString: CFStringRef, prefix: CFStringRef) -> Boolean;
    pub fn CFStringHasSuffix(theString: CFStringRef, suffix: CFStringRef) -> Boolean;

    /* Accessing Characters */
    pub fn CFStringCreateExternalRepresentation(
        alloc: CFAllocatorRef,
        theString: CFStringRef,
        encoding: CFStringEncoding,
        lossByte: UInt8,
    ) -> CFDataRef;
    pub fn CFStringGetBytes(
        theString: CFStringRef,
        range: CFRange,
        encoding: CFStringEncoding,
        lossByte: UInt8,
        isExternalRepresentation: Boolean,
        buffer: *mut UInt8,
        maxBufLen: CFIndex,
        usedBufLen: *mut CFIndex,
    ) -> CFIndex;
    pub fn CFStringGetCharacterAtIndex(theString: CFStringRef, idx: CFIndex) -> UniChar;
    pub fn CFStringGetCharacters(theString: CFStringRef, range: CFRange, buffer: *mut UniChar);
    pub fn CFStringGetCharactersPtr(theString: CFStringRef) -> *const UniChar;
    pub fn CFStringGetCString(
        theString: CFStringRef,
        buffer: *mut c_char,
        bufferSize: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;
    pub fn CFStringGetCStringPtr(
        theString: CFStringRef,
        encoding: CFStringEncoding,
    ) -> *const c_char;
    pub fn CFStringGetLength(theString: CFStringRef) -> CFIndex;
    pub fn CFStringGetPascalString(
        theString: CFStringRef,
        buffer: StringPtr,
        bufferSize: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;
    pub fn CFStringGetPascalStringPtr(
        theString: CFStringRef,
        encoding: CFStringEncoding,
    ) -> ConstStringPtr;
    pub fn CFStringGetRangeOfComposedCharactersAtIndex(
        theString: CFStringRef,
        theIndex: CFIndex,
    ) -> CFRange;

    /* Working With Hyphenation */
    pub fn CFStringGetHyphenationLocationBeforeIndex(
        string: CFStringRef,
        location: CFIndex,
        limitRange: CFRange,
        options: CFOptionFlags,
        locale: CFLocaleRef,
        character: *mut UTF32Char,
    ) -> CFIndex;
    pub fn CFStringIsHyphenationAvailableForLocale(locale: CFLocaleRef) -> Boolean;

    /* Working With Encodings */
    pub fn CFStringConvertEncodingToIANACharSetName(encoding: CFStringEncoding) -> CFStringRef;
    pub fn CFStringConvertEncodingToNSStringEncoding(encoding: CFStringEncoding) -> c_ulong;
    pub fn CFStringConvertEncodingToWindowsCodepage(encoding: CFStringEncoding) -> UInt32;
    pub fn CFStringConvertIANACharSetNameToEncoding(theString: CFStringRef) -> CFStringEncoding;
    pub fn CFStringConvertNSStringEncodingToEncoding(encoding: c_ulong) -> CFStringEncoding;
    pub fn CFStringConvertWindowsCodepageToEncoding(codepage: UInt32) -> CFStringEncoding;
    pub fn CFStringGetFastestEncoding(theString: CFStringRef) -> CFStringEncoding;
    pub fn CFStringGetListOfAvailableEncodings() -> *const CFStringEncoding;
    pub fn CFStringGetMaximumSizeForEncoding(
        length: CFIndex,
        encoding: CFStringEncoding,
    ) -> CFIndex;
    pub fn CFStringGetMostCompatibleMacStringEncoding(
        encoding: CFStringEncoding,
    ) -> CFStringEncoding;
    pub fn CFStringGetNameOfEncoding(encoding: CFStringEncoding) -> CFStringRef;
    pub fn CFStringGetSmallestEncoding(theString: CFStringRef) -> CFStringEncoding;
    pub fn CFStringGetSystemEncoding() -> CFStringEncoding;
    pub fn CFStringIsEncodingAvailable(encoding: CFStringEncoding) -> Boolean;

    /* Getting Numeric Values */
    pub fn CFStringGetDoubleValue(str: CFStringRef) -> c_double;
    pub fn CFStringGetIntValue(str: CFStringRef) -> SInt32;

    /* Getting String Properties */
    pub fn CFShowStr(str: CFStringRef);
    pub fn CFStringGetTypeID() -> CFTypeID;

    /* String File System Representations */
    pub fn CFStringCreateWithFileSystemRepresentation(
        alloc: CFAllocatorRef,
        buffer: *const c_char,
    ) -> CFStringRef;
    pub fn CFStringGetFileSystemRepresentation(
        string: CFStringRef,
        buffer: *mut c_char,
        maxBufLen: CFIndex,
    ) -> Boolean;
    pub fn CFStringGetMaximumSizeOfFileSystemRepresentation(string: CFStringRef) -> CFIndex;

    /* Getting Paragraph Bounds */
    pub fn CFStringGetParagraphBounds(
        string: CFStringRef,
        range: CFRange,
        parBeginIndex: *mut CFIndex,
        parEndIndex: *mut CFIndex,
        contentsEndIndex: *mut CFIndex,
    );

    /* CFMutableString */
    /* CFMutableString Miscellaneous Functions */
    pub fn CFStringAppend(theString: CFMutableStringRef, appendedString: CFStringRef);
    pub fn CFStringAppendCharacters(
        theString: CFMutableStringRef,
        chars: *const UniChar,
        numChars: CFIndex,
    );
    pub fn CFStringAppendCString(
        theString: CFMutableStringRef,
        cStr: *const c_char,
        encoding: CFStringEncoding,
    );
    pub fn CFStringAppendFormat(
        theString: CFMutableStringRef,
        formatOptions: CFDictionaryRef,
        format: CFStringRef,
        ...
    );
    //pub fn CFStringAppendFormatAndArguments(theString: CFMutableStringRef, formatOptions: CFDictionaryRef, format: CFStringRef, arguments: va_list);
    pub fn CFStringAppendPascalString(
        theString: CFMutableStringRef,
        pStr: ConstStr255Param,
        encoding: CFStringEncoding,
    );
    pub fn CFStringCapitalize(theString: CFMutableStringRef, locale: CFLocaleRef);
    pub fn CFStringCreateMutable(alloc: CFAllocatorRef, maxLength: CFIndex) -> CFMutableStringRef;
    pub fn CFStringCreateMutableCopy(
        alloc: CFAllocatorRef,
        maxLength: CFIndex,
        theString: CFStringRef,
    ) -> CFMutableStringRef;
    pub fn CFStringCreateMutableWithExternalCharactersNoCopy(
        alloc: CFAllocatorRef,
        chars: *mut UniChar,
        numChars: CFIndex,
        capacity: CFIndex,
        externalCharactersAllocator: CFAllocatorRef,
    ) -> CFMutableStringRef;
    pub fn CFStringDelete(theString: CFMutableStringRef, range: CFRange);
    pub fn CFStringFindAndReplace(
        theString: CFMutableStringRef,
        stringToFind: CFStringRef,
        replacementString: CFStringRef,
        rangeToSearch: CFRange,
        compareOptions: CFStringCompareFlags,
    ) -> CFIndex;
    pub fn CFStringFold(
        theString: CFMutableStringRef,
        theFlags: CFStringCompareFlags,
        theLocale: CFLocaleRef,
    );
    pub fn CFStringInsert(str: CFMutableStringRef, idx: CFIndex, insertedStr: CFStringRef);
    pub fn CFStringLowercase(theString: CFMutableStringRef, locale: CFLocaleRef);
    pub fn CFStringNormalize(theString: CFMutableStringRef, theForm: CFStringNormalizationForm);
    pub fn CFStringPad(
        theString: CFMutableStringRef,
        padString: CFStringRef,
        length: CFIndex,
        indexIntoPad: CFIndex,
    );
    pub fn CFStringReplace(theString: CFMutableStringRef, range: CFRange, replacement: CFStringRef);
    pub fn CFStringReplaceAll(theString: CFMutableStringRef, replacement: CFStringRef);
    pub fn CFStringSetExternalCharactersNoCopy(
        theString: CFMutableStringRef,
        chars: *mut UniChar,
        length: CFIndex,
        capacity: CFIndex,
    );
    pub fn CFStringTransform(
        string: CFMutableStringRef,
        range: *mut CFRange,
        transform: CFStringRef,
        reverse: Boolean,
    ) -> Boolean;
    pub fn CFStringTrim(theString: CFMutableStringRef, trimString: CFStringRef);
    pub fn CFStringTrimWhitespace(theString: CFMutableStringRef);
    pub fn CFStringUppercase(theString: CFMutableStringRef, locale: CFLocaleRef);
}
