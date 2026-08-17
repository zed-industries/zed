use crate::fraction::{GenericFraction, Sign};
use crate::Integer;
use std::ops::{Add, Sub};

impl<T, O> Add<O> for GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    type Output = Self;

    fn add(self, other: O) -> Self {
        let other = other.into();

        match self {
            GenericFraction::NaN => self,
            GenericFraction::Infinity(sign) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Rational(_, _) => self,
                GenericFraction::Infinity(osign) => {
                    if sign != osign {
                        GenericFraction::NaN
                    } else {
                        self
                    }
                }
            },
            GenericFraction::Rational(ls, l) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(_) => other,
                GenericFraction::Rational(rs, r) => {
                    if ls == Sign::Plus && rs == Sign::Plus {
                        GenericFraction::Rational(Sign::Plus, l.add(r))
                    } else if ls == Sign::Plus {
                        if l < r {
                            GenericFraction::Rational(Sign::Minus, r.sub(l))
                        } else {
                            GenericFraction::Rational(Sign::Plus, l.sub(r))
                        }
                    } else if rs == Sign::Plus {
                        if r < l {
                            GenericFraction::Rational(Sign::Minus, l.sub(r))
                        } else {
                            GenericFraction::Rational(Sign::Plus, r.sub(l))
                        }
                    } else {
                        GenericFraction::Rational(Sign::Minus, l.add(r))
                    }
                }
            },
        }
    }
}

impl<'a, T, O> Add<O> for &'a GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    type Output = GenericFraction<T>;

    fn add(self, other: O) -> GenericFraction<T> {
        let other = other.into();

        match *self {
            GenericFraction::NaN => self.clone(),
            GenericFraction::Infinity(sign) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Rational(_, _) => self.clone(),
                GenericFraction::Infinity(osign) => {
                    if sign != osign {
                        GenericFraction::NaN
                    } else {
                        self.clone()
                    }
                }
            },
            GenericFraction::Rational(ls, ref l) => match other {
                GenericFraction::NaN => other,
                GenericFraction::Infinity(_) => other,
                GenericFraction::Rational(rs, ref r) => {
                    if ls == Sign::Plus && rs == Sign::Plus {
                        GenericFraction::Rational(Sign::Plus, l.add(r))
                    } else if ls == Sign::Plus {
                        if l < r {
                            GenericFraction::Rational(Sign::Minus, r.sub(l))
                        } else {
                            GenericFraction::Rational(Sign::Plus, l.sub(r))
                        }
                    } else if rs == Sign::Plus {
                        if r < l {
                            GenericFraction::Rational(Sign::Minus, l.sub(r))
                        } else {
                            GenericFraction::Rational(Sign::Plus, r.sub(l))
                        }
                    } else {
                        GenericFraction::Rational(Sign::Minus, l.add(r))
                    }
                }
            },
        }
    }
}

impl<'a, T> Add for &'a GenericFraction<T>
where
    T: Clone + Integer,
{
    type Output = GenericFraction<T>;

    fn add(self, other: Self) -> GenericFraction<T> {
        match *self {
            GenericFraction::NaN => self.clone(),
            GenericFraction::Infinity(sign) => match *other {
                GenericFraction::NaN => other.clone(),
                GenericFraction::Rational(_, _) => self.clone(),
                GenericFraction::Infinity(osign) => {
                    if sign != osign {
                        GenericFraction::NaN
                    } else {
                        self.clone()
                    }
                }
            },
            GenericFraction::Rational(ls, ref l) => match *other {
                GenericFraction::NaN => other.clone(),
                GenericFraction::Infinity(_) => other.clone(),
                GenericFraction::Rational(rs, ref r) => {
                    if ls == Sign::Plus && rs == Sign::Plus {
                        GenericFraction::Rational(Sign::Plus, l.add(r))
                    } else if ls == Sign::Plus {
                        if l < r {
                            GenericFraction::Rational(Sign::Minus, r.sub(l))
                        } else {
                            GenericFraction::Rational(Sign::Plus, l.sub(r))
                        }
                    } else if rs == Sign::Plus {
                        if r < l {
                            GenericFraction::Rational(Sign::Minus, l.sub(r))
                        } else {
                            GenericFraction::Rational(Sign::Plus, r.sub(l))
                        }
                    } else {
                        GenericFraction::Rational(Sign::Minus, l.add(r))
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GenericFraction;
    use crate::Zero;

    type F = GenericFraction<u8>;

    #[test]
    fn add_scalar() {
        assert_eq!(F::zero() + 1, F::from(1));
        assert_eq!(F::zero() + 1.5, F::from(1.5));

        assert_eq!(&F::zero() + 1, F::from(1));
        assert_eq!(&F::zero() + 1.5, F::from(1.5));

        assert_eq!(F::zero() + F::from(1), F::from(1));
        assert_eq!(F::zero() + F::from(1.5), F::from(1.5));

        assert_eq!(&F::zero() + F::from(1), F::from(1));
        assert_eq!(&F::zero() + F::from(1.5), F::from(1.5));
    }
}
