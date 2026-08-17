use subtle::CtOption;

use crate::{modular::inv::inv_montgomery_form, traits::Invert, CtChoice};

use super::DynResidue;

impl<const LIMBS: usize> DynResidue<LIMBS> {
    /// Computes the residue `self^-1` representing the multiplicative inverse of `self`.
    /// I.e. `self * self^-1 = 1`.
    /// If the number was invertible, the second element of the tuple is the truthy value,
    /// otherwise it is the falsy value (in which case the first element's value is unspecified).
    pub const fn invert(&self) -> (Self, CtChoice) {
        let (montgomery_form, is_some) = inv_montgomery_form(
            &self.montgomery_form,
            &self.residue_params.modulus,
            &self.residue_params.r3,
            self.residue_params.mod_neg_inv,
        );

        let value = Self {
            montgomery_form,
            residue_params: self.residue_params,
        };

        (value, is_some)
    }
}

impl<const LIMBS: usize> Invert for DynResidue<LIMBS> {
    type Output = CtOption<Self>;
    fn invert(&self) -> Self::Output {
        let (value, is_some) = self.invert();
        CtOption::new(value, is_some.into())
    }
}
