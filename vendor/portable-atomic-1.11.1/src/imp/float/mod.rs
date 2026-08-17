// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Atomic float implementations
*/

#![allow(clippy::float_arithmetic)]

mod int;

#[cfg(all(
    any(target_arch = "aarch64", target_arch = "arm64ec"),
    any(target_feature = "lsfe", portable_atomic_target_feature = "lsfe"),
    target_feature = "neon", // for vreg
    not(any(miri, portable_atomic_sanitize_thread)),
    any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
))]
mod aarch64;

#[cfg(portable_atomic_unstable_f16)]
cfg_has_atomic_16! {
    pub(crate) use self::int::AtomicF16;
}
cfg_has_atomic_32! {
    pub(crate) use self::int::AtomicF32;
}
cfg_has_atomic_64! {
    pub(crate) use self::int::AtomicF64;
}
#[cfg(portable_atomic_unstable_f128)]
cfg_has_atomic_128! {
    pub(crate) use self::int::AtomicF128;
}
