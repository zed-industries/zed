use std::ffi::c_void;

use wasmtime::component::{Instance, Linker, LinkerInstance, Val};

use crate::{
    WasmtimeStoreContextMut, WasmtimeStoreData, wasm_engine_t, wasmtime_error_t, wasmtime_module_t,
};

use super::{wasmtime_component_t, wasmtime_component_val_t};

#[repr(transparent)]
pub struct wasmtime_component_linker_t {
    pub(crate) linker: Linker<WasmtimeStoreData>,
}

#[repr(transparent)]
pub struct wasmtime_component_linker_instance_t<'a> {
    pub(crate) linker_instance: LinkerInstance<'a, WasmtimeStoreData>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_linker_new(
    engine: &wasm_engine_t,
) -> Box<wasmtime_component_linker_t> {
    Box::new(wasmtime_component_linker_t {
        linker: Linker::new(&engine.engine),
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_linker_root(
    linker: &mut wasmtime_component_linker_t,
) -> Box<wasmtime_component_linker_instance_t<'_>> {
    Box::new(wasmtime_component_linker_instance_t {
        linker_instance: linker.linker.root(),
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_linker_instantiate(
    linker: &wasmtime_component_linker_t,
    context: WasmtimeStoreContextMut<'_>,
    component: &wasmtime_component_t,
    instance_out: &mut Instance,
) -> Option<Box<wasmtime_error_t>> {
    let result = linker.linker.instantiate(context, &component.component);
    crate::handle_result(result, |instance| *instance_out = instance)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_linker_delete(
    _linker: Box<wasmtime_component_linker_t>,
) {
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_linker_instance_add_instance<'a>(
    linker_instance: &'a mut wasmtime_component_linker_instance_t<'a>,
    name: *const u8,
    name_len: usize,
    linker_instance_out: &mut *mut wasmtime_component_linker_instance_t<'a>,
) -> Option<Box<wasmtime_error_t>> {
    let name = unsafe { std::slice::from_raw_parts(name, name_len) };
    let Ok(name) = std::str::from_utf8(name) else {
        return crate::bad_utf8();
    };

    let result = linker_instance.linker_instance.instance(&name);
    crate::handle_result(result, |linker_instance| {
        *linker_instance_out = Box::into_raw(Box::new(wasmtime_component_linker_instance_t {
            linker_instance,
        }));
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_linker_instance_add_module(
    linker_instance: &mut wasmtime_component_linker_instance_t,
    name: *const u8,
    name_len: usize,
    module: &wasmtime_module_t,
) -> Option<Box<wasmtime_error_t>> {
    let name = unsafe { std::slice::from_raw_parts(name, name_len) };
    let Ok(name) = std::str::from_utf8(name) else {
        return crate::bad_utf8();
    };

    let result = linker_instance
        .linker_instance
        .module(&name, &module.module);

    crate::handle_result(result, |_| ())
}

pub type wasmtime_component_func_callback_t = extern "C" fn(
    *mut c_void,
    WasmtimeStoreContextMut<'_>,
    *const wasmtime_component_val_t,
    usize,
    *mut wasmtime_component_val_t,
    usize,
) -> Option<Box<wasmtime_error_t>>;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_linker_instance_add_func(
    linker_instance: &mut wasmtime_component_linker_instance_t,
    name: *const u8,
    name_len: usize,
    callback: wasmtime_component_func_callback_t,
    data: *mut c_void,
    finalizer: Option<extern "C" fn(*mut c_void)>,
) -> Option<Box<wasmtime_error_t>> {
    let name = unsafe { std::slice::from_raw_parts(name, name_len) };
    let Ok(name) = std::str::from_utf8(name) else {
        return crate::bad_utf8();
    };

    let foreign = crate::ForeignData { data, finalizer };

    let result = linker_instance
        .linker_instance
        .func_new(&name, move |ctx, args, rets| {
            let _ = &foreign;

            let args = args
                .iter()
                .map(|x| wasmtime_component_val_t::from(x))
                .collect::<Vec<_>>();

            let mut c_rets = vec![wasmtime_component_val_t::Bool(false); rets.len()];

            let res = callback(
                foreign.data,
                ctx,
                args.as_ptr(),
                args.len(),
                c_rets.as_mut_ptr(),
                c_rets.len(),
            );

            if let Some(res) = res {
                return Err((*res).into());
            }

            for (rust_val, c_val) in std::iter::zip(rets, c_rets) {
                *rust_val = Val::from(&c_val);
            }

            Ok(())
        });

    crate::handle_result(result, |_| ())
}

#[unsafe(no_mangle)]
#[cfg(feature = "wasi")]
pub unsafe extern "C" fn wasmtime_component_linker_add_wasip2(
    linker: &mut wasmtime_component_linker_t,
) -> Option<Box<wasmtime_error_t>> {
    let result = wasmtime_wasi::p2::add_to_linker_sync(&mut linker.linker);
    crate::handle_result(result, |_| ())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_linker_instance_delete(
    _linker_instance: Box<wasmtime_component_linker_instance_t>,
) {
}
