//! Raw API bindings to the WebAssembly System Interface (WASI)
//!
//! This crate provides Rust API bindings to WASI APIs. All WASI APIs are
//! exported from this crate and provided with the appropriate type signatures.
//! This crate is entirely procedurally generated from the `*.witx` files that
//! describe the WASI API.
//!
//! # WASI API Version
//!
//! The WASI API is evolving over time. It is both gaining new features as well
//! as tweaking the ABI of existing features. As a result it's important to
//! understand what version of this crate you're using and how it relates to
//! the WASI version of the spec.
//!
//! The WASI specification is organized into phases where there is a snapshot
//! at any one point in time describing the current state of the specification.
//! This crate implements a particular snapshot. You can find the snapshot
//! version implemented in this crate in the build metadata of the crate
//! version number. For example something like `0.9.0+wasi-snapshot-preview1`
//! means that this crate's own personal version is 0.9.0 and it implements the
//! `wasi-snapshot-preview1` snapshot. A major release of this crate (i.e.
//! bumping the "0.9.0") is expected whenever the generated code changes
//! or a new WASI snapshot is used.
//!
//! # Crate Features
//!
//! This crate supports one feature, `std`, which implements the standard
//! `Error` trait for the exported [`Error`] type in this crate. This is
//! enabled by default but can be disabled to make the library `no_std`
//! compatible.

#![no_std]

mod lib_generated;
pub use lib_generated::*;

/// Special `Dircookie` value indicating the start of a directory.
pub const DIRCOOKIE_START: Dircookie = 0;

/// The "standard input" descriptor number.
pub const FD_STDIN: Fd = 0;

/// The "standard output" descriptor number.
pub const FD_STDOUT: Fd = 1;

/// The "standard error" descriptor number.
pub const FD_STDERR: Fd = 2;
