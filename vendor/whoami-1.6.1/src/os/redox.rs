// We don't need unsafe, yay!
#![forbid(unsafe_code)]
// Redox support has its own MSRV of 1.65, is nightly lint
#![allow(unknown_lints, clippy::incompatible_msrv)]

use std::{borrow::Cow, ffi::OsString, fs, io::Error};

use libredox::{call, error};

use crate::{
    os::{Os, Target},
    Arch, DesktopEnv, Platform, Result,
};

/// Row in the Redox /etc/passwd file
struct Passwd<'a>(Cow<'a, str>);

impl Passwd<'_> {
    fn column(&self, number: usize) -> Option<&str> {
        self.0.split(';').nth(number)
    }

    fn username(&self) -> Option<String> {
        self.column(0).map(ToString::to_string)
    }

    fn uid(&self) -> Option<usize> {
        self.column(1)?.parse().ok()
    }

    fn gid(&self) -> Option<usize> {
        self.column(2)?.parse().ok()
    }

    fn fullname(&self) -> Option<String> {
        self.column(3).map(ToString::to_string)
    }
}

struct Uname<'a>(Cow<'a, str>);

impl Uname<'_> {
    fn row(&self, number: usize) -> Option<&str> {
        self.0.lines().nth(number)
    }

    fn machine_arch(&self) -> Option<Arch> {
        // FIXME: Don't hardcode unknown arch
        Some(Arch::Unknown(self.row(4)?.to_string()))
    }
}

fn to_io_error(error: error::Error) -> Error {
    Error::from_raw_os_error(error.errno())
}

fn euid() -> Result<usize> {
    call::geteuid().map_err(to_io_error)
}

fn egid() -> Result<usize> {
    call::getegid().map_err(to_io_error)
}

fn passwd() -> Result<Passwd<'static>> {
    let (euid, egid) = (euid()?, egid()?);
    let passwd_file = fs::read_to_string("/etc/passwd")?;

    for user in passwd_file.lines() {
        let passwd = Passwd(user.into());

        if passwd.uid() == Some(euid) && passwd.gid() == Some(egid) {
            return Ok(Passwd(passwd.0.into_owned().into()));
        }
    }

    Err(super::err_missing_record())
}

fn uname() -> Result<Uname<'static>> {
    let uname_file = fs::read_to_string("sys:uname")?;

    Ok(Uname(uname_file.into()))
}

fn hostname() -> Result<String> {
    let hostname_file = fs::read_to_string("/etc/hostname")?;

    Ok(hostname_file.lines().next().unwrap_or_default().to_string())
}

impl Target for Os {
    fn langs(self) -> Result<String> {
        super::unix_lang()
    }

    #[inline(always)]
    fn realname(self) -> Result<OsString> {
        Ok(passwd()?.fullname().unwrap_or_default().into())
    }

    #[inline(always)]
    fn username(self) -> Result<OsString> {
        Ok(passwd()?.username().unwrap_or_default().into())
    }

    #[inline(always)]
    fn devicename(self) -> Result<OsString> {
        hostname().map(OsString::from)
    }

    #[inline(always)]
    fn hostname(self) -> Result<String> {
        hostname()
    }

    #[inline(always)]
    fn distro(self) -> Result<String> {
        let release_file = fs::read_to_string("/etc/os-release")?;

        for kv in release_file.lines() {
            if let Some(kv) = kv.strip_prefix("PRETTY_NAME=\"") {
                if let Some(kv) = kv.strip_suffix('\"') {
                    return Ok(kv.to_string());
                }
            }
        }

        Err(super::err_missing_record())
    }

    #[inline(always)]
    fn desktop_env(self) -> DesktopEnv {
        DesktopEnv::Orbital
    }

    #[inline(always)]
    fn platform(self) -> Platform {
        Platform::Redox
    }

    #[inline(always)]
    fn arch(self) -> Result<Arch> {
        uname()?
            .machine_arch()
            .ok_or_else(super::err_missing_record)
    }
}
