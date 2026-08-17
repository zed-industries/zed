use wasmtime::component::{Func, Val};

use crate::{WasmtimeStoreContextMut, wasmtime_error_t};

use super::wasmtime_component_val_t;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_func_call(
    func: &Func,
    mut context: WasmtimeStoreContextMut<'_>,
    args: *const wasmtime_component_val_t,
    args_len: usize,
    results: *mut wasmtime_component_val_t,
    results_len: usize,
) -> Option<Box<wasmtime_error_t>> {
    let c_args = unsafe { std::slice::from_raw_parts(args, args_len) };
    let c_results = unsafe { std::slice::from_raw_parts_mut(results, results_len) };

    let args = c_args.iter().map(Val::from).collect::<Vec<_>>();
    let mut results = vec![Val::Bool(false); results_len];

    let result = func.call(&mut context, &args, &mut results);

    crate::handle_result(result, |_| {
        for (c_val, rust_val) in std::iter::zip(c_results, results) {
            *c_val = wasmtime_component_val_t::from(&rust_val);
        }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_func_post_return(
    func: &Func,
    mut context: WasmtimeStoreContextMut<'_>,
) -> Option<Box<wasmtime_error_t>> {
    let result = func.post_return(&mut context);

    crate::handle_result(result, |_| {})
}
