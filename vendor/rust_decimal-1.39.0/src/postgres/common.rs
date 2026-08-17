use crate::{
    ops::array::{div_by_u32, is_all_zero, mul_by_u32},
    Decimal,
};
use core::fmt;
use std::error;

#[derive(Debug, Clone)]
pub struct InvalidDecimal {
    inner: Option<String>,
}

impl fmt::Display for InvalidDecimal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref msg) = self.inner {
            fmt.write_fmt(format_args!("Invalid Decimal: {}", msg))
        } else {
            fmt.write_str("Invalid Decimal")
        }
    }
}

impl error::Error for InvalidDecimal {}

pub(in crate::postgres) struct PostgresDecimal<D> {
    pub neg: bool,
    pub weight: i16,
    pub scale: u16,
    pub digits: D,
}

impl Decimal {
    pub(in crate::postgres) fn checked_from_postgres<D: ExactSizeIterator<Item = u16>>(
        PostgresDecimal {
            neg,
            scale,
            digits,
            weight,
        }: PostgresDecimal<D>,
    ) -> Option<Self> {
        let mut digits = digits.into_iter().collect::<Vec<_>>();

        let fractionals_part_count = digits.len() as i32 + (-weight as i32) - 1;
        let integers_part_count = weight as i32 + 1;

        let mut result = Self::ZERO;
        // adding integer part
        if integers_part_count > 0 {
            let (start_integers, last) = if integers_part_count > digits.len() as i32 {
                (integers_part_count - digits.len() as i32, digits.len() as i32)
            } else {
                (0, integers_part_count)
            };
            let integers: Vec<_> = digits.drain(..last as usize).collect();
            for digit in integers {
                result = result.checked_mul(Self::from_i128_with_scale(10i128.pow(4), 0))?;
                result = result.checked_add(Self::new(digit as i64, 0))?;
            }
            result = result.checked_mul(Self::from_i128_with_scale(10i128.pow(4 * start_integers as u32), 0))?;
        }
        // adding fractional part
        if fractionals_part_count > 0 {
            let start_fractionals = if weight < 0 { (-weight as u32) - 1 } else { 0 };
            for (i, digit) in digits.into_iter().enumerate() {
                let fract_pow = 4_u32.checked_mul(i as u32 + 1 + start_fractionals)?;
                if fract_pow <= Self::MAX_SCALE {
                    result = result.checked_add(
                        Self::new(digit as i64, 0) / Self::from_i128_with_scale(10i128.pow(fract_pow), 0),
                    )?;
                } else if fract_pow == Self::MAX_SCALE + 4 {
                    // rounding last digit
                    if digit >= 5000 {
                        result = result.checked_add(
                            Self::new(1_i64, 0) / Self::from_i128_with_scale(10i128.pow(Self::MAX_SCALE), 0),
                        )?;
                    }
                }
            }
        }

        result.set_sign_negative(neg);
        // Rescale to the postgres value, automatically rounding as needed.
        result.rescale((scale as u32).min(Self::MAX_SCALE));
        Some(result)
    }

    pub(in crate::postgres) fn to_postgres(self) -> PostgresDecimal<Vec<i16>> {
        if self.is_zero() {
            return PostgresDecimal {
                neg: false,
                weight: 0,
                scale: 0,
                digits: vec![0],
            };
        }
        let scale = self.scale() as u16;

        let groups_diff = scale & 0x3; // groups_diff = scale % 4

        let mut mantissa = self.mantissa_array4();

        if groups_diff > 0 {
            let remainder = 4 - groups_diff;
            let power = 10u32.pow(u32::from(remainder));
            mul_by_u32(&mut mantissa, power);
        }

        // array to store max mantissa of Decimal in Postgres decimal format
        const MAX_GROUP_COUNT: usize = 8;
        let mut digits = Vec::with_capacity(MAX_GROUP_COUNT);

        while !is_all_zero(&mantissa) {
            let digit = div_by_u32(&mut mantissa, 10000) as u16;
            digits.push(digit.try_into().unwrap());
        }
        digits.reverse();
        let digits_after_decimal = (scale + 3) / 4;
        let weight = digits.len() as i16 - digits_after_decimal as i16 - 1;

        let unnecessary_zeroes = if weight >= 0 {
            let index_of_decimal = (weight + 1) as usize;
            digits
                .get(index_of_decimal..)
                .expect("enough digits exist")
                .iter()
                .rev()
                .take_while(|i| **i == 0)
                .count()
        } else {
            0
        };
        let relevant_digits = digits.len() - unnecessary_zeroes;
        digits.truncate(relevant_digits);

        PostgresDecimal {
            neg: self.is_sign_negative(),
            digits,
            scale,
            weight,
        }
    }
}
