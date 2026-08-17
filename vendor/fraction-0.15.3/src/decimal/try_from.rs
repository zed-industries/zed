use super::GenericDecimal;
use generic::GenericInteger;
use num::ToPrimitive;
use std::convert::TryFrom;

macro_rules! generic_decimal_into_primitive {
    ( $( $(#[$cfg:meta])* fn $method:ident -> $IntoT:ident ; )*) => {$(
        $(#[$cfg])*
        impl<T, P> TryFrom<GenericDecimal<T, P>> for $IntoT
        where
            T: Clone + ToPrimitive + GenericInteger,
            P: Copy + Into<usize> + GenericInteger
        {
            type Error = ();

            fn try_from(value: GenericDecimal<T, P>) -> Result<Self, Self::Error> {
                value.$method().ok_or(())
            }
        }
    )*}
}

generic_decimal_into_primitive!(
    fn to_u8 -> u8 ;
    fn to_i8 -> i8 ;
    fn to_u16 -> u16 ;
    fn to_i16 -> i16 ;
    fn to_u32 -> u32 ;
    fn to_i32 -> i32 ;
    fn to_u64 -> u64 ;
    fn to_i64 -> i64 ;
    fn to_u128 -> u128 ;
    fn to_i128 -> i128 ;
    fn to_usize -> usize ;
    fn to_isize -> isize ;
    fn to_f32 -> f32 ;
    fn to_f64 -> f64 ;
);

#[cfg(feature = "with-bigint")]
pub mod with_bigint {
    use num::BigInt;

    use crate::{generic::GenericInteger, BigUint, GenericDecimal};
    use std::convert::TryFrom;

    impl<T, P> TryFrom<GenericDecimal<T, P>> for BigUint
    where
        T: Clone + GenericInteger,
        P: Copy + GenericInteger + Into<usize>,
    {
        type Error = ();

        fn try_from(value: GenericDecimal<T, P>) -> Result<Self, Self::Error> {
            Self::try_from(value.0)
        }
    }

    impl<T, P> TryFrom<GenericDecimal<T, P>> for BigInt
    where
        T: Clone + GenericInteger,
        P: Copy + GenericInteger + Into<usize>,
    {
        type Error = ();

        fn try_from(value: GenericDecimal<T, P>) -> Result<Self, Self::Error> {
            Self::try_from(value.0)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::Decimal;
    use num::{One, Zero};
    use std::convert::TryInto;

    #[test]
    fn decimal_to_i() {
        let n_inf = Decimal::neg_infinity();
        let n_one = Decimal::one() * -1;
        let n_half = n_one / 2;
        let zero = Decimal::zero();
        let p_one = Decimal::one();
        let p_half = p_one / 2;
        let p_inf = Decimal::infinity();
        let nan = Decimal::nan();

        assert_eq!(n_inf.try_into() as Result<i8, ()>, Err(()));
        assert_eq!(n_inf.try_into() as Result<i16, ()>, Err(()));
        assert_eq!(n_inf.try_into() as Result<i32, ()>, Err(()));
        assert_eq!(n_inf.try_into() as Result<i64, ()>, Err(()));
        assert_eq!(n_inf.try_into() as Result<i128, ()>, Err(()));
        assert_eq!(n_inf.try_into() as Result<isize, ()>, Err(()));

        assert_eq!(n_one.try_into(), Ok(-1i8));
        assert_eq!(n_one.try_into(), Ok(-1i16));
        assert_eq!(n_one.try_into(), Ok(-1i32));
        assert_eq!(n_one.try_into(), Ok(-1i64));
        assert_eq!(n_one.try_into(), Ok(-1i128));
        assert_eq!(n_one.try_into(), Ok(-1isize));

        assert_eq!(n_half.try_into() as Result<i8, ()>, Err(()));
        assert_eq!(n_half.try_into() as Result<i16, ()>, Err(()));
        assert_eq!(n_half.try_into() as Result<i32, ()>, Err(()));
        assert_eq!(n_half.try_into() as Result<i64, ()>, Err(()));
        assert_eq!(n_half.try_into() as Result<i128, ()>, Err(()));
        assert_eq!(n_half.try_into() as Result<isize, ()>, Err(()));

        assert_eq!(zero.try_into(), Ok(0i8));
        assert_eq!(zero.try_into(), Ok(0i16));
        assert_eq!(zero.try_into(), Ok(0i32));
        assert_eq!(zero.try_into(), Ok(0i64));
        assert_eq!(zero.try_into(), Ok(0i128));
        assert_eq!(zero.try_into(), Ok(0isize));

        assert_eq!(p_half.try_into() as Result<i8, ()>, Err(()));
        assert_eq!(p_half.try_into() as Result<i16, ()>, Err(()));
        assert_eq!(p_half.try_into() as Result<i32, ()>, Err(()));
        assert_eq!(p_half.try_into() as Result<i64, ()>, Err(()));
        assert_eq!(p_half.try_into() as Result<i128, ()>, Err(()));
        assert_eq!(p_half.try_into() as Result<isize, ()>, Err(()));

        assert_eq!(p_one.try_into(), Ok(1i8));
        assert_eq!(p_one.try_into(), Ok(1i16));
        assert_eq!(p_one.try_into(), Ok(1i32));
        assert_eq!(p_one.try_into(), Ok(1i64));
        assert_eq!(p_one.try_into(), Ok(1i128));
        assert_eq!(p_one.try_into(), Ok(1isize));

        assert_eq!(p_inf.try_into() as Result<i8, ()>, Err(()));
        assert_eq!(p_inf.try_into() as Result<i16, ()>, Err(()));
        assert_eq!(p_inf.try_into() as Result<i32, ()>, Err(()));
        assert_eq!(p_inf.try_into() as Result<i64, ()>, Err(()));
        assert_eq!(p_inf.try_into() as Result<i128, ()>, Err(()));
        assert_eq!(p_inf.try_into() as Result<isize, ()>, Err(()));

        assert_eq!(nan.try_into() as Result<i8, ()>, Err(()));
        assert_eq!(nan.try_into() as Result<i16, ()>, Err(()));
        assert_eq!(nan.try_into() as Result<i32, ()>, Err(()));
        assert_eq!(nan.try_into() as Result<i64, ()>, Err(()));
        assert_eq!(nan.try_into() as Result<i128, ()>, Err(()));
        assert_eq!(nan.try_into() as Result<isize, ()>, Err(()));
    }

    #[test]
    fn decimal_to_u() {
        let n_inf = Decimal::neg_infinity();
        let n_one = Decimal::one() * -1;
        let n_half = n_one / 2;
        let zero = Decimal::zero();
        let p_one = Decimal::one();
        let p_half = p_one / 2;
        let p_inf = Decimal::infinity();
        let nan = Decimal::nan();

        assert_eq!(n_inf.try_into() as Result<u8, ()>, Err(()));
        assert_eq!(n_inf.try_into() as Result<u16, ()>, Err(()));
        assert_eq!(n_inf.try_into() as Result<u32, ()>, Err(()));
        assert_eq!(n_inf.try_into() as Result<u64, ()>, Err(()));
        assert_eq!(n_inf.try_into() as Result<u128, ()>, Err(()));
        assert_eq!(n_inf.try_into() as Result<usize, ()>, Err(()));

        assert_eq!(n_one.try_into() as Result<u8, ()>, Err(()));
        assert_eq!(n_one.try_into() as Result<u16, ()>, Err(()));
        assert_eq!(n_one.try_into() as Result<u32, ()>, Err(()));
        assert_eq!(n_one.try_into() as Result<u64, ()>, Err(()));
        assert_eq!(n_one.try_into() as Result<u128, ()>, Err(()));
        assert_eq!(n_one.try_into() as Result<usize, ()>, Err(()));

        assert_eq!(n_half.try_into() as Result<u8, ()>, Err(()));
        assert_eq!(n_half.try_into() as Result<u16, ()>, Err(()));
        assert_eq!(n_half.try_into() as Result<u32, ()>, Err(()));
        assert_eq!(n_half.try_into() as Result<u64, ()>, Err(()));
        assert_eq!(n_half.try_into() as Result<u128, ()>, Err(()));
        assert_eq!(n_half.try_into() as Result<usize, ()>, Err(()));

        assert_eq!(zero.try_into(), Ok(0u8));
        assert_eq!(zero.try_into(), Ok(0u16));
        assert_eq!(zero.try_into(), Ok(0u32));
        assert_eq!(zero.try_into(), Ok(0u64));
        assert_eq!(zero.try_into(), Ok(0u128));
        assert_eq!(zero.try_into(), Ok(0usize));

        assert_eq!(p_half.try_into() as Result<u8, ()>, Err(()));
        assert_eq!(p_half.try_into() as Result<u16, ()>, Err(()));
        assert_eq!(p_half.try_into() as Result<u32, ()>, Err(()));
        assert_eq!(p_half.try_into() as Result<u64, ()>, Err(()));
        assert_eq!(p_half.try_into() as Result<u128, ()>, Err(()));
        assert_eq!(p_half.try_into() as Result<usize, ()>, Err(()));

        assert_eq!(p_one.try_into(), Ok(1u8));
        assert_eq!(p_one.try_into(), Ok(1u16));
        assert_eq!(p_one.try_into(), Ok(1u32));
        assert_eq!(p_one.try_into(), Ok(1u64));
        assert_eq!(p_one.try_into(), Ok(1u128));
        assert_eq!(p_one.try_into(), Ok(1usize));

        assert_eq!(p_inf.try_into() as Result<u8, ()>, Err(()));
        assert_eq!(p_inf.try_into() as Result<u16, ()>, Err(()));
        assert_eq!(p_inf.try_into() as Result<u32, ()>, Err(()));
        assert_eq!(p_inf.try_into() as Result<u64, ()>, Err(()));
        assert_eq!(p_inf.try_into() as Result<u128, ()>, Err(()));
        assert_eq!(p_inf.try_into() as Result<usize, ()>, Err(()));

        assert_eq!(nan.try_into() as Result<u8, ()>, Err(()));
        assert_eq!(nan.try_into() as Result<u16, ()>, Err(()));
        assert_eq!(nan.try_into() as Result<u32, ()>, Err(()));
        assert_eq!(nan.try_into() as Result<u64, ()>, Err(()));
        assert_eq!(nan.try_into() as Result<u128, ()>, Err(()));
        assert_eq!(nan.try_into() as Result<usize, ()>, Err(()));
    }

    #[test]
    fn decimal_to_f32() {
        let n_inf = Decimal::neg_infinity();
        let n_one = Decimal::one() * -1;
        let n_half = n_one / 2;
        let zero = Decimal::zero();
        let p_one = Decimal::one();
        let p_half = p_one / 2;
        let p_inf = Decimal::infinity();
        let nan = Decimal::nan();

        assert_eq!(n_inf.try_into(), Ok(f32::NEG_INFINITY));
        assert_eq!(n_one.try_into(), Ok(-1f32));
        assert_eq!(n_half.try_into(), Ok(-0.5f32));
        assert_eq!(zero.try_into(), Ok(0f32));
        assert_eq!(p_half.try_into(), Ok(0.5f32));
        assert_eq!(p_one.try_into(), Ok(1f32));
        assert_eq!(p_inf.try_into(), Ok(f32::INFINITY));
        assert!((nan.try_into() as Result<f32, ()>).unwrap_or(0f32).is_nan());
    }

    #[test]
    fn decimal_to_f64() {
        let n_inf = Decimal::neg_infinity();
        let n_one = Decimal::one() * -1;
        let n_half = n_one / 2;
        let zero = Decimal::zero();
        let p_one = Decimal::one();
        let p_half = p_one / 2;
        let p_inf = Decimal::infinity();
        let nan = Decimal::nan();

        assert_eq!(n_inf.try_into(), Ok(f64::NEG_INFINITY));
        assert_eq!(n_one.try_into(), Ok(-1f64));
        assert_eq!(n_half.try_into(), Ok(-0.5f64));
        assert_eq!(zero.try_into(), Ok(0f64));
        assert_eq!(p_half.try_into(), Ok(0.5f64));
        assert_eq!(p_one.try_into(), Ok(1f64));
        assert_eq!(p_inf.try_into(), Ok(f64::INFINITY));
        assert!((nan.try_into() as Result<f64, ()>).unwrap_or(0f64).is_nan());
    }

    #[cfg(feature = "with-bigint")]
    #[test]
    fn decimal_to_bigint() {
        use crate::BigInt;

        let n_inf = Decimal::neg_infinity();
        let n_one = Decimal::one() * -1;
        let n_half = n_one / 2;
        let zero = Decimal::zero();
        let p_one = Decimal::one();
        let p_half = p_one / 2;
        let p_inf = Decimal::infinity();
        let nan = Decimal::nan();

        assert_eq!(n_inf.try_into() as Result<BigInt, ()>, Err(()));
        assert_eq!(n_one.try_into(), Ok(BigInt::one() * -1));
        assert_eq!(n_half.try_into() as Result<BigInt, ()>, Err(()));
        assert_eq!(zero.try_into(), Ok(BigInt::zero()));
        assert_eq!(p_half.try_into() as Result<BigInt, ()>, Err(()));
        assert_eq!(p_one.try_into(), Ok(BigInt::one()));
        assert_eq!(p_inf.try_into() as Result<BigInt, ()>, Err(()));
        assert_eq!(nan.try_into() as Result<BigInt, ()>, Err(()));
    }

    #[cfg(feature = "with-bigint")]
    #[test]
    fn decimal_to_biguint() {
        use crate::BigUint;

        let n_inf = Decimal::neg_infinity();
        let n_one = Decimal::one() * -1;
        let n_half = n_one / 2;
        let zero = Decimal::zero();
        let p_one = Decimal::one();
        let p_half = p_one / 2;
        let p_inf = Decimal::infinity();
        let nan = Decimal::nan();

        assert_eq!(n_inf.try_into() as Result<BigUint, ()>, Err(()));
        assert_eq!(n_one.try_into() as Result<BigUint, ()>, Err(()));
        assert_eq!(n_half.try_into() as Result<BigUint, ()>, Err(()));
        assert_eq!(zero.try_into(), Ok(BigUint::zero()));
        assert_eq!(p_half.try_into() as Result<BigUint, ()>, Err(()));
        assert_eq!(p_one.try_into(), Ok(BigUint::one()));
        assert_eq!(p_inf.try_into() as Result<BigUint, ()>, Err(()));
        assert_eq!(nan.try_into() as Result<BigUint, ()>, Err(()));
    }

    #[cfg(feature = "with-bigint")]
    #[test]
    fn bigdecimal_to_i() {
        use crate::BigDecimal;

        let n_inf = BigDecimal::neg_infinity();
        let n_one = BigDecimal::one() * -1;
        let n_half = n_one.clone() / 2;
        let zero = BigDecimal::zero();
        let p_one = BigDecimal::one();
        let p_half = p_one.clone() / 2;
        let p_inf = BigDecimal::infinity();
        let nan = BigDecimal::nan();

        assert_eq!(n_inf.clone().try_into() as Result<i8, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<i16, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<i32, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<i64, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<i128, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<isize, ()>, Err(()));

        assert_eq!(n_one.clone().try_into(), Ok(-1i8));
        assert_eq!(n_one.clone().try_into(), Ok(-1i16));
        assert_eq!(n_one.clone().try_into(), Ok(-1i32));
        assert_eq!(n_one.clone().try_into(), Ok(-1i64));
        assert_eq!(n_one.clone().try_into(), Ok(-1i128));
        assert_eq!(n_one.clone().try_into(), Ok(-1isize));

        assert_eq!(n_half.clone().try_into() as Result<i8, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<i16, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<i32, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<i64, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<i128, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<isize, ()>, Err(()));

        assert_eq!(zero.clone().try_into(), Ok(0i8));
        assert_eq!(zero.clone().try_into(), Ok(0i16));
        assert_eq!(zero.clone().try_into(), Ok(0i32));
        assert_eq!(zero.clone().try_into(), Ok(0i64));
        assert_eq!(zero.clone().try_into(), Ok(0i128));
        assert_eq!(zero.clone().try_into(), Ok(0isize));

        assert_eq!(p_half.clone().try_into() as Result<i8, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<i16, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<i32, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<i64, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<i128, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<isize, ()>, Err(()));

        assert_eq!(p_one.clone().try_into(), Ok(1i8));
        assert_eq!(p_one.clone().try_into(), Ok(1i16));
        assert_eq!(p_one.clone().try_into(), Ok(1i32));
        assert_eq!(p_one.clone().try_into(), Ok(1i64));
        assert_eq!(p_one.clone().try_into(), Ok(1i128));
        assert_eq!(p_one.clone().try_into(), Ok(1isize));

        assert_eq!(p_inf.clone().try_into() as Result<i8, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<i16, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<i32, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<i64, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<i128, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<isize, ()>, Err(()));

        assert_eq!(nan.clone().try_into() as Result<i8, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<i16, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<i32, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<i64, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<i128, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<isize, ()>, Err(()));
    }

    #[cfg(feature = "with-bigint")]
    #[test]
    fn bigintdecimal_to_i() {
        use crate::GenericDecimal;
        use num::BigInt;

        type BigDecimal = GenericDecimal<BigInt, u8>;

        let n_inf = BigDecimal::neg_infinity();
        let n_one = BigDecimal::one() * -1;
        let n_half = n_one.clone() / 2;
        let zero = BigDecimal::zero();
        let p_one = BigDecimal::one();
        let p_half = p_one.clone() / 2;
        let p_inf = BigDecimal::infinity();
        let nan = BigDecimal::nan();

        assert_eq!(n_inf.clone().try_into() as Result<i8, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<i16, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<i32, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<i64, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<i128, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<isize, ()>, Err(()));

        assert_eq!(n_one.clone().try_into(), Ok(-1i8));
        assert_eq!(n_one.clone().try_into(), Ok(-1i16));
        assert_eq!(n_one.clone().try_into(), Ok(-1i32));
        assert_eq!(n_one.clone().try_into(), Ok(-1i64));
        assert_eq!(n_one.clone().try_into(), Ok(-1i128));
        assert_eq!(n_one.clone().try_into(), Ok(-1isize));

        assert_eq!(n_half.clone().try_into() as Result<i8, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<i16, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<i32, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<i64, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<i128, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<isize, ()>, Err(()));

        assert_eq!(zero.clone().try_into(), Ok(0i8));
        assert_eq!(zero.clone().try_into(), Ok(0i16));
        assert_eq!(zero.clone().try_into(), Ok(0i32));
        assert_eq!(zero.clone().try_into(), Ok(0i64));
        assert_eq!(zero.clone().try_into(), Ok(0i128));
        assert_eq!(zero.clone().try_into(), Ok(0isize));

        assert_eq!(p_half.clone().try_into() as Result<i8, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<i16, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<i32, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<i64, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<i128, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<isize, ()>, Err(()));

        assert_eq!(p_one.clone().try_into(), Ok(1i8));
        assert_eq!(p_one.clone().try_into(), Ok(1i16));
        assert_eq!(p_one.clone().try_into(), Ok(1i32));
        assert_eq!(p_one.clone().try_into(), Ok(1i64));
        assert_eq!(p_one.clone().try_into(), Ok(1i128));
        assert_eq!(p_one.clone().try_into(), Ok(1isize));

        assert_eq!(p_inf.clone().try_into() as Result<i8, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<i16, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<i32, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<i64, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<i128, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<isize, ()>, Err(()));

        assert_eq!(nan.clone().try_into() as Result<i8, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<i16, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<i32, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<i64, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<i128, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<isize, ()>, Err(()));
    }

    #[cfg(feature = "with-bigint")]
    #[test]
    fn bigdecimal_to_u() {
        use crate::BigDecimal;

        let n_inf = BigDecimal::neg_infinity();
        let n_one = BigDecimal::one() * -1;
        let n_half = n_one.clone() / 2;
        let zero = BigDecimal::zero();
        let p_one = BigDecimal::one();
        let p_half = p_one.clone() / 2;
        let p_inf = BigDecimal::infinity();
        let nan = Decimal::nan();

        assert_eq!(n_inf.clone().try_into() as Result<u8, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<u16, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<u32, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<u64, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<u128, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<usize, ()>, Err(()));

        assert_eq!(n_one.clone().try_into() as Result<u8, ()>, Err(()));
        assert_eq!(n_one.clone().try_into() as Result<u16, ()>, Err(()));
        assert_eq!(n_one.clone().try_into() as Result<u32, ()>, Err(()));
        assert_eq!(n_one.clone().try_into() as Result<u64, ()>, Err(()));
        assert_eq!(n_one.clone().try_into() as Result<u128, ()>, Err(()));
        assert_eq!(n_one.clone().try_into() as Result<usize, ()>, Err(()));

        assert_eq!(n_half.clone().try_into() as Result<u8, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<u16, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<u32, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<u64, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<u128, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<usize, ()>, Err(()));

        assert_eq!(zero.clone().try_into(), Ok(0u8));
        assert_eq!(zero.clone().try_into(), Ok(0u16));
        assert_eq!(zero.clone().try_into(), Ok(0u32));
        assert_eq!(zero.clone().try_into(), Ok(0u64));
        assert_eq!(zero.clone().try_into(), Ok(0u128));
        assert_eq!(zero.clone().try_into(), Ok(0usize));

        assert_eq!(p_half.clone().try_into() as Result<u8, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<u16, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<u32, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<u64, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<u128, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<usize, ()>, Err(()));

        assert_eq!(p_one.clone().try_into(), Ok(1u8));
        assert_eq!(p_one.clone().try_into(), Ok(1u16));
        assert_eq!(p_one.clone().try_into(), Ok(1u32));
        assert_eq!(p_one.clone().try_into(), Ok(1u64));
        assert_eq!(p_one.clone().try_into(), Ok(1u128));
        assert_eq!(p_one.clone().try_into(), Ok(1usize));

        assert_eq!(p_inf.clone().try_into() as Result<u8, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<u16, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<u32, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<u64, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<u128, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<usize, ()>, Err(()));

        assert_eq!(nan.clone().try_into() as Result<u8, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<u16, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<u32, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<u64, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<u128, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<usize, ()>, Err(()));
    }

    #[cfg(feature = "with-bigint")]
    #[test]
    fn bigintdecimal_to_u() {
        use crate::GenericDecimal;
        use num::BigInt;

        type BigDecimal = GenericDecimal<BigInt, u8>;

        let n_inf = BigDecimal::neg_infinity();
        let n_one = BigDecimal::one() * -1;
        let n_half = n_one.clone() / 2;
        let zero = BigDecimal::zero();
        let p_one = BigDecimal::one();
        let p_half = p_one.clone() / 2;
        let p_inf = BigDecimal::infinity();
        let nan = Decimal::nan();

        assert_eq!(n_inf.clone().try_into() as Result<u8, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<u16, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<u32, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<u64, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<u128, ()>, Err(()));
        assert_eq!(n_inf.clone().try_into() as Result<usize, ()>, Err(()));

        assert_eq!(n_one.clone().try_into() as Result<u8, ()>, Err(()));
        assert_eq!(n_one.clone().try_into() as Result<u16, ()>, Err(()));
        assert_eq!(n_one.clone().try_into() as Result<u32, ()>, Err(()));
        assert_eq!(n_one.clone().try_into() as Result<u64, ()>, Err(()));
        assert_eq!(n_one.clone().try_into() as Result<u128, ()>, Err(()));
        assert_eq!(n_one.clone().try_into() as Result<usize, ()>, Err(()));

        assert_eq!(n_half.clone().try_into() as Result<u8, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<u16, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<u32, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<u64, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<u128, ()>, Err(()));
        assert_eq!(n_half.clone().try_into() as Result<usize, ()>, Err(()));

        assert_eq!(zero.clone().try_into(), Ok(0u8));
        assert_eq!(zero.clone().try_into(), Ok(0u16));
        assert_eq!(zero.clone().try_into(), Ok(0u32));
        assert_eq!(zero.clone().try_into(), Ok(0u64));
        assert_eq!(zero.clone().try_into(), Ok(0u128));
        assert_eq!(zero.clone().try_into(), Ok(0usize));

        assert_eq!(p_half.clone().try_into() as Result<u8, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<u16, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<u32, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<u64, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<u128, ()>, Err(()));
        assert_eq!(p_half.clone().try_into() as Result<usize, ()>, Err(()));

        assert_eq!(p_one.clone().try_into(), Ok(1u8));
        assert_eq!(p_one.clone().try_into(), Ok(1u16));
        assert_eq!(p_one.clone().try_into(), Ok(1u32));
        assert_eq!(p_one.clone().try_into(), Ok(1u64));
        assert_eq!(p_one.clone().try_into(), Ok(1u128));
        assert_eq!(p_one.clone().try_into(), Ok(1usize));

        assert_eq!(p_inf.clone().try_into() as Result<u8, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<u16, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<u32, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<u64, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<u128, ()>, Err(()));
        assert_eq!(p_inf.clone().try_into() as Result<usize, ()>, Err(()));

        assert_eq!(nan.clone().try_into() as Result<u8, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<u16, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<u32, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<u64, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<u128, ()>, Err(()));
        assert_eq!(nan.clone().try_into() as Result<usize, ()>, Err(()));
    }

    #[cfg(feature = "with-bigint")]
    #[test]
    fn bigdecimal_to_f32() {
        use crate::BigDecimal;

        let n_inf = BigDecimal::neg_infinity();
        let n_one = BigDecimal::one() * -1;
        let n_half = n_one.clone() / 2;
        let zero = BigDecimal::zero();
        let p_one = BigDecimal::one();
        let p_half = p_one.clone() / 2;
        let p_inf = BigDecimal::infinity();
        let nan = BigDecimal::nan();

        assert_eq!(n_inf.try_into(), Ok(f32::NEG_INFINITY));
        assert_eq!(n_one.try_into(), Ok(-1f32));
        assert_eq!(n_half.try_into(), Ok(-0.5f32));
        assert_eq!(zero.try_into(), Ok(0f32));
        assert_eq!(p_half.try_into(), Ok(0.5f32));
        assert_eq!(p_one.try_into(), Ok(1f32));
        assert_eq!(p_inf.try_into(), Ok(f32::INFINITY));
        assert!((nan.try_into() as Result<f32, ()>).unwrap_or(0f32).is_nan());
    }

    #[cfg(feature = "with-bigint")]
    #[test]
    fn bigintdecimal_to_f32() {
        use crate::GenericDecimal;
        use num::BigInt;

        type BigDecimal = GenericDecimal<BigInt, u8>;

        let n_inf = BigDecimal::neg_infinity();
        let n_one = BigDecimal::one() * -1;
        let n_half = n_one.clone() / 2;
        let zero = BigDecimal::zero();
        let p_one = BigDecimal::one();
        let p_half = p_one.clone() / 2;
        let p_inf = BigDecimal::infinity();
        let nan = BigDecimal::nan();

        assert_eq!(n_inf.try_into(), Ok(f32::NEG_INFINITY));
        assert_eq!(n_one.try_into(), Ok(-1f32));
        assert_eq!(n_half.try_into(), Ok(-0.5f32));
        assert_eq!(zero.try_into(), Ok(0f32));
        assert_eq!(p_half.try_into(), Ok(0.5f32));
        assert_eq!(p_one.try_into(), Ok(1f32));
        assert_eq!(p_inf.try_into(), Ok(f32::INFINITY));
        assert!((nan.try_into() as Result<f32, ()>).unwrap_or(0f32).is_nan());
    }

    #[cfg(feature = "with-bigint")]
    #[test]
    fn bigdecimal_to_f64() {
        use crate::BigDecimal;

        let n_inf = BigDecimal::neg_infinity();
        let n_one = BigDecimal::one() * -1;
        let n_half = n_one.clone() / 2;
        let zero = BigDecimal::zero();
        let p_one = BigDecimal::one();
        let p_half = p_one.clone() / 2;
        let p_inf = BigDecimal::infinity();
        let nan = BigDecimal::nan();

        assert_eq!(n_inf.try_into(), Ok(f64::NEG_INFINITY));
        assert_eq!(n_one.try_into(), Ok(-1f64));
        assert_eq!(n_half.try_into(), Ok(-0.5f64));
        assert_eq!(zero.try_into(), Ok(0f64));
        assert_eq!(p_half.try_into(), Ok(0.5f64));
        assert_eq!(p_one.try_into(), Ok(1f64));
        assert_eq!(p_inf.try_into(), Ok(f64::INFINITY));
        assert!((nan.try_into() as Result<f64, ()>).unwrap_or(0f64).is_nan());
    }

    #[cfg(feature = "with-bigint")]
    #[test]
    fn bigintdecimal_to_f64() {
        use crate::GenericDecimal;
        use num::BigInt;

        type BigDecimal = GenericDecimal<BigInt, u8>;

        let n_inf = BigDecimal::neg_infinity();
        let n_one = BigDecimal::one() * -1;
        let n_half = n_one.clone() / 2;
        let zero = BigDecimal::zero();
        let p_one = BigDecimal::one();
        let p_half = p_one.clone() / 2;
        let p_inf = BigDecimal::infinity();
        let nan = BigDecimal::nan();

        assert_eq!(n_inf.try_into(), Ok(f64::NEG_INFINITY));
        assert_eq!(n_one.try_into(), Ok(-1f64));
        assert_eq!(n_half.try_into(), Ok(-0.5f64));
        assert_eq!(zero.try_into(), Ok(0f64));
        assert_eq!(p_half.try_into(), Ok(0.5f64));
        assert_eq!(p_one.try_into(), Ok(1f64));
        assert_eq!(p_inf.try_into(), Ok(f64::INFINITY));
        assert!((nan.try_into() as Result<f64, ()>).unwrap_or(0f64).is_nan());
    }
}
