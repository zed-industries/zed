//! JSON parser written in Rust, using Logos.
//!
//! If the file is a valid JSON value, it will be printed
//! to the terminal using the debug format.
//!
//! Otherwise, an error will be printed with its location.
//!
//! Usage:
//!     cargo run --example json <path/to/file>
//!
//! Example:
//!     cargo run --example json examples/example.json

/* ANCHOR: all */
use logos::{Lexer, Logos, Span};

use std::collections::HashMap;
use std::env;
use std::fs;

type Error = (String, Span);

type Result<T> = std::result::Result<T, Error>;

/* ANCHOR: tokens */
/// All meaningful JSON tokens.
///
/// > NOTE: regexes for [`Token::Number`] and [`Token::String`] may not
/// > catch all possible values, especially for strings. If you find
/// > errors, please report them so that we can improve the regex.
#[derive(Debug, Logos)]
#[logos(skip r"[ \t\r\n\f]+")]
enum Token {
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("null")]
    Null,

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().to_owned())]
    String(String),
}
/* ANCHOR_END: tokens */

/* ANCHOR: values */
/// Represent any valid JSON value.
#[derive(Debug)]
enum Value {
    /// null.
    Null,
    /// true or false.
    Bool(bool),
    /// Any floating point number.
    Number(f64),
    /// Any quoted string.
    String(String),
    /// An array of values
    Array(Vec<Value>),
    /// An dictionary mapping keys and values.
    Object(HashMap<String, Value>),
}
/* ANCHOR_END: values */

/* ANCHOR: value */
/// Parse a token stream into a JSON value.
fn parse_value(lexer: &mut Lexer<'_, Token>) -> Result<Value> {
    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::Bool(b)) => Ok(Value::Bool(b)),
            Ok(Token::BraceOpen) => parse_object(lexer),
            Ok(Token::BracketOpen) => parse_array(lexer),
            Ok(Token::Null) => Ok(Value::Null),
            Ok(Token::Number(n)) => Ok(Value::Number(n)),
            Ok(Token::String(s)) => Ok(Value::String(s)),
            _ => Err((
                "unexpected token here (context: value)".to_owned(),
                lexer.span(),
            )),
        }
    } else {
        Err(("empty values are not allowed".to_owned(), lexer.span()))
    }
}
/* ANCHOR_END: value */

/* ANCHOR: array */
/// Parse a token stream into an array and return when
/// a valid terminator is found.
///
/// > NOTE: we assume '[' was consumed.
fn parse_array(lexer: &mut Lexer<'_, Token>) -> Result<Value> {
    let mut array = Vec::new();
    let span = lexer.span();
    let mut awaits_comma = false;
    let mut awaits_value = false;

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Bool(b)) if !awaits_comma => {
                array.push(Value::Bool(b));
                awaits_value = false;
            }
            Ok(Token::BraceOpen) if !awaits_comma => {
                let object = parse_object(lexer)?;
                array.push(object);
                awaits_value = false;
            }
            Ok(Token::BracketOpen) if !awaits_comma => {
                let sub_array = parse_array(lexer)?;
                array.push(sub_array);
                awaits_value = false;
            }
            Ok(Token::BracketClose) if !awaits_value => return Ok(Value::Array(array)),
            Ok(Token::Comma) if awaits_comma => awaits_value = true,
            Ok(Token::Null) if !awaits_comma => {
                array.push(Value::Null);
                awaits_value = false
            }
            Ok(Token::Number(n)) if !awaits_comma => {
                array.push(Value::Number(n));
                awaits_value = false;
            }
            Ok(Token::String(s)) if !awaits_comma => {
                array.push(Value::String(s));
                awaits_value = false;
            }
            _ => {
                return Err((
                    "unexpected token here (context: array)".to_owned(),
                    lexer.span(),
                ))
            }
        }
        awaits_comma = !awaits_value;
    }
    Err(("unmatched opening bracket defined here".to_owned(), span))
}
/* ANCHOR_END: array */

/* ANCHOR: object */
/// Parse a token stream into an object and return when
/// a valid terminator is found.
///
/// > NOTE: we assume '{' was consumed.
fn parse_object(lexer: &mut Lexer<'_, Token>) -> Result<Value> {
    let mut map = HashMap::new();
    let span = lexer.span();
    let mut awaits_comma = false;
    let mut awaits_key = false;

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::BraceClose) if !awaits_key => return Ok(Value::Object(map)),
            Ok(Token::Comma) if awaits_comma => awaits_key = true,
            Ok(Token::String(key)) if !awaits_comma => {
                match lexer.next() {
                    Some(Ok(Token::Colon)) => (),
                    _ => {
                        return Err((
                            "unexpected token here, expecting ':'".to_owned(),
                            lexer.span(),
                        ))
                    }
                }
                let value = parse_value(lexer)?;
                map.insert(key, value);
                awaits_key = false;
            }
            _ => {
                return Err((
                    "unexpected token here (context: object)".to_owned(),
                    lexer.span(),
                ))
            }
        }
        awaits_comma = !awaits_key;
    }
    Err(("unmatched opening brace defined here".to_owned(), span))
}
/* ANCHOR_END: object */

fn main() {
    let filename = env::args().nth(1).expect("Expected file argument");
    let src = fs::read_to_string(&filename).expect("Failed to read file");

    let mut lexer = Token::lexer(src.as_str());

    match parse_value(&mut lexer) {
        Ok(value) => println!("{:#?}", value),
        Err((msg, span)) => {
            use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};

            let mut colors = ColorGenerator::new();

            let a = colors.next();

            Report::build(ReportKind::Error, &filename, 12)
                .with_message("Invalid JSON".to_string())
                .with_label(
                    Label::new((&filename, span))
                        .with_message(msg)
                        .with_color(a),
                )
                .finish()
                .eprint((&filename, Source::from(src)))
                .unwrap();
        }
    }
}
/* ANCHOR_END: all */
