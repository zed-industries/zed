// TODO: Use `adt_const_params` once stabilized to make a `Residue` generic around a modulus rather than having to implement a ZST + trait
#[macro_export]
/// Implements a modulus with the given name, type, and value, in that specific order. Please `use crypto_bigint::traits::Encoding` to make this work.
/// For example, `impl_modulus!(MyModulus, U256, "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001");` implements a 256-bit modulus named `MyModulus`.
/// The modulus _must_ be odd, or this will panic.
macro_rules! impl_modulus {
    ($name:ident, $uint_type:ty, $value:expr) => {
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct $name {}
        impl<const DLIMBS: usize>
            $crate::modular::constant_mod::ResidueParams<{ <$uint_type>::LIMBS }> for $name
        where
            $uint_type: $crate::ConcatMixed<MixedOutput = $crate::Uint<DLIMBS>>,
        {
            const LIMBS: usize = <$uint_type>::LIMBS;
            const MODULUS: $uint_type = {
                let res = <$uint_type>::from_be_hex($value);

                // Check that the modulus is odd
                if res.as_limbs()[0].0 & 1 == 0 {
                    panic!("modulus must be odd");
                }

                res
            };
            const R: $uint_type = $crate::Uint::MAX
                .const_rem(&Self::MODULUS)
                .0
                .wrapping_add(&$crate::Uint::ONE);
            const R2: $uint_type =
                $crate::Uint::const_rem_wide(Self::R.square_wide(), &Self::MODULUS).0;
            const MOD_NEG_INV: $crate::Limb = $crate::Limb(
                $crate::Word::MIN.wrapping_sub(
                    Self::MODULUS
                        .inv_mod2k_vartime($crate::Word::BITS as usize)
                        .as_limbs()[0]
                        .0,
                ),
            );
            const R3: $uint_type = $crate::modular::montgomery_reduction(
                &Self::R2.square_wide(),
                &Self::MODULUS,
                Self::MOD_NEG_INV,
            );
        }
    };
}

#[macro_export]
/// Creates a `Residue` with the given value for a specific modulus.
/// For example, `residue!(U256::from(105u64), MyModulus);` creates a `Residue` for 105 mod `MyModulus`.
/// The modulus _must_ be odd, or this will panic.
macro_rules! const_residue {
    ($variable:ident, $modulus:ident) => {
        $crate::modular::constant_mod::Residue::<$modulus, { $modulus::LIMBS }>::new(&$variable)
    };
}
