//! Simple Doxygen to Rustdoc translation.
//!
//! Provides a simple and straightforward API to translate _raw_ Doxygen comments to Rustdoc
//! comments. Purely experimental right now, maybe practical in a future?
//!
//! # Examples
//!
//! ```
//! use doxygen_rs::transform;
//!
//! let rustdoc = transform("@brief Example Doxygen brief");
//! assert_eq!(rustdoc, "Example Doxygen brief");
//! ```
//!
//! # Usage with bindgen >= 0.63
//!
//! ```
//! # trait ParseCallbacks {
//! #   fn process_comment(&self, comment: &str) -> Option<String>;
//! # }
//! #[derive(Debug)]
//! struct Cb;
//!
//! impl ParseCallbacks for Cb {
//!     fn process_comment(&self, comment: &str) -> Option<String> {
//!         Some(doxygen_rs::transform(comment))
//!     }
//! }
//! ```

mod emojis;
pub mod generator;
mod lexer;
mod parser;

/// This function transforms the Doxygen of a single element (function, struct, etc.)
///
/// # Panics
///
/// This function will panic if any error from [`generator::rustdoc`] is returned.
pub fn transform(value: &str) -> String {
    generator::rustdoc(value.into()).expect("failed to transform the comments")
}
