// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::{c_char, c_void};

use crate::array::CFArrayRef;
use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFTypeID};
use crate::dictionary::CFDictionaryRef;
use crate::string::{CFStringEncoding, CFStringRef};
use crate::tree::CFTreeRef;
use crate::url::CFURLRef;

#[repr(C)]
pub struct __CFXMLNode(c_void);

pub type CFXMLNodeRef = *mut __CFXMLNode;
pub type CFXMLTreeRef = CFTreeRef;

pub const kCFXMLNodeCurrentVersion: CFIndex = 1;

pub type CFXMLNodeTypeCode = CFIndex;
pub const kCFXMLNodeTypeDocument: CFXMLNodeTypeCode = 1;
pub const kCFXMLNodeTypeElement: CFXMLNodeTypeCode = 2;
pub const kCFXMLNodeTypeAttribute: CFXMLNodeTypeCode = 3;
pub const kCFXMLNodeTypeProcessingInstruction: CFXMLNodeTypeCode = 4;
pub const kCFXMLNodeTypeComment: CFXMLNodeTypeCode = 5;
pub const kCFXMLNodeTypeText: CFXMLNodeTypeCode = 6;
pub const kCFXMLNodeTypeCDATASection: CFXMLNodeTypeCode = 7;
pub const kCFXMLNodeTypeDocumentFragment: CFXMLNodeTypeCode = 8;
pub const kCFXMLNodeTypeEntity: CFXMLNodeTypeCode = 9;
pub const kCFXMLNodeTypeEntityReference: CFXMLNodeTypeCode = 10;
pub const kCFXMLNodeTypeDocumentType: CFXMLNodeTypeCode = 11;
pub const kCFXMLNodeTypeWhitespace: CFXMLNodeTypeCode = 12;
pub const kCFXMLNodeTypeNotation: CFXMLNodeTypeCode = 13;
pub const kCFXMLNodeTypeElementTypeDeclaration: CFXMLNodeTypeCode = 14;
pub const kCFXMLNodeTypeAttributeListDeclaration: CFXMLNodeTypeCode = 15;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLElementInfo {
    pub attributes: CFDictionaryRef,
    pub attributeOrder: CFArrayRef,
    pub isEmpty: Boolean,
    pub _reserved: [c_char; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLProcessingInstructionInfo {
    pub dataString: CFStringRef,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLDocumentInfo {
    pub sourceURL: CFURLRef,
    pub encoding: CFStringEncoding,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLExternalID {
    pub systemID: CFURLRef,
    pub publicID: CFStringRef,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLDocumentTypeInfo {
    pub externalID: CFXMLExternalID,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLNotationInfo {
    pub externalID: CFXMLExternalID,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLElementTypeDeclarationInfo {
    pub contentDescription: CFStringRef,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLAttributeDeclarationInfo {
    pub attributeName: CFStringRef,
    pub typeString: CFStringRef,
    pub defaultString: CFStringRef,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLAttributeListDeclarationInfo {
    pub numberOfAttributes: CFIndex,
    pub attributes: *mut CFXMLAttributeDeclarationInfo,
}

pub type CFXMLEntityTypeCode = CFIndex;
pub const kCFXMLEntityTypeParameter: CFXMLEntityTypeCode = 0;
pub const kCFXMLEntityTypeParsedInternal: CFXMLEntityTypeCode = 1;
pub const kCFXMLEntityTypeParsedExternal: CFXMLEntityTypeCode = 2;
pub const kCFXMLEntityTypeUnparsed: CFXMLEntityTypeCode = 3;
pub const kCFXMLEntityTypeCharacter: CFXMLEntityTypeCode = 4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLEntityInfo {
    pub entityType: CFXMLEntityTypeCode,
    pub replacementText: CFStringRef,
    pub entityID: CFXMLExternalID,
    pub notationName: CFStringRef,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFXMLEntityReferenceInfo {
    pub entityType: CFXMLEntityTypeCode,
}

extern "C" {
    /*
     * CFXMLNode.h
     */
    pub fn CFXMLNodeGetTypeID() -> CFTypeID;
    pub fn CFXMLNodeCreate(
        alloc: CFAllocatorRef,
        xmlType: CFXMLNodeTypeCode,
        dataString: CFStringRef,
        additionalInfoPtr: *const c_void,
        version: CFIndex,
    ) -> CFXMLNodeRef;
    pub fn CFXMLNodeCreateCopy(alloc: CFAllocatorRef, origNode: CFXMLNodeRef) -> CFXMLNodeRef;
    pub fn CFXMLNodeGetTypeCode(node: CFXMLNodeRef) -> CFXMLNodeTypeCode;
    pub fn CFXMLNodeGetString(node: CFXMLNodeRef) -> CFStringRef;
    pub fn CFXMLNodeGetInfoPtr(node: CFXMLNodeRef) -> *const c_void;
    pub fn CFXMLNodeGetVersion(node: CFXMLNodeRef) -> CFIndex;
    pub fn CFXMLTreeCreateWithNode(alloc: CFAllocatorRef, node: CFXMLNodeRef) -> CFXMLTreeRef;
    pub fn CFXMLTreeGetNode(xmlTree: CFXMLTreeRef) -> CFXMLNodeRef;
}
