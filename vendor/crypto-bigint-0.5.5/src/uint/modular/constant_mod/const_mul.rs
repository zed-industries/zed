use core::{
    marker::PhantomData,
    ops::{Mul, MulAssign},
};

use crate::{
    modular::mul::{mul_montgomery_form, square_montgomery_form},
    traits::Square,
};

use super::{Residue, ResidueParams};

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Residue<MOD, LIMBS> {
    /// Multiplies by `rhs`.
    pub const fn mul(&self, rhs: &Self) -> Self {
        Self {
            montgomery_form: mul_montgomery_form(
                &self.montgomery_form,
                &rhs.montgomery_form,
                &MOD::MODULUS,
                MOD::MOD_NEG_INV,
            ),
            phantom: PhantomData,
        }
    }

    /// Computes the (reduced) square of a residue.
    pub const fn square(&self) -> Self {
        Self {
            montgomery_form: square_montgomery_form(
                &self.montgomery_form,
                &MOD::MODULUS,
                MOD::MOD_NEG_INV,
            ),
            phantom: PhantomData,
        }
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Mul<&Residue<MOD, LIMBS>>
    for &Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;
    fn mul(self, rhs: &Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        self.mul(rhs)
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Mul<Residue<MOD, LIMBS>>
    for &Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;
    #[allow(clippy::op_ref)]
    fn mul(self, rhs: Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        self * &rhs
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Mul<&Residue<MOD, LIMBS>>
    for Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;
    #[allow(clippy::op_ref)]
    fn mul(self, rhs: &Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        &self * rhs
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Mul<Residue<MOD, LIMBS>>
    for Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;
    fn mul(self, rhs: Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        &self * &rhs
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> MulAssign<&Self> for Residue<MOD, LIMBS> {
    fn mul_assign(&mut self, rhs: &Residue<MOD, LIMBS>) {
        *self = *self * rhs;
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> MulAssign<Self> for Residue<MOD, LIMBS> {
    fn mul_assign(&mut self, rhs: Self) {
        *self *= &rhs;
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Square for Residue<MOD, LIMBS> {
    fn square(&self) -> Self {
        Residue::square(self)
    }
}
