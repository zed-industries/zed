use crate::generic::GenericInteger;
use crate::GenericDecimal;
use std::cmp;
use std::ops::Rem;

impl<O, T, P> Rem<O> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    O: Into<GenericDecimal<T, P>>,
{
    type Output = Self;

    fn rem(self, other: O) -> Self::Output {
        let other: GenericDecimal<T, P> = other.into();

        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Rem::rem(sf, of), cmp::max(sp, op)),
            },
        }
    }
}

impl<'a, T, P> Rem for &'a GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    &'a T: Rem<Output = T>,
{
    type Output = GenericDecimal<T, P>;

    fn rem(self, other: Self) -> Self::Output {
        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Rem::rem(sf, of), cmp::max(*sp, *op)),
            },
        }
    }
}

impl<'a, O, T, P> Rem<O> for &'a GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    &'a T: Rem<Output = T>,
    O: Into<GenericDecimal<T, P>>,
{
    type Output = GenericDecimal<T, P>;

    fn rem(self, other: O) -> Self::Output {
        let other: GenericDecimal<T, P> = other.into();

        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Rem::rem(sf, of), cmp::max(*sp, op)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GenericDecimal;

    type F = GenericDecimal<u8, u8>;

    #[test]
    fn rem_scalar() {
        assert_eq!(F::from(3) % 2, F::from(1));
        assert_eq!(F::from(3) % 1.5, F::from(0));

        assert_eq!(&F::from(3) % 2, F::from(1));
        assert_eq!(&F::from(3) % 1.5, F::from(0));

        assert_eq!(F::from(3) % F::from(2), F::from(1));
        assert_eq!(F::from(3) % F::from(1.5), F::from(0));

        assert_eq!(&F::from(3) % &F::from(2), F::from(1));
        assert_eq!(&F::from(3) % &F::from(1.5), F::from(0));
    }
}
