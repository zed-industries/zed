//! Async networking primitives for TCP/UDP/Unix communication.
//!
//! This crate is an async version of [`std::net`] and [`std::os::unix::net`].
//!
//! # Implementation
//!
//! This crate uses [`async-io`] for async I/O and [`blocking`] for DNS lookups.
//!
//! [`async-io`]: https://docs.rs/async-io
//! [`blocking`]: https://docs.rs/blocking
//!
//! # Examples
//!
//! A simple UDP server that echoes messages back to the sender:
//!
//! ```no_run
//! use async_net::UdpSocket;
//!
//! # futures_lite::future::block_on(async {
//! let socket = UdpSocket::bind("127.0.0.1:8080").await?;
//! let mut buf = vec![0u8; 1024];
//!
//! loop {
//!     let (n, addr) = socket.recv_from(&mut buf).await?;
//!     socket.send_to(&buf[..n], &addr).await?;
//! }
//! # std::io::Result::Ok(()) });
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]

#[cfg(unix)]
pub mod unix;

mod addr;
mod tcp;
mod udp;

pub use addr::AsyncToSocketAddrs;
pub use tcp::{Incoming, TcpListener, TcpStream};
pub use udp::UdpSocket;

use std::io;

#[doc(no_inline)]
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4, SocketAddrV6};

#[doc(no_inline)]
pub use std::net::AddrParseError;

/// Converts or resolves addresses to [`SocketAddr`] values.
///
/// # Examples
///
/// ```
/// # futures_lite::future::block_on(async {
/// for addr in async_net::resolve("google.com:80").await? {
///     println!("{}", addr);
/// }
/// # std::io::Result::Ok(()) });
/// ```
pub async fn resolve<A: AsyncToSocketAddrs>(addr: A) -> io::Result<Vec<SocketAddr>> {
    Ok(addr.to_socket_addrs().await?.collect())
}
