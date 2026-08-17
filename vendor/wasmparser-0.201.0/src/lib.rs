/* Copyright 2017 Mozilla Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! A simple event-driven library for parsing WebAssembly binary files
//! (or streams).
//!
//! The parser library reports events as they happen and only stores
//! parsing information for a brief period of time, making it very fast
//! and memory-efficient. The event-driven model, however, has some drawbacks.
//! If you need random access to the entire WebAssembly data-structure,
//! this is not the right library for you. You could however, build such
//! a data-structure using this library.
//!
//! To get started, create a [`Parser`] using [`Parser::new`] and then follow
//! the examples documented for [`Parser::parse`] or [`Parser::parse_all`].

#![deny(missing_docs)]

/// A helper macro to conveniently iterate over all opcodes recognized by this
/// crate. This can be used to work with either the [`Operator`] enumeration or
/// the [`VisitOperator`] trait if your use case uniformly handles all operators
/// the same way.
///
/// It is also possible to specialize handling of operators depending on the
/// Wasm proposal from which they are originating.
///
/// This is an "iterator macro" where this macro is invoked with the name of
/// another macro, and then that macro is invoked with the list of all
/// operators. An example invocation of this looks like:
///
/// The list of specializable Wasm proposals is as follows:
///
/// - `@mvp`: Denoting a Wasm operator from the initial Wasm MVP version.
/// - `@exceptions`: [Wasm `expection-handling` proposal]
/// - `@tail_call`: [Wasm `tail-calls` proposal]
/// - `@reference_types`: [Wasm `reference-types` proposal]
/// - `@sign_extension`: [Wasm `sign-extension-ops` proposal]
/// - `@saturating_float_to_int`: [Wasm `non_trapping_float-to-int-conversions` proposal]
/// - `@bulk_memory `:[Wasm `bulk-memory` proposal]
/// - `@threads`: [Wasm `threads` proposal]
/// - `@simd`: [Wasm `simd` proposal]
/// - `@relaxed_simd`: [Wasm `relaxed-simd` proposal]
/// - `@gc`: [Wasm `gc` proposal]
///
/// [Wasm `expection-handling` proposal]:
/// https://github.com/WebAssembly/exception-handling
///
/// [Wasm `tail-calls` proposal]:
/// https://github.com/WebAssembly/tail-call
///
/// [Wasm `reference-types` proposal]:
/// https://github.com/WebAssembly/reference-types
///
/// [Wasm `sign-extension-ops` proposal]:
/// https://github.com/WebAssembly/sign-extension-ops
///
/// [Wasm `non_trapping_float-to-int-conversions` proposal]:
/// https://github.com/WebAssembly/nontrapping-float-to-int-conversions
///
/// [Wasm `bulk-memory` proposal]:
/// https://github.com/WebAssembly/bulk-memory-operations
///
/// [Wasm `threads` proposal]:
/// https://github.com/webassembly/threads
///
/// [Wasm `simd` proposal]:
/// https://github.com/webassembly/simd
///
/// [Wasm `relaxed-simd` proposal]:
/// https://github.com/WebAssembly/relaxed-simd
///
/// [Wasm `gc` proposal]:
/// https://github.com/WebAssembly/gc
///
/// ```
/// macro_rules! define_visit_operator {
///     // The outer layer of repetition represents how all operators are
///     // provided to the macro at the same time.
///     //
///     // The `$proposal` identifier indicates the Wasm proposals from which
///     // the Wasm operator is originating.
///     // For example to specialize the macro match arm for Wasm SIMD proposal
///     // operators you could write `@simd` instead of `@$proposal:ident` to
///     // only catch those operators.
///     //
///     // The `$op` name is bound to the `Operator` variant name. The
///     // payload of the operator is optionally specified (the `$(...)?`
///     // clause) since not all instructions have payloads. Within the payload
///     // each argument is named and has its type specified.
///     //
///     // The `$visit` name is bound to the corresponding name in the
///     // `VisitOperator` trait that this corresponds to.
///     ($( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident)*) => {
///         $(
///             fn $visit(&mut self $($(,$arg: $argty)*)?) {
///                 // do nothing for this example
///             }
///         )*
///     }
/// }
///
/// pub struct VisitAndDoNothing;
///
/// impl<'a> wasmparser::VisitOperator<'a> for VisitAndDoNothing {
///     type Output = ();
///
///     wasmparser::for_each_operator!(define_visit_operator);
/// }
/// ```
#[macro_export]
macro_rules! for_each_operator {
    ($mac:ident) => {
        $mac! {
            @mvp Unreachable => visit_unreachable
            @mvp Nop => visit_nop
            @mvp Block { blockty: $crate::BlockType } => visit_block
            @mvp Loop { blockty: $crate::BlockType } => visit_loop
            @mvp If { blockty: $crate::BlockType } => visit_if
            @mvp Else => visit_else
            @exceptions TryTable { try_table: $crate::TryTable } => visit_try_table
            @exceptions Throw { tag_index: u32 } => visit_throw
            @exceptions ThrowRef => visit_throw_ref
            // Deprecated old instructions from the exceptions proposal
            @exceptions Try { blockty: $crate::BlockType } => visit_try
            @exceptions Catch { tag_index: u32 } => visit_catch
            @exceptions Rethrow { relative_depth: u32 } => visit_rethrow
            @exceptions Delegate { relative_depth: u32 } => visit_delegate
            @exceptions CatchAll => visit_catch_all
            @mvp End => visit_end
            @mvp Br { relative_depth: u32 } => visit_br
            @mvp BrIf { relative_depth: u32 } => visit_br_if
            @mvp BrTable { targets: $crate::BrTable<'a> } => visit_br_table
            @mvp Return => visit_return
            @mvp Call { function_index: u32 } => visit_call
            @mvp CallIndirect { type_index: u32, table_index: u32, table_byte: u8 } => visit_call_indirect
            @tail_call ReturnCall { function_index: u32 } => visit_return_call
            @tail_call ReturnCallIndirect { type_index: u32, table_index: u32 } => visit_return_call_indirect
            @mvp Drop => visit_drop
            @mvp Select => visit_select
            @reference_types TypedSelect { ty: $crate::ValType } => visit_typed_select
            @mvp LocalGet { local_index: u32 } => visit_local_get
            @mvp LocalSet { local_index: u32 } => visit_local_set
            @mvp LocalTee { local_index: u32 } => visit_local_tee
            @mvp GlobalGet { global_index: u32 } => visit_global_get
            @mvp GlobalSet { global_index: u32 } => visit_global_set
            @mvp I32Load { memarg: $crate::MemArg } => visit_i32_load
            @mvp I64Load { memarg: $crate::MemArg } => visit_i64_load
            @mvp F32Load { memarg: $crate::MemArg } => visit_f32_load
            @mvp F64Load { memarg: $crate::MemArg } => visit_f64_load
            @mvp I32Load8S { memarg: $crate::MemArg } => visit_i32_load8_s
            @mvp I32Load8U { memarg: $crate::MemArg } => visit_i32_load8_u
            @mvp I32Load16S { memarg: $crate::MemArg } => visit_i32_load16_s
            @mvp I32Load16U { memarg: $crate::MemArg } => visit_i32_load16_u
            @mvp I64Load8S { memarg: $crate::MemArg } => visit_i64_load8_s
            @mvp I64Load8U { memarg: $crate::MemArg } => visit_i64_load8_u
            @mvp I64Load16S { memarg: $crate::MemArg } => visit_i64_load16_s
            @mvp I64Load16U { memarg: $crate::MemArg } => visit_i64_load16_u
            @mvp I64Load32S { memarg: $crate::MemArg } => visit_i64_load32_s
            @mvp I64Load32U { memarg: $crate::MemArg } => visit_i64_load32_u
            @mvp I32Store { memarg: $crate::MemArg } => visit_i32_store
            @mvp I64Store { memarg: $crate::MemArg } => visit_i64_store
            @mvp F32Store { memarg: $crate::MemArg } => visit_f32_store
            @mvp F64Store { memarg: $crate::MemArg } => visit_f64_store
            @mvp I32Store8 { memarg: $crate::MemArg } => visit_i32_store8
            @mvp I32Store16 { memarg: $crate::MemArg } => visit_i32_store16
            @mvp I64Store8 { memarg: $crate::MemArg } => visit_i64_store8
            @mvp I64Store16 { memarg: $crate::MemArg } => visit_i64_store16
            @mvp I64Store32 { memarg: $crate::MemArg } => visit_i64_store32
            @mvp MemorySize { mem: u32, mem_byte: u8 } => visit_memory_size
            @mvp MemoryGrow { mem: u32, mem_byte: u8 } => visit_memory_grow
            @mvp I32Const { value: i32 } => visit_i32_const
            @mvp I64Const { value: i64 } => visit_i64_const
            @mvp F32Const { value: $crate::Ieee32 } => visit_f32_const
            @mvp F64Const { value: $crate::Ieee64 } => visit_f64_const
            @reference_types RefNull { hty: $crate::HeapType } => visit_ref_null
            @reference_types RefIsNull => visit_ref_is_null
            @reference_types RefFunc { function_index: u32 } => visit_ref_func
            @gc RefEq => visit_ref_eq
            @mvp I32Eqz => visit_i32_eqz
            @mvp I32Eq => visit_i32_eq
            @mvp I32Ne => visit_i32_ne
            @mvp I32LtS => visit_i32_lt_s
            @mvp I32LtU => visit_i32_lt_u
            @mvp I32GtS => visit_i32_gt_s
            @mvp I32GtU => visit_i32_gt_u
            @mvp I32LeS => visit_i32_le_s
            @mvp I32LeU => visit_i32_le_u
            @mvp I32GeS => visit_i32_ge_s
            @mvp I32GeU => visit_i32_ge_u
            @mvp I64Eqz => visit_i64_eqz
            @mvp I64Eq => visit_i64_eq
            @mvp I64Ne => visit_i64_ne
            @mvp I64LtS => visit_i64_lt_s
            @mvp I64LtU => visit_i64_lt_u
            @mvp I64GtS => visit_i64_gt_s
            @mvp I64GtU => visit_i64_gt_u
            @mvp I64LeS => visit_i64_le_s
            @mvp I64LeU => visit_i64_le_u
            @mvp I64GeS => visit_i64_ge_s
            @mvp I64GeU => visit_i64_ge_u
            @mvp F32Eq => visit_f32_eq
            @mvp F32Ne => visit_f32_ne
            @mvp F32Lt => visit_f32_lt
            @mvp F32Gt => visit_f32_gt
            @mvp F32Le => visit_f32_le
            @mvp F32Ge => visit_f32_ge
            @mvp F64Eq => visit_f64_eq
            @mvp F64Ne => visit_f64_ne
            @mvp F64Lt => visit_f64_lt
            @mvp F64Gt => visit_f64_gt
            @mvp F64Le => visit_f64_le
            @mvp F64Ge => visit_f64_ge
            @mvp I32Clz => visit_i32_clz
            @mvp I32Ctz => visit_i32_ctz
            @mvp I32Popcnt => visit_i32_popcnt
            @mvp I32Add => visit_i32_add
            @mvp I32Sub => visit_i32_sub
            @mvp I32Mul => visit_i32_mul
            @mvp I32DivS => visit_i32_div_s
            @mvp I32DivU => visit_i32_div_u
            @mvp I32RemS => visit_i32_rem_s
            @mvp I32RemU => visit_i32_rem_u
            @mvp I32And => visit_i32_and
            @mvp I32Or => visit_i32_or
            @mvp I32Xor => visit_i32_xor
            @mvp I32Shl => visit_i32_shl
            @mvp I32ShrS => visit_i32_shr_s
            @mvp I32ShrU => visit_i32_shr_u
            @mvp I32Rotl => visit_i32_rotl
            @mvp I32Rotr => visit_i32_rotr
            @mvp I64Clz => visit_i64_clz
            @mvp I64Ctz => visit_i64_ctz
            @mvp I64Popcnt => visit_i64_popcnt
            @mvp I64Add => visit_i64_add
            @mvp I64Sub => visit_i64_sub
            @mvp I64Mul => visit_i64_mul
            @mvp I64DivS => visit_i64_div_s
            @mvp I64DivU => visit_i64_div_u
            @mvp I64RemS => visit_i64_rem_s
            @mvp I64RemU => visit_i64_rem_u
            @mvp I64And => visit_i64_and
            @mvp I64Or => visit_i64_or
            @mvp I64Xor => visit_i64_xor
            @mvp I64Shl => visit_i64_shl
            @mvp I64ShrS => visit_i64_shr_s
            @mvp I64ShrU => visit_i64_shr_u
            @mvp I64Rotl => visit_i64_rotl
            @mvp I64Rotr => visit_i64_rotr
            @mvp F32Abs => visit_f32_abs
            @mvp F32Neg => visit_f32_neg
            @mvp F32Ceil => visit_f32_ceil
            @mvp F32Floor => visit_f32_floor
            @mvp F32Trunc => visit_f32_trunc
            @mvp F32Nearest => visit_f32_nearest
            @mvp F32Sqrt => visit_f32_sqrt
            @mvp F32Add => visit_f32_add
            @mvp F32Sub => visit_f32_sub
            @mvp F32Mul => visit_f32_mul
            @mvp F32Div => visit_f32_div
            @mvp F32Min => visit_f32_min
            @mvp F32Max => visit_f32_max
            @mvp F32Copysign => visit_f32_copysign
            @mvp F64Abs => visit_f64_abs
            @mvp F64Neg => visit_f64_neg
            @mvp F64Ceil => visit_f64_ceil
            @mvp F64Floor => visit_f64_floor
            @mvp F64Trunc => visit_f64_trunc
            @mvp F64Nearest => visit_f64_nearest
            @mvp F64Sqrt => visit_f64_sqrt
            @mvp F64Add => visit_f64_add
            @mvp F64Sub => visit_f64_sub
            @mvp F64Mul => visit_f64_mul
            @mvp F64Div => visit_f64_div
            @mvp F64Min => visit_f64_min
            @mvp F64Max => visit_f64_max
            @mvp F64Copysign => visit_f64_copysign
            @mvp I32WrapI64 => visit_i32_wrap_i64
            @mvp I32TruncF32S => visit_i32_trunc_f32_s
            @mvp I32TruncF32U => visit_i32_trunc_f32_u
            @mvp I32TruncF64S => visit_i32_trunc_f64_s
            @mvp I32TruncF64U => visit_i32_trunc_f64_u
            @mvp I64ExtendI32S => visit_i64_extend_i32_s
            @mvp I64ExtendI32U => visit_i64_extend_i32_u
            @mvp I64TruncF32S => visit_i64_trunc_f32_s
            @mvp I64TruncF32U => visit_i64_trunc_f32_u
            @mvp I64TruncF64S => visit_i64_trunc_f64_s
            @mvp I64TruncF64U => visit_i64_trunc_f64_u
            @mvp F32ConvertI32S => visit_f32_convert_i32_s
            @mvp F32ConvertI32U => visit_f32_convert_i32_u
            @mvp F32ConvertI64S => visit_f32_convert_i64_s
            @mvp F32ConvertI64U => visit_f32_convert_i64_u
            @mvp F32DemoteF64 => visit_f32_demote_f64
            @mvp F64ConvertI32S => visit_f64_convert_i32_s
            @mvp F64ConvertI32U => visit_f64_convert_i32_u
            @mvp F64ConvertI64S => visit_f64_convert_i64_s
            @mvp F64ConvertI64U => visit_f64_convert_i64_u
            @mvp F64PromoteF32 => visit_f64_promote_f32
            @mvp I32ReinterpretF32 => visit_i32_reinterpret_f32
            @mvp I64ReinterpretF64 => visit_i64_reinterpret_f64
            @mvp F32ReinterpretI32 => visit_f32_reinterpret_i32
            @mvp F64ReinterpretI64 => visit_f64_reinterpret_i64
            @sign_extension I32Extend8S => visit_i32_extend8_s
            @sign_extension I32Extend16S => visit_i32_extend16_s
            @sign_extension I64Extend8S => visit_i64_extend8_s
            @sign_extension I64Extend16S => visit_i64_extend16_s
            @sign_extension I64Extend32S => visit_i64_extend32_s

            // 0xFB prefixed operators
            // Garbage Collection
            // http://github.com/WebAssembly/gc
            @gc StructNew { struct_type_index: u32 } => visit_struct_new
            @gc StructNewDefault { struct_type_index: u32 } => visit_struct_new_default
            @gc StructGet { struct_type_index: u32, field_index: u32 } => visit_struct_get
            @gc StructGetS { struct_type_index: u32, field_index: u32 } => visit_struct_get_s
            @gc StructGetU { struct_type_index: u32, field_index: u32 } => visit_struct_get_u
                @gc StructSet { struct_type_index: u32, field_index: u32 } => visit_struct_set
            @gc ArrayNew { array_type_index: u32 } => visit_array_new
            @gc ArrayNewDefault { array_type_index: u32 } => visit_array_new_default
            @gc ArrayNewFixed { array_type_index: u32, array_size: u32 } => visit_array_new_fixed
            @gc ArrayNewData { array_type_index: u32, array_data_index: u32 } => visit_array_new_data
            @gc ArrayNewElem { array_type_index: u32, array_elem_index: u32 } => visit_array_new_elem
            @gc ArrayGet { array_type_index: u32 } => visit_array_get
            @gc ArrayGetS { array_type_index: u32 } => visit_array_get_s
            @gc ArrayGetU { array_type_index: u32 } => visit_array_get_u
            @gc ArraySet { array_type_index: u32 } => visit_array_set
            @gc ArrayLen => visit_array_len
            @gc ArrayFill { array_type_index: u32 } => visit_array_fill
            @gc ArrayCopy { array_type_index_dst: u32, array_type_index_src: u32 } => visit_array_copy
            @gc ArrayInitData { array_type_index: u32, array_data_index: u32 } => visit_array_init_data
            @gc ArrayInitElem { array_type_index: u32, array_elem_index: u32 } => visit_array_init_elem
            @gc RefTestNonNull { hty: $crate::HeapType } => visit_ref_test_non_null
            @gc RefTestNullable { hty: $crate::HeapType } => visit_ref_test_nullable
            @gc RefCastNonNull { hty: $crate::HeapType } => visit_ref_cast_non_null
            @gc RefCastNullable { hty: $crate::HeapType } => visit_ref_cast_nullable
            @gc BrOnCast {
                relative_depth: u32,
                from_ref_type: $crate::RefType,
                to_ref_type: $crate::RefType
            } => visit_br_on_cast
            @gc BrOnCastFail {
                relative_depth: u32,
                from_ref_type: $crate::RefType,
                to_ref_type: $crate::RefType
            } => visit_br_on_cast_fail
            @gc AnyConvertExtern => visit_any_convert_extern
            @gc ExternConvertAny => visit_extern_convert_any
            @gc RefI31 => visit_ref_i31
            @gc I31GetS => visit_i31_get_s
            @gc I31GetU => visit_i31_get_u

            // 0xFC operators
            // Non-trapping Float-to-int Conversions
            // https://github.com/WebAssembly/nontrapping-float-to-int-conversions
            @saturating_float_to_int I32TruncSatF32S => visit_i32_trunc_sat_f32_s
            @saturating_float_to_int I32TruncSatF32U => visit_i32_trunc_sat_f32_u
            @saturating_float_to_int I32TruncSatF64S => visit_i32_trunc_sat_f64_s
            @saturating_float_to_int I32TruncSatF64U => visit_i32_trunc_sat_f64_u
            @saturating_float_to_int I64TruncSatF32S => visit_i64_trunc_sat_f32_s
            @saturating_float_to_int I64TruncSatF32U => visit_i64_trunc_sat_f32_u
            @saturating_float_to_int I64TruncSatF64S => visit_i64_trunc_sat_f64_s
            @saturating_float_to_int I64TruncSatF64U => visit_i64_trunc_sat_f64_u

            // 0xFC prefixed operators
            // bulk memory operations
            // https://github.com/WebAssembly/bulk-memory-operations
            @bulk_memory MemoryInit { data_index: u32, mem: u32 } => visit_memory_init
            @bulk_memory DataDrop { data_index: u32 } => visit_data_drop
            @bulk_memory MemoryCopy { dst_mem: u32, src_mem: u32 } => visit_memory_copy
            @bulk_memory MemoryFill { mem: u32 } => visit_memory_fill
            @bulk_memory TableInit { elem_index: u32, table: u32 } => visit_table_init
            @bulk_memory ElemDrop { elem_index: u32 } => visit_elem_drop
            @bulk_memory TableCopy { dst_table: u32, src_table: u32 } => visit_table_copy

            // 0xFC prefixed operators
            // reference-types
            // https://github.com/WebAssembly/reference-types
            @reference_types TableFill { table: u32 } => visit_table_fill
            @reference_types TableGet { table: u32 } => visit_table_get
            @reference_types TableSet { table: u32 } => visit_table_set
            @reference_types TableGrow { table: u32 } => visit_table_grow
            @reference_types TableSize { table: u32 } => visit_table_size

            // OxFC prefixed operators
            // memory control (experimental)
            // https://github.com/WebAssembly/design/issues/1439
            @memory_control MemoryDiscard { mem: u32 } => visit_memory_discard

            // 0xFE prefixed operators
            // threads
            // https://github.com/WebAssembly/threads
            @threads MemoryAtomicNotify { memarg: $crate::MemArg } => visit_memory_atomic_notify
            @threads MemoryAtomicWait32 { memarg: $crate::MemArg } => visit_memory_atomic_wait32
            @threads MemoryAtomicWait64 { memarg: $crate::MemArg } => visit_memory_atomic_wait64
            @threads AtomicFence => visit_atomic_fence
            @threads I32AtomicLoad { memarg: $crate::MemArg } => visit_i32_atomic_load
            @threads I64AtomicLoad { memarg: $crate::MemArg } => visit_i64_atomic_load
            @threads I32AtomicLoad8U { memarg: $crate::MemArg } => visit_i32_atomic_load8_u
            @threads I32AtomicLoad16U { memarg: $crate::MemArg } => visit_i32_atomic_load16_u
            @threads I64AtomicLoad8U { memarg: $crate::MemArg } => visit_i64_atomic_load8_u
            @threads I64AtomicLoad16U { memarg: $crate::MemArg } => visit_i64_atomic_load16_u
            @threads I64AtomicLoad32U { memarg: $crate::MemArg } => visit_i64_atomic_load32_u
            @threads I32AtomicStore { memarg: $crate::MemArg } => visit_i32_atomic_store
            @threads I64AtomicStore { memarg: $crate::MemArg } => visit_i64_atomic_store
            @threads I32AtomicStore8 { memarg: $crate::MemArg } => visit_i32_atomic_store8
            @threads I32AtomicStore16 { memarg: $crate::MemArg } => visit_i32_atomic_store16
            @threads I64AtomicStore8 { memarg: $crate::MemArg } => visit_i64_atomic_store8
            @threads I64AtomicStore16 { memarg: $crate::MemArg } => visit_i64_atomic_store16
            @threads I64AtomicStore32 { memarg: $crate::MemArg } => visit_i64_atomic_store32
            @threads I32AtomicRmwAdd { memarg: $crate::MemArg } => visit_i32_atomic_rmw_add
            @threads I64AtomicRmwAdd { memarg: $crate::MemArg } => visit_i64_atomic_rmw_add
            @threads I32AtomicRmw8AddU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_add_u
            @threads I32AtomicRmw16AddU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_add_u
            @threads I64AtomicRmw8AddU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_add_u
            @threads I64AtomicRmw16AddU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_add_u
            @threads I64AtomicRmw32AddU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_add_u
            @threads I32AtomicRmwSub { memarg: $crate::MemArg } => visit_i32_atomic_rmw_sub
            @threads I64AtomicRmwSub { memarg: $crate::MemArg } => visit_i64_atomic_rmw_sub
            @threads I32AtomicRmw8SubU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_sub_u
            @threads I32AtomicRmw16SubU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_sub_u
            @threads I64AtomicRmw8SubU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_sub_u
            @threads I64AtomicRmw16SubU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_sub_u
            @threads I64AtomicRmw32SubU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_sub_u
            @threads I32AtomicRmwAnd { memarg: $crate::MemArg } => visit_i32_atomic_rmw_and
            @threads I64AtomicRmwAnd { memarg: $crate::MemArg } => visit_i64_atomic_rmw_and
            @threads I32AtomicRmw8AndU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_and_u
            @threads I32AtomicRmw16AndU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_and_u
            @threads I64AtomicRmw8AndU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_and_u
            @threads I64AtomicRmw16AndU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_and_u
            @threads I64AtomicRmw32AndU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_and_u
            @threads I32AtomicRmwOr { memarg: $crate::MemArg } => visit_i32_atomic_rmw_or
            @threads I64AtomicRmwOr { memarg: $crate::MemArg } => visit_i64_atomic_rmw_or
            @threads I32AtomicRmw8OrU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_or_u
            @threads I32AtomicRmw16OrU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_or_u
            @threads I64AtomicRmw8OrU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_or_u
            @threads I64AtomicRmw16OrU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_or_u
            @threads I64AtomicRmw32OrU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_or_u
            @threads I32AtomicRmwXor { memarg: $crate::MemArg } => visit_i32_atomic_rmw_xor
            @threads I64AtomicRmwXor { memarg: $crate::MemArg } => visit_i64_atomic_rmw_xor
            @threads I32AtomicRmw8XorU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_xor_u
            @threads I32AtomicRmw16XorU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_xor_u
            @threads I64AtomicRmw8XorU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_xor_u
            @threads I64AtomicRmw16XorU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_xor_u
            @threads I64AtomicRmw32XorU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_xor_u
            @threads I32AtomicRmwXchg { memarg: $crate::MemArg } => visit_i32_atomic_rmw_xchg
            @threads I64AtomicRmwXchg { memarg: $crate::MemArg } => visit_i64_atomic_rmw_xchg
            @threads I32AtomicRmw8XchgU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_xchg_u
            @threads I32AtomicRmw16XchgU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_xchg_u
            @threads I64AtomicRmw8XchgU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_xchg_u
            @threads I64AtomicRmw16XchgU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_xchg_u
            @threads I64AtomicRmw32XchgU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_xchg_u
            @threads I32AtomicRmwCmpxchg { memarg: $crate::MemArg } => visit_i32_atomic_rmw_cmpxchg
            @threads I64AtomicRmwCmpxchg { memarg: $crate::MemArg } => visit_i64_atomic_rmw_cmpxchg
            @threads I32AtomicRmw8CmpxchgU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_cmpxchg_u
            @threads I32AtomicRmw16CmpxchgU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_cmpxchg_u
            @threads I64AtomicRmw8CmpxchgU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_cmpxchg_u
            @threads I64AtomicRmw16CmpxchgU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_cmpxchg_u
            @threads I64AtomicRmw32CmpxchgU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_cmpxchg_u

            // 0xFD operators
            // 128-bit SIMD
            // - https://github.com/webassembly/simd
            // - https://webassembly.github.io/simd/core/binary/instructions.html
            @simd V128Load { memarg: $crate::MemArg } => visit_v128_load
            @simd V128Load8x8S { memarg: $crate::MemArg } => visit_v128_load8x8_s
            @simd V128Load8x8U { memarg: $crate::MemArg } => visit_v128_load8x8_u
            @simd V128Load16x4S { memarg: $crate::MemArg } => visit_v128_load16x4_s
            @simd V128Load16x4U { memarg: $crate::MemArg } => visit_v128_load16x4_u
            @simd V128Load32x2S { memarg: $crate::MemArg } => visit_v128_load32x2_s
            @simd V128Load32x2U { memarg: $crate::MemArg } => visit_v128_load32x2_u
            @simd V128Load8Splat { memarg: $crate::MemArg } => visit_v128_load8_splat
            @simd V128Load16Splat { memarg: $crate::MemArg } => visit_v128_load16_splat
            @simd V128Load32Splat { memarg: $crate::MemArg } => visit_v128_load32_splat
            @simd V128Load64Splat { memarg: $crate::MemArg } => visit_v128_load64_splat
            @simd V128Load32Zero { memarg: $crate::MemArg } => visit_v128_load32_zero
            @simd V128Load64Zero { memarg: $crate::MemArg } => visit_v128_load64_zero
            @simd V128Store { memarg: $crate::MemArg } => visit_v128_store
            @simd V128Load8Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_load8_lane
            @simd V128Load16Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_load16_lane
            @simd V128Load32Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_load32_lane
            @simd V128Load64Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_load64_lane
            @simd V128Store8Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_store8_lane
            @simd V128Store16Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_store16_lane
            @simd V128Store32Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_store32_lane
            @simd V128Store64Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_store64_lane
            @simd V128Const { value: $crate::V128 } => visit_v128_const
            @simd I8x16Shuffle { lanes: [u8; 16] } => visit_i8x16_shuffle
            @simd I8x16ExtractLaneS { lane: u8 } => visit_i8x16_extract_lane_s
            @simd I8x16ExtractLaneU { lane: u8 } => visit_i8x16_extract_lane_u
            @simd I8x16ReplaceLane { lane: u8 } => visit_i8x16_replace_lane
            @simd I16x8ExtractLaneS { lane: u8 } => visit_i16x8_extract_lane_s
            @simd I16x8ExtractLaneU { lane: u8 } => visit_i16x8_extract_lane_u
            @simd I16x8ReplaceLane { lane: u8 } => visit_i16x8_replace_lane
            @simd I32x4ExtractLane { lane: u8 } => visit_i32x4_extract_lane
            @simd I32x4ReplaceLane { lane: u8 } => visit_i32x4_replace_lane
            @simd I64x2ExtractLane { lane: u8 } => visit_i64x2_extract_lane
            @simd I64x2ReplaceLane { lane: u8 } => visit_i64x2_replace_lane
            @simd F32x4ExtractLane { lane: u8 } => visit_f32x4_extract_lane
            @simd F32x4ReplaceLane { lane: u8 } => visit_f32x4_replace_lane
            @simd F64x2ExtractLane { lane: u8 } => visit_f64x2_extract_lane
            @simd F64x2ReplaceLane { lane: u8 } => visit_f64x2_replace_lane
            @simd I8x16Swizzle => visit_i8x16_swizzle
            @simd I8x16Splat => visit_i8x16_splat
            @simd I16x8Splat => visit_i16x8_splat
            @simd I32x4Splat => visit_i32x4_splat
            @simd I64x2Splat => visit_i64x2_splat
            @simd F32x4Splat => visit_f32x4_splat
            @simd F64x2Splat => visit_f64x2_splat
            @simd I8x16Eq => visit_i8x16_eq
            @simd I8x16Ne => visit_i8x16_ne
            @simd I8x16LtS => visit_i8x16_lt_s
            @simd I8x16LtU => visit_i8x16_lt_u
            @simd I8x16GtS => visit_i8x16_gt_s
            @simd I8x16GtU => visit_i8x16_gt_u
            @simd I8x16LeS => visit_i8x16_le_s
            @simd I8x16LeU => visit_i8x16_le_u
            @simd I8x16GeS => visit_i8x16_ge_s
            @simd I8x16GeU => visit_i8x16_ge_u
            @simd I16x8Eq => visit_i16x8_eq
            @simd I16x8Ne => visit_i16x8_ne
            @simd I16x8LtS => visit_i16x8_lt_s
            @simd I16x8LtU => visit_i16x8_lt_u
            @simd I16x8GtS => visit_i16x8_gt_s
            @simd I16x8GtU => visit_i16x8_gt_u
            @simd I16x8LeS => visit_i16x8_le_s
            @simd I16x8LeU => visit_i16x8_le_u
            @simd I16x8GeS => visit_i16x8_ge_s
            @simd I16x8GeU => visit_i16x8_ge_u
            @simd I32x4Eq => visit_i32x4_eq
            @simd I32x4Ne => visit_i32x4_ne
            @simd I32x4LtS => visit_i32x4_lt_s
            @simd I32x4LtU => visit_i32x4_lt_u
            @simd I32x4GtS => visit_i32x4_gt_s
            @simd I32x4GtU => visit_i32x4_gt_u
            @simd I32x4LeS => visit_i32x4_le_s
            @simd I32x4LeU => visit_i32x4_le_u
            @simd I32x4GeS => visit_i32x4_ge_s
            @simd I32x4GeU => visit_i32x4_ge_u
            @simd I64x2Eq => visit_i64x2_eq
            @simd I64x2Ne => visit_i64x2_ne
            @simd I64x2LtS => visit_i64x2_lt_s
            @simd I64x2GtS => visit_i64x2_gt_s
            @simd I64x2LeS => visit_i64x2_le_s
            @simd I64x2GeS => visit_i64x2_ge_s
            @simd F32x4Eq => visit_f32x4_eq
            @simd F32x4Ne => visit_f32x4_ne
            @simd F32x4Lt => visit_f32x4_lt
            @simd F32x4Gt => visit_f32x4_gt
            @simd F32x4Le => visit_f32x4_le
            @simd F32x4Ge => visit_f32x4_ge
            @simd F64x2Eq => visit_f64x2_eq
            @simd F64x2Ne => visit_f64x2_ne
            @simd F64x2Lt => visit_f64x2_lt
            @simd F64x2Gt => visit_f64x2_gt
            @simd F64x2Le => visit_f64x2_le
            @simd F64x2Ge => visit_f64x2_ge
            @simd V128Not => visit_v128_not
            @simd V128And => visit_v128_and
            @simd V128AndNot => visit_v128_andnot
            @simd V128Or => visit_v128_or
            @simd V128Xor => visit_v128_xor
            @simd V128Bitselect => visit_v128_bitselect
            @simd V128AnyTrue => visit_v128_any_true
            @simd I8x16Abs => visit_i8x16_abs
            @simd I8x16Neg => visit_i8x16_neg
            @simd I8x16Popcnt => visit_i8x16_popcnt
            @simd I8x16AllTrue => visit_i8x16_all_true
            @simd I8x16Bitmask => visit_i8x16_bitmask
            @simd I8x16NarrowI16x8S => visit_i8x16_narrow_i16x8_s
            @simd I8x16NarrowI16x8U => visit_i8x16_narrow_i16x8_u
            @simd I8x16Shl => visit_i8x16_shl
            @simd I8x16ShrS => visit_i8x16_shr_s
            @simd I8x16ShrU => visit_i8x16_shr_u
            @simd I8x16Add => visit_i8x16_add
            @simd I8x16AddSatS => visit_i8x16_add_sat_s
            @simd I8x16AddSatU => visit_i8x16_add_sat_u
            @simd I8x16Sub => visit_i8x16_sub
            @simd I8x16SubSatS => visit_i8x16_sub_sat_s
            @simd I8x16SubSatU => visit_i8x16_sub_sat_u
            @simd I8x16MinS => visit_i8x16_min_s
            @simd I8x16MinU => visit_i8x16_min_u
            @simd I8x16MaxS => visit_i8x16_max_s
            @simd I8x16MaxU => visit_i8x16_max_u
            @simd I8x16AvgrU => visit_i8x16_avgr_u
            @simd I16x8ExtAddPairwiseI8x16S => visit_i16x8_extadd_pairwise_i8x16_s
            @simd I16x8ExtAddPairwiseI8x16U => visit_i16x8_extadd_pairwise_i8x16_u
            @simd I16x8Abs => visit_i16x8_abs
            @simd I16x8Neg => visit_i16x8_neg
            @simd I16x8Q15MulrSatS => visit_i16x8_q15mulr_sat_s
            @simd I16x8AllTrue => visit_i16x8_all_true
            @simd I16x8Bitmask => visit_i16x8_bitmask
            @simd I16x8NarrowI32x4S => visit_i16x8_narrow_i32x4_s
            @simd I16x8NarrowI32x4U => visit_i16x8_narrow_i32x4_u
            @simd I16x8ExtendLowI8x16S => visit_i16x8_extend_low_i8x16_s
            @simd I16x8ExtendHighI8x16S => visit_i16x8_extend_high_i8x16_s
            @simd I16x8ExtendLowI8x16U => visit_i16x8_extend_low_i8x16_u
            @simd I16x8ExtendHighI8x16U => visit_i16x8_extend_high_i8x16_u
            @simd I16x8Shl => visit_i16x8_shl
            @simd I16x8ShrS => visit_i16x8_shr_s
            @simd I16x8ShrU => visit_i16x8_shr_u
            @simd I16x8Add => visit_i16x8_add
            @simd I16x8AddSatS => visit_i16x8_add_sat_s
            @simd I16x8AddSatU => visit_i16x8_add_sat_u
            @simd I16x8Sub => visit_i16x8_sub
            @simd I16x8SubSatS => visit_i16x8_sub_sat_s
            @simd I16x8SubSatU => visit_i16x8_sub_sat_u
            @simd I16x8Mul => visit_i16x8_mul
            @simd I16x8MinS => visit_i16x8_min_s
            @simd I16x8MinU => visit_i16x8_min_u
            @simd I16x8MaxS => visit_i16x8_max_s
            @simd I16x8MaxU => visit_i16x8_max_u
            @simd I16x8AvgrU => visit_i16x8_avgr_u
            @simd I16x8ExtMulLowI8x16S => visit_i16x8_extmul_low_i8x16_s
            @simd I16x8ExtMulHighI8x16S => visit_i16x8_extmul_high_i8x16_s
            @simd I16x8ExtMulLowI8x16U => visit_i16x8_extmul_low_i8x16_u
            @simd I16x8ExtMulHighI8x16U => visit_i16x8_extmul_high_i8x16_u
            @simd I32x4ExtAddPairwiseI16x8S => visit_i32x4_extadd_pairwise_i16x8_s
            @simd I32x4ExtAddPairwiseI16x8U => visit_i32x4_extadd_pairwise_i16x8_u
            @simd I32x4Abs => visit_i32x4_abs
            @simd I32x4Neg => visit_i32x4_neg
            @simd I32x4AllTrue => visit_i32x4_all_true
            @simd I32x4Bitmask => visit_i32x4_bitmask
            @simd I32x4ExtendLowI16x8S => visit_i32x4_extend_low_i16x8_s
            @simd I32x4ExtendHighI16x8S => visit_i32x4_extend_high_i16x8_s
            @simd I32x4ExtendLowI16x8U => visit_i32x4_extend_low_i16x8_u
            @simd I32x4ExtendHighI16x8U => visit_i32x4_extend_high_i16x8_u
            @simd I32x4Shl => visit_i32x4_shl
            @simd I32x4ShrS => visit_i32x4_shr_s
            @simd I32x4ShrU => visit_i32x4_shr_u
            @simd I32x4Add => visit_i32x4_add
            @simd I32x4Sub => visit_i32x4_sub
            @simd I32x4Mul => visit_i32x4_mul
            @simd I32x4MinS => visit_i32x4_min_s
            @simd I32x4MinU => visit_i32x4_min_u
            @simd I32x4MaxS => visit_i32x4_max_s
            @simd I32x4MaxU => visit_i32x4_max_u
            @simd I32x4DotI16x8S => visit_i32x4_dot_i16x8_s
            @simd I32x4ExtMulLowI16x8S => visit_i32x4_extmul_low_i16x8_s
            @simd I32x4ExtMulHighI16x8S => visit_i32x4_extmul_high_i16x8_s
            @simd I32x4ExtMulLowI16x8U => visit_i32x4_extmul_low_i16x8_u
            @simd I32x4ExtMulHighI16x8U => visit_i32x4_extmul_high_i16x8_u
            @simd I64x2Abs => visit_i64x2_abs
            @simd I64x2Neg => visit_i64x2_neg
            @simd I64x2AllTrue => visit_i64x2_all_true
            @simd I64x2Bitmask => visit_i64x2_bitmask
            @simd I64x2ExtendLowI32x4S => visit_i64x2_extend_low_i32x4_s
            @simd I64x2ExtendHighI32x4S => visit_i64x2_extend_high_i32x4_s
            @simd I64x2ExtendLowI32x4U => visit_i64x2_extend_low_i32x4_u
            @simd I64x2ExtendHighI32x4U => visit_i64x2_extend_high_i32x4_u
            @simd I64x2Shl => visit_i64x2_shl
            @simd I64x2ShrS => visit_i64x2_shr_s
            @simd I64x2ShrU => visit_i64x2_shr_u
            @simd I64x2Add => visit_i64x2_add
            @simd I64x2Sub => visit_i64x2_sub
            @simd I64x2Mul => visit_i64x2_mul
            @simd I64x2ExtMulLowI32x4S => visit_i64x2_extmul_low_i32x4_s
            @simd I64x2ExtMulHighI32x4S => visit_i64x2_extmul_high_i32x4_s
            @simd I64x2ExtMulLowI32x4U => visit_i64x2_extmul_low_i32x4_u
            @simd I64x2ExtMulHighI32x4U => visit_i64x2_extmul_high_i32x4_u
            @simd F32x4Ceil => visit_f32x4_ceil
            @simd F32x4Floor => visit_f32x4_floor
            @simd F32x4Trunc => visit_f32x4_trunc
            @simd F32x4Nearest => visit_f32x4_nearest
            @simd F32x4Abs => visit_f32x4_abs
            @simd F32x4Neg => visit_f32x4_neg
            @simd F32x4Sqrt => visit_f32x4_sqrt
            @simd F32x4Add => visit_f32x4_add
            @simd F32x4Sub => visit_f32x4_sub
            @simd F32x4Mul => visit_f32x4_mul
            @simd F32x4Div => visit_f32x4_div
            @simd F32x4Min => visit_f32x4_min
            @simd F32x4Max => visit_f32x4_max
            @simd F32x4PMin => visit_f32x4_pmin
            @simd F32x4PMax => visit_f32x4_pmax
            @simd F64x2Ceil => visit_f64x2_ceil
            @simd F64x2Floor => visit_f64x2_floor
            @simd F64x2Trunc => visit_f64x2_trunc
            @simd F64x2Nearest => visit_f64x2_nearest
            @simd F64x2Abs => visit_f64x2_abs
            @simd F64x2Neg => visit_f64x2_neg
            @simd F64x2Sqrt => visit_f64x2_sqrt
            @simd F64x2Add => visit_f64x2_add
            @simd F64x2Sub => visit_f64x2_sub
            @simd F64x2Mul => visit_f64x2_mul
            @simd F64x2Div => visit_f64x2_div
            @simd F64x2Min => visit_f64x2_min
            @simd F64x2Max => visit_f64x2_max
            @simd F64x2PMin => visit_f64x2_pmin
            @simd F64x2PMax => visit_f64x2_pmax
            @simd I32x4TruncSatF32x4S => visit_i32x4_trunc_sat_f32x4_s
            @simd I32x4TruncSatF32x4U => visit_i32x4_trunc_sat_f32x4_u
            @simd F32x4ConvertI32x4S => visit_f32x4_convert_i32x4_s
            @simd F32x4ConvertI32x4U => visit_f32x4_convert_i32x4_u
            @simd I32x4TruncSatF64x2SZero => visit_i32x4_trunc_sat_f64x2_s_zero
            @simd I32x4TruncSatF64x2UZero => visit_i32x4_trunc_sat_f64x2_u_zero
            @simd F64x2ConvertLowI32x4S => visit_f64x2_convert_low_i32x4_s
            @simd F64x2ConvertLowI32x4U => visit_f64x2_convert_low_i32x4_u
            @simd F32x4DemoteF64x2Zero => visit_f32x4_demote_f64x2_zero
            @simd F64x2PromoteLowF32x4 => visit_f64x2_promote_low_f32x4

            // Relaxed SIMD operators
            // https://github.com/WebAssembly/relaxed-simd
            @relaxed_simd I8x16RelaxedSwizzle => visit_i8x16_relaxed_swizzle
            @relaxed_simd I32x4RelaxedTruncF32x4S => visit_i32x4_relaxed_trunc_f32x4_s
            @relaxed_simd I32x4RelaxedTruncF32x4U => visit_i32x4_relaxed_trunc_f32x4_u
            @relaxed_simd I32x4RelaxedTruncF64x2SZero => visit_i32x4_relaxed_trunc_f64x2_s_zero
            @relaxed_simd I32x4RelaxedTruncF64x2UZero => visit_i32x4_relaxed_trunc_f64x2_u_zero
            @relaxed_simd F32x4RelaxedMadd => visit_f32x4_relaxed_madd
            @relaxed_simd F32x4RelaxedNmadd => visit_f32x4_relaxed_nmadd
            @relaxed_simd F64x2RelaxedMadd => visit_f64x2_relaxed_madd
            @relaxed_simd F64x2RelaxedNmadd => visit_f64x2_relaxed_nmadd
            @relaxed_simd I8x16RelaxedLaneselect => visit_i8x16_relaxed_laneselect
            @relaxed_simd I16x8RelaxedLaneselect => visit_i16x8_relaxed_laneselect
            @relaxed_simd I32x4RelaxedLaneselect => visit_i32x4_relaxed_laneselect
            @relaxed_simd I64x2RelaxedLaneselect => visit_i64x2_relaxed_laneselect
            @relaxed_simd F32x4RelaxedMin => visit_f32x4_relaxed_min
            @relaxed_simd F32x4RelaxedMax => visit_f32x4_relaxed_max
            @relaxed_simd F64x2RelaxedMin => visit_f64x2_relaxed_min
            @relaxed_simd F64x2RelaxedMax => visit_f64x2_relaxed_max
            @relaxed_simd I16x8RelaxedQ15mulrS => visit_i16x8_relaxed_q15mulr_s
            @relaxed_simd I16x8RelaxedDotI8x16I7x16S => visit_i16x8_relaxed_dot_i8x16_i7x16_s
            @relaxed_simd I32x4RelaxedDotI8x16I7x16AddS => visit_i32x4_relaxed_dot_i8x16_i7x16_add_s

            // Typed Function references
            @function_references CallRef { type_index: u32 } => visit_call_ref
            @function_references ReturnCallRef { type_index: u32 } => visit_return_call_ref
            @function_references RefAsNonNull => visit_ref_as_non_null
            @function_references BrOnNull { relative_depth: u32 } => visit_br_on_null
            @function_references BrOnNonNull { relative_depth: u32 } => visit_br_on_non_null
        }
    };
}

macro_rules! format_err {
    ($offset:expr, $($arg:tt)*) => {
        crate::BinaryReaderError::fmt(format_args!($($arg)*), $offset)
    }
}

macro_rules! bail {
    ($($arg:tt)*) => {return Err(format_err!($($arg)*))}
}

pub use crate::binary_reader::{BinaryReader, BinaryReaderError, Result};
pub use crate::parser::*;
pub use crate::readers::*;
pub use crate::resources::*;
pub use crate::validator::*;

mod binary_reader;
mod limits;
mod parser;
mod readers;
mod resources;
mod validator;
