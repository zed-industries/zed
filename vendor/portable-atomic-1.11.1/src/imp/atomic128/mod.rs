// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
128-bit atomic implementations on 64-bit architectures

See README.md for details.
*/

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
// Use intrinsics.rs on Miri and Sanitizer that do not support inline assembly.
#[cfg_attr(any(miri, portable_atomic_sanitize_thread), path = "intrinsics.rs")]
pub(super) mod aarch64;

// powerpc64
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
                    any(test, portable_atomic_outline_atomics), // TODO(aix): currently disabled by default
                ),
            ),
            not(any(miri, portable_atomic_sanitize_thread)),
        ),
    ),
))]
// Use intrinsics.rs on Miri and Sanitizer that do not support inline assembly.
#[cfg_attr(any(miri, portable_atomic_sanitize_thread), path = "intrinsics.rs")]
pub(super) mod powerpc64;

// riscv64
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
pub(super) mod riscv64;

// s390x
#[cfg(all(
    target_arch = "s390x",
    not(all(any(miri, portable_atomic_sanitize_thread), not(portable_atomic_atomic_intrinsics))),
    not(portable_atomic_no_asm),
))]
// Use intrinsics.rs on Miri and Sanitizer that do not support inline assembly.
#[cfg_attr(any(miri, portable_atomic_sanitize_thread), path = "intrinsics.rs")]
pub(super) mod s390x;

// x86_64
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
// Use intrinsics.rs on Miri and Sanitizer that do not support inline assembly.
#[cfg_attr(any(miri, portable_atomic_sanitize_thread), path = "intrinsics.rs")]
pub(super) mod x86_64;
