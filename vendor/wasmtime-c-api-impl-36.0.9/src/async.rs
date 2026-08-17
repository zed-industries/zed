use std::ffi::c_void;
use std::future::Future;
use std::mem::{self, MaybeUninit};
use std::num::NonZeroU64;
use std::ops::Range;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use std::{ptr, str};
use wasmtime::{
    AsContextMut, Func, Instance, Result, RootScope, StackCreator, StackMemory, Trap, Val,
};

use crate::{
    WASMTIME_I32, WasmtimeCaller, WasmtimeStoreContextMut, bad_utf8, handle_result, to_str,
    translate_args, wasm_config_t, wasm_functype_t, wasm_trap_t, wasmtime_caller_t,
    wasmtime_error_t, wasmtime_instance_pre_t, wasmtime_linker_t, wasmtime_module_t,
    wasmtime_val_t, wasmtime_val_union,
};

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_async_support_set(c: &mut wasm_config_t, enable: bool) {
    c.config.async_support(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_async_stack_size_set(c: &mut wasm_config_t, size: usize) {
    c.config.async_stack_size(size);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_context_epoch_deadline_async_yield_and_update(
    mut store: WasmtimeStoreContextMut<'_>,
    delta: u64,
) {
    store.epoch_deadline_async_yield_and_update(delta);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_context_fuel_async_yield_interval(
    mut store: WasmtimeStoreContextMut<'_>,
    interval: Option<NonZeroU64>,
) -> Option<Box<wasmtime_error_t>> {
    handle_result(
        store.fuel_async_yield_interval(interval.map(|n| n.get())),
        |()| {},
    )
}

pub type wasmtime_func_async_callback_t = extern "C" fn(
    *mut c_void,
    *mut wasmtime_caller_t,
    *const wasmtime_val_t,
    usize,
    *mut wasmtime_val_t,
    usize,
    &mut Option<Box<wasm_trap_t>>,
    &mut wasmtime_async_continuation_t,
);

#[repr(C)]
pub struct wasmtime_async_continuation_t {
    pub callback: wasmtime_func_async_continuation_callback_t,
    pub env: *mut c_void,
    pub finalizer: Option<extern "C" fn(*mut c_void)>,
}

unsafe impl Send for wasmtime_async_continuation_t {}
unsafe impl Sync for wasmtime_async_continuation_t {}
impl Drop for wasmtime_async_continuation_t {
    fn drop(&mut self) {
        if let Some(f) = self.finalizer {
            f(self.env);
        }
    }
}
impl Future for wasmtime_async_continuation_t {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        let this = self.get_mut();
        let cb = this.callback;
        if cb(this.env) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

/// Internal structure to add Send/Sync to a c_void member.
///
/// This is useful in closures that need to capture some C data.
#[derive(Debug)]
struct CallbackDataPtr {
    pub ptr: *mut std::ffi::c_void,
}

unsafe impl Send for CallbackDataPtr {}
unsafe impl Sync for CallbackDataPtr {}

pub type wasmtime_func_async_continuation_callback_t = extern "C" fn(*mut c_void) -> bool;

async fn invoke_c_async_callback<'a>(
    cb: wasmtime_func_async_callback_t,
    data: CallbackDataPtr,
    mut caller: WasmtimeCaller<'a>,
    params: &'a [Val],
    results: &'a mut [Val],
) -> Result<()> {
    // Convert `params/results` to `wasmtime_val_t`. Use the previous
    // storage in `hostcall_val_storage` to help avoid allocations all the
    // time.
    let mut hostcall_val_storage = mem::take(&mut caller.data_mut().hostcall_val_storage);
    debug_assert!(hostcall_val_storage.is_empty());
    hostcall_val_storage.reserve(params.len() + results.len());
    hostcall_val_storage.extend(
        params
            .iter()
            .cloned()
            .map(|p| wasmtime_val_t::from_val_unscoped(&mut caller, p)),
    );
    hostcall_val_storage.extend((0..results.len()).map(|_| wasmtime_val_t {
        kind: WASMTIME_I32,
        of: wasmtime_val_union { i32: 0 },
    }));
    let (params, out_results) = hostcall_val_storage.split_at_mut(params.len());

    // Invoke the C function pointer.
    // The result will be a continuation which we will wrap in a Future.
    let mut caller = wasmtime_caller_t { caller };
    let mut trap = None;
    extern "C" fn panic_callback(_: *mut c_void) -> bool {
        panic!("callback must be set")
    }
    let mut continuation = wasmtime_async_continuation_t {
        callback: panic_callback,
        env: ptr::null_mut(),
        finalizer: None,
    };
    cb(
        data.ptr,
        &mut caller,
        params.as_ptr(),
        params.len(),
        out_results.as_mut_ptr(),
        out_results.len(),
        &mut trap,
        &mut continuation,
    );
    continuation.await;

    if let Some(trap) = trap {
        return Err(trap.error);
    }

    // Translate the `wasmtime_val_t` results into the `results` space
    for (i, result) in out_results.iter().enumerate() {
        unsafe {
            results[i] = result.to_val_unscoped(&mut caller.caller);
        }
    }
    // Move our `vals` storage back into the store now that we no longer
    // need it. This'll get picked up by the next hostcall and reuse our
    // same storage.
    hostcall_val_storage.truncate(0);
    caller.caller.data_mut().hostcall_val_storage = hostcall_val_storage;
    Ok(())
}

unsafe fn c_async_callback_to_rust_fn(
    callback: wasmtime_func_async_callback_t,
    data: *mut c_void,
    finalizer: Option<extern "C" fn(*mut std::ffi::c_void)>,
) -> impl for<'a> Fn(
    WasmtimeCaller<'a>,
    &'a [Val],
    &'a mut [Val],
) -> Box<dyn Future<Output = Result<()>> + Send + 'a>
+ Send
+ Sync
+ 'static {
    let foreign = crate::ForeignData { data, finalizer };
    move |caller, params, results| {
        let _ = &foreign; // move entire foreign into this closure
        let data = CallbackDataPtr { ptr: foreign.data };
        Box::new(invoke_c_async_callback(
            callback, data, caller, params, results,
        ))
    }
}

#[repr(transparent)]
pub struct wasmtime_call_future_t<'a> {
    underlying: Pin<Box<dyn Future<Output = ()> + 'a>>,
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_call_future_delete(_future: Box<wasmtime_call_future_t>) {}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_call_future_poll(future: &mut wasmtime_call_future_t) -> bool {
    match future
        .underlying
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(()) => true,
        Poll::Pending => false,
    }
}

fn handle_call_error(
    err: wasmtime::Error,
    trap_ret: &mut *mut wasm_trap_t,
    err_ret: &mut *mut wasmtime_error_t,
) {
    if err.is::<Trap>() {
        *trap_ret = Box::into_raw(Box::new(wasm_trap_t::new(err)));
    } else {
        *err_ret = Box::into_raw(Box::new(wasmtime_error_t::from(err)));
    }
}

async fn do_func_call_async(
    mut store: RootScope<WasmtimeStoreContextMut<'_>>,
    func: &Func,
    args: impl ExactSizeIterator<Item = Val>,
    results: &mut [MaybeUninit<wasmtime_val_t>],
    trap_ret: &mut *mut wasm_trap_t,
    err_ret: &mut *mut wasmtime_error_t,
) {
    let mut params = mem::take(&mut store.as_context_mut().data_mut().wasm_val_storage);
    let (wt_params, wt_results) = translate_args(&mut params, args, results.len());
    let result = func.call_async(&mut store, wt_params, wt_results).await;

    match result {
        Ok(()) => {
            for (slot, val) in results.iter_mut().zip(wt_results.iter()) {
                crate::initialize(slot, wasmtime_val_t::from_val(&mut store, *val));
            }
            params.truncate(0);
            store.as_context_mut().data_mut().wasm_val_storage = params;
        }
        Err(err) => handle_call_error(err, trap_ret, err_ret),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_func_call_async<'a>(
    store: WasmtimeStoreContextMut<'a>,
    func: &'a Func,
    args: *const wasmtime_val_t,
    nargs: usize,
    results: *mut MaybeUninit<wasmtime_val_t>,
    nresults: usize,
    trap_ret: &'a mut *mut wasm_trap_t,
    err_ret: &'a mut *mut wasmtime_error_t,
) -> Box<wasmtime_call_future_t<'a>> {
    let mut scope = RootScope::new(store);
    let args = crate::slice_from_raw_parts(args, nargs)
        .iter()
        .map(|i| i.to_val(&mut scope))
        .collect::<Vec<_>>();
    let results = crate::slice_from_raw_parts_mut(results, nresults);
    let fut = Box::pin(do_func_call_async(
        scope,
        func,
        args.into_iter(),
        results,
        trap_ret,
        err_ret,
    ));
    Box::new(wasmtime_call_future_t { underlying: fut })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_linker_define_async_func(
    linker: &mut wasmtime_linker_t,
    module: *const u8,
    module_len: usize,
    name: *const u8,
    name_len: usize,
    ty: &wasm_functype_t,
    callback: crate::wasmtime_func_async_callback_t,
    data: *mut c_void,
    finalizer: Option<extern "C" fn(*mut std::ffi::c_void)>,
) -> Option<Box<wasmtime_error_t>> {
    let ty = ty.ty().ty(linker.linker.engine());
    let module = to_str!(module, module_len);
    let name = to_str!(name, name_len);
    let cb = c_async_callback_to_rust_fn(callback, data, finalizer);

    handle_result(
        linker.linker.func_new_async(module, name, ty, cb),
        |_linker| (),
    )
}

async fn do_linker_instantiate_async(
    linker: &wasmtime_linker_t,
    store: WasmtimeStoreContextMut<'_>,
    module: &wasmtime_module_t,
    instance_ptr: &mut Instance,
    trap_ret: &mut *mut wasm_trap_t,
    err_ret: &mut *mut wasmtime_error_t,
) {
    let result = linker.linker.instantiate_async(store, &module.module).await;
    match result {
        Ok(instance) => *instance_ptr = instance,
        Err(err) => handle_call_error(err, trap_ret, err_ret),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_linker_instantiate_async<'a>(
    linker: &'a wasmtime_linker_t,
    store: WasmtimeStoreContextMut<'a>,
    module: &'a wasmtime_module_t,
    instance_ptr: &'a mut Instance,
    trap_ret: &'a mut *mut wasm_trap_t,
    err_ret: &'a mut *mut wasmtime_error_t,
) -> Box<crate::wasmtime_call_future_t<'a>> {
    let fut = Box::pin(do_linker_instantiate_async(
        linker,
        store,
        module,
        instance_ptr,
        trap_ret,
        err_ret,
    ));
    Box::new(crate::wasmtime_call_future_t { underlying: fut })
}

async fn do_instance_pre_instantiate_async(
    instance_pre: &wasmtime_instance_pre_t,
    store: WasmtimeStoreContextMut<'_>,
    instance_ptr: &mut Instance,
    trap_ret: &mut *mut wasm_trap_t,
    err_ret: &mut *mut wasmtime_error_t,
) {
    let result = instance_pre.underlying.instantiate_async(store).await;
    match result {
        Ok(instance) => *instance_ptr = instance,
        Err(err) => handle_call_error(err, trap_ret, err_ret),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_instance_pre_instantiate_async<'a>(
    instance_pre: &'a wasmtime_instance_pre_t,
    store: WasmtimeStoreContextMut<'a>,
    instance_ptr: &'a mut Instance,
    trap_ret: &'a mut *mut wasm_trap_t,
    err_ret: &'a mut *mut wasmtime_error_t,
) -> Box<crate::wasmtime_call_future_t<'a>> {
    let fut = Box::pin(do_instance_pre_instantiate_async(
        instance_pre,
        store,
        instance_ptr,
        trap_ret,
        err_ret,
    ));
    Box::new(crate::wasmtime_call_future_t { underlying: fut })
}

pub type wasmtime_stack_memory_get_callback_t =
    extern "C" fn(env: *mut std::ffi::c_void, out_len: &mut usize) -> *mut u8;

#[repr(C)]
pub struct wasmtime_stack_memory_t {
    env: *mut std::ffi::c_void,
    get_stack_memory: wasmtime_stack_memory_get_callback_t,
    finalizer: Option<extern "C" fn(arg1: *mut std::ffi::c_void)>,
}

struct CHostStackMemory {
    foreign: crate::ForeignData,
    get_memory: wasmtime_stack_memory_get_callback_t,
}
unsafe impl Send for CHostStackMemory {}
unsafe impl Sync for CHostStackMemory {}
unsafe impl StackMemory for CHostStackMemory {
    fn top(&self) -> *mut u8 {
        let mut len = 0;
        let cb = self.get_memory;
        cb(self.foreign.data, &mut len)
    }
    fn range(&self) -> Range<usize> {
        let mut len = 0;
        let cb = self.get_memory;
        let top = cb(self.foreign.data, &mut len);
        let base = unsafe { top.sub(len) as usize };
        base..base + len
    }
    fn guard_range(&self) -> Range<*mut u8> {
        std::ptr::null_mut()..std::ptr::null_mut()
    }
}

pub type wasmtime_new_stack_memory_callback_t = extern "C" fn(
    env: *mut std::ffi::c_void,
    size: usize,
    zeroed: bool,
    stack_ret: &mut wasmtime_stack_memory_t,
) -> Option<Box<wasmtime_error_t>>;

#[repr(C)]
pub struct wasmtime_stack_creator_t {
    env: *mut std::ffi::c_void,
    new_stack: wasmtime_new_stack_memory_callback_t,
    finalizer: Option<extern "C" fn(arg1: *mut std::ffi::c_void)>,
}

struct CHostStackCreator {
    foreign: crate::ForeignData,
    new_stack: wasmtime_new_stack_memory_callback_t,
}
unsafe impl Send for CHostStackCreator {}
unsafe impl Sync for CHostStackCreator {}
unsafe impl StackCreator for CHostStackCreator {
    fn new_stack(&self, size: usize, zeroed: bool) -> Result<Box<dyn wasmtime::StackMemory>> {
        extern "C" fn panic_callback(_env: *mut std::ffi::c_void, _out_len: &mut usize) -> *mut u8 {
            panic!("a callback must be set");
        }
        let mut out = wasmtime_stack_memory_t {
            env: ptr::null_mut(),
            get_stack_memory: panic_callback,
            finalizer: None,
        };
        let cb = self.new_stack;
        let result = cb(self.foreign.data, size, zeroed, &mut out);
        match result {
            Some(error) => Err((*error).into()),
            None => Ok(Box::new(CHostStackMemory {
                foreign: crate::ForeignData {
                    data: out.env,
                    finalizer: out.finalizer,
                },
                get_memory: out.get_stack_memory,
            })),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_config_host_stack_creator_set(
    c: &mut wasm_config_t,
    creator: &wasmtime_stack_creator_t,
) {
    c.config.with_host_stack(Arc::new(CHostStackCreator {
        foreign: crate::ForeignData {
            data: creator.env,
            finalizer: creator.finalizer,
        },
        new_stack: creator.new_stack,
    }));
}
