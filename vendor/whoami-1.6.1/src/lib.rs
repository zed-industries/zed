//! Crate for getting the user's username, realname and environment.
//!
//! ## Getting Started
//! Using the whoami crate is super easy!  All of the public items are simple
//! functions with no parameters that return [`String`]s or [`OsString`]s (with
//! the exception of [`desktop_env()`], [`platform()`], and [`arch()`], which
//! return enums, and [`langs()`] that returns an iterator of [`String`]s).  The
//! following example shows how to use all of the functions (except those that
//! return [`OsString`]):
//!
//! ```rust
//! println!(
//!     "User's Name            whoami::realname():    {}",
//!     whoami::realname(),
//! );
//! println!(
//!     "User's Username        whoami::username():    {}",
//!     whoami::username(),
//! );
//! println!(
//!     "User's Language        whoami::lang():        {:?}",
//!     whoami::lang().collect::<Vec<String>>(),
//! );
//! println!(
//!     "Device's Pretty Name   whoami::devicename():  {}",
//!     whoami::devicename(),
//! );
//! println!(
//!     "Device's Hostname      whoami::hostname():    {}",
//!     whoami::hostname(),
//! );
//! println!(
//!     "Device's Platform      whoami::platform():    {}",
//!     whoami::platform(),
//! );
//! println!(
//!     "Device's OS Distro     whoami::distro():      {}",
//!     whoami::distro(),
//! );
//! println!(
//!     "Device's Desktop Env.  whoami::desktop_env(): {}",
//!     whoami::desktop_env(),
//! );
//! println!(
//!     "Device's CPU Arch      whoami::arch():        {}",
//!     whoami::arch(),
//! );
//! ```
//!
//! [`OsString`]: std::ffi::OsString

#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences,
    unsafe_code
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ardaku/whoami/v1/res/icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/ardaku/whoami/v1/res/icon.svg"
)]

mod api;
mod arch;
mod conversions;
mod desktop_env;
pub mod fallible;
mod language;
mod os;
mod platform;
mod result;

#[allow(deprecated)]
pub use self::{
    api::{
        arch, desktop_env, devicename, devicename_os, distro, distro_os,
        hostname, hostname_os, lang, langs, platform, realname, realname_os,
        username, username_os,
    },
    arch::{Arch, Width},
    desktop_env::DesktopEnv,
    language::{Country, Language},
    platform::Platform,
    result::Result,
};
