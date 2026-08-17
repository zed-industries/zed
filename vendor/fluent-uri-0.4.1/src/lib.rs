#![warn(
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    clippy::checked_conversions,
    clippy::if_not_else,
    clippy::ignored_unit_patterns,
    clippy::map_unwrap_or,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    // clippy::redundant_closure_for_method_calls,
    clippy::redundant_else,
    clippy::semicolon_if_nothing_returned,
    // clippy::single_match_else,
    clippy::use_self,
)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

//! A generic URI/IRI handling library compliant with [RFC 3986] and [RFC 3987].
//!
//! [RFC 3986]: https://datatracker.ietf.org/doc/html/rfc3986
//! [RFC 3987]: https://datatracker.ietf.org/doc/html/rfc3987
//!
//! **Examples:** [Parsing](Uri#examples). [Building](build::Builder#examples).
//! [Reference resolution](UriRef::resolve_against). [Normalization](Uri::normalize).
//! [Percent-decoding](crate::pct_enc::EStr#examples).
//! [Percent-encoding](crate::pct_enc::EString#examples).
//!
//! # Terminology
//!
//! A *[URI reference]* is either a *[URI]* or a *[relative reference]*. If it starts with a *[scheme]*
//! (like `http`, `ftp`, `mailto`, etc.) followed by a colon (`:`), it is a URI. For example,
//! `http://example.com/` and `mailto:user@example.com` are URIs. Otherwise, it is
//! a relative reference. For example, `//example.org/`, `/index.html`, `../`, `foo`,
//! `?bar`, and `#baz` are relative references.
//!
//! An *[IRI]* (reference) is an internationalized version of URI (reference)
//! which may contain non-ASCII characters.
//!
//! [URI]: https://datatracker.ietf.org/doc/html/rfc3986#section-3
//! [URI reference]: https://datatracker.ietf.org/doc/html/rfc3986#section-4.1
//! [IRI]: https://datatracker.ietf.org/doc/html/rfc3987#section-2
//! [relative reference]: https://datatracker.ietf.org/doc/html/rfc3986#section-4.2
//! [scheme]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.1
//!
//! # Guidance for crate users
//!
//! Advice for designers of new URI schemes can be found in [RFC 7595].
//! Guidance on the specification of URI substructure in standards
//! can be found in [RFC 8820]. The crate author recommends [RFC 9413]
//! for further reading as the long-term interoperability
//! of URI schemes may be of concern.
//!
//! [RFC 7595]: https://datatracker.ietf.org/doc/html/rfc7595
//! [RFC 8820]: https://datatracker.ietf.org/doc/html/rfc8820
//! [RFC 9413]: https://datatracker.ietf.org/doc/html/rfc9413
//!
//! # Crate features
//!
//! - `std` (default): Implies `alloc` and `impl-error`. Required for [`Authority::socket_addrs`].
//!
//! - `alloc`: Required for memory-allocating types and functions.
//!
//! - `impl-error`: Required for [`Error`] implementations. Disabling `std`
//!   while enabling `impl-error` requires a minimum Rust version of 1.81.
//!
//! - `net`: Required for IP address fields in [`Host`], for [`Builder::host`] to
//!   take an IP address as argument, and for [`Authority::socket_addrs`].
//!   Disabling `std` while enabling `net` requires a minimum Rust version of 1.77.
//!
//! - `serde`: Required for [`Serialize`] and [`Deserialize`] implementations.
//!
//! [`Host`]: component::Host
//! [`Builder::host`]: build::Builder::host
//! [`Authority::socket_addrs`]: component::Authority::socket_addrs
//! [`Error`]: core::error::Error
//! [`Serialize`]: serde::Serialize
//! [`Deserialize`]: serde::Deserialize

#[cfg(feature = "alloc")]
pub mod build;
pub mod component;
mod convert;
mod fmt;
mod imp;
#[cfg(feature = "alloc")]
pub mod normalize;
mod parse;
pub mod pct_enc;
#[cfg(feature = "alloc")]
pub mod resolve;
mod utf8;

pub use convert::ConvertError;
pub use imp::{Iri, IriRef, Uri, UriRef};
pub use parse::{ParseError, ParseErrorKind};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(feature = "net", not(feature = "std")))]
use core::net;
#[cfg(all(feature = "net", feature = "std"))]
use std::net;

#[cfg(all(feature = "impl-error", not(feature = "std")))]
use core::error::Error;
#[cfg(all(feature = "impl-error", feature = "std"))]
use std::error::Error;
