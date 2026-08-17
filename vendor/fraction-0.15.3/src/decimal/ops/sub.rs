use crate::generic::GenericInteger;
use crate::GenericDecimal;
use std::cmp;
use std::ops::Sub;

impl<O, T, P> Sub<O> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    O: Into<GenericDecimal<T, P>>,
{
    type Output = Self;

    fn sub(self, other: O) -> Self::Output {
        let other: GenericDecimal<T, P> = other.into();

        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Sub::sub(sf, of), cmp::max(sp, op)),
            },
        }
    }
}

impl<'a, T, P> Sub for &'a GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    &'a T: Sub<Output = T>,
{
    type Output = GenericDecimal<T, P>;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Sub::sub(sf, of), cmp::max(*sp, *op)),
            },
        }
    }
}

impl<'a, O, T, P> Sub<O> for &'a GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    &'a T: Sub<Output = T>,
    O: Into<GenericDecimal<T, P>>,
{
    type Output = GenericDecimal<T, P>;

    fn sub(self, other: O) -> Self::Output {
        let other: GenericDecimal<T, P> = other.into();

        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Sub::sub(sf, of), cmp::max(*sp, op)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{GenericDecimal, One, Zero};

    type F = GenericDecimal<u8, u8>;

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
