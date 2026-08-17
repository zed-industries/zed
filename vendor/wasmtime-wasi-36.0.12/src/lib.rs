#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Wasmtime's WASI Implementation
//!
//! This crate provides a Wasmtime host implementations of different versions of WASI.
//! WASI is implemented with the Rust crates [`tokio`] and [`cap-std`](cap_std) primarily, meaning that
//! operations are implemented in terms of their native platform equivalents by
//! default.
//!
//! For components and WASIp2, see [`p2`].
//! For WASIp1 and core modules, see the [`preview1`] module documentation.
//!
//! For WASIp3, see [`p3`]. WASIp3 support is experimental, unstable and incomplete.

/// The maximum size, in bytes, that this crate will allocate on the host on a
/// per-read basis.
///
/// This is used to limit the size of `wasi:io/streams.input-stream#read`, for
/// example, along with a variety of other read-style APIs. All of these APIs
/// have the shape where the guest asks the host to read some data for it, and
/// then it's returned. The data is allocated on the host, however, meaning that
/// when the guest asks for an extremely large read that could cause a very
/// large allocations on the host. For now this constant serves as a hard limit
/// on the size of the allocation a host will make.
///
/// TODO: make this configurable per-I/O? Per-WASI? It's not great that this
/// just a hard and unconfigurable limit. There are probably situations where
/// performance will be improved if this limit were higher or removed. Doing so
/// without exposing hosts to guest-controlled resource exhaustion is the
/// important part, though.
const MAX_READ_SIZE_ALLOC: usize = 64 * 1024;

pub mod cli;
pub mod clocks;
mod ctx;
mod error;
mod filesystem;
pub mod p2;
#[cfg(feature = "p3")]
pub mod p3;
#[cfg(feature = "preview1")]
pub mod preview0;
#[cfg(feature = "preview1")]
pub mod preview1;
pub mod random;
pub mod runtime;
pub mod sockets;
mod view;

pub use self::clocks::{HostMonotonicClock, HostWallClock};
pub use self::ctx::{WasiCtx, WasiCtxBuilder};
pub use self::error::{I32Exit, TrappableError};
pub use self::filesystem::{DirPerms, FilePerms, OpenMode};
pub use self::random::{Deterministic, thread_rng};
pub use self::sockets::{AllowedNetworkUses, SocketAddrUse};
pub use self::view::{WasiCtxView, WasiView};
#[doc(no_inline)]
pub use async_trait::async_trait;
#[doc(no_inline)]
pub use cap_fs_ext::SystemTimeSpec;
#[doc(no_inline)]
pub use cap_rand::RngCore;
#[doc(no_inline)]
pub use wasmtime::component::{ResourceTable, ResourceTableError};
