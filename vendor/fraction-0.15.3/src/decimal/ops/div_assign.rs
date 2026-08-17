use crate::{decimal::GenericDecimal, generic::GenericInteger};

use std::cmp;
use std::ops::DivAssign;

impl<O, T, P> DivAssign<O> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    O: Into<Self>,
{
    fn div_assign(&mut self, other: O) {
        let other: Self = other.into();

        match *self {
            GenericDecimal(ref mut sf, ref mut sp) => match other {
                GenericDecimal(of, op) => {
                    DivAssign::div_assign(sf, of);
                    *sp = cmp::max(*sp, op);
                }
            },
        };
    }
}

impl<'a, T, P> DivAssign<&'a Self> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + DivAssign,
    P: Copy + GenericInteger + Into<usize>,
{
    fn div_assign(&mut self, other: &'a Self) {
        match *self {
            GenericDecimal(ref mut sf, ref mut sp) => match other {
                GenericDecimal(of, op) => {
                    DivAssign::div_assign(sf, of);
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
            v /= -F::one();
            assert_eq!(v, -F::one());
        }

        {
            let mut v = -F::one();
            v /= F::from(2);
            assert_eq!(v, -F::from(0.5));
        }

        {
            let mut v = -F::one();
            v /= -F::one();
            assert_eq!(v, F::one());
        }

        {
            let mut v = -F::one();
            v /= -F::from(2);
            assert_eq!(v, F::from(0.5));
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
            v /= &(-F::one());
            assert_eq!(v, -F::one());
        }

        {
            let mut v = -F::one();
            v /= &F::from(2);
            assert_eq!(v, -F::from(0.5));
        }

        {
            let mut v = -F::one();
            v /= &(-F::one());
            assert_eq!(v, F::one());
        }

        {
            let mut v = -F::one();
            v /= &(-F::from(2));
            assert_eq!(v, F::from(0.5));
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
