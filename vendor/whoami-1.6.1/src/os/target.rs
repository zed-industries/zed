//! Unknown target, fake implementation.
//!
//! This can be used as a template when adding new target support.

use std::{
    ffi::OsString,
    io::{Error, ErrorKind},
};

use crate::{
    os::{Os, Target},
    Arch, DesktopEnv, Platform, Result,
};

impl Target for Os {
    #[inline(always)]
    fn langs(self) -> Result<String> {
        Ok("en/US".to_string())
    }

    #[inline(always)]
    fn realname(self) -> Result<OsString> {
        Ok("Anonymous".to_string().into())
    }

    #[inline(always)]
    fn username(self) -> Result<OsString> {
        Ok("anonymous".to_string().into())
    }

    #[inline(always)]
    fn devicename(self) -> Result<OsString> {
        Ok("Unknown".to_string().into())
    }

    #[inline(always)]
    fn hostname(self) -> Result<String> {
        Ok("localhost".to_string())
    }

    #[inline(always)]
    fn distro(self) -> Result<String> {
        Ok(format!("Unknown {}", self.platform()))
    }

    #[inline(always)]
    fn desktop_env(self) -> DesktopEnv {
        DesktopEnv::Unknown("Unknown".to_string())
    }

    #[inline(always)]
    fn platform(self) -> Platform {
        if cfg!(daku) {
            Platform::Unknown("Daku".to_string())
        } else if cfg!(target_os = "wasi") {
            Platform::Unknown("WASI".to_string())
        } else if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "macos") {
            Platform::MacOS
        } else if cfg!(target_os = "redox") {
            Platform::Redox
        } else if cfg!(target_os = "linux") {
            Platform::Linux
        } else if cfg!(target_os = "android") {
            Platform::Android
        } else if cfg!(target_os = "tvos") {
            Platform::Unknown("tvOS".to_string())
        } else if cfg!(target_os = "watchos") {
            Platform::Unknown("watchOS".to_string())
        } else if cfg!(target_os = "ios") {
            Platform::Unknown("iOS".to_string())
        } else if cfg!(target_os = "fuchsia") {
            Platform::Fuchsia
        } else if cfg!(target_os = "illumos") {
            Platform::Illumos
        } else if cfg!(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        )) {
            Platform::Bsd
        } else if cfg!(target_os = "haiku") {
            Platform::Unknown("Haiku".to_string())
        } else if cfg!(target_os = "vxworks") {
            Platform::Unknown("VxWorks".to_string())
        } else if cfg!(target_os = "nto") {
            Platform::Unknown("QNX Neutrino".to_string())
        } else if cfg!(target_os = "horizon") {
            Platform::Nintendo
        } else if cfg!(target_os = "vita") {
            Platform::PlayStation
        } else if cfg!(target_os = "hurd") {
            Platform::Hurd
        } else if cfg!(target_os = "aix") {
            Platform::Unknown("AIX OS".to_string())
        } else if cfg!(target_os = "espidf") {
            Platform::Unknown("ESP-IDF".to_string())
        } else if cfg!(target_os = "emscripten") {
            Platform::Unknown("Emscripten".to_string())
        } else if cfg!(target_os = "solaris") {
            Platform::Unknown("Solaris".to_string())
        } else if cfg!(target_os = "l4re") {
            Platform::Unknown("L4 Runtime Environment".to_string())
        } else {
            Platform::Unknown("Unknown".to_string())
        }
    }

    #[inline(always)]
    fn arch(self) -> Result<Arch> {
        Ok(if cfg!(target_pointer_width = "64") {
            Arch::Wasm64
        } else if cfg!(target_pointer_width = "32") {
            Arch::Wasm32
        } else {
            return Err(Error::new(
                ErrorKind::Other, // FIXME: WhoAmI 2.0, Unsupported
                "Unexpected pointer width for target platform",
            ));
        })
    }
}
