use CheckedDiv;

use crate::decimal::GenericDecimal;
use crate::generic::GenericInteger;
use std::cmp;

impl<T, P> CheckedDiv for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
{
    fn checked_div(&self, other: &Self) -> Option<Self> {
        match *self {
            GenericDecimal(ref sf, sp) => match *other {
                GenericDecimal(ref of, op) => {
                    CheckedDiv::checked_div(sf, of).map(|val| GenericDecimal(val, cmp::max(sp, op)))
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckedDiv, GenericDecimal};
    use crate::{One, Zero};

    type F = GenericDecimal<u8, u8>;

    #[test]
    fn checked_div() {
        assert_eq!(Some(F::nan()), F::nan().checked_div(&F::nan()));

        assert_eq!(Some(F::nan()), F::infinity().checked_div(&F::nan()));
        assert_eq!(Some(F::nan()), F::infinity().checked_div(&F::infinity()));
        assert_eq!(
            Some(F::nan()),
            F::infinity().checked_div(&F::neg_infinity())
        );
        assert_eq!(Some(F::infinity()), F::infinity().checked_div(&F::one()));
        assert_eq!(
            Some(F::neg_infinity()),
            F::infinity().checked_div(&(-F::one()))
        );

        assert_eq!(Some(F::infinity()), F::infinity().checked_div(&F::zero()));
        assert_eq!(Some(F::zero()), F::zero().checked_div(&F::infinity()));

        assert_eq!(
            Some(F::neg_infinity()),
            F::infinity().checked_div(&(-F::one()))
        );

        assert_eq!(Some(F::nan()), F::one().checked_div(&F::nan()));
        assert_eq!(Some(F::zero()), F::one().checked_div(&F::infinity()));
        assert_eq!(Some(F::zero()), F::one().checked_div(&F::neg_infinity()));

        assert_eq!(Some(F::one()), F::one().checked_div(&F::one()));
        assert_eq!(Some(-F::one()), F::one().checked_div(&(-F::one())));
        assert_eq!(Some(-F::one()), (-F::one()).checked_div(&F::one()));
        assert_eq!(Some(F::one()), (-F::one()).checked_div(&(-F::one())));

        assert_eq!(Some(F::from(0.5)), F::one().checked_div(&F::from(2)));
        assert_eq!(Some(F::from(0.5)), (-F::one()).checked_div(&(-F::from(2))));

        assert_eq!(Some(F::infinity()), F::one().checked_div(&F::zero()));
        assert_eq!(Some(F::neg_infinity()), (-F::one()).checked_div(&F::zero()));
        assert_eq!(Some(F::zero()), F::zero().checked_div(&F::one()));
        assert_eq!(Some(F::zero()), F::zero().checked_div(&(-F::one())));

        assert_eq!(Some(F::nan()), F::zero().checked_div(&F::zero()));
    }
}
