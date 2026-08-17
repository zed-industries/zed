use core::{fmt::Debug, marker::PhantomData};

use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::{Limb, Uint, Zero};

use super::{div_by_2::div_by_2, reduction::montgomery_reduction, Retrieve};

#[cfg(feature = "rand_core")]
use crate::{rand_core::CryptoRngCore, NonZero, Random, RandomMod};

#[cfg(feature = "serde")]
use {
    crate::Encoding,
    serdect::serde::de::Error,
    serdect::serde::{Deserialize, Deserializer, Serialize, Serializer},
};

/// Additions between residues with a constant modulus
mod const_add;
/// Multiplicative inverses of residues with a constant modulus
mod const_inv;
/// Multiplications between residues with a constant modulus
mod const_mul;
/// Negations of residues with a constant modulus
mod const_neg;
/// Exponentiation of residues with a constant modulus
mod const_pow;
/// Subtractions between residues with a constant modulus
mod const_sub;

/// Macros to remove the boilerplate code when dealing with constant moduli.
#[macro_use]
mod macros;

pub use macros::*;

/// The parameters to efficiently go to and from the Montgomery form for a given odd modulus. An easy way to generate these parameters is using the `impl_modulus!` macro. These parameters are constant, so they cannot be set at runtime.
///
/// Unfortunately, `LIMBS` must be generic for now until const generics are stabilized.
pub trait ResidueParams<const LIMBS: usize>:
    Copy + Debug + Default + Eq + Send + Sync + 'static
{
    /// Number of limbs required to encode a residue
    const LIMBS: usize;

    /// The constant modulus
    const MODULUS: Uint<LIMBS>;
    /// Parameter used in Montgomery reduction
    const R: Uint<LIMBS>;
    /// R^2, used to move into Montgomery form
    const R2: Uint<LIMBS>;
    /// R^3, used to perform a multiplicative inverse
    const R3: Uint<LIMBS>;
    /// The lowest limbs of -(MODULUS^-1) mod R
    // We only need the LSB because during reduction this value is multiplied modulo 2**Limb::BITS.
    const MOD_NEG_INV: Limb;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A residue mod `MOD`, represented using `LIMBS` limbs. The modulus of this residue is constant, so it cannot be set at runtime.
/// Internally, the value is stored in Montgomery form (multiplied by MOD::R) until it is retrieved.
pub struct Residue<MOD, const LIMBS: usize>
where
    MOD: ResidueParams<LIMBS>,
{
    montgomery_form: Uint<LIMBS>,
    phantom: PhantomData<MOD>,
}

#[cfg(feature = "zeroize")]
impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> zeroize::DefaultIsZeroes
    for Residue<MOD, LIMBS>
{
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Residue<MOD, LIMBS> {
    /// The representation of 0 mod `MOD`.
    pub const ZERO: Self = Self {
        montgomery_form: Uint::<LIMBS>::ZERO,
        phantom: PhantomData,
    };

    /// The representation of 1 mod `MOD`.
    pub const ONE: Self = Self {
        montgomery_form: MOD::R,
        phantom: PhantomData,
    };

    // Internal helper function to generate a residue; this lets us wrap the constructors more cleanly
    const fn generate_residue(integer: &Uint<LIMBS>) -> Self {
        let product = integer.mul_wide(&MOD::R2);
        let montgomery_form =
            montgomery_reduction::<LIMBS>(&product, &MOD::MODULUS, MOD::MOD_NEG_INV);

        Self {
            montgomery_form,
            phantom: PhantomData,
        }
    }

    /// Instantiates a new `Residue` that represents this `integer` mod `MOD`.
    /// If the modulus represented by `MOD` is not odd, this function will panic; use [`new_checked`][`Residue::new_checked`] if you want to be able to detect an invalid modulus.
    pub const fn new(integer: &Uint<LIMBS>) -> Self {
        // A valid modulus must be odd
        if MOD::MODULUS.ct_is_odd().to_u8() == 0 {
            panic!("modulus must be odd");
        }

        Self::generate_residue(integer)
    }

    /// Instantiates a new `Residue` that represents this `integer` mod `MOD` if the modulus is odd.
    /// Returns a `CtOption` that is `None` if the provided modulus is not odd; this is a safer version of [`new`][`Residue::new`], which can panic.
    // TODO: remove this method when we can use `generic_const_exprs.` to ensure the modulus is
    // always valid.
    pub fn new_checked(integer: &Uint<LIMBS>) -> CtOption<Self> {
        // A valid modulus must be odd.
        CtOption::new(
            Self::generate_residue(integer),
            MOD::MODULUS.ct_is_odd().into(),
        )
    }

    /// Retrieves the integer currently encoded in this `Residue`, guaranteed to be reduced.
    pub const fn retrieve(&self) -> Uint<LIMBS> {
        montgomery_reduction::<LIMBS>(
            &(self.montgomery_form, Uint::ZERO),
            &MOD::MODULUS,
            MOD::MOD_NEG_INV,
        )
    }

    /// Access the `Residue` value in Montgomery form.
    pub const fn as_montgomery(&self) -> &Uint<LIMBS> {
        &self.montgomery_form
    }

    /// Mutably access the `Residue` value in Montgomery form.
    pub fn as_montgomery_mut(&mut self) -> &mut Uint<LIMBS> {
        &mut self.montgomery_form
    }

    /// Create a `Residue` from a value in Montgomery form.
    pub const fn from_montgomery(integer: Uint<LIMBS>) -> Self {
        Self {
            montgomery_form: integer,
            phantom: PhantomData,
        }
    }

    /// Extract the value from the `Residue` in Montgomery form.
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
            montgomery_form: div_by_2(&self.montgomery_form, &MOD::MODULUS),
            phantom: PhantomData,
        }
    }
}

impl<MOD: ResidueParams<LIMBS> + Copy, const LIMBS: usize> ConditionallySelectable
    for Residue<MOD, LIMBS>
{
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Residue {
            montgomery_form: Uint::conditional_select(
                &a.montgomery_form,
                &b.montgomery_form,
                choice,
            ),
            phantom: PhantomData,
        }
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> ConstantTimeEq for Residue<MOD, LIMBS> {
    fn ct_eq(&self, other: &Self) -> Choice {
        ConstantTimeEq::ct_eq(&self.montgomery_form, &other.montgomery_form)
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Default for Residue<MOD, LIMBS> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Zero for Residue<MOD, LIMBS> {
    const ZERO: Self = Self::ZERO;
}

#[cfg(feature = "rand_core")]
impl<MOD, const LIMBS: usize> Random for Residue<MOD, LIMBS>
where
    MOD: ResidueParams<LIMBS>,
{
    #[inline]
    fn random(rng: &mut impl CryptoRngCore) -> Self {
        Self::new(&Uint::random_mod(rng, &NonZero::from_uint(MOD::MODULUS)))
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Retrieve for Residue<MOD, LIMBS> {
    type Output = Uint<LIMBS>;
    fn retrieve(&self) -> Self::Output {
        self.retrieve()
    }
}

#[cfg(feature = "serde")]
impl<'de, MOD, const LIMBS: usize> Deserialize<'de> for Residue<MOD, LIMBS>
where
    MOD: ResidueParams<LIMBS>,
    Uint<LIMBS>: Encoding,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Uint::<LIMBS>::deserialize(deserializer).and_then(|montgomery_form| {
            if Uint::ct_lt(&montgomery_form, &MOD::MODULUS).into() {
                Ok(Self {
                    montgomery_form,
                    phantom: PhantomData,
                })
            } else {
                Err(D::Error::custom("montgomery form must be reduced"))
            }
        })
    }
}

#[cfg(feature = "serde")]
impl<MOD, const LIMBS: usize> Serialize for Residue<MOD, LIMBS>
where
    MOD: ResidueParams<LIMBS>,
    Uint<LIMBS>: Encoding,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.montgomery_form.serialize(serializer)
    }
}
