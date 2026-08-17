use super::{ArrayInit, GcCompiler};
use crate::bounds_checks::BoundsCheck;
use crate::func_environ::{Extension, FuncEnvironment};
use crate::translate::{Heap, HeapData, StructFieldsVec, TargetEnvironment};
use crate::{Reachability, TRAP_INTERNAL_ASSERT};
use cranelift_codegen::ir::immediates::Offset32;
use cranelift_codegen::{
    cursor::FuncCursor,
    ir::{self, InstBuilder, condcodes::IntCC},
};
use cranelift_entity::packed_option::ReservedValue;
use cranelift_frontend::FunctionBuilder;
use smallvec::SmallVec;
use wasmtime_environ::{
    Collector, GcArrayLayout, GcLayout, GcStructLayout, I31_DISCRIMINANT, ModuleInternedTypeIndex,
    PtrSize, TypeIndex, VMGcKind, WasmHeapTopType, WasmHeapType, WasmRefType, WasmResult,
    WasmStorageType, WasmValType, wasm_unsupported,
};

#[cfg(feature = "gc-drc")]
mod drc;
#[cfg(feature = "gc-null")]
mod null;

/// Get the default GC compiler.
pub fn gc_compiler(func_env: &mut FuncEnvironment<'_>) -> WasmResult<Box<dyn GcCompiler>> {
    // If this function requires a GC compiler, that is not too bad of an
    // over-approximation for it requiring a GC heap.
    func_env.needs_gc_heap = true;

    match func_env.tunables.collector {
        #[cfg(feature = "gc-drc")]
        Some(Collector::DeferredReferenceCounting) => Ok(Box::new(drc::DrcCompiler::default())),
        #[cfg(not(feature = "gc-drc"))]
        Some(Collector::DeferredReferenceCounting) => Err(wasm_unsupported!(
            "the DRC collector is unavailable because the `gc-drc` feature \
             was disabled at compile time",
        )),

        #[cfg(feature = "gc-null")]
        Some(Collector::Null) => Ok(Box::new(null::NullCompiler::default())),
        #[cfg(not(feature = "gc-null"))]
        Some(Collector::Null) => Err(wasm_unsupported!(
            "the null collector is unavailable because the `gc-null` feature \
             was disabled at compile time",
        )),

        #[cfg(any(feature = "gc-drc", feature = "gc-null"))]
        None => Err(wasm_unsupported!(
            "support for GC types disabled at configuration time"
        )),
        #[cfg(not(any(feature = "gc-drc", feature = "gc-null")))]
        None => Err(wasm_unsupported!(
            "support for GC types disabled because no collector implementation \
             was selected at compile time; enable one of the `gc-drc` or \
             `gc-null` features",
        )),
    }
}

#[cfg_attr(
    not(feature = "gc-drc"),
    expect(dead_code, reason = "easier to define")
)]
fn unbarriered_load_gc_ref(
    builder: &mut FunctionBuilder,
    ty: WasmHeapType,
    ptr_to_gc_ref: ir::Value,
    flags: ir::MemFlags,
) -> WasmResult<ir::Value> {
    debug_assert!(ty.is_vmgcref_type());
    let gc_ref = builder.ins().load(ir::types::I32, flags, ptr_to_gc_ref, 0);
    if ty != WasmHeapType::I31 {
        builder.declare_value_needs_stack_map(gc_ref);
    }
    Ok(gc_ref)
}

#[cfg_attr(
    not(any(feature = "gc-drc", feature = "gc-null")),
    expect(dead_code, reason = "easier to define")
)]
fn unbarriered_store_gc_ref(
    builder: &mut FunctionBuilder,
    ty: WasmHeapType,
    dst: ir::Value,
    gc_ref: ir::Value,
    flags: ir::MemFlags,
) -> WasmResult<()> {
    debug_assert!(ty.is_vmgcref_type());
    builder.ins().store(flags, gc_ref, dst, 0);
    Ok(())
}

/// Emit code to read a struct field or array element from its raw address in
/// the GC heap.
///
/// The given address MUST have already been bounds-checked via
/// `prepare_gc_ref_access`.
fn read_field_at_addr(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    ty: WasmStorageType,
    addr: ir::Value,
    extension: Option<Extension>,
) -> WasmResult<ir::Value> {
    assert_eq!(extension.is_none(), matches!(ty, WasmStorageType::Val(_)));
    assert_eq!(
        extension.is_some(),
        matches!(ty, WasmStorageType::I8 | WasmStorageType::I16)
    );

    // Data inside GC objects is always little endian.
    let flags = ir::MemFlags::trusted().with_endianness(ir::Endianness::Little);

    let value = match ty {
        WasmStorageType::I8 => builder.ins().load(ir::types::I8, flags, addr, 0),
        WasmStorageType::I16 => builder.ins().load(ir::types::I16, flags, addr, 0),
        WasmStorageType::Val(v) => match v {
            WasmValType::I32 => builder.ins().load(ir::types::I32, flags, addr, 0),
            WasmValType::I64 => builder.ins().load(ir::types::I64, flags, addr, 0),
            WasmValType::F32 => builder.ins().load(ir::types::F32, flags, addr, 0),
            WasmValType::F64 => builder.ins().load(ir::types::F64, flags, addr, 0),
            WasmValType::V128 => builder.ins().load(ir::types::I8X16, flags, addr, 0),
            WasmValType::Ref(r) => match r.heap_type.top() {
                WasmHeapTopType::Any | WasmHeapTopType::Extern | WasmHeapTopType::Exn => {
                    gc_compiler(func_env)?
                        .translate_read_gc_reference(func_env, builder, r, addr, flags)?
                }
                WasmHeapTopType::Func => {
                    let expected_ty = match r.heap_type {
                        WasmHeapType::Func => ModuleInternedTypeIndex::reserved_value(),
                        WasmHeapType::ConcreteFunc(ty) => ty.unwrap_module_type_index(),
                        WasmHeapType::NoFunc => {
                            let null = builder.ins().iconst(func_env.pointer_type(), 0);
                            if !r.nullable {
                                // Because `nofunc` is uninhabited, and this
                                // reference is non-null, this is unreachable
                                // code. Unconditionally trap via conditional
                                // trap instructions to avoid inserting block
                                // terminators in the middle of this block.
                                builder.ins().trapz(null, TRAP_INTERNAL_ASSERT);
                            }
                            return Ok(null);
                        }
                        _ => unreachable!("not a function heap type"),
                    };
                    let expected_ty = builder
                        .ins()
                        .iconst(ir::types::I32, i64::from(expected_ty.as_bits()));

                    let vmctx = func_env.vmctx_val(&mut builder.cursor());

                    let func_ref_id = builder.ins().load(ir::types::I32, flags, addr, 0);
                    let get_interned_func_ref = func_env
                        .builtin_functions
                        .get_interned_func_ref(builder.func);

                    let call_inst = builder
                        .ins()
                        .call(get_interned_func_ref, &[vmctx, func_ref_id, expected_ty]);
                    builder.func.dfg.first_result(call_inst)
                }
                WasmHeapTopType::Cont => {
                    // TODO(#10248) GC integration for stack switching
                    return Err(wasmtime_environ::WasmError::Unsupported(
                        "Stack switching feature not compatbile with GC, yet".to_string(),
                    ));
                }
            },
        },
    };

    let value = match extension {
        Some(Extension::Sign) => builder.ins().sextend(ir::types::I32, value),
        Some(Extension::Zero) => builder.ins().uextend(ir::types::I32, value),
        None => value,
    };

    Ok(value)
}

fn write_func_ref_at_addr(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    ref_type: WasmRefType,
    flags: ir::MemFlags,
    field_addr: ir::Value,
    func_ref: ir::Value,
) -> WasmResult<()> {
    assert_eq!(ref_type.heap_type.top(), WasmHeapTopType::Func);

    let vmctx = func_env.vmctx_val(&mut builder.cursor());

    let intern_func_ref_for_gc_heap = func_env
        .builtin_functions
        .intern_func_ref_for_gc_heap(builder.func);

    let func_ref = if ref_type.heap_type == WasmHeapType::NoFunc {
        let null = builder.ins().iconst(func_env.pointer_type(), 0);
        if !ref_type.nullable {
            // Because `nofunc` is uninhabited, and this reference is
            // non-null, this is unreachable code. Unconditionally trap
            // via conditional trap instructions to avoid inserting
            // block terminators in the middle of this block.
            builder.ins().trapz(null, TRAP_INTERNAL_ASSERT);
        }
        null
    } else {
        func_ref
    };

    // Convert the raw `funcref` into a `FuncRefTableId` for use in the
    // GC heap.
    let call_inst = builder
        .ins()
        .call(intern_func_ref_for_gc_heap, &[vmctx, func_ref]);
    let func_ref_id = builder.func.dfg.first_result(call_inst);
    let func_ref_id = builder.ins().ireduce(ir::types::I32, func_ref_id);

    // Store the id in the field.
    builder.ins().store(flags, func_ref_id, field_addr, 0);

    Ok(())
}

fn write_field_at_addr(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    field_ty: WasmStorageType,
    field_addr: ir::Value,
    new_val: ir::Value,
) -> WasmResult<()> {
    // Data inside GC objects is always little endian.
    let flags = ir::MemFlags::trusted().with_endianness(ir::Endianness::Little);

    match field_ty {
        WasmStorageType::I8 => {
            builder.ins().istore8(flags, new_val, field_addr, 0);
        }
        WasmStorageType::I16 => {
            builder.ins().istore16(flags, new_val, field_addr, 0);
        }
        WasmStorageType::Val(WasmValType::Ref(r)) if r.heap_type.top() == WasmHeapTopType::Func => {
            write_func_ref_at_addr(func_env, builder, r, flags, field_addr, new_val)?;
        }
        WasmStorageType::Val(WasmValType::Ref(r)) => {
            gc_compiler(func_env)?
                .translate_write_gc_reference(func_env, builder, r, field_addr, new_val, flags)?;
        }
        WasmStorageType::Val(_) => {
            assert_eq!(
                builder.func.dfg.value_type(new_val).bytes(),
                wasmtime_environ::byte_size_of_wasm_ty_in_gc_heap(&field_ty)
            );
            builder.ins().store(flags, new_val, field_addr, 0);
        }
    }
    Ok(())
}

pub fn translate_struct_new(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    struct_type_index: TypeIndex,
    fields: &[ir::Value],
) -> WasmResult<ir::Value> {
    gc_compiler(func_env)?.alloc_struct(func_env, builder, struct_type_index, &fields)
}

fn default_value(
    cursor: &mut FuncCursor,
    func_env: &FuncEnvironment<'_>,
    ty: &WasmStorageType,
) -> ir::Value {
    match ty {
        WasmStorageType::I8 | WasmStorageType::I16 => cursor.ins().iconst(ir::types::I32, 0),
        WasmStorageType::Val(v) => match v {
            WasmValType::I32 => cursor.ins().iconst(ir::types::I32, 0),
            WasmValType::I64 => cursor.ins().iconst(ir::types::I64, 0),
            WasmValType::F32 => cursor.ins().f32const(0.0),
            WasmValType::F64 => cursor.ins().f64const(0.0),
            WasmValType::V128 => {
                let c = cursor.func.dfg.constants.insert(vec![0; 16].into());
                cursor.ins().vconst(ir::types::I8X16, c)
            }
            WasmValType::Ref(r) => {
                assert!(r.nullable);
                let (ty, needs_stack_map) = func_env.reference_type(r.heap_type);

                // NB: The collector doesn't need to know about null references.
                let _ = needs_stack_map;

                cursor.ins().iconst(ty, 0)
            }
        },
    }
}

pub fn translate_struct_new_default(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    struct_type_index: TypeIndex,
) -> WasmResult<ir::Value> {
    let interned_ty = func_env.module.types[struct_type_index].unwrap_module_type_index();
    let struct_ty = func_env.types.unwrap_struct(interned_ty)?;
    let fields = struct_ty
        .fields
        .iter()
        .map(|f| default_value(&mut builder.cursor(), func_env, &f.element_type))
        .collect::<StructFieldsVec>();
    gc_compiler(func_env)?.alloc_struct(func_env, builder, struct_type_index, &fields)
}

pub fn translate_struct_get(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    struct_type_index: TypeIndex,
    field_index: u32,
    struct_ref: ir::Value,
    extension: Option<Extension>,
) -> WasmResult<ir::Value> {
    log::trace!(
        "translate_struct_get({struct_type_index:?}, {field_index:?}, {struct_ref:?}, {extension:?})"
    );

    // TODO: If we know we have a `(ref $my_struct)` here, instead of maybe a
    // `(ref null $my_struct)`, we could omit the `trapz`. But plumbing that
    // type info from `wasmparser` and through to here is a bit funky.
    func_env.trapz(builder, struct_ref, crate::TRAP_NULL_REFERENCE);

    let field_index = usize::try_from(field_index).unwrap();
    let interned_type_index = func_env.module.types[struct_type_index].unwrap_module_type_index();

    let struct_layout = func_env.struct_layout(interned_type_index);
    let struct_size = struct_layout.size;

    let field_offset = struct_layout.fields[field_index].offset;
    let field_ty = &func_env.types.unwrap_struct(interned_type_index)?.fields[field_index];
    let field_size = wasmtime_environ::byte_size_of_wasm_ty_in_gc_heap(&field_ty.element_type);
    assert!(field_offset + field_size <= struct_size);

    let field_addr = func_env.prepare_gc_ref_access(
        builder,
        struct_ref,
        BoundsCheck::StaticObjectField {
            offset: field_offset,
            access_size: u8::try_from(field_size).unwrap(),
            object_size: struct_size,
        },
    );

    let result = read_field_at_addr(
        func_env,
        builder,
        field_ty.element_type,
        field_addr,
        extension,
    );
    log::trace!("translate_struct_get(..) -> {result:?}");
    result
}

pub fn translate_struct_set(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    struct_type_index: TypeIndex,
    field_index: u32,
    struct_ref: ir::Value,
    new_val: ir::Value,
) -> WasmResult<()> {
    log::trace!(
        "translate_struct_set({struct_type_index:?}, {field_index:?}, struct_ref: {struct_ref:?}, new_val: {new_val:?})"
    );

    // TODO: See comment in `translate_struct_get` about the `trapz`.
    func_env.trapz(builder, struct_ref, crate::TRAP_NULL_REFERENCE);

    let field_index = usize::try_from(field_index).unwrap();
    let interned_type_index = func_env.module.types[struct_type_index].unwrap_module_type_index();

    let struct_layout = func_env.struct_layout(interned_type_index);
    let struct_size = struct_layout.size;

    let field_offset = struct_layout.fields[field_index].offset;
    let field_ty = &func_env.types.unwrap_struct(interned_type_index)?.fields[field_index];
    let field_size = wasmtime_environ::byte_size_of_wasm_ty_in_gc_heap(&field_ty.element_type);
    assert!(field_offset + field_size <= struct_size);

    let field_addr = func_env.prepare_gc_ref_access(
        builder,
        struct_ref,
        BoundsCheck::StaticObjectField {
            offset: field_offset,
            access_size: u8::try_from(field_size).unwrap(),
            object_size: struct_size,
        },
    );

    write_field_at_addr(
        func_env,
        builder,
        field_ty.element_type,
        field_addr,
        new_val,
    )?;

    log::trace!("translate_struct_set: finished");
    Ok(())
}

pub fn translate_array_new(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder,
    array_type_index: TypeIndex,
    elem: ir::Value,
    len: ir::Value,
) -> WasmResult<ir::Value> {
    log::trace!("translate_array_new({array_type_index:?}, {elem:?}, {len:?})");
    let result = gc_compiler(func_env)?.alloc_array(
        func_env,
        builder,
        array_type_index,
        ArrayInit::Fill { elem, len },
    )?;
    log::trace!("translate_array_new(..) -> {result:?}");
    Ok(result)
}

pub fn translate_array_new_default(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder,
    array_type_index: TypeIndex,
    len: ir::Value,
) -> WasmResult<ir::Value> {
    log::trace!("translate_array_new_default({array_type_index:?}, {len:?})");

    let interned_ty = func_env.module.types[array_type_index].unwrap_module_type_index();
    let array_ty = func_env.types.unwrap_array(interned_ty)?;
    let elem = default_value(&mut builder.cursor(), func_env, &array_ty.0.element_type);
    let result = gc_compiler(func_env)?.alloc_array(
        func_env,
        builder,
        array_type_index,
        ArrayInit::Fill { elem, len },
    )?;
    log::trace!("translate_array_new_default(..) -> {result:?}");
    Ok(result)
}

pub fn translate_array_new_fixed(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder,
    array_type_index: TypeIndex,
    elems: &[ir::Value],
) -> WasmResult<ir::Value> {
    log::trace!("translate_array_new_fixed({array_type_index:?}, {elems:?})");
    let result = gc_compiler(func_env)?.alloc_array(
        func_env,
        builder,
        array_type_index,
        ArrayInit::Elems(elems),
    )?;
    log::trace!("translate_array_new_fixed(..) -> {result:?}");
    Ok(result)
}

impl ArrayInit<'_> {
    /// Get the length (as an `i32`-typed `ir::Value`) of these array elements.
    #[cfg_attr(
        not(any(feature = "gc-drc", feature = "gc-null")),
        expect(dead_code, reason = "easier to define")
    )]
    fn len(self, pos: &mut FuncCursor) -> ir::Value {
        match self {
            ArrayInit::Fill { len, .. } => len,
            ArrayInit::Elems(e) => {
                let len = u32::try_from(e.len()).unwrap();
                pos.ins().iconst(ir::types::I32, i64::from(len))
            }
        }
    }

    /// Initialize a newly-allocated array's elements.
    #[cfg_attr(
        not(any(feature = "gc-drc", feature = "gc-null")),
        expect(dead_code, reason = "easier to define")
    )]
    fn initialize(
        self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder<'_>,
        interned_type_index: ModuleInternedTypeIndex,
        base_size: u32,
        size: ir::Value,
        elems_addr: ir::Value,
        mut init_field: impl FnMut(
            &mut FuncEnvironment<'_>,
            &mut FunctionBuilder<'_>,
            WasmStorageType,
            ir::Value,
            ir::Value,
        ) -> WasmResult<()>,
    ) -> WasmResult<()> {
        log::trace!(
            "initialize_array({interned_type_index:?}, {base_size:?}, {size:?}, {elems_addr:?})"
        );

        assert!(!func_env.types[interned_type_index].composite_type.shared);
        let array_ty = func_env.types[interned_type_index]
            .composite_type
            .inner
            .unwrap_array();
        let elem_ty = array_ty.0.element_type;
        let elem_size = wasmtime_environ::byte_size_of_wasm_ty_in_gc_heap(&elem_ty);
        let pointer_type = func_env.pointer_type();
        let elem_size = builder.ins().iconst(pointer_type, i64::from(elem_size));
        match self {
            ArrayInit::Elems(elems) => {
                let mut elem_addr = elems_addr;
                for val in elems {
                    init_field(func_env, builder, elem_ty, elem_addr, *val)?;
                    elem_addr = builder.ins().iadd(elem_addr, elem_size);
                }
            }
            ArrayInit::Fill { elem, len: _ } => {
                // Compute the end address of the elements.
                let base_size = builder.ins().iconst(pointer_type, i64::from(base_size));
                let array_addr = builder.ins().isub(elems_addr, base_size);
                let size = uextend_i32_to_pointer_type(builder, pointer_type, size);
                let elems_end = builder.ins().iadd(array_addr, size);

                emit_array_fill_impl(
                    func_env,
                    builder,
                    elems_addr,
                    elem_size,
                    elems_end,
                    |func_env, builder, elem_addr| {
                        init_field(func_env, builder, elem_ty, elem_addr, elem)
                    },
                )?;
            }
        }
        log::trace!("initialize_array: finished");
        Ok(())
    }
}

fn emit_array_fill_impl(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    elem_addr: ir::Value,
    elem_size: ir::Value,
    fill_end: ir::Value,
    mut emit_elem_write: impl FnMut(
        &mut FuncEnvironment<'_>,
        &mut FunctionBuilder<'_>,
        ir::Value,
    ) -> WasmResult<()>,
) -> WasmResult<()> {
    log::trace!(
        "emit_array_fill_impl(elem_addr: {elem_addr:?}, elem_size: {elem_size:?}, fill_end: {fill_end:?})"
    );

    let pointer_ty = func_env.pointer_type();

    assert_eq!(builder.func.dfg.value_type(elem_addr), pointer_ty);
    assert_eq!(builder.func.dfg.value_type(elem_size), pointer_ty);
    assert_eq!(builder.func.dfg.value_type(fill_end), pointer_ty);

    // Loop to fill the elements, emitting the equivalent of the following
    // pseudo-CLIF:
    //
    // current_block:
    //     ...
    //     jump loop_header_block(elem_addr)
    //
    // loop_header_block(elem_addr: i32):
    //     done = icmp eq elem_addr, fill_end
    //     brif done, continue_block, loop_body_block
    //
    // loop_body_block:
    //     emit_elem_write()
    //     next_elem_addr = iadd elem_addr, elem_size
    //     jump loop_header_block(next_elem_addr)
    //
    // continue_block:
    //     ...

    let current_block = builder.current_block().unwrap();
    let loop_header_block = builder.create_block();
    let loop_body_block = builder.create_block();
    let continue_block = builder.create_block();

    builder.ensure_inserted_block();
    builder.insert_block_after(loop_header_block, current_block);
    builder.insert_block_after(loop_body_block, loop_header_block);
    builder.insert_block_after(continue_block, loop_body_block);

    // Current block: jump to the loop header block with the first element's
    // address.
    builder.ins().jump(loop_header_block, &[elem_addr.into()]);

    // Loop header block: check if we're done, then jump to either the continue
    // block or the loop body block.
    builder.switch_to_block(loop_header_block);
    builder.append_block_param(loop_header_block, pointer_ty);
    log::trace!("emit_array_fill_impl: loop header");
    let elem_addr = builder.block_params(loop_header_block)[0];
    let done = builder.ins().icmp(IntCC::Equal, elem_addr, fill_end);
    builder
        .ins()
        .brif(done, continue_block, &[], loop_body_block, &[]);

    // Loop body block: write the value to the current element, compute the next
    // element's address, and then jump back to the loop header block.
    builder.switch_to_block(loop_body_block);
    log::trace!("emit_array_fill_impl: loop body");
    emit_elem_write(func_env, builder, elem_addr)?;
    let next_elem_addr = builder.ins().iadd(elem_addr, elem_size);
    builder
        .ins()
        .jump(loop_header_block, &[next_elem_addr.into()]);

    // Continue...
    builder.switch_to_block(continue_block);
    log::trace!("emit_array_fill_impl: finished");
    builder.seal_block(loop_header_block);
    builder.seal_block(loop_body_block);
    builder.seal_block(continue_block);
    Ok(())
}

pub fn translate_array_fill(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    array_type_index: TypeIndex,
    array_ref: ir::Value,
    index: ir::Value,
    value: ir::Value,
    n: ir::Value,
) -> WasmResult<()> {
    log::trace!(
        "translate_array_fill({array_type_index:?}, {array_ref:?}, {index:?}, {value:?}, {n:?})"
    );

    let len = translate_array_len(func_env, builder, array_ref)?;

    // Check that the full range of elements we want to fill is within bounds.
    let end_index = func_env.uadd_overflow_trap(builder, index, n, crate::TRAP_ARRAY_OUT_OF_BOUNDS);
    let out_of_bounds = builder
        .ins()
        .icmp(IntCC::UnsignedGreaterThan, end_index, len);
    func_env.trapnz(builder, out_of_bounds, crate::TRAP_ARRAY_OUT_OF_BOUNDS);

    // Get the address of the first element we want to fill.
    let interned_type_index = func_env.module.types[array_type_index].unwrap_module_type_index();
    let ArraySizeInfo {
        obj_size,
        one_elem_size,
        base_size,
    } = emit_array_size_info(func_env, builder, interned_type_index, len);
    let offset_in_elems = builder.ins().imul(index, one_elem_size);
    let obj_offset = builder.ins().iadd(base_size, offset_in_elems);
    let elem_addr = func_env.prepare_gc_ref_access(
        builder,
        array_ref,
        BoundsCheck::DynamicObjectField {
            offset: obj_offset,
            object_size: obj_size,
        },
    );

    // Calculate the end address, just after the filled region.
    let fill_size = builder.ins().imul(n, one_elem_size);
    let fill_size = uextend_i32_to_pointer_type(builder, func_env.pointer_type(), fill_size);
    let fill_end = builder.ins().iadd(elem_addr, fill_size);

    let one_elem_size =
        uextend_i32_to_pointer_type(builder, func_env.pointer_type(), one_elem_size);

    let result = emit_array_fill_impl(
        func_env,
        builder,
        elem_addr,
        one_elem_size,
        fill_end,
        |func_env, builder, elem_addr| {
            let elem_ty = func_env
                .types
                .unwrap_array(interned_type_index)?
                .0
                .element_type;
            write_field_at_addr(func_env, builder, elem_ty, elem_addr, value)
        },
    );
    log::trace!("translate_array_fill(..) -> {result:?}");
    result
}

pub fn translate_array_len(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder,
    array_ref: ir::Value,
) -> WasmResult<ir::Value> {
    log::trace!("translate_array_len({array_ref:?})");

    func_env.trapz(builder, array_ref, crate::TRAP_NULL_REFERENCE);

    let len_offset = gc_compiler(func_env)?.layouts().array_length_field_offset();
    let len_field = func_env.prepare_gc_ref_access(
        builder,
        array_ref,
        // Note: We can't bounds check the whole array object's size because we
        // don't know its length yet. Chicken and egg problem.
        BoundsCheck::StaticOffset {
            offset: len_offset,
            access_size: u8::try_from(ir::types::I32.bytes()).unwrap(),
        },
    );
    let result = builder.ins().load(
        ir::types::I32,
        ir::MemFlags::trusted().with_readonly(),
        len_field,
        0,
    );
    log::trace!("translate_array_len(..) -> {result:?}");
    Ok(result)
}

struct ArraySizeInfo {
    /// The `i32` size of the whole array object, in bytes.
    obj_size: ir::Value,

    /// The `i32` size of each one of the array's elements, in bytes.
    one_elem_size: ir::Value,

    /// The `i32` size of the array's base object, in bytes. This is also the
    /// offset from the start of the array object to its elements.
    base_size: ir::Value,
}

/// Emit code to get the dynamic size (in bytes) of a whole array object, along
/// with some other related bits.
fn emit_array_size_info(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    array_type_index: ModuleInternedTypeIndex,
    // `i32` value containing the array's length.
    array_len: ir::Value,
) -> ArraySizeInfo {
    let array_layout = func_env.array_layout(array_type_index);

    // Note that we check for overflow below because we can't trust the array's
    // length: it came from inside the GC heap.
    //
    // We check for 32-bit multiplication overflow by performing a 64-bit
    // multiplication and testing the high bits.
    let one_elem_size = builder
        .ins()
        .iconst(ir::types::I64, i64::from(array_layout.elem_size));
    let array_len = builder.ins().uextend(ir::types::I64, array_len);
    let all_elems_size = builder.ins().imul(one_elem_size, array_len);

    let high_bits = builder.ins().ushr_imm(all_elems_size, 32);
    builder.ins().trapnz(high_bits, TRAP_INTERNAL_ASSERT);

    let all_elems_size = builder.ins().ireduce(ir::types::I32, all_elems_size);
    let base_size = builder
        .ins()
        .iconst(ir::types::I32, i64::from(array_layout.base_size));
    let obj_size =
        builder
            .ins()
            .uadd_overflow_trap(all_elems_size, base_size, TRAP_INTERNAL_ASSERT);

    let one_elem_size = builder.ins().ireduce(ir::types::I32, one_elem_size);

    ArraySizeInfo {
        obj_size,
        one_elem_size,
        base_size,
    }
}

/// Get the bounds-checked address of an element in an array.
///
/// The emitted code will trap if `index >= array.length`.
///
/// Returns the `ir::Value` containing the address of the `index`th element in
/// the array. You may read or write a value of the array's element type at this
/// address. You may not use it for any other kind of access, nor reuse this
/// value across GC safepoints.
fn array_elem_addr(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    array_type_index: ModuleInternedTypeIndex,
    array_ref: ir::Value,
    index: ir::Value,
) -> ir::Value {
    // First, assert that `index < array.length`.
    //
    // This check is visible at the Wasm-semantics level.
    //
    // TODO: We should emit spectre-safe bounds checks for array accesses (if
    // configured) but we don't currently have a great way to do that here. The
    // proper solution is to use linear memories to back GC heaps and reuse the
    // code in `bounds_check.rs` to implement these bounds checks. That is all
    // planned, but not yet implemented.

    let len = translate_array_len(func_env, builder, array_ref).unwrap();

    let in_bounds = builder.ins().icmp(IntCC::UnsignedLessThan, index, len);
    func_env.trapz(builder, in_bounds, crate::TRAP_ARRAY_OUT_OF_BOUNDS);

    // Compute the size (in bytes) of the whole array object.
    let ArraySizeInfo {
        obj_size,
        one_elem_size,
        base_size,
    } = emit_array_size_info(func_env, builder, array_type_index, len);

    // Compute the offset of the `index`th element within the array object.
    //
    // NB: no need to check for overflow here, since at this point we know that
    // `len * elem_size + base_size` did not overflow and `i < len`.
    let offset_in_elems = builder.ins().imul(index, one_elem_size);
    let offset_in_array = builder.ins().iadd(offset_in_elems, base_size);

    // Finally, use the object size and element offset we just computed to
    // perform our implementation-internal bounds checks.
    //
    // Checking the whole object's size, rather than the `index`th element's
    // size allows these bounds checks to be deduplicated across repeated
    // accesses to the same array at different indices.
    //
    // This check should not be visible to Wasm, and serve to protect us from
    // our own implementation bugs. The goal is to keep any potential widgets
    // confined within the GC heap, and turn what would otherwise be a security
    // vulnerability into a simple bug.
    //
    // TODO: Ideally we should fold the first Wasm-visible bounds check into
    // this internal bounds check, so that we aren't performing multiple,
    // redundant bounds checks. But we should figure out how to do this in a way
    // that doesn't defeat the object-size bounds checking's deduplication
    // mentioned above.
    func_env.prepare_gc_ref_access(
        builder,
        array_ref,
        BoundsCheck::DynamicObjectField {
            offset: offset_in_array,
            object_size: obj_size,
        },
    )
}

pub fn translate_array_get(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder,
    array_type_index: TypeIndex,
    array_ref: ir::Value,
    index: ir::Value,
    extension: Option<Extension>,
) -> WasmResult<ir::Value> {
    log::trace!("translate_array_get({array_type_index:?}, {array_ref:?}, {index:?})");

    let array_type_index = func_env.module.types[array_type_index].unwrap_module_type_index();
    let elem_addr = array_elem_addr(func_env, builder, array_type_index, array_ref, index);

    let array_ty = func_env.types.unwrap_array(array_type_index)?;
    let elem_ty = array_ty.0.element_type;

    let result = read_field_at_addr(func_env, builder, elem_ty, elem_addr, extension)?;
    log::trace!("translate_array_get(..) -> {result:?}");
    Ok(result)
}

pub fn translate_array_set(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder,
    array_type_index: TypeIndex,
    array_ref: ir::Value,
    index: ir::Value,
    value: ir::Value,
) -> WasmResult<()> {
    log::trace!("translate_array_set({array_type_index:?}, {array_ref:?}, {index:?}, {value:?})");

    let array_type_index = func_env.module.types[array_type_index].unwrap_module_type_index();
    let elem_addr = array_elem_addr(func_env, builder, array_type_index, array_ref, index);

    let array_ty = func_env.types.unwrap_array(array_type_index)?;
    let elem_ty = array_ty.0.element_type;

    write_field_at_addr(func_env, builder, elem_ty, elem_addr, value)?;

    log::trace!("translate_array_set: finished");
    Ok(())
}

pub fn translate_ref_test(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    test_ty: WasmRefType,
    val: ir::Value,
    val_ty: WasmRefType,
) -> WasmResult<ir::Value> {
    log::trace!("translate_ref_test({test_ty:?}, {val:?})");

    // First special case: testing for references to bottom types.
    if test_ty.heap_type.is_bottom() {
        let result = if test_ty.nullable {
            // All null references (within the same type hierarchy) match null
            // references to the bottom type.
            func_env.translate_ref_is_null(builder.cursor(), val, val_ty)?
        } else {
            // `ref.test` is always false for non-nullable bottom types, as the
            // bottom types are uninhabited.
            builder.ins().iconst(ir::types::I32, 0)
        };
        log::trace!("translate_ref_test(..) -> {result:?}");
        return Ok(result);
    }

    // And because `ref.test heap_ty` is only valid on operands whose type is in
    // the same type hierarchy as `heap_ty`, if `heap_ty` is its hierarchy's top
    // type, we only need to worry about whether we are testing for nullability
    // or not.
    if test_ty.heap_type.is_top() {
        let result = if test_ty.nullable {
            builder.ins().iconst(ir::types::I32, 1)
        } else {
            let is_null = func_env.translate_ref_is_null(builder.cursor(), val, val_ty)?;
            let zero = builder.ins().iconst(ir::types::I32, 0);
            let one = builder.ins().iconst(ir::types::I32, 1);
            builder.ins().select(is_null, zero, one)
        };
        log::trace!("translate_ref_test(..) -> {result:?}");
        return Ok(result);
    }

    // `i31ref`s are a little interesting because they don't point to GC
    // objects; we test the bit pattern of the reference itself.
    if test_ty.heap_type == WasmHeapType::I31 {
        let i31_mask = builder.ins().iconst(
            ir::types::I32,
            i64::from(wasmtime_environ::I31_DISCRIMINANT),
        );
        let is_i31 = builder.ins().band(val, i31_mask);
        let result = if test_ty.nullable {
            let is_null = func_env.translate_ref_is_null(builder.cursor(), val, val_ty)?;
            builder.ins().bor(is_null, is_i31)
        } else {
            is_i31
        };
        log::trace!("translate_ref_test(..) -> {result:?}");
        return Ok(result);
    }

    // Otherwise, in the general case, we need to inspect our given object's
    // actual type, which also requires null-checking and i31-checking it.

    let is_any_hierarchy = test_ty.heap_type.top() == WasmHeapTopType::Any;

    let non_null_block = builder.create_block();
    let non_null_non_i31_block = builder.create_block();
    let continue_block = builder.create_block();

    // Current block: check if the reference is null and branch appropriately.
    let is_null = func_env.translate_ref_is_null(builder.cursor(), val, val_ty)?;
    let result_when_is_null = builder
        .ins()
        .iconst(ir::types::I32, test_ty.nullable as i64);
    builder.ins().brif(
        is_null,
        continue_block,
        &[result_when_is_null.into()],
        non_null_block,
        &[],
    );

    // Non-null block: We know the GC ref is non-null, but we need to also check
    // for `i31` references that don't point to GC objects.
    builder.switch_to_block(non_null_block);
    log::trace!("translate_ref_test: non-null ref block");
    if is_any_hierarchy {
        let i31_mask = builder.ins().iconst(
            ir::types::I32,
            i64::from(wasmtime_environ::I31_DISCRIMINANT),
        );
        let is_i31 = builder.ins().band(val, i31_mask);
        // If it is an `i31`, then create the result value based on whether we
        // want `i31`s to pass the test or not.
        let result_when_is_i31 = builder.ins().iconst(
            ir::types::I32,
            matches!(
                test_ty.heap_type,
                WasmHeapType::Any | WasmHeapType::Eq | WasmHeapType::I31
            ) as i64,
        );
        builder.ins().brif(
            is_i31,
            continue_block,
            &[result_when_is_i31.into()],
            non_null_non_i31_block,
            &[],
        );
    } else {
        // If we aren't testing the `any` hierarchy, the reference cannot be an
        // `i31ref`. Jump directly to the non-null and non-i31 block; rely on
        // branch folding during lowering to clean this up.
        builder.ins().jump(non_null_non_i31_block, &[]);
    }

    // Non-null and non-i31 block: Read the actual `VMGcKind` or
    // `VMSharedTypeIndex` out of the object's header and check whether it
    // matches the expected type.
    builder.switch_to_block(non_null_non_i31_block);
    log::trace!("translate_ref_test: non-null and non-i31 ref block");
    let check_header_kind = |func_env: &mut FuncEnvironment<'_>,
                             builder: &mut FunctionBuilder,
                             val: ir::Value,
                             expected_kind: VMGcKind|
     -> ir::Value {
        let kind_addr = func_env.prepare_gc_ref_access(
            builder,
            val,
            BoundsCheck::StaticObjectField {
                offset: wasmtime_environ::VM_GC_HEADER_KIND_OFFSET,
                access_size: wasmtime_environ::VM_GC_KIND_SIZE,
                object_size: wasmtime_environ::VM_GC_HEADER_SIZE,
            },
        );
        let actual_kind = builder.ins().load(
            ir::types::I32,
            ir::MemFlags::trusted().with_readonly(),
            kind_addr,
            0,
        );
        let expected_kind = builder
            .ins()
            .iconst(ir::types::I32, i64::from(expected_kind.as_u32()));
        // Inline version of `VMGcKind::matches`.
        let and = builder.ins().band(actual_kind, expected_kind);
        let kind_matches = builder
            .ins()
            .icmp(ir::condcodes::IntCC::Equal, and, expected_kind);
        builder.ins().uextend(ir::types::I32, kind_matches)
    };
    let result = match test_ty.heap_type {
        WasmHeapType::Any
        | WasmHeapType::None
        | WasmHeapType::Extern
        | WasmHeapType::NoExtern
        | WasmHeapType::Func
        | WasmHeapType::NoFunc
        | WasmHeapType::Cont
        | WasmHeapType::NoCont
        | WasmHeapType::Exn
        | WasmHeapType::NoExn
        | WasmHeapType::I31 => unreachable!("handled top, bottom, and i31 types above"),

        // For these abstract but non-top and non-bottom types, we check the
        // `VMGcKind` that is in the object's header.
        WasmHeapType::Eq => check_header_kind(func_env, builder, val, VMGcKind::EqRef),
        WasmHeapType::Struct => check_header_kind(func_env, builder, val, VMGcKind::StructRef),
        WasmHeapType::Array => check_header_kind(func_env, builder, val, VMGcKind::ArrayRef),

        // For concrete types, we need to do a full subtype check between the
        // `VMSharedTypeIndex` in the object's header and the
        // `ModuleInternedTypeIndex` we have here.
        //
        // TODO: This check should ideally be done inline, but we don't have a
        // good way to access the `TypeRegistry`'s supertypes arrays from Wasm
        // code at the moment.
        WasmHeapType::ConcreteArray(ty)
        | WasmHeapType::ConcreteStruct(ty)
        | WasmHeapType::ConcreteExn(ty) => {
            let expected_interned_ty = ty.unwrap_module_type_index();
            let expected_shared_ty =
                func_env.module_interned_to_shared_ty(&mut builder.cursor(), expected_interned_ty);

            let ty_addr = func_env.prepare_gc_ref_access(
                builder,
                val,
                BoundsCheck::StaticOffset {
                    offset: wasmtime_environ::VM_GC_HEADER_TYPE_INDEX_OFFSET,
                    access_size: func_env.offsets.size_of_vmshared_type_index(),
                },
            );
            let actual_shared_ty = builder.ins().load(
                ir::types::I32,
                ir::MemFlags::trusted().with_readonly(),
                ty_addr,
                0,
            );

            func_env.is_subtype(builder, actual_shared_ty, expected_shared_ty)
        }

        // Same as for concrete arrays and structs except that a `VMFuncRef`
        // doesn't begin with a `VMGcHeader` and is a raw pointer rather than GC
        // heap index.
        WasmHeapType::ConcreteFunc(ty) => {
            let expected_interned_ty = ty.unwrap_module_type_index();
            let expected_shared_ty =
                func_env.module_interned_to_shared_ty(&mut builder.cursor(), expected_interned_ty);

            let actual_shared_ty = func_env.load_funcref_type_index(
                &mut builder.cursor(),
                ir::MemFlags::trusted().with_readonly(),
                val,
            );

            func_env.is_subtype(builder, actual_shared_ty, expected_shared_ty)
        }
        WasmHeapType::ConcreteCont(_) => {
            // TODO(#10248) GC integration for stack switching
            return Err(wasmtime_environ::WasmError::Unsupported(
                "Stack switching feature not compatbile with GC, yet".to_string(),
            ));
        }
    };
    builder.ins().jump(continue_block, &[result.into()]);

    // Control flow join point with the result.
    builder.switch_to_block(continue_block);
    let result = builder.append_block_param(continue_block, ir::types::I32);
    log::trace!("translate_ref_test(..) -> {result:?}");

    builder.seal_block(non_null_block);
    builder.seal_block(non_null_non_i31_block);
    builder.seal_block(continue_block);

    Ok(result)
}

fn uextend_i32_to_pointer_type(
    builder: &mut FunctionBuilder,
    pointer_type: ir::Type,
    value: ir::Value,
) -> ir::Value {
    assert_eq!(builder.func.dfg.value_type(value), ir::types::I32);
    match pointer_type {
        ir::types::I32 => value,
        ir::types::I64 => builder.ins().uextend(ir::types::I64, value),
        _ => unreachable!(),
    }
}

/// Emit CLIF to compute an array object's total size, given the dynamic length
/// in its initialization.
///
/// Traps if the size overflows.
#[cfg_attr(
    not(any(feature = "gc-drc", feature = "gc-null")),
    expect(dead_code, reason = "easier to define")
)]
fn emit_array_size(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    array_layout: &GcArrayLayout,
    len: ir::Value,
) -> ir::Value {
    let base_size = builder
        .ins()
        .iconst(ir::types::I32, i64::from(array_layout.base_size));

    // `elems_size = len * elem_size`
    //
    // Check for multiplication overflow and trap if it occurs, since that
    // means Wasm is attempting to allocate an array that is larger than our
    // implementation limits. (Note: there is no standard implementation
    // limit for array length beyond `u32::MAX`.)
    //
    // We implement this check by encoding our logically-32-bit operands as
    // i64 values, doing a 64-bit multiplication, and then checking the high
    // 32 bits of the multiplication's result. If the high 32 bits are not
    // all zeros, then the multiplication overflowed.
    debug_assert_eq!(builder.func.dfg.value_type(len), ir::types::I32);
    let len = builder.ins().uextend(ir::types::I64, len);
    let elems_size_64 = builder
        .ins()
        .imul_imm(len, i64::from(array_layout.elem_size));
    let high_bits = builder.ins().ushr_imm(elems_size_64, 32);
    func_env.trapnz(builder, high_bits, crate::TRAP_ALLOCATION_TOO_LARGE);
    let elems_size = builder.ins().ireduce(ir::types::I32, elems_size_64);

    // And if adding the base size and elements size overflows, then the
    // allocation is too large.
    let size = func_env.uadd_overflow_trap(
        builder,
        base_size,
        elems_size,
        crate::TRAP_ALLOCATION_TOO_LARGE,
    );

    size
}

/// Common helper for struct-field initialization that can be reused across
/// collectors.
#[cfg_attr(
    not(any(feature = "gc-drc", feature = "gc-null")),
    expect(dead_code, reason = "easier to define")
)]
fn initialize_struct_fields(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    struct_ty: ModuleInternedTypeIndex,
    raw_ptr_to_struct: ir::Value,
    field_values: &[ir::Value],
    mut init_field: impl FnMut(
        &mut FuncEnvironment<'_>,
        &mut FunctionBuilder<'_>,
        WasmStorageType,
        ir::Value,
        ir::Value,
    ) -> WasmResult<()>,
) -> WasmResult<()> {
    let struct_layout = func_env.struct_layout(struct_ty);
    let struct_size = struct_layout.size;
    let field_offsets: SmallVec<[_; 8]> = struct_layout.fields.iter().map(|f| f.offset).collect();
    assert_eq!(field_offsets.len(), field_values.len());

    assert!(!func_env.types[struct_ty].composite_type.shared);
    let struct_ty = func_env.types[struct_ty]
        .composite_type
        .inner
        .unwrap_struct();
    let field_types: SmallVec<[_; 8]> = struct_ty.fields.iter().cloned().collect();
    assert_eq!(field_types.len(), field_values.len());

    for ((ty, val), offset) in field_types.into_iter().zip(field_values).zip(field_offsets) {
        let size_of_access = wasmtime_environ::byte_size_of_wasm_ty_in_gc_heap(&ty.element_type);
        assert!(offset + size_of_access <= struct_size);
        let field_addr = builder.ins().iadd_imm(raw_ptr_to_struct, i64::from(offset));
        init_field(func_env, builder, ty.element_type, field_addr, *val)?;
    }

    Ok(())
}

impl FuncEnvironment<'_> {
    fn gc_layout(&mut self, type_index: ModuleInternedTypeIndex) -> &GcLayout {
        // Lazily compute and cache the layout.
        if !self.ty_to_gc_layout.contains_key(&type_index) {
            let ty = &self.types[type_index].composite_type;
            let layout = gc_compiler(self)
                .unwrap()
                .layouts()
                .gc_layout(ty)
                .expect("should only call `FuncEnvironment::gc_layout` for GC types");
            self.ty_to_gc_layout.insert(type_index, layout);
        }

        self.ty_to_gc_layout.get(&type_index).unwrap()
    }

    /// Get the `GcArrayLayout` for the array type at the given `type_index`.
    fn array_layout(&mut self, type_index: ModuleInternedTypeIndex) -> &GcArrayLayout {
        self.gc_layout(type_index).unwrap_array()
    }

    /// Get the `GcStructLayout` for the struct type at the given `type_index`.
    fn struct_layout(&mut self, type_index: ModuleInternedTypeIndex) -> &GcStructLayout {
        self.gc_layout(type_index).unwrap_struct()
    }

    /// Get or create the global for our GC heap's base pointer.
    fn get_gc_heap_base_global(&mut self, func: &mut ir::Function) -> ir::GlobalValue {
        if let Some(base) = self.gc_heap_base {
            return base;
        }

        let store_context_ptr = self.get_vmstore_context_ptr_global(func);
        let offset = self.offsets.ptr.vmstore_context_gc_heap_base();

        let mut flags = ir::MemFlags::trusted();
        if !self
            .tunables
            .gc_heap_memory_type()
            .memory_may_move(self.tunables)
        {
            flags.set_readonly();
            flags.set_can_move();
        }

        let base = func.create_global_value(ir::GlobalValueData::Load {
            base: store_context_ptr,
            offset: Offset32::new(offset.into()),
            global_type: self.pointer_type(),
            flags,
        });

        self.gc_heap_base = Some(base);
        base
    }

    /// Get the GC heap's base.
    #[cfg(any(feature = "gc-null", feature = "gc-drc"))]
    fn get_gc_heap_base(&mut self, builder: &mut FunctionBuilder) -> ir::Value {
        let global = self.get_gc_heap_base_global(&mut builder.func);
        builder.ins().global_value(self.pointer_type(), global)
    }

    fn get_gc_heap_bound_global(&mut self, func: &mut ir::Function) -> ir::GlobalValue {
        if let Some(bound) = self.gc_heap_bound {
            return bound;
        }
        let store_context_ptr = self.get_vmstore_context_ptr_global(func);
        let offset = self.offsets.ptr.vmstore_context_gc_heap_current_length();
        let bound = func.create_global_value(ir::GlobalValueData::Load {
            base: store_context_ptr,
            offset: Offset32::new(offset.into()),
            global_type: self.pointer_type(),
            flags: ir::MemFlags::trusted(),
        });
        self.gc_heap_bound = Some(bound);
        bound
    }

    /// Get the GC heap's bound.
    #[cfg(feature = "gc-null")]
    fn get_gc_heap_bound(&mut self, builder: &mut FunctionBuilder) -> ir::Value {
        let global = self.get_gc_heap_bound_global(&mut builder.func);
        builder.ins().global_value(self.pointer_type(), global)
    }

    /// Get or create the `Heap` for our GC heap.
    fn get_gc_heap(&mut self, func: &mut ir::Function) -> Heap {
        if let Some(heap) = self.gc_heap {
            return heap;
        }

        let base = self.get_gc_heap_base_global(func);
        let bound = self.get_gc_heap_bound_global(func);
        let memory = self.tunables.gc_heap_memory_type();
        let heap = self.heaps.push(HeapData {
            base,
            bound,
            pcc_memory_type: None,
            memory,
        });
        self.gc_heap = Some(heap);
        heap
    }

    /// Get the raw pointer of `gc_ref[offset]` bounds checked for an access of
    /// `size` bytes.
    ///
    /// The given `gc_ref` must be a non-null, non-i31 GC reference.
    ///
    /// If `check` is a `BoundsCheck::Object`, then it is the callers
    /// responsibility to ensure that `offset + access_size <= object_size`.
    ///
    /// Returns a raw pointer to `gc_ref[offset]` -- not a raw pointer to the GC
    /// object itself (unless `offset` happens to be `0`). This raw pointer may
    /// be used to read or write up to as many bytes as described by `bound`. Do
    /// NOT attempt accesses bytes outside of `bound`; that may lead to
    /// unchecked out-of-bounds accesses.
    ///
    /// This method is collector-agnostic.
    fn prepare_gc_ref_access(
        &mut self,
        builder: &mut FunctionBuilder,
        gc_ref: ir::Value,
        bounds_check: BoundsCheck,
    ) -> ir::Value {
        log::trace!("prepare_gc_ref_access({gc_ref:?}, {bounds_check:?})");
        assert_eq!(builder.func.dfg.value_type(gc_ref), ir::types::I32);

        let gc_heap = self.get_gc_heap(&mut builder.func);
        let gc_heap = self.heaps[gc_heap].clone();
        let result = match crate::bounds_checks::bounds_check_and_compute_addr(
            builder,
            self,
            &gc_heap,
            gc_ref,
            bounds_check,
            crate::TRAP_INTERNAL_ASSERT,
        ) {
            Reachability::Reachable(v) => v,
            Reachability::Unreachable => {
                // We are now in unreachable code, but we don't want to plumb
                // through a bunch of `Reachability` through all of our callers,
                // so just assert we won't reach here and return `null`
                let null = builder.ins().iconst(self.pointer_type(), 0);
                builder.ins().trapz(null, crate::TRAP_INTERNAL_ASSERT);
                null
            }
        };
        log::trace!("prepare_gc_ref_access(..) -> {result:?}");
        result
    }

    /// Emit checks (if necessary) for whether the given `gc_ref` is null or is
    /// an `i31ref`.
    ///
    /// Takes advantage of static information based on `ty` as to whether the GC
    /// reference is nullable or can ever be an `i31`.
    ///
    /// Returns an `ir::Value` that is an `i32` will be non-zero if the GC
    /// reference is null or is an `i31ref`; otherwise, it will be zero.
    ///
    /// This method is collector-agnostic.
    #[cfg_attr(
        not(feature = "gc-drc"),
        expect(dead_code, reason = "easier to define")
    )]
    fn gc_ref_is_null_or_i31(
        &mut self,
        builder: &mut FunctionBuilder,
        ty: WasmRefType,
        gc_ref: ir::Value,
    ) -> ir::Value {
        assert_eq!(builder.func.dfg.value_type(gc_ref), ir::types::I32);
        assert!(ty.is_vmgcref_type_and_not_i31());

        let might_be_i31 = match ty.heap_type {
            // If we are definitely dealing with an i31, we shouldn't be
            // emitting dynamic checks for it, and the caller shouldn't call
            // this function. Should have been caught by the assertion at the
            // start of the function.
            WasmHeapType::I31 => unreachable!(),

            // Could potentially be an i31.
            WasmHeapType::Any | WasmHeapType::Eq => true,

            // If it is definitely a struct, array, or uninhabited type, then it
            // is definitely not an i31.
            WasmHeapType::Array
            | WasmHeapType::ConcreteArray(_)
            | WasmHeapType::Struct
            | WasmHeapType::ConcreteStruct(_)
            | WasmHeapType::None => false,

            // Despite being a different type hierarchy, this *could* be an
            // `i31` if it is the result of
            //
            //     (extern.convert_any (ref.i31 ...))
            WasmHeapType::Extern => true,

            // Can only ever be `null`.
            WasmHeapType::NoExtern => false,

            WasmHeapType::Exn | WasmHeapType::ConcreteExn(_) | WasmHeapType::NoExn => false,

            // Wrong type hierarchy, and also funcrefs are not GC-managed
            // types. Should have been caught by the assertion at the start of
            // the function.
            WasmHeapType::Func | WasmHeapType::ConcreteFunc(_) | WasmHeapType::NoFunc => {
                unreachable!()
            }
            WasmHeapType::Cont | WasmHeapType::ConcreteCont(_) | WasmHeapType::NoCont => {
                unreachable!()
            }
        };

        match (ty.nullable, might_be_i31) {
            // This GC reference statically cannot be null nor an i31. (Let
            // Cranelift's optimizer const-propagate this value and erase any
            // unnecessary control flow resulting from branching on this value.)
            (false, false) => builder.ins().iconst(ir::types::I32, 0),

            // This GC reference is always non-null, but might be an i31.
            (false, true) => builder.ins().band_imm(gc_ref, i64::from(I31_DISCRIMINANT)),

            // This GC reference might be null, but can never be an i31.
            (true, false) => builder.ins().icmp_imm(IntCC::Equal, gc_ref, 0),

            // Fully general case: this GC reference could be either null or an
            // i31.
            (true, true) => {
                let is_i31 = builder.ins().band_imm(gc_ref, i64::from(I31_DISCRIMINANT));
                let is_null = builder.ins().icmp_imm(IntCC::Equal, gc_ref, 0);
                let is_null = builder.ins().uextend(ir::types::I32, is_null);
                builder.ins().bor(is_i31, is_null)
            }
        }
    }

    // Emit code to check whether `a <: b` for two `VMSharedTypeIndex`es.
    pub(crate) fn is_subtype(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        a: ir::Value,
        b: ir::Value,
    ) -> ir::Value {
        log::trace!("is_subtype({a:?}, {b:?})");

        let diff_tys_block = builder.create_block();
        let continue_block = builder.create_block();

        // Current block: fast path for when `a == b`.
        log::trace!("is_subtype: fast path check for exact same types");
        let same_ty = builder.ins().icmp(IntCC::Equal, a, b);
        let same_ty = builder.ins().uextend(ir::types::I32, same_ty);
        builder.ins().brif(
            same_ty,
            continue_block,
            &[same_ty.into()],
            diff_tys_block,
            &[],
        );

        // Different types block: fall back to the `is_subtype` libcall.
        builder.switch_to_block(diff_tys_block);
        log::trace!("is_subtype: slow path to do full `is_subtype` libcall");
        let is_subtype = self.builtin_functions.is_subtype(builder.func);
        let vmctx = self.vmctx_val(&mut builder.cursor());
        let call_inst = builder.ins().call(is_subtype, &[vmctx, a, b]);
        let result = builder.func.dfg.first_result(call_inst);
        builder.ins().jump(continue_block, &[result.into()]);

        // Continue block: join point for the result.
        builder.switch_to_block(continue_block);
        let result = builder.append_block_param(continue_block, ir::types::I32);
        log::trace!("is_subtype(..) -> {result:?}");

        builder.seal_block(diff_tys_block);
        builder.seal_block(continue_block);

        result
    }
}
