// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::array::CFMutableArrayRef;
use crate::base::{CFAllocatorRef, CFIndex, CFOptionFlags, CFRange, CFTypeID, CFTypeRef};
use crate::locale::CFLocaleRef;
use crate::string::CFStringRef;

#[repr(C)]
pub struct __CFStringTokenizer(c_void);
pub type CFStringTokenizerRef = *mut __CFStringTokenizer;

pub type CFStringTokenizerTokenType = CFOptionFlags;

pub const kCFStringTokenizerTokenNone: CFStringTokenizerTokenType = 0;
pub const kCFStringTokenizerTokenNormal: CFStringTokenizerTokenType = 1 << 0;
pub const kCFStringTokenizerTokenHasSubTokensMask: CFStringTokenizerTokenType = 1 << 1;
pub const kCFStringTokenizerTokenHasDerivedSubTokensMask: CFStringTokenizerTokenType = 1 << 2;
pub const kCFStringTokenizerTokenHasHasNumbersMask: CFStringTokenizerTokenType = 1 << 3;
pub const kCFStringTokenizerTokenHasNonLettersMask: CFStringTokenizerTokenType = 1 << 4;
pub const kCFStringTokenizerTokenIsCJWordMask: CFStringTokenizerTokenType = 1 << 5;

/* Tokenization Modifiers */
pub const kCFStringTokenizerUnitWord: CFOptionFlags = 0;
pub const kCFStringTokenizerUnitSentence: CFOptionFlags = 1;
pub const kCFStringTokenizerUnitParagraph: CFOptionFlags = 2;
pub const kCFStringTokenizerUnitLineBreak: CFOptionFlags = 3;
pub const kCFStringTokenizerUnitWordBoundary: CFOptionFlags = 4;
pub const kCFStringTokenizerAttributeLatinTranscription: CFOptionFlags = 1 << 16;
pub const kCFStringTokenizerAttributeLanguage: CFOptionFlags = 1 << 17;

extern "C" {
    /*
     * CFStringTokenizer.h
     */

    /* Creating a Tokenizer */
    pub fn CFStringTokenizerCreate(
        alloc: CFAllocatorRef,
        string: CFStringRef,
        range: CFRange,
        options: CFOptionFlags,
        locale: CFLocaleRef,
    ) -> CFStringTokenizerRef;

    /* Setting the String */
    pub fn CFStringTokenizerSetString(
        tokenizer: CFStringTokenizerRef,
        string: CFStringRef,
        range: CFRange,
    );

    /* Changing the Location */
    pub fn CFStringTokenizerAdvanceToNextToken(
        tokenizer: CFStringTokenizerRef,
    ) -> CFStringTokenizerTokenType;
    pub fn CFStringTokenizerGoToTokenAtIndex(
        tokenizer: CFStringTokenizerRef,
        index: CFIndex,
    ) -> CFStringTokenizerTokenType;

    /* Getting Information About the Current Token */
    pub fn CFStringTokenizerCopyCurrentTokenAttribute(
        tokenizer: CFStringTokenizerRef,
        attribute: CFOptionFlags,
    ) -> CFTypeRef;
    pub fn CFStringTokenizerGetCurrentTokenRange(tokenizer: CFStringTokenizerRef) -> CFRange;
    pub fn CFStringTokenizerGetCurrentSubTokens(
        tokenizer: CFStringTokenizerRef,
        ranges: *mut CFRange,
        maxRangeLength: CFIndex,
        derivedSubTokens: CFMutableArrayRef,
    ) -> CFIndex;

    /* Identifying a Language */
    pub fn CFStringTokenizerCopyBestStringLanguage(
        string: CFStringRef,
        range: CFRange,
    ) -> CFStringRef;

    /* Getting the CFStringTokenizer Type ID */
    pub fn CFStringTokenizerGetTypeID() -> CFTypeID;
}
