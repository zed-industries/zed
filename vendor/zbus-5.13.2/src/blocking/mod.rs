//! The blocking API.
//!
//! This module hosts all our blocking API. All the types under this module are thin wrappers
//! around the corresponding asynchronous types. Most of the method calls are simply calling their
//! asynchronous counterparts on the underlying types and use [`async_io::block_on`] (or
//! [`tokio::runtime::Runtime::block_on`]) to turn them into blocking calls.
//!
//! This module is only available when the `blocking-api` feature is enabled (default).
//!
//! # Caveats
//!
//! Since methods provided by these types run their own little runtime (`block_on`), you must not
//! use them in async contexts because of the infamous [async sandwich footgun][asf]. This is
//! an especially important fact to keep in mind for [`crate::interface`]. While `interface` allows
//! non-async methods for convenience, these methods are called from an async context. The
//! [`blocking` crate] provides an easy way around this problem though.
//!
//! [asf]: https://rust-lang.github.io/wg-async/vision/shiny_future/users_manual.html#caveat-beware-the-async-sandwich
//! [`blocking` crate]: https://docs.rs/blocking/

pub mod connection;
pub use connection::Connection;

mod message_iterator;
pub use message_iterator::*;
pub mod object_server;
pub use object_server::ObjectServer;
pub mod proxy;
pub use proxy::Proxy;

pub mod fdo;
