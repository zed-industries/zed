use crate::{
    CExternType, handle_result, wasm_byte_vec_t, wasm_engine_t, wasm_exporttype_t,
    wasm_exporttype_vec_t, wasm_importtype_t, wasm_importtype_vec_t, wasm_store_t,
    wasmtime_error_t,
};
use anyhow::Context;
use std::ffi::CStr;
use std::os::raw::c_char;
use wasmtime::{Engine, Module};

#[derive(Clone)]
pub struct wasm_module_t {
    pub(crate) module: Module,
}

wasmtime_c_api_macros::declare_ref!(wasm_module_t);

impl wasm_module_t {
    pub(crate) fn new(module: Module) -> wasm_module_t {
        wasm_module_t { module }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct wasm_shared_module_t {
    module: Module,
}

wasmtime_c_api_macros::declare_own!(wasm_shared_module_t);

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub unsafe extern "C" fn wasm_module_new(
    store: &mut wasm_store_t,
    binary: &wasm_byte_vec_t,
) -> Option<Box<wasm_module_t>> {
    match Module::from_binary(store.store.context().engine(), binary.as_slice()) {
        Ok(module) => Some(Box::new(wasm_module_t::new(module))),
        Err(_) => None,
    }
}

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub unsafe extern "C" fn wasm_module_validate(
    store: &mut wasm_store_t,
    binary: &wasm_byte_vec_t,
) -> bool {
    Module::validate(store.store.context().engine(), binary.as_slice()).is_ok()
}

fn fill_exports(module: &Module, out: &mut wasm_exporttype_vec_t) {
    let exports = module
        .exports()
        .map(|e| {
            Some(Box::new(wasm_exporttype_t::new(
                e.name().to_owned(),
                CExternType::new(e.ty()),
            )))
        })
        .collect::<Vec<_>>();
    out.set_buffer(exports);
}

fn fill_imports(module: &Module, out: &mut wasm_importtype_vec_t) {
    let imports = module
        .imports()
        .map(|i| {
            Some(Box::new(wasm_importtype_t::new(
                i.module().to_owned(),
                i.name().to_owned(),
                CExternType::new(i.ty()),
            )))
        })
        .collect::<Vec<_>>();
    out.set_buffer(imports);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_module_exports(module: &wasm_module_t, out: &mut wasm_exporttype_vec_t) {
    fill_exports(&module.module, out);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_module_imports(module: &wasm_module_t, out: &mut wasm_importtype_vec_t) {
    fill_imports(&module.module, out);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_module_share(module: &wasm_module_t) -> Box<wasm_shared_module_t> {
    Box::new(wasm_shared_module_t {
        module: module.module.clone(),
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_module_obtain(
    store: &mut wasm_store_t,
    shared_module: &wasm_shared_module_t,
) -> Option<Box<wasm_module_t>> {
    let module = shared_module.module.clone();
    if Engine::same(store.store.context().engine(), module.engine()) {
        Some(Box::new(wasm_module_t::new(module)))
    } else {
        None
    }
}

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub extern "C" fn wasm_module_serialize(module: &wasm_module_t, ret: &mut wasm_byte_vec_t) {
    if let Ok(buf) = module.module.serialize() {
        ret.set_buffer(buf);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_module_deserialize(
    store: &mut wasm_store_t,
    binary: &wasm_byte_vec_t,
) -> Option<Box<wasm_module_t>> {
    match Module::deserialize(store.store.context().engine(), binary.as_slice()) {
        Ok(module) => Some(Box::new(wasm_module_t::new(module))),
        Err(_) => None,
    }
}

#[derive(Clone)]
pub struct wasmtime_module_t {
    pub(crate) module: Module,
}

wasmtime_c_api_macros::declare_own!(wasmtime_module_t);

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub unsafe extern "C" fn wasmtime_module_new(
    engine: &wasm_engine_t,
    wasm: *const u8,
    len: usize,
    out: &mut *mut wasmtime_module_t,
) -> Option<Box<wasmtime_error_t>> {
    handle_result(
        Module::from_binary(&engine.engine, crate::slice_from_raw_parts(wasm, len)),
        |module| {
            *out = Box::into_raw(Box::new(wasmtime_module_t { module }));
        },
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_module_clone(module: &wasmtime_module_t) -> Box<wasmtime_module_t> {
    Box::new(module.clone())
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_module_exports(
    module: &wasmtime_module_t,
    out: &mut wasm_exporttype_vec_t,
) {
    fill_exports(&module.module, out);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_module_imports(
    module: &wasmtime_module_t,
    out: &mut wasm_importtype_vec_t,
) {
    fill_imports(&module.module, out);
}

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub unsafe extern "C" fn wasmtime_module_validate(
    engine: &wasm_engine_t,
    wasm: *const u8,
    len: usize,
) -> Option<Box<wasmtime_error_t>> {
    let binary = crate::slice_from_raw_parts(wasm, len);
    handle_result(Module::validate(&engine.engine, binary), |()| {})
}

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub extern "C" fn wasmtime_module_serialize(
    module: &wasmtime_module_t,
    ret: &mut wasm_byte_vec_t,
) -> Option<Box<wasmtime_error_t>> {
    handle_result(module.module.serialize(), |buf| ret.set_buffer(buf))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_module_image_range(
    module: &wasmtime_module_t,
    start: &mut *const u8,
    end: &mut *const u8,
) {
    let range = module.module.image_range();
    *start = range.start;
    *end = range.end;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_module_deserialize(
    engine: &wasm_engine_t,
    bytes: *const u8,
    len: usize,
    out: &mut *mut wasmtime_module_t,
) -> Option<Box<wasmtime_error_t>> {
    let bytes = crate::slice_from_raw_parts(bytes, len);
    handle_result(Module::deserialize(&engine.engine, bytes), |module| {
        *out = Box::into_raw(Box::new(wasmtime_module_t { module }));
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_module_deserialize_file(
    engine: &wasm_engine_t,
    path: *const c_char,
    out: &mut *mut wasmtime_module_t,
) -> Option<Box<wasmtime_error_t>> {
    let path = CStr::from_ptr(path);
    let result = path
        .to_str()
        .context("input path is not valid utf-8")
        .and_then(|path| Module::deserialize_file(&engine.engine, path));
    handle_result(result, |module| {
        *out = Box::into_raw(Box::new(wasmtime_module_t { module }));
    })
}
