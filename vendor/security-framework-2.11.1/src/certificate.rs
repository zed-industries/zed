//! Certificate support.

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{TCFType, ToVoid};
use core_foundation::data::CFData;
use core_foundation::dictionary::CFMutableDictionary;
use core_foundation::string::CFString;
use core_foundation_sys::base::kCFAllocatorDefault;
#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
use security_framework_sys::base::{errSecNotTrusted, errSecSuccess};
use security_framework_sys::base::{errSecParam, SecCertificateRef};
use security_framework_sys::certificate::*;
use security_framework_sys::keychain_item::SecItemDelete;
use std::fmt;
use std::ptr;

use crate::base::{Error, Result};
use crate::cvt;
#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
use crate::key;
#[cfg(target_os = "macos")]
use crate::os::macos::keychain::SecKeychain;
#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
use core_foundation::base::FromVoid;
#[cfg(any(feature = "OSX_10_13", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
use core_foundation::error::{CFError, CFErrorRef};
#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
use core_foundation::number::CFNumber;
#[cfg(feature = "serial-number-bigint")]
use num_bigint::BigUint;
use security_framework_sys::item::kSecValueRef;
#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
use std::ops::Deref;

declare_TCFType! {
    /// A type representing a certificate.
    SecCertificate, SecCertificateRef
}
impl_TCFType!(SecCertificate, SecCertificateRef, SecCertificateGetTypeID);

unsafe impl Sync for SecCertificate {}
unsafe impl Send for SecCertificate {}

impl fmt::Debug for SecCertificate {
    #[cold]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("SecCertificate")
            .field("subject", &self.subject_summary())
            .finish()
    }
}

impl SecCertificate {
    /// Creates a `SecCertificate` from DER encoded certificate data.
    pub fn from_der(der_data: &[u8]) -> Result<Self> {
        let der_data = CFData::from_buffer(der_data);
        unsafe {
            let certificate =
                SecCertificateCreateWithData(kCFAllocatorDefault, der_data.as_concrete_TypeRef());
            if certificate.is_null() {
                Err(Error::from_code(errSecParam))
            } else {
                Ok(Self::wrap_under_create_rule(certificate))
            }
        }
    }

    /// Returns DER encoded data describing this certificate.
    #[must_use]
    pub fn to_der(&self) -> Vec<u8> {
        unsafe {
            let der_data = SecCertificateCopyData(self.0);
            CFData::wrap_under_create_rule(der_data).to_vec()
        }
    }

    /// Adds a certificate to a keychain.
    #[cfg(target_os="macos")]
    pub fn add_to_keychain(&self, keychain: Option<SecKeychain>) -> Result<()> {
        let kch = match keychain {
            Some(kch) => kch,
            _ => SecKeychain::default()?,
        };
        cvt(unsafe {
            SecCertificateAddToKeychain(self.as_CFTypeRef() as *mut _, kch.as_CFTypeRef() as *mut _)
        })
    }

    /// Returns a human readable summary of this certificate.
    #[must_use]
    pub fn subject_summary(&self) -> String {
        unsafe {
            let summary = SecCertificateCopySubjectSummary(self.0);
            CFString::wrap_under_create_rule(summary).to_string()
        }
    }

    /// Returns a vector of email addresses for the subject of the certificate.
    pub fn email_addresses(&self) -> Result<Vec<String>, Error> {
        let mut array: CFArrayRef = ptr::null();
        unsafe {
            cvt(SecCertificateCopyEmailAddresses(
                self.as_concrete_TypeRef(),
                &mut array,
            ))?;

            let array = CFArray::<CFString>::wrap_under_create_rule(array);
            Ok(array.into_iter().map(|p| p.to_string()).collect())
        }
    }

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    /// Returns DER encoded X.509 distinguished name of the certificate issuer.
    #[must_use]
    pub fn issuer(&self) -> Vec<u8> {
        unsafe {
            let issuer = SecCertificateCopyNormalizedIssuerSequence(self.0);
            CFData::wrap_under_create_rule(issuer).to_vec()
        }
    }

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    /// Returns DER encoded X.509 distinguished name of the certificate subject.
    #[must_use]
    pub fn subject(&self) -> Vec<u8> {
        unsafe {
            let subject = SecCertificateCopyNormalizedSubjectSequence(self.0);
            CFData::wrap_under_create_rule(subject).to_vec()
        }
    }

    #[cfg(any(feature = "OSX_10_13", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    /// Returns DER encoded serial number of the certificate.
    pub fn serial_number_bytes(&self) -> Result<Vec<u8>, CFError> {
        unsafe {
            let mut error: CFErrorRef = ptr::null_mut();
            let serial_number = SecCertificateCopySerialNumberData(self.0, &mut error);
            if error.is_null() {
                Ok(CFData::wrap_under_create_rule(serial_number).to_vec())
            } else {
                Err(CFError::wrap_under_create_rule(error))
            }
        }
    }

    /// Use `BigUint::from_bytes_be(serial_number_bytes())` instead
    #[deprecated(note = "use serial_number_bytes()")]
    #[cfg(feature = "serial-number-bigint")]
    pub fn serial_number(&self) -> Result<BigUint, CFError> {
        Ok(BigUint::from_bytes_be(&self.serial_number_bytes()?))
    }

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    /// Returns DER encoded subjectPublicKeyInfo of certificate if available. This can be used
    /// for certificate pinning.
    pub fn public_key_info_der(&self) -> Result<Option<Vec<u8>>> {
        // Imported from TrustKit
        // https://github.com/datatheorem/TrustKit/blob/master/TrustKit/Pinning/TSKSPKIHashCache.m
        let public_key = self.public_key()?;
        Ok(self.pk_to_der(public_key))
    }

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    #[must_use]
    fn pk_to_der(&self, public_key: key::SecKey) -> Option<Vec<u8>> {
        use security_framework_sys::item::kSecAttrKeyType;
        use security_framework_sys::item::kSecAttrKeySizeInBits;

        let public_key_attributes = public_key.attributes();
        let public_key_type = public_key_attributes
            .find(unsafe { kSecAttrKeyType }.cast::<std::os::raw::c_void>())?;
        let public_keysize = public_key_attributes
            .find(unsafe { kSecAttrKeySizeInBits }.cast::<std::os::raw::c_void>())?;
        let public_keysize = unsafe { CFNumber::from_void(*public_keysize.deref()) };
        let public_keysize_val = public_keysize.to_i64()? as u32;
        let hdr_bytes = get_asn1_header_bytes(
            unsafe { CFString::wrap_under_get_rule(*public_key_type.deref() as _) },
            public_keysize_val,
        )?;
        let public_key_data = public_key.external_representation()?;
        let mut out = Vec::with_capacity(hdr_bytes.len() + public_key_data.len() as usize);
        out.extend_from_slice(hdr_bytes);
        out.extend_from_slice(public_key_data.bytes());
        Some(out)
    }

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    /// Get public key from certificate
    pub fn public_key(&self) -> Result<key::SecKey> {
        use crate::policy::SecPolicy;
        use crate::trust::SecTrust;
        use std::slice::from_ref;

        let policy = SecPolicy::create_x509();
        let mut trust = SecTrust::create_with_certificates(from_ref(self), from_ref(&policy))?;
        #[allow(deprecated)]
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos")))]
        trust.evaluate()?;
        #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
        cvt(match trust.evaluate_with_error() {
            Ok(_) => errSecSuccess,
            Err(_) => errSecNotTrusted,
        })?;
        trust.copy_public_key()
    }

    /// Translates to `SecItemDelete`, passing in the `SecCertificateRef`
    pub fn delete(&self) -> Result<(), Error> {
        let query = CFMutableDictionary::from_CFType_pairs(&[(
            unsafe { kSecValueRef }.to_void(),
            self.to_void(),
        )]);

        cvt(unsafe { SecItemDelete(query.as_concrete_TypeRef()) })
    }
}

#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
fn get_asn1_header_bytes(pkt: CFString, ksz: u32) -> Option<&'static [u8]> {
    use security_framework_sys::item::kSecAttrKeyTypeRSA;
    use security_framework_sys::item::kSecAttrKeyTypeECSECPrimeRandom;

    if pkt == unsafe { CFString::wrap_under_get_rule(kSecAttrKeyTypeRSA) } && ksz == 2048 {
        return Some(&RSA_2048_ASN1_HEADER);
    }
    if pkt == unsafe { CFString::wrap_under_get_rule(kSecAttrKeyTypeRSA) } && ksz == 4096 {
        return Some(&RSA_4096_ASN1_HEADER);
    }
    if pkt == unsafe { CFString::wrap_under_get_rule(kSecAttrKeyTypeECSECPrimeRandom) }
        && ksz == 256
    {
        return Some(&EC_DSA_SECP_256_R1_ASN1_HEADER);
    }
    if pkt == unsafe { CFString::wrap_under_get_rule(kSecAttrKeyTypeECSECPrimeRandom) }
        && ksz == 384
    {
        return Some(&EC_DSA_SECP_384_R1_ASN1_HEADER);
    }
    None
}

#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
const RSA_2048_ASN1_HEADER: [u8; 24] = [
    0x30, 0x82, 0x01, 0x22, 0x30, 0x0d, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01,
    0x01, 0x05, 0x00, 0x03, 0x82, 0x01, 0x0f, 0x00,
];

#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
const RSA_4096_ASN1_HEADER: [u8; 24] = [
    0x30, 0x82, 0x02, 0x22, 0x30, 0x0d, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01,
    0x01, 0x05, 0x00, 0x03, 0x82, 0x02, 0x0f, 0x00,
];

#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
const EC_DSA_SECP_256_R1_ASN1_HEADER: [u8; 26] = [
    0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a,
    0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, 0x03, 0x42, 0x00,
];

#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
const EC_DSA_SECP_384_R1_ASN1_HEADER: [u8; 23] = [
    0x30, 0x76, 0x30, 0x10, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x05, 0x2b,
    0x81, 0x04, 0x00, 0x22, 0x03, 0x62, 0x00,
];

#[cfg(test)]
mod test {
    use crate::test::certificate;
    #[cfg(feature = "serial-number-bigint")]
    use num_bigint::BigUint;
    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    use x509_parser::prelude::*;

    #[test]
    fn subject_summary() {
        let cert = certificate();
        assert_eq!("foobar.com", cert.subject_summary());
    }

    #[test]
    fn email_addresses() {
        let cert = certificate();
        assert_eq!(Vec::<String>::new(), cert.email_addresses().unwrap());
    }

    #[test]
    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    fn issuer() {
        let cert = certificate();
        let issuer = cert.issuer();
        let (_, name) = X509Name::from_der(&issuer).unwrap();
        let name_str = name.to_string_with_registry(oid_registry()).unwrap();
        assert_eq!(
            "C=US, ST=CALIFORNIA, L=PALO ALTO, O=FOOBAR LLC, OU=DEV LAND, CN=FOOBAR.COM",
            name_str
        );
    }

    #[test]
    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    fn subject() {
        let cert = certificate();
        let subject = cert.subject();
        let (_, name) = X509Name::from_der(&subject).unwrap();
        let name_str = name.to_string_with_registry(oid_registry()).unwrap();
        assert_eq!(
            "C=US, ST=CALIFORNIA, L=PALO ALTO, O=FOOBAR LLC, OU=DEV LAND, CN=FOOBAR.COM",
            name_str
        );
    }

    #[test]
    #[cfg(feature = "serial-number-bigint")]
    #[allow(deprecated)]
    fn serial_number() {
        let cert = certificate();
        let serial_number = cert.serial_number().unwrap();
        assert_eq!(BigUint::from(16452297291294946383_u128), serial_number);
    }
}
