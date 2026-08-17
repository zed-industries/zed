//! Trust evaluation support.

use core_foundation::array::CFArray;
#[cfg(target_os = "macos")]
use core_foundation::array::CFArrayRef;
use core_foundation::base::TCFType;
use core_foundation::data::CFData;
use core_foundation::date::CFDate;
use core_foundation_sys::base::{Boolean, CFIndex};

use security_framework_sys::trust::*;
use std::ptr;

use crate::base::Result;
use crate::certificate::SecCertificate;
use crate::cvt;
use crate::key::SecKey;
use crate::policy::SecPolicy;
use core_foundation::error::{CFError, CFErrorRef};

/// The result of trust evaluation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TrustResult(SecTrustResultType);

impl TrustResult {
    /// An invalid setting or result.
    pub const INVALID: Self = Self(kSecTrustResultInvalid);

    /// You may proceed.
    pub const PROCEED: Self = Self(kSecTrustResultProceed);

    /// Indicates a denial by the user, do not proceed.
    pub const DENY: Self = Self(kSecTrustResultDeny);

    /// The certificate is implicitly trusted.
    pub const UNSPECIFIED: Self = Self(kSecTrustResultUnspecified);

    /// Indicates a trust policy failure that the user can override.
    pub const RECOVERABLE_TRUST_FAILURE: Self = Self(kSecTrustResultRecoverableTrustFailure);

    /// Indicates a trust policy failure that the user cannot override.
    pub const FATAL_TRUST_FAILURE: Self = Self(kSecTrustResultFatalTrustFailure);

    /// An error not related to trust validation.
    pub const OTHER_ERROR: Self = Self(kSecTrustResultOtherError);
}

impl TrustResult {
    /// Returns true if the result is "successful" - specifically `PROCEED` or `UNSPECIFIED`.
    #[inline]
    #[must_use]
    pub fn success(self) -> bool {
        matches!(self, Self::PROCEED | Self::UNSPECIFIED)
    }
}

declare_TCFType! {
    /// A type representing a trust evaluation for a certificate.
    SecTrust, SecTrustRef
}
impl_TCFType!(SecTrust, SecTrustRef, SecTrustGetTypeID);

unsafe impl Sync for SecTrust {}
unsafe impl Send for SecTrust {}

#[cfg(target_os = "macos")]
bitflags::bitflags! {
    /// The option flags used to configure the evaluation of a `SecTrust`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TrustOptions: SecTrustOptionFlags {
        /// Allow expired certificates (except for the root certificate).
        const ALLOW_EXPIRED = kSecTrustOptionAllowExpired;
        /// Allow CA certificates as leaf certificates.
        const LEAF_IS_CA = kSecTrustOptionLeafIsCA;
        /// Allow network downloads of CA certificates.
        const FETCH_ISSUER_FROM_NET = kSecTrustOptionFetchIssuerFromNet;
        /// Allow expired root certificates.
        const ALLOW_EXPIRED_ROOT = kSecTrustOptionAllowExpiredRoot;
        /// Require a positive revocation check for each certificate.
        const REQUIRE_REVOCATION_PER_CERT =  kSecTrustOptionRequireRevPerCert;
        /// Use TrustSettings instead of anchors.
        const USE_TRUST_SETTINGS = kSecTrustOptionUseTrustSettings;
        /// Treat properly self-signed certificates as anchors implicitly.
        const IMPLICIT_ANCHORS =  kSecTrustOptionImplicitAnchors;
    }
}

impl SecTrust {
    /// Creates a `SecTrustRef` that is configured with a certificate chain, for validating
    /// that chain against a collection of policies.
    pub fn create_with_certificates(
        certs: &[SecCertificate],
        policies: &[SecPolicy],
    ) -> Result<Self> {
        let cert_array = CFArray::from_CFTypes(certs);
        let policy_array = CFArray::from_CFTypes(policies);
        let mut trust = ptr::null_mut();
        unsafe {
            cvt(SecTrustCreateWithCertificates(
                cert_array.as_CFTypeRef(),
                policy_array.as_CFTypeRef(),
                &mut trust,
            ))?;
            Ok(Self(trust))
        }
    }

    /// Sets the date and time against which the certificates in this trust object
    /// are verified.
    #[inline]
    pub fn set_trust_verify_date(&mut self, date: &CFDate) -> Result<()> {
        unsafe { cvt(SecTrustSetVerifyDate(self.0, date.as_concrete_TypeRef())) }
    }

    /// Sets additional anchor certificates used to validate trust.
    pub fn set_anchor_certificates(&mut self, certs: &[SecCertificate]) -> Result<()> {
        let certs = CFArray::from_CFTypes(certs);

        unsafe {
            cvt(SecTrustSetAnchorCertificates(
                self.0,
                certs.as_concrete_TypeRef(),
            ))
        }
    }

    /// Retrieves the anchor (root) certificates stored by macOS
    #[cfg(target_os = "macos")]
    pub fn copy_anchor_certificates() -> Result<Vec<SecCertificate>> {
        let mut array: CFArrayRef = ptr::null();

        unsafe {
            cvt(SecTrustCopyAnchorCertificates(&mut array))?;
        }

        if array.is_null() {
            return Ok(vec![]);
        }

        let array = unsafe { CFArray::<SecCertificate>::wrap_under_create_rule(array) };
        Ok(array.into_iter().map(|c| c.clone()).collect())
    }

    /// If set to `true`, only the certificates specified by
    /// `set_anchor_certificates` will be trusted, but not globally trusted
    /// certificates.
    #[inline]
    pub fn set_trust_anchor_certificates_only(&mut self, only: bool) -> Result<()> {
        unsafe { cvt(SecTrustSetAnchorCertificatesOnly(self.0, Boolean::from(only))) }
    }

    /// Sets the policy used to evaluate trust.
    #[inline]
    pub fn set_policy(&mut self, policy: &SecPolicy) -> Result<()> {
        unsafe { cvt(SecTrustSetPolicies(self.0, policy.as_CFTypeRef())) }
    }

    /// Sets option flags for customizing evaluation of a trust object.
    #[cfg(target_os = "macos")]
    #[inline]
    pub fn set_options(&mut self, options: TrustOptions) -> Result<()> {
        unsafe { cvt(SecTrustSetOptions(self.0, options.bits())) }
    }

    /// Indicates whether this trust object is permitted to
    /// fetch missing intermediate certificates from the network.
    pub fn get_network_fetch_allowed(&mut self) -> Result<bool> {
        let mut allowed = 0;

        unsafe { cvt(SecTrustGetNetworkFetchAllowed(self.0, &mut allowed))? };

        Ok(allowed != 0)
    }

    /// Specifies whether this trust object is permitted to
    /// fetch missing intermediate certificates from the network.
    #[inline]
    pub fn set_network_fetch_allowed(&mut self, allowed: bool) -> Result<()> {
        unsafe { cvt(SecTrustSetNetworkFetchAllowed(self.0, allowed as u8)) }
    }

    /// Attaches Online Certificate Status Protocol (OSCP) response data
    /// to this trust object.
    pub fn set_trust_ocsp_response<I: Iterator<Item = impl AsRef<[u8]>>>(
        &mut self,
        ocsp_response: I,
    ) -> Result<()> {
        let response: Vec<CFData> = ocsp_response
            .into_iter()
            .map(|bytes| CFData::from_buffer(bytes.as_ref()))
            .collect();

        let response = CFArray::from_CFTypes(&response);

        unsafe { cvt(SecTrustSetOCSPResponse(self.0, response.as_CFTypeRef())) }
    }

    /// Attaches signed certificate timestamp data to this trust object.
    #[cfg(any(feature = "OSX_10_14", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn set_signed_certificate_timestamps<I: Iterator<Item = impl AsRef<[u8]>>>(
        &mut self,
        scts: I,
    ) -> Result<()> {
        let scts: Vec<CFData> = scts
            .into_iter()
            .map(|bytes| CFData::from_buffer(bytes.as_ref()))
            .collect();

        let scts = CFArray::from_CFTypes(&scts);

        unsafe { cvt(SecTrustSetSignedCertificateTimestamps(self.0, scts.as_concrete_TypeRef())) }
    }

    /// Returns the public key for a leaf certificate after it has been evaluated.
    #[inline]
    pub fn copy_public_key(&mut self) -> Result<SecKey> {
        unsafe {
            Ok(SecKey::wrap_under_create_rule(SecTrustCopyPublicKey(
                self.0,
            )))
        }
    }

    /// Evaluates trust.
    #[deprecated(note = "use evaluate_with_error")]
    pub fn evaluate(&self) -> Result<TrustResult> {
        #[allow(deprecated)]
        unsafe {
            let mut result = kSecTrustResultInvalid;
            cvt(SecTrustEvaluate(self.0, &mut result))?;
            Ok(TrustResult(result))
        }
    }

    /// Evaluates trust. Requires macOS 10.14 or iOS, otherwise it just calls `evaluate()`
    pub fn evaluate_with_error(&self) -> Result<(), CFError> {
        #[cfg(any(feature = "OSX_10_14", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
        unsafe {
            let mut error: CFErrorRef = ::std::ptr::null_mut();
            if !SecTrustEvaluateWithError(self.0, &mut error) {
                assert!(!error.is_null());
                let error = CFError::wrap_under_create_rule(error);
                return Err(error);
            }
            Ok(())
        }
        #[cfg(not(any(feature = "OSX_10_14", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos")))]
        #[allow(deprecated)]
        {
            use security_framework_sys::base::errSecNotTrusted;
            use security_framework_sys::base::errSecTrustSettingDeny;

            let code = match self.evaluate() {
                Ok(res) if res.success() => return Ok(()),
                Ok(TrustResult::DENY) => errSecTrustSettingDeny,
                Ok(_) => errSecNotTrusted,
                Err(err) => err.code(),
            };
            Err(cferror_from_osstatus(code))
        }
    }

    /// Returns the number of certificates in an evaluated certificate chain.
    ///
    /// Note: evaluate must first be called on the `SecTrust`.
    #[inline(always)]
    #[must_use]
    pub fn certificate_count(&self) -> CFIndex {
        unsafe { SecTrustGetCertificateCount(self.0) }
    }

    /// Returns a specific certificate from the certificate chain used to evaluate trust.
    ///
    /// Note: evaluate must first be called on the `SecTrust`.
    #[deprecated(note = "deprecated by Apple")]
    #[must_use]
    pub fn certificate_at_index(&self, ix: CFIndex) -> Option<SecCertificate> {
        #[allow(deprecated)]
        unsafe {
            if self.certificate_count() <= ix {
                None
            } else {
                let certificate = SecTrustGetCertificateAtIndex(self.0, ix);
                Some(SecCertificate::wrap_under_get_rule(certificate.cast()))
            }
        }
    }
}

#[cfg(not(any(feature = "OSX_10_14", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos")))]
extern "C" {
    fn CFErrorCreate(allocator: core_foundation_sys::base::CFAllocatorRef, domain: core_foundation_sys::string::CFStringRef, code: CFIndex, userInfo: core_foundation_sys::dictionary::CFDictionaryRef) -> CFErrorRef;
}

#[cfg(not(any(feature = "OSX_10_14", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos")))]
fn cferror_from_osstatus(code: core_foundation_sys::base::OSStatus) -> CFError {
    unsafe {
        let error = CFErrorCreate(ptr::null_mut(), core_foundation_sys::error::kCFErrorDomainOSStatus, code as _, ptr::null_mut());
        assert!(!error.is_null());
        CFError::wrap_under_create_rule(error)
    }
}

#[cfg(test)]
mod test {
    use crate::policy::SecPolicy;
    use crate::secure_transport::SslProtocolSide;
    use crate::test::certificate;
    use crate::trust::SecTrust;

    #[test]
    #[allow(deprecated)]
    fn create_with_certificates() {
        let cert = certificate();
        let ssl_policy = SecPolicy::create_ssl(SslProtocolSide::CLIENT, Some("certifi.io"));
        let trust = SecTrust::create_with_certificates(&[cert], &[ssl_policy]).unwrap();
        assert!(!trust.evaluate().unwrap().success());
    }

    #[test]
    fn create_with_certificates_new() {
        let cert = certificate();
        let ssl_policy = SecPolicy::create_ssl(SslProtocolSide::CLIENT, Some("certifi.io"));
        let trust = SecTrust::create_with_certificates(&[cert], &[ssl_policy]).unwrap();
        assert!(trust.evaluate_with_error().is_err());
    }

    #[test]
    #[allow(deprecated)]
    fn certificate_count_and_at_index() {
        let cert = certificate();
        let ssl_policy = SecPolicy::create_ssl(SslProtocolSide::CLIENT, Some("certifi.io"));
        let trust = SecTrust::create_with_certificates(&[cert], &[ssl_policy]).unwrap();
        trust.evaluate().unwrap();

        let count = trust.certificate_count();
        assert_eq!(count, 1);

        let cert_bytes = trust.certificate_at_index(0).unwrap().to_der();
        assert_eq!(cert_bytes, certificate().to_der());
    }

    #[test]
    #[allow(deprecated)]
    fn certificate_count_and_at_index_new() {
        let cert = certificate();
        let ssl_policy = SecPolicy::create_ssl(SslProtocolSide::CLIENT, Some("certifi.io"));
        let trust = SecTrust::create_with_certificates(&[cert], &[ssl_policy]).unwrap();
        assert!(trust.evaluate_with_error().is_err());

        let count = trust.certificate_count();
        assert_eq!(count, 1);

        let cert_bytes = trust.certificate_at_index(0).unwrap().to_der();
        assert_eq!(cert_bytes, certificate().to_der());
    }

    #[test]
    #[allow(deprecated)]
    fn certificate_at_index_out_of_bounds() {
        let cert = certificate();
        let ssl_policy = SecPolicy::create_ssl(SslProtocolSide::CLIENT, Some("certifi.io"));

        let trust = SecTrust::create_with_certificates(&[cert.clone()], &[ssl_policy.clone()]).unwrap();
        trust.evaluate().unwrap();
        assert!(trust.certificate_at_index(1).is_none());

        let trust = SecTrust::create_with_certificates(&[cert], &[ssl_policy]).unwrap();
        assert!(trust.evaluate_with_error().is_err());
        assert!(trust.certificate_at_index(1).is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn set_policy() {
        let cert = certificate();
        let ssl_policy = SecPolicy::create_ssl(SslProtocolSide::CLIENT, Some("certifi.io.bogus"));
        let mut trust = SecTrust::create_with_certificates(&[cert], &[ssl_policy]).unwrap();
        let ssl_policy = SecPolicy::create_ssl(SslProtocolSide::CLIENT, Some("certifi.io"));
        trust.set_policy(&ssl_policy).unwrap();
        assert!(!trust.evaluate().unwrap().success());
    }

    #[test]
    fn set_policy_new() {
        let cert = certificate();
        let ssl_policy = SecPolicy::create_ssl(SslProtocolSide::CLIENT, Some("certifi.io.bogus"));
        let mut trust = SecTrust::create_with_certificates(&[cert], &[ssl_policy]).unwrap();
        let ssl_policy = SecPolicy::create_ssl(SslProtocolSide::CLIENT, Some("certifi.io"));
        trust.set_policy(&ssl_policy).unwrap();
        assert!(trust.evaluate_with_error().is_err());
    }
}
