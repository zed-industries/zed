use crate::fraction::{GenericFraction, Sign};
use crate::{Integer, Ratio};
use std::mem;
use std::ops::{Add, Sub, SubAssign};

impl<T, O> SubAssign<O> for GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    fn sub_assign(&mut self, other: O) {
        let other = other.into();

        *self = match *self {
            GenericFraction::NaN => GenericFraction::NaN,
            GenericFraction::Infinity(ls) => match other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(rs) => {
                    if ls == rs {
                        GenericFraction::NaN
                    } else {
                        GenericFraction::Infinity(ls)
                    }
                }
                GenericFraction::Rational(_, _) => GenericFraction::Infinity(ls),
            },
            GenericFraction::Rational(ls, ref mut l) => match other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(s) => GenericFraction::Infinity(-s),
                GenericFraction::Rational(rs, r) => {
                    let l_ = mem::replace(l, Ratio::new_raw(T::zero(), T::zero()));

                    if ls == Sign::Plus && rs == Sign::Plus {
                        if r > l_ {
                            GenericFraction::Rational(Sign::Minus, r.sub(l_))
                        } else {
                            GenericFraction::Rational(Sign::Plus, l_.sub(r))
                        }
                    } else if ls == Sign::Plus {
                        GenericFraction::Rational(Sign::Plus, l_.add(r))
                    } else if rs == Sign::Plus {
                        GenericFraction::Rational(Sign::Minus, l_.add(r))
                    } else if l_ > r {
                        GenericFraction::Rational(Sign::Minus, l_.sub(r))
                    } else {
                        GenericFraction::Rational(Sign::Plus, r.sub(l_))
                    }
                }
            },
        };
    }
}

impl<'a, T> SubAssign<&'a GenericFraction<T>> for GenericFraction<T>
where
    T: Clone + Integer,
{
    fn sub_assign(&mut self, other: &'a Self) {
        *self = match *self {
            GenericFraction::NaN => GenericFraction::NaN,
            GenericFraction::Infinity(ls) => match *other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(rs) => {
                    if ls == rs {
                        GenericFraction::NaN
                    } else {
                        GenericFraction::Infinity(ls)
                    }
                }
                GenericFraction::Rational(_, _) => GenericFraction::Infinity(ls),
            },
            GenericFraction::Rational(ls, ref mut l) => match *other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(s) => GenericFraction::Infinity(-s),
                GenericFraction::Rational(rs, ref r) => {
                    let l_ = mem::replace(l, Ratio::new_raw(T::zero(), T::zero()));

                    if ls == Sign::Plus && rs == Sign::Plus {
                        if l_ < *r {
                            GenericFraction::Rational(Sign::Minus, r.sub(l_))
                        } else {
                            GenericFraction::Rational(Sign::Plus, l_.sub(r))
                        }
                    } else if ls == Sign::Plus {
                        GenericFraction::Rational(Sign::Plus, l_.add(r))
                    } else if rs == Sign::Plus {
                        GenericFraction::Rational(Sign::Minus, l_.add(r))
                    } else if l_ > *r {
                        GenericFraction::Rational(Sign::Minus, l_.sub(r))
                    } else {
                        GenericFraction::Rational(Sign::Plus, r.sub(l_))
                    }
                }
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::GenericFraction;
    use crate::{One, Zero};

    type F = GenericFraction<u8>;

    #[test]
    fn owned() {
        {
            let mut v = F::nan();
            v -= F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v -= F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v -= F::infinity();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v -= F::neg_infinity();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::infinity();
            v -= F::one();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::one();
            v -= F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::one();
            v -= F::infinity();
            assert_eq!(v, F::neg_infinity());
        }

        {
            let mut v = F::one();
            v -= F::neg_infinity();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::one();
            v -= F::new_neg(1, 1);
            assert_eq!(v, F::new(2, 1));
        }

        {
            let mut v = F::new_neg(1, 1);
            v -= F::new(2, 1);
            assert_eq!(v, F::new_neg(3, 1));
        }

        {
            let mut v = F::new_neg(1, 1);
            v -= F::new_neg(1, 1);
            assert_eq!(v, F::zero());
        }

        {
            let mut v = F::new_neg(1, 1);
            v -= F::new_neg(2, 1);
            assert_eq!(v, F::one());
        }
    }

    #[test]
    fn refs() {
        {
            let mut v = F::nan();
            v -= &F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v -= &F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v -= &F::infinity();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v -= &F::neg_infinity();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::infinity();
            v -= &F::one();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::one();
            v -= &F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::one();
            v -= &F::infinity();
            assert_eq!(v, F::neg_infinity());
        }

        {
            let mut v = F::one();
            v -= &F::neg_infinity();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::one();
            v -= &F::new_neg(1, 1);
            assert_eq!(v, F::new(2, 1));
        }

        {
            let mut v = F::new_neg(1, 1);
            v -= &F::new(2, 1);
            assert_eq!(v, F::new_neg(3, 1));
        }

        {
            let mut v = F::new_neg(1, 1);
            v -= &F::new_neg(1, 1);
            assert_eq!(v, F::zero());
        }

        {
            let mut v = F::new_neg(1, 1);
            v -= &F::new_neg(2, 1);
            assert_eq!(v, F::one());
        }
    }

    #[test]
    fn cast() {
        {
            let mut v = F::one();
            v -= 1;
            assert_eq!(v, F::zero());
        }

        {
            let mut v = F::one();
            v -= 1;
            assert_eq!(v, F::zero());
        }
    }
}
