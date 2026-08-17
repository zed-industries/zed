// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(missing_docs)]

#[cfg(not(all(
    portable_atomic_no_atomic_load_store,
    not(any(
        target_arch = "avr",
        target_arch = "msp430",
        target_arch = "riscv32",
        target_arch = "riscv64",
        feature = "critical-section",
    )),
)))]
#[macro_use]
mod atomic_8_16_macros {
    #[macro_export]
    macro_rules! cfg_has_atomic_8 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    #[macro_export]
    macro_rules! cfg_no_atomic_8 {
        ($($tt:tt)*) => {};
    }
    #[macro_export]
    macro_rules! cfg_has_atomic_16 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    #[macro_export]
    macro_rules! cfg_no_atomic_16 {
        ($($tt:tt)*) => {};
    }
}
#[cfg(all(
    portable_atomic_no_atomic_load_store,
    not(any(
        target_arch = "avr",
        target_arch = "msp430",
        target_arch = "riscv32",
        target_arch = "riscv64",
        feature = "critical-section",
    )),
))]
#[macro_use]
mod atomic_8_16_macros {
    #[macro_export]
    macro_rules! cfg_has_atomic_8 {
        ($($tt:tt)*) => {};
    }
    #[macro_export]
    macro_rules! cfg_no_atomic_8 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    #[macro_export]
    macro_rules! cfg_has_atomic_16 {
        ($($tt:tt)*) => {};
    }
    #[macro_export]
    macro_rules! cfg_no_atomic_16 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
}

#[cfg(all(
    any(not(target_pointer_width = "16"), feature = "fallback"),
    not(all(
        portable_atomic_no_atomic_load_store,
        not(any(
            target_arch = "avr",
            target_arch = "msp430",
            target_arch = "riscv32",
            target_arch = "riscv64",
            feature = "critical-section",
        )),
    )),
))]
#[macro_use]
mod atomic_32_macros {
    #[macro_export]
    macro_rules! cfg_has_atomic_32 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    #[macro_export]
    macro_rules! cfg_no_atomic_32 {
        ($($tt:tt)*) => {};
    }
}
#[cfg(not(all(
    any(not(target_pointer_width = "16"), feature = "fallback"),
    not(all(
        portable_atomic_no_atomic_load_store,
        not(any(
            target_arch = "avr",
            target_arch = "msp430",
            target_arch = "riscv32",
            target_arch = "riscv64",
            feature = "critical-section",
        )),
    )),
)))]
#[macro_use]
mod atomic_32_macros {
    #[macro_export]
    macro_rules! cfg_has_atomic_32 {
        ($($tt:tt)*) => {};
    }
    #[macro_export]
    macro_rules! cfg_no_atomic_32 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
}

#[cfg_attr(
    portable_atomic_no_cfg_target_has_atomic,
    cfg(any(
        all(
            feature = "fallback",
            any(
                not(portable_atomic_no_atomic_cas),
                portable_atomic_unsafe_assume_single_core,
                feature = "critical-section",
                target_arch = "avr",
                target_arch = "msp430",
            ),
        ),
        not(portable_atomic_no_atomic_64),
        not(any(target_pointer_width = "16", target_pointer_width = "32")),
        all(
            target_arch = "riscv32",
            not(any(miri, portable_atomic_sanitize_thread)),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            any(target_feature = "zacas", portable_atomic_target_feature = "zacas"),
        ),
    ))
)]
#[cfg_attr(
    not(portable_atomic_no_cfg_target_has_atomic),
    cfg(any(
        all(
            feature = "fallback",
            any(
                target_has_atomic = "ptr",
                portable_atomic_unsafe_assume_single_core,
                feature = "critical-section",
                target_arch = "avr",
                target_arch = "msp430",
            ),
        ),
        target_has_atomic = "64",
        not(any(target_pointer_width = "16", target_pointer_width = "32")),
        all(
            target_arch = "riscv32",
            not(any(miri, portable_atomic_sanitize_thread)),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            any(target_feature = "zacas", portable_atomic_target_feature = "zacas"),
        ),
    ))
)]
#[macro_use]
mod atomic_64_macros {
    #[macro_export]
    macro_rules! cfg_has_atomic_64 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    #[macro_export]
    macro_rules! cfg_no_atomic_64 {
        ($($tt:tt)*) => {};
    }
}
#[cfg_attr(
    portable_atomic_no_cfg_target_has_atomic,
    cfg(not(any(
        all(
            feature = "fallback",
            any(
                not(portable_atomic_no_atomic_cas),
                portable_atomic_unsafe_assume_single_core,
                feature = "critical-section",
                target_arch = "avr",
                target_arch = "msp430",
            ),
        ),
        not(portable_atomic_no_atomic_64),
        not(any(target_pointer_width = "16", target_pointer_width = "32")),
        all(
            target_arch = "riscv32",
            not(any(miri, portable_atomic_sanitize_thread)),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            any(target_feature = "zacas", portable_atomic_target_feature = "zacas"),
        ),
    )))
)]
#[cfg_attr(
    not(portable_atomic_no_cfg_target_has_atomic),
    cfg(not(any(
        all(
            feature = "fallback",
            any(
                target_has_atomic = "ptr",
                portable_atomic_unsafe_assume_single_core,
                feature = "critical-section",
                target_arch = "avr",
                target_arch = "msp430",
            ),
        ),
        target_has_atomic = "64",
        not(any(target_pointer_width = "16", target_pointer_width = "32")),
        all(
            target_arch = "riscv32",
            not(any(miri, portable_atomic_sanitize_thread)),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            any(target_feature = "zacas", portable_atomic_target_feature = "zacas"),
        ),
    )))
)]
#[macro_use]
mod atomic_64_macros {
    #[macro_export]
    macro_rules! cfg_has_atomic_64 {
        ($($tt:tt)*) => {};
    }
    #[macro_export]
    macro_rules! cfg_no_atomic_64 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
}

#[cfg_attr(
    not(feature = "fallback"),
    cfg(any(
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
)]
#[cfg_attr(
    all(feature = "fallback", portable_atomic_no_cfg_target_has_atomic),
    cfg(any(
        not(portable_atomic_no_atomic_cas),
        portable_atomic_unsafe_assume_single_core,
        feature = "critical-section",
        target_arch = "avr",
        target_arch = "msp430",
    ))
)]
#[cfg_attr(
    all(feature = "fallback", not(portable_atomic_no_cfg_target_has_atomic)),
    cfg(any(
        target_has_atomic = "ptr",
        portable_atomic_unsafe_assume_single_core,
        feature = "critical-section",
        target_arch = "avr",
        target_arch = "msp430",
    ))
)]
#[macro_use]
mod atomic_128_macros {
    #[macro_export]
    macro_rules! cfg_has_atomic_128 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    #[macro_export]
    macro_rules! cfg_no_atomic_128 {
        ($($tt:tt)*) => {};
    }
}
#[cfg_attr(
    not(feature = "fallback"),
    cfg(not(any(
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
    )))
)]
#[cfg_attr(
    all(feature = "fallback", portable_atomic_no_cfg_target_has_atomic),
    cfg(not(any(
        not(portable_atomic_no_atomic_cas),
        portable_atomic_unsafe_assume_single_core,
        feature = "critical-section",
        target_arch = "avr",
        target_arch = "msp430",
    )))
)]
#[cfg_attr(
    all(feature = "fallback", not(portable_atomic_no_cfg_target_has_atomic)),
    cfg(not(any(
        target_has_atomic = "ptr",
        portable_atomic_unsafe_assume_single_core,
        feature = "critical-section",
        target_arch = "avr",
        target_arch = "msp430",
    )))
)]
#[macro_use]
mod atomic_128_macros {
    #[macro_export]
    macro_rules! cfg_has_atomic_128 {
        ($($tt:tt)*) => {};
    }
    #[macro_export]
    macro_rules! cfg_no_atomic_128 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
}

#[cfg_attr(
    portable_atomic_no_cfg_target_has_atomic,
    cfg(any(
        not(portable_atomic_no_atomic_cas),
        portable_atomic_unsafe_assume_single_core,
        feature = "critical-section",
        target_arch = "avr",
        target_arch = "msp430",
    ))
)]
#[cfg_attr(
    not(portable_atomic_no_cfg_target_has_atomic),
    cfg(any(
        target_has_atomic = "ptr",
        portable_atomic_unsafe_assume_single_core,
        feature = "critical-section",
        target_arch = "avr",
        target_arch = "msp430",
    ))
)]
#[macro_use]
mod atomic_cas_macros {
    #[macro_export]
    macro_rules! cfg_has_atomic_cas {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    #[macro_export]
    macro_rules! cfg_no_atomic_cas {
        ($($tt:tt)*) => {};
    }
    // private
    macro_rules! cfg_has_atomic_cas_or_amo32 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    macro_rules! cfg_has_atomic_cas_or_amo8 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
}
#[cfg_attr(
    portable_atomic_no_cfg_target_has_atomic,
    cfg(not(any(
        not(portable_atomic_no_atomic_cas),
        portable_atomic_unsafe_assume_single_core,
        feature = "critical-section",
        target_arch = "avr",
        target_arch = "msp430",
    )))
)]
#[cfg_attr(
    not(portable_atomic_no_cfg_target_has_atomic),
    cfg(not(any(
        target_has_atomic = "ptr",
        portable_atomic_unsafe_assume_single_core,
        feature = "critical-section",
        target_arch = "avr",
        target_arch = "msp430",
    )))
)]
#[macro_use]
mod atomic_cas_macros {
    #[macro_export]
    macro_rules! cfg_has_atomic_cas {
        ($($tt:tt)*) => {};
    }
    #[macro_export]
    macro_rules! cfg_no_atomic_cas {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    // private
    #[cfg_attr(
        any(target_arch = "riscv32", target_arch = "riscv64"),
        cfg(not(any(target_feature = "zaamo", portable_atomic_target_feature = "zaamo")))
    )]
    macro_rules! cfg_has_atomic_cas_or_amo32 {
        ($($tt:tt)*) => {};
    }
    #[cfg_attr(
        any(target_arch = "riscv32", target_arch = "riscv64"),
        cfg(not(any(target_feature = "zaamo", portable_atomic_target_feature = "zaamo")))
    )]
    macro_rules! cfg_no_atomic_cas_or_amo32 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    #[cfg(all(
        any(target_arch = "riscv32", target_arch = "riscv64"),
        any(target_feature = "zaamo", portable_atomic_target_feature = "zaamo"),
    ))]
    macro_rules! cfg_has_atomic_cas_or_amo32 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    #[cfg(all(
        any(target_arch = "riscv32", target_arch = "riscv64"),
        any(target_feature = "zaamo", portable_atomic_target_feature = "zaamo"),
    ))]
    macro_rules! cfg_no_atomic_cas_or_amo32 {
        ($($tt:tt)*) => {};
    }
    #[cfg_attr(
        any(target_arch = "riscv32", target_arch = "riscv64"),
        cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))
    )]
    #[allow(unused_macros)]
    macro_rules! cfg_has_atomic_cas_or_amo8 {
        ($($tt:tt)*) => {};
    }
    #[cfg_attr(
        any(target_arch = "riscv32", target_arch = "riscv64"),
        cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))
    )]
    #[cfg_attr(target_arch = "bpf", allow(unused_macros))]
    macro_rules! cfg_no_atomic_cas_or_amo8 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    #[cfg(all(
        any(target_arch = "riscv32", target_arch = "riscv64"),
        any(target_feature = "zabha", portable_atomic_target_feature = "zabha"),
    ))]
    macro_rules! cfg_has_atomic_cas_or_amo8 {
        ($($tt:tt)*) => {
            $($tt)*
        };
    }
    #[cfg(all(
        any(target_arch = "riscv32", target_arch = "riscv64"),
        any(target_feature = "zabha", portable_atomic_target_feature = "zabha"),
    ))]
    macro_rules! cfg_no_atomic_cas_or_amo8 {
        ($($tt:tt)*) => {};
    }
}

// Check that all cfg_ macros work.
mod check {
    crate::cfg_has_atomic_8! { type _Atomic8 = (); }
    crate::cfg_no_atomic_8! { type _Atomic8 = (); }
    crate::cfg_has_atomic_16! { type _Atomic16 = (); }
    crate::cfg_no_atomic_16! { type _Atomic16 = (); }
    crate::cfg_has_atomic_32! { type _Atomic32 = (); }
    crate::cfg_no_atomic_32! { type _Atomic32 = (); }
    crate::cfg_has_atomic_64! { type _Atomic64 = (); }
    crate::cfg_no_atomic_64! { type _Atomic64 = (); }
    crate::cfg_has_atomic_128! { type _Atomic128 = (); }
    crate::cfg_no_atomic_128! { type _Atomic128 = (); }
    crate::cfg_has_atomic_ptr! { type _AtomicPtr = (); }
    crate::cfg_no_atomic_ptr! { type _AtomicPtr = (); }
    crate::cfg_has_atomic_cas! { type __AtomicPtr = (); }
    crate::cfg_no_atomic_cas! { type __AtomicPtr = (); }
    #[allow(unused_imports)]
    use self::{
        __AtomicPtr as _, _Atomic8 as _, _Atomic16 as _, _Atomic32 as _, _Atomic64 as _,
        _Atomic128 as _, _AtomicPtr as _,
    };
}
