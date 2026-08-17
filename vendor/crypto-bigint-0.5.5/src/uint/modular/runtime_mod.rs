use crate::{Limb, Uint, Word};

use super::{
    constant_mod::{Residue, ResidueParams},
    div_by_2::div_by_2,
    reduction::montgomery_reduction,
    Retrieve,
};

use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

/// Additions between residues with a modulus set at runtime
mod runtime_add;
/// Multiplicative inverses of residues with a modulus set at runtime
mod runtime_inv;
/// Multiplications between residues with a modulus set at runtime
mod runtime_mul;
/// Negations of residues with a modulus set at runtime
mod runtime_neg;
/// Exponentiation of residues with a modulus set at runtime
mod runtime_pow;
/// Subtractions between residues with a modulus set at runtime
mod runtime_sub;

/// The parameters to efficiently go to and from the Montgomery form for an odd modulus provided at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynResidueParams<const LIMBS: usize> {
    // The constant modulus
    modulus: Uint<LIMBS>,
    // Parameter used in Montgomery reduction
    r: Uint<LIMBS>,
    // R^2, used to move into Montgomery form
    r2: Uint<LIMBS>,
    // R^3, used to compute the multiplicative inverse
    r3: Uint<LIMBS>,
    // The lowest limbs of -(MODULUS^-1) mod R
    // We only need the LSB because during reduction this value is multiplied modulo 2**Limb::BITS.
    mod_neg_inv: Limb,
}

impl<const LIMBS: usize> DynResidueParams<LIMBS> {
    // Internal helper function to generate parameters; this lets us wrap the constructors more cleanly
    const fn generate_params(modulus: &Uint<LIMBS>) -> Self {
        let r = Uint::MAX.const_rem(modulus).0.wrapping_add(&Uint::ONE);
        let r2 = Uint::const_rem_wide(r.square_wide(), modulus).0;

        // Since we are calculating the inverse modulo (Word::MAX+1),
        // we can take the modulo right away and calculate the inverse of the first limb only.
        let modulus_lo = Uint::<1>::from_words([modulus.limbs[0].0]);
        let mod_neg_inv = Limb(
            Word::MIN.wrapping_sub(modulus_lo.inv_mod2k_vartime(Word::BITS as usize).limbs[0].0),
        );

        let r3 = montgomery_reduction(&r2.square_wide(), modulus, mod_neg_inv);

        Self {
            modulus: *modulus,
            r,
            r2,
            r3,
            mod_neg_inv,
        }
    }

    /// Instantiates a new set of `ResidueParams` representing the given `modulus`, which _must_ be odd.
    /// If `modulus` is not odd, this function will panic; use [`new_checked`][`DynResidueParams::new_checked`] if you want to be able to detect an invalid modulus.
    pub const fn new(modulus: &Uint<LIMBS>) -> Self {
        // A valid modulus must be odd
        if modulus.ct_is_odd().to_u8() == 0 {
            panic!("modulus must be odd");
        }

        Self::generate_params(modulus)
    }

    /// Instantiates a new set of `ResidueParams` representing the given `modulus` if it is odd.
    /// Returns a `CtOption` that is `None` if the provided modulus is not odd; this is a safer version of [`new`][`DynResidueParams::new`], which can panic.
    #[deprecated(
        since = "0.5.3",
        note = "This functionality will be moved to `new` in a future release."
    )]
    pub fn new_checked(modulus: &Uint<LIMBS>) -> CtOption<Self> {
        // A valid modulus must be odd.
        CtOption::new(Self::generate_params(modulus), modulus.ct_is_odd().into())
    }

    /// Returns the modulus which was used to initialize these parameters.
    pub const fn modulus(&self) -> &Uint<LIMBS> {
        &self.modulus
    }

    /// Create `DynResidueParams` corresponding to a `ResidueParams`.
    pub const fn from_residue_params<P>() -> Self
    where
        P: ResidueParams<LIMBS>,
    {
        Self {
            modulus: P::MODULUS,
            r: P::R,
            r2: P::R2,
            r3: P::R3,
            mod_neg_inv: P::MOD_NEG_INV,
        }
    }
}

impl<const LIMBS: usize> ConditionallySelectable for DynResidueParams<LIMBS> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self {
            modulus: Uint::conditional_select(&a.modulus, &b.modulus, choice),
            r: Uint::conditional_select(&a.r, &b.r, choice),
            r2: Uint::conditional_select(&a.r2, &b.r2, choice),
            r3: Uint::conditional_select(&a.r3, &b.r3, choice),
            mod_neg_inv: Limb::conditional_select(&a.mod_neg_inv, &b.mod_neg_inv, choice),
        }
    }
}

impl<const LIMBS: usize> ConstantTimeEq for DynResidueParams<LIMBS> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.modulus.ct_eq(&other.modulus)
            & self.r.ct_eq(&other.r)
            & self.r2.ct_eq(&other.r2)
            & self.r3.ct_eq(&other.r3)
            & self.mod_neg_inv.ct_eq(&other.mod_neg_inv)
    }
}

/// A residue represented using `LIMBS` limbs. The odd modulus of this residue is set at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynResidue<const LIMBS: usize> {
    montgomery_form: Uint<LIMBS>,
    residue_params: DynResidueParams<LIMBS>,
}

impl<const LIMBS: usize> DynResidue<LIMBS> {
    /// Instantiates a new `Residue` that represents this `integer` mod `MOD`.
    pub const fn new(integer: &Uint<LIMBS>, residue_params: DynResidueParams<LIMBS>) -> Self {
        let product = integer.mul_wide(&residue_params.r2);
        let montgomery_form = montgomery_reduction(
            &product,
            &residue_params.modulus,
            residue_params.mod_neg_inv,
        );

        Self {
            montgomery_form,
            residue_params,
        }
    }

    /// Retrieves the integer currently encoded in this `Residue`, guaranteed to be reduced.
    pub const fn retrieve(&self) -> Uint<LIMBS> {
        montgomery_reduction(
            &(self.montgomery_form, Uint::ZERO),
            &self.residue_params.modulus,
            self.residue_params.mod_neg_inv,
        )
    }

    /// Instantiates a new `Residue` that represents zero.
    pub const fn zero(residue_params: DynResidueParams<LIMBS>) -> Self {
        Self {
            montgomery_form: Uint::<LIMBS>::ZERO,
            residue_params,
        }
    }

    /// Instantiates a new `Residue` that represents 1.
    pub const fn one(residue_params: DynResidueParams<LIMBS>) -> Self {
        Self {
            montgomery_form: residue_params.r,
            residue_params,
        }
    }

    /// Returns the parameter struct used to initialize this residue.
    pub const fn params(&self) -> &DynResidueParams<LIMBS> {
        &self.residue_params
    }

    /// Access the `DynResidue` value in Montgomery form.
    pub const fn as_montgomery(&self) -> &Uint<LIMBS> {
        &self.montgomery_form
    }

    /// Mutably access the `DynResidue` value in Montgomery form.
    pub fn as_montgomery_mut(&mut self) -> &mut Uint<LIMBS> {
        &mut self.montgomery_form
    }

    /// Create a `DynResidue` from a value in Montgomery form.
    pub const fn from_montgomery(
        integer: Uint<LIMBS>,
        residue_params: DynResidueParams<LIMBS>,
    ) -> Self {
        Self {
            montgomery_form: integer,
            residue_params,
        }
    }

    /// Extract the value from the `DynResidue` in Montgomery form.
    pub const fn to_montgomery(&self) -> Uint<LIMBS> {
        self.montgomery_form
    }

    /// Performs the modular division by 2, that is for given `x` returns `y`
    /// such that `y * 2 = x mod p`. This means:
    /// - if `x` is even, returns `x / 2`,
    /// - if `x` is odd, returns `(x + p) / 2`
    ///   (since the modulus `p` in Montgomery form is always odd, this divides entirely).
    pub fn div_by_2(&self) -> Self {
        Self {
            montgomery_form: div_by_2(&self.montgomery_form, &self.residue_params.modulus),
            residue_params: self.residue_params,
        }
    }
}

impl<const LIMBS: usize> Retrieve for DynResidue<LIMBS> {
    type Output = Uint<LIMBS>;
    fn retrieve(&self) -> Self::Output {
        self.retrieve()
    }
}

impl<const LIMBS: usize, P: ResidueParams<LIMBS>> From<&Residue<P, LIMBS>> for DynResidue<LIMBS> {
    fn from(residue: &Residue<P, LIMBS>) -> Self {
        Self {
            montgomery_form: residue.to_montgomery(),
            residue_params: DynResidueParams::from_residue_params::<P>(),
        }
    }
}

impl<const LIMBS: usize> ConditionallySelectable for DynResidue<LIMBS> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self {
            montgomery_form: Uint::conditional_select(
                &a.montgomery_form,
                &b.montgomery_form,
                choice,
            ),
            residue_params: DynResidueParams::conditional_select(
                &a.residue_params,
                &b.residue_params,
                choice,
            ),
        }
    }
}

impl<const LIMBS: usize> ConstantTimeEq for DynResidue<LIMBS> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.montgomery_form.ct_eq(&other.montgomery_form)
            & self.residue_params.ct_eq(&other.residue_params)
    }
}

/// NOTE: this does _not_ zeroize the parameters, in order to maintain some form of type consistency
#[cfg(feature = "zeroize")]
impl<const LIMBS: usize> zeroize::Zeroize for DynResidue<LIMBS> {
    fn zeroize(&mut self) {
        self.montgomery_form.zeroize()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const LIMBS: usize = nlimbs!(64);

    #[test]
    #[allow(deprecated)]
    // Test that a valid modulus yields `DynResidueParams`
    fn test_valid_modulus() {
        let valid_modulus = Uint::<LIMBS>::from(3u8);

        DynResidueParams::<LIMBS>::new_checked(&valid_modulus).unwrap();
        DynResidueParams::<LIMBS>::new(&valid_modulus);
    }

    #[test]
    #[allow(deprecated)]
    // Test that an invalid checked modulus does not yield `DynResidueParams`
    fn test_invalid_checked_modulus() {
        assert!(bool::from(
            DynResidueParams::<LIMBS>::new_checked(&Uint::from(2u8)).is_none()
        ))
    }

    #[test]
    #[should_panic]
    // Tets that an invalid modulus panics
    fn test_invalid_modulus() {
        DynResidueParams::<LIMBS>::new(&Uint::from(2u8));
    }
}
