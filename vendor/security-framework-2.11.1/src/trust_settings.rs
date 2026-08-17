//! Querying trust settings.

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFIndex, TCFType};
use core_foundation::dictionary::CFDictionary;
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;

use core_foundation_sys::base::CFTypeRef;
use security_framework_sys::base::errSecNoTrustSettings;
use security_framework_sys::base::errSecSuccess;
use security_framework_sys::trust_settings::*;

use std::ptr;

use crate::base::Error;
use crate::base::Result;
use crate::certificate::SecCertificate;
use crate::cvt;

/// Which set of trust settings to query
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Domain {
    /// Per-user trust settings
    User = kSecTrustSettingsDomainUser,
    /// Locally administered, system-wide trust settings
    Admin = kSecTrustSettingsDomainAdmin,
    /// System trust settings
    System = kSecTrustSettingsDomainSystem,
}

impl From<Domain> for SecTrustSettingsDomain {
    #[inline]
    fn from(domain: Domain) -> SecTrustSettingsDomain {
        match domain {
            Domain::User => kSecTrustSettingsDomainUser,
            Domain::Admin => kSecTrustSettingsDomainAdmin,
            Domain::System => kSecTrustSettingsDomainSystem,
        }
    }
}

/// Trust settings for a specific certificate in a specific domain
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TrustSettingsForCertificate {
    /// Not used
    Invalid,

    /// This is a root certificate and is trusted, either explicitly or
    /// implicitly.
    TrustRoot,

    /// This is a non-root certificate but is explicitly trusted.
    TrustAsRoot,

    /// Cert is explicitly distrusted.
    Deny,

    /// Neither trusted nor distrusted.
    Unspecified,
}

impl TrustSettingsForCertificate {
    /// Create from `kSecTrustSettingsResult*` constant
    fn new(value: i64) -> Self {
        if value < 0 || value > i64::from(u32::max_value()) {
            return Self::Invalid;
        }
        match value as u32 {
            kSecTrustSettingsResultTrustRoot => Self::TrustRoot,
            kSecTrustSettingsResultTrustAsRoot => Self::TrustAsRoot,
            kSecTrustSettingsResultDeny => Self::Deny,
            kSecTrustSettingsResultUnspecified => Self::Unspecified,
            _ => Self::Invalid,
        }
    }
}

/// Allows access to the certificates and their trust settings in a given domain.
pub struct TrustSettings {
    domain: Domain,
}

impl TrustSettings {
    /// Create a new `TrustSettings` for the given domain.
    ///
    /// You can call `iter()` to discover the certificates with settings in this domain.
    ///
    /// Then you can call `tls_trust_settings_for_certificate()` with a given certificate
    /// to learn what the aggregate trust setting for that certificate within this domain.
    #[inline(always)]
    #[must_use]
    pub fn new(domain: Domain) -> Self {
        Self { domain }
    }

    /// Create an iterator over the certificates with settings in this domain.
    /// This produces an empty iterator if there are no such certificates.
    pub fn iter(&self) -> Result<TrustSettingsIter> {
        let array = unsafe {
            let mut array_ptr: CFArrayRef = ptr::null_mut();

            // SecTrustSettingsCopyCertificates returns errSecNoTrustSettings
            // if no items have trust settings in the given domain.  We map
            // that to an empty TrustSettings iterator.
            match SecTrustSettingsCopyCertificates(self.domain.into(), &mut array_ptr) {
                errSecNoTrustSettings => CFArray::from_CFTypes(&[]),
                errSecSuccess => CFArray::<SecCertificate>::wrap_under_create_rule(array_ptr),
                err => return Err(Error::from_code(err)),
            }
        };

        Ok(TrustSettingsIter { index: 0, array })
    }

    ///set trust settings to ""always trust this root certificate regardless of use.".
    /// Sets the trust settings for the provided certificate to "always trust this root certificate
    /// regardless of use."
    ///
    /// This method configures the trust settings for the specified certificate, indicating that it should
    /// always be trusted as a TLS root certificate, regardless of its usage.
    ///
    /// If successful, the trust settings are updated for the certificate in the given domain. If the
    /// certificate had no previous trust settings in the domain, new trust settings are created. If the
    /// certificate had existing trust settings, they are replaced with the new settings.
    ///
    /// It is not possible to modify per-user trust settings when not running in a GUI
    /// environment, if you try it will return error `2070: errSecInternalComponent`
    #[cfg(target_os="macos")]
    pub fn set_trust_settings_always(&self, cert: &SecCertificate) -> Result<()> {
        let domain = self.domain;
        let trust_settings: CFTypeRef = ptr::null_mut();
        cvt(unsafe {
            SecTrustSettingsSetTrustSettings(
                cert.as_CFTypeRef() as *mut _,
                domain.into(),
                trust_settings,
            )
        })
    }

    /// Returns the aggregate trust setting for the given certificate.
    ///
    /// This tells you whether the certificate should be trusted as a TLS
    /// root certificate.
    ///
    /// If the certificate has no trust settings in the given domain, the
    /// `errSecItemNotFound` error is returned.
    ///
    /// If the certificate has no specific trust settings for TLS in the
    /// given domain `None` is returned.
    ///
    /// Otherwise, the specific trust settings are aggregated and returned.
    pub fn tls_trust_settings_for_certificate(&self, cert: &SecCertificate)
        -> Result<Option<TrustSettingsForCertificate>> {
        let trust_settings = unsafe {
            let mut array_ptr: CFArrayRef = ptr::null_mut();
            let cert_ptr = cert.as_CFTypeRef() as *mut _;
            cvt(SecTrustSettingsCopyTrustSettings(cert_ptr,
                                                  self.domain.into(),
                                                  &mut array_ptr))?;
            CFArray::<CFDictionary>::wrap_under_create_rule(array_ptr)
        };

        for settings in trust_settings.iter() {
            // Reject settings for non-SSL policies
            let is_not_ssl_policy = {
                let policy_name_key = CFString::from_static_string("kSecTrustSettingsPolicyName");
                let ssl_policy_name = CFString::from_static_string("sslServer");

                let maybe_name: Option<CFString> = settings
                    .find(policy_name_key.as_CFTypeRef().cast())
                    .map(|name| unsafe { CFString::wrap_under_get_rule((*name).cast()) });

                matches!(maybe_name, Some(ref name) if name != &ssl_policy_name)
            };

            if is_not_ssl_policy {
                continue;
            }

            // Evaluate "effective trust settings" for this usage constraint.
            let maybe_trust_result = {
                let settings_result_key = CFString::from_static_string("kSecTrustSettingsResult");
                settings
                    .find(settings_result_key.as_CFTypeRef().cast())
                    .map(|num| unsafe { CFNumber::wrap_under_get_rule((*num).cast()) })
                    .and_then(|num| num.to_i64())
            };

            // "Note that an empty Trust Settings array means "always trust this cert,
            //  with a resulting kSecTrustSettingsResult of kSecTrustSettingsResultTrustRoot"."
            let trust_result = TrustSettingsForCertificate::new(maybe_trust_result
                .unwrap_or_else(|| i64::from(kSecTrustSettingsResultTrustRoot)));

            match trust_result {
                TrustSettingsForCertificate::Unspecified |
                TrustSettingsForCertificate::Invalid => { continue; },
                _ => return Ok(Some(trust_result)),
            }
        }

        // There were no more specific settings.  This might mean the certificate
        // is to be trusted anyway (since, eg, it's in system store), but leave
        // the caller to make this decision.
        Ok(None)
    }
}

/// Iterator over certificates.
pub struct TrustSettingsIter {
    array: CFArray<SecCertificate>,
    index: CFIndex,
}

impl Iterator for TrustSettingsIter {
    type Item = SecCertificate;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.len() {
            None
        } else {
            let cert = self.array.get(self.index).unwrap();
            self.index += 1;
            Some(cert.clone())
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = (self.array.len() as usize).saturating_sub(self.index as usize);
        (left, Some(left))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::certificate;

    fn list_for_domain(domain: Domain) {
        println!("--- domain: {domain:?}");
        let ts = TrustSettings::new(domain);
        let iterator = ts.iter().unwrap();

        for (i, cert) in iterator.enumerate() {
            println!("cert({i:?}) = {cert:?}");
            println!("  settings = {:?}", ts.tls_trust_settings_for_certificate(&cert));
        }
        println!("---");
    }

    #[test]
    fn list_for_user() {
        list_for_domain(Domain::User);
    }

    #[test]
    fn list_for_system() {
        list_for_domain(Domain::System);
    }

    #[test]
    fn list_for_admin() {
        list_for_domain(Domain::Admin);
    }

    #[test]
    fn test_system_certs_are_present() {
        let system = TrustSettings::new(Domain::System).iter().unwrap().count();

        // 168 at the time of writing
        assert!(system > 100);
    }

    #[test]
    fn test_isrg_root_exists_and_is_trusted() {
        let ts = TrustSettings::new(Domain::System);
        assert_eq!(
            ts.iter()
                .unwrap()
                .find(|cert| cert.subject_summary() == "ISRG Root X1")
                .and_then(|cert| ts.tls_trust_settings_for_certificate(&cert).unwrap()),
            None
        );
        // ^ this is a case where None means "always trust", according to Apple docs:
        //
        // "Note that an empty Trust Settings array means "always trust this cert,
        //  with a resulting kSecTrustSettingsResult of kSecTrustSettingsResultTrustRoot"."
    }

    #[test]
    fn test_unknown_cert_is_not_trusted() {
        let ts = TrustSettings::new(Domain::System);
        let cert = certificate();
        assert_eq!(ts.tls_trust_settings_for_certificate(&cert)
                   .err()
                   .unwrap()
                   .message(),
                   Some("The specified item could not be found in the keychain.".into()));
    }
}

