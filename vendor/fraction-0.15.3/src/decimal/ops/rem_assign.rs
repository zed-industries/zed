use std::cmp;
use std::ops::RemAssign;

use crate::{decimal::GenericDecimal, generic::GenericInteger};

// impl<T, P> RemAssign for GenericDecimal<T, P>
impl<O, T, P> RemAssign<O> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    O: Into<Self>,
{
    fn rem_assign(&mut self, other: O) {
        let other: Self = other.into();

        match *self {
            GenericDecimal(ref mut sf, ref mut sp) => match other {
                GenericDecimal(of, op) => {
                    RemAssign::rem_assign(sf, of);
                    *sp = cmp::max(*sp, op);
                }
            },
        };
    }
}

impl<'a, T, P> RemAssign<&'a Self> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
{
    fn rem_assign(&mut self, other: &'a Self) {
        match *self {
            GenericDecimal(ref mut sf, ref mut sp) => match other {
                GenericDecimal(of, op) => {
                    RemAssign::rem_assign(sf, of);
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
