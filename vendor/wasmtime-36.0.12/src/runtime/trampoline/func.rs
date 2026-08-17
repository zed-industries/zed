//! Support for a calling of an imported function.

use crate::prelude::*;
use crate::runtime::vm::{StoreBox, VMArrayCallHostFuncContext, VMContext, VMOpaqueContext};
use crate::type_registry::RegisteredType;
use crate::{FuncType, ValRaw};
use core::ptr::NonNull;

struct TrampolineState<F> {
    func: F,

    // Need to keep our `VMSharedTypeIndex` registered in the engine.
    _sig: RegisteredType,
}

/// Shim to call a host-defined function that uses the array calling convention.
///
/// Together with `VMArrayCallHostFuncContext`, this implements the transition
/// from a raw, non-closure function pointer to a Rust closure that associates
/// data and function together.
///
/// Also shepherds panics and traps across Wasm.
///
/// # Safety
///
/// Requires that all parameters are valid from a wasm function call and
/// additionally that `vmctx` is backed by `VMArrayCallHostFuncContext`.
unsafe extern "C" fn array_call_shim<F>(
    vmctx: NonNull<VMOpaqueContext>,
    caller_vmctx: NonNull<VMContext>,
    values_vec: NonNull<ValRaw>,
    values_vec_len: usize,
) -> bool
where
    F: Fn(NonNull<VMContext>, &mut [ValRaw]) -> Result<()> + 'static,
{
    // Be sure to catch Rust panics to manually shepherd them across the wasm
    // boundary, and then otherwise delegate as normal.
    crate::runtime::vm::catch_unwind_and_record_trap(|| {
        // SAFETY: this function itself requires that the `vmctx` is valid to
        // use here.
        let state = unsafe {
            let vmctx = VMArrayCallHostFuncContext::from_opaque(vmctx);
            vmctx.as_ref().host_state()
        };

        // Double-check ourselves in debug mode, but we control
        // the `Any` here so an unsafe downcast should also
        // work.
        //
        // SAFETY: this function is only usable with `TrampolineState<F>`.
        let state = unsafe {
            debug_assert!(state.is::<TrampolineState<F>>());
            &*(state as *const _ as *const TrampolineState<F>)
        };
        let mut values_vec = NonNull::slice_from_raw_parts(values_vec, values_vec_len);
        // SAFETY: it's a contract of this function itself that the values
        // provided are valid to view as a slice.
        let values_vec = unsafe { values_vec.as_mut() };
        (state.func)(caller_vmctx, values_vec)
    })
}

pub fn create_array_call_function<F>(
    ft: &FuncType,
    func: F,
) -> Result<StoreBox<VMArrayCallHostFuncContext>>
where
    F: Fn(NonNull<VMContext>, &mut [ValRaw]) -> Result<()> + Send + Sync + 'static,
{
    let array_call = array_call_shim::<F>;

    let sig = ft.clone().into_registered_type();

    unsafe {
        Ok(VMArrayCallHostFuncContext::new(
            array_call,
            sig.index(),
            Box::new(TrampolineState { func, _sig: sig }),
        ))
    }
}
