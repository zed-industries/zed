// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Run-time CPU feature detection on AArch64/PowerPC64 Linux/Android/FreeBSD/OpenBSD by parsing ELF auxiliary vectors.

Supported platforms:
- Linux 6.4+ (through prctl)
  https://github.com/torvalds/linux/commit/ddc65971bb677aa9f6a4c21f76d3133e106f88eb
  prctl returns an unsupported error if operation is not supported,
  so we can safely use this on older versions.
- glibc 2.16+ (through getauxval)
  https://github.com/bminor/glibc/commit/c7683a6d02f3ed59f5cd119b3e8547f45a15912f
  Always available on:
  - aarch64 (glibc 2.17+ https://github.com/bminor/glibc/blob/glibc-2.17/NEWS#L36)
  - powerpc64 (le) (glibc 2.19+ or RHEL/CentOS's patched glibc 2.17+ https://github.com/bminor/glibc/blob/glibc-2.19/NEWS#L108)
  Not always available on:
  - powerpc64 (be) (glibc 2.3+ https://github.com/bminor/glibc/blob/glibc-2.3/NEWS#L56)
  Since Rust 1.64, std requires glibc 2.17+ https://blog.rust-lang.org/2022/08/01/Increasing-glibc-kernel-requirements.html
- musl 1.1.0+ (through getauxval)
  https://github.com/bminor/musl/commit/21ada94c4b8c01589367cea300916d7db8461ae7
  Always available on:
  - aarch64 (musl 1.1.7+ https://github.com/bminor/musl/blob/v1.1.7/WHATSNEW#L1422)
  - powerpc64 (musl 1.1.15+ https://github.com/bminor/musl/blob/v1.1.15/WHATSNEW#L1702)
  At least since Rust 1.15, std requires musl 1.1.14+ https://github.com/rust-lang/rust/blob/1.15.0/src/ci/docker/x86_64-musl/build-musl.sh#L15
  Since Rust 1.18, std requires musl 1.1.16+ https://github.com/rust-lang/rust/pull/41089
  Since Rust 1.23, std requires musl 1.1.17+ https://github.com/rust-lang/rust/pull/45393
  Since Rust 1.25, std requires musl 1.1.18+ https://github.com/rust-lang/rust/pull/47283
  Since Rust 1.29, std requires musl 1.1.19+ https://github.com/rust-lang/rust/pull/52087
  Since Rust 1.31, std requires musl 1.1.20+ https://github.com/rust-lang/rust/pull/54430
  Since Rust 1.37, std requires musl 1.1.22+ https://github.com/rust-lang/rust/pull/61252
  Since Rust 1.46, std requires musl 1.1.24+ https://github.com/rust-lang/rust/pull/73089
  Since Rust 1.71, std requires musl 1.2.3+ https://blog.rust-lang.org/2023/05/09/Updating-musl-targets.html
  OpenHarmony uses a fork of musl 1.2 https://gitee.com/openharmony/docs/blob/master/en/application-dev/reference/native-lib/musl.md
- uClibc-ng 1.0.43+ (through getauxval)
  https://github.com/wbx-github/uclibc-ng/commit/d869bb1600942c01a77539128f9ba5b5b55ad647
  Not always available on:
  - aarch64 (uClibc-ng 1.0.22+ https://github.com/wbx-github/uclibc-ng/commit/dba942c80dc2cfa5768a856fff98e22a755fdd27)
  (powerpc64 is not supported https://github.com/wbx-github/uclibc-ng/commit/d4d4f37fda7fa57e57132ff2f0d735ce7cc2178e)
  In L4Re which uses uClibc-ng, a dummy API was added first in 2020 that always returns 0 (https://github.com/kernkonzept/l4re-core/commit/e88fa67198074d3e6b4983c5c8af1538e2089ff3),
  then implemented in 2024 (https://github.com/kernkonzept/l4re-core/commit/3ee2a50dd1b3bc22955e593004990887a0a5b4a3).
  However, getauxval(AT_HWCAP*) always returns 0 (as of 2025-03-20). (see tests/l4re test)
- Picolibc 1.4.6+ (through getauxval)
  https://github.com/picolibc/picolibc/commit/19bfe51d62ad7e32533c7f664b5bca8e26286e31
- Android 4.3+ (API level 18+) (through getauxval)
  https://github.com/aosp-mirror/platform_bionic/commit/2c5153b043b44e9935a334ae9b2d5a4bc5258b40
  https://github.com/aosp-mirror/platform_bionic/commit/655e430b28d7404f763e7ebefe84fba5a387666d
  Always available on:
  - 64-bit architectures (Android 5.0+ (API level 21+) https://android-developers.googleblog.com/2014/10/whats-new-in-android-50-lollipop.html)
  Since Rust 1.68, std requires API level 19+ https://blog.rust-lang.org/2023/01/09/android-ndk-update-r25.html
  Since Rust 1.82, std requires API level 21+ https://github.com/rust-lang/rust/pull/120593
- FreeBSD 12.0+ and 11.4+ (through elf_aux_info)
  https://github.com/freebsd/freebsd-src/commit/0b08ae2120cdd08c20a2b806e2fcef4d0a36c470
  https://github.com/freebsd/freebsd-src/blob/release/11.4.0/sys/sys/auxv.h
  Always available on:
  - powerpc64 (le) (FreeBSD 12.4+ https://www.freebsd.org/releases/12.4R/announce, https://man.freebsd.org/cgi/man.cgi?arch)
  Not always available on:
  - aarch64 (FreeBSD 11.0+ https://www.freebsd.org/releases/11.0R/announce, https://man.freebsd.org/cgi/man.cgi?arch)
  - powerpc64 (be) (FreeBSD 9.0+ https://www.freebsd.org/releases/9.0R/announce, https://man.freebsd.org/cgi/man.cgi?arch)
  Since Rust 1.75, std requires FreeBSD 12+ https://github.com/rust-lang/rust/pull/114521
  Dropping support for FreeBSD 12 in std was decided in https://github.com/rust-lang/rust/pull/120869,
  but the actual update to the FreeBSD 13 toolchain was attempted twice, but both times there were
  problems, so they were reverted: https://github.com/rust-lang/rust/pull/132228 https://github.com/rust-lang/rust/pull/136582
- OpenBSD 7.6+ (through elf_aux_info)
  https://github.com/openbsd/src/commit/ef873df06dac50249b2dd380dc6100eee3b0d23d
  Not always available on:
  - aarch64 (OpenBSD 6.1+ https://www.openbsd.org/61.html)
  - powerpc64 (OpenBSD 6.8+ https://www.openbsd.org/68.html)

On platforms that we can assume that getauxval/elf_aux_info is always available, we directly call
them on except for musl with static linking. (At this time, we also retain compatibility with
versions that reached EoL or no longer supported by `std`, with the exception of AArch64 FreeBSD described below.)

On musl with static linking, it seems that getauxval is not always available, independent of version
requirements: https://github.com/rust-lang/rust/issues/89626
(That problem may have been fixed in https://github.com/rust-lang/rust/commit/9a04ae4997493e9260352064163285cddc43de3c,
but even in the version containing that patch, [there is report](https://github.com/rust-lang/rust/issues/89626#issuecomment-1242636038)
of the same error.)
This seems to be due to the fact that compiler-builtins is built before libc (which is a dependency
of std) links musl. And the std and its dependent can use getauxval without this problem at least
since the rust-lang/rust patch mentioned above:
https://github.com/rust-lang/rust/blob/1.85.0/library/std/src/sys/pal/unix/stack_overflow.rs#L268
(According to https://github.com/rust-lang/rust/issues/89626#issuecomment-2420469392, this problem
may have been fixed in https://github.com/rust-lang/rust/commit/9ed0d11efbec18a1fa4155576a3bcb685676d23c.)
See also https://github.com/rust-lang/stdarch/pull/1746.
So as for musl with static linking, we assume that getauxval is always available also when `std` feature enabled.

On platforms that we cannot assume that getauxval/elf_aux_info is always available, so we use dlsym
instead of directly calling getauxval/elf_aux_info. (You can force getauxval/elf_aux_info to be
called directly instead of using dlsym by `--cfg portable_atomic_outline_atomics`).

Also, note that dlsym usually not working with static linking.

# Linux/Android

As of Rust 1.69, is_aarch64_feature_detected always uses dlsym by default
on AArch64 Linux/Android, but on some platforms, we can safely assume
getauxval is linked to the binary (see the above).

See also https://github.com/rust-lang/stdarch/pull/1375

See tests::test_alternative and aarch64_aa64reg.rs for (test-only) alternative implementations.

# FreeBSD

As of nightly-2024-09-07, is_aarch64_feature_detected always uses mrs on
AArch64 FreeBSD. However, they do not work on FreeBSD 12 on QEMU (confirmed
on FreeBSD 12.{2,3,4}), and we got SIGILL (worked on FreeBSD 13 and 14).

So use elf_aux_info instead of mrs like compiler-rt does.
https://reviews.llvm.org/D109330

elf_aux_info is available on FreeBSD 12.0+ and 11.4+:
https://github.com/freebsd/freebsd-src/commit/0b08ae2120cdd08c20a2b806e2fcef4d0a36c470
https://github.com/freebsd/freebsd-src/blob/release/11.4.0/sys/sys/auxv.h
On FreeBSD, [AArch64 support is available on FreeBSD 11.0+](https://www.freebsd.org/releases/11.0R/announce),
but FreeBSD 11 (11.4) was EoL on 2021-09-30, and FreeBSD 11.3 was EoL on 2020-09-30:
https://www.freebsd.org/security/unsupported
See also https://github.com/rust-lang/stdarch/pull/611#issuecomment-445464613

See tests::test_alternative and aarch64_aa64reg.rs for (test-only) alternative implementations.

# OpenBSD

elf_aux_info is available on OpenBSD 7.6+:
https://github.com/openbsd/src/commit/ef873df06dac50249b2dd380dc6100eee3b0d23d

On AArch64, there is an alternative that available on older version,
so we use it (see aarch64_aa64reg.rs).

*/

include!("common.rs");

use self::os::ffi;
#[cfg(any(target_os = "linux", target_os = "android"))]
mod os {
    // libc requires Rust 1.63
    #[cfg_attr(test, allow(dead_code))]
    pub(super) mod ffi {
        pub(crate) use crate::utils::ffi::c_ulong;
        #[allow(unused_imports)]
        pub(crate) use crate::utils::ffi::{c_char, c_int, c_void};

        sys_const!({
            // https://github.com/torvalds/linux/blob/v6.13/include/uapi/linux/auxvec.h
            pub(crate) const AT_HWCAP: c_ulong = 16;
            #[cfg(any(
                test,
                all(target_arch = "aarch64", target_pointer_width = "64"),
                target_arch = "powerpc64",
            ))]
            pub(crate) const AT_HWCAP2: c_ulong = 26;

            // Defined in dlfcn.h.
            // https://github.com/bminor/glibc/blob/glibc-2.40/dlfcn/dlfcn.h
            // https://github.com/bminor/musl/blob/v1.2.5/include/dlfcn.h
            // https://github.com/wbx-github/uclibc-ng/blob/v1.0.47/include/dlfcn.h
            // https://github.com/aosp-mirror/platform_bionic/blob/android-15.0.0_r1/libc/include/dlfcn.h
            #[cfg(any(
                test,
                not(any(
                    all(
                        target_os = "linux",
                        any(
                            all(
                                target_env = "gnu",
                                any(
                                    target_arch = "aarch64",
                                    all(target_arch = "powerpc64", target_endian = "little"),
                                ),
                            ),
                            target_env = "musl",
                            target_env = "ohos",
                        ),
                    ),
                    all(target_os = "android", target_pointer_width = "64"),
                    portable_atomic_outline_atomics,
                )),
            ))]
            pub(crate) const RTLD_DEFAULT: *mut c_void = core::ptr::null_mut();

            // Defined in sys/system_properties.h.
            // https://github.com/aosp-mirror/platform_bionic/blob/android-15.0.0_r1/libc/include/sys/system_properties.h
            #[cfg(all(target_arch = "aarch64", target_os = "android"))]
            pub(crate) const PROP_VALUE_MAX: c_int = 92;
        });

        sys_fn!({
            extern "C" {
                // Defined in sys/auxv.h.
                // https://man7.org/linux/man-pages/man3/getauxval.3.html
                // https://github.com/bminor/glibc/blob/glibc-2.40/misc/sys/auxv.h
                // https://github.com/bminor/musl/blob/v1.2.5/include/sys/auxv.h
                // https://github.com/wbx-github/uclibc-ng/blob/v1.0.47/include/sys/auxv.h
                // https://github.com/kernkonzept/l4re-core/blob/4351d4474804636122d64ea5a5d41f5e78e9208e/uclibc/lib/contrib/uclibc/include/sys/auxv.h
                // https://github.com/aosp-mirror/platform_bionic/blob/android-15.0.0_r1/libc/include/sys/auxv.h
                // https://github.com/picolibc/picolibc/blob/1.8.6/newlib/libc/include/sys/auxv.h
                #[cfg(any(
                    test,
                    all(
                        target_os = "linux",
                        any(
                            all(
                                target_env = "gnu",
                                any(
                                    target_arch = "aarch64",
                                    all(target_arch = "powerpc64", target_endian = "little"),
                                ),
                            ),
                            target_env = "musl",
                            target_env = "ohos",
                        ),
                    ),
                    all(target_os = "android", target_pointer_width = "64"),
                    portable_atomic_outline_atomics,
                ))]
                pub(crate) fn getauxval(type_: c_ulong) -> c_ulong;

                // Defined in dlfcn.h.
                // https://man7.org/linux/man-pages/man3/dlsym.3.html
                // https://github.com/bminor/glibc/blob/glibc-2.40/dlfcn/dlfcn.h
                // https://github.com/bminor/musl/blob/v1.2.5/include/dlfcn.h
                // https://github.com/wbx-github/uclibc-ng/blob/v1.0.47/include/dlfcn.h
                // https://github.com/aosp-mirror/platform_bionic/blob/android-15.0.0_r1/libc/include/dlfcn.h
                #[cfg(any(
                    test,
                    not(any(
                        all(
                            target_os = "linux",
                            any(
                                all(
                                    target_env = "gnu",
                                    any(
                                        target_arch = "aarch64",
                                        all(target_arch = "powerpc64", target_endian = "little"),
                                    ),
                                ),
                                target_env = "musl",
                                target_env = "ohos",
                            ),
                        ),
                        all(target_os = "android", target_pointer_width = "64"),
                        portable_atomic_outline_atomics,
                    )),
                ))]
                pub(crate) fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;

                // Defined in sys/system_properties.h.
                // https://github.com/aosp-mirror/platform_bionic/blob/android-15.0.0_r1/libc/include/sys/system_properties.h
                #[cfg(all(target_arch = "aarch64", target_os = "android"))]
                pub(crate) fn __system_property_get(
                    name: *const c_char,
                    value: *mut c_char,
                ) -> c_int;
            }
        });
    }

    pub(super) type GetauxvalTy = unsafe extern "C" fn(ffi::c_ulong) -> ffi::c_ulong;
    pub(super) fn getauxval(type_: ffi::c_ulong) -> ffi::c_ulong {
        #[cfg(any(
            all(
                target_os = "linux",
                any(
                    all(
                        target_env = "gnu",
                        any(
                            target_arch = "aarch64",
                            all(target_arch = "powerpc64", target_endian = "little"),
                        ),
                    ),
                    target_env = "musl",
                    target_env = "ohos",
                ),
            ),
            all(target_os = "android", target_pointer_width = "64"),
            portable_atomic_outline_atomics,
        ))]
        let getauxval: GetauxvalTy = ffi::getauxval;
        #[cfg(not(any(
            all(
                target_os = "linux",
                any(
                    all(
                        target_env = "gnu",
                        any(
                            target_arch = "aarch64",
                            all(target_arch = "powerpc64", target_endian = "little"),
                        ),
                    ),
                    target_env = "musl",
                    target_env = "ohos",
                ),
            ),
            all(target_os = "android", target_pointer_width = "64"),
            portable_atomic_outline_atomics,
        )))]
        // SAFETY: we passed a valid C string to dlsym, and a pointer returned by dlsym
        // is a valid pointer to the function if it is non-null.
        let getauxval: GetauxvalTy = unsafe {
            let ptr = ffi::dlsym(ffi::RTLD_DEFAULT, c!("getauxval").as_ptr());
            if ptr.is_null() {
                return 0;
            }
            core::mem::transmute::<*mut ffi::c_void, GetauxvalTy>(ptr)
        };

        // SAFETY: `getauxval` is thread-safe.
        unsafe { getauxval(type_) }
    }
}
#[cfg(any(target_os = "freebsd", target_os = "openbsd"))]
mod os {
    use core::mem;

    // libc requires Rust 1.63
    #[cfg_attr(test, allow(dead_code))]
    pub(super) mod ffi {
        #[allow(unused_imports)]
        pub(crate) use crate::utils::ffi::c_char;
        pub(crate) use crate::utils::ffi::{c_int, c_ulong, c_void};

        sys_const!({
            // FreeBSD
            // Defined in sys/elf_common.h.
            // https://github.com/freebsd/freebsd-src/blob/release/14.2.0/sys/sys/elf_common.h
            // OpenBSD
            // Defined in sys/auxv.h.
            // https://github.com/openbsd/src/blob/ed8f5e8d82ace15e4cefca2c82941b15cb1a7830/sys/sys/auxv.h
            pub(crate) const AT_HWCAP: c_int = 25;
            #[cfg(any(
                test,
                all(target_os = "freebsd", target_arch = "aarch64", target_pointer_width = "64"),
                target_arch = "powerpc64",
            ))]
            pub(crate) const AT_HWCAP2: c_int = 26;

            // FreeBSD
            // Defined in dlfcn.h.
            // https://man.freebsd.org/dlsym(3)
            // https://github.com/freebsd/freebsd-src/blob/release/14.2.0/include/dlfcn.h
            // OpenBSD
            // Defined in dlfcn.h.
            // https://man.openbsd.org/dlsym.3
            // https://github.com/openbsd/src/blob/ed8f5e8d82ace15e4cefca2c82941b15cb1a7830/include/dlfcn.h
            #[cfg(any(
                test,
                not(any(
                    all(
                        target_os = "freebsd",
                        any(
                            target_arch = "aarch64",
                            all(target_arch = "powerpc64", target_endian = "little"),
                        ),
                    ),
                    portable_atomic_outline_atomics,
                )),
            ))]
            #[allow(clippy::cast_sign_loss)]
            pub(crate) const RTLD_DEFAULT: *mut c_void = -2_isize as usize as *mut c_void;
        });

        sys_fn!({
            extern "C" {
                // FreeBSD
                // Defined in sys/auxv.h.
                // https://man.freebsd.org/elf_aux_info(3)
                // https://github.com/freebsd/freebsd-src/blob/release/14.2.0/sys/sys/auxv.h
                // OpenBSD
                // Defined in sys/auxv.h.
                // https://man.openbsd.org/elf_aux_info.3
                // https://github.com/openbsd/src/blob/ed8f5e8d82ace15e4cefca2c82941b15cb1a7830/sys/sys/auxv.h
                #[cfg(any(
                    test,
                    any(
                        all(
                            target_os = "freebsd",
                            any(
                                target_arch = "aarch64",
                                all(target_arch = "powerpc64", target_endian = "little"),
                            ),
                        ),
                        portable_atomic_outline_atomics,
                    ),
                ))]
                pub(crate) fn elf_aux_info(aux: c_int, buf: *mut c_void, buf_len: c_int) -> c_int;

                // FreeBSD
                // Defined in dlfcn.h.
                // https://man.freebsd.org/dlsym(3)
                // https://github.com/freebsd/freebsd-src/blob/release/14.2.0/include/dlfcn.h
                // OpenBSD
                // Defined in dlfcn.h.
                // https://man.openbsd.org/dlsym.3
                // https://github.com/openbsd/src/blob/ed8f5e8d82ace15e4cefca2c82941b15cb1a7830/include/dlfcn.h
                #[cfg(any(
                    test,
                    not(any(
                        all(
                            target_os = "freebsd",
                            any(
                                target_arch = "aarch64",
                                all(target_arch = "powerpc64", target_endian = "little"),
                            ),
                        ),
                        portable_atomic_outline_atomics,
                    )),
                ))]
                pub(crate) fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
            }
        });
    }

    pub(super) type ElfAuxInfoTy =
        unsafe extern "C" fn(ffi::c_int, *mut ffi::c_void, ffi::c_int) -> ffi::c_int;
    pub(super) fn getauxval(aux: ffi::c_int) -> ffi::c_ulong {
        #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
        const OUT_LEN: ffi::c_int = mem::size_of::<ffi::c_ulong>() as ffi::c_int;

        #[cfg(any(
            all(
                target_os = "freebsd",
                any(
                    target_arch = "aarch64",
                    all(target_arch = "powerpc64", target_endian = "little"),
                ),
            ),
            portable_atomic_outline_atomics,
        ))]
        let elf_aux_info: ElfAuxInfoTy = ffi::elf_aux_info;
        #[cfg(not(any(
            all(
                target_os = "freebsd",
                any(
                    target_arch = "aarch64",
                    all(target_arch = "powerpc64", target_endian = "little"),
                ),
            ),
            portable_atomic_outline_atomics,
        )))]
        // SAFETY: we passed a valid C string to dlsym, and a pointer returned by dlsym
        // is a valid pointer to the function if it is non-null.
        let elf_aux_info: ElfAuxInfoTy = unsafe {
            let ptr = ffi::dlsym(ffi::RTLD_DEFAULT, c!("elf_aux_info").as_ptr());
            if ptr.is_null() {
                return 0;
            }
            mem::transmute::<*mut ffi::c_void, ElfAuxInfoTy>(ptr)
        };

        let mut out: ffi::c_ulong = 0;
        // SAFETY:
        // - the pointer is valid because we got it from a reference.
        // - `OUT_LEN` is the same as the size of `out`.
        // - `elf_aux_info` is thread-safe.
        let res = unsafe {
            elf_aux_info(aux, (&mut out as *mut ffi::c_ulong).cast::<ffi::c_void>(), OUT_LEN)
        };
        // If elf_aux_info fails, `out` will be left at zero (which is the proper default value).
        debug_assert!(res == 0 || out == 0);
        out
    }
}

// Basically, Linux/FreeBSD/OpenBSD use the same hwcap values.
// FreeBSD and OpenBSD usually support a subset of the hwcap values supported by Linux.
use self::arch::_detect;
#[cfg(target_arch = "aarch64")]
mod arch {
    use super::{CpuInfo, CpuInfoFlag, ffi, os};

    sys_const!({
        // Linux
        // https://github.com/torvalds/linux/blob/v6.13/arch/arm64/include/uapi/asm/hwcap.h
        // https://github.com/torvalds/linux/blob/v6.13/Documentation/arch/arm64/elf_hwcaps.rst
        // FreeBSD
        // Defined in machine/elf.h.
        // https://github.com/freebsd/freebsd-src/blob/release/14.2.0/sys/arm64/include/elf.h
        // OpenBSD
        // Defined in machine/elf.h.
        // https://github.com/openbsd/src/blob/ed8f5e8d82ace15e4cefca2c82941b15cb1a7830/sys/arch/arm64/include/elf.h
        // Linux 4.3+
        // https://github.com/torvalds/linux/commit/40a1db2434a1b62332b1af25cfa14d7b8c0301fe
        // FreeBSD 13.0+/12.2+
        // https://github.com/freebsd/freebsd-src/blob/release/13.0.0/sys/arm64/include/elf.h
        // https://github.com/freebsd/freebsd-src/blob/release/12.2.0/sys/arm64/include/elf.h
        // OpenBSD 7.6+
        // https://github.com/openbsd/src/commit/ef873df06dac50249b2dd380dc6100eee3b0d23d
        pub(crate) const HWCAP_ATOMICS: ffi::c_ulong = 1 << 8;
        // Linux 4.11+
        // https://github.com/torvalds/linux/commit/77c97b4ee21290f5f083173d957843b615abbff2
        // FreeBSD 13.0+/12.2+
        // https://github.com/freebsd/freebsd-src/blob/release/13.0.0/sys/arm64/include/elf.h
        // https://github.com/freebsd/freebsd-src/blob/release/12.2.0/sys/arm64/include/elf.h
        // OpenBSD 7.6+
        // https://github.com/openbsd/src/commit/ef873df06dac50249b2dd380dc6100eee3b0d23d
        #[cfg(test)]
        pub(crate) const HWCAP_CPUID: ffi::c_ulong = 1 << 11;
        // Linux 4.17+
        // https://github.com/torvalds/linux/commit/7206dc93a58fb76421c4411eefa3c003337bcb2d
        // FreeBSD 13.0+/12.2+
        // https://github.com/freebsd/freebsd-src/blob/release/13.0.0/sys/arm64/include/elf.h
        // https://github.com/freebsd/freebsd-src/blob/release/12.2.0/sys/arm64/include/elf.h
        // OpenBSD 7.6+
        // https://github.com/openbsd/src/commit/ef873df06dac50249b2dd380dc6100eee3b0d23d
        pub(crate) const HWCAP_USCAT: ffi::c_ulong = 1 << 25;
        // Linux 6.7+
        // https://github.com/torvalds/linux/commit/338a835f40a849cd89b993e342bd9fbd5684825c
        // FreeBSD 15.0+
        // https://github.com/freebsd/freebsd-src/commit/94686b081fdb0c1bb0fc1dfeda14bd53f26ce7c5
        #[cfg(not(target_os = "openbsd"))]
        #[cfg(target_pointer_width = "64")]
        pub(crate) const HWCAP2_LRCPC3: ffi::c_ulong = 1 << 46;
        // Linux 6.7+
        // https://github.com/torvalds/linux/commit/94d0657f9f0d311489606589133ebf49e28104d8
        // FreeBSD 15.0+
        // https://github.com/freebsd/freebsd-src/commit/94686b081fdb0c1bb0fc1dfeda14bd53f26ce7c5
        #[cfg(not(target_os = "openbsd"))]
        #[cfg(target_pointer_width = "64")]
        pub(crate) const HWCAP2_LSE128: ffi::c_ulong = 1 << 47;
    });

    #[cold]
    pub(super) fn _detect(info: &mut CpuInfo) {
        #[cfg(target_os = "android")]
        {
            // Samsung Exynos 9810 has a bug that big and little cores have different
            // ISAs. And on older Android (pre-9), the kernel incorrectly reports
            // that features available only on some cores are available on all cores.
            // https://reviews.llvm.org/D114523
            let mut arch = [0_u8; ffi::PROP_VALUE_MAX as usize];
            // SAFETY: we've passed a valid C string and a buffer with max length.
            let len = unsafe {
                ffi::__system_property_get(
                    c!("ro.arch").as_ptr(),
                    arch.as_mut_ptr().cast::<ffi::c_char>(),
                )
            };
            // On Exynos, ro.arch is not available on Android 12+, but it is fine
            // because Android 9+ includes the fix.
            if len > 0 && arch.starts_with(b"exynos9810") {
                return;
            }
        }

        macro_rules! check {
            ($x:ident, $flag:ident, $bit:ident) => {
                if $x & $bit != 0 {
                    info.set(CpuInfoFlag::$flag);
                }
            };
        }
        let hwcap = os::getauxval(ffi::AT_HWCAP);
        check!(hwcap, lse, HWCAP_ATOMICS);
        check!(hwcap, lse2, HWCAP_USCAT);
        #[cfg(test)]
        check!(hwcap, cpuid, HWCAP_CPUID);
        #[cfg(not(target_os = "openbsd"))]
        // HWCAP2 is not yet available on ILP32: https://git.kernel.org/pub/scm/linux/kernel/git/arm64/linux.git/tree/arch/arm64/include/uapi/asm/hwcap.h?h=staging/ilp32-5.1
        #[cfg(target_pointer_width = "64")]
        {
            let hwcap2 = os::getauxval(ffi::AT_HWCAP2);
            check!(hwcap2, rcpc3, HWCAP2_LRCPC3);
            check!(hwcap2, lse128, HWCAP2_LSE128);
        }
    }
}
#[cfg(target_arch = "powerpc64")]
mod arch {
    use super::{CpuInfo, CpuInfoFlag, ffi, os};

    sys_const!({
        // Linux
        // https://github.com/torvalds/linux/blob/v6.13/arch/powerpc/include/uapi/asm/cputable.h
        // https://github.com/torvalds/linux/blob/v6.13/Documentation/arch/powerpc/elf_hwcaps.rst
        // FreeBSD
        // Defined in machine/cpu.h.
        // https://github.com/freebsd/freebsd-src/blob/release/14.2.0/sys/powerpc/include/cpu.h
        // OpenBSD
        // Defined in machine/elf.h.
        // https://github.com/openbsd/src/blob/ed8f5e8d82ace15e4cefca2c82941b15cb1a7830/sys/arch/powerpc64/include/elf.h
        // Linux 2.6.16+
        // https://github.com/torvalds/linux/commit/80f15dc703b3677d0b025bafd215f1f3664c8978
        // FreeBSD 11.0+
        // https://github.com/freebsd/freebsd-src/commit/b0bf7fcd298133457991b27625bbed766e612730
        // OpenBSD 7.6+
        // https://github.com/openbsd/src/commit/0b0568a19fc4c197871ceafbabc91fabf17ca152
        pub(crate) const PPC_FEATURE_BOOKE: ffi::c_ulong = 0x00008000;
        // Linux 3.10+
        // https://github.com/torvalds/linux/commit/cbbc6f1b1433ef553d57826eee87a84ca49645ce
        // FreeBSD 11.0+
        // https://github.com/freebsd/freebsd-src/commit/b0bf7fcd298133457991b27625bbed766e612730
        // OpenBSD 7.6+
        // https://github.com/openbsd/src/commit/0b0568a19fc4c197871ceafbabc91fabf17ca152
        pub(crate) const PPC_FEATURE2_ARCH_2_07: ffi::c_ulong = 0x80000000;
        // Linux 4.5+
        // https://github.com/torvalds/linux/commit/e708c24cd01ce80b1609d8baccee40ccc3608a01
        // FreeBSD 12.0+
        // https://github.com/freebsd/freebsd-src/commit/18f48e0c72f91bc2d4373078a3f1ab1bcab4d8b3
        // OpenBSD 7.6+
        // https://github.com/openbsd/src/commit/0b0568a19fc4c197871ceafbabc91fabf17ca152
        pub(crate) const PPC_FEATURE2_ARCH_3_00: ffi::c_ulong = 0x00800000;
        // Linux 5.8+
        // https://github.com/torvalds/linux/commit/ee988c11acf6f9464b7b44e9a091bf6afb3b3a49
        // FreeBSD 15.0+/14.2+
        // https://github.com/freebsd/freebsd-src/commit/1e434da3b065ef96b389e5e0b604ae05a51e794e
        // https://github.com/freebsd/freebsd-src/blob/release/14.2.0/sys/powerpc/include/cpu.h
        // OpenBSD 7.7+
        // https://github.com/openbsd/src/commit/483a78e15aaa23c010911940770c1c97db5c1287
        pub(crate) const PPC_FEATURE2_ARCH_3_1: ffi::c_ulong = 0x00040000;
    });

    #[cold]
    pub(super) fn _detect(info: &mut CpuInfo) {
        let hwcap = os::getauxval(ffi::AT_HWCAP);
        if hwcap & PPC_FEATURE_BOOKE != 0 {
            // quadword-atomics (Load/Store Quadword category in ISA 2.07) is requirement of ISA 2.07
            // server processors. It is always optional in ISA 2.07 BookE (embedded category)
            // processors and there is no corresponding HWCAP bit. (Although there are no ISA 2.07
            // BookE processors that appear to be supported on these platforms.)
            // Refs: Appendix B "Platform Support Requirements" of Power ISA 2.07B
            // https://ibm.ent.box.com/s/jd5w15gz301s5b5dt375mshpq9c3lh4u
            return;
        }
        let hwcap2 = os::getauxval(ffi::AT_HWCAP2);
        // Check both 2_07 and later ISAs (which are superset of 2_07) because
        // OpenBSD currently doesn't set 2_07 even when 3_00 is set.
        // https://github.com/openbsd/src/blob/d8ec5edcdf1fb224619831ad90668c95e45c3e36/sys/arch/powerpc64/powerpc64/cpu.c#L222-L238
        // Other OSes should be fine, but check all OSs in the same way just in case.
        let isa_2_07_or_later =
            PPC_FEATURE2_ARCH_2_07 | PPC_FEATURE2_ARCH_3_00 | PPC_FEATURE2_ARCH_3_1;
        if hwcap2 & isa_2_07_or_later != 0 {
            info.set(CpuInfoFlag::quadword_atomics);
        }
    }
}

#[allow(
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::undocumented_unsafe_blocks,
    clippy::wildcard_imports
)]
#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    #[allow(clippy::cast_sign_loss)]
    #[cfg(all(target_arch = "aarch64", target_os = "android"))]
    #[test]
    fn test_android() {
        use std::{slice, str};
        unsafe {
            let mut arch = [1; ffi::PROP_VALUE_MAX as usize];
            let len = ffi::__system_property_get(
                c!("ro.arch").as_ptr(),
                arch.as_mut_ptr().cast::<ffi::c_char>(),
            );
            assert!(len >= 0);
            test_helper::eprintln_nocapture!("ro.arch=raw={:?},len={}", arch, len);
            test_helper::eprintln_nocapture!(
                "ro.arch={:?}",
                str::from_utf8(slice::from_raw_parts(arch.as_ptr(), len as usize)).unwrap()
            );
        }
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn test_dlsym_getauxval() {
        unsafe {
            let ptr = ffi::dlsym(ffi::RTLD_DEFAULT, c!("getauxval").as_ptr());
            if cfg!(target_feature = "crt-static") {
                assert!(ptr.is_null());
            } else if cfg!(any(
                all(
                    target_os = "linux",
                    any(target_env = "gnu", target_env = "musl", target_env = "ohos"),
                ),
                target_os = "android",
            )) {
                assert!(!ptr.is_null());
            } else if option_env!("CI").is_some() {
                assert!(ptr.is_null());
            }
            if ptr.is_null() {
                return;
            }
            let dlsym_getauxval = mem::transmute::<*mut ffi::c_void, os::GetauxvalTy>(ptr);
            assert_eq!(dlsym_getauxval(ffi::AT_HWCAP), ffi::getauxval(ffi::AT_HWCAP));
            assert_eq!(dlsym_getauxval(ffi::AT_HWCAP2), ffi::getauxval(ffi::AT_HWCAP2));
        }
    }
    #[cfg(any(target_os = "freebsd", target_os = "openbsd"))]
    #[test]
    fn test_dlsym_elf_aux_info() {
        unsafe {
            let ptr = ffi::dlsym(ffi::RTLD_DEFAULT, c!("elf_aux_info").as_ptr());
            if cfg!(target_feature = "crt-static") {
                assert!(ptr.is_null());
            } else if cfg!(target_os = "freebsd") || option_env!("CI").is_some() {
                assert!(!ptr.is_null());
            }
            if ptr.is_null() {
                return;
            }
            let dlsym_elf_aux_info = mem::transmute::<*mut ffi::c_void, os::ElfAuxInfoTy>(ptr);
            let mut out: ffi::c_ulong = 0;
            let mut dlsym_out: ffi::c_ulong = 0;
            #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
            let out_len = mem::size_of::<ffi::c_ulong>() as ffi::c_int;
            assert_eq!(
                ffi::elf_aux_info(
                    ffi::AT_HWCAP,
                    (&mut out as *mut ffi::c_ulong).cast::<ffi::c_void>(),
                    out_len,
                ),
                dlsym_elf_aux_info(
                    ffi::AT_HWCAP,
                    (&mut dlsym_out as *mut ffi::c_ulong).cast::<ffi::c_void>(),
                    out_len,
                ),
            );
            assert_eq!(out, dlsym_out);
            out = 0;
            dlsym_out = 0;
            assert_eq!(
                ffi::elf_aux_info(
                    ffi::AT_HWCAP2,
                    (&mut out as *mut ffi::c_ulong).cast::<ffi::c_void>(),
                    out_len,
                ),
                dlsym_elf_aux_info(
                    ffi::AT_HWCAP2,
                    (&mut dlsym_out as *mut ffi::c_ulong).cast::<ffi::c_void>(),
                    out_len,
                ),
            );
            assert_eq!(out, dlsym_out);
        }
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[cfg(not(all(target_arch = "aarch64", target_pointer_width = "32")))]
    #[test]
    fn test_alternative() {
        use crate::utils::ffi::*;
        #[cfg(not(portable_atomic_no_asm))]
        use std::arch::asm;
        use std::{str, vec};
        #[cfg(target_pointer_width = "32")]
        use sys::Elf32_auxv_t as Elf_auxv_t;
        #[cfg(target_pointer_width = "64")]
        use sys::Elf64_auxv_t as Elf_auxv_t;
        use test_helper::sys;

        // Linux kernel 6.4 has added a way to read auxv without depending on either libc or mrs trap.
        // https://github.com/torvalds/linux/commit/ddc65971bb677aa9f6a4c21f76d3133e106f88eb
        // (Actually 6.5? https://github.com/torvalds/linux/commit/636e348353a7cc52609fdba5ff3270065da140d5)
        //
        // This is currently used only for testing.
        fn getauxval_pr_get_auxv_no_libc(type_: c_ulong) -> Result<c_ulong, c_int> {
            #[cfg(target_arch = "aarch64")]
            unsafe fn prctl_get_auxv(out: *mut c_void, len: usize) -> Result<usize, c_int> {
                let r: i64;
                unsafe {
                    asm!(
                        "svc 0",
                        in("x8") sys::__NR_prctl as u64,
                        inout("x0") sys::PR_GET_AUXV as u64 => r,
                        in("x1") ptr_reg!(out),
                        in("x2") len as u64,
                        // arg4 and arg5 must be zero.
                        in("x3") 0_u64,
                        in("x4") 0_u64,
                        options(nostack, preserves_flags),
                    );
                }
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                if (r as c_int) < 0 { Err(r as c_int) } else { Ok(r as usize) }
            }
            #[cfg(target_arch = "powerpc64")]
            unsafe fn prctl_get_auxv(out: *mut c_void, len: usize) -> Result<usize, c_int> {
                let r: i64;
                unsafe {
                    asm!(
                        "sc",
                        "bns+ 2f",
                        "neg %r3, %r3",
                        "2:",
                        inout("r0") sys::__NR_prctl as u64 => _,
                        inout("r3") sys::PR_GET_AUXV as u64 => r,
                        inout("r4") ptr_reg!(out) => _,
                        inout("r5") len as u64 => _,
                        // arg4 and arg5 must be zero.
                        inout("r6") 0_u64 => _,
                        inout("r7") 0_u64 => _,
                        out("r8") _,
                        out("r9") _,
                        out("r10") _,
                        out("r11") _,
                        out("r12") _,
                        out("cr0") _,
                        options(nostack, preserves_flags),
                    );
                }
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                if (r as c_int) < 0 { Err(r as c_int) } else { Ok(r as usize) }
            }

            let mut auxv = vec![unsafe { mem::zeroed::<Elf_auxv_t>() }; 38];

            let old_len = auxv.len() * mem::size_of::<Elf_auxv_t>();

            // SAFETY:
            // - `out_len` does not exceed the size of `auxv`.
            let _len = unsafe { prctl_get_auxv(auxv.as_mut_ptr().cast::<c_void>(), old_len)? };

            for aux in &auxv {
                if aux.a_type == type_ {
                    // SAFETY: aux.a_un is #[repr(C)] union and all fields have
                    // the same size and can be safely transmuted to integers.
                    return Ok(unsafe { aux.a_un.a_val });
                }
            }
            Err(0)
        }
        // Similar to the above, but call libc prctl instead of syscall using asm.
        //
        // This is currently used only for testing.
        fn getauxval_pr_get_auxv_libc(type_: c_ulong) -> Result<c_ulong, c_int> {
            unsafe fn prctl_get_auxv(out: *mut c_void, len: usize) -> Result<usize, c_int> {
                // arg4 and arg5 must be zero.
                #[allow(clippy::cast_possible_wrap)]
                let r = unsafe { libc::prctl(sys::PR_GET_AUXV as c_int, out, len, 0, 0) };
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                if (r as c_int) < 0 { Err(r as c_int) } else { Ok(r as usize) }
            }

            let mut auxv = vec![unsafe { mem::zeroed::<Elf_auxv_t>() }; 38];

            let old_len = auxv.len() * mem::size_of::<Elf_auxv_t>();

            // SAFETY:
            // - `out_len` does not exceed the size of `auxv`.
            let _len = unsafe { prctl_get_auxv(auxv.as_mut_ptr().cast::<c_void>(), old_len)? };

            for aux in &auxv {
                if aux.a_type == type_ {
                    // SAFETY: aux.a_un is #[repr(C)] union and all fields have
                    // the same size and can be safely transmuted to integers.
                    return Ok(unsafe { aux.a_un.a_val });
                }
            }
            Err(0)
        }

        unsafe {
            let mut u = mem::zeroed();
            assert_eq!(libc::uname(&mut u), 0);
            let release = std::ffi::CStr::from_ptr(u.release.as_ptr());
            let release = str::from_utf8(release.to_bytes()).unwrap();
            let mut digits = release.split('.');
            let major = digits.next().unwrap().parse::<u32>().unwrap();
            let minor = digits.next().unwrap().parse::<u32>().unwrap();
            // TODO: qemu-user bug (fails even on kernel >= 6.4) (as of 9.2)
            if (major, minor) < (6, 4) || cfg!(qemu) {
                std::eprintln!("kernel version: {}.{} (no pr_get_auxv)", major, minor);
                assert_eq!(getauxval_pr_get_auxv_libc(ffi::AT_HWCAP).unwrap_err(), -1);
                assert_eq!(getauxval_pr_get_auxv_libc(ffi::AT_HWCAP2).unwrap_err(), -1);
                assert_eq!(
                    getauxval_pr_get_auxv_no_libc(ffi::AT_HWCAP).unwrap_err(),
                    -libc::EINVAL
                );
                assert_eq!(
                    getauxval_pr_get_auxv_no_libc(ffi::AT_HWCAP2).unwrap_err(),
                    -libc::EINVAL
                );
            } else if cfg!(valgrind) {
                // TODO: valgrind bug (result value mismatch) (as of 3.24)
            } else {
                std::eprintln!("kernel version: {}.{} (has pr_get_auxv)", major, minor);
                assert_eq!(
                    os::getauxval(ffi::AT_HWCAP),
                    getauxval_pr_get_auxv_libc(ffi::AT_HWCAP).unwrap()
                );
                assert_eq!(
                    os::getauxval(ffi::AT_HWCAP2),
                    getauxval_pr_get_auxv_libc(ffi::AT_HWCAP2).unwrap()
                );
                assert_eq!(
                    os::getauxval(ffi::AT_HWCAP),
                    getauxval_pr_get_auxv_no_libc(ffi::AT_HWCAP).unwrap()
                );
                assert_eq!(
                    os::getauxval(ffi::AT_HWCAP2),
                    getauxval_pr_get_auxv_no_libc(ffi::AT_HWCAP2).unwrap()
                );
            }
        }
    }
    #[allow(clippy::cast_possible_wrap)]
    #[cfg(target_os = "freebsd")]
    #[test]
    fn test_alternative() {
        use crate::utils::ffi::*;
        #[cfg(not(portable_atomic_no_asm))]
        use std::arch::asm;
        use std::ptr;
        use test_helper::sys;

        // This is almost equivalent to what elf_aux_info does.
        // https://man.freebsd.org/elf_aux_info(3)
        // On FreeBSD, [AArch64 support is available on FreeBSD 11.0+](https://www.freebsd.org/releases/11.0R/announce),
        // but elf_aux_info is available on FreeBSD 12.0+ and 11.4+:
        // https://github.com/freebsd/freebsd-src/commit/0b08ae2120cdd08c20a2b806e2fcef4d0a36c470
        // https://github.com/freebsd/freebsd-src/blob/release/11.4.0/sys/sys/auxv.h
        // so use sysctl instead of elf_aux_info.
        // Note that FreeBSD 11 (11.4) was EoL on 2021-09-30, and FreeBSD 11.3 was EoL on 2020-09-30:
        // https://www.freebsd.org/security/unsupported
        //
        // This is currently used only for testing.
        // If you want us to use this implementation for compatibility with the older FreeBSD
        // version that came to EoL a few years ago, please open an issue.
        fn getauxval_sysctl_libc(type_: ffi::c_int) -> Result<ffi::c_ulong, c_int> {
            let mut auxv: [sys::Elf_Auxinfo; sys::AT_COUNT as usize] = unsafe { mem::zeroed() };

            let mut len = mem::size_of_val(&auxv) as c_size_t;

            // SAFETY: calling getpid is safe.
            let pid = unsafe { libc::getpid() };
            let mib = [
                sys::CTL_KERN as c_int,
                sys::KERN_PROC as c_int,
                sys::KERN_PROC_AUXV as c_int,
                pid,
            ];

            #[allow(clippy::cast_possible_truncation)]
            // SAFETY:
            // - `mib.len()` does not exceed the size of `mib`.
            // - `len` does not exceed the size of `auxv`.
            // - `sysctl` is thread-safe.
            let res = unsafe {
                libc::sysctl(
                    mib.as_ptr(),
                    mib.len() as c_uint,
                    auxv.as_mut_ptr().cast::<c_void>(),
                    &mut len,
                    ptr::null_mut(),
                    0,
                )
            };
            if res == -1 {
                return Err(res);
            }

            for aux in &auxv {
                #[allow(clippy::cast_sign_loss)]
                if aux.a_type == type_ as c_long {
                    // SAFETY: aux.a_un is #[repr(C)] union and all fields have
                    // the same size and can be safely transmuted to integers.
                    return Ok(unsafe { aux.a_un.a_val as c_ulong });
                }
            }
            Err(0)
        }
        // Similar to the above, but call syscall using asm instead of libc.
        // Note that FreeBSD does not guarantee the stability of raw syscall as
        // much as Linux does (It may actually be stable enough, though:
        // https://lists.llvm.org/pipermail/llvm-dev/2019-June/133393.html,
        // https://github.com/ziglang/zig/issues/16590).
        //
        // This is currently used only for testing.
        fn getauxval_sysctl_no_libc(type_: ffi::c_int) -> Result<ffi::c_ulong, c_int> {
            #[allow(non_camel_case_types)]
            type pid_t = c_int;

            // https://github.com/freebsd/freebsd-src/blob/release/14.2.0/lib/libc/aarch64/SYS.h
            // https://github.com/golang/go/blob/go1.24.0/src/syscall/asm_freebsd_arm64.s
            #[cfg(target_arch = "aarch64")]
            #[inline]
            fn getpid() -> pid_t {
                #[allow(clippy::cast_possible_truncation)]
                // SAFETY: calling getpid is safe.
                unsafe {
                    let n = sys::SYS_getpid;
                    let r: i64;
                    asm!(
                        "svc 0",
                        in("x8") n as u64,
                        out("x0") r,
                        options(nostack, readonly),
                    );
                    r as pid_t
                }
            }
            #[cfg(target_arch = "aarch64")]
            #[inline]
            unsafe fn sysctl(
                name: *const c_int,
                name_len: c_uint,
                old_p: *mut c_void,
                old_len_p: *mut c_size_t,
                new_p: *const c_void,
                new_len: c_size_t,
            ) -> Result<c_int, c_int> {
                #[allow(clippy::cast_possible_truncation)]
                // SAFETY: the caller must uphold the safety contract.
                unsafe {
                    let mut n = sys::SYS___sysctl as u64;
                    let r: i64;
                    asm!(
                        "svc 0",
                        "b.cc 2f",
                        "mov x8, x0",
                        "mov x0, #-1",
                        "2:",
                        inout("x8") n,
                        inout("x0") ptr_reg!(name) => r,
                        inout("x1") name_len as u64 => _,
                        in("x2") ptr_reg!(old_p),
                        in("x3") ptr_reg!(old_len_p),
                        in("x4") ptr_reg!(new_p),
                        in("x5") new_len as u64,
                        options(nostack),
                    );
                    if r as c_int == -1 { Err(n as c_int) } else { Ok(r as c_int) }
                }
            }

            // https://github.com/freebsd/freebsd-src/blob/release/14.2.0/lib/libc/powerpc64/SYS.h
            #[cfg(target_arch = "powerpc64")]
            #[inline]
            fn getpid() -> pid_t {
                #[allow(clippy::cast_possible_truncation)]
                // SAFETY: calling getpid is safe.
                unsafe {
                    let n = sys::SYS_getpid;
                    let r: i64;
                    asm!(
                        "sc",
                        inout("r0") n as u64 => _,
                        out("r3") r,
                        out("r4") _,
                        out("r5") _,
                        out("r6") _,
                        out("r7") _,
                        out("r8") _,
                        out("r9") _,
                        out("r10") _,
                        out("r11") _,
                        out("r12") _,
                        out("cr0") _,
                        options(nostack, preserves_flags, readonly),
                    );
                    r as pid_t
                }
            }
            #[cfg(target_arch = "powerpc64")]
            #[inline]
            unsafe fn sysctl(
                name: *const c_int,
                name_len: c_uint,
                old_p: *mut c_void,
                old_len_p: *mut c_size_t,
                new_p: *const c_void,
                new_len: c_size_t,
            ) -> Result<c_int, c_int> {
                #[allow(clippy::cast_possible_truncation)]
                // SAFETY: the caller must uphold the safety contract.
                unsafe {
                    let mut n = sys::SYS___sysctl as u64;
                    let r: i64;
                    asm!(
                        "sc",
                        "bns+ 2f",
                        "mr %r0, %r3",
                        "li %r3, -1",
                        "2:",
                        inout("r0") n,
                        inout("r3") ptr_reg!(name) => r,
                        inout("r4") name_len as u64 => _,
                        inout("r5") ptr_reg!(old_p) => _,
                        inout("r6") ptr_reg!(old_len_p) => _,
                        inout("r7") ptr_reg!(new_p) => _,
                        inout("r8") new_len as u64 => _,
                        out("r9") _,
                        out("r10") _,
                        out("r11") _,
                        out("r12") _,
                        out("cr0") _,
                        options(nostack, preserves_flags),
                    );
                    if r as c_int == -1 { Err(n as c_int) } else { Ok(r as c_int) }
                }
            }

            let mut auxv: [sys::Elf_Auxinfo; sys::AT_COUNT as usize] = unsafe { mem::zeroed() };

            let mut len = mem::size_of_val(&auxv) as c_size_t;

            let pid = getpid();
            let mib = [
                sys::CTL_KERN as c_int,
                sys::KERN_PROC as c_int,
                sys::KERN_PROC_AUXV as c_int,
                pid,
            ];

            #[allow(clippy::cast_possible_truncation)]
            // SAFETY:
            // - `mib.len()` does not exceed the size of `mib`.
            // - `len` does not exceed the size of `auxv`.
            // - `sysctl` is thread-safe.
            unsafe {
                sysctl(
                    mib.as_ptr(),
                    mib.len() as c_uint,
                    auxv.as_mut_ptr().cast::<c_void>(),
                    &mut len,
                    ptr::null_mut(),
                    0,
                )?;
            }

            for aux in &auxv {
                #[allow(clippy::cast_sign_loss)]
                if aux.a_type == type_ as c_long {
                    // SAFETY: aux.a_un is #[repr(C)] union and all fields have
                    // the same size and can be safely transmuted to integers.
                    return Ok(unsafe { aux.a_un.a_val as c_ulong });
                }
            }
            Err(0)
        }

        // AT_HWCAP2 is only available on FreeBSD 13+ on AArch64.
        let hwcap2_else = |e| if cfg!(target_arch = "aarch64") { 0 } else { panic!("{:?}", e) };
        assert_eq!(os::getauxval(ffi::AT_HWCAP), getauxval_sysctl_libc(ffi::AT_HWCAP).unwrap());
        assert_eq!(
            os::getauxval(ffi::AT_HWCAP2),
            getauxval_sysctl_libc(ffi::AT_HWCAP2).unwrap_or_else(hwcap2_else)
        );
        assert_eq!(os::getauxval(ffi::AT_HWCAP), getauxval_sysctl_no_libc(ffi::AT_HWCAP).unwrap());
        assert_eq!(
            os::getauxval(ffi::AT_HWCAP2),
            getauxval_sysctl_no_libc(ffi::AT_HWCAP2).unwrap_or_else(hwcap2_else)
        );
    }
}
