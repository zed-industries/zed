//! Support for password options, to be used with the passwords module

use core_foundation::{string::CFString, base::{CFType, TCFType, CFOptionFlags}, number::CFNumber};
use security_framework_sys::{keychain::{SecProtocolType, SecAuthenticationType}, access_control::*};
use security_framework_sys::item::{
    kSecAttrAccessControl, kSecAttrAccount, kSecAttrAuthenticationType, kSecAttrPath, kSecAttrPort, kSecAttrProtocol,
    kSecAttrSecurityDomain, kSecAttrServer, kSecAttrService, kSecClass, kSecClassGenericPassword,
    kSecClassInternetPassword,
};
use crate::access_control::SecAccessControl;

/// `PasswordOptions` constructor
pub struct PasswordOptions {
    /// query built for the keychain request
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
    #[must_use] pub fn new_generic_password(service: &str, account: &str) -> Self {
        let query = vec![
            (
                unsafe { CFString::wrap_under_get_rule(kSecClass) },
                unsafe { CFString::wrap_under_get_rule(kSecClassGenericPassword).into_CFType() },
            ),
            (
                unsafe { CFString::wrap_under_get_rule(kSecAttrService) },
                CFString::from(service).into_CFType(),
            ),
            (
                unsafe { CFString::wrap_under_get_rule(kSecAttrAccount) },
                CFString::from(account).into_CFType(),
            ),
        ];
        Self { query }
    }

    /// Create a new internet password options
    /// Internet passwords are identified by a number of attributes.
    /// They can have others, but this interface doesn't allow specifying them.
    #[must_use] pub fn new_internet_password(
        server: &str,
        security_domain: Option<&str>,
        account: &str,
        path: &str,
        port: Option<u16>,
        protocol: SecProtocolType,
        authentication_type: SecAuthenticationType,
    ) -> Self {
        let mut query = vec![
            (
                unsafe { CFString::wrap_under_get_rule(kSecClass) },
                unsafe { CFString::wrap_under_get_rule(kSecClassInternetPassword) }.into_CFType(),
            ),
            (
                unsafe { CFString::wrap_under_get_rule(kSecAttrServer) },
                CFString::from(server).into_CFType(),
            ),
            (
                unsafe { CFString::wrap_under_get_rule(kSecAttrPath) },
                CFString::from(path).into_CFType(),
            ),
            (
                unsafe { CFString::wrap_under_get_rule(kSecAttrAccount) },
                CFString::from(account).into_CFType(),
            ),
            (
                unsafe { CFString::wrap_under_get_rule(kSecAttrProtocol) },
                CFNumber::from(protocol as i32).into_CFType(),
            ),
            (
                unsafe { CFString::wrap_under_get_rule(kSecAttrAuthenticationType) },
                CFNumber::from(authentication_type as i32).into_CFType(),
            ),
        ];
        if let Some(domain) = security_domain {
            query.push((
                unsafe { CFString::wrap_under_get_rule(kSecAttrSecurityDomain) },
                CFString::from(domain).into_CFType(),
            ));
        }
        if let Some(port) = port {
            query.push((
                unsafe { CFString::wrap_under_get_rule(kSecAttrPort) },
                CFNumber::from(i32::from(port)).into_CFType(),
            ));
        }
        Self { query }
    }

    /// Add access control to the password
    pub fn set_access_control_options(&mut self, options: AccessControlOptions) {
        self.query.push((
            unsafe { CFString::wrap_under_get_rule(kSecAttrAccessControl) },
            SecAccessControl::create_with_flags(options.bits())
                .unwrap()
                .into_CFType(),
        ));
    }
}
