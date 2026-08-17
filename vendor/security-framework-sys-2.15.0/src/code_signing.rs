use core_foundation_sys::base::CFTypeID;
use core_foundation_sys::base::OSStatus;
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::string::CFStringRef;
use core_foundation_sys::url::CFURLRef;

pub enum OpaqueSecRequirementRef {}
pub type SecRequirementRef = *mut OpaqueSecRequirementRef;

pub enum OpaqueSecCodeRef {}
pub type SecCodeRef = *mut OpaqueSecCodeRef;

pub enum OpaqueSecStaticCodeRef {}
pub type SecStaticCodeRef = *mut OpaqueSecStaticCodeRef;

pub type SecCSFlags = u32;
pub const kSecCSCheckAllArchitectures: SecCSFlags = 1 << 0;
pub const kSecCSDoNotValidateExecutable: SecCSFlags = 1 << 1;
pub const kSecCSDoNotValidateResources: SecCSFlags = 1 << 2;
pub const kSecCSBasicValidateOnly: SecCSFlags =
    kSecCSDoNotValidateExecutable | kSecCSDoNotValidateResources;
pub const kSecCSCheckNestedCode: SecCSFlags = 1 << 3;
pub const kSecCSStrictValidate: SecCSFlags = 1 << 4;
pub const kSecCSFullReport: SecCSFlags = 1 << 5;
pub const kSecCSCheckGatekeeperArchitectures: SecCSFlags = (1 << 6) | kSecCSCheckAllArchitectures;
pub const kSecCSRestrictSymlinks: SecCSFlags = 1 << 7;
pub const kSecCSRestrictToAppLike: SecCSFlags = 1 << 8;
pub const kSecCSRestrictSidebandData: SecCSFlags = 1 << 9;
pub const kSecCSUseSoftwareSigningCert: SecCSFlags = 1 << 10;
pub const kSecCSValidatePEH: SecCSFlags = 1 << 11;
pub const kSecCSSingleThreaded: SecCSFlags = 1 << 12;
// 13 - 15 are unused
// This is only available in macOS 11.3:
// pub const kSecCSAllowNetworkAccess: SecCSFlags = 1 << 16;
// 17 - 25 are unused
pub const kSecCSQuickCheck: SecCSFlags = 1 << 26;
pub const kSecCSCheckTrustedAnchors: SecCSFlags = 1 << 27;
pub const kSecCSReportProgress: SecCSFlags = 1 << 28;
pub const kSecCSNoNetworkAccess: SecCSFlags = 1 << 29;
pub const kSecCSEnforceRevocationChecks: SecCSFlags = 1 << 30;
pub const kSecCSConsiderExpiration: SecCSFlags = 1 << 31;

extern "C" {
    pub static kSecGuestAttributeArchitecture: CFStringRef;
    pub static kSecGuestAttributeAudit: CFStringRef;
    pub static kSecGuestAttributeCanonical: CFStringRef;
    pub static kSecGuestAttributeDynamicCode: CFStringRef;
    pub static kSecGuestAttributeDynamicCodeInfoPlist: CFStringRef;
    pub static kSecGuestAttributeHash: CFStringRef;
    pub static kSecGuestAttributeMachPort: CFStringRef;
    pub static kSecGuestAttributePid: CFStringRef;
    pub static kSecGuestAttributeSubarchitecture: CFStringRef;

    pub fn SecCodeGetTypeID() -> CFTypeID;
    pub fn SecStaticCodeGetTypeID() -> CFTypeID;
    pub fn SecRequirementGetTypeID() -> CFTypeID;

    pub fn SecCodeCheckValidity(
        code: SecCodeRef,
        flags: SecCSFlags,
        requirement: SecRequirementRef,
    ) -> OSStatus;

    pub fn SecCodeCopyGuestWithAttributes(
        host: SecCodeRef,
        attrs: CFDictionaryRef,
        flags: SecCSFlags,
        guest: *mut SecCodeRef,
    ) -> OSStatus;

    pub fn SecCodeCopyPath(
        code: SecStaticCodeRef,
        flags: SecCSFlags,
        path: *mut CFURLRef,
    ) -> OSStatus;

    pub fn SecCodeCopySelf(flags: SecCSFlags, out: *mut SecCodeRef) -> OSStatus;

    pub fn SecRequirementCreateWithString(
        text: CFStringRef,
        flags: SecCSFlags,
        requirement: *mut SecRequirementRef,
    ) -> OSStatus;

    pub fn SecStaticCodeCheckValidity(
        code: SecStaticCodeRef,
        flags: SecCSFlags,
        requirement: SecRequirementRef,
    ) -> OSStatus;

    pub fn SecStaticCodeCreateWithPath(
        path: CFURLRef,
        flags: SecCSFlags,
        code: *mut SecStaticCodeRef,
    ) -> OSStatus;
}
