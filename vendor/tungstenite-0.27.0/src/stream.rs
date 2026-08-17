//! Convenience wrapper for streams to switch between plain TCP and TLS at runtime.
//!
//!  There is no dependency on actual TLS implementations. Everything like
//! `native_tls` or `openssl` will work as long as there is a TLS stream supporting standard
//! `Read + Write` traits.

#[cfg(feature = "__rustls-tls")]
use std::ops::Deref;
use std::{
    fmt::{self, Debug},
    io::{Read, Result as IoResult, Write},
};

use std::net::TcpStream;

#[cfg(feature = "native-tls")]
use native_tls_crate::TlsStream;
#[cfg(feature = "__rustls-tls")]
use rustls::StreamOwned;

/// Stream mode, either plain TCP or TLS.
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    /// Plain mode (`ws://` URL).
    Plain,
    /// TLS mode (`wss://` URL).
    Tls,
}

/// Trait to switch TCP_NODELAY.
pub trait NoDelay {
    /// Set the TCP_NODELAY option to the given value.
    fn set_nodelay(&mut self, nodelay: bool) -> IoResult<()>;
}

impl NoDelay for TcpStream {
    fn set_nodelay(&mut self, nodelay: bool) -> IoResult<()> {
        TcpStream::set_nodelay(self, nodelay)
    }
}

#[cfg(feature = "native-tls")]
impl<S: Read + Write + NoDelay> NoDelay for TlsStream<S> {
    fn set_nodelay(&mut self, nodelay: bool) -> IoResult<()> {
        self.get_mut().set_nodelay(nodelay)
    }
}

#[cfg(feature = "__rustls-tls")]
impl<S, SD, T> NoDelay for StreamOwned<S, T>
where
    S: Deref<Target = rustls::ConnectionCommon<SD>>,
    SD: rustls::SideData,
    T: Read + Write + NoDelay,
{
    fn set_nodelay(&mut self, nodelay: bool) -> IoResult<()> {
        self.sock.set_nodelay(nodelay)
    }
}

/// A stream that might be protected with TLS.
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum MaybeTlsStream<S: Read + Write> {
    /// Unencrypted socket stream.
    Plain(S),
    #[cfg(feature = "native-tls")]
    /// Encrypted socket stream using `native-tls`.
    NativeTls(native_tls_crate::TlsStream<S>),
    #[cfg(feature = "__rustls-tls")]
    /// Encrypted socket stream using `rustls`.
    Rustls(rustls::StreamOwned<rustls::ClientConnection, S>),
}

impl<S: Read + Write + Debug> Debug for MaybeTlsStream<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plain(s) => f.debug_tuple("MaybeTlsStream::Plain").field(s).finish(),
            #[cfg(feature = "native-tls")]
            Self::NativeTls(s) => f.debug_tuple("MaybeTlsStream::NativeTls").field(s).finish(),
            #[cfg(feature = "__rustls-tls")]
            Self::Rustls(s) => {
                struct RustlsStreamDebug<'a, S: Read + Write>(
                    &'a rustls::StreamOwned<rustls::ClientConnection, S>,
                );

                impl<S: Read + Write + Debug> Debug for RustlsStreamDebug<'_, S> {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.debug_struct("StreamOwned")
                            .field("conn", &self.0.conn)
                            .field("sock", &self.0.sock)
                            .finish()
                    }
                }

                f.debug_tuple("MaybeTlsStream::Rustls").field(&RustlsStreamDebug(s)).finish()
            }
        }
    }
}

impl<S: Read + Write> Read for MaybeTlsStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        match *self {
            MaybeTlsStream::Plain(ref mut s) => s.read(buf),
            #[cfg(feature = "native-tls")]
            MaybeTlsStream::NativeTls(ref mut s) => s.read(buf),
            #[cfg(feature = "__rustls-tls")]
            MaybeTlsStream::Rustls(ref mut s) => s.read(buf),
        }
    }
}

impl<S: Read + Write> Write for MaybeTlsStream<S> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        match *self {
            MaybeTlsStream::Plain(ref mut s) => s.write(buf),
            #[cfg(feature = "native-tls")]
            MaybeTlsStream::NativeTls(ref mut s) => s.write(buf),
            #[cfg(feature = "__rustls-tls")]
            MaybeTlsStream::Rustls(ref mut s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> IoResult<()> {
        match *self {
            MaybeTlsStream::Plain(ref mut s) => s.flush(),
            #[cfg(feature = "native-tls")]
            MaybeTlsStream::NativeTls(ref mut s) => s.flush(),
            #[cfg(feature = "__rustls-tls")]
            MaybeTlsStream::Rustls(ref mut s) => s.flush(),
        }
    }
}

impl<S: Read + Write + NoDelay> NoDelay for MaybeTlsStream<S> {
    fn set_nodelay(&mut self, nodelay: bool) -> IoResult<()> {
        match *self {
            MaybeTlsStream::Plain(ref mut s) => s.set_nodelay(nodelay),
            #[cfg(feature = "native-tls")]
            MaybeTlsStream::NativeTls(ref mut s) => s.set_nodelay(nodelay),
            #[cfg(feature = "__rustls-tls")]
            MaybeTlsStream::Rustls(ref mut s) => s.set_nodelay(nodelay),
        }
    }
}
