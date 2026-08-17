//! Module for Windows x64 ABI unwind registry.

use crate::prelude::*;
use std::mem;
use windows_sys::Win32::System::Diagnostics::Debug::*;

/// Represents a registry of function unwind information for Windows x64 ABI.
pub struct UnwindRegistration {
    functions: usize,
}

impl UnwindRegistration {
    pub const SECTION_NAME: &'static str = ".pdata";

    pub unsafe fn new(
        base_address: *const u8,
        unwind_info: *const u8,
        unwind_len: usize,
    ) -> Result<UnwindRegistration> {
        #[cfg(target_arch = "aarch64")]
        use IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY as Entry;
        #[cfg(not(target_arch = "aarch64"))]
        use IMAGE_RUNTIME_FUNCTION_ENTRY as Entry;

        assert!(unwind_info as usize % 4 == 0);
        let unit_len = mem::size_of::<Entry>();
        assert!(unwind_len % unit_len == 0);
        unsafe {
            if !RtlAddFunctionTable(
                unwind_info as *mut Entry,
                (unwind_len / unit_len).try_into().unwrap(),
                base_address as _,
            ) {
                bail!("failed to register function table");
            }
        }

        Ok(UnwindRegistration {
            functions: unwind_info as usize,
        })
    }
}

impl Drop for UnwindRegistration {
    fn drop(&mut self) {
        unsafe {
            RtlDeleteFunctionTable(self.functions as _);
        }
    }
}
