//! HTTP Client
//!
//! There are two levels of APIs provided for construct HTTP clients:
//!
//! - The higher-level [`Client`](Client) type.
//! - The lower-level [`conn`](conn) module.
//!
//! # Client
//!
//! The [`Client`](Client) is the main way to send HTTP requests to a server.
//! The default `Client` provides these things on top of the lower-level API:
//!
//! - A default **connector**, able to resolve hostnames and connect to
//!   destinations over plain-text TCP.
//! - A **pool** of existing connections, allowing better performance when
//!   making multiple requests to the same hostname.
//! - Automatic setting of the `Host` header, based on the request `Uri`.
//! - Automatic request **retries** when a pooled connection is closed by the
//!   server before any bytes have been written.
//!
//! Many of these features can configured, by making use of
//! [`Client::builder`](Client::builder).
//!
//! ## Example
//!
//! For a small example program simply fetching a URL, take a look at the
//! [full client example](https://github.com/hyperium/hyper/blob/master/examples/client.rs).
//!
//! ```
//! # #[cfg(all(feature = "tcp", feature = "client", any(feature = "http1", feature = "http2")))]
//! # async fn fetch_httpbin() -> hyper::Result<()> {
//! use hyper::{body::HttpBody as _, Client, Uri};
//!
//! let client = Client::new();
//!
//! // Make a GET /ip to 'http://httpbin.org'
//! let res = client.get(Uri::from_static("http://httpbin.org/ip")).await?;
//!
//! // And then, if the request gets a response...
//! println!("status: {}", res.status());
//!
//! // Concatenate the body stream into a single buffer...
//! let buf = hyper::body::to_bytes(res).await?;
//!
//! println!("body: {:?}", buf);
//! # Ok(())
//! # }
//! # fn main () {}
//! ```

#[cfg(feature = "tcp")]
pub use self::connect::HttpConnector;

pub mod connect;
#[cfg(all(test, feature = "runtime"))]
mod tests;

cfg_feature! {
    #![any(feature = "http1", feature = "http2")]

    pub use self::client::{Builder, Client, ResponseFuture};

    mod client;
    pub mod conn;
    pub(super) mod dispatch;
    mod pool;
    pub mod service;
}
