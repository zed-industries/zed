use CheckedMul;

use crate::decimal::GenericDecimal;
use crate::generic::GenericInteger;
use std::cmp;

impl<T, P> CheckedMul for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
{
    fn checked_mul(&self, other: &Self) -> Option<Self> {
        match *self {
            GenericDecimal(ref sf, sp) => match *other {
                GenericDecimal(ref of, op) => {
                    CheckedMul::checked_mul(sf, of).map(|val| GenericDecimal(val, cmp::max(sp, op)))
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckedMul, GenericDecimal};
    use crate::{One, Zero};

    type F = GenericDecimal<u8, u8>;

    #[test]
    fn checked_mul() {
        assert_eq!(Some(F::nan()), F::nan().checked_mul(&F::nan()));

        assert_eq!(Some(F::nan()), F::infinity().checked_mul(&F::nan()));
        assert_eq!(
            Some(F::infinity()),
            F::infinity().checked_mul(&F::infinity())
        );
        assert_eq!(
            Some(F::neg_infinity()),
            F::infinity().checked_mul(&F::neg_infinity())
        );
        assert_eq!(Some(F::infinity()), F::infinity().checked_mul(&F::one()));

        assert_eq!(Some(F::nan()), F::infinity().checked_mul(&F::zero()));
        assert_eq!(Some(F::nan()), F::zero().checked_mul(&F::infinity()));

        assert_eq!(
            Some(F::neg_infinity()),
            F::infinity().checked_mul(&(-F::one()))
        );

        assert_eq!(Some(F::nan()), F::one().checked_mul(&F::nan()));
        assert_eq!(Some(F::infinity()), F::one().checked_mul(&F::infinity()));
        assert_eq!(
            Some(F::neg_infinity()),
            F::one().checked_mul(&F::neg_infinity())
        );

        assert_eq!(Some(F::one()), F::one().checked_mul(&F::one()));
        assert_eq!(Some(-F::one()), F::one().checked_mul(&(-F::one())));
        assert_eq!(Some(-F::one()), (-F::one()).checked_mul(&F::one()));
        assert_eq!(Some(F::one()), (-F::one()).checked_mul(&(-F::one())));

        assert_eq!(Some(F::from(2)), F::one().checked_mul(&F::from(2)));
        assert_eq!(Some(F::from(2)), (-F::one()).checked_mul(&(-F::from(2))));

        assert_eq!(Some(F::zero()), F::one().checked_mul(&F::zero()));
        assert_eq!(Some(F::zero()), (-F::one()).checked_mul(&F::zero()));
        assert_eq!(Some(F::zero()), F::zero().checked_mul(&F::one()));
        assert_eq!(Some(F::zero()), F::zero().checked_mul(&(-F::one())));
    }
}
