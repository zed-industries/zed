/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![crate_name = "cssparser"]
#![crate_type = "rlib"]
#![cfg_attr(feature = "bench", feature(test))]
#![deny(missing_docs)]

/*!

Implementation of [CSS Syntax Module Level 3](https://drafts.csswg.org/css-syntax/) for Rust.

# Input

Everything is based on `Parser` objects, which borrow a `&str` input.
If you have bytes (from a file, the network, or something)
and want to support character encodings other than UTF-8,
see the `stylesheet_encoding` function,
which can be used together with rust-encoding or encoding-rs.

# Conventions for parsing functions

* Take (at least) a `input: &mut cssparser::Parser` parameter
* Return `Result<_, ()>`
* When returning `Ok(_)`,
  the function must have consumed exactly the amount of input that represents the parsed value.
* When returning `Err(())`, any amount of input may have been consumed.

As a consequence, when calling another parsing function, either:

* Any `Err(())` return value must be propagated.
  This happens by definition for tail calls,
  and can otherwise be done with the `?` operator.
* Or the call must be wrapped in a `Parser::try` call.
  `try` takes a closure that takes a `Parser` and returns a `Result`,
  calls it once,
  and returns itself that same result.
  If the result is `Err`,
  it restores the position inside the input to the one saved before calling the closure.

Examples:

```{rust,ignore}
// 'none' | <image>
fn parse_background_image(context: &ParserContext, input: &mut Parser)
                                    -> Result<Option<Image>, ()> {
    if input.try_parse(|input| input.expect_ident_matching("none")).is_ok() {
        Ok(None)
    } else {
        Image::parse(context, input).map(Some)  // tail call
    }
}
```

```{rust,ignore}
// [ <length> | <percentage> ] [ <length> | <percentage> ]?
fn parse_border_spacing(_context: &ParserContext, input: &mut Parser)
                          -> Result<(LengthOrPercentage, LengthOrPercentage), ()> {
    let first = LengthOrPercentage::parse?;
    let second = input.try_parse(LengthOrPercentage::parse).unwrap_or(first);
    (first, second)
}
```

*/

#![recursion_limit = "200"] // For color::parse_color_keyword

pub use crate::cow_rc_str::CowRcStr;
pub use crate::from_bytes::{stylesheet_encoding, EncodingSupport};
#[doc(hidden)]
pub use crate::macros::{
    _cssparser_internal_create_uninit_array, _cssparser_internal_to_lowercase,
};
pub use crate::nth::parse_nth;
pub use crate::parser::{BasicParseError, BasicParseErrorKind, ParseError, ParseErrorKind};
pub use crate::parser::{Delimiter, Delimiters, Parser, ParserInput, ParserState};
pub use crate::rules_and_declarations::{parse_important, parse_one_declaration};
pub use crate::rules_and_declarations::{parse_one_rule, StyleSheetParser};
pub use crate::rules_and_declarations::{AtRuleParser, QualifiedRuleParser};
pub use crate::rules_and_declarations::{DeclarationParser, RuleBodyItemParser, RuleBodyParser};
pub use crate::serializer::{serialize_identifier, serialize_name, serialize_string};
pub use crate::serializer::{CssStringWriter, ToCss, TokenSerializationType};
pub use crate::tokenizer::{SourceLocation, SourcePosition, Token};
pub use crate::unicode_range::UnicodeRange;
pub use cssparser_macros::*;
#[doc(hidden)]
pub use phf as _cssparser_internal_phf;

#[macro_use]
mod macros;

mod rules_and_declarations;
mod tokenizer;

pub mod color;
mod cow_rc_str;
mod from_bytes;
mod nth;
mod parser;
mod serializer;
mod unicode_range;

#[cfg(test)]
mod size_of_tests;
#[cfg(test)]
mod tests;
