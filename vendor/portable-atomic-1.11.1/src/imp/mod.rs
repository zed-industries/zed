// SPDX-License-Identifier: Apache-2.0 OR MIT

// -----------------------------------------------------------------------------
// Lock-free implementations

#[cfg(not(any(
    all(
        portable_atomic_no_atomic_load_store,
        not(all(target_arch = "bpf", not(feature = "critical-section"))),
    ),
    target_arch = "avr",
    target_arch = "msp430",
)))]
#[cfg_attr(
    portable_atomic_no_cfg_target_has_atomic,
    cfg(not(all(
        any(
            target_arch = "riscv32",
            target_arch = "riscv64",
            feature = "critical-section",
            portable_atomic_unsafe_assume_single_core,
        ),
        portable_atomic_no_atomic_cas,
    )))
)]
#[cfg_attr(
    not(portable_atomic_no_cfg_target_has_atomic),
    cfg(not(all(
        any(
            target_arch = "riscv32",
            target_arch = "riscv64",
            feature = "critical-section",
            portable_atomic_unsafe_assume_single_core,
        ),
        not(target_has_atomic = "ptr"),
    )))
)]
mod core_atomic;

// AVR
#[cfg(target_arch = "avr")]
#[cfg(not(portable_atomic_no_asm))]
#[cfg(not(feature = "critical-section"))]
mod avr;

// MSP430
#[cfg(target_arch = "msp430")]
pub(crate) mod msp430;

// RISC-V without A-extension
#[cfg(any(test, not(feature = "critical-section")))]
#[cfg_attr(
    portable_atomic_no_cfg_target_has_atomic,
    cfg(any(
        all(test, not(any(miri, portable_atomic_sanitize_thread))),
        portable_atomic_no_atomic_cas,
    ))
)]
#[cfg_attr(
    not(portable_atomic_no_cfg_target_has_atomic),
    cfg(any(
        all(test, not(any(miri, portable_atomic_sanitize_thread))),
        not(target_has_atomic = "ptr"),
    ))
)]
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

// x86-specific optimizations
// Miri and Sanitizer do not support inline assembly.
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(any(miri, portable_atomic_sanitize_thread)),
    any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
))]
mod x86;

// 64-bit atomic implementations on 32-bit architectures
#[cfg(any(target_arch = "arm", target_arch = "riscv32"))]
mod atomic64;

// 128-bit atomic implementations on 64-bit architectures
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "powerpc64",
    target_arch = "riscv64",
    target_arch = "s390x",
    target_arch = "x86_64",
))]
mod atomic128;

// -----------------------------------------------------------------------------
// Lock-based fallback implementations

#[cfg(feature = "fallback")]
#[cfg_attr(portable_atomic_no_cfg_target_has_atomic, cfg(not(portable_atomic_no_atomic_cas)))]
#[cfg_attr(not(portable_atomic_no_cfg_target_has_atomic), cfg(target_has_atomic = "ptr"))]
#[cfg(any(
    test,
    not(any(
        all(
            target_arch = "aarch64",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
        ),
        all(
            target_arch = "arm64ec",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            not(portable_atomic_no_asm),
        ),
        all(
            target_arch = "x86_64",
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"),
        ),
        all(
            target_arch = "riscv64",
            not(any(miri, portable_atomic_sanitize_thread)),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            any(target_feature = "zacas", portable_atomic_target_feature = "zacas"),
        ),
        all(
            target_arch = "powerpc64",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            portable_atomic_unstable_asm_experimental_arch,
            any(
                target_feature = "quadword-atomics",
                portable_atomic_target_feature = "quadword-atomics",
            ),
        ),
        all(
            target_arch = "s390x",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            not(portable_atomic_no_asm),
        ),
    ))
))]
mod fallback;

// -----------------------------------------------------------------------------
// Critical section based fallback implementations

// On AVR, we always use critical section based fallback implementation.
// AVR can be safely assumed to be single-core, so this is sound.
// MSP430 as well.
// See the module-level comments of interrupt module for more.
#[cfg(any(
    all(test, target_os = "none"),
    portable_atomic_unsafe_assume_single_core,
    feature = "critical-section",
    target_arch = "avr",
    target_arch = "msp430",
))]
#[cfg_attr(portable_atomic_no_cfg_target_has_atomic, cfg(any(test, portable_atomic_no_atomic_cas)))]
#[cfg_attr(
    not(portable_atomic_no_cfg_target_has_atomic),
    cfg(any(test, not(target_has_atomic = "ptr")))
)]
#[cfg(any(
    target_arch = "arm",
    target_arch = "avr",
    target_arch = "msp430",
    target_arch = "riscv32",
    target_arch = "riscv64",
    target_arch = "xtensa",
    feature = "critical-section",
))]
mod interrupt;

// -----------------------------------------------------------------------------
// Atomic float implementations

#[cfg(feature = "float")]
pub(crate) mod float;

// -----------------------------------------------------------------------------

// has CAS | (has core atomic & !(avr | msp430 | critical section)) => core atomic
#[cfg(not(any(
    portable_atomic_no_atomic_load_store,
    target_arch = "avr",
    target_arch = "msp430",
)))]
#[cfg_attr(
    portable_atomic_no_cfg_target_has_atomic,
    cfg(not(all(
        any(
            target_arch = "riscv32",
            target_arch = "riscv64",
            feature = "critical-section",
            portable_atomic_unsafe_assume_single_core,
        ),
        portable_atomic_no_atomic_cas,
    )))
)]
#[cfg_attr(
    not(portable_atomic_no_cfg_target_has_atomic),
    cfg(not(all(
        any(
            target_arch = "riscv32",
            target_arch = "riscv64",
            feature = "critical-section",
            portable_atomic_unsafe_assume_single_core,
        ),
        not(target_has_atomic = "ptr"),
    )))
)]
items! {
    pub(crate) use self::core_atomic::{
        AtomicI16, AtomicI32, AtomicI8, AtomicIsize, AtomicPtr, AtomicU16, AtomicU32, AtomicU8,
        AtomicUsize,
    };
    #[cfg_attr(
        portable_atomic_no_cfg_target_has_atomic,
        cfg(any(
            not(portable_atomic_no_atomic_64),
            not(any(target_pointer_width = "16", target_pointer_width = "32")),
        ))
    )]
    #[cfg_attr(
        not(portable_atomic_no_cfg_target_has_atomic),
        cfg(any(
            target_has_atomic = "64",
            not(any(target_pointer_width = "16", target_pointer_width = "32")),
        ))
    )]
    pub(crate) use self::core_atomic::{AtomicI64, AtomicU64};
}
// bpf & !(critical section) => core atomic
#[cfg(all(
    target_arch = "bpf",
    portable_atomic_no_atomic_load_store,
    not(feature = "critical-section"),
))]
pub(crate) use self::core_atomic::{AtomicI64, AtomicIsize, AtomicPtr, AtomicU64, AtomicUsize};

// RISC-V without A-extension & !(assume single core | critical section)
#[cfg(not(any(portable_atomic_unsafe_assume_single_core, feature = "critical-section")))]
#[cfg_attr(portable_atomic_no_cfg_target_has_atomic, cfg(portable_atomic_no_atomic_cas))]
#[cfg_attr(not(portable_atomic_no_cfg_target_has_atomic), cfg(not(target_has_atomic = "ptr")))]
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
items! {
    pub(crate) use self::riscv::{
        AtomicI16, AtomicI32, AtomicI8, AtomicIsize, AtomicPtr, AtomicU16, AtomicU32, AtomicU8,
        AtomicUsize,
    };
    #[cfg(target_arch = "riscv64")]
    pub(crate) use self::riscv::{AtomicI64, AtomicU64};
}

// no core atomic CAS & (assume single core | critical section) => critical section based fallback
#[cfg(any(
    portable_atomic_unsafe_assume_single_core,
    feature = "critical-section",
    target_arch = "avr",
    target_arch = "msp430",
))]
#[cfg_attr(portable_atomic_no_cfg_target_has_atomic, cfg(portable_atomic_no_atomic_cas))]
#[cfg_attr(not(portable_atomic_no_cfg_target_has_atomic), cfg(not(target_has_atomic = "ptr")))]
items! {
    pub(crate) use self::interrupt::{
        AtomicI16, AtomicI8, AtomicIsize, AtomicPtr, AtomicU16, AtomicU8, AtomicUsize,
    };
    #[cfg(any(not(target_pointer_width = "16"), feature = "fallback"))]
    pub(crate) use self::interrupt::{AtomicI32, AtomicU32};
    #[cfg(any(
        not(any(target_pointer_width = "16", target_pointer_width = "32")),
        feature = "fallback",
    ))]
    pub(crate) use self::interrupt::{AtomicI64, AtomicU64};
    #[cfg(feature = "fallback")]
    pub(crate) use self::interrupt::{AtomicI128, AtomicU128};
}

// no core (64-bit | 128-bit) atomic & has CAS => use lock-base fallback
#[cfg(feature = "fallback")]
#[cfg_attr(portable_atomic_no_cfg_target_has_atomic, cfg(not(portable_atomic_no_atomic_cas)))]
#[cfg_attr(not(portable_atomic_no_cfg_target_has_atomic), cfg(target_has_atomic = "ptr"))]
items! {
    #[cfg(not(any(
        all(
            target_arch = "arm",
            not(any(miri, portable_atomic_sanitize_thread)),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            any(target_os = "linux", target_os = "android"),
            not(any(target_feature = "v6", portable_atomic_target_feature = "v6")),
            not(portable_atomic_no_outline_atomics),
        ),
        all(
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
        ),
    )))]
    #[cfg_attr(portable_atomic_no_cfg_target_has_atomic, cfg(portable_atomic_no_atomic_64))]
    #[cfg_attr(not(portable_atomic_no_cfg_target_has_atomic), cfg(not(target_has_atomic = "64")))]
    pub(crate) use self::fallback::{AtomicI64, AtomicU64};
    #[cfg(not(any(
        all(
            target_arch = "aarch64",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
        ),
        all(
            target_arch = "arm64ec",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            not(portable_atomic_no_asm),
        ),
        all(
            target_arch = "x86_64",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                portable_atomic_no_cmpxchg16b_intrinsic,
            )),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            any(
                target_feature = "cmpxchg16b",
                portable_atomic_target_feature = "cmpxchg16b",
                all(
                    feature = "fallback",
                    not(portable_atomic_no_outline_atomics),
                    not(any(target_env = "sgx", miri)),
                ),
            ),
        ),
        all(
            target_arch = "riscv64",
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
        ),
        all(
            target_arch = "powerpc64",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            portable_atomic_unstable_asm_experimental_arch,
            any(
                target_feature = "quadword-atomics",
                portable_atomic_target_feature = "quadword-atomics",
                all(
                    feature = "fallback",
                    not(portable_atomic_no_outline_atomics),
                    any(
                        all(
                            target_os = "linux",
                            any(
                                all(
                                    target_env = "gnu",
                                    any(target_endian = "little", not(target_feature = "crt-static")),
                                ),
                                all(
                                    target_env = "musl",
                                    any(not(target_feature = "crt-static"), feature = "std"),
                                ),
                                target_env = "ohos",
                                all(target_env = "uclibc", not(target_feature = "crt-static")),
                                portable_atomic_outline_atomics,
                            ),
                        ),
                        target_os = "android",
                        all(
                            target_os = "freebsd",
                            any(
                                target_endian = "little",
                                not(target_feature = "crt-static"),
                                portable_atomic_outline_atomics,
                            ),
                        ),
                        target_os = "openbsd",
                        all(
                            target_os = "aix",
                            not(portable_atomic_pre_llvm_20),
                            portable_atomic_outline_atomics, // TODO(aix): currently disabled by default
                        ),
                    ),
                    not(any(miri, portable_atomic_sanitize_thread)),
                ),
            ),
        ),
        all(
            target_arch = "s390x",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            not(portable_atomic_no_asm),
        ),
    )))]
    pub(crate) use self::fallback::{AtomicI128, AtomicU128};
}

// 64-bit atomics (platform-specific)
// pre-v6 Arm Linux
#[cfg(feature = "fallback")]
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
pub(crate) use self::atomic64::arm_linux::{AtomicI64, AtomicU64};
// riscv32 & (zacas | outline-atomics)
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
pub(crate) use self::atomic64::riscv32::{AtomicI64, AtomicU64};

// 128-bit atomics (platform-specific)
// AArch64
#[cfg(any(
    all(
        target_arch = "aarch64",
        not(all(
            any(miri, portable_atomic_sanitize_thread),
            not(portable_atomic_atomic_intrinsics),
        )),
        any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
    ),
    all(
        target_arch = "arm64ec",
        not(all(
            any(miri, portable_atomic_sanitize_thread),
            not(portable_atomic_atomic_intrinsics),
        )),
        not(portable_atomic_no_asm),
    ),
))]
pub(crate) use self::atomic128::aarch64::{AtomicI128, AtomicU128};
// x86_64 & (cmpxchg16b | outline-atomics)
#[cfg(all(
    target_arch = "x86_64",
    not(all(any(miri, portable_atomic_sanitize_thread), portable_atomic_no_cmpxchg16b_intrinsic)),
    any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
    any(
        target_feature = "cmpxchg16b",
        portable_atomic_target_feature = "cmpxchg16b",
        all(
            feature = "fallback",
            not(portable_atomic_no_outline_atomics),
            not(any(target_env = "sgx", miri)),
        ),
    ),
))]
pub(crate) use self::atomic128::x86_64::{AtomicI128, AtomicU128};
// riscv64 & (zacas | outline-atomics)
#[cfg(all(
    target_arch = "riscv64",
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
pub(crate) use self::atomic128::riscv64::{AtomicI128, AtomicU128};
// powerpc64 & (pwr8 | outline-atomics)
#[cfg(all(
    target_arch = "powerpc64",
    not(all(
        any(miri, portable_atomic_sanitize_thread),
        not(portable_atomic_atomic_intrinsics),
    )),
    portable_atomic_unstable_asm_experimental_arch,
    any(
        target_feature = "quadword-atomics",
        portable_atomic_target_feature = "quadword-atomics",
        all(
            feature = "fallback",
            not(portable_atomic_no_outline_atomics),
            any(
                all(
                    target_os = "linux",
                    any(
                        all(
                            target_env = "gnu",
                            any(target_endian = "little", not(target_feature = "crt-static")),
                        ),
                        all(
                            target_env = "musl",
                            any(not(target_feature = "crt-static"), feature = "std"),
                        ),
                        target_env = "ohos",
                        all(target_env = "uclibc", not(target_feature = "crt-static")),
                        portable_atomic_outline_atomics,
                    ),
                ),
                target_os = "android",
                all(
                    target_os = "freebsd",
                    any(
                        target_endian = "little",
                        not(target_feature = "crt-static"),
                        portable_atomic_outline_atomics,
                    ),
                ),
                target_os = "openbsd",
                all(
                    target_os = "aix",
                    not(portable_atomic_pre_llvm_20),
                    portable_atomic_outline_atomics, // TODO(aix): currently disabled by default
                ),
            ),
            not(any(miri, portable_atomic_sanitize_thread)),
        ),
    ),
))]
pub(crate) use self::atomic128::powerpc64::{AtomicI128, AtomicU128};
// s390x
#[cfg(all(
    target_arch = "s390x",
    not(all(any(miri, portable_atomic_sanitize_thread), not(portable_atomic_atomic_intrinsics))),
    not(portable_atomic_no_asm),
))]
pub(crate) use self::atomic128::s390x::{AtomicI128, AtomicU128};
