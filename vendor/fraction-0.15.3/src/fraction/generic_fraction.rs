use crate::fraction::Sign;
use crate::{
    display, Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, ConstOne, ConstZero,
    FromPrimitive, Integer, Num, One, ParseRatioError, Ratio, Signed, ToPrimitive, Zero,
};
#[cfg(feature = "with-bigint")]
use crate::{BigInt, BigUint};
use error::ParseError;
use std::iter::{Product, Sum};

use generic::{read_generic_integer, GenericInteger};

use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use std::hash::{Hash, Hasher};
use std::num::FpCategory;
use std::ops::{Add, Mul, Neg};

use std::f64;
use std::fmt;
use std::str::FromStr;

/// Generic implementation of the fraction type
///
/// # Examples
///
/// ```
/// use fraction::GenericFraction;
///
/// type F = GenericFraction<u8>;
///
/// let first = F::new (1u8, 2u8);
/// let second = F::new (2u8, 8u8);
///
/// assert_eq! (first + second, F::new (3u8, 4u8));
/// ```
///
///
/// Since GenericFraction keeps its sign explicitly and independently of the numerics,
/// it is not recommended to use signed types, although it's completely valid with the cost of target type capacity.
///
/// ```
/// use fraction::GenericFraction;
///
/// type F = GenericFraction<i8>;
///
/// let first = F::new (1, 2);
/// let second = F::new (2, 8);
///
/// assert_eq! (first + second, F::new (3, 4));
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "with-serde-support", derive(Serialize, Deserialize))]
pub enum GenericFraction<T>
where
    T: Clone + Integer,
{
    Rational(Sign, Ratio<T>),
    Infinity(Sign),
    NaN,
}

/// Copy semantics to be applied for the target type, but only if T also has it.
impl<T> Copy for GenericFraction<T> where T: Copy + Integer {}

impl<T> Default for GenericFraction<T>
where
    T: Clone + Integer,
{
    fn default() -> Self {
        Self::zero()
    }
}

impl<T> FromStr for GenericFraction<T>
where
    T: Clone + Integer + CheckedAdd + CheckedMul,
{
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let (sign, start) = if src.starts_with('-') {
            (Sign::Minus, 1)
        } else if src.starts_with('+') {
            (Sign::Plus, 1)
        } else {
            (Sign::Plus, 0)
        };

        if let Some(split_idx) = src.find('.') {
            let who = &src[start..split_idx];

            let mut num = match T::from_str_radix(who, 10) {
                Err(_) => return Err(ParseError::ParseIntError),
                Ok(value) => value,
            };

            // skip trailing zeros
            let len = src[split_idx + 1..].trim_end_matches('0').len();
            let fra = if len > 0 {
                let p = T::from_str_radix(&src[split_idx + 1..split_idx + 1 + len], 10);
                match p {
                    Err(_) => return Err(ParseError::ParseIntError),
                    Ok(value) => value,
                }
            } else {
                T::zero()
            };

            let mut den = T::one();

            if len > 0 {
                let mut t10 = T::one();
                for _ in 0..9 {
                    t10 = if let Some(t10) = t10.checked_add(&den) {
                        t10
                    } else {
                        return Err(ParseError::OverflowError);
                    };
                }

                for _ in 0..len {
                    num = if let Some(num) = num.checked_mul(&t10) {
                        num
                    } else {
                        return Err(ParseError::OverflowError);
                    };
                    den = if let Some(den) = den.checked_mul(&t10) {
                        den
                    } else {
                        return Err(ParseError::OverflowError);
                    };
                }
            }

            let num = if let Some(num) = num.checked_add(&fra) {
                num
            } else {
                return Err(ParseError::OverflowError);
            };

            Ok(GenericFraction::Rational(sign, Ratio::new(num, den)))
        } else if let Some(split_idx) = src.find('/') {
            let num = match T::from_str_radix(&src[start..split_idx], 10) {
                Ok(value) => value,
                Err(_) => return Err(ParseError::ParseIntError),
            };
            let den = match T::from_str_radix(&src[split_idx + 1..], 10) {
                Ok(value) => value,
                Err(_) => return Err(ParseError::ParseIntError),
            };

            Ok(GenericFraction::Rational(sign, Ratio::new(num, den)))
        } else {
            let num = match T::from_str_radix(&src[start..], 10) {
                Ok(value) => value,
                Err(_) => return Err(ParseError::ParseIntError),
            };

            Ok(GenericFraction::Rational(sign, Ratio::new(num, T::one())))
        }
    }
}

impl<T> GenericFraction<T>
where
    T: Clone + Integer,
{
    /// Constructs a new fraction with the specified numerator and denominator
    /// Handles gracefully signed integers even if the storage type is unsigned and vise versa
    /// The arguments can be of any integer types imlementing the necessary traits
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::{GenericFraction, Sign};
    /// type F = GenericFraction<u16>;
    ///
    /// let f12 = F::new_generic(Sign::Plus, 1i8, 2u8).unwrap();
    /// let f34 = F::new_generic(Sign::Plus, 3i16, 4u32).unwrap();
    /// let f56 = F::new_generic(Sign::Plus, 5i64, 6u128).unwrap();
    /// let f78 = F::new_generic(Sign::Plus, 7usize, 8isize).unwrap();
    ///
    /// assert_eq! ((*f12.numer().unwrap(), *f12.denom().unwrap()), (1u16, 2u16));
    /// assert_eq! ((*f34.numer().unwrap(), *f34.denom().unwrap()), (3u16, 4u16));
    /// assert_eq! ((*f56.numer().unwrap(), *f56.denom().unwrap()), (5u16, 6u16));
    /// assert_eq! ((*f78.numer().unwrap(), *f78.denom().unwrap()), (7u16, 8u16));
    /// ````
    pub fn new_generic<N, D>(sign: Sign, num: N, den: D) -> Option<GenericFraction<T>>
    where
        N: GenericInteger + PartialOrd,
        D: GenericInteger + PartialOrd,
        T: GenericInteger,
    {
        let (ns, num): (Sign, T) = read_generic_integer(num)?;
        let (ds, den): (Sign, T) = read_generic_integer(den)?;

        let sign = ns * ds * sign;

        Some(Self::_new(sign, num, den))
    }

    fn _new<N, D>(sign: Sign, num: N, den: D) -> GenericFraction<T>
    where
        N: Into<T>,
        D: Into<T>,
    {
        let num: T = num.into();
        let den: T = den.into();

        if den.is_zero() {
            if num.is_zero() {
                GenericFraction::NaN
            } else {
                GenericFraction::Infinity(sign)
            }
        } else {
            GenericFraction::Rational(sign, Ratio::new(num, den))
        }
    }

    /// Constructs a new fraction with the specified numerator and denominator
    ///
    /// The arguments must me either of `T` type, or implement `Into<T>` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u16>;
    ///
    /// let _f = F::new(1u8, 2u16);
    /// ```
    pub fn new<N, D>(num: N, den: D) -> GenericFraction<T>
    where
        N: Into<T>,
        D: Into<T>,
    {
        Self::_new(Sign::Plus, num, den)
    }

    /// Constructs a new negative fraction with the specified numerator and denominator
    ///
    /// The arguments must be either of `T` type, or implement `Into<T>` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u16>;
    ///
    /// let _f = F::new_neg (1u8, 2u16);
    /// ```
    pub fn new_neg<N, D>(num: N, den: D) -> GenericFraction<T>
    where
        N: Into<T>,
        D: Into<T>,
    {
        Self::_new(Sign::Minus, num, den)
    }

    /// Constructs a new fraction without types casting, checking for denom == 0 and reducing numbers.
    ///
    /// You must be careful with this function because all the other functionality parts rely on the
    /// numbers to be reduced. That said, in the normal case 2/4 has to be reduced to 1/2, but it will not
    /// happen with new_raw.
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// let _f = F::new_raw (1u8, 2u8);
    /// ```
    pub const fn new_raw(num: T, den: T) -> GenericFraction<T> {
        GenericFraction::Rational(Sign::Plus, Ratio::new_raw(num, den))
    }

    /// The same as [fn new_raw](enum.GenericFraction.html#method.new_raw), but allows explicitly set sign.
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::{GenericFraction, Sign};
    /// type F = GenericFraction<u8>;
    ///
    /// let _f = F::new_raw_signed(Sign::Minus, 1u8, 2u8);
    /// ```
    pub const fn new_raw_signed(sign: Sign, num: T, den: T) -> GenericFraction<T> {
        GenericFraction::Rational(sign, Ratio::new_raw(num, den))
    }

    /// Returns a reference to the numerator value
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// let fra = F::new (5u8, 6u8);
    /// assert_eq! (5, *fra.numer ().unwrap ());
    /// ```
    pub const fn numer(&self) -> Option<&T> {
        match *self {
            GenericFraction::Rational(_, ref r) => Some(r.numer()),
            _ => None,
        }
    }

    /// Returns a reference to the denominator value
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// let fra = F::new (5u8, 6u8);
    /// assert_eq! (6, *fra.denom ().unwrap ());
    /// ```
    pub const fn denom(&self) -> Option<&T> {
        match *self {
            GenericFraction::Rational(_, ref r) => Some(r.denom()),
            _ => None,
        }
    }

    /// Returns a reference to the sign value
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::{ GenericFraction, Sign };
    /// type F = GenericFraction<u8>;
    ///
    ///
    /// let fra = F::new (5u8, 6u8);
    /// assert_eq! (Sign::Plus, fra.sign ().unwrap ());
    ///
    /// let fra = F::new_neg (5u8, 6u8);
    /// assert_eq! (Sign::Minus, fra.sign ().unwrap ());
    ///
    ///
    /// let fra = F::infinity ();
    /// assert_eq! (Sign::Plus, fra.sign ().unwrap ());
    ///
    /// let fra = F::neg_infinity ();
    /// assert_eq! (Sign::Minus, fra.sign ().unwrap ());
    ///
    ///
    /// let fra = F::nan ();
    /// assert_eq! (None, fra.sign ());
    /// ```
    pub const fn sign(&self) -> Option<Sign> {
        match self {
            GenericFraction::Rational(s, _) => Some(*s),
            GenericFraction::Infinity(s) => Some(*s),
            _ => None,
        }
    }

    /// Generates a GenericFraction<T> from GenericFraction<F>
    /// where T: From<F>
    ///
    /// ```
    /// use fraction::{ Fraction, GenericFraction };
    /// type F8 = GenericFraction<u8>;
    ///
    /// let fra8 = F8::new (5u8, 6u8);
    /// assert_eq! (Fraction::new (5u64, 6u64), Fraction::from_fraction(fra8));
    /// ```
    pub fn from_fraction<F>(from: GenericFraction<F>) -> GenericFraction<T>
    where
        T: From<F>,
        F: Clone + Integer,
    {
        match from {
            GenericFraction::NaN => GenericFraction::NaN,
            GenericFraction::Infinity(sign) => GenericFraction::Infinity(sign),
            GenericFraction::Rational(sign, ratio) => {
                let (num, den): (F, F) = ratio.into();
                GenericFraction::Rational(sign, Ratio::new_raw(T::from(num), T::from(den)))
            }
        }
    }

    /// Generates a GenericFraction<I> from GenericFraction<T>
    /// where T: Into<I>
    ///
    /// ```
    /// use fraction::{ Fraction, GenericFraction };
    /// type F8 = GenericFraction<u8>;
    ///
    /// let fra8 = F8::new (5u8, 6u8);
    /// assert_eq! (Fraction::new (5u64, 6u64), fra8.into_fraction());
    /// ```
    pub fn into_fraction<I>(self) -> GenericFraction<I>
    where
        T: Into<I>,
        I: Clone + Integer,
    {
        match self {
            GenericFraction::NaN => GenericFraction::NaN,
            GenericFraction::Infinity(sign) => GenericFraction::Infinity(sign),
            GenericFraction::Rational(sign, ratio) => {
                let (num, den): (T, T) = ratio.into();
                GenericFraction::Rational(sign, Ratio::new_raw(num.into(), den.into()))
            }
        }
    }
}

impl<T: Bounded + Clone + Integer> Bounded for GenericFraction<T> {
    fn min_value() -> Self {
        let one = T::one();
        let max = T::max_value();

        GenericFraction::Rational(Sign::Minus, Ratio::new(max, one))
    }

    fn max_value() -> Self {
        let one = T::one();
        let max = T::max_value();

        GenericFraction::Rational(Sign::Plus, Ratio::new(max, one))
    }
}

impl<T: Clone + Integer> PartialEq for GenericFraction<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GenericFraction::NaN, GenericFraction::NaN) => true,
            (GenericFraction::Infinity(sign), GenericFraction::Infinity(osign)) => sign == osign,
            (
                GenericFraction::Rational(ref ls, ref l),
                GenericFraction::Rational(ref rs, ref r),
            ) => {
                if ls == rs {
                    l.eq(r)
                } else {
                    l.is_zero() && r.is_zero()
                }
            }
            _ => false,
        }
    }
}

impl<T: Clone + Integer + Hash> Hash for GenericFraction<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            GenericFraction::NaN => state.write_u8(0u8),
            GenericFraction::Infinity(sign) => {
                if let Sign::Plus = sign {
                    state.write_u8(1u8)
                } else {
                    state.write_u8(2u8)
                }
            }
            GenericFraction::Rational(sign, ratio) => {
                if *sign == Sign::Plus || ratio.is_zero() {
                    state.write_u8(3u8);
                } else {
                    state.write_u8(4u8);
                }

                ratio.hash(state);
            }
        }
    }
}

impl<T: Clone + Integer> Eq for GenericFraction<T> {}

impl<T: Clone + Integer> PartialOrd for GenericFraction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone + Integer> Ord for GenericFraction<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (GenericFraction::NaN, GenericFraction::NaN) => Ordering::Equal,
            (GenericFraction::NaN, _) => Ordering::Less,
            (_, GenericFraction::NaN) => Ordering::Greater,
            (GenericFraction::Infinity(sign), GenericFraction::Infinity(osign)) => sign.cmp(osign),
            (GenericFraction::Infinity(Sign::Plus), GenericFraction::Rational(_, _)) => {
                Ordering::Greater
            }
            (GenericFraction::Infinity(Sign::Minus), GenericFraction::Rational(_, _)) => {
                Ordering::Less
            }
            (GenericFraction::Rational(_, _), GenericFraction::Infinity(Sign::Plus)) => {
                Ordering::Less
            }
            (GenericFraction::Rational(_, _), GenericFraction::Infinity(Sign::Minus)) => {
                Ordering::Greater
            }
            (
                GenericFraction::Rational(ref ls, ref l),
                GenericFraction::Rational(ref rs, ref r),
            ) => {
                if ls == rs {
                    match *ls {
                        Sign::Plus => l.cmp(r),
                        Sign::Minus => r.cmp(l),
                    }
                } else if l.is_zero() && r.is_zero() {
                    Ordering::Equal
                } else if *ls == Sign::Minus {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    }
}

impl<T: Clone + Integer> Neg for GenericFraction<T> {
    type Output = GenericFraction<T>;

    fn neg(self) -> Self {
        match self {
            GenericFraction::NaN => self,
            GenericFraction::Infinity(sign) => GenericFraction::Infinity(-sign),
            GenericFraction::Rational(s, r) => {
                if r.is_zero() {
                    GenericFraction::Rational(Sign::Plus, r)
                } else {
                    GenericFraction::Rational(s.neg(), r)
                }
            }
        }
    }
}

impl<'a, T: Clone + Integer> Neg for &'a GenericFraction<T> {
    type Output = GenericFraction<T>;

    fn neg(self) -> Self::Output {
        match *self {
            GenericFraction::NaN => GenericFraction::nan(),
            GenericFraction::Infinity(sign) => GenericFraction::Infinity(-sign),
            GenericFraction::Rational(s, ref r) => {
                if r.is_zero() {
                    GenericFraction::Rational(Sign::Plus, r.clone())
                } else {
                    GenericFraction::Rational(-s, r.clone())
                }
            }
        }
    }
}

impl<T: Clone + Integer> Sum for GenericFraction<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(GenericFraction::<T>::zero(), Add::add)
    }
}

impl<'a, T: 'a + Clone + Integer> Sum<&'a GenericFraction<T>> for GenericFraction<T> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        #[allow(clippy::redundant_closure)]
        iter.fold(GenericFraction::<T>::zero(), |ref s, x| Add::add(s, x))
    }
}

impl<T: Clone + Integer> Product for GenericFraction<T> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(GenericFraction::<T>::one(), Mul::mul)
    }
}

impl<'a, T: 'a + Clone + Integer> Product<&'a GenericFraction<T>> for GenericFraction<T> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        #[allow(clippy::redundant_closure)]
        iter.fold(GenericFraction::<T>::one(), |ref s, x| Mul::mul(s, x))
    }
}

impl<T: Clone + Integer> Zero for GenericFraction<T> {
    fn zero() -> Self {
        GenericFraction::Rational(Sign::Plus, Ratio::zero())
    }

    fn is_zero(&self) -> bool {
        match *self {
            GenericFraction::Rational(_, ref r) => r.is_zero(),
            _ => false,
        }
    }
}

impl<T: ConstOne + ConstZero + Integer + Clone> ConstZero for GenericFraction<T> {
    const ZERO: GenericFraction<T> =
        GenericFraction::Rational(Sign::Plus, Ratio::new_raw(ConstZero::ZERO, ConstOne::ONE));
}

impl<T: Clone + Integer> One for GenericFraction<T> {
    fn one() -> Self {
        GenericFraction::Rational(Sign::Plus, Ratio::one())
    }
}

impl<T: ConstOne + Integer + Clone> ConstOne for GenericFraction<T> {
    const ONE: GenericFraction<T> =
        GenericFraction::Rational(Sign::Plus, Ratio::new_raw(ConstOne::ONE, ConstOne::ONE));
}

impl<T: Clone + Integer> Num for GenericFraction<T> {
    type FromStrRadixErr = ParseRatioError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if let Some(rem) = str.strip_prefix('-') {
            Ratio::from_str_radix(rem, radix)
                .map(|ratio| GenericFraction::Rational(Sign::Minus, ratio))
        } else if let Some(rem) = str.strip_prefix('+') {
            Ratio::from_str_radix(rem, radix)
                .map(|ratio| GenericFraction::Rational(Sign::Plus, ratio))
        } else {
            Ratio::from_str_radix(str, radix)
                .map(|ratio| GenericFraction::Rational(Sign::Plus, ratio))
        }
    }
}

impl<T: Clone + Integer> Signed for GenericFraction<T> {
    fn abs(&self) -> Self {
        GenericFraction::abs(self)
    }

    fn abs_sub(&self, other: &Self) -> Self {
        match *self {
            GenericFraction::NaN => GenericFraction::NaN,
            GenericFraction::Infinity(sign) => match *other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(osign) => {
                    if sign == Sign::Minus || osign == Sign::Plus {
                        GenericFraction::zero()
                    } else {
                        GenericFraction::Infinity(Sign::Plus)
                    }
                }
                GenericFraction::Rational(_, _) => {
                    if sign == Sign::Plus {
                        GenericFraction::Infinity(sign)
                    } else {
                        GenericFraction::zero()
                    }
                }
            },
            GenericFraction::Rational(sign, ref l) => match *other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(osign) => {
                    if osign == Sign::Plus {
                        GenericFraction::zero()
                    } else if sign == Sign::Minus {
                        GenericFraction::Infinity(Sign::Minus)
                    } else {
                        GenericFraction::Infinity(Sign::Plus)
                    }
                }
                GenericFraction::Rational(_, ref r) => {
                    if l < r {
                        GenericFraction::Rational(Sign::Plus, r - l)
                    } else {
                        GenericFraction::Rational(Sign::Plus, l - r)
                    }
                }
            },
        }
    }

    fn signum(&self) -> Self {
        GenericFraction::signum(self)
    }

    fn is_positive(&self) -> bool {
        GenericFraction::is_sign_positive(self)
    }

    fn is_negative(&self) -> bool {
        GenericFraction::is_sign_negative(self)
    }
}

impl<T: Clone + Integer + PartialEq + ToPrimitive> ToPrimitive for GenericFraction<T> {
    fn to_i64(&self) -> Option<i64> {
        match *self {
            GenericFraction::NaN => None,
            GenericFraction::Infinity(_) => None,
            #[allow(clippy::manual_filter)]
            GenericFraction::Rational(sign, ref r) if *r.denom() == T::one() => {
                if let Some(n) = r.numer().to_i64() {
                    if sign == Sign::Minus {
                        Some(-n)
                    } else {
                        Some(n)
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match *self {
            GenericFraction::NaN => None,
            GenericFraction::Infinity(_) => None,
            GenericFraction::Rational(sign, ref r) if *r.denom() == T::one() => {
                if sign == Sign::Minus {
                    None
                } else {
                    r.numer().to_u64()
                }
            }
            _ => None,
        }
    }

    fn to_f64(&self) -> Option<f64> {
        match *self {
            GenericFraction::NaN => Some(f64::NAN),
            GenericFraction::Infinity(sign) => Some(if sign == Sign::Minus {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            }),
            GenericFraction::Rational(sign, ref r) => r
                .numer()
                .to_f64()
                .and_then(|n| r.denom().to_f64().map(|d| n / d))
                .map(|x| if sign == Sign::Minus { -x } else { x }),
        }
    }
}

impl<T: Clone + Integer> GenericFraction<T> {
    /// Returns NaN value
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::nan (), F::new (0, 0));
    /// ```
    pub const fn nan() -> Self {
        GenericFraction::NaN
    }

    /// Returns positive Infinity value
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::infinity (), F::new (1, 0));
    /// ```
    pub const fn infinity() -> Self {
        GenericFraction::Infinity(Sign::Plus)
    }

    /// Returns negative Infinity value
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::neg_infinity (), F::new_neg (1, 0));
    /// ```
    pub const fn neg_infinity() -> Self {
        GenericFraction::Infinity(Sign::Minus)
    }

    /// Returns zero with negative sign
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::neg_zero (), F::new_neg (0, 1));
    /// ```
    pub fn neg_zero() -> Self {
        GenericFraction::Rational(Sign::Minus, Ratio::zero())
    }

    /// Returns minimal value greater than zero
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F8 = GenericFraction<u8>;
    /// type F16 = GenericFraction<u16>;
    ///
    /// assert_eq! (F8::min_positive_value (), F8::new (1u8, 255u8));
    /// assert_eq! (F16::min_positive_value (), F16::new (1u16, 65535u16));
    /// ```
    pub fn min_positive_value() -> Self
    where
        T: Bounded,
    {
        GenericFraction::Rational(Sign::Plus, Ratio::new(T::one(), T::max_value()))
    }

    /// Returns true if the value is NaN
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert! (F::nan ().is_nan ());
    /// assert! (F::new (0, 0).is_nan ());
    /// ```
    pub const fn is_nan(&self) -> bool {
        matches!(*self, GenericFraction::NaN)
    }

    /// Returns true if the value is Infinity (does not matter positive or negative)
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert! (F::infinity ().is_infinite ());
    /// assert! (F::new (1u8, 0).is_infinite ());
    /// assert! (F::new_neg (1u8, 0).is_infinite ());
    /// ```
    pub const fn is_infinite(&self) -> bool {
        matches!(*self, GenericFraction::Infinity(_))
    }

    /// Returns true if the value is not Infinity (does not matter positive or negative)
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert! (! F::infinity ().is_finite ());
    /// assert! (! F::new (1u8, 0).is_finite ());
    /// assert! (! F::new_neg (1u8, 0).is_finite ());
    /// ```
    pub const fn is_finite(&self) -> bool {
        !matches!(*self, GenericFraction::Infinity(_))
    }

    /// Returns true if the number is neither zero, Infinity or NaN
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert! (! F::nan ().is_normal ());
    /// assert! (! F::infinity ().is_normal ());
    /// assert! (! F::neg_infinity ().is_normal ());
    /// assert! (! F::new (0, 1u8).is_normal ());
    /// assert! (! F::neg_zero ().is_normal ());
    /// ```
    pub fn is_normal(&self) -> bool {
        match *self {
            GenericFraction::Rational(_, ref v) => !v.is_zero(),
            _ => false,
        }
    }

    /// Returns the floating point category of the number
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::FpCategory;
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::nan ().classify (), FpCategory::Nan);
    /// assert_eq! (F::infinity ().classify (), FpCategory::Infinite);
    /// assert_eq! (F::new (0, 1u8).classify (), FpCategory::Zero);
    /// assert_eq! (F::new (1u8, 1u8).classify (), FpCategory::Normal);
    /// ```
    pub fn classify(&self) -> FpCategory {
        match *self {
            GenericFraction::NaN => FpCategory::Nan,
            GenericFraction::Infinity(_) => FpCategory::Infinite,
            GenericFraction::Rational(_, ref v) if v.is_zero() => FpCategory::Zero,
            _ => FpCategory::Normal,
        }
    }

    /// Returns the largest integer less than or equal to the value
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::new (7u8, 5u8).floor (), F::new (5u8, 5u8));
    /// assert_eq! (F::new_neg (4u8, 3u8).floor (), F::new_neg (2u8, 1u8));
    /// ```
    pub fn floor(&self) -> Self {
        match *self {
            GenericFraction::Rational(Sign::Plus, ref r) => {
                GenericFraction::Rational(Sign::Plus, r.floor())
            }
            GenericFraction::Rational(Sign::Minus, ref r) => {
                // Ceil of the unsigned ratio results in a floor for the signed fraction
                GenericFraction::Rational(Sign::Minus, r.ceil())
            }
            _ => self.clone(),
        }
    }

    /// Returns the smallest integer greater than or equal to the value
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::new (7u8, 5u8).ceil (), F::new (10u8, 5u8));
    /// assert_eq! (F::new_neg (4u8, 3u8).ceil (), F::new_neg (1u8, 1u8));
    /// ```
    pub fn ceil(&self) -> Self {
        match *self {
            GenericFraction::Rational(Sign::Plus, ref r) => {
                GenericFraction::Rational(Sign::Plus, r.ceil())
            }
            GenericFraction::Rational(Sign::Minus, ref r) => {
                // Floor of the unsigned ratio results in a ceil for the signed fraction
                GenericFraction::Rational(Sign::Minus, r.floor())
            }
            _ => self.clone(),
        }
    }

    /// Returns the nearest integer to the value (.5 goes away from zero)
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::new (7u8, 5u8).round (), F::new (5u8, 5u8));
    /// assert_eq! (F::new (8u8, 5u8).round (), F::new (10u8, 5u8));
    /// assert_eq! (F::new (3u8, 2u8).round (), F::new (4u8, 2u8));
    /// assert_eq! (F::new (1u8, 2u8).round (), F::new (2u8, 2u8));
    /// assert_eq! (F::new_neg (3u8, 2u8).round (), F::new_neg (2u8, 1u8));
    /// ```
    pub fn round(&self) -> Self {
        match *self {
            GenericFraction::Rational(s, ref r) => GenericFraction::Rational(s, r.round()),
            _ => self.clone(),
        }
    }

    /// Returns the integer part of the value
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::new (7u8, 5u8).trunc (), F::new (5u8, 5u8));
    /// assert_eq! (F::new (8u8, 5u8).trunc (), F::new (5u8, 5u8));
    /// ```
    pub fn trunc(&self) -> Self {
        match *self {
            GenericFraction::Rational(s, ref r) => GenericFraction::Rational(s, r.trunc()),
            _ => self.clone(),
        }
    }

    /// Returns the fractional part of a number
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::new (7u8, 5u8).fract (), F::new (2u8, 5u8));
    /// assert_eq! (F::new (8u8, 5u8).fract (), F::new (3u8, 5u8));
    /// ```
    pub fn fract(&self) -> Self {
        match *self {
            GenericFraction::Rational(s, ref r) => GenericFraction::Rational(s, r.fract()),
            _ => GenericFraction::NaN,
        }
    }

    /// Returns the absolute value of self
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::nan ().abs (), F::nan ());
    /// assert_eq! (F::infinity ().abs (), F::infinity ());
    /// assert_eq! (F::neg_infinity ().abs (), F::infinity ());
    /// assert_eq! (F::new (1u8, 2u8).abs (), F::new (1u8, 2u8));
    /// assert_eq! (F::new_neg (1u8, 2u8).abs (), F::new (1u8, 2u8));
    /// ```
    pub fn abs(&self) -> Self {
        match *self {
            GenericFraction::NaN => GenericFraction::NaN,
            GenericFraction::Infinity(_) => GenericFraction::Infinity(Sign::Plus),
            GenericFraction::Rational(_, ref r) => GenericFraction::Rational(Sign::Plus, r.clone()),
        }
    }

    /// Returns a number that represents the sign of self
    ///
    ///  * 1.0 if the number is positive, +0.0 or INFINITY
    ///  * -1.0 if the number is negative, -0.0 or NEG_INFINITY
    ///  * NAN if the number is NAN
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::new (1u8, 2u8).signum (), F::new (1u8, 1u8));
    /// assert_eq! (F::new (0u8, 1u8).signum (), F::new (1u8, 1u8));
    /// assert_eq! (F::infinity ().signum (), F::new (1u8, 1u8));
    /// assert_eq! (F::new_neg (1u8, 2u8).signum (), F::new_neg (1u8, 1u8));
    /// assert_eq! (F::neg_zero ().signum (), F::new_neg (1u8, 1u8));
    /// assert_eq! (F::neg_infinity ().signum (), F::new_neg (1u8, 1u8));
    /// assert_eq! (F::nan ().signum (), F::nan ());
    /// ```
    pub fn signum(&self) -> Self {
        match *self {
            GenericFraction::NaN => GenericFraction::NaN,

            GenericFraction::Infinity(Sign::Plus) => {
                GenericFraction::Rational(Sign::Plus, Ratio::new(T::one(), T::one()))
            }
            GenericFraction::Infinity(Sign::Minus) => {
                GenericFraction::Rational(Sign::Minus, Ratio::new(T::one(), T::one()))
            }

            GenericFraction::Rational(Sign::Plus, _) => {
                GenericFraction::Rational(Sign::Plus, Ratio::new(T::one(), T::one()))
            }
            GenericFraction::Rational(Sign::Minus, _) => {
                GenericFraction::Rational(Sign::Minus, Ratio::new(T::one(), T::one()))
            }
        }
    }

    /// Returns true if the sign is positive
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert! (F::new (1u8, 2u8).is_sign_positive ());
    /// assert! (F::infinity ().is_sign_positive ());
    /// assert! (! F::nan ().is_sign_positive ());
    /// ```
    pub const fn is_sign_positive(&self) -> bool {
        match *self {
            GenericFraction::NaN => false,
            GenericFraction::Infinity(Sign::Plus) => true,
            GenericFraction::Infinity(Sign::Minus) => false,
            GenericFraction::Rational(Sign::Plus, _) => true,
            GenericFraction::Rational(Sign::Minus, _) => false,
        }
    }

    /// Returns true if the sign is negative
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert! (F::new_neg (1u8, 2u8).is_sign_negative ());
    /// assert! (F::neg_zero ().is_sign_negative ());
    /// assert! (F::neg_infinity ().is_sign_negative ());
    /// assert! (! F::nan ().is_sign_negative ());
    /// ```
    pub const fn is_sign_negative(&self) -> bool {
        match *self {
            GenericFraction::NaN => false,
            GenericFraction::Infinity(Sign::Plus) => false,
            GenericFraction::Infinity(Sign::Minus) => true,
            GenericFraction::Rational(Sign::Plus, _) => false,
            GenericFraction::Rational(Sign::Minus, _) => true,
        }
    }

    /// self.clone () * a + b
    ///
    /// Added for interface compatibility with float types
    pub fn mul_add(&self, a: Self, b: Self) -> Self {
        self.clone() * a + b
    }

    /// Takes the reciprocal (inverse) of the value (1/x)
    ///
    /// # Examples
    ///
    /// ```
    /// use fraction::GenericFraction;
    /// type F = GenericFraction<u8>;
    ///
    /// assert_eq! (F::new (1u8, 2u8).recip (), F::new (2u8, 1u8));
    /// assert_eq! (F::new (0u8, 1u8).recip (), F::infinity ());
    /// assert_eq! (F::infinity ().recip (), F::new (0u8, 1u8));
    /// assert_eq! (F::nan ().recip (), F::nan ());
    /// ```
    pub fn recip(&self) -> Self {
        match *self {
            GenericFraction::NaN => GenericFraction::NaN,
            GenericFraction::Infinity(_) => {
                GenericFraction::Rational(Sign::Plus, Ratio::new(T::zero(), T::one()))
            }
            GenericFraction::Rational(s, ref r) if r.is_zero() => GenericFraction::Infinity(s),
            GenericFraction::Rational(s, ref r) => GenericFraction::Rational(s, r.recip()),
        }
    }

    /* ... Some stuff here that has not been implemented for Ratio<T> ... */
}

impl<T: Clone + GenericInteger> fmt::Display for GenericFraction<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let format = display::Format::new(formatter);
        display::format_fraction(self, formatter, &format)
    }
}

macro_rules! fraction_from_generic_int {
    ( $($F:ty),* ) => {
        $(
        impl<T> From<$F> for GenericFraction<T>
        where
            T: Clone + Integer + GenericInteger + CheckedAdd + CheckedMul + CheckedSub,
            $F: GenericInteger + CheckedAdd + CheckedDiv + CheckedMul + CheckedSub + PartialOrd
        {
            fn from(val: $F) -> GenericFraction<T> {
                if let Some((sign, value)) = read_generic_integer::<T, $F>(val) {
                    GenericFraction::Rational(sign, Ratio::new (value, T::one()))
                } else {
                    GenericFraction::nan()
                }
            }
        }
        )*
    };
}

#[cfg(feature = "with-bigint")]
fraction_from_generic_int!(BigUint, BigInt);

fraction_from_generic_int!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

macro_rules! generic_fraction_from_float {
    ( $($from:ty),*) => {
        $(
        impl<T: Clone + FromPrimitive + Integer + CheckedAdd + CheckedMul + CheckedSub> From<$from> for GenericFraction<T> {
            fn from(val: $from) -> GenericFraction<T> {
                if val.is_nan () { return Self::NaN };
                if val.is_infinite () { return Self::Infinity (if val.is_sign_negative () { Sign::Minus } else { Sign::Plus }) };

                let sign = if val < 0.0 { Sign::Minus } else { Sign::Plus };

                // Using https://math.stackexchange.com/a/1049723/17452 , but rely on Ratio::new() to compute the gcd.
                // Find the max precision of this number
                // Note: the power computations happen in i32 until the end.
                let mut p: i32 = 0;
                let mut new_val = val;
                let ten: $from = 10.0;
                let fallback_to_string_conversion = || Self::from_str(&format!("{:+}", val)).unwrap_or(Self::NaN);
                loop {
                    if (new_val.floor() - new_val).abs() < <$from>::EPSILON {
                        // Yay, we've found the precision of this number
                        break;
                    }
                    // Multiply by the precision
                    // Note: we multiply by powers of ten to avoid this kind of round error with f32s:
                    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b760579f103b7192c20413ebbe167b90
                    p += 1;
                    new_val = val * ten.powi(p);
                    if new_val.is_infinite() {
                        return fallback_to_string_conversion();
                    }
                }

                // We store the sign of the ratio externally, so let's oppose the numerator if need be.
                // The denominator is always positive.
                if new_val < 0.0 {
                    new_val = -new_val;
                }

                let numer: T = match T::from_f64(new_val.into()) {
                    Some(v) => v,
                    None => {
                        return fallback_to_string_conversion();
                    }
                };
                let denom: T = match T::from_f64(ten.powi(p).into()) {
                    Some(v) => v,
                    None => {
                        return fallback_to_string_conversion();
                    }
                };

                Self::Rational(sign, Ratio::new(numer, denom))
            }
        }
        )*
    };
}

generic_fraction_from_float!(f32, f64);

impl<T, N, D> From<(N, D)> for GenericFraction<T>
where
    T: Clone + GenericInteger,
    N: GenericInteger + PartialOrd,
    D: GenericInteger + PartialOrd,
{
    fn from(pair: (N, D)) -> GenericFraction<T> {
        GenericFraction::new_generic(Sign::Plus, pair.0, pair.1).unwrap_or(GenericFraction::NaN)
    }
}

#[cfg(test)]
mod tests {
    use num::Integer;
    #[cfg(feature = "with-bigint")]
    use prelude::BigFraction;

    use crate::error::ParseError;
    #[cfg(feature = "with-bigint")]
    use crate::{BigInt, BigUint};

    use crate::{
        Bounded, ConstOne, ConstZero, Fraction, GenericFraction, Num, One, Sign, Signed,
        ToPrimitive, Zero,
    };

    use super::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    use std::num::FpCategory;
    use std::str::FromStr;

    extern crate rand;
    use self::rand::Rng;
    use generic::GenericInteger;

    type Frac = GenericFraction<u8>;

    fn hash_it(target: &impl Hash) -> u64 {
        use std::collections::hash_map::DefaultHasher;

        let mut h = DefaultHasher::new();
        target.hash(&mut h);
        h.finish()
    }

    generate_ops_tests! (
        NaN => {Frac::nan()};
        NegInf => {Frac::neg_infinity()};
        PosInf => {Frac::infinity()};
        Zero => {Frac::new(0, 1)};
        Half => {Frac::new(1, 2)};
        One => {Frac::new(1, 1)};
        Two => {Frac::new(2, 1)};
        Three => {Frac::new(3, 1)};
        Four => {Frac::new(4, 1)};
    );

    #[test]
    fn op_ord() {
        let pin = Frac::infinity();
        let nin = Frac::neg_infinity();
        let nil = Frac::zero();

        let a = Frac::new(3, 4);
        let b = Frac::new(5, 7);

        assert!(a > b);
        assert!(a > nil);
        assert!(b > nil);
        assert!(nin < nil);
        assert!(nil < pin);
    }

    #[test]
    fn from_i8() {
        let f = Fraction::from(-2i8);
        assert_eq!(Sign::Minus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(0i8);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(2i8);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_u8() {
        let f = Fraction::from(0u8);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(2u8);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(u8::max_value());
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(u8::max_value() as u64, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_i16() {
        let f = Fraction::from(-2i16);
        assert_eq!(Sign::Minus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(0i16);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(2i16);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_u16() {
        let f = Fraction::from(0u16);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(2u16);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_i32() {
        let f = Fraction::from(-2i32);
        assert_eq!(Sign::Minus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(0i32);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(2i32);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_u32() {
        let f = Fraction::from(0u32);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(2u32);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_i64() {
        let f = Fraction::from(-2i64);
        assert_eq!(Sign::Minus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(0i64);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(2i64);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_u64() {
        let f = Fraction::from(0u64);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(2u64);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_i128() {
        let f = Fraction::from(0i128);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(2i128);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(-2i128);
        assert_eq!(Sign::Minus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(22460602606i128);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(22460602606, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_u128() {
        let f = Fraction::from(0u128);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(2u128);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_isize() {
        let f = Fraction::from(-2isize);
        assert_eq!(Sign::Minus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(0isize);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(2isize);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_usize() {
        let f = Fraction::from(0usize);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(2usize);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(2, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_f32() {
        let f = Fraction::from(0f32);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(0.01f32);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(1, *f.numer().unwrap());
        assert_eq!(100, *f.denom().unwrap());

        let f = Fraction::from(-0.01f32);
        assert_eq!(Sign::Minus, f.sign().unwrap());
        assert_eq!(1, *f.numer().unwrap());
        assert_eq!(100, *f.denom().unwrap());

        let f = Fraction::from(16584253f32);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(16584253u64, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn from_f64() {
        let f = Fraction::from(0f64);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(0, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());

        let f = Fraction::from(0.01f64);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(1, *f.numer().unwrap());
        assert_eq!(100, *f.denom().unwrap());

        let f = Fraction::from(-0.01f64);
        assert_eq!(Sign::Minus, f.sign().unwrap());
        assert_eq!(1, *f.numer().unwrap());
        assert_eq!(100, *f.denom().unwrap());

        let f = Fraction::from(1658425342060f64);
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(1658425342060u64, *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    #[cfg(feature = "with-bigint")]
    fn from_insanity() {
        let number = "2062065394209534095362056240654064520645230645230645230645230645206452064520645203642530940959212130935957";
        let fraction = format!("{}/1", number);
        let f = BigFraction::from_str_radix(&fraction, 10);
        assert!(f.is_ok());
        let f = f.ok().unwrap();
        assert_eq!(
            BigUint::from_str_radix(&number, 10).ok().unwrap(),
            *f.numer().unwrap()
        );
        assert_eq!(BigUint::from(1u8), *f.denom().unwrap());
    }

    #[test]
    #[cfg(feature = "with-bigint")]
    fn from_bigint() {
        let number = BigInt::from(42);
        let frac = BigFraction::from(number);

        assert_eq!(frac, BigFraction::from((42, 1)));

        let number = BigInt::from(-44);
        let frac = BigFraction::from(number);

        assert_eq!(frac, -BigFraction::from((44, 1)));
    }

    #[test]
    #[cfg(feature = "with-bigint")]
    fn from_biguint() {
        let number = BigUint::from(42u32);
        let frac = BigFraction::from(number);

        assert_eq!(frac, BigFraction::from((42, 1)));
    }

    #[test]
    fn from_extremum() {
        type F8 = GenericFraction<u8>;

        let f = F8::from(u8::max_value());
        assert_eq!(Sign::Plus, f.sign().unwrap());
        assert_eq!(u8::max_value(), *f.numer().unwrap());
        assert_eq!(1, *f.denom().unwrap());
    }

    #[test]
    fn hashy() {
        {
            let mut map: HashMap<Fraction, ()> = HashMap::new();

            map.insert(Fraction::from(0.75), ());

            assert!(map.contains_key(&Fraction::new(3u8, 4u8))); // 0.75 == 3/4
            assert!(map.contains_key(&Fraction::new(6u16, 8u16))); // 0.75 == 6/8
            assert!(map.contains_key(&Fraction::new(12u32, 16u32))); // 0.75 == 12/16
            assert!(map.contains_key(&Fraction::new(24u64, 32u64))); // 0.75 == 24/32
            assert!(map.contains_key(&Fraction::new(48u8, 64u16))); // 0.75 == 48/64

            assert!(map.contains_key(&Fraction::from((3i8, 4i8))));
            assert!(map.contains_key(&Fraction::from((6i16, 8i16))));
            assert!(map.contains_key(&Fraction::from((12i32, 16i32))));
            assert!(map.contains_key(&Fraction::from((24i64, 32i64))));
            assert!(map.contains_key(&Fraction::from((48i8, 64i16))));

            assert!(!map.contains_key(&Fraction::from(0.5))); // 0.75 != 1/2
        }

        {
            assert_eq!(hash_it(&Fraction::nan()), hash_it(&Fraction::nan()));
            assert_ne!(hash_it(&Fraction::nan()), hash_it(&Fraction::zero()));
            assert_ne!(
                hash_it(&Fraction::infinity()),
                hash_it(&Fraction::neg_infinity())
            );
            assert_ne!(hash_it(&Fraction::infinity()), hash_it(&Fraction::nan()));
            assert_eq!(
                hash_it(&Fraction::infinity()),
                hash_it(&Fraction::infinity())
            );
            assert_eq!(
                hash_it(&Fraction::neg_infinity()),
                hash_it(&Fraction::neg_infinity())
            );
            assert_eq!(
                hash_it(&Fraction::neg_infinity()),
                hash_it(&Fraction::neg_infinity())
            );

            assert_eq!(
                hash_it(&Fraction::new(1u8, 2u8)),
                hash_it(&Fraction::new(2u8, 4u8))
            );
            assert_eq!(
                hash_it(&Fraction::new(1u8, 0u8)),
                hash_it(&Fraction::new(2u8, 0u8))
            );
            assert_eq!(
                hash_it(&Fraction::new(0u8, 1u8)),
                hash_it(&Fraction::new(0u8, 2u8))
            );

            assert_eq!(hash_it(&Fraction::zero()), hash_it(&Fraction::zero()));
            assert_eq!(hash_it(&Frac::zero()), hash_it(&Frac::neg_zero()));
        }
    }

    #[test]
    fn comparison() {
        assert_eq!(Frac::zero(), Frac::zero());
        assert_eq!(Frac::zero(), Frac::neg_zero());
        assert_eq!(Frac::from(0), Frac::zero());
        assert_eq!(Frac::from(0), Frac::neg_zero());
        assert_eq!(Frac::from(0.5), Frac::new(1u8, 2u8));
        assert_eq!(Frac::from(-0.5), Frac::new_neg(1u8, 2u8));
        assert_ne!(Frac::from(-0.5), Frac::new(1u8, 2u8));

        assert!(!(Frac::zero() < Frac::neg_zero()));
        assert!(!(Frac::neg_zero() < Frac::zero()));

        assert!(!(Frac::zero() > Frac::neg_zero()));
        assert!(!(Frac::neg_zero() > Frac::zero()));

        assert!(Frac::neg_zero() < Frac::new(1u8, 2u8));
        assert!(!(Frac::neg_zero() > Frac::new(1u8, 2u8)));

        assert!(Frac::zero() < Frac::new(1u8, 2u8));
        assert!(!(Frac::zero() > Frac::new(1u8, 2u8)));

        assert!(Frac::new_neg(1u8, 2u8) < Frac::neg_zero());
        assert!(Frac::new_neg(1u8, 2u8) < Frac::zero());

        assert!(!(Frac::new_neg(1u8, 2u8) > Frac::neg_zero()));
        assert!(!(Frac::new_neg(1u8, 2u8) > Frac::zero()));

        assert_eq!(Frac::new(1u8, 2u8), Frac::new(1u8, 2u8));
        assert_eq!(Frac::new_neg(1u8, 2u8), Frac::new_neg(1u8, 2u8));

        assert!(Frac::new_neg(1u8, 2u8) < Frac::new(1u8, 2u8));
        assert!(!(Frac::new(1u8, 2u8) < Frac::new_neg(1u8, 2u8)));
        assert!(!(Frac::new_neg(1u8, 2u8) < Frac::new_neg(1u8, 2u8)));
        assert!(Frac::new_neg(1u8, 2u8) < Frac::new_neg(1u8, 4u8));

        assert!(Frac::new_neg(1u8, 2u8) < Frac::neg_zero());
        assert!(Frac::new_neg(1u8, 2u8) < Frac::zero());
        assert!(!(Frac::neg_zero() < Frac::new_neg(1u8, 2u8)));
        assert!(!(Frac::zero() < Frac::new_neg(1u8, 2u8)));
        assert!(Frac::neg_zero() < Frac::new(1u8, 2u8));
        assert!(Frac::neg_zero() > Frac::new_neg(1u8, 2u8));
        assert!(Frac::zero() > Frac::new_neg(1u8, 2u8));
        assert!(Frac::new(1u8, 2u8) > Frac::neg_zero());
        assert!(!(Frac::new(1u8, 2u8) < Frac::neg_zero()));
        assert!(Frac::zero() < Frac::new(1u8, 2u8));
    }

    #[test]
    fn cmp() {
        /* CMP */
        let nan = Frac::nan();
        let neg_inf = Frac::neg_infinity();
        let neg_one = Frac::one() * -1;
        let zero = Frac::zero();
        let one = Frac::one();
        let inf = Frac::infinity();

        assert_eq!(nan.cmp(&nan), Ordering::Equal);
        assert_eq!(nan.cmp(&neg_inf), Ordering::Less);
        assert_eq!(nan.cmp(&neg_one), Ordering::Less);
        assert_eq!(nan.cmp(&zero), Ordering::Less);
        assert_eq!(nan.cmp(&one), Ordering::Less);
        assert_eq!(nan.cmp(&inf), Ordering::Less);

        assert_eq!(neg_inf.cmp(&nan), Ordering::Greater);
        assert_eq!(neg_inf.cmp(&neg_inf), Ordering::Equal);
        assert_eq!(neg_inf.cmp(&neg_one), Ordering::Less);
        assert_eq!(neg_inf.cmp(&zero), Ordering::Less);
        assert_eq!(neg_inf.cmp(&one), Ordering::Less);
        assert_eq!(neg_inf.cmp(&inf), Ordering::Less);

        assert_eq!(neg_one.cmp(&nan), Ordering::Greater);
        assert_eq!(neg_one.cmp(&neg_inf), Ordering::Greater);
        assert_eq!(neg_one.cmp(&neg_one), Ordering::Equal);
        assert_eq!(neg_one.cmp(&zero), Ordering::Less);
        assert_eq!(neg_one.cmp(&one), Ordering::Less);
        assert_eq!(neg_one.cmp(&inf), Ordering::Less);

        assert_eq!(zero.cmp(&nan), Ordering::Greater);
        assert_eq!(zero.cmp(&neg_inf), Ordering::Greater);
        assert_eq!(zero.cmp(&neg_one), Ordering::Greater);
        assert_eq!(zero.cmp(&zero), Ordering::Equal);
        assert_eq!(zero.cmp(&one), Ordering::Less);
        assert_eq!(zero.cmp(&inf), Ordering::Less);

        assert_eq!(one.cmp(&nan), Ordering::Greater);
        assert_eq!(one.cmp(&neg_inf), Ordering::Greater);
        assert_eq!(one.cmp(&neg_one), Ordering::Greater);
        assert_eq!(one.cmp(&zero), Ordering::Greater);
        assert_eq!(one.cmp(&one), Ordering::Equal);
        assert_eq!(one.cmp(&inf), Ordering::Less);

        assert_eq!(inf.cmp(&nan), Ordering::Greater);
        assert_eq!(inf.cmp(&neg_inf), Ordering::Greater);
        assert_eq!(inf.cmp(&neg_one), Ordering::Greater);
        assert_eq!(inf.cmp(&zero), Ordering::Greater);
        assert_eq!(inf.cmp(&one), Ordering::Greater);
        assert_eq!(inf.cmp(&inf), Ordering::Equal);
    }

    #[test]
    fn floor() {
        assert_eq!(Frac::zero(), Frac::new(1, 3).floor());
        assert_eq!(Frac::one() * -1, Frac::new_neg(1, 3).floor());
    }

    #[test]
    fn ceil() {
        assert_eq!(Frac::zero(), Frac::new_neg(1, 3).ceil());
        assert_eq!(Frac::one(), Frac::new(1, 3).ceil());
    }

    #[test]
    fn from_str() {
        assert_eq!(Ok(Frac::zero()), Frac::from_str("0"));
        assert_eq!(Ok(Frac::zero()), Frac::from_str("-0"));
        assert_eq!(Ok(Frac::zero()), Frac::from_str("+0"));

        assert_eq!(Ok(Frac::zero()), Frac::from_str("0.0"));
        assert_eq!(Ok(Frac::zero()), Frac::from_str("-0.0"));
        assert_eq!(Ok(Frac::zero()), Frac::from_str("+0.0"));

        assert_eq!(Ok(Fraction::zero()), Fraction::from_str("0.000000"));
        assert_eq!(Ok(Fraction::zero()), Fraction::from_str("-0.000000"));
        assert_eq!(Ok(Fraction::zero()), Fraction::from_str("+0.000000"));

        assert_eq!(Ok(Fraction::zero()), Fraction::from_str("0/1"));
        assert_eq!(Ok(Fraction::zero()), Fraction::from_str("-0/1"));
        assert_eq!(Ok(Fraction::zero()), Fraction::from_str("+0/1"));

        assert_eq!(
            Ok(Fraction::from(10000000000000_u64)),
            Fraction::from_str("10000000000000.0000000000")
        );

        #[cfg(feature = "with-bigint")]
        {
            assert_eq!(
                Ok(BigFraction::zero()),
                BigFraction::from_str(
                    "00000000000000000000000000.0000000000000000000000000000000000000000000"
                )
            );
            assert_eq!(
                Ok(BigFraction::zero()),
                BigFraction::from_str(
                    "-0000000000000000000000000.0000000000000000000000000000000000000000000"
                )
            );
            assert_eq!(
                Ok(BigFraction::zero()),
                BigFraction::from_str(
                    "+0000000000000000000000000.0000000000000000000000000000000000000000000"
                )
            );

            assert_eq!(
                Ok(BigFraction::zero()),
                BigFraction::from_str("00000000000000000000000000/1")
            );
            assert_eq!(
                Ok(BigFraction::zero()),
                BigFraction::from_str("-0000000000000000000000000/1")
            );
            assert_eq!(
                Ok(BigFraction::zero()),
                BigFraction::from_str("+0000000000000000000000000/1")
            );
        }

        assert_eq!(Ok(Frac::one()), Frac::from_str("1"));
        assert_eq!(Ok(Frac::new_neg(1, 1)), Frac::from_str("-1"));
        assert_eq!(Ok(Frac::one()), Frac::from_str("+1"));

        assert_eq!(Ok(Frac::one()), Frac::from_str("1.0"));
        assert_eq!(Ok(Frac::new_neg(1, 1)), Frac::from_str("-1.0"));
        assert_eq!(Ok(Frac::one()), Frac::from_str("+1.00"));

        assert_eq!(Ok(Frac::one()), Frac::from_str("1/1"));
        assert_eq!(Ok(Frac::new_neg(1, 1)), Frac::from_str("-1/1"));
        assert_eq!(Ok(Frac::one()), Frac::from_str("+1/1"));

        assert_eq!(Ok(Frac::new(1, 2)), Frac::from_str("0.5"));
        assert_eq!(Ok(Frac::new(1, 2)), Frac::from_str("1/2"));

        assert_eq!(
            Ok(Fraction::new(3333u64, 5000u64)),
            Fraction::from_str("0.6666")
        );
        assert_eq!(
            Ok(Fraction::new(3333u64, 5000u64)),
            Fraction::from_str("3333/5000")
        );

        assert_eq!(Err(ParseError::ParseIntError), Frac::from_str("test"));
        assert_eq!(Err(ParseError::ParseIntError), Frac::from_str("1test"));
        assert_eq!(Err(ParseError::ParseIntError), Frac::from_str("1.26t8"));

        // this is due to the std library which issues ParseIntError on the whole part overflow
        assert_eq!(Err(ParseError::ParseIntError), Frac::from_str("120202040"));
        assert_eq!(Err(ParseError::ParseIntError), Frac::from_str("1.20602604"));

        assert_eq!(Err(ParseError::OverflowError), Frac::from_str("255.255"));

        assert_eq!(Err(ParseError::ParseIntError), Frac::from_str("256/256"));
    }

    #[test]
    fn new_generic() {
        {
            type F = GenericFraction<u8>;

            let f12 = F::new_generic(Sign::Plus, 1i8, 2u8).unwrap();
            let f34 = F::new_generic(Sign::Minus, 3i16, 4u32).unwrap();
            let f56 = F::new_generic(Sign::Plus, -5i64, 6u128).unwrap();
            let f78 = F::new_generic(Sign::Minus, 7usize, -8isize).unwrap();

            #[cfg(feature = "with-bigint")]
            {
                let fbig =
                    F::new_generic(Sign::Minus, -BigInt::from(254), BigUint::from(255u32)).unwrap();

                assert_eq!(
                    (
                        fbig.sign().unwrap(),
                        *fbig.numer().unwrap(),
                        *fbig.denom().unwrap()
                    ),
                    (Sign::Plus, 254u8, 255u8)
                );
            }

            assert_eq!(
                (
                    f12.sign().unwrap(),
                    *f12.numer().unwrap(),
                    *f12.denom().unwrap()
                ),
                (Sign::Plus, 1u8, 2u8)
            );
            assert_eq!(
                (
                    f34.sign().unwrap(),
                    *f34.numer().unwrap(),
                    *f34.denom().unwrap()
                ),
                (Sign::Minus, 3u8, 4u8)
            );
            assert_eq!(
                (
                    f56.sign().unwrap(),
                    *f56.numer().unwrap(),
                    *f56.denom().unwrap()
                ),
                (Sign::Minus, 5u8, 6u8)
            );
            assert_eq!(
                (
                    f78.sign().unwrap(),
                    *f78.numer().unwrap(),
                    *f78.denom().unwrap()
                ),
                (Sign::Plus, 7u8, 8u8)
            );

            assert_eq!(None, F::new_generic(Sign::Plus, 256, 1)); // overflow
            assert_eq!(None, F::new_generic(Sign::Plus, 1, 257)); // overflow
        }

        {
            type F = GenericFraction<i8>;

            let f12 = F::new_generic(Sign::Plus, 1i8, 2u8).unwrap();
            let f34 = F::new_generic(Sign::Minus, 3i16, 4u32).unwrap();
            let f56 = F::new_generic(Sign::Plus, -5i64, 6u128).unwrap();
            let f78 = F::new_generic(Sign::Minus, 7usize, -8isize).unwrap();

            assert_eq!(
                (
                    f12.sign().unwrap(),
                    *f12.numer().unwrap(),
                    *f12.denom().unwrap()
                ),
                (Sign::Plus, 1i8, 2i8)
            );
            assert_eq!(
                (
                    f34.sign().unwrap(),
                    *f34.numer().unwrap(),
                    *f34.denom().unwrap()
                ),
                (Sign::Minus, 3i8, 4i8)
            );
            assert_eq!(
                (
                    f56.sign().unwrap(),
                    *f56.numer().unwrap(),
                    *f56.denom().unwrap()
                ),
                (Sign::Minus, 5i8, 6i8)
            );
            assert_eq!(
                (
                    f78.sign().unwrap(),
                    *f78.numer().unwrap(),
                    *f78.denom().unwrap()
                ),
                (Sign::Plus, 7i8, 8i8)
            );

            assert_eq!(None, F::new_generic(Sign::Plus, 128, 1)); // overflow
            assert_eq!(None, F::new_generic(Sign::Plus, 256, 1)); // overflow

            #[cfg(feature = "with-bigint")]
            {
                let fbig =
                    F::new_generic(Sign::Minus, -BigInt::from(126), BigUint::from(127u8)).unwrap();

                assert_eq!(
                    (
                        fbig.sign().unwrap(),
                        *fbig.numer().unwrap(),
                        *fbig.denom().unwrap()
                    ),
                    (Sign::Plus, 126i8, 127i8)
                );
            }
        }

        #[cfg(feature = "with-bigint")]
        {
            type F = GenericFraction<BigUint>;

            let f12 = F::new_generic(Sign::Plus, 1i8, 2u8).unwrap();
            let f34 = F::new_generic(Sign::Minus, 3i16, 4u32).unwrap();
            let f56 = F::new_generic(Sign::Plus, -5i64, 6u128).unwrap();
            let f78 = F::new_generic(Sign::Minus, 7usize, -8isize).unwrap();
            let fbig =
                F::new_generic(Sign::Minus, -BigInt::from(254), BigUint::from(255u32)).unwrap();

            assert_eq!(
                (
                    f12.sign().unwrap(),
                    f12.numer().unwrap(),
                    f12.denom().unwrap()
                ),
                (Sign::Plus, &BigUint::from(1u8), &BigUint::from(2u8))
            );
            assert_eq!(
                (
                    f34.sign().unwrap(),
                    f34.numer().unwrap(),
                    f34.denom().unwrap()
                ),
                (Sign::Minus, &BigUint::from(3u8), &BigUint::from(4u8))
            );
            assert_eq!(
                (
                    f56.sign().unwrap(),
                    f56.numer().unwrap(),
                    f56.denom().unwrap()
                ),
                (Sign::Minus, &BigUint::from(5u8), &BigUint::from(6u8))
            );
            assert_eq!(
                (
                    f78.sign().unwrap(),
                    f78.numer().unwrap(),
                    f78.denom().unwrap()
                ),
                (Sign::Plus, &BigUint::from(7u8), &BigUint::from(8u8))
            );
            assert_eq!(
                (
                    fbig.sign().unwrap(),
                    fbig.numer().unwrap(),
                    fbig.denom().unwrap()
                ),
                (Sign::Plus, &BigUint::from(254u8), &BigUint::from(255u8))
            );
        }

        #[cfg(feature = "with-bigint")]
        {
            type F = GenericFraction<BigInt>;

            let f12 = F::new_generic(Sign::Plus, 1i8, 2u8).unwrap();
            let f34 = F::new_generic(Sign::Minus, 3i16, 4u32).unwrap();
            let f56 = F::new_generic(Sign::Plus, -5i64, 6u128).unwrap();
            let f78 = F::new_generic(Sign::Minus, 7usize, -8isize).unwrap();
            let fbig =
                F::new_generic(Sign::Minus, -BigInt::from(254), BigUint::from(255u32)).unwrap();

            assert_eq!(
                (
                    f12.sign().unwrap(),
                    f12.numer().unwrap(),
                    f12.denom().unwrap()
                ),
                (Sign::Plus, &BigInt::from(1u8), &BigInt::from(2u8))
            );
            assert_eq!(
                (
                    f34.sign().unwrap(),
                    f34.numer().unwrap(),
                    f34.denom().unwrap()
                ),
                (Sign::Minus, &BigInt::from(3u8), &BigInt::from(4u8))
            );
            assert_eq!(
                (
                    f56.sign().unwrap(),
                    f56.numer().unwrap(),
                    f56.denom().unwrap()
                ),
                (Sign::Minus, &BigInt::from(5u8), &BigInt::from(6u8))
            );
            assert_eq!(
                (
                    f78.sign().unwrap(),
                    f78.numer().unwrap(),
                    f78.denom().unwrap()
                ),
                (Sign::Plus, &BigInt::from(7u8), &BigInt::from(8u8))
            );
            assert_eq!(
                (
                    fbig.sign().unwrap(),
                    fbig.numer().unwrap(),
                    fbig.denom().unwrap()
                ),
                (Sign::Plus, &BigInt::from(254u8), &BigInt::from(255u8))
            );
        }

        {
            type F = GenericFraction<u128>;

            let f1 = F::new_generic(Sign::Plus, 123456788u64, 123456789i32).unwrap();
            let f2 = F::new_generic(Sign::Minus, 1234567890122u64, -1234567890123i64).unwrap();

            assert_eq!(
                (
                    f1.sign().unwrap(),
                    *f1.numer().unwrap(),
                    *f1.denom().unwrap()
                ),
                (Sign::Plus, 123456788u128, 123456789u128)
            );
            assert_eq!(
                (
                    f2.sign().unwrap(),
                    *f2.numer().unwrap(),
                    *f2.denom().unwrap()
                ),
                (Sign::Plus, 1234567890122u128, 1234567890123u128)
            );
        }
    }

    #[test]
    fn sign() {
        let p = Sign::Plus;
        let m = Sign::Minus;

        assert_ne!(p, m);
        assert_eq!(p, Sign::Plus);
        assert_eq!(m, Sign::Minus);

        assert_eq!(m, p * m);
        assert_eq!(p, p * p);
        assert_eq!(p, m * m);
        assert_eq!(m, m * p);

        assert!(p.is_positive());
        assert!(!p.is_negative());

        assert!(!m.is_positive());
        assert!(m.is_negative());
    }

    #[test]
    fn fraction() {
        assert_eq!(Frac::nan(), Frac::new(0, 0));
        assert_eq!(Frac::infinity(), Frac::new(1, 0));

        assert!(Frac::nan().numer().is_none());
        assert!(Frac::nan().denom().is_none());

        assert_eq!(
            Fraction::new(5u64, 8u64),
            Fraction::from_fraction(Frac::new(5u8, 8u8))
        );
        assert_eq!(Fraction::nan(), Fraction::from_fraction(Frac::nan()));
        assert_eq!(
            Fraction::infinity(),
            Fraction::from_fraction(Frac::infinity())
        );

        assert_eq!(Frac::min_value(), Frac::new_neg(255, 1));
        assert_eq!(Frac::max_value(), Frac::new(255, 1));

        assert_ne!(Frac::neg_infinity(), Frac::infinity());
        assert_ne!(Frac::nan(), Frac::zero());
        assert_ne!(Frac::zero(), Frac::nan());
        assert_ne!(Frac::new_neg(1, 2), Frac::new(1, 2));
        assert_eq!(Frac::neg_zero(), Frac::zero());
        assert_ne!(Frac::infinity(), Frac::nan());

        assert!(!(Frac::infinity() > Frac::infinity()));
        assert!((Frac::infinity() > Frac::neg_infinity()));
        assert!(!(Frac::neg_infinity() > Frac::infinity()));

        assert!(Frac::infinity() > Frac::max_value());
        assert!(Frac::min_value() > Frac::neg_infinity());

        assert_eq!(-Frac::infinity(), Frac::neg_infinity());

        assert_eq!(-&Frac::nan(), Frac::nan());
        assert_eq!(-&Frac::infinity(), Frac::neg_infinity());
        assert_eq!(-&Frac::one(), Frac::new_neg(1, 1));

        assert_eq!(-&Frac::zero(), Frac::zero());

        assert_eq!(
            Frac::new_neg(1, 1) + Frac::new_neg(1, 1),
            Frac::new_neg(2, 1)
        );
        assert_eq!(
            &Frac::new_neg(1, 1) + &Frac::new_neg(1, 1),
            Frac::new_neg(2, 1)
        );

        assert_eq!(Frac::new_neg(1, 1) - Frac::new_neg(1, 1), Frac::zero());
        assert_eq!(Frac::new_neg(1, 1) - Frac::new_neg(2, 1), Frac::one());
        assert_eq!(&Frac::new_neg(1, 1) - &Frac::new_neg(1, 1), Frac::zero());
        assert_eq!(&Frac::new_neg(1, 1) - &Frac::new_neg(2, 1), Frac::one());

        assert_eq!(Frac::new(1, 255), Frac::min_positive_value());

        assert!(Frac::infinity().is_infinite());
        assert!(Frac::neg_infinity().is_infinite());
        assert!(!Frac::one().is_infinite());

        assert!(!Frac::infinity().is_finite());
        assert!(!Frac::neg_infinity().is_finite());
        assert!(Frac::one().is_finite());

        assert_eq!(FpCategory::Normal, Frac::one().classify());
        assert_eq!(FpCategory::Infinite, Frac::infinity().classify());
        assert_eq!(FpCategory::Zero, Frac::zero().classify());
        assert_eq!(FpCategory::Nan, Frac::nan().classify());

        assert_eq!(Frac::nan(), Frac::nan().floor());
        assert_eq!(Frac::one(), Frac::new(3, 2).floor());

        assert_eq!(Frac::nan(), Frac::nan().ceil());
        assert_eq!(Frac::one(), Frac::new(1, 2).ceil());

        assert_eq!(Frac::nan(), Frac::nan().round());
        assert_eq!(Frac::one(), Frac::new(1, 2).round());
        assert_eq!(Frac::new(2, 1), Frac::new(3, 2).round());
        assert_eq!(Frac::one(), Frac::new(4, 3).round());

        assert_eq!(Frac::nan(), Frac::nan().trunc());
        assert_eq!(Frac::zero(), Frac::new(1, 2).trunc());
        assert_eq!(Frac::one(), Frac::new(3, 2).trunc());

        assert_eq!(Frac::nan(), Frac::nan().fract());
        assert_eq!(Frac::new(1, 2), Frac::new(1, 2).fract());
        assert_eq!(Frac::new(1, 2), Frac::new(3, 2).fract());

        assert!(!Frac::nan().is_sign_positive());
        assert!(!Frac::nan().is_sign_negative());

        assert!(Frac::infinity().is_sign_positive());
        assert!(!Frac::neg_infinity().is_sign_positive());

        assert!(!Frac::infinity().is_sign_negative());
        assert!(Frac::neg_infinity().is_sign_negative());

        assert!(Frac::one().is_sign_positive());
        assert!(!Frac::one().is_sign_negative());

        assert!(!Frac::new_neg(1, 1).is_sign_positive());
        assert!(Frac::new_neg(1, 1).is_sign_negative());

        assert_eq!(
            Frac::new(3, 1),
            Frac::one().mul_add(Frac::new(2, 1), Frac::one())
        );

        assert_eq!(Frac::nan(), Frac::nan().recip());
        assert_eq!(Frac::zero(), Frac::infinity().recip());
        assert_eq!(Frac::zero(), Frac::neg_infinity().recip());
        assert_eq!(Frac::infinity(), Frac::zero().recip());
        assert_eq!(Frac::new(2, 1), Frac::new(1, 2).recip());
    }

    #[test]
    fn from_str_radix() {
        assert_eq!(Frac::one(), Frac::from_str_radix("5/5", 10).unwrap());
        assert_eq!(Frac::one(), Frac::from_str_radix("+5/5", 10).unwrap());
        assert_eq!(Frac::new(1, 2), Frac::from_str_radix("+5/10", 10).unwrap());
        assert_eq!(
            Frac::new_neg(4, 3),
            Frac::from_str_radix("-4/3", 10).unwrap()
        );
    }

    #[test]
    fn signed() {
        // abs
        assert_eq!(Frac::one(), <Frac as Signed>::abs(&Frac::new_neg(1, 1)));
        assert_eq!(Frac::nan(), <Frac as Signed>::abs(&Frac::nan()));
        assert_eq!(Frac::infinity(), <Frac as Signed>::abs(&Frac::infinity()));
        assert_eq!(
            Frac::infinity(),
            <Frac as Signed>::abs(&Frac::neg_infinity())
        );

        // abs_sub
        assert_eq!(Frac::nan(), Frac::nan().abs_sub(&Frac::nan()));
        assert_eq!(Frac::nan(), Frac::infinity().abs_sub(&Frac::nan()));
        assert_eq!(Frac::zero(), Frac::infinity().abs_sub(&Frac::infinity()));
        assert_eq!(
            Frac::infinity(),
            Frac::infinity().abs_sub(&Frac::neg_infinity())
        );
        assert_eq!(
            Frac::zero(),
            Frac::neg_infinity().abs_sub(&Frac::neg_infinity())
        );
        assert_eq!(
            Frac::zero(),
            Frac::neg_infinity().abs_sub(&Frac::infinity())
        );
        assert_eq!(Frac::infinity(), Frac::infinity().abs_sub(&Frac::one()));
        assert_eq!(
            Frac::infinity(),
            Frac::infinity().abs_sub(&Frac::new_neg(1, 1))
        );
        assert_eq!(Frac::zero(), Frac::neg_infinity().abs_sub(&Frac::one()));
        assert_eq!(Frac::nan(), Frac::one().abs_sub(&Frac::nan()));
        assert_eq!(Frac::zero(), Frac::one().abs_sub(&Frac::infinity()));
        assert_eq!(Frac::infinity(), Frac::one().abs_sub(&Frac::neg_infinity()));
        assert_eq!(
            Frac::neg_infinity(),
            Frac::new_neg(1, 1).abs_sub(&Frac::neg_infinity())
        );
        assert_eq!(Frac::one(), Frac::new(2, 1).abs_sub(&Frac::one()));
        assert_eq!(Frac::one(), Frac::one().abs_sub(&Frac::new(2, 1)));

        // signum
        assert_eq!(Frac::nan(), Frac::nan().signum());
        assert_eq!(Frac::one(), Frac::infinity().signum());
        assert_eq!(Frac::one(), Frac::zero().signum());
        assert_eq!(Frac::one(), Frac::one().signum());
        assert_eq!(-Frac::one(), Frac::neg_infinity().signum());
        assert_eq!(-Frac::one(), Frac::new_neg(1, 1).signum());
    }

    #[test]
    fn to_primitive() {
        assert!(Frac::nan().to_i64().is_none());
        assert!(Frac::nan().to_u64().is_none());

        assert!(Frac::infinity().to_i64().is_none());
        assert!(Frac::infinity().to_u64().is_none());

        assert!(Frac::neg_infinity().to_i64().is_none());
        assert!(Frac::neg_infinity().to_u64().is_none());

        assert_eq!(Some(1), Frac::one().to_i64());
        assert_eq!(Some(1), Frac::one().to_u64());

        assert!(Frac::new(1, 2).to_i64().is_none());
        assert!(Frac::new(1, 2).to_u64().is_none());

        assert_eq!(Some(-1), Frac::new_neg(1, 1).to_i64());
        assert!(Frac::new_neg(1, 1).to_u64().is_none());

        /* f64 */

        assert!(Frac::nan().to_f64().unwrap().is_nan());
        assert_eq!(::std::f64::INFINITY, Frac::infinity().to_f64().unwrap());
        assert_eq!(
            ::std::f64::NEG_INFINITY,
            Frac::neg_infinity().to_f64().unwrap()
        );

        assert_eq!(1f64, Frac::one().to_f64().unwrap());
    }

    #[test]
    fn summing_iterator() {
        let values = vec![Fraction::new(2u64, 3u64), Fraction::new(1u64, 3u64)];
        let sum: Fraction = values.iter().sum();
        assert_eq!(sum, Fraction::new(1u8, 1u8));
    }

    #[test]
    fn product_iterator() {
        let values = vec![Fraction::new(2u64, 3u64), Fraction::new(1u64, 3u64)];
        let product: Fraction = values.iter().product();
        assert_eq!(product, Fraction::new(2u8, 9u8));
    }

    #[test]
    fn fraction_from_float() {
        macro_rules! test_for_smaller_t {
            ( $($t:ty),*) => {
                $(
                    // f32 tests
                    let f = GenericFraction::<$t>::from(-std::f32::NAN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f32::NAN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f32::MIN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f32::MIN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f32::MAX);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f32::MAX);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f32::INFINITY);
                    assert_eq!(format!("{}", f), "-inf");
                    let f = GenericFraction::<$t>::from(std::f32::INFINITY);
                    assert_eq!(format!("{}", f), "inf");
                    let f = GenericFraction::<$t>::from(-0.0_f32);
                    assert_eq!(format!("{}", f), "0");
                    let f = GenericFraction::<$t>::from(0.0_f32);
                    assert_eq!(format!("{}", f), "0");
                    let f = GenericFraction::<$t>::from(-1.0_f32);
                    assert_eq!(format!("{}", f), "-1");
                    let f = GenericFraction::<$t>::from(1.0_f32);
                    assert_eq!(format!("{}", f), "1");
                    // f64 tests
                    let f = GenericFraction::<$t>::from(-std::f64::NAN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f64::NAN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f64::MIN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f64::MIN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f64::MAX);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f64::MAX);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f64::INFINITY);
                    assert_eq!(format!("{}", f), "-inf");
                    let f = GenericFraction::<$t>::from(std::f64::INFINITY);
                    assert_eq!(format!("{}", f), "inf");
                    let f = GenericFraction::<$t>::from(-0.0_f64);
                    assert_eq!(format!("{}", f), "0");
                    let f = GenericFraction::<$t>::from(0.0_f64);
                    assert_eq!(format!("{}", f), "0");
                    let f = GenericFraction::<$t>::from(-1.0_f64);
                    assert_eq!(format!("{}", f), "-1");
                    let f = GenericFraction::<$t>::from(1.0_f64);
                    assert_eq!(format!("{}", f), "1");
                    // Arbitrary tests
                    assert_eq!(format!("{}", f), "1");
                    let f = GenericFraction::<$t>::from(2.0);
                    assert_eq!(format!("{}", f), "2");
                    let f = GenericFraction::<$t>::from(0.5);
                    assert_eq!(format!("{}", f), "1/2");
                    let f = GenericFraction::<$t>::from(15978.649);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-0.75);
                    assert_eq!(format!("{}", f), "-3/4");
                )*
            };
        }

        macro_rules! test_for_larger_t {
            ( $($t:ty),*) => {
                $(
                    // f32 tests
                    let f = GenericFraction::<$t>::from(-std::f32::NAN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f32::NAN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f32::MIN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f32::MIN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f32::MAX);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f32::MAX);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f32::INFINITY);
                    assert_eq!(format!("{}", f), "-inf");
                    let f = GenericFraction::<$t>::from(std::f32::INFINITY);
                    assert_eq!(format!("{}", f), "inf");
                    let f = GenericFraction::<$t>::from(-0.0_f32);
                    assert_eq!(format!("{}", f), "0");
                    let f = GenericFraction::<$t>::from(0.0_f32);
                    assert_eq!(format!("{}", f), "0");
                    let f = GenericFraction::<$t>::from(-1.0_f32);
                    assert_eq!(format!("{}", f), "-1");
                    let f = GenericFraction::<$t>::from(1.0_f32);
                    assert_eq!(format!("{}", f), "1");
                    // f64 tests
                    let f = GenericFraction::<$t>::from(-std::f64::NAN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f64::NAN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f64::MIN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f64::MIN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f64::MAX);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f64::MAX);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f64::INFINITY);
                    assert_eq!(format!("{}", f), "-inf");
                    let f = GenericFraction::<$t>::from(std::f64::INFINITY);
                    assert_eq!(format!("{}", f), "inf");
                    let f = GenericFraction::<$t>::from(-0.0_f64);
                    assert_eq!(format!("{}", f), "0");
                    let f = GenericFraction::<$t>::from(0.0_f64);
                    assert_eq!(format!("{}", f), "0");
                    let f = GenericFraction::<$t>::from(-1.0_f64);
                    assert_eq!(format!("{}", f), "-1");
                    let f = GenericFraction::<$t>::from(1.0_f64);
                    assert_eq!(format!("{}", f), "1");
                    // Arbitrary tests
                    let f = GenericFraction::<$t>::from(2.0);
                    assert_eq!(format!("{}", f), "2");
                    let f = GenericFraction::<$t>::from(0.5);
                    assert_eq!(format!("{}", f), "1/2");
                    let f = GenericFraction::<$t>::from(15978.649);
                    assert_eq!(format!("{}", f), "15978649/1000");
                    let f = GenericFraction::<$t>::from(-0.75);
                    assert_eq!(format!("{}", f), "-3/4");
                )*
            };
        }

        macro_rules! test_for_big_t {
            ( $($t:ty),*) => {
                $(
                    // Note: we don't test min/max for big_t because the value depends on the type
                    // f32 tests
                    let f = GenericFraction::<$t>::from(-std::f32::NAN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f32::NAN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f32::INFINITY);
                    assert_eq!(format!("{}", f), "-inf");
                    let f = GenericFraction::<$t>::from(std::f32::INFINITY);
                    assert_eq!(format!("{}", f), "inf");
                    let f = GenericFraction::<$t>::from(-0.0_f32);
                    assert_eq!(format!("{}", f), "0");
                    let f = GenericFraction::<$t>::from(0.0_f32);
                    assert_eq!(format!("{}", f), "0");
                    let f = GenericFraction::<$t>::from(-1.0_f32);
                    assert_eq!(format!("{}", f), "-1");
                    let f = GenericFraction::<$t>::from(1.0_f32);
                    assert_eq!(format!("{}", f), "1");
                    // f64 tests
                    let f = GenericFraction::<$t>::from(-std::f64::NAN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(std::f64::NAN);
                    assert_eq!(format!("{}", f), "NaN");
                    let f = GenericFraction::<$t>::from(-std::f64::INFINITY);
                    assert_eq!(format!("{}", f), "-inf");
                    let f = GenericFraction::<$t>::from(std::f64::INFINITY);
                    assert_eq!(format!("{}", f), "inf");
                    let f = GenericFraction::<$t>::from(-0.0_f64);
                    assert_eq!(format!("{}", f), "0");
                    let f = GenericFraction::<$t>::from(0.0_f64);
                    assert_eq!(format!("{}", f), "0");
                    let f = GenericFraction::<$t>::from(-1.0_f64);
                    assert_eq!(format!("{}", f), "-1");
                    let f = GenericFraction::<$t>::from(1.0_f64);
                    assert_eq!(format!("{}", f), "1");
                    // Arbitrary tests
                    let f = GenericFraction::<$t>::from(2.0);
                    assert_eq!(format!("{}", f), "2");
                    let f = GenericFraction::<$t>::from(0.5);
                    assert_eq!(format!("{}", f), "1/2");
                    let f = GenericFraction::<$t>::from(15978.649);
                    assert_eq!(format!("{}", f), "15978649/1000");
                    let f = GenericFraction::<$t>::from(-0.75);
                    assert_eq!(format!("{}", f), "-3/4");
                    let f = GenericFraction::<$t>::from(-0.5);
                    assert_eq!(format!("{}", f), "-1/2");
                )*
            };
        }

        test_for_smaller_t!(u8, i8, u16, i16);
        test_for_larger_t!(u32, i32, u64, i64, usize, isize);
        test_for_big_t!(u128, i128);

        #[cfg(feature = "with-bigint")]
        {
            use crate::{BigInt, BigUint};
            test_for_big_t!(BigUint, BigInt);
        }
    }

    #[test]
    fn fraction_test_default() {
        let fra = Frac::default();
        assert_eq!(fra.numer(), Some(&0u8));
        assert_eq!(fra.denom(), Some(&1u8));

        #[cfg(feature = "with-bigint")]
        {
            let fra = BigFraction::default();
            assert_eq!(fra.numer(), Some(&BigUint::from(0u8)));
            assert_eq!(fra.denom(), Some(&BigUint::from(1u8)));
        }
    }

    fn clamp_agree_with_cmp<T: Clone + Integer + std::fmt::Debug + GenericInteger>(
        min: &GenericFraction<T>,
        max: &GenericFraction<T>,
        test_value: &GenericFraction<T>,
    ) {
        if min.cmp(max) == Ordering::Greater {
            panic!("min is greater than max");
        }

        let clamped = test_value.clamp(min, max);

        match (test_value.cmp(min), test_value.cmp(max)) {
            (Ordering::Less, Ordering::Less) => assert_eq!(clamped, min),
            (Ordering::Less, Ordering::Equal) => assert_eq!(clamped, min),
            (Ordering::Less, Ordering::Greater) => {
                panic!("Shouldn't be possible to be less than min and greater than max")
            }

            (Ordering::Equal, Ordering::Less) => assert_eq!(clamped, min),
            (Ordering::Equal, Ordering::Equal) => {
                assert_eq!(clamped, min);
                assert_eq!(clamped, max);
            }
            (Ordering::Equal, Ordering::Greater) => assert_eq!(clamped, max),

            (Ordering::Greater, Ordering::Less) => assert_eq!(clamped, test_value),
            (Ordering::Greater, Ordering::Equal) => assert_eq!(clamped, max),
            (Ordering::Greater, Ordering::Greater) => assert_eq!(clamped, max),
        }
    }
    #[test]
    fn rational_rational_rational_clamp_cmp_agreement_test() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let base_value: i8 = rng.gen();
            let min = GenericFraction::<i64>::from(base_value);

            let bump: u8 = rng.gen();
            let bump: i32 = base_value as i32 + bump as i32;
            let max = GenericFraction::<i64>::from(bump);

            let test_value = GenericFraction::<i64>::new(rng.gen::<u8>(), rng.gen::<u8>());

            clamp_agree_with_cmp(&min, &max, &test_value);
        }
    }
    #[test]
    fn rational_nan_rational_clamp_cmp_agreement_test() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let base_value: i8 = rng.gen();
            let min = GenericFraction::<i64>::from(base_value);

            let bump: u8 = rng.gen();
            let bump: i32 = base_value as i32 + bump as i32;
            let max = GenericFraction::<i64>::from(bump);

            let nan = GenericFraction::<i64>::NaN;

            clamp_agree_with_cmp(&min, &max, &nan);
        }
    }
    #[test]
    fn nan_pos_infinity_partail_cmp_cmp_agreement_test() {
        let nan = GenericFraction::<i64>::NaN;
        let pos_inf = GenericFraction::<i64>::infinity();

        assert_eq!(nan.partial_cmp(&pos_inf), Some(nan.cmp(&pos_inf)));
    }
    #[test]
    fn nan_neg_infinity_partail_cmp_cmp_agreement_test() {
        let nan = GenericFraction::<i64>::NaN;
        let neg_inf = GenericFraction::<i64>::neg_infinity();

        assert_eq!(nan.partial_cmp(&neg_inf), Some(nan.cmp(&neg_inf)));
    }
    #[test]
    fn nan_rational_partail_cmp_cmp_agreement_test() {
        let mut rng = rand::thread_rng();

        let nan = GenericFraction::<i64>::NaN;

        for _ in 0..10 {
            let base_value: i8 = rng.gen();
            let test_value = GenericFraction::<i64>::from(base_value);

            assert_eq!(nan.partial_cmp(&test_value), Some(nan.cmp(&test_value)));
        }
    }
    #[test]
    fn pos_inf_rational_partail_cmp_cmp_agreement_test() {
        let mut rng = rand::thread_rng();

        let pos_inf = GenericFraction::<i64>::infinity();
        for _ in 0..10 {
            let base_value: i8 = rng.gen();
            let test_value = GenericFraction::<i64>::from(base_value);

            assert_eq!(
                pos_inf.partial_cmp(&test_value),
                Some(pos_inf.cmp(&test_value))
            );
        }
    }
    #[test]
    fn neg_inf_rational_partail_cmp_cmp_agreement_test() {
        let mut rng = rand::thread_rng();

        let neg_inf = GenericFraction::<i64>::neg_infinity();
        for _ in 0..10 {
            let base_value: i8 = rng.gen();
            let test_value = GenericFraction::<i64>::from(base_value);

            assert_eq!(
                neg_inf.partial_cmp(&test_value),
                Some(neg_inf.cmp(&test_value))
            );
        }
    }

    #[test]
    fn constant_one() {
        let constant_one = <Frac as ConstOne>::ONE;
        let one = Frac::one();
        assert_eq!(constant_one, one);
    }

    #[test]
    fn constant_zero() {
        let constant_zero = <Frac as ConstZero>::ZERO;
        let zero = Frac::zero();
        assert_eq!(constant_zero, zero);
    }

    #[test]
    fn consistency_partial_cmp() {
        let nan = Frac::nan();
        let neg_inf = Frac::neg_infinity();
        let neg_one = Frac::one() * -1;
        let zero = Frac::zero();
        let one = Frac::one();
        let inf = Frac::infinity();

        // 1. a == b if and only if partial_cmp(a, b) == Some(Equal).

        assert_eq!(
            Some(Ordering::Equal),
            GenericFraction::partial_cmp(&nan, &nan)
        );
        assert_eq!(
            Some(Ordering::Equal),
            GenericFraction::partial_cmp(&neg_inf, &neg_inf)
        );
        assert_eq!(
            Some(Ordering::Equal),
            GenericFraction::partial_cmp(&neg_one, &neg_one)
        );
        assert_eq!(
            Some(Ordering::Equal),
            GenericFraction::partial_cmp(&zero, &zero)
        );
        assert_eq!(
            Some(Ordering::Equal),
            GenericFraction::partial_cmp(&one, &one)
        );
        assert_eq!(
            Some(Ordering::Equal),
            GenericFraction::partial_cmp(&inf, &inf)
        );

        // 2. a < b if and only if partial_cmp(a, b) == Some(Less)

        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&nan, &neg_inf)
        );
        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&nan, &neg_one)
        );
        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&nan, &zero)
        );
        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&nan, &one)
        );
        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&nan, &inf)
        );

        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&neg_inf, &neg_one)
        );
        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&neg_inf, &zero)
        );
        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&neg_inf, &one)
        );
        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&neg_inf, &inf)
        );

        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&neg_one, &zero)
        );
        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&neg_one, &one)
        );
        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&neg_one, &inf)
        );

        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&zero, &one)
        );
        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&zero, &inf)
        );

        assert_eq!(
            Some(Ordering::Less),
            GenericFraction::partial_cmp(&one, &inf)
        );

        // 3. a > b if and only if partial_cmp(a, b) == Some(Greater)

        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&neg_inf, &nan)
        );

        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&neg_one, &neg_inf)
        );
        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&neg_one, &nan)
        );

        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&zero, &nan)
        );
        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&zero, &neg_inf)
        );
        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&zero, &neg_one)
        );

        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&one, &nan)
        );
        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&one, &neg_inf)
        );
        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&one, &neg_one)
        );
        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&one, &zero)
        );

        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&inf, &nan)
        );
        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&inf, &neg_inf)
        );
        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&inf, &neg_one)
        );
        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&inf, &zero)
        );
        assert_eq!(
            Some(Ordering::Greater),
            GenericFraction::partial_cmp(&inf, &one)
        );
    }

    #[test]
    fn consistency_cmp() {
        let nan = Frac::nan();
        let neg_inf = Frac::neg_infinity();
        let neg_one = Frac::one() * -1;
        let zero = Frac::zero();
        let one = Frac::one();
        let inf = Frac::infinity();

        // 1. a == b if and only if partial_cmp(a, b) == Some(Equal).

        assert_eq!(Ordering::Equal, GenericFraction::cmp(&nan, &nan));
        assert_eq!(Ordering::Equal, GenericFraction::cmp(&neg_inf, &neg_inf));
        assert_eq!(Ordering::Equal, GenericFraction::cmp(&neg_one, &neg_one));
        assert_eq!(Ordering::Equal, GenericFraction::cmp(&zero, &zero));
        assert_eq!(Ordering::Equal, GenericFraction::cmp(&one, &one));
        assert_eq!(Ordering::Equal, GenericFraction::cmp(&inf, &inf));

        // 2. a < b if and only if partial_cmp(a, b) == Some(Less)

        assert_eq!(Ordering::Less, GenericFraction::cmp(&nan, &neg_inf));
        assert_eq!(Ordering::Less, GenericFraction::cmp(&nan, &neg_one));
        assert_eq!(Ordering::Less, GenericFraction::cmp(&nan, &zero));
        assert_eq!(Ordering::Less, GenericFraction::cmp(&nan, &one));
        assert_eq!(Ordering::Less, GenericFraction::cmp(&nan, &inf));

        assert_eq!(Ordering::Less, GenericFraction::cmp(&neg_inf, &neg_one));
        assert_eq!(Ordering::Less, GenericFraction::cmp(&neg_inf, &zero));
        assert_eq!(Ordering::Less, GenericFraction::cmp(&neg_inf, &one));
        assert_eq!(Ordering::Less, GenericFraction::cmp(&neg_inf, &inf));

        assert_eq!(Ordering::Less, GenericFraction::cmp(&neg_one, &zero));
        assert_eq!(Ordering::Less, GenericFraction::cmp(&neg_one, &one));
        assert_eq!(Ordering::Less, GenericFraction::cmp(&neg_one, &inf));

        assert_eq!(Ordering::Less, GenericFraction::cmp(&zero, &one));
        assert_eq!(Ordering::Less, GenericFraction::cmp(&zero, &inf));

        assert_eq!(Ordering::Less, GenericFraction::cmp(&one, &inf));

        // 3. a > b if and only if partial_cmp(a, b) == Some(Greater)

        assert_eq!(Ordering::Greater, GenericFraction::cmp(&neg_inf, &nan));

        assert_eq!(Ordering::Greater, GenericFraction::cmp(&neg_one, &neg_inf));
        assert_eq!(Ordering::Greater, GenericFraction::cmp(&neg_one, &nan));

        assert_eq!(Ordering::Greater, GenericFraction::cmp(&zero, &nan));
        assert_eq!(Ordering::Greater, GenericFraction::cmp(&zero, &neg_inf));
        assert_eq!(Ordering::Greater, GenericFraction::cmp(&zero, &neg_one));

        assert_eq!(Ordering::Greater, GenericFraction::cmp(&one, &nan));
        assert_eq!(Ordering::Greater, GenericFraction::cmp(&one, &neg_inf));
        assert_eq!(Ordering::Greater, GenericFraction::cmp(&one, &neg_one));
        assert_eq!(Ordering::Greater, GenericFraction::cmp(&one, &zero));

        assert_eq!(Ordering::Greater, GenericFraction::cmp(&inf, &nan));
        assert_eq!(Ordering::Greater, GenericFraction::cmp(&inf, &neg_inf));
        assert_eq!(Ordering::Greater, GenericFraction::cmp(&inf, &neg_one));
        assert_eq!(Ordering::Greater, GenericFraction::cmp(&inf, &zero));
        assert_eq!(Ordering::Greater, GenericFraction::cmp(&inf, &one));
    }

    #[test]
    fn consistency_eq() {
        let nan = Frac::nan();
        let neg_inf = Frac::neg_infinity();
        let neg_one = Frac::one() * -1;
        let zero = Frac::zero();
        let one = Frac::one();
        let inf = Frac::infinity();

        assert!(GenericFraction::eq(&nan, &nan));
        assert!(!GenericFraction::eq(&nan, &neg_inf));
        assert!(!GenericFraction::eq(&nan, &neg_one));
        assert!(!GenericFraction::eq(&nan, &zero));
        assert!(!GenericFraction::eq(&nan, &one));
        assert!(!GenericFraction::eq(&nan, &inf));

        assert!(GenericFraction::eq(&neg_inf, &neg_inf));
        assert!(!GenericFraction::eq(&neg_inf, &neg_one));
        assert!(!GenericFraction::eq(&neg_inf, &zero));
        assert!(!GenericFraction::eq(&neg_inf, &one));
        assert!(!GenericFraction::eq(&neg_inf, &inf));

        assert!(GenericFraction::eq(&neg_one, &neg_one));
        assert!(!GenericFraction::eq(&neg_one, &zero));
        assert!(!GenericFraction::eq(&neg_one, &one));
        assert!(!GenericFraction::eq(&neg_one, &inf));

        assert!(GenericFraction::eq(&zero, &zero));
        assert!(!GenericFraction::eq(&zero, &one));
        assert!(!GenericFraction::eq(&zero, &inf));

        assert!(GenericFraction::eq(&one, &one));
        assert!(!GenericFraction::eq(&one, &inf));

        assert!(GenericFraction::eq(&inf, &inf));

        assert!(!GenericFraction::ne(&nan, &nan));
        assert!(GenericFraction::ne(&nan, &neg_inf));
        assert!(GenericFraction::ne(&nan, &neg_one));
        assert!(GenericFraction::ne(&nan, &zero));
        assert!(GenericFraction::ne(&nan, &one));
        assert!(GenericFraction::ne(&nan, &inf));

        assert!(!GenericFraction::ne(&neg_inf, &neg_inf));
        assert!(GenericFraction::ne(&neg_inf, &neg_one));
        assert!(GenericFraction::ne(&neg_inf, &zero));
        assert!(GenericFraction::ne(&neg_inf, &one));
        assert!(GenericFraction::ne(&neg_inf, &inf));

        assert!(!GenericFraction::ne(&neg_one, &neg_one));
        assert!(GenericFraction::ne(&neg_one, &zero));
        assert!(GenericFraction::ne(&neg_one, &one));
        assert!(GenericFraction::ne(&neg_one, &inf));

        assert!(!GenericFraction::ne(&zero, &zero));
        assert!(GenericFraction::ne(&zero, &one));
        assert!(GenericFraction::ne(&zero, &inf));

        assert!(!GenericFraction::ne(&one, &one));
        assert!(GenericFraction::ne(&one, &inf));

        assert!(!GenericFraction::ne(&inf, &inf));
    }
}
