// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

#[cfg(feature = "mac_os_10_8_features")]
use crate::base::CFOptionFlags;
use crate::base::{Boolean, CFAllocatorRef, CFTypeID};
use crate::uuid::CFUUIDRef;

#[repr(C)]
pub struct __CFFileSecurity(c_void);
pub type CFFileSecurityRef = *mut __CFFileSecurity;

#[cfg(feature = "mac_os_10_8_features")]
pub type CFFileSecurityClearOptions = CFOptionFlags;
#[cfg(feature = "mac_os_10_8_features")]
pub const kCFFileSecurityClearOwner: CFFileSecurityClearOptions = 1 << 0;
#[cfg(feature = "mac_os_10_8_features")]
pub const kCFFileSecurityClearGroup: CFFileSecurityClearOptions = 1 << 1;
#[cfg(feature = "mac_os_10_8_features")]
pub const kCFFileSecurityClearMode: CFFileSecurityClearOptions = 1 << 2;
#[cfg(feature = "mac_os_10_8_features")]
pub const kCFFileSecurityClearOwnerUUID: CFFileSecurityClearOptions = 1 << 3;
#[cfg(feature = "mac_os_10_8_features")]
pub const kCFFileSecurityClearGroupUUID: CFFileSecurityClearOptions = 1 << 4;
#[cfg(feature = "mac_os_10_8_features")]
pub const kCFFileSecurityClearAccessControlList: CFFileSecurityClearOptions = 1 << 5;

extern "C" {
    /*
     * CFFileSecurity.h
     */
    pub fn CFFileSecurityGetTypeID() -> CFTypeID;
    pub fn CFFileSecurityCreate(allocator: CFAllocatorRef) -> CFFileSecurityRef;
    pub fn CFFileSecurityCreateCopy(
        allocator: CFAllocatorRef,
        fileSec: CFFileSecurityRef,
    ) -> CFFileSecurityRef;
    pub fn CFFileSecurityCopyOwnerUUID(
        fileSec: CFFileSecurityRef,
        ownerUUID: *mut CFUUIDRef,
    ) -> Boolean;
    pub fn CFFileSecuritySetOwnerUUID(fileSec: CFFileSecurityRef, ownerUUID: CFUUIDRef) -> Boolean;
    pub fn CFFileSecurityCopyGroupUUID(
        fileSec: CFFileSecurityRef,
        groupUUID: *mut CFUUIDRef,
    ) -> Boolean;
    pub fn CFFileSecuritySetGroupUUID(fileSec: CFFileSecurityRef, groupUUID: CFUUIDRef) -> Boolean;
    //pub fn CFFileSecurityCopyAccessControlList(fileSec: CFFileSecurityRef, accessControlList: *mut acl_t) -> Boolean;
    //pub fn CFFileSecuritySetAccessControlList(fileSec: CFFileSecurityRef, accessControlList: acl_t) -> Boolean;
    //pub fn CFFileSecurityGetOwner(fileSec: CFFileSecurityRef, owner: *mut uid_t) -> Boolean;
    //pub fn CFFileSecuritySetOwner(fileSec: CFFileSecurityRef, owner: uid_t) -> Boolean;
    //pub fn CFFileSecurityGetGroup(fileSec: CFFileSecurityRef, group: *mut gid_t) -> Boolean;
    //pub fn CFFileSecuritySetGroup(fileSec: CFFileSecurityRef, group: gid_t) -> Boolean;
    //pub fn CFFileSecurityGetMode(fileSec: CFFileSecurityRef, mode: *mut mode_t) -> Boolean;
    //pub fn CFFileSecuritySetMode(fileSec: CFFileSecurityRef, mode: mode_t) -> Boolean;

    #[cfg(feature = "mac_os_10_8_features")]
    #[cfg_attr(feature = "mac_os_10_7_support", linkage = "extern_weak")]
    pub fn CFFileSecurityClearProperties(
        fileSec: CFFileSecurityRef,
        clearPropertyMask: CFFileSecurityClearOptions,
    ) -> Boolean;
}
