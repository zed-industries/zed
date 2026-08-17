// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::array::CFArrayRef;
#[cfg(target_os = "macos")]
use crate::base::SInt32;
use crate::base::{Boolean, CFAllocatorRef, CFTypeID, CFTypeRef, UInt32};
use crate::dictionary::CFDictionaryRef;
use crate::error::CFErrorRef;
use crate::string::CFStringRef;
use crate::url::CFURLRef;
use std::os::raw::{c_int, c_uint};

#[repr(C)]
pub struct __CFBundle(c_void);

pub type CFBundleRef = *mut __CFBundle;
pub type CFPlugInRef = *mut __CFBundle;
pub type CFBundleRefNum = c_int;

#[allow(unused)]
#[inline(always)]
pub unsafe fn CFCopyLocalizedString(key: CFStringRef, comment: CFStringRef) -> CFStringRef {
    CFBundleCopyLocalizedString(CFBundleGetMainBundle(), key, key, std::ptr::null())
}
#[allow(unused)]
#[inline(always)]
pub unsafe fn CFCopyLocalizedStringFromTable(
    key: CFStringRef,
    tbl: CFStringRef,
    comment: CFStringRef,
) -> CFStringRef {
    CFBundleCopyLocalizedString(CFBundleGetMainBundle(), key, key, tbl)
}
#[allow(unused)]
#[inline(always)]
pub unsafe fn CFCopyLocalizedStringFromTableInBundle(
    key: CFStringRef,
    tbl: CFStringRef,
    bundle: CFBundleRef,
    comment: CFStringRef,
) -> CFStringRef {
    CFBundleCopyLocalizedString(bundle, key, key, tbl)
}
#[allow(unused)]
#[inline(always)]
pub unsafe fn CFCopyLocalizedStringWithDefaultValue(
    key: CFStringRef,
    tbl: CFStringRef,
    bundle: CFBundleRef,
    value: CFStringRef,
    comment: CFStringRef,
) -> CFStringRef {
    CFBundleCopyLocalizedString(bundle, key, value, tbl)
}

pub static kCFBundleExecutableArchitectureI386: c_uint = 0x00000007;
pub static kCFBundleExecutableArchitecturePPC: c_uint = 0x00000012;
pub static kCFBundleExecutableArchitectureX86_64: c_uint = 0x01000007;
pub static kCFBundleExecutableArchitecturePPC64: c_uint = 0x01000012;
//pub static kCFBundleExecutableArchitectureARM64: c_uint = 0x0100000c; //macos(11.0)+

extern "C" {
    /*
     * CFBundle.h
     */

    /* Information Property List Keys */
    pub static kCFBundleInfoDictionaryVersionKey: CFStringRef;
    pub static kCFBundleExecutableKey: CFStringRef;
    pub static kCFBundleIdentifierKey: CFStringRef;
    pub static kCFBundleVersionKey: CFStringRef;
    pub static kCFBundleDevelopmentRegionKey: CFStringRef;
    pub static kCFBundleNameKey: CFStringRef;
    pub static kCFBundleLocalizationsKey: CFStringRef;

    /* Creating and Accessing Bundles */
    pub fn CFBundleCreate(allocator: CFAllocatorRef, bundleURL: CFURLRef) -> CFBundleRef;
    pub fn CFBundleCreateBundlesFromDirectory(
        allocator: CFAllocatorRef,
        directoryURL: CFURLRef,
        bundleType: CFStringRef,
    ) -> CFArrayRef;
    pub fn CFBundleGetAllBundles() -> CFArrayRef;
    pub fn CFBundleGetBundleWithIdentifier(bundleID: CFStringRef) -> CFBundleRef;
    pub fn CFBundleGetMainBundle() -> CFBundleRef;

    /* Loading and Unloading a Bundle */
    pub fn CFBundleIsExecutableLoaded(bundle: CFBundleRef) -> Boolean;
    pub fn CFBundlePreflightExecutable(bundle: CFBundleRef, error: *mut CFErrorRef) -> Boolean;
    pub fn CFBundleLoadExecutable(bundle: CFBundleRef) -> Boolean;
    pub fn CFBundleLoadExecutableAndReturnError(
        bundle: CFBundleRef,
        error: *mut CFErrorRef,
    ) -> Boolean;
    pub fn CFBundleUnloadExecutable(bundle: CFBundleRef);

    /* Finding Locations in a Bundle */
    pub fn CFBundleCopyAuxiliaryExecutableURL(
        bundle: CFBundleRef,
        executableName: CFStringRef,
    ) -> CFURLRef;
    pub fn CFBundleCopyBuiltInPlugInsURL(bundle: CFBundleRef) -> CFURLRef;
    pub fn CFBundleCopyExecutableURL(bundle: CFBundleRef) -> CFURLRef;
    pub fn CFBundleCopyPrivateFrameworksURL(bundle: CFBundleRef) -> CFURLRef;
    pub fn CFBundleCopyResourcesDirectoryURL(bundle: CFBundleRef) -> CFURLRef;
    pub fn CFBundleCopySharedFrameworksURL(bundle: CFBundleRef) -> CFURLRef;
    pub fn CFBundleCopySharedSupportURL(bundle: CFBundleRef) -> CFURLRef;
    pub fn CFBundleCopySupportFilesDirectoryURL(bundle: CFBundleRef) -> CFURLRef;

    /* Locating Bundle Resources */
    #[cfg(target_os = "macos")]
    pub fn CFBundleCloseBundleResourceMap(bundle: CFBundleRef, refNum: CFBundleRefNum); // DEPRECATED macosx(10.0, 10.15)
    pub fn CFBundleCopyResourceURL(
        bundle: CFBundleRef,
        resourceName: CFStringRef,
        resourceType: CFStringRef,
        subDirName: CFStringRef,
    ) -> CFURLRef;
    pub fn CFBundleCopyResourceURLInDirectory(
        bundleURL: CFURLRef,
        resourceName: CFStringRef,
        resourceType: CFStringRef,
        subDirName: CFStringRef,
    ) -> CFURLRef;
    pub fn CFBundleCopyResourceURLsOfType(
        bundle: CFBundleRef,
        resourceType: CFStringRef,
        subDirName: CFStringRef,
    ) -> CFArrayRef;
    pub fn CFBundleCopyResourceURLsOfTypeInDirectory(
        bundleURL: CFURLRef,
        resourceType: CFStringRef,
        subDirName: CFStringRef,
    ) -> CFArrayRef;
    pub fn CFBundleCopyResourceURLForLocalization(
        bundle: CFBundleRef,
        resourceName: CFStringRef,
        resourceType: CFStringRef,
        subDirName: CFStringRef,
        localizationName: CFStringRef,
    ) -> CFURLRef;
    pub fn CFBundleCopyResourceURLsOfTypeForLocalization(
        bundle: CFBundleRef,
        resourceType: CFStringRef,
        subDirName: CFStringRef,
        localizationName: CFStringRef,
    ) -> CFArrayRef;
    #[cfg(target_os = "macos")]
    pub fn CFBundleOpenBundleResourceFiles(
        bundle: CFBundleRef,
        refNum: *mut CFBundleRefNum,
        localizedRefNum: *mut CFBundleRefNum,
    ) -> SInt32; // DEPRECATED macosx(10.0, 10.15)
    #[cfg(target_os = "macos")]
    pub fn CFBundleOpenBundleResourceMap(bundle: CFBundleRef) -> CFBundleRefNum; // DEPRECATED macosx(10.0, 10.15)

    /* Managing Localizations */
    pub fn CFBundleCopyBundleLocalizations(bundle: CFBundleRef) -> CFArrayRef;
    pub fn CFBundleCopyLocalizedString(
        bundle: CFBundleRef,
        key: CFStringRef,
        value: CFStringRef,
        tableName: CFStringRef,
    ) -> CFStringRef;
    pub fn CFBundleCopyLocalizationsForPreferences(
        locArray: CFArrayRef,
        prefArray: CFArrayRef,
    ) -> CFArrayRef;
    pub fn CFBundleCopyLocalizationsForURL(url: CFURLRef) -> CFArrayRef;
    pub fn CFBundleCopyPreferredLocalizationsFromArray(locArray: CFArrayRef) -> CFArrayRef;

    /* Managing Executable Code */
    pub fn CFBundleGetDataPointerForName(
        bundle: CFBundleRef,
        symbolName: CFStringRef,
    ) -> *mut c_void;
    pub fn CFBundleGetDataPointersForNames(
        bundle: CFBundleRef,
        symbolNames: CFArrayRef,
        stbl: *mut [c_void],
    );
    pub fn CFBundleGetFunctionPointerForName(
        bundle: CFBundleRef,
        function_name: CFStringRef,
    ) -> *const c_void;
    pub fn CFBundleGetFunctionPointersForNames(
        bundle: CFBundleRef,
        functionNames: CFArrayRef,
        ftbl: *mut [c_void],
    );
    pub fn CFBundleGetPlugIn(bundle: CFBundleRef) -> CFPlugInRef;

    /* Getting Bundle Properties */
    pub fn CFBundleCopyBundleURL(bundle: CFBundleRef) -> CFURLRef;
    pub fn CFBundleGetDevelopmentRegion(bundle: CFBundleRef) -> CFStringRef;
    pub fn CFBundleGetIdentifier(bundle: CFBundleRef) -> CFStringRef;
    pub fn CFBundleGetInfoDictionary(bundle: CFBundleRef) -> CFDictionaryRef;
    pub fn CFBundleGetLocalInfoDictionary(bundle: CFBundleRef) -> CFDictionaryRef;
    pub fn CFBundleGetValueForInfoDictionaryKey(bundle: CFBundleRef, key: CFStringRef)
        -> CFTypeRef;
    pub fn CFBundleCopyInfoDictionaryInDirectory(bundleURL: CFURLRef) -> CFDictionaryRef;
    pub fn CFBundleCopyInfoDictionaryForURL(url: CFURLRef) -> CFDictionaryRef;
    pub fn CFBundleGetPackageInfo(
        bundle: CFBundleRef,
        packageType: *mut UInt32,
        packageCreator: *mut UInt32,
    );
    pub fn CFBundleGetPackageInfoInDirectory(
        url: CFURLRef,
        packageType: *mut UInt32,
        packageCreator: *mut UInt32,
    ) -> Boolean;
    pub fn CFBundleCopyExecutableArchitectures(bundle: CFBundleRef) -> CFArrayRef;
    pub fn CFBundleCopyExecutableArchitecturesForURL(url: CFURLRef) -> CFArrayRef;
    pub fn CFBundleGetVersionNumber(bundle: CFBundleRef) -> UInt32;

    /* macos(11.0)+
    pub fn CFBundleIsExecutableLoadable(bundle: CFBundleRef) -> Boolean;
    pub fn CFBundleIsExecutableLoadableForURL(url: CFURLRef) -> Boolean;
    pub fn CFBundleIsArchitectureLoadable(arch: cpu_type_t) -> Boolean;
    */

    /* Getting the CFBundle Type ID */
    pub fn CFBundleGetTypeID() -> CFTypeID;
}
