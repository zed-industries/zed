//! > winnow, making parsing a breeze
//!
//! `winnow` is a parser combinator library
//!
//! Quick links:
//! - [List of combinators][crate::combinator]
//! - [Tutorial][_tutorial::chapter_0]
//! - [Special Topics][_topic]
//! - [Discussions](https://github.com/winnow-rs/winnow/discussions)
//! - [CHANGELOG](https://github.com/winnow-rs/winnow/blob/v1.0.2/CHANGELOG.md) (includes major version migration
//!   guides)
//!
//! ## Aspirations
//!
//! `winnow` aims to be your "do everything" parser, much like people treat regular expressions.
//!
//! In roughly priority order:
//! 1. Support writing parser declaratively while not getting in the way of imperative-style
//!    parsing when needed, working as an open-ended toolbox rather than a close-ended framework.
//! 2. Flexible enough to be used for any application, including parsing strings, binary data,
//!    or separate [lexing and parsing phases][_topic::lexing]
//! 3. Zero-cost abstractions, making it easy to write high performance parsers
//! 4. Easy to use, making it trivial for one-off uses
//!
//! In addition:
//! - Resilient maintainership, including
//!   - Willing to break compatibility rather than batching up breaking changes in large releases
//!   - Leverage feature flags to keep one active branch
//! - We will support the last 6 months of rust releases (MSRV)
//!
//! See also [Special Topic: Why winnow?][crate::_topic::why]
//!
//! ## Example
//!
//! Run
//! ```console
//! $ cargo add winnow
//! ```
//!
//! Then use it to parse:
//! ```rust
//! # #[cfg(all(feature = "alloc", feature = "parser"))] {
#![doc = include_str!("../examples/css/parser.rs")]
//! # }
//! ```
//!
//! See also the [Tutorial][_tutorial::chapter_0] and [Special Topics][_topic]

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(extended_key_value_attributes))]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[cfg(feature = "alloc")]
#[cfg_attr(test, macro_use)]
#[allow(unused_extern_crates)]
extern crate alloc;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

pub(crate) mod util {
    #[allow(dead_code)]
    pub(crate) fn from_fn<F: Fn(&mut core::fmt::Formatter<'_>) -> core::fmt::Result>(
        f: F,
    ) -> FromFn<F> {
        FromFn(f)
    }

    pub(crate) struct FromFn<F>(F)
    where
        F: Fn(&mut core::fmt::Formatter<'_>) -> core::fmt::Result;

    impl<F> core::fmt::Debug for FromFn<F>
    where
        F: Fn(&mut core::fmt::Formatter<'_>) -> core::fmt::Result,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            (self.0)(f)
        }
    }

    impl<F> core::fmt::Display for FromFn<F>
    where
        F: Fn(&mut core::fmt::Formatter<'_>) -> core::fmt::Result,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            (self.0)(f)
        }
    }
}

#[macro_use]
mod macros;

#[macro_use]
#[cfg(feature = "parser")]
pub mod error;

#[cfg(feature = "parser")]
mod parser;

pub mod stream;

#[cfg(feature = "ascii")]
pub mod ascii;
#[cfg(feature = "binary")]
pub mod binary;
#[cfg(feature = "parser")]
pub mod combinator;
#[cfg(feature = "parser")]
pub mod token;

#[cfg(feature = "unstable-doc")]
pub mod _topic;
#[cfg(feature = "unstable-doc")]
pub mod _tutorial;

/// Core concepts available for glob import
///
/// Including
/// - [`StreamIsPartial`][crate::stream::StreamIsPartial]
/// - [`Parser`]
///
/// ## Example
///
/// ```rust
/// # #[cfg(feature = "ascii")] {
/// use winnow::prelude::*;
///
/// fn parse_data(input: &mut &str) -> ModalResult<u64> {
///     // ...
/// #   winnow::ascii::dec_uint(input)
/// }
///
/// fn main() {
///   let result = parse_data.parse("100");
///   assert_eq!(result, Ok(100));
/// }
/// # }
/// ```
pub mod prelude {
    #[cfg(feature = "parser")]
    pub use crate::error::ModalError as _;
    #[cfg(feature = "parser")]
    pub use crate::error::ParserError as _;
    pub use crate::stream::AsChar as _;
    pub use crate::stream::ContainsToken as _;
    pub use crate::stream::Stream as _;
    pub use crate::stream::StreamIsPartial as _;
    #[cfg(feature = "parser")]
    pub use crate::ModalParser;
    #[cfg(feature = "parser")]
    pub use crate::ModalResult;
    #[cfg(feature = "parser")]
    pub use crate::Parser;
    #[cfg(feature = "unstable-recover")]
    #[cfg(feature = "std")]
    #[cfg(feature = "parser")]
    pub use crate::RecoverableParser as _;

    #[cfg(all(test, feature = "parser"))]
    pub(crate) use crate::TestResult;
}

#[cfg(feature = "parser")]
pub use error::ModalResult;
#[cfg(feature = "parser")]
pub use error::Result;
#[cfg(feature = "unstable-recover")]
#[cfg(feature = "std")]
#[cfg(feature = "parser")]
pub use parser::RecoverableParser;
#[cfg(feature = "parser")]
pub use parser::{ModalParser, Parser};
pub use stream::BStr;
pub use stream::Bytes;
pub use stream::LocatingSlice;
pub use stream::Partial;
pub use stream::Stateful;
pub use stream::Str;

#[cfg(all(test, feature = "parser"))]
pub(crate) use error::TestResult;
