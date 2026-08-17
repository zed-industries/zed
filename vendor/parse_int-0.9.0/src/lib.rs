#![deny(warnings)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "ranges")]
pub mod range;
mod utils;

use std::fmt::{Binary, Display, LowerHex, Octal};

use num_traits::Num;
use utils::{is_floaty, process_fractional, process_integer};

/// True if the crate had the feature "implicit-octal" enabled at build time
pub const IMPLICIT_OCTAL_ENABLED: bool = {
    #[cfg(feature = "implicit-octal")]
    {
        true
    }
    #[cfg(not(feature = "implicit-octal"))]
    {
        false
    }
};

/// Parse [&str] with common prefixes to integer values
///
/// ```rust
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use parse_int::parse;
///
/// // decimal
/// let d = parse::<usize>("42")?;
/// assert_eq!(42, d);
///
/// // hex
/// let d = parse::<isize>("0x42")?;
/// assert_eq!(66, d);
///
/// // you can use underscores for more readable inputs
/// let d = parse::<isize>("0x42_424_242")?;
/// assert_eq!(1_111_638_594, d);
///
/// // octal explicit
/// let d = parse::<u8>("0o42")?;
/// assert_eq!(34, d);
///
/// ##[cfg(feature = "implicit-octal")]
/// {
///     let d = parse::<i8>("042")?;
///     assert_eq!(34, d);
/// }
///
/// // binary
/// let d = parse::<u16>("0b0110")?;
/// assert_eq!(6, d);
/// #
/// #     Ok(())
/// # }
/// ```
#[inline]
pub fn parse<T: Num>(input: &str) -> Result<T, T::FromStrRadixErr> {
    let input = input.trim();

    let (is_negative, input) = if let Some(input) = input.strip_prefix('-') {
        (true, input)
    } else {
        (false, input)
    };

    // invalid start
    if input.starts_with("_") {
        /* With rust 1.55 the return type is stable but we can not construct it yet

        let kind = ::core::num::IntErrorKind::InvalidDigit;
        //let pie = ::core::num::ParseIntError {
        let pie = <<T as num_traits::Num>::FromStrRadixErr as Trait>::A {
            kind
        };
        return Err(pie);
        */
        return T::from_str_radix("_", 2);
    }

    let num: T =
    // hex
    if input.starts_with("0x") || input.starts_with("0X") {
        parse_with_base(&input[2..], 16)?
    } else

    // binary
    if input.starts_with("0b") || input.starts_with("0B") {
        parse_with_base(&input[2..], 2)?
    } else

    // octal
    if input.starts_with("0o") || input.starts_with("0O") {
        parse_with_base(&input[2..], 8)?
    } else if IMPLICIT_OCTAL_ENABLED &&  input.starts_with("0") {
        if input == "0" {
            T::zero()
        } else {
            parse_with_base(&input[1..], 8)?
        }
    } else {
        // decimal
        parse_with_base(input, 10)?
    };

    Ok(if is_negative {
        num * (T::zero() - T::one())
    } else {
        num
    })
}

#[inline]
fn parse_with_base<T: Num>(input: &str, base: u32) -> Result<T, T::FromStrRadixErr> {
    let input = input.chars().filter(|&c| c != '_').collect::<String>();
    T::from_str_radix(&input, base)
}

/// Pretty print integer and float numbers with base 10, separated by underscores grouping the digits in threes
///
/// ```rust
/// use parse_int::format_pretty_dec;
/// assert_eq!("12_345.678_9", format_pretty_dec(12345.6789));
/// assert_eq!("12_345", format_pretty_dec(12345));
/// ```
pub fn format_pretty_dec<N: Num + Display + 'static>(num: N) -> String {
    let is_floaty = is_floaty(&num);
    let short = format!("{num}");

    let (integer, fraction) = if is_floaty {
        match short.split_once('.') {
            Some((i, f)) => (i, Some(f)),
            None => (short.as_ref(), Some("0")),
        }
    } else {
        (short.as_ref(), None)
    };

    let processed_integer = process_integer(integer, 3);

    if let Some(fraction) = fraction {
        let processed_fractional = process_fractional(fraction);
        format!("{processed_integer}.{processed_fractional}")
    } else {
        processed_integer
    }
}

/// Outputs a human readable string like:
///
/// ```
/// let pretty = parse_int::format_pretty_octal(1024);
/// assert_eq!("0o2_000", pretty);
/// ```
///
/// Floats do not implement Octal
pub fn format_pretty_octal<N: Num + Display + Octal + PartialOrd + 'static>(num: N) -> String {
    let is_negative = num < N::zero();

    let num = if is_negative {
        // TODO optimise in the future
        num * (N::zero() - N::one())
    } else {
        num
    };

    let short = format!("{num:o}");

    let sign = if is_negative { "-" } else { "" };

    let processed = process_integer(&short, 3);

    format!("{sign}0o{processed}")
}

/// Outputs a human readable string like:
///
/// ```
/// let pretty = parse_int::format_pretty_hex(1024);
/// assert_eq!("0x4_00", pretty);
/// ```
///
/// Floats do not implement LowerHex
pub fn format_pretty_hex<N: Num + Display + LowerHex + PartialOrd + 'static>(num: N) -> String {
    let is_negative = num < N::zero();

    let num = if is_negative {
        // TODO optimise in the future
        num * (N::zero() - N::one())
    } else {
        num
    };

    let short = format!("{num:x}");

    let sign = if is_negative { "-" } else { "" };

    let processed = process_integer(&short, 2);

    format!("{sign}0x{processed}")
}

/// Outputs a human readable string like:
///
/// ```
/// let pretty = parse_int::format_pretty_bin(1024);
/// assert_eq!("0b100_0000_0000", pretty);
/// ```
///
/// Floats do not implement LowerHex
pub fn format_pretty_bin<N: Num + Display + Binary + PartialOrd + 'static>(num: N) -> String {
    let is_negative = num < N::zero();

    let num = if is_negative {
        // TODO optimise in the future
        num * (N::zero() - N::one())
    } else {
        num
    };

    let short = format!("{num:b}");

    let sign = if is_negative { "-" } else { "" };

    let processed = process_integer(&short, 4);

    format!("{sign}0b{processed}")
}

/*
/// The available bases for pretty printing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Base {
    #[default]
    Decimal = 10,
    Hex = 16,
    Octal = 8,
}

impl Base {
    pub fn format<N: Num + PartialOrd + Display + LowerHex + Octal + 'static>(
        &self,
        num: N,
    ) -> String {
        format_pretty_with(*self, num)
    }
}
*/

#[cfg(test)]
mod test_parsing {
    use super::*;

    #[test]
    fn turbofish_usize_dec() {
        let s = "42";

        let u = parse::<usize>(s).unwrap();

        assert_eq!(42, u);
    }

    #[test]
    fn deduct_usize_dec() {
        let s = "42";

        let u = parse(s).unwrap();

        assert_eq!(42usize, u);
    }

    #[test]
    fn deduct_isize_dec_neg() {
        let s = "-42";

        let u = parse(s).unwrap();

        assert_eq!(-42_isize, u);
    }

    macro_rules! int_parse {
        ($type:ident, $s:literal, $e:literal) => {
            #[test]
            fn $type() {
                let l: Result<$type, _> = crate::parse($s);
                assert_eq!(Ok($e), l, "lower case");

                let u: Result<$type, _> = crate::parse(&$s.to_uppercase());
                assert_eq!(Ok($e), u, "upper case");
            }
        };
    }

    macro_rules! int_parse_err {
        ($type:ident, $s:literal) => {
            int_parse_err!($type, $s, err);
        };
        ($type:ident, $s:literal, $opt:ident) => {
            mod $opt {
                #[test]
                fn $type() {
                    let u: Result<$type, _> = crate::parse($s);
                    assert!(u.is_err(), "expected Err(_), got = {:?}", u);
                }
            }
        };
    }

    mod decimal {
        int_parse!(usize, "42", 42);
        int_parse!(isize, "42", 42);
        int_parse!(u8, "42", 42);
        int_parse!(i8, "42", 42);
        int_parse!(u16, "42", 42);
        int_parse!(i16, "42", 42);
        int_parse!(u32, "42", 42);
        int_parse!(i32, "42", 42);
        int_parse!(u64, "42", 42);
        int_parse!(i64, "42", 42);
        int_parse!(u128, "42", 42);
        int_parse!(i128, "42", 42);
    }

    mod decimal_negative {
        int_parse!(isize, "-42", -42);
        int_parse!(i8, "-42", -42);
        int_parse!(i128, "-42_000", -42_000);
    }

    mod hexadecimal {
        int_parse!(usize, "0x42", 66);
        int_parse!(isize, "0x42", 66);
        int_parse!(u8, "0x42", 66);
        int_parse!(i8, "0x42", 66);
        int_parse!(u16, "0x42", 66);
        int_parse!(i16, "0x42", 66);
        int_parse!(u32, "0x42", 66);
        int_parse!(i32, "0x42", 66);
        int_parse!(u64, "0x42", 66);
        int_parse!(i64, "0x42", 66);
        int_parse!(u128, "0x42", 66);
        int_parse!(i128, "0x42", 66);
    }

    mod hex_negative {
        int_parse!(isize, "-0x42", -66);
        int_parse!(i8, "-0x42", -66);
        int_parse!(i16, "-0x42", -66);
        int_parse!(i32, "-0x42", -66);
        int_parse!(i64, "-0x42", -66);
        int_parse!(i128, "-0x42", -66);
    }

    mod octal_explicit {
        int_parse!(usize, "0o42", 34);
        int_parse!(isize, "0o42", 34);
        int_parse!(u8, "0o42", 34);
        int_parse!(i8, "0o42", 34);
        int_parse!(u16, "0o42", 34);
        int_parse!(i16, "0o42", 34);
        int_parse!(u32, "0o42", 34);
        int_parse!(i32, "0o42", 34);
        int_parse!(u64, "0o42", 34);
        int_parse!(i64, "0o42", 34);
        int_parse!(u128, "0o42", 34);
        int_parse!(i128, "0o42", 34);
    }

    mod octal_explicit_negative {
        int_parse!(isize, "-0o42", -34);
        int_parse!(i8, "-0o42", -34);
        int_parse!(i16, "-0o42", -34);
        int_parse!(i32, "-0o42", -34);
        int_parse!(i64, "-0o42", -34);
        int_parse!(i128, "-0o42", -34);
    }

    #[cfg(feature = "implicit-octal")]
    mod octal_implicit {
        use super::*;
        int_parse!(usize, "042", 34);
        int_parse!(isize, "042", 34);
        int_parse!(u8, "042", 34);
        int_parse!(i8, "042", 34);
        int_parse!(u16, "042", 34);
        int_parse!(i16, "042", 34);
        int_parse!(u32, "042", 34);
        int_parse!(i32, "042", 34);
        int_parse!(u64, "042", 34);
        int_parse!(i64, "042", 34);
        int_parse!(u128, "042", 34);
        int_parse!(i128, "042", 34);

        #[test]
        fn issue_nr_0() {
            let s = "0";

            assert_eq!(0, parse::<usize>(s).unwrap());
            assert_eq!(0, parse::<isize>(s).unwrap());
            assert_eq!(0, parse::<i8>(s).unwrap());
            assert_eq!(0, parse::<u8>(s).unwrap());
            assert_eq!(0, parse::<i16>(s).unwrap());
            assert_eq!(0, parse::<u16>(s).unwrap());
            assert_eq!(0, parse::<i32>(s).unwrap());
            assert_eq!(0, parse::<u32>(s).unwrap());
            assert_eq!(0, parse::<i64>(s).unwrap());
            assert_eq!(0, parse::<u64>(s).unwrap());
            assert_eq!(0, parse::<i128>(s).unwrap());
            assert_eq!(0, parse::<u128>(s).unwrap());
        }
    }
    #[cfg(feature = "implicit-octal")]
    mod octal_implicit_negative {
        int_parse!(isize, "-042", -34);
        int_parse!(i8, "-042", -34);
        int_parse!(i16, "-042", -34);
        int_parse!(i32, "-042", -34);
        int_parse!(i64, "-042", -34);
        int_parse!(i128, "-042", -34);
    }
    #[cfg(not(feature = "implicit-octal"))]
    mod octal_implicit_disabled {
        use super::*;
        #[test]
        /// maybe this will change in the future
        fn no_implicit_is_int() {
            let s = "042";

            let u = parse::<usize>(s);
            assert_eq!(Ok(42), u, "{:?}", u);
        }
        #[test]
        fn no_implicit_is_int_neg() {
            let s = "-042";

            let u = parse::<isize>(s);
            assert_eq!(Ok(-42), u, "{:?}", u);
        }
    }

    mod binary {
        int_parse!(usize, "0b0110", 6);
        int_parse!(isize, "0b0110", 6);
        int_parse!(u8, "0b0110", 6);
        int_parse!(i8, "0b0110", 6);
        int_parse!(u16, "0b0110", 6);
        int_parse!(i16, "0b0110", 6);
        int_parse!(u32, "0b0110", 6);
        int_parse!(i32, "0b0110", 6);
        int_parse!(u64, "0b0110", 6);
        int_parse!(i64, "0b0110", 6);
        int_parse!(u128, "0b0110", 6);
        int_parse!(i128, "0b0110", 6);
    }

    mod binary_negative {
        int_parse_err!(i8, "0b1000_0000");
        int_parse!(i8, "0b-0111_1111", -127);
    }

    mod underscore {
        int_parse!(usize, "0b0110_0110", 102);
        int_parse!(isize, "0x0110_0110", 17_826_064);
        int_parse!(u64, "0o0110_0110", 294_984);
        int_parse!(u128, "1_100_110", 1_100_110);

        #[cfg(feature = "implicit-octal")]
        mod implicit_octal {
            int_parse!(i128, "0110_0110", 294_984);
        }
        #[cfg(not(feature = "implicit-octal"))]
        mod implicit_octal {
            int_parse!(i128, "0110_0110", 1_100_110);
        }
    }

    mod underscore_in_prefix {
        #[test]
        fn invalid_underscore_in_prefix() {
            let r = crate::parse::<isize>("_4");
            println!("{:?}", r);
            assert!(r.is_err());
        }
        int_parse_err!(isize, "0_x_4", hex);
        int_parse_err!(isize, "_4", decimal);
        int_parse_err!(isize, "0_o_4", octal);
        int_parse_err!(isize, "0_b_1", binary);
    }
}

#[cfg(test)]
mod test_parsing_floats {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn dec() {
        let f = parse("12.34");
        assert_eq!(12.34, f.unwrap());
    }

    #[test]
    fn dec_separators() {
        assert_eq!(12.34, parse("1_2.3_4").unwrap());
        assert_eq!(12.34_f32, parse("1_2.3_4").unwrap());
        assert_eq!(12.34_f64, parse("1_2.3_4").unwrap());

        assert_eq!(PI, parse("3.141_592_653_589_793").unwrap());
    }
}

#[cfg(test)]
mod pretty_print_test {
    use super::*;

    #[test]
    fn intuitive_syntax() {
        assert_eq!("42_230_123", format_pretty_dec(42_230_123));

        assert_eq!("42.0", format_pretty_dec(42.0_f32));
    }

    #[test]
    fn positive_ints_based() {
        assert_eq!("42", format_pretty_dec(42));
        assert_eq!("42_000", format_pretty_dec(42_000));

        assert_eq!("-42_000", format_pretty_dec(-42_000));
    }

    #[test]
    fn positive_ints_oct() {
        let ft = 0o42;
        assert_eq!("0o42", format_pretty_octal(ft));
        assert_eq!("0o42_000", format_pretty_octal(0o42_000));

        assert_eq!("0o42_000", format_pretty_octal(0o42_000_u64));
        assert_eq!("0o42_000", format_pretty_octal(0o42_000_i64));
    }

    #[test]
    fn positive_ints_oct_neg() {
        let ft = -0o42;
        assert_eq!("-0o42", format_pretty_octal(ft));
        assert_eq!("-0o42_000", format_pretty_octal(-0o42_000));
        assert_eq!("-0o4_200", format_pretty_octal(-0o4_200));

        assert_eq!("-0o42_000", format_pretty_octal(-0o42_000_i64));
    }

    #[test]
    fn positive_ints_hex() {
        assert_eq!("0x42", format_pretty_hex(0x42));

        assert_eq!("0xf_ff", format_pretty_hex(0xf_ff));
        assert_eq!("0x42_00", format_pretty_hex(0x4_200));

        assert_eq!("0xff_ff", format_pretty_hex(0xff_FF));
        assert_eq!("0x4_20_00", format_pretty_hex(0x42_000));

        assert_eq!("0x4_20_00", format_pretty_hex(0x42_000_u64));
        assert_eq!("0x4_20_00", format_pretty_hex(0x42_000_i64));
    }

    #[test]
    fn negative_ints_hex() {
        assert_eq!("-0x42", format_pretty_hex(-0x42_i8));
        assert_eq!("-0x42", format_pretty_hex(-0x42_i16));

        assert_eq!("-0x42_00", format_pretty_hex(-0x42_00_i32));
        assert_eq!("-0x4_20_00", format_pretty_hex(-0x4_20_00_i64));
    }

    #[test]
    fn dec_ints() {
        assert_eq!("0", format_pretty_dec(0));
        assert_eq!("42", format_pretty_dec(42));
        assert_eq!("42_000", format_pretty_dec(42_000));

        assert_eq!("-4_200", format_pretty_dec(-4200));
        assert_eq!("-42_000", format_pretty_dec(-42_000));
    }

    #[test]
    fn dec_floats() {
        assert_eq!("0.0", format_pretty_dec(0.0_f32));
        assert_eq!("0.0", format_pretty_dec(0.0_f64));

        assert_eq!("42.0", format_pretty_dec(42.0_f32));
        assert_eq!("42_000.0", format_pretty_dec(42_000.0_f64));

        assert_eq!("-42_000.0", format_pretty_dec(-42_000.0));
        assert_eq!("-42_000.001_002", format_pretty_dec(-42_000.001_002));
    }

    #[test]
    fn bin_ints() {
        assert_eq!("0b0", format_pretty_bin(0));
        assert_eq!("0b10_1010", format_pretty_bin(42));
        assert_eq!("0b1010_0100_0001_0000", format_pretty_bin(42_000));

        assert_eq!("-0b1_0000_0110_1000", format_pretty_bin(-4200));
        assert_eq!("-0b1010_0100_0001_0000", format_pretty_bin(-42_000));
    }
}
