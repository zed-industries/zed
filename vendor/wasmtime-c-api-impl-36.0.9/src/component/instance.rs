use wasmtime::component::{Func, Instance};

use crate::WasmtimeStoreContextMut;

use super::wasmtime_component_export_index_t;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_instance_get_export_index(
    instance: &Instance,
    context: WasmtimeStoreContextMut<'_>,
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

    instance
        .get_export_index(context, instance_export_index.as_ref(), &name)
        .map(|export_index| Box::new(wasmtime_component_export_index_t { export_index }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_instance_get_func(
    instance: &Instance,
    context: WasmtimeStoreContextMut<'_>,
    export_index: &wasmtime_component_export_index_t,
    func_out: &mut Func,
) -> bool {
    if let Some(func) = instance.get_func(context, export_index.export_index) {
        *func_out = func;
        true
    } else {
        false
    }
}
