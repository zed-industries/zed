use crate::fraction::{GenericFraction, Sign};
use crate::{CheckedDiv, CheckedMul, Integer, Ratio, Zero};

impl<T> CheckedDiv for GenericFraction<T>
where
    T: Clone + Integer + CheckedDiv + CheckedMul,
{
    fn checked_div(&self, other: &Self) -> Option<GenericFraction<T>> {
        match *self {
            GenericFraction::NaN => Some(self.clone()),
            GenericFraction::Infinity(sign) => match *other {
                GenericFraction::NaN => Some(other.clone()),
                GenericFraction::Infinity(_) => Some(GenericFraction::NaN),
                GenericFraction::Rational(osign, _) => {
                    Some(GenericFraction::Infinity(if sign == osign {
                        Sign::Plus
                    } else {
                        Sign::Minus
                    }))
                }
            },
            GenericFraction::Rational(sign, ref l) => match *other {
                GenericFraction::NaN => Some(other.clone()),
                GenericFraction::Infinity(_) => {
                    Some(GenericFraction::Rational(Sign::Plus, Ratio::zero()))
                }
                GenericFraction::Rational(osign, ref r) => {
                    if l.is_zero() && r.is_zero() {
                        Some(GenericFraction::NaN)
                    } else if r.is_zero() {
                        Some(GenericFraction::Infinity(sign))
                    } else if l.is_zero() {
                        Some(GenericFraction::Rational(Sign::Plus, l.clone()))
                    } else {
                        l.checked_div(r).map(|value| {
                            GenericFraction::Rational(
                                if sign == osign {
                                    Sign::Plus
                                } else {
                                    Sign::Minus
                                },
                                value,
                            )
                        })
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckedDiv, GenericFraction};
    use crate::{One, Zero};

    type F = GenericFraction<u8>;

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
            F::infinity().checked_div(&F::new_neg(1, 1))
        );

        assert_eq!(Some(F::infinity()), F::infinity().checked_div(&F::zero()));
        assert_eq!(Some(F::zero()), F::zero().checked_div(&F::infinity()));

        assert_eq!(
            Some(F::neg_infinity()),
            F::infinity().checked_div(&F::new_neg(1, 1))
        );

        assert_eq!(Some(F::nan()), F::one().checked_div(&F::nan()));
        assert_eq!(Some(F::zero()), F::one().checked_div(&F::infinity()));
        assert_eq!(Some(F::zero()), F::one().checked_div(&F::neg_infinity()));

        assert_eq!(Some(F::one()), F::one().checked_div(&F::one()));
        assert_eq!(
            Some(F::new_neg(1, 1)),
            F::one().checked_div(&F::new_neg(1, 1))
        );
        assert_eq!(
            Some(F::new_neg(1, 1)),
            F::new_neg(1, 1).checked_div(&F::one())
        );
        assert_eq!(
            Some(F::one()),
            F::new_neg(1, 1).checked_div(&F::new_neg(1, 1))
        );

        assert_eq!(Some(F::new(1, 2)), F::one().checked_div(&F::new(2, 1)));
        assert_eq!(
            Some(F::new(1, 2)),
            F::new_neg(1, 1).checked_div(&F::new_neg(2, 1))
        );

        assert_eq!(Some(F::infinity()), F::one().checked_div(&F::zero()));
        assert_eq!(
            Some(F::neg_infinity()),
            F::new_neg(1, 1).checked_div(&F::zero())
        );
        assert_eq!(Some(F::zero()), F::zero().checked_div(&F::one()));
        assert_eq!(Some(F::zero()), F::zero().checked_div(&F::new_neg(1, 1)));

        assert_eq!(Some(F::nan()), F::zero().checked_div(&F::zero()));
    }
}
