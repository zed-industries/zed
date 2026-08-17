//! Definition of `VM*Context` variant for host functions.
//!
//! Keep in sync with `wasmtime_environ::VMHostFuncOffsets`.

use super::{VMArrayCallNative, VMOpaqueContext};
use crate::prelude::*;
use crate::runtime::vm::{StoreBox, VMFuncRef};
use core::any::Any;
use core::ptr::NonNull;
use wasmtime_environ::{VM_ARRAY_CALL_HOST_FUNC_MAGIC, VMSharedTypeIndex};

/// The `VM*Context` for array-call host functions.
///
/// Its `magic` field must always be
/// `wasmtime_environ::VM_ARRAY_CALL_HOST_FUNC_MAGIC`, and this is how you can
/// determine whether a `VM*Context` is a `VMArrayCallHostFuncContext` versus a
/// different kind of context.
#[repr(C)]
pub struct VMArrayCallHostFuncContext {
    magic: u32,
    // _padding: u32, // (on 64-bit systems)
    pub(crate) func_ref: VMFuncRef,
    host_state: Box<dyn Any + Send + Sync>,
}

impl VMArrayCallHostFuncContext {
    /// Create the context for the given host function.
    ///
    /// # Safety
    ///
    /// The `host_func` must be a pointer to a host (not Wasm) function and it
    /// must be `Send` and `Sync`.
    pub unsafe fn new(
        host_func: VMArrayCallNative,
        type_index: VMSharedTypeIndex,
        host_state: Box<dyn Any + Send + Sync>,
    ) -> StoreBox<VMArrayCallHostFuncContext> {
        let ctx = StoreBox::new(VMArrayCallHostFuncContext {
            magic: wasmtime_environ::VM_ARRAY_CALL_HOST_FUNC_MAGIC,
            func_ref: VMFuncRef {
                array_call: NonNull::new(host_func as *mut u8).unwrap().cast().into(),
                type_index,
                wasm_call: None,
                vmctx: NonNull::dangling().into(),
            },
            host_state,
        });
        let vmctx = VMOpaqueContext::from_vm_array_call_host_func_context(ctx.get());
        unsafe {
            ctx.get().as_mut().func_ref.vmctx = vmctx.into();
        }
        ctx
    }

    /// Get the host state for this host function context.
    #[inline]
    pub fn host_state(&self) -> &(dyn Any + Send + Sync) {
        &*self.host_state
    }

    /// Get this context's `VMFuncRef`.
    #[inline]
    pub fn func_ref(&self) -> &VMFuncRef {
        &self.func_ref
    }

    /// Helper function to cast between context types using a debug assertion to
    /// protect against some mistakes.
    #[inline]
    pub unsafe fn from_opaque(
        opaque: NonNull<VMOpaqueContext>,
    ) -> NonNull<VMArrayCallHostFuncContext> {
        // See comments in `VMContext::from_opaque` for this debug assert
        unsafe {
            debug_assert_eq!(opaque.as_ref().magic, VM_ARRAY_CALL_HOST_FUNC_MAGIC);
        }
        opaque.cast()
    }
}

#[test]
fn vmarray_call_host_func_context_offsets() {
    use core::mem::offset_of;
    use wasmtime_environ::{HostPtr, PtrSize};
    assert_eq!(
        usize::from(HostPtr.vmarray_call_host_func_context_func_ref()),
        offset_of!(VMArrayCallHostFuncContext, func_ref)
    );
}
