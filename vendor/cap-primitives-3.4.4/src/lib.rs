//! Capability-based primitives.

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![allow(stable_features)]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(all(windows, windows_by_handle), feature(windows_by_handle))]
#![cfg_attr(all(windows, windows_file_type_ext), feature(windows_file_type_ext))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.ico"
)]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]
#![cfg_attr(io_error_more, feature(io_error_more))]
#![cfg_attr(io_error_uncategorized, feature(io_error_uncategorized))]

#[cfg(not(windows))]
mod rustix;
#[cfg(windows)]
mod windows;

pub mod fs;
pub mod net;
pub mod time;

#[doc(hidden)]
pub use ambient_authority::ambient_authority_known_at_compile_time;
pub use ambient_authority::{ambient_authority, AmbientAuthority};
// This is part of our public API.
pub use ipnet;
