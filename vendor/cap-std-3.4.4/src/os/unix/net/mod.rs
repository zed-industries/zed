//! Unix-specific networking functionality
//!
//! This corresponds to [`std::os::unix::net`].
//!
//! This module is not yet implemented. And it's not easily implementable
//! on many platforms. See [this POSIX discussion] which ultimately didn't
//! succeed in adding support to POSIX.
//!
//! [`std::os::unix::net`]: https://doc.rust-lang.org/std/os/unix/net/
//! [this POSIX discussion]: https://www.austingroupbugs.net/view.php?id=980

mod incoming;
mod unix_datagram;
mod unix_listener;
mod unix_stream;

pub use incoming::*;
pub use unix_datagram::*;
pub use unix_listener::*;
pub use unix_stream::*;

pub use std::os::unix::net::SocketAddr;
