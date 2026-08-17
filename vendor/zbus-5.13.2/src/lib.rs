#![deny(rust_2018_idioms)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/z-galaxy/zbus/9f7a90d2b594ddc48b7a5f39fda5e00cd56a7dfb/logo.png"
)]
#![doc = include_str!("../README.md")]
#![doc(test(attr(
    warn(unused),
    deny(warnings),
    allow(dead_code),
    // W/o this, we seem to get some bogus warning about `extern crate zbus`.
    allow(unused_extern_crates),
)))]

#[cfg(doctest)]
mod doctests {
    // Repo README.
    doc_comment::doctest!("../../README.md");
    // Book markdown checks
    doc_comment::doctest!("../../book/src/client.md");
    doc_comment::doctest!("../../book/src/concepts.md");
    // The connection chapter contains a p2p example.
    #[cfg(feature = "p2p")]
    doc_comment::doctest!("../../book/src/connection.md");
    doc_comment::doctest!("../../book/src/contributors.md");
    doc_comment::doctest!("../../book/src/introduction.md");
    doc_comment::doctest!("../../book/src/service.md");
    #[cfg(feature = "blocking-api")]
    doc_comment::doctest!("../../book/src/blocking.md");
    doc_comment::doctest!("../../book/src/faq.md");
}

#[cfg(all(not(feature = "async-io"), not(feature = "tokio")))]
mod error_message {
    #[cfg(windows)]
    compile_error!(
        "Either \"async-io\" (default) or \"tokio\" must be enabled. On Windows \"async-io\" is (currently) required for UNIX socket support"
    );

    #[cfg(not(windows))]
    compile_error!("Either \"async-io\" (default) or \"tokio\" must be enabled.");
}

#[cfg(windows)]
mod win32;

mod dbus_error;
pub use dbus_error::*;

mod error;
pub use error::*;

pub mod address;
pub use address::Address;

mod guid;
pub use guid::*;

pub mod message;
pub use message::Message;

pub mod connection;
/// Alias for `connection` module, for convenience.
pub use connection as conn;
pub use connection::Connection;
#[deprecated(
    since = "5.0.0",
    note = "Please use `connection::AuthMechanism` instead"
)]
pub use connection::handshake::AuthMechanism;

mod message_stream;
pub use message_stream::*;
mod abstractions;
pub use abstractions::*;

pub mod match_rule;
pub use match_rule::{MatchRule, OwnedMatchRule};

pub mod proxy;
pub use proxy::Proxy;

pub mod object_server;
pub use object_server::ObjectServer;

mod utils;
pub use utils::*;

#[macro_use]
pub mod fdo;

#[cfg(feature = "blocking-api")]
pub mod blocking;

pub use zbus_macros::{DBusError, interface, proxy};

// Required for the macros to function within this crate.
extern crate self as zbus;

// Macro support module, not part of the public API.
#[doc(hidden)]
pub mod export {
    pub use async_trait;
    pub use futures_core;
    pub use ordered_stream;
    pub use serde;
}

pub use zbus_names as names;
pub use zvariant;
