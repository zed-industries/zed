use rust_decimal::Decimal;

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::numeric::{PgNumeric, PgNumericSign};
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};

use rust_decimal::MathematicalOps;

impl Type<Postgres> for Decimal {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::NUMERIC
    }
}

impl PgHasArrayType for Decimal {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::NUMERIC_ARRAY
    }
}

impl TryFrom<PgNumeric> for Decimal {
    type Error = BoxDynError;

    fn try_from(numeric: PgNumeric) -> Result<Self, BoxDynError> {
        Decimal::try_from(&numeric)
    }
}

impl TryFrom<&'_ PgNumeric> for Decimal {
    type Error = BoxDynError;

    fn try_from(numeric: &'_ PgNumeric) -> Result<Self, BoxDynError> {
        let (digits, sign, mut weight, scale) = match *numeric {
            PgNumeric::Number {
                ref digits,
                sign,
                weight,
                scale,
            } => (digits, sign, weight, scale),

            PgNumeric::NotANumber => {
                return Err("Decimal does not support NaN values".into());
            }
        };

        if digits.is_empty() {
            // Postgres returns an empty digit array for 0
            return Ok(Decimal::ZERO);
        }

        let scale = u32::try_from(scale)
            .map_err(|_| format!("invalid scale value for Pg NUMERIC: {scale}"))?;

        let mut value = Decimal::ZERO;

        // Sum over `digits`, multiply each by its weight and add it to `value`.
        for &digit in digits {
            let mul = Decimal::from(10_000i16)
                .checked_powi(weight as i64)
                .ok_or("value not representable as rust_decimal::Decimal")?;

            let part = Decimal::from(digit) * mul;

            value = value
                .checked_add(part)
                .ok_or("value not representable as rust_decimal::Decimal")?;

            weight = weight.checked_sub(1).ok_or("weight underflowed")?;
        }

        match sign {
            PgNumericSign::Positive => value.set_sign_positive(true),
            PgNumericSign::Negative => value.set_sign_negative(true),
        }

        value.rescale(scale);

        Ok(value)
    }
}

impl From<Decimal> for PgNumeric {
    fn from(value: Decimal) -> Self {
        PgNumeric::from(&value)
    }
}

// This impl is effectively infallible because `NUMERIC` has a greater range than `Decimal`.
impl From<&'_ Decimal> for PgNumeric {
    // Impl has been manually validated.
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn from(decimal: &Decimal) -> Self {
        if Decimal::is_zero(decimal) {
            return PgNumeric::ZERO;
        }

        assert!(
            (0u32..=28).contains(&decimal.scale()),
            "decimal scale out of range {:?}",
            decimal.unpack(),
        );

        // Cannot overflow: always in the range [0, 28]
        let scale = decimal.scale() as u16;

        let mut mantissa = decimal.mantissa().unsigned_abs();

        // If our scale is not a multiple of 4, we need to go to the next multiple.
        let groups_diff = scale % 4;
        if groups_diff > 0 {
            let remainder = 4 - groups_diff as u32;
            let power = 10u32.pow(remainder) as u128;

            // Impossible to overflow; 0 <= mantissa <= 2^96,
            // and we're multiplying by at most 1,000 (giving us a result < 2^106)
            mantissa *= power;
        }

        // Array to store max mantissa of Decimal in Postgres decimal format.
        let mut digits = Vec::with_capacity(8);

        // Convert to base-10000.
        while mantissa != 0 {
            // Cannot overflow or wrap because of the modulus
            digits.push((mantissa % 10_000) as i16);
            mantissa /= 10_000;
        }

        // We started with the low digits first, but they should actually be at the end.
        digits.reverse();

        // Cannot overflow: strictly smaller than `scale`.
        let digits_after_decimal = scale.div_ceil(4) as i16;

        // `mantissa` contains at most 29 decimal digits (log10(2^96)),
        // split into at most 8 4-digit segments.
        assert!(
            digits.len() <= 8,
            "digits.len() out of range: {}; unpacked: {:?}",
            digits.len(),
            decimal.unpack()
        );

        // Cannot overflow; at most 8
        let num_digits = digits.len() as i16;

        // Find how many 4-digit segments should go before the decimal point.
        // `weight = 0` puts just `digit[0]` before the decimal point, and the rest after.
        let weight = num_digits - digits_after_decimal - 1;

        // Remove non-significant zeroes.
        while let Some(&0) = digits.last() {
            digits.pop();
        }

        PgNumeric::Number {
            sign: match decimal.is_sign_negative() {
                false => PgNumericSign::Positive,
                true => PgNumericSign::Negative,
            },
            // Cannot overflow; between 0 and 28
            scale: scale as i16,
            weight,
            digits,
        }
    }
}

impl Encode<'_, Postgres> for Decimal {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        PgNumeric::from(self).encode(buf)?;

        Ok(IsNull::No)
    }
}

#[doc=include_str!("rust_decimal-range.md")]
impl Decode<'_, Postgres> for Decimal {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        match value.format() {
            PgValueFormat::Binary => PgNumeric::decode(value.as_bytes()?)?.try_into(),
            PgValueFormat::Text => Ok(value.as_str()?.parse::<Decimal>()?),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Decimal, PgNumeric, PgNumericSign};
    use std::convert::TryFrom;

    #[test]
    fn zero() {
        let zero: Decimal = "0".parse().unwrap();

        assert_eq!(PgNumeric::from(&zero), PgNumeric::ZERO,);

        assert_eq!(Decimal::try_from(&PgNumeric::ZERO).unwrap(), Decimal::ZERO);
    }

    #[test]
    fn one() {
        let one: Decimal = "1".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&one).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Positive,
                scale: 0,
                weight: 0,
                digits: vec![1]
            }
        );
    }

    #[test]
    fn ten() {
        let ten: Decimal = "10".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&ten).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Positive,
                scale: 0,
                weight: 0,
                digits: vec![10]
            }
        );
    }

    #[test]
    fn one_hundred() {
        let one_hundred: Decimal = "100".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&one_hundred).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Positive,
                scale: 0,
                weight: 0,
                digits: vec![100]
            }
        );
    }

    #[test]
    fn ten_thousand() {
        // Decimal doesn't normalize here
        let ten_thousand: Decimal = "10000".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&ten_thousand).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Positive,
                scale: 0,
                weight: 1,
                digits: vec![1]
            }
        );
    }

    #[test]
    fn two_digits() {
        let two_digits: Decimal = "12345".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&two_digits).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Positive,
                scale: 0,
                weight: 1,
                digits: vec![1, 2345]
            }
        );
    }

    #[test]
    fn one_tenth() {
        let one_tenth: Decimal = "0.1".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&one_tenth).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Positive,
                scale: 1,
                weight: -1,
                digits: vec![1000]
            }
        );
    }

    #[test]
    fn decimal_1() {
        let decimal: Decimal = "1.2345".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&decimal).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Positive,
                scale: 4,
                weight: 0,
                digits: vec![1, 2345]
            }
        );
    }

    #[test]
    fn decimal_2() {
        let decimal: Decimal = "0.12345".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&decimal).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Positive,
                scale: 5,
                weight: -1,
                digits: vec![1234, 5000]
            }
        );
    }

    #[test]
    fn decimal_3() {
        let decimal: Decimal = "0.01234".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&decimal).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Positive,
                scale: 5,
                weight: -1,
                digits: vec![0123, 4000]
            }
        );
    }

    #[test]
    fn decimal_4() {
        let decimal: Decimal = "12345.67890".parse().unwrap();
        let expected_numeric = PgNumeric::Number {
            sign: PgNumericSign::Positive,
            scale: 5,
            weight: 1,
            digits: vec![1, 2345, 6789],
        };
        assert_eq!(PgNumeric::try_from(&decimal).unwrap(), expected_numeric);

        let actual_decimal = Decimal::try_from(expected_numeric).unwrap();
        assert_eq!(actual_decimal, decimal);
        assert_eq!(actual_decimal.mantissa(), 1234567890);
        assert_eq!(actual_decimal.scale(), 5);
    }

    #[test]
    fn one_digit_decimal() {
        let one_digit_decimal: Decimal = "0.00001234".parse().unwrap();
        let expected_numeric = PgNumeric::Number {
            sign: PgNumericSign::Positive,
            scale: 8,
            weight: -2,
            digits: vec![1234],
        };
        assert_eq!(
            PgNumeric::try_from(&one_digit_decimal).unwrap(),
            expected_numeric
        );

        let actual_decimal = Decimal::try_from(expected_numeric).unwrap();
        assert_eq!(actual_decimal, one_digit_decimal);
        assert_eq!(actual_decimal.mantissa(), 1234);
        assert_eq!(actual_decimal.scale(), 8);
    }

    #[test]
    fn max_value() {
        let expected_numeric = PgNumeric::Number {
            sign: PgNumericSign::Positive,
            scale: 0,
            weight: 7,
            digits: vec![7, 9228, 1625, 1426, 4337, 5935, 4395, 0335],
        };
        assert_eq!(
            PgNumeric::try_from(&Decimal::MAX).unwrap(),
            expected_numeric
        );

        let actual_decimal = Decimal::try_from(expected_numeric).unwrap();
        assert_eq!(actual_decimal, Decimal::MAX);
        // Value split by 10,000's to match the expected digits[]
        assert_eq!(
            actual_decimal.mantissa(),
            7_9228_1625_1426_4337_5935_4395_0335
        );
        assert_eq!(actual_decimal.scale(), 0);
    }

    #[test]
    fn max_value_max_scale() {
        let mut max_value_max_scale = Decimal::MAX;
        max_value_max_scale.set_scale(28).unwrap();

        let expected_numeric = PgNumeric::Number {
            sign: PgNumericSign::Positive,
            scale: 28,
            weight: 0,
            digits: vec![7, 9228, 1625, 1426, 4337, 5935, 4395, 0335],
        };
        assert_eq!(
            PgNumeric::try_from(&max_value_max_scale).unwrap(),
            expected_numeric
        );

        let actual_decimal = Decimal::try_from(expected_numeric).unwrap();
        assert_eq!(actual_decimal, max_value_max_scale);
        assert_eq!(
            actual_decimal.mantissa(),
            79_228_162_514_264_337_593_543_950_335
        );
        assert_eq!(actual_decimal.scale(), 28);
    }

    #[test]
    fn issue_423_four_digit() {
        // This is a regression test for https://github.com/launchbadge/sqlx/issues/423
        let four_digit: Decimal = "1234".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&four_digit).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Positive,
                scale: 0,
                weight: 0,
                digits: vec![1234]
            }
        );
    }

    #[test]
    fn issue_423_negative_four_digit() {
        // This is a regression test for https://github.com/launchbadge/sqlx/issues/423
        let negative_four_digit: Decimal = "-1234".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&negative_four_digit).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Negative,
                scale: 0,
                weight: 0,
                digits: vec![1234]
            }
        );
    }

    #[test]
    fn issue_423_eight_digit() {
        // This is a regression test for https://github.com/launchbadge/sqlx/issues/423
        let eight_digit: Decimal = "12345678".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&eight_digit).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Positive,
                scale: 0,
                weight: 1,
                digits: vec![1234, 5678]
            }
        );
    }

    #[test]
    fn issue_423_negative_eight_digit() {
        // This is a regression test for https://github.com/launchbadge/sqlx/issues/423
        let negative_eight_digit: Decimal = "-12345678".parse().unwrap();
        assert_eq!(
            PgNumeric::try_from(&negative_eight_digit).unwrap(),
            PgNumeric::Number {
                sign: PgNumericSign::Negative,
                scale: 0,
                weight: 1,
                digits: vec![1234, 5678]
            }
        );
    }

    #[test]
    fn issue_2247_trailing_zeros() {
        // This is a regression test for https://github.com/launchbadge/sqlx/issues/2247
        let one_hundred: Decimal = "100.00".parse().unwrap();
        let expected_numeric = PgNumeric::Number {
            sign: PgNumericSign::Positive,
            scale: 2,
            weight: 0,
            digits: vec![100],
        };
        assert_eq!(PgNumeric::try_from(&one_hundred).unwrap(), expected_numeric);

        let actual_decimal = Decimal::try_from(expected_numeric).unwrap();
        assert_eq!(actual_decimal, one_hundred);
        assert_eq!(actual_decimal.mantissa(), 10000);
        assert_eq!(actual_decimal.scale(), 2);
    }
}
