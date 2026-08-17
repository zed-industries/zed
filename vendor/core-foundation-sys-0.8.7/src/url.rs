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
use crate::base::{
    Boolean, CFAllocatorRef, CFIndex, CFOptionFlags, CFRange, CFTypeID, CFTypeRef, SInt32,
};
use crate::data::CFDataRef;
use crate::dictionary::CFDictionaryRef;
use crate::error::CFErrorRef;
use crate::string::{CFStringEncoding, CFStringRef};

#[repr(C)]
pub struct __CFURL(c_void);

pub type CFURLRef = *const __CFURL;

pub type CFURLBookmarkCreationOptions = CFOptionFlags;
pub type CFURLBookmarkResolutionOptions = CFOptionFlags;
pub type CFURLBookmarkFileCreationOptions = CFOptionFlags;

pub type CFURLPathStyle = CFIndex;

/* typedef CF_ENUM(CFIndex, CFURLPathStyle) */
pub const kCFURLPOSIXPathStyle: CFURLPathStyle = 0;
pub const kCFURLHFSPathStyle: CFURLPathStyle = 1;
pub const kCFURLWindowsPathStyle: CFURLPathStyle = 2;

/* Bookmark Data Creation Options */
pub static kCFURLBookmarkCreationMinimalBookmarkMask: CFURLBookmarkCreationOptions =
    (1u32 << 9) as usize;
pub static kCFURLBookmarkCreationSuitableForBookmarkFile: CFURLBookmarkCreationOptions =
    (1u32 << 10) as usize;

#[cfg(target_os = "macos")]
pub static kCFURLBookmarkCreationWithSecurityScope: CFURLBookmarkCreationOptions =
    (1u32 << 11) as usize;

#[cfg(target_os = "macos")]
pub static kCFURLBookmarkCreationSecurityScopeAllowOnlyReadAccess: CFURLBookmarkCreationOptions =
    (1u32 << 12) as usize;

pub static kCFURLBookmarkCreationWithoutImplicitSecurityScope: CFURLBookmarkCreationOptions =
    (1u32 << 29) as usize;

pub static kCFURLBookmarkCreationPreferFileIDResolutionMask: CFURLBookmarkCreationOptions =
    (1u32 << 8) as usize; // deprecated

/* The types of components in a URL. */
pub type CFURLComponentType = CFIndex;
pub const kCFURLComponentScheme: CFIndex = 1;
pub const kCFURLComponentNetLocation: CFIndex = 2;
pub const kCFURLComponentPath: CFIndex = 3;
pub const kCFURLComponentResourceSpecifier: CFIndex = 4;
pub const kCFURLComponentUser: CFIndex = 5;
pub const kCFURLComponentPassword: CFIndex = 6;
pub const kCFURLComponentUserInfo: CFIndex = 7;
pub const kCFURLComponentHost: CFIndex = 8;
pub const kCFURLComponentPort: CFIndex = 9;
pub const kCFURLComponentParameterString: CFIndex = 10;
pub const kCFURLComponentQuery: CFIndex = 11;
pub const kCFURLComponentFragment: CFIndex = 12;

/* Bookmark Data Resolution Options */
pub const kCFURLBookmarkResolutionWithoutUIMask: CFURLBookmarkResolutionOptions =
    (1u32 << 8) as usize;
pub const kCFURLBookmarkResolutionWithoutMountingMask: CFURLBookmarkResolutionOptions =
    (1u32 << 9) as usize;
#[cfg(target_os = "macos")]
pub const kCFURLBookmarkResolutionWithSecurityScope: CFURLBookmarkResolutionOptions =
    (1u32 << 10) as usize;
//pub const kCFURLBookmarkResolutionWithoutImplicitStartAccessing: CFURLBookmarkResolutionOptions = ( 1u32 << 15 ) as usize; // macos(11.2)+
pub const kCFBookmarkResolutionWithoutUIMask: CFURLBookmarkResolutionOptions = (1u32 << 8) as usize;
pub const kCFBookmarkResolutionWithoutMountingMask: CFURLBookmarkResolutionOptions =
    (1u32 << 9) as usize;

extern "C" {
    /*
     * CFURL.h
     */

    /* Common File System Resource Keys */
    pub static kCFURLNameKey: CFStringRef;
    pub static kCFURLLocalizedNameKey: CFStringRef;
    pub static kCFURLIsRegularFileKey: CFStringRef;
    pub static kCFURLIsDirectoryKey: CFStringRef;
    pub static kCFURLIsSymbolicLinkKey: CFStringRef;
    pub static kCFURLIsVolumeKey: CFStringRef;
    pub static kCFURLIsPackageKey: CFStringRef;
    pub static kCFURLIsApplicationKey: CFStringRef;
    // pub static kCFURLApplicationIsScriptableKey: CFStringRef; //macos(10.11)+

    pub static kCFURLIsSystemImmutableKey: CFStringRef;
    pub static kCFURLIsUserImmutableKey: CFStringRef;
    pub static kCFURLIsHiddenKey: CFStringRef;
    pub static kCFURLHasHiddenExtensionKey: CFStringRef;
    pub static kCFURLCreationDateKey: CFStringRef;
    pub static kCFURLContentAccessDateKey: CFStringRef;
    pub static kCFURLContentModificationDateKey: CFStringRef;
    pub static kCFURLAttributeModificationDateKey: CFStringRef;
    // pub static kCFURLFileIdentifierKey: CFStringRef; //macos(13.3)+
    // pub static kCFURLFileContentIdentifierKey: CFStringRef; //macos(11.0)+
    // pub static kCFURLMayShareFileContentKey: CFStringRef; //macos(11.0)+
    // pub static kCFURLMayHaveExtendedAttributesKey: CFStringRef; //macos(11.0)+
    // pub static kCFURLIsPurgeableKey: CFStringRef; //macos(11.0)+
    // pub static kCFURLIsSparseKey: CFStringRef; //macos(11.0)+

    pub static kCFURLLinkCountKey: CFStringRef;
    pub static kCFURLParentDirectoryURLKey: CFStringRef;
    pub static kCFURLVolumeURLKey: CFStringRef;

    pub static kCFURLTypeIdentifierKey: CFStringRef; //deprecated

    pub static kCFURLLocalizedTypeDescriptionKey: CFStringRef;
    pub static kCFURLLabelNumberKey: CFStringRef;
    pub static kCFURLLabelColorKey: CFStringRef; //deprecated
    pub static kCFURLLocalizedLabelKey: CFStringRef;
    pub static kCFURLEffectiveIconKey: CFStringRef; //deprecated
    pub static kCFURLCustomIconKey: CFStringRef; //deprecated

    pub static kCFURLFileResourceIdentifierKey: CFStringRef;
    pub static kCFURLVolumeIdentifierKey: CFStringRef;
    pub static kCFURLPreferredIOBlockSizeKey: CFStringRef;
    pub static kCFURLIsReadableKey: CFStringRef;
    pub static kCFURLIsWritableKey: CFStringRef;
    pub static kCFURLIsExecutableKey: CFStringRef;
    pub static kCFURLFileSecurityKey: CFStringRef;

    #[cfg(feature = "mac_os_10_8_features")]
    #[cfg_attr(feature = "mac_os_10_7_support", linkage = "extern_weak")]
    pub static kCFURLIsExcludedFromBackupKey: CFStringRef;
    // pub static kCFURLTagNamesKey: CFStringRef; //macos(10.9)+
    #[cfg(feature = "mac_os_10_8_features")]
    #[cfg_attr(feature = "mac_os_10_7_support", linkage = "extern_weak")]
    pub static kCFURLPathKey: CFStringRef; // macos(10.8)+
    pub static kCFURLCanonicalPathKey: CFStringRef; // macos(10.12)+

    pub static kCFURLIsMountTriggerKey: CFStringRef;

    // pub static kCFURLGenerationIdentifierKey: CFStringRef; // macos(10.10)+
    // pub static kCFURLDocumentIdentifierKey: CFStringRef; // macos(10.10)+
    // pub static kCFURLAddedToDirectoryDateKey: CFStringRef; // macos(10.10)+
    // pub static kCFURLQuarantinePropertiesKey: CFStringRef; // macos(10.10)+

    pub static kCFURLFileResourceTypeKey: CFStringRef;

    /* File Resource Types. The file system object type values returned for the kCFURLFileResourceTypeKey */
    pub static kCFURLFileResourceTypeNamedPipe: CFStringRef;
    pub static kCFURLFileResourceTypeCharacterSpecial: CFStringRef;
    pub static kCFURLFileResourceTypeDirectory: CFStringRef;
    pub static kCFURLFileResourceTypeBlockSpecial: CFStringRef;
    pub static kCFURLFileResourceTypeRegular: CFStringRef;
    pub static kCFURLFileResourceTypeSymbolicLink: CFStringRef;
    pub static kCFURLFileResourceTypeSocket: CFStringRef;
    pub static kCFURLFileResourceTypeUnknown: CFStringRef;

    /* File Property Keys */
    pub static kCFURLFileSizeKey: CFStringRef;
    pub static kCFURLFileAllocatedSizeKey: CFStringRef;
    pub static kCFURLTotalFileSizeKey: CFStringRef;
    pub static kCFURLTotalFileAllocatedSizeKey: CFStringRef;
    pub static kCFURLIsAliasFileKey: CFStringRef;

    // pub static kCFURLFileProtectionKey: CFStringRef; // ios(9.0)+

    /* The protection level values returned for the kCFURLFileProtectionKey */
    // pub static kCFURLFileProtectionNone: CFStringRef; // ios(9.0)+
    // pub static kCFURLFileProtectionComplete: CFStringRef; // ios(9.0)+
    // pub static kCFURLFileProtectionCompleteUnlessOpen: CFStringRef; // ios(9.0)+
    // pub static kCFURLFileProtectionCompleteUntilFirstUserAuthentication: CFStringRef; // ios(9.0)+

    /* Volume Property Keys */
    pub static kCFURLVolumeLocalizedFormatDescriptionKey: CFStringRef;
    pub static kCFURLVolumeTotalCapacityKey: CFStringRef;
    pub static kCFURLVolumeAvailableCapacityKey: CFStringRef;
    //pub static kCFURLVolumeAvailableCapacityForImportantUsageKey: CFStringRef; //macos(10.13)+
    //pub static kCFURLVolumeAvailableCapacityForOpportunisticUsageKey: CFStringRef; //macos(10.13)+

    pub static kCFURLVolumeResourceCountKey: CFStringRef;
    pub static kCFURLVolumeSupportsPersistentIDsKey: CFStringRef;
    pub static kCFURLVolumeSupportsSymbolicLinksKey: CFStringRef;
    pub static kCFURLVolumeSupportsHardLinksKey: CFStringRef;
    pub static kCFURLVolumeSupportsJournalingKey: CFStringRef;
    pub static kCFURLVolumeIsJournalingKey: CFStringRef;
    pub static kCFURLVolumeSupportsSparseFilesKey: CFStringRef;
    pub static kCFURLVolumeSupportsZeroRunsKey: CFStringRef;
    pub static kCFURLVolumeSupportsCaseSensitiveNamesKey: CFStringRef;
    pub static kCFURLVolumeSupportsCasePreservedNamesKey: CFStringRef;
    pub static kCFURLVolumeSupportsRootDirectoryDatesKey: CFStringRef;
    pub static kCFURLVolumeSupportsVolumeSizesKey: CFStringRef;
    pub static kCFURLVolumeSupportsRenamingKey: CFStringRef;
    pub static kCFURLVolumeSupportsAdvisoryFileLockingKey: CFStringRef;
    pub static kCFURLVolumeSupportsExtendedSecurityKey: CFStringRef;
    pub static kCFURLVolumeIsBrowsableKey: CFStringRef;
    pub static kCFURLVolumeMaximumFileSizeKey: CFStringRef;
    pub static kCFURLVolumeIsEjectableKey: CFStringRef;
    pub static kCFURLVolumeIsRemovableKey: CFStringRef;
    pub static kCFURLVolumeIsInternalKey: CFStringRef;
    pub static kCFURLVolumeIsAutomountedKey: CFStringRef;
    pub static kCFURLVolumeIsLocalKey: CFStringRef;
    pub static kCFURLVolumeIsReadOnlyKey: CFStringRef;
    pub static kCFURLVolumeCreationDateKey: CFStringRef;
    pub static kCFURLVolumeURLForRemountingKey: CFStringRef;
    pub static kCFURLVolumeUUIDStringKey: CFStringRef;
    pub static kCFURLVolumeNameKey: CFStringRef;
    pub static kCFURLVolumeLocalizedNameKey: CFStringRef;
    // pub static kCFURLVolumeIsEncryptedKey: CFStringRef; //macos(10.12)+
    // pub static kCFURLVolumeIsRootFileSystemKey: CFStringRef; //macos(10.12)+
    // pub static kCFURLVolumeSupportsCompressionKey: CFStringRef; //macos(10.12)+
    // pub static kCFURLVolumeSupportsFileCloningKey: CFStringRef; //macos(10.12)+
    // pub static kCFURLVolumeSupportsSwapRenamingKey: CFStringRef; //macos(10.12)+
    // pub static kCFURLVolumeSupportsExclusiveRenamingKey: CFStringRef; //macos(10.12)+
    // pub static kCFURLVolumeSupportsImmutableFilesKey: CFStringRef; //macos(10.13)+
    // pub static kCFURLVolumeSupportsAccessPermissionsKey: CFStringRef; //macos(10.13)+
    // pub static kCFURLVolumeSupportsFileProtectionKey: CFStringRef;  //macos(11.0)+
    // pub static kCFURLVolumeTypeNameKey: CFStringRef;  //macos(13.3)+
    // pub static kCFURLVolumeSubtypeKey: CFStringRef; //macos(13.3)+
    // pub static kCFURLVolumeMountFromLocationKey: CFStringRef; //macos(13.3)+

    /* iCloud Constants */
    pub static kCFURLIsUbiquitousItemKey: CFStringRef;
    pub static kCFURLUbiquitousItemHasUnresolvedConflictsKey: CFStringRef;
    pub static kCFURLUbiquitousItemIsDownloadedKey: CFStringRef; // deprecated
    pub static kCFURLUbiquitousItemIsDownloadingKey: CFStringRef;
    pub static kCFURLUbiquitousItemIsUploadedKey: CFStringRef;
    pub static kCFURLUbiquitousItemIsUploadingKey: CFStringRef;
    pub static kCFURLUbiquitousItemPercentDownloadedKey: CFStringRef; // deprecated
    pub static kCFURLUbiquitousItemPercentUploadedKey: CFStringRef; // deprecated
                                                                    // pub static kCFURLUbiquitousItemDownloadingStatusKey: CFStringRef; // macos(10.9)+
                                                                    // pub static kCFURLUbiquitousItemDownloadingErrorKey: CFStringRef; // macos(10.9)+
                                                                    // pub static kCFURLUbiquitousItemUploadingErrorKey: CFStringRef; // macos(10.9)+
                                                                    // pub static kCFURLUbiquitousItemIsExcludedFromSyncKey: CFStringRef; // macos(11.3)+

    /* The values returned for kCFURLUbiquitousItemDownloadingStatusKey */
    // pub static kCFURLUbiquitousItemDownloadingStatusNotDownloaded: CFStringRef; // macos(10.9)+
    // pub static kCFURLUbiquitousItemDownloadingStatusDownloaded: CFStringRef; // macos(10.9)+
    // pub static kCFURLUbiquitousItemDownloadingStatusCurrent: CFStringRef; // macos(10.9)+

    /* CFError userInfo Dictionary Keys */
    pub static kCFURLKeysOfUnsetValuesKey: CFStringRef;

    /* Creating a CFURL */
    pub fn CFURLCopyAbsoluteURL(anURL: CFURLRef) -> CFURLRef;
    pub fn CFURLCreateAbsoluteURLWithBytes(
        allocator: CFAllocatorRef,
        relativeURLBytes: *const u8,
        length: CFIndex,
        encoding: CFStringEncoding,
        baseURL: CFURLRef,
        useCompatibilityMode: Boolean,
    ) -> CFURLRef;
    pub fn CFURLCreateByResolvingBookmarkData(
        allocator: CFAllocatorRef,
        bookmark: CFDataRef,
        options: CFURLBookmarkResolutionOptions,
        relativeToURL: CFURLRef,
        resourcePropertiesToInclude: CFArrayRef,
        isStale: *mut Boolean,
        error: *mut CFErrorRef,
    ) -> CFURLRef;
    pub fn CFURLCreateCopyAppendingPathComponent(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        pathComponent: CFStringRef,
        isDirectory: Boolean,
    ) -> CFURLRef;
    pub fn CFURLCreateCopyAppendingPathExtension(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        extension: CFStringRef,
    ) -> CFURLRef;
    pub fn CFURLCreateCopyDeletingLastPathComponent(
        allocator: CFAllocatorRef,
        url: CFURLRef,
    ) -> CFURLRef;
    pub fn CFURLCreateCopyDeletingPathExtension(
        allocator: CFAllocatorRef,
        url: CFURLRef,
    ) -> CFURLRef;
    pub fn CFURLCreateFilePathURL(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFURLRef;
    pub fn CFURLCreateFileReferenceURL(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFURLRef;
    pub fn CFURLCreateFromFileSystemRepresentation(
        allocator: CFAllocatorRef,
        buffer: *const u8,
        bufLen: CFIndex,
        isDirectory: Boolean,
    ) -> CFURLRef;
    pub fn CFURLCreateFromFileSystemRepresentationRelativeToBase(
        allocator: CFAllocatorRef,
        buffer: *const u8,
        bufLen: CFIndex,
        isDirectory: Boolean,
        baseURL: CFURLRef,
    ) -> CFURLRef;
    //pub fn CFURLCreateFromFSRef(allocator: CFAllocatorRef, fsRef: *const FSRef) -> CFURLRef
    pub fn CFURLCreateWithBytes(
        allocator: CFAllocatorRef,
        URLBytes: *const u8,
        length: CFIndex,
        encoding: CFStringEncoding,
        baseURL: CFURLRef,
    ) -> CFURLRef;
    pub fn CFURLCreateWithFileSystemPath(
        allocator: CFAllocatorRef,
        filePath: CFStringRef,
        pathStyle: CFURLPathStyle,
        isDirectory: Boolean,
    ) -> CFURLRef;
    pub fn CFURLCreateWithFileSystemPathRelativeToBase(
        allocator: CFAllocatorRef,
        filePath: CFStringRef,
        pathStyle: CFURLPathStyle,
        isDirectory: Boolean,
        baseURL: CFURLRef,
    ) -> CFURLRef;
    pub fn CFURLCreateWithString(
        allocator: CFAllocatorRef,
        URLString: CFStringRef,
        baseURL: CFURLRef,
    ) -> CFURLRef;

    /* Accessing the Parts of a URL */
    pub fn CFURLCanBeDecomposed(anURL: CFURLRef) -> Boolean;
    pub fn CFURLCopyFileSystemPath(anURL: CFURLRef, pathStyle: CFURLPathStyle) -> CFStringRef;
    pub fn CFURLCopyFragment(anURL: CFURLRef, charactersToLeaveEscaped: CFStringRef)
        -> CFStringRef;
    pub fn CFURLCopyHostName(anURL: CFURLRef) -> CFStringRef;
    pub fn CFURLCopyLastPathComponent(anURL: CFURLRef) -> CFStringRef;
    pub fn CFURLCopyNetLocation(anURL: CFURLRef) -> CFStringRef;
    pub fn CFURLCopyParameterString(
        anURL: CFURLRef,
        charactersToLeaveEscaped: CFStringRef,
    ) -> CFStringRef; // deprecated
    pub fn CFURLCopyPassword(anURL: CFURLRef) -> CFStringRef;
    pub fn CFURLCopyPath(anURL: CFURLRef) -> CFStringRef;
    pub fn CFURLCopyPathExtension(anURL: CFURLRef) -> CFStringRef;
    pub fn CFURLCopyQueryString(
        anURL: CFURLRef,
        charactersToLeaveEscaped: CFStringRef,
    ) -> CFStringRef;
    pub fn CFURLCopyResourceSpecifier(anURL: CFURLRef) -> CFStringRef;
    pub fn CFURLCopyScheme(anURL: CFURLRef) -> CFStringRef;
    pub fn CFURLCopyStrictPath(anURL: CFURLRef, isAbsolute: *mut Boolean) -> CFStringRef;
    pub fn CFURLCopyUserName(anURL: CFURLRef) -> CFStringRef;
    pub fn CFURLGetPortNumber(anURL: CFURLRef) -> SInt32;
    pub fn CFURLHasDirectoryPath(anURL: CFURLRef) -> Boolean;

    /* Converting URLs to Other Representations */
    pub fn CFURLCreateData(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        encoding: CFStringEncoding,
        escapeWhitespace: Boolean,
    ) -> CFDataRef;
    pub fn CFURLCreateStringByAddingPercentEscapes(
        allocator: CFAllocatorRef,
        originalString: CFStringRef,
        charactersToLeaveUnescaped: CFStringRef,
        legalURLCharactersToBeEscaped: CFStringRef,
        encoding: CFStringEncoding,
    ) -> CFStringRef; // API_DEPRECATED("Use [NSString stringByAddingPercentEncodingWithAllowedCharacters:] instead, which always uses the recommended UTF-8 encoding, and which encodes for a specific URL component or subcomponent (since each URL component or subcomponent has different rules for what characters are valid).", macos(10.0,10.11), ios(2.0,9.0), watchos(2.0,2.0), tvos(9.0,9.0));
    pub fn CFURLCreateStringByReplacingPercentEscapes(
        allocator: CFAllocatorRef,
        originalString: CFStringRef,
        charactersToLeaveEscaped: CFStringRef,
    ) -> CFStringRef;
    pub fn CFURLCreateStringByReplacingPercentEscapesUsingEncoding(
        allocator: CFAllocatorRef,
        origString: CFStringRef,
        charsToLeaveEscaped: CFStringRef,
        encoding: CFStringEncoding,
    ) -> CFStringRef; // deprecated
    pub fn CFURLGetFileSystemRepresentation(
        anURL: CFURLRef,
        resolveAgainstBase: Boolean,
        buffer: *mut u8,
        maxBufLen: CFIndex,
    ) -> Boolean;
    //pub fn CFURLIsFileReferenceURL(url: CFURLRef) -> Boolean; // macos(10.9)+
    //pub fn CFURLGetFSRef(url: CFURLRef, fsRef: *mut FSRef) -> Boolean;
    pub fn CFURLGetString(anURL: CFURLRef) -> CFStringRef;

    /* Getting URL Properties */
    pub fn CFURLGetBaseURL(anURL: CFURLRef) -> CFURLRef;
    pub fn CFURLGetBytes(anURL: CFURLRef, buffer: *mut u8, bufferLength: CFIndex) -> CFIndex;
    pub fn CFURLGetByteRangeForComponent(
        url: CFURLRef,
        component: CFURLComponentType,
        rangeIncludingSeparators: *mut CFRange,
    ) -> CFRange;
    pub fn CFURLGetTypeID() -> CFTypeID;
    pub fn CFURLResourceIsReachable(url: CFURLRef, error: *mut CFErrorRef) -> Boolean;

    /* Getting and Setting File System Resource Properties */
    pub fn CFURLClearResourcePropertyCache(url: CFURLRef);
    pub fn CFURLClearResourcePropertyCacheForKey(url: CFURLRef, key: CFStringRef);
    pub fn CFURLCopyResourcePropertiesForKeys(
        url: CFURLRef,
        keys: CFArrayRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
    //pub fn CFURLCopyResourcePropertyForKey(url: CFURLRef, key: CFStringRef, propertyValueTypeRefPtr: *mut c_void, error: *mut CFErrorRef) -> Boolean
    pub fn CFURLCreateResourcePropertiesForKeysFromBookmarkData(
        allocator: CFAllocatorRef,
        resourcePropertiesToReturn: CFArrayRef,
        bookmark: CFDataRef,
    ) -> CFDictionaryRef;
    pub fn CFURLCreateResourcePropertyForKeyFromBookmarkData(
        allocator: CFAllocatorRef,
        resourcePropertyKey: CFStringRef,
        bookmark: CFDataRef,
    ) -> CFTypeRef;
    pub fn CFURLSetResourcePropertiesForKeys(
        url: CFURLRef,
        keyedPropertyValues: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> Boolean;
    pub fn CFURLSetResourcePropertyForKey(
        url: CFURLRef,
        key: CFStringRef,
        value: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> Boolean;
    pub fn CFURLSetTemporaryResourcePropertyForKey(
        url: CFURLRef,
        key: CFStringRef,
        propertyValue: CFTypeRef,
    );

    /* Working with Bookmark Data */
    pub fn CFURLCreateBookmarkData(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        options: CFURLBookmarkCreationOptions,
        resourcePropertiesToInclude: CFArrayRef,
        relativeToURL: CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;

    #[cfg(target_os = "macos")]
    pub fn CFURLCreateBookmarkDataFromAliasRecord(
        allocator: CFAllocatorRef,
        aliasRecordDataRef: CFDataRef,
    ) -> CFDataRef; // deprecated

    pub fn CFURLCreateBookmarkDataFromFile(
        allocator: CFAllocatorRef,
        fileURL: CFURLRef,
        errorRef: *mut CFErrorRef,
    ) -> CFDataRef;
    pub fn CFURLWriteBookmarkDataToFile(
        bookmarkRef: CFDataRef,
        fileURL: CFURLRef,
        options: CFURLBookmarkFileCreationOptions,
        errorRef: *mut CFErrorRef,
    ) -> Boolean;
    pub fn CFURLStartAccessingSecurityScopedResource(url: CFURLRef) -> Boolean;
    pub fn CFURLStopAccessingSecurityScopedResource(url: CFURLRef);
}

#[test]
#[cfg(feature = "mac_os_10_8_features")]
fn can_see_excluded_from_backup_key() {
    let _ = unsafe { kCFURLIsExcludedFromBackupKey };
}
