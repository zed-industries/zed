//! Implementation of `errno` functionality for Unix systems.
//!
//! Adapted from `src/libstd/sys/unix/os.rs` in the Rust distribution.

// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(feature = "std")]
use std::ffi::CStr;
use libc::c_int;
#[cfg(feature = "std")]
use libc::{self, c_char};
#[cfg(target_os = "dragonfly")]
use errno_dragonfly::errno_location;

use Errno;

#[cfg(feature = "std")]
pub fn with_description<F, T>(err: Errno, callback: F) -> T where
    F: FnOnce(Result<&str, Errno>) -> T
{
    let mut buf = [0 as c_char; 1024];
    unsafe {
        if strerror_r(err.0, buf.as_mut_ptr(), buf.len() as libc::size_t) < 0 {
            let fm_err = errno();
            if fm_err != Errno(libc::ERANGE) {
                return callback(Err(fm_err));
            }
        }
    }
    let c_str = unsafe { CStr::from_ptr(buf.as_ptr()) };
    callback(Ok(&String::from_utf8_lossy(c_str.to_bytes())))
}

#[cfg(feature = "std")]
pub const STRERROR_NAME: &'static str = "strerror_r";

pub fn errno() -> Errno {
    unsafe {
        Errno(*errno_location())
    }
}

pub fn set_errno(Errno(errno): Errno) {
    unsafe {
        *errno_location() = errno;
    }
}

extern {
    #[cfg(not(target_os = "dragonfly"))]
    #[cfg_attr(any(target_os = "macos",
                   target_os = "ios",
                   target_os = "freebsd"),
               link_name = "__error")]
    #[cfg_attr(any(target_os = "openbsd",
                   target_os = "netbsd",
                   target_os = "bitrig",
                   target_os = "android"),
               link_name = "__errno")]
    #[cfg_attr(any(target_os = "solaris",
                   target_os = "illumos"),
               link_name = "___errno")]
    #[cfg_attr(target_os = "linux",
               link_name = "__errno_location")]
    fn errno_location() -> *mut c_int;

    #[cfg(feature = "std")]
    #[cfg_attr(target_os = "linux", link_name = "__xpg_strerror_r")]
    fn strerror_r(errnum: c_int, buf: *mut c_char,
                  buflen: libc::size_t) -> c_int;
}
