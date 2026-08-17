use std::cmp;
use std::ops::{AddAssign, MulAssign};

use crate::{decimal::GenericDecimal, generic::GenericInteger};

impl<O, T, P> AddAssign<O> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + AddAssign + AddAssign + MulAssign,
    P: Copy + GenericInteger + Into<usize>,
    O: Into<GenericDecimal<T, P>>,
{
    fn add_assign(&mut self, other: O) {
        let other: GenericDecimal<T, P> = other.into();

        match *self {
            GenericDecimal(ref mut sf, ref mut sp) => match other {
                GenericDecimal(of, op) => {
                    AddAssign::add_assign(sf, of);
                    *sp = cmp::max(*sp, op);
                }
            },
        };
    }
}

impl<'a, T, P> AddAssign<&'a Self> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + AddAssign,
    P: Copy + GenericInteger + Into<usize>,
{
    fn add_assign(&mut self, other: &'a Self) {
        match *self {
            GenericDecimal(ref mut sf, ref mut sp) => match other {
                GenericDecimal(of, op) => {
                    AddAssign::add_assign(sf, of);
                    *sp = cmp::max(*sp, *op);
                }
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::GenericDecimal;
    use crate::{One, Zero};

    type F = GenericDecimal<u8, u8>;

    #[test]
    fn owned() {
        {
            let mut v = F::zero();
            v += F::infinity();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::infinity();
            v += F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v += F::infinity();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::infinity();
            v += F::neg_infinity();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v += F::one();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::one();
            v += -F::one();
            assert_eq!(v, F::zero());
        }

        {
            let mut v = F::one();
            v += -F::from(2);
            assert_eq!(v, -F::one());
        }

        {
            let mut v = -F::one();
            v += -F::one();
            assert_eq!(v, -F::from(2));
        }

        {
            let mut v = -F::one();
            v += F::one();
            assert_eq!(v, F::zero());
        }

        {
            let mut v = F::one();
            v += -F::one();
            assert_eq!(v, F::zero());
        }

        {
            let mut v = -F::from(2);
            v += F::one();
            assert_eq!(v, -F::one());
        }
    }

    #[test]
    fn refs() {
        {
            let mut v = F::zero();
            v += &F::infinity();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::infinity();
            v += &F::nan();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v += &F::infinity();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::infinity();
            v += &F::neg_infinity();
            assert_eq!(v, F::nan());
        }

        {
            let mut v = F::infinity();
            v += &F::one();
            assert_eq!(v, F::infinity());
        }

        {
            let mut v = F::one();
            v += &(-F::one());
            assert_eq!(v, F::zero());
        }

        {
            let mut v = F::one();
            v += &(-F::from(2));
            assert_eq!(v, -F::one());
        }

        {
            let mut v = -F::one();
            v += &(-F::one());
            assert_eq!(v, -F::from(2));
        }

        {
            let mut v = -F::one();
            v += &F::one();
            assert_eq!(v, F::zero());
        }

        {
            let mut v = -F::from(2);
            v += &F::one();
            assert_eq!(v, -F::one());
        }
    }

    #[test]
    fn cast() {
        {
            let mut v = F::zero();
            v += 1;
            assert_eq!(v, F::one());
        }

        {
            let mut v = F::zero();
            v += 1;
            assert_eq!(v, F::one());
        }
    }
}
