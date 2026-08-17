// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFOptionFlags, CFTypeRef};
use crate::data::CFDataRef;
use crate::error::CFErrorRef;
use crate::stream::{CFReadStreamRef, CFWriteStreamRef};
use crate::string::CFStringRef;

pub type CFPropertyListRef = CFTypeRef;

pub type CFPropertyListFormat = CFIndex;
pub const kCFPropertyListOpenStepFormat: CFPropertyListFormat = 1;
pub const kCFPropertyListXMLFormat_v1_0: CFPropertyListFormat = 100;
pub const kCFPropertyListBinaryFormat_v1_0: CFPropertyListFormat = 200;

pub type CFPropertyListMutabilityOptions = CFOptionFlags;
pub const kCFPropertyListImmutable: CFPropertyListMutabilityOptions = 0;
pub const kCFPropertyListMutableContainers: CFPropertyListMutabilityOptions = 1;
pub const kCFPropertyListMutableContainersAndLeaves: CFPropertyListMutabilityOptions = 2;

/* Reading and Writing Error Codes */
pub const kCFPropertyListReadCorruptError: CFIndex = 3840;
pub const kCFPropertyListReadUnknownVersionError: CFIndex = 3841;
pub const kCFPropertyListReadStreamError: CFIndex = 3842;
pub const kCFPropertyListWriteStreamError: CFIndex = 3851;

extern "C" {
    /*
     * CFPropertyList.h
     */

    /* Creating a Property List */
    pub fn CFPropertyListCreateWithData(
        allocator: CFAllocatorRef,
        data: CFDataRef,
        options: CFPropertyListMutabilityOptions,
        format: *mut CFPropertyListFormat,
        error: *mut CFErrorRef,
    ) -> CFPropertyListRef;
    pub fn CFPropertyListCreateWithStream(
        allocator: CFAllocatorRef,
        stream: CFReadStreamRef,
        streamLength: CFIndex,
        options: CFOptionFlags,
        format: *mut CFPropertyListFormat,
        error: *mut CFErrorRef,
    ) -> CFPropertyListRef;
    pub fn CFPropertyListCreateDeepCopy(
        allocator: CFAllocatorRef,
        propertyList: CFPropertyListRef,
        mutabilityOption: CFOptionFlags,
    ) -> CFPropertyListRef;
    pub fn CFPropertyListCreateFromXMLData(
        allocator: CFAllocatorRef,
        xmlData: CFDataRef,
        mutabilityOption: CFOptionFlags,
        errorString: *mut CFStringRef,
    ) -> CFPropertyListRef; // deprecated
    pub fn CFPropertyListCreateFromStream(
        allocator: CFAllocatorRef,
        stream: CFReadStreamRef,
        streamLength: CFIndex,
        mutabilityOption: CFOptionFlags,
        format: *mut CFPropertyListFormat,
        errorString: *mut CFStringRef,
    ) -> CFPropertyListRef; // deprecated

    /* Exporting a Property List */
    pub fn CFPropertyListCreateData(
        allocator: CFAllocatorRef,
        propertyList: CFPropertyListRef,
        format: CFPropertyListFormat,
        options: CFOptionFlags,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
    pub fn CFPropertyListWrite(
        propertyList: CFPropertyListRef,
        stream: CFWriteStreamRef,
        format: CFPropertyListFormat,
        options: CFOptionFlags,
        error: *mut CFErrorRef,
    ) -> CFIndex;
    pub fn CFPropertyListCreateXMLData(
        allocator: CFAllocatorRef,
        propertyList: CFPropertyListRef,
    ) -> CFDataRef; // deprecated
    pub fn CFPropertyListWriteToStream(
        propertyList: CFPropertyListRef,
        stream: CFWriteStreamRef,
        format: CFPropertyListFormat,
        errorString: *mut CFStringRef,
    ) -> CFIndex;

    /* Validating a Property List */
    pub fn CFPropertyListIsValid(plist: CFPropertyListRef, format: CFPropertyListFormat)
        -> Boolean;
}
