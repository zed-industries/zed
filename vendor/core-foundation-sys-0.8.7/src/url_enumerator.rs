// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::array::CFArrayRef;
use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFOptionFlags, CFTypeID};
use crate::error::CFErrorRef;
use crate::url::CFURLRef;

#[repr(C)]
pub struct __CFURLEnumerator(c_void);

pub type CFURLEnumeratorRef = *mut __CFURLEnumerator;

pub type CFURLEnumeratorOptions = CFOptionFlags;
pub const kCFURLEnumeratorDefaultBehavior: CFURLEnumeratorOptions = 0;
pub const kCFURLEnumeratorDescendRecursively: CFURLEnumeratorOptions = 1 << 0;
pub const kCFURLEnumeratorSkipInvisibles: CFURLEnumeratorOptions = 1 << 1;
pub const kCFURLEnumeratorGenerateFileReferenceURLs: CFURLEnumeratorOptions = 1 << 2;
pub const kCFURLEnumeratorSkipPackageContents: CFURLEnumeratorOptions = 1 << 3;
pub const kCFURLEnumeratorIncludeDirectoriesPreOrder: CFURLEnumeratorOptions = 1 << 4;
pub const kCFURLEnumeratorIncludeDirectoriesPostOrder: CFURLEnumeratorOptions = 1 << 5;
//pub const kCFURLEnumeratorGenerateRelativePathURLs = 1UL << 6; // macos(10.15)+

pub type CFURLEnumeratorResult = CFIndex;
pub const kCFURLEnumeratorSuccess: CFURLEnumeratorOptions = 1;
pub const kCFURLEnumeratorEnd: CFURLEnumeratorOptions = 2;
pub const kCFURLEnumeratorError: CFURLEnumeratorOptions = 3;
pub const kCFURLEnumeratorDirectoryPostOrderSuccess: CFURLEnumeratorOptions = 4;

extern "C" {
    /*
     * CFURLEnumerator.h
     */
    pub fn CFURLEnumeratorGetTypeID() -> CFTypeID;
    pub fn CFURLEnumeratorCreateForDirectoryURL(
        alloc: CFAllocatorRef,
        directoryURL: CFURLRef,
        option: CFURLEnumeratorOptions,
        propertyKeys: CFArrayRef,
    ) -> CFURLEnumeratorRef;
    pub fn CFURLEnumeratorCreateForMountedVolumes(
        alloc: CFAllocatorRef,
        option: CFURLEnumeratorOptions,
        propertyKeys: CFArrayRef,
    ) -> CFURLEnumeratorRef;
    pub fn CFURLEnumeratorGetNextURL(
        enumerator: CFURLEnumeratorRef,
        url: *mut CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFURLEnumeratorResult;
    pub fn CFURLEnumeratorSkipDescendents(enumerator: CFURLEnumeratorRef);
    pub fn CFURLEnumeratorGetDescendentLevel(enumerator: CFURLEnumeratorRef) -> CFIndex;
    pub fn CFURLEnumeratorGetSourceDidChange(enumerator: CFURLEnumeratorRef) -> Boolean; // deprecated since macos 10.7
}
