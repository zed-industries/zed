#![doc(hidden)]

use crate::runtime::vm::instance::InstanceAndStore;
use crate::runtime::vm::vmcontext::VMContext;
use core::ptr::NonNull;
use wasmtime_environ::{EntityRef, MemoryIndex};
use wasmtime_versioned_export_macros::versioned_export;

static mut VMCTX_AND_MEMORY: (NonNull<VMContext>, usize) = (NonNull::dangling(), 0);

// These implementations are referenced from C code in "helpers.c". The symbols defined
// there (prefixed by "wasmtime_") are the real 'public' interface used in the debug info.

#[versioned_export]
pub unsafe extern "C" fn resolve_vmctx_memory_ptr(p: *const u32) -> *const u8 {
    unsafe {
        let ptr = core::ptr::read(p);
        assert!(
            VMCTX_AND_MEMORY.0 != NonNull::dangling(),
            "must call `__vmctx->set()` before resolving Wasm pointers"
        );
        InstanceAndStore::from_vmctx(VMCTX_AND_MEMORY.0, |handle| {
            let (handle, _) = handle.unpack_mut();
            assert!(
                VMCTX_AND_MEMORY.1 < handle.env_module().memories.len(),
                "memory index for debugger is out of bounds"
            );
            let index = MemoryIndex::new(VMCTX_AND_MEMORY.1);
            let mem = handle.get_memory(index);
            mem.base.as_ptr().add(ptr as usize)
        })
    }
}

#[versioned_export]
pub unsafe extern "C" fn set_vmctx_memory(vmctx_ptr: *mut VMContext) {
    unsafe {
        // TODO multi-memory
        VMCTX_AND_MEMORY = (NonNull::new(vmctx_ptr).unwrap(), 0);
    }
}

/// A bit of a hack around various linkage things. The goal here is to force the
/// `wasmtime_*` symbols defined in `helpers.c` to actually get exported. That
/// means they need to be referenced for the linker to include them which is
/// what this function does with trickery in C.
pub fn init() {
    unsafe extern "C" {
        #[wasmtime_versioned_export_macros::versioned_link]
        fn wasmtime_debug_builtins_init();
    }

    unsafe {
        wasmtime_debug_builtins_init();
    }
}
