#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(rust_2018_idioms))]
#![cfg_attr(all(test, feature = "full"), deny(unreachable_pub))]
// 0.14.x is not actively developed, new warnings just get in the way.
//#![cfg_attr(all(test, feature = "full", not(feature = "nightly")), deny(warnings))]
#![cfg_attr(all(test, feature = "nightly"), feature(test))]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # hyper
//!
//! hyper is a **fast** and **correct** HTTP implementation written in and for Rust.
//!
//! ## Features
//!
//! - HTTP/1 and HTTP/2
//! - Asynchronous design
//! - Leading in performance
//! - Tested and **correct**
//! - Extensive production use
//! - [Client](client/index.html) and [Server](server/index.html) APIs
//!
//! If just starting out, **check out the [Guides](https://hyper.rs/guides)
//! first.**
//!
//! ## "Low-level"
//!
//! hyper is a lower-level HTTP library, meant to be a building block
//! for libraries and applications.
//!
//! If looking for just a convenient HTTP client, consider the
//! [reqwest](https://crates.io/crates/reqwest) crate.
//!
//! # Optional Features
//!
//! hyper uses a set of [feature flags] to reduce the amount of compiled code.
//! It is possible to just enable certain features over others. By default,
//! hyper does not enable any features but allows one to enable a subset for
//! their use case. Below is a list of the available feature flags. You may
//! also notice above each function, struct and trait there is listed one or
//! more feature flags that are required for that item to be used.
//!
//! If you are new to hyper it is possible to enable the `full` feature flag
//! which will enable all public APIs. Beware though that this will pull in
//! many extra dependencies that you may not need.
//!
//! The following optional features are available:
//!
//! - `http1`: Enables HTTP/1 support.
//! - `http2`: Enables HTTP/2 support.
//! - `client`: Enables the HTTP `client`.
//! - `server`: Enables the HTTP `server`.
//! - `runtime`: Enables convenient integration with `tokio`, providing
//!   connectors and acceptors for TCP, and a default executor.
//! - `tcp`: Enables convenient implementations over TCP (using tokio).
//! - `stream`: Provides `futures::Stream` capabilities.
//! - `backports`: 1.0 functionality backported to 0.14.
//! - `deprecated`: opt-in to deprecation warnings to prepare you for 1.0.
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

#[doc(hidden)]
pub use http;

#[cfg(all(test, feature = "nightly"))]
extern crate test;

pub use crate::http::{header, Method, Request, Response, StatusCode, Uri, Version};

#[doc(no_inline)]
pub use crate::http::HeaderMap;

pub use crate::body::Body;
pub use crate::error::{Error, Result};

#[macro_use]
mod cfg;
#[macro_use]
mod common;
pub mod body;
mod error;
pub mod ext;
#[cfg(test)]
mod mock;
pub mod rt;
pub mod service;
pub mod upgrade;

#[cfg(feature = "ffi")]
pub mod ffi;

cfg_proto! {
    mod headers;
    mod proto;
}

cfg_feature! {
    #![feature = "client"]

    pub mod client;
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[doc(no_inline)]
    pub use crate::client::Client;
}

cfg_feature! {
    #![feature = "server"]

    pub mod server;
    #[doc(no_inline)]
    pub use crate::server::Server;
}
