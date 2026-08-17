use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::{CFAllocatorRef, CFTypeID, OSStatus};
use core_foundation_sys::data::CFDataRef;
#[cfg(target_os = "macos")]
use core_foundation_sys::dictionary::CFDictionaryRef;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
use core_foundation_sys::error::CFErrorRef;
use core_foundation_sys::string::CFStringRef;

use crate::base::SecCertificateRef;
use crate::base::SecKeyRef;
use crate::base::SecKeychainRef;

extern "C" {
    #[cfg(target_os = "macos")]
    pub static kSecPropertyKeyType: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyKeyLabel: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyKeyLocalizedLabel: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyKeyValue: CFStringRef;

    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeWarning: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeSuccess: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeSection: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeData: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeString: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeURL: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeDate: CFStringRef;

    // certificate policies
    pub static kSecPolicyAppleX509Basic: CFStringRef;
    pub static kSecPolicyAppleSSL: CFStringRef;
    pub static kSecPolicyAppleSMIME: CFStringRef;
    pub static kSecPolicyAppleEAP: CFStringRef;
    pub static kSecPolicyAppleIPsec: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPolicyApplePKINITClient: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPolicyApplePKINITServer: CFStringRef;
    pub static kSecPolicyAppleCodeSigning: CFStringRef;
    pub static kSecPolicyMacAppStoreReceipt: CFStringRef;
    pub static kSecPolicyAppleIDValidation: CFStringRef;
    pub static kSecPolicyAppleTimeStamping: CFStringRef;
    pub static kSecPolicyAppleRevocation: CFStringRef;
    pub static kSecPolicyApplePassbookSigning: CFStringRef;
    pub static kSecPolicyApplePayIssuerEncryption: CFStringRef;

    pub fn SecCertificateGetTypeID() -> CFTypeID;
    pub fn SecCertificateCreateWithData(
        allocator: CFAllocatorRef,
        data: CFDataRef,
    ) -> SecCertificateRef;
    pub fn SecCertificateAddToKeychain(
        certificate: SecCertificateRef,
        keychain: SecKeychainRef,
    ) -> OSStatus;
    pub fn SecCertificateCopyData(certificate: SecCertificateRef) -> CFDataRef;
    pub fn SecCertificateCopySubjectSummary(certificate: SecCertificateRef) -> CFStringRef;
    pub fn SecCertificateCopyCommonName(
        certificate: SecCertificateRef,
        common_name: *mut CFStringRef,
    ) -> OSStatus;
    pub fn SecCertificateCopyEmailAddresses(
        certificate: SecCertificateRef,
        email_addresses: *mut CFArrayRef,
    ) -> OSStatus;
    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecCertificateCopyNormalizedIssuerSequence(certificate: SecCertificateRef) -> CFDataRef;
    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecCertificateCopyNormalizedSubjectSequence(certificate: SecCertificateRef)
        -> CFDataRef;
    #[cfg(target_os = "macos")]
    #[cfg_attr(target_arch = "aarch64", link_name = "SecCertificateCopyPublicKey$LEGACYMAC")]
    #[deprecated(note = "Deprecated by Apple. May not work any more. Use SecCertificateCopyKey")]
    pub fn SecCertificateCopyPublicKey(
        certificate: SecCertificateRef,
        key: *mut SecKeyRef,
    ) -> OSStatus;
    #[cfg(any(feature = "OSX_10_14", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecCertificateCopyKey(certificate: SecCertificateRef) -> SecKeyRef;
    #[cfg(any(feature = "OSX_10_13", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecCertificateCopySerialNumberData(
        certificate: SecCertificateRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
    #[cfg(target_os = "macos")]
    pub fn SecCertificateCopyValues(
        certificate: SecCertificateRef,
        keys: CFArrayRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}
