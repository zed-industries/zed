//! Implements a tokenizer and parsers for POSIX / bash shell syntax.

pub mod arithmetic;
pub mod ast;
pub mod pattern;
pub mod prompt;
pub mod readline_binding;
pub mod test_command;
pub mod word;

mod error;
mod parser;
mod tokenizer;

#[cfg(test)]
mod snapshot_tests;

pub use error::{
    BindingParseError, ParseError, ParseErrorLocation, TestCommandParseError, WordParseError,
};

#[cfg(feature = "diagnostics")]
pub use error::miette::PrettyError;

pub use parser::{Parser, ParserBuilder, ParserOptions, SourceInfo, parse_tokens};
pub use tokenizer::{
    SourcePosition, Token, TokenLocation, TokenizerError, TokenizerOptions, tokenize_str,
    tokenize_str_with_options, uncached_tokenize_str, unquote_str,
};
