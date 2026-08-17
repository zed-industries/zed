use crate::{WasmtimeCaller, WasmtimeStoreData, wasm_trap_t};
use crate::{
    WasmtimeStoreContext, WasmtimeStoreContextMut, wasm_extern_t, wasm_functype_t, wasm_store_t,
    wasm_val_t, wasm_val_vec_t, wasmtime_error_t, wasmtime_extern_t, wasmtime_val_t,
    wasmtime_val_union,
};
use anyhow::{Error, Result};
use std::any::Any;
use std::ffi::c_void;
use std::mem::{self, MaybeUninit};
use std::panic::{self, AssertUnwindSafe};
use std::ptr;
use std::str;
use wasmtime::{
    AsContext, AsContextMut, Extern, Func, RootScope, StoreContext, StoreContextMut, Trap, Val,
    ValRaw,
};

#[derive(Clone)]
#[repr(transparent)]
pub struct wasm_func_t {
    ext: wasm_extern_t,
}

wasmtime_c_api_macros::declare_ref!(wasm_func_t);

pub type wasm_func_callback_t = extern "C" fn(
    args: *const wasm_val_vec_t,
    results: *mut wasm_val_vec_t,
) -> Option<Box<wasm_trap_t>>;

pub type wasm_func_callback_with_env_t = extern "C" fn(
    env: *mut std::ffi::c_void,
    args: *const wasm_val_vec_t,
    results: *mut wasm_val_vec_t,
) -> Option<Box<wasm_trap_t>>;

impl wasm_func_t {
    pub(crate) fn try_from(e: &wasm_extern_t) -> Option<&wasm_func_t> {
        match &e.which {
            Extern::Func(_) => Some(unsafe { &*(e as *const _ as *const _) }),
            _ => None,
        }
    }

    pub(crate) fn func(&self) -> Func {
        match self.ext.which {
            Extern::Func(f) => f,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

unsafe fn create_function(
    store: &mut wasm_store_t,
    ty: &wasm_functype_t,
    func: impl Fn(*const wasm_val_vec_t, *mut wasm_val_vec_t) -> Option<Box<wasm_trap_t>>
    + Send
    + Sync
    + 'static,
) -> Box<wasm_func_t> {
    let ty = ty.ty().ty(store.store.context().engine());
    let func = Func::new(
        store.store.context_mut(),
        ty,
        move |_caller, params, results| {
            let params: wasm_val_vec_t = params
                .iter()
                .cloned()
                .map(|p| wasm_val_t::from_val(p))
                .collect::<Vec<_>>()
                .into();
            let mut out_results: wasm_val_vec_t = vec![wasm_val_t::default(); results.len()].into();
            let out = func(&params, &mut out_results);
            if let Some(trap) = out {
                return Err(trap.error);
            }

            let out_results = out_results.as_slice();
            for i in 0..results.len() {
                results[i] = out_results[i].val();
            }
            Ok(())
        },
    );
    Box::new(wasm_func_t {
        ext: wasm_extern_t {
            store: store.store.clone(),
            which: func.into(),
        },
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_func_new(
    store: &mut wasm_store_t,
    ty: &wasm_functype_t,
    callback: wasm_func_callback_t,
) -> Box<wasm_func_t> {
    create_function(store, ty, move |params, results| callback(params, results))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_func_new_with_env(
    store: &mut wasm_store_t,
    ty: &wasm_functype_t,
    callback: wasm_func_callback_with_env_t,
    data: *mut c_void,
    finalizer: Option<extern "C" fn(arg1: *mut std::ffi::c_void)>,
) -> Box<wasm_func_t> {
    let finalizer = crate::ForeignData { data, finalizer };
    create_function(store, ty, move |params, results| {
        let _ = &finalizer; // move entire finalizer into this closure
        callback(finalizer.data, params, results)
    })
}

/// Places the `args` into `dst` and additionally reserves space in `dst` for `results_size`
/// returns. The params/results slices are then returned separately.
pub(crate) fn translate_args<'a>(
    dst: &'a mut Vec<Val>,
    args: impl ExactSizeIterator<Item = Val>,
    results_size: usize,
) -> (&'a [Val], &'a mut [Val]) {
    debug_assert!(dst.is_empty());
    let num_args = args.len();
    dst.reserve(args.len() + results_size);
    dst.extend(args);
    dst.extend((0..results_size).map(|_| Val::null_func_ref()));
    let (a, b) = dst.split_at_mut(num_args);
    (a, b)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_func_call(
    func: &mut wasm_func_t,
    args: *const wasm_val_vec_t,
    results: *mut wasm_val_vec_t,
) -> *mut wasm_trap_t {
    let f = func.func();
    let results = (*results).as_uninit_slice();
    let args = (*args).as_slice();
    let mut dst = Vec::new();
    let (wt_params, wt_results) =
        translate_args(&mut dst, args.iter().map(|i| i.val()), results.len());

    // We're calling arbitrary code here most of the time, and we in general
    // want to try to insulate callers against bugs in wasmtime/wasi/etc if we
    // can. As a result we catch panics here and transform them to traps to
    // allow the caller to have any insulation possible against Rust panics.
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        f.call(func.ext.store.context_mut(), wt_params, wt_results)
    }));
    match result {
        Ok(Ok(())) => {
            for (slot, val) in results.iter_mut().zip(wt_results.iter().cloned()) {
                crate::initialize(slot, wasm_val_t::from_val(val));
            }
            ptr::null_mut()
        }
        Ok(Err(err)) => Box::into_raw(Box::new(wasm_trap_t::new(err))),
        Err(panic) => {
            let err = error_from_panic(panic);
            let trap = Box::new(wasm_trap_t::new(err));
            Box::into_raw(trap)
        }
    }
}

fn error_from_panic(panic: Box<dyn Any + Send>) -> Error {
    if let Some(msg) = panic.downcast_ref::<String>() {
        Error::msg(msg.clone())
    } else if let Some(msg) = panic.downcast_ref::<&'static str>() {
        Error::msg(*msg)
    } else {
        Error::msg("rust panic happened")
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_func_type(f: &wasm_func_t) -> Box<wasm_functype_t> {
    Box::new(wasm_functype_t::new(f.func().ty(f.ext.store.context())))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_func_param_arity(f: &wasm_func_t) -> usize {
    f.func().ty(f.ext.store.context()).params().len()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_func_result_arity(f: &wasm_func_t) -> usize {
    f.func().ty(f.ext.store.context()).results().len()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_func_as_extern(f: &mut wasm_func_t) -> &mut wasm_extern_t {
    &mut (*f).ext
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_func_as_extern_const(f: &wasm_func_t) -> &wasm_extern_t {
    &(*f).ext
}

#[repr(C)]
pub struct wasmtime_caller_t<'a> {
    pub(crate) caller: WasmtimeCaller<'a>,
}

impl AsContext for wasmtime_caller_t<'_> {
    type Data = WasmtimeStoreData;

    fn as_context(&self) -> StoreContext<'_, WasmtimeStoreData> {
        self.caller.as_context()
    }
}

impl AsContextMut for wasmtime_caller_t<'_> {
    fn as_context_mut(&mut self) -> StoreContextMut<'_, WasmtimeStoreData> {
        self.caller.as_context_mut()
    }
}

pub type wasmtime_func_callback_t = extern "C" fn(
    *mut c_void,
    *mut wasmtime_caller_t,
    *const wasmtime_val_t,
    usize,
    *mut wasmtime_val_t,
    usize,
) -> Option<Box<wasm_trap_t>>;

pub type wasmtime_func_unchecked_callback_t = extern "C" fn(
    *mut c_void,
    *mut wasmtime_caller_t,
    *mut ValRaw,
    usize,
) -> Option<Box<wasm_trap_t>>;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_func_new(
    store: WasmtimeStoreContextMut<'_>,
    ty: &wasm_functype_t,
    callback: wasmtime_func_callback_t,
    data: *mut c_void,
    finalizer: Option<extern "C" fn(*mut std::ffi::c_void)>,
    func: &mut Func,
) {
    let ty = ty.ty().ty(store.engine());
    let cb = c_callback_to_rust_fn(callback, data, finalizer);
    let f = Func::new(store, ty, cb);
    *func = f;
}

pub(crate) unsafe fn c_callback_to_rust_fn(
    callback: wasmtime_func_callback_t,
    data: *mut c_void,
    finalizer: Option<extern "C" fn(*mut std::ffi::c_void)>,
) -> impl Fn(WasmtimeCaller<'_>, &[Val], &mut [Val]) -> Result<()> {
    let foreign = crate::ForeignData { data, finalizer };
    move |mut caller, params, results| {
        let _ = &foreign; // move entire foreign into this closure

        // Convert `params/results` to `wasmtime_val_t`. Use the previous
        // storage in `hostcall_val_storage` to help avoid allocations all the
        // time.
        let mut vals = mem::take(&mut caller.data_mut().hostcall_val_storage);
        debug_assert!(vals.is_empty());
        vals.reserve(params.len() + results.len());
        vals.extend(
            params
                .iter()
                .cloned()
                .map(|p| wasmtime_val_t::from_val_unscoped(&mut caller, p)),
        );
        vals.extend((0..results.len()).map(|_| wasmtime_val_t {
            kind: crate::WASMTIME_I32,
            of: wasmtime_val_union { i32: 0 },
        }));
        let (params, out_results) = vals.split_at_mut(params.len());

        // Invoke the C function pointer, getting the results.
        let mut caller = wasmtime_caller_t { caller };
        let out = callback(
            foreign.data,
            &mut caller,
            params.as_ptr(),
            params.len(),
            out_results.as_mut_ptr(),
            out_results.len(),
        );
        if let Some(trap) = out {
            return Err(trap.error);
        }

        // Translate the `wasmtime_val_t` results into the `results` space
        for (i, result) in out_results.iter().enumerate() {
            results[i] = result.to_val_unscoped(&mut caller);
        }

        // Move our `vals` storage back into the store now that we no longer
        // need it. This'll get picked up by the next hostcall and reuse our
        // same storage.
        vals.truncate(0);
        caller.caller.data_mut().hostcall_val_storage = vals;
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_func_new_unchecked(
    store: WasmtimeStoreContextMut<'_>,
    ty: &wasm_functype_t,
    callback: wasmtime_func_unchecked_callback_t,
    data: *mut c_void,
    finalizer: Option<extern "C" fn(*mut std::ffi::c_void)>,
    func: &mut Func,
) {
    let ty = ty.ty().ty(store.engine());
    let cb = c_unchecked_callback_to_rust_fn(callback, data, finalizer);
    *func = Func::new_unchecked(store, ty, cb);
}

pub(crate) unsafe fn c_unchecked_callback_to_rust_fn(
    callback: wasmtime_func_unchecked_callback_t,
    data: *mut c_void,
    finalizer: Option<extern "C" fn(*mut std::ffi::c_void)>,
) -> impl Fn(WasmtimeCaller<'_>, &mut [ValRaw]) -> Result<()> {
    let foreign = crate::ForeignData { data, finalizer };
    move |caller, values| {
        let _ = &foreign; // move entire foreign into this closure
        let mut caller = wasmtime_caller_t { caller };
        match callback(foreign.data, &mut caller, values.as_mut_ptr(), values.len()) {
            None => Ok(()),
            Some(trap) => Err(trap.error),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_func_call(
    mut store: WasmtimeStoreContextMut<'_>,
    func: &Func,
    args: *const wasmtime_val_t,
    nargs: usize,
    results: *mut MaybeUninit<wasmtime_val_t>,
    nresults: usize,
    trap_ret: &mut *mut wasm_trap_t,
) -> Option<Box<wasmtime_error_t>> {
    let mut scope = RootScope::new(&mut store);
    let mut params = mem::take(&mut scope.as_context_mut().data_mut().wasm_val_storage);
    let (wt_params, wt_results) = translate_args(
        &mut params,
        crate::slice_from_raw_parts(args, nargs)
            .iter()
            .map(|i| i.to_val(&mut scope)),
        nresults,
    );

    // We're calling arbitrary code here most of the time, and we in general
    // want to try to insulate callers against bugs in wasmtime/wasi/etc if we
    // can. As a result we catch panics here and transform them to traps to
    // allow the caller to have any insulation possible against Rust panics.
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        func.call(&mut scope, wt_params, wt_results)
    }));
    match result {
        Ok(Ok(())) => {
            let results = crate::slice_from_raw_parts_mut(results, nresults);
            for (slot, val) in results.iter_mut().zip(wt_results.iter()) {
                crate::initialize(slot, wasmtime_val_t::from_val(&mut scope, *val));
            }
            params.truncate(0);
            scope.as_context_mut().data_mut().wasm_val_storage = params;
            None
        }
        Ok(Err(trap)) => store_err(trap, trap_ret),
        Err(panic) => {
            let err = error_from_panic(panic);
            *trap_ret = Box::into_raw(Box::new(wasm_trap_t::new(err)));
            None
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_func_call_unchecked(
    store: WasmtimeStoreContextMut<'_>,
    func: &Func,
    args_and_results: *mut ValRaw,
    args_and_results_len: usize,
    trap_ret: &mut *mut wasm_trap_t,
) -> Option<Box<wasmtime_error_t>> {
    let slice = std::ptr::slice_from_raw_parts_mut(args_and_results, args_and_results_len);
    match func.call_unchecked(store, slice) {
        Ok(()) => None,
        Err(trap) => store_err(trap, trap_ret),
    }
}

fn store_err(err: Error, trap_ret: &mut *mut wasm_trap_t) -> Option<Box<wasmtime_error_t>> {
    if err.is::<Trap>() {
        *trap_ret = Box::into_raw(Box::new(wasm_trap_t::new(err)));
        None
    } else {
        Some(Box::new(wasmtime_error_t::from(err)))
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_func_type(
    store: WasmtimeStoreContext<'_>,
    func: &Func,
) -> Box<wasm_functype_t> {
    Box::new(wasm_functype_t::new(func.ty(store)))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_caller_context<'a>(
    caller: &'a mut wasmtime_caller_t,
) -> WasmtimeStoreContextMut<'a> {
    caller.caller.as_context_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_caller_export_get(
    caller: &mut wasmtime_caller_t,
    name: *const u8,
    name_len: usize,
    item: &mut MaybeUninit<wasmtime_extern_t>,
) -> bool {
    let name = match str::from_utf8(crate::slice_from_raw_parts(name, name_len)) {
        Ok(name) => name,
        Err(_) => return false,
    };
    let which = match caller.caller.get_export(name) {
        Some(item) => item,
        None => return false,
    };
    crate::initialize(item, which.into());
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_func_from_raw(
    store: WasmtimeStoreContextMut<'_>,
    raw: *mut c_void,
    func: &mut Func,
) {
    *func = Func::from_raw(store, raw).unwrap();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_func_to_raw(
    store: WasmtimeStoreContextMut<'_>,
    func: &Func,
) -> *mut c_void {
    func.to_raw(store)
}
