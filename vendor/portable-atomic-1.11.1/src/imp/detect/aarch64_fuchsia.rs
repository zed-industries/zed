// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Run-time CPU feature detection on AArch64 Fuchsia by using zx_system_get_features.

As of nightly-2024-09-07, is_aarch64_feature_detected doesn't support run-time detection on Fuchsia.
https://github.com/rust-lang/stdarch/blob/d9466edb4c53cece8686ee6e17b028436ddf4151/crates/std_detect/src/detect/mod.rs

Refs:
- https://fuchsia.dev/reference/syscalls/system_get_features
- https://github.com/llvm/llvm-project/commit/4e731abc55681751b5d736b613f7720e50eb1ad4
*/

include!("common.rs");

#[allow(non_camel_case_types)]
mod ffi {
    sys_type!({
        // https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/zircon/system/public/zircon/types.h
        pub(crate) type zx_status_t = i32;
    });

    sys_const!({
        // https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/zircon/system/public/zircon/errors.h
        pub(crate) const ZX_OK: zx_status_t = 0;

        // https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/zircon/system/public/zircon/features.h
        pub(crate) const ZX_FEATURE_KIND_CPU: u32 = 0;
        pub(crate) const ZX_ARM64_FEATURE_ISA_ATOMICS: u32 = 1 << 8;
    });

    // TODO: use sys_fn!
    #[link(name = "zircon")]
    extern "C" {
        // https://fuchsia.dev/reference/syscalls/system_get_features
        pub(crate) fn zx_system_get_features(kind: u32, features: *mut u32) -> zx_status_t;
    }
}

fn zx_system_get_features(kind: u32) -> u32 {
    let mut out = 0_u32;
    // SAFETY: the pointer is valid because we got it from a reference.
    let res = unsafe { ffi::zx_system_get_features(kind, &mut out) };
    if res != ffi::ZX_OK {
        return 0;
    }
    out
}

#[cold]
fn _detect(info: &mut CpuInfo) {
    let features = zx_system_get_features(ffi::ZX_FEATURE_KIND_CPU);
    macro_rules! check {
        ($flag:ident, $bit:ident) => {
            if features & ffi::$bit != 0 {
                info.set(CpuInfoFlag::$flag);
            }
        };
    }
    check!(lse, ZX_ARM64_FEATURE_ISA_ATOMICS);
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
    fn test_fuchsia() {
        let features = zx_system_get_features(ffi::ZX_FEATURE_KIND_CPU);
        test_helper::eprintln_nocapture!(
            "zx_system_get_features(ZX_FEATURE_KIND_CPU): {:b}",
            features
        );
        assert_ne!(features, 0);
    }
}
