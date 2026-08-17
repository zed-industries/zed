use crate::fraction::{GenericFraction, Sign};
use crate::{CheckedMul, Integer, Zero};

impl<T> CheckedMul for GenericFraction<T>
where
    T: Clone + Integer + CheckedMul,
{
    fn checked_mul(&self, other: &Self) -> Option<GenericFraction<T>> {
        match *self {
            GenericFraction::NaN => Some(self.clone()),
            GenericFraction::Infinity(sign) => match *other {
                GenericFraction::NaN => Some(other.clone()),
                GenericFraction::Infinity(osign) => {
                    Some(GenericFraction::Infinity(if sign == osign {
                        Sign::Plus
                    } else {
                        Sign::Minus
                    }))
                }
                GenericFraction::Rational(osign, ref l) => {
                    if l.is_zero() {
                        Some(GenericFraction::NaN)
                    } else {
                        Some(GenericFraction::Infinity(if sign == osign {
                            Sign::Plus
                        } else {
                            Sign::Minus
                        }))
                    }
                }
            },
            GenericFraction::Rational(sign, ref l) => match *other {
                GenericFraction::NaN => Some(other.clone()),
                GenericFraction::Infinity(osign) => {
                    if l.is_zero() {
                        Some(GenericFraction::NaN)
                    } else {
                        Some(GenericFraction::Infinity(if sign == osign {
                            Sign::Plus
                        } else {
                            Sign::Minus
                        }))
                    }
                }
                GenericFraction::Rational(osign, ref r) => l.checked_mul(r).map(|value| {
                    GenericFraction::Rational(
                        if l.is_zero() || r.is_zero() || sign == osign {
                            Sign::Plus
                        } else {
                            Sign::Minus
                        },
                        value,
                    )
                }),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckedMul, GenericFraction};
    use crate::{One, Zero};

    type F = GenericFraction<u8>;

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
            F::infinity().checked_mul(&F::new_neg(1, 1))
        );

        assert_eq!(Some(F::nan()), F::one().checked_mul(&F::nan()));
        assert_eq!(Some(F::infinity()), F::one().checked_mul(&F::infinity()));
        assert_eq!(
            Some(F::neg_infinity()),
            F::one().checked_mul(&F::neg_infinity())
        );

        assert_eq!(Some(F::one()), F::one().checked_mul(&F::one()));
        assert_eq!(
            Some(F::new_neg(1, 1)),
            F::one().checked_mul(&F::new_neg(1, 1))
        );
        assert_eq!(
            Some(F::new_neg(1, 1)),
            F::new_neg(1, 1).checked_mul(&F::one())
        );
        assert_eq!(
            Some(F::one()),
            F::new_neg(1, 1).checked_mul(&F::new_neg(1, 1))
        );

        assert_eq!(Some(F::new(2, 1)), F::one().checked_mul(&F::new(2, 1)));
        assert_eq!(
            Some(F::new(2, 1)),
            F::new_neg(1, 1).checked_mul(&F::new_neg(2, 1))
        );

        assert_eq!(Some(F::zero()), F::one().checked_mul(&F::zero()));
        assert_eq!(Some(F::zero()), F::new_neg(1, 1).checked_mul(&F::zero()));
        assert_eq!(Some(F::zero()), F::zero().checked_mul(&F::one()));
        assert_eq!(Some(F::zero()), F::zero().checked_mul(&F::new_neg(1, 1)));
    }
}
