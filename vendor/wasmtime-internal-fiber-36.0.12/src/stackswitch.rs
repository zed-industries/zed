//! ISA-specific stack-switching routines.

// The bodies are defined in inline assembly in the conditionally
// included modules below; their symbols are visible in the binary and
// accessed via the `extern "C"` declarations below that.

cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub(crate) use supported::*;
    } else if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub(crate) use supported::*;
    } else if #[cfg(target_arch = "x86")] {
        mod x86;
        pub(crate) use supported::*;
    } else if #[cfg(target_arch = "arm")] {
        mod arm;
        pub(crate) use supported::*;
    } else if #[cfg(target_arch = "s390x")] {
        // currently `global_asm!` isn't stable on s390x so this is an external
        // assembler file built with the `build.rs`.
        pub(crate) use supported::*;
    } else if #[cfg(target_arch = "riscv64")]  {
        mod riscv64;
        pub(crate) use supported::*;
    } else {
        // No support for this platform. Don't fail compilation though and
        // instead defer the error to happen at runtime when a fiber is created.
        // Should help keep compiles working and narrows the failure to only
        // situations that need fibers on unsupported platforms.
        pub(crate) use unsupported::*;
    }
}

/// A helper module to get reeported above in each case that we actually have
/// stack-switching routines available in in line asm. The fall-through case
/// though reexports the `unsupported` module instead.
#[allow(
    dead_code,
    reason = "expected to have dead code in some configurations"
)]
mod supported {
    pub const SUPPORTED_ARCH: bool = true;
    unsafe extern "C" {
        #[wasmtime_versioned_export_macros::versioned_link]
        pub(crate) fn wasmtime_fiber_init(
            top_of_stack: *mut u8,
            entry: extern "C" fn(*mut u8, *mut u8),
            entry_arg0: *mut u8,
        );
        #[wasmtime_versioned_export_macros::versioned_link]
        pub(crate) fn wasmtime_fiber_switch(top_of_stack: *mut u8);
        #[wasmtime_versioned_export_macros::versioned_link]
        pub(crate) fn wasmtime_fiber_start();
    }
}

/// Helper module reexported in the fallback case above when the current host
/// architecture is not supported for stack switching. The `SUPPORTED_ARCH`
/// boolean here is set to `false` which causes `Fiber::new` to return `false`.
#[allow(
    dead_code,
    reason = "expected to have dead code in some configurations"
)]
mod unsupported {
    pub const SUPPORTED_ARCH: bool = false;

    pub(crate) unsafe fn wasmtime_fiber_init(
        _top_of_stack: *mut u8,
        _entry: extern "C" fn(*mut u8, *mut u8),
        _entry_arg0: *mut u8,
    ) {
        unreachable!();
    }

    pub(crate) unsafe fn wasmtime_fiber_switch(_top_of_stack: *mut u8) {
        unreachable!();
    }
}
