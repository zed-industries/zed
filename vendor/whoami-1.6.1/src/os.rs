#![allow(unsafe_code)]

// Daku
#[cfg_attr(all(target_arch = "wasm32", daku), path = "os/daku.rs")]
// Redox
#[cfg_attr(
    all(target_os = "redox", not(target_arch = "wasm32")),
    path = "os/redox.rs"
)]
// Unix
#[cfg_attr(
    all(
        any(
            target_os = "linux",
            target_os = "macos",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "illumos",
            target_os = "hurd",
        ),
        not(target_arch = "wasm32")
    ),
    path = "os/unix.rs"
)]
// Wasi
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "wasi"),
    path = "os/wasi.rs"
)]
// Web
#[cfg_attr(
    all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(daku),
        feature = "web",
    ),
    path = "os/web.rs"
)]
// Windows
#[cfg_attr(
    all(target_os = "windows", not(target_arch = "wasm32")),
    path = "os/windows.rs"
)]
mod target;

use std::{
    env::{self, VarError},
    ffi::OsString,
    io::{Error, ErrorKind},
};

use crate::{Arch, DesktopEnv, Platform, Result};

/// Implement `Target for Os` to add platform support for a target.
pub(crate) struct Os;

/// Target platform support
pub(crate) trait Target: Sized {
    /// Return a semicolon-delimited string of language/COUNTRY codes.
    fn langs(self) -> Result<String>;
    /// Return the user's "real" / "full" name.
    fn realname(self) -> Result<OsString>;
    /// Return the user's username.
    fn username(self) -> Result<OsString>;
    /// Return the computer's "fancy" / "pretty" name.
    fn devicename(self) -> Result<OsString>;
    /// Return the computer's hostname.
    fn hostname(self) -> Result<String>;
    /// Return the OS distribution's name.
    fn distro(self) -> Result<String>;
    /// Return the desktop environment.
    fn desktop_env(self) -> DesktopEnv;
    /// Return the target platform.
    fn platform(self) -> Platform;
    /// Return the computer's CPU architecture.
    fn arch(self) -> Result<Arch>;

    /// Return the user's account name (usually just the username, but may
    /// include an account server hostname).
    fn account(self) -> Result<OsString> {
        self.username()
    }
}

// This is only used on some platforms
#[allow(dead_code)]
fn err_missing_record() -> Error {
    Error::new(ErrorKind::NotFound, "Missing record")
}

// This is only used on some platforms
#[allow(dead_code)]
fn err_null_record() -> Error {
    Error::new(ErrorKind::NotFound, "Null record")
}

// This is only used on some platforms
#[allow(dead_code)]
fn err_empty_record() -> Error {
    Error::new(ErrorKind::NotFound, "Empty record")
}

// This is only used on some platforms
#[allow(dead_code)]
fn unix_lang() -> Result<String> {
    let check_var = |var| {
        env::var(var).map_err(|e| {
            let kind = match e {
                VarError::NotPresent => ErrorKind::NotFound,
                VarError::NotUnicode(_) => ErrorKind::InvalidData,
            };
            Error::new(kind, e)
        })
    };

    // Uses priority defined in
    // <https://www.gnu.org/software/gettext/manual/html_node/Locale-Environment-Variables.html>
    let locale = check_var("LC_ALL").or_else(|_| check_var("LANG"));

    // The LANGUAGE environment variable takes precedence if and only if
    // localization is enabled, i.e., LC_ALL / LANG is not "C".
    // <https://www.gnu.org/software/gettext/manual/html_node/The-LANGUAGE-variable.html>
    let langs = match &locale {
        Ok(loc) if loc != "C" => check_var("LANGUAGE").or(locale),
        _ => locale,
    }?;

    if langs.is_empty() {
        return Err(err_empty_record());
    }

    Ok(langs)
}
