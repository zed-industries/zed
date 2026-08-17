//! Keychain support.

use core_foundation::base::{Boolean, TCFType};
use security_framework_sys::base::{errSecSuccess, SecKeychainRef};
use security_framework_sys::keychain::*;
use std::ffi::CString;
use std::os::raw::c_void;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;

use crate::base::{Error, Result};
use crate::cvt;
use crate::os::macos::access::SecAccess;

pub use security_framework_sys::keychain::SecPreferencesDomain;

declare_TCFType! {
    /// A type representing a keychain.
    SecKeychain, SecKeychainRef
}
impl_TCFType!(SecKeychain, SecKeychainRef, SecKeychainGetTypeID);

unsafe impl Sync for SecKeychain {}
unsafe impl Send for SecKeychain {}

impl SecKeychain {
    /// Creates a `SecKeychain` object corresponding to the user's default
    /// keychain.
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Result<Self> {
        unsafe {
            let mut keychain = ptr::null_mut();
            cvt(SecKeychainCopyDefault(&mut keychain))?;
            Ok(Self::wrap_under_create_rule(keychain))
        }
    }

    /// Creates a `SecKeychain` object corresponding to the user's default
    /// keychain for the given domain.
    pub fn default_for_domain(domain: SecPreferencesDomain) -> Result<Self> {
        unsafe {
            let mut keychain = ptr::null_mut();
            cvt(SecKeychainCopyDomainDefault(domain, &mut keychain))?;
            Ok(Self::wrap_under_create_rule(keychain))
        }
    }

    /// Opens a keychain from a file.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_name = [
            path.as_ref().as_os_str().as_bytes(),
            std::slice::from_ref(&0)
        ].concat();

        unsafe {
            let mut keychain = ptr::null_mut();
            cvt(SecKeychainOpen(path_name.as_ptr().cast(), &mut keychain))?;
            Ok(Self::wrap_under_create_rule(keychain))
        }
    }

    /// Unlocks the keychain.
    ///
    /// If a password is not specified, the user will be prompted to enter it.
    pub fn unlock(&mut self, password: Option<&str>) -> Result<()> {
        let (len, ptr, use_password) = match password {
            Some(password) => (password.len(), password.as_ptr().cast(), true),
            None => (0, ptr::null(), false),
        };

        unsafe {
            cvt(SecKeychainUnlock(
                self.as_concrete_TypeRef(),
                len as u32,
                ptr,
                Boolean::from(use_password),
            ))
        }
    }

    /// Sets settings of the keychain.
    #[inline]
    pub fn set_settings(&mut self, settings: &KeychainSettings) -> Result<()> {
        unsafe {
            cvt(SecKeychainSetSettings(
                self.as_concrete_TypeRef(),
                &settings.0,
            ))
        }
    }

    #[cfg(target_os = "macos")]
    /// Disables the user interface for keychain services functions that
    /// automatically display a user interface.
    pub fn disable_user_interaction() -> Result<KeychainUserInteractionLock> {
        let code = unsafe { SecKeychainSetUserInteractionAllowed(0u8) };

        if code != errSecSuccess {
            Err(Error::from_code(code))
        } else {
            Ok(KeychainUserInteractionLock)
        }
    }

    #[cfg(target_os = "macos")]
    /// Indicates whether keychain services functions that normally display a
    /// user interaction are allowed to do so.
    pub fn user_interaction_allowed() -> Result<bool> {
        let mut state: Boolean = 0;
        let code = unsafe { SecKeychainGetUserInteractionAllowed(&mut state) };

        if code != errSecSuccess {
            Err(Error::from_code(code))
        } else {
            Ok(state != 0)
        }
    }
}

/// A builder type to create new keychains.
#[derive(Default)]
pub struct CreateOptions {
    password: Option<String>,
    prompt_user: bool,
    access: Option<SecAccess>,
}

impl CreateOptions {
    /// Creates a new builder with default options.
    #[inline(always)]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the password to be used to protect the keychain.
    #[inline]
    pub fn password(&mut self, password: &str) -> &mut Self {
        self.password = Some(password.into());
        self
    }

    /// If set, the user will be prompted to provide a password used to
    /// protect the keychain.
    #[inline(always)]
    pub fn prompt_user(&mut self, prompt_user: bool) -> &mut Self {
        self.prompt_user = prompt_user;
        self
    }

    /// Sets the access control applied to the keychain.
    #[inline(always)]
    pub fn access(&mut self, access: SecAccess) -> &mut Self {
        self.access = Some(access);
        self
    }

    /// Creates a new keychain at the specified location on the filesystem.
    pub fn create<P: AsRef<Path>>(&self, path: P) -> Result<SecKeychain> {
        unsafe {
            let path_name = path.as_ref().as_os_str().as_bytes();
            // FIXME
            let path_name = CString::new(path_name).unwrap();

            let (password, password_len) = match self.password {
                Some(ref password) => (password.as_ptr().cast::<c_void>(), password.len() as u32),
                None => (ptr::null(), 0),
            };

            let access = match self.access {
                Some(ref access) => access.as_concrete_TypeRef(),
                None => ptr::null_mut(),
            };

            let mut keychain = ptr::null_mut();
            cvt(SecKeychainCreate(
                path_name.as_ptr(),
                password_len,
                password,
                Boolean::from(self.prompt_user),
                access,
                &mut keychain,
            ))?;

            Ok(SecKeychain::wrap_under_create_rule(keychain))
        }
    }
}

/// Settings associated with a `SecKeychain`.
pub struct KeychainSettings(SecKeychainSettings);

impl KeychainSettings {
    /// Creates a new `KeychainSettings` with default settings.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self(SecKeychainSettings {
            version: SEC_KEYCHAIN_SETTINGS_VERS1,
            lockOnSleep: 0,
            useLockInterval: 0,
            lockInterval: i32::max_value() as u32,
        })
    }

    /// If set, the keychain will automatically lock when the computer sleeps.
    ///
    /// Defaults to `false`.
    #[inline(always)]
    pub fn set_lock_on_sleep(&mut self, lock_on_sleep: bool) {
        self.0.lockOnSleep = Boolean::from(lock_on_sleep);
    }

    /// Sets the interval of time in seconds after which the keychain is
    /// automatically locked.
    ///
    /// Defaults to `None`.
    pub fn set_lock_interval(&mut self, lock_interval: Option<u32>) {
        match lock_interval {
            Some(lock_interval) => {
                self.0.useLockInterval = 1;
                self.0.lockInterval = lock_interval;
            }
            None => {
                self.0.useLockInterval = 0;
                self.0.lockInterval = i32::max_value() as u32;
            }
        }
    }
}

impl Default for KeychainSettings {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "macos")]
#[must_use = "The user interaction is disabled for the lifetime of the returned object"]
/// Automatically re-enables user interaction.
pub struct KeychainUserInteractionLock;

#[cfg(target_os = "macos")]
impl Drop for KeychainUserInteractionLock {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { SecKeychainSetUserInteractionAllowed(1u8) };
    }
}

#[cfg(test)]
mod test {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn create_options() {
        let dir = tempdir().unwrap();

        let mut keychain = CreateOptions::new()
            .password("foobar")
            .create(dir.path().join("test.keychain"))
            .unwrap();

        keychain.set_settings(&KeychainSettings::new()).unwrap();
    }

    #[test]
    fn disable_user_interaction() {
        assert!(SecKeychain::user_interaction_allowed().unwrap());
        {
            let _lock = SecKeychain::disable_user_interaction().unwrap();
            assert!(!SecKeychain::user_interaction_allowed().unwrap());
        }
        assert!(SecKeychain::user_interaction_allowed().unwrap());
    }
}
