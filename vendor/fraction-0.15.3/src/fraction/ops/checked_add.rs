use crate::fraction::{GenericFraction, Sign};
use crate::{CheckedAdd, CheckedMul, CheckedSub, Integer};

impl<T> CheckedAdd for GenericFraction<T>
where
    T: Clone + Integer + CheckedAdd + CheckedSub + CheckedMul,
{
    fn checked_add(&self, other: &Self) -> Option<GenericFraction<T>> {
        match *self {
            GenericFraction::NaN => Some(self.clone()),
            GenericFraction::Infinity(sign) => match *other {
                GenericFraction::NaN => Some(other.clone()),
                GenericFraction::Rational(_, _) => Some(self.clone()),
                GenericFraction::Infinity(osign) => {
                    if sign != osign {
                        Some(GenericFraction::NaN)
                    } else {
                        Some(self.clone())
                    }
                }
            },
            GenericFraction::Rational(ls, ref l) => match *other {
                GenericFraction::NaN => Some(other.clone()),
                GenericFraction::Infinity(_) => Some(other.clone()),
                GenericFraction::Rational(rs, ref r) => {
                    if ls == Sign::Plus && rs == Sign::Plus {
                        l.checked_add(r)
                            .map(|value| GenericFraction::Rational(Sign::Plus, value))
                    } else if ls == Sign::Plus {
                        if l < r {
                            r.checked_sub(l)
                                .map(|value| GenericFraction::Rational(Sign::Minus, value))
                        } else {
                            l.checked_sub(r)
                                .map(|value| GenericFraction::Rational(Sign::Plus, value))
                        }
                    } else if rs == Sign::Plus {
                        if r < l {
                            l.checked_sub(r)
                                .map(|value| GenericFraction::Rational(Sign::Minus, value))
                        } else {
                            r.checked_sub(l)
                                .map(|value| GenericFraction::Rational(Sign::Plus, value))
                        }
                    } else {
                        l.checked_add(r)
                            .map(|value| GenericFraction::Rational(Sign::Minus, value))
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckedAdd, GenericFraction};
    use crate::{One, Zero};

    type F = GenericFraction<u8>;

    #[test]
    fn checked_add() {
        assert_eq!(Some(F::nan()), F::nan().checked_add(&F::nan()));

        assert_eq!(Some(F::nan()), F::infinity().checked_add(&F::nan()));
        assert_eq!(
            Some(F::infinity()),
            F::infinity().checked_add(&F::infinity())
        );
        assert_eq!(
            Some(F::nan()),
            F::infinity().checked_add(&F::neg_infinity())
        );
        assert_eq!(Some(F::infinity()), F::infinity().checked_add(&F::one()));

        assert_eq!(Some(F::nan()), F::one().checked_add(&F::nan()));
        assert_eq!(Some(F::infinity()), F::one().checked_add(&F::infinity()));
        assert_eq!(
            Some(F::neg_infinity()),
            F::one().checked_add(&F::neg_infinity())
        );

        assert_eq!(Some(F::new(2, 1)), F::one().checked_add(&F::one()));
        assert_eq!(Some(F::zero()), F::one().checked_add(&F::new_neg(1, 1)));
        assert_eq!(Some(F::zero()), F::new_neg(1, 1).checked_add(&F::one()));
        assert_eq!(
            Some(F::new_neg(2, 1)),
            F::new_neg(1, 1).checked_add(&F::new_neg(1, 1))
        );

        assert_eq!(
            Some(F::new_neg(1, 1)),
            F::one().checked_add(&F::new_neg(2, 1))
        );
        assert_eq!(
            Some(F::new_neg(1, 1)),
            F::new_neg(2, 1).checked_add(&F::one())
        );
    }
}
