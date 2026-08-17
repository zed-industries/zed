use crate::fraction::{GenericFraction, Sign};
use crate::{Integer, Ratio, Zero};
use std::ops::Rem;

impl<T, O> Rem<O> for GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    type Output = Self;

    fn rem(self, other: O) -> Self {
        let other = other.into();

        match self {
            GenericFraction::NaN => self,
            GenericFraction::Infinity(_) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(_) => GenericFraction::NaN,
                GenericFraction::Rational(_, _) => GenericFraction::NaN,
            },
            GenericFraction::Rational(sign, l) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(_) => GenericFraction::Rational(sign, l),
                GenericFraction::Rational(_, r) => {
                    if r.is_zero() {
                        GenericFraction::NaN
                    } else if l == r {
                        GenericFraction::Rational(Sign::Plus, Ratio::zero())
                    } else {
                        GenericFraction::Rational(sign, l % r)
                    }
                }
            },
        }
    }
}

impl<'a, T, O> Rem<O> for &'a GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    type Output = GenericFraction<T>;

    fn rem(self, other: O) -> GenericFraction<T> {
        let other = other.into();

        match *self {
            GenericFraction::NaN => self.clone(),
            GenericFraction::Infinity(_) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(_) => GenericFraction::NaN,
                GenericFraction::Rational(_, _) => GenericFraction::NaN,
            },
            GenericFraction::Rational(sign, ref l) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(_) => GenericFraction::Rational(sign, l.clone()),
                GenericFraction::Rational(_, ref r) => {
                    if r.is_zero() {
                        GenericFraction::NaN
                    } else if l == r {
                        GenericFraction::Rational(Sign::Plus, Ratio::zero())
                    } else {
                        GenericFraction::Rational(sign, l.rem(r))
                    }
                }
            },
        }
    }
}

impl<'a, T> Rem for &'a GenericFraction<T>
where
    T: Clone + Integer,
{
    type Output = GenericFraction<T>;

    fn rem(self, other: Self) -> GenericFraction<T> {
        match *self {
            GenericFraction::NaN => self.clone(),
            GenericFraction::Infinity(_) => match *other {
                GenericFraction::NaN => other.clone(),
                GenericFraction::Infinity(_) => GenericFraction::NaN,
                GenericFraction::Rational(_, _) => GenericFraction::NaN,
            },
            GenericFraction::Rational(sign, ref l) => match *other {
                GenericFraction::NaN => other.clone(),
                GenericFraction::Infinity(_) => GenericFraction::Rational(sign, l.clone()),
                GenericFraction::Rational(_, ref r) => {
                    if r.is_zero() {
                        GenericFraction::NaN
                    } else if l == r {
                        GenericFraction::Rational(Sign::Plus, Ratio::zero())
                    } else {
                        GenericFraction::Rational(sign, l.rem(r))
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GenericFraction;

    type F = GenericFraction<u8>;

    #[test]
    fn mul_scalar() {
        assert_eq!(F::from(3) % 2, F::from(1));
        assert_eq!(F::from(3) % 1.5, F::from(0));

        assert_eq!(&F::from(3) % 2, F::from(1));
        assert_eq!(&F::from(3) % 1.5, F::from(0));

        assert_eq!(F::from(3) % F::from(2), F::from(1));
        assert_eq!(F::from(3) % F::from(1.5), F::from(0));

        assert_eq!(&F::from(3) % &F::from(2), F::from(1));
        assert_eq!(&F::from(3) % &F::from(1.5), F::from(0));
    }
}
