use crate::fraction::{GenericFraction, Sign};
use crate::{Integer, Ratio, Zero};
use std::mem;
use std::ops::{Rem, RemAssign};

impl<T, O> RemAssign<O> for GenericFraction<T>
where
    T: Clone + Integer,
    O: Into<GenericFraction<T>>,
{
    fn rem_assign(&mut self, other: O) {
        let other = other.into();

        *self = match *self {
            GenericFraction::NaN => GenericFraction::NaN,
            GenericFraction::Infinity(_) => match other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(_) => GenericFraction::NaN,
                GenericFraction::Rational(_, _) => GenericFraction::NaN,
            },
            GenericFraction::Rational(ls, ref mut l) => match other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(_) => GenericFraction::Rational(
                    ls,
                    mem::replace(l, Ratio::new_raw(T::zero(), T::zero())),
                ),
                GenericFraction::Rational(_, r) => {
                    let l_ = mem::replace(l, Ratio::new_raw(T::zero(), T::zero()));

                    if r.is_zero() {
                        GenericFraction::NaN
                    } else if l_ == r {
                        GenericFraction::Rational(Sign::Plus, Ratio::zero())
                    } else {
                        GenericFraction::Rational(ls, l_.rem(r))
                    }
                }
            },
        };
    }
}

impl<'a, T> RemAssign<&'a GenericFraction<T>> for GenericFraction<T>
where
    T: Clone + Integer,
{
    fn rem_assign(&mut self, other: &'a Self) {
        *self = match *self {
            GenericFraction::NaN => GenericFraction::NaN,
            GenericFraction::Infinity(_) => match *other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(_) => GenericFraction::NaN,
                GenericFraction::Rational(_, _) => GenericFraction::NaN,
            },
            GenericFraction::Rational(ls, ref mut l) => match *other {
                GenericFraction::NaN => GenericFraction::NaN,
                GenericFraction::Infinity(_) => GenericFraction::Rational(
                    ls,
                    mem::replace(l, Ratio::new_raw(T::zero(), T::zero())),
                ),
                GenericFraction::Rational(_, ref r) => {
                    let l_ = mem::replace(l, Ratio::new_raw(T::zero(), T::zero()));

                    if r.is_zero() {
                        GenericFraction::NaN
                    } else if l_ == *r {
                        GenericFraction::Rational(Sign::Plus, Ratio::zero())
                    } else {
                        GenericFraction::Rational(ls, l_.rem(r))
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
            let mut v = F::infinity();
            v %= F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v %= F::infinity();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v %= F::one();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::one();
            v %= F::infinity();
            assert_eq!(v, F::one());
        }
    }

    #[test]
    fn refs() {
        {
            let mut v = F::infinity();
            v %= &F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v %= &F::infinity();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v %= &F::one();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::one();
            v %= &F::infinity();
            assert_eq!(v, F::one());
        }
    }

    #[test]
    fn cast() {
        {
            let mut v = F::from(4);
            v %= 1.5;
            assert_eq!(v, F::one());
        }

        {
            let mut v = F::from(4);
            v %= 2;
            assert_eq!(v, F::zero());
        }
    }
}
