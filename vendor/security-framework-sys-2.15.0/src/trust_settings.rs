use crate::base::SecCertificateRef;
use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::CFTypeRef;
use core_foundation_sys::base::OSStatus;

pub type SecTrustSettingsDomain = u32;

pub const kSecTrustSettingsDomainUser: SecTrustSettingsDomain = 0;
pub const kSecTrustSettingsDomainAdmin: SecTrustSettingsDomain = 1;
pub const kSecTrustSettingsDomainSystem: SecTrustSettingsDomain = 2;

pub type SecTrustSettingsResult = u32;

pub const kSecTrustSettingsResultInvalid: SecTrustSettingsResult = 0;
pub const kSecTrustSettingsResultTrustRoot: SecTrustSettingsResult = 1;
pub const kSecTrustSettingsResultTrustAsRoot: SecTrustSettingsResult = 2;
pub const kSecTrustSettingsResultDeny: SecTrustSettingsResult = 3;
pub const kSecTrustSettingsResultUnspecified: SecTrustSettingsResult = 4;

extern "C" {
    pub fn SecTrustSettingsCopyCertificates(
        domain: SecTrustSettingsDomain,
        certsOut: *mut CFArrayRef,
    ) -> OSStatus;
    pub fn SecTrustSettingsCopyTrustSettings(
        certificateRef: SecCertificateRef,
        domain: SecTrustSettingsDomain,
        trustSettings: *mut CFArrayRef,
    ) -> OSStatus;
    pub fn SecTrustSettingsSetTrustSettings(
        certificateRef: SecCertificateRef,
        domain: SecTrustSettingsDomain,
        trustSettingsDictOrArray: CFTypeRef,
    ) -> OSStatus;
}
