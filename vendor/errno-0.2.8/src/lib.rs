//! Cross-platform interface to the `errno` variable.
//!
//! # Examples
//! ```
//! use errno::{Errno, errno, set_errno};
//!
//! // Get the current value of errno
//! let e = errno();
//!
//! // Set the current value of errno
//! set_errno(e);
//!
//! // Extract the error code as an i32
//! let code = e.0;
//!
//! // Display a human-friendly error message
//! println!("Error {}: {}", code, e);
//! ```

#![cfg_attr(target_os = "wasi", feature(thread_local))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(unix)] extern crate libc;
#[cfg(windows)] extern crate winapi;
#[cfg(target_os = "dragonfly")] extern crate errno_dragonfly;
#[cfg(target_os = "wasi")] extern crate libc;
#[cfg(target_os = "hermit")] extern crate libc;

#[cfg_attr(unix, path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "wasi", path = "wasi.rs")]
#[cfg_attr(target_os = "hermit", path = "hermit.rs")]
mod sys;

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
use std::io;
#[cfg(feature = "std")]
use std::error::Error;

/// Wraps a platform-specific error code.
///
/// The `Display` instance maps the code to a human-readable string. It
/// calls [`strerror_r`][1] under POSIX, and [`FormatMessageW`][2] on
/// Windows.
///
/// [1]: http://pubs.opengroup.org/onlinepubs/009695399/functions/strerror.html
/// [2]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms679351%28v=vs.85%29.aspx
#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Errno(pub i32);

#[cfg(feature = "std")]
impl fmt::Debug for Errno {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        sys::with_description(*self, |desc| {
            fmt.debug_struct("Errno")
                .field("code", &self.0)
                .field("description", &desc.ok())
                .finish()
        })
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Errno {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        sys::with_description(*self, |desc| match desc {
            Ok(desc) => fmt.write_str(&desc),
            Err(fm_err) => write!(
                fmt, "OS error {} ({} returned error {})",
                self.0, sys::STRERROR_NAME, fm_err.0),
        })
    }
}

impl Into<i32> for Errno {
    fn into(self) -> i32 {
        self.0
    }
}

#[cfg(feature = "std")]
impl Error for Errno {
    // TODO: Remove when MSRV >= 1.27
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "system error"
    }
}

#[cfg(feature = "std")]
impl From<Errno> for io::Error {
    fn from(errno: Errno) -> Self {
        io::Error::from_raw_os_error(errno.0)
    }
}

/// Returns the platform-specific value of `errno`.
pub fn errno() -> Errno {
    sys::errno()
}

/// Sets the platform-specific value of `errno`.
pub fn set_errno(err: Errno) {
    sys::set_errno(err)
}

#[test]
fn it_works() {
    let x = errno();
    set_errno(x);
}

#[cfg(feature = "std")]
#[test]
fn it_works_with_to_string() {
    let x = errno();
    let _ = x.to_string();
}

#[cfg(feature = "std")]
#[test]
fn check_description() {
    let expect = if cfg!(windows) {
        "Incorrect function."
    } else if cfg!(target_os = "illumos") {
        "Not owner"
    } else if cfg!(target_os = "wasi") {
        "Argument list too long"
    } else {
        "Operation not permitted"
    };

    set_errno(Errno(1));

    assert_eq!(errno().to_string(), expect);
    assert_eq!(
        format!("{:?}", errno()),
        format!("Errno {{ code: 1, description: Some({:?}) }}", expect));
}

#[cfg(feature = "std")]
#[test]
fn check_error_into_errno() {
    const ERROR_CODE: i32 = 1;

    let error = io::Error::from_raw_os_error(ERROR_CODE);
    let new_error: io::Error = Errno(ERROR_CODE).into();
    assert_eq!(error.kind(), new_error.kind());
}
