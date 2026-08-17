//! Wrapper types to enable optimized handling of `&[u8]` and `Vec<u8>`.
//!
//! Without specialization, Rust forces Serde to treat `&[u8]` just like any
//! other slice and `Vec<u8>` just like any other vector. In reality this
//! particular slice and vector can often be serialized and deserialized in a
//! more efficient, compact representation in many formats.
//!
//! When working with such a format, you can opt into specialized handling of
//! `&[u8]` by wrapping it in `serde_bytes::Bytes` and `Vec<u8>` by wrapping it
//! in `serde_bytes::ByteBuf`.
//!
//! Additionally this crate supports the Serde `with` attribute to enable
//! efficient handling of `&[u8]` and `Vec<u8>` in structs without needing a
//! wrapper type.
//!
//! ```
//! # use serde_derive::{Deserialize, Serialize};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Deserialize, Serialize)]
//! struct Efficient<'a> {
//!     #[serde(with = "serde_bytes")]
//!     bytes: &'a [u8],
//!
//!     #[serde(with = "serde_bytes")]
//!     byte_buf: Vec<u8>,
//!
//!     #[serde(with = "serde_bytes")]
//!     byte_array: [u8; 314],
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/serde_bytes/0.11.19")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![allow(
    clippy::elidable_lifetime_names,
    clippy::into_iter_without_iter,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::needless_lifetimes,
    clippy::ptr_as_ptr
)]

extern crate serde_core as serde;

mod bytearray;
mod bytes;
mod de;
mod ser;

#[cfg(any(feature = "std", feature = "alloc"))]
mod bytebuf;

#[cfg(feature = "alloc")]
extern crate alloc;

use serde::{Deserializer, Serializer};

pub use crate::bytearray::ByteArray;
pub use crate::bytes::Bytes;
pub use crate::de::Deserialize;
pub use crate::ser::Serialize;

#[cfg(any(feature = "std", feature = "alloc"))]
pub use crate::bytebuf::ByteBuf;

/// Serde `serialize_with` function to serialize bytes efficiently.
///
/// This function can be used with either of the following Serde attributes:
///
/// - `#[serde(with = "serde_bytes")]`
/// - `#[serde(serialize_with = "serde_bytes::serialize")]`
///
/// ```
/// # use serde_derive::Serialize;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Efficient<'a> {
///     #[serde(with = "serde_bytes")]
///     bytes: &'a [u8],
///
///     #[serde(with = "serde_bytes")]
///     byte_buf: Vec<u8>,
///
///     #[serde(with = "serde_bytes")]
///     byte_array: [u8; 314],
/// }
/// ```
pub fn serialize<T, S>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ?Sized + Serialize,
    S: Serializer,
{
    Serialize::serialize(bytes, serializer)
}

/// Serde `deserialize_with` function to deserialize bytes efficiently.
///
/// This function can be used with either of the following Serde attributes:
///
/// - `#[serde(with = "serde_bytes")]`
/// - `#[serde(deserialize_with = "serde_bytes::deserialize")]`
///
/// ```
/// # use serde_derive::Deserialize;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Packet {
///     #[serde(with = "serde_bytes")]
///     payload: Vec<u8>,
///
///     #[serde(with = "serde_bytes")]
///     byte_array: [u8; 314],
/// }
/// ```
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer)
}
