//! Asynchronous Services
//!
//! A [`Service`](Service) is a trait representing an asynchronous
//! function of a request to a response. It's similar to
//! `async fn(Request) -> Result<Response, Error>`.
//!
//! The argument and return value isn't strictly required to be for HTTP.
//! Therefore, hyper uses several "trait aliases" to reduce clutter around
//! bounds. These are:
//!
//! - `HttpService`: This is blanketly implemented for all types that
//!   implement `Service<http::Request<B1>, Response = http::Response<B2>>`.
//! - `MakeService`: When a `Service` returns a new `Service` as its "response",
//!   we consider it a `MakeService`. Again, blanketly implemented in those cases.
//! - `MakeConnection`: A `Service` that returns a "connection", a type that
//!   implements `AsyncRead` and `AsyncWrite`.
//!
//! # HttpService
//!
//! In hyper, especially in the server setting, a `Service` is usually bound
//! to a single connection. It defines how to respond to **all** requests that
//! connection will receive.
//!
//! The helper [`service_fn`](service_fn) should be sufficient for most cases, but
//! if you need to implement `Service` for a type manually, you can follow the example
//! in `service_struct_impl.rs`.
//!
//! # MakeService
//!
//! Since a `Service` is bound to a single connection, a [`Server`](crate::Server)
//! needs a way to make them as it accepts connections. This is what a
//! `MakeService` does.
//!
//! Resources that need to be shared by all `Service`s can be put into a
//! `MakeService`, and then passed to individual `Service`s when `call`
//! is called.

pub use tower_service::Service;

mod http;
mod make;
#[cfg(all(any(feature = "http1", feature = "http2"), feature = "client"))]
mod oneshot;
mod util;

pub(super) use self::http::HttpService;
#[cfg(all(any(feature = "http1", feature = "http2"), feature = "client"))]
pub(super) use self::make::MakeConnection;
#[cfg(all(any(feature = "http1", feature = "http2"), feature = "server"))]
pub(super) use self::make::MakeServiceRef;
#[cfg(all(any(feature = "http1", feature = "http2"), feature = "client"))]
pub(super) use self::oneshot::{oneshot, Oneshot};

pub use self::make::make_service_fn;
pub use self::util::service_fn;
