//! Implementation of std::fmt traits & other stringification functions
//!

use crate::*;
use rounding::{NonDigitRoundingData, InsigData};
use stdlib::fmt::Write;
use stdlib::num::NonZeroUsize;


// const EXPONENTIAL_FORMAT_LEADING_ZERO_THRESHOLD: usize = ${RUST_BIGDECIMAL_FMT_EXPONENTIAL_LOWER_THRESHOLD} or 5;
// const EXPONENTIAL_FORMAT_TRAILING_ZERO_THRESHOLD: usize = ${RUST_BIGDECIMAL_FMT_EXPONENTIAL_UPPER_THRESHOLD} or 15;
// const FMT_MAX_INTEGER_PADDING: usize = = ${RUST_BIGDECIMAL_FMT_MAX_INTEGER_PADDING} or  1000;
include!(concat!(env!("OUT_DIR"), "/exponential_format_threshold.rs"));


impl fmt::Display for BigDecimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        dynamically_format_decimal(
            self.to_ref(),
            f,
            EXPONENTIAL_FORMAT_LEADING_ZERO_THRESHOLD,
            EXPONENTIAL_FORMAT_TRAILING_ZERO_THRESHOLD,
        )
    }
}

impl fmt::Display for BigDecimalRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        dynamically_format_decimal(
            *self,
            f,
            EXPONENTIAL_FORMAT_LEADING_ZERO_THRESHOLD,
            EXPONENTIAL_FORMAT_TRAILING_ZERO_THRESHOLD,
        )
    }
}


impl fmt::LowerExp for BigDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerExp::fmt(&self.to_ref(), f)
    }
}

impl fmt::LowerExp for BigDecimalRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (abs_int, scale) = get_abs_int_scale(*self);
        format_exponential(f, abs_int, self.sign, scale, "e")
    }
}


impl fmt::UpperExp for BigDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperExp::fmt(&self.to_ref(), f)
    }
}

impl fmt::UpperExp for BigDecimalRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (abs_int, scale) = get_abs_int_scale(*self);
        format_exponential(f, abs_int, self.sign, scale, "E")
    }
}


impl fmt::Debug for BigDecimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "BigDecimal(\"{}e{:}\")", self.int_val, -self.scale)
        } else {
            write!(f,
                "BigDecimal(sign={:?}, scale={}, digits={:?})",
                self.sign(), self.scale, self.int_val.magnitude().to_u64_digits()
            )
        }
    }
}

fn get_abs_int_scale(this: BigDecimalRef) -> (String, i64) {
    // Acquire the absolute integer as a decimal string
    let abs_int = this.digits.to_str_radix(10);
    // Special-case 0: no zero-padding should be done.
    let scale = if this.is_zero() { 0 } else { this.scale };

    (abs_int, scale)
}


fn dynamically_format_decimal(
    this: BigDecimalRef,
    f: &mut fmt::Formatter,
    leading_zero_threshold: usize,
    trailing_zero_threshold: usize,
) -> fmt::Result {
    let (abs_int, scale) = get_abs_int_scale(this);

    // number of zeros between most significant digit and decimal point
    let leading_zero_count = scale
                                 .to_u64()
                                 .and_then(|scale| scale.checked_sub(abs_int.len() as u64))
                                 .unwrap_or(0);

    // number of zeros between least significant digit and decimal point
    let trailing_zero_count = scale
                                  .checked_neg()
                                  .and_then(|d| d.to_u64());

    // this ignores scientific-formatting if precision is requested
    let trailing_zeros = f.precision().map(|_| 0)
                          .or(trailing_zero_count)
                          .unwrap_or(0);

    let leading_zero_threshold = leading_zero_threshold as u64;
    let trailing_zero_threshold = trailing_zero_threshold as u64;

    // use exponential form if decimal point is outside
    // the upper and lower thresholds of the decimal,
    // and precision was not requested
    if f.precision().is_none() && leading_zero_threshold < leading_zero_count {
        format_exponential(f, abs_int, this.sign, scale, "E")
    } else if trailing_zero_threshold < trailing_zeros {
        // non-scientific notation
        format_dotless_exponential(f, abs_int, this.sign, scale, "e")
    } else {
        format_full_scale(f, abs_int, this.sign, scale)
    }
}


pub(crate) struct FullScaleFormatter<'a>(pub BigDecimalRef<'a>);

impl fmt::Display for FullScaleFormatter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.0;
        let non_negative = matches!(n.sign, Sign::Plus | Sign::NoSign);

        let mut digits = n.digits.to_string();

        if n.scale <= 0 {
            digits.extend(stdlib::iter::repeat('0').take(n.scale.neg() as usize));
        } else if n.scale < digits.len() as i64 {
            digits.insert(digits.len() - n.scale as usize, '.');
        } else {
            let mut digit_vec = digits.into_bytes();

            let dest_str_size = n.scale as usize + 2;
            let digit_count = digit_vec.len();
            let leading_char_idx = dest_str_size - digit_count;

            digit_vec.resize(dest_str_size, b'0');
            digit_vec.copy_within(0..digit_count, leading_char_idx);
            fill_slice(&mut digit_vec[..digit_count.min(leading_char_idx)], b'0');

            digit_vec[1] = b'.';
            digits = String::from_utf8(digit_vec).unwrap();
        }

        f.pad_integral(non_negative, "", &digits)
    }
}


fn format_full_scale(
    f: &mut fmt::Formatter,
    abs_int: String,
    sign: Sign,
    scale: i64,
) -> fmt::Result {
    use stdlib::cmp::Ordering::*;

    let mut digits = abs_int.into_bytes();
    let mut exp = 0;
    let non_negative = matches!(sign, Sign::Plus | Sign::NoSign);

    debug_assert_ne!(digits.len(), 0);

    let rounder = NonDigitRoundingData::default_with_sign(sign);

    if scale <= 0 {
        exp = (scale as i128).neg();
        // format an integer value by adding trailing zeros to the right
        zero_right_pad_integer_ascii_digits(&mut digits, &mut exp, f.precision());
    } else {
        let scale = scale as u64;

        // std::fmt 'precision' has same meaning as bigdecimal 'scale'
        //
        // interpret 'no-precision' to mean the same as matching the scale
        // of the deicmal (i.e. no padding or rounding)
        let target_scale = f.precision().and_then(|prec| prec.to_u64()).unwrap_or(scale);

        if scale < digits.len() as u64 {
            // format both integer and fractional digits (always 'trim' to precision)
            format_ascii_digits_with_integer_and_fraction(&mut digits, scale, target_scale, rounder);
        } else {
            // format only fractional digits
            format_ascii_digits_no_integer(&mut digits, scale, target_scale, rounder);
        }
    }

    // move digits back into String form
    let mut buf = String::from_utf8(digits).unwrap();

    // add exp part to buffer (if not zero)
    if exp != 0 {
        write!(buf, "e{:+}", exp)?;
    }

    // write buffer to formatter
    f.pad_integral(non_negative, "", &buf)
}

/// Fill appropriate number of zeros and decimal point into Vec of (ascii/utf-8) digits
///
/// Exponent is set to zero if zeros were added
///
fn zero_right_pad_integer_ascii_digits(
    digits: &mut Vec<u8>,
    exp: &mut i128,
    // number of zeros after the decimal point
    target_scale: Option<usize>,
) {
    debug_assert!(*exp >= 0);
    debug_assert_ne!(digits.len(), 0);

    let integer_zero_count = match exp.to_usize() {
        Some(n) => n,
        None => { return; }
    };

    // did not explicitly request precision, so we'll only
    // implicitly right-pad if less than this threshold.
    if target_scale.is_none() && integer_zero_count > 20 {
        // no padding
        return;
    }

    let fraction_zero_char_count;
    let decimal_place_idx;

    if let Some(frac_zero_count) = target_scale.and_then(NonZeroUsize::new) {
        // add one char for '.' if target_scale is not zero
        fraction_zero_char_count = frac_zero_count.get() + 1;
        // indicate we'll need to add a decimal point
        decimal_place_idx = Some(digits.len() + integer_zero_count);
    } else {
        fraction_zero_char_count = 0;
        decimal_place_idx = None;
    }

    let total_additional_zeros = integer_zero_count.saturating_add(fraction_zero_char_count);

    // no padding if out of bounds
    if total_additional_zeros > FMT_MAX_INTEGER_PADDING {
        return;
    }

    digits.resize(digits.len() + total_additional_zeros, b'0');
    if let Some(decimal_place_idx) = decimal_place_idx {
        digits[decimal_place_idx] = b'.';
    }

    // set exp to zero so it won't be written in `format_full_scale`
    *exp = 0;
}


/// Insert decimal point into digits_ascii_be, rounding or padding with zeros when necessary
///
/// (digits_ascii_be, scale) represents a decimal with both integer and fractional digits.
///
fn format_ascii_digits_with_integer_and_fraction(
    digits_ascii_be: &mut Vec<u8>,
    scale: u64,
    target_scale: u64,
    rounder: NonDigitRoundingData,
) {
    debug_assert!(scale < digits_ascii_be.len() as u64, "No integer digits");
    let mut digit_scale = scale;

    // decimal has more fractional digits than requested: round (trimming insignificant digits)
    if target_scale < scale {
        let digit_count_to_remove = (scale - target_scale)
                                    .to_usize()
                                    .expect("Precision exceeds maximum usize");

        let rounding_idx = NonZeroUsize::new(digits_ascii_be.len() - digit_count_to_remove)
                           .expect("decimal must have integer digits");

        // round and trim the digits at the 'rounding index'
        let scale_diff = round_ascii_digits(digits_ascii_be, rounding_idx, rounder);

        match scale_diff.checked_sub(digit_scale as usize) {
            None | Some(0) => {
                digit_scale -= scale_diff as u64;
            }
            Some(zeros_to_add) => {
                digits_ascii_be.resize(digits_ascii_be.len() + zeros_to_add, b'0');
                digit_scale = 0;
            }
        }
    }

    // ignore the decimal point if the target scale is zero
    if target_scale != 0 {
        // there are both integer and fractional digits
        let integer_digit_count = (digits_ascii_be.len() as u64 - digit_scale)
                                  .to_usize()
                                  .expect("Number of digits exceeds maximum usize");


        digits_ascii_be.insert(integer_digit_count, b'.');
    }

    if digit_scale < target_scale {
        let trailing_zero_count = (target_scale - digit_scale)
                                  .to_usize()
                                  .expect("Too Big");

        // precision required beyond scale
        digits_ascii_be.resize(digits_ascii_be.len() + trailing_zero_count, b'0');
        digit_scale += trailing_zero_count as u64;
    }

    debug_assert_eq!(digit_scale, target_scale);
}


/// Insert decimal point into digits_ascii_be, rounding or padding with zeros when necessary
///
/// (digits_ascii_be, scale) represents a decimal with only fractional digits.
///
fn format_ascii_digits_no_integer(
    digits_ascii_be: &mut Vec<u8>,
    scale: u64,
    target_scale: u64,
    rounder: NonDigitRoundingData,
) {
    use stdlib::cmp::Ordering::*;

    debug_assert!(scale >= digits_ascii_be.len() as u64);
    let leading_zeros = scale - digits_ascii_be.len() as u64;

    match arithmetic::diff(target_scale, leading_zeros) {
        // handle rounding point before the start of digits
        (Less, intermediate_zeros) | (Equal, intermediate_zeros)  => {
            // get insignificant digit
            let (insig_digit, trailing_digits) = if intermediate_zeros > 0 {
                (0, digits_ascii_be.as_slice())
            } else {
                (digits_ascii_be[0] - b'0', &digits_ascii_be[1..])
            };

            let insig_data = InsigData::from_digit_and_lazy_trailing_zeros(
                rounder, insig_digit, || trailing_digits.iter().all(|&d| d == b'0')
            );

            let rounded_digit = insig_data.round_digit(0);
            debug_assert_ne!(rounded_digit, 10);

            digits_ascii_be.clear();

            if target_scale > 0 {
                digits_ascii_be.resize(target_scale as usize + 1, b'0');
            }

            digits_ascii_be.push(rounded_digit + b'0');

            if target_scale > 0 {
                digits_ascii_be[1] = b'.';
            }
        }
        (Greater, sig_digit_count) => {
            let significant_digit_count = sig_digit_count
                                          .to_usize()
                                          .and_then(NonZeroUsize::new)
                                          .expect("Request overflow in sig_digit_count");

            let mut digit_scale = scale;

            // if 'digits_ascii_be' has more digits than requested, round
            if significant_digit_count.get() < digits_ascii_be.len() {
                let removed_digit_count = round_ascii_digits(
                    digits_ascii_be, significant_digit_count, rounder
                );

                digit_scale -= removed_digit_count as u64;
            }

            // number of zeros to keep after the significant digits
            let trailing_zeros = target_scale - digit_scale;

            // expected length is target scale (number of digits after decimal point) + "0."
            let dest_len = target_scale as usize + 2;

            // number of significant digits is whatever is left in the digit vector
            let sig_digit_count = digits_ascii_be.len();

            // index where to store significant digits
            let sig_digit_idx = dest_len - trailing_zeros as usize - sig_digit_count;

            // fill output with zeros
            digits_ascii_be.resize(dest_len, b'0');

            // very likely case where there are digits after the decimal point
            if digit_scale != 0 {
                // copy significant digits to their index location
                digits_ascii_be.copy_within(..sig_digit_count, sig_digit_idx);

                // clear copied values
                fill_slice(&mut digits_ascii_be[..sig_digit_count.min(sig_digit_idx)], b'0');
            } else {
                debug_assert_eq!(sig_digit_count, 1);
            }

            // add decimal point
            digits_ascii_be[1] = b'.';
        }
    }
}

#[cfg(rust_1_50)]
fn fill_slice<T: Clone>(v: &mut [T], value: T) {
    v.fill(value);
}

#[cfg(not(rust_1_50))]
fn fill_slice<T: Clone>(v: &mut [T], value: T) {
    for i in v.iter_mut() {
        *i = value.clone();
    }
}


/// Format integer as {int}e+{exp}
///
/// Slightly different than scientific notation,
///
fn format_dotless_exponential(
    f: &mut fmt::Formatter,
    mut abs_int: String,
    sign: Sign,
    scale: i64,
    e_symbol: &str,
) -> fmt::Result {
    debug_assert!(scale <= 0);

    write!(abs_int, "{}{:+}", e_symbol, -scale).unwrap();
    let non_negative = matches!(sign, Sign::Plus | Sign::NoSign);
    f.pad_integral(non_negative, "", &abs_int)
}

fn format_exponential(
    f: &mut fmt::Formatter,
    abs_int: String,
    sign: Sign,
    scale: i64,
    e_symbol: &str,
) -> fmt::Result {
    // Steps:
    //  1. Truncate integer based on precision
    //  2. calculate exponent from the scale and the length of the internal integer
    //  3. Place decimal point after a single digit of the number, or omit if there is only a single digit
    //  4. Append `E{exponent}` and format the resulting string based on some `Formatter` flags

    let exp = (scale as i128).neg();
    let digits = abs_int.into_bytes();

    format_exponential_bigendian_ascii_digits(
        digits, sign, exp, f, e_symbol
    )
}


fn format_exponential_bigendian_ascii_digits(
    mut digits: Vec<u8>,
    sign: Sign,
    mut exp: i128,
    f: &mut fmt::Formatter,
    e_symbol: &str,
) -> fmt::Result {
    // how many zeros to pad at the end of the decimal
    let mut extra_trailing_zero_count = 0;

    if let Some(prec) = f.precision() {
        // 'prec' is number of digits after the decimal point
        let total_prec = prec + 1;

        if total_prec < digits.len() {
            // round to smaller precision
            let rounder = NonDigitRoundingData::default_with_sign(sign);
            let target_scale = NonZeroUsize::new(total_prec).unwrap();
            let delta_exp = round_ascii_digits(&mut digits, target_scale, rounder);
            exp += delta_exp as i128;
        }

        extra_trailing_zero_count = total_prec - digits.len();
    }

    let needs_decimal_point = digits.len() > 1 || extra_trailing_zero_count > 0;

    let mut abs_int = String::from_utf8(digits).unwrap();

    // Determine the exponent value based on the scale
    //
    // # First case: the integer representation falls completely behind the
    //   decimal point.
    //
    //   Example of this.scale > abs_int.len():
    //   0.000001234509876
    //   abs_int.len() = 10
    //   scale = 15
    //   target is 1.234509876
    //   exponent = -6
    //
    //   Example of this.scale == abs_int.len():
    //   0.333333333333333314829616256247390992939472198486328125
    //   abs_int.len() = 54
    //   scale = 54
    //   target is 3.33333333333333314829616256247390992939472198486328125
    //   exponent = -1
    //
    // # Second case: the integer representation falls around, or before the
    //   decimal point
    //
    //   ## Case 2.1, entirely before the decimal point.
    //     Example of (abs_int.len() - this.scale) > abs_int.len():
    //     123450987600000
    //     abs_int.len() = 10
    //     scale = -5
    //     location = 15
    //     target is 1.234509876
    //     exponent = 14
    //
    //   ## Case 2.2, somewhere around the decimal point.
    //     Example of (abs_int.len() - this.scale) < abs_int.len():
    //     12.339999999999999857891452847979962825775146484375
    //     abs_int.len() = 50
    //     scale = 48
    //     target is 1.2339999999999999857891452847979962825775146484375
    //     exponent = 1
    //
    //     For the (abs_int.len() - this.scale) == abs_int.len() I couldn't
    //     come up with an example
    let exponent = abs_int.len() as i128 + exp - 1;

    if needs_decimal_point {
        // only add decimal point if there is more than 1 decimal digit
        abs_int.insert(1, '.');
    }

    if extra_trailing_zero_count > 0 {
        abs_int.extend(stdlib::iter::repeat('0').take(extra_trailing_zero_count));
    }

    // always print exponent in exponential mode
    write!(abs_int, "{}{:+}", e_symbol, exponent)?;

    let non_negative = matches!(sign, Sign::Plus | Sign::NoSign);
    //pad_integral does the right thing although we have a decimal
    f.pad_integral(non_negative, "", &abs_int)
}


/// Round big-endian ascii digits to given significant digit count,
/// updating the scale appropriately
///
/// Returns the number of digits removed; this should be treated as the
/// change in the decimal's scale, and should be subtracted from the scale
/// when appropriate.
///
fn round_ascii_digits(
    // bigendian ascii digits
    digits_ascii_be: &mut Vec<u8>,
    // number of significant digits to keep
    significant_digit_count: NonZeroUsize,
    // how to round
    rounder: NonDigitRoundingData,
) -> usize {
    debug_assert!(significant_digit_count.get() < digits_ascii_be.len());

    let (sig_digits, insig_digits) = digits_ascii_be.split_at(significant_digit_count.get());
    let (&insig_digit, trailing_digits) = insig_digits.split_first().unwrap_or((&b'0', &[]));

    let insig_data = InsigData::from_digit_and_lazy_trailing_zeros(
        rounder, insig_digit - b'0', || trailing_digits.iter().all(|&d| d == b'0')
    );

    let rounding_digit_pos = significant_digit_count.get() - 1;
    let sig_digit = sig_digits[rounding_digit_pos] - b'0';
    let rounded_digit = insig_data.round_digit(sig_digit);

    // record how many digits to remove (changes the 'scale')
    let mut removed_digit_count = insig_digits.len();

    // discard insignificant digits (and rounding digit)
    digits_ascii_be.truncate(rounding_digit_pos);

    if rounded_digit < 10 {
        // simple case: no carrying/overflow, push rounded digit
        digits_ascii_be.push(rounded_digit + b'0');
        return removed_digit_count;
    }

    // handle carrying the 1
    debug_assert!(rounded_digit == 10);

    // carry the one past trailing 9's: replace them with zeros
    let next_non_nine_rev_pos = digits_ascii_be.iter().rev().position(|&d| d != b'9');
    match next_non_nine_rev_pos {
        // number of nines to remove
        Some(backwards_nine_count) => {
            let digits_to_trim = backwards_nine_count + 1;
            let idx = digits_ascii_be.len() - digits_to_trim;
            // increment least significant non-nine zero
            digits_ascii_be[idx] += 1;
            // remove trailing nines
            digits_ascii_be.truncate(idx + 1);
            // count truncation
            removed_digit_count += digits_to_trim;
        }
        // all nines! overflow to 1.000
        None => {
            digits_ascii_be.clear();
            digits_ascii_be.push(b'1');
            // count digit clearning
            removed_digit_count += significant_digit_count.get();
        }
    }

    // return the number of removed digits
    return removed_digit_count;
}


#[inline(never)]
pub(crate) fn write_scientific_notation<W: Write>(n: &BigDecimal, w: &mut W) -> fmt::Result {
    if n.is_zero() {
        return w.write_str("0e0");
    }

    if n.int_val.sign() == Sign::Minus {
        w.write_str("-")?;
    }

    let digits = n.int_val.magnitude();

    let dec_str = digits.to_str_radix(10);
    let (first_digit, remaining_digits) = dec_str.as_str().split_at(1);
    w.write_str(first_digit)?;
    if !remaining_digits.is_empty() {
        w.write_str(".")?;
        w.write_str(remaining_digits)?;
    }
    write!(w, "e{}", remaining_digits.len() as i128 - n.scale as i128)
}


#[inline(never)]
pub(crate) fn write_engineering_notation<W: Write>(n: &BigDecimal, out: &mut W) -> fmt::Result {
    if n.is_zero() {
        return out.write_str("0e0");
    }

    if n.int_val.sign() == Sign::Minus {
        out.write_char('-')?;
    }

    let digits = n.int_val.magnitude();

    let dec_str = digits.to_str_radix(10);
    let digit_count = dec_str.len();

    let top_digit_exponent = digit_count as i128 - n.scale as i128;

    let shift_amount = match top_digit_exponent.rem_euclid(3) {
        0 => 3,
        i => i as usize,
    };

    let exp = top_digit_exponent - shift_amount as i128;

    // handle adding zero padding
    if let Some(padding_zero_count) = shift_amount.checked_sub(dec_str.len()) {
        let zeros = &"000"[..padding_zero_count];
        out.write_str(&dec_str)?;
        out.write_str(zeros)?;
        return write!(out, "e{}", exp);
    }

    let (head, rest) = dec_str.split_at(shift_amount);
    debug_assert_eq!(exp % 3, 0);

    out.write_str(head)?;

    if !rest.is_empty() {
        out.write_char('.')?;
        out.write_str(rest)?;
    }

    return write!(out, "e{}", exp);
}


#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;
    use paste::*;

    /// test case builder for mapping decimal-string to formatted-string
    /// define test_fmt_function! macro to test your function
    #[cfg(test)]
    macro_rules! impl_case {
        ($name:ident : $in:literal => $ex:literal) => {
            #[test]
            fn $name() {
                let n: BigDecimal = $in.parse().unwrap();
                let s = test_fmt_function!(n);
                assert_eq!(&s, $ex);
            }
        };
    }

    /// "Mock" Formatter
    ///
    /// Given callable, forwards formatter to callable.
    /// Required work-around due to lack of constructor in fmt::Formatter
    ///
    struct Fmt<F>(F)
        where F: Fn(&mut fmt::Formatter) -> fmt::Result;

    impl<F> fmt::Display for Fmt<F>
        where F: Fn(&mut fmt::Formatter) -> fmt::Result
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // call closure with given formatter
            (self.0)(f)
        }
    }

    impl<F> fmt::Debug for Fmt<F>
        where F: Fn(&mut fmt::Formatter) -> fmt::Result
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            (self.0)(f)
        }
    }

    mod dynamic_fmt {
        use super::*;

        macro_rules! test_fmt_function {
            ($n:ident) => {{
                format!("{}", Fmt(|f| dynamically_format_decimal($n.to_ref(), f, 2, 9)))
            }};
        }

        impl_case!(case_0d123: "0.123" => "0.123");
        impl_case!(case_0d0123: "0.0123" => "0.0123");
        impl_case!(case_0d00123: "0.00123" => "0.00123");
        impl_case!(case_0d000123: "0.000123" => "1.23E-4");

        impl_case!(case_123d: "123." => "123");
        impl_case!(case_123de1: "123.e1" => "1230");
    }

    mod fmt_options {
        use super::*;

        macro_rules! impl_case {
            ($name:ident: $fmt:literal => $expected:literal) => {
                #[test]
                fn $name() {
                    let x = test_input();
                    let y = format!($fmt, x);
                    assert_eq!(y, $expected);
                }
            };
        }

        mod dec_1 {
            use super::*;

            fn test_input() -> BigDecimal {
                "1".parse().unwrap()
            }

            impl_case!(fmt_default:      "{}" => "1");
            impl_case!(fmt_d1:        "{:.1}" => "1.0");
            impl_case!(fmt_d4:        "{:.4}" => "1.0000");
            impl_case!(fmt_4d1:      "{:4.1}" => " 1.0");
            impl_case!(fmt_r4d1:    "{:>4.1}" => " 1.0");
            impl_case!(fmt_l4d1:    "{:<4.1}" => "1.0 ");
            impl_case!(fmt_p05d1:  "{:+05.1}" => "+01.0");

            impl_case!(fmt_e:      "{:e}" => "1e+0");
            impl_case!(fmt_E:      "{:E}" => "1E+0");
        }

        mod dec_1e1 {
            use super::*;

            fn test_input() -> BigDecimal {
                BigDecimal::new(1.into(), -1)
            }

            impl_case!(fmt_default:      "{}" => "10");
            impl_case!(fmt_debug:      "{:?}" => "BigDecimal(sign=Plus, scale=-1, digits=[1])");
            impl_case!(fmt_debug_alt: "{:#?}" => "BigDecimal(\"1e1\")");
            impl_case!(fmt_d0:        "{:.0}" => "10");
            impl_case!(fmt_d1:        "{:.1}" => "10.0");
            impl_case!(fmt_d2:        "{:.2}" => "10.00");
        }

        mod dec_1en1 {
            use super::*;

            fn test_input() -> BigDecimal {
                BigDecimal::new(1.into(), 1)
            }

            impl_case!(fmt_default:      "{}" => "0.1");
            impl_case!(fmt_d0:        "{:.0}" => "0");
            impl_case!(fmt_d1:        "{:.1}" => "0.1");
            impl_case!(fmt_d2:        "{:.2}" => "0.10");
        }

        mod dec_9en1 {
            use super::*;

            fn test_input() -> BigDecimal {
                BigDecimal::new(9.into(), 1)
            }

            impl_case!(fmt_default:      "{}" => "0.9");
            impl_case!(fmt_d0:        "{:.0}" => "1");
            impl_case!(fmt_d1:        "{:.1}" => "0.9");
            impl_case!(fmt_d4:        "{:.4}" => "0.9000");
        }

        mod dec_800en3 {
            use super::*;

            fn test_input() -> BigDecimal {
                BigDecimal::new(800.into(), 3)
            }

            impl_case!(fmt_default:      "{}" => "0.800");
            impl_case!(fmt_d0:        "{:.0}" => "1");
            impl_case!(fmt_d1:        "{:.1}" => "0.8");
            impl_case!(fmt_d3:        "{:.3}" => "0.800");
            impl_case!(fmt_d9:        "{:.9}" => "0.800000000");
        }

        mod dec_123456 {
            use super::*;

            fn test_input() -> BigDecimal {
                "123456".parse().unwrap()
            }

            impl_case!(fmt_default:      "{}" => "123456");
            impl_case!(fmt_d1:        "{:.1}" => "123456.0");
            impl_case!(fmt_d4:        "{:.4}" => "123456.0000");
            impl_case!(fmt_4d1:      "{:4.1}" => "123456.0");
            impl_case!(fmt_15d2:    "{:15.2}" => "      123456.00");
            impl_case!(fmt_r15d2:  "{:>15.2}" => "      123456.00");
            impl_case!(fmt_l15d2:  "{:<15.2}" => "123456.00      ");
            impl_case!(fmt_p05d1:  "{:+05.7}" => "+123456.0000000");
        }

        mod dec_d099995 {
            use super::*;

            fn test_input() -> BigDecimal {
                ".099995".parse().unwrap()
            }

            impl_case!(fmt_default:  "{}" => "0.099995");
            impl_case!(fmt_d0:  "{:.0}" => "0");
            impl_case!(fmt_d1:  "{:.1}" => "0.1");
            impl_case!(fmt_d3:  "{:.3}" => "0.100");
        }

        mod dec_d9999999 {
            use super::*;

            fn test_input() -> BigDecimal {
                ".9999999".parse().unwrap()
            }

            impl_case!(fmt_default:  "{}" => "0.9999999");
            impl_case!(fmt_d0:  "{:.0}" => "1");
            impl_case!(fmt_d1:  "{:.1}" => "1.0");
            impl_case!(fmt_d3:  "{:.3}" => "1.000");
        }

        mod dec_9999999 {
            use super::*;

            fn test_input() -> BigDecimal {
                "9999999".parse().unwrap()
            }

            impl_case!(fmt_default:  "{}" => "9999999");
            impl_case!(fmt_d1:  "{:.1}" => "9999999.0");
            impl_case!(fmt_d8:  "{:.8}" => "9999999.00000000");

            impl_case!(fmt_e:  "{:e}" => "9.999999e+6");
            impl_case!(fmt_E:  "{:E}" => "9.999999E+6");
            impl_case!(fmt_d0e:  "{:.0e}" => "1e+7");
            impl_case!(fmt_d1e:  "{:.1e}" => "1.0e+7");
            impl_case!(fmt_d2e:  "{:.2e}" => "1.00e+7");
            impl_case!(fmt_d4e:  "{:.4e}" => "1.0000e+7");
            impl_case!(fmt_d6e:  "{:.6e}" => "9.999999e+6");
            impl_case!(fmt_d7e:  "{:.7e}" => "9.9999990e+6");
            impl_case!(fmt_d10e:  "{:.10e}" => "9.9999990000e+6");
        }

        mod dec_19073d97235939614856 {
            use super::*;

            fn test_input() -> BigDecimal {
                "19073.97235939614856".parse().unwrap()
            }

            impl_case!(fmt_default:      "{}" => "19073.97235939614856");
            impl_case!(fmt_pd7:      "{:+.7}" => "+19073.9723594");
            impl_case!(fmt_d0:        "{:.0}" => "19074");
            impl_case!(fmt_d1:        "{:.1}" => "19074.0");
            impl_case!(fmt_d3:        "{:.3}" => "19073.972");
            impl_case!(fmt_d4:        "{:.4}" => "19073.9724");
            impl_case!(fmt_8d3:      "{:8.3}" => "19073.972");
            impl_case!(fmt_10d3:    "{:10.3}" => " 19073.972");
            impl_case!(fmt_010d3:  "{:010.3}" => "019073.972");
        }

        mod dec_10950633712399d557 {
            use super::*;

            fn test_input() -> BigDecimal {
                "10950633712399.557".parse().unwrap()
            }

            impl_case!(fmt_default:  "{}" => "10950633712399.557");
            impl_case!(fmt_d0:    "{:.0}" => "10950633712400");
            impl_case!(fmt_d1:    "{:.1}" => "10950633712399.6");
            impl_case!(fmt_d2:    "{:.2}" => "10950633712399.56");
            impl_case!(fmt_d3:    "{:.3}" => "10950633712399.557");
            impl_case!(fmt_d4:    "{:.4}" => "10950633712399.5570");
        }

        mod dec_n90037659d6902 {
            use super::*;

            fn test_input() -> BigDecimal {
                "-90037659.6905".parse().unwrap()
            }

            impl_case!(fmt_default:      "{}" => "-90037659.6905");
            impl_case!(fmt_debug:      "{:?}" => "BigDecimal(sign=Minus, scale=4, digits=[900376596905])");
            impl_case!(fmt_debug_alt: "{:#?}" => "BigDecimal(\"-900376596905e-4\")");
            impl_case!(fmt_pd7:      "{:+.7}" => "-90037659.6905000");
            impl_case!(fmt_d0:        "{:.0}" => "-90037660");
            impl_case!(fmt_d3:        "{:.3}" => "-90037659.690");
            impl_case!(fmt_d4:        "{:.4}" => "-90037659.6905");
            impl_case!(fmt_14d4:    "{:14.4}" => "-90037659.6905");
            impl_case!(fmt_15d4:    "{:15.4}" => " -90037659.6905");
            impl_case!(fmt_l17d5:  "{:<17.5}" => "-90037659.69050  ");
        }

        mod dec_0d0002394899999500 {
            use super::*;

            fn test_input() -> BigDecimal {
                "0.0002394899999500".parse().unwrap()
            }

            impl_case!(fmt_default:    "{}" => "0.0002394899999500");
            impl_case!(fmt_d0:      "{:.0}" => "0");
            impl_case!(fmt_d1:      "{:.1}" => "0.0");
            impl_case!(fmt_d3:      "{:.3}" => "0.000");
            impl_case!(fmt_d4:      "{:.4}" => "0.0002");
            impl_case!(fmt_d5:      "{:.5}" => "0.00024");
            impl_case!(fmt_d10:    "{:.10}" => "0.0002394900");
            impl_case!(fmt_d13:    "{:.13}" => "0.0002394900000");
            impl_case!(fmt_d14:    "{:.14}" => "0.00023948999995");
            impl_case!(fmt_d20:    "{:.20}" => "0.00023948999995000000");

        }

        mod dec_1764031078en13 {
            use super::*;

            fn test_input() -> BigDecimal {
                BigDecimal::new(1764031078.into(), 13)
            }

            impl_case!(fmt_default:    "{}" => "0.0001764031078");
            impl_case!(fmt_d1:      "{:.1}" => "0.0");
            impl_case!(fmt_d3:      "{:.3}" => "0.000");
            impl_case!(fmt_d4:      "{:.4}" => "0.0002");
            impl_case!(fmt_d5:      "{:.5}" => "0.00018");
            impl_case!(fmt_d13:    "{:.13}" => "0.0001764031078");
            impl_case!(fmt_d20:    "{:.20}" => "0.00017640310780000000");

        }

        mod dec_1e15 {
            use super::*;

            fn test_input() -> BigDecimal {
                "1e15".parse().unwrap()
            }

            impl_case!(fmt_default:        "{}" => "1000000000000000");
            impl_case!(fmt_d0:          "{:.0}" => "1000000000000000");
            impl_case!(fmt_d1:          "{:.1}" => "1000000000000000.0");
        }

        mod dec_1e16 {
            use super::*;

            fn test_input() -> BigDecimal {
                "1e16".parse().unwrap()
            }

            impl_case!(fmt_default:        "{}" => "1e+16");
            impl_case!(fmt_d0:          "{:.0}" => "10000000000000000");
            impl_case!(fmt_d2:          "{:.2}" => "10000000000000000.00");
        }


        mod dec_491326en12 {
            use super::*;

            fn test_input() -> BigDecimal {
                "491326e-12".parse().unwrap()
            }

            impl_case!(fmt_default:     "{}" => "4.91326E-7");
            impl_case!(fmt_d0:       "{:.0}" => "0");
            impl_case!(fmt_d1:       "{:.1}" => "0.0");
            impl_case!(fmt_d3:       "{:.3}" => "0.000");
            impl_case!(fmt_d5:       "{:.5}" => "0.00000");
            impl_case!(fmt_d6:       "{:.7}" => "0.0000005");
            impl_case!(fmt_d9:       "{:.9}" => "0.000000491");
            impl_case!(fmt_d20:     "{:.20}" => "0.00000049132600000000");

            impl_case!(fmt_d0e:     "{:.0E}" => "5E-7");
            impl_case!(fmt_d1e:     "{:.1E}" => "4.9E-7");
            impl_case!(fmt_d3e:     "{:.3E}" => "4.913E-7");
            impl_case!(fmt_d5e:     "{:.5E}" => "4.91326E-7");
            impl_case!(fmt_d6e:     "{:.6E}" => "4.913260E-7");
        }

        mod dec_0d00003102564500 {
            use super::*;

            fn test_input() -> BigDecimal {
                "0.00003102564500".parse().unwrap()
            }

            impl_case!(fmt_default: "{}" => "0.00003102564500");
            impl_case!(fmt_d0:   "{:.0}" => "0");
            impl_case!(fmt_d1:   "{:.1}" => "0.0");
            impl_case!(fmt_d2:   "{:.2}" => "0.00");
            impl_case!(fmt_d4:   "{:.4}" => "0.0000");
            impl_case!(fmt_d5:   "{:.5}" => "0.00003");
            impl_case!(fmt_d10:  "{:.10}" => "0.0000310256");
            impl_case!(fmt_d14:  "{:.14}" => "0.00003102564500");
            impl_case!(fmt_d17:  "{:.17}" => "0.00003102564500000");

            impl_case!(fmt_e:      "{:e}" => "3.102564500e-5");
            impl_case!(fmt_de:    "{:.e}" => "3.102564500e-5");
            impl_case!(fmt_d0e:  "{:.0e}" => "3e-5");
            impl_case!(fmt_d1e:  "{:.1e}" => "3.1e-5");
            impl_case!(fmt_d4e:  "{:.4e}" => "3.1026e-5");
        }

        mod dec_1en100000 {
            use super::*;

            fn test_input() -> BigDecimal {
                "1E-10000".parse().unwrap()
            }

            impl_case!(fmt_default: "{}" => "1E-10000");
            impl_case!(fmt_d:    "{:.0}" => "0");
            impl_case!(fmt_d1:   "{:.1}" => "0.0");
            impl_case!(fmt_d4:   "{:.4}" => "0.0000");

            impl_case!(fmt_d1E: "{:.1E}" => "1.0E-10000");
            impl_case!(fmt_d4E: "{:.4E}" => "1.0000E-10000");
        }

        mod dec_1e100000 {
            use super::*;

            fn test_input() -> BigDecimal {
                "1e100000".parse().unwrap()
            }

            impl_case!(fmt_default: "{}" => "1e+100000");
            impl_case!(fmt_d1: "{:.1}" => "1e+100000");
            impl_case!(fmt_d4: "{:.4}" => "1e+100000");
        }


        mod dec_1234506789E5 {
            use super::*;

            fn test_input() -> BigDecimal {
                BigDecimal::new(1234506789.into(), -5)
            }

            impl_case!(fmt_default: "{}" => "123450678900000");
            impl_case!(fmt_d1: "{:.1}" => "123450678900000.0");
            impl_case!(fmt_d3: "{:.3}" => "123450678900000.000");
            impl_case!(fmt_d4: "{:.4}" => "123450678900000.0000");
            impl_case!(fmt_l13d4: "{:<23.4}" => "123450678900000.0000   ");
            impl_case!(fmt_r13d4: "{:>23.4}" => "   123450678900000.0000");
        }

        mod dec_1234506789E15 {
            use super::*;

            fn test_input() -> BigDecimal {
                BigDecimal::new(1234506789.into(), -15)
            }

            impl_case!(fmt_default: "{}" => "1234506789000000000000000");
            impl_case!(fmt_d1: "{:.1}" => "1234506789000000000000000.0");
            impl_case!(fmt_d3: "{:.3}" => "1234506789000000000000000.000");
            impl_case!(fmt_l13d4: "{:<+32.2}" => "+1234506789000000000000000.00   ");
            impl_case!(fmt_r13d4: "{:>+32.2}" => "   +1234506789000000000000000.00");
        }

        mod dec_13400476439814628800E2502 {
            use super::*;

            fn test_input() -> BigDecimal {
                BigDecimal::new(13400476439814628800u64.into(), -2502)
            }

            impl_case!(fmt_default: "{}" => "13400476439814628800e+2502");
            impl_case!(fmt_d1: "{:.1}" => "13400476439814628800e+2502");
        }

        mod dec_d9999 {
            use super::*;

            fn test_input() -> BigDecimal {
                "0.9999".parse().unwrap()
            }

            impl_case!(fmt_default:      "{}" => "0.9999");
            impl_case!(fmt_d0:        "{:.0}" => "1");
            impl_case!(fmt_d1:        "{:.1}" => "1.0");
            impl_case!(fmt_d2:        "{:.2}" => "1.00");
            impl_case!(fmt_d3:        "{:.3}" => "1.000");
            impl_case!(fmt_d4:        "{:.4}" => "0.9999");
            impl_case!(fmt_d6:        "{:.6}" => "0.999900");
        }

        mod dec_9d99 {
            use super::*;

            fn test_input() -> BigDecimal {
                "9.99".parse().unwrap()
            }

            impl_case!(fmt_default:      "{}" => "9.99");
            impl_case!(fmt_d0:        "{:.0}" => "10");
            impl_case!(fmt_d1:        "{:.1}" => "10.0");
            impl_case!(fmt_d2:        "{:.2}" => "9.99");
            impl_case!(fmt_d3:        "{:.3}" => "9.990");
            impl_case!(fmt_10d3:    "{:10.3}" => "     9.990");
        }

        mod dec_0 {
            use super::*;

            fn test_input() -> BigDecimal {
                "0".parse().unwrap()
            }

            impl_case!(fmt_default:        "{}" => "0");
            impl_case!(fmt_d0:          "{:.0}" => "0");
            impl_case!(fmt_d1:          "{:.1}" => "0.0");

            impl_case!(fmt_e:      "{:e}" => "0e+0");
            impl_case!(fmt_E:      "{:E}" => "0E+0");
        }

        mod dec_0e15 {
            use super::*;

            fn test_input() -> BigDecimal {
                "0e15".parse().unwrap()
            }

            impl_case!(fmt_default:        "{}" => "0");
            impl_case!(fmt_d0:          "{:.0}" => "0");
            impl_case!(fmt_d1:          "{:.1}" => "0.0");
            impl_case!(fmt_5d1:        "{:5.1}" => "  0.0");
            impl_case!(fmt_010d1:    "{:010.1}" => "00000000.0");

            impl_case!(fmt_e:      "{:e}" => "0e+0");
            impl_case!(fmt_E:      "{:E}" => "0E+0");

            impl_case!(fmt_d2e:        "{:.2e}" => "0.00e+0");
            impl_case!(fmt_8d2e:      "{:8.2e}" => " 0.00e+0");
        }

        mod dec_0en15 {
            use super::*;

            fn test_input() -> BigDecimal {
                "0e-15".parse().unwrap()
            }

            impl_case!(fmt_default:        "{}" => "0");
            impl_case!(fmt_d0:          "{:.0}" => "0");
            impl_case!(fmt_d1:          "{:.1}" => "0.0");
            impl_case!(fmt_6d2:        "{:6.2}" => "  0.00");
            impl_case!(fmt_010d1:    "{:010.1}" => "00000000.0");

            impl_case!(fmt_e:      "{:e}" => "0e+0");
            impl_case!(fmt_E:      "{:E}" => "0E+0");
        }

        mod dec_n0e6 {
            use super::*;

            fn test_input() -> BigDecimal {
                "-0e6".parse().unwrap()
            }

            impl_case!(fmt_default:        "{}" => "0");
            impl_case!(fmt_d1:          "{:.1}" => "0.0");

            impl_case!(fmt_e:      "{:e}" => "0e+0");
            impl_case!(fmt_E:      "{:E}" => "0E+0");
        }
    }

    mod fmt_boundaries {
        use super::*;

        macro_rules! impl_case {
            ( $name:ident: $src:expr => $expected:literal ) => {
                #[test]
                fn $name() {
                    let src = $src;
                    let bd: BigDecimal = src.parse().unwrap();
                    let result = bd.to_string();
                    assert_eq!(result, $expected);

                    let round_trip = BigDecimal::from_str(&result).unwrap();
                    assert_eq!(round_trip, bd);

                    let sci = bd.to_scientific_notation();
                    let sci_round_trip = BigDecimal::from_str(&sci).unwrap();
                    assert_eq!(sci_round_trip, bd);

                    let eng = bd.to_engineering_notation();
                    let eng_round_trip = BigDecimal::from_str(&eng).unwrap();
                    assert_eq!(eng_round_trip, bd);
                }
            };
            ( (eng-check-invalid) $name:ident: $src:expr => $expected:literal ) => {
                #[test]
                fn $name() {
                    let src = $src;
                    let bd: BigDecimal = src.parse().unwrap();
                    let result = bd.to_string();
                    assert_eq!(result, $expected);

                    let round_trip = BigDecimal::from_str(&result).unwrap();
                    assert_eq!(round_trip, bd);

                    let sci = bd.to_scientific_notation();
                    let sci_round_trip = BigDecimal::from_str(&sci).unwrap();
                    assert_eq!(sci_round_trip, bd);

                    let eng = bd.to_engineering_notation();
                    let eng_round_trip = BigDecimal::from_str(&eng);
                    assert!(eng_round_trip.is_err());
                }
            };
            ( (panics) $name:ident: $src:expr ) => {
                #[test]
                #[should_panic]
                fn $name() {
                    let src = $src;
                    let _bd: BigDecimal = src.parse().unwrap();
                }
            };
        }


        impl_case!(test_max: format!("1E{}", i64::MAX) => "1e+9223372036854775807");
        impl_case!(test_max_multiple_digits: format!("314156E{}", i64::MAX) => "314156e+9223372036854775807");
        impl_case!(test_min_scale: "1E9223372036854775807" => "1e+9223372036854775807");

        impl_case!((eng-check-invalid) test_max_scale: "1E-9223372036854775807" => "1E-9223372036854775807");
        impl_case!(test_min_multiple_digits: format!("271828182E-{}", i64::MAX) => "2.71828182E-9223372036854775799");

        impl_case!((panics) test_max_exp_overflow: "1E9223372036854775809");
        impl_case!((panics) test_min_exp_overflow: "1E-9223372036854775808");
    }

    #[test]
    fn test_fmt() {
        let vals = vec![
            // b  s (      {}     {:.1}        {:.4}    {:4.1}   {:+05.1}   {:<4.1}
            (1, 0,  (     "1",    "1.0",    "1.0000",   " 1.0",   "+01.0",   "1.0 " )),
            (1, 1,  (   "0.1",    "0.1",    "0.1000",   " 0.1",   "+00.1",   "0.1 " )),
            (1, 2,  (  "0.01",    "0.0",    "0.0100",   " 0.0",   "+00.0",   "0.0 " )),
            (1, -2, (   "100",  "100.0",  "100.0000",  "100.0",  "+100.0",  "100.0" )),
            (-1, 0, (    "-1",   "-1.0",   "-1.0000",   "-1.0",   "-01.0",   "-1.0" )),
            (-1, 1, (  "-0.1",   "-0.1",   "-0.1000",   "-0.1",   "-00.1",   "-0.1" )),
            (-1, 2, ( "-0.01",   "-0.0",   "-0.0100",   "-0.0",   "-00.0",   "-0.0" )),
        ];
        for (i, scale, results) in vals {
            let x = BigDecimal::new(num_bigint::BigInt::from(i), scale);
            assert_eq!(format!("{}", x), results.0);
            assert_eq!(format!("{:.1}", x), results.1);
            assert_eq!(format!("{:.4}", x), results.2);
            assert_eq!(format!("{:4.1}", x), results.3);
            assert_eq!(format!("{:+05.1}", x), results.4);
            assert_eq!(format!("{:<4.1}", x), results.5);
        }
    }


    mod fmt_debug {
        use super::*;

        macro_rules! impl_case {
            ($name:ident: $input:literal => $expected:literal => $expected_alt:literal) => {
                paste! {
                    #[test]
                    fn $name() {
                        let x: BigDecimal = $input.parse().unwrap();
                        let y = format!("{:?}", x);
                        assert_eq!(y, $expected);
                    }

                    #[test]
                    fn [< $name _alt >]() {
                        let x: BigDecimal = $input.parse().unwrap();
                        let y = format!("{:#?}", x);
                        assert_eq!(y, $expected_alt);
                    }
                }
            }
        }

        impl_case!(case_0: "0" => r#"BigDecimal(sign=NoSign, scale=0, digits=[])"#
                               => r#"BigDecimal("0e0")"#);

        impl_case!(case_n0: "-0" => r#"BigDecimal(sign=NoSign, scale=0, digits=[])"#
                               => r#"BigDecimal("0e0")"#);

        impl_case!(case_1: "1" => r#"BigDecimal(sign=Plus, scale=0, digits=[1])"#
                               => r#"BigDecimal("1e0")"#);

        impl_case!(case_123_400: "123.400" => r#"BigDecimal(sign=Plus, scale=3, digits=[123400])"#
                                           => r#"BigDecimal("123400e-3")"#);

        impl_case!(case_123_4en2: "123.4e-2" => r#"BigDecimal(sign=Plus, scale=3, digits=[1234])"#
                                             => r#"BigDecimal("1234e-3")"#);

        impl_case!(case_123_456:   "123.456" => r#"BigDecimal(sign=Plus, scale=3, digits=[123456])"#
                                             => r#"BigDecimal("123456e-3")"#);

        impl_case!(case_01_20:       "01.20" => r#"BigDecimal(sign=Plus, scale=2, digits=[120])"#
                                             => r#"BigDecimal("120e-2")"#);

        impl_case!(case_1_20:         "1.20" => r#"BigDecimal(sign=Plus, scale=2, digits=[120])"#
                                             => r#"BigDecimal("120e-2")"#);
        impl_case!(case_01_2e3:     "01.2E3" => r#"BigDecimal(sign=Plus, scale=-2, digits=[12])"#
                                             => r#"BigDecimal("12e2")"#);

        impl_case!(case_avagadro: "6.02214076e1023" => r#"BigDecimal(sign=Plus, scale=-1015, digits=[602214076])"#
                                                    => r#"BigDecimal("602214076e1015")"#);

        impl_case!(case_1e99999999999999 : "1e99999999999999" => r#"BigDecimal(sign=Plus, scale=-99999999999999, digits=[1])"#
                                                              => r#"BigDecimal("1e99999999999999")"#);

        impl_case!(case_n144d3308279 : "-144.3308279" => r#"BigDecimal(sign=Minus, scale=7, digits=[1443308279])"#
                                                      => r#"BigDecimal("-1443308279e-7")"#);

        impl_case!(case_n349983058835858339619e2 : "-349983058835858339619e2"
                                                      => r#"BigDecimal(sign=Minus, scale=-2, digits=[17941665509086410531, 18])"#
                                                      => r#"BigDecimal("-349983058835858339619e2")"#);
    }

    mod write_scientific_notation {
        use super::*;

        macro_rules! test_fmt_function {
            ($n:expr) => { $n.to_scientific_notation() };
        }

        impl_case!(case_4_1592480782835e9 : "4159248078.2835" => "4.1592480782835e9");
        impl_case!(case_1_234e_5 : "0.00001234" => "1.234e-5");
        impl_case!(case_0 : "0" => "0e0");
        impl_case!(case_1 : "1" => "1e0");
        impl_case!(case_2_00e0 : "2.00" => "2.00e0");
        impl_case!(case_neg_5_70e1 : "-57.0" => "-5.70e1");
    }

    mod write_engineering_notation {
        use super::*;

        macro_rules! test_fmt_function {
            ($n:expr) => { $n.to_engineering_notation() };
        }

        impl_case!(case_4_1592480782835e9 : "4159248078.2835" => "4.1592480782835e9");
        impl_case!(case_12_34e_6 : "0.00001234" => "12.34e-6");
        impl_case!(case_0 : "0" => "0e0");
        impl_case!(case_1 : "1" => "1e0");
        impl_case!(case_2_00e0 : "2.00" => "2.00e0");
        impl_case!(case_neg_5_70e1 : "-57.0" => "-57.0e0");
        impl_case!(case_5_31e4 : "5.31e4" => "53.1e3");
        impl_case!(case_5_31e5 : "5.31e5" => "531e3");
        impl_case!(case_5_31e6 : "5.31e6" => "5.31e6");
        impl_case!(case_5_31e7 : "5.31e7" => "53.1e6");

        impl_case!(case_1e2 : "1e2" => "100e0");
        impl_case!(case_1e119 : "1e19" => "10e18");
        impl_case!(case_1e3000 : "1e3000" => "1e3000");
        impl_case!(case_4_2e7 : "4.2e7" => "42e6");
        impl_case!(case_4_2e8 : "4.2e8" => "420e6");

        impl_case!(case_4e99999999999999 : "4e99999999999999" => "4e99999999999999");
        impl_case!(case_4e99999999999998 : "4e99999999999998" => "400e99999999999996");
        impl_case!(case_44e99999999999998 : "44e99999999999998" => "4.4e99999999999999");
        impl_case!(case_4e99999999999997 : "4e99999999999997" => "40e99999999999996");
        impl_case!(case_41e99999999999997 : "41e99999999999997" => "410e99999999999996");
        impl_case!(case_413e99999999999997 : "413e99999999999997" => "4.13e99999999999999");
        // impl_case!(case_413e99999999999997 : "413e99999999999997" => "4.13e99999999999999");
    }
}


#[cfg(all(test, property_tests))]
mod proptests {
    use super::*;
    use paste::paste;
    use proptest::prelude::*;
    use proptest::num::f64::NORMAL as NormalF64;


    macro_rules! impl_parsing_test {
        ($t:ty) => {
            paste! { proptest! {
                #[test]
                fn [< roudtrip_to_str_and_back_ $t >](n: $t) {
                    let original = BigDecimal::from(n);
                    let display = format!("{}", original);
                    let parsed = display.parse::<BigDecimal>().unwrap();

                    prop_assert_eq!(&original, &parsed);
                }
            } }
        };
        (from-float $t:ty) => {
            paste! { proptest! {
                #[test]
                fn [< roudtrip_to_str_and_back_ $t >](n: $t) {
                    let original = BigDecimal::try_from(n).unwrap();
                    let display = format!("{}", original);
                    let parsed = display.parse::<BigDecimal>().unwrap();

                    prop_assert_eq!(&original, &parsed);
                }
            } }
        };
    }

    impl_parsing_test!(u8);
    impl_parsing_test!(u16);
    impl_parsing_test!(u32);
    impl_parsing_test!(u64);
    impl_parsing_test!(u128);

    impl_parsing_test!(i8);
    impl_parsing_test!(i16);
    impl_parsing_test!(i32);
    impl_parsing_test!(i64);
    impl_parsing_test!(i128);

    impl_parsing_test!(from-float f32);
    impl_parsing_test!(from-float f64);

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(32_000))]

        #[test]
        fn float_formatting(f in NormalF64, prec in 0..21usize) {
            let d = BigDecimal::from_f64(f).unwrap();
            let f_fmt = format!("{f:.prec$}");
            let d_fmt = format!("{d:.prec$}").replace("+", "");
            prop_assert_eq!(f_fmt, d_fmt);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn scientific_notation_roundtrip(f: f64) {
            prop_assume!(!f.is_nan() && !f.is_infinite());
            let n = BigDecimal::from_f64(f).unwrap();
            let s = n.to_scientific_notation();
            let m: BigDecimal = s.parse().unwrap();
            prop_assert_eq!(n, m);
        }

        #[test]
        fn engineering_notation_roundtrip(f: f64) {
            prop_assume!(!f.is_nan() && !f.is_infinite());
            let n = BigDecimal::from_f64(f).unwrap();
            let s = n.to_engineering_notation();
            let m: BigDecimal = s.parse().unwrap();
            prop_assert_eq!(n, m);
        }
    }
}
