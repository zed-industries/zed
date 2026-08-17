#![deny(missing_docs)]
#![allow(unknown_lints, bare_trait_objects, deprecated)]

//! Bincode is a crate for encoding and decoding using a tiny binary
//! serialization strategy.  Using it, you can easily go from having
//! an object in memory, quickly serialize it to bytes, and then
//! deserialize it back just as fast!
//!
//! ### Using Basic Functions
//!
//! ```edition2018
//! fn main() {
//!     // The object that we will serialize.
//!     let target: Option<String>  = Some("hello world".to_string());
//!
//!     let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
//!     let decoded: Option<String> = bincode::deserialize(&encoded[..]).unwrap();
//!     assert_eq!(target, decoded);
//! }
//! ```
//!
//! ### 128bit numbers
//!
//! Support for `i128` and `u128` is automatically enabled on Rust toolchains
//! greater than or equal to `1.26.0` and disabled for targets which do not support it

#![doc(html_root_url = "https://docs.rs/bincode/1.3.3")]
#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#[macro_use]
extern crate serde;

pub mod config;
/// Deserialize bincode data to a Rust data structure.
pub mod de;

mod byteorder;
mod error;
mod internal;
mod ser;

pub use config::{Config, DefaultOptions, Options};
pub use de::read::BincodeRead;
pub use de::Deserializer;
pub use error::{Error, ErrorKind, Result};
pub use ser::Serializer;

/// Get a default configuration object.
///
/// ### Default Configuration:
///
/// | Byte limit | Endianness |
/// |------------|------------|
/// | Unlimited  | Little     |
#[inline(always)]
#[deprecated(since = "1.3.0", note = "please use `options()` instead")]
pub fn config() -> Config {
    Config::new()
}

/// Get a default configuration object.
///
/// **Warning:** the default configuration returned by this function
/// is not the same as that used by the other functions in this
/// module. See the
/// [config](config/index.html#options-struct-vs-bincode-functions)
/// module for more details
///
/// ### Default Configuration:
///
/// | Byte limit | Endianness | Int Encoding | Trailing Behavior |
/// |------------|------------|--------------|-------------------|
/// | Unlimited  | Little     | Varint       | Reject            |
#[inline(always)]
pub fn options() -> DefaultOptions {
    DefaultOptions::new()
}

/// Serializes an object directly into a `Writer` using the default configuration.
///
/// If the serialization would take more bytes than allowed by the size limit, an error
/// is returned and *no bytes* will be written into the `Writer`.
///
/// **Warning:** the default configuration used by this function is not
/// the same as that used by the `DefaultOptions` struct. See the
/// [config](config/index.html#options-struct-vs-bincode-functions)
/// module for more details
pub fn serialize_into<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: serde::Serialize,
{
    DefaultOptions::new()
        .with_fixint_encoding()
        .serialize_into(writer, value)
}

/// Serializes a serializable object into a `Vec` of bytes using the default configuration.
///
/// **Warning:** the default configuration used by this function is not
/// the same as that used by the `DefaultOptions` struct. See the
/// [config](config/index.html#options-struct-vs-bincode-functions)
/// module for more details
pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: serde::Serialize,
{
    DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .serialize(value)
}

/// Deserializes an object directly from a `Read`er using the default configuration.
///
/// If this returns an `Error`, `reader` may be in an invalid state.
///
/// **Warning:** the default configuration used by this function is not
/// the same as that used by the `DefaultOptions` struct. See the
/// [config](config/index.html#options-struct-vs-bincode-functions)
/// module for more details
pub fn deserialize_from<R, T>(reader: R) -> Result<T>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize_from(reader)
}

/// Deserializes an object from a custom `BincodeRead`er using the default configuration.
/// It is highly recommended to use `deserialize_from` unless you need to implement
/// `BincodeRead` for performance reasons.
///
/// If this returns an `Error`, `reader` may be in an invalid state.
///
/// **Warning:** the default configuration used by this function is not
/// the same as that used by the `DefaultOptions` struct. See the
/// [config](config/index.html#options-struct-vs-bincode-functions)
/// module for more details
pub fn deserialize_from_custom<'a, R, T>(reader: R) -> Result<T>
where
    R: de::read::BincodeRead<'a>,
    T: serde::de::DeserializeOwned,
{
    DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize_from_custom(reader)
}

/// Only use this if you know what you're doing.
///
/// This is part of the public API.
#[doc(hidden)]
pub fn deserialize_in_place<'a, R, T>(reader: R, place: &mut T) -> Result<()>
where
    T: serde::de::Deserialize<'a>,
    R: BincodeRead<'a>,
{
    DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize_in_place(reader, place)
}

/// Deserializes a slice of bytes into an instance of `T` using the default configuration.
///
/// **Warning:** the default configuration used by this function is not
/// the same as that used by the `DefaultOptions` struct. See the
/// [config](config/index.html#options-struct-vs-bincode-functions)
/// module for more details
pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize(bytes)
}

/// Returns the size that an object would be if serialized using Bincode with the default configuration.
///
/// **Warning:** the default configuration used by this function is not
/// the same as that used by the `DefaultOptions` struct. See the
/// [config](config/index.html#options-struct-vs-bincode-functions)
/// module for more details
pub fn serialized_size<T: ?Sized>(value: &T) -> Result<u64>
where
    T: serde::Serialize,
{
    DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .serialized_size(value)
}
