// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.
//! # pest meta
//!
//! This crate parses, validates, optimizes, and converts pest's own grammars to ASTs.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/pest-parser/pest/master/pest-logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/pest-parser/pest/master/pest-logo.svg"
)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(test)]
#[macro_use]
extern crate pest;

use std::fmt::Display;
use std::sync::LazyLock;

use pest::error::Error;
use pest::unicode::unicode_property_names;

pub mod ast;
pub mod optimizer;
pub mod parser;
pub mod validator;

/// A helper that will unwrap the result or panic
/// with the nicely formatted error message.
pub fn unwrap_or_report<T, E>(result: Result<T, E>) -> T
where
    E: IntoIterator,
    E::Item: Display,
{
    result.unwrap_or_else(|e| {
        panic!(
            "{}{}",
            "grammar error\n\n".to_owned(),
            &e.into_iter()
                .map(|error| format!("{}", error))
                .collect::<Vec<_>>()
                .join("\n\n")
        )
    })
}

/// A tuple returned by the validation and processing of the parsed grammar.
/// The first element is the vector of used builtin rule names,
/// the second element is the vector of optimized rules.
type UsedBuiltinAndOptimized<'i> = (Vec<&'i str>, Vec<optimizer::OptimizedRule>);

/// Parses, validates, processes and optimizes the provided grammar.
pub fn parse_and_optimize(
    grammar: &str,
) -> Result<UsedBuiltinAndOptimized<'_>, Vec<Error<parser::Rule>>> {
    let pairs = match parser::parse(parser::Rule::grammar_rules, grammar) {
        Ok(pairs) => Ok(pairs),
        Err(error) => Err(vec![error]),
    }?;

    let defaults = validator::validate_pairs(pairs.clone())?;
    let ast = parser::consume_rules(pairs)?;

    Ok((defaults, optimizer::optimize(ast)))
}

#[doc(hidden)]
#[deprecated(note = "use `pest::unicode::unicode_property_names` instead")]
pub static UNICODE_PROPERTY_NAMES: LazyLock<Vec<&str>> =
    LazyLock::new(|| unicode_property_names().collect::<Vec<_>>());
