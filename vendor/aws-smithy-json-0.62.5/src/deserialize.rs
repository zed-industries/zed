/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::deserialize::error::{DeserializeError as Error, DeserializeErrorKind as ErrorKind};
use aws_smithy_types::Number;
use ErrorKind::*;

pub mod error;
pub mod token;

pub use token::{EscapeError, EscapedStr, Offset, Token};

/// JSON token parser as a Rust iterator
///
/// This parser will parse and yield exactly one [`Token`] per iterator `next()` call.
/// Validation is done on the fly, so it is possible for it to parse an invalid JSON document
/// until it gets to the first [`Error`].
///
/// JSON string values are left escaped in the [`Token::ValueString`] as an [`EscapedStr`],
/// which is a new type around a slice of original `input` bytes so that the caller can decide
/// when to unescape and allocate into a [`String`].
///
/// The parser *will* accept multiple valid JSON values. For example, `b"null true"` will
/// yield `ValueNull` and `ValueTrue`. It is the responsibility of the caller to handle this for
/// their use-case.
pub fn json_token_iter(input: &[u8]) -> JsonTokenIterator<'_> {
    JsonTokenIterator {
        input,
        index: 0,
        state_stack: vec![State::Initial],
    }
}

/// Internal parser state for the iterator. Used to context between successive `next` calls.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    /// Entry point. Expecting any JSON value.
    Initial,
    /// Expecting the next token to be the *first* value in an array, or the end of the array.
    ArrayFirstValueOrEnd,
    /// Expecting the next token to the next value in an array, or the end of the array.
    ArrayNextValueOrEnd,
    /// Expecting the next token to be the *first* key in the object, or the end of the object.
    ObjectFirstKeyOrEnd,
    /// Expecting the next token to the next object key, or the end of the object.
    ObjectNextKeyOrEnd,
    /// Expecting the next token to be the value of a field in an object.
    ObjectFieldValue,
}

/// An iterator over a `&[u8]` that yields `Result<Token, Error>` with [Token] being JSON tokens.
/// Construct with [json_token_iter].
pub struct JsonTokenIterator<'a> {
    input: &'a [u8],
    index: usize,
    state_stack: Vec<State>,
}

impl<'a> JsonTokenIterator<'a> {
    /// Previews the next byte.
    fn peek_byte(&self) -> Option<u8> {
        if self.index >= self.input.len() {
            None
        } else {
            Some(self.input[self.index])
        }
    }

    /// Expects there to be another byte coming up, and previews it.
    /// If there isn't, an `UnexpectedEOS` error is returned.
    fn peek_expect(&self) -> Result<u8, Error> {
        self.peek_byte().ok_or_else(|| self.error(UnexpectedEos))
    }

    /// Advances to the next byte in the stream.
    fn advance(&mut self) {
        if self.index < self.input.len() {
            self.index += 1;
        }
    }

    /// Advances and returns the next byte in the stream.
    fn next_byte(&mut self) -> Option<u8> {
        let next = self.peek_byte();
        self.advance();
        next
    }

    /// Expects there to be another byte coming up, and returns it while advancing.
    /// If there isn't, an `UnexpectedEOS` error is returned.
    fn next_expect(&mut self) -> Result<u8, Error> {
        self.next_byte().ok_or_else(|| self.error(UnexpectedEos))
    }

    /// Creates an error at the given `offset` in the stream.
    fn error_at(&self, offset: usize, kind: ErrorKind) -> Error {
        Error::new(kind, Some(offset))
    }

    /// Creates an error at the current offset in the stream.
    fn error(&self, kind: ErrorKind) -> Error {
        self.error_at(self.index, kind)
    }

    /// Advances until it hits a non-whitespace character or the end of the slice.
    fn discard_whitespace(&mut self) {
        while let Some(byte) = self.peek_byte() {
            match byte {
                b' ' | b'\t' | b'\r' | b'\n' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    /// Returns the top of the state stack (current state).
    fn state(&self) -> State {
        self.state_stack[self.state_stack.len() - 1]
    }

    /// Replaces the top of the state stack with a new `state`.
    fn replace_state(&mut self, state: State) {
        self.state_stack.pop();
        self.state_stack.push(state);
    }

    /// Returns current offset
    fn offset(&self) -> Offset {
        Offset(self.index)
    }

    /// Discards the '{' character and pushes the `ObjectFirstKeyOrEnd` state.
    fn start_object(&mut self) -> Token<'a> {
        let offset = self.offset();
        let byte = self.next_byte();
        debug_assert_eq!(byte, Some(b'{'));
        self.state_stack.push(State::ObjectFirstKeyOrEnd);
        Token::StartObject { offset }
    }

    /// Discards the '}' character and pops the current state.
    fn end_object(&mut self) -> Token<'a> {
        let offset = self.offset();
        let (byte, state) = (self.next_byte(), self.state_stack.pop());
        debug_assert_eq!(byte, Some(b'}'));
        debug_assert!(
            state == Some(State::ObjectFirstKeyOrEnd) || state == Some(State::ObjectNextKeyOrEnd)
        );
        Token::EndObject { offset }
    }

    /// Discards the '[' character and pushes the `ArrayFirstValueOrEnd` state.
    fn start_array(&mut self) -> Token<'a> {
        let offset = self.offset();
        let byte = self.next_byte();
        debug_assert_eq!(byte, Some(b'['));
        self.state_stack.push(State::ArrayFirstValueOrEnd);
        Token::StartArray { offset }
    }

    /// Discards the ']' character and pops the current state.
    fn end_array(&mut self) -> Token<'a> {
        let offset = self.offset();
        let (byte, state) = (self.next_byte(), self.state_stack.pop());
        debug_assert_eq!(byte, Some(b']'));
        debug_assert!(
            state == Some(State::ArrayFirstValueOrEnd) || state == Some(State::ArrayNextValueOrEnd)
        );
        Token::EndArray { offset }
    }

    /// Reads a JSON string out of the stream.
    fn read_string(&mut self) -> Result<&'a str, Error> {
        // Skip the starting quote
        let quote_byte = self.next_byte();
        debug_assert_eq!(quote_byte, Some(b'\"'));

        // Read bytes until a non-escaped end-quote, unescaping sequences as needed on the fly
        let start = self.index;
        loop {
            match self.peek_expect()? {
                b'"' => {
                    let value = std::str::from_utf8(&self.input[start..self.index])
                        .map_err(|_| self.error(InvalidUtf8))?;
                    self.advance();
                    return Ok(value);
                }
                b'\\' => match self.next_expect()? {
                    b'\\' | b'/' | b'"' | b'b' | b'f' | b'n' | b'r' | b't' => self.advance(),
                    b'u' => {
                        if self.index + 4 > self.input.len() {
                            return Err(self.error_at(self.input.len(), UnexpectedEos));
                        }
                        self.index += 4;
                    }
                    byte => return Err(self.error(InvalidEscape(byte.into()))),
                },
                byte @ 0x00..=0x1F => return Err(self.error(UnexpectedControlCharacter(byte))),
                _ => self.advance(),
            }
        }
    }

    /// Expects the given literal to be next in the stream.
    fn expect_literal(&mut self, expected: &[u8]) -> Result<(), Error> {
        let (start, end) = (self.index, self.index + expected.len());
        if end > self.input.len() {
            return Err(self.error_at(self.input.len(), UnexpectedEos));
        }
        if expected != &self.input[start..end] {
            return Err(self.error_at(
                start,
                ExpectedLiteral(std::str::from_utf8(expected).unwrap().into()),
            ));
        }
        self.index = end;
        Ok(())
    }

    /// Expects a literal `null` next in the stream.
    fn expect_null(&mut self) -> Result<Token<'a>, Error> {
        let offset = self.offset();
        self.expect_literal(b"null")?;
        Ok(Token::ValueNull { offset })
    }

    /// Expects a boolean `true` / `false` to be next in the stream and returns its value.
    fn expect_bool(&mut self) -> Result<Token<'a>, Error> {
        let offset = self.offset();
        match self.peek_expect()? {
            b't' => {
                self.expect_literal(b"true")?;
                Ok(Token::ValueBool {
                    offset,
                    value: true,
                })
            }
            b'f' => {
                self.expect_literal(b"false")?;
                Ok(Token::ValueBool {
                    offset,
                    value: false,
                })
            }
            _ => unreachable!(
                "this function must only be called when the next character is 't' or 'f'"
            ),
        }
    }

    /// Advances passed the exponent part of a floating point number.
    fn skip_exponent(&mut self) {
        self.advance();
        match self.peek_byte() {
            Some(b'-') => self.advance(),
            Some(b'+') => self.advance(),
            _ => {}
        }
        while let Some(b'0'..=b'9') = self.peek_byte() {
            self.advance();
        }
    }

    /// Advances passed the decimal part of a floating point number.
    fn skip_decimal(&mut self) {
        self.advance();
        while let Some(byte) = self.peek_byte() {
            match byte {
                b'0'..=b'9' => self.advance(),
                b'e' | b'E' => self.skip_exponent(),
                _ => break,
            }
        }
    }

    /// Starting from the current location in the stream, this advances until
    /// it finds a character that doesn't look like its part of a number, and then
    /// returns `(start_index, end_index, negative, floating)`, with `start_index`
    /// and `end_index` representing the slice of the stream that is the number,
    /// `negative` whether or not it is a negative number, and `floating` whether or not
    /// the number contains a decimal point and/or an exponent.
    fn scan_number(&mut self) -> (usize, usize, bool, bool) {
        let start_index = self.index;
        let negative = if self.peek_byte() == Some(b'-') {
            self.advance();
            true
        } else {
            false
        };
        let mut floating = false;
        while let Some(byte) = self.peek_byte() {
            match byte {
                b'0'..=b'9' => self.advance(),
                b'.' => {
                    floating = true;
                    self.skip_decimal();
                }
                b'e' | b'E' => {
                    floating = true;
                    self.skip_exponent();
                }
                _ => break,
            }
        }
        (start_index, self.index, negative, floating)
    }

    /// Expects a number in the stream, and returns its value.
    fn expect_number(&mut self) -> Result<Token<'a>, Error> {
        let offset = self.offset();
        let (start, end, negative, floating) = self.scan_number();
        let number_slice = &self.input[start..end];

        // Unsafe: we examined every character in the range, and they are all number characters
        debug_assert!(std::str::from_utf8(number_slice).is_ok());
        let number_str = unsafe { std::str::from_utf8_unchecked(number_slice) };

        use std::str::FromStr;
        Ok(Token::ValueNumber {
            offset,
            value: if floating {
                Number::Float(
                    f64::from_str(number_str).map_err(|_| self.error_at(start, InvalidNumber))?,
                )
            } else if negative {
                // If the negative value overflows, then stuff it into an f64
                match u64::from_str(&number_str[1..]) {
                    Ok(positive) => {
                        // Check if the positive value fits in i64's negative range
                        if positive <= i64::MAX as u64 {
                            Number::NegInt(-(positive as i64))
                        } else if positive == (i64::MAX as u64) + 1 {
                            // Special case: i64::MIN
                            Number::NegInt(i64::MIN)
                        } else {
                            // Too large for i64, use f64
                            Number::Float(-(positive as f64))
                        }
                    }
                    Err(_) => {
                        // Number too large for u64, parse as f64 (may be infinity)
                        Number::Float(
                            f64::from_str(number_str)
                                .map_err(|_| self.error_at(start, InvalidNumber))?,
                        )
                    }
                }
            } else {
                // Try to parse as u64, fall back to f64 if too large
                match u64::from_str(number_str) {
                    Ok(n) => Number::PosInt(n),
                    Err(_) => {
                        // Number too large for u64, parse as f64 (may be infinity)
                        Number::Float(
                            f64::from_str(number_str)
                                .map_err(|_| self.error_at(start, InvalidNumber))?,
                        )
                    }
                }
            },
        })
    }

    /// Reads a value from the stream and returns the next token. For objects and arrays,
    /// the entire object or array will not be ready, but rather, a [Token::StartObject]/[Token::StartArray]
    /// will be returned.
    fn read_value(&mut self) -> Result<Token<'a>, Error> {
        self.discard_whitespace();
        let offset = self.offset();
        match self.peek_expect()? {
            b'{' => Ok(self.start_object()),
            b'[' => Ok(self.start_array()),
            b'"' => self.read_string().map(|s| Token::ValueString {
                offset,
                value: EscapedStr::new(s),
            }),
            byte => {
                let value = match byte {
                    b'n' => self.expect_null(),
                    b't' | b'f' => self.expect_bool(),
                    b'-' | (b'0'..=b'9') => self.expect_number(),
                    byte => Err(self.error(UnexpectedToken(
                        byte.into(),
                        "'{', '[', '\"', 'null', 'true', 'false', <number>",
                    ))),
                }?;
                // Verify there are no unexpected trailers on the end of the value
                if let Some(byte) = self.peek_byte() {
                    match byte {
                        b' ' | b'\t' | b'\r' | b'\n' | b'}' | b']' | b',' => {}
                        _ => {
                            return Err(self.error(UnexpectedToken(
                                byte.into(),
                                "<whitespace>, '}', ']', ','",
                            )))
                        }
                    }
                }
                Ok(value)
            }
        }
    }

    /// Handles the [State::ArrayFirstValueOrEnd] state.
    fn state_array_first_value_or_end(&mut self) -> Result<Token<'a>, Error> {
        match self.peek_expect()? {
            b']' => Ok(self.end_array()),
            _ => {
                self.replace_state(State::ArrayNextValueOrEnd);
                self.read_value()
            }
        }
    }

    /// Handles the [State::ArrayNextValueOrEnd] state.
    fn state_array_next_value_or_end(&mut self) -> Result<Token<'a>, Error> {
        match self.peek_expect()? {
            b']' => Ok(self.end_array()),
            b',' => {
                self.advance();
                self.read_value()
            }
            byte => Err(self.error(UnexpectedToken(byte.into(), "']', ','"))),
        }
    }

    /// Expects an object key.
    fn object_key(&mut self) -> Result<Token<'a>, Error> {
        let offset = self.offset();
        match self.peek_expect()? {
            b'"' => {
                self.replace_state(State::ObjectFieldValue);
                self.read_string().map(|s| Token::ObjectKey {
                    offset,
                    key: EscapedStr::new(s),
                })
            }
            byte => Err(self.error(UnexpectedToken(byte.into(), "'\"'"))),
        }
    }

    /// Handles the [State::ObjectFirstKeyOrEnd] state.
    fn state_object_first_key_or_end(&mut self) -> Result<Token<'a>, Error> {
        match self.peek_expect()? {
            b'}' => Ok(self.end_object()),
            _ => self.object_key(),
        }
    }

    /// Handles the [State::ObjectNextKeyOrEnd] state.
    fn state_object_next_key_or_end(&mut self) -> Result<Token<'a>, Error> {
        match self.peek_expect()? {
            b'}' => Ok(self.end_object()),
            b',' => {
                self.advance();
                self.discard_whitespace();
                self.object_key()
            }
            byte => Err(self.error(UnexpectedToken(byte.into(), "'}', ','"))),
        }
    }

    /// Handles the [State::ObjectFieldValue] state.
    fn state_object_field_value(&mut self) -> Result<Token<'a>, Error> {
        match self.peek_expect()? {
            b':' => {
                self.advance();
                self.replace_state(State::ObjectNextKeyOrEnd);
                self.read_value()
            }
            byte => Err(self.error(UnexpectedToken(byte.into(), "':'"))),
        }
    }
}

impl<'a> Iterator for JsonTokenIterator<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        debug_assert!(self.index <= self.input.len());
        if self.index == self.input.len() {
            return None;
        }

        self.discard_whitespace();
        let result = match self.state() {
            State::Initial => self.peek_byte().map(|_| self.read_value()),
            State::ArrayFirstValueOrEnd => Some(self.state_array_first_value_or_end()),
            State::ArrayNextValueOrEnd => Some(self.state_array_next_value_or_end()),
            State::ObjectFirstKeyOrEnd => Some(self.state_object_first_key_or_end()),
            State::ObjectNextKeyOrEnd => Some(self.state_object_next_key_or_end()),
            State::ObjectFieldValue => Some(self.state_object_field_value()),
        };
        // Invalidate the stream if we encountered an error
        if result.as_ref().map(|r| r.is_err()).unwrap_or(false) {
            self.index = self.input.len();
        }
        result
    }
}

fn must_not_be_finite(f: f64) -> Result<f64, ()> {
    if !f.is_finite() {
        Ok(f)
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use crate::deserialize::error::{DeserializeError as Error, DeserializeErrorKind as ErrorKind};
    use crate::deserialize::token::expect_number_as_string_or_null;
    use crate::deserialize::token::test::{
        end_array, end_object, object_key, start_array, start_object, value_bool, value_null,
        value_number, value_string,
    };
    use crate::deserialize::{json_token_iter, EscapedStr, Token};
    use aws_smithy_types::Number;
    use proptest::prelude::*;

    #[track_caller]
    fn expect_token(
        expected: Option<Result<Token<'_>, Error>>,
        actual: Option<Result<Token<'_>, Error>>,
    ) {
        let (expected, actual) = (
            expected.transpose().expect("err in expected"),
            actual.transpose().expect("err in actual"),
        );
        assert_eq!(expected, actual);
    }

    macro_rules! expect_err {
        ($kind:pat, $offset:expr, $value:expr) => {
            let err: Error = $value.transpose().err().expect("expected error");
            assert!(matches!(err.kind, $kind));
            assert_eq!($offset, err.offset);
        };
    }

    #[test]
    fn test_empty() {
        assert!(json_token_iter(b"").next().is_none());
        assert!(json_token_iter(b" ").next().is_none());
        assert!(json_token_iter(b"\t").next().is_none());
    }

    #[test]
    fn test_empty_string() {
        let mut iter = json_token_iter(b"\"\"");
        expect_token(value_string(0, ""), iter.next());
        expect_token(None, iter.next());

        let mut iter = json_token_iter(b" \r\n\t \"\"  ");
        expect_token(value_string(5, ""), iter.next());
        expect_token(None, iter.next());
    }

    #[test]
    fn test_empty_array() {
        let mut iter = json_token_iter(b"[]");
        expect_token(start_array(0), iter.next());
        expect_token(end_array(1), iter.next());
        expect_token(None, iter.next());
    }

    #[test]
    fn test_empty_object() {
        let mut iter = json_token_iter(b"{}");
        expect_token(start_object(0), iter.next());
        expect_token(end_object(1), iter.next());
        expect_token(None, iter.next());
    }

    #[test]
    fn test_null() {
        expect_token(value_null(1), json_token_iter(b" null ").next());

        let mut iter = json_token_iter(b"[null, null,null]");
        expect_token(start_array(0), iter.next());
        expect_token(value_null(1), iter.next());
        expect_token(value_null(7), iter.next());
        expect_token(value_null(12), iter.next());
        expect_token(end_array(16), iter.next());
        expect_token(None, iter.next());

        assert!(json_token_iter(b"n").next().unwrap().is_err());
        assert!(json_token_iter(b"nul").next().unwrap().is_err());
        assert!(json_token_iter(b"nulll").next().unwrap().is_err());
    }

    #[test]
    fn test_bools() {
        assert!(json_token_iter(b"tru").next().unwrap().is_err());
        assert!(json_token_iter(b"truee").next().unwrap().is_err());
        assert!(json_token_iter(b"f").next().unwrap().is_err());
        assert!(json_token_iter(b"falsee").next().unwrap().is_err());
        expect_token(value_bool(1, true), json_token_iter(b" true ").next());
        expect_token(value_bool(0, false), json_token_iter(b"false").next());

        let mut iter = json_token_iter(b"[true,false]");
        expect_token(start_array(0), iter.next());
        expect_token(value_bool(1, true), iter.next());
        expect_token(value_bool(6, false), iter.next());
        expect_token(end_array(11), iter.next());
        expect_token(None, iter.next());
    }

    proptest! {
        #[test]
        fn string_prop_test(input in ".*") {
            let json: String = serde_json::to_string(&input).unwrap();
            let mut iter = json_token_iter(json.as_bytes());
            expect_token(value_string(0, &json[1..(json.len() - 1)]), iter.next());
            expect_token(None, iter.next());
        }

        #[test]
        fn integer_prop_test(input: i64) {
            let json = serde_json::to_string(&input).unwrap();
            let mut iter = json_token_iter(json.as_bytes());
            let expected = if input < 0 {
                Number::NegInt(input)
            } else {
                Number::PosInt(input as u64)
            };
            expect_token(value_number(0, expected), iter.next());
            expect_token(None, iter.next());
        }

        #[test]
        fn float_prop_test(input: f64) {
            let json = serde_json::to_string(&input).unwrap();
            let mut iter = json_token_iter(json.as_bytes());
            expect_token(value_number(0, Number::Float(input)), iter.next());
            expect_token(None, iter.next());
        }
    }

    #[test]
    fn valid_numbers() {
        let expect = |number, input| {
            expect_token(value_number(0, number), json_token_iter(input).next());
        };
        expect(Number::Float(0.0), b"0.");
        expect(Number::Float(0.0), b"0e0");
        expect(Number::Float(0.0), b"0E0");
        expect(Number::Float(10.0), b"1E1");
        expect(Number::Float(10.0), b"1E+1");
        expect(Number::Float(100.0), b"1e+2");

        expect(Number::NegInt(-50000), b"-50000");
        expect(
            Number::Float(-18446744073709551615.0),
            b"-18446744073709551615",
        );
    }

    #[test]
    fn out_of_range_floats_produce_infinity() {
        // Values exceeding f64::MAX should tokenize as infinity
        // The consumer layer (token.rs) will convert to NaN for BigInteger/BigDecimal
        let expect_infinity = |input, should_be_positive| {
            let token = json_token_iter(input).next().unwrap().unwrap();
            if let Token::ValueNumber {
                value: Number::Float(f),
                ..
            } = token
            {
                assert!(
                    f.is_infinite(),
                    "Expected infinity for out-of-range value, got {}",
                    f
                );
                if should_be_positive {
                    assert!(f.is_sign_positive(), "Expected positive infinity");
                } else {
                    assert!(f.is_sign_negative(), "Expected negative infinity");
                }
            } else {
                panic!("Expected Float token, got {:?}", token);
            }
        };

        // Values > f64::MAX
        expect_infinity(b"1.8e308", true);
        expect_infinity(b"9.9e999", true);

        // Negative values < -f64::MAX
        expect_infinity(b"-1.8e308", false);
        expect_infinity(b"-9.9e999", false);
    }

    // These cases actually shouldn't parse according to the spec, but it's easier
    // to be lenient on these, and it doesn't really impact the SDK use-case.
    #[test]
    fn invalid_numbers_we_are_intentionally_accepting() {
        let expect = |number, input| {
            expect_token(value_number(0, number), json_token_iter(input).next());
        };

        expect(Number::NegInt(-1), b"-01");
        expect(Number::Float(-2.0), b"-2.");
        expect(Number::Float(0.0), b"0.e1");
        expect(Number::Float(0.002), b"2.e-3");
        expect(Number::Float(2000.0), b"2.e3");
        expect(Number::NegInt(-12), b"-012");
        expect(Number::Float(-0.123), b"-.123");
        expect(Number::Float(1.0), b"1.");
        expect(Number::PosInt(12), b"012");
    }

    #[test]
    fn invalid_numbers() {
        macro_rules! unexpected_token {
            ($input:expr, $token:pat, $offset:expr, $msg:pat) => {
                let tokens: Vec<Result<Token<'_>, Error>> = json_token_iter($input).collect();
                assert_eq!(1, tokens.len());
                expect_err!(
                    ErrorKind::UnexpectedToken($token, $msg),
                    Some($offset),
                    tokens.into_iter().next()
                );
            };
        }

        let invalid_number = |input, offset| {
            let tokens: Vec<Result<Token<'_>, Error>> = json_token_iter(input).collect();
            assert_eq!(1, tokens.len());
            expect_err!(
                ErrorKind::InvalidNumber,
                Some(offset),
                tokens.into_iter().next()
            );
        };

        unexpected_token!(
            b".",
            '.',
            0,
            "'{', '[', '\"', 'null', 'true', 'false', <number>"
        );
        unexpected_token!(
            b".0",
            '.',
            0,
            "'{', '[', '\"', 'null', 'true', 'false', <number>"
        );
        unexpected_token!(b"0-05", '-', 1, "<whitespace>, '}', ']', ','");
        unexpected_token!(b"0x05", 'x', 1, "<whitespace>, '}', ']', ','");
        unexpected_token!(b"123.invalid", 'i', 4, "<whitespace>, '}', ']', ','");
        unexpected_token!(b"123invalid", 'i', 3, "<whitespace>, '}', ']', ','");
        unexpected_token!(
            b"asdf",
            'a',
            0,
            "'{', '[', '\"', 'null', 'true', 'false', <number>"
        );

        invalid_number(b"-a", 0);
        invalid_number(b"1e", 0);
        invalid_number(b"1e-", 0);

        // Number parsing fails before it even looks at the trailer because of invalid exponent
        invalid_number(b"123.0Einvalid", 0);
    }

    #[test]
    fn test_unclosed_array() {
        let mut iter = json_token_iter(br#" [null "#);
        expect_token(start_array(1), iter.next());
        expect_token(value_null(2), iter.next());
        expect_err!(ErrorKind::UnexpectedEos, Some(7), iter.next());
    }

    #[test]
    fn test_array_with_items() {
        let mut iter = json_token_iter(b"[[], {}, \"test\"]");
        expect_token(start_array(0), iter.next());
        expect_token(start_array(1), iter.next());
        expect_token(end_array(2), iter.next());
        expect_token(start_object(5), iter.next());
        expect_token(end_object(6), iter.next());
        expect_token(value_string(9, "test"), iter.next());
        expect_token(end_array(15), iter.next());
        expect_token(None, iter.next());
    }

    #[test]
    fn test_object_with_items() {
        let mut tokens = json_token_iter(
            br#"{ "some_int": 5,
                  "some_float": 5.2,
                  "some_negative": -5,
                  "some_negative_float": -2.4,
                  "some_string": "test",
                  "some_struct": { "nested": "asdf" },
                  "some_array": ["one", "two"] }"#,
        );
        expect_token(start_object(0), tokens.next());
        expect_token(object_key(2, "some_int"), tokens.next());
        expect_token(value_number(14, Number::PosInt(5)), tokens.next());
        expect_token(object_key(35, "some_float"), tokens.next());
        expect_token(value_number(49, Number::Float(5.2)), tokens.next());
        expect_token(object_key(72, "some_negative"), tokens.next());
        expect_token(value_number(89, Number::NegInt(-5)), tokens.next());
        expect_token(object_key(111, "some_negative_float"), tokens.next());
        expect_token(value_number(134, Number::Float(-2.4)), tokens.next());
        expect_token(object_key(158, "some_string"), tokens.next());
        expect_token(value_string(173, "test"), tokens.next());
        expect_token(object_key(199, "some_struct"), tokens.next());
        expect_token(start_object(214), tokens.next());
        expect_token(object_key(216, "nested"), tokens.next());
        expect_token(value_string(226, "asdf"), tokens.next());
        expect_token(end_object(233), tokens.next());
        expect_token(object_key(254, "some_array"), tokens.next());
        expect_token(start_array(268), tokens.next());
        expect_token(value_string(269, "one"), tokens.next());
        expect_token(value_string(276, "two"), tokens.next());
        expect_token(end_array(281), tokens.next());
        expect_token(end_object(283), tokens.next());
        expect_token(None, tokens.next());
    }

    #[test]
    fn test_object_trailing_comma() {
        let mut iter = json_token_iter(br#" { "test": "trailing", } "#);
        expect_token(start_object(1), iter.next());
        expect_token(object_key(3, "test"), iter.next());
        expect_token(value_string(11, "trailing"), iter.next());
        expect_err!(
            ErrorKind::UnexpectedToken('}', "'\"'"),
            Some(23),
            iter.next()
        );
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_object_no_colon() {
        let mut iter = json_token_iter(br#" {"test" "#);
        expect_token(start_object(1), iter.next());
        expect_token(object_key(2, "test"), iter.next());
        expect_err!(ErrorKind::UnexpectedEos, Some(9), iter.next());
        expect_token(None, iter.next());
    }

    #[test]
    fn unescaped_ctrl_characters() {
        assert!(json_token_iter(b"\"test\x00test\"")
            .next()
            .unwrap()
            .is_err());
        assert!(json_token_iter(b"\"test\ntest\"").next().unwrap().is_err());
        assert!(json_token_iter(b"\"test\ttest\"").next().unwrap().is_err());
    }

    #[test]
    fn escaped_str() {
        let escaped = EscapedStr::new("foo\\nbar");
        assert_eq!("foo\\nbar", escaped.as_escaped_str());
        assert_eq!("foo\nbar", escaped.to_unescaped().unwrap());
    }

    #[test]
    fn test_integer_overflow_to_float() {
        // Positive integer larger than u64::MAX should parse as Float
        let input = b"18450000000000000000";
        let mut iter = json_token_iter(input);
        match iter.next() {
            Some(Ok(Token::ValueNumber {
                value: Number::Float(f),
                ..
            })) => {
                assert!(f.is_finite());
                assert!(f > 0.0);
            }
            other => panic!("Expected Float token, got {:?}", other),
        }

        // Negative integer smaller than i64::MIN should parse as Float
        let input = b"-9223372036854775809";
        let mut iter = json_token_iter(input);
        match iter.next() {
            Some(Ok(Token::ValueNumber {
                value: Number::Float(f),
                ..
            })) => {
                assert!(f.is_finite());
                assert!(f < 0.0);
            }
            other => panic!("Expected Float token, got {:?}", other),
        }

        // Extremely large number should parse as infinity
        let large_num = b"100000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000";
        let mut iter = json_token_iter(large_num);
        match iter.next() {
            Some(Ok(Token::ValueNumber {
                value: Number::Float(f),
                ..
            })) => {
                assert_eq!(f, f64::INFINITY);
            }
            other => panic!("Expected Float(infinity) token, got {:?}", other),
        }
    }

    #[test]
    fn test_integer_within_range() {
        // Numbers that fit in u64/i64 should still parse as PosInt/NegInt
        let input = b"9007199254740993";
        let mut iter = json_token_iter(input);
        match iter.next() {
            Some(Ok(Token::ValueNumber {
                value: Number::PosInt(n),
                ..
            })) => {
                assert_eq!(n, 9007199254740993);
            }
            other => panic!("Expected PosInt token, got {:?}", other),
        }

        let input = b"-9223372036854775808";
        let mut iter = json_token_iter(input);
        match iter.next() {
            Some(Ok(Token::ValueNumber {
                value: Number::NegInt(n),
                ..
            })) => {
                assert_eq!(n, i64::MIN);
            }
            other => panic!("Expected NegInt token, got {:?}", other),
        }
    }

    #[test]
    fn test_integer_boundaries() {
        // Zero
        let input = b"0";
        let mut iter = json_token_iter(input);
        match iter.next() {
            Some(Ok(Token::ValueNumber {
                value: Number::PosInt(0),
                ..
            })) => {}
            other => panic!("Expected PosInt(0), got {:?}", other),
        }

        // Regular negative number
        let input = b"-123";
        let mut iter = json_token_iter(input);
        match iter.next() {
            Some(Ok(Token::ValueNumber {
                value: Number::NegInt(-123),
                ..
            })) => {}
            other => panic!("Expected NegInt(-123), got {:?}", other),
        }

        // i64::MAX (largest positive i64)
        let input = b"9223372036854775807";
        let mut iter = json_token_iter(input);
        match iter.next() {
            Some(Ok(Token::ValueNumber {
                value: Number::PosInt(n),
                ..
            })) => {
                assert_eq!(n, i64::MAX as u64);
            }
            other => panic!("Expected PosInt(i64::MAX), got {:?}", other),
        }

        // i64::MIN + 1 (edge case for negative range check)
        let input = b"-9223372036854775807";
        let mut iter = json_token_iter(input);
        match iter.next() {
            Some(Ok(Token::ValueNumber {
                value: Number::NegInt(n),
                ..
            })) => {
                assert_eq!(n, i64::MIN + 1);
            }
            other => panic!("Expected NegInt(i64::MIN + 1), got {:?}", other),
        }

        // u64::MAX (fits in u64, should be PosInt)
        let input = b"18446744073709551615";
        let mut iter = json_token_iter(input);
        match iter.next() {
            Some(Ok(Token::ValueNumber {
                value: Number::PosInt(n),
                ..
            })) => {
                assert_eq!(n, u64::MAX);
            }
            other => panic!("Expected PosInt(u64::MAX), got {:?}", other),
        }
    }

    #[cfg(test)]
    mod proptest_tests {
        use super::*;

        proptest! {
            #[test]
            fn positive_integers_within_u64_parse_as_posint(n in 0u64..=u64::MAX) {
                let input = n.to_string();
                let input_bytes = input.as_bytes();
                let mut iter = json_token_iter(input_bytes);

                match iter.next() {
                    Some(Ok(Token::ValueNumber { value: Number::PosInt(parsed), .. })) => {
                        prop_assert_eq!(parsed, n);
                    }
                    other => {
                        return Err(proptest::test_runner::TestCaseError::fail(
                            format!("Expected PosInt({}), got {:?}", n, other)
                        ));
                    }
                }
            }

            #[test]
            fn negative_integers_within_i64_parse_as_negint(n in i64::MIN..=i64::MAX) {
                if n >= 0 {
                    return Ok(());
                }

                let input = n.to_string();
                let input_bytes = input.as_bytes();
                let mut iter = json_token_iter(input_bytes);

                match iter.next() {
                    Some(Ok(Token::ValueNumber { value: Number::NegInt(parsed), .. })) => {
                        prop_assert_eq!(parsed, n);
                    }
                    other => {
                        return Err(proptest::test_runner::TestCaseError::fail(
                            format!("Expected NegInt({}), got {:?}", n, other)
                        ));
                    }
                }
            }

            #[test]
            fn large_integers_overflow_to_float(
                // u64::MAX = 18_446_744_073_709_551_615 (20 digits)
                // Generate numbers with 21+ digits to guarantee overflow
                num_str in "1[0-9]{20,49}"
            ) {
                let input_bytes = num_str.as_bytes();
                let mut iter = json_token_iter(input_bytes);

                match iter.next() {
                    Some(Ok(Token::ValueNumber { value: Number::Float(f), .. })) => {
                        prop_assert!(f.is_finite());
                        prop_assert!(f > 0.0);
                    }
                    other => {
                        return Err(proptest::test_runner::TestCaseError::fail(
                            format!("Expected Float for large number, got {:?}", other)
                        ));
                    }
                }

                // Validate expect_number_as_string_or_null extracts the correct string
                let mut iter = json_token_iter(input_bytes);
                let result = expect_number_as_string_or_null(iter.next(), input_bytes)?;
                prop_assert_eq!(result, Some(num_str.as_str()));
            }
        }
    }
}
