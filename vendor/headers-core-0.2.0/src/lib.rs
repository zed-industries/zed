#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]
#![doc(html_root_url = "https://docs.rs/headers-core/0.2.0")]

//! # headers-core
//!
//! This is the core crate of the typed HTTP headers system, providing only
//! the relevant traits. All actual header implementations are in other crates.

extern crate http;

pub use http::header::{self, HeaderName, HeaderValue};

use std::error;
use std::fmt::{self, Display, Formatter};

/// A trait for any object that will represent a header field and value.
///
/// This trait represents the construction and identification of headers,
/// and contains trait-object unsafe methods.
pub trait Header {
    /// The name of this header.
    fn name() -> &'static HeaderName;

    /// Decode this type from an iterator of `HeaderValue`s.
    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>;

    /// Encode this type to a `HeaderMap`.
    ///
    /// This function should be infallible. Any errors converting to a
    /// `HeaderValue` should have been caught when parsing or constructing
    /// this value.
    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E);
}

/// Errors trying to decode a header.
#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    Invalid,
}

impl Error {
    /// Create an 'invalid' Error.
    pub fn invalid() -> Error {
        Error {
            kind: Kind::Invalid,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.kind {
            Kind::Invalid => f.write_str("invalid HTTP header"),
        }
    }
}

impl error::Error for Error {}
