cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub type RawContextCPU = minidump_common::format::CONTEXT_AMD64;
        pub type FloatStateCPU = minidump_common::format::XMM_SAVE_AREA32;
    } else if #[cfg(target_arch = "x86")] {
        pub type RawContextCPU = minidump_common::format::CONTEXT_X86;
        pub type FloatStateCPU = minidump_common::format::FLOATING_SAVE_AREA_X86;
    } else if #[cfg(target_arch = "arm")] {
        pub type RawContextCPU = minidump_common::format::CONTEXT_ARM;
        pub type FloatStateCPU = minidump_common::format::FLOATING_SAVE_AREA_ARM;
    } else if #[cfg(target_arch = "aarch64")] {
        /// This is the number of general purpose registers _not_ counting
        /// the stack pointer
        #[cfg(any(target_os = "linux", target_os = "android"))]
        pub(crate) const GP_REG_COUNT: usize = 31;
        /// The number of floating point registers in the floating point save area
        #[cfg(any(target_os = "linux", target_os = "android"))]
        pub(crate) const FP_REG_COUNT: usize = 32;

        pub type RawContextCPU = minidump_common::format::CONTEXT_ARM64_OLD;
    } else if #[cfg(target_arch = "mips")] {
        compile_error!("flesh me out");
    } else {
        compile_error!("unsupported target architecture");
    }
}
