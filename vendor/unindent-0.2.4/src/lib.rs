//! [![github]](https://github.com/dtolnay/indoc)&ensp;[![crates-io]](https://crates.io/crates/unindent)&ensp;[![docs-rs]](https://docs.rs/unindent)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! ## Unindent
//!
//! This crate provides [`indoc`]'s indentation logic for use with strings that
//! are not statically known at compile time. For unindenting string literals,
//! use `indoc` instead.
//!
//! [`indoc`]: https://github.com/dtolnay/indoc
//!
//! This crate exposes two unindent functions and an extension trait:
//!
//! - `fn unindent(&str) -> String`
//! - `fn unindent_bytes(&[u8]) -> Vec<u8>`
//! - `trait Unindent`
//!
//! ```
//! use unindent::unindent;
//!
//! fn main() {
//!     let indented = "
//!             line one
//!             line two";
//!     assert_eq!("line one\nline two", unindent(indented));
//! }
//! ```
//!
//! The `Unindent` extension trait expose the same functionality under an
//! extension method.
//!
//! ```
//! use unindent::Unindent;
//!
//! fn main() {
//!     let indented = format!("
//!             line {}
//!             line {}", "one", "two");
//!     assert_eq!("line one\nline two", indented.unindent());
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/unindent/0.2.4")]
#![allow(
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::trivially_copy_pass_by_ref,
    clippy::type_complexity
)]

mod unindent;

pub use crate::unindent::*;
