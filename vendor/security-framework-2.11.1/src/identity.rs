//! Identity support.

use core_foundation::base::TCFType;
use core_foundation::base::ToVoid;
use core_foundation::dictionary::CFMutableDictionary;
use security_framework_sys::base::SecIdentityRef;
use security_framework_sys::identity::{SecIdentityCopyCertificate, SecIdentityCopyPrivateKey, SecIdentityGetTypeID};
use security_framework_sys::item::kSecValueRef;
use security_framework_sys::keychain_item::SecItemDelete;
use std::fmt;
use std::ptr;

use crate::base::Error;
use crate::base::Result;
use crate::certificate::SecCertificate;
use crate::cvt;
use crate::key::SecKey;

declare_TCFType! {
    /// A type representing an identity.
    ///
    /// Identities are a certificate paired with the corresponding private key.
    SecIdentity, SecIdentityRef
}
impl_TCFType!(SecIdentity, SecIdentityRef, SecIdentityGetTypeID);

unsafe impl Sync for SecIdentity {}
unsafe impl Send for SecIdentity {}

impl fmt::Debug for SecIdentity {
    #[cold]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = fmt.debug_struct("SecIdentity");
        if let Ok(cert) = self.certificate() {
            builder.field("certificate", &cert);
        }
        if let Ok(key) = self.private_key() {
            builder.field("private_key", &key);
        }
        builder.finish()
    }
}

impl SecIdentity {
    /// Returns the certificate corresponding to this identity.
    pub fn certificate(&self) -> Result<SecCertificate> {
        unsafe {
            let mut certificate = ptr::null_mut();
            cvt(SecIdentityCopyCertificate(self.0, &mut certificate))?;
            Ok(SecCertificate::wrap_under_create_rule(certificate))
        }
    }

    /// Returns the private key corresponding to this identity.
    pub fn private_key(&self) -> Result<SecKey> {
        unsafe {
            let mut key = ptr::null_mut();
            cvt(SecIdentityCopyPrivateKey(self.0, &mut key))?;
            Ok(SecKey::wrap_under_create_rule(key))
        }
    }

    /// Translates to `SecItemDelete`, passing in the `SecIdentityRef`
    pub fn delete(&self) -> Result<(), Error> {
        let query = CFMutableDictionary::from_CFType_pairs(&[(
            unsafe { kSecValueRef }.to_void(),
            self.to_void(),
        )]);

        cvt(unsafe { SecItemDelete(query.as_concrete_TypeRef()) })
    }
}

#[cfg(test)]
mod test {
    use super::SecIdentity;

    #[test]
    fn identity_has_send_bound() {
        fn assert_send<T: Send>() {}
        assert_send::<SecIdentity>();
    }
}
