// pest. The Elegant Parser
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{format, vec, vec::Vec};

#[macro_use]
extern crate pest;
extern crate pest_derive;

#[cfg(feature = "grammar-extras")]
use pest_derive::Parser;

/// Surround-string parser.
///
/// This is a simple grammar of string parsing, where the strings end with a different delimiter char than what they
/// start with. The allowable forms are:
///
/// ```text
/// (foo)
/// <bar>
/// ```
///
/// To keep things simple, strings do not support any escape sequences.
#[derive(Parser)]
#[grammar = "../tests/surround.pest"]
#[cfg(feature = "grammar-extras")]
pub struct SurroundParser;

#[test]
#[cfg(feature = "grammar-extras")]
fn surround_parenthesis() {
    parses_to! {
        parser: SurroundParser,
        input: "(hello world)",
        rule: Rule::Quote,
        tokens: [
            QuoteChars(1, 12)
        ]
    }
}

#[test]
#[cfg(feature = "grammar-extras")]
fn surround_angle_brackets() {
    parses_to! {
        parser: SurroundParser,
        input: "<hello world>",
        rule: Rule::Quote,
        tokens: [
            QuoteChars(1, 12)
        ]
    }
}

#[test]
#[cfg(feature = "grammar-extras")]
fn start_with_one_end_with_other() {
    fails_with! {
        parser: SurroundParser,
        input: "(hello world>",
        rule: Rule::Quote,
        positives: vec![],
        negatives: vec![],
        pos: 0
    }
}
