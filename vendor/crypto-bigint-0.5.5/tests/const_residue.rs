//! Test to ensure that `const_residue!` works from outside this crate.

use crypto_bigint::{const_residue, impl_modulus, modular::constant_mod::ResidueParams, U64};

impl_modulus!(TestMod, U64, "30e4b8f030ab42f3");

fn _test_fun() {
    let base = U64::from(2u64);
    let _base_mod = const_residue!(base, TestMod);
}
