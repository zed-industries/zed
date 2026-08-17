// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate provides a push based XML parser library that
//! adheres to XML5 specification. In other words this library
//! trades well-formedness for error recovery.
//!
//! The idea behind this, was to minimize number of errors from
//! tools that generate XML (e.g. `&#83` won't just return `&#83`
//! as text, but will parse it into `S` ).
//! You can check out full specification [here](https://ygg01.github.io/xml5_draft/).
//!
//! What this library provides is a solid XML parser that can:
//!
//!   * Parse somewhat erroneous XML input
//!   * Provide support for [Numeric character references](https://en.wikipedia.org/wiki/Numeric_character_reference).
//!   * Provide partial [XML namespace](http://www.w3.org/TR/xml-names11/) support.
//!   * Provide full set of SVG/MathML entities
//!
//! What isn't in scope for this library:
//!
//!   * Document Type Definition parsing - this is pretty hard to do right and nowadays, its used
//!

#![crate_name = "xml5ever"]
#![crate_type = "dylib"]
#![deny(missing_docs)]

pub use markup5ever::*;

macro_rules! time {
    ($e:expr) => {{
        let t0 = ::std::time::Instant::now();
        let result = $e;
        let dt = t0.elapsed().as_nanos() as u64;
        (result, dt)
    }};
}

/// Driver
pub mod driver;
/// Serializer for XML5.
pub mod serialize;
/// XML5 tokenizer - converts input into tokens
pub mod tokenizer;
/// XML5 tree builder - converts tokens into a tree like structure
pub mod tree_builder;
