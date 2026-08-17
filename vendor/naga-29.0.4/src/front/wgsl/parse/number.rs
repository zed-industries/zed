use alloc::format;

use crate::front::wgsl::error::NumberError;
use crate::front::wgsl::parse::directive::enable_extension::ImplementedEnableExtension;
use crate::front::wgsl::parse::lexer::Token;
use half::f16;

/// When using this type assume no Abstract Int/Float for now
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Number {
    /// Abstract Int (-2^63 ≤ i < 2^63)
    AbstractInt(i64),
    /// Abstract Float (IEEE-754 binary64)
    AbstractFloat(f64),
    /// Concrete i32
    I32(i32),
    /// Concrete u32
    U32(u32),
    /// Concrete i64
    I64(i64),
    /// Concrete u64
    U64(u64),
    /// Concrete f16
    F16(f16),
    /// Concrete f32
    F32(f32),
    /// Concrete f64
    F64(f64),
}

impl Number {
    pub(super) const fn requires_enable_extension(&self) -> Option<ImplementedEnableExtension> {
        match *self {
            Number::F16(_) => Some(ImplementedEnableExtension::F16),
            _ => None,
        }
    }
}

pub(in crate::front::wgsl) fn consume_number(input: &str) -> (Token<'_>, &str) {
    let (result, rest) = parse(input);
    (Token::Number(result), rest)
}

enum Kind {
    Int(IntKind),
    Float(FloatKind),
}

enum IntKind {
    I32,
    U32,
    I64,
    U64,
}

#[derive(Debug)]
enum FloatKind {
    F16,
    F32,
    F64,
}

// The following regexes (from the WGSL spec) will be matched:

// int_literal:
// | / 0                                                                [iu]?   /
// | / [1-9][0-9]*                                                      [iu]?   /
// | / 0[xX][0-9a-fA-F]+                                                [iu]?   /

// decimal_float_literal:
// | / 0                                                                [fh]    /
// | / [1-9][0-9]*                                                      [fh]    /
// | / [0-9]*               \.[0-9]+            ([eE][+-]?[0-9]+)?      [fh]?   /
// | / [0-9]+               \.[0-9]*            ([eE][+-]?[0-9]+)?      [fh]?   /
// | / [0-9]+                                    [eE][+-]?[0-9]+        [fh]?   /

// hex_float_literal:
// | / 0[xX][0-9a-fA-F]*    \.[0-9a-fA-F]+      ([pP][+-]?[0-9]+        [fh]?)? /
// | / 0[xX][0-9a-fA-F]+    \.[0-9a-fA-F]*      ([pP][+-]?[0-9]+        [fh]?)? /
// | / 0[xX][0-9a-fA-F]+                         [pP][+-]?[0-9]+        [fh]?   /

// You could visualize the regex below via https://debuggex.com to get a rough idea what `parse` is doing
// (?:0[xX](?:([0-9a-fA-F]+\.[0-9a-fA-F]*|[0-9a-fA-F]*\.[0-9a-fA-F]+)(?:([pP][+-]?[0-9]+)([fh]?))?|([0-9a-fA-F]+)([pP][+-]?[0-9]+)([fh]?)|([0-9a-fA-F]+)([iu]?))|((?:[0-9]+[eE][+-]?[0-9]+|(?:[0-9]+\.[0-9]*|[0-9]*\.[0-9]+)(?:[eE][+-]?[0-9]+)?))([fh]?)|((?:[0-9]|[1-9][0-9]+))([iufh]?))

// Leading signs are handled as unary operators.

fn parse(input: &str) -> (Result<Number, NumberError>, &str) {
    /// returns `true` and consumes `X` bytes from the given byte buffer
    /// if the given `X` nr of patterns are found at the start of the buffer
    macro_rules! consume {
        ($bytes:ident, $($pattern:pat),*) => {
            match $bytes {
                &[$($pattern),*, ref rest @ ..] => { $bytes = rest; true },
                _ => false,
            }
        };
    }

    /// consumes one byte from the given byte buffer
    /// if one of the given patterns are found at the start of the buffer
    /// returning the corresponding expr for the matched pattern
    macro_rules! consume_map {
        ($bytes:ident, [$( $($pattern:pat_param),* => $to:expr),* $(,)?]) => {
            match $bytes {
                $( &[ $($pattern),*, ref rest @ ..] => { $bytes = rest; Some($to) }, )*
                _ => None,
            }
        };
    }

    /// consumes all consecutive bytes matched by the `0-9` pattern from the given byte buffer
    /// returning the number of consumed bytes
    macro_rules! consume_dec_digits {
        ($bytes:ident) => {{
            let start_len = $bytes.len();
            while let &[b'0'..=b'9', ref rest @ ..] = $bytes {
                $bytes = rest;
            }
            start_len - $bytes.len()
        }};
    }

    /// consumes all consecutive bytes matched by the `0-9 | a-f | A-F` pattern from the given byte buffer
    /// returning the number of consumed bytes
    macro_rules! consume_hex_digits {
        ($bytes:ident) => {{
            let start_len = $bytes.len();
            while let &[b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F', ref rest @ ..] = $bytes {
                $bytes = rest;
            }
            start_len - $bytes.len()
        }};
    }

    macro_rules! consume_float_suffix {
        ($bytes:ident) => {
            consume_map!($bytes, [
                b'h' => FloatKind::F16,
                b'f' => FloatKind::F32,
                b'l', b'f' => FloatKind::F64,
            ])
        };
    }

    /// maps the given `&[u8]` (tail of the initial `input: &str`) to a `&str`
    macro_rules! rest_to_str {
        ($bytes:ident) => {
            &input[input.len() - $bytes.len()..]
        };
    }

    struct ExtractSubStr<'a>(&'a str);

    impl<'a> ExtractSubStr<'a> {
        /// given an `input` and a `start` (tail of the `input`)
        /// creates a new [`ExtractSubStr`](`Self`)
        fn start(input: &'a str, start: &'a [u8]) -> Self {
            let start = input.len() - start.len();
            Self(&input[start..])
        }
        /// given an `end` (tail of the initial `input`)
        /// returns a substring of `input`
        fn end(&self, end: &'a [u8]) -> &'a str {
            let end = self.0.len() - end.len();
            &self.0[..end]
        }
    }

    let mut bytes = input.as_bytes();

    let general_extract = ExtractSubStr::start(input, bytes);

    if consume!(bytes, b'0', b'x' | b'X') {
        let digits_extract = ExtractSubStr::start(input, bytes);

        let consumed = consume_hex_digits!(bytes);

        if consume!(bytes, b'.') {
            let consumed_after_period = consume_hex_digits!(bytes);

            if consumed + consumed_after_period == 0 {
                return (Err(NumberError::Invalid), rest_to_str!(bytes));
            }

            let significand = general_extract.end(bytes);

            if consume!(bytes, b'p' | b'P') {
                consume!(bytes, b'+' | b'-');
                let consumed = consume_dec_digits!(bytes);

                if consumed == 0 {
                    return (Err(NumberError::Invalid), rest_to_str!(bytes));
                }

                let number = general_extract.end(bytes);

                let kind = consume_float_suffix!(bytes);

                (parse_hex_float(number, kind), rest_to_str!(bytes))
            } else {
                (
                    parse_hex_float_missing_exponent(significand, None),
                    rest_to_str!(bytes),
                )
            }
        } else {
            if consumed == 0 {
                return (Err(NumberError::Invalid), rest_to_str!(bytes));
            }

            let significand = general_extract.end(bytes);
            let digits = digits_extract.end(bytes);

            let exp_extract = ExtractSubStr::start(input, bytes);

            if consume!(bytes, b'p' | b'P') {
                consume!(bytes, b'+' | b'-');
                let consumed = consume_dec_digits!(bytes);

                if consumed == 0 {
                    return (Err(NumberError::Invalid), rest_to_str!(bytes));
                }

                let exponent = exp_extract.end(bytes);

                let kind = consume_float_suffix!(bytes);

                (
                    parse_hex_float_missing_period(significand, exponent, kind),
                    rest_to_str!(bytes),
                )
            } else {
                let kind = consume_map!(bytes, [
                    b'i' => IntKind::I32,
                    b'u' => IntKind::U32,
                    b'l', b'i' => IntKind::I64,
                    b'l', b'u' => IntKind::U64,
                ]);

                (parse_hex_int(digits, kind), rest_to_str!(bytes))
            }
        }
    } else {
        let is_first_zero = bytes.first() == Some(&b'0');

        let consumed = consume_dec_digits!(bytes);

        if consume!(bytes, b'.') {
            let consumed_after_period = consume_dec_digits!(bytes);

            if consumed + consumed_after_period == 0 {
                return (Err(NumberError::Invalid), rest_to_str!(bytes));
            }

            if consume!(bytes, b'e' | b'E') {
                consume!(bytes, b'+' | b'-');
                let consumed = consume_dec_digits!(bytes);

                if consumed == 0 {
                    return (Err(NumberError::Invalid), rest_to_str!(bytes));
                }
            }

            let number = general_extract.end(bytes);

            let kind = consume_float_suffix!(bytes);

            (parse_dec_float(number, kind), rest_to_str!(bytes))
        } else {
            if consumed == 0 {
                return (Err(NumberError::Invalid), rest_to_str!(bytes));
            }

            if consume!(bytes, b'e' | b'E') {
                consume!(bytes, b'+' | b'-');
                let consumed = consume_dec_digits!(bytes);

                if consumed == 0 {
                    return (Err(NumberError::Invalid), rest_to_str!(bytes));
                }

                let number = general_extract.end(bytes);

                let kind = consume_float_suffix!(bytes);

                (parse_dec_float(number, kind), rest_to_str!(bytes))
            } else {
                // make sure the multi-digit numbers don't start with zero
                if consumed > 1 && is_first_zero {
                    return (Err(NumberError::Invalid), rest_to_str!(bytes));
                }

                let digits = general_extract.end(bytes);

                let kind = consume_map!(bytes, [
                    b'i' => Kind::Int(IntKind::I32),
                    b'u' => Kind::Int(IntKind::U32),
                    b'l', b'i' => Kind::Int(IntKind::I64),
                    b'l', b'u' => Kind::Int(IntKind::U64),
                    b'h' => Kind::Float(FloatKind::F16),
                    b'f' => Kind::Float(FloatKind::F32),
                    b'l', b'f' => Kind::Float(FloatKind::F64),
                ]);

                (parse_dec(digits, kind), rest_to_str!(bytes))
            }
        }
    }
}

fn parse_hex_float_missing_exponent(
    // format: 0[xX] ( [0-9a-fA-F]+\.[0-9a-fA-F]* | [0-9a-fA-F]*\.[0-9a-fA-F]+ )
    significand: &str,
    kind: Option<FloatKind>,
) -> Result<Number, NumberError> {
    let hexf_input = format!("{}{}", significand, "p0");
    parse_hex_float(&hexf_input, kind)
}

fn parse_hex_float_missing_period(
    // format: 0[xX] [0-9a-fA-F]+
    significand: &str,
    // format: [pP][+-]?[0-9]+
    exponent: &str,
    kind: Option<FloatKind>,
) -> Result<Number, NumberError> {
    let hexf_input = format!("{significand}.{exponent}");
    parse_hex_float(&hexf_input, kind)
}

fn parse_hex_int(
    // format: [0-9a-fA-F]+
    digits: &str,
    kind: Option<IntKind>,
) -> Result<Number, NumberError> {
    parse_int(digits, kind, 16)
}

fn parse_dec(
    // format: ( [0-9] | [1-9][0-9]+ )
    digits: &str,
    kind: Option<Kind>,
) -> Result<Number, NumberError> {
    match kind {
        None => parse_int(digits, None, 10),
        Some(Kind::Int(kind)) => parse_int(digits, Some(kind), 10),
        Some(Kind::Float(kind)) => parse_dec_float(digits, Some(kind)),
    }
}

// Float parsing notes

// The following chapters of IEEE 754-2019 are relevant:
//
// 7.4 Overflow (largest finite number is exceeded by what would have been
//     the rounded floating-point result were the exponent range unbounded)
//
// 7.5 Underflow (tiny non-zero result is detected;
//     for decimal formats tininess is detected before rounding when a non-zero result
//     computed as though both the exponent range and the precision were unbounded
//     would lie strictly between 2^−126)
//
// 7.6 Inexact (rounded result differs from what would have been computed
//     were both exponent range and precision unbounded)

// The WGSL spec requires us to error:
//   on overflow for decimal floating point literals
//   on overflow and inexact for hexadecimal floating point literals
// (underflow is not mentioned)

// hexf_parse errors on overflow, underflow, inexact
// rust std lib float from str handles overflow, underflow, inexact transparently (rounds and will not error)

// Therefore we only check for overflow manually for decimal floating point literals

// input format: 0[xX] ( [0-9a-fA-F]+\.[0-9a-fA-F]* | [0-9a-fA-F]*\.[0-9a-fA-F]+ ) [pP][+-]?[0-9]+
fn parse_hex_float(input: &str, kind: Option<FloatKind>) -> Result<Number, NumberError> {
    match kind {
        None => match hexf_parse::parse_hexf64(input, false) {
            Ok(num) => Ok(Number::AbstractFloat(num)),
            // can only be ParseHexfErrorKind::Inexact but we can't check since it's private
            _ => Err(NumberError::NotRepresentable),
        },
        // TODO: f16 is not supported by hexf_parse
        Some(FloatKind::F16) => Err(NumberError::NotRepresentable),
        Some(FloatKind::F32) => match hexf_parse::parse_hexf32(input, false) {
            Ok(num) => Ok(Number::F32(num)),
            // can only be ParseHexfErrorKind::Inexact but we can't check since it's private
            _ => Err(NumberError::NotRepresentable),
        },
        Some(FloatKind::F64) => match hexf_parse::parse_hexf64(input, false) {
            Ok(num) => Ok(Number::F64(num)),
            // can only be ParseHexfErrorKind::Inexact but we can't check since it's private
            _ => Err(NumberError::NotRepresentable),
        },
    }
}

// input format: ( [0-9]+\.[0-9]* | [0-9]*\.[0-9]+ ) ([eE][+-]?[0-9]+)?
//             | [0-9]+ [eE][+-]?[0-9]+
fn parse_dec_float(input: &str, kind: Option<FloatKind>) -> Result<Number, NumberError> {
    match kind {
        None => {
            let num = input.parse::<f64>().unwrap(); // will never fail
            num.is_finite()
                .then_some(Number::AbstractFloat(num))
                .ok_or(NumberError::NotRepresentable)
        }
        Some(FloatKind::F32) => {
            let num = input.parse::<f32>().unwrap(); // will never fail
            num.is_finite()
                .then_some(Number::F32(num))
                .ok_or(NumberError::NotRepresentable)
        }
        Some(FloatKind::F64) => {
            let num = input.parse::<f64>().unwrap(); // will never fail
            num.is_finite()
                .then_some(Number::F64(num))
                .ok_or(NumberError::NotRepresentable)
        }
        Some(FloatKind::F16) => {
            let num = input.parse::<f16>().unwrap(); // will never fail
            num.is_finite()
                .then_some(Number::F16(num))
                .ok_or(NumberError::NotRepresentable)
        }
    }
}

fn parse_int(input: &str, kind: Option<IntKind>, radix: u32) -> Result<Number, NumberError> {
    fn map_err(e: core::num::ParseIntError) -> NumberError {
        match *e.kind() {
            core::num::IntErrorKind::PosOverflow | core::num::IntErrorKind::NegOverflow => {
                NumberError::NotRepresentable
            }
            _ => unreachable!(),
        }
    }
    match kind {
        None => match i64::from_str_radix(input, radix) {
            Ok(num) => Ok(Number::AbstractInt(num)),
            Err(e) => Err(map_err(e)),
        },
        Some(IntKind::I32) => match i32::from_str_radix(input, radix) {
            Ok(num) => Ok(Number::I32(num)),
            Err(e) => Err(map_err(e)),
        },
        Some(IntKind::U32) => match u32::from_str_radix(input, radix) {
            Ok(num) => Ok(Number::U32(num)),
            Err(e) => Err(map_err(e)),
        },
        Some(IntKind::I64) => match i64::from_str_radix(input, radix) {
            Ok(num) => Ok(Number::I64(num)),
            Err(e) => Err(map_err(e)),
        },
        Some(IntKind::U64) => match u64::from_str_radix(input, radix) {
            Ok(num) => Ok(Number::U64(num)),
            Err(e) => Err(map_err(e)),
        },
    }
}
