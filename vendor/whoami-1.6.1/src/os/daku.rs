//! This is mostly the same as fake.rs for now

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
        Ok("Emulated".to_string())
    }

    #[inline(always)]
    fn desktop_env(self) -> DesktopEnv {
        DesktopEnv::Unknown("Unknown Daku".to_string())
    }

    #[inline(always)]
    fn platform(self) -> Platform {
        Platform::Unknown("Daku".to_string())
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
