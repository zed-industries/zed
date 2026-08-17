//! Networking primitives for TCP/UDP communication.
//!
//! This module provides networking functionality for the Transmission Control and User
//! Datagram Protocols, as well as types for IP and socket addresses.
//!
//! This module is an async version of [`std::net`].
//!
//! # Organization
//!
//! * [`TcpListener`] and [`TcpStream`] provide functionality for communication over TCP
//! * [`UdpSocket`] provides functionality for communication over UDP
//! * [`IpAddr`] represents IP addresses of either IPv4 or IPv6; [`Ipv4Addr`] and
//!   [`Ipv6Addr`] are respectively IPv4 and IPv6 addresses
//! * [`SocketAddr`] represents socket addresses of either IPv4 or IPv6; [`SocketAddrV4`]
//!   and [`SocketAddrV6`] are respectively IPv4 and IPv6 socket addresses
//! * [`ToSocketAddrs`] is a trait that used for generic address resolution when interacting
//!   with networking objects like [`TcpListener`], [`TcpStream`] or [`UdpSocket`]
//! * Other types are return or parameter types for various methods in this module
//!
//! [`IpAddr`]: enum.IpAddr.html
//! [`Ipv4Addr`]: struct.Ipv4Addr.html
//! [`Ipv6Addr`]: struct.Ipv6Addr.html
//! [`SocketAddr`]: enum.SocketAddr.html
//! [`SocketAddrV4`]: struct.SocketAddrV4.html
//! [`SocketAddrV6`]: struct.SocketAddrV6.html
//! [`TcpListener`]: struct.TcpListener.html
//! [`TcpStream`]: struct.TcpStream.html
//! [`ToSocketAddrs`]: trait.ToSocketAddrs.html
//! [`UdpSocket`]: struct.UdpSocket.html
//!
//! # Platform-specific extensions
//!
//! APIs such as Unix domain sockets are available on certain platforms only. You can find
//! platform-specific extensions in the [`async_std::os`] module.
//!
//! [`async_std::os`]: ../os/index.html
//! [`std::net`]: https://doc.rust-lang.org/std/net/index.html
//!
//! # Examples
//!
//! A simple UDP echo server:
//!
//! ```no_run
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! use async_std::net::UdpSocket;
//!
//! let socket = UdpSocket::bind("127.0.0.1:8080").await?;
//! let mut buf = vec![0u8; 1024];
//!
//! loop {
//!     let (n, peer) = socket.recv_from(&mut buf).await?;
//!     socket.send_to(&buf[..n], &peer).await?;
//! }
//! #
//! # }) }
//! ```

pub use std::net::AddrParseError;
pub use std::net::Shutdown;
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
pub use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

#[cfg(not(target_os = "unknown"))]
pub use addr::ToSocketAddrs;
#[cfg(not(target_os = "unknown"))]
pub use tcp::{Incoming, TcpListener, TcpStream};
#[cfg(not(target_os = "unknown"))]
pub use udp::UdpSocket;

#[cfg(not(target_os = "unknown"))]
mod addr;
#[cfg(not(target_os = "unknown"))]
mod tcp;
#[cfg(not(target_os = "unknown"))]
mod udp;
