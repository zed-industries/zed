use crate::generic::GenericInteger;
use crate::GenericDecimal;
use std::cmp;
use std::ops::Add;

impl<O, T, P> Add<O> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    O: Into<GenericDecimal<T, P>>,
{
    type Output = Self;

    fn add(self, other: O) -> Self::Output {
        let other: GenericDecimal<T, P> = other.into();

        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Add::add(sf, of), cmp::max(sp, op)),
            },
        }
    }
}

impl<'a, T, P> Add for &'a GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    &'a T: Add<Output = T>,
{
    type Output = GenericDecimal<T, P>;

    fn add(self, other: Self) -> Self::Output {
        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Add::add(sf, of), cmp::max(*sp, *op)),
            },
        }
    }
}

impl<'a, O, T, P> Add<O> for &'a GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    &'a T: Add<Output = T>,
    O: Into<GenericDecimal<T, P>>,
{
    type Output = GenericDecimal<T, P>;

    fn add(self, other: O) -> Self::Output {
        let other: GenericDecimal<T, P> = other.into();

        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Add::add(sf, of), cmp::max(*sp, op)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GenericDecimal;
    use crate::Zero;

    type F = GenericDecimal<u8, u8>;

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
