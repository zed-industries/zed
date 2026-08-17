//! Security Policies support.
use core_foundation::base::CFOptionFlags;
use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use security_framework_sys::base::errSecParam;
use security_framework_sys::base::SecPolicyRef;
use security_framework_sys::policy::*;
use std::fmt;
use std::ptr;

use crate::secure_transport::SslProtocolSide;
use crate::Error;

declare_TCFType! {
    /// A type representing a certificate validation policy.
    SecPolicy, SecPolicyRef
}
impl_TCFType!(SecPolicy, SecPolicyRef, SecPolicyGetTypeID);

unsafe impl Sync for SecPolicy {}
unsafe impl Send for SecPolicy {}

impl fmt::Debug for SecPolicy {
    #[cold]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("SecPolicy").finish()
    }
}

bitflags::bitflags! {
    /// The flags used to specify revocation policy options.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RevocationPolicy: CFOptionFlags {
        /// Perform revocation checking using OCSP (Online Certificate Status Protocol).
        const OCSP_METHOD = kSecRevocationOCSPMethod;
        /// Perform revocation checking using the CRL (Certification Revocation List) method.
        const CRL_METHOD = kSecRevocationCRLMethod;
        /// Prefer CRL revocation checking over OCSP; by default, OCSP is preferred.
        const PREFER_CRL = kSecRevocationPreferCRL;
        /// Require a positive response to pass the policy.
        const REQUIRE_POSITIVE_RESPONSE = kSecRevocationRequirePositiveResponse;
        /// Consult only locally cached replies; do not use network access.
        const NETWORK_ACCESS_DISABLED = kSecRevocationNetworkAccessDisabled;
        /// Perform either OCSP or CRL checking.
        const USE_ANY_METHOD_AVAILABLE = kSecRevocationUseAnyAvailableMethod;
    }
}

impl SecPolicy {
    /// Creates a `SecPolicy` for evaluating SSL certificate chains.
    ///
    /// The side which you are evaluating should be provided (i.e. pass `SslSslProtocolSide::SERVER` if
    /// you are a client looking to validate a server's certificate chain).
    pub fn create_ssl(protocol_side: SslProtocolSide, hostname: Option<&str>) -> Self {
        let hostname = hostname.map(CFString::new);
        let hostname = hostname
            .as_ref()
            .map(|s| s.as_concrete_TypeRef())
            .unwrap_or(ptr::null_mut());
        let is_server = protocol_side == SslProtocolSide::SERVER;
        unsafe {
            let policy = SecPolicyCreateSSL(is_server as _, hostname);
            Self::wrap_under_create_rule(policy)
        }
    }

    /// Creates a `SecPolicy` for checking revocation of certificates.
    ///
    /// If you do not specify this policy creating a `SecTrust` object, the system defaults
    /// will be used during evaluation.
    pub fn create_revocation(options: RevocationPolicy) -> crate::Result<Self> {
        let policy = unsafe { SecPolicyCreateRevocation(options.bits()) };

        if policy.is_null() {
            Err(Error::from_code(errSecParam))
        } else {
            Ok(unsafe { Self::wrap_under_create_rule(policy) })
        }
    }

    /// Returns a policy object for the default X.509 policy.
    #[must_use]
    pub fn create_x509() -> Self {
        unsafe {
            let policy = SecPolicyCreateBasicX509();
            Self::wrap_under_create_rule(policy)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::policy::SecPolicy;
    use crate::secure_transport::SslProtocolSide;

    #[test]
    fn create_ssl() {
        SecPolicy::create_ssl(SslProtocolSide::SERVER, Some("certifi.org"));
    }
}
