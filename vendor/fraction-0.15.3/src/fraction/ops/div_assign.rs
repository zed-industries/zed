use crate::fraction::{GenericFraction, Sign};
use crate::{Integer, Ratio, Zero};
use std::mem;
use std::ops::{Div, DivAssign};

impl<T, O> DivAssign<O> for GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    fn div_assign(&mut self, other: O) {
        let other = other.into();

        *self = match *self {
            GenericFraction::NaN => GenericFraction::NaN,
            GenericFraction::Infinity(ls) => match other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(_) => GenericFraction::NaN,
                GenericFraction::Rational(rs, _) => {
                    GenericFraction::Infinity(if ls == rs { Sign::Plus } else { Sign::Minus })
                }
            },
            GenericFraction::Rational(ls, ref mut l) => match other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(_) => {
                    GenericFraction::Rational(Sign::Plus, Ratio::zero())
                }
                GenericFraction::Rational(rs, r) => {
                    let l_ = mem::replace(l, Ratio::new_raw(T::zero(), T::zero()));

                    if l_.is_zero() && r.is_zero() {
                        GenericFraction::NaN
                    } else if r.is_zero() {
                        GenericFraction::Infinity(ls)
                    } else if l_.is_zero() {
                        GenericFraction::Rational(Sign::Plus, l_)
                    } else {
                        GenericFraction::Rational(
                            if ls == rs { Sign::Plus } else { Sign::Minus },
                            l_.div(r),
                        )
                    }
                }
            },
        };
    }
}

impl<'a, T> DivAssign<&'a GenericFraction<T>> for GenericFraction<T>
where
    T: Clone + Integer,
{
    fn div_assign(&mut self, other: &'a Self) {
        *self = match *self {
            GenericFraction::NaN => GenericFraction::NaN,
            GenericFraction::Infinity(ls) => match *other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(_) => GenericFraction::NaN,
                GenericFraction::Rational(rs, _) => {
                    GenericFraction::Infinity(if ls == rs { Sign::Plus } else { Sign::Minus })
                }
            },
            GenericFraction::Rational(ls, ref mut l) => match *other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(_) => {
                    GenericFraction::Rational(Sign::Plus, Ratio::zero())
                }
                GenericFraction::Rational(rs, ref r) => {
                    let l_ = mem::replace(l, Ratio::new_raw(T::zero(), T::zero()));

                    if l_.is_zero() && r.is_zero() {
                        GenericFraction::NaN
                    } else if r.is_zero() {
                        GenericFraction::Infinity(ls)
                    } else if l_.is_zero() {
                        GenericFraction::Rational(Sign::Plus, l_)
                    } else {
                        GenericFraction::Rational(
                            if ls == rs { Sign::Plus } else { Sign::Minus },
                            l_.div(r),
                        )
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
            v /= F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v /= F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v /= F::infinity();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v /= F::neg_infinity();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v /= F::one();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::one();
            v /= F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::one();
            v /= F::infinity();
            assert_eq!(v, F::zero());
        }

        {
            let mut v = F::one();
            v /= F::neg_infinity();
            assert_eq!(v, F::zero());
        }

        {
            let mut v = F::one();
            v /= F::new_neg(1, 1);
            assert_eq!(v, F::new_neg(1, 1));
        }

        {
            let mut v = F::new_neg(1, 1);
            v /= F::new(2, 1);
            assert_eq!(v, F::new_neg(1, 2));
        }

        {
            let mut v = F::new_neg(1, 1);
            v /= F::new_neg(1, 1);
            assert_eq!(v, F::one());
        }

        {
            let mut v = F::new_neg(1, 1);
            v /= F::new_neg(2, 1);
            assert_eq!(v, F::new(1, 2));
        }

        {
            let mut v = F::infinity();
            v /= F::zero();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::zero();
            v /= F::infinity();
            assert_eq!(v, F::zero());
        }

        {
            let mut v = F::one();
            v /= F::zero();
            assert_eq!(v, F::infinity());
        }
    }

    #[test]
    fn refs() {
        {
            let mut v = F::nan();
            v /= &F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v /= &F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v /= &F::infinity();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v /= &F::neg_infinity();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v /= &F::one();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::one();
            v /= &F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::one();
            v /= &F::infinity();
            assert_eq!(v, F::zero());
        }

        {
            let mut v = F::one();
            v /= &F::neg_infinity();
            assert_eq!(v, F::zero());
        }

        {
            let mut v = F::one();
            v /= &F::new_neg(1, 1);
            assert_eq!(v, F::new_neg(1, 1));
        }

        {
            let mut v = F::new_neg(1, 1);
            v /= &F::new(2, 1);
            assert_eq!(v, F::new_neg(1, 2));
        }

        {
            let mut v = F::new_neg(1, 1);
            v /= &F::new_neg(1, 1);
            assert_eq!(v, F::one());
        }

        {
            let mut v = F::new_neg(1, 1);
            v /= &F::new_neg(2, 1);
            assert_eq!(v, F::new(1, 2));
        }

        {
            let mut v = F::infinity();
            v /= &F::zero();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::zero();
            v /= &F::infinity();
            assert_eq!(v, F::zero());
        }

        {
            let mut v = F::one();
            v /= &F::zero();
            assert_eq!(v, F::infinity());
        }
    }

    #[test]
    fn cast() {
        {
            let mut v = F::from(3);
            v /= 1.5;
            assert_eq!(v, F::from(2));
        }

        {
            let mut v = F::from(3);
            v /= 2;
            assert_eq!(v, F::from(1.5));
        }
    }
}
