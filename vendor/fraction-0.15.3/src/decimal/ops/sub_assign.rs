use std::cmp;
use std::ops::SubAssign;

use crate::{decimal::GenericDecimal, generic::GenericInteger};

impl<O, T, P> SubAssign<O> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    O: Into<Self>,
{
    fn sub_assign(&mut self, other: O) {
        let other: Self = other.into();

        match *self {
            GenericDecimal(ref mut sf, ref mut sp) => match other {
                GenericDecimal(of, op) => {
                    SubAssign::sub_assign(sf, of);
                    *sp = cmp::max(*sp, op);
                }
            },
        };
    }
}

impl<'a, T, P> SubAssign<&'a Self> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger, // + $trait + $trait<&'a T>,
    P: Copy + GenericInteger + Into<usize>,
{
    fn sub_assign(&mut self, other: &'a Self) {
        match *self {
            GenericDecimal(ref mut sf, ref mut sp) => match other {
                GenericDecimal(of, op) => {
                    SubAssign::sub_assign(sf, of);
                    *sp = cmp::max(*sp, *op);
                }
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{GenericDecimal, One, Zero};

    type F = GenericDecimal<u8, u8>;

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
            v -= -F::one();
            assert_eq!(v, F::from(2));
        }

        {
            let mut v = -F::one();
            v -= F::from(2);
            assert_eq!(v, -F::from(3));
        }

        {
            let mut v = -F::one();
            v -= -F::one();
            assert_eq!(v, F::zero());
        }

        {
            let mut v = -F::one();
            v -= -F::from(2);
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
            v -= &(-F::one());
            assert_eq!(v, F::from(2));
        }

        {
            let mut v = -F::one();
            v -= &F::from(2);
            assert_eq!(v, -F::from(3));
        }

        {
            let mut v = -F::one();
            v -= &(-F::one());
            assert_eq!(v, F::zero());
        }

        {
            let mut v = -F::one();
            v -= &(-F::from(2));
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
