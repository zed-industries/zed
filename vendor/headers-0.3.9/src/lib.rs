#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(all(test, feature = "nightly"), feature(test))]
#![doc(html_root_url = "https://docs.rs/headers/0.3.9")]

//! # Typed HTTP Headers
//!
//! hyper has the opinion that headers should be strongly-typed, because that's
//! why we're using Rust in the first place. To set or get any header, an object
//! must implement the `Header` trait from this module. Several common headers
//! are already provided, such as `Host`, `ContentType`, `UserAgent`, and others.
//!
//! # Why Typed?
//!
//! Or, why not stringly-typed? Types give the following advantages:
//!
//! - More difficult to typo, since typos in types should be caught by the compiler
//! - Parsing to a proper type by default
//!
//! # Defining Custom Headers
//!
//! ## Implementing the `Header` trait
//!
//! Consider a Do Not Track header. It can be true or false, but it represents
//! that via the numerals `1` and `0`.
//!
//! ```
//! extern crate http;
//! extern crate headers;
//!
//! use headers::{Header, HeaderName, HeaderValue};
//!
//! struct Dnt(bool);
//!
//! impl Header for Dnt {
//!     fn name() -> &'static HeaderName {
//!          &http::header::DNT
//!     }
//!
//!     fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
//!     where
//!         I: Iterator<Item = &'i HeaderValue>,
//!     {
//!         let value = values
//!             .next()
//!             .ok_or_else(headers::Error::invalid)?;
//!
//!         if value == "0" {
//!             Ok(Dnt(false))
//!         } else if value == "1" {
//!             Ok(Dnt(true))
//!         } else {
//!             Err(headers::Error::invalid())
//!         }
//!     }
//!
//!     fn encode<E>(&self, values: &mut E)
//!     where
//!         E: Extend<HeaderValue>,
//!     {
//!         let s = if self.0 {
//!             "1"
//!         } else {
//!             "0"
//!         };
//!
//!         let value = HeaderValue::from_static(s);
//!
//!         values.extend(std::iter::once(value));
//!     }
//! }
//! ```

extern crate base64;
extern crate bytes;
extern crate headers_core;
extern crate http;
extern crate httpdate;
extern crate mime;
extern crate sha1;
#[cfg(all(test, feature = "nightly"))]
extern crate test;

pub use headers_core::{Error, Header};

#[doc(hidden)]
pub use http::HeaderMap;

#[doc(hidden)]
pub use http::header::{HeaderName, HeaderValue};

#[macro_use]
mod util;
mod common;
mod map_ext;

pub use self::common::*;
pub use self::map_ext::HeaderMapExt;
