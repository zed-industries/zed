use std::ops::RangeInclusive;

use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::opt;
use winnow::combinator::peek;
use winnow::combinator::preceded;
use winnow::combinator::repeat;
use winnow::combinator::trace;
use winnow::token::one_of;
use winnow::token::rest;
use winnow::token::take;

use crate::parser::prelude::*;
use crate::parser::trivia::from_utf8_unchecked;

// ;; Boolean

// boolean = true / false
#[allow(dead_code)] // directly define in `fn value`
pub(crate) fn boolean(input: &mut Input<'_>) -> ModalResult<bool> {
    trace("boolean", alt((true_, false_))).parse_next(input)
}

pub(crate) fn true_(input: &mut Input<'_>) -> ModalResult<bool> {
    (peek(TRUE[0]), cut_err(TRUE)).value(true).parse_next(input)
}
const TRUE: &[u8] = b"true";

pub(crate) fn false_(input: &mut Input<'_>) -> ModalResult<bool> {
    (peek(FALSE[0]), cut_err(FALSE))
        .value(false)
        .parse_next(input)
}
const FALSE: &[u8] = b"false";

// ;; Integer

// integer = dec-int / hex-int / oct-int / bin-int
pub(crate) fn integer(input: &mut Input<'_>) -> ModalResult<i64> {
    trace("integer",
    dispatch! {peek(opt::<_, &[u8], _, _>(take(2usize)));
        Some(b"0x") => cut_err(hex_int.try_map(|s| i64::from_str_radix(&s.replace('_', ""), 16))),
        Some(b"0o") => cut_err(oct_int.try_map(|s| i64::from_str_radix(&s.replace('_', ""), 8))),
        Some(b"0b") => cut_err(bin_int.try_map(|s| i64::from_str_radix(&s.replace('_', ""), 2))),
        _ => dec_int.and_then(cut_err(rest
            .try_map(|s: &str| s.replace('_', "").parse())))
    })
    .parse_next(input)
}

// dec-int = [ minus / plus ] unsigned-dec-int
// unsigned-dec-int = DIGIT / digit1-9 1*( DIGIT / underscore DIGIT )
pub(crate) fn dec_int<'i>(input: &mut Input<'i>) -> ModalResult<&'i str> {
    trace(
        "dec-int",
        (
            opt(one_of((b'+', b'-'))),
            alt((
                (
                    one_of(DIGIT1_9),
                    repeat(
                        0..,
                        alt((
                            digit.void(),
                            (
                                one_of(b'_'),
                                cut_err(digit).context(StrContext::Expected(
                                    StrContextValue::Description("digit"),
                                )),
                            )
                                .void(),
                        )),
                    )
                    .map(|()| ()),
                )
                    .void(),
                digit.void(),
            )),
        )
            .take()
            .map(|b: &[u8]| unsafe {
                from_utf8_unchecked(b, "`digit` and `_` filter out non-ASCII")
            })
            .context(StrContext::Label("integer")),
    )
    .parse_next(input)
}
const DIGIT1_9: RangeInclusive<u8> = b'1'..=b'9';

// hex-prefix = %x30.78               ; 0x
// hex-int = hex-prefix HEXDIG *( HEXDIG / underscore HEXDIG )
pub(crate) fn hex_int<'i>(input: &mut Input<'i>) -> ModalResult<&'i str> {
    trace(
        "hex-int",
        preceded(
            HEX_PREFIX,
            cut_err((
                hexdig,
                repeat(
                    0..,
                    alt((
                        hexdig.void(),
                        (
                            one_of(b'_'),
                            cut_err(hexdig).context(StrContext::Expected(
                                StrContextValue::Description("digit"),
                            )),
                        )
                            .void(),
                    )),
                )
                .map(|()| ()),
            ))
            .take(),
        )
        .map(|b| unsafe { from_utf8_unchecked(b, "`hexdig` and `_` filter out non-ASCII") })
        .context(StrContext::Label("hexadecimal integer")),
    )
    .parse_next(input)
}
const HEX_PREFIX: &[u8] = b"0x";

// oct-prefix = %x30.6F               ; 0o
// oct-int = oct-prefix digit0-7 *( digit0-7 / underscore digit0-7 )
pub(crate) fn oct_int<'i>(input: &mut Input<'i>) -> ModalResult<&'i str> {
    trace(
        "oct-int",
        preceded(
            OCT_PREFIX,
            cut_err((
                one_of(DIGIT0_7),
                repeat(
                    0..,
                    alt((
                        one_of(DIGIT0_7).void(),
                        (
                            one_of(b'_'),
                            cut_err(one_of(DIGIT0_7)).context(StrContext::Expected(
                                StrContextValue::Description("digit"),
                            )),
                        )
                            .void(),
                    )),
                )
                .map(|()| ()),
            ))
            .take(),
        )
        .map(|b| unsafe { from_utf8_unchecked(b, "`DIGIT0_7` and `_` filter out non-ASCII") })
        .context(StrContext::Label("octal integer")),
    )
    .parse_next(input)
}
const OCT_PREFIX: &[u8] = b"0o";
const DIGIT0_7: RangeInclusive<u8> = b'0'..=b'7';

// bin-prefix = %x30.62               ; 0b
// bin-int = bin-prefix digit0-1 *( digit0-1 / underscore digit0-1 )
pub(crate) fn bin_int<'i>(input: &mut Input<'i>) -> ModalResult<&'i str> {
    trace(
        "bin-int",
        preceded(
            BIN_PREFIX,
            cut_err((
                one_of(DIGIT0_1),
                repeat(
                    0..,
                    alt((
                        one_of(DIGIT0_1).void(),
                        (
                            one_of(b'_'),
                            cut_err(one_of(DIGIT0_1)).context(StrContext::Expected(
                                StrContextValue::Description("digit"),
                            )),
                        )
                            .void(),
                    )),
                )
                .map(|()| ()),
            ))
            .take(),
        )
        .map(|b| unsafe { from_utf8_unchecked(b, "`DIGIT0_1` and `_` filter out non-ASCII") })
        .context(StrContext::Label("binary integer")),
    )
    .parse_next(input)
}
const BIN_PREFIX: &[u8] = b"0b";
const DIGIT0_1: RangeInclusive<u8> = b'0'..=b'1';

// ;; Float

// float = float-int-part ( exp / frac [ exp ] )
// float =/ special-float
// float-int-part = dec-int
pub(crate) fn float(input: &mut Input<'_>) -> ModalResult<f64> {
    trace(
        "float",
        alt((
            float_.and_then(cut_err(
                rest.try_map(|s: &str| s.replace('_', "").parse())
                    .verify(|f: &f64| *f != f64::INFINITY),
            )),
            special_float,
        ))
        .context(StrContext::Label("floating-point number")),
    )
    .parse_next(input)
}

fn float_<'i>(input: &mut Input<'i>) -> ModalResult<&'i str> {
    (
        dec_int,
        alt((exp.void(), (frac.void(), opt(exp.void())).void())),
    )
        .take()
        .map(|b: &[u8]| unsafe {
            from_utf8_unchecked(
                b,
                "`dec_int`, `one_of`, `exp`, and `frac` filter out non-ASCII",
            )
        })
        .parse_next(input)
}

// frac = decimal-point zero-prefixable-int
// decimal-point = %x2E               ; .
fn frac<'i>(input: &mut Input<'i>) -> ModalResult<&'i str> {
    (
        b'.',
        cut_err(zero_prefixable_int)
            .context(StrContext::Expected(StrContextValue::Description("digit"))),
    )
        .take()
        .map(|b: &[u8]| unsafe {
            from_utf8_unchecked(
                b,
                "`.` and `parse_zero_prefixable_int` filter out non-ASCII",
            )
        })
        .parse_next(input)
}

// zero-prefixable-int = DIGIT *( DIGIT / underscore DIGIT )
fn zero_prefixable_int<'i>(input: &mut Input<'i>) -> ModalResult<&'i str> {
    (
        digit,
        repeat(
            0..,
            alt((
                digit.void(),
                (
                    one_of(b'_'),
                    cut_err(digit)
                        .context(StrContext::Expected(StrContextValue::Description("digit"))),
                )
                    .void(),
            )),
        )
        .map(|()| ()),
    )
        .take()
        .map(|b: &[u8]| unsafe { from_utf8_unchecked(b, "`digit` and `_` filter out non-ASCII") })
        .parse_next(input)
}

// exp = "e" float-exp-part
// float-exp-part = [ minus / plus ] zero-prefixable-int
fn exp<'i>(input: &mut Input<'i>) -> ModalResult<&'i str> {
    (
        one_of((b'e', b'E')),
        opt(one_of([b'+', b'-'])),
        cut_err(zero_prefixable_int),
    )
        .take()
        .map(|b: &[u8]| unsafe {
            from_utf8_unchecked(
                b,
                "`one_of` and `parse_zero_prefixable_int` filter out non-ASCII",
            )
        })
        .parse_next(input)
}

// special-float = [ minus / plus ] ( inf / nan )
fn special_float(input: &mut Input<'_>) -> ModalResult<f64> {
    (opt(one_of((b'+', b'-'))), alt((inf, nan)))
        .map(|(s, f)| match s {
            Some(b'+') | None => f,
            Some(b'-') => -f,
            _ => unreachable!("one_of should prevent this"),
        })
        .parse_next(input)
}
// inf = %x69.6e.66  ; inf
pub(crate) fn inf(input: &mut Input<'_>) -> ModalResult<f64> {
    INF.value(f64::INFINITY).parse_next(input)
}
const INF: &[u8] = b"inf";
// nan = %x6e.61.6e  ; nan
pub(crate) fn nan(input: &mut Input<'_>) -> ModalResult<f64> {
    NAN.value(f64::NAN.copysign(1.0)).parse_next(input)
}
const NAN: &[u8] = b"nan";

// DIGIT = %x30-39 ; 0-9
fn digit(input: &mut Input<'_>) -> ModalResult<u8> {
    one_of(DIGIT).parse_next(input)
}
const DIGIT: RangeInclusive<u8> = b'0'..=b'9';

// HEXDIG = DIGIT / "A" / "B" / "C" / "D" / "E" / "F"
fn hexdig(input: &mut Input<'_>) -> ModalResult<u8> {
    one_of(HEXDIG).parse_next(input)
}
pub(crate) const HEXDIG: (RangeInclusive<u8>, RangeInclusive<u8>, RangeInclusive<u8>) =
    (DIGIT, b'A'..=b'F', b'a'..=b'f');

#[cfg(test)]
#[cfg(feature = "parse")]
#[cfg(feature = "display")]
mod test {
    use super::*;

    #[test]
    fn integers() {
        let cases = [
            ("+99", 99),
            ("42", 42),
            ("0", 0),
            ("-17", -17),
            ("1_000", 1_000),
            ("5_349_221", 5_349_221),
            ("1_2_3_4_5", 1_2_3_4_5),
            ("0xF", 15),
            ("0o0_755", 493),
            ("0b1_0_1", 5),
            (&i64::MIN.to_string()[..], i64::MIN),
            (&i64::MAX.to_string()[..], i64::MAX),
        ];
        for &(input, expected) in &cases {
            dbg!(input);
            let parsed = integer.parse(new_input(input));
            assert_eq!(parsed, Ok(expected), "Parsing {input:?}");
        }

        let overflow = "1000000000000000000000000000000000";
        let parsed = integer.parse(new_input(overflow));
        assert!(parsed.is_err());
    }

    #[track_caller]
    fn assert_float_eq(actual: f64, expected: f64) {
        if expected.is_nan() {
            assert!(actual.is_nan());
            assert_eq!(expected.is_sign_positive(), actual.is_sign_positive());
        } else if expected.is_infinite() {
            assert!(actual.is_infinite());
            assert_eq!(expected.is_sign_positive(), actual.is_sign_positive());
        } else {
            dbg!(expected);
            dbg!(actual);
            assert!((expected - actual).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn floats() {
        let cases = [
            ("+1.0", 1.0),
            ("3.1419", 3.1419),
            ("-0.01", -0.01),
            ("5e+22", 5e+22),
            ("1e6", 1e6),
            ("-2E-2", -2E-2),
            ("6.626e-34", 6.626e-34),
            ("9_224_617.445_991_228_313", 9_224_617.445_991_227),
            ("-1.7976931348623157e+308", f64::MIN),
            ("1.7976931348623157e+308", f64::MAX),
            ("nan", f64::NAN.copysign(1.0)),
            ("+nan", f64::NAN.copysign(1.0)),
            ("-nan", f64::NAN.copysign(-1.0)),
            ("inf", f64::INFINITY),
            ("+inf", f64::INFINITY),
            ("-inf", f64::NEG_INFINITY),
            // ("1e+400", f64::INFINITY),
        ];
        for &(input, expected) in &cases {
            dbg!(input);
            let parsed = float.parse(new_input(input)).unwrap();
            assert_float_eq(parsed, expected);

            let overflow = "9e99999";
            let parsed = float.parse(new_input(overflow));
            assert!(parsed.is_err(), "{parsed:?}");
        }
    }
}
