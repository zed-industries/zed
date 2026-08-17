use std::ffi::{CStr, c_char};

use anyhow::Context;
use wasmtime::component::{Component, ComponentExportIndex};

use crate::{wasm_byte_vec_t, wasm_config_t, wasm_engine_t, wasmtime_error_t};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_config_wasm_component_model_set(
    c: &mut wasm_config_t,
    enable: bool,
) {
    c.config.wasm_component_model(enable);
}

#[derive(Clone)]
#[repr(transparent)]
pub struct wasmtime_component_t {
    pub(crate) component: Component,
}

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub unsafe extern "C" fn wasmtime_component_new(
    engine: &wasm_engine_t,
    buf: *const u8,
    len: usize,
    component_out: &mut *mut wasmtime_component_t,
) -> Option<Box<wasmtime_error_t>> {
    let bytes = unsafe { crate::slice_from_raw_parts(buf, len) };
    crate::handle_result(Component::new(&engine.engine, bytes), |component| {
        *component_out = Box::into_raw(Box::new(wasmtime_component_t { component }));
    })
}

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub unsafe extern "C" fn wasmtime_component_serialize(
    component: &wasmtime_component_t,
    ret: &mut wasm_byte_vec_t,
) -> Option<Box<wasmtime_error_t>> {
    crate::handle_result(component.component.serialize(), |buffer| {
        ret.set_buffer(buffer);
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_deserialize(
    engine: &wasm_engine_t,
    buf: *const u8,
    len: usize,
    component_out: &mut *mut wasmtime_component_t,
) -> Option<Box<wasmtime_error_t>> {
    let binary = unsafe { crate::slice_from_raw_parts(buf, len) };
    crate::handle_result(
        unsafe { Component::deserialize(&engine.engine, binary) },
        |component| {
            *component_out = Box::into_raw(Box::new(wasmtime_component_t { component }));
        },
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_deserialize_file(
    engine: &wasm_engine_t,
    path: *const c_char,
    component_out: &mut *mut wasmtime_component_t,
) -> Option<Box<wasmtime_error_t>> {
    let path = unsafe { CStr::from_ptr(path) };
    let result = path
        .to_str()
        .context("input path is not valid utf-8")
        .and_then(|path| unsafe { Component::deserialize_file(&engine.engine, path) });
    crate::handle_result(result, |component| {
        *component_out = Box::into_raw(Box::new(wasmtime_component_t { component }));
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_clone(
    component: &wasmtime_component_t,
) -> Box<wasmtime_component_t> {
    Box::new(component.clone())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_delete(_component: Box<wasmtime_component_t>) {}

#[repr(transparent)]
pub struct wasmtime_component_export_index_t {
    pub(crate) export_index: ComponentExportIndex,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_get_export_index(
    component: &wasmtime_component_t,
    instance_export_index: *const wasmtime_component_export_index_t,
    name: *const u8,
    name_len: usize,
) -> Option<Box<wasmtime_component_export_index_t>> {
    let name = unsafe { std::slice::from_raw_parts(name, name_len) };
    let Ok(name) = std::str::from_utf8(name) else {
        return None;
    };

    let instance_export_index = if instance_export_index.is_null() {
        None
    } else {
        Some((*instance_export_index).export_index)
    };

    component
        .component
        .get_export_index(instance_export_index.as_ref(), &name)
        .map(|export_index| Box::new(wasmtime_component_export_index_t { export_index }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_export_index_delete(
    _export_index: Box<wasmtime_component_export_index_t>,
) {
}
