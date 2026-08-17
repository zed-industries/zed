//! HTTP Server
//!
//! A `Server` is created to listen on a port, parse HTTP requests, and hand
//! them off to a `Service`.
//!
//! There are two levels of APIs provide for constructing HTTP servers:
//!
//! - The higher-level [`Server`](Server) type.
//! - The lower-level [`conn`](conn) module.
//!
//! # Server
//!
//! The [`Server`](Server) is main way to start listening for HTTP requests.
//! It wraps a listener with a [`MakeService`](crate::service), and then should
//! be executed to start serving requests.
//!
//! [`Server`](Server) accepts connections in both HTTP1 and HTTP2 by default.
//!
//! ## Examples
//!
//! ```no_run
//! use std::convert::Infallible;
//! use std::net::SocketAddr;
//! use hyper::{Body, Request, Response, Server};
//! use hyper::service::{make_service_fn, service_fn};
//!
//! async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//!     Ok(Response::new(Body::from("Hello World")))
//! }
//!
//! # #[cfg(feature = "runtime")]
//! #[tokio::main]
//! async fn main() {
//!     // Construct our SocketAddr to listen on...
//!     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//!
//!     // And a MakeService to handle each connection...
//!     let make_service = make_service_fn(|_conn| async {
//!         Ok::<_, Infallible>(service_fn(handle))
//!     });
//!
//!     // Then bind and serve...
//!     let server = Server::bind(&addr).serve(make_service);
//!
//!     // And run forever...
//!     if let Err(e) = server.await {
//!         eprintln!("server error: {}", e);
//!     }
//! }
//! # #[cfg(not(feature = "runtime"))]
//! # fn main() {}
//! ```
//!
//! If you don't need the connection and your service implements `Clone` you can use
//! [`tower::make::Shared`] instead of `make_service_fn` which is a bit simpler:
//!
//! ```no_run
//! # use std::convert::Infallible;
//! # use std::net::SocketAddr;
//! # use hyper::{Body, Request, Response, Server};
//! # use hyper::service::{make_service_fn, service_fn};
//! # use tower::make::Shared;
//! # async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//! #     Ok(Response::new(Body::from("Hello World")))
//! # }
//! # #[cfg(feature = "runtime")]
//! #[tokio::main]
//! async fn main() {
//!     // Construct our SocketAddr to listen on...
//!     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//!
//!     // Shared is a MakeService that produces services by cloning an inner service...
//!     let make_service = Shared::new(service_fn(handle));
//!
//!     // Then bind and serve...
//!     let server = Server::bind(&addr).serve(make_service);
//!
//!     // And run forever...
//!     if let Err(e) = server.await {
//!         eprintln!("server error: {}", e);
//!     }
//! }
//! # #[cfg(not(feature = "runtime"))]
//! # fn main() {}
//! ```
//!
//! Passing data to your request handler can be done like so:
//!
//! ```no_run
//! use std::convert::Infallible;
//! use std::net::SocketAddr;
//! use hyper::{Body, Request, Response, Server};
//! use hyper::service::{make_service_fn, service_fn};
//! # #[cfg(feature = "runtime")]
//! use hyper::server::conn::AddrStream;
//!
//! #[derive(Clone)]
//! struct AppContext {
//!     // Whatever data your application needs can go here
//! }
//!
//! async fn handle(
//!     context: AppContext,
//!     addr: SocketAddr,
//!     req: Request<Body>
//! ) -> Result<Response<Body>, Infallible> {
//!     Ok(Response::new(Body::from("Hello World")))
//! }
//!
//! # #[cfg(feature = "runtime")]
//! #[tokio::main]
//! async fn main() {
//!     let context = AppContext {
//!         // ...
//!     };
//!
//!     // A `MakeService` that produces a `Service` to handle each connection.
//!     let make_service = make_service_fn(move |conn: &AddrStream| {
//!         // We have to clone the context to share it with each invocation of
//!         // `make_service`. If your data doesn't implement `Clone` consider using
//!         // an `std::sync::Arc`.
//!         let context = context.clone();
//!
//!         // You can grab the address of the incoming connection like so.
//!         let addr = conn.remote_addr();
//!
//!         // Create a `Service` for responding to the request.
//!         let service = service_fn(move |req| {
//!             handle(context.clone(), addr, req)
//!         });
//!
//!         // Return the service to hyper.
//!         async move { Ok::<_, Infallible>(service) }
//!     });
//!
//!     // Run the server like above...
//!     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//!
//!     let server = Server::bind(&addr).serve(make_service);
//!
//!     if let Err(e) = server.await {
//!         eprintln!("server error: {}", e);
//!     }
//! }
//! # #[cfg(not(feature = "runtime"))]
//! # fn main() {}
//! ```
//!
//! [`tower::make::Shared`]: https://docs.rs/tower/latest/tower/make/struct.Shared.html

pub mod accept;
pub mod conn;
#[cfg(feature = "tcp")]
mod tcp;

pub use self::server::Server;

cfg_feature! {
    #![any(feature = "http1", feature = "http2")]

    #[cfg_attr(feature = "deprecated", allow(deprecated))]
    pub(crate) mod server;
    pub use self::server::Builder;

    mod shutdown;
}

cfg_feature! {
    #![not(any(feature = "http1", feature = "http2"))]

    mod server_stub;
    use server_stub as server;
}
