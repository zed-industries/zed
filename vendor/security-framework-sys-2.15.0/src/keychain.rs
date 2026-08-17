#[cfg(target_os = "macos")]
use core_foundation_sys::base::CFTypeRef;
use core_foundation_sys::base::{Boolean, CFTypeID, OSStatus};
use std::os::raw::{c_char, c_uint, c_void};

#[cfg(target_os = "macos")]
use crate::base::SecKeychainItemRef;
use crate::base::{SecAccessRef, SecKeychainRef};

pub const SEC_KEYCHAIN_SETTINGS_VERS1: c_uint = 1;

#[repr(C)]
pub struct SecKeychainSettings {
    pub version: c_uint,
    pub lockOnSleep: Boolean,
    pub useLockInterval: Boolean,
    pub lockInterval: c_uint,
}

/// Like Apple's headers, it assumes Little Endian,
/// as there are no supported Big Endian machines any more :(
macro_rules! char_lit {
    ($e:expr) => {
        ($e[3] as u32) + (($e[2] as u32) << 8) + (($e[1] as u32) << 16) + (($e[0] as u32) << 24)
    };
}

macro_rules! char_lit_swapped {
    ($e:expr) => {
        ($e[0] as u32) + (($e[1] as u32) << 8) + (($e[2] as u32) << 16) + (($e[3] as u32) << 24)
    };
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum SecProtocolType {
    FTP = char_lit!(b"ftp "),
    FTPAccount = char_lit!(b"ftpa"),
    HTTP = char_lit!(b"http"),
    IRC = char_lit!(b"irc "),
    NNTP = char_lit!(b"nntp"),
    POP3 = char_lit!(b"pop3"),
    SMTP = char_lit!(b"smtp"),
    SOCKS = char_lit!(b"sox "),
    IMAP = char_lit!(b"imap"),
    LDAP = char_lit!(b"ldap"),
    AppleTalk = char_lit!(b"atlk"),
    AFP = char_lit!(b"afp "),
    Telnet = char_lit!(b"teln"),
    SSH = char_lit!(b"ssh "),
    FTPS = char_lit!(b"ftps"),
    HTTPS = char_lit!(b"htps"),
    HTTPProxy = char_lit!(b"htpx"),
    HTTPSProxy = char_lit!(b"htsx"),
    FTPProxy = char_lit!(b"ftpx"),
    CIFS = char_lit!(b"cifs"),
    SMB = char_lit!(b"smb "),
    RTSP = char_lit!(b"rtsp"),
    RTSPProxy = char_lit!(b"rtsx"),
    DAAP = char_lit!(b"daap"),
    EPPC = char_lit!(b"eppc"),
    IPP = char_lit!(b"ipp "),
    NNTPS = char_lit!(b"ntps"),
    LDAPS = char_lit!(b"ldps"),
    TelnetS = char_lit!(b"tels"),
    IMAPS = char_lit!(b"imps"),
    IRCS = char_lit!(b"ircs"),
    POP3S = char_lit!(b"pops"),
    CVSpserver = char_lit!(b"cvsp"),
    SVN = char_lit!(b"svn "),
    Any = 0,
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum SecAuthenticationType {
    // [sic] Apple has got two related enums each with a different endianness!
    NTLM = char_lit_swapped!(b"ntlm"),
    MSN = char_lit_swapped!(b"msna"),
    DPA = char_lit_swapped!(b"dpaa"),
    RPA = char_lit_swapped!(b"rpaa"),
    HTTPBasic = char_lit_swapped!(b"http"),
    HTTPDigest = char_lit_swapped!(b"httd"),
    HTMLForm = char_lit_swapped!(b"form"),
    Default = char_lit_swapped!(b"dflt"),
    Any = 0,
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SecPreferencesDomain {
    User = 0,
    System = 1,
    Common = 2,
    Dynamic = 3,
}

extern "C" {
    pub fn SecKeychainGetTypeID() -> CFTypeID;
    pub fn SecKeychainCopyDefault(keychain: *mut SecKeychainRef) -> OSStatus;
    pub fn SecKeychainCopyDomainDefault(
        domain: SecPreferencesDomain,
        keychain: *mut SecKeychainRef,
    ) -> OSStatus;
    pub fn SecKeychainCreate(
        pathName: *const c_char,
        passwordLength: c_uint,
        password: *const c_void,
        promptUser: Boolean,
        initialAccess: SecAccessRef,
        keychain: *mut SecKeychainRef,
    ) -> OSStatus;
    pub fn SecKeychainOpen(pathName: *const c_char, keychain: *mut SecKeychainRef) -> OSStatus;
    pub fn SecKeychainUnlock(
        keychain: SecKeychainRef,
        passwordLength: c_uint,
        password: *const c_void,
        usePassword: Boolean,
    ) -> OSStatus;
    #[cfg(target_os = "macos")]
    pub fn SecKeychainFindGenericPassword(
        keychainOrArray: CFTypeRef,
        serviceNameLength: u32,
        serviceName: *const c_char,
        accountNameLength: u32,
        accountName: *const c_char,
        passwordLength: *mut u32,
        passwordData: *mut *mut c_void,
        itemRef: *mut SecKeychainItemRef,
    ) -> OSStatus;

    #[cfg(target_os = "macos")]
    pub fn SecKeychainFindInternetPassword(
        keychainOrArray: CFTypeRef,
        serverNameLength: u32,
        serverName: *const c_char,
        securityDomainLength: u32,
        securityDomain: *const c_char,
        accountNameLength: u32,
        accountName: *const c_char,
        pathLength: u32,
        path: *const c_char,
        port: u16,
        protocol: SecProtocolType,
        authenticationType: SecAuthenticationType,
        passwordLength: *mut u32,
        passwordData: *mut *mut c_void,
        itemRef: *mut SecKeychainItemRef,
    ) -> OSStatus;

    #[cfg(target_os = "macos")]
    pub fn SecKeychainAddGenericPassword(
        keychain: SecKeychainRef,
        serviceNameLength: u32,
        serviceName: *const c_char,
        accountNameLength: u32,
        accountName: *const c_char,
        passwordLength: u32,
        passwordData: *const c_void,
        itemRef: *mut SecKeychainItemRef,
    ) -> OSStatus;

    #[cfg(target_os = "macos")]
    pub fn SecKeychainAddInternetPassword(
        keychain: SecKeychainRef,
        serverNameLength: u32,
        serverName: *const c_char,
        securityDomainLength: u32,
        securityDomain: *const c_char,
        accountNameLength: u32,
        accountName: *const c_char,
        pathLength: u32,
        path: *const c_char,
        port: u16,
        protocol: SecProtocolType,
        authenticationType: SecAuthenticationType,
        passwordLength: u32,
        passwordData: *const c_void,
        itemRef: *mut SecKeychainItemRef,
    ) -> OSStatus;

    pub fn SecKeychainSetSettings(
        keychain: SecKeychainRef,
        newSettings: *const SecKeychainSettings,
    ) -> OSStatus;

    #[cfg(target_os = "macos")]
    pub fn SecKeychainGetUserInteractionAllowed(state: *mut Boolean) -> OSStatus;

    #[cfg(target_os = "macos")]
    pub fn SecKeychainSetUserInteractionAllowed(state: Boolean) -> OSStatus;
}
