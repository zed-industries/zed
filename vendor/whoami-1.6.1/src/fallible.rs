//! Fallible versions of the whoami APIs.
//!
//! Some of the functions in the root module will return "Unknown" or
//! "localhost" on error.  This might not be desirable in some situations.  The
//! functions in this module all return a [`Result`].

use std::ffi::OsString;

use crate::{
    conversions,
    os::{Os, Target},
    Result,
};

/// Get the user's account name; usually just the username, but may include an
/// account server hostname.
///
/// If you don't want the account server hostname, use [`username()`].
///
/// Example: `username@example.com`
#[inline(always)]
pub fn account() -> Result<String> {
    account_os().and_then(conversions::string_from_os)
}

/// Get the user's account name; usually just the username, but may include an
/// account server hostname.
///
/// If you don't want the account server hostname, use [`username()`].
///
/// Example: `username@example.com`
#[inline(always)]
pub fn account_os() -> Result<OsString> {
    Target::account(Os)
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname()`] most notably in that spaces
/// are not allowed in the username.
#[inline(always)]
pub fn username() -> Result<String> {
    username_os().and_then(conversions::string_from_os)
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname_os()`] most notably in that
/// spaces are not allowed in the username.
#[inline(always)]
pub fn username_os() -> Result<OsString> {
    Target::username(Os)
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname() -> Result<String> {
    realname_os().and_then(conversions::string_from_os)
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname_os() -> Result<OsString> {
    Target::realname(Os)
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro() -> Result<String> {
    Target::distro(Os)
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename() -> Result<String> {
    devicename_os().and_then(conversions::string_from_os)
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename_os() -> Result<OsString> {
    Target::devicename(Os)
}

/// Get the host device's hostname.
///
/// Usually hostnames are case-insensitive, but it's
/// not a hard requirement.
///
/// FIXME: Document platform-specific character limitations
#[inline(always)]
pub fn hostname() -> Result<String> {
    Target::hostname(Os)
}
