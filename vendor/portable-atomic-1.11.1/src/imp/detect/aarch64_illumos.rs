// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Run-time CPU feature detection on AArch64 illumos by using getisax.

As of nightly-2024-09-07, is_aarch64_feature_detected doesn't support run-time detection on illumos.
https://github.com/rust-lang/stdarch/blob/d9466edb4c53cece8686ee6e17b028436ddf4151/crates/std_detect/src/detect/mod.rs

Run-time detection on AArch64 illumos is currently disabled by default as experimental
because illumos AArch64 port is experimental and we cannot run tests on the VM or real machine.
*/

include!("common.rs");

// libc requires Rust 1.63
mod ffi {
    pub(crate) use crate::utils::ffi::c_uint;

    sys_const!({
        // Defined in sys/auxv_aarch64.h.
        // https://github.com/richlowe/illumos-gate/blob/arm64-gate/usr/src/uts/common/sys/auxv_aarch64.h
        pub(crate) const AV_AARCH64_LSE: u32 = 1 << 15;
        pub(crate) const AV_AARCH64_2_LSE2: u32 = 1 << 2;
    });

    sys_fn!({
        extern "C" {
            // Defined in sys/auxv.h.
            // https://illumos.org/man/2/getisax
            // https://github.com/richlowe/illumos-gate/blob/arm64-gate/usr/src/uts/common/sys/auxv.h
            pub(crate) fn getisax(array: *mut u32, n: c_uint) -> c_uint;
        }
    });
}

#[cold]
fn _detect(info: &mut CpuInfo) {
    const OUT_LEN: ffi::c_uint = 2;
    let mut out = [0_u32; OUT_LEN as usize];
    // SAFETY: the pointer is valid because we got it from a reference.
    unsafe {
        ffi::getisax(out.as_mut_ptr(), OUT_LEN);
    }
    macro_rules! check {
        ($x:ident, $flag:ident, $bit:ident) => {
            if $x & ffi::$bit != 0 {
                info.set(CpuInfoFlag::$flag);
            }
        };
    }
    let v1 = out[0];
    check!(v1, lse, AV_AARCH64_LSE);
    let v2 = out[1];
    check!(v2, lse2, AV_AARCH64_2_LSE2);
}
