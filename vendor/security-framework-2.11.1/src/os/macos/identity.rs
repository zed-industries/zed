//! OSX specific extensions to identity functionality.
use core_foundation::array::CFArray;
use core_foundation::base::TCFType;
use security_framework_sys::identity::SecIdentityCreateWithCertificate;
use std::ptr;

use crate::base::Result;
use crate::certificate::SecCertificate;
use crate::cvt;
use crate::identity::SecIdentity;
use crate::os::macos::keychain::SecKeychain;

/// An extension trait adding OSX specific functionality to `SecIdentity`.
pub trait SecIdentityExt {
    /// Creates an identity corresponding to a certificate, looking in the
    /// provided keychains for the corresponding private key.
    ///
    /// To search the default keychains, use an empty slice for `keychains`.
    ///
    /// <https://developer.apple.com/documentation/security/1401160-secidentitycreatewithcertificate>
    fn with_certificate(
        keychains: &[SecKeychain],
        certificate: &SecCertificate,
    ) -> Result<SecIdentity>;
}

impl SecIdentityExt for SecIdentity {
    fn with_certificate(keychains: &[SecKeychain], certificate: &SecCertificate) -> Result<Self> {
        let keychains = CFArray::from_CFTypes(keychains);
        unsafe {
            let mut identity = ptr::null_mut();
            cvt(SecIdentityCreateWithCertificate(
                if keychains.len() > 0 {keychains.as_CFTypeRef()} else {ptr::null()},
                certificate.as_concrete_TypeRef(),
                &mut identity,
            ))?;
            Ok(Self::wrap_under_create_rule(identity))
        }
    }
}

#[cfg(test)]
mod test {
    use tempfile::tempdir;

    use super::*;
    use crate::os::macos::certificate::SecCertificateExt;
    use crate::os::macos::import_export::ImportOptions;
    use crate::os::macos::keychain::CreateOptions;
    use crate::os::macos::test::identity;
    use crate::test;

    #[test]
    fn certificate() {
        let dir = p!(tempdir());
        let identity = identity(dir.path());
        let certificate = p!(identity.certificate());
        assert_eq!("foobar.com", p!(certificate.common_name()));
    }

    #[test]
    fn private_key() {
        let dir = p!(tempdir());
        let identity = identity(dir.path());
        p!(identity.private_key());
    }

    #[test]
    fn with_certificate() {
        let dir = p!(tempdir());

        let mut keychain = p!(CreateOptions::new()
            .password("foobar")
            .create(dir.path().join("test.keychain")));

        let key = include_bytes!("../../../test/server.key");
        p!(ImportOptions::new()
            .filename("server.key")
            .keychain(&mut keychain)
            .import(key));

        let cert = test::certificate();
        p!(SecIdentity::with_certificate(&[keychain], &cert));
    }
}
