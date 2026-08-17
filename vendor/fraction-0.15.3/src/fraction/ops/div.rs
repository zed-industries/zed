use crate::fraction::{GenericFraction, Sign};
use crate::{Integer, Ratio, Zero};
use std::ops::Div;

impl<T, O> Div<O> for GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    type Output = Self;

    fn div(self, other: O) -> Self {
        let other = other.into();

        match self {
            GenericFraction::NaN => self,
            GenericFraction::Infinity(sign) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(_) => GenericFraction::NaN,
                GenericFraction::Rational(osign, _) => {
                    GenericFraction::Infinity(if sign == osign {
                        Sign::Plus
                    } else {
                        Sign::Minus
                    })
                }
            },
            GenericFraction::Rational(sign, l) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(_) => {
                    GenericFraction::Rational(Sign::Plus, Ratio::zero())
                }
                GenericFraction::Rational(osign, r) => {
                    if l.is_zero() && r.is_zero() {
                        GenericFraction::NaN
                    } else if r.is_zero() {
                        GenericFraction::Infinity(sign)
                    } else if l.is_zero() {
                        GenericFraction::Rational(Sign::Plus, l)
                    } else {
                        GenericFraction::Rational(
                            if sign == osign {
                                Sign::Plus
                            } else {
                                Sign::Minus
                            },
                            l.div(r),
                        )
                    }
                }
            },
        }
    }
}

impl<'a, T, O> Div<O> for &'a GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    type Output = GenericFraction<T>;

    fn div(self, other: O) -> GenericFraction<T> {
        let other = other.into();
        match *self {
            GenericFraction::NaN => self.clone(),
            GenericFraction::Infinity(sign) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(_) => GenericFraction::NaN,
                GenericFraction::Rational(osign, _) => {
                    GenericFraction::Infinity(if sign == osign {
                        Sign::Plus
                    } else {
                        Sign::Minus
                    })
                }
            },
            GenericFraction::Rational(sign, ref l) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(_) => {
                    GenericFraction::Rational(Sign::Plus, Ratio::zero())
                }
                GenericFraction::Rational(osign, ref r) => {
                    if l.is_zero() && r.is_zero() {
                        GenericFraction::NaN
                    } else if r.is_zero() {
                        GenericFraction::Infinity(sign)
                    } else if l.is_zero() {
                        GenericFraction::Rational(Sign::Plus, l.clone())
                    } else {
                        GenericFraction::Rational(
                            if sign == osign {
                                Sign::Plus
                            } else {
                                Sign::Minus
                            },
                            l.div(r),
                        )
                    }
                }
            },
        }
    }
}

impl<'a, T> Div for &'a GenericFraction<T>
where
    T: Clone + Integer,
{
    type Output = GenericFraction<T>;

    fn div(self, other: Self) -> GenericFraction<T> {
        match *self {
            GenericFraction::NaN => self.clone(),
            GenericFraction::Infinity(sign) => match *other {
                GenericFraction::NaN => other.clone(),
                GenericFraction::Infinity(_) => GenericFraction::NaN,
                GenericFraction::Rational(osign, _) => {
                    GenericFraction::Infinity(if sign == osign {
                        Sign::Plus
                    } else {
                        Sign::Minus
                    })
                }
            },
            GenericFraction::Rational(sign, ref l) => match *other {
                GenericFraction::NaN => other.clone(),
                GenericFraction::Infinity(_) => {
                    GenericFraction::Rational(Sign::Plus, Ratio::zero())
                }
                GenericFraction::Rational(osign, ref r) => {
                    if l.is_zero() && r.is_zero() {
                        GenericFraction::NaN
                    } else if r.is_zero() {
                        GenericFraction::Infinity(sign)
                    } else if l.is_zero() {
                        GenericFraction::Rational(Sign::Plus, l.clone())
                    } else {
                        GenericFraction::Rational(
                            if sign == osign {
                                Sign::Plus
                            } else {
                                Sign::Minus
                            },
                            l.div(r),
                        )
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GenericFraction;
    // use crate::{One, Zero};

    type F = GenericFraction<u8>;

    #[test]
    fn div_scalar() {
        assert_eq!(F::from(3) / 2, F::from(1.5));
        assert_eq!(F::from(3) / 1.5, F::from(2));

        assert_eq!(&F::from(3) / 2, F::from(1.5));
        assert_eq!(&F::from(3) / 1.5, F::from(2));

        assert_eq!(F::from(3) / F::from(2), F::from(1.5));
        assert_eq!(F::from(3) / F::from(1.5), F::from(2));

        assert_eq!(&F::from(3) / &F::from(2), F::from(1.5));
        assert_eq!(&F::from(3) / &F::from(1.5), F::from(2));
    }
}
