//! Implements the X Input Method (XIM) protocol.
//!
//! XIM is the input method framework used for X11 applications. To clarify, it provides
//! a strategy for users of non-English keyboard to type symbols using only keys that are
//! available on the keyboard. XIM involves two processes. One is the server, which waits
//! for keyboard input in order to compose it into a symbol. The other is the client, which
//! is usually a normal X11 application that waits for and acts on XIM events.
//!
//! This crate provides the following features:
//!
//! - An implementation of an XIM client, via the [`Client`] trait (requires the `client`
//!   feature).
//! - An implementation of an XIM server, via the [`Server`] trait (requires the `server`
//!   feature).
//! - A wrapper around [`x11rb`](x11rb-library), the X rust bindings. See the [`x11rb`] module
//!   for more information (requires the `x11rb-client` or `x11rb-server` feature).
//! - A wrapper around [`x11-dl`](x11dl-library), the standard X11 library. See the [`xlib`]
//!   module for more information (requires the `xlib-client` feature).
//!
//! [x11rb-library]: https://crates.io/crates/x11rb
//! [x11dl-library]: https://crates.io/crates/x11-dl

#![no_std]
#![allow(clippy::uninlined_format_args, clippy::too_many_arguments)]
#![cfg_attr(not(feature = "xlib-client"), forbid(unsafe_code))]
#![forbid(future_incompatible)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "server")]
mod server;

#[cfg(any(feature = "x11rb-server", feature = "x11rb-client"))]
pub mod x11rb;
#[cfg(feature = "xlib-client")]
pub mod xlib;

#[cfg(feature = "client")]
pub use crate::client::{Client, ClientError, ClientHandler};

#[cfg(feature = "server")]
pub const ALL_LOCALES: &str = include_str!("./all_locales.txt");

#[cfg(feature = "server")]
pub use crate::server::{
    InputContext, InputMethod, Server, ServerCore, ServerError, ServerHandler, UserInputContext,
    XimConnection, XimConnections,
};
pub type AHashMap<K, V> = hashbrown::HashMap<K, V, ahash::RandomState>;
pub use xim_parser::*;

#[allow(non_snake_case, dead_code)]
struct Atoms<Atom> {
    XIM_SERVERS: Atom,
    LOCALES: Atom,
    TRANSPORT: Atom,
    XIM_XCONNECT: Atom,
    XIM_PROTOCOL: Atom,
}

impl<Atom> Atoms<Atom> {
    #[allow(unused)]
    pub fn new<E, F>(f: F) -> Result<Self, E>
    where
        F: Fn(&'static str) -> Result<Atom, E>,
    {
        Ok(Self {
            XIM_SERVERS: f("XIM_SERVERS")?,
            LOCALES: f("LOCALES")?,
            TRANSPORT: f("TRANSPORT")?,
            XIM_XCONNECT: f("_XIM_XCONNECT")?,
            XIM_PROTOCOL: f("_XIM_PROTOCOL")?,
        })
    }

    #[allow(unused)]
    pub fn new_null<E, F>(f: F) -> Result<Self, E>
    where
        F: Fn(&'static str) -> Result<Atom, E>,
    {
        Ok(Self {
            XIM_SERVERS: f("XIM_SERVERS\0")?,
            LOCALES: f("LOCALES\0")?,
            TRANSPORT: f("TRANSPORT\0")?,
            XIM_XCONNECT: f("_XIM_XCONNECT\0")?,
            XIM_PROTOCOL: f("_XIM_PROTOCOL\0")?,
        })
    }
}
