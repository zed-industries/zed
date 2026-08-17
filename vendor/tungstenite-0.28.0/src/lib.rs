//! Lightweight, flexible WebSockets for Rust.
#![deny(
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces
)]
// This can be removed when `error::Error::Http`, `handshake::HandshakeError::Interrupted` and
// `handshake::server::ErrorResponse` are boxed.
#![allow(clippy::result_large_err)]

#[cfg(feature = "handshake")]
pub use http;

pub mod buffer;
#[cfg(feature = "handshake")]
pub mod client;
pub mod error;
#[cfg(feature = "handshake")]
pub mod handshake;
pub mod protocol;
#[cfg(feature = "handshake")]
mod server;
pub mod stream;
#[cfg(all(any(feature = "native-tls", feature = "__rustls-tls"), feature = "handshake"))]
mod tls;
pub mod util;

const READ_BUFFER_CHUNK_SIZE: usize = 4096;
type ReadBuffer = buffer::ReadBuffer<READ_BUFFER_CHUNK_SIZE>;

pub use crate::{
    error::{Error, Result},
    protocol::{frame::Utf8Bytes, Message, WebSocket},
};
// re-export bytes since used in `Message` API.
pub use bytes::Bytes;

#[cfg(feature = "handshake")]
pub use crate::{
    client::{client, connect, ClientRequestBuilder},
    handshake::{client::ClientHandshake, server::ServerHandshake, HandshakeError},
    server::{accept, accept_hdr, accept_hdr_with_config, accept_with_config},
};

#[cfg(all(any(feature = "native-tls", feature = "__rustls-tls"), feature = "handshake"))]
pub use tls::{client_tls, client_tls_with_config, Connector};
