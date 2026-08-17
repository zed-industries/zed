use core::ops::{Sub, SubAssign};

use crate::modular::sub::sub_montgomery_form;

use super::{Residue, ResidueParams};

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Residue<MOD, LIMBS> {
    /// Subtracts `rhs`.
    pub const fn sub(&self, rhs: &Self) -> Self {
        Self {
            montgomery_form: sub_montgomery_form(
                &self.montgomery_form,
                &rhs.montgomery_form,
                &MOD::MODULUS,
            ),
            phantom: core::marker::PhantomData,
        }
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Sub<&Residue<MOD, LIMBS>>
    for &Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;
    fn sub(self, rhs: &Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        self.sub(rhs)
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Sub<Residue<MOD, LIMBS>>
    for &Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;
    #[allow(clippy::op_ref)]
    fn sub(self, rhs: Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        self - &rhs
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Sub<&Residue<MOD, LIMBS>>
    for Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;
    #[allow(clippy::op_ref)]
    fn sub(self, rhs: &Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        &self - rhs
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Sub<Residue<MOD, LIMBS>>
    for Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;
    fn sub(self, rhs: Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        &self - &rhs
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> SubAssign<&Self> for Residue<MOD, LIMBS> {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = *self - rhs;
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> SubAssign<Self> for Residue<MOD, LIMBS> {
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::{const_residue, impl_modulus, modular::constant_mod::ResidueParams, U256};

    impl_modulus!(
        Modulus,
        U256,
        "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551"
    );

    #[test]
    fn sub_overflow() {
        let x =
            U256::from_be_hex("44acf6b7e36c1342c2c5897204fe09504e1e2efb1a900377dbc4e7a6a133ec56");
        let mut x_mod = const_residue!(x, Modulus);

        let y =
            U256::from_be_hex("d5777c45019673125ad240f83094d4252d829516fac8601ed01979ec1ec1a251");
        let y_mod = const_residue!(y, Modulus);

        x_mod -= &y_mod;

        let expected =
            U256::from_be_hex("6f357a71e1d5a03167f34879d469352add829491c6df41ddff65387d7ed56f56");

        assert_eq!(expected, x_mod.retrieve());
    }
}
