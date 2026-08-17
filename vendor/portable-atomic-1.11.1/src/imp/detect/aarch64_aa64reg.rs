// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Run-time CPU feature detection on AArch64 Linux/Android/FreeBSD/NetBSD/OpenBSD by parsing system registers.

As of nightly-2024-09-07, is_aarch64_feature_detected doesn't support run-time detection on NetBSD.
https://github.com/rust-lang/stdarch/blob/d9466edb4c53cece8686ee6e17b028436ddf4151/crates/std_detect/src/detect/mod.rs
Run-time detection on OpenBSD by is_aarch64_feature_detected is supported on Rust 1.70+.
https://github.com/rust-lang/stdarch/pull/1374

Refs:
- https://developer.arm.com/documentation/ddi0601/2024-12/AArch64-Registers
- https://github.com/torvalds/linux/blob/v6.13/Documentation/arch/arm64/cpu-feature-registers.rst
- https://github.com/rust-lang/stdarch/blob/a0c30f3e3c75adcd6ee7efc94014ebcead61c507/crates/std_detect/src/detect/os/aarch64.rs

Supported platforms:
- Linux 4.11+ (emulate mrs instruction)
  https://github.com/torvalds/linux/commit/77c97b4ee21290f5f083173d957843b615abbff2
- FreeBSD 12.0+ (emulate mrs instruction)
  https://github.com/freebsd/freebsd-src/commit/398810619cb32abf349f8de23f29510b2ee0839b
- NetBSD 9.0+ (through sysctl/sysctlbyname)
  https://github.com/NetBSD/src/commit/0e9d25528729f7fea53e78275d1bc5039dfe8ffb
  sysctl/sysctlbyname returns an unsupported error if operation is not supported,
  so we can safely use this on older versions.
- OpenBSD 7.1+ (through sysctl)
  https://github.com/openbsd/src/commit/d335af936b9d7dd9cf655cae1ce19560c45de6c8
  sysctl returns an unsupported error if operation is not supported,
  so we can safely use this on older versions.

For now, this module is only used on NetBSD/OpenBSD.

On Linux/Android/FreeBSD, we use auxv.rs and this module is test-only because:
- On Linux/Android, this approach requires a higher kernel version than Rust supports,
  and also does not work with qemu-user (as of 7.2) and Valgrind (as of 3.24).
  (Looking into HWCAP_CPUID in auxvec, it appears that Valgrind is setting it
  to false correctly, but qemu-user is setting it to true.)
  - qemu-user issue seem to be fixed as of 9.2.
- On FreeBSD, this approach does not work on FreeBSD 12 on QEMU (confirmed on
  FreeBSD 12.{2,3,4}), and we got SIGILL (worked on FreeBSD 13 and 14).
*/

include!("common.rs");

#[cfg_attr(test, derive(Debug, PartialEq))]
struct AA64Reg {
    aa64isar0: u64,
    aa64isar1: u64,
    #[cfg(test)]
    aa64isar3: u64,
    aa64mmfr2: u64,
}

#[cold]
fn _detect(info: &mut CpuInfo) {
    let AA64Reg {
        aa64isar0,
        aa64isar1,
        #[cfg(test)]
        aa64isar3,
        aa64mmfr2,
    } = imp::aa64reg();

    // ID_AA64ISAR0_EL1, AArch64 Instruction Set Attribute Register 0
    // https://developer.arm.com/documentation/ddi0601/2024-12/AArch64-Registers/ID-AA64ISAR0-EL1--AArch64-Instruction-Set-Attribute-Register-0
    // Atomic, bits [23:20]
    // > FEAT_LSE implements the functionality identified by the value 0b0010.
    // > FEAT_LSE128 implements the functionality identified by the value 0b0011.
    // > From Armv8.1, the value 0b0000 is not permitted.
    let atomic = extract(aa64isar0, 23, 20);
    if atomic >= 0b0010 {
        info.set(CpuInfoFlag::lse);
        if atomic >= 0b0011 {
            info.set(CpuInfoFlag::lse128);
        }
    }
    // ID_AA64ISAR1_EL1, AArch64 Instruction Set Attribute Register 1
    // https://developer.arm.com/documentation/ddi0601/2024-12/AArch64-Registers/ID-AA64ISAR1-EL1--AArch64-Instruction-Set-Attribute-Register-1
    // LRCPC, bits [23:20]
    // > FEAT_LRCPC implements the functionality identified by the value 0b0001.
    // > FEAT_LRCPC2 implements the functionality identified by the value 0b0010.
    // > FEAT_LRCPC3 implements the functionality identified by the value 0b0011.
    // > From Armv8.3, the value 0b0000 is not permitted.
    // > From Armv8.4, the value 0b0001 is not permitted.
    if extract(aa64isar1, 23, 20) >= 0b0011 {
        info.set(CpuInfoFlag::rcpc3);
    }
    // ID_AA64ISAR3_EL1, AArch64 Instruction Set Attribute Register 3
    // https://developer.arm.com/documentation/ddi0601/2024-12/AArch64-Registers/ID-AA64ISAR3-EL1--AArch64-Instruction-Set-Attribute-Register-3
    // LSFE, bits [19:16]
    // > FEAT_LSFE implements the functionality identified by the value 0b0001
    #[cfg(test)]
    if extract(aa64isar3, 19, 16) >= 0b0001 {
        info.set(CpuInfoFlag::lsfe);
    }
    // ID_AA64MMFR2_EL1, AArch64 Memory Model Feature Register 2
    // https://developer.arm.com/documentation/ddi0601/2024-12/AArch64-Registers/ID-AA64MMFR2-EL1--AArch64-Memory-Model-Feature-Register-2
    // AT, bits [35:32]
    // > FEAT_LSE2 implements the functionality identified by the value 0b0001.
    // > From Armv8.4, the value 0b0000 is not permitted.
    if extract(aa64mmfr2, 35, 32) >= 0b0001 {
        info.set(CpuInfoFlag::lse2);
    }
}

fn extract(x: u64, high: usize, low: usize) -> u64 {
    (x >> low) & ((1 << (high - low + 1)) - 1)
}

#[cfg(not(any(target_os = "netbsd", target_os = "openbsd")))]
mod imp {
    // This module is test-only. See parent module docs for details.

    #[cfg(not(portable_atomic_no_asm))]
    use core::arch::asm;

    use super::AA64Reg;

    pub(super) fn aa64reg() -> AA64Reg {
        // SAFETY: This is safe on FreeBSD 12.0+. FreeBSD 11 was EoL on 2021-09-30.
        // Note that stdarch has been doing the same thing since before FreeBSD 11 was EoL.
        // https://github.com/rust-lang/stdarch/pull/611
        unsafe {
            let aa64isar0: u64;
            asm!(
                "mrs {}, ID_AA64ISAR0_EL1",
                out(reg) aa64isar0,
                options(pure, nomem, nostack, preserves_flags),
            );
            let aa64isar1: u64;
            asm!(
                "mrs {}, ID_AA64ISAR1_EL1",
                out(reg) aa64isar1,
                options(pure, nomem, nostack, preserves_flags),
            );
            #[cfg(test)]
            #[cfg(not(portable_atomic_pre_llvm_18))]
            let aa64isar3: u64;
            // ID_AA64ISAR3_EL1 is only recognized on LLVM 18+.
            // https://github.com/llvm/llvm-project/commit/17baba9fa2728b1b1134f9dccb9318debd5a9a1b
            #[cfg(test)]
            #[cfg(not(portable_atomic_pre_llvm_18))]
            asm!(
                "mrs {}, ID_AA64ISAR3_EL1",
                out(reg) aa64isar3,
                options(pure, nomem, nostack, preserves_flags),
            );
            let aa64mmfr2: u64;
            asm!(
                "mrs {}, ID_AA64MMFR2_EL1",
                out(reg) aa64mmfr2,
                options(pure, nomem, nostack, preserves_flags),
            );
            AA64Reg {
                aa64isar0,
                aa64isar1,
                #[cfg(test)]
                #[cfg(not(portable_atomic_pre_llvm_18))]
                aa64isar3,
                #[cfg(test)]
                #[cfg(portable_atomic_pre_llvm_18)]
                aa64isar3: 0,
                aa64mmfr2,
            }
        }
    }
}
#[cfg(target_os = "netbsd")]
mod imp {
    // NetBSD doesn't trap the mrs instruction, but exposes the system registers through sysctl.
    // https://github.com/NetBSD/src/commit/0e9d25528729f7fea53e78275d1bc5039dfe8ffb
    // https://github.com/golang/sys/commit/ef9fd89ba245e184bdd308f7f2b4f3c551fa5b0f

    use core::{mem, ptr};

    use super::AA64Reg;

    // libc requires Rust 1.63
    #[allow(non_camel_case_types)]
    pub(super) mod ffi {
        pub(crate) use crate::utils::ffi::{CStr, c_char, c_int, c_size_t, c_void};

        sys_struct!({
            // Defined in machine/armreg.h.
            // https://github.com/NetBSD/src/blob/432a1357026b10c184d8a0ddb683008a23cc7cd9/sys/arch/aarch64/include/armreg.h#L1863
            pub(crate) struct aarch64_sysctl_cpu_id {
                // NetBSD 9.0+
                // https://github.com/NetBSD/src/commit/0e9d25528729f7fea53e78275d1bc5039dfe8ffb
                pub(crate) ac_midr: u64,
                pub(crate) ac_revidr: u64,
                pub(crate) ac_mpidr: u64,
                pub(crate) ac_aa64dfr0: u64,
                pub(crate) ac_aa64dfr1: u64,
                pub(crate) ac_aa64isar0: u64,
                pub(crate) ac_aa64isar1: u64,
                pub(crate) ac_aa64mmfr0: u64,
                pub(crate) ac_aa64mmfr1: u64,
                pub(crate) ac_aa64mmfr2: u64,
                pub(crate) ac_aa64pfr0: u64,
                pub(crate) ac_aa64pfr1: u64,
                pub(crate) ac_aa64zfr0: u64,
                pub(crate) ac_mvfr0: u32,
                pub(crate) ac_mvfr1: u32,
                pub(crate) ac_mvfr2: u32,
                // NetBSD 10.0+
                // https://github.com/NetBSD/src/commit/0c7bdc13f0e332cccec56e307f023b4888638973
                pub(crate) ac_pad: u32,
                pub(crate) ac_clidr: u64,
                pub(crate) ac_ctr: u64,
            }
        });

        sys_fn!({
            extern "C" {
                // Defined in sys/sysctl.h.
                // https://man.netbsd.org/sysctl.3
                // https://github.com/NetBSD/src/blob/432a1357026b10c184d8a0ddb683008a23cc7cd9/sys/sys/sysctl.h
                pub(crate) fn sysctlbyname(
                    name: *const c_char,
                    old_p: *mut c_void,
                    old_len_p: *mut c_size_t,
                    new_p: *const c_void,
                    new_len: c_size_t,
                ) -> c_int;
            }
        });
    }

    pub(super) fn sysctl_cpu_id(name: &ffi::CStr) -> Option<AA64Reg> {
        const OUT_LEN: ffi::c_size_t =
            mem::size_of::<ffi::aarch64_sysctl_cpu_id>() as ffi::c_size_t;

        // SAFETY: all fields of aarch64_sysctl_cpu_id are zero-able and we use
        // the result when machdep.cpuN.cpu_id sysctl was successful.
        let mut buf: ffi::aarch64_sysctl_cpu_id = unsafe { mem::zeroed() };
        let mut out_len = OUT_LEN;
        // SAFETY:
        // - `name` a valid C string.
        // - `out_len` does not exceed the size of the value at `buf`.
        // - `sysctlbyname` is thread-safe.
        let res = unsafe {
            ffi::sysctlbyname(
                name.as_ptr(),
                (&mut buf as *mut ffi::aarch64_sysctl_cpu_id).cast::<ffi::c_void>(),
                &mut out_len,
                ptr::null_mut(),
                0,
            )
        };
        if res != 0 {
            return None;
        }
        Some(AA64Reg {
            aa64isar0: buf.ac_aa64isar0,
            aa64isar1: buf.ac_aa64isar1,
            #[cfg(test)]
            aa64isar3: 0,
            aa64mmfr2: buf.ac_aa64mmfr2,
        })
    }

    pub(super) fn aa64reg() -> AA64Reg {
        // Get system registers for cpu0.
        // If failed, returns default because machdep.cpuN.cpu_id sysctl is not available.
        // machdep.cpuN.cpu_id sysctl was added in NetBSD 9.0 so it is not available on older versions.
        // It is ok to check only cpu0, even if there are more CPUs.
        // https://github.com/NetBSD/src/commit/bd9707e06ea7d21b5c24df6dfc14cb37c2819416
        // https://github.com/golang/sys/commit/ef9fd89ba245e184bdd308f7f2b4f3c551fa5b0f
        match sysctl_cpu_id(c!("machdep.cpu0.cpu_id")) {
            Some(cpu_id) => cpu_id,
            None => AA64Reg {
                aa64isar0: 0,
                aa64isar1: 0,
                #[cfg(test)]
                aa64isar3: 0,
                aa64mmfr2: 0,
            },
        }
    }
}
#[cfg(target_os = "openbsd")]
mod imp {
    // OpenBSD doesn't trap the mrs instruction, but exposes the system registers through sysctl.
    // https://github.com/openbsd/src/commit/d335af936b9d7dd9cf655cae1ce19560c45de6c8
    // https://github.com/golang/go/commit/cd54ef1f61945459486e9eea2f016d99ef1da925

    use core::{mem, ptr};

    use super::AA64Reg;

    // libc requires Rust 1.63
    pub(super) mod ffi {
        pub(crate) use crate::utils::ffi::{c_int, c_size_t, c_uint, c_void};

        sys_const!({
            // Defined in sys/sysctl.h.
            // https://github.com/openbsd/src/blob/ed8f5e8d82ace15e4cefca2c82941b15cb1a7830/sys/sys/sysctl.h#L82
            pub(crate) const CTL_MACHDEP: c_int = 7;

            // Defined in machine/cpu.h.
            // https://github.com/openbsd/src/blob/ed8f5e8d82ace15e4cefca2c82941b15cb1a7830/sys/arch/arm64/include/cpu.h#L25-L40
            // OpenBSD 7.1+
            // https://github.com/openbsd/src/commit/d335af936b9d7dd9cf655cae1ce19560c45de6c8
            pub(crate) const CPU_ID_AA64ISAR0: c_int = 2;
            pub(crate) const CPU_ID_AA64ISAR1: c_int = 3;
            // OpenBSD 7.3+
            // https://github.com/openbsd/src/commit/c7654cd65262d532212f65123ee3905ba200365c
            // However, on OpenBSD 7.3-7.5, querying CPU_ID_AA64MMFR2 always returns 0.
            // https://github.com/openbsd/src/commit/e8331b74e5c20302d4bd948c9db722af688ccfc1
            pub(crate) const CPU_ID_AA64MMFR2: c_int = 7;
        });

        sys_fn!({
            extern "C" {
                // Defined in sys/sysctl.h.
                // https://man.openbsd.org/sysctl.2
                // https://github.com/openbsd/src/blob/ed8f5e8d82ace15e4cefca2c82941b15cb1a7830/sys/sys/sysctl.h
                pub(crate) fn sysctl(
                    name: *const c_int,
                    name_len: c_uint,
                    old_p: *mut c_void,
                    old_len_p: *mut c_size_t,
                    new_p: *mut c_void,
                    new_len: c_size_t,
                ) -> c_int;
            }
        });
    }

    // sysctl returns an unsupported error if operation is not supported,
    // so we can safely use this function on older versions of OpenBSD.
    pub(super) fn aa64reg() -> AA64Reg {
        let aa64isar0 = sysctl64(&[ffi::CTL_MACHDEP, ffi::CPU_ID_AA64ISAR0]).unwrap_or(0);
        let aa64isar1 = sysctl64(&[ffi::CTL_MACHDEP, ffi::CPU_ID_AA64ISAR1]).unwrap_or(0);
        let aa64mmfr2 = sysctl64(&[ffi::CTL_MACHDEP, ffi::CPU_ID_AA64MMFR2]).unwrap_or(0);
        AA64Reg {
            aa64isar0,
            aa64isar1,
            #[cfg(test)]
            aa64isar3: 0,
            aa64mmfr2,
        }
    }

    fn sysctl64(mib: &[ffi::c_int]) -> Option<u64> {
        const OUT_LEN: ffi::c_size_t = mem::size_of::<u64>() as ffi::c_size_t;
        let mut out = 0_u64;
        let mut out_len = OUT_LEN;
        #[allow(clippy::cast_possible_truncation)]
        let mib_len = mib.len() as ffi::c_uint;
        // SAFETY:
        // - `mib.len()` does not exceed the size of `mib`.
        // - `out_len` does not exceed the size of `out`.
        // - `sysctl` is thread-safe.
        let res = unsafe {
            ffi::sysctl(
                mib.as_ptr(),
                mib_len,
                (&mut out as *mut u64).cast::<ffi::c_void>(),
                &mut out_len,
                ptr::null_mut(),
                0,
            )
        };
        if res == -1 {
            return None;
        }
        debug_assert_eq!(out_len, OUT_LEN);
        Some(out)
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
    use super::*;

    #[test]
    #[cfg_attr(portable_atomic_test_detect_false, ignore = "detection disabled")]
    fn test_aa64reg() {
        let AA64Reg { aa64isar0, aa64isar1, aa64isar3, aa64mmfr2 } = imp::aa64reg();
        test_helper::eprintln_nocapture!(
            "aa64isar0={},aa64isar1={},aa64isar3={},aa64mmfr2={}",
            aa64isar0,
            aa64isar1,
            aa64isar3,
            aa64mmfr2,
        );
        let atomic = extract(aa64isar0, 23, 20);
        if detect().lse() {
            if detect().lse128() {
                assert_eq!(atomic, 0b0011);
            } else {
                assert_eq!(atomic, 0b0010);
            }
        } else {
            assert_eq!(atomic, 0b0000);
        }
        let lrcpc = extract(aa64isar1, 23, 20);
        if detect().rcpc3() {
            assert_eq!(lrcpc, 0b0011);
        } else {
            assert!(lrcpc < 0b0011, "{}", lrcpc);
        }
        let lsfe = extract(aa64isar3, 19, 16);
        if detect().lsfe() {
            assert_eq!(lsfe, 0b0001);
        } else {
            assert_eq!(lsfe, 0b0000);
        }
        let at = extract(aa64mmfr2, 35, 32);
        if detect().lse2() {
            assert_eq!(at, 0b0001);
        } else {
            assert_eq!(at, 0b0000);
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    #[cfg(target_os = "netbsd")]
    #[test]
    fn test_alternative() {
        use crate::utils::ffi::*;
        use imp::ffi;
        #[cfg(not(portable_atomic_no_asm))]
        use std::arch::asm;
        use std::{mem, ptr, vec, vec::Vec};
        use test_helper::sys;

        // Call syscall using asm instead of libc.
        // Note that NetBSD does not guarantee the stability of raw syscall as
        // much as Linux does (It may actually be stable enough, though: https://lists.llvm.org/pipermail/llvm-dev/2019-June/133393.html).
        //
        // This is currently used only for testing.
        fn sysctl_cpu_id_no_libc(name: &[&[u8]]) -> Result<AA64Reg, c_int> {
            // https://github.com/golang/go/blob/go1.24.0/src/syscall/asm_netbsd_arm64.s
            #[inline]
            unsafe fn sysctl(
                name: *const c_int,
                name_len: c_uint,
                old_p: *mut c_void,
                old_len_p: *mut c_size_t,
                new_p: *const c_void,
                new_len: c_size_t,
            ) -> Result<c_int, c_int> {
                // SAFETY: the caller must uphold the safety contract.
                unsafe {
                    let mut n = sys::SYS___sysctl as u64;
                    let r: i64;
                    asm!(
                        "svc 0",
                        "b.cc 2f",
                        "mov x17, x0",
                        "mov x0, #-1",
                        "2:",
                        inout("x17") n,
                        inout("x0") ptr_reg!(name) => r,
                        inout("x1") name_len as u64 => _,
                        in("x2") ptr_reg!(old_p),
                        in("x3") ptr_reg!(old_len_p),
                        in("x4") ptr_reg!(new_p),
                        in("x5") new_len as u64,
                        options(nostack),
                    );
                    #[allow(clippy::cast_possible_truncation)]
                    if r as c_int == -1 { Err(n as c_int) } else { Ok(r as c_int) }
                }
            }

            // https://github.com/golang/sys/blob/v0.31.0/cpu/cpu_netbsd_arm64.go
            fn sysctl_nodes(mib: &mut Vec<i32>) -> Result<Vec<sys::sysctlnode>, i32> {
                mib.push(sys::CTL_QUERY);
                let mut q_node = sys::sysctlnode {
                    sysctl_flags: sys::SYSCTL_VERS_1,
                    ..unsafe { mem::zeroed() }
                };
                let qp = (&mut q_node as *mut sys::sysctlnode).cast::<ffi::c_void>();
                let sz = mem::size_of::<sys::sysctlnode>();
                let mut olen = 0;
                #[allow(clippy::cast_possible_truncation)]
                let mib_len = mib.len() as c_uint;
                unsafe {
                    sysctl(mib.as_ptr(), mib_len, ptr::null_mut(), &mut olen, qp, sz)?;
                }

                let mut nodes = Vec::<sys::sysctlnode>::with_capacity(olen / sz);
                let np = nodes.as_mut_ptr().cast::<ffi::c_void>();
                unsafe {
                    sysctl(mib.as_ptr(), mib_len, np, &mut olen, qp, sz)?;
                    nodes.set_len(olen / sz);
                }

                mib.pop(); // pop CTL_QUERY
                Ok(nodes)
            }
            fn name_to_mib(parts: &[&[u8]]) -> Result<Vec<i32>, i32> {
                let mut mib = vec![];
                for (part_no, &part) in parts.iter().enumerate() {
                    let nodes = sysctl_nodes(&mut mib)?;
                    for node in nodes {
                        let mut n = vec![];
                        for b in node.sysctl_name {
                            if b != 0 {
                                n.push(b);
                            }
                        }
                        if n == part {
                            mib.push(node.sysctl_num);
                            break;
                        }
                    }
                    if mib.len() != part_no + 1 {
                        return Err(0);
                    }
                }

                Ok(mib)
            }

            const OUT_LEN: ffi::c_size_t =
                mem::size_of::<ffi::aarch64_sysctl_cpu_id>() as ffi::c_size_t;

            let mib = name_to_mib(name)?;

            let mut buf: ffi::aarch64_sysctl_cpu_id = unsafe { mem::zeroed() };
            let mut out_len = OUT_LEN;
            #[allow(clippy::cast_possible_truncation)]
            let mib_len = mib.len() as c_uint;
            unsafe {
                sysctl(
                    mib.as_ptr(),
                    mib_len,
                    (&mut buf as *mut ffi::aarch64_sysctl_cpu_id).cast::<ffi::c_void>(),
                    &mut out_len,
                    ptr::null_mut(),
                    0,
                )?;
            }
            Ok(AA64Reg {
                aa64isar0: buf.ac_aa64isar0,
                aa64isar1: buf.ac_aa64isar1,
                aa64isar3: 0,
                aa64mmfr2: buf.ac_aa64mmfr2,
            })
        }

        assert_eq!(
            imp::sysctl_cpu_id(c!("machdep.cpu0.cpu_id")).unwrap(),
            sysctl_cpu_id_no_libc(&[b"machdep", b"cpu0", b"cpu_id"]).unwrap()
        );
    }
    #[cfg(target_os = "openbsd")]
    #[test]
    fn test_alternative() {
        use std::{format, process::Command, string::String};

        // Call sysctl command instead of libc API.
        //
        // This is used only for testing.
        struct SysctlMachdepOutput(String);
        impl SysctlMachdepOutput {
            fn new() -> Self {
                let output = Command::new("sysctl").arg("machdep").output().unwrap();
                assert!(output.status.success());
                let stdout = String::from_utf8(output.stdout).unwrap();
                Self(stdout)
            }
            fn field(&self, name: &str) -> Option<u64> {
                Some(
                    self.0
                        .lines()
                        .find_map(|s| s.strip_prefix(&format!("{}=", name)))?
                        .parse()
                        .unwrap(),
                )
            }
        }

        let AA64Reg { aa64isar0, aa64isar1, aa64isar3, aa64mmfr2 } = imp::aa64reg();
        let sysctl_output = SysctlMachdepOutput::new();
        assert_eq!(aa64isar0, sysctl_output.field("machdep.id_aa64isar0").unwrap_or(0));
        assert_eq!(aa64isar1, sysctl_output.field("machdep.id_aa64isar1").unwrap_or(0));
        assert_eq!(aa64isar3, sysctl_output.field("machdep.id_aa64isar3").unwrap_or(0));
        assert_eq!(aa64mmfr2, sysctl_output.field("machdep.id_aa64mmfr2").unwrap_or(0));
    }
}
