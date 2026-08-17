use crate::fraction::{GenericFraction, Sign};
use crate::Integer;
use std::ops::{Add, Sub};

impl<T, O> Sub<O> for GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    type Output = Self;

    fn sub(self, other: O) -> Self {
        let other = other.into();

        match self {
            GenericFraction::NaN => self,
            GenericFraction::Infinity(sign) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(osign) => {
                    if sign == osign {
                        GenericFraction::NaN
                    } else {
                        self
                    }
                }
                GenericFraction::Rational(_, _) => self,
            },
            GenericFraction::Rational(ls, l) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(sign) => GenericFraction::Infinity(-sign),
                GenericFraction::Rational(rs, r) => {
                    if ls == Sign::Plus && rs == Sign::Plus {
                        if l < r {
                            GenericFraction::Rational(Sign::Minus, r.sub(l))
                        } else {
                            GenericFraction::Rational(Sign::Plus, l.sub(r))
                        }
                    } else if ls == Sign::Plus {
                        GenericFraction::Rational(Sign::Plus, l.add(r))
                    } else if rs == Sign::Plus {
                        GenericFraction::Rational(Sign::Minus, l.add(r))
                    } else if l < r {
                        GenericFraction::Rational(Sign::Plus, r.sub(l))
                    } else {
                        GenericFraction::Rational(Sign::Minus, l.sub(r))
                    }
                }
            },
        }
    }
}

impl<'a, T, O> Sub<O> for &'a GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    type Output = GenericFraction<T>;

    fn sub(self, other: O) -> GenericFraction<T> {
        let other = other.into();

        match *self {
            GenericFraction::NaN => self.clone(),
            GenericFraction::Infinity(sign) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(osign) => {
                    if sign == osign {
                        GenericFraction::NaN
                    } else {
                        self.clone()
                    }
                }
                GenericFraction::Rational(_, _) => self.clone(),
            },
            GenericFraction::Rational(ls, ref l) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(sign) => GenericFraction::Infinity(-sign),
                GenericFraction::Rational(rs, ref r) => {
                    if ls == Sign::Plus && rs == Sign::Plus {
                        if l < r {
                            GenericFraction::Rational(Sign::Minus, r.sub(l))
                        } else {
                            GenericFraction::Rational(Sign::Plus, l.sub(r))
                        }
                    } else if ls == Sign::Plus {
                        GenericFraction::Rational(Sign::Plus, l.add(r))
                    } else if rs == Sign::Plus {
                        GenericFraction::Rational(Sign::Minus, l.add(r))
                    } else if l < r {
                        GenericFraction::Rational(Sign::Plus, r.sub(l))
                    } else {
                        GenericFraction::Rational(Sign::Minus, l.sub(r))
                    }
                }
            },
        }
    }
}

impl<'a, T> Sub for &'a GenericFraction<T>
where
    T: Clone + Integer,
{
    type Output = GenericFraction<T>;

    fn sub(self, other: Self) -> GenericFraction<T> {
        match *self {
            GenericFraction::NaN => self.clone(),
            GenericFraction::Infinity(sign) => match *other {
                GenericFraction::NaN => other.clone(),
                GenericFraction::Infinity(osign) => {
                    if sign == osign {
                        GenericFraction::NaN
                    } else {
                        self.clone()
                    }
                }
                GenericFraction::Rational(_, _) => self.clone(),
            },
            GenericFraction::Rational(ls, ref l) => match *other {
                GenericFraction::NaN => other.clone(),
                GenericFraction::Infinity(sign) => GenericFraction::Infinity(-sign),
                GenericFraction::Rational(rs, ref r) => {
                    if ls == Sign::Plus && rs == Sign::Plus {
                        if l < r {
                            GenericFraction::Rational(Sign::Minus, r.sub(l))
                        } else {
                            GenericFraction::Rational(Sign::Plus, l.sub(r))
                        }
                    } else if ls == Sign::Plus {
                        GenericFraction::Rational(Sign::Plus, l.add(r))
                    } else if rs == Sign::Plus {
                        GenericFraction::Rational(Sign::Minus, l.add(r))
                    } else if l < r {
                        GenericFraction::Rational(Sign::Plus, r.sub(l))
                    } else {
                        GenericFraction::Rational(Sign::Minus, l.sub(r))
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GenericFraction;
    use crate::{One, Zero};

    type F = GenericFraction<u8>;

    #[test]
    fn sub_scalar() {
        assert_eq!(F::one() - 1, F::zero());
        assert_eq!(F::one() - 0.5, F::from(0.5));

        assert_eq!(&F::one() - 1, F::zero());
        assert_eq!(&F::one() - 0.5, F::from(0.5));

        assert_eq!(F::one() - F::from(1), F::zero());
        assert_eq!(F::one() - F::from(0.5), F::from(0.5));

        assert_eq!(&F::one() - &F::from(1), F::zero());
        assert_eq!(&F::one() - &F::from(0.5), F::from(0.5));
    }
}
