//! Dynamic unsigned integer type selection
//!
//! Implements a wrapper around two unsigned integer types that picks the size
//! dynamically (at runtime) depending on the value.
//!
//! Most useful in combination with heap allocated integers such as [BigUint].
//! If we expect small numbers to operate with, and we wish to work without
//! allocations by default. However, we want to handle huge numbers
//! gracefully, without stack overflows.
//!
//! # Examples
//! ```
//! use fraction::{dynaint::DynaInt, One};
//! type DI = DynaInt<u8, u16>;
//!
//! let one: DI = DynaInt::one();
//! let mut num: DI = DynaInt::from(255u8);
//! assert_eq!(num.unpack(), Ok(255u8));
//!
//! num += one;
//! assert_eq!(num.unpack(), Err(256u16));
//!
//! num -= one;  // here it automatically detects the number fits into u8 again
//! assert_eq!(num.unpack(), Ok(255u8));
//! ```

use std::mem;

use num::{
    bigint::{ToBigInt, ToBigUint},
    Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, Num, One, ToPrimitive, Zero,
};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use super::Sign;

#[cfg(feature = "with-bigint")]
use num::BigUint;

use convert::TryToConvertFrom;
use generic::{read_generic_integer, GenericInteger};

/// The wrapper implementation
///
/// Keeps data within S (small) whenever possible, performing
/// checked arithmetic and moving onto __H (huge) when
/// overflows happen. Every math operation on __H performs a read of the
/// resulting number with [TryToConvertFrom::try_to_convert_from].
/// Every math operation on S is checked for overflows.
#[cfg_attr(feature = "with-serde-support", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8>,
    G: Clone + GenericInteger,
{
    /// Represents the small type, implementing Copy and allocated on stack.
    /// The wrapper tries to reduce contained values to this type whenever possible
    S(T),

    /// Represents the huge type, implementing Clone.
    /// To be used when values overflow T
    __H(G),
}

impl<T, G> Copy for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8>,
    G: Copy + GenericInteger,
{
}

impl<T, G> fmt::Display for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + fmt::Display,
    G: Clone + GenericInteger + fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DynaInt::S(value) => write!(formatter, "{}", value),
            DynaInt::__H(value) => write!(formatter, "{}", value),
        }
    }
}

impl<T, G> Bounded for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + Bounded,
    G: Clone + GenericInteger + Bounded,
{
    fn min_value() -> Self {
        DynaInt::S(T::min_value())
    }

    fn max_value() -> Self {
        DynaInt::__H(G::max_value())
    }
}

impl<T, G> DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8>,
    G: Clone + GenericInteger,
{
    fn h(value: G) -> Self {
        T::try_to_convert_from(value.clone()).map_or(DynaInt::__H(value), DynaInt::S)
    }

    /// Unpacks the value
    ///
    /// Utilises [Result::Ok] for S(small) numbers and [Result::Err] for __H(huge) ones
    ///
    /// # Examples
    /// ```
    /// use fraction::dynaint::DynaInt;
    /// type D = DynaInt<u8, u16>;
    ///
    /// assert_eq!(Ok(1u8), D::from(1u8).unpack());
    /// assert_eq!(Err(256u16), D::from(256u16).unpack());
    /// ```
    pub fn unpack(self) -> Result<T, G> {
        match self {
            DynaInt::S(value) => Ok(value),
            DynaInt::__H(value) => Err(value),
        }
    }
}

impl<T, G> ToPrimitive for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + ToPrimitive,
    G: Clone + GenericInteger,
{
    fn to_i64(&self) -> Option<i64> {
        match self {
            DynaInt::S(val) => val.to_i64(),
            DynaInt::__H(val) => val.to_i64(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            DynaInt::S(val) => val.to_u64(),
            DynaInt::__H(val) => val.to_u64(),
        }
    }
}

impl<T, G> GenericInteger for DynaInt<T, G>
where
    T: GenericInteger + Copy + Integer + Into<G> + TryToConvertFrom<G> + From<u8>,
    G: Clone + GenericInteger + 'static,
{
    #[inline]
    fn _0() -> Self {
        DynaInt::S(T::_0())
    }
    #[inline]
    fn _1() -> Self {
        DynaInt::S(T::_1())
    }
    #[inline]
    fn _10() -> Self {
        DynaInt::S(T::_10())
    }
    #[inline]
    fn _0r() -> Option<&'static Self> {
        None
    }
    #[inline]
    fn _1r() -> Option<&'static Self> {
        None
    }
    #[inline]
    fn _10r() -> Option<&'static Self> {
        None
    }

    #[inline]
    fn get_signed_value(self) -> (Sign, Self) {
        (Sign::Plus, self)
    }
}

macro_rules! dyna_impl {
    (impl_trait_birefs_customret; $trait:ident, $fn:ident, $ret:ty) => {
        impl<T, G> $trait for DynaInt<T, G>
        where
            T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + $trait,
            G: Clone + GenericInteger + $trait
        {
            fn $fn(&self, other: &Self) -> $ret {
                match *self {
                    DynaInt::S(a) => match *other {
                        DynaInt::S(b) => $trait::$fn(&a, &b),
                        DynaInt::__H(ref b) => $trait::$fn(&<T as Into<G>>::into(a), b),
                    },
                    DynaInt::__H(ref a) => match *other {
                        DynaInt::S(b) => $trait::$fn(a, &<T as Into<G>>::into(b)),
                        DynaInt::__H(ref b) => $trait::$fn(a, b),
                    },
                }
            }
        }
    };

    (impl_trait_math_unary; $trait:ident, $fn:ident) => {
        impl<T, G> $trait for DynaInt<T, G>
        where
            T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + $trait<Output=T>,
            G: Clone + GenericInteger + $trait<Output=G>
        {
            type Output = Self;

            fn $fn(self) -> Self::Output {
                match self {
                    DynaInt::S(v) => DynaInt::S($trait::$fn(v)),
                    DynaInt::__H(v) => DynaInt::h($trait::$fn(v))
                }
            }
        }
    };

    (impl_trait_math_assign; $trait:ident, $fn:ident, $checked_trait:ident, $checked_fn:ident) => {
        impl<T, G> $trait for DynaInt<T, G>
        where
            T: Copy
                + GenericInteger
                + Into<G>
                + TryToConvertFrom<G>
                + From<u8>
                + $trait
                + $checked_trait,
            G: Clone + GenericInteger + $trait + $checked_trait,
        {
            fn $fn(&mut self, other: Self) {
                *self = match *self {
                    DynaInt::S(a) => match other {
                        DynaInt::S(b) => $checked_trait::$checked_fn(&a, &b).map_or_else(
                            || {
                                let mut a_: G = a.into();
                                $trait::$fn(&mut a_, b.into());
                                DynaInt::h(a_)
                            },
                            DynaInt::S
                        ),
                        DynaInt::__H(b) => {
                            let mut a_: G = a.into();
                            $trait::$fn(&mut a_, b);
                            DynaInt::h(a_)
                        },
                    },
                    DynaInt::__H(ref mut a) => {
                        let mut a_ = mem::replace(a, G::zero());

                        match other {
                            DynaInt::S(b) => {
                                $trait::$fn(&mut a_, b.into());
                                DynaInt::h(a_)
                            },
                            DynaInt::__H(b) => {
                                $trait::$fn(&mut a_, b);
                                DynaInt::h(a_)
                            }
                        }
                    }
                }
            }
        }

        impl<'a, T, G> $trait<&'a Self> for DynaInt<T, G>
        where
            T: Copy
                + GenericInteger
                + Into<G>
                + TryToConvertFrom<G>
                + From<u8>
                + $trait
                + $checked_trait,
            G: Clone + GenericInteger + $trait + $checked_trait,
        {
            fn $fn(&mut self, other: &'a Self) {
                *self = match *self {
                    DynaInt::S(a) => match other {
                        DynaInt::S(b) => $checked_trait::$checked_fn(&a, b).map_or_else(
                            || {
                                let mut a_: G = a.into();
                                $trait::$fn(&mut a_, (*b).into());
                                DynaInt::h(a_)
                            },
                            DynaInt::S
                        ),
                        DynaInt::__H(b) => {
                            let mut a_: G = a.into();
                            $trait::$fn(&mut a_, b);
                            DynaInt::h(a_)
                        },
                    },
                    DynaInt::__H(ref mut a) => {
                        let mut a_ = mem::replace(a, G::zero());

                        match other {
                            DynaInt::S(b) => {
                                $trait::$fn(&mut a_, (*b).into());
                                DynaInt::h(a_)
                            },
                            DynaInt::__H(b) => {
                                $trait::$fn(&mut a_, b);
                                DynaInt::h(a_)
                            }
                        }
                    }
                }
            }
        }
    };

    (impl_trait_math; $trait:ident, $fn:ident, $checked_trait:ident, $checked_fn:ident) => {
        impl<T, G> $trait for DynaInt<T, G>
        where
            T: Copy
                + GenericInteger
                + Into<G>
                + TryToConvertFrom<G>
                + From<u8>
                + $trait<Output = T>
                + $checked_trait,
            G: Clone + GenericInteger + $trait<Output=G> + $checked_trait,
        {
            type Output = Self;

            fn $fn(self, other: Self) -> Self::Output {
                match self {
                    DynaInt::S(a) => match other {
                        DynaInt::S(b) => $checked_trait::$checked_fn(&a, &b).map_or_else(
                            || DynaInt::h($trait::$fn(a.into(), b.into())),
                            DynaInt::S
                        ),
                        DynaInt::__H(b) => DynaInt::h($trait::$fn(a.into(), b)),
                    },
                    DynaInt::__H(a) => match other {
                        DynaInt::S(b) => DynaInt::h($trait::$fn(a, b.into())),
                        DynaInt::__H(b) => DynaInt::h($trait::$fn(a, b)),
                    },
                }
            }
        }

        impl<'a, T, G> $trait for &'a DynaInt<T, G>
        where
            T: Copy
                + GenericInteger
                + Into<G>
                + TryToConvertFrom<G>
                + From<u8>
                + $trait<Output = T>
                + $checked_trait,
            G: Clone + GenericInteger + $trait<Output=G> + $checked_trait,
            &'a G: $trait<G, Output=G> + $trait<Output=G>
        {
            type Output = DynaInt<T, G>;

            fn $fn(self, other: Self) -> Self::Output {
                match *self {
                    DynaInt::S(a) => match *other {
                        DynaInt::S(b) => $checked_trait::$checked_fn(&a, &b).map_or_else(
                            || DynaInt::h($trait::$fn(a.into(), b.into())),
                            DynaInt::S
                        ),
                        DynaInt::__H(ref b) => DynaInt::h($trait::$fn(a.into(), b)),
                    },
                    DynaInt::__H(ref a) => match *other {
                        DynaInt::S(b) => DynaInt::h($trait::$fn(a, b.into())),
                        DynaInt::__H(ref b) => DynaInt::h($trait::$fn(a, b)),
                    },
                }
            }
        }

        impl<'a, T, G> $trait<&'a DynaInt<T, G>> for DynaInt<T, G>
        where
            T: Copy
            + GenericInteger
            + Into<G>
            + TryToConvertFrom<G>
            + From<u8>,
        G: Clone + GenericInteger
        {
            type Output = DynaInt<T, G>;

            fn $fn(self, other: &'a Self) -> Self::Output {
                match self {
                    DynaInt::S(a) => match other {
                        DynaInt::S(b) => $checked_trait::$checked_fn(&a, b).map_or_else(
                            || DynaInt::h($trait::$fn(a.into(), (*b).into())),
                            DynaInt::S
                        ),
                        DynaInt::__H(b) => DynaInt::h($trait::$fn(a.into(), b))
                    },
                    DynaInt::__H(a) => match other {
                        DynaInt::S(b) => DynaInt::h($trait::$fn(a, (*b).into())),
                        DynaInt::__H(b) => DynaInt::h($trait::$fn(a, b))
                    },
                }
            }
        }
    };

    (impl_trait_math_unchecked; $trait:ident, $fn:ident) => {
        impl<T, G> $trait for DynaInt<T, G>
        where
            T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + $trait<Output=T>,
            G: Clone + GenericInteger + $trait<Output=G>
        {
            type Output = Self;

            fn $fn(self, other: Self) -> Self::Output {
                match self {
                    DynaInt::S(a) => match other {
                        DynaInt::S(b) => DynaInt::S($trait::$fn(a, b)),
                        DynaInt::__H(b) => DynaInt::h($trait::$fn(a.into(), b)),
                    },
                    DynaInt::__H(a) => match other {
                        DynaInt::S(b) => DynaInt::h($trait::$fn(a, b.into())),
                        DynaInt::__H(b) => DynaInt::h($trait::$fn(a, b)),
                    },
                }
            }
        }

        impl<'a, T, G> $trait for &'a DynaInt<T, G>
        where
            T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + $trait<Output=T>,
            G: Clone + GenericInteger + $trait<Output=G> + $trait<&'a G, Output=G>,
            &'a G: $trait<G, Output=G> + $trait<Output=G>
        {
            type Output = DynaInt<T, G>;

            fn $fn(self, other: Self) -> Self::Output {
                match *self {
                    DynaInt::S(a) => match *other {
                        DynaInt::S(b) => DynaInt::S($trait::$fn(a, b)),
                        DynaInt::__H(ref b) => DynaInt::h($trait::$fn(a.into(), b)),
                    },
                    DynaInt::__H(ref a) => match *other {
                        DynaInt::S(b) => DynaInt::h($trait::$fn(a, b.into())),
                        DynaInt::__H(ref b) => DynaInt::h($trait::$fn(a, b)),
                    },
                }
            }
        }
    };

    (impl_trait_math_unchecked_no_g_to_ref; $trait:ident, $fn:ident) => {
        impl<T, G> $trait for DynaInt<T, G>
        where
            T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + $trait<Output=T>,
            G: Clone + GenericInteger + $trait<Output=G>
        {
            type Output = Self;

            fn $fn(self, other: Self) -> Self::Output {
                match self {
                    DynaInt::S(a) => match other {
                        DynaInt::S(b) => DynaInt::S($trait::$fn(a, b)),
                        DynaInt::__H(b) => DynaInt::h($trait::$fn(a.into(), b)),
                    },
                    DynaInt::__H(a) => match other {
                        DynaInt::S(b) => DynaInt::h($trait::$fn(a, b.into())),
                        DynaInt::__H(b) => DynaInt::h($trait::$fn(a, b)),
                    },
                }
            }
        }

        impl<'a, T, G> $trait for &'a DynaInt<T, G>
        where
            T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + $trait<Output=T>,
            G: Clone + GenericInteger + $trait<Output=G>,
            &'a G: $trait<G, Output=G> + $trait<Output=G>
        {
            type Output = DynaInt<T, G>;

            fn $fn(self, other: Self) -> Self::Output {
                match *self {
                    DynaInt::S(a) => match *other {
                        DynaInt::S(b) => DynaInt::S($trait::$fn(a, b)),
                        DynaInt::__H(ref b) => DynaInt::h($trait::$fn(a.into(), b)),
                    },
                    DynaInt::__H(ref a) => match *other {
                        DynaInt::S(b) => DynaInt::h($trait::$fn(a, b.into())),
                        DynaInt::__H(ref b) => DynaInt::h($trait::$fn(a, b)),
                    },
                }
            }
        }

        impl<'a, T, G> $trait<&'a DynaInt<T, G>> for DynaInt<T, G>
        where
            T: Copy
            + GenericInteger
            + Into<G>
            + TryToConvertFrom<G>
            + From<u8>,
        G: Clone + GenericInteger
        {
            type Output = DynaInt<T, G>;

            fn $fn(self, other: &'a Self) -> Self::Output {
                match self {
                    DynaInt::S(a) => match other {
                        DynaInt::S(b) => DynaInt::S($trait::$fn(a, b)),
                        DynaInt::__H(b) => DynaInt::h($trait::$fn(a.into(), b))
                    },
                    DynaInt::__H(a) => match other {
                        DynaInt::S(b) => DynaInt::h($trait::$fn(a, (*b).into())),
                        DynaInt::__H(b) => DynaInt::h($trait::$fn(a, b))
                    },
                }
            }
        }
    };

    (impl_trait_math_unchecked_assigned; $trait:ident, $fn:ident) => {
        impl<T, G> $trait for DynaInt<T, G>
        where
            T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + $trait,
            G: Clone + GenericInteger + $trait
        {
            fn $fn(&mut self, other: Self) {
                *self = match *self {
                    DynaInt::S(mut a) => match other {
                        DynaInt::S(b) => { $trait::$fn(&mut a, b); DynaInt::S(a) }
                        DynaInt::__H(b) => { let mut a_ = a.into(); $trait::$fn(&mut a_, b); DynaInt::h(a_) }
                    },
                    DynaInt::__H(ref mut a) => {
                        let mut a_ = mem::replace(a, G::zero());

                        match other {
                            DynaInt::S(b) => { $trait::$fn(&mut a_, b.into()); DynaInt::h(a_) }
                            DynaInt::__H(b) => { $trait::$fn(&mut a_, b); DynaInt::h(a_) }
                        }
                    },
                }
            }
        }

        impl<'a, T, G> $trait<&'a Self> for DynaInt<T, G>
        where
            T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + $trait<&'a T>,
            G: Clone + GenericInteger + $trait<&'a G> + $trait<G>,
        {
            fn $fn(&mut self, other: &'a Self) {
                *self = match *self {
                    DynaInt::S(mut a) => match other {
                        DynaInt::S(b) => { $trait::$fn(&mut a, b); DynaInt::S(a) }
                        DynaInt::__H(b) => { let mut a_ = a.into(); $trait::$fn(&mut a_, b); DynaInt::h(a_) }
                    },
                    DynaInt::__H(ref mut a) => {
                        let mut a_ = mem::replace(a, G::zero());

                        match other {
                            DynaInt::S(b) => { $trait::$fn(&mut a_, (*b).into()); DynaInt::h(a_) }
                            DynaInt::__H(b) => { $trait::$fn(&mut a_, b); DynaInt::h(a_) }
                        }
                    },
                }
            }
        }
    };

    (impl_trait_math_unchecked_assigned_no_trait_for_ref; $trait:ident, $fn:ident) => {
        impl<T, G> $trait for DynaInt<T, G>
        where
            T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + $trait,
            G: Clone + GenericInteger + $trait
        {
            fn $fn(&mut self, other: Self) {
                *self = match *self {
                    DynaInt::S(mut a) => match other {
                        DynaInt::S(b) => { $trait::$fn(&mut a, b); DynaInt::S(a) }
                        DynaInt::__H(b) => { let mut a_ = a.into(); $trait::$fn(&mut a_, b); DynaInt::h(a_) }
                    },
                    DynaInt::__H(ref mut a) => {
                        let mut a_ = mem::replace(a, G::zero());

                        match other {
                            DynaInt::S(b) => { $trait::$fn(&mut a_, b.into()); DynaInt::h(a_) }
                            DynaInt::__H(b) => { $trait::$fn(&mut a_, b); DynaInt::h(a_) }
                        }
                    },
                }
            }
        }

        impl<'a, T, G> $trait<&'a Self> for DynaInt<T, G>
        where
            T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8>,
            G: Clone + GenericInteger,
        {
            fn $fn(&mut self, other: &'a Self) {
                *self = match *self {
                    DynaInt::S(mut a) => match other {
                        DynaInt::S(b) => { $trait::$fn(&mut a, b); DynaInt::S(a) }
                        DynaInt::__H(b) => { let mut a_ = a.into(); $trait::$fn(&mut a_, b); DynaInt::h(a_) }
                    },
                    DynaInt::__H(ref mut a) => {
                        let mut a_ = mem::replace(a, G::zero());

                        match other {
                            DynaInt::S(b) => { $trait::$fn(&mut a_, (*b).into()); DynaInt::h(a_) }
                            DynaInt::__H(b) => { $trait::$fn(&mut a_, b); DynaInt::h(a_) }
                        }
                    },
                }
            }
        }
    };

    (impl_trait_math_checked; $checked_trait:ident, $checked_fn:ident) => {
        impl<T, G> $checked_trait for DynaInt<T, G>
        where
            T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + $checked_trait,
            G: Clone + GenericInteger + $checked_trait
        {
            fn $checked_fn(&self, other: &Self) -> Option<Self> {
                match self {
                    DynaInt::S(a) => match other {
                        DynaInt::S(b) => $checked_trait::$checked_fn(a, b).map_or_else(
                            || {
                                $checked_trait::$checked_fn(
                                    &<T as Into<G>>::into(*a),
                                    &<T as Into<G>>::into(*b),
                                ).map(DynaInt::h)
                            },
                            |val| Some(DynaInt::S(val)),
                        ),
                        DynaInt::__H(b) => {
                            $checked_trait::$checked_fn(&<T as Into<G>>::into(*a), b)
                                .map(DynaInt::h)
                        }
                    },
                    DynaInt::__H(a) => match other {
                        DynaInt::S(b) => {
                            $checked_trait::$checked_fn(a, &<T as Into<G>>::into(*b))
                                .map(DynaInt::h)
                        }
                        DynaInt::__H(b) => {
                            $checked_trait::$checked_fn(a, b).map(DynaInt::h)
                        }
                    },
                }
            }
        }
    };

    (impl_trait_from_uint; $($F:ty),+) => {
        $(
            impl<T, G> From<$F> for DynaInt<T, G>
            where
                $F: GenericInteger + PartialOrd + Into<G>,
                T: GenericInteger + Copy + Into<G> + TryToConvertFrom<G> + From<u8>,
                G: Clone + GenericInteger
            {
                fn from(v: $F) -> DynaInt<T, G> {
                    read_generic_integer::<T, $F>(v)
                        .map_or_else(
                            || DynaInt::h(v.into()),
                            |(_, val)| DynaInt::S(val)
                        )
                }
            }
        )+
    };

    (impl_fn_refmath; $fn:ident) => {
        fn $fn(&self, other: &Self) -> Self {
            match *self {
                DynaInt::S(ref a) => match *other {
                    DynaInt::S(ref b) => DynaInt::S(T::$fn(a, b)),
                    DynaInt::__H(ref b) => DynaInt::h(G::$fn(&<T as Into<G>>::into(*a), b))
                },
                DynaInt::__H(ref a) => match *other {
                    DynaInt::S(ref b) => DynaInt::h(G::$fn(a, &<T as Into<G>>::into(*b))),
                    DynaInt::__H(ref b) => DynaInt::h(G::$fn(a, b)),
                },
            }
        }
    };

    (impl_fn_refmath_bool; $fn:ident) => {
        fn $fn(&self, other: &Self) -> bool {
            match *self {
                DynaInt::S(ref a) => match *other {
                    DynaInt::S(ref b) => T::$fn(a, b),
                    DynaInt::__H(ref b) => G::$fn(&<T as Into<G>>::into(*a), b)
                },
                DynaInt::__H(ref a) => match *other {
                    DynaInt::S(ref b) => G::$fn(a, &<T as Into<G>>::into(*b)),
                    DynaInt::__H(ref b) => G::$fn(a, b)
                }
            }
        }
    };

    (impl_fn_refmath_tuple2self; $fn:ident) => {
        fn $fn(&self, other: &Self) -> (Self, Self) {
            match *self {
                DynaInt::S(ref a) => match *other {
                    DynaInt::S(ref b) => {
                        let (c, d) = T::$fn(a, b);
                        (DynaInt::S(c), DynaInt::S(d))
                    }
                    DynaInt::__H(ref b) => {
                        let (c, d) = G::$fn(&<T as Into<G>>::into(*a), b);
                        (DynaInt::h(c), DynaInt::h(d))
                    }
                },
                DynaInt::__H(ref a) => match *other {
                    DynaInt::S(ref b) => {
                        let (c, d) = G::$fn(a, &<T as Into<G>>::into(*b));
                        (DynaInt::h(c), DynaInt::h(d))
                    }
                    DynaInt::__H(ref b) => {
                        let (c, d) = G::$fn(a, b);
                        (DynaInt::h(c), DynaInt::h(d))
                    }
                }
            }
        }
    };

    (impl_fn_isref; $fn:ident) => {
        fn $fn(&self) -> bool {
            match *self {
                DynaInt::S(ref a) => a.$fn(),
                DynaInt::__H(ref b) => b.$fn()
            }
        }
    };
}

dyna_impl! (impl_trait_birefs_customret; PartialEq, eq, bool);
dyna_impl! (impl_trait_birefs_customret; Ord, cmp, Ordering);

dyna_impl! (impl_trait_math_unary; Neg, neg);
dyna_impl! (impl_trait_math_unary; Not, not);

dyna_impl! (impl_trait_math_unchecked; BitAnd, bitand);
dyna_impl! (impl_trait_math_unchecked; BitOr, bitor);
dyna_impl! (impl_trait_math_unchecked; BitXor, bitxor);
dyna_impl! (impl_trait_math_unchecked; Shl, shl);
dyna_impl! (impl_trait_math_unchecked; Shr, shr);

dyna_impl! (impl_trait_math_unchecked_no_g_to_ref; Rem, rem);

dyna_impl! (impl_trait_math_unchecked_assigned; BitAndAssign, bitand_assign);
dyna_impl! (impl_trait_math_unchecked_assigned; BitOrAssign, bitor_assign);
dyna_impl! (impl_trait_math_unchecked_assigned; BitXorAssign, bitxor_assign);
dyna_impl! (impl_trait_math_unchecked_assigned; ShlAssign, shl_assign);
dyna_impl! (impl_trait_math_unchecked_assigned; ShrAssign, shr_assign);

dyna_impl! (impl_trait_math_unchecked_assigned_no_trait_for_ref; RemAssign, rem_assign);

dyna_impl! (impl_trait_math; Add, add, CheckedAdd, checked_add);
dyna_impl! (impl_trait_math; Div, div, CheckedDiv, checked_div);
dyna_impl! (impl_trait_math; Mul, mul, CheckedMul, checked_mul);
dyna_impl! (impl_trait_math; Sub, sub, CheckedSub, checked_sub);

dyna_impl! (impl_trait_math_assign; AddAssign, add_assign, CheckedAdd, checked_add);
dyna_impl! (impl_trait_math_assign; DivAssign, div_assign, CheckedDiv, checked_div);
dyna_impl! (impl_trait_math_assign; MulAssign, mul_assign, CheckedMul, checked_mul);
dyna_impl! (impl_trait_math_assign; SubAssign, sub_assign, CheckedSub, checked_sub);

dyna_impl! (impl_trait_math_checked; CheckedAdd, checked_add);
dyna_impl! (impl_trait_math_checked; CheckedDiv, checked_div);
dyna_impl! (impl_trait_math_checked; CheckedMul, checked_mul);
dyna_impl! (impl_trait_math_checked; CheckedSub, checked_sub);

dyna_impl!(impl_trait_from_uint; u16, u32, u64, u128, usize);

impl<T, G> PartialOrd for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + PartialOrd,
    G: Clone + GenericInteger + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, G> Eq for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + Eq,
    G: Clone + GenericInteger + Eq,
{
}

impl<T, G> From<u8> for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8>,
    G: Clone + GenericInteger,
{
    fn from(value: u8) -> DynaInt<T, G> {
        DynaInt::S(value.into())
    }
}

#[cfg(feature = "with-bigint")]
impl<T, G> From<BigUint> for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8>,
    G: Clone + GenericInteger + From<BigUint>,
{
    fn from(value: BigUint) -> DynaInt<T, G> {
        DynaInt::h(G::from(value))
    }
}

#[cfg(feature = "with-bigint")]
impl<T, G> From<DynaInt<T, G>> for BigUint
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8>,
    G: Clone + GenericInteger + Into<BigUint>,
{
    fn from(value: DynaInt<T, G>) -> BigUint {
        match value {
            DynaInt::S(v) => <T as Into<G>>::into(v).into(),
            DynaInt::__H(v) => v.into(),
        }
    }
}

impl<T, G> Zero for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8>,
    G: Clone + GenericInteger,
{
    #[inline]
    fn zero() -> Self {
        DynaInt::S(T::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl<T, G> One for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8>,
    G: Clone + GenericInteger,
{
    #[inline]
    fn one() -> Self {
        DynaInt::S(T::one())
    }
}

impl<T, G> Num for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + Num,
    G: Clone + GenericInteger,
{
    type FromStrRadixErr = <G as Num>::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(s, radix)
            .map(DynaInt::S)
            .or_else(|_| G::from_str_radix(s, radix).map(DynaInt::h))
    }
}

impl<T, G> Integer for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8>,
    G: Clone + GenericInteger,
{
    dyna_impl!(impl_fn_refmath; div_floor);
    dyna_impl!(impl_fn_refmath; mod_floor);
    dyna_impl!(impl_fn_refmath; gcd);
    dyna_impl!(impl_fn_refmath; lcm);

    dyna_impl!(impl_fn_refmath_bool; divides);
    dyna_impl!(impl_fn_refmath_bool; is_multiple_of);

    dyna_impl!(impl_fn_isref; is_even);
    dyna_impl!(impl_fn_isref; is_odd);

    dyna_impl!(impl_fn_refmath_tuple2self; div_rem);
}

#[cfg(feature = "with-bigint")]
impl<T, G> ToBigInt for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + ToBigInt,
    G: Clone + GenericInteger + ToBigInt,
{
    fn to_bigint(&self) -> Option<num::BigInt> {
        match self {
            DynaInt::S(s) => s.to_bigint(),
            DynaInt::__H(h) => h.to_bigint(),
        }
    }
}

#[cfg(feature = "with-bigint")]
impl<T, G> ToBigUint for DynaInt<T, G>
where
    T: Copy + GenericInteger + Into<G> + TryToConvertFrom<G> + From<u8> + ToBigUint,
    G: Clone + GenericInteger + ToBigUint,
{
    fn to_biguint(&self) -> Option<num::BigUint> {
        match self {
            DynaInt::S(s) => s.to_biguint(),
            DynaInt::__H(h) => h.to_biguint(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, DynaInt, GenericInteger, Integer, Num, One,
        Sign, ToPrimitive, Zero,
    };

    type D = DynaInt<u8, u16>;

    generate_ops_tests!(
        Zero => {D::zero()};
        One => {D::one()};
        Two => {D::from(2u8)};
        Three => {D::from(3u8)};
        Four => {D::from(4u8)};
    );

    #[test]
    fn growth() {
        let m8 = u8::max_value();

        let mut val = D::from(m8);

        assert_eq!(m8, val.clone().unpack().ok().unwrap());

        val += D::zero();

        assert_eq!(m8, val.clone().unpack().ok().unwrap());

        val += D::one();

        assert_eq!(u16::from(m8) + 1, val.clone().unpack().err().unwrap());

        val += D::one();

        assert_eq!(u16::from(m8) + 2, val.clone().unpack().err().unwrap());

        val -= D::from(2u8);

        assert_eq!(m8, val.clone().unpack().ok().unwrap());
    }

    #[test]
    fn to_primitive() {
        {
            let mut val = D::from(255u8);

            assert_eq!(Some(255u64), val.to_u64());
            val += D::one();
            assert_eq!(Some(256u64), val.to_u64());
        }

        {
            let mut val = D::from(255u8);

            assert_eq!(Some(255i64), val.to_i64());
            val += D::one();
            assert_eq!(Some(256i64), val.to_i64());
        }
    }

    #[test]
    fn generic_integer() {
        assert_eq!(0u8, D::_0().unpack().ok().unwrap());
        assert_eq!(1u8, D::_1().unpack().ok().unwrap());
        assert_eq!(10u8, D::_10().unpack().ok().unwrap());

        assert!(D::_0r().is_none());
        assert!(D::_1r().is_none());
        assert!(D::_10r().is_none());

        let (s, v) = D::from(254u8).get_signed_value();
        assert_eq!(s, Sign::Plus);
        assert_eq!(254u8, v.unpack().ok().unwrap());
    }

    #[test]
    fn partial_eq() {
        assert_ne!(D::from(1u8), D::from(256u16));
        assert_eq!(D::from(256u16), D::from(256u16));
        assert_ne!(D::from(256u16), D::from(257u16));
    }

    #[test]
    fn add_assign() {
        {
            let mut val = D::from(255u8);
            val += D::from(256u16);

            assert_eq!(511u16, val.unpack().err().unwrap());
        }

        {
            let mut val = D::from(255u8);
            val += &D::from(256u16);

            assert_eq!(511u16, val.unpack().err().unwrap());
        }

        {
            let mut val = D::from(256u16);
            val += &D::one();

            assert_eq!(257u16, val.unpack().err().unwrap());
        }

        {
            let mut val = D::from(255u8);
            val += &D::from(255u8);

            assert_eq!(510u16, val.unpack().err().unwrap());
        }

        {
            let mut val = D::from(256u16);
            val += D::from(256u16);

            assert_eq!(512u16, val.unpack().err().unwrap());
        }

        {
            let mut val = D::from(256u16);
            val += &D::from(256u16);

            assert_eq!(512u16, val.unpack().err().unwrap());
        }
    }

    #[test]
    fn op_add() {
        assert_eq!(2u8, (D::one() + &D::one()).unpack().ok().unwrap());

        assert_eq!(257u16, (D::one() + D::from(256u16)).unpack().err().unwrap());
        assert_eq!(256u16, (D::one() + &D::from(255u8)).unpack().err().unwrap());
        assert_eq!(
            257u16,
            (D::one() + &D::from(256u16)).unpack().err().unwrap()
        );
        assert_eq!(
            257u16,
            (&D::one() + &D::from(256u16)).unpack().err().unwrap()
        );

        assert_eq!(257u16, (D::from(256u16) + D::one()).unpack().err().unwrap());
        assert_eq!(
            257u16,
            (D::from(256u16) + &D::one()).unpack().err().unwrap()
        );
        assert_eq!(
            257u16,
            (&D::from(256u16) + &D::one()).unpack().err().unwrap()
        );

        assert_eq!(
            512u16,
            (D::from(256u16) + D::from(256u16)).unpack().err().unwrap()
        );
        assert_eq!(
            512u16,
            (D::from(256u16) + &D::from(256u16)).unpack().err().unwrap()
        );
        assert_eq!(
            512u16,
            (&D::from(256u16) + &D::from(256u16))
                .unpack()
                .err()
                .unwrap()
        );
    }

    #[test]
    fn op_bit_and() {
        assert_eq!(D::one(), D::one() & D::one());
        assert_eq!(D::one(), &D::one() & &D::one());

        assert_eq!(D::zero(), D::one() & D::from(256u16));
        assert_eq!(D::zero(), &D::one() & &D::from(256u16));

        assert_eq!(D::zero(), D::from(256u16) & D::one());
        assert_eq!(D::zero(), &D::from(256u16) & &D::one());

        assert_eq!(D::from(256u16), D::from(256u16) & D::from(257u16));
        assert_eq!(D::from(256u16), &D::from(256u16) & &D::from(257u16));

        {
            let mut v = D::one();
            v &= D::one();
            assert_eq!(v, D::one());
        }

        {
            let mut v = D::one();
            v &= &D::one();
            assert_eq!(v, D::one());
        }

        {
            let mut v = D::one();
            v &= D::from(256u16);
            assert_eq!(v, D::zero());
        }

        {
            let mut v = D::one();
            v &= &D::from(256u16);
            assert_eq!(v, D::zero());
        }

        {
            let mut v = D::from(256u16);
            v &= D::one();
            assert_eq!(v, D::zero());
        }

        {
            let mut v = D::from(256u16);
            v &= &D::one();
            assert_eq!(v, D::zero());
        }

        {
            let mut v = D::from(256u16);
            v &= D::from(257u16);
            assert_eq!(v, D::from(256u16));
        }

        {
            let mut v = D::from(256u16);
            v &= &D::from(257u16);
            assert_eq!(v, D::from(256u16));
        }
    }

    #[test]
    fn op_rem() {
        assert_eq!(D::one(), D::from(3u8) % D::from(2u8));
        assert_eq!(D::one(), D::from(3u8) % &D::from(2u8));
        assert_eq!(D::one(), &D::from(3u8) % &D::from(2u8));

        {
            let mut v = D::from(3u8);
            v %= D::from(2u8);
            assert_eq!(v, D::one());
        }

        {
            let mut v = D::from(3u8);
            v %= &D::from(2u8);
            assert_eq!(v, D::one());
        }

        assert_eq!(D::one(), D::one() % D::from(256u16));
        assert_eq!(D::one(), D::one() % &D::from(256u16));
        assert_eq!(D::one(), &D::one() % &D::from(256u16));

        {
            let mut v = D::from(D::one());
            v %= D::from(256u16);
            assert_eq!(v, D::one());
        }

        {
            let mut v = D::from(D::one());
            v %= &D::from(256u16);
            assert_eq!(v, D::one());
        }

        assert_eq!(D::zero(), D::from(256u16) % D::one());
        assert_eq!(D::zero(), D::from(256u16) % &D::one());
        assert_eq!(D::zero(), &D::from(256u16) % &D::one());

        {
            let mut v = D::from(256u16);
            v %= D::one();
            assert_eq!(v, D::zero());
        }

        {
            let mut v = D::from(256u16);
            v %= &D::one();
            assert_eq!(v, D::zero());
        }

        assert_eq!(D::zero(), D::from(256u16) % D::from(256u16));
        assert_eq!(D::zero(), D::from(256u16) % &D::from(256u16));
        assert_eq!(D::zero(), &D::from(256u16) % &D::from(256u16));

        {
            let mut v = D::from(D::from(256u16));
            v %= D::from(256u16);
            assert_eq!(v, D::zero());
        }

        {
            let mut v = D::from(D::from(256u16));
            v %= &D::from(256u16);
            assert_eq!(v, D::zero());
        }
    }

    #[test]
    fn checked_math() {
        assert_eq!(Some(D::zero()), D::zero().checked_add(&D::zero()));
        assert_eq!(Some(D::from(256u16)), D::one().checked_add(&D::from(255u8)));
        assert_eq!(
            Some(D::from(257u16)),
            D::one().checked_add(&D::from(256u16))
        );

        assert_eq!(
            Some(D::from(257u16)),
            D::from(256u16).checked_add(&D::one())
        );
        assert_eq!(
            Some(D::from(512u16)),
            D::from(256u16).checked_add(&D::from(256u16))
        );

        assert_eq!(None, D::from(u16::max_value()).checked_add(&D::one()));
        assert_eq!(None, D::one().checked_add(&D::from(u16::max_value())));
    }

    #[test]
    fn from_uint() {
        type D = DynaInt<u16, u32>;

        assert_eq!(D::one(), D::from(1u16));
    }

    #[cfg(feature = "with-bigint")]
    #[test]
    fn from_biguint() {
        use super::BigUint;

        type D = DynaInt<u8, BigUint>;

        assert_eq!(D::one(), D::from(BigUint::one()));
    }

    #[test]
    fn zero() {
        assert!(D::zero().is_zero());
    }

    #[test]
    fn from_str_radix() {
        assert_eq!(D::from(1u8), D::from_str_radix("1", 10).unwrap());
        assert_eq!(D::from(256u16), D::from_str_radix("256", 10).unwrap());
    }

    #[test]
    fn integer() {
        assert_eq!(D::from(127u8), D::from(255u8).div_floor(&D::from(2u8)));
        assert_eq!(D::zero(), D::one().div_floor(&D::from(256u16)));

        assert_eq!(D::from(128u8), D::from(256u16).div_floor(&D::from(2u8)));
        assert_eq!(D::one(), D::from(257u16).div_floor(&D::from(256u16)));

        assert!(D::one().divides(&D::one()));
        assert!(!D::one().divides(&D::from(257u16)));
        assert!(D::from(257u16).divides(&D::one()));
        assert!(!D::from(257u16).divides(&D::from(256u16)));

        assert!(D::one().is_odd());
        assert!(D::from(256u16).is_even());

        assert_eq!((D::one(), D::zero()), D::one().div_rem(&D::one()));
        assert_eq!((D::zero(), D::one()), D::one().div_rem(&D::from(256u16)));
        assert_eq!(
            (D::from(256u16), D::zero()),
            D::from(256u16).div_rem(&D::one())
        );
        assert_eq!(
            (D::one(), D::one()),
            D::from(257u16).div_rem(&D::from(256u16))
        );
    }
}
