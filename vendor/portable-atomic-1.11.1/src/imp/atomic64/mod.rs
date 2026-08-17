// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
64-bit atomic implementations on 32-bit architectures

See README.md for details.
*/

// pre-v6 Arm Linux
#[cfg(feature = "fallback")]
// Miri and Sanitizer do not support inline assembly.
#[cfg(all(
    target_arch = "arm",
    not(any(miri, portable_atomic_sanitize_thread)),
    any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
    any(target_os = "linux", target_os = "android"),
    not(any(target_feature = "v6", portable_atomic_target_feature = "v6")),
    not(portable_atomic_no_outline_atomics),
))]
#[cfg_attr(portable_atomic_no_cfg_target_has_atomic, cfg(portable_atomic_no_atomic_64))]
#[cfg_attr(not(portable_atomic_no_cfg_target_has_atomic), cfg(not(target_has_atomic = "64")))]
pub(super) mod arm_linux;

// riscv32
// Miri and Sanitizer do not support inline assembly.
#[cfg(all(
    target_arch = "riscv32",
    not(any(miri, portable_atomic_sanitize_thread)),
    any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
    any(
        target_feature = "zacas",
        portable_atomic_target_feature = "zacas",
        all(
            feature = "fallback",
            not(portable_atomic_no_outline_atomics),
            any(target_os = "linux", target_os = "android"),
        ),
    ),
))]
pub(super) mod riscv32;
