//! Security Framework type import/export support.

use core_foundation::array::CFArray;
use core_foundation::base::{CFType, TCFType};
use core_foundation::data::CFData;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;
use security_framework_sys::import_export::*;
use std::ptr;

use crate::base::Result;
use crate::certificate::SecCertificate;
use crate::cvt;
use crate::identity::SecIdentity;
#[cfg(target_os = "macos")]
use crate::os::macos::access::SecAccess;
#[cfg(target_os = "macos")]
use crate::os::macos::keychain::SecKeychain;
use crate::trust::SecTrust;

/// Information about an imported identity.
pub struct ImportedIdentity {
    /// The label of the identity.
    pub label: Option<String>,
    /// The ID of the identity. Typically the SHA-1 hash of the public key.
    pub key_id: Option<Vec<u8>>,
    /// A `SecTrust` object set up to validate this identity.
    pub trust: Option<SecTrust>,
    /// A certificate chain validating this identity.
    pub cert_chain: Option<Vec<SecCertificate>>,
    /// The identity itself.
    pub identity: Option<SecIdentity>,
    _p: (),
}

/// A builder type to import an identity from PKCS#12 formatted data.
#[derive(Default)]
pub struct Pkcs12ImportOptions {
    passphrase: Option<CFString>,
    #[cfg(target_os = "macos")]
    keychain: Option<SecKeychain>,
    #[cfg(target_os = "macos")]
    access: Option<SecAccess>,
}

#[cfg(target_os = "macos")]
impl crate::Pkcs12ImportOptionsInternals for Pkcs12ImportOptions {
    #[inline(always)]
    fn keychain(&mut self, keychain: SecKeychain) -> &mut Self {
        self.keychain = Some(keychain);
        self
    }

    #[inline(always)]
    fn access(&mut self, access: SecAccess) -> &mut Self {
        self.access = Some(access);
        self
    }
}

impl Pkcs12ImportOptions {
    /// Creates a new builder with default options.
    #[inline(always)]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Specifies the passphrase to be used to decrypt the data.
    ///
    /// This must be specified, as unencrypted PKCS#12 data is not supported.
    #[inline]
    pub fn passphrase(&mut self, passphrase: &str) -> &mut Self {
        self.passphrase = Some(CFString::new(passphrase));
        self
    }

    /// Imports identities from PKCS#12 encoded data.
    pub fn import(&self, pkcs12_data: &[u8]) -> Result<Vec<ImportedIdentity>> {
        unsafe {
            let pkcs12_data = CFData::from_buffer(pkcs12_data);

            let mut options = vec![];

            if let Some(ref passphrase) = self.passphrase {
                options.push((
                    CFString::wrap_under_get_rule(kSecImportExportPassphrase),
                    passphrase.as_CFType(),
                ));
            }

            self.import_setup(&mut options);

            let options = CFDictionary::from_CFType_pairs(&options);

            let mut raw_items = ptr::null();
            cvt(SecPKCS12Import(
                pkcs12_data.as_concrete_TypeRef(),
                options.as_concrete_TypeRef(),
                &mut raw_items,
            ))?;
            let raw_items = CFArray::<CFDictionary<CFString, *const _>>::wrap_under_create_rule(raw_items);

            let mut items = vec![];

            for raw_item in &raw_items {
                let label = raw_item
                    .find(kSecImportItemLabel)
                    .map(|label| CFString::wrap_under_get_rule((*label).cast()).to_string());
                let key_id = raw_item
                    .find(kSecImportItemKeyID)
                    .map(|key_id| CFData::wrap_under_get_rule((*key_id).cast()).to_vec());
                let trust = raw_item
                    .find(kSecImportItemTrust)
                    .map(|trust| SecTrust::wrap_under_get_rule(*trust as *mut _));
                let cert_chain = raw_item.find(kSecImportItemCertChain.cast()).map(
                    |cert_chain| {
                        CFArray::<SecCertificate>::wrap_under_get_rule((*cert_chain).cast())
                            .iter()
                            .map(|c| c.clone())
                            .collect()
                    },
                );
                let identity = raw_item
                    .find(kSecImportItemIdentity)
                    .map(|identity| SecIdentity::wrap_under_get_rule(*identity as *mut _));

                items.push(ImportedIdentity {
                    label,
                    key_id,
                    trust,
                    cert_chain,
                    identity,
                    _p: (),
                });
            }

            Ok(items)
        }
    }

    #[cfg(target_os = "macos")]
    fn import_setup(&self, options: &mut Vec<(CFString, CFType)>) {
        unsafe {
            if let Some(ref keychain) = self.keychain {
                options.push((
                    CFString::wrap_under_get_rule(kSecImportExportKeychain),
                    keychain.as_CFType(),
                ));
            }

            if let Some(ref access) = self.access {
                options.push((
                    CFString::wrap_under_get_rule(kSecImportExportAccess),
                    access.as_CFType(),
                ));
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    fn import_setup(&self, _: &mut Vec<(CFString, CFType)>) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn missing_passphrase() {
        let data = include_bytes!("../test/server.p12");
        assert!(Pkcs12ImportOptions::new().import(data).is_err());
    }
}
