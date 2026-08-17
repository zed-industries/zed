//! The goal of this library is to stay within the style of the
//! Rust standard library while extending it to support more features.

#![deny(missing_docs)]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(all(unix, unix_socket_peek), feature(unix_socket_peek))]

pub mod fs;
pub mod io;
