#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Unexpected pointer width for target platform");

use std::{env, ffi::OsString};

use crate::{
    os::{Os, Target},
    Arch, DesktopEnv, Platform, Result,
};

impl Target for Os {
    fn langs(self) -> Result<String> {
        super::unix_lang()
    }

    #[inline(always)]
    fn realname(self) -> Result<OsString> {
        Ok(wasite::user()
            .unwrap_or_else(|_e| "Anonymous".to_string())
            .into())
    }

    #[inline(always)]
    fn username(self) -> Result<OsString> {
        Ok(wasite::user()
            .unwrap_or_else(|_e| "anonymous".to_string())
            .into())
    }

    #[inline(always)]
    fn devicename(self) -> Result<OsString> {
        Ok(wasite::name()
            .unwrap_or_else(|_e| "Unknown".to_string())
            .into())
    }

    #[inline(always)]
    fn hostname(self) -> Result<String> {
        Ok(wasite::hostname().unwrap_or_else(|_e| "localhost".to_string()))
    }

    #[inline(always)]
    fn distro(self) -> Result<String> {
        Ok("Unknown WASI".to_string())
    }

    #[inline(always)]
    fn desktop_env(self) -> DesktopEnv {
        if let Some(ref env) = env::var_os("DESKTOP_SESSION") {
            DesktopEnv::Unknown(env.to_string_lossy().to_string())
        } else {
            DesktopEnv::Unknown("Unknown WASI".to_string())
        }
    }

    #[inline(always)]
    fn platform(self) -> Platform {
        Platform::Unknown("WASI".to_string())
    }

    #[inline(always)]
    fn arch(self) -> Result<Arch> {
        Ok(if cfg!(target_pointer_width = "64") {
            Arch::Wasm64
        } else {
            Arch::Wasm32
        })
    }
}
