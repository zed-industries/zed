// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An implementation of the Rust channel API over process boundaries. Under the
//! hood, this API uses Mach ports on Mac and file descriptor passing over Unix
//! sockets on Linux. The serde library is used to serialize values for transport
//! over the wire.
//!
//! # Features
//! ## `force-inprocess`
//!
//! Force the `inprocess` backend to be used instead of the OS specific backend.
//! The `inprocess` backend is a dummy back-end, that behaves like the real ones,
//! but doesn't actually work between processes.
//!
//! ## `memfd`
//!
//! Use [memfd_create] to back [OsIpcSharedMemory] on Linux. [memfd_create] was
//! introduced in kernel version 3.17. __WARNING:__ Enabling this feature with kernel
//! version less than 3.17 will cause panics on any use of [IpcSharedMemory].
//!
//! ## `unstable`
//!
//! [IpcReceiver]: ipc/struct.IpcReceiver.html
//! [IpcSender]: ipc/struct.IpcSender.html
//! [IpcReceiverSet]: ipc/struct.IpcReceiverSet.html
//! [IpcSharedMemory]: ipc/struct.IpcSharedMemory.html
//! [OsIpcSharedMemory]: platform/struct.OsIpcSharedMemory.html
//! [memfd_create]: http://man7.org/linux/man-pages/man2/memfd_create.2.html

#[cfg(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
))]
#[cfg(all(
    feature = "memfd",
    not(feature = "force-inprocess"),
    target_os = "linux"
))]
#[cfg(feature = "async")]
use futures;

#[cfg(feature = "async")]
pub mod asynch;

#[cfg(all(not(feature = "force-inprocess"), target_os = "windows"))]
extern crate windows;

pub mod ipc;
pub mod platform;
pub mod router;

#[cfg(test)]
mod test;

pub use bincode::{Error, ErrorKind};
