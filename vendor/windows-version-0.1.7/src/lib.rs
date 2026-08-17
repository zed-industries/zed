#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![cfg_attr(not(test), no_std)]
#![allow(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]

mod bindings;
use bindings::*;

/// Operating system version information.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OsVersion {
    /// The major version number of the operating system.
    pub major: u32,

    /// The minor version number of the operating system.
    pub minor: u32,

    /// The major version number of the latest service pack installed on the system.
    pub pack: u32,

    /// The build number of the operating system.
    pub build: u32,
}

impl OsVersion {
    /// Creates a new `OsVersion` with the given values.
    pub const fn new(major: u32, minor: u32, pack: u32, build: u32) -> Self {
        Self {
            major,
            minor,
            pack,
            build,
        }
    }

    /// Gets the version information of the currently running operating system.
    #[cfg(not(test))]
    pub fn current() -> Self {
        let mut info = OSVERSIONINFOEXW::new();

        unsafe {
            RtlGetVersion(&mut info as *mut _ as *mut _);
        }

        Self {
            major: info.dwMajorVersion,
            minor: info.dwMinorVersion,
            pack: info.wServicePackMajor as u32,
            build: info.dwBuildNumber,
        }
    }

    /// Hook used for testing `ge`.
    #[cfg(test)]
    fn current() -> Self {
        test::test_current()
    }
}

/// Determines if the currently running operating system is a Windows Server release.
pub fn is_server() -> bool {
    let mut info = OSVERSIONINFOEXW::new();

    unsafe {
        RtlGetVersion(&mut info as *mut _ as *mut _);
    }

    info.wProductType as u32 != VER_NT_WORKSTATION
}

/// Gets the revision number of the operating system.
pub fn revision() -> u32 {
    let mut value = [0; 4];
    let mut len = 4;

    let result = unsafe {
        RegGetValueA(
            HKEY_LOCAL_MACHINE,
            b"SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\0".as_ptr(),
            b"UBR\0".as_ptr(),
            RRF_RT_REG_DWORD,
            core::ptr::null_mut(),
            value.as_mut_ptr() as _,
            &mut len,
        )
    };

    if result == 0 {
        u32::from_le_bytes(value)
    } else {
        0
    }
}

impl OSVERSIONINFOEXW {
    fn new() -> Self {
        Self {
            dwOSVersionInfoSize: core::mem::size_of::<Self>() as u32,
            ..Default::default()
        }
    }
}

#[cfg(test)]
#[allow(clippy::nonminimal_bool)] // explicit logic is intentionally being tested
mod test {
    use super::*;
    use std::sync::RwLock;

    static TEST_CURRENT: RwLock<OsVersion> = RwLock::new(OsVersion::new(0, 0, 0, 0));

    pub fn test_current() -> OsVersion {
        *TEST_CURRENT.read().unwrap()
    }

    fn set_current(version: OsVersion) {
        *TEST_CURRENT.write().unwrap() = version;
    }

    #[test]
    fn test() {
        assert_eq!(OsVersion::current(), OsVersion::new(0, 0, 0, 0));

        set_current(OsVersion::new(1, 2, 3, 4));
        assert_eq!(OsVersion::current(), OsVersion::new(1, 2, 3, 4));

        set_current(OsVersion::new(10, 0, 0, 0));
        assert!(OsVersion::current() >= OsVersion::new(9, 0, 0, 0));
        assert!(OsVersion::current() >= OsVersion::new(10, 0, 0, 0));
        assert!(!(OsVersion::current() >= OsVersion::new(11, 0, 0, 0)));

        set_current(OsVersion::new(10, 100, 0, 0));
        assert!(OsVersion::current() >= OsVersion::new(10, 99, 0, 0));
        assert!(OsVersion::current() >= OsVersion::new(10, 100, 0, 0));
        assert!(!(OsVersion::current() >= OsVersion::new(10, 101, 0, 0)));

        set_current(OsVersion::new(10, 100, 1000, 0));
        assert!(OsVersion::current() >= OsVersion::new(10, 100, 999, 0));
        assert!(OsVersion::current() >= OsVersion::new(10, 100, 1000, 0));
        assert!(!(OsVersion::current() >= OsVersion::new(10, 100, 1001, 0)));

        set_current(OsVersion::new(10, 100, 1_000, 10_000));
        assert!(OsVersion::current() >= OsVersion::new(10, 100, 1_000, 9_999));
        assert!(OsVersion::current() >= OsVersion::new(10, 100, 1_000, 10_000));
        assert!(!(OsVersion::current() >= OsVersion::new(10, 100, 1_000, 10_001)));
    }

    // These tests just ensure the queries succeed without validating the results.
    #[test]
    fn test_uncertain() {
        is_server();
        assert_ne!(revision(), 0);
    }
}
