use super::DynResidue;
use crate::modular::pow::multi_exponentiate_montgomery_form_array;
#[cfg(feature = "alloc")]
use crate::modular::pow::multi_exponentiate_montgomery_form_slice;
use crate::{modular::pow::pow_montgomery_form, MultiExponentiateBoundedExp, PowBoundedExp, Uint};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

impl<const LIMBS: usize> DynResidue<LIMBS> {
    /// Raises to the `exponent` power.
    pub const fn pow<const RHS_LIMBS: usize>(
        &self,
        exponent: &Uint<RHS_LIMBS>,
    ) -> DynResidue<LIMBS> {
        self.pow_bounded_exp(exponent, Uint::<RHS_LIMBS>::BITS)
    }

    /// Raises to the `exponent` power,
    /// with `exponent_bits` representing the number of (least significant) bits
    /// to take into account for the exponent.
    ///
    /// NOTE: `exponent_bits` may be leaked in the time pattern.
    pub const fn pow_bounded_exp<const RHS_LIMBS: usize>(
        &self,
        exponent: &Uint<RHS_LIMBS>,
        exponent_bits: usize,
    ) -> Self {
        Self {
            montgomery_form: pow_montgomery_form(
                &self.montgomery_form,
                exponent,
                exponent_bits,
                &self.residue_params.modulus,
                &self.residue_params.r,
                self.residue_params.mod_neg_inv,
            ),
            residue_params: self.residue_params,
        }
    }
}

impl<const LIMBS: usize, const RHS_LIMBS: usize> PowBoundedExp<Uint<RHS_LIMBS>>
    for DynResidue<LIMBS>
{
    fn pow_bounded_exp(&self, exponent: &Uint<RHS_LIMBS>, exponent_bits: usize) -> Self {
        self.pow_bounded_exp(exponent, exponent_bits)
    }
}

impl<const N: usize, const LIMBS: usize, const RHS_LIMBS: usize>
    MultiExponentiateBoundedExp<Uint<RHS_LIMBS>, [(Self, Uint<RHS_LIMBS>); N]>
    for DynResidue<LIMBS>
{
    fn multi_exponentiate_bounded_exp(
        bases_and_exponents: &[(Self, Uint<RHS_LIMBS>); N],
        exponent_bits: usize,
    ) -> Self {
        const_assert_ne!(N, 0, "bases_and_exponents must not be empty");
        let residue_params = bases_and_exponents[0].0.residue_params;

        let mut bases_and_exponents_montgomery_form =
            [(Uint::<LIMBS>::ZERO, Uint::<RHS_LIMBS>::ZERO); N];

        let mut i = 0;
        while i < N {
            let (base, exponent) = bases_and_exponents[i];
            bases_and_exponents_montgomery_form[i] = (base.montgomery_form, exponent);
            i += 1;
        }

        Self {
            montgomery_form: multi_exponentiate_montgomery_form_array(
                &bases_and_exponents_montgomery_form,
                exponent_bits,
                &residue_params.modulus,
                &residue_params.r,
                residue_params.mod_neg_inv,
            ),
            residue_params,
        }
    }
}

#[cfg(feature = "alloc")]
impl<const LIMBS: usize, const RHS_LIMBS: usize>
    MultiExponentiateBoundedExp<Uint<RHS_LIMBS>, [(Self, Uint<RHS_LIMBS>)]> for DynResidue<LIMBS>
{
    fn multi_exponentiate_bounded_exp(
        bases_and_exponents: &[(Self, Uint<RHS_LIMBS>)],
        exponent_bits: usize,
    ) -> Self {
        assert!(
            !bases_and_exponents.is_empty(),
            "bases_and_exponents must not be empty"
        );
        let residue_params = bases_and_exponents[0].0.residue_params;

        let bases_and_exponents: Vec<(Uint<LIMBS>, Uint<RHS_LIMBS>)> = bases_and_exponents
            .iter()
            .map(|(base, exp)| (base.montgomery_form, *exp))
            .collect();
        Self {
            montgomery_form: multi_exponentiate_montgomery_form_slice(
                &bases_and_exponents,
                exponent_bits,
                &residue_params.modulus,
                &residue_params.r,
                residue_params.mod_neg_inv,
            ),
            residue_params,
        }
    }
}
