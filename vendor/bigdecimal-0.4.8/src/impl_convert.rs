//! Code for implementing From/To BigDecimals

use crate::BigDecimal;
use crate::stdlib::convert::TryFrom;

use num_bigint::BigInt;


macro_rules! impl_from_int_primitive {
    ($t:ty) => {
        impl From<$t> for BigDecimal {
            fn from(n: $t) -> Self {
                BigDecimal {
                    int_val: n.into(),
                    scale: 0,
                }
            }
        }

        impl From<&$t> for BigDecimal {
            fn from(n: &$t) -> Self {
                BigDecimal {
                    int_val: (*n).into(),
                    scale: 0,
                }
            }
        }
    };
}

impl_from_int_primitive!(u8);
impl_from_int_primitive!(u16);
impl_from_int_primitive!(u32);
impl_from_int_primitive!(u64);
impl_from_int_primitive!(u128);
impl_from_int_primitive!(i8);
impl_from_int_primitive!(i16);
impl_from_int_primitive!(i32);
impl_from_int_primitive!(i64);
impl_from_int_primitive!(i128);


impl TryFrom<f32> for BigDecimal {
    type Error = super::ParseBigDecimalError;

    #[inline]
    fn try_from(n: f32) -> Result<Self, Self::Error> {
        crate::parsing::try_parse_from_f32(n)
    }
}

impl TryFrom<f64> for BigDecimal {
    type Error = super::ParseBigDecimalError;

    #[inline]
    fn try_from(n: f64) -> Result<Self, Self::Error> {
        crate::parsing::try_parse_from_f64(n)
    }
}


impl From<BigInt> for BigDecimal {
    fn from(int_val: BigInt) -> Self {
        BigDecimal {
            int_val: int_val,
            scale: 0,
        }
    }
}

// Anything that may be a big-integer paired with a scale
// parameter may be a bigdecimal
impl<T: Into<BigInt>> From<(T, i64)> for BigDecimal {
    fn from((int_val, scale): (T, i64)) -> Self {
        Self {
            int_val: int_val.into(),
            scale: scale,
        }
    }
}
