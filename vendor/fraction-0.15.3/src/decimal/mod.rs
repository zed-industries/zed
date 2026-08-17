use convert::TryToConvertFrom;
use error;

use num::integer::Integer;
use num::traits::{
    Bounded, CheckedAdd, CheckedMul, CheckedSub, FromPrimitive, Num, One, Signed, ToPrimitive, Zero,
};

use std::cmp::{self, Eq, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::{Product, Sum};
use std::num::FpCategory;
use std::ops::{Add, DivAssign, Mul, MulAssign, Neg, SubAssign};
use std::str::FromStr;

use super::{GenericFraction, Sign};
use division;
use fraction::display;
use generic::GenericInteger;

#[cfg(feature = "with-bigint")]
use super::{BigInt, BigUint};

#[cfg(feature = "with-postgres-support")]
mod postgres_support;

#[cfg(feature = "with-juniper-support")]
mod juniper_support;

#[cfg(feature = "with-approx")]
mod approx;

mod ops;
mod try_from;

/// Decimal type implementation
///
/// T is the type for data
/// P is the type for precision
///
/// Uses [GenericFraction]<T> internally to represent the data.
/// Precision is being used for representation purposes only.
/// Calculations do not use precision, but comparison does.
///
/// # Examples
///
/// ```
/// use fraction::GenericDecimal;
///
/// type Decimal = GenericDecimal<u64, u8>;
///
/// let d1 = Decimal::from(12);
/// let d2 = Decimal::from(0.5);
///
/// let mul = d1 * d2;
/// let div = d1 / d2;
/// let add = d1 + d2;
///
/// assert_eq!(mul, 6.into());
/// assert_eq!(div, Decimal::from("24.00"));
/// assert_eq!(add, Decimal::from(12.5));
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "with-serde-support", derive(Serialize, Deserialize))]
pub struct GenericDecimal<T, P>(pub(crate) GenericFraction<T>, pub(crate) P)
where
    T: Clone + Integer,
    P: Copy + Integer + Into<usize>;

impl<T, P> Copy for GenericDecimal<T, P>
where
    T: Copy + Integer,
    P: Copy + Integer + Into<usize>,
{
}

impl<T, P> Default for GenericDecimal<T, P>
where
    T: Clone + Integer,
    P: Copy + Integer + Into<usize>,
{
    fn default() -> Self {
        Self(GenericFraction::default(), P::zero())
    }
}

impl<T, P> fmt::Display for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + Integer + Into<usize>,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenericDecimal(ref fraction, precision) => {
                let format = display::Format::new(formatter).set_precision(Some(
                    formatter.precision().unwrap_or_else(|| precision.into()),
                ));
                display::format_fraction(fraction, formatter, &format)
            }
        }
    }
}

impl<T, P> fmt::Debug for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + From<u8> + ToPrimitive + fmt::Debug,
    P: Copy + Integer + Into<usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenericDecimal(ref fraction, precision) => {
                let prec = precision.into();
                let debug_prec = f.precision().unwrap_or(32);
                let value = format!("{:.1$}", fraction, prec);
                let debug_value = format!("{:.1$}", fraction, debug_prec);

                write!(
                    f,
                    "GenericDecimal({} | prec={}; {:?}; {})",
                    value, prec, fraction, debug_value
                )
            }
        }
    }
}

impl<T, P> FromStr for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + CheckedAdd + CheckedMul + CheckedSub,
    P: Copy + GenericInteger + Into<usize> + From<u8> + CheckedAdd,
{
    type Err = error::ParseError;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        if val == "NaN" {
            Ok(Self::nan())
        } else if val == "-inf" {
            Ok(Self::neg_infinity())
        } else if val == "+inf" || val == "inf" {
            Ok(Self::infinity())
        } else {
            // Check if the number is float like (1.0, 123.456, etc).
            if let Some(split_idx) = val.find('.') {
                let mut prec_iter = val.len() - split_idx - 1;
                let mut precision: P = P::zero();

                loop {
                    if prec_iter == 0 {
                        break;
                    }
                    prec_iter -= 1;

                    if let Some(p) = precision.checked_add(&P::one()) {
                        precision = p;
                    } else {
                        break;
                    }
                }

                Ok(GenericDecimal::from_str_radix(val, 10)?.set_precision(precision))
            // Check if the number is fraction like (1/1, 123/456, etc).
            } else if val.find('/').is_some() {
                Ok(GenericDecimal(GenericFraction::from_str(val)?, 16u8.into()))
            // Check if the number is int like (1, 123, etc).
            } else {
                Ok(GenericDecimal::from_str_radix(val, 10)?.set_precision(P::zero()))
            }
        }
    }
}

macro_rules! dec_impl {
    (impl_trait_math_unary; $trait:ident, $fn:ident) => {
        impl<T, P> $trait for GenericDecimal<T, P>
        where
            T: Clone + GenericInteger,
            P: Copy + Integer + Into<usize>
        {
            type Output = Self;

            fn $fn(self) -> Self::Output {
                match self {
                    GenericDecimal(sf, sp) => GenericDecimal($trait::$fn(sf), sp)
                }
            }
        }


        impl<'a, T, P> $trait for &'a GenericDecimal<T, P>
        where
            T: Clone + GenericInteger,
            P: Copy + Integer + Into<usize>,
            &'a T: $trait<Output=T>
        {
            type Output = GenericDecimal<T, P>;

            fn $fn(self) -> Self::Output {
                match self {
                    GenericDecimal(sf, sp) => GenericDecimal($trait::$fn(sf), *sp)
                }
            }
        }
    };

    (impl_trait_cmp; $trait:ident; $fn:ident; $return:ty) => {
        impl<T, P> $trait for GenericDecimal<T, P>
        where
            T: Clone + GenericInteger + $trait,
            P: Copy + GenericInteger + Into<usize>
        {
            fn $fn(&self, other: &Self) -> $return {
                match self {
                    GenericDecimal(sf, _) => match other {
                        GenericDecimal(of, _) => $trait::$fn(sf, of)
                    }
                }
            }
        }
    };


    (impl_trait_proxy; $trait:ident; $(($fn:ident ; $self:tt ; ; $return:ty)),*) => {
        impl<T, P> $trait for GenericDecimal<T, P>
        where
            T: Clone + GenericInteger + $trait,
            P: Copy + GenericInteger + Into<usize>
        {$(
            dec_impl!(_impl_trait_proxy_fn; $trait; $self; $fn; ; $return);
        )*}
    };

    (_impl_trait_proxy_fn; $trait:ident; rself; $fn:ident ; ; $return:ty) => {
        fn $fn(&self) -> $return {
            match self {
                GenericDecimal(f, _) => {
                    <GenericFraction<T> as $trait>::$fn(f)
                }
            }
        }
    };

    (impl_trait_from_int; $($t:ty),*) => {$(
        impl<T, P> From<$t> for GenericDecimal<T, P>
        where
            T: Clone + GenericInteger,
            P: Copy + GenericInteger + Into<usize>
        {
            fn from(value: $t) -> Self {
                GenericDecimal(GenericFraction::from(value), P::zero())
            }
        }
    )*};

    (impl_trait_from_float; $($t:ty),*) => {$(
        impl<T, P> From<$t> for GenericDecimal<T, P>
        where
            T: Clone + GenericInteger + FromPrimitive,
            P: Copy + GenericInteger + Into<usize> + From<u8> + Bounded
        {
            fn from(value: $t) -> Self {
                if value.is_nan () { return GenericDecimal::nan() };
                if value.is_infinite () { return if value.is_sign_negative () { GenericDecimal::neg_infinity() } else { GenericDecimal::infinity() } };

                GenericDecimal(GenericFraction::from(value), P::zero()).calc_precision(None)
            }
        }
    )*}
}

dec_impl!(impl_trait_from_float; f32, f64);
dec_impl!(impl_trait_from_int; u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

impl<'a, T, P> From<&'a str> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize> + From<u8>,
{
    fn from(value: &'a str) -> Self {
        GenericDecimal::from_str(value).unwrap_or_else(|_| GenericDecimal::nan())
    }
}

#[cfg(feature = "with-bigint")]
dec_impl!(impl_trait_from_int; BigUint, BigInt);

dec_impl!(impl_trait_math_unary; Neg, neg);

dec_impl!(impl_trait_proxy;
    ToPrimitive;
        (to_i64; rself;; Option<i64>),
        (to_u64; rself;; Option<u64>),
        (to_f64; rself;; Option<f64>)
);

impl<T, P> Sum for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + PartialEq,
    P: Copy + GenericInteger + Into<usize>,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(GenericDecimal::<T, P>::zero(), Add::add)
    }
}
impl<'a, T, P> Sum<&'a GenericDecimal<T, P>> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + PartialEq,
    P: Copy + GenericInteger + Into<usize>,
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut sum = Self::zero();

        for x in iter {
            sum += x;
        }

        sum
    }
}

impl<T, P> Product for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + PartialEq,
    P: Copy + GenericInteger + Into<usize>,
{
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(GenericDecimal::<T, P>::one(), Mul::mul)
    }
}
impl<'a, T, P> Product<&'a GenericDecimal<T, P>> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + PartialEq,
    P: Copy + GenericInteger + Into<usize>,
{
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut sum = Self::one();

        for x in iter {
            sum *= x;
        }

        sum
    }
}

dec_impl!(impl_trait_cmp; Ord; cmp; Ordering);

impl<T, P> PartialOrd for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + PartialOrd,
    P: Copy + GenericInteger + Into<usize>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, P> PartialEq for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + PartialEq,
    P: Copy + GenericInteger + Into<usize>,
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => {
                    if sf.sign() != of.sign() {
                        return false;
                    }

                    if sf.is_zero() {
                        return of.is_zero();
                    }

                    if of.is_zero() {
                        return false;
                    }

                    if !sf.is_normal() || !of.is_normal() {
                        return PartialEq::eq(sf, of);
                    }

                    if sp == op && PartialEq::eq(sf, of) {
                        return true;
                    }

                    // if either precision or fractions are not equal,
                    // then we potentially have different numbers represented so
                    // we need to figure out their numeric representation and compare it

                    if let GenericFraction::Rational(_, sr) = sf {
                        if let GenericFraction::Rational(_, or) = of {
                            let (si, srem) = sr.numer().div_rem(sr.denom());
                            let (oi, orem) = or.numer().div_rem(or.denom());

                            if si != oi {
                                return false;
                            }

                            let mut s_state =
                                Some(division::DivisionState::new(srem, sr.denom().clone()));
                            let mut o_state =
                                Some(division::DivisionState::new(orem, or.denom().clone()));

                            let mut s_digit: u8 = 0;
                            let mut o_digit: u8 = 0;

                            let mut precision: usize = 0;
                            loop {
                                s_state = if let Ok(s) =
                                    division::divide_rem_resume(s_state.take().unwrap(), |s, d| {
                                        s_digit = d;
                                        Ok(Err(s))
                                    }) {
                                    if s.remainder.is_zero() {
                                        None
                                    } else {
                                        Some(s)
                                    }
                                } else {
                                    return false;
                                };

                                o_state = if let Ok(s) =
                                    division::divide_rem_resume(o_state.take().unwrap(), |s, d| {
                                        o_digit = d;
                                        Ok(Err(s))
                                    }) {
                                    if s.remainder.is_zero() {
                                        None
                                    } else {
                                        Some(s)
                                    }
                                } else {
                                    return false;
                                };

                                if s_digit != o_digit {
                                    return false;
                                }

                                precision += 1;

                                if precision == (*sp).into() {
                                    if sp == op {
                                        return true;
                                    }

                                    if op > sp {
                                        for _ in 0..(*op - *sp).into() {
                                            if let Some(state) = o_state.take() {
                                                let div_result = division::divide_rem_resume(
                                                    state,
                                                    |state, digit| {
                                                        o_digit = digit;
                                                        Ok(Err(state))
                                                    },
                                                );
                                                o_state = match div_result {
                                                    Ok(s) => {
                                                        if s.remainder.is_zero() {
                                                            None
                                                        } else {
                                                            Some(s)
                                                        }
                                                    }
                                                    Err(_) => return false,
                                                };

                                                if o_digit != 0 {
                                                    return false;
                                                }
                                            } else {
                                                return true;
                                            }
                                        }
                                        return true;
                                    } else {
                                        return o_state.is_none();
                                    }
                                }

                                if precision == (*op).into() {
                                    if sp > op {
                                        for _ in 0..(*sp - *op).into() {
                                            if let Some(state) = s_state.take() {
                                                let div_result = division::divide_rem_resume(
                                                    state,
                                                    |state, digit| {
                                                        s_digit = digit;
                                                        Ok(Err(state))
                                                    },
                                                );
                                                s_state = match div_result {
                                                    Ok(s) => {
                                                        if s.remainder.is_zero() {
                                                            None
                                                        } else {
                                                            Some(s)
                                                        }
                                                    }
                                                    Err(_) => return false,
                                                };

                                                if s_digit != 0 {
                                                    return false;
                                                }
                                            } else {
                                                return true;
                                            }
                                        }
                                        return true;
                                    } else {
                                        return s_state.is_none();
                                    }
                                }

                                if s_state.is_none() {
                                    return o_state.is_none();
                                }

                                if o_state.is_none() {
                                    return s_state.is_none();
                                }
                            }
                        } else {
                            unreachable!()
                        }
                    } else {
                        unreachable!()
                    }
                }
            },
        }
    }
}

impl<T, P> Hash for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + PartialEq,
    P: Copy + GenericInteger + Into<usize>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            GenericDecimal(fraction, precision) => match fraction {
                GenericFraction::NaN => state.write_u8(0u8),
                GenericFraction::Infinity(sign) => {
                    if let Sign::Plus = sign {
                        state.write_u8(1u8)
                    } else {
                        state.write_u8(2u8)
                    }
                }
                GenericFraction::Rational(sign, ratio) => {
                    if let Sign::Plus = sign {
                        state.write_u8(3u8)
                    } else {
                        state.write_u8(4u8)
                    }

                    let num = ratio.numer();
                    let den = ratio.denom();

                    let div_state =
                        division::divide_integral(num.clone(), den.clone(), |digit: u8| {
                            state.write_u8(digit);
                            Ok(true)
                        })
                        .ok();

                    if !precision.is_zero() {
                        let mut dot = false;
                        let mut trailing_zeroes: usize = 0;

                        if let Some(div_state) = div_state {
                            if !div_state.remainder.is_zero() {
                                let mut precision = *precision;
                                division::divide_rem(
                                    div_state.remainder,
                                    div_state.divisor,
                                    |s, digit: u8| {
                                        precision -= P::one();

                                        if digit == 0 {
                                            trailing_zeroes += 1;
                                        } else {
                                            if !dot {
                                                dot = true;
                                                state.write_u8(10u8);
                                            }

                                            if trailing_zeroes > 0 {
                                                trailing_zeroes = 0;
                                                state.write_usize(trailing_zeroes);
                                            }

                                            state.write_u8(digit);
                                        }

                                        Ok(if precision.is_zero() { Err(s) } else { Ok(s) })
                                    },
                                )
                                .ok();
                            }
                        }
                    }
                }
            },
        };
    }
}

impl<T, P> Eq for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + Eq,
    P: Copy + GenericInteger + Into<usize>,
{
}

impl<T, P> Bounded for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + Bounded,
    P: Copy + GenericInteger + Into<usize> + Bounded,
{
    fn min_value() -> Self {
        GenericDecimal(GenericFraction::min_value(), P::max_value())
    }

    fn max_value() -> Self {
        GenericDecimal(GenericFraction::max_value(), P::max_value())
    }
}

impl<T, P> Zero for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize> + Zero,
{
    fn zero() -> Self {
        GenericDecimal(GenericFraction::zero(), P::zero())
    }

    fn is_zero(&self) -> bool {
        match self {
            GenericDecimal(fra, _) => fra.is_zero(),
        }
    }
}

impl<T, P> One for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
{
    fn one() -> Self {
        GenericDecimal(GenericFraction::one(), P::zero())
    }
}

impl<T, P> Num for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize> + From<u8>,
{
    type FromStrRadixErr = error::ParseError;

    fn from_str_radix(value: &str, base: u32) -> Result<Self, error::ParseError> {
        if base != 10 {
            return Err(error::ParseError::UnsupportedBase);
        }

        Ok(GenericDecimal(
            GenericFraction::from_str(value)?,
            16u8.into(),
        ))
    }
}

impl<T, P> Signed for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + Neg,
    P: Copy + GenericInteger + Into<usize> + From<u8>,
{
    fn abs(&self) -> Self {
        match self {
            GenericDecimal(fra, pres) => GenericDecimal(fra.abs(), *pres),
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(sf.abs_sub(of), cmp::max(*sp, *op)),
            },
        }
    }

    fn signum(&self) -> Self {
        match self {
            GenericDecimal(fra, pres) => GenericDecimal(fra.signum(), *pres),
        }
    }

    fn is_positive(&self) -> bool {
        match self {
            GenericDecimal(f, _) => f.is_positive(),
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            GenericDecimal(f, _) => f.is_negative(),
        }
    }
}

impl<T, P> GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
{
    /// Returns Some(Sign) of the decimal, or None if NaN is the value
    pub const fn sign(&self) -> Option<Sign>
    where
        T: CheckedAdd + CheckedMul + CheckedSub,
    {
        self.0.sign()
    }

    /// Sets representational precision for the Decimal
    /// The precision is only used for comparison and representation, not for calculations.
    ///
    /// This is suggested method for you to utilise if you know what precision you want to
    /// work with.
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericDecimal;
    ///
    /// type D = GenericDecimal<u32, u8>;
    ///
    /// let first = D::from("0.004")  // initial precision is 4
    ///             .set_precision(2);  // but we want to work with 2
    /// let second = D::from("0.006").set_precision(2);
    ///
    /// // Even though "first" and "second" both have precision 2
    /// // the actual calculations are still performed with their
    /// // exact initial precision
    /// assert_eq!(first + second, D::from("0.01"));
    ///
    /// // The comparison, on the other hand, takes the precision into account
    /// // so the actual compared numbers will be calculated up the highest
    /// // precision of both operands
    /// assert_ne!(  // compares "0.010" with "0.011"
    ///     D::from("0.01"),  // has precision 2
    ///     D::from("0.011")  // has precision 3
    /// );
    ///
    /// assert_eq!(  // compares "0.01" with "0.01"
    ///     D::from("0.01").set_precision(2),
    ///     D::from("0.011").set_precision(2)
    /// );
    /// ```
    pub fn set_precision(self, precision: P) -> Self {
        match self {
            GenericDecimal(fraction, _) => GenericDecimal(fraction, precision),
        }
    }

    /// Returns the current representational precision for the Decimal
    pub const fn get_precision(&self) -> P {
        match self {
            GenericDecimal(_, precision) => *precision,
        }
    }

    /// Try to recalculate the representational precision
    /// depending on the internal Fraction, which is the actual value.
    ///
    /// Performs the actual division until the exact decimal value is calculated,
    /// the precision type (P) capacity is reached (e.g. 255 for u8) or max_precision
    /// is reached, if it is given.
    ///
    /// # WARNING
    /// You only need this method if you want to find the max available
    /// precision for the current decimal value.
    /// However, keep in mind that irrational values (such as 1/3) do not have finite precision,
    /// so if this method returns P::MAX (or max_precision), most likely you have
    /// an irrational value.
    /// Be careful with max numbers for `usize` - that can take very long time to
    /// compute (more than a minute)
    pub fn calc_precision(self, max_precision: Option<P>) -> Self
    where
        T: CheckedMul + DivAssign + MulAssign + SubAssign + ToPrimitive + GenericInteger,
        P: Bounded + CheckedAdd,
    {
        match self {
            GenericDecimal(fraction, _) => {
                let precision = match fraction {
                    GenericFraction::NaN => P::zero(),
                    GenericFraction::Infinity(_) => P::zero(),
                    GenericFraction::Rational(_, ref ratio) => {
                        let mut precision: P = P::zero();
                        let max_precision: P = max_precision.unwrap_or_else(P::max_value);

                        let num = ratio.numer();
                        let den = ratio.denom();

                        if let Ok(div_state) =
                            division::divide_integral(num.clone(), den.clone(), |_| Ok(true))
                        {
                            if !div_state.remainder.is_zero() {
                                let one = P::one();

                                let _result = division::divide_rem(
                                    div_state.remainder,
                                    div_state.divisor,
                                    |s, _| {
                                        if precision >= max_precision {
                                            // stop here, we have reached the limit
                                            return Ok(Err(s));
                                        }

                                        precision = if let Some(p) = precision.checked_add(&one) {
                                            p
                                        } else {
                                            return Ok(Err(s));
                                        };
                                        Ok(Ok(s))
                                    },
                                );
                            }
                        }

                        precision
                    }
                };

                GenericDecimal(fraction, precision)
            }
        }
    }

    #[inline]
    pub fn nan() -> Self {
        GenericDecimal(GenericFraction::nan(), P::zero())
    }

    #[inline]
    pub fn infinity() -> Self {
        GenericDecimal(GenericFraction::infinity(), P::zero())
    }

    #[inline]
    pub fn neg_infinity() -> Self {
        GenericDecimal(GenericFraction::neg_infinity(), P::zero())
    }

    #[inline]
    pub fn neg_zero() -> Self {
        GenericDecimal(GenericFraction::neg_zero(), P::zero())
    }

    pub fn min_positive_value() -> Self
    where
        T: Bounded,
        P: Bounded,
    {
        GenericDecimal(GenericFraction::min_positive_value(), P::max_value())
    }

    pub const fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    pub const fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }

    pub const fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    pub fn is_normal(&self) -> bool {
        self.0.is_normal()
    }

    pub fn classify(&self) -> FpCategory {
        self.0.classify()
    }

    pub fn floor(&self) -> Self {
        match self {
            GenericDecimal(f, _) => GenericDecimal(f.floor(), P::zero()),
        }
    }

    pub fn ceil(&self) -> Self {
        match self {
            GenericDecimal(f, _) => GenericDecimal(f.ceil(), P::zero()),
        }
    }

    pub fn round(&self) -> Self {
        match self {
            GenericDecimal(f, _) => GenericDecimal(f.round(), P::zero()),
        }
    }

    pub fn trunc(&self) -> Self {
        match self {
            GenericDecimal(f, _) => GenericDecimal(f.trunc(), P::zero()),
        }
    }

    pub fn fract(&self) -> Self {
        self.map_ref(|f| f.fract())
    }

    pub fn abs(&self) -> Self {
        self.map_ref(|f| f.abs())
    }

    pub fn signum(&self) -> Self {
        self.map_ref(|f| f.signum())
    }

    pub const fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }

    pub const fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    pub fn mul_add(&self, a: Self, b: Self) -> Self {
        self.clone() * a + b
    }

    pub fn recip(&self) -> Self {
        self.map_ref(|f| f.recip())
    }

    pub fn map(self, fun: impl FnOnce(GenericFraction<T>) -> GenericFraction<T>) -> Self {
        match self {
            GenericDecimal(fra, pres) => GenericDecimal(fun(fra), pres),
        }
    }

    pub fn map_mut(&mut self, fun: impl FnOnce(&mut GenericFraction<T>)) {
        match self {
            GenericDecimal(fra, _) => fun(fra),
        }
    }

    pub fn map_ref(&self, fun: impl FnOnce(&GenericFraction<T>) -> GenericFraction<T>) -> Self {
        match self {
            GenericDecimal(fra, pres) => GenericDecimal(fun(fra), *pres),
        }
    }

    #[deprecated(note = "Use `match decimal {GenericDecimal(fraction, precision) => ... }`")]
    pub fn apply_ref<R>(&self, fun: impl FnOnce(&GenericFraction<T>, P) -> R) -> R {
        match self {
            GenericDecimal(fra, pres) => fun(fra, *pres),
        }
    }

    /// Convert from a GenericFraction
    ///
    /// Automatically calculates precision, so for "bad" numbers
    /// may take a lot of CPU cycles, especially if precision
    /// represented by big types (e.g. usize)
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::{Fraction, Decimal};
    ///
    /// let from_fraction = Decimal::from_fraction(Fraction::new(1u64, 3u64));
    /// let from_division = Decimal::from(1) / Decimal::from(3);
    ///
    /// let d1 = Decimal::from(4) / from_fraction;
    /// let d2 = Decimal::from(4) / from_division;
    ///
    /// assert_eq!(d1, d2);
    /// assert_eq!(d1, Decimal::from(12));
    /// ```
    #[inline]
    pub fn from_fraction(fraction: GenericFraction<T>) -> Self
    where
        T: GenericInteger + ToPrimitive,
        P: Bounded + CheckedAdd,
    {
        let two = P::one() + P::one();
        let hun = P::_10() * P::_10();
        let max_precision = two * hun + hun / two + P::_10() / two; // 255

        GenericDecimal(fraction, P::zero()).calc_precision(Some(max_precision))
    }
}

impl<T, F, P1, P2> TryToConvertFrom<GenericDecimal<F, P1>> for GenericDecimal<T, P2>
where
    T: Copy + Integer + TryToConvertFrom<F>,
    F: Copy + Integer,
    P2: Copy + Integer + Into<usize> + TryToConvertFrom<P1>,
    P1: Copy + Integer + Into<usize>,
{
    fn try_to_convert_from(src: GenericDecimal<F, P1>) -> Option<Self> {
        Some(match src {
            GenericDecimal(fraction, precision) => GenericDecimal(
                GenericFraction::try_to_convert_from(fraction)?,
                P2::try_to_convert_from(precision)?,
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use {CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

    use super::{GenericDecimal, One};
    use fraction::GenericFraction;
    use prelude::Decimal;
    use std::hash::{Hash, Hasher};

    type D = GenericDecimal<u8, u8>;

    fn hash_it(target: &impl Hash) -> u64 {
        use std::collections::hash_map::DefaultHasher;

        let mut h = DefaultHasher::new();
        target.hash(&mut h);
        h.finish()
    }

    generate_ops_tests! (
        NaN => {D::nan()};
        NegInf => {D::neg_infinity()};
        PosInf => {D::infinity()};
        Zero => {D::from(0)};
        Half => {D::from(0.5)};
        One => {D::from(1)};
        Two => {D::from(2)};
        Three => {D::from(3)};
        Four => {D::from(4)};
    );

    #[test]
    fn hash_and_partial_eq() {
        {
            let one = Decimal::from(152.568);
            let two = Decimal::from(328.76842);

            let div = two / one.set_precision(16);
            let red = Decimal::from("2.1548976194221592");

            assert_eq!(div.get_precision(), 16);
            assert_eq!(div, red);
            assert_eq!(hash_it(&div), hash_it(&red));
        }

        {
            let one = Decimal::from(152.568);
            let two = Decimal::from(328.76842);

            let mul = one * two;

            assert_eq!(mul.get_precision(), 5);
            assert_eq!(mul, Decimal::from("50159.5403"));
            assert_eq!(hash_it(&mul), hash_it(&Decimal::from("50159.5403")));
            assert_eq!(mul.set_precision(6), Decimal::from("50159.540302"));
            assert_eq!(
                hash_it(&mul.set_precision(6)),
                hash_it(&Decimal::from("50159.540302"))
            );
        }
    }

    #[test]
    fn fmt_debug() {
        type F = GenericFraction<u64>;
        assert_eq!(
            format!("{:?}", Decimal::one()),
            format!("GenericDecimal(1 | prec=0; {:?}; 1)", F::one())
        );
    }

    #[test]
    fn summing_iterator() {
        let values = vec![Decimal::from(152.568), Decimal::from(328.76842)];
        let sum: Decimal = values.iter().sum();
        assert_eq!(sum, values[0] + values[1])
    }

    #[test]
    fn product_iterator() {
        let values = vec![Decimal::from(152.568), Decimal::from(328.76842)];
        let product: Decimal = values.iter().product();
        assert_eq!(product, values[0] * values[1])
    }

    #[test]
    fn calc_precision() {
        use super::BigUint;
        type BigDecimal = GenericDecimal<BigUint, usize>;

        let one = BigDecimal::from(1);
        let two = BigDecimal::from(2);
        let three = BigDecimal::from(3);
        let half = BigDecimal::from(1) / BigDecimal::from(2);
        let onethird = BigDecimal::from(1) / BigDecimal::from(3);

        assert_eq!(0, one.get_precision());
        assert_eq!(0, two.get_precision());
        assert_eq!(0, three.get_precision());
        assert_eq!(0, half.get_precision());
        assert_eq!(0, onethird.get_precision());
        assert_eq!(1, half.clone().calc_precision(None).get_precision());
        assert_eq!(0, half.clone().calc_precision(Some(0)).get_precision());
        assert_eq!(1, half.clone().calc_precision(Some(1)).get_precision());
        assert_eq!(1, half.clone().calc_precision(Some(255)).get_precision());

        assert_eq!(0, onethird.clone().calc_precision(Some(0)).get_precision());
        assert_eq!(1, onethird.clone().calc_precision(Some(1)).get_precision());
        assert_eq!(
            255,
            onethird.clone().calc_precision(Some(255)).get_precision()
        );
        assert_eq!(
            2056,
            onethird.clone().calc_precision(Some(2056)).get_precision()
        );

        type D = GenericDecimal<u64, u8>;
        let one = D::from(1);
        let two = D::from(2);
        let three = D::from(3);
        let half = one / two;
        let onethird = one / three;

        assert_eq!(0, one.get_precision());
        assert_eq!(0, two.get_precision());
        assert_eq!(0, three.get_precision());
        assert_eq!(0, half.get_precision());
        assert_eq!(0, onethird.get_precision());
        assert_eq!(1, half.calc_precision(None).get_precision());
        assert_eq!(255, onethird.calc_precision(None).get_precision());
    }

    #[test]
    fn decimal_test_default() {
        let dec = D::default();
        assert_eq!("0", format!("{}", dec));
        assert_eq!(0, dec.get_precision());

        #[cfg(feature = "with-bigint")]
        {
            use crate::BigDecimal;
            let dec = BigDecimal::default();
            assert_eq!("0", format!("{}", dec));
            assert_eq!(0, dec.get_precision());
        }
    }

    // TODO: more tests
}
