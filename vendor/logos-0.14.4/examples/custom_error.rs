//! ASCII tokens lexer with custom error type.
//!
//! Takes tabs-or-spaces separated words or u8 numbers,
//! only accepting ascii letters.
//!
//! Usage:
//!     cargo run --example custom_error

/* ANCHOR: all */
use logos::Logos;

use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq)]
enum LexingError {
    InvalidInteger(String),
    #[default]
    NonAsciiCharacter,
}

/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexingError::InvalidInteger("overflow error".to_owned()),
            _ => LexingError::InvalidInteger("other error".to_owned()),
        }
    }
}

#[derive(Debug, Logos, PartialEq)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t]+")]
enum Token {
    #[regex(r"[a-zA-Z]+")]
    Word,
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Integer(u8),
}

fn main() {
    // 256 overflows u8, since u8's max value is 255.
    // 'é' is not a valid ascii letter.
    let mut lex = Token::lexer("Hello 256 Jérome");

    assert_eq!(lex.next(), Some(Ok(Token::Word)));
    assert_eq!(lex.slice(), "Hello");

    assert_eq!(
        lex.next(),
        Some(Err(LexingError::InvalidInteger(
            "overflow error".to_owned()
        )))
    );
    assert_eq!(lex.slice(), "256");

    assert_eq!(lex.next(), Some(Ok(Token::Word)));
    assert_eq!(lex.slice(), "J");

    assert_eq!(lex.next(), Some(Err(LexingError::NonAsciiCharacter)));
    assert_eq!(lex.slice(), "é");

    assert_eq!(lex.next(), Some(Ok(Token::Word)));
    assert_eq!(lex.slice(), "rome");

    assert_eq!(lex.next(), None);
}
/* ANCHOR_END: all */
