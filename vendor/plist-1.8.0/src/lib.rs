//! # Plist
//!
//! A rusty plist parser.
//!
//! ## Usage
//!
//! Put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! plist = "1"
//! ```
//!
//! And put this in your crate root:
//!
//! ```rust
//! extern crate plist;
//! ```
//!
//! ## Examples
//!
//! ### Using `serde`
//!
//! ```rust
//! extern crate plist;
//! # #[cfg(feature = "serde")]
//! #[macro_use]
//! extern crate serde_derive;
//!
//! # #[cfg(feature = "serde")]
//! # fn main() {
//! #[derive(Deserialize)]
//! #[serde(rename_all = "PascalCase")]
//! struct Book {
//!     title: String,
//!     author: String,
//!     excerpt: String,
//!     copies_sold: u64,
//! }
//!
//! let book: Book = plist::from_file("tests/data/book.plist")
//!     .expect("failed to read book.plist");
//!
//! assert_eq!(book.title, "Great Expectations");
//! # }
//! #
//! # #[cfg(not(feature = "serde"))]
//! # fn main() {}
//! ```
//!
//! ### Using `Value`
//!
//! ```rust
//! use plist::Value;
//!
//! let book = Value::from_file("tests/data/book.plist")
//!     .expect("failed to read book.plist");
//!
//! let title = book
//!     .as_dictionary()
//!     .and_then(|dict| dict.get("Title"))
//!     .and_then(|title| title.as_string());
//!
//! assert_eq!(title, Some("Great Expectations"));
//! ```
//!
//! ## Unstable Features
//!
//! Many features from previous versions are now hidden behind the
//! `enable_unstable_features_that_may_break_with_minor_version_bumps` feature. These will break in
//! minor version releases after the 1.0 release. If you really really must use them you should
//! specify a tilde requirement e.g. `plist = "~1.0.3"` in you `Cargo.toml` so that the plist crate
//! is not automatically updated to version 1.1.

#![deny(warnings)] // Treat all warnings as errors
#![deny(rustdoc::broken_intra_doc_links)]

pub mod dictionary;

#[cfg(feature = "enable_unstable_features_that_may_break_with_minor_version_bumps")]
pub mod stream;
#[cfg(not(feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"))]
mod stream;

#[cfg(feature = "serde")]
mod data;
mod date;
mod error;
mod integer;
mod uid;
mod value;

#[cfg(feature = "serde")]
pub use data::Data;
pub use date::{Date, InvalidXmlDate};
pub use dictionary::Dictionary;
pub use error::Error;
pub use integer::Integer;
pub use stream::XmlWriteOptions;
pub use uid::Uid;
pub use value::Value;

// Optional serde module
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;
#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;
#[cfg(all(
    feature = "serde",
    any(
        test,
        feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"
    )
))]
pub use self::{de::Deserializer, ser::Serializer};
#[cfg(feature = "serde")]
pub use self::{
    de::{from_bytes, from_file, from_reader, from_reader_ascii, from_reader_xml, from_value},
    ser::{
        to_file_binary, to_file_xml, to_value, to_writer_binary, to_writer_xml,
        to_writer_xml_with_options,
    },
};

#[cfg(all(test, feature = "serde"))]
#[macro_use]
extern crate serde_derive;

#[cfg(all(test, feature = "serde"))]
mod serde_tests;

fn u64_to_usize(len_u64: u64) -> Option<usize> {
    usize::try_from(len_u64).ok()
}
