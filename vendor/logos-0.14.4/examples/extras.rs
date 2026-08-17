//! Print line and column positions for each word in a file.
//!
//! Usage:
//!     cargo run --example extras <path/to/file>
//!
//! Example:
//!     cargo run --example extras Cargo.toml
//!
//! This is a small example on how to use
//! [`Extras`](https://docs.rs/logos/latest/logos/trait.Logos.html#associatedtype.Extras)
//! to convey some (mutable) internal state from token to token.
//!
//! Here, the extras will be a tuple with the following fields:
//!
//! + 0. the line number;
//! + 1. the char index of the current line.
//!
//! From then, one can easily compute the column number of some token by computing:
//!
//! ```rust,no_run,no_playground
//! fn get_column(lex: &Lexer<Token>) -> usize {
//!     lex.span().start - lex.extras.1
//! }
//! ```

/* ANCHOR: all */
use logos::{Lexer, Logos, Skip};
use std::env;
use std::fs;

/* ANCHOR: callbacks */
/// Update the line count and the char index.
fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    Skip
}

/// Compute the line and column position for the current word.
fn word_callback(lex: &mut Lexer<Token>) -> (usize, usize) {
    let line = lex.extras.0;
    let column = lex.span().start - lex.extras.1;

    (line, column)
}
/* ANCHOR_END: callbacks */

/* ANCHOR: tokens */
/// Simple tokens to retrieve words and their location.
#[derive(Debug, Logos)]
#[logos(extras = (usize, usize))]
enum Token {
    #[regex(r"\n", newline_callback)]
    Newline,

    #[regex(r"\w+", word_callback)]
    Word((usize, usize)),
}
/* ANCHOR_END: tokens */

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    let mut lex = Token::lexer(src.as_str());

    while let Some(token) = lex.next() {
        if let Ok(Token::Word((line, column))) = token {
            println!("Word '{}' found at ({}, {})", lex.slice(), line, column);
        }
    }
}
/* ANCHOR_END: all */
