//! Parses hexadecimal float literals.
//! There are two functions `parse_hexf32` and `parse_hexf64` provided for each type.
//!
//! ```rust
//! use hexf_parse::*;
//! assert_eq!(parse_hexf32("0x1.99999ap-4", false), Ok(0.1f32));
//! assert_eq!(parse_hexf64("0x1.999999999999ap-4", false), Ok(0.1f64));
//! ```
//!
//! An additional `bool` parameter can be set to true if you want to allow underscores.
//!
//! ```rust
//! use hexf_parse::*;
//! assert!(parse_hexf64("0x0.1_7p8", false).is_err());
//! assert_eq!(parse_hexf64("0x0.1_7p8", true), Ok(23.0f64));
//! ```
//!
//! The error is reported via an opaque `ParseHexfError` type.

use std::{f32, f64, fmt, isize, str};

/// An opaque error type from `parse_hexf32` and `parse_hexf64`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseHexfError {
    kind: ParseHexfErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseHexfErrorKind {
    Empty,
    Invalid,
    Inexact,
}

const EMPTY: ParseHexfError = ParseHexfError {
    kind: ParseHexfErrorKind::Empty,
};
const INVALID: ParseHexfError = ParseHexfError {
    kind: ParseHexfErrorKind::Invalid,
};
const INEXACT: ParseHexfError = ParseHexfError {
    kind: ParseHexfErrorKind::Inexact,
};

impl ParseHexfError {
    fn text(&self) -> &'static str {
        match self.kind {
            ParseHexfErrorKind::Empty => "cannot parse float from empty string",
            ParseHexfErrorKind::Invalid => "invalid hexadecimal float literal",
            ParseHexfErrorKind::Inexact => "cannot exactly represent float in target type",
        }
    }
}

impl fmt::Display for ParseHexfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.text(), f)
    }
}

impl std::error::Error for ParseHexfError {
    fn description(&self) -> &'static str {
        self.text()
    }
}

fn parse(s: &[u8], allow_underscore: bool) -> Result<(bool, u64, isize), ParseHexfError> {
    // ^[+-]?
    let (s, negative) = match s.split_first() {
        Some((&b'+', s)) => (s, false),
        Some((&b'-', s)) => (s, true),
        Some(_) => (s, false),
        None => return Err(EMPTY),
    };

    // 0[xX]
    if !(s.starts_with(b"0x") || s.starts_with(b"0X")) {
        return Err(INVALID);
    }

    // ([0-9a-fA-F][0-9a-fA-F_]*)?
    let mut s = &s[2..];
    let mut acc = 0; // the accumulated mantissa
    let mut digit_seen = false;
    loop {
        let (s_, digit) = match s.split_first() {
            Some((&c @ b'0'..=b'9', s)) => (s, c - b'0'),
            Some((&c @ b'a'..=b'f', s)) => (s, c - b'a' + 10),
            Some((&c @ b'A'..=b'F', s)) => (s, c - b'A' + 10),
            Some((&b'_', s_)) if allow_underscore && digit_seen => {
                s = s_;
                continue;
            }
            _ => break,
        };

        s = s_;
        digit_seen = true;

        // if `acc << 4` fails, mantissa definitely exceeds 64 bits so we should bail out
        if acc >> 60 != 0 {
            return Err(INEXACT);
        }
        acc = acc << 4 | digit as u64;
    }

    // (\.[0-9a-fA-F][0-9a-fA-F_]*)?
    // we want to ignore trailing zeroes but shifting at each digit will overflow first.
    // therefore we separately count the number of zeroes and flush it on non-zero digits.
    let mut nfracs = 0isize; // this is suboptimal but also practical, see below
    let mut nzeroes = 0isize;
    let mut frac_digit_seen = false;
    if s.starts_with(b".") {
        s = &s[1..];
        loop {
            let (s_, digit) = match s.split_first() {
                Some((&c @ b'0'..=b'9', s)) => (s, c - b'0'),
                Some((&c @ b'a'..=b'f', s)) => (s, c - b'a' + 10),
                Some((&c @ b'A'..=b'F', s)) => (s, c - b'A' + 10),
                Some((&b'_', s_)) if allow_underscore && frac_digit_seen => {
                    s = s_;
                    continue;
                }
                _ => break,
            };

            s = s_;
            frac_digit_seen = true;

            if digit == 0 {
                nzeroes = nzeroes.checked_add(1).ok_or(INEXACT)?;
            } else {
                // flush nzeroes
                let nnewdigits = nzeroes.checked_add(1).ok_or(INEXACT)?;
                nfracs = nfracs.checked_add(nnewdigits).ok_or(INEXACT)?;
                nzeroes = 0;

                // if the accumulator is non-zero, the shift cannot exceed 64
                // (therefore the number of new digits cannot exceed 16).
                // this will catch e.g. `0.40000....00001` with sufficiently many zeroes
                if acc != 0 {
                    if nnewdigits >= 16 || acc >> (64 - nnewdigits * 4) != 0 {
                        return Err(INEXACT);
                    }
                    acc = acc << (nnewdigits * 4);
                }
                acc |= digit as u64;
            }
        }
    }

    // at least one digit should be present
    if !(digit_seen || frac_digit_seen) {
        return Err(INVALID);
    }

    // [pP]
    let s = match s.split_first() {
        Some((&b'P', s)) | Some((&b'p', s)) => s,
        _ => return Err(INVALID),
    };

    // [+-]?
    let (mut s, negative_exponent) = match s.split_first() {
        Some((&b'+', s)) => (s, false),
        Some((&b'-', s)) => (s, true),
        Some(_) => (s, false),
        None => return Err(INVALID),
    };

    // [0-9_]*[0-9][0-9_]*$
    let mut digit_seen = false;
    let mut exponent = 0isize; // this is suboptimal but also practical, see below
    loop {
        let (s_, digit) = match s.split_first() {
            Some((&c @ b'0'..=b'9', s)) => (s, c - b'0'),
            Some((&b'_', s_)) if allow_underscore => {
                s = s_;
                continue;
            }
            None if digit_seen => break,
            // no more bytes expected, and at least one exponent digit should be present
            _ => return Err(INVALID),
        };

        s = s_;
        digit_seen = true;

        // if we have no non-zero digits at this point, ignore the exponent :-)
        if acc != 0 {
            exponent = exponent
                .checked_mul(10)
                .and_then(|v| v.checked_add(digit as isize))
                .ok_or(INEXACT)?;
        }
    }
    if negative_exponent {
        exponent = -exponent;
    }

    if acc == 0 {
        // ignore the exponent as above
        Ok((negative, 0, 0))
    } else {
        // the exponent should be biased by (nfracs * 4) to match with the mantissa read.
        // we still miss valid inputs like `0.0000...0001pX` where the input is filling
        // at least 1/4 of the total addressable memory, but I dare not handle them!
        let exponent = nfracs
            .checked_mul(4)
            .and_then(|v| exponent.checked_sub(v))
            .ok_or(INEXACT)?;
        Ok((negative, acc, exponent))
    }
}

#[test]
fn test_parse() {
    assert_eq!(parse(b"", false), Err(EMPTY));
    assert_eq!(parse(b" ", false), Err(INVALID));
    assert_eq!(parse(b"3.14", false), Err(INVALID));
    assert_eq!(parse(b"0x3.14", false), Err(INVALID));
    assert_eq!(parse(b"0x3.14fp+3", false), Ok((false, 0x314f, 3 - 12)));
    assert_eq!(parse(b" 0x3.14p+3", false), Err(INVALID));
    assert_eq!(parse(b"0x3.14p+3 ", false), Err(INVALID));
    assert_eq!(parse(b"+0x3.14fp+3", false), Ok((false, 0x314f, 3 - 12)));
    assert_eq!(parse(b"-0x3.14fp+3", false), Ok((true, 0x314f, 3 - 12)));
    assert_eq!(parse(b"0xAbC.p1", false), Ok((false, 0xabc, 1)));
    assert_eq!(parse(b"0x0.7p1", false), Ok((false, 0x7, 1 - 4)));
    assert_eq!(parse(b"0x.dEfP-1", false), Ok((false, 0xdef, -1 - 12)));
    assert_eq!(parse(b"0x.p1", false), Err(INVALID));
    assert_eq!(parse(b"0x.P1", false), Err(INVALID));
    assert_eq!(parse(b"0xp1", false), Err(INVALID));
    assert_eq!(parse(b"0xP1", false), Err(INVALID));
    assert_eq!(parse(b"0x0p", false), Err(INVALID));
    assert_eq!(parse(b"0xp", false), Err(INVALID));
    assert_eq!(parse(b"0x.p", false), Err(INVALID));
    assert_eq!(parse(b"0x0p1", false), Ok((false, 0, 0)));
    assert_eq!(parse(b"0x0P1", false), Ok((false, 0, 0)));
    assert_eq!(parse(b"0x0.p1", false), Ok((false, 0, 0)));
    assert_eq!(parse(b"0x0.P1", false), Ok((false, 0, 0)));
    assert_eq!(parse(b"0x0.0p1", false), Ok((false, 0, 0)));
    assert_eq!(parse(b"0x0.0P1", false), Ok((false, 0, 0)));
    assert_eq!(parse(b"0x.0p1", false), Ok((false, 0, 0)));
    assert_eq!(parse(b"0x.0P1", false), Ok((false, 0, 0)));
    assert_eq!(parse(b"0x0p0", false), Ok((false, 0, 0)));
    assert_eq!(parse(b"0x0.p999999999", false), Ok((false, 0, 0)));
    assert_eq!(
        parse(b"0x0.p99999999999999999999999999999", false),
        Ok((false, 0, 0))
    );
    assert_eq!(
        parse(b"0x0.p-99999999999999999999999999999", false),
        Ok((false, 0, 0))
    );
    assert_eq!(
        parse(b"0x1.p99999999999999999999999999999", false),
        Err(INEXACT)
    );
    assert_eq!(
        parse(b"0x1.p-99999999999999999999999999999", false),
        Err(INEXACT)
    );
    assert_eq!(
        parse(b"0x4.00000000000000000000p55", false),
        Ok((false, 4, 55))
    );
    assert_eq!(
        parse(b"0x4.00001000000000000000p55", false),
        Ok((false, 0x400001, 55 - 20))
    );
    assert_eq!(parse(b"0x4.00000000000000000001p55", false), Err(INEXACT));

    // underscore insertion
    assert_eq!(
        parse(b"-0x3____.1_4___p+___5___", true),
        Ok((true, 0x314, 5 - 8))
    );
    assert_eq!(parse(b"-_0x3.14p+5", true), Err(INVALID));
    assert_eq!(parse(b"_0x3.14p+5", true), Err(INVALID));
    assert_eq!(parse(b"0x_3.14p+5", true), Err(INVALID));
    assert_eq!(parse(b"0x3._14p+5", true), Err(INVALID));
    assert_eq!(parse(b"0x3.14p_+5", true), Err(INVALID));
    assert_eq!(parse(b"-0x____.1_4___p+___5___", true), Err(INVALID));
    assert_eq!(parse(b"-0x3____.____p+___5___", true), Err(INVALID));
    assert_eq!(parse(b"-0x3____.1_4___p+______", true), Err(INVALID));
    assert_eq!(parse(b"0x_p0", false), Err(INVALID));
    assert_eq!(parse(b"0x_0p0", true), Err(INVALID));
    assert_eq!(parse(b"0x_p0", true), Err(INVALID));
    assert_eq!(parse(b"0x._p0", true), Err(INVALID));
    assert_eq!(parse(b"0x._0p0", true), Err(INVALID));
    assert_eq!(parse(b"0x0._0p0", true), Err(INVALID));
    assert_eq!(parse(b"0x0_p0", true), Ok((false, 0, 0)));
    assert_eq!(parse(b"0x.0_p0", true), Ok((false, 0, 0)));
    assert_eq!(parse(b"0x0.0_p0", true), Ok((false, 0, 0)));

    // issues
    // #11 (https://github.com/lifthrasiir/hexf/issues/11)
    assert_eq!(parse(b"0x1p-149", false), parse(b"0x1.0p-149", false));
}

macro_rules! define_convert {
    ($name:ident => $f:ident) => {
        fn $name(negative: bool, mantissa: u64, exponent: isize) -> Result<$f, ParseHexfError> {
            // guard the exponent with the definitely safe range (we will exactly bound it later)
            if exponent < -0xffff || exponent > 0xffff {
                return Err(INEXACT);
            }

            // strip the trailing zeroes in mantissa and adjust exponent.
            // we do this because a unit in the least significant bit of mantissa is
            // always safe to represent while one in the most significant bit isn't.
            let trailing = mantissa.trailing_zeros() & 63; // guard mantissa=0 case
            let mantissa = mantissa >> trailing;
            let exponent = exponent + trailing as isize;

            // normalize the exponent that the number is (1.xxxx * 2^normalexp),
            // and check for the mantissa and exponent ranges
            let leading = mantissa.leading_zeros();
            let normalexp = exponent + (63 - leading as isize);
            let mantissasize = if normalexp < $f::MIN_EXP as isize - $f::MANTISSA_DIGITS as isize {
                // the number is smaller than the minimal denormal number
                return Err(INEXACT);
            } else if normalexp < ($f::MIN_EXP - 1) as isize {
                // the number is denormal, the # of bits in the mantissa is:
                // - minimum (1) at MIN_EXP - MANTISSA_DIGITS
                // - maximum (MANTISSA_DIGITS - 1) at MIN_EXP - 2
                $f::MANTISSA_DIGITS as isize - $f::MIN_EXP as isize + normalexp + 1
            } else if normalexp < $f::MAX_EXP as isize {
                // the number is normal, the # of bits in the mantissa is fixed
                $f::MANTISSA_DIGITS as isize
            } else {
                // the number is larger than the maximal denormal number
                // ($f::MAX_EXP denotes NaN and infinities here)
                return Err(INEXACT);
            };

            if mantissa >> mantissasize == 0 {
                let mut mantissa = mantissa as $f;
                if negative {
                    mantissa = -mantissa;
                }
                // yes, powi somehow does not work!
                Ok(mantissa * (2.0 as $f).powf(exponent as $f))
            } else {
                Err(INEXACT)
            }
        }
    };
}

define_convert!(convert_hexf32 => f32);
define_convert!(convert_hexf64 => f64);

#[test]
fn test_convert_hexf32() {
    assert_eq!(convert_hexf32(false, 0, 0), Ok(0.0));
    assert_eq!(convert_hexf32(false, 1, 0), Ok(1.0));
    assert_eq!(convert_hexf32(false, 10, 0), Ok(10.0));
    assert_eq!(convert_hexf32(false, 10, 1), Ok(20.0));
    assert_eq!(convert_hexf32(false, 10, -1), Ok(5.0));
    assert_eq!(convert_hexf32(true, 0, 0), Ok(-0.0));
    assert_eq!(convert_hexf32(true, 1, 0), Ok(-1.0));

    // negative zeroes
    assert_eq!(convert_hexf32(false, 0, 0).unwrap().signum(), 1.0);
    assert_eq!(convert_hexf32(true, 0, 0).unwrap().signum(), -1.0);

    // normal truncation
    assert_eq!(
        convert_hexf32(false, 0x0000_0000_00ff_ffff, 0),
        Ok(16777215.0)
    );
    assert_eq!(
        convert_hexf32(false, 0x0000_0000_01ff_ffff, 0),
        Err(INEXACT)
    );
    assert_eq!(
        convert_hexf32(false, 0xffff_ff00_0000_0000, -40),
        Ok(16777215.0)
    );
    assert_eq!(
        convert_hexf32(false, 0xffff_ff80_0000_0000, -40),
        Err(INEXACT)
    );

    // denormal truncation
    assert!(convert_hexf32(false, 0x0000_0000_007f_ffff, -149).is_ok());
    assert!(convert_hexf32(false, 0x0000_0000_00ff_ffff, -150).is_err());
    assert!(convert_hexf32(false, 0x0000_0000_00ff_fffe, -150).is_ok());
    assert!(convert_hexf32(false, 0xffff_ff00_0000_0000, -190).is_err());
    assert!(convert_hexf32(false, 0xffff_fe00_0000_0000, -190).is_ok());

    // minimum
    assert!(convert_hexf32(false, 0x0000_0000_0000_0001, -149).is_ok());
    assert!(convert_hexf32(false, 0x0000_0000_0000_0001, -150).is_err());
    assert!(convert_hexf32(false, 0x0000_0000_0000_0002, -150).is_ok());
    assert!(convert_hexf32(false, 0x0000_0000_0000_0002, -151).is_err());
    assert!(convert_hexf32(false, 0x0000_0000_0000_0003, -150).is_err());
    assert!(convert_hexf32(false, 0x0000_0000_0000_0003, -151).is_err());
    assert!(convert_hexf32(false, 0x8000_0000_0000_0000, -212).is_ok());
    assert!(convert_hexf32(false, 0x8000_0000_0000_0000, -213).is_err());

    // maximum
    assert_eq!(
        convert_hexf32(false, 0x0000_0000_00ff_ffff, 104),
        Ok(f32::MAX)
    );
    assert_eq!(
        convert_hexf32(false, 0x0000_0000_01ff_ffff, 104),
        Err(INEXACT)
    );
    assert_eq!(
        convert_hexf32(false, 0x0000_0000_01ff_fffe, 104),
        Err(INEXACT)
    );
    assert_eq!(
        convert_hexf32(false, 0x0000_0000_0000_0001, 128),
        Err(INEXACT)
    );
    assert_eq!(
        convert_hexf32(false, 0x8000_0000_0000_0000, 65),
        Err(INEXACT)
    );
    assert_eq!(
        convert_hexf32(false, 0xffff_ff00_0000_0000, 64),
        Ok(f32::MAX)
    );
    assert_eq!(
        convert_hexf32(false, 0xffff_ff80_0000_0000, 64),
        Err(INEXACT)
    );
}

#[test]
fn test_convert_hexf64() {
    assert_eq!(convert_hexf64(false, 0, 0), Ok(0.0));
    assert_eq!(convert_hexf64(false, 1, 0), Ok(1.0));
    assert_eq!(convert_hexf64(false, 10, 0), Ok(10.0));
    assert_eq!(convert_hexf64(false, 10, 1), Ok(20.0));
    assert_eq!(convert_hexf64(false, 10, -1), Ok(5.0));
    assert_eq!(convert_hexf64(true, 0, 0), Ok(-0.0));
    assert_eq!(convert_hexf64(true, 1, 0), Ok(-1.0));

    // negative zeroes
    assert_eq!(convert_hexf64(false, 0, 0).unwrap().signum(), 1.0);
    assert_eq!(convert_hexf64(true, 0, 0).unwrap().signum(), -1.0);

    // normal truncation
    assert_eq!(
        convert_hexf64(false, 0x001f_ffff_ffff_ffff, 0),
        Ok(9007199254740991.0)
    );
    assert_eq!(
        convert_hexf64(false, 0x003f_ffff_ffff_ffff, 0),
        Err(INEXACT)
    );
    assert_eq!(
        convert_hexf64(false, 0xffff_ffff_ffff_f800, -11),
        Ok(9007199254740991.0)
    );
    assert_eq!(
        convert_hexf64(false, 0xffff_ffff_ffff_fc00, -11),
        Err(INEXACT)
    );

    // denormal truncation
    assert!(convert_hexf64(false, 0x000f_ffff_ffff_ffff, -1074).is_ok());
    assert!(convert_hexf64(false, 0x001f_ffff_ffff_ffff, -1075).is_err());
    assert!(convert_hexf64(false, 0x001f_ffff_ffff_fffe, -1075).is_ok());
    assert!(convert_hexf64(false, 0xffff_ffff_ffff_f800, -1086).is_err());
    assert!(convert_hexf64(false, 0xffff_ffff_ffff_f000, -1086).is_ok());

    // minimum
    assert!(convert_hexf64(false, 0x0000_0000_0000_0001, -1074).is_ok());
    assert!(convert_hexf64(false, 0x0000_0000_0000_0001, -1075).is_err());
    assert!(convert_hexf64(false, 0x0000_0000_0000_0002, -1075).is_ok());
    assert!(convert_hexf64(false, 0x0000_0000_0000_0002, -1076).is_err());
    assert!(convert_hexf64(false, 0x0000_0000_0000_0003, -1075).is_err());
    assert!(convert_hexf64(false, 0x0000_0000_0000_0003, -1076).is_err());
    assert!(convert_hexf64(false, 0x8000_0000_0000_0000, -1137).is_ok());
    assert!(convert_hexf64(false, 0x8000_0000_0000_0000, -1138).is_err());

    // maximum
    assert_eq!(
        convert_hexf64(false, 0x001f_ffff_ffff_ffff, 971),
        Ok(f64::MAX)
    );
    assert_eq!(
        convert_hexf64(false, 0x003f_ffff_ffff_ffff, 971),
        Err(INEXACT)
    );
    assert_eq!(
        convert_hexf64(false, 0x003f_ffff_ffff_fffe, 971),
        Err(INEXACT)
    );
    assert_eq!(
        convert_hexf32(false, 0x0000_0000_0000_0001, 1024),
        Err(INEXACT)
    );
    assert_eq!(
        convert_hexf32(false, 0x8000_0000_0000_0000, 961),
        Err(INEXACT)
    );
    assert_eq!(
        convert_hexf64(false, 0xffff_ffff_ffff_f800, 960),
        Ok(f64::MAX)
    );
    assert_eq!(
        convert_hexf64(false, 0xffff_ffff_ffff_fc00, 960),
        Err(INEXACT)
    );
}

/// Tries to parse a hexadecimal float literal to `f32`.
/// The underscore is allowed only when `allow_underscore` is true.
pub fn parse_hexf32(s: &str, allow_underscore: bool) -> Result<f32, ParseHexfError> {
    let (negative, mantissa, exponent) = parse(s.as_bytes(), allow_underscore)?;
    convert_hexf32(negative, mantissa, exponent)
}

/// Tries to parse a hexadecimal float literal to `f64`.
/// The underscore is allowed only when `allow_underscore` is true.
pub fn parse_hexf64(s: &str, allow_underscore: bool) -> Result<f64, ParseHexfError> {
    let (negative, mantissa, exponent) = parse(s.as_bytes(), allow_underscore)?;
    convert_hexf64(negative, mantissa, exponent)
}

#[test]
fn test_parse_hexf() {
    // issues
    // #6 (https://github.com/lifthrasiir/hexf/issues/6)
    assert!(parse_hexf64("0x.000000000000000000102", false).is_err());
}
