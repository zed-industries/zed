/* Copyright (c) 2019-2023 Torbjørn Birch Moltu, 2020 Jon Magnuson, 2022 Jake Shadle, 2022 David Carlier, 2023 Dominik Maier
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */

//! A unix domain sockets library that supports abstract addresses, fd-passing,
//! SOCK_SEQPACKET sockets and more.
//!
//! File-descriptor passing and abstract socket support
//! for stream and datagram sockets is provided via extension traits for
//! existing types in `std::os::unix::net` and from [mio](https://github.com/tokio-rs/mio)
//! (the latter is opt-in and must be enabled with `features=["mio_08"]` in Cargo.toml).
//!
//! See README for status of operating system support and other general info.

#![cfg(unix)] // compile as empty crate on windows

// Too many features unavailable on solarish to bother cfg()ing individually.
#![cfg_attr(any(target_os="illumos", target_os="solaris"), allow(unused))]

#![allow(
    clippy::cast_lossless, // improves portability when values are limited by the OS anyway
    clippy::unnecessary_cast, // not necessarily unnecessary on other OSes
    clippy::len_zero, // the `!` in `if !foo.is_empty()` can be easy to miss
    clippy::needless_return, // consistency with early returns, and to not look incomplete
    clippy::redundant_closure, // avoiding auto-functions of tuple structs and enum variants
    clippy::needless_lifetimes, // explicity when useful
    clippy::needless_borrow, // dereferencing one field from a raw pointer
    clippy::bool_to_int_with_if, // clearer than usize::from()
    // more lints are disabled inside ancillary.rs and credentials.rs
)]

extern crate libc;
#[cfg(feature="mio_08")]
extern crate mio_08;
#[cfg(feature="tokio")]
extern crate tokio as tokio_crate;

/// Get errno as io::Error on -1.
macro_rules! cvt {($syscall:expr) => {
    match $syscall {
        -1 => Err(io::Error::last_os_error()),
        ok => Ok(ok),
    }
}}

/// Get errno as io::Error on -1 and retry on EINTR.
macro_rules! cvt_r {($syscall:expr) => {
    loop {
        let result = $syscall;
        if result != -1 {
            break Ok(result);
        }
        let err = io::Error::last_os_error();
        if err.kind() != ErrorKind::Interrupted {
            break Err(err);
        }
    }
}}

mod addr;
mod credentials;
mod helpers;
mod ancillary;
mod traits;
mod seqpacket;
#[cfg(feature="tokio")]
pub mod tokio;

pub use addr::{UnixSocketAddr, UnixSocketAddrRef, AddrName};
pub use traits::{UnixListenerExt, UnixStreamExt, UnixDatagramExt};
pub use seqpacket::{UnixSeqpacketListener, UnixSeqpacketConn};
pub use credentials::ConnCredentials;

pub mod nonblocking {
    pub use crate::seqpacket::NonblockingUnixSeqpacketListener as UnixSeqpacketListener;
    pub use crate::seqpacket::NonblockingUnixSeqpacketConn as UnixSeqpacketConn;
}

#[cfg(debug_assertions)]
mod doctest_md_files {
    macro_rules! mdfile {($content:expr, $(#[$meta:meta])* $attach_to:ident) => {
        #[doc=$content]
        #[allow(unused)]
        $(#[$meta])* // can't #[cfg_attr(, doc=)] in .md file
        enum $attach_to {}
    }}
    mdfile!{
        include_str!("../README.md"),
        #[cfg(any(target_os="linux", target_os="android"))] // uses abstract addrs
        Readme
    }
}
