use crate::{
    constants::{BYTES_TO_OVERFLOW_U64, MAX_SCALE, MAX_STR_BUFFER_SIZE, OVERFLOW_U96, WILL_OVERFLOW_U64},
    error::{tail_error, Error},
    ops::array::{add_by_internal_flattened, add_one_internal, div_by_u32, is_all_zero, mul_by_u32},
    Decimal,
};

use arrayvec::{ArrayString, ArrayVec};

use alloc::{string::String, vec::Vec};
use core::fmt;

// impl that doesn't allocate for serialization purposes.
pub(crate) fn to_str_internal(
    value: &Decimal,
    append_sign: bool,
    precision: Option<usize>,
) -> (ArrayString<MAX_STR_BUFFER_SIZE>, Option<usize>) {
    // Get the scale - where we need to put the decimal point
    let scale = value.scale() as usize;

    // Convert to a string and manipulate that (neg at front, inject decimal)
    let mut chars = ArrayVec::<_, MAX_STR_BUFFER_SIZE>::new();
    let mut working = value.mantissa_array3();
    while !is_all_zero(&working) {
        let remainder = div_by_u32(&mut working, 10u32);
        chars.push(char::from(b'0' + remainder as u8));
    }
    while scale > chars.len() {
        chars.push('0');
    }

    let (prec, additional) = match precision {
        Some(prec) => {
            let max: usize = MAX_SCALE.into();
            if prec > max {
                (max, Some(prec - max))
            } else {
                (prec, None)
            }
        }
        None => (scale, None),
    };

    let len = chars.len();
    let whole_len = len - scale;
    let mut rep = ArrayString::new();
    // Append the negative sign if necessary while also keeping track of the length of an "empty" string representation
    let empty_len = if append_sign && value.is_sign_negative() {
        rep.push('-');
        1
    } else {
        0
    };
    for i in 0..whole_len + prec {
        if i == len - scale {
            if i == 0 {
                rep.push('0');
            }
            rep.push('.');
        }

        if i >= len {
            rep.push('0');
        } else {
            let c = chars[len - i - 1];
            rep.push(c);
        }
    }

    // corner case for when we truncated everything in a low fractional
    if rep.len() == empty_len {
        rep.push('0');
    }

    (rep, additional)
}

pub(crate) fn fmt_scientific_notation(
    value: &Decimal,
    exponent_symbol: &str,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    #[cfg(not(feature = "std"))]
    use alloc::string::ToString;

    // Get the scale - this is the e value. With multiples of 10 this may get bigger.
    let mut exponent = -(value.scale() as isize);

    // Convert the integral to a string
    let mut chars = Vec::new();
    let mut working = value.mantissa_array3();
    while !is_all_zero(&working) {
        let remainder = div_by_u32(&mut working, 10u32);
        chars.push(char::from(b'0' + remainder as u8));
    }

    // First of all, apply scientific notation rules. That is:
    //  1. If non-zero digit comes first, move decimal point left so that e is a positive integer
    //  2. If decimal point comes first, move decimal point right until after the first non-zero digit
    // Since decimal notation naturally lends itself this way, we just need to inject the decimal
    // point in the right place and adjust the exponent accordingly.

    let len = chars.len();
    let mut rep;
    // We either are operating with a precision specified, or on defaults. Defaults will perform "smart"
    // reduction of precision.
    if let Some(precision) = f.precision() {
        if len > 1 {
            // If we're zero precision AND it's trailing zeros then strip them
            if precision == 0 && chars.iter().take(len - 1).all(|c| *c == '0') {
                rep = chars.iter().skip(len - 1).collect::<String>();
            } else {
                // We may still be zero precision, however we aren't trailing zeros
                if precision > 0 {
                    chars.insert(len - 1, '.');
                }
                rep = chars
                    .iter()
                    .rev()
                    // Add on extra zeros according to the precision. At least one, since we added a decimal place.
                    .chain(core::iter::repeat(&'0'))
                    .take(if precision == 0 { 1 } else { 2 + precision })
                    .collect::<String>();
            }
            exponent += (len - 1) as isize;
        } else if precision > 0 {
            // We have precision that we want to add
            chars.push('.');
            rep = chars
                .iter()
                .chain(core::iter::repeat(&'0'))
                .take(2 + precision)
                .collect::<String>();
        } else {
            rep = chars.iter().collect::<String>();
        }
    } else if len > 1 {
        // If the number is just trailing zeros then we treat it like 0 precision
        if chars.iter().take(len - 1).all(|c| *c == '0') {
            rep = chars.iter().skip(len - 1).collect::<String>();
        } else {
            // Otherwise, we need to insert a decimal place and make it a scientific number
            chars.insert(len - 1, '.');
            rep = chars.iter().rev().collect::<String>();
        }
        exponent += (len - 1) as isize;
    } else {
        rep = chars.iter().collect::<String>();
    }

    rep.push_str(exponent_symbol);
    rep.push_str(&exponent.to_string());
    f.pad_integral(value.is_sign_positive(), "", &rep)
}

// dedicated implementation for the most common case.
#[inline]
pub(crate) fn parse_str_radix_10(str: &str) -> Result<Decimal, Error> {
    let bytes = str.as_bytes();
    if bytes.len() < BYTES_TO_OVERFLOW_U64 {
        parse_str_radix_10_dispatch::<false, true>(bytes)
    } else {
        parse_str_radix_10_dispatch::<true, true>(bytes)
    }
}

#[inline]
pub(crate) fn parse_str_radix_10_exact(str: &str) -> Result<Decimal, Error> {
    let bytes = str.as_bytes();
    if bytes.len() < BYTES_TO_OVERFLOW_U64 {
        parse_str_radix_10_dispatch::<false, false>(bytes)
    } else {
        parse_str_radix_10_dispatch::<true, false>(bytes)
    }
}

#[inline]
fn parse_str_radix_10_dispatch<const BIG: bool, const ROUND: bool>(bytes: &[u8]) -> Result<Decimal, Error> {
    match bytes {
        [b, rest @ ..] => byte_dispatch_u64::<false, false, false, BIG, true, ROUND>(rest, 0, 0, *b),
        [] => tail_error("Invalid decimal: empty"),
    }
}

#[inline]
fn overflow_64(val: u64) -> bool {
    val >= WILL_OVERFLOW_U64
}

#[inline]
pub fn overflow_128(val: u128) -> bool {
    val >= OVERFLOW_U96
}

/// Dispatch the next byte:
///
/// * POINT - a decimal point has been seen
/// * NEG - we've encountered a `-` and the number is negative
/// * HAS - a digit has been encountered (when HAS is false it's invalid)
/// * BIG - a number that uses 96 bits instead of only 64 bits
/// * FIRST - true if it is the first byte in the string
#[inline]
fn dispatch_next<const POINT: bool, const NEG: bool, const HAS: bool, const BIG: bool, const ROUND: bool>(
    bytes: &[u8],
    data64: u64,
    scale: u8,
) -> Result<Decimal, Error> {
    if let Some((next, bytes)) = bytes.split_first() {
        byte_dispatch_u64::<POINT, NEG, HAS, BIG, false, ROUND>(bytes, data64, scale, *next)
    } else {
        handle_data::<NEG, HAS>(data64 as u128, scale)
    }
}

/// Dispatch the next non-digit byte:
///
/// * POINT - a decimal point has been seen
/// * NEG - we've encountered a `-` and the number is negative
/// * HAS - a digit has been encountered (when HAS is false it's invalid)
/// * BIG - a number that uses 96 bits instead of only 64 bits
/// * FIRST - true if it is the first byte in the string
/// * ROUND - attempt to round underflow
#[inline(never)]
fn non_digit_dispatch_u64<
    const POINT: bool,
    const NEG: bool,
    const HAS: bool,
    const BIG: bool,
    const FIRST: bool,
    const ROUND: bool,
>(
    bytes: &[u8],
    data64: u64,
    scale: u8,
    b: u8,
) -> Result<Decimal, Error> {
    match b {
        b'-' if FIRST && !HAS => dispatch_next::<false, true, false, BIG, ROUND>(bytes, data64, scale),
        b'+' if FIRST && !HAS => dispatch_next::<false, false, false, BIG, ROUND>(bytes, data64, scale),
        b'_' if HAS => handle_separator::<POINT, NEG, BIG, ROUND>(bytes, data64, scale),
        b => tail_invalid_digit(b),
    }
}

#[inline]
fn byte_dispatch_u64<
    const POINT: bool,
    const NEG: bool,
    const HAS: bool,
    const BIG: bool,
    const FIRST: bool,
    const ROUND: bool,
>(
    bytes: &[u8],
    data64: u64,
    scale: u8,
    b: u8,
) -> Result<Decimal, Error> {
    match b {
        b'0'..=b'9' => handle_digit_64::<POINT, NEG, BIG, ROUND>(bytes, data64, scale, b - b'0'),
        b'.' if !POINT => handle_point::<NEG, HAS, BIG, ROUND>(bytes, data64, scale),
        b => non_digit_dispatch_u64::<POINT, NEG, HAS, BIG, FIRST, ROUND>(bytes, data64, scale, b),
    }
}

#[inline(never)]
fn handle_digit_64<const POINT: bool, const NEG: bool, const BIG: bool, const ROUND: bool>(
    bytes: &[u8],
    data64: u64,
    scale: u8,
    digit: u8,
) -> Result<Decimal, Error> {
    // we have already validated that we cannot overflow
    let data64 = data64 * 10 + digit as u64;
    let scale = if POINT { scale + 1 } else { 0 };

    if let Some((next, bytes)) = bytes.split_first() {
        let next = *next;
        if POINT && BIG && scale >= 28 {
            if ROUND {
                maybe_round(data64 as u128, next, scale, POINT, NEG)
            } else {
                Err(Error::Underflow)
            }
        } else if BIG && overflow_64(data64) {
            handle_full_128::<POINT, NEG, ROUND>(data64 as u128, bytes, scale, next)
        } else {
            byte_dispatch_u64::<POINT, NEG, true, BIG, false, ROUND>(bytes, data64, scale, next)
        }
    } else {
        let data: u128 = data64 as u128;

        handle_data::<NEG, true>(data, scale)
    }
}

#[inline(never)]
fn handle_point<const NEG: bool, const HAS: bool, const BIG: bool, const ROUND: bool>(
    bytes: &[u8],
    data64: u64,
    scale: u8,
) -> Result<Decimal, Error> {
    dispatch_next::<true, NEG, HAS, BIG, ROUND>(bytes, data64, scale)
}

#[inline(never)]
fn handle_separator<const POINT: bool, const NEG: bool, const BIG: bool, const ROUND: bool>(
    bytes: &[u8],
    data64: u64,
    scale: u8,
) -> Result<Decimal, Error> {
    dispatch_next::<POINT, NEG, true, BIG, ROUND>(bytes, data64, scale)
}

#[inline(never)]
#[cold]
fn tail_invalid_digit(digit: u8) -> Result<Decimal, Error> {
    match digit {
        b'.' => tail_error("Invalid decimal: two decimal points"),
        b'_' => tail_error("Invalid decimal: must start lead with a number"),
        _ => tail_error("Invalid decimal: unknown character"),
    }
}

#[inline(never)]
#[cold]
fn handle_full_128<const POINT: bool, const NEG: bool, const ROUND: bool>(
    mut data: u128,
    bytes: &[u8],
    scale: u8,
    next_byte: u8,
) -> Result<Decimal, Error> {
    let b = next_byte;
    match b {
        b'0'..=b'9' => {
            let digit = u32::from(b - b'0');

            // If the data is going to overflow then we should go into recovery mode
            let next = (data * 10) + digit as u128;
            if overflow_128(next) {
                if !POINT {
                    return tail_error("Invalid decimal: overflow from too many digits");
                }

                if ROUND {
                    maybe_round(data, next_byte, scale, POINT, NEG)
                } else {
                    Err(Error::Underflow)
                }
            } else {
                data = next;
                let scale = scale + POINT as u8;
                if let Some((next, bytes)) = bytes.split_first() {
                    let next = *next;
                    if POINT && scale >= 28 {
                        if ROUND {
                            // If it is an underscore at the rounding position we require slightly different handling to look ahead another digit
                            if next == b'_' {
                                // Skip consecutive underscores to find the next actual character
                                let mut remaining_bytes = bytes;
                                let mut next_char = None;
                                while let Some((n, rest)) = remaining_bytes.split_first() {
                                    if *n != b'_' {
                                        next_char = Some(*n);
                                        break;
                                    }
                                    remaining_bytes = rest;
                                }

                                if let Some(ch) = next_char {
                                    // Skip underscores and use the next character for rounding
                                    maybe_round(data, ch, scale, POINT, NEG)
                                } else {
                                    handle_data::<NEG, true>(data, scale)
                                }
                            } else {
                                // Otherwise, we round as usual
                                maybe_round(data, next, scale, POINT, NEG)
                            }
                        } else {
                            Err(Error::Underflow)
                        }
                    } else {
                        handle_full_128::<POINT, NEG, ROUND>(data, bytes, scale, next)
                    }
                } else {
                    handle_data::<NEG, true>(data, scale)
                }
            }
        }
        b'.' if !POINT => {
            // This call won't tail?
            if let Some((next, bytes)) = bytes.split_first() {
                handle_full_128::<true, NEG, ROUND>(data, bytes, scale, *next)
            } else {
                handle_data::<NEG, true>(data, scale)
            }
        }
        b'_' => {
            if let Some((next, bytes)) = bytes.split_first() {
                handle_full_128::<POINT, NEG, ROUND>(data, bytes, scale, *next)
            } else {
                handle_data::<NEG, true>(data, scale)
            }
        }
        b => tail_invalid_digit(b),
    }
}

#[inline(never)]
#[cold]
fn maybe_round(mut data: u128, next_byte: u8, mut scale: u8, point: bool, negative: bool) -> Result<Decimal, Error> {
    let digit = match next_byte {
        b'0'..=b'9' => u32::from(next_byte - b'0'),
        b'_' => 0, // This is perhaps an error case, but keep this here for compatibility
        b'.' if !point => 0,
        b => return tail_invalid_digit(b),
    };

    // Round at midpoint
    if digit >= 5 {
        data += 1;

        // If the mantissa is now overflowing, round to the next
        // next least significant digit and discard precision
        if overflow_128(data) {
            if scale == 0 {
                return tail_error("Invalid decimal: overflow from mantissa after rounding");
            }
            data += 4;
            data /= 10;
            scale -= 1;
        }
    }

    if negative {
        handle_data::<true, true>(data, scale)
    } else {
        handle_data::<false, true>(data, scale)
    }
}

#[inline(never)]
fn tail_no_has() -> Result<Decimal, Error> {
    tail_error("Invalid decimal: no digits found")
}

#[inline]
fn handle_data<const NEG: bool, const HAS: bool>(data: u128, scale: u8) -> Result<Decimal, Error> {
    debug_assert_eq!(data >> 96, 0);
    if !HAS {
        tail_no_has()
    } else {
        Ok(Decimal::from_parts(
            data as u32,
            (data >> 32) as u32,
            (data >> 64) as u32,
            NEG,
            scale as u32,
        ))
    }
}

pub(crate) fn parse_str_radix_n(str: &str, radix: u32) -> Result<Decimal, Error> {
    if str.is_empty() {
        return Err(Error::from("Invalid decimal: empty"));
    }
    if radix < 2 {
        return Err(Error::from("Unsupported radix < 2"));
    }
    if radix > 36 {
        // As per trait documentation
        return Err(Error::from("Unsupported radix > 36"));
    }

    let mut offset = 0;
    let mut len = str.len();
    let bytes = str.as_bytes();
    let mut negative = false; // assume positive

    // handle the sign
    if bytes[offset] == b'-' {
        negative = true; // leading minus means negative
        offset += 1;
        len -= 1;
    } else if bytes[offset] == b'+' {
        // leading + allowed
        offset += 1;
        len -= 1;
    }

    // should now be at numeric part of the significand
    let mut digits_before_dot: i32 = -1; // digits before '.', -1 if no '.'
    let mut coeff = ArrayVec::<_, 96>::new(); // integer significand array

    // Supporting different radix
    let (max_n, max_alpha_lower, max_alpha_upper) = if radix <= 10 {
        (b'0' + (radix - 1) as u8, 0, 0)
    } else {
        let adj = (radix - 11) as u8;
        (b'9', adj + b'a', adj + b'A')
    };

    // Estimate the max precision. All in all, it needs to fit into 96 bits.
    // Rather than try to estimate, I've included the constants directly in here. We could,
    // perhaps, replace this with a formula if it's faster - though it does appear to be log2.
    let estimated_max_precision = match radix {
        2 => 96,
        3 => 61,
        4 => 48,
        5 => 42,
        6 => 38,
        7 => 35,
        8 => 32,
        9 => 31,
        10 => 28,
        11 => 28,
        12 => 27,
        13 => 26,
        14 => 26,
        15 => 25,
        16 => 24,
        17 => 24,
        18 => 24,
        19 => 23,
        20 => 23,
        21 => 22,
        22 => 22,
        23 => 22,
        24 => 21,
        25 => 21,
        26 => 21,
        27 => 21,
        28 => 20,
        29 => 20,
        30 => 20,
        31 => 20,
        32 => 20,
        33 => 20,
        34 => 19,
        35 => 19,
        36 => 19,
        _ => return Err(Error::from("Unsupported radix")),
    };

    let mut maybe_round = false;
    while len > 0 {
        let b = bytes[offset];
        match b {
            b'0'..=b'9' => {
                if b > max_n {
                    return Err(Error::from("Invalid decimal: invalid character"));
                }
                coeff.push(u32::from(b - b'0'));
                offset += 1;
                len -= 1;

                // If the coefficient is longer than the max, exit early
                if coeff.len() as u32 > estimated_max_precision {
                    maybe_round = true;
                    break;
                }
            }
            b'a'..=b'z' => {
                if b > max_alpha_lower {
                    return Err(Error::from("Invalid decimal: invalid character"));
                }
                coeff.push(u32::from(b - b'a') + 10);
                offset += 1;
                len -= 1;

                if coeff.len() as u32 > estimated_max_precision {
                    maybe_round = true;
                    break;
                }
            }
            b'A'..=b'Z' => {
                if b > max_alpha_upper {
                    return Err(Error::from("Invalid decimal: invalid character"));
                }
                coeff.push(u32::from(b - b'A') + 10);
                offset += 1;
                len -= 1;

                if coeff.len() as u32 > estimated_max_precision {
                    maybe_round = true;
                    break;
                }
            }
            b'.' => {
                if digits_before_dot >= 0 {
                    return Err(Error::from("Invalid decimal: two decimal points"));
                }
                digits_before_dot = coeff.len() as i32;
                offset += 1;
                len -= 1;
            }
            b'_' => {
                // Must start with a number...
                if coeff.is_empty() {
                    return Err(Error::from("Invalid decimal: must start lead with a number"));
                }
                offset += 1;
                len -= 1;
            }
            _ => return Err(Error::from("Invalid decimal: unknown character")),
        }
    }

    // If we exited before the end of the string then do some rounding if necessary
    if maybe_round && offset < bytes.len() {
        let next_byte = bytes[offset];
        let digit = match next_byte {
            b'0'..=b'9' => {
                if next_byte > max_n {
                    return Err(Error::from("Invalid decimal: invalid character"));
                }
                u32::from(next_byte - b'0')
            }
            b'a'..=b'z' => {
                if next_byte > max_alpha_lower {
                    return Err(Error::from("Invalid decimal: invalid character"));
                }
                u32::from(next_byte - b'a') + 10
            }
            b'A'..=b'Z' => {
                if next_byte > max_alpha_upper {
                    return Err(Error::from("Invalid decimal: invalid character"));
                }
                u32::from(next_byte - b'A') + 10
            }
            b'_' => 0,
            b'.' => {
                // Still an error if we have a second dp
                if digits_before_dot >= 0 {
                    return Err(Error::from("Invalid decimal: two decimal points"));
                }
                0
            }
            _ => return Err(Error::from("Invalid decimal: unknown character")),
        };

        // Round at midpoint
        let midpoint = if radix & 0x1 == 1 { radix / 2 } else { (radix + 1) / 2 };
        if digit >= midpoint {
            let mut index = coeff.len() - 1;
            loop {
                let new_digit = coeff[index] + 1;
                if new_digit <= 9 {
                    coeff[index] = new_digit;
                    break;
                } else {
                    coeff[index] = 0;
                    if index == 0 {
                        coeff.insert(0, 1u32);
                        digits_before_dot += 1;
                        coeff.pop();
                        break;
                    }
                }
                index -= 1;
            }
        }
    }

    // here when no characters left
    if coeff.is_empty() {
        return Err(Error::from("Invalid decimal: no digits found"));
    }

    let mut scale = if digits_before_dot >= 0 {
        // we had a decimal place so set the scale
        (coeff.len() as u32) - (digits_before_dot as u32)
    } else {
        0
    };

    // Parse this using specified radix
    let mut data = [0u32, 0u32, 0u32];
    let mut tmp = [0u32, 0u32, 0u32];
    let len = coeff.len();
    for (i, digit) in coeff.iter().enumerate() {
        // If the data is going to overflow then we should go into recovery mode
        tmp[0] = data[0];
        tmp[1] = data[1];
        tmp[2] = data[2];
        let overflow = mul_by_u32(&mut tmp, radix);
        if overflow > 0 {
            // This means that we have more data to process, that we're not sure what to do with.
            // This may or may not be an issue - depending on whether we're past a decimal point
            // or not.
            if (i as i32) < digits_before_dot && i + 1 < len {
                return Err(Error::from("Invalid decimal: overflow from too many digits"));
            }

            if *digit >= 5 {
                let carry = add_one_internal(&mut data);
                if carry > 0 {
                    // Highly unlikely scenario which is more indicative of a bug
                    return Err(Error::from("Invalid decimal: overflow when rounding"));
                }
            }
            // We're also one less digit so reduce the scale
            let diff = (len - i) as u32;
            if diff > scale {
                return Err(Error::from("Invalid decimal: overflow from scale mismatch"));
            }
            scale -= diff;
            break;
        } else {
            data[0] = tmp[0];
            data[1] = tmp[1];
            data[2] = tmp[2];
            let carry = add_by_internal_flattened(&mut data, *digit);
            if carry > 0 {
                // Highly unlikely scenario which is more indicative of a bug
                return Err(Error::from("Invalid decimal: overflow from carry"));
            }
        }
    }

    Ok(Decimal::from_parts(data[0], data[1], data[2], negative, scale))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Decimal;
    use arrayvec::ArrayString;
    use core::{fmt::Write, str::FromStr};

    #[test]
    fn display_does_not_overflow_max_capacity() {
        let num = Decimal::from_str("1.2").unwrap();
        let mut buffer = ArrayString::<64>::new();
        buffer.write_fmt(format_args!("{num:.31}")).unwrap();
        assert_eq!("1.2000000000000000000000000000000", buffer.as_str());
    }

    #[test]
    fn from_str_rounding_0() {
        assert_eq!(
            parse_str_radix_10("1.234").unwrap().unpack(),
            Decimal::new(1234, 3).unpack()
        );
    }

    #[test]
    fn from_str_rounding_1() {
        assert_eq!(
            parse_str_radix_10("11111_11111_11111.11111_11111_11111")
                .unwrap()
                .unpack(),
            Decimal::from_i128_with_scale(11_111_111_111_111_111_111_111_111_111, 14).unpack()
        );
    }

    #[test]
    fn from_str_rounding_2() {
        assert_eq!(
            parse_str_radix_10("11111_11111_11111.11111_11111_11115")
                .unwrap()
                .unpack(),
            Decimal::from_i128_with_scale(11_111_111_111_111_111_111_111_111_112, 14).unpack()
        );
    }

    #[test]
    fn from_str_rounding_3() {
        assert_eq!(
            parse_str_radix_10("11111_11111_11111.11111_11111_11195")
                .unwrap()
                .unpack(),
            Decimal::from_i128_with_scale(1_111_111_111_111_111_111_111_111_1120, 14).unpack() // was Decimal::from_i128_with_scale(1_111_111_111_111_111_111_111_111_112, 13)
        );
    }

    #[test]
    fn from_str_rounding_4() {
        assert_eq!(
            parse_str_radix_10("99999_99999_99999.99999_99999_99995")
                .unwrap()
                .unpack(),
            Decimal::from_i128_with_scale(10_000_000_000_000_000_000_000_000_000, 13).unpack() // was Decimal::from_i128_with_scale(1_000_000_000_000_000_000_000_000_000, 12)
        );
    }

    #[test]
    fn from_str_no_rounding_0() {
        assert_eq!(
            parse_str_radix_10_exact("1.234").unwrap().unpack(),
            Decimal::new(1234, 3).unpack()
        );
    }

    #[test]
    fn from_str_no_rounding_1() {
        assert_eq!(
            parse_str_radix_10_exact("11111_11111_11111.11111_11111_11111"),
            Err(Error::Underflow)
        );
    }

    #[test]
    fn from_str_no_rounding_2() {
        assert_eq!(
            parse_str_radix_10_exact("11111_11111_11111.11111_11111_11115"),
            Err(Error::Underflow)
        );
    }

    #[test]
    fn from_str_no_rounding_3() {
        assert_eq!(
            parse_str_radix_10_exact("11111_11111_11111.11111_11111_11195"),
            Err(Error::Underflow)
        );
    }

    #[test]
    fn from_str_no_rounding_4() {
        assert_eq!(
            parse_str_radix_10_exact("99999_99999_99999.99999_99999_99995"),
            Err(Error::Underflow)
        );
    }

    #[test]
    fn from_str_many_pointless_chars() {
        assert_eq!(
            parse_str_radix_10("00________________________________________________________________001.1")
                .unwrap()
                .unpack(),
            Decimal::from_i128_with_scale(11, 1).unpack()
        );
    }

    #[test]
    fn from_str_leading_0s_1() {
        assert_eq!(
            parse_str_radix_10("00001.1").unwrap().unpack(),
            Decimal::from_i128_with_scale(11, 1).unpack()
        );
    }

    #[test]
    fn from_str_leading_0s_2() {
        assert_eq!(
            parse_str_radix_10("00000_00000_00000_00000_00001.00001")
                .unwrap()
                .unpack(),
            Decimal::from_i128_with_scale(100001, 5).unpack()
        );
    }

    #[test]
    fn from_str_leading_0s_3() {
        assert_eq!(
            parse_str_radix_10("0.00000_00000_00000_00000_00000_00100")
                .unwrap()
                .unpack(),
            Decimal::from_i128_with_scale(1, 28).unpack()
        );
    }

    #[test]
    fn from_str_trailing_0s_1() {
        assert_eq!(
            parse_str_radix_10("0.00001_00000_00000").unwrap().unpack(),
            Decimal::from_i128_with_scale(10_000_000_000, 15).unpack()
        );
    }

    #[test]
    fn from_str_trailing_0s_2() {
        assert_eq!(
            parse_str_radix_10("0.00001_00000_00000_00000_00000_00000")
                .unwrap()
                .unpack(),
            Decimal::from_i128_with_scale(100_000_000_000_000_000_000_000, 28).unpack()
        );
    }

    #[test]
    fn from_str_overflow_1() {
        assert_eq!(
            parse_str_radix_10("99999_99999_99999_99999_99999_99999.99999"),
            // The original implementation returned
            //              Ok(10000_00000_00000_00000_00000_0000)
            // Which is a bug!
            Err(Error::from("Invalid decimal: overflow from too many digits"))
        );
    }

    #[test]
    fn from_str_overflow_2() {
        assert!(
            parse_str_radix_10("99999_99999_99999_99999_99999_11111.11111").is_err(),
            // The original implementation is 'overflow from scale mismatch'
            // but we got rid of that now
        );
    }

    #[test]
    fn from_str_overflow_3() {
        assert!(
            parse_str_radix_10("99999_99999_99999_99999_99999_99994").is_err() // We could not get into 'overflow when rounding' or 'overflow from carry'
                                                                               // in the original implementation because the rounding logic before prevented it
        );
    }

    #[test]
    fn from_str_overflow_4() {
        assert_eq!(
            // This does not overflow, moving the decimal point 1 more step would result in
            // 'overflow from too many digits'
            parse_str_radix_10("99999_99999_99999_99999_99999_999.99")
                .unwrap()
                .unpack(),
            Decimal::from_i128_with_scale(10_000_000_000_000_000_000_000_000_000, 0).unpack()
        );
    }

    #[test]
    fn from_str_mantissa_overflow_1() {
        // reminder:
        assert_eq!(OVERFLOW_U96, 79_228_162_514_264_337_593_543_950_336);
        assert_eq!(
            parse_str_radix_10("79_228_162_514_264_337_593_543_950_33.56")
                .unwrap()
                .unpack(),
            Decimal::from_i128_with_scale(79_228_162_514_264_337_593_543_950_34, 0).unpack()
        );
        // This is a mantissa of OVERFLOW_U96 - 1 just before reaching the last digit.
        // Previously, this would return Err("overflow from mantissa after rounding")
        // instead of successfully rounding.
    }

    #[test]
    fn from_str_mantissa_overflow_2() {
        assert_eq!(
            parse_str_radix_10("79_228_162_514_264_337_593_543_950_335.6"),
            Err(Error::from("Invalid decimal: overflow from mantissa after rounding"))
        );
        // this case wants to round to 79_228_162_514_264_337_593_543_950_340.
        // (79_228_162_514_264_337_593_543_950_336 is OVERFLOW_U96 and too large
        // to fit in 96 bits) which is also too large for the mantissa so fails.
    }

    #[test]
    fn from_str_mantissa_overflow_3() {
        // this hits the other avoidable overflow case in maybe_round
        assert_eq!(
            parse_str_radix_10("7.92281625142643375935439503356").unwrap().unpack(),
            Decimal::from_i128_with_scale(79_228_162_514_264_337_593_543_950_34, 27).unpack()
        );
    }

    #[test]
    fn from_str_mantissa_overflow_4() {
        // Same test as above, however with underscores. This causes issues.
        assert_eq!(
            parse_str_radix_10("7.9_228_162_514_264_337_593_543_950_335_6")
                .unwrap()
                .unpack(),
            Decimal::from_i128_with_scale(79_228_162_514_264_337_593_543_950_34, 27).unpack()
        );
    }

    #[test]
    fn invalid_input_1() {
        assert_eq!(
            parse_str_radix_10("1.0000000000000000000000000000.5"),
            Err(Error::from("Invalid decimal: two decimal points"))
        );
    }

    #[test]
    fn invalid_input_2() {
        assert_eq!(
            parse_str_radix_10("1.0.5"),
            Err(Error::from("Invalid decimal: two decimal points"))
        );
    }

    #[test]
    fn character_at_rounding_position() {
        let tests = [
            // digit is at the rounding position
            (
                "1.000_000_000_000_000_000_000_000_000_04",
                Ok(Decimal::from_i128_with_scale(
                    1_000_000_000_000_000_000_000_000_000_0,
                    28,
                )),
            ),
            (
                "1.000_000_000_000_000_000_000_000_000_06",
                Ok(Decimal::from_i128_with_scale(
                    1_000_000_000_000_000_000_000_000_000_1,
                    28,
                )),
            ),
            // Decimal point is at the rounding position
            (
                "1_000_000_000_000_000_000_000_000_000_0.4",
                Ok(Decimal::from_i128_with_scale(
                    1_000_000_000_000_000_000_000_000_000_0,
                    0,
                )),
            ),
            (
                "1_000_000_000_000_000_000_000_000_000_0.6",
                Ok(Decimal::from_i128_with_scale(
                    1_000_000_000_000_000_000_000_000_000_1,
                    0,
                )),
            ),
            // Placeholder is at the rounding position
            (
                "1.000_000_000_000_000_000_000_000_000_0_4",
                Ok(Decimal::from_i128_with_scale(
                    1_000_000_000_000_000_000_000_000_000_0,
                    28,
                )),
            ),
            (
                "1.000_000_000_000_000_000_000_000_000_0_6",
                Ok(Decimal::from_i128_with_scale(
                    1_000_000_000_000_000_000_000_000_000_1,
                    28,
                )),
            ),
            // Multiple placeholders at rounding position
            (
                "1.000_000_000_000_000_000_000_000_000_0__4",
                Ok(Decimal::from_i128_with_scale(
                    1_000_000_000_000_000_000_000_000_000_0,
                    28,
                )),
            ),
            (
                "1.000_000_000_000_000_000_000_000_000_0__6",
                Ok(Decimal::from_i128_with_scale(
                    1_000_000_000_000_000_000_000_000_000_1,
                    28,
                )),
            ),
            (
                "1.234567890123456789012345678_9",
                Ok(Decimal::from_i128_with_scale(12345678901234567890123456789, 28)),
            ),
            (
                "0.234567890123456789012345678_9",
                Ok(Decimal::from_i128_with_scale(2345678901234567890123456789, 28)),
            ),
            (
                "0.1234567890123456789012345678_9",
                Ok(Decimal::from_i128_with_scale(1234567890123456789012345679, 28)),
            ),
            (
                "0.1234567890123456789012345678_4",
                Ok(Decimal::from_i128_with_scale(1234567890123456789012345678, 28)),
            ),
        ];

        for (input, expected) in tests.iter() {
            assert_eq!(parse_str_radix_10(input), *expected, "Test input {}", input);
        }
    }

    #[test]
    fn from_str_edge_cases_1() {
        assert_eq!(parse_str_radix_10(""), Err(Error::from("Invalid decimal: empty")));
    }

    #[test]
    fn from_str_edge_cases_2() {
        assert_eq!(
            parse_str_radix_10("0.1."),
            Err(Error::from("Invalid decimal: two decimal points"))
        );
    }

    #[test]
    fn from_str_edge_cases_3() {
        assert_eq!(
            parse_str_radix_10("_"),
            Err(Error::from("Invalid decimal: must start lead with a number"))
        );
    }

    #[test]
    fn from_str_edge_cases_4() {
        assert_eq!(
            parse_str_radix_10("1?2"),
            Err(Error::from("Invalid decimal: unknown character"))
        );
    }

    #[test]
    fn from_str_edge_cases_5() {
        assert_eq!(
            parse_str_radix_10("."),
            Err(Error::from("Invalid decimal: no digits found"))
        );
    }

    #[test]
    fn from_str_edge_cases_6() {
        // Decimal::MAX + 0.99999
        assert_eq!(
            parse_str_radix_10("79_228_162_514_264_337_593_543_950_335.99999"),
            Err(Error::from("Invalid decimal: overflow from mantissa after rounding"))
        );
    }
}
