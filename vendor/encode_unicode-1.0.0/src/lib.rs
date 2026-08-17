/* Copyright 2016-2022 Torbjørn Birch Moltu
 * Copyright 2018 Aljoscha Meyer
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */


/*!
Miscellaneous UTF-8 and UTF-16 types and methods.

# Optional features:
* `#![no_std]`-mode: There are a few differences:
  * `Error` doesn't exist, but `description()` is made available as an inherent impl.
  * `Extend`/`FromIterator`-implementations for `String`/`Vec<u8>`/`Vec<u16>` are missing.
  * There is no `io`, so `Utf8Iterator` and `Utf8CharSplitter` doesn't implement `Read`.

  This feature is enabled by setting `default-features=false` in `Cargo.toml`:
  `encode_unicode = {version="0.3.4", default-features=false}`
* Integration with the [ascii](https://tomprogrammer.github.io/rust-ascii/ascii/index.html) crate:  
  Convert `Utf8Char` and `Utf16Char` to and from
  [`ascii::AsciiChar`](https://tomprogrammer.github.io/rust-ascii/ascii/enum.AsciiChar.html).

# Minimum supported Rust version

The minimum supported Rust version for 1.0.\* releases is 1.56.  
Later 1.y.0 releases might require newer Rust versions, but the three most
recent stable releases at the time of publishing will always be supported.
For example this means that if the current stable Rust version is 1.66 when
`encode_unicode` 1.1.0 is released, then `encode_unicode` 1.1.\* will
not require a newer Rust version than 1.63.

[crates.io page](https://crates.io/crates/encode_unicode)  
[github repository](https://github.com/tormol/encode_unicode)

*/

#![cfg_attr(not(feature="std"), no_std)]

#![warn(missing_docs, unsafe_op_in_unsafe_fn)]
#![allow(
    clippy::unusual_byte_groupings,// I sometimes group into UTF-8 control part and codepoint part
    clippy::derive_hash_xor_eq,// tested
    clippy::len_without_is_empty,// the character types are never empty
    clippy::needless_return,// `foo.bar();\n foo` looks unfinished
    clippy::redundant_closure_call,// not redundant in macros
    clippy::cast_lossless,// the sizes are part of the struct name and so won't change
    clippy::many_single_char_names,// the variables are in different scopes
    clippy::cmp_owned,// smaller than pointer, and no allocations anyway
    clippy::wrong_self_convention,// smaller than pointer
    clippy::needless_range_loop,// the suggested iterator chains are less intuitive
    clippy::identity_op,// applying a set of opereations with varying arguments to many elements looks nice
    clippy::get_first,// .get(0), .get(1) is more readable
    clippy::question_mark,// I prefer it very explicit
)]
#![warn(clippy::doc_markdown, clippy::manual_filter_map)]
// opt-in lints that might be interesting to recheck once in a while:
//#![warn(clippy::unwrap_used)]

mod errors;
mod traits;
mod utf8_char;
mod utf8_iterators;
mod utf16_char;
mod utf16_iterators;
mod decoding_iterators;

pub use traits::{CharExt, U8UtfExt, U16UtfExt, StrExt, IterExt, SliceExt};
pub use utf8_char::Utf8Char;
pub use utf16_char::Utf16Char;

pub mod error {// keeping the public interface in one file
    //! Errors returned by various conversion methods in this crate.
    pub use crate::errors::{FromStrError, EmptyStrError};
    pub use crate::errors::{CodepointError, NonAsciiError, NonBmpError};
    pub use crate::errors::{Utf8Error, Utf8ErrorKind};
    pub use crate::errors::{Utf16SliceError, Utf16ArrayError, Utf16TupleError};
    pub use crate::errors::{Utf16FirstUnitError, Utf16PairError};
}

pub mod iterator {
    //! Iterator types that you should rarely need to name
    pub use crate::utf8_iterators::{Utf8Iterator, Utf8CharSplitter, Utf8Chars, Utf8CharIndices};
    pub use crate::utf16_iterators::{Utf16Iterator, Utf16CharSplitter, Utf16Chars, Utf16CharIndices};
    pub use crate::decoding_iterators::{Utf8CharMerger, Utf8CharDecoder};
    pub use crate::decoding_iterators::{Utf16CharMerger, Utf16CharDecoder};
}
