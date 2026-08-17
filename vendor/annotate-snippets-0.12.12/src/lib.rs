//! Format [diagnostic reports][Report], including highlighting snippets of text
//!
//! # Example
//!
//! ```rust
//! # #[allow(clippy::needless_doctest_main)]
#![doc = include_str!("../examples/expected_type.rs")]
//! ```
//!
#![doc = include_str!("../examples/expected_type.svg")]
//!
//! # Visual overview
//!
//! [`Report`]
//!
#![doc = include_str!("../examples/multi_suggestion.svg")]
//!
//! ### Primary group
//!
//! [`Title`]
//! ```text
//! error: cannot construct `Box<_, _>` with struct literal syntax due to private fields
//! ```
//!
//!
//! [`Annotation`] on a [`Snippet`]
//! ```text
//!    ╭▸ $DIR/multi-suggestion.rs:17:13
//!    │
//! 17 │     let _ = Box {};
//!    │             ━━━
//!    │
//! ```
//!
//! [`Message`]
//! ```text
//!    ╰ note: private fields `0` and `1` that were not provided
//! ```
//!
//!
//!
//! ### Secondary group: suggested fix
//!
//! [`Title`] (proposed solution)
//! ```text
//! help: you might have meant to use an associated function to build this type
//! ```
//!
//! [`Patch`] Option 1 on a [`Snippet`]
//! ```text
//!    ╭╴
//! 21 -     let _ = Box {};
//! 21 +     let _ = Box::new(_);
//!    ├╴
//! ```
//!
//! [`Patch`] Option 2 on a [`Snippet`]
//! ```text
//!    ├╴
//! 17 -     let _ = Box {};
//! 17 +     let _ = Box::new_uninit();
//!    ├╴
//! ```
//!
//! *etc for Options 3 and 4*
//!
//! [`Message`]
//! ```text
//!    ╰ and 12 other candidates
//! ```
//!
//! ### Secondary group: alternative suggested fix
//!
//! [`Title`] (proposed solution)
//! ```text
//! help: consider using the `Default` trait
//! ```
//!
//! Only [`Patch`] on a [`Snippet`]
//! ```text
//!    ╭╴
//! 17 -     let _ = Box {};
//! 17 +     let _ = <Box as std::default::Default>::default();
//!    ╰╴
//! ```
//!
//! # Cargo `features`
//!
//! - `simd` - Speeds up folding
//!
//! - `testing-colors` - Makes [Renderer::styled] colors OS independent, which
//! allows for easier testing when testing colored output. It should be added as
//! a feature in `[dev-dependencies]`, which can be done with the following command:
//! ```text
//! cargo add annotate-snippets --dev --feature testing-colors
//! ```

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![warn(missing_debug_implementations)]

extern crate alloc;

use alloc::string::String;

pub mod level;
pub mod renderer;
mod snippet;

/// Normalize the string to avoid any unicode control characters.
///
/// This is important for untrusted input, as it can contain
/// invalid unicode sequences.
pub fn normalize_untrusted_str(s: &str) -> String {
    renderer::normalize_whitespace(s)
}

#[doc(inline)]
pub use level::Level;
#[doc(inline)]
pub use renderer::Renderer;
pub use snippet::*;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
