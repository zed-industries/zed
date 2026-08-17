//! Support for password options, to be used with the passwords module

// NB: re-export these types in the `passwords` module!

use crate::access_control::SecAccessControl;
use core_foundation::base::{CFOptionFlags, CFType, TCFType};
#[allow(unused_imports)]
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::CFDictionary;
use core_foundation::number::CFNumber;
use core_foundation::string::{CFString, CFStringRef};
use security_framework_sys::access_control::*;
use security_framework_sys::item::{
    kSecAttrAccessControl, kSecAttrAccessGroup, kSecAttrAccount, kSecAttrAuthenticationType,
    kSecAttrComment, kSecAttrDescription, kSecAttrLabel,
    kSecAttrPath, kSecAttrPort, kSecAttrProtocol, kSecAttrSecurityDomain, kSecAttrServer,
    kSecAttrService, kSecClass, kSecClassGenericPassword, kSecClassInternetPassword,
};
#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
use security_framework_sys::item::kSecAttrSynchronizable;
#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
use security_framework_sys::item::kSecAttrSynchronizableAny;
#[cfg(any(feature = "OSX_10_15", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
use security_framework_sys::item::kSecUseDataProtectionKeychain;
use security_framework_sys::keychain::{SecAuthenticationType, SecProtocolType};

/// `PasswordOptions` constructor
pub struct PasswordOptions {
    /// query built for the keychain request
    #[deprecated(note = "This field should have been private. Please use setters that don't expose CFType")]
    pub query: Vec<(CFString, CFType)>,
}

bitflags::bitflags! {
    /// The option flags used to configure the evaluation of a `SecAccessControl`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AccessControlOptions: CFOptionFlags {
        /** Constraint to access an item with either biometry or passcode. */
        const USER_PRESENCE = kSecAccessControlUserPresence;
        #[cfg(feature = "OSX_10_13")]
        /** Constraint to access an item with Touch ID for any enrolled fingers, or Face ID. */
        const BIOMETRY_ANY = kSecAccessControlBiometryAny;
        #[cfg(feature = "OSX_10_13")]
        /** Constraint to access an item with Touch ID for currently enrolled fingers, or from Face ID with the currently enrolled user. */
        const BIOMETRY_CURRENT_SET = kSecAccessControlBiometryCurrentSet;
        /** Constraint to access an item with a passcode. */
        const DEVICE_PASSCODE = kSecAccessControlDevicePasscode;
        #[cfg(feature = "OSX_10_15")]
        /** Constraint to access an item with a watch. */
        const WATCH = kSecAccessControlWatch;
        /** Indicates that at least one constraint must be satisfied. */
        const OR = kSecAccessControlOr;
        /** Indicates that all constraints must be satisfied. */
        const AND = kSecAccessControlAnd;
        /** Enable a private key to be used in signing a block of data or verifying a signed block. */
        const PRIVATE_KEY_USAGE = kSecAccessControlPrivateKeyUsage;
        /** Option to use an application-provided password for data encryption key generation. */
        const APPLICATION_PASSWORD = kSecAccessControlApplicationPassword;
    }
}

impl PasswordOptions {
    /// Create a new generic password options
    /// Generic passwords are identified by service and account.  They have other
    /// attributes, but this interface doesn't allow specifying them.
    #[must_use]
    pub fn new_generic_password(service: &str, account: &str) -> Self {
        #[allow(deprecated)]
        Self { query: vec![
            (
                unsafe { CFString::wrap_under_get_rule(kSecClass) },
                unsafe { CFString::wrap_under_get_rule(kSecClassGenericPassword).into_CFType() },
            ),
            (unsafe { CFString::wrap_under_get_rule(kSecAttrService) }, CFString::from(service).into_CFType()),
            (unsafe { CFString::wrap_under_get_rule(kSecAttrAccount) }, CFString::from(account).into_CFType()),
        ] }
    }

    /// Create a new internet password options
    /// Internet passwords are identified by a number of attributes.
    /// They can have others, but this interface doesn't allow specifying them.
    #[must_use]
    pub fn new_internet_password(
        server: &str,
        security_domain: Option<&str>,
        account: &str,
        path: &str,
        port: Option<u16>,
        protocol: SecProtocolType,
        authentication_type: SecAuthenticationType,
    ) -> Self {
        #[allow(deprecated)]
        let mut this = Self { query: vec![
            (
                unsafe { CFString::wrap_under_get_rule(kSecClass) },
                unsafe { CFString::wrap_under_get_rule(kSecClassInternetPassword) }.into_CFType(),
            ),
            (unsafe { CFString::wrap_under_get_rule(kSecAttrServer) }, CFString::from(server).into_CFType()),
            (unsafe { CFString::wrap_under_get_rule(kSecAttrPath) }, CFString::from(path).into_CFType()),
            (unsafe { CFString::wrap_under_get_rule(kSecAttrAccount) }, CFString::from(account).into_CFType()),
            (unsafe { CFString::wrap_under_get_rule(kSecAttrProtocol) }, CFNumber::from(protocol as i32).into_CFType()),
            (
                unsafe { CFString::wrap_under_get_rule(kSecAttrAuthenticationType) },
                CFNumber::from(authentication_type as i32).into_CFType(),
            ),
        ] };
        if let Some(domain) = security_domain {
            unsafe {
                this.push_query(kSecAttrSecurityDomain, CFString::from(domain));
            }
        }
        if let Some(port) = port {
            unsafe {
                this.push_query(kSecAttrPort, CFNumber::from(i32::from(port)));
            }
        }
        this
    }

    /// Add access control to the password
    pub fn set_access_control_options(&mut self, options: AccessControlOptions) {
        unsafe {
            self.push_query(kSecAttrAccessControl, SecAccessControl::create_with_flags(options.bits()).unwrap());
        }
    }

    /// Add access control to the password
    pub fn set_access_control(&mut self, access_control: SecAccessControl) {
        unsafe {
            self.push_query(kSecAttrAccessControl, access_control);
        }
    }

    /// Add access group to the password
    pub fn set_access_group(&mut self, group: &str) {
        unsafe {
            self.push_query(kSecAttrAccessGroup, CFString::from(group));
        }
    }

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    /// Specify whether password is cloud-synchronized, not cloud-synchronized, or either (`None`).
    ///
    /// Note: cloud-synchronized and not-cloud-synchronized passwords are kept
    /// in completely different stores, so they are uniquely identified not just
    /// by their `service` and `account` but also their cloud-synchronized option.
    ///
    /// If you specify a non-`None` value for this option, any operation you
    /// perform - whether set, get, or delete - will only affect the store matching
    /// the value: Some(`true`) will only affect the cloud-synchronized store and
    /// Some(`false`) will only affect the not-cloud-synchronized store.
    ///
    /// If you specify `None` for this option, the effect depends on your operation:
    ///
    /// - Performing a delete will delete from both stores.
    /// - Performing a get will return values from both stores, but since get only
    ///   returns one value you can't be sure which store that value was in.
    /// - Performing a set will update existing values in both stores. _But_, before
    ///   doing any updates, set will first try to create a new value in the
    ///   not-cloud-synchronized store (interpreting `None` as `false`). If
    ///   that creation attempt succeeds, no update will be done of any existing
    ///   value in the cloud-synchronized store. Thus, only if there is an existing
    ///   value in the not-cloud-synchronized store will set update the
    ///   cloud-synchronized store.
    pub fn set_access_synchronized(&mut self, synchronized: Option<bool>) {
        unsafe {
            if let Some(synchronizable) = synchronized {
                self.push_query(kSecAttrSynchronizable, CFBoolean::from(synchronizable));
            } else {
                let either = CFString::wrap_under_get_rule(kSecAttrSynchronizableAny);
                self.push_query(kSecAttrSynchronizable, either);
            }
        }
    }

    /// Set the comment on the password
    pub fn set_comment(&mut self, comment: &str) {
        unsafe {
            self.push_query(kSecAttrComment, CFString::from(comment));
        }
    }

    /// Add a description to the password
    pub fn set_description(&mut self, description: &str) {
        unsafe {
            self.push_query(kSecAttrDescription, CFString::from(description));
        }
    }

    /// Add a label to the password
    pub fn set_label(&mut self, label: &str) {
        unsafe {
            self.push_query(kSecAttrLabel, CFString::from(label));
        }
    }

    #[cfg(any(feature = "OSX_10_15", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    /// Use the data protection keychain (always true except on macOS)
    pub fn use_protected_keychain(&mut self) {
        unsafe {
            self.push_query(kSecUseDataProtectionKeychain, CFBoolean::from(true));
        }
    }

    /// The key must be a `kSec*` constant.
    /// Value is any owned ObjC object, like `CFString`.
    pub(crate) unsafe fn push_query(&mut self, static_key_constant: CFStringRef, value: impl TCFType) {
        #[allow(deprecated)]
        self.query.push((
            unsafe { CFString::wrap_under_get_rule(static_key_constant) },
            value.into_CFType(),
        ));
    }

    pub(crate) fn to_dictionary(&self) -> CFDictionary<CFString, CFType> {
        #[allow(deprecated)]
        CFDictionary::from_CFType_pairs(&self.query[..])
    }
}
