// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Run-time CPU feature detection on AArch64 Windows by using IsProcessorFeaturePresent.

Run-time detection of FEAT_LSE on Windows by is_aarch64_feature_detected is supported on Rust 1.70+.
https://github.com/rust-lang/stdarch/pull/1373

Refs: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-isprocessorfeaturepresent
*/

include!("common.rs");

// windows-sys requires Rust 1.60
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
mod ffi {
    sys_type!({
        pub(crate) type [Win32::System::Threading] PROCESSOR_FEATURE_ID = u32;
        pub(crate) type [Win32::Foundation] BOOL = i32;
    });

    sys_const!({
        pub(crate) const [Win32::Foundation] FALSE: BOOL = 0;

        // Defined in winnt.h of Windows SDK.
        pub(crate) const [Win32::System::Threading]
            PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 34;
    });

    sys_fn!({
        extern "system" {
            // https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-isprocessorfeaturepresent
            pub(crate) fn [Win32::System::Threading] IsProcessorFeaturePresent(
                ProcessorFeature: PROCESSOR_FEATURE_ID,
            ) -> BOOL;
        }
    });
}

#[cold]
fn _detect(info: &mut CpuInfo) {
    macro_rules! check {
        ($flag:ident, $bit:ident) => {
            // SAFETY: calling IsProcessorFeaturePresent is safe, and FALSE is also
            // returned if the HAL does not support detection of the specified feature.
            if unsafe { ffi::IsProcessorFeaturePresent(ffi::$bit) != ffi::FALSE } {
                info.set(CpuInfoFlag::$flag);
            }
        };
    }
    check!(lse, PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE);
}
