use crate::fraction::{GenericFraction, Sign};
use crate::{Integer, Zero};
use std::ops::Mul;

impl<T, O> Mul<O> for GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    type Output = Self;

    fn mul(self, other: O) -> Self {
        let other = other.into();
        match self {
            GenericFraction::NaN => self,
            GenericFraction::Infinity(sign) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(osign) => GenericFraction::Infinity(if sign == osign {
                    Sign::Plus
                } else {
                    Sign::Minus
                }),
                GenericFraction::Rational(osign, l) => {
                    if l.is_zero() {
                        GenericFraction::NaN
                    } else {
                        GenericFraction::Infinity(if sign == osign {
                            Sign::Plus
                        } else {
                            Sign::Minus
                        })
                    }
                }
            },
            GenericFraction::Rational(sign, l) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(osign) => {
                    if l.is_zero() {
                        GenericFraction::NaN
                    } else {
                        GenericFraction::Infinity(if sign == osign {
                            Sign::Plus
                        } else {
                            Sign::Minus
                        })
                    }
                }
                GenericFraction::Rational(osign, r) => {
                    let s = if l.is_zero() || r.is_zero() || sign == osign {
                        Sign::Plus
                    } else {
                        Sign::Minus
                    };
                    GenericFraction::Rational(s, l.mul(r))
                }
            },
        }
    }
}

impl<'a, T, O> Mul<O> for &'a GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    type Output = GenericFraction<T>;

    fn mul(self, other: O) -> GenericFraction<T> {
        let other = other.into();

        match *self {
            GenericFraction::NaN => self.clone(),
            GenericFraction::Infinity(sign) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(osign) => GenericFraction::Infinity(if sign == osign {
                    Sign::Plus
                } else {
                    Sign::Minus
                }),
                GenericFraction::Rational(osign, ref l) => {
                    if l.is_zero() {
                        GenericFraction::NaN
                    } else {
                        GenericFraction::Infinity(if sign == osign {
                            Sign::Plus
                        } else {
                            Sign::Minus
                        })
                    }
                }
            },
            GenericFraction::Rational(sign, ref l) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(osign) => {
                    if l.is_zero() {
                        GenericFraction::NaN
                    } else {
                        GenericFraction::Infinity(if sign == osign {
                            Sign::Plus
                        } else {
                            Sign::Minus
                        })
                    }
                }
                GenericFraction::Rational(osign, ref r) => {
                    let s = if l.is_zero() || r.is_zero() || sign == osign {
                        Sign::Plus
                    } else {
                        Sign::Minus
                    };
                    GenericFraction::Rational(s, l.mul(r))
                }
            },
        }
    }
}

impl<'a, T> Mul for &'a GenericFraction<T>
where
    T: Clone + Integer,
{
    type Output = GenericFraction<T>;

    fn mul(self, other: Self) -> GenericFraction<T> {
        match *self {
            GenericFraction::NaN => self.clone(),
            GenericFraction::Infinity(sign) => match *other {
                GenericFraction::NaN => other.clone(),
                GenericFraction::Infinity(osign) => GenericFraction::Infinity(if sign == osign {
                    Sign::Plus
                } else {
                    Sign::Minus
                }),
                GenericFraction::Rational(osign, ref l) => {
                    if l.is_zero() {
                        GenericFraction::NaN
                    } else {
                        GenericFraction::Infinity(if sign == osign {
                            Sign::Plus
                        } else {
                            Sign::Minus
                        })
                    }
                }
            },
            GenericFraction::Rational(sign, ref l) => match *other {
                GenericFraction::NaN => other.clone(),
                GenericFraction::Infinity(osign) => {
                    if l.is_zero() {
                        GenericFraction::NaN
                    } else {
                        GenericFraction::Infinity(if sign == osign {
                            Sign::Plus
                        } else {
                            Sign::Minus
                        })
                    }
                }
                GenericFraction::Rational(osign, ref r) => {
                    let s = if l.is_zero() || r.is_zero() || sign == osign {
                        Sign::Plus
                    } else {
                        Sign::Minus
                    };
                    GenericFraction::Rational(s, l.mul(r))
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
        assert_eq!(F::from(1.5) * 2, F::from(3));
        assert_eq!(F::from(2) * 1.5, F::from(3));

        assert_eq!(&F::from(1.5) * 2, F::from(3));
        assert_eq!(&F::from(2) * 1.5, F::from(3));

        assert_eq!(F::from(1.5) * F::from(2), F::from(3));
        assert_eq!(F::from(2) * F::from(1.5), F::from(3));

        assert_eq!(&F::from(1.5) * &F::from(2), F::from(3));
        assert_eq!(&F::from(2) * &F::from(1.5), F::from(3));
    }
}
