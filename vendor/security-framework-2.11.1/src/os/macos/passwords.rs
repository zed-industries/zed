//! Password support.

use crate::os::macos::keychain::SecKeychain;
use crate::os::macos::keychain_item::SecKeychainItem;
use core_foundation::array::CFArray;
use core_foundation::base::TCFType;
pub use security_framework_sys::keychain::{SecAuthenticationType, SecProtocolType};
use security_framework_sys::keychain::{
    SecKeychainAddGenericPassword, SecKeychainAddInternetPassword, SecKeychainFindGenericPassword,
    SecKeychainFindInternetPassword,
};
use security_framework_sys::keychain_item::{
    SecKeychainItemDelete, SecKeychainItemFreeContent, SecKeychainItemModifyAttributesAndData,
};
use std::fmt;
use std::fmt::Write;
use std::ops::Deref;
use std::ptr;
use std::slice;

use crate::base::Result;
use crate::cvt;

/// Password slice. Use `.as_ref()` to get `&[u8]` or `.to_owned()` to get `Vec<u8>`
pub struct SecKeychainItemPassword {
    data: *const u8,
    data_len: usize,
}

impl fmt::Debug for SecKeychainItemPassword {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.data_len {
            f.write_char('•')?;
        }
        Ok(())
    }
}

impl AsRef<[u8]> for SecKeychainItemPassword {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.data, self.data_len) }
    }
}

impl Deref for SecKeychainItemPassword {
    type Target = [u8];
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Drop for SecKeychainItemPassword {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            SecKeychainItemFreeContent(ptr::null_mut(), self.data as *mut _);
        }
    }
}

impl SecKeychainItem {
    /// Modify keychain item in-place, replacing its password with the given one
    pub fn set_password(&mut self, password: &[u8]) -> Result<()> {
        unsafe {
            cvt(SecKeychainItemModifyAttributesAndData(
                self.as_CFTypeRef() as *mut _,
                ptr::null(),
                password.len() as u32,
                password.as_ptr().cast(),
            ))?;
        }
        Ok(())
    }

    /// Delete this item from its keychain
    #[inline]
    pub fn delete(self) {
        unsafe {
            SecKeychainItemDelete(self.as_CFTypeRef() as *mut _);
        }
    }
}

/// Find a generic password.
///
/// The underlying system supports passwords with 0 values, so this
/// returns a vector of bytes rather than a string.
///
/// * `keychains` is an array of keychains to search or None to search
///   the default keychain.
/// * `service` is the name of the service to search for.
/// * `account` is the name of the account to search for.
pub fn find_generic_password(
    keychains: Option<&[SecKeychain]>,
    service: &str,
    account: &str,
) -> Result<(SecKeychainItemPassword, SecKeychainItem)> {
    let keychains_or_none = keychains.map(CFArray::from_CFTypes);

    let keychains_or_null = match keychains_or_none {
        None => ptr::null(),
        Some(ref keychains) => keychains.as_CFTypeRef(),
    };

    let mut data_len = 0;
    let mut data = ptr::null_mut();
    let mut item = ptr::null_mut();

    unsafe {
        cvt(SecKeychainFindGenericPassword(
            keychains_or_null,
            service.len() as u32,
            service.as_ptr().cast(),
            account.len() as u32,
            account.as_ptr().cast(),
            &mut data_len,
            &mut data,
            &mut item,
        ))?;
        Ok((
            SecKeychainItemPassword {
                data: data as *const _,
                data_len: data_len as usize,
            },
            SecKeychainItem::wrap_under_create_rule(item),
        ))
    }
}

/// * `keychains` is an array of keychains to search or None to search
///   the default keychain.
/// * `server`: server name.
/// * `security_domain`: security domain. This parameter is optional.
/// * `account`: account name.
/// * `path`: the path.
/// * `port`: The TCP/IP port number.
/// * `protocol`: The protocol associated with this password.
/// * `authentication_type`: The authentication scheme used.
#[allow(clippy::too_many_arguments)]
pub fn find_internet_password(
    keychains: Option<&[SecKeychain]>,
    server: &str,
    security_domain: Option<&str>,
    account: &str,
    path: &str,
    port: Option<u16>,
    protocol: SecProtocolType,
    authentication_type: SecAuthenticationType,
) -> Result<(SecKeychainItemPassword, SecKeychainItem)> {
    let keychains_or_none = keychains.map(CFArray::from_CFTypes);

    let keychains_or_null = match keychains_or_none {
        None => ptr::null(),
        Some(ref keychains) => keychains.as_CFTypeRef(),
    };

    let mut data_len = 0;
    let mut data = ptr::null_mut();
    let mut item = ptr::null_mut();

    unsafe {
        cvt(SecKeychainFindInternetPassword(
            keychains_or_null,
            server.len() as u32,
            server.as_ptr().cast(),
            security_domain.map_or(0, |s| s.len() as u32),
            security_domain
                .map_or(ptr::null(), |s| s.as_ptr().cast()),
            account.len() as u32,
            account.as_ptr().cast(),
            path.len() as u32,
            path.as_ptr().cast(),
            port.unwrap_or(0),
            protocol,
            authentication_type,
            &mut data_len,
            &mut data,
            &mut item,
        ))?;
        Ok((
            SecKeychainItemPassword {
                data: data as *const _,
                data_len: data_len as usize,
            },
            SecKeychainItem::wrap_under_create_rule(item),
        ))
    }
}

impl SecKeychain {
    /// Find application password in this keychain
    #[inline]
    pub fn find_generic_password(
        &self,
        service: &str,
        account: &str,
    ) -> Result<(SecKeychainItemPassword, SecKeychainItem)> {
        find_generic_password(Some(&[self.clone()]), service, account)
    }

    /// Find internet password in this keychain
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn find_internet_password(
        &self,
        server: &str,
        security_domain: Option<&str>,
        account: &str,
        path: &str,
        port: Option<u16>,
        protocol: SecProtocolType,
        authentication_type: SecAuthenticationType,
    ) -> Result<(SecKeychainItemPassword, SecKeychainItem)> {
        find_internet_password(
            Some(&[self.clone()]),
            server,
            security_domain,
            account,
            path,
            port,
            protocol,
            authentication_type,
        )
    }

    /// Update existing or add new internet password
    #[allow(clippy::too_many_arguments)]
    pub fn set_internet_password(
        &self,
        server: &str,
        security_domain: Option<&str>,
        account: &str,
        path: &str,
        port: Option<u16>,
        protocol: SecProtocolType,
        authentication_type: SecAuthenticationType,
        password: &[u8],
    ) -> Result<()> {
        match self.find_internet_password(
            server,
            security_domain,
            account,
            path,
            port,
            protocol,
            authentication_type,
        ) {
            Ok((_, mut item)) => item.set_password(password),
            _ => self.add_internet_password(
                server,
                security_domain,
                account,
                path,
                port,
                protocol,
                authentication_type,
                password,
            ),
        }
    }

    /// Set a generic password.
    ///
    /// * `keychain_opt` is the keychain to use or None to use the default
    ///   keychain.
    /// * `service` is the associated service name for the password.
    /// * `account` is the associated account name for the password.
    /// * `password` is the password itself.
    pub fn set_generic_password(
        &self,
        service: &str,
        account: &str,
        password: &[u8],
    ) -> Result<()> {
        match self.find_generic_password(service, account) {
            Ok((_, mut item)) => item.set_password(password),
            _ => self.add_generic_password(service, account, password),
        }
    }

    /// Add application password to the keychain, without checking if it exists already
    ///
    /// See `set_generic_password()`
    #[inline]
    pub fn add_generic_password(
        &self,
        service: &str,
        account: &str,
        password: &[u8],
    ) -> Result<()> {
        unsafe {
            cvt(SecKeychainAddGenericPassword(
                self.as_CFTypeRef() as *mut _,
                service.len() as u32,
                service.as_ptr().cast(),
                account.len() as u32,
                account.as_ptr().cast(),
                password.len() as u32,
                password.as_ptr().cast(),
                ptr::null_mut(),
            ))?;
        }
        Ok(())
    }

    /// Add internet password to the keychain, without checking if it exists already
    ///
    /// See `set_internet_password()`
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn add_internet_password(
        &self,
        server: &str,
        security_domain: Option<&str>,
        account: &str,
        path: &str,
        port: Option<u16>,
        protocol: SecProtocolType,
        authentication_type: SecAuthenticationType,
        password: &[u8],
    ) -> Result<()> {
        unsafe {
            cvt(SecKeychainAddInternetPassword(
                self.as_CFTypeRef() as *mut _,
                server.len() as u32,
                server.as_ptr().cast(),
                security_domain.map_or(0, |s| s.len() as u32),
                security_domain
                    .map_or(ptr::null(), |s| s.as_ptr().cast()),
                account.len() as u32,
                account.as_ptr().cast(),
                path.len() as u32,
                path.as_ptr().cast(),
                port.unwrap_or(0),
                protocol,
                authentication_type,
                password.len() as u32,
                password.as_ptr().cast(),
                ptr::null_mut(),
            ))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::os::macos::keychain::CreateOptions;
    use tempfile::tempdir;
    use tempfile::TempDir;

    fn temp_keychain_setup(name: &str) -> (TempDir, SecKeychain) {
        let dir = tempdir().expect("TempDir::new");
        let keychain = CreateOptions::new()
            .password("foobar")
            .create(dir.path().join(name.to_string() + ".keychain"))
            .expect("create keychain");

        (dir, keychain)
    }

    fn temp_keychain_teardown(dir: TempDir) {
        dir.close().expect("temp dir close");
    }

    #[test]
    fn missing_password_temp() {
        let (dir, keychain) = temp_keychain_setup("missing_password");
        let keychains = vec![keychain];

        let service = "temp_this_service_does_not_exist";
        let account = "this_account_is_bogus";
        let found = find_generic_password(Some(&keychains), service, account);

        assert!(found.is_err());

        temp_keychain_teardown(dir);
    }

    #[test]
    #[cfg(feature = "default_keychain_tests")]
    fn missing_password_default() {
        let service = "default_this_service_does_not_exist";
        let account = "this_account_is_bogus";
        let found = find_generic_password(None, service, account);

        assert!(found.is_err());
    }

    #[test]
    fn round_trip_password_temp() {
        let (dir, keychain) = temp_keychain_setup("round_trip_password");

        let service = "test_round_trip_password_temp";
        let account = "temp_this_is_the_test_account";
        let password = String::from("deadbeef").into_bytes();

        keychain
            .set_generic_password(service, account, &password)
            .expect("set_generic_password");
        let (found, item) = keychain
            .find_generic_password(service, account)
            .expect("find_generic_password");
        assert_eq!(found.to_owned(), password);

        item.delete();

        temp_keychain_teardown(dir);
    }

    #[test]
    #[cfg(feature = "default_keychain_tests")]
    fn round_trip_password_default() {
        let service = "test_round_trip_password_default";
        let account = "this_is_the_test_account";
        let password = String::from("deadbeef").into_bytes();

        SecKeychain::default()
            .expect("default keychain")
            .set_generic_password(service, account, &password)
            .expect("set_generic_password");
        let (found, item) =
            find_generic_password(None, service, account).expect("find_generic_password");
        assert_eq!(&*found, &password[..]);

        item.delete();
    }

    #[test]
    fn change_password_temp() {
        let (dir, keychain) = temp_keychain_setup("change_password");
        let keychains = vec![keychain];

        let service = "test_change_password_temp";
        let account = "this_is_the_test_account";
        let pw1 = String::from("password1").into_bytes();
        let pw2 = String::from("password2").into_bytes();

        keychains[0]
            .set_generic_password(service, account, &pw1)
            .expect("set_generic_password1");
        let (found, _) = find_generic_password(Some(&keychains), service, account)
            .expect("find_generic_password1");
        assert_eq!(found.as_ref(), &pw1[..]);

        keychains[0]
            .set_generic_password(service, account, &pw2)
            .expect("set_generic_password2");
        let (found, item) = find_generic_password(Some(&keychains), service, account)
            .expect("find_generic_password2");
        assert_eq!(&*found, &pw2[..]);

        item.delete();

        temp_keychain_teardown(dir);
    }

    #[test]
    #[cfg(feature = "default_keychain_tests")]
    fn change_password_default() {
        let service = "test_change_password_default";
        let account = "this_is_the_test_account";
        let pw1 = String::from("password1").into_bytes();
        let pw2 = String::from("password2").into_bytes();

        SecKeychain::default()
            .expect("default keychain")
            .set_generic_password(service, account, &pw1)
            .expect("set_generic_password1");
        let (found, _) =
            find_generic_password(None, service, account).expect("find_generic_password1");
        assert_eq!(found.to_owned(), pw1);

        SecKeychain::default()
            .expect("default keychain")
            .set_generic_password(service, account, &pw2)
            .expect("set_generic_password2");
        let (found, item) =
            find_generic_password(None, service, account).expect("find_generic_password2");
        assert_eq!(found.to_owned(), pw2);

        item.delete();
    }

    #[test]
    fn cross_keychain_corruption_temp() {
        let (dir1, keychain1) = temp_keychain_setup("cross_corrupt1");
        let (dir2, keychain2) = temp_keychain_setup("cross_corrupt2");
        let keychains1 = vec![keychain1.clone()];
        let keychains2 = vec![keychain2.clone()];
        let both_keychains = vec![keychain1, keychain2];

        let service = "temp_this_service_does_not_exist";
        let account = "this_account_is_bogus";
        let password = String::from("deadbeef").into_bytes();

        // Make sure this password doesn't exist in either keychain.
        let found = find_generic_password(Some(&both_keychains), service, account);
        assert!(found.is_err());

        // Set a password in one keychain.
        keychains1[0]
            .set_generic_password(service, account, &password)
            .expect("set_generic_password");

        // Make sure it's found in that keychain.
        let (found, item) = find_generic_password(Some(&keychains1), service, account)
            .expect("find_generic_password1");
        assert_eq!(found.to_owned(), password);

        // Make sure it's _not_ found in the other keychain.
        let found = find_generic_password(Some(&keychains2), service, account);
        assert!(found.is_err());

        // Cleanup.
        item.delete();

        temp_keychain_teardown(dir1);
        temp_keychain_teardown(dir2);
    }
}
