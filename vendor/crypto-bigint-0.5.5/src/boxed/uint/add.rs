//! [`BoxedUint`] addition operations.

use crate::{BoxedUint, CheckedAdd, Limb, Zero};
use subtle::CtOption;

impl BoxedUint {
    /// Computes `a + b + carry`, returning the result along with the new carry.
    #[inline(always)]
    pub fn adc(&self, rhs: &Self, carry: Limb) -> (Self, Limb) {
        Self::chain(self, rhs, carry, |a, b, c| a.adc(b, c))
    }

    /// Perform wrapping addition, discarding overflow.
    pub fn wrapping_add(&self, rhs: &Self) -> Self {
        self.adc(rhs, Limb::ZERO).0
    }
}

impl CheckedAdd<&BoxedUint> for BoxedUint {
    type Output = Self;

    fn checked_add(&self, rhs: &Self) -> CtOption<Self> {
        let (result, carry) = self.adc(rhs, Limb::ZERO);
        CtOption::new(result, carry.is_zero())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{BoxedUint, CheckedAdd, Limb};

    #[test]
    fn adc_no_carry() {
        let (res, carry) = BoxedUint::zero().adc(&BoxedUint::one(), Limb::ZERO);
        assert_eq!(res, BoxedUint::one());
        assert_eq!(carry, Limb::ZERO);
    }

    #[test]
    fn adc_with_carry() {
        let (res, carry) = BoxedUint::max(Limb::BITS)
            .unwrap()
            .adc(&BoxedUint::one(), Limb::ZERO);
        assert_eq!(res, BoxedUint::zero());
        assert_eq!(carry, Limb::ONE);
    }

    #[test]
    fn checked_add_ok() {
        let result = BoxedUint::zero().checked_add(&BoxedUint::one());
        assert_eq!(result.unwrap(), BoxedUint::one());
    }

    #[test]
    fn checked_add_overflow() {
        let result = BoxedUint::max(Limb::BITS)
            .unwrap()
            .checked_add(&BoxedUint::one());
        assert!(!bool::from(result.is_some()));
    }
}
