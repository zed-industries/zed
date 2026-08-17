//! A [serde]-compatible [TOML]-parsing library
//!
//! TOML itself is a simple, ergonomic, and readable configuration format:
//!
//! ```toml
//! [package]
//! name = "toml"
//! version = "0.4.2"
//! authors = ["Alex Crichton <alex@alexcrichton.com>"]
//!
//! [dependencies]
//! serde = "1.0"
//! ```
//!
//! The TOML format tends to be relatively common throughout the Rust community
//! for configuration, notably being used by [Cargo], Rust's package manager.
//!
//! ## TOML values
//!
//! A value in TOML is represented with the [`Value`] enum in this crate:
//!
//! ```rust,ignore
//! pub enum Value {
//!     String(String),
//!     Integer(i64),
//!     Float(f64),
//!     Boolean(bool),
//!     Datetime(Datetime),
//!     Array(Array),
//!     Table(Table),
//! }
//! ```
//!
//! TOML is similar to JSON with the notable addition of a [`Datetime`]
//! type. In general, TOML and JSON are interchangeable in terms of
//! formats.
//!
//! ## Parsing TOML
//!
//! The easiest way to parse a TOML document is via the [`Value`] type:
//!
//! ```rust
//! use toml::Value;
//!
//! let value = "foo = 'bar'".parse::<Value>().unwrap();
//!
//! assert_eq!(value["foo"].as_str(), Some("bar"));
//! ```
//!
//! The [`Value`] type implements a number of convenience methods and
//! traits; the example above uses [`FromStr`] to parse a [`str`] into a
//! [`Value`].
//!
//! ## Deserialization and Serialization
//!
//! This crate supports [`serde`] 1.0 with a number of
//! implementations of the `Deserialize`, `Serialize`, `Deserializer`, and
//! `Serializer` traits. Namely, you'll find:
//!
//! * `Deserialize for Value`
//! * `Serialize for Value`
//! * `Deserialize for Datetime`
//! * `Serialize for Datetime`
//! * `Deserializer for de::Deserializer`
//! * `Serializer for ser::Serializer`
//! * `Deserializer for Value`
//!
//! This means that you can use Serde to deserialize/serialize the
//! [`Value`] type as well as the [`Datetime`] type in this crate. You can also
//! use the [`Deserializer`], [`Serializer`], or [`Value`] type itself to act as
//! a deserializer/serializer for arbitrary types.
//!
//! An example of deserializing with TOML is:
//!
//! ```rust
//! use serde_derive::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     ip: String,
//!     port: Option<u16>,
//!     keys: Keys,
//! }
//!
//! #[derive(Deserialize)]
//! struct Keys {
//!     github: String,
//!     travis: Option<String>,
//! }
//!
//! fn main() {
//!     let config: Config = toml::from_str(r#"
//!         ip = '127.0.0.1'
//!
//!         [keys]
//!         github = 'xxxxxxxxxxxxxxxxx'
//!         travis = 'yyyyyyyyyyyyyyyyy'
//!     "#).unwrap();
//!
//!     assert_eq!(config.ip, "127.0.0.1");
//!     assert_eq!(config.port, None);
//!     assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
//!     assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
//! }
//! ```
//!
//! You can serialize types in a similar fashion:
//!
//! ```rust
//! use serde_derive::Serialize;
//!
//! #[derive(Serialize)]
//! struct Config {
//!     ip: String,
//!     port: Option<u16>,
//!     keys: Keys,
//! }
//!
//! #[derive(Serialize)]
//! struct Keys {
//!     github: String,
//!     travis: Option<String>,
//! }
//!
//! fn main() {
//!     let config = Config {
//!         ip: "127.0.0.1".to_string(),
//!         port: None,
//!         keys: Keys {
//!             github: "xxxxxxxxxxxxxxxxx".to_string(),
//!             travis: Some("yyyyyyyyyyyyyyyyy".to_string()),
//!         },
//!     };
//!
//!     let toml = toml::to_string(&config).unwrap();
//! }
//! ```
//!
//! [TOML]: https://github.com/toml-lang/toml
//! [Cargo]: https://crates.io/
//! [`serde`]: https://serde.rs/
//! [serde]: https://serde.rs/

#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
// Makes rustc abort compilation if there are any unsafe blocks in the crate.
// Presence of this annotation is picked up by tools such as cargo-geiger
// and lets them ensure that there is indeed no unsafe code as opposed to
// something they couldn't detect (e.g. unsafe added via macro expansion, etc).
#![forbid(unsafe_code)]

pub mod map;
pub mod value;
#[doc(no_inline)]
pub use crate::value::Value;
mod datetime;

pub mod ser;
#[doc(no_inline)]
pub use crate::ser::{to_string, to_string_pretty, to_vec, Serializer};
pub mod de;
#[doc(no_inline)]
pub use crate::de::{from_slice, from_str, Deserializer};
mod tokens;

#[doc(hidden)]
pub mod macros;

mod spanned;
pub use crate::spanned::Spanned;

// Just for rustdoc
#[allow(unused_imports)]
use crate::datetime::Datetime;
#[allow(unused_imports)]
use core::str::FromStr;
