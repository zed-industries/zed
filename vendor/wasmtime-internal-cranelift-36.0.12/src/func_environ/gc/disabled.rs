//! `GcCompiler` implementation when GC support is disabled.

use super::GcCompiler;
use crate::func_environ::{Extension, FuncEnvironment};
use cranelift_codegen::ir;
use cranelift_frontend::FunctionBuilder;
use wasmtime_environ::{TypeIndex, WasmRefType, WasmResult, wasm_unsupported};

fn disabled<T>() -> WasmResult<T> {
    Err(wasm_unsupported!(
        "support for Wasm GC disabled at compile time because the `gc` cargo \
         feature was not enabled"
    ))
}

/// Get the default GC compiler.
pub fn gc_compiler(_: &FuncEnvironment<'_>) -> WasmResult<Box<dyn GcCompiler>> {
    disabled()
}

pub fn translate_struct_new(
    _func_env: &mut FuncEnvironment<'_>,
    _builder: &mut FunctionBuilder<'_>,
    _struct_type_index: TypeIndex,
    _fields: &[ir::Value],
) -> WasmResult<ir::Value> {
    disabled()
}

pub fn translate_struct_new_default(
    _func_env: &mut FuncEnvironment<'_>,
    _builder: &mut FunctionBuilder<'_>,
    _struct_type_index: TypeIndex,
) -> WasmResult<ir::Value> {
    disabled()
}

pub fn translate_struct_get(
    _func_env: &mut FuncEnvironment<'_>,
    _builder: &mut FunctionBuilder<'_>,
    _struct_type_index: TypeIndex,
    _field_index: u32,
    _struct_ref: ir::Value,
    _extension: Option<Extension>,
) -> WasmResult<ir::Value> {
    disabled()
}

pub fn translate_struct_set(
    _func_env: &mut FuncEnvironment<'_>,
    _builder: &mut FunctionBuilder<'_>,
    _struct_type_index: TypeIndex,
    _field_index: u32,
    _struct_ref: ir::Value,
    _new_val: ir::Value,
) -> WasmResult<()> {
    disabled()
}

pub fn translate_array_new(
    _func_env: &mut FuncEnvironment<'_>,
    _builder: &mut FunctionBuilder,
    _array_type_index: TypeIndex,
    _elem: ir::Value,
    _len: ir::Value,
) -> WasmResult<ir::Value> {
    disabled()
}

pub fn translate_array_new_default(
    _func_env: &mut FuncEnvironment<'_>,
    _builder: &mut FunctionBuilder,
    _array_type_index: TypeIndex,
    _len: ir::Value,
) -> WasmResult<ir::Value> {
    disabled()
}

pub fn translate_array_new_fixed(
    _func_env: &mut FuncEnvironment<'_>,
    _builder: &mut FunctionBuilder,
    _array_type_index: TypeIndex,
    _elems: &[ir::Value],
) -> WasmResult<ir::Value> {
    disabled()
}

pub fn translate_array_fill(
    _func_env: &mut FuncEnvironment<'_>,
    _builder: &mut FunctionBuilder<'_>,
    _array_type_index: TypeIndex,
    _array_ref: ir::Value,
    _index: ir::Value,
    _value: ir::Value,
    _n: ir::Value,
) -> WasmResult<()> {
    disabled()
}

pub fn translate_array_len(
    _func_env: &mut FuncEnvironment<'_>,
    _builder: &mut FunctionBuilder,
    _array: ir::Value,
) -> WasmResult<ir::Value> {
    disabled()
}

pub fn translate_array_get(
    _func_env: &mut FuncEnvironment<'_>,
    _builder: &mut FunctionBuilder,
    _array_type_index: TypeIndex,
    _array: ir::Value,
    _index: ir::Value,
    _extension: Option<Extension>,
) -> WasmResult<ir::Value> {
    disabled()
}

pub fn translate_array_set(
    _func_env: &mut FuncEnvironment<'_>,
    _builder: &mut FunctionBuilder,
    _array_type_index: TypeIndex,
    _array: ir::Value,
    _index: ir::Value,
    _value: ir::Value,
) -> WasmResult<()> {
    disabled()
}

pub fn translate_ref_test(
    _func_env: &mut FuncEnvironment<'_>,
    _builder: &mut FunctionBuilder<'_>,
    _test_ty: WasmRefType,
    _val: ir::Value,
    _val_ty: WasmRefType,
) -> WasmResult<ir::Value> {
    disabled()
}
