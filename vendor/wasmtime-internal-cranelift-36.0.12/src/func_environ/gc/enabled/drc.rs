//! Compiler for the deferred reference-counting (DRC) collector and its
//! barriers.

use super::*;
use crate::translate::TargetEnvironment;
use crate::{TRAP_INTERNAL_ASSERT, func_environ::FuncEnvironment};
use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{self, InstBuilder};
use cranelift_frontend::FunctionBuilder;
use smallvec::SmallVec;
use wasmtime_environ::{
    GcTypeLayouts, ModuleInternedTypeIndex, PtrSize, TypeIndex, VMGcKind, WasmHeapTopType,
    WasmHeapType, WasmRefType, WasmResult, WasmStorageType, WasmValType, drc::DrcTypeLayouts,
};

#[derive(Default)]
pub struct DrcCompiler {
    layouts: DrcTypeLayouts,
}

impl DrcCompiler {
    /// Generate code to load the given GC reference's ref count.
    ///
    /// Assumes that the given `gc_ref` is a non-null, non-i31 GC reference.
    fn load_ref_count(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder,
        gc_ref: ir::Value,
    ) -> ir::Value {
        let offset = func_env.offsets.vm_drc_header_ref_count();
        let pointer = func_env.prepare_gc_ref_access(
            builder,
            gc_ref,
            BoundsCheck::StaticOffset {
                offset,
                access_size: u8::try_from(ir::types::I64.bytes()).unwrap(),
            },
        );
        builder
            .ins()
            .load(ir::types::I64, ir::MemFlags::trusted(), pointer, 0)
    }

    /// Generate code to update the given GC reference's ref count to the new
    /// value.
    ///
    /// Assumes that the given `gc_ref` is a non-null, non-i31 GC reference.
    fn store_ref_count(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder,
        gc_ref: ir::Value,
        new_ref_count: ir::Value,
    ) {
        let offset = func_env.offsets.vm_drc_header_ref_count();
        let pointer = func_env.prepare_gc_ref_access(
            builder,
            gc_ref,
            BoundsCheck::StaticOffset {
                offset,
                access_size: u8::try_from(ir::types::I64.bytes()).unwrap(),
            },
        );
        builder
            .ins()
            .store(ir::MemFlags::trusted(), new_ref_count, pointer, 0);
    }

    /// Generate code to increment or decrement the given GC reference's ref
    /// count.
    ///
    /// The new ref count is returned.
    ///
    /// Assumes that the given `gc_ref` is a non-null, non-i31 GC reference.
    fn mutate_ref_count(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder,
        gc_ref: ir::Value,
        delta: i64,
    ) -> ir::Value {
        debug_assert!(delta == -1 || delta == 1);
        let old_ref_count = self.load_ref_count(func_env, builder, gc_ref);
        let new_ref_count = builder.ins().iadd_imm(old_ref_count, delta);
        self.store_ref_count(func_env, builder, gc_ref, new_ref_count);
        new_ref_count
    }

    /// Push `gc_ref` onto the over-approximated-stack-roots list.
    ///
    /// `gc_ref` must not already be in the list.
    ///
    /// `reserved` must be the current reserved bits for this `gc_ref`.
    fn push_onto_over_approximated_stack_roots(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder<'_>,
        gc_ref: ir::Value,
        reserved: ir::Value,
    ) {
        debug_assert_eq!(builder.func.dfg.value_type(gc_ref), ir::types::I32);
        debug_assert_eq!(builder.func.dfg.value_type(reserved), ir::types::I32);

        let head = self.load_over_approximated_stack_roots_head(func_env, builder);

        // Load the current first list element, which will be our new next list
        // element.
        let next = builder
            .ins()
            .load(ir::types::I32, ir::MemFlags::trusted(), head, 0);

        // Update our object's header to point to `next` and consider itself part of the list.
        self.set_next_over_approximated_stack_root(func_env, builder, gc_ref, next);
        self.set_in_over_approximated_stack_roots_bit(func_env, builder, gc_ref, reserved);

        // Increment our ref count because the list is logically holding a strong reference.
        self.mutate_ref_count(func_env, builder, gc_ref, 1);

        // Commit this object as the new head of the list.
        builder
            .ins()
            .store(ir::MemFlags::trusted(), gc_ref, head, 0);
    }

    /// Load a pointer to the first element of the DRC heap's
    /// over-approximated-stack-roots list.
    fn load_over_approximated_stack_roots_head(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder,
    ) -> ir::Value {
        let ptr_ty = func_env.pointer_type();
        let vmctx = func_env.vmctx(&mut builder.func);
        let vmctx = builder.ins().global_value(ptr_ty, vmctx);
        builder.ins().load(
            ptr_ty,
            ir::MemFlags::trusted().with_readonly(),
            vmctx,
            i32::from(func_env.offsets.ptr.vmctx_gc_heap_data()),
        )
    }

    /// Set the `VMDrcHeader::next_over_approximated_stack_root` field.
    fn set_next_over_approximated_stack_root(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder<'_>,
        gc_ref: ir::Value,
        next: ir::Value,
    ) {
        debug_assert_eq!(builder.func.dfg.value_type(gc_ref), ir::types::I32);
        debug_assert_eq!(builder.func.dfg.value_type(next), ir::types::I32);
        let ptr = func_env.prepare_gc_ref_access(
            builder,
            gc_ref,
            BoundsCheck::StaticOffset {
                offset: func_env
                    .offsets
                    .vm_drc_header_next_over_approximated_stack_root(),
                access_size: u8::try_from(ir::types::I32.bytes()).unwrap(),
            },
        );
        builder.ins().store(ir::MemFlags::trusted(), next, ptr, 0);
    }

    /// Set the in-over-approximated-stack-roots list bit in a `VMDrcHeader`'s
    /// reserved bits.
    fn set_in_over_approximated_stack_roots_bit(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder<'_>,
        gc_ref: ir::Value,
        old_reserved_bits: ir::Value,
    ) {
        let in_set_bit = builder.ins().iconst(
            ir::types::I32,
            i64::from(wasmtime_environ::drc::HEADER_IN_OVER_APPROX_LIST_BIT),
        );
        let new_reserved = builder.ins().bor(old_reserved_bits, in_set_bit);
        self.set_reserved_bits(func_env, builder, gc_ref, new_reserved);
    }

    /// Update the reserved bits in a `VMDrcHeader`.
    fn set_reserved_bits(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder<'_>,
        gc_ref: ir::Value,
        new_reserved: ir::Value,
    ) {
        let ptr = func_env.prepare_gc_ref_access(
            builder,
            gc_ref,
            BoundsCheck::StaticOffset {
                offset: func_env.offsets.vm_gc_header_reserved_bits(),
                access_size: u8::try_from(ir::types::I32.bytes()).unwrap(),
            },
        );
        builder
            .ins()
            .store(ir::MemFlags::trusted(), new_reserved, ptr, 0);
    }

    /// Write to an uninitialized field or element inside a GC object.
    fn init_field(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder<'_>,
        field_addr: ir::Value,
        ty: WasmStorageType,
        val: ir::Value,
    ) -> WasmResult<()> {
        // Data inside GC objects is always little endian.
        let flags = ir::MemFlags::trusted().with_endianness(ir::Endianness::Little);

        match ty {
            WasmStorageType::Val(WasmValType::Ref(r))
                if r.heap_type.top() == WasmHeapTopType::Func =>
            {
                write_func_ref_at_addr(func_env, builder, r, flags, field_addr, val)?;
            }
            WasmStorageType::Val(WasmValType::Ref(r)) => {
                self.translate_init_gc_reference(func_env, builder, r, field_addr, val, flags)?;
            }
            WasmStorageType::I8 => {
                assert_eq!(builder.func.dfg.value_type(val), ir::types::I32);
                builder.ins().istore8(flags, val, field_addr, 0);
            }
            WasmStorageType::I16 => {
                assert_eq!(builder.func.dfg.value_type(val), ir::types::I32);
                builder.ins().istore16(flags, val, field_addr, 0);
            }
            WasmStorageType::Val(_) => {
                let size_of_access = wasmtime_environ::byte_size_of_wasm_ty_in_gc_heap(&ty);
                assert_eq!(builder.func.dfg.value_type(val).bytes(), size_of_access);
                builder.ins().store(flags, val, field_addr, 0);
            }
        }

        Ok(())
    }

    /// Write to an uninitialized GC reference field, initializing it.
    ///
    /// ```text
    /// *dst = new_val
    /// ```
    ///
    /// Doesn't need to do a full write barrier: we don't have an old reference
    /// that is being overwritten and needs its refcount decremented, just a new
    /// reference whose count should be incremented.
    fn translate_init_gc_reference(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder,
        ty: WasmRefType,
        dst: ir::Value,
        new_val: ir::Value,
        flags: ir::MemFlags,
    ) -> WasmResult<()> {
        let (ref_ty, needs_stack_map) = func_env.reference_type(ty.heap_type);
        debug_assert!(needs_stack_map);

        // Special case for references to uninhabited bottom types: see
        // `translate_write_gc_reference` for details.
        if let WasmHeapType::None = ty.heap_type {
            if ty.nullable {
                let null = builder.ins().iconst(ref_ty, 0);
                builder.ins().store(flags, null, dst, 0);
            } else {
                let zero = builder.ins().iconst(ir::types::I32, 0);
                builder.ins().trapz(zero, TRAP_INTERNAL_ASSERT);
            }
            return Ok(());
        };

        // Special case for `i31ref`s: no need for any barriers.
        if let WasmHeapType::I31 = ty.heap_type {
            return unbarriered_store_gc_ref(builder, ty.heap_type, dst, new_val, flags);
        }

        // Our initialization barrier for GC references being copied out of the
        // stack and initializing a table/global/struct field/etc... is roughly
        // equivalent to the following pseudo-CLIF:
        //
        // ```
        // current_block:
        //     ...
        //     let new_val_is_null_or_i31 = ...
        //     brif new_val_is_null_or_i31, continue_block, inc_ref_block
        //
        // inc_ref_block:
        //     let ref_count = load new_val.ref_count
        //     let new_ref_count = iadd_imm ref_count, 1
        //     store new_val.ref_count, new_ref_count
        //     jump check_old_val_block
        //
        // continue_block:
        //     store dst, new_val
        //     ...
        // ```
        //
        // This write barrier is responsible for ensuring that the new value's
        // ref count is incremented now that the table/global/struct/etc... is
        // holding onto it.

        let current_block = builder.current_block().unwrap();
        let inc_ref_block = builder.create_block();
        let continue_block = builder.create_block();

        builder.ensure_inserted_block();
        builder.insert_block_after(inc_ref_block, current_block);
        builder.insert_block_after(continue_block, inc_ref_block);

        // Current block: check whether the new value is non-null and
        // non-i31. If so, branch to the `inc_ref_block`.
        log::trace!("DRC initialization barrier: check if the value is null or i31");
        let new_val_is_null_or_i31 = func_env.gc_ref_is_null_or_i31(builder, ty, new_val);
        builder.ins().brif(
            new_val_is_null_or_i31,
            continue_block,
            &[],
            inc_ref_block,
            &[],
        );

        // Block to increment the ref count of the new value when it is non-null
        // and non-i31.
        builder.switch_to_block(inc_ref_block);
        builder.seal_block(inc_ref_block);
        log::trace!("DRC initialization barrier: increment the ref count of the initial value");
        self.mutate_ref_count(func_env, builder, new_val, 1);
        builder.ins().jump(continue_block, &[]);

        // Join point after we're done with the GC barrier: do the actual store
        // to initialize the field.
        builder.switch_to_block(continue_block);
        builder.seal_block(continue_block);
        log::trace!(
            "DRC initialization barrier: finally, store into {dst:?} to initialize the field"
        );
        unbarriered_store_gc_ref(builder, ty.heap_type, dst, new_val, flags)?;

        Ok(())
    }
}

/// Emit CLIF to call the `gc_raw_alloc` libcall.
fn emit_gc_raw_alloc(
    func_env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder<'_>,
    kind: VMGcKind,
    ty: ModuleInternedTypeIndex,
    size: ir::Value,
    align: u32,
) -> ir::Value {
    let gc_alloc_raw_builtin = func_env.builtin_functions.gc_alloc_raw(builder.func);
    let vmctx = func_env.vmctx_val(&mut builder.cursor());

    let kind = builder
        .ins()
        .iconst(ir::types::I32, i64::from(kind.as_u32()));

    let ty = builder.ins().iconst(ir::types::I32, i64::from(ty.as_u32()));

    assert!(align.is_power_of_two());
    let align = builder.ins().iconst(ir::types::I32, i64::from(align));

    let call_inst = builder
        .ins()
        .call(gc_alloc_raw_builtin, &[vmctx, kind, ty, size, align]);

    let gc_ref = builder.func.dfg.first_result(call_inst);
    builder.declare_value_needs_stack_map(gc_ref);
    gc_ref
}

impl GcCompiler for DrcCompiler {
    fn layouts(&self) -> &dyn GcTypeLayouts {
        &self.layouts
    }

    fn alloc_array(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder<'_>,
        array_type_index: TypeIndex,
        init: super::ArrayInit<'_>,
    ) -> WasmResult<ir::Value> {
        let interned_type_index =
            func_env.module.types[array_type_index].unwrap_module_type_index();
        let ptr_ty = func_env.pointer_type();

        let len_offset = gc_compiler(func_env)?.layouts().array_length_field_offset();
        let array_layout = func_env.array_layout(interned_type_index).clone();
        let base_size = array_layout.base_size;
        let align = array_layout.align;
        let len_to_elems_delta = base_size.checked_sub(len_offset).unwrap();

        // First, compute the array's total size from its base size, element
        // size, and length.
        let len = init.len(&mut builder.cursor());
        let size = emit_array_size(func_env, builder, &array_layout, len);

        // Second, now that we have the array object's total size, call the
        // `gc_alloc_raw` builtin libcall to allocate the array.
        let array_ref = emit_gc_raw_alloc(
            func_env,
            builder,
            VMGcKind::ArrayRef,
            interned_type_index,
            size,
            align,
        );

        // Write the array's length into the appropriate slot.
        //
        // Note: we don't need to bounds-check the GC ref access here, since we
        // trust the results of the allocation libcall.
        let base = func_env.get_gc_heap_base(builder);
        let extended_array_ref =
            uextend_i32_to_pointer_type(builder, func_env.pointer_type(), array_ref);
        let object_addr = builder.ins().iadd(base, extended_array_ref);
        let len_addr = builder.ins().iadd_imm(object_addr, i64::from(len_offset));
        let len = init.len(&mut builder.cursor());
        builder
            .ins()
            .store(ir::MemFlags::trusted(), len, len_addr, 0);

        // Finally, initialize the elements.
        let len_to_elems_delta = builder.ins().iconst(ptr_ty, i64::from(len_to_elems_delta));
        let elems_addr = builder.ins().iadd(len_addr, len_to_elems_delta);
        init.initialize(
            func_env,
            builder,
            interned_type_index,
            base_size,
            size,
            elems_addr,
            |func_env, builder, elem_ty, elem_addr, val| {
                self.init_field(func_env, builder, elem_addr, elem_ty, val)
            },
        )?;
        Ok(array_ref)
    }

    fn alloc_struct(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder<'_>,
        struct_type_index: TypeIndex,
        field_vals: &[ir::Value],
    ) -> WasmResult<ir::Value> {
        // First, call the `gc_alloc_raw` builtin libcall to allocate the
        // struct.
        let interned_type_index =
            func_env.module.types[struct_type_index].unwrap_module_type_index();

        let struct_layout = func_env.struct_layout(interned_type_index);

        // Copy some stuff out of the struct layout to avoid borrowing issues.
        let struct_size = struct_layout.size;
        let struct_align = struct_layout.align;
        let field_offsets: SmallVec<[_; 8]> = struct_layout.fields.iter().copied().collect();
        assert_eq!(field_vals.len(), field_offsets.len());

        let struct_size_val = builder.ins().iconst(ir::types::I32, i64::from(struct_size));

        let struct_ref = emit_gc_raw_alloc(
            func_env,
            builder,
            VMGcKind::StructRef,
            interned_type_index,
            struct_size_val,
            struct_align,
        );

        // Second, initialize each of the newly-allocated struct's fields.
        //
        // Note: we don't need to bounds-check the GC ref access here, since we
        // trust the results of the allocation libcall.
        let base = func_env.get_gc_heap_base(builder);
        let extended_struct_ref =
            uextend_i32_to_pointer_type(builder, func_env.pointer_type(), struct_ref);
        let raw_ptr_to_struct = builder.ins().iadd(base, extended_struct_ref);
        initialize_struct_fields(
            func_env,
            builder,
            interned_type_index,
            raw_ptr_to_struct,
            field_vals,
            |func_env, builder, ty, field_addr, val| {
                self.init_field(func_env, builder, field_addr, ty, val)
            },
        )?;

        Ok(struct_ref)
    }

    fn translate_read_gc_reference(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder,
        ty: WasmRefType,
        src: ir::Value,
        flags: ir::MemFlags,
    ) -> WasmResult<ir::Value> {
        log::trace!("translate_read_gc_reference({ty:?}, {src:?}, {flags:?})");

        assert!(ty.is_vmgcref_type());

        let (reference_type, needs_stack_map) = func_env.reference_type(ty.heap_type);
        debug_assert!(needs_stack_map);

        // Special case for references to uninhabited bottom types: the
        // reference must either be nullable and we can just eagerly return
        // null, or we are in dynamically unreachable code and should just trap.
        if let WasmHeapType::None = ty.heap_type {
            let null = builder.ins().iconst(reference_type, 0);

            // If the `flags` can trap, then we need to do an actual load. We
            // might be relying on, e.g., this load trapping to raise a
            // out-of-bounds-table-index trap, rather than successfully loading
            // a null `noneref`.
            //
            // That said, while we will do the load, we won't use the loaded
            // value, and will still use our null constant below. This will
            // avoid an unnecessary load dependency, slightly improving the code
            // we ultimately emit. This probably doesn't matter, but it is easy
            // to do and can only improve things, so we do it.
            if flags.trap_code().is_some() {
                let _ = builder.ins().load(reference_type, flags, src, 0);
            }

            if !ty.nullable {
                // NB: Don't use an unconditional trap instruction, since that
                // is a block terminator, and we still need to integrate with
                // the rest of the surrounding code.
                let zero = builder.ins().iconst(ir::types::I32, 0);
                builder.ins().trapz(zero, TRAP_INTERNAL_ASSERT);
            }

            return Ok(null);
        };

        // Special case for `i31` references: they don't need barriers.
        if let WasmHeapType::I31 = ty.heap_type {
            return unbarriered_load_gc_ref(builder, ty.heap_type, src, flags);
        }

        // Our read barrier for GC references is roughly equivalent to the
        // following pseudo-CLIF:
        //
        // ```
        // current_block:
        //     ...
        //     let gc_ref = load src
        //     let gc_ref_is_null = is_null gc_ref
        //     let gc_ref_is_i31 = ...
        //     let gc_ref_is_null_or_i31 = bor gc_ref_is_null, gc_ref_is_i31
        //     brif gc_ref_is_null_or_i31, continue_block, non_null_gc_ref_block
        //
        // non_null_gc_ref_block:
        //     let reserved = load reserved bits from gc_ref's header
        //     let in_set_bit = iconst OVER_APPROX_SET_BIT
        //     let in_set = band reserved, in_set_bit
        //     br_if in_set, continue_block, insert_block
        //
        // insert_block:
        //     let next = load over-approximated-stack-roots head from DRC heap
        //     store gc_ref to over-approximated-stack-roots head in DRC heap
        //     store next to gc_ref's header's next_over_approximated_stack_root field
        //     let new_reserved = bor reserved, in_set_bit
        //     store new_reserved to gc_ref's headers reserved bits
        //     inc_ref(gc_ref)
        //     jump continue_block
        //
        // continue_block:
        //     ...
        // ```
        //
        // This ensures that all GC references entering the Wasm stack are in
        // the over-approximated-stack-roots list.

        let current_block = builder.current_block().unwrap();
        let non_null_gc_ref_block = builder.create_block();
        let insert_block = builder.create_block();
        let continue_block = builder.create_block();

        builder.ensure_inserted_block();
        builder.insert_block_after(non_null_gc_ref_block, current_block);
        builder.insert_block_after(insert_block, non_null_gc_ref_block);
        builder.insert_block_after(continue_block, insert_block);

        log::trace!("DRC read barrier: load the gc reference and check for null or i31");
        let gc_ref = unbarriered_load_gc_ref(builder, ty.heap_type, src, flags)?;
        let gc_ref_is_null_or_i31 = func_env.gc_ref_is_null_or_i31(builder, ty, gc_ref);
        builder.ins().brif(
            gc_ref_is_null_or_i31,
            continue_block,
            &[],
            non_null_gc_ref_block,
            &[],
        );

        // Block for when the GC reference is not null and is not an `i31ref`.
        //
        // Tests whether the object is already in the
        // over-approximated-stack-roots list or not.
        builder.switch_to_block(non_null_gc_ref_block);
        builder.seal_block(non_null_gc_ref_block);
        log::trace!(
            "DRC read barrier: check whether this object is already in the \
             over-approximated-stack-roots list"
        );
        let ptr = func_env.prepare_gc_ref_access(
            builder,
            gc_ref,
            BoundsCheck::StaticOffset {
                offset: func_env.offsets.vm_gc_header_reserved_bits(),
                access_size: u8::try_from(ir::types::I32.bytes()).unwrap(),
            },
        );
        let reserved = builder
            .ins()
            .load(ir::types::I32, ir::MemFlags::trusted(), ptr, 0);
        let in_set_bit = builder.ins().iconst(
            ir::types::I32,
            i64::from(wasmtime_environ::drc::HEADER_IN_OVER_APPROX_LIST_BIT),
        );
        let in_set = builder.ins().band(reserved, in_set_bit);
        builder
            .ins()
            .brif(in_set, continue_block, &[], insert_block, &[]);

        // Block for when the object needs to be inserted into the
        // over-approximated-stack-roots list.
        builder.switch_to_block(insert_block);
        builder.seal_block(insert_block);
        log::trace!(
            "DRC read barrier: push the object onto the over-approximated-stack-roots list"
        );
        self.push_onto_over_approximated_stack_roots(func_env, builder, gc_ref, reserved);
        builder.ins().jump(continue_block, &[]);

        // Join point after we're done with the GC barrier.
        builder.switch_to_block(continue_block);
        builder.seal_block(continue_block);
        log::trace!("translate_read_gc_reference(..) -> {gc_ref:?}");
        Ok(gc_ref)
    }

    fn translate_write_gc_reference(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder,
        ty: WasmRefType,
        dst: ir::Value,
        new_val: ir::Value,
        flags: ir::MemFlags,
    ) -> WasmResult<()> {
        assert!(ty.is_vmgcref_type());

        let (ref_ty, needs_stack_map) = func_env.reference_type(ty.heap_type);
        debug_assert!(needs_stack_map);

        // Special case for references to uninhabited bottom types: either the
        // reference is nullable and we can just eagerly store null into `dst`
        // or we are in unreachable code and should just trap.
        if let WasmHeapType::None = ty.heap_type {
            if ty.nullable {
                let null = builder.ins().iconst(ref_ty, 0);
                builder.ins().store(flags, null, dst, 0);
            } else {
                // NB: Don't use an unconditional trap instruction, since that
                // is a block terminator, and we still need to integrate with
                // the rest of the surrounding code.
                let zero = builder.ins().iconst(ir::types::I32, 0);
                builder.ins().trapz(zero, TRAP_INTERNAL_ASSERT);
            }
            return Ok(());
        };

        // Special case for `i31` references: they don't need barriers.
        if let WasmHeapType::I31 = ty.heap_type {
            return unbarriered_store_gc_ref(builder, ty.heap_type, dst, new_val, flags);
        }

        // Our write barrier for GC references being copied out of the stack and
        // written into a table/global/etc... is roughly equivalent to the
        // following pseudo-CLIF:
        //
        // ```
        // current_block:
        //     ...
        //     let old_val = *dst
        //     let new_val_is_null = ref.null new_val
        //     let new_val_is_i31 = ...
        //     let new_val_is_null_or_i31 = bor new_val_is_null, new_val_is_i31
        //     brif new_val_is_null_or_i31, check_old_val_block, inc_ref_block
        //
        // inc_ref_block:
        //     let ref_count = load new_val.ref_count
        //     let new_ref_count = iadd_imm ref_count, 1
        //     store new_val.ref_count, new_ref_count
        //     jump check_old_val_block
        //
        // check_old_val_block:
        //     store dst, new_val
        //     let old_val_is_null = ref.null old_val
        //     let old_val_is_i31 = ...
        //     let old_val_is_null_or_i31 = bor old_val_is_null, old_val_is_i31
        //     brif old_val_is_null_or_i31, continue_block, dec_ref_block
        //
        // dec_ref_block:
        //     let ref_count = load old_val.ref_count
        //     let new_ref_count = isub_imm ref_count, 1
        //     let old_val_needs_drop = icmp_imm eq new_ref_count, 0
        //     brif old_val_needs_drop, drop_old_val_block, store_dec_ref_block
        //
        // cold drop_old_val_block:
        //     call drop_gc_ref(old_val)
        //     jump continue_block
        //
        // store_dec_ref_block:
        //     store old_val.ref_count, new_ref_count
        //     jump continue_block
        //
        // continue_block:
        //     ...
        // ```
        //
        // This write barrier is responsible for ensuring that:
        //
        // 1. The new value's ref count is incremented now that the table is
        //    holding onto it.
        //
        // 2. The old value's ref count is decremented, and that it is dropped
        //    if the ref count reaches zero.
        //
        // We must do the increment before the decrement. If we did it in the
        // other order, then when `*dst == new_val`, we could confuse ourselves
        // by observing a zero ref count after the decrement but before it would
        // become non-zero again with the subsequent increment.
        //
        // Additionally, we take care that we don't ever call out-out-of-line to
        // drop the old value until all the new value has been written into
        // `dst` and its reference count has been updated. This makes sure that
        // host code has a consistent view of the world.

        let current_block = builder.current_block().unwrap();
        let inc_ref_block = builder.create_block();
        let check_old_val_block = builder.create_block();
        let dec_ref_block = builder.create_block();
        let drop_old_val_block = builder.create_block();
        let store_dec_ref_block = builder.create_block();
        let continue_block = builder.create_block();

        builder.ensure_inserted_block();
        builder.set_cold_block(drop_old_val_block);

        builder.insert_block_after(inc_ref_block, current_block);
        builder.insert_block_after(check_old_val_block, inc_ref_block);
        builder.insert_block_after(dec_ref_block, check_old_val_block);
        builder.insert_block_after(drop_old_val_block, dec_ref_block);
        builder.insert_block_after(store_dec_ref_block, drop_old_val_block);
        builder.insert_block_after(continue_block, store_dec_ref_block);

        // Load the old value and then check whether the new value is non-null
        // and non-i31.
        log::trace!("DRC write barrier: load old ref; check if new ref is null or i31");
        let old_val = unbarriered_load_gc_ref(builder, ty.heap_type, dst, flags)?;
        let new_val_is_null_or_i31 = func_env.gc_ref_is_null_or_i31(builder, ty, new_val);
        builder.ins().brif(
            new_val_is_null_or_i31,
            check_old_val_block,
            &[],
            inc_ref_block,
            &[],
        );

        // Block to increment the ref count of the new value when it is non-null
        // and non-i31.
        builder.switch_to_block(inc_ref_block);
        log::trace!("DRC write barrier: increment new ref's ref count");
        builder.seal_block(inc_ref_block);
        self.mutate_ref_count(func_env, builder, new_val, 1);
        builder.ins().jump(check_old_val_block, &[]);

        // Block to store the new value into `dst` and then check whether the
        // old value is non-null and non-i31 and therefore needs its ref count
        // decremented.
        builder.switch_to_block(check_old_val_block);
        builder.seal_block(check_old_val_block);
        log::trace!("DRC write barrier: store new ref into field; check if old ref is null or i31");
        unbarriered_store_gc_ref(builder, ty.heap_type, dst, new_val, flags)?;
        let old_val_is_null_or_i31 = func_env.gc_ref_is_null_or_i31(builder, ty, old_val);
        builder.ins().brif(
            old_val_is_null_or_i31,
            continue_block,
            &[],
            dec_ref_block,
            &[],
        );

        // Block to decrement the ref count of the old value when it is non-null
        // and non-i31.
        builder.switch_to_block(dec_ref_block);
        builder.seal_block(dec_ref_block);
        log::trace!(
            "DRC write barrier: decrement old ref's ref count and check for zero ref count"
        );
        let ref_count = self.load_ref_count(func_env, builder, old_val);
        let new_ref_count = builder.ins().iadd_imm(ref_count, -1);
        let old_val_needs_drop = builder.ins().icmp_imm(IntCC::Equal, new_ref_count, 0);
        builder.ins().brif(
            old_val_needs_drop,
            drop_old_val_block,
            &[],
            store_dec_ref_block,
            &[],
        );

        // Block to call out-of-line to drop a GC reference when its ref count
        // reaches zero.
        //
        // Note that this libcall does its own dec-ref operation, so we only
        // actually store `new_ref_count` back to the `old_val` object when
        // `new_ref_count != 0`.
        builder.switch_to_block(drop_old_val_block);
        builder.seal_block(drop_old_val_block);
        log::trace!("DRC write barrier: drop old ref with a ref count of zero");
        let drop_gc_ref_libcall = func_env.builtin_functions.drop_gc_ref(builder.func);
        let vmctx = func_env.vmctx_val(&mut builder.cursor());
        builder.ins().call(drop_gc_ref_libcall, &[vmctx, old_val]);
        builder.ins().jump(continue_block, &[]);

        // Block to store the new ref count back to `old_val` for when
        // `new_ref_count != 0`, as explained above.
        builder.switch_to_block(store_dec_ref_block);
        builder.seal_block(store_dec_ref_block);
        log::trace!("DRC write barrier: store decremented ref count into old ref");
        self.store_ref_count(func_env, builder, old_val, new_ref_count);
        builder.ins().jump(continue_block, &[]);

        // Join point after we're done with the GC barrier.
        builder.switch_to_block(continue_block);
        builder.seal_block(continue_block);
        log::trace!("DRC write barrier: finished");
        Ok(())
    }
}
