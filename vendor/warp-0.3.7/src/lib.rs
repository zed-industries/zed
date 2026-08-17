#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]
#![cfg_attr(test, deny(warnings))]

//! # warp
//!
//! warp is a super-easy, composable, web server framework for warp speeds.
//!
//! Thanks to its [`Filter`][Filter] system, warp provides these out of the box:
//!
//! - Path routing and parameter extraction
//! - Header requirements and extraction
//! - Query string deserialization
//! - JSON and Form bodies
//! - Multipart form data
//! - Static Files and Directories
//! - Websockets
//! - Access logging
//! - Etc
//!
//! Since it builds on top of [hyper](https://hyper.rs), you automatically get:
//!
//! - HTTP/1
//! - HTTP/2
//! - Asynchronous
//! - One of the fastest HTTP implementations
//! - Tested and **correct**
//!
//! ## Filters
//!
//! The main concept in warp is the [`Filter`][Filter], which allows composition
//! to describe various endpoints in your web service. Besides this powerful
//! trait, warp comes with several built in [filters](filters/index.html), which
//! can be combined for your specific needs.
//!
//! As a small example, consider an endpoint that has path and header requirements:
//!
//! ```
//! use warp::Filter;
//!
//! let hi = warp::path("hello")
//!     .and(warp::path::param())
//!     .and(warp::header("user-agent"))
//!     .map(|param: String, agent: String| {
//!         format!("Hello {}, whose agent is {}", param, agent)
//!     });
//! ```
//!
//! This example composes several [`Filter`s][Filter] together using `and`:
//!
//! - A path prefix of "hello"
//! - A path parameter of a `String`
//! - The `user-agent` header parsed as a `String`
//!
//! These specific filters will [`reject`][reject] requests that don't match
//! their requirements.
//!
//! This ends up matching requests like:
//!
//! ```notrust
//! GET /hello/sean HTTP/1.1
//! Host: hyper.rs
//! User-Agent: reqwest/v0.8.6
//!
//! ```
//! And it returns a response similar to this:
//!
//! ```notrust
//! HTTP/1.1 200 OK
//! Content-Length: 41
//! Date: ...
//!
//! Hello sean, whose agent is reqwest/v0.8.6
//! ```
//!
//! Take a look at the full list of [`filters`](filters/index.html) to see what
//! you can build.
//!
//! ## Testing
//!
//! Testing your web services easily is extremely important, and warp provides
//! a [`test`](mod@self::test) module to help send mocked requests through your service.
//!
//! [Filter]: trait.Filter.html
//! [reject]: reject/index.html

#[macro_use]
mod error;
mod filter;
pub mod filters;
mod generic;
pub mod redirect;
pub mod reject;
pub mod reply;
mod route;
mod server;
mod service;
pub mod test;
#[cfg(feature = "tls")]
mod tls;
mod transport;

pub use self::error::Error;
pub use self::filter::Filter;
// This otherwise shows a big dump of re-exports in the doc homepage,
// with zero context, so just hide it from the docs. Doc examples
// on each can show that a convenient import exists.
#[cfg(feature = "compression")]
#[doc(hidden)]
pub use self::filters::compression;
#[cfg(feature = "multipart")]
#[doc(hidden)]
pub use self::filters::multipart;
#[cfg(feature = "websocket")]
#[doc(hidden)]
pub use self::filters::ws;
#[doc(hidden)]
pub use self::filters::{
    addr,
    // any() function
    any::any,
    body,
    cookie,
    // cookie() function
    cookie::cookie,
    cors,
    // cors() function
    cors::cors,
    ext,
    fs,
    header,
    // header() function
    header::header,
    host,
    log,
    // log() function
    log::log,
    method::{delete, get, head, method, options, patch, post, put},
    path,
    // path() function and macro
    path::path,
    query,
    // query() function
    query::query,
    sse,
    trace,
    // trace() function
    trace::trace,
};
// ws() function
pub use self::filter::wrap_fn;
#[cfg(feature = "websocket")]
#[doc(hidden)]
pub use self::filters::ws::ws;
#[doc(hidden)]
pub use self::redirect::redirect;
#[doc(hidden)]
#[allow(deprecated)]
pub use self::reject::{reject, Rejection};
#[doc(hidden)]
pub use self::reply::{reply, Reply};
#[cfg(feature = "tls")]
pub use self::server::TlsServer;
pub use self::server::{serve, Server};
pub use self::service::service;
#[doc(hidden)]
pub use http;
#[doc(hidden)]
pub use hyper;

#[doc(hidden)]
pub use bytes::Buf;
#[doc(hidden)]
pub use futures_util::{Future, Sink, Stream};
#[doc(hidden)]

pub(crate) type Request = http::Request<hyper::Body>;
