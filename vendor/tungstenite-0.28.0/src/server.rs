//! Methods to accept an incoming WebSocket connection on a server.

pub use crate::handshake::server::ServerHandshake;

use crate::handshake::{
    server::{Callback, NoCallback},
    HandshakeError,
};

use crate::protocol::{WebSocket, WebSocketConfig};

use std::io::{Read, Write};

/// Accept the given Stream as a WebSocket.
///
/// Uses a configuration provided as an argument. Calling it with `None` will use the default one
/// used by `accept()`.
///
/// This function starts a server WebSocket handshake over the given stream.
/// If you want TLS support, use `native_tls::TlsStream`, `rustls::Stream` or
/// `openssl::ssl::SslStream` for the stream here. Any `Read + Write` streams are supported,
/// including those from `Mio` and others.
pub fn accept_with_config<S: Read + Write>(
    stream: S,
    config: Option<WebSocketConfig>,
) -> Result<WebSocket<S>, HandshakeError<ServerHandshake<S, NoCallback>>> {
    accept_hdr_with_config(stream, NoCallback, config)
}

/// Accept the given Stream as a WebSocket.
///
/// This function starts a server WebSocket handshake over the given stream.
/// If you want TLS support, use `native_tls::TlsStream`, `rustls::Stream` or
/// `openssl::ssl::SslStream` for the stream here. Any `Read + Write` streams are supported,
/// including those from `Mio` and others.
pub fn accept<S: Read + Write>(
    stream: S,
) -> Result<WebSocket<S>, HandshakeError<ServerHandshake<S, NoCallback>>> {
    accept_with_config(stream, None)
}

/// Accept the given Stream as a WebSocket.
///
/// Uses a configuration provided as an argument. Calling it with `None` will use the default one
/// used by `accept_hdr()`.
///
/// This function does the same as `accept()` but accepts an extra callback
/// for header processing. The callback receives headers of the incoming
/// requests and is able to add extra headers to the reply.
pub fn accept_hdr_with_config<S: Read + Write, C: Callback>(
    stream: S,
    callback: C,
    config: Option<WebSocketConfig>,
) -> Result<WebSocket<S>, HandshakeError<ServerHandshake<S, C>>> {
    ServerHandshake::start(stream, callback, config).handshake()
}

/// Accept the given Stream as a WebSocket.
///
/// This function does the same as `accept()` but accepts an extra callback
/// for header processing. The callback receives headers of the incoming
/// requests and is able to add extra headers to the reply.
pub fn accept_hdr<S: Read + Write, C: Callback>(
    stream: S,
    callback: C,
) -> Result<WebSocket<S>, HandshakeError<ServerHandshake<S, C>>> {
    accept_hdr_with_config(stream, callback, None)
}
