//! Compiler for the null collector.
//!
//! Note that we don't need to mark any value as requiring inclusion in stack
//! maps inside this module, because the null collector doesn't ever collect
//! anything.

use super::*;
use crate::func_environ::FuncEnvironment;
use cranelift_codegen::ir::{self, InstBuilder};
use cranelift_frontend::FunctionBuilder;
use wasmtime_environ::VMSharedTypeIndex;
use wasmtime_environ::{
    GcTypeLayouts, ModuleInternedTypeIndex, PtrSize, TypeIndex, VMGcKind, WasmRefType, WasmResult,
    null::NullTypeLayouts,
};

#[derive(Default)]
pub struct NullCompiler {
    layouts: NullTypeLayouts,
}

impl NullCompiler {
    /// Emit code to perform an allocation inline.
    ///
    /// `kind` may be `VMGcKind::ExternRef` iff `ty` is `None`.
    ///
    /// `size` must be greater than or equal to `size_of(VMGcHeader)`.
    ///
    /// `align` must be greater than or equal to `align_of(VMGcHeader)` and a
    /// power of two.
    ///
    /// The resulting values are
    ///
    /// 1. The `VMGcRef` indexing into the GC heap.
    ///
    /// 2. The raw pointer to the start of the object inside the GC heap. This
    ///    may be used to access up to `size` bytes.
    fn emit_inline_alloc(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder,
        kind: VMGcKind,
        ty: Option<ModuleInternedTypeIndex>,
        size: ir::Value,
        align: ir::Value,
    ) -> (ir::Value, ir::Value) {
        log::trace!("emit_inline_alloc(kind={kind:?}, ty={ty:?}, size={size}, align={align})");

        assert_eq!(builder.func.dfg.value_type(size), ir::types::I32);
        assert_eq!(builder.func.dfg.value_type(align), ir::types::I32);

        let current_block = builder.current_block().unwrap();
        let continue_block = builder.create_block();
        let grow_block = builder.create_block();

        builder.ensure_inserted_block();
        builder.insert_block_after(continue_block, current_block);
        builder.insert_block_after(grow_block, continue_block);

        // Check that the size fits in the unused bits of a `VMGcKind`, since
        // the null collector stores the object's size there.
        let mask = builder
            .ins()
            .iconst(ir::types::I32, i64::from(VMGcKind::MASK));
        let masked = builder.ins().band(size, mask);
        func_env.trapnz(builder, masked, crate::TRAP_ALLOCATION_TOO_LARGE);

        // Load the bump "pointer" (it is actually an index into the GC heap,
        // not a raw pointer).
        let pointer_type = func_env.pointer_type();
        let vmctx = func_env.vmctx_val(&mut builder.cursor());
        let ptr_to_next = builder.ins().load(
            pointer_type,
            ir::MemFlags::trusted().with_readonly(),
            vmctx,
            i32::from(func_env.offsets.ptr.vmctx_gc_heap_data()),
        );
        let next = builder
            .ins()
            .load(ir::types::I32, ir::MemFlags::trusted(), ptr_to_next, 0);

        // Increment the bump "pointer" to the requested alignment:
        //
        //     next + (align - 1) & !(align - 1)
        //
        // Overflow means that the alignment is too large to satisfy, so trap
        // accordingly. Note that `align - 1` can't overflow because `align` is
        // a power of two.
        let minus_one = builder.ins().iconst(ir::types::I32, -1);
        let align_minus_one = builder.ins().iadd(align, minus_one);
        let next_plus_align_minus_one = func_env.uadd_overflow_trap(
            builder,
            next,
            align_minus_one,
            crate::TRAP_ALLOCATION_TOO_LARGE,
        );
        let not_align_minus_one = builder.ins().bnot(align_minus_one);
        let aligned = builder
            .ins()
            .band(next_plus_align_minus_one, not_align_minus_one);

        // Check whether the allocation fits in the heap space we have left.
        let end_of_object =
            func_env.uadd_overflow_trap(builder, aligned, size, crate::TRAP_ALLOCATION_TOO_LARGE);
        let uext_end_of_object = uextend_i32_to_pointer_type(builder, pointer_type, end_of_object);
        let bound = func_env.get_gc_heap_bound(builder);
        let is_in_bounds = builder.ins().icmp(
            ir::condcodes::IntCC::UnsignedLessThanOrEqual,
            uext_end_of_object,
            bound,
        );
        builder
            .ins()
            .brif(is_in_bounds, continue_block, &[], grow_block, &[]);

        log::trace!("emit_inline_alloc: grow_block");
        builder.switch_to_block(grow_block);
        builder.seal_block(grow_block);
        builder.set_cold_block(grow_block);
        let grow_gc_heap_builtin = func_env.builtin_functions.grow_gc_heap(builder.func);
        let vmctx = func_env.vmctx_val(&mut builder.cursor());
        let bytes_needed = builder.ins().isub(uext_end_of_object, bound);
        let bytes_needed = match func_env.pointer_type() {
            ir::types::I32 => builder.ins().uextend(ir::types::I64, bytes_needed),
            ir::types::I64 => bytes_needed,
            _ => unreachable!(),
        };
        builder
            .ins()
            .call(grow_gc_heap_builtin, &[vmctx, bytes_needed]);
        builder.ins().jump(continue_block, &[]);

        // Write the header, update the bump "pointer", and return the newly
        // allocated object.
        log::trace!("emit_inline_alloc: continue_block");
        builder.switch_to_block(continue_block);
        builder.seal_block(continue_block);

        // TODO: Ideally we would use a single `i64` store to write both the
        // header and the type index, but that requires generating different
        // code for big-endian architectures, and I haven't bothered doing that
        // yet.
        let base = func_env.get_gc_heap_base(builder);
        let uext_aligned = uextend_i32_to_pointer_type(builder, pointer_type, aligned);
        let ptr_to_object = builder.ins().iadd(base, uext_aligned);
        let kind = builder
            .ins()
            .iconst(ir::types::I32, i64::from(kind.as_u32()));
        let kind_and_size = builder.ins().bor(kind, size);
        let ty = match ty {
            Some(ty) => func_env.module_interned_to_shared_ty(&mut builder.cursor(), ty),
            None => builder.ins().iconst(
                func_env.vmshared_type_index_ty(),
                i64::from(VMSharedTypeIndex::reserved_value().as_bits()),
            ),
        };
        builder.ins().store(
            ir::MemFlags::trusted(),
            kind_and_size,
            ptr_to_object,
            i32::try_from(wasmtime_environ::VM_GC_HEADER_KIND_OFFSET).unwrap(),
        );
        builder.ins().store(
            ir::MemFlags::trusted(),
            ty,
            ptr_to_object,
            i32::try_from(wasmtime_environ::VM_GC_HEADER_TYPE_INDEX_OFFSET).unwrap(),
        );
        builder
            .ins()
            .store(ir::MemFlags::trusted(), end_of_object, ptr_to_next, 0);

        log::trace!("emit_inline_alloc(..) -> ({aligned}, {ptr_to_object})");
        (aligned, ptr_to_object)
    }
}

impl GcCompiler for NullCompiler {
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

        // Next, allocate the array.
        assert!(align.is_power_of_two());
        let align = builder.ins().iconst(ir::types::I32, i64::from(align));
        let (gc_ref, ptr_to_object) = self.emit_inline_alloc(
            func_env,
            builder,
            VMGcKind::ArrayRef,
            Some(interned_type_index),
            size,
            align,
        );

        // Write the array's length into its field.
        //
        // Note: we don't need to bounds-check the GC ref access here, because
        // the result of the inline allocation is trusted and we aren't reading
        // any pointers or offsets out from the (untrusted) GC heap.
        let len_addr = builder.ins().iadd_imm(ptr_to_object, i64::from(len_offset));
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
                write_field_at_addr(func_env, builder, elem_ty, elem_addr, val)
            },
        )?;

        Ok(gc_ref)
    }

    fn alloc_struct(
        &mut self,
        func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder<'_>,
        struct_type_index: TypeIndex,
        field_vals: &[ir::Value],
    ) -> WasmResult<ir::Value> {
        let interned_type_index =
            func_env.module.types[struct_type_index].unwrap_module_type_index();
        let struct_layout = func_env.struct_layout(interned_type_index);

        // Copy some stuff out of the struct layout to avoid borrowing issues.
        let struct_size = struct_layout.size;
        let struct_align = struct_layout.align;

        assert_eq!(VMGcKind::MASK & struct_size, 0);
        assert_eq!(VMGcKind::UNUSED_MASK & struct_size, struct_size);
        let struct_size_val = builder.ins().iconst(ir::types::I32, i64::from(struct_size));

        let align = builder
            .ins()
            .iconst(ir::types::I32, i64::from(struct_align));

        let (struct_ref, raw_struct_pointer) = self.emit_inline_alloc(
            func_env,
            builder,
            VMGcKind::StructRef,
            Some(interned_type_index),
            struct_size_val,
            align,
        );

        // Initialize the struct's fields.
        //
        // Note: we don't need to bounds-check the GC ref access here, because
        // the result of the inline allocation is trusted and we aren't reading
        // any pointers or offsets out from the (untrusted) GC heap.
        initialize_struct_fields(
            func_env,
            builder,
            interned_type_index,
            raw_struct_pointer,
            field_vals,
            |func_env, builder, ty, field_addr, val| {
                write_field_at_addr(func_env, builder, ty, field_addr, val)
            },
        )?;

        Ok(struct_ref)
    }

    fn translate_read_gc_reference(
        &mut self,
        _func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder,
        _ty: WasmRefType,
        src: ir::Value,
        flags: ir::MemFlags,
    ) -> WasmResult<ir::Value> {
        // NB: Don't use `unbarriered_load_gc_ref` here because we don't need to
        // mark the value as requiring inclusion in stack maps.
        Ok(builder.ins().load(ir::types::I32, flags, src, 0))
    }

    fn translate_write_gc_reference(
        &mut self,
        _func_env: &mut FuncEnvironment<'_>,
        builder: &mut FunctionBuilder,
        ty: WasmRefType,
        dst: ir::Value,
        new_val: ir::Value,
        flags: ir::MemFlags,
    ) -> WasmResult<()> {
        unbarriered_store_gc_ref(builder, ty.heap_type, dst, new_val, flags)
    }
}
