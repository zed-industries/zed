use CheckedSub;

use crate::decimal::GenericDecimal;
use crate::generic::GenericInteger;
use std::cmp;

impl<T, P> CheckedSub for GenericDecimal<T, P>
where
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
{
    fn checked_sub(&self, other: &Self) -> Option<Self> {
        match *self {
            GenericDecimal(ref sf, sp) => match *other {
                GenericDecimal(ref of, op) => {
                    CheckedSub::checked_sub(sf, of).map(|val| GenericDecimal(val, cmp::max(sp, op)))
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckedSub, GenericDecimal};
    use crate::{One, Zero};

    type F = GenericDecimal<u8, u8>;

    #[test]
    fn checked_sub() {
        assert_eq!(Some(F::nan()), F::nan().checked_sub(&F::nan()));

        assert_eq!(Some(F::nan()), F::infinity().checked_sub(&F::nan()));
        assert_eq!(Some(F::nan()), F::infinity().checked_sub(&F::infinity()));
        assert_eq!(
            Some(F::infinity()),
            F::infinity().checked_sub(&F::neg_infinity())
        );
        assert_eq!(Some(F::infinity()), F::infinity().checked_sub(&F::one()));

        assert_eq!(Some(F::nan()), F::one().checked_sub(&F::nan()));
        assert_eq!(
            Some(F::neg_infinity()),
            F::one().checked_sub(&F::infinity())
        );
        assert_eq!(
            Some(F::infinity()),
            F::one().checked_sub(&F::neg_infinity())
        );

        assert_eq!(Some(F::zero()), F::one().checked_sub(&F::one()));
        assert_eq!(Some(F::from(2)), F::one().checked_sub(&(-F::one())));
        assert_eq!(Some(-F::from(2)), (-F::one()).checked_sub(&F::one()));
        assert_eq!(Some(F::zero()), (-F::one()).checked_sub(&(-F::one())));

        assert_eq!(Some(-F::one()), F::one().checked_sub(&F::from(2)));
        assert_eq!(Some(F::one()), (-F::one()).checked_sub(&(-F::from(2))));
    }
}
