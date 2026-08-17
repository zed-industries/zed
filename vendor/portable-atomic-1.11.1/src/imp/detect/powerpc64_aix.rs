// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Run-time CPU feature detection on PowerPC64 AIX by using getsystemcfg.

Refs:
- https://github.com/golang/go/blob/go1.24.0/src/internal/cpu/cpu_ppc64x_aix.go

As of nightly-2024-09-07, is_powerpc_feature_detected doesn't support run-time detection on AIX.
https://github.com/rust-lang/stdarch/blob/d9466edb4c53cece8686ee6e17b028436ddf4151/crates/std_detect/src/detect/mod.rs

Run-time detection on PowerPC64 AIX is currently disabled by default as experimental
because we cannot run tests on the VM or real machine.
*/

include!("common.rs");

// libc requires Rust 1.63
mod ffi {
    pub(crate) use crate::utils::ffi::{c_int, c_ulong};

    sys_const!({
        // https://github.com/rust-lang/libc/blob/0.2.158/src/unix/aix/mod.rs#L2058
        // https://github.com/golang/go/blob/go1.24.0/src/internal/cpu/cpu_ppc64x_aix.go
        pub(crate) const SC_IMPL: c_int = 2;
        pub(crate) const POWER_8: c_ulong = 0x10000;
        pub(crate) const POWER_9: c_ulong = 0x20000;
    });
    // TODO: use sys_const! once libc crate defined it.
    // https://github.com/golang/go/blob/go1.24.0/src/internal/cpu/cpu_ppc64x_aix.go
    pub(crate) const POWER_10: c_ulong = 0x40000;

    sys_fn!({
        extern "C" {
            // https://www.ibm.com/docs/en/aix/7.3?topic=g-getsystemcfg-subroutine
            // https://github.com/rust-lang/libc/blob/0.2.158/src/unix/aix/powerpc64.rs#L643
            pub(crate) fn getsystemcfg(name: c_int) -> c_ulong;
        }
    });
}

#[cold]
fn _detect(info: &mut CpuInfo) {
    // SAFETY: calling getsystemcfg is safe.
    let impl_ = unsafe { ffi::getsystemcfg(ffi::SC_IMPL) };
    if impl_ == ffi::c_ulong::MAX {
        return;
    }
    // Check both POWER_8 and later ISAs (which are superset of POWER_8) because
    // AIX currently doesn't set POWER_8 when POWER_9 is set.
    // https://github.com/golang/go/commit/51859ec2292d9c1d82a7054ec672ff551a0d7497
    if impl_ & (ffi::POWER_8 | ffi::POWER_9 | ffi::POWER_10) != 0 {
        info.set(CpuInfoFlag::quadword_atomics);
    }
}
