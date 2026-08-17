use core::marker::PhantomData;

use subtle::CtOption;

use crate::{modular::inv::inv_montgomery_form, traits::Invert, CtChoice, NonZero};

use super::{Residue, ResidueParams};

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Residue<MOD, LIMBS> {
    /// Computes the residue `self^-1` representing the multiplicative inverse of `self`.
    /// I.e. `self * self^-1 = 1`.
    /// If the number was invertible, the second element of the tuple is the truthy value,
    /// otherwise it is the falsy value (in which case the first element's value is unspecified).
    pub const fn invert(&self) -> (Self, CtChoice) {
        let (montgomery_form, is_some) = inv_montgomery_form(
            &self.montgomery_form,
            &MOD::MODULUS,
            &MOD::R3,
            MOD::MOD_NEG_INV,
        );

        let value = Self {
            montgomery_form,
            phantom: PhantomData,
        };

        (value, is_some)
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Invert for Residue<MOD, LIMBS> {
    type Output = CtOption<Self>;
    fn invert(&self) -> Self::Output {
        let (value, is_some) = self.invert();
        CtOption::new(value, is_some.into())
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Invert for NonZero<Residue<MOD, LIMBS>> {
    type Output = Self;
    fn invert(&self) -> Self::Output {
        // Always succeeds for a non-zero argument
        let (value, _is_some) = self.as_ref().invert();
        NonZero::new(value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{const_residue, impl_modulus, modular::constant_mod::ResidueParams, U256};

    impl_modulus!(
        Modulus,
        U256,
        "15477BCCEFE197328255BFA79A1217899016D927EF460F4FF404029D24FA4409"
    );

    #[test]
    fn test_self_inverse() {
        let x =
            U256::from_be_hex("77117F1273373C26C700D076B3F780074D03339F56DD0EFB60E7F58441FD3685");
        let x_mod = const_residue!(x, Modulus);

        let (inv, _is_some) = x_mod.invert();
        let res = x_mod * inv;

        assert_eq!(res.retrieve(), U256::ONE);
    }
}
