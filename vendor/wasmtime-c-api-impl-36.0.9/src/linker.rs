use crate::{
    WasmtimeStoreContext, WasmtimeStoreContextMut, bad_utf8, handle_result, wasm_engine_t,
    wasm_functype_t, wasm_trap_t, wasmtime_error_t, wasmtime_extern_t, wasmtime_instance_pre_t,
    wasmtime_module_t,
};
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::str;
use wasmtime::{Func, Instance, Linker};

#[repr(C)]
pub struct wasmtime_linker_t {
    pub(crate) linker: Linker<crate::WasmtimeStoreData>,
}

wasmtime_c_api_macros::declare_own!(wasmtime_linker_t);

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_linker_new(engine: &wasm_engine_t) -> Box<wasmtime_linker_t> {
    Box::new(wasmtime_linker_t {
        linker: Linker::new(&engine.engine),
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_linker_clone(linker: &wasmtime_linker_t) -> Box<wasmtime_linker_t> {
    Box::new(wasmtime_linker_t {
        linker: linker.linker.clone(),
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_linker_allow_shadowing(
    linker: &mut wasmtime_linker_t,
    allow_shadowing: bool,
) {
    linker.linker.allow_shadowing(allow_shadowing);
}

macro_rules! to_str {
    ($ptr:expr, $len:expr) => {
        match str::from_utf8(crate::slice_from_raw_parts($ptr, $len)) {
            Ok(s) => s,
            Err(_) => return bad_utf8(),
        }
    };
}

pub(crate) use to_str;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_linker_define(
    linker: &mut wasmtime_linker_t,
    store: WasmtimeStoreContext<'_>,
    module: *const u8,
    module_len: usize,
    name: *const u8,
    name_len: usize,
    item: &wasmtime_extern_t,
) -> Option<Box<wasmtime_error_t>> {
    let linker = &mut linker.linker;
    let module = to_str!(module, module_len);
    let name = to_str!(name, name_len);
    let item = item.to_extern();
    handle_result(linker.define(&store, module, name, item), |_linker| ())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_linker_define_func(
    linker: &mut wasmtime_linker_t,
    module: *const u8,
    module_len: usize,
    name: *const u8,
    name_len: usize,
    ty: &wasm_functype_t,
    callback: crate::wasmtime_func_callback_t,
    data: *mut c_void,
    finalizer: Option<extern "C" fn(*mut std::ffi::c_void)>,
) -> Option<Box<wasmtime_error_t>> {
    let ty = ty.ty().ty(linker.linker.engine());
    let module = to_str!(module, module_len);
    let name = to_str!(name, name_len);
    let cb = crate::func::c_callback_to_rust_fn(callback, data, finalizer);
    handle_result(linker.linker.func_new(module, name, ty, cb), |_linker| ())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_linker_define_func_unchecked(
    linker: &mut wasmtime_linker_t,
    module: *const u8,
    module_len: usize,
    name: *const u8,
    name_len: usize,
    ty: &wasm_functype_t,
    callback: crate::wasmtime_func_unchecked_callback_t,
    data: *mut c_void,
    finalizer: Option<extern "C" fn(*mut std::ffi::c_void)>,
) -> Option<Box<wasmtime_error_t>> {
    let ty = ty.ty().ty(linker.linker.engine());
    let module = to_str!(module, module_len);
    let name = to_str!(name, name_len);
    let cb = crate::func::c_unchecked_callback_to_rust_fn(callback, data, finalizer);
    handle_result(
        linker.linker.func_new_unchecked(module, name, ty, cb),
        |_linker| (),
    )
}

#[cfg(feature = "wasi")]
#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_linker_define_wasi(
    linker: &mut wasmtime_linker_t,
) -> Option<Box<wasmtime_error_t>> {
    handle_result(
        wasmtime_wasi::preview1::add_to_linker_sync(&mut linker.linker, |ctx| {
            ctx.wasi.as_mut().expect("wasi context must be populated")
        }),
        |_linker| (),
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_linker_define_instance(
    linker: &mut wasmtime_linker_t,
    store: WasmtimeStoreContextMut<'_>,
    name: *const u8,
    name_len: usize,
    instance: &Instance,
) -> Option<Box<wasmtime_error_t>> {
    let linker = &mut linker.linker;
    let name = to_str!(name, name_len);
    handle_result(linker.instance(store, name, *instance), |_linker| ())
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_linker_instantiate(
    linker: &wasmtime_linker_t,
    store: WasmtimeStoreContextMut<'_>,
    module: &wasmtime_module_t,
    instance_ptr: &mut Instance,
    trap_ptr: &mut *mut wasm_trap_t,
) -> Option<Box<wasmtime_error_t>> {
    let result = linker.linker.instantiate(store, &module.module);
    super::instance::handle_instantiate(result, instance_ptr, trap_ptr)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_linker_instantiate_pre(
    linker: &wasmtime_linker_t,
    module: &wasmtime_module_t,
    instance_ptr: &mut *mut wasmtime_instance_pre_t,
) -> Option<Box<wasmtime_error_t>> {
    let linker = &linker.linker;
    handle_result(linker.instantiate_pre(&module.module), |i| {
        let instance_pre = Box::new(wasmtime_instance_pre_t { underlying: i });
        *instance_ptr = Box::into_raw(instance_pre)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_linker_module(
    linker: &mut wasmtime_linker_t,
    store: WasmtimeStoreContextMut<'_>,
    name: *const u8,
    name_len: usize,
    module: &wasmtime_module_t,
) -> Option<Box<wasmtime_error_t>> {
    let linker = &mut linker.linker;
    let name = to_str!(name, name_len);
    handle_result(linker.module(store, name, &module.module), |_linker| ())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_linker_get_default(
    linker: &wasmtime_linker_t,
    store: WasmtimeStoreContextMut<'_>,
    name: *const u8,
    name_len: usize,
    func: &mut Func,
) -> Option<Box<wasmtime_error_t>> {
    let linker = &linker.linker;
    let name = to_str!(name, name_len);
    handle_result(linker.get_default(store, name), |f| *func = f)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_linker_get(
    linker: &wasmtime_linker_t,
    store: WasmtimeStoreContextMut<'_>,
    module: *const u8,
    module_len: usize,
    name: *const u8,
    name_len: usize,
    item_ptr: &mut MaybeUninit<wasmtime_extern_t>,
) -> bool {
    let linker = &linker.linker;
    let module = match str::from_utf8(crate::slice_from_raw_parts(module, module_len)) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let name = match str::from_utf8(crate::slice_from_raw_parts(name, name_len)) {
        Ok(s) => s,
        Err(_) => return false,
    };
    match linker.get(store, module, name) {
        Some(which) => {
            crate::initialize(item_ptr, which.into());
            true
        }
        None => false,
    }
}
