use std::{fmt, str::FromStr};

use crate::{
    err::{perr, ParseErrorKind::*},
    parse::{check_suffix, end_dec_digits, first_byte_or_empty},
    Buffer, ParseError,
};



/// A floating point literal, e.g. `3.14`, `8.`, `135e12`, or `1.956e2f64`.
///
/// This kind of literal has several forms, but generally consists of a main
/// number part, an optional exponent and an optional type suffix. See
/// [the reference][ref] for more information.
///
/// A leading minus sign `-` is not part of the literal grammar! `-3.14` are two
/// tokens in the Rust grammar. Further, `27` and `27f32` are both not float,
/// but integer literals! Consequently `FloatLit::parse` will reject them.
///
///
/// [ref]: https://doc.rust-lang.org/reference/tokens.html#floating-point-literals
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatLit<B: Buffer> {
    /// The whole raw input. The `usize` fields in this struct partition this
    /// string. Always true: `end_integer_part <= end_fractional_part`.
    ///
    /// ```text
    ///    12_3.4_56e789f32
    ///        ╷    ╷   ╷
    ///        |    |   └ end_number_part = 13
    ///        |    └ end_fractional_part = 9
    ///        └ end_integer_part = 4
    ///
    ///    246.
    ///       ╷╷
    ///       |└ end_fractional_part = end_number_part = 4
    ///       └ end_integer_part = 3
    ///
    ///    1234e89
    ///        ╷  ╷
    ///        |  └ end_number_part = 7
    ///        └ end_integer_part = end_fractional_part = 4
    /// ```
    raw: B,

    /// The first index not part of the integer part anymore. Since the integer
    /// part is at the start, this is also the length of that part.
    end_integer_part: usize,

    /// The first index after the fractional part.
    end_fractional_part: usize,

    /// The first index after the whole number part (everything except type suffix).
    end_number_part: usize,
}

impl<B: Buffer> FloatLit<B> {
    /// Parses the input as a floating point literal. Returns an error if the
    /// input is invalid or represents a different kind of literal. Will also
    /// reject decimal integer literals like `23` or `17f32`, in accordance
    /// with the spec.
    pub fn parse(s: B) -> Result<Self, ParseError> {
        match first_byte_or_empty(&s)? {
            b'0'..=b'9' => {
                // TODO: simplify once RFC 2528 is stabilized
                let FloatLit {
                    end_integer_part,
                    end_fractional_part,
                    end_number_part,
                    ..
                } = parse_impl(&s)?;

                Ok(Self { raw: s, end_integer_part, end_fractional_part, end_number_part })
            }
            _ => Err(perr(0, DoesNotStartWithDigit)),
        }
    }

    /// Returns the number part (including integer part, fractional part and
    /// exponent), but without the suffix. If you want an actual floating
    /// point value, you need to parse this string, e.g. with `f32::from_str`
    /// or an external crate.
    pub fn number_part(&self) -> &str {
        &(*self.raw)[..self.end_number_part]
    }

    /// Returns the non-empty integer part of this literal.
    pub fn integer_part(&self) -> &str {
        &(*self.raw)[..self.end_integer_part]
    }

    /// Returns the optional fractional part of this literal. Does not include
    /// the period. If a period exists in the input, `Some` is returned, `None`
    /// otherwise. Note that `Some("")` might be returned, e.g. for `3.`.
    pub fn fractional_part(&self) -> Option<&str> {
        if self.end_integer_part == self.end_fractional_part {
            None
        } else {
            Some(&(*self.raw)[self.end_integer_part + 1..self.end_fractional_part])
        }
    }

    /// Optional exponent part. Might be empty if there was no exponent part in
    /// the input. Includes the `e` or `E` at the beginning.
    pub fn exponent_part(&self) -> &str {
        &(*self.raw)[self.end_fractional_part..self.end_number_part]
    }

    /// The optional suffix. Returns `""` if the suffix is empty/does not exist.
    pub fn suffix(&self) -> &str {
        &(*self.raw)[self.end_number_part..]
    }

    /// Returns the raw input that was passed to `parse`.
    pub fn raw_input(&self) -> &str {
        &self.raw
    }

    /// Returns the raw input that was passed to `parse`, potentially owned.
    pub fn into_raw_input(self) -> B {
        self.raw
    }
}

impl FloatLit<&str> {
    /// Makes a copy of the underlying buffer and returns the owned version of
    /// `Self`.
    pub fn to_owned(&self) -> FloatLit<String> {
        FloatLit {
            raw: self.raw.to_owned(),
            end_integer_part: self.end_integer_part,
            end_fractional_part: self.end_fractional_part,
            end_number_part: self.end_number_part,
        }
    }
}

impl<B: Buffer> fmt::Display for FloatLit<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &*self.raw)
    }
}

/// Precondition: first byte of string has to be in `b'0'..=b'9'`.
#[inline(never)]
pub(crate) fn parse_impl(input: &str) -> Result<FloatLit<&str>, ParseError> {
    // Integer part.
    let end_integer_part = end_dec_digits(input.as_bytes());
    let rest = &input[end_integer_part..];


    // Fractional part.
    let end_fractional_part = if rest.as_bytes().get(0) == Some(&b'.') {
        // The fractional part must not start with `_`.
        if rest.as_bytes().get(1) == Some(&b'_') {
            return Err(perr(end_integer_part + 1, UnexpectedChar));
        }

        end_dec_digits(rest[1..].as_bytes()) + 1 + end_integer_part
    } else {
        end_integer_part
    };
    let rest = &input[end_fractional_part..];

    // If we have a period that is not followed by decimal digits, the
    // literal must end now.
    if end_integer_part + 1 == end_fractional_part && !rest.is_empty() {
        return Err(perr(end_integer_part + 1, UnexpectedChar));
    }

    // Optional exponent.
    let end_number_part = if rest.starts_with('e') || rest.starts_with('E') {
        // Strip single - or + sign at the beginning.
        let exp_number_start = match rest.as_bytes().get(1) {
            Some(b'-') | Some(b'+') => 2,
            _ => 1,
        };

        // Find end of exponent and make sure there is at least one digit.
        let end_exponent = end_dec_digits(rest[exp_number_start..].as_bytes()) + exp_number_start;
        if !rest[exp_number_start..end_exponent].bytes().any(|b| matches!(b, b'0'..=b'9')) {
            return Err(perr(
                end_fractional_part..end_fractional_part + end_exponent,
                NoExponentDigits,
            ));
        }

        end_exponent + end_fractional_part
    } else {
        end_fractional_part
    };

    // Make sure the suffix is valid.
    let suffix = &input[end_number_part..];
    check_suffix(suffix).map_err(|kind| perr(end_number_part..input.len(), kind))?;

    // A float literal needs either a fractional or exponent part, otherwise its
    // an integer literal.
    if end_integer_part == end_number_part {
        return Err(perr(None, UnexpectedIntegerLit));
    }

    Ok(FloatLit {
        raw: input,
        end_integer_part,
        end_fractional_part,
        end_number_part,
    })
}


/// All possible float type suffixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FloatType {
    F32,
    F64,
}

impl FloatType {
    /// Returns the type corresponding to the given suffix (e.g. `"f32"` is
    /// mapped to `Self::F32`). If the suffix is not a valid float type, `None`
    /// is returned.
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        match suffix {
            "f32" => Some(FloatType::F32),
            "f64" => Some(FloatType::F64),
            _ => None,
        }
    }

    /// Returns the suffix for this type, e.g. `"f32"` for `Self::F32`.
    pub fn suffix(self) -> &'static str {
        match self {
            Self::F32 => "f32",
            Self::F64 => "f64",
        }
    }
}

impl FromStr for FloatType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_suffix(s).ok_or(())
    }
}

impl fmt::Display for FloatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.suffix().fmt(f)
    }
}


#[cfg(test)]
mod tests;
