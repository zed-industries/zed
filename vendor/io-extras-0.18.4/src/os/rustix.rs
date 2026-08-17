//! All Posix-ish platforms have `RawFd` and related traits. Re-export them
//! so that users don't need target-specific code to import them.

#[cfg(unix)]
pub use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(target_os = "wasi")]
pub use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

pub use crate::read_write::{AsRawReadWriteFd, AsReadWriteFd};

// In theory we could do something similar for
// `std::os::fortanix_sgx::io::{AsRawFd, FromRawFd, RawFd}`, however it lacks
// `IntoRawFd`, and `std::fs::File` doesn't implement its `AsRawFd`, so it
// doesn't seem to qualify as Posix-ish.
