//! Integer generic traits and operations
//!
//! These traits and functions are for use in generic algorithms
//! that work indifferently to particular types, relying on
//! some common traits.

use super::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, ToPrimitive};
use std::{any::TypeId, cmp::PartialOrd};
use Sign;

#[cfg(feature = "with-bigint")]
use super::{BigInt, BigUint, One, Signed, Zero};

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// Methods common to all integer types that
/// could be used generically in abstract algorithms
pub trait GenericInteger:
    'static
    + Sized
    + Integer
    + ToPrimitive
    + CheckedAdd
    + CheckedDiv
    + CheckedMul
    + CheckedSub
    + Add
    + Div
    + Mul
    + Rem
    + Sub
    + AddAssign
    + DivAssign
    + MulAssign
    + RemAssign
    + SubAssign
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Div<&'a Self, Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> Rem<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> DivAssign<&'a Self>
    + for<'a> MulAssign<&'a Self>
    + for<'a> RemAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
{
    /// Returns value 0 of the type
    fn _0() -> Self;

    /// Returns value 1 of the type
    fn _1() -> Self;

    /// Returns value 10 of the type
    fn _10() -> Self;

    /// Returns Maybe<static reference> of 0
    fn _0r() -> Option<&'static Self>;

    /// Returns Maybe<static reference> of 1
    fn _1r() -> Option<&'static Self>;

    /// Returns Maybe<static reference> of 10
    fn _10r() -> Option<&'static Self>;

    /// Returns the sign and the value itself.
    /// Zero values must have [Sign::Plus]
    fn get_signed_value(self) -> (Sign, Self);
}

#[cfg(feature = "with-bigint")]
lazy_static! {
    static ref _0_BU: BigUint = BigUint::zero();
    static ref _1_BU: BigUint = BigUint::one();
    static ref _10_BU: BigUint = BigUint::from(10u8);
    static ref _0_BI: BigInt = BigInt::zero();
    static ref _1_BI: BigInt = BigInt::one();
    static ref _10_BI: BigInt = BigInt::from(10i8);
}

#[cfg(feature = "with-bigint")]
impl GenericInteger for BigUint {
    #[inline]
    fn _0() -> Self {
        BigUint::zero()
    }
    #[inline]
    fn _1() -> Self {
        BigUint::one()
    }
    #[inline]
    fn _10() -> Self {
        _10_BU.clone()
    }
    #[inline]
    fn _0r() -> Option<&'static Self> {
        Some(&_0_BU)
    }
    #[inline]
    fn _1r() -> Option<&'static Self> {
        Some(&_1_BU)
    }
    #[inline]
    fn _10r() -> Option<&'static Self> {
        Some(&_10_BU)
    }

    #[inline]
    fn get_signed_value(self) -> (Sign, Self) {
        (Sign::Plus, self)
    }
}

#[cfg(feature = "with-bigint")]
impl GenericInteger for BigInt {
    #[inline]
    fn _0() -> Self {
        BigInt::zero()
    }
    #[inline]
    fn _1() -> Self {
        BigInt::one()
    }
    #[inline]
    fn _10() -> Self {
        _10_BI.clone()
    }
    #[inline]
    fn _0r() -> Option<&'static Self> {
        Some(&_0_BI)
    }
    #[inline]
    fn _1r() -> Option<&'static Self> {
        Some(&_1_BI)
    }
    #[inline]
    fn _10r() -> Option<&'static Self> {
        Some(&_10_BI)
    }

    #[inline]
    fn get_signed_value(self) -> (Sign, Self) {
        (
            if self.is_negative() {
                Sign::Minus
            } else {
                Sign::Plus
            },
            self,
        )
    }
}

macro_rules! generic_integer_for_uint {
    ($($t:ty),*) => {
        $(
            impl GenericInteger for $t {
                #[inline]
                fn _0() -> Self { 0 }
                #[inline]
                fn _1() -> Self { 1 }
                #[inline]
                fn _10() -> Self { 10 }
                #[inline]
                fn _0r() -> Option<&'static Self> { None }
                #[inline]
                fn _1r() -> Option<&'static Self> { None }
                #[inline]
                fn _10r() -> Option<&'static Self> { None }

                #[inline]
                fn get_signed_value(self) -> (Sign, Self) { (Sign::Plus, self) }
            }
        )*
    };
}

generic_integer_for_uint!(u8, u16, u32, u64, u128, usize);

macro_rules! generic_integer_for_int {
    ($($t:ty),*) => {
        $(
            impl GenericInteger for $t {
                #[inline]
                fn _0() -> Self { 0 }
                #[inline]
                fn _1() -> Self { 1 }
                #[inline]
                fn _10() -> Self { 10 }
                #[inline]
                fn _0r() -> Option<&'static Self> { None }
                #[inline]
                fn _1r() -> Option<&'static Self> { None }
                #[inline]
                fn _10r() -> Option<&'static Self> { None }

                #[inline]
                fn get_signed_value(self) -> (Sign, Self) {
                    (if self.is_negative() { Sign::Minus } else { Sign::Plus }, self)
                }
            }
        )*
    };
}

generic_integer_for_int!(i8, i16, i32, i64, i128, isize);

/// Builds integer of type T from another integer of type F in a generic way.
/// Guarantees to only return positive values.
///
/// Allows safe runtime conversions between integer types when it's not possible
/// statically. E.g: `i8 -> u8`, `u8 -> i8`, `usize -> u8` or even `BigUint -> u8` and so on.
///
/// Simply reinterprets type F as T when they are the same type.
///
/// # Examples
///
/// ```
/// use fraction::{Sign, generic::read_generic_integer};
///
/// assert_eq!((Sign::Plus, 127i8), read_generic_integer(127u8).unwrap());
/// assert_eq!((Sign::Minus, 128u8), read_generic_integer(-128i8).unwrap());
/// assert_eq!((Sign::Minus, 255u8), read_generic_integer(-255isize).unwrap());
/// ```
pub fn read_generic_integer<T, F>(val: F) -> Option<(Sign, T)>
where
    F: GenericInteger + PartialOrd,
    T: GenericInteger,
{
    let (sign, mut val) = val.get_signed_value();

    if TypeId::of::<T>() == TypeId::of::<F>() && val >= F::zero() {
        let cast = Some((sign, unsafe { (&mut val as *mut F as *mut T).read() }));
        let _ = std::mem::ManuallyDrop::new(val);

        return cast;
    }

    let mut vptr: F = F::_1();
    let mut rptr: T = T::_1();
    let mut result: T = T::_0();

    loop {
        vptr = match F::_10r().map_or_else(
            || vptr.checked_mul(&GenericInteger::_10()),
            |_10| vptr.checked_mul(_10),
        ) {
            Some(v) => v,
            None => break,
        };

        let vdelta: F = val.checked_sub(&val.checked_div(&vptr)?.checked_mul(&vptr)?)?;

        let mut rdelta: T = T::_0();

        let mut vldelta: F = vdelta.checked_div(&F::_10r().map_or_else(
            || vptr.checked_div(&GenericInteger::_10()),
            |_10| vptr.checked_div(_10),
        )?)?;

        loop {
            if F::_0r().map_or_else(|| vldelta == GenericInteger::_0(), |v| vldelta == *v) {
                break;
            }

            rdelta = T::_1r()
                .map_or_else(|| rdelta.checked_add(&T::_1()), |_1| rdelta.checked_add(_1))?;

            vldelta = F::_1r().map_or_else(
                || {
                    if sign == Sign::Plus {
                        vldelta.checked_sub(&GenericInteger::_1())
                    } else {
                        vldelta.checked_add(&GenericInteger::_1())
                    }
                },
                |_1| {
                    if sign == Sign::Plus {
                        vldelta.checked_sub(_1)
                    } else {
                        vldelta.checked_add(_1)
                    }
                },
            )?;
        }

        result = result.checked_add(&rdelta.checked_mul(&rptr)?)?;
        val = val.checked_sub(&vdelta)?;

        if F::_0r().map_or_else(|| val == GenericInteger::_0(), |_0| val == *_0) {
            break;
        }

        rptr = T::_10r().map_or_else(
            || rptr.checked_mul(&GenericInteger::_10()),
            |_10| rptr.checked_mul(_10),
        )?;
    }

    if F::_0r().map_or_else(|| val != GenericInteger::_0(), |_0| val != *_0) {
        let mut vldelta: F = val.checked_div(&vptr)?;

        let mut rdelta: T = T::_0();

        loop {
            if F::_0r().map_or_else(|| vldelta == GenericInteger::_0(), |_0| vldelta == *_0) {
                break;
            }

            rdelta = T::_1r().map_or_else(
                || rdelta.checked_add(&GenericInteger::_1()),
                |_1| rdelta.checked_add(_1),
            )?;

            vldelta = F::_1r().map_or_else(
                || {
                    if sign == Sign::Plus {
                        vldelta.checked_sub(&GenericInteger::_1())
                    } else {
                        vldelta.checked_add(&GenericInteger::_1())
                    }
                },
                |_1| {
                    if sign == Sign::Plus {
                        vldelta.checked_sub(_1)
                    } else {
                        vldelta.checked_add(_1)
                    }
                },
            )?;
        }
        result = result.checked_add(&rdelta.checked_mul(&rptr)?)?;
    }

    Some((sign, result))
}

#[cfg(test)]
mod tests {
    // TODO: tests

    use super::*;

    #[test]
    fn max_to_max() {
        let (s, v) = read_generic_integer::<u32, u32>(u32::max_value()).unwrap();
        assert_eq!(s, Sign::Plus);
        assert_eq!(v, u32::max_value());
    }

    #[test]
    fn sign() {
        let (s, _) = read_generic_integer::<i8, i8>(0i8).unwrap();
        assert_eq!(s, Sign::Plus);

        let (s, _) = read_generic_integer::<i8, i8>(1i8).unwrap();
        assert_eq!(s, Sign::Plus);

        let (s, _) = read_generic_integer::<i8, i8>(-1i8).unwrap();
        assert_eq!(s, Sign::Minus);
    }
}
