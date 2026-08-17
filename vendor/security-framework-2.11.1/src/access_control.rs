//! Access Control support.

use std::ptr::{self, null};

use core_foundation::string::CFString;
use core_foundation::base::{TCFType, CFOptionFlags, kCFAllocatorDefault};
use security_framework_sys::access_control::{
    SecAccessControlGetTypeID, SecAccessControlCreateWithFlags,
    kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly,
    kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
    kSecAttrAccessibleWhenUnlocked,
    kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly,
    kSecAttrAccessibleAfterFirstUnlock
};
use security_framework_sys::base::{SecAccessControlRef, errSecParam};
use crate::base::{Error, Result};

declare_TCFType! {
    /// A type representing sec access control settings.
    SecAccessControl, SecAccessControlRef
}
impl_TCFType!(
    SecAccessControl,
    SecAccessControlRef,
    SecAccessControlGetTypeID
);

unsafe impl Sync for SecAccessControl {}
unsafe impl Send for SecAccessControl {}

/// Specify when an item is available.
pub enum ProtectionMode {
    /// The data in the keychain can only be accessed when the device is
    /// unlocked. Only available if a passcode is set on the device.
    AccessibleWhenPasscodeSetThisDeviceOnly,
    ///The data in the keychain item can be accessed only while the device is
    /// unlocked by the user.
    AccessibleWhenUnlockedThisDeviceOnly,
    /// The data in the keychain item can be accessed only while the device is
    /// unlocked by the user.
    AccessibleWhenUnlocked,
    /// The data in the keychain item cannot be accessed after a restart until
    /// the device has been unlocked once by the user.
    AccessibleAfterFirstUnlockThisDeviceOnly,
    /// The data in the keychain item cannot be accessed after a restart until
    /// the device has been unlocked once by the user.
    AccessibleAfterFirstUnlock,
}

impl SecAccessControl {
    /// Create `AccessControl` object from flags
    pub fn create_with_flags(flags: CFOptionFlags) -> Result<Self> {
        Self::create_with_protection(None, flags)
    }

    /// Create `AccessControl` object from a protection value and flags.
    pub fn create_with_protection(protection: Option<ProtectionMode>, flags: CFOptionFlags) -> Result<Self> {
        let protection_val = protection.map(|v| {
            match v {
                ProtectionMode::AccessibleWhenPasscodeSetThisDeviceOnly => unsafe { CFString::wrap_under_get_rule(kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly) },
                ProtectionMode::AccessibleWhenUnlockedThisDeviceOnly => unsafe { CFString::wrap_under_get_rule(kSecAttrAccessibleWhenUnlockedThisDeviceOnly) },
                ProtectionMode::AccessibleWhenUnlocked => unsafe { CFString::wrap_under_get_rule(kSecAttrAccessibleWhenUnlocked) },
                ProtectionMode::AccessibleAfterFirstUnlockThisDeviceOnly => unsafe { CFString::wrap_under_get_rule(kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly) },
                ProtectionMode::AccessibleAfterFirstUnlock => unsafe { CFString::wrap_under_get_rule(kSecAttrAccessibleAfterFirstUnlock) },
            }
        });
        unsafe {
            let access_control = SecAccessControlCreateWithFlags(
                kCFAllocatorDefault,
                protection_val.map(|v| v.as_CFTypeRef()).unwrap_or(null()),
                flags,
                ptr::null_mut(),
            );
            if access_control.is_null() {
                Err(Error::from_code(errSecParam))
            } else {
                Ok(Self::wrap_under_create_rule(access_control))
            }
        }
    }
}
