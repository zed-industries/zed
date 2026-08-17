use core_foundation_sys::base::OSStatus;
use core_foundation_sys::string::CFStringRef;
use std::os::raw::c_void;

pub enum OpaqueSecKeychainRef {}
pub type SecKeychainRef = *mut OpaqueSecKeychainRef;

pub enum OpaqueSecKeychainItemRef {}
pub type SecKeychainItemRef = *mut OpaqueSecKeychainItemRef;

// OSType from MacTypes.h
pub type SecKeychainAttrType = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecKeychainAttribute {
    pub tag: SecKeychainAttrType,
    pub length: u32,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecKeychainAttributeList {
    pub count: u32,
    pub attr: *mut SecKeychainAttribute,
}

pub enum OpaqueSecCertificateRef {}
pub type SecCertificateRef = *mut OpaqueSecCertificateRef;

pub enum OpaqueSecAccessRef {}
pub type SecAccessRef = *mut OpaqueSecAccessRef;

pub enum OpaqueSecAccessControlRef {}
pub type SecAccessControlRef = *mut OpaqueSecAccessControlRef;

pub enum OpaqueSecKeyRef {}
pub type SecKeyRef = *mut OpaqueSecKeyRef;

pub enum OpaqueSecIdentityRef {}
pub type SecIdentityRef = *mut OpaqueSecIdentityRef;

pub enum OpaqueSecPolicyRef {}
pub type SecPolicyRef = *mut OpaqueSecPolicyRef;

pub const errSecSuccess: OSStatus = 0;
pub const errSecUnimplemented: OSStatus = -4;
pub const errSecIO: OSStatus = -36;
pub const errSecParam: OSStatus = -50;
pub const errSecBadReq: OSStatus = -909;
pub const errSecNoTrustSettings: OSStatus = -25263;
pub const errSecAuthFailed: OSStatus = -25293;
pub const errSecDuplicateItem: OSStatus = -25299;
pub const errSecItemNotFound: OSStatus = -25300;
pub const errSecCreateChainFailed: OSStatus = -25318;
pub const errSecConversionError: OSStatus = -67594;
pub const errSecHostNameMismatch: OSStatus = -67602;
pub const errSecInvalidExtendedKeyUsage: OSStatus = -67609;
pub const errSecTrustSettingDeny: OSStatus = -67654;
pub const errSecCertificateRevoked: OSStatus = -67820;
pub const errSecNotTrusted: OSStatus = -67843;
pub const errSecInternalComponent: OSStatus = -2070;

extern "C" {
    // this is available on iOS 11.3+, MacOS 10.3+
    pub fn SecCopyErrorMessageString(status: OSStatus, reserved: *mut c_void) -> CFStringRef;
}
