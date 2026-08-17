use core::ops::Neg;

use crate::{CtChoice, Limb, Uint, WideWord, Word, Wrapping};

impl<const LIMBS: usize> Neg for Wrapping<Uint<LIMBS>> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.wrapping_neg())
    }
}

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Negates based on `choice` by wrapping the integer.
    pub(crate) const fn conditional_wrapping_neg(&self, choice: CtChoice) -> Uint<LIMBS> {
        Uint::ct_select(self, &self.wrapping_neg(), choice)
    }

    /// Perform wrapping negation.
    pub const fn wrapping_neg(&self) -> Self {
        let mut ret = [Limb::ZERO; LIMBS];
        let mut carry = 1;
        let mut i = 0;
        while i < LIMBS {
            let r = (!self.limbs[i].0 as WideWord) + carry;
            ret[i] = Limb(r as Word);
            carry = r >> Limb::BITS;
            i += 1;
        }
        Uint::new(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::U256;

    #[test]
    fn wrapping_neg() {
        assert_eq!(U256::ZERO.wrapping_neg(), U256::ZERO);
        assert_eq!(U256::MAX.wrapping_neg(), U256::ONE);
        assert_eq!(
            U256::from_u64(13).wrapping_neg(),
            U256::from_u64(13).not().saturating_add(&U256::ONE)
        );
        assert_eq!(
            U256::from_u64(42).wrapping_neg(),
            U256::from_u64(42).saturating_sub(&U256::ONE).not()
        );
    }
}
