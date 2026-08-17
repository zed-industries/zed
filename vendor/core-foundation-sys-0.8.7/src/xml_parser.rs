// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFOptionFlags, CFTypeID};
use crate::data::CFDataRef;
use crate::dictionary::CFDictionaryRef;
use crate::string::CFStringRef;
use crate::url::CFURLRef;
use crate::xml_node::{CFXMLExternalID, CFXMLNodeRef, CFXMLTreeRef};

#[repr(C)]
pub struct __CFXMLParser(c_void);

pub type CFXMLParserRef = *mut __CFXMLParser;

pub type CFXMLParserOptions = CFOptionFlags;
pub const kCFXMLParserValidateDocument: CFXMLParserOptions = 1 << 0;
pub const kCFXMLParserSkipMetaData: CFXMLParserOptions = 1 << 1;
pub const kCFXMLParserReplacePhysicalEntities: CFXMLParserOptions = 1 << 2;
pub const kCFXMLParserSkipWhitespace: CFXMLParserOptions = 1 << 3;
pub const kCFXMLParserResolveExternalEntities: CFXMLParserOptions = 1 << 4;
pub const kCFXMLParserAddImpliedAttributes: CFXMLParserOptions = 1 << 5;
pub const kCFXMLParserAllOptions: CFXMLParserOptions = 0x00FFFFFF;
pub const kCFXMLParserNoOptions: CFXMLParserOptions = 0;

pub type CFXMLParserStatusCode = CFIndex;
pub const kCFXMLStatusParseNotBegun: CFIndex = -2;
pub const kCFXMLStatusParseInProgress: CFIndex = -1;
pub const kCFXMLStatusParseSuccessful: CFIndex = 0;
pub const kCFXMLErrorUnexpectedEOF: CFIndex = 1;
pub const kCFXMLErrorUnknownEncoding: CFIndex = 2;
pub const kCFXMLErrorEncodingConversionFailure: CFIndex = 3;
pub const kCFXMLErrorMalformedProcessingInstruction: CFIndex = 4;
pub const kCFXMLErrorMalformedDTD: CFIndex = 5;
pub const kCFXMLErrorMalformedName: CFIndex = 6;
pub const kCFXMLErrorMalformedCDSect: CFIndex = 7;
pub const kCFXMLErrorMalformedCloseTag: CFIndex = 8;
pub const kCFXMLErrorMalformedStartTag: CFIndex = 9;
pub const kCFXMLErrorMalformedDocument: CFIndex = 10;
pub const kCFXMLErrorElementlessDocument: CFIndex = 11;
pub const kCFXMLErrorMalformedComment: CFIndex = 12;
pub const kCFXMLErrorMalformedCharacterReference: CFIndex = 13;
pub const kCFXMLErrorMalformedParsedCharacterData: CFIndex = 14;
pub const kCFXMLErrorNoData: CFIndex = 15;

pub type CFXMLParserCreateXMLStructureCallBack =
    extern "C" fn(parser: CFXMLParserRef, nodeDesc: CFXMLNodeRef, info: *mut c_void) -> *mut c_void;
pub type CFXMLParserAddChildCallBack = extern "C" fn(
    parser: CFXMLParserRef,
    parent: *mut c_void,
    child: *mut c_void,
    info: *mut c_void,
);
pub type CFXMLParserEndXMLStructureCallBack =
    extern "C" fn(parser: CFXMLParserRef, xmlType: *mut c_void, info: *mut c_void);
pub type CFXMLParserResolveExternalEntityCallBack = extern "C" fn(
    parser: CFXMLParserRef,
    extID: *mut CFXMLExternalID,
    info: *mut c_void,
) -> CFDataRef;
pub type CFXMLParserHandleErrorCallBack = extern "C" fn(
    parser: CFXMLParserRef,
    error: CFXMLParserStatusCode,
    info: *mut c_void,
) -> Boolean;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLParserCallBacks {
    pub version: CFIndex,
    pub createXMLStructure: CFXMLParserCreateXMLStructureCallBack,
    pub addChild: CFXMLParserAddChildCallBack,
    pub endXMLStructure: CFXMLParserEndXMLStructureCallBack,
    pub resolveExternalEntity: CFXMLParserResolveExternalEntityCallBack,
    pub handleError: CFXMLParserHandleErrorCallBack,
}

pub type CFXMLParserRetainCallBack = extern "C" fn(info: *const c_void) -> *const c_void;
pub type CFXMLParserReleaseCallBack = extern "C" fn(info: *const c_void);
pub type CFXMLParserCopyDescriptionCallBack = extern "C" fn(info: *const c_void) -> CFStringRef;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLParserContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: CFXMLParserRetainCallBack,
    pub release: CFXMLParserReleaseCallBack,
    pub copyDescription: CFXMLParserCopyDescriptionCallBack,
}

extern "C" {
    /*
     * CFXMLParser.h
     */

    pub static kCFXMLTreeErrorDescription: CFStringRef;
    pub static kCFXMLTreeErrorLineNumber: CFStringRef;
    pub static kCFXMLTreeErrorLocation: CFStringRef;
    pub static kCFXMLTreeErrorStatusCode: CFStringRef;

    pub fn CFXMLParserGetTypeID() -> CFTypeID;
    pub fn CFXMLParserCreate(
        allocator: CFAllocatorRef,
        xmlData: CFDataRef,
        dataSource: CFURLRef,
        parseOptions: CFOptionFlags,
        versionOfNodes: CFIndex,
        callBacks: *mut CFXMLParserCallBacks,
        context: *mut CFXMLParserContext,
    ) -> CFXMLParserRef;
    pub fn CFXMLParserCreateWithDataFromURL(
        allocator: CFAllocatorRef,
        dataSource: CFURLRef,
        parseOptions: CFOptionFlags,
        versionOfNodes: CFIndex,
        callBacks: *mut CFXMLParserCallBacks,
        context: *mut CFXMLParserContext,
    ) -> CFXMLParserRef;
    pub fn CFXMLParserGetContext(parser: CFXMLParserRef, context: *mut CFXMLParserContext);
    pub fn CFXMLParserGetCallBacks(parser: CFXMLParserRef, callBacks: *mut CFXMLParserCallBacks);
    pub fn CFXMLParserGetSourceURL(parser: CFXMLParserRef) -> CFURLRef;
    pub fn CFXMLParserGetLocation(parser: CFXMLParserRef) -> CFIndex;
    pub fn CFXMLParserGetLineNumber(parser: CFXMLParserRef) -> CFIndex;
    pub fn CFXMLParserGetDocument(parser: CFXMLParserRef) -> *mut c_void;
    pub fn CFXMLParserGetStatusCode(parser: CFXMLParserRef) -> CFXMLParserStatusCode;
    pub fn CFXMLParserCopyErrorDescription(parser: CFXMLParserRef) -> CFStringRef;
    pub fn CFXMLParserAbort(
        parser: CFXMLParserRef,
        errorCode: CFXMLParserStatusCode,
        errorDescription: CFStringRef,
    );
    pub fn CFXMLParserParse(parser: CFXMLParserRef) -> Boolean;
    pub fn CFXMLTreeCreateFromData(
        allocator: CFAllocatorRef,
        xmlData: CFDataRef,
        dataSource: CFURLRef,
        parseOptions: CFOptionFlags,
        versionOfNodes: CFIndex,
    ) -> CFXMLTreeRef;
    pub fn CFXMLTreeCreateFromDataWithError(
        allocator: CFAllocatorRef,
        xmlData: CFDataRef,
        dataSource: CFURLRef,
        parseOptions: CFOptionFlags,
        versionOfNodes: CFIndex,
        errorDict: *mut CFDictionaryRef,
    ) -> CFXMLTreeRef;
    pub fn CFXMLTreeCreateWithDataFromURL(
        allocator: CFAllocatorRef,
        dataSource: CFURLRef,
        parseOptions: CFOptionFlags,
        versionOfNodes: CFIndex,
    ) -> CFXMLTreeRef;
    pub fn CFXMLTreeCreateXMLData(allocator: CFAllocatorRef, xmlTree: CFXMLTreeRef) -> CFDataRef;
    pub fn CFXMLCreateStringByEscapingEntities(
        allocator: CFAllocatorRef,
        string: CFStringRef,
        entitiesDictionary: CFDictionaryRef,
    ) -> CFStringRef;
    pub fn CFXMLCreateStringByUnescapingEntities(
        allocator: CFAllocatorRef,
        string: CFStringRef,
        entitiesDictionary: CFDictionaryRef,
    ) -> CFStringRef;
}
