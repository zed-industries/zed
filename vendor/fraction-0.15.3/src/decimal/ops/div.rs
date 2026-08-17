use crate::decimal::GenericDecimal;
use crate::generic::GenericInteger;
use std::cmp;
use std::ops::Div;

impl<O, T, P> Div<O> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + Div,
    P: Copy + GenericInteger + Into<usize>,
    O: Into<Self>,
{
    type Output = Self;

    fn div(self, other: O) -> Self::Output {
        let other: Self = other.into();

        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Div::div(sf, of), cmp::max(sp, op)),
            },
        }
    }
}

impl<'a, T, P> Div for &'a GenericDecimal<T, P>
where
    T: Clone + GenericInteger + Div,
    P: Copy + GenericInteger + Into<usize>,
    &'a T: Div<Output = T>,
{
    type Output = GenericDecimal<T, P>;

    fn div(self, other: Self) -> Self::Output {
        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Div::div(sf, of), cmp::max(*sp, *op)),
            },
        }
    }
}

impl<'a, O, T, P> Div<O> for &'a GenericDecimal<T, P>
where
    T: Clone + GenericInteger + Div,
    P: Copy + GenericInteger + Into<usize>,
    &'a T: Div<Output = T>,
    O: Into<GenericDecimal<T, P>>,
{
    type Output = GenericDecimal<T, P>;

    fn div(self, other: O) -> Self::Output {
        let other: GenericDecimal<T, P> = other.into();

        match self {
            GenericDecimal(sf, sp) => match other {
                GenericDecimal(of, op) => GenericDecimal(Div::div(sf, of), cmp::max(*sp, op)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GenericDecimal;
    type F = GenericDecimal<u8, u8>;

    #[test]
    fn div_scalar() {
        assert_eq!(F::from(3) / 2, F::from(1.5));
        assert_eq!(F::from(3) / 1.5, F::from(2));

        assert_eq!(&F::from(3) / 2, F::from(1.5));
        assert_eq!(&F::from(3) / 1.5, F::from(2));

        assert_eq!(F::from(3) / F::from(2), F::from(1.5));
        assert_eq!(F::from(3) / F::from(1.5), F::from(2));

        assert_eq!(&F::from(3) / &F::from(2), F::from(1.5));
        assert_eq!(&F::from(3) / &F::from(1.5), F::from(2));
    }
}
