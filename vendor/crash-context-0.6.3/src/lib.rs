//! This crate exposes a platform specific [`CrashContext`] which contains the
//! details for a crash (signal, exception, etc). This crate is fairly minimal
//! since the intended use case is to more easily share these crash details
//! between different crates without requiring lots of dependencies, and is
//! currently only really made for the purposes of the crates in this repo, and
//! [minidump-writer](https://github.com/rust-minidump/minidump-writer).
//!
//! ## Linux/Android
//!
//! This crate also contains a portable implementation of [`getcontext`](
//! https://man7.org/linux/man-pages/man3/getcontext.3.html), as not all libc
//! implementations (notably `musl`) implement it as it has been deprecated from
//! POSIX.
//!
//! ## Macos
//!
//! One major difference on Macos is that the details in the [`CrashContext`]
//! cannot be transferred to another process via normal methods (eg. sockets)
//! and must be sent via the criminally undocumented mach ports. This crate
//! provides a `Client` and `Server` that can be used to send and receive a
//! [`CrashContext`] across processes so that you don't have to suffer like I
//! did.

// crate-specific exceptions:
#![allow(unsafe_code, nonstandard_style)]

cfg_if::cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod linux;
        pub use linux::*;
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        pub use windows::*;
    } else if #[cfg(target_os = "macos")] {
        mod mac;
        pub use mac::*;
    }
}
