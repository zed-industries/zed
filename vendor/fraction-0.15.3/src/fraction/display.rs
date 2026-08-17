//! Implementation of fmt::Display for [GenericFraction] and [Sign] structures
//!
//! Complete support of the [std::fmt] formatting syntax with exceptions for alternate
//! integer types (hexadecimal, octal, binary)
//!
//! There are a couple of extensions on top of std::fmt, which are
//!   - Alternate flag (`#`) is used to enable `trailing zeroes` output
//!   - Negative sign (`-`) is used to suppress sign output (even negative)
//!
//! Otherwise, the standard formatting is followed as is.
//!
//! # Examples
//!
//! ```
//! use fraction::prelude::Fraction;
//!
//! let fraction = Fraction::from(1.75);
//!
//! assert_eq!("7/4", format!("{}", fraction));
//! assert_eq!("1.75", format!("{:.4}", fraction));
//! assert_eq!("1.7500", format!("{:#.4}", fraction));
//! assert_eq!("-1.75", format!("{:.2}", -fraction));
//! assert_eq!("1.75", format!("{:-.2}", -fraction));
//! ```

use division;
use fraction::{GenericFraction, Sign};
use generic::GenericInteger;

use std::fmt;

/// Same as fmt::Formatter, but allowing to change flags
/// so we can reuse it in recursive format implementations
pub struct Format {
    flags: u8,
    fill: char,
    width: Option<usize>,
    precision: Option<usize>,
    align: Option<fmt::Alignment>,
}

const SIGN_PLUS: u8 = 1;
const SIGN_MINUS: u8 = 2;
const ALTERNATE: u8 = 4;
const SIGAZEPAD: u8 = 8;

impl Format {
    pub const fn empty() -> Self {
        Format {
            fill: ' ',
            align: None,
            width: None,
            precision: None,
            flags: 0,
        }
    }

    pub fn new(formatter: &fmt::Formatter) -> Self {
        let sign_plus = if formatter.sign_plus() { SIGN_PLUS } else { 0 };
        let sign_minus = if formatter.sign_minus() {
            SIGN_MINUS
        } else {
            0
        };
        let alternate = if formatter.alternate() { ALTERNATE } else { 0 };
        let sigazepad = if formatter.sign_aware_zero_pad() {
            SIGAZEPAD
        } else {
            0
        };

        let flags = sign_plus | sign_minus | alternate | sigazepad;

        Format {
            fill: formatter.fill(),
            align: formatter.align(),
            width: formatter.width(),
            precision: formatter.precision(),
            flags,
        }
    }

    pub const fn fill(&self) -> char {
        self.fill
    }

    pub const fn cloned_align(&self) -> Option<fmt::Alignment> {
        match self.align {
            Some(ref align) => match *align {
                fmt::Alignment::Left => Some(fmt::Alignment::Left),
                fmt::Alignment::Right => Some(fmt::Alignment::Right),
                fmt::Alignment::Center => Some(fmt::Alignment::Center),
            },
            None => None,
        }
    }

    pub const fn align(&self) -> &Option<fmt::Alignment> {
        &self.align
    }

    pub const fn set_align(mut self, align: Option<fmt::Alignment>) -> Self {
        self.align = align;
        self
    }

    pub const fn width(&self) -> &Option<usize> {
        &self.width
    }

    pub const fn set_width(mut self, width: Option<usize>) -> Self {
        self.width = width;
        self
    }

    pub const fn precision(&self) -> &Option<usize> {
        &self.precision
    }

    pub const fn set_precision(mut self, precision: Option<usize>) -> Self {
        self.precision = precision;
        self
    }

    pub const fn sign_plus(&self) -> bool {
        self.flags & SIGN_PLUS > 0
    }

    pub const fn set_sign_plus(mut self, flag: bool) -> Self {
        if flag {
            self.flags |= SIGN_PLUS;
        } else {
            self.flags &= !SIGN_PLUS;
        }

        self
    }

    pub const fn sign_minus(&self) -> bool {
        self.flags & SIGN_MINUS > 0
    }

    pub const fn set_sign_minus(mut self, flag: bool) -> Self {
        if flag {
            self.flags |= SIGN_MINUS;
        } else {
            self.flags &= !SIGN_MINUS;
        }

        self
    }

    pub const fn alternate(&self) -> bool {
        self.flags & ALTERNATE > 0
    }

    pub const fn set_alternate(mut self, flag: bool) -> Self {
        if flag {
            self.flags |= ALTERNATE;
        } else {
            self.flags &= !ALTERNATE;
        }

        self
    }

    pub const fn sign_aware_zero_pad(&self) -> bool {
        self.flags & SIGAZEPAD > 0
    }

    pub const fn set_sign_aware_zero_pad(mut self, flag: bool) -> Self {
        if flag {
            self.flags |= SIGAZEPAD;
        } else {
            self.flags &= !SIGAZEPAD;
        }

        self
    }
}

impl Clone for Format {
    fn clone(&self) -> Self {
        Format {
            fill: self.fill(),
            align: self.cloned_align(),
            width: *self.width(),
            precision: *self.precision(),
            flags: self.flags,
        }
    }
}

pub fn format_sign(sign: Sign, buffer: &mut dyn fmt::Write, format: &Format) -> fmt::Result {
    if format.sign_plus() || (!format.sign_minus() && sign.is_negative()) {
        if format.sign_aware_zero_pad() {
            let format = format.clone().set_sign_aware_zero_pad(false);
            format_value(
                buffer,
                &format,
                None,
                |_| 1,
                |b, _| {
                    b.write_char(sign.into())?;
                    Ok(1)
                },
            )
        } else {
            format_value(
                buffer,
                format,
                None,
                |_| 1,
                |b, _| {
                    b.write_char(sign.into())?;
                    Ok(1)
                },
            )
        }
    } else {
        Ok(())
    }
}

pub fn format_fraction<T>(
    fraction: &GenericFraction<T>,
    buffer: &mut dyn fmt::Write,
    format: &Format,
) -> fmt::Result
where
    T: Clone + GenericInteger,
{
    match *fraction {
        GenericFraction::NaN => format_value(
            buffer,
            format,
            None,
            |_| 3,
            |b, _| {
                b.write_str("NaN")?;
                Ok(3)
            },
        ),
        GenericFraction::Infinity(sign) => format_value(
            buffer,
            format,
            Some(sign),
            |_| 3,
            |b, _| {
                b.write_str("inf")?;
                Ok(3)
            },
        ),
        GenericFraction::Rational(sign, ref ratio) => format_value(
            buffer,
            format,
            Some(sign),
            |format| {
                let numer = ratio.numer().clone();
                let denom = ratio.denom().clone();

                let mut length = 0;

                if let Some(precision) = format.precision() {
                    division::divide_to_callback(
                        numer,
                        denom,
                        *precision,
                        format.alternate(),
                        |_| {
                            length += 1;
                            Ok(true)
                        },
                    )
                    .ok();
                } else {
                    division::divide_to_callback(numer, T::one(), 0, false, |_| {
                        length += 1;
                        Ok(true)
                    })
                    .ok();

                    if !ratio.numer().is_zero() && !ratio.denom().is_one() {
                        length += 1;
                        division::divide_to_callback(denom, T::one(), 0, false, |_| {
                            length += 1;
                            Ok(true)
                        })
                        .ok();
                    }
                }

                length
            },
            |buffer, format| {
                let numer = ratio.numer().clone();
                let denom = ratio.denom().clone();

                let mut length = 0;

                if let Some(precision) = format.precision() {
                    let callback = |digit| {
                        length += 1;
                        division::write_digit(buffer, digit)
                    };

                    if division::divide_to_callback(
                        numer,
                        denom,
                        *precision,
                        format.alternate(),
                        callback,
                    )
                    .is_err()
                    {
                        return Err(fmt::Error);
                    }
                } else {
                    let callback = |digit| {
                        length += 1;
                        division::write_digit(buffer, digit)
                    };

                    if division::divide_to_callback(numer, T::one(), 0, false, callback).is_err() {
                        return Err(fmt::Error);
                    }

                    if !ratio.numer().is_zero() && !ratio.denom().is_one() {
                        length += 1;
                        buffer.write_char('/')?;

                        let callback = |digit| {
                            length += 1;
                            division::write_digit(buffer, digit)
                        };

                        if division::divide_to_callback(denom, T::one(), 0, false, callback)
                            .is_err()
                        {
                            return Err(fmt::Error);
                        };
                    }
                }

                Ok(length)
            },
        ),
    }
}

fn format_value<L, V>(
    buffer: &mut dyn fmt::Write,
    format: &Format,
    sign: Option<Sign>,
    value_length: L,
    value: V,
) -> fmt::Result
where
    L: Fn(&Format) -> usize,
    V: FnOnce(&mut dyn fmt::Write, &Format) -> Result<usize, fmt::Error>,
{
    if let Some(width) = format.width() {
        let width = *width;
        let sign_len = if let Some(sign) = sign {
            usize::from(format.sign_plus() || (!format.sign_minus() && sign.is_negative()))
        } else {
            0
        };

        if format.sign_aware_zero_pad() {
            let value_len = value_length(format);

            if sign_len > 0 {
                buffer.write_char(sign.unwrap().into())?;
            }

            if width > sign_len + value_len {
                for _ in sign_len + value_len..width {
                    buffer.write_char('0')?;
                }
            }
            value(buffer, format)?;
        } else {
            match format.align() {
                Some(fmt::Alignment::Center) => {
                    let value_len = value_length(format) + sign_len;
                    let mut prefix_len = 0;
                    let mut excess = 0;

                    if width > value_len {
                        excess = width - value_len;
                        prefix_len = excess / 2;
                        for _ in 0..prefix_len {
                            buffer.write_char(format.fill())?;
                        }
                    };

                    if sign_len > 0 {
                        buffer.write_char(sign.unwrap().into())?;
                    }
                    value(buffer, format)?;

                    if width > value_len {
                        for _ in 0..excess - prefix_len {
                            buffer.write_char(format.fill())?;
                        }
                    }
                }
                None | Some(fmt::Alignment::Right) => {
                    let value_len = value_length(format);

                    for _ in sign_len + value_len..width {
                        buffer.write_char(format.fill())?;
                    }

                    if sign_len > 0 {
                        buffer.write_char(sign.unwrap().into())?;
                    }
                    value(buffer, format)?;
                }
                Some(fmt::Alignment::Left) => {
                    if sign_len > 0 {
                        buffer.write_char(sign.unwrap().into())?;
                    }
                    let value_len = value(buffer, format)?;

                    if width > sign_len + value_len {
                        for _ in sign_len + value_len..width {
                            buffer.write_char(format.fill())?;
                        }
                    }
                }
            }
        }
    } else {
        if let Some(sign) = sign {
            if format.sign_plus() || (!format.sign_minus() && sign.is_negative()) {
                buffer.write_char(sign.into())?;
            }
        }
        value(buffer, format)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "with-bigint")]
    use prelude::{BigFraction, BigUint};

    use super::super::super::{One, Zero};
    use prelude::GenericFraction;

    type F32 = GenericFraction<i32>;
    type F64 = GenericFraction<u64>;

    #[test]
    fn display() {
        assert_eq!("NaN", format!("{}", F32::nan()));
        assert_eq!(format!("{}", ::std::f64::NAN), format!("{}", F32::nan()));

        assert_eq!("NaN", format!("{:+}", F32::nan()));
        assert_eq!(
            format!("{:+}", ::std::f64::NAN),
            format!("{:+}", F32::nan())
        );
        assert_eq!("==NaN==", format!("{:=^7}", F32::nan()));
        assert_eq!("=NaN==", format!("{:=^6}", F32::nan()));
        assert_eq!("NaN====", format!("{:=<7}", F32::nan()));
        assert_eq!("====NaN", format!("{:=>7}", F32::nan()));
        assert_eq!("0000NaN", format!("{:=>07}", F32::nan()));
        assert_eq!("0000NaN", format!("{:07}", F32::nan()));
        assert_eq!("0000NaN", format!("{:+07}", F32::nan()));

        assert_eq!("inf", format!("{}", F32::infinity()));
        assert_eq!(
            format!("{}", ::std::f64::INFINITY),
            format!("{}", F32::infinity())
        );
        assert_eq!("==inf==", format!("{:=^7}", F32::infinity()));
        assert_eq!("inf====", format!("{:=<7}", F32::infinity()));
        assert_eq!("====inf", format!("{:=>7}", F32::infinity()));
        assert_eq!("0000inf", format!("{:=<07}", F32::infinity()));
        assert_eq!("+000inf", format!("{:=<+07}", F32::infinity()));

        assert_eq!("+inf", format!("{:+}", F32::infinity()));
        assert_eq!(
            format!("{:+}", ::std::f64::INFINITY),
            format!("{:+}", F32::infinity())
        );
        assert_eq!("=+inf==", format!("{:=^+7}", F32::infinity()));

        assert_eq!("-inf", format!("{}", F32::neg_infinity()));
        assert_eq!(
            format!("{}", ::std::f64::NEG_INFINITY),
            format!("{}", F32::neg_infinity())
        );
        assert_eq!("=-inf==", format!("{:=^7}", F32::neg_infinity()));

        assert_eq!("-inf", format!("{:+}", F32::neg_infinity()));
        assert_eq!(
            format!("{:+}", ::std::f64::NEG_INFINITY),
            format!("{:+}", F32::neg_infinity())
        );
        assert_eq!("=-inf==", format!("{:=^+7}", F32::neg_infinity()));

        // zero
        assert_eq!("0", format!("{}", F32::zero()));
        assert_eq!(format!("{}", 0.0f64), format!("{}", F32::zero()));

        assert_eq!("+0", format!("{:+}", F32::zero()));
        assert_eq!(format!("{:+}", 0.0f64), format!("{:+}", F32::zero()));

        assert_eq!("0", format!("{:.2}", F32::zero()));
        assert_eq!("+0", format!("{:+.2}", F32::zero()));
        assert_eq!("0", format!("{:-.2}", F32::zero()));

        assert_eq!("0.00", format!("{:#.2}", F32::zero()));
        assert_eq!("+0.00", format!("{:+#.2}", F32::zero()));
        assert_eq!("0.00", format!("{:-#.2}", F32::zero()));

        // neg zero
        assert_eq!("-0", format!("{}", F32::neg_zero()));
        assert_eq!("-0", format!("{:+}", F32::neg_zero()));
        assert_eq!("0", format!("{:-}", F32::neg_zero()));

        assert_eq!("-0", format!("{:.2}", F32::neg_zero()));
        assert_eq!("-0", format!("{:+.2}", F32::neg_zero()));
        assert_eq!("0", format!("{:-.2}", F32::neg_zero()));

        // positive whole
        assert_eq!("1", format!("{}", F32::one()));
        assert_eq!("+1", format!("{:+}", F32::one()));
        assert_eq!("1", format!("{:-}", F32::one()));

        assert_eq!("0001", format!("{:04}", F32::one()));
        assert_eq!("+001", format!("{:+04}", F32::one()));

        assert_eq!("=1==", format!("{:=^4}", F32::one()));
        assert_eq!("===1", format!("{:=>4}", F32::one()));
        assert_eq!("1===", format!("{:=<4}", F32::one()));
        assert_eq!("=+1=", format!("{:=^+4}", F32::one()));

        // negative whole
        assert_eq!("-1", format!("{}", -F32::one()));
        assert_eq!("-1", format!("{:+}", -F32::one()));
        assert_eq!("1", format!("{:-}", -F32::one()));

        assert_eq!("=-1=", format!("{:=^4}", -F32::one()));
        assert_eq!("=1==", format!("{:=^-4}", -F32::one()));
        assert_eq!("=-1=", format!("{:=^+4}", -F32::one()));

        // positive fraction
        assert_eq!("00.5", format!("{:04.1}", F32::new(1, 2)));
        assert_eq!("+0.5", format!("{:+04.1}", F32::new(1, 2)));
        assert_eq!("00.5", format!("{:-04.1}", F32::new(1, 2)));

        assert_eq!("0.5", format!("{:.16}", F32::new(1, 2)));
        assert_eq!("+0.5", format!("{:+.16}", F32::new(1, 2)));
        assert_eq!("+1.75", format!("{:=^+.4}", F32::new(7, 4)));
        assert_eq!("=+1.75==", format!("{:=^+8.4}", F32::new(7, 4)));
        assert_eq!("   +1.75", format!("{:+8.4}", F32::new(7, 4)));
        assert_eq!("   +1.75", format!("{:>+8.4}", F32::new(7, 4)));
        assert_eq!("+1.75   ", format!("{:<+8.4}", F32::new(7, 4)));
        assert_eq!("+0001.75", format!("{:<+08.4}", F32::new(7, 4)));

        // negative fraction
        assert_eq!("-00.5", format!("{:05.2}", -F32::new(1, 2)));
        assert_eq!("-00.5", format!("{:05.2}", -F32::new(1, 2)));
        assert_eq!("-00.5", format!("{:+05.2}", -F32::new(1, 2)));
        assert_eq!("000.5", format!("{:-05.2}", -F32::new(1, 2)));

        assert_eq!("-0.5", format!("{:.16}", -F32::new(1, 2)));
        assert_eq!("0.5", format!("{:-.16}", -F32::new(1, 2)));
        assert_eq!("-0.5", format!("{:+.16}", -F32::new(1, 2)));

        assert_eq!("-1.75", format!("{:=^+.4}", -F32::new(7, 4)));
        assert_eq!("=-1.75==", format!("{:=^+8.4}", -F32::new(7, 4)));
        assert_eq!("   -1.75", format!("{:+8.4}", -F32::new(7, 4)));
        assert_eq!("   -1.75", format!("{:>+8.4}", -F32::new(7, 4)));
        assert_eq!("-1.75   ", format!("{:<+8.4}", -F32::new(7, 4)));
        assert_eq!("-0001.75", format!("{:<+08.4}", -F32::new(7, 4)));

        assert_eq!("-1.75", format!("{:=^.4}", -F32::new(7, 4)));
        assert_eq!("=-1.75==", format!("{:=^8.4}", -F32::new(7, 4)));
        assert_eq!("   -1.75", format!("{:8.4}", -F32::new(7, 4)));
        assert_eq!("   -1.75", format!("{:>8.4}", -F32::new(7, 4)));
        assert_eq!("-1.75   ", format!("{:<8.4}", -F32::new(7, 4)));
        assert_eq!("-0001.75", format!("{:<08.4}", -F32::new(7, 4)));

        assert_eq!("-01.7500", format!("{:<#08.4}", -F32::new(7, 4)));

        assert_eq!("1.75", format!("{:=^-.4}", -F32::new(7, 4)));
        assert_eq!("==1.75==", format!("{:=^-8.4}", -F32::new(7, 4)));
        assert_eq!("    1.75", format!("{:-8.4}", -F32::new(7, 4)));
        assert_eq!("    1.75", format!("{:>-8.4}", -F32::new(7, 4)));
        assert_eq!("1.75    ", format!("{:<-8.4}", -F32::new(7, 4)));
        assert_eq!("00001.75", format!("{:<-08.4}", -F32::new(7, 4)));

        // ratios
        assert_eq!("7/4", format!("{}", F32::new(7, 4)));
        assert_eq!("7/4", format!("{:-}", F32::new(7, 4)));
        assert_eq!("+7/4", format!("{:+}", F32::new(7, 4)));

        assert_eq!("-7/4", format!("{}", -F32::new(7, 4)));
        assert_eq!("7/4", format!("{:-}", -F32::new(7, 4)));
        assert_eq!("-7/4", format!("{:+}", -F32::new(7, 4)));

        assert_eq!("07/4", format!("{:04}", F32::new(7, 4)));
        assert_eq!("-7/4", format!("{:+04}", -F32::new(7, 4)));

        assert_eq!("==7/4==", format!("{:=^7}", F32::new(7, 4)));
        assert_eq!("=-7/4==", format!("{:=^7}", -F32::new(7, 4)));
        assert_eq!("    7/4", format!("{:7}", F32::new(7, 4)));
        assert_eq!("7/4    ", format!("{:<7}", F32::new(7, 4)));
        assert_eq!("-7/4   ", format!("{:<7}", -F32::new(7, 4)));
        assert_eq!("+7/4   ", format!("{:<+7}", F32::new(7, 4)));
        assert_eq!("+0007/4", format!("{:<+07}", F32::new(7, 4)));

        // former format_as_decimal tests
        assert_eq!("NaN", format!("{:.64}", F32::from(::std::f32::NAN)));
        assert_eq!("inf", format!("{:.64}", F32::from(::std::f32::INFINITY)));
        assert_eq!(
            "-inf",
            format!("{:.64}", F32::from(::std::f32::NEG_INFINITY))
        );

        assert_eq!("0.75", format!("{:.64}", F32::from(0.75)));
        assert_eq!("-0.75", format!("{:.64}", F32::from(-0.75)));
        assert_eq!("0.33", format!("{:.64}", F32::from((33, 100))));
        assert_eq!(
            "0.0000000456",
            format!("{:.64}", F64::new(456u64, 10000000000u64))
        );

        #[cfg(feature = "with-bigint")]
        {
            let fra = BigFraction::new(
                BigUint::from(42u8),
                BigUint::from(1000000000000000u64)
                    * BigUint::from(1000000000000000u64)
                    * BigUint::from(1000000000000000u64),
            );

            assert_eq!(
                "0.000000000000000000000000000000000000000000042",
                format!("{:.64}", fra)
            );
        }
    }
}
