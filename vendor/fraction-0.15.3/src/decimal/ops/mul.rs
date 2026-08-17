use crate::generic::GenericInteger;
use crate::GenericDecimal;
use std::cmp;
use std::ops::Mul;

impl<O, T, P> Mul<O> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    O: Into<GenericDecimal<T, P>>,
{
    type Output = Self;

    fn mul(self, other: O) -> Self::Output {
        let other: GenericDecimal<T, P> = other.into();

        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Mul::mul(sf, of), cmp::max(sp, op)),
            },
        }
    }
}

impl<'a, T, P> Mul for &'a GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    &'a T: Mul<Output = T>,
{
    type Output = GenericDecimal<T, P>;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Mul::mul(sf, of), cmp::max(*sp, *op)),
            },
        }
    }
}

impl<'a, O, T, P> Mul<O> for &'a GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
    &'a T: Mul<Output = T>,
    O: Into<GenericDecimal<T, P>>,
{
    type Output = GenericDecimal<T, P>;

    fn mul(self, other: O) -> Self::Output {
        let other: GenericDecimal<T, P> = other.into();

        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Mul::mul(sf, of), cmp::max(*sp, op)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GenericDecimal;

    type F = GenericDecimal<u8, u8>;

    #[test]
    fn mul_scalar() {
        assert_eq!(F::from(1.5) * 2, F::from(3));
        assert_eq!(F::from(2) * 1.5, F::from(3));

        assert_eq!(&F::from(1.5) * 2, F::from(3));
        assert_eq!(&F::from(2) * 1.5, F::from(3));

        assert_eq!(F::from(1.5) * F::from(2), F::from(3));
        assert_eq!(F::from(2) * F::from(1.5), F::from(3));

        assert_eq!(&F::from(1.5) * &F::from(2), F::from(3));
        assert_eq!(&F::from(2) * &F::from(1.5), F::from(3));
    }
}
