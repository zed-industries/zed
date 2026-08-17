// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Run-time CPU feature detection on x86_64 by using CPUID.

Adapted from https://github.com/rust-lang/stdarch.
*/

#![cfg_attr(portable_atomic_sanitize_thread, allow(dead_code))]

// Miri doesn't support inline assembly used in __cpuid: https://github.com/rust-lang/miri/issues/932
// SGX doesn't support CPUID: https://github.com/rust-lang/stdarch/blob/a0c30f3e3c75adcd6ee7efc94014ebcead61c507/crates/core_arch/src/x86/cpuid.rs#L102-L105
#[cfg(any(target_env = "sgx", miri))]
compile_error!("internal error: this module is not supported on this environment");

include!("common.rs");

#[cfg(not(portable_atomic_no_asm))]
use core::arch::asm;
use core::arch::x86_64::CpuidResult;

// Workaround for https://github.com/rust-lang/rust/issues/101346
// It is not clear if our use cases are affected, but we implement this just in case.
//
// Refs:
// - https://www.felixcloutier.com/x86/cpuid
// - https://en.wikipedia.org/wiki/CPUID
// - https://github.com/rust-lang/stdarch/blob/a0c30f3e3c75adcd6ee7efc94014ebcead61c507/crates/core_arch/src/x86/cpuid.rs
#[cfg(not(target_env = "sgx"))]
fn __cpuid(leaf: u32) -> CpuidResult {
    let eax;
    let mut ebx;
    let ecx;
    let edx;
    // SAFETY: Calling `__cpuid` is safe on all x86_64 CPUs except for SGX,
    // which doesn't support `cpuid`.
    // https://github.com/rust-lang/stdarch/blob/a0c30f3e3c75adcd6ee7efc94014ebcead61c507/crates/core_arch/src/x86/cpuid.rs#L102-L109
    unsafe {
        asm!(
            "mov {ebx_tmp:r}, rbx", // save rbx which is reserved by LLVM
            "cpuid",
            "xchg {ebx_tmp:r}, rbx", // restore rbx
            ebx_tmp = out(reg) ebx,
            inout("eax") leaf => eax,
            inout("ecx") 0 => ecx,
            out("edx") edx,
            options(nostack, preserves_flags),
        );
    }
    CpuidResult { eax, ebx, ecx, edx }
}

// https://en.wikipedia.org/wiki/CPUID
const _VENDOR_ID_INTEL: [u32; 3] = _vender(b"GenuineIntel"); // Intel
const _VENDOR_ID_INTEL2: [u32; 3] = _vender(b"GenuineIotel"); // Intel https://github.com/InstLatx64/InstLatx64/commit/8fdd319884c67d2c6ec1ca0c595b42c1c4b8d803
const _VENDOR_ID_AMD: [u32; 3] = _vender(b"AuthenticAMD"); // AMD
const _VENDOR_ID_CENTAUR: [u32; 3] = _vender(b"CentaurHauls"); // Centaur/VIA/Zhaoxin
const _VENDOR_ID_ZHAOXIN: [u32; 3] = _vender(b"  Shanghai  "); // Zhaoxin
const fn _vender(b: &[u8; 12]) -> [u32; 3] {
    [
        u32::from_ne_bytes([b[0], b[1], b[2], b[3]]),
        u32::from_ne_bytes([b[4], b[5], b[6], b[7]]),
        u32::from_ne_bytes([b[8], b[9], b[10], b[11]]),
    ]
}
fn _vendor_id() -> [u32; 3] {
    let CpuidResult { ebx, ecx, edx, .. } = __cpuid(0);
    [ebx, edx, ecx]
}
fn _vendor_has_vmovdqa_atomic(vendor_id: [u32; 3], family: u32) -> bool {
    // VMOVDQA is atomic on Intel, AMD, and Zhaoxin CPUs with AVX.
    // See https://gcc.gnu.org/bugzilla/show_bug.cgi?id=104688 for details.
    vendor_id == _VENDOR_ID_INTEL
        || vendor_id == _VENDOR_ID_INTEL2
        || vendor_id == _VENDOR_ID_AMD
        || vendor_id == _VENDOR_ID_ZHAOXIN
        || vendor_id == _VENDOR_ID_CENTAUR && family > 6
}

#[cold]
fn _detect(info: &mut CpuInfo) {
    let CpuidResult {
        #[cfg(target_feature = "sse")]
            eax: proc_info_eax,
        ecx: proc_info_ecx,
        ..
    } = __cpuid(1);

    // https://github.com/rust-lang/stdarch/blob/a0c30f3e3c75adcd6ee7efc94014ebcead61c507/crates/std_detect/src/detect/os/x86.rs#L111
    if test(proc_info_ecx, 13) {
        info.set(CpuInfoFlag::cmpxchg16b);
    }

    // We only use VMOVDQA when SSE is enabled. See atomic_load_vmovdqa() in atomic128/x86_64.rs for more.
    #[cfg(target_feature = "sse")]
    {
        use core::arch::x86_64::_xgetbv;

        // https://github.com/rust-lang/stdarch/blob/a0c30f3e3c75adcd6ee7efc94014ebcead61c507/crates/std_detect/src/detect/os/x86.rs#L131-L224
        let cpu_xsave = test(proc_info_ecx, 26);
        if cpu_xsave {
            let cpu_osxsave = test(proc_info_ecx, 27);
            if cpu_osxsave {
                // SAFETY: Calling `_xgetbv` is safe because the CPU has `xsave` support
                // and OS has set `osxsave`.
                let xcr0 = unsafe { _xgetbv(0) };
                let os_avx_support = xcr0 & 6 == 6;
                if os_avx_support && test(proc_info_ecx, 28) {
                    let vendor_id = _vendor_id();
                    let family = (proc_info_eax >> 8) & 0x0F;
                    if _vendor_has_vmovdqa_atomic(vendor_id, family) {
                        info.set(CpuInfoFlag::vmovdqa_atomic);
                    }
                }
            }
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
    use std::{mem, str};

    use super::*;

    #[test]
    #[cfg_attr(portable_atomic_test_detect_false, ignore = "detection disabled")]
    fn test_cpuid() {
        assert_eq!(std::is_x86_feature_detected!("cmpxchg16b"), detect().cmpxchg16b());
        let vendor_id = _vendor_id();
        test_helper::eprintln_nocapture!(
            "\n  vendor_id: {} (ebx: {:x}, edx: {:x}, ecx: {:x})",
            str::from_utf8(&unsafe { mem::transmute::<[u32; 3], [u8; 12]>(vendor_id) }).unwrap(),
            vendor_id[0],
            vendor_id[1],
            vendor_id[2],
        );
        let CpuidResult { eax: proc_info_eax, .. } = __cpuid(1);
        let family = (proc_info_eax >> 8) & 0x0F;
        if _vendor_has_vmovdqa_atomic(vendor_id, family) {
            assert_eq!(std::is_x86_feature_detected!("avx"), detect().vmovdqa_atomic());
        } else {
            assert!(!detect().vmovdqa_atomic());
        }
        assert_eq!(
            unsafe { mem::transmute::<[u32; 3], [u8; 12]>(_VENDOR_ID_INTEL) },
            *b"GenuineIntel"
        );
    }
}
