// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Run-time CPU feature detection on AArch64 Apple targets by using sysctlbyname.

On macOS, this module is currently only enabled on tests because there are no
instructions that were not available on the M1 but are now available on the
latest Apple hardware and this library currently wants to use:

```console
$ comm -23 <(rustc --print cfg --target aarch64-apple-darwin -C target-cpu=apple-m4 | grep -F target_feature) <(rustc --print cfg --target aarch64-apple-darwin | grep -F target_feature)
target_feature="bf16"
target_feature="bti"
target_feature="ecv"
target_feature="i8mm"
target_feature="sme"
target_feature="sme-f64f64"
target_feature="sme-i16i64"
target_feature="sme2"
target_feature="v8.5a"
target_feature="v8.6a"
target_feature="v8.7a"
target_feature="wfxt"
```

Refs: https://developer.apple.com/documentation/kernel/1387446-sysctlbyname/determining_instruction_set_characteristics

TODO: non-macOS targets doesn't always supports FEAT_LSE2, but sysctl on them on the App Store is...?
- https://developer.apple.com/forums/thread/9440
- https://nabla-c0d3.github.io/blog/2015/06/16/ios9-security-privacy
- https://github.com/rust-lang/stdarch/pull/1636
*/

include!("common.rs");

use core::{mem, ptr};

// libc requires Rust 1.63
mod ffi {
    pub(crate) use crate::utils::ffi::{CStr, c_char, c_int, c_size_t, c_void};

    sys_fn!({
        extern "C" {
            // https://developer.apple.com/documentation/kernel/1387446-sysctlbyname
            // https://github.com/apple-oss-distributions/xnu/blob/8d741a5de7ff4191bf97d57b9f54c2f6d4a15585/bsd/sys/sysctl.h
            pub(crate) fn sysctlbyname(
                name: *const c_char,
                old_p: *mut c_void,
                old_len_p: *mut c_size_t,
                new_p: *mut c_void,
                new_len: c_size_t,
            ) -> c_int;
        }
    });
}

fn sysctlbyname32(name: &ffi::CStr) -> Option<u32> {
    const OUT_LEN: ffi::c_size_t = mem::size_of::<u32>() as ffi::c_size_t;

    let mut out = 0_u32;
    let mut out_len = OUT_LEN;
    // SAFETY:
    // - `name` a valid C string.
    // - `out_len` does not exceed the size of `out`.
    // - `sysctlbyname` is thread-safe.
    let res = unsafe {
        ffi::sysctlbyname(
            name.as_ptr(),
            (&mut out as *mut u32).cast::<ffi::c_void>(),
            &mut out_len,
            ptr::null_mut(),
            0,
        )
    };
    if res != 0 {
        return None;
    }
    debug_assert_eq!(out_len, OUT_LEN);
    Some(out)
}

#[cold]
fn _detect(info: &mut CpuInfo) {
    macro_rules! check {
        ($flag:ident, $($name:tt) ||+) => {
            if $(sysctlbyname32(c!($name)).unwrap_or(0) != 0) ||+ {
                info.set(CpuInfoFlag::$flag);
            }
        };
    }

    // On macOS, AArch64 support was added in macOS 11,
    // hw.optional.armv8_1_atomics is available on macOS 11+,
    // hw.optional.arm.FEAT_* are only available on macOS 12+.
    // Query both names in case future versions of macOS remove the old name.
    // https://github.com/golang/go/commit/c15593197453b8bf90fc3a9080ba2afeaf7934ea
    // https://github.com/google/boringssl/commit/91e0b11eba517d83b910b20fe3740eeb39ecb37e
    check!(lse, "hw.optional.arm.FEAT_LSE" || "hw.optional.armv8_1_atomics");
    check!(lse2, "hw.optional.arm.FEAT_LSE2");
    check!(lse128, "hw.optional.arm.FEAT_LSE128");
    #[cfg(test)]
    check!(lsfe, "hw.optional.arm.FEAT_LSFE");
    check!(rcpc3, "hw.optional.arm.FEAT_LRCPC3");
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
    use std::{format, process::Command, str, string::String};

    use super::*;

    #[test]
    fn test_alternative() {
        use crate::utils::ffi::*;

        // Call syscall using asm instead of libc.
        // Note that macOS does not guarantee the stability of raw syscall.
        // (And they actually changed it: https://go-review.googlesource.com/c/go/+/25495)
        //
        // This is currently used only for testing.
        #[cfg(target_pointer_width = "64")]
        fn sysctlbyname32_no_libc(name: &CStr) -> Result<u32, c_int> {
            #[cfg(not(portable_atomic_no_asm))]
            use std::arch::asm;
            use std::mem;
            use test_helper::sys;

            // https://github.com/apple-oss-distributions/xnu/blob/8d741a5de7ff4191bf97d57b9f54c2f6d4a15585/bsd/kern/syscalls.master#L298
            #[inline]
            unsafe fn sysctl(
                name: *const c_int,
                name_len: c_uint,
                old_p: *mut c_void,
                old_len_p: *mut c_size_t,
                new_p: *const c_void,
                new_len: c_size_t,
            ) -> Result<c_int, c_int> {
                // https://github.com/apple-oss-distributions/xnu/blob/8d741a5de7ff4191bf97d57b9f54c2f6d4a15585/osfmk/mach/i386/syscall_sw.h#L158
                #[inline]
                const fn syscall_construct_unix(n: u64) -> u64 {
                    const SYSCALL_CLASS_UNIX: u64 = 2;
                    const SYSCALL_CLASS_SHIFT: u64 = 24;
                    const SYSCALL_CLASS_MASK: u64 = 0xFF << SYSCALL_CLASS_SHIFT;
                    const SYSCALL_NUMBER_MASK: u64 = !SYSCALL_CLASS_MASK;
                    (SYSCALL_CLASS_UNIX << SYSCALL_CLASS_SHIFT) | (SYSCALL_NUMBER_MASK & n)
                }
                #[allow(clippy::cast_possible_truncation)]
                // SAFETY: the caller must uphold the safety contract.
                unsafe {
                    // https://github.com/apple-oss-distributions/xnu/blob/8d741a5de7ff4191bf97d57b9f54c2f6d4a15585/bsd/kern/syscalls.master#L4
                    let mut n = syscall_construct_unix(202);
                    let r: i64;
                    asm!(
                        "svc 0",
                        "b.cc 2f",
                        "mov x16, x0",
                        "mov x0, #-1",
                        "2:",
                        inout("x16") n,
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
            // https://github.com/apple-oss-distributions/Libc/blob/af11da5ca9d527ea2f48bb7efbd0f0f2a4ea4812/gen/FreeBSD/sysctlbyname.c
            unsafe fn sysctlbyname(
                name: &CStr,
                old_p: *mut c_void,
                old_len_p: *mut c_size_t,
                new_p: *mut c_void,
                new_len: c_size_t,
            ) -> Result<c_int, c_int> {
                let mut real_oid: [c_int; sys::CTL_MAXNAME as usize + 2] = unsafe { mem::zeroed() };

                // Note that this is undocumented API.
                // Although FreeBSD defined it in sys/sysctl.h since https://github.com/freebsd/freebsd-src/commit/382e01c8dc7f328f46c61c82a29222f432f510f7
                let mut name2oid_oid: [c_int; 2] = [0, 3];

                let mut oid_len = mem::size_of_val(&real_oid);
                unsafe {
                    sysctl(
                        name2oid_oid.as_mut_ptr(),
                        2,
                        real_oid.as_mut_ptr().cast::<c_void>(),
                        &mut oid_len,
                        name.as_ptr().cast::<c_void>() as *mut c_void,
                        name.to_bytes_with_nul().len() - 1,
                    )?;
                }
                oid_len /= mem::size_of::<c_int>();
                #[allow(clippy::cast_possible_truncation)]
                unsafe {
                    sysctl(real_oid.as_mut_ptr(), oid_len as u32, old_p, old_len_p, new_p, new_len)
                }
            }

            const OUT_LEN: ffi::c_size_t = mem::size_of::<u32>() as ffi::c_size_t;

            let mut out = 0_u32;
            let mut out_len = OUT_LEN;
            // SAFETY:
            // - `out_len` does not exceed the size of `out`.
            // - `sysctlbyname` is thread-safe.
            let res = unsafe {
                sysctlbyname(
                    name,
                    (&mut out as *mut u32).cast::<ffi::c_void>(),
                    &mut out_len,
                    ptr::null_mut(),
                    0,
                )?
            };
            debug_assert_eq!(res, 0);
            debug_assert_eq!(out_len, OUT_LEN);
            Ok(out)
        }

        // Call sysctl command instead of libc API.
        //
        // This is used only for testing.
        struct SysctlHwOptionalOutput(String);
        impl SysctlHwOptionalOutput {
            fn new() -> Self {
                let output = Command::new("sysctl").arg("hw.optional").output().unwrap();
                assert!(output.status.success());
                let stdout = String::from_utf8(output.stdout).unwrap();
                test_helper::eprintln_nocapture!("sysctl hw.optional:\n{}", stdout);
                Self(stdout)
            }
            fn field(&self, name: &CStr) -> Option<u32> {
                let name = name.to_bytes_with_nul();
                let name = str::from_utf8(&name[..name.len() - 1]).unwrap();
                Some(
                    self.0
                        .lines()
                        .find_map(|s| s.strip_prefix(&format!("{}: ", name)))?
                        .parse()
                        .unwrap(),
                )
            }
        }

        let sysctl_output = SysctlHwOptionalOutput::new();
        for (name, expected_on_macos) in [
            (c!("hw.optional.arm.FEAT_LSE"), Some(1)),
            (c!("hw.optional.armv8_1_atomics"), Some(1)),
            (c!("hw.optional.arm.FEAT_LSE2"), Some(1)),
            (c!("hw.optional.arm.FEAT_LSE128"), None),
            (c!("hw.optional.arm.FEAT_LSFE"), None),
            (c!("hw.optional.arm.FEAT_LRCPC"), Some(1)),
            (c!("hw.optional.arm.FEAT_LRCPC2"), Some(1)),
            (c!("hw.optional.arm.FEAT_LRCPC3"), None),
        ] {
            let res = sysctlbyname32(name);
            if res.is_none() {
                assert_eq!(std::io::Error::last_os_error().kind(), std::io::ErrorKind::NotFound);
            }
            if cfg!(any(target_os = "macos", target_abi = "macabi")) {
                assert_eq!(res, expected_on_macos);
            }
            if let Some(res) = res {
                #[cfg(target_pointer_width = "64")]
                assert_eq!(res, sysctlbyname32_no_libc(name).unwrap());
                assert_eq!(res, sysctl_output.field(name).unwrap());
            } else {
                #[cfg(target_pointer_width = "64")]
                assert_eq!(sysctlbyname32_no_libc(name).unwrap_err(), libc::ENOENT);
                assert!(sysctl_output.field(name).is_none());
            }
        }
    }
}
