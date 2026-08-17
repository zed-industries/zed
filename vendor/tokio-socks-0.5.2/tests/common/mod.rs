#![allow(dead_code)]
#[cfg(feature = "futures-io")]
pub mod futures_utils;
#[cfg(feature = "tokio")]
pub mod tokio_utils;

#[cfg(feature = "tokio")]
pub use tokio_utils::*;

pub const UNIX_PROXY_ADDR: &str = "/tmp/proxy.s";
pub const PROXY_ADDR: &str = "127.0.0.1:41080";
pub const UNIX_SOCKS4_PROXY_ADDR: &str = "/tmp/socks4_proxy.s";
pub const SOCKS4_PROXY_ADDR: &str = "127.0.0.1:41081";
pub const ECHO_SERVER_ADDR: &str = "localhost:10007";
pub const MSG: &[u8] = b"hello";
