//! # Objective-C type-encoding
//!
//! The Objective-C directive `@encode` encodes types as strings, and this is
//! used in various places in the runtime.
//!
//! This crate provides the [`Encoding`] type to describe and compare these
//! type-encodings, and the [`EncodingBox`] type which does the same, except
//! it can be parsed from an encoding at runtime.
//!
//! The types from this crate is exported under the [`objc2`] crate as
//! `objc2::encode`, so usually you would use it from there.
//!
//! [`objc2`]: https://crates.io/crates/objc2
//!
//!
//! ## Example
//!
//! Parse an encoding from a string and compare it to a known encoding.
//!
//! ```rust
//! use objc2_encode::{Encoding, EncodingBox};
//! let s = "{s=i}";
//! let enc = Encoding::Struct("s", &[Encoding::Int]);
//! let parsed: EncodingBox = s.parse()?;
//! assert!(enc.equivalent_to_box(&parsed));
//! assert_eq!(enc.to_string(), s);
//! # Ok::<(), objc2_encode::ParseError>(())
//! ```
//!
//!
//! ## Further resources
//!
//! - [Objective-C, Encoding and You](https://dmaclach.medium.com/objective-c-encoding-and-you-866624cc02de).
//! - [Apple's documentation on Type Encodings](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html).
//! - [How are the digits in ObjC method type encoding calculated?](https://stackoverflow.com/a/11527925)
//! - [`clang`'s source code for generating `@encode`](https://github.com/llvm/llvm-project/blob/fae0dfa6421ea6c02f86ba7292fa782e1e2b69d1/clang/lib/AST/ASTContext.cpp#L7500-L7850).

#![no_std]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-encode/4.1.0")]

#[cfg(not(feature = "alloc"))]
compile_error!("the `alloc` feature currently must be enabled");

extern crate alloc;
#[cfg(any(feature = "std", doc))]
extern crate std;

mod encoding;
mod encoding_box;
mod helper;
mod parse;

// Will be used at some point when generic constants are available
#[allow(dead_code)]
mod static_str;

pub use self::encoding::Encoding;
pub use self::encoding_box::EncodingBox;
pub use self::parse::ParseError;
