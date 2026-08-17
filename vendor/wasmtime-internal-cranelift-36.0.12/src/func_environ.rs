mod gc;

use crate::compiler::Compiler;
use crate::translate::{
    FuncTranslationStacks, GlobalVariable, Heap, HeapData, StructFieldsVec, TableData, TableSize,
    TargetEnvironment,
};
use crate::{BuiltinFunctionSignatures, TRAP_INTERNAL_ASSERT};
use cranelift_codegen::cursor::FuncCursor;
use cranelift_codegen::ir::condcodes::{FloatCC, IntCC};
use cranelift_codegen::ir::immediates::{Imm64, Offset32, V128Imm};
use cranelift_codegen::ir::pcc::Fact;
use cranelift_codegen::ir::types::*;
use cranelift_codegen::ir::{self, types};
use cranelift_codegen::ir::{ArgumentPurpose, ConstantData, Function, InstBuilder, MemFlags};
use cranelift_codegen::isa::{TargetFrontendConfig, TargetIsa};
use cranelift_entity::packed_option::{PackedOption, ReservedValue};
use cranelift_entity::{EntityRef, PrimaryMap, SecondaryMap};
use cranelift_frontend::Variable;
use cranelift_frontend::{FuncInstBuilder, FunctionBuilder};
use smallvec::SmallVec;
use std::mem;
use wasmparser::{Operator, WasmFeatures};
use wasmtime_environ::{
    BuiltinFunctionIndex, DataIndex, ElemIndex, EngineOrModuleTypeIndex, FuncIndex, GlobalIndex,
    IndexType, Memory, MemoryIndex, Module, ModuleInternedTypeIndex, ModuleTranslation,
    ModuleTypesBuilder, PtrSize, Table, TableIndex, TripleExt, Tunables, TypeConvert, TypeIndex,
    VMOffsets, WasmCompositeInnerType, WasmFuncType, WasmHeapTopType, WasmHeapType, WasmRefType,
    WasmResult, WasmValType,
};
use wasmtime_environ::{FUNCREF_INIT_BIT, FUNCREF_MASK};
use wasmtime_math::f64_cvt_to_int_bounds;

#[derive(Debug)]
pub(crate) enum Extension {
    Sign,
    Zero,
}

/// A struct with an `Option<ir::FuncRef>` member for every builtin
/// function, to de-duplicate constructing/getting its function.
pub(crate) struct BuiltinFunctions {
    types: BuiltinFunctionSignatures,

    builtins: [Option<ir::FuncRef>; BuiltinFunctionIndex::len() as usize],
}

impl BuiltinFunctions {
    fn new(compiler: &Compiler) -> Self {
        Self {
            types: BuiltinFunctionSignatures::new(compiler),
            builtins: [None; BuiltinFunctionIndex::len() as usize],
        }
    }

    fn load_builtin(&mut self, func: &mut Function, index: BuiltinFunctionIndex) -> ir::FuncRef {
        let cache = &mut self.builtins[index.index() as usize];
        if let Some(f) = cache {
            return *f;
        }
        let signature = func.import_signature(self.types.wasm_signature(index));
        let name =
            ir::ExternalName::User(func.declare_imported_user_function(ir::UserExternalName {
                namespace: crate::NS_WASMTIME_BUILTIN,
                index: index.index(),
            }));
        let f = func.import_function(ir::ExtFuncData {
            name,
            signature,
            colocated: true,
        });
        *cache = Some(f);
        f
    }
}

// Generate helper methods on `BuiltinFunctions` above for each named builtin
// as well.
macro_rules! declare_function_signatures {
    ($(
        $( #[$attr:meta] )*
        $name:ident( $( $pname:ident: $param:ident ),* ) $( -> $result:ident )?;
    )*) => {
        $(impl BuiltinFunctions {
            $( #[$attr] )*
            pub(crate) fn $name(&mut self, func: &mut Function) -> ir::FuncRef {
                self.load_builtin(func, BuiltinFunctionIndex::$name())
            }
        })*
    };
}
wasmtime_environ::foreach_builtin_function!(declare_function_signatures);

/// The `FuncEnvironment` implementation for use by the `ModuleEnvironment`.
pub struct FuncEnvironment<'module_environment> {
    compiler: &'module_environment Compiler,
    isa: &'module_environment (dyn TargetIsa + 'module_environment),
    pub(crate) module: &'module_environment Module,
    types: &'module_environment ModuleTypesBuilder,
    wasm_func_ty: &'module_environment WasmFuncType,
    sig_ref_to_ty: SecondaryMap<ir::SigRef, Option<&'module_environment WasmFuncType>>,
    needs_gc_heap: bool,
    entities: WasmEntities,

    #[cfg(feature = "gc")]
    ty_to_gc_layout: std::collections::HashMap<
        wasmtime_environ::ModuleInternedTypeIndex,
        wasmtime_environ::GcLayout,
    >,

    #[cfg(feature = "gc")]
    gc_heap: Option<Heap>,

    /// The Cranelift global holding the GC heap's base address.
    #[cfg(feature = "gc")]
    gc_heap_base: Option<ir::GlobalValue>,

    /// The Cranelift global holding the GC heap's base address.
    #[cfg(feature = "gc")]
    gc_heap_bound: Option<ir::GlobalValue>,

    translation: &'module_environment ModuleTranslation<'module_environment>,

    /// Heaps implementing WebAssembly linear memories.
    heaps: PrimaryMap<Heap, HeapData>,

    /// The Cranelift global holding the vmctx address.
    vmctx: Option<ir::GlobalValue>,

    /// The Cranelift global for our vmctx's `*mut VMStoreContext`.
    vm_store_context: Option<ir::GlobalValue>,

    /// The PCC memory type describing the vmctx layout, if we're
    /// using PCC.
    pcc_vmctx_memtype: Option<ir::MemoryType>,

    /// Caches of signatures for builtin functions.
    builtin_functions: BuiltinFunctions,

    /// Offsets to struct fields accessed by JIT code.
    pub(crate) offsets: VMOffsets<u8>,

    tunables: &'module_environment Tunables,

    /// A function-local variable which stores the cached value of the amount of
    /// fuel remaining to execute. If used this is modified frequently so it's
    /// stored locally as a variable instead of always referenced from the field
    /// in `*const VMStoreContext`
    fuel_var: cranelift_frontend::Variable,

    /// A cached epoch deadline value, when performing epoch-based
    /// interruption. Loaded from `VMStoreContext` and reloaded after
    /// any yield.
    epoch_deadline_var: cranelift_frontend::Variable,

    /// A cached pointer to the per-Engine epoch counter, when
    /// performing epoch-based interruption. Initialized in the
    /// function prologue. We prefer to use a variable here rather
    /// than reload on each check because it's better to let the
    /// regalloc keep it in a register if able; if not, it can always
    /// spill, and this isn't any worse than reloading each time.
    epoch_ptr_var: cranelift_frontend::Variable,

    fuel_consumed: i64,

    /// A `GlobalValue` in CLIF which represents the stack limit.
    ///
    /// Typically this resides in the `stack_limit` value of `ir::Function` but
    /// that requires signal handlers on the host and when that's disabled this
    /// is here with an explicit check instead. Note that the explicit check is
    /// always present even if this is a "leaf" function, as we have to call
    /// into the host to trap when signal handlers are disabled.
    pub(crate) stack_limit_at_function_entry: Option<ir::GlobalValue>,
}

impl<'module_environment> FuncEnvironment<'module_environment> {
    pub fn new(
        compiler: &'module_environment Compiler,
        translation: &'module_environment ModuleTranslation<'module_environment>,
        types: &'module_environment ModuleTypesBuilder,
        wasm_func_ty: &'module_environment WasmFuncType,
    ) -> Self {
        let tunables = compiler.tunables();
        let builtin_functions = BuiltinFunctions::new(compiler);

        // This isn't used during translation, so squash the warning about this
        // being unused from the compiler.
        let _ = BuiltinFunctions::raise;

        Self {
            isa: compiler.isa(),
            module: &translation.module,
            compiler,
            types,
            wasm_func_ty,
            sig_ref_to_ty: SecondaryMap::default(),
            needs_gc_heap: false,
            entities: WasmEntities::default(),

            #[cfg(feature = "gc")]
            ty_to_gc_layout: std::collections::HashMap::new(),
            #[cfg(feature = "gc")]
            gc_heap: None,
            #[cfg(feature = "gc")]
            gc_heap_base: None,
            #[cfg(feature = "gc")]
            gc_heap_bound: None,

            heaps: PrimaryMap::default(),
            vmctx: None,
            vm_store_context: None,
            pcc_vmctx_memtype: None,
            builtin_functions,
            offsets: VMOffsets::new(compiler.isa().pointer_bytes(), &translation.module),
            tunables,
            fuel_var: Variable::reserved_value(),
            epoch_deadline_var: Variable::reserved_value(),
            epoch_ptr_var: Variable::reserved_value(),

            // Start with at least one fuel being consumed because even empty
            // functions should consume at least some fuel.
            fuel_consumed: 1,

            translation,

            stack_limit_at_function_entry: None,
        }
    }

    pub(crate) fn pointer_type(&self) -> ir::Type {
        self.isa.pointer_type()
    }

    pub(crate) fn vmctx(&mut self, func: &mut Function) -> ir::GlobalValue {
        self.vmctx.unwrap_or_else(|| {
            let vmctx = func.create_global_value(ir::GlobalValueData::VMContext);
            if self.isa.flags().enable_pcc() {
                // Create a placeholder memtype for the vmctx; we'll
                // add fields to it as we lazily create HeapData
                // structs and global values.
                let vmctx_memtype = func.create_memory_type(ir::MemoryTypeData::Struct {
                    size: 0,
                    fields: vec![],
                });

                self.pcc_vmctx_memtype = Some(vmctx_memtype);
                func.global_value_facts[vmctx] = Some(Fact::Mem {
                    ty: vmctx_memtype,
                    min_offset: 0,
                    max_offset: 0,
                    nullable: false,
                });
            }

            self.vmctx = Some(vmctx);
            vmctx
        })
    }

    pub(crate) fn vmctx_val(&mut self, pos: &mut FuncCursor<'_>) -> ir::Value {
        let pointer_type = self.pointer_type();
        let vmctx = self.vmctx(&mut pos.func);
        pos.ins().global_value(pointer_type, vmctx)
    }

    fn get_table_copy_func(
        &mut self,
        func: &mut Function,
        dst_table_index: TableIndex,
        src_table_index: TableIndex,
    ) -> (ir::FuncRef, usize, usize) {
        let sig = self.builtin_functions.table_copy(func);
        (
            sig,
            dst_table_index.as_u32() as usize,
            src_table_index.as_u32() as usize,
        )
    }

    #[cfg(feature = "threads")]
    fn get_memory_atomic_wait(&mut self, func: &mut Function, ty: ir::Type) -> ir::FuncRef {
        match ty {
            I32 => self.builtin_functions.memory_atomic_wait32(func),
            I64 => self.builtin_functions.memory_atomic_wait64(func),
            x => panic!("get_memory_atomic_wait unsupported type: {x:?}"),
        }
    }

    fn get_global_location(
        &mut self,
        func: &mut ir::Function,
        index: GlobalIndex,
    ) -> (ir::GlobalValue, i32) {
        let pointer_type = self.pointer_type();
        let vmctx = self.vmctx(func);
        if let Some(def_index) = self.module.defined_global_index(index) {
            let offset = i32::try_from(self.offsets.vmctx_vmglobal_definition(def_index)).unwrap();
            (vmctx, offset)
        } else {
            let from_offset = self.offsets.vmctx_vmglobal_import_from(index);
            let global = func.create_global_value(ir::GlobalValueData::Load {
                base: vmctx,
                offset: Offset32::new(i32::try_from(from_offset).unwrap()),
                global_type: pointer_type,
                flags: MemFlags::trusted().with_readonly().with_can_move(),
            });
            (global, 0)
        }
    }

    /// Get or create the `ir::Global` for the `*mut VMStoreContext` in our
    /// `VMContext`.
    fn get_vmstore_context_ptr_global(&mut self, func: &mut ir::Function) -> ir::GlobalValue {
        if let Some(ptr) = self.vm_store_context {
            return ptr;
        }

        let offset = self.offsets.ptr.vmctx_store_context();
        let base = self.vmctx(func);
        let ptr = func.create_global_value(ir::GlobalValueData::Load {
            base,
            offset: Offset32::new(offset.into()),
            global_type: self.pointer_type(),
            flags: ir::MemFlags::trusted().with_readonly().with_can_move(),
        });
        self.vm_store_context = Some(ptr);
        ptr
    }

    /// Get the `*mut VMStoreContext` value for our `VMContext`.
    fn get_vmstore_context_ptr(&mut self, builder: &mut FunctionBuilder) -> ir::Value {
        let global = self.get_vmstore_context_ptr_global(&mut builder.func);
        builder.ins().global_value(self.pointer_type(), global)
    }

    fn fuel_function_entry(&mut self, builder: &mut FunctionBuilder<'_>) {
        // On function entry we load the amount of fuel into a function-local
        // `self.fuel_var` to make fuel modifications fast locally. This cache
        // is then periodically flushed to the Store-defined location in
        // `VMStoreContext` later.
        debug_assert!(self.fuel_var.is_reserved_value());
        self.fuel_var = builder.declare_var(ir::types::I64);
        self.fuel_load_into_var(builder);
        self.fuel_check(builder);
    }

    fn fuel_function_exit(&mut self, builder: &mut FunctionBuilder<'_>) {
        // On exiting the function we need to be sure to save the fuel we have
        // cached locally in `self.fuel_var` back into the Store-defined
        // location.
        self.fuel_save_from_var(builder);
    }

    fn fuel_before_op(
        &mut self,
        op: &Operator<'_>,
        builder: &mut FunctionBuilder<'_>,
        reachable: bool,
    ) {
        if !reachable {
            // In unreachable code we shouldn't have any leftover fuel we
            // haven't accounted for since the reason for us to become
            // unreachable should have already added it to `self.fuel_var`.
            debug_assert_eq!(self.fuel_consumed, 0);
            return;
        }

        self.fuel_consumed += match op {
            // Nop and drop generate no code, so don't consume fuel for them.
            Operator::Nop | Operator::Drop => 0,

            // Control flow may create branches, but is generally cheap and
            // free, so don't consume fuel. Note the lack of `if` since some
            // cost is incurred with the conditional check.
            Operator::Block { .. }
            | Operator::Loop { .. }
            | Operator::Unreachable
            | Operator::Return
            | Operator::Else
            | Operator::End => 0,

            // everything else, just call it one operation.
            _ => 1,
        };

        match op {
            // Exiting a function (via a return or unreachable) or otherwise
            // entering a different function (via a call) means that we need to
            // update the fuel consumption in `VMStoreContext` because we're
            // about to move control out of this function itself and the fuel
            // may need to be read.
            //
            // Before this we need to update the fuel counter from our own cost
            // leading up to this function call, and then we can store
            // `self.fuel_var` into `VMStoreContext`.
            Operator::Unreachable
            | Operator::Return
            | Operator::CallIndirect { .. }
            | Operator::Call { .. }
            | Operator::ReturnCall { .. }
            | Operator::ReturnCallRef { .. }
            | Operator::ReturnCallIndirect { .. } => {
                self.fuel_increment_var(builder);
                self.fuel_save_from_var(builder);
            }

            // To ensure all code preceding a loop is only counted once we
            // update the fuel variable on entry.
            Operator::Loop { .. }

            // Entering into an `if` block means that the edge we take isn't
            // known until runtime, so we need to update our fuel consumption
            // before we take the branch.
            | Operator::If { .. }

            // Control-flow instructions mean that we're moving to the end/exit
            // of a block somewhere else. That means we need to update the fuel
            // counter since we're effectively terminating our basic block.
            | Operator::Br { .. }
            | Operator::BrIf { .. }
            | Operator::BrTable { .. }

            // Exiting a scope means that we need to update the fuel
            // consumption because there are multiple ways to exit a scope and
            // this is the only time we have to account for instructions
            // executed so far.
            | Operator::End

            // This is similar to `end`, except that it's only the terminator
            // for an `if` block. The same reasoning applies though in that we
            // are terminating a basic block and need to update the fuel
            // variable.
            | Operator::Else => self.fuel_increment_var(builder),

            // This is a normal instruction where the fuel is buffered to later
            // get added to `self.fuel_var`.
            //
            // Note that we generally ignore instructions which may trap and
            // therefore result in exiting a block early. Current usage of fuel
            // means that it's not too important to account for a precise amount
            // of fuel consumed but rather "close to the actual amount" is good
            // enough. For 100% precise counting, however, we'd probably need to
            // not only increment but also save the fuel amount more often
            // around trapping instructions. (see the `unreachable` instruction
            // case above)
            //
            // Note that `Block` is specifically omitted from incrementing the
            // fuel variable. Control flow entering a `block` is unconditional
            // which means it's effectively executing straight-line code. We'll
            // update the counter when exiting a block, but we shouldn't need to
            // do so upon entering a block.
            _ => {}
        }
    }

    fn fuel_after_op(&mut self, op: &Operator<'_>, builder: &mut FunctionBuilder<'_>) {
        // After a function call we need to reload our fuel value since the
        // function may have changed it.
        match op {
            Operator::Call { .. } | Operator::CallIndirect { .. } => {
                self.fuel_load_into_var(builder);
            }
            _ => {}
        }
    }

    /// Adds `self.fuel_consumed` to the `fuel_var`, zero-ing out the amount of
    /// fuel consumed at that point.
    fn fuel_increment_var(&mut self, builder: &mut FunctionBuilder<'_>) {
        let consumption = mem::replace(&mut self.fuel_consumed, 0);
        if consumption == 0 {
            return;
        }

        let fuel = builder.use_var(self.fuel_var);
        let fuel = builder.ins().iadd_imm(fuel, consumption);
        builder.def_var(self.fuel_var, fuel);
    }

    /// Loads the fuel consumption value from `VMStoreContext` into `self.fuel_var`
    fn fuel_load_into_var(&mut self, builder: &mut FunctionBuilder<'_>) {
        let (addr, offset) = self.fuel_addr_offset(builder);
        let fuel = builder
            .ins()
            .load(ir::types::I64, ir::MemFlags::trusted(), addr, offset);
        builder.def_var(self.fuel_var, fuel);
    }

    /// Stores the fuel consumption value from `self.fuel_var` into
    /// `VMStoreContext`.
    fn fuel_save_from_var(&mut self, builder: &mut FunctionBuilder<'_>) {
        let (addr, offset) = self.fuel_addr_offset(builder);
        let fuel_consumed = builder.use_var(self.fuel_var);
        builder
            .ins()
            .store(ir::MemFlags::trusted(), fuel_consumed, addr, offset);
    }

    /// Returns the `(address, offset)` of the fuel consumption within
    /// `VMStoreContext`, used to perform loads/stores later.
    fn fuel_addr_offset(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
    ) -> (ir::Value, ir::immediates::Offset32) {
        let vmstore_ctx = self.get_vmstore_context_ptr(builder);
        (
            vmstore_ctx,
            i32::from(self.offsets.ptr.vmstore_context_fuel_consumed()).into(),
        )
    }

    /// Checks the amount of remaining, and if we've run out of fuel we call
    /// the out-of-fuel function.
    fn fuel_check(&mut self, builder: &mut FunctionBuilder) {
        self.fuel_increment_var(builder);
        let out_of_gas_block = builder.create_block();
        let continuation_block = builder.create_block();

        // Note that our fuel is encoded as adding positive values to a
        // negative number. Whenever the negative number goes positive that
        // means we ran out of fuel.
        //
        // Compare to see if our fuel is positive, and if so we ran out of gas.
        // Otherwise we can continue on like usual.
        let zero = builder.ins().iconst(ir::types::I64, 0);
        let fuel = builder.use_var(self.fuel_var);
        let cmp = builder
            .ins()
            .icmp(IntCC::SignedGreaterThanOrEqual, fuel, zero);
        builder
            .ins()
            .brif(cmp, out_of_gas_block, &[], continuation_block, &[]);
        builder.seal_block(out_of_gas_block);

        // If we ran out of gas then we call our out-of-gas intrinsic and it
        // figures out what to do. Note that this may raise a trap, or do
        // something like yield to an async runtime. In either case we don't
        // assume what happens and handle the case the intrinsic returns.
        //
        // Note that we save/reload fuel around this since the out-of-gas
        // intrinsic may alter how much fuel is in the system.
        builder.switch_to_block(out_of_gas_block);
        self.fuel_save_from_var(builder);
        let out_of_gas = self.builtin_functions.out_of_gas(builder.func);
        let vmctx = self.vmctx_val(&mut builder.cursor());
        builder.ins().call(out_of_gas, &[vmctx]);
        self.fuel_load_into_var(builder);
        builder.ins().jump(continuation_block, &[]);
        builder.seal_block(continuation_block);

        builder.switch_to_block(continuation_block);
    }

    fn epoch_function_entry(&mut self, builder: &mut FunctionBuilder<'_>) {
        debug_assert!(self.epoch_deadline_var.is_reserved_value());
        self.epoch_deadline_var = builder.declare_var(ir::types::I64);
        // Let epoch_check_full load the current deadline and call def_var

        debug_assert!(self.epoch_ptr_var.is_reserved_value());
        self.epoch_ptr_var = builder.declare_var(self.pointer_type());
        let epoch_ptr = self.epoch_ptr(builder);
        builder.def_var(self.epoch_ptr_var, epoch_ptr);

        // We must check for an epoch change when entering a
        // function. Why? Why aren't checks at loops sufficient to
        // bound runtime to O(|static program size|)?
        //
        // The reason is that one can construct a "zip-bomb-like"
        // program with exponential-in-program-size runtime, with no
        // backedges (loops), by building a tree of function calls: f0
        // calls f1 ten times, f1 calls f2 ten times, etc. E.g., nine
        // levels of this yields a billion function calls with no
        // backedges. So we can't do checks only at backedges.
        //
        // In this "call-tree" scenario, and in fact in any program
        // that uses calls as a sort of control flow to try to evade
        // backedge checks, a check at every function entry is
        // sufficient. Then, combined with checks at every backedge
        // (loop) the longest runtime between checks is bounded by the
        // straightline length of any function body.
        let continuation_block = builder.create_block();
        let cur_epoch_value = self.epoch_load_current(builder);
        self.epoch_check_full(builder, cur_epoch_value, continuation_block);
    }

    #[cfg(feature = "wmemcheck")]
    fn hook_malloc_exit(&mut self, builder: &mut FunctionBuilder, retvals: &[ir::Value]) {
        let check_malloc = self.builtin_functions.check_malloc(builder.func);
        let vmctx = self.vmctx_val(&mut builder.cursor());
        let func_args = builder
            .func
            .dfg
            .block_params(builder.func.layout.entry_block().unwrap());
        let len = if func_args.len() < 3 {
            return;
        } else {
            // If a function named `malloc` has at least one argument, we assume the
            // first argument is the requested allocation size.
            func_args[2]
        };
        let retval = if retvals.len() < 1 {
            return;
        } else {
            retvals[0]
        };
        builder.ins().call(check_malloc, &[vmctx, retval, len]);
    }

    #[cfg(feature = "wmemcheck")]
    fn hook_free_exit(&mut self, builder: &mut FunctionBuilder) {
        let check_free = self.builtin_functions.check_free(builder.func);
        let vmctx = self.vmctx_val(&mut builder.cursor());
        let func_args = builder
            .func
            .dfg
            .block_params(builder.func.layout.entry_block().unwrap());
        let ptr = if func_args.len() < 3 {
            return;
        } else {
            // If a function named `free` has at least one argument, we assume the
            // first argument is a pointer to memory.
            func_args[2]
        };
        builder.ins().call(check_free, &[vmctx, ptr]);
    }

    fn epoch_ptr(&mut self, builder: &mut FunctionBuilder<'_>) -> ir::Value {
        let vmctx = self.vmctx(builder.func);
        let pointer_type = self.pointer_type();
        let base = builder.ins().global_value(pointer_type, vmctx);
        let offset = i32::from(self.offsets.ptr.vmctx_epoch_ptr());
        let epoch_ptr = builder
            .ins()
            .load(pointer_type, ir::MemFlags::trusted(), base, offset);
        epoch_ptr
    }

    fn epoch_load_current(&mut self, builder: &mut FunctionBuilder<'_>) -> ir::Value {
        let addr = builder.use_var(self.epoch_ptr_var);
        builder.ins().load(
            ir::types::I64,
            ir::MemFlags::trusted(),
            addr,
            ir::immediates::Offset32::new(0),
        )
    }

    fn epoch_check(&mut self, builder: &mut FunctionBuilder<'_>) {
        let continuation_block = builder.create_block();

        // Load new epoch and check against the cached deadline.
        let cur_epoch_value = self.epoch_load_current(builder);
        self.epoch_check_cached(builder, cur_epoch_value, continuation_block);

        // At this point we've noticed that the epoch has exceeded our
        // cached deadline. However the real deadline may have been
        // updated (within another yield) during some function that we
        // called in the meantime, so reload the cache and check again.
        self.epoch_check_full(builder, cur_epoch_value, continuation_block);
    }

    fn epoch_check_cached(
        &mut self,
        builder: &mut FunctionBuilder,
        cur_epoch_value: ir::Value,
        continuation_block: ir::Block,
    ) {
        let new_epoch_block = builder.create_block();
        builder.set_cold_block(new_epoch_block);

        let epoch_deadline = builder.use_var(self.epoch_deadline_var);
        let cmp = builder.ins().icmp(
            IntCC::UnsignedGreaterThanOrEqual,
            cur_epoch_value,
            epoch_deadline,
        );
        builder
            .ins()
            .brif(cmp, new_epoch_block, &[], continuation_block, &[]);
        builder.seal_block(new_epoch_block);

        builder.switch_to_block(new_epoch_block);
    }

    fn epoch_check_full(
        &mut self,
        builder: &mut FunctionBuilder,
        cur_epoch_value: ir::Value,
        continuation_block: ir::Block,
    ) {
        // We keep the deadline cached in a register to speed the checks
        // in the common case (between epoch ticks) but we want to do a
        // precise check here by reloading the cache first.
        let vmstore_ctx = self.get_vmstore_context_ptr(builder);
        let deadline = builder.ins().load(
            ir::types::I64,
            ir::MemFlags::trusted(),
            vmstore_ctx,
            ir::immediates::Offset32::new(self.offsets.ptr.vmstore_context_epoch_deadline() as i32),
        );
        builder.def_var(self.epoch_deadline_var, deadline);
        self.epoch_check_cached(builder, cur_epoch_value, continuation_block);

        let new_epoch = self.builtin_functions.new_epoch(builder.func);
        let vmctx = self.vmctx_val(&mut builder.cursor());
        // new_epoch() returns the new deadline, so we don't have to
        // reload it.
        let call = builder.ins().call(new_epoch, &[vmctx]);
        let new_deadline = *builder.func.dfg.inst_results(call).first().unwrap();
        builder.def_var(self.epoch_deadline_var, new_deadline);
        builder.ins().jump(continuation_block, &[]);
        builder.seal_block(continuation_block);

        builder.switch_to_block(continuation_block);
    }

    /// Get the Memory for the given index.
    fn memory(&self, index: MemoryIndex) -> Memory {
        self.module.memories[index]
    }

    /// Get the Table for the given index.
    fn table(&self, index: TableIndex) -> Table {
        self.module.tables[index]
    }

    /// Cast the value to I64 and sign extend if necessary.
    ///
    /// Returns the value casted to I64.
    fn cast_index_to_i64(
        &self,
        pos: &mut FuncCursor<'_>,
        val: ir::Value,
        index_type: IndexType,
    ) -> ir::Value {
        match index_type {
            IndexType::I32 => pos.ins().uextend(I64, val),
            IndexType::I64 => val,
        }
    }

    /// Convert the target pointer-sized integer `val` into the memory/table's index type.
    ///
    /// For memory, `val` is holding a memory length (or the `-1` `memory.grow`-failed sentinel).
    /// For table, `val` is holding a table length.
    ///
    /// This might involve extending or truncating it depending on the memory/table's
    /// index type and the target's pointer type.
    fn convert_pointer_to_index_type(
        &self,
        mut pos: FuncCursor<'_>,
        val: ir::Value,
        index_type: IndexType,
        // When it is a memory and the memory is using single-byte pages,
        // we need to handle the truncation differently. See comments below.
        //
        // When it is a table, this should be set to false.
        single_byte_pages: bool,
    ) -> ir::Value {
        let desired_type = index_type_to_ir_type(index_type);
        let pointer_type = self.pointer_type();
        assert_eq!(pos.func.dfg.value_type(val), pointer_type);

        // The current length is of type `pointer_type` but we need to fit it
        // into `desired_type`. We are guaranteed that the result will always
        // fit, so we just need to do the right ireduce/sextend here.
        if pointer_type == desired_type {
            val
        } else if pointer_type.bits() > desired_type.bits() {
            pos.ins().ireduce(desired_type, val)
        } else {
            // We have a 64-bit memory/table on a 32-bit host -- this combo doesn't
            // really make a whole lot of sense to do from a user perspective
            // but that is neither here nor there. We want to logically do an
            // unsigned extend *except* when we are given the `-1` sentinel,
            // which we must preserve as `-1` in the wider type.
            match single_byte_pages {
                false => {
                    // In the case that we have default page sizes, we can
                    // always sign extend, since valid memory lengths (in pages)
                    // never have their sign bit set, and so if the sign bit is
                    // set then this must be the `-1` sentinel, which we want to
                    // preserve through the extension.
                    //
                    // When it comes to table, `single_byte_pages` should have always been set to false.
                    // Then we simply do a signed extension.
                    pos.ins().sextend(desired_type, val)
                }
                true => {
                    // For single-byte pages, we have to explicitly check for
                    // `-1` and choose whether to do an unsigned extension or
                    // return a larger `-1` because there are valid memory
                    // lengths (in pages) that have the sign bit set.
                    let extended = pos.ins().uextend(desired_type, val);
                    let neg_one = pos.ins().iconst(desired_type, -1);
                    let is_failure = pos.ins().icmp_imm(IntCC::Equal, val, -1);
                    pos.ins().select(is_failure, neg_one, extended)
                }
            }
        }
    }

    fn get_or_init_func_ref_table_elem(
        &mut self,
        builder: &mut FunctionBuilder,
        table_index: TableIndex,
        index: ir::Value,
        cold_blocks: bool,
    ) -> ir::Value {
        let pointer_type = self.pointer_type();
        let table_data = self.get_or_create_table(builder.func, table_index);

        // To support lazy initialization of table
        // contents, we check for a null entry here, and
        // if null, we take a slow-path that invokes a
        // libcall.
        let (table_entry_addr, flags) = table_data.prepare_table_addr(self, builder, index);
        let value = builder.ins().load(pointer_type, flags, table_entry_addr, 0);

        if !self.tunables.table_lazy_init {
            return value;
        }

        // Mask off the "initialized bit". See documentation on
        // FUNCREF_INIT_BIT in crates/environ/src/ref_bits.rs for more
        // details. Note that `FUNCREF_MASK` has type `usize` which may not be
        // appropriate for the target architecture. Right now its value is
        // always -2 so assert that part doesn't change and then thread through
        // -2 as the immediate.
        assert_eq!(FUNCREF_MASK as isize, -2);
        let value_masked = builder.ins().band_imm(value, Imm64::from(-2));

        let null_block = builder.create_block();
        let continuation_block = builder.create_block();
        if cold_blocks {
            builder.set_cold_block(null_block);
            builder.set_cold_block(continuation_block);
        }
        let result_param = builder.append_block_param(continuation_block, pointer_type);
        builder.set_cold_block(null_block);

        builder.ins().brif(
            value,
            continuation_block,
            &[value_masked.into()],
            null_block,
            &[],
        );
        builder.seal_block(null_block);

        builder.switch_to_block(null_block);
        let index_type = self.table(table_index).idx_type;
        let table_index = builder.ins().iconst(I32, table_index.index() as i64);
        let lazy_init = self
            .builtin_functions
            .table_get_lazy_init_func_ref(builder.func);
        let vmctx = self.vmctx_val(&mut builder.cursor());
        let index = self.cast_index_to_i64(&mut builder.cursor(), index, index_type);
        let call_inst = builder.ins().call(lazy_init, &[vmctx, table_index, index]);
        let returned_entry = builder.func.dfg.inst_results(call_inst)[0];
        builder
            .ins()
            .jump(continuation_block, &[returned_entry.into()]);
        builder.seal_block(continuation_block);

        builder.switch_to_block(continuation_block);
        result_param
    }

    #[cfg(feature = "wmemcheck")]
    fn check_malloc_start(&mut self, builder: &mut FunctionBuilder) {
        let malloc_start = self.builtin_functions.malloc_start(builder.func);
        let vmctx = self.vmctx_val(&mut builder.cursor());
        builder.ins().call(malloc_start, &[vmctx]);
    }

    #[cfg(feature = "wmemcheck")]
    fn check_free_start(&mut self, builder: &mut FunctionBuilder) {
        let free_start = self.builtin_functions.free_start(builder.func);
        let vmctx = self.vmctx_val(&mut builder.cursor());
        builder.ins().call(free_start, &[vmctx]);
    }

    #[cfg(feature = "wmemcheck")]
    fn current_func_name(&self, builder: &mut FunctionBuilder) -> Option<&str> {
        let func_index = match &builder.func.name {
            ir::UserFuncName::User(user) => FuncIndex::from_u32(user.index),
            _ => {
                panic!("function name not a UserFuncName::User as expected")
            }
        };
        self.translation
            .debuginfo
            .name_section
            .func_names
            .get(&func_index)
            .copied()
    }

    /// Proof-carrying code: create a memtype describing an empty
    /// runtime struct (to be updated later).
    fn create_empty_struct_memtype(&self, func: &mut ir::Function) -> ir::MemoryType {
        func.create_memory_type(ir::MemoryTypeData::Struct {
            size: 0,
            fields: vec![],
        })
    }

    /// Proof-carrying code: add a new field to a memtype used to
    /// describe a runtime struct. A memory region of type `memtype`
    /// will have a pointer at `offset` pointing to another memory
    /// region of type `pointee`. `readonly` indicates whether the
    /// PCC-checked code is expected to update this field or not.
    fn add_field_to_memtype(
        &self,
        func: &mut ir::Function,
        memtype: ir::MemoryType,
        offset: u32,
        pointee: ir::MemoryType,
        readonly: bool,
    ) {
        let ptr_size = self.pointer_type().bytes();
        match &mut func.memory_types[memtype] {
            ir::MemoryTypeData::Struct { size, fields } => {
                *size = std::cmp::max(*size, offset.checked_add(ptr_size).unwrap().into());
                fields.push(ir::MemoryTypeField {
                    ty: self.pointer_type(),
                    offset: offset.into(),
                    readonly,
                    fact: Some(ir::Fact::Mem {
                        ty: pointee,
                        min_offset: 0,
                        max_offset: 0,
                        nullable: false,
                    }),
                });

                // Sort fields by offset -- we need to do this now
                // because we may create an arbitrary number of
                // memtypes for imported memories and we don't
                // otherwise track them.
                fields.sort_by_key(|f| f.offset);
            }
            _ => panic!("Cannot add field to non-struct memtype"),
        }
    }

    /// Create an `ir::Global` that does `load(ptr + offset)` and, when PCC and
    /// memory types are enabled, adds a field to the pointer's memory type for
    /// this value we are loading.
    pub(crate) fn global_load_with_memory_type(
        &mut self,
        func: &mut ir::Function,
        ptr: ir::GlobalValue,
        offset: u32,
        flags: ir::MemFlags,
        ptr_mem_ty: Option<ir::MemoryType>,
    ) -> (ir::GlobalValue, Option<ir::MemoryType>) {
        let pointee = func.create_global_value(ir::GlobalValueData::Load {
            base: ptr,
            offset: Offset32::new(i32::try_from(offset).unwrap()),
            global_type: self.pointer_type(),
            flags,
        });

        let pointee_mem_ty = ptr_mem_ty.map(|ptr_mem_ty| {
            let pointee_mem_ty = self.create_empty_struct_memtype(func);
            self.add_field_to_memtype(func, ptr_mem_ty, offset, pointee_mem_ty, flags.readonly());
            func.global_value_facts[pointee] = Some(Fact::Mem {
                ty: pointee_mem_ty,
                min_offset: 0,
                max_offset: 0,
                nullable: false,
            });
            pointee_mem_ty
        });

        (pointee, pointee_mem_ty)
    }

    /// Like `global_load_with_memory_type` but specialized for loads out of the
    /// `vmctx`.
    pub(crate) fn global_load_from_vmctx_with_memory_type(
        &mut self,
        func: &mut ir::Function,
        offset: u32,
        flags: ir::MemFlags,
    ) -> (ir::GlobalValue, Option<ir::MemoryType>) {
        let vmctx = self.vmctx(func);
        self.global_load_with_memory_type(func, vmctx, offset, flags, self.pcc_vmctx_memtype)
    }

    /// Helper to emit a conditional trap based on `trap_cond`.
    ///
    /// This should only be used if `self.clif_instruction_traps_enabled()` is
    /// false, otherwise native CLIF instructions should be used instead.
    pub fn conditionally_trap(
        &mut self,
        builder: &mut FunctionBuilder,
        trap_cond: ir::Value,
        trap: ir::TrapCode,
    ) {
        assert!(!self.clif_instruction_traps_enabled());

        let trap_block = builder.create_block();
        builder.set_cold_block(trap_block);
        let continuation_block = builder.create_block();

        builder
            .ins()
            .brif(trap_cond, trap_block, &[], continuation_block, &[]);

        builder.seal_block(trap_block);
        builder.seal_block(continuation_block);

        builder.switch_to_block(trap_block);
        self.trap(builder, trap);
        builder.switch_to_block(continuation_block);
    }

    /// Helper used when `!self.clif_instruction_traps_enabled()` is enabled to
    /// test whether the divisor is zero.
    fn guard_zero_divisor(&mut self, builder: &mut FunctionBuilder, rhs: ir::Value) {
        if self.clif_instruction_traps_enabled() {
            return;
        }
        self.trapz(builder, rhs, ir::TrapCode::INTEGER_DIVISION_BY_ZERO);
    }

    /// Helper used when `!self.clif_instruction_traps_enabled()` is enabled to
    /// test whether a signed division operation will raise a trap.
    fn guard_signed_divide(
        &mut self,
        builder: &mut FunctionBuilder,
        lhs: ir::Value,
        rhs: ir::Value,
    ) {
        if self.clif_instruction_traps_enabled() {
            return;
        }
        self.trapz(builder, rhs, ir::TrapCode::INTEGER_DIVISION_BY_ZERO);

        let ty = builder.func.dfg.value_type(rhs);
        let minus_one = builder.ins().iconst(ty, -1);
        let rhs_is_minus_one = builder.ins().icmp(IntCC::Equal, rhs, minus_one);
        let int_min = builder.ins().iconst(
            ty,
            match ty {
                I32 => i64::from(i32::MIN),
                I64 => i64::MIN,
                _ => unreachable!(),
            },
        );
        let lhs_is_int_min = builder.ins().icmp(IntCC::Equal, lhs, int_min);
        let is_integer_overflow = builder.ins().band(rhs_is_minus_one, lhs_is_int_min);
        self.conditionally_trap(builder, is_integer_overflow, ir::TrapCode::INTEGER_OVERFLOW);
    }

    /// Helper used when `!self.clif_instruction_traps_enabled()` is enabled to
    /// guard the traps from float-to-int conversions.
    fn guard_fcvt_to_int(
        &mut self,
        builder: &mut FunctionBuilder,
        ty: ir::Type,
        val: ir::Value,
        signed: bool,
    ) {
        assert!(!self.clif_instruction_traps_enabled());
        let val_ty = builder.func.dfg.value_type(val);
        let val = if val_ty == F64 {
            val
        } else {
            builder.ins().fpromote(F64, val)
        };
        let isnan = builder.ins().fcmp(FloatCC::NotEqual, val, val);
        self.trapnz(builder, isnan, ir::TrapCode::BAD_CONVERSION_TO_INTEGER);
        let val = self.trunc_f64(builder, val);
        let (lower_bound, upper_bound) = f64_cvt_to_int_bounds(signed, ty.bits());
        let lower_bound = builder.ins().f64const(lower_bound);
        let too_small = builder
            .ins()
            .fcmp(FloatCC::LessThanOrEqual, val, lower_bound);
        self.trapnz(builder, too_small, ir::TrapCode::INTEGER_OVERFLOW);
        let upper_bound = builder.ins().f64const(upper_bound);
        let too_large = builder
            .ins()
            .fcmp(FloatCC::GreaterThanOrEqual, val, upper_bound);
        self.trapnz(builder, too_large, ir::TrapCode::INTEGER_OVERFLOW);
    }

    /// Get the `ir::Type` for a `VMSharedTypeIndex`.
    pub(crate) fn vmshared_type_index_ty(&self) -> Type {
        Type::int_with_byte_size(self.offsets.size_of_vmshared_type_index().into()).unwrap()
    }

    /// Given a `ModuleInternedTypeIndex`, emit code to get the corresponding
    /// `VMSharedTypeIndex` at runtime.
    pub(crate) fn module_interned_to_shared_ty(
        &mut self,
        pos: &mut FuncCursor,
        interned_ty: ModuleInternedTypeIndex,
    ) -> ir::Value {
        let vmctx = self.vmctx_val(pos);
        let pointer_type = self.pointer_type();
        let mem_flags = ir::MemFlags::trusted().with_readonly().with_can_move();

        // Load the base pointer of the array of `VMSharedTypeIndex`es.
        let shared_indices = pos.ins().load(
            pointer_type,
            mem_flags,
            vmctx,
            i32::from(self.offsets.ptr.vmctx_type_ids_array()),
        );

        // Calculate the offset in that array for this type's entry.
        let ty = self.vmshared_type_index_ty();
        let offset = i32::try_from(interned_ty.as_u32().checked_mul(ty.bytes()).unwrap()).unwrap();

        // Load the`VMSharedTypeIndex` that this `ModuleInternedTypeIndex` is
        // associated with at runtime from the array.
        pos.ins().load(ty, mem_flags, shared_indices, offset)
    }

    /// Load the associated `VMSharedTypeIndex` from inside a `*const VMFuncRef`.
    ///
    /// Does not check for null; just assumes that the `funcref` is a valid
    /// pointer.
    pub(crate) fn load_funcref_type_index(
        &mut self,
        pos: &mut FuncCursor,
        mem_flags: ir::MemFlags,
        funcref: ir::Value,
    ) -> ir::Value {
        let ty = self.vmshared_type_index_ty();
        pos.ins().load(
            ty,
            mem_flags,
            funcref,
            i32::from(self.offsets.ptr.vm_func_ref_type_index()),
        )
    }

    /// Does this function need a GC heap?
    pub fn needs_gc_heap(&self) -> bool {
        self.needs_gc_heap
    }

    /// Get the number of Wasm parameters for the given function.
    pub(crate) fn num_params_for_func(&self, function_index: FuncIndex) -> usize {
        let ty = self.module.functions[function_index]
            .signature
            .unwrap_module_type_index();
        self.types[ty].unwrap_func().params().len()
    }

    /// Get the number of Wasm parameters for the given function type.
    ///
    /// Panics on non-function types.
    pub(crate) fn num_params_for_function_type(&self, type_index: TypeIndex) -> usize {
        let ty = self.module.types[type_index].unwrap_module_type_index();
        self.types[ty].unwrap_func().params().len()
    }
}

#[derive(Default)]
pub(crate) struct WasmEntities {
    /// Map from a Wasm global index from this module to its implementation in
    /// the Cranelift function we are building.
    pub(crate) globals: SecondaryMap<GlobalIndex, Option<GlobalVariable>>,

    /// Map from a Wasm memory index to its `Heap` implementation in the
    /// Cranelift function we are building.
    pub(crate) memories: SecondaryMap<MemoryIndex, PackedOption<Heap>>,

    /// Map from an (interned) Wasm type index from this module to its
    /// `ir::SigRef` in the Cranelift function we are building.
    pub(crate) sig_refs: SecondaryMap<ModuleInternedTypeIndex, PackedOption<ir::SigRef>>,

    /// Map from a Wasm function index to its associated function reference in
    /// the Cranelift function we are building.
    pub(crate) func_refs: SecondaryMap<FuncIndex, PackedOption<ir::FuncRef>>,

    /// Map from a Wasm table index to its associated implementation in the
    /// Cranelift function we are building.
    pub(crate) tables: SecondaryMap<TableIndex, Option<TableData>>,
}

macro_rules! define_get_or_create_methods {
    ( $( $name:ident ( $map:ident ) : $create:ident : $key:ty => $val:ty ; )* ) => {
        $(
            pub(crate) fn $name(&mut self, func: &mut ir::Function, key: $key) -> $val {
                match self.entities.$map[key].clone().into() {
                    Some(val) => val,
                    None => {
                        let val = self.$create(func, key);
                        self.entities.$map[key] = Some(val.clone()).into();
                        val
                    }
                }
            }
        )*
    };
}

impl FuncEnvironment<'_> {
    define_get_or_create_methods! {
        get_or_create_global(globals) : make_global : GlobalIndex => GlobalVariable;
        get_or_create_heap(memories) : make_heap : MemoryIndex => Heap;
        get_or_create_interned_sig_ref(sig_refs) : make_sig_ref : ModuleInternedTypeIndex => ir::SigRef;
        get_or_create_func_ref(func_refs) : make_func_ref : FuncIndex => ir::FuncRef;
        get_or_create_table(tables) : make_table : TableIndex => TableData;
    }

    fn make_global(&mut self, func: &mut ir::Function, index: GlobalIndex) -> GlobalVariable {
        let ty = self.module.globals[index].wasm_ty;

        if ty.is_vmgcref_type() {
            // Although reference-typed globals live at the same memory location as
            // any other type of global at the same index would, getting or
            // setting them requires ref counting barriers. Therefore, we need
            // to use `GlobalVariable::Custom`, as that is the only kind of
            // `GlobalVariable` for which translation supports custom
            // access translation.
            return GlobalVariable::Custom;
        }

        let (gv, offset) = self.get_global_location(func, index);
        GlobalVariable::Memory {
            gv,
            offset: offset.into(),
            ty: super::value_type(self.isa, ty),
        }
    }

    pub(crate) fn get_or_create_sig_ref(
        &mut self,
        func: &mut ir::Function,
        ty: TypeIndex,
    ) -> ir::SigRef {
        let ty = self.module.types[ty].unwrap_module_type_index();
        self.get_or_create_interned_sig_ref(func, ty)
    }

    fn make_sig_ref(
        &mut self,
        func: &mut ir::Function,
        index: ModuleInternedTypeIndex,
    ) -> ir::SigRef {
        let wasm_func_ty = self.types[index].unwrap_func();
        let sig = crate::wasm_call_signature(self.isa, wasm_func_ty, &self.tunables);
        let sig_ref = func.import_signature(sig);
        self.sig_ref_to_ty[sig_ref] = Some(wasm_func_ty);
        sig_ref
    }

    fn make_func_ref(&mut self, func: &mut ir::Function, index: FuncIndex) -> ir::FuncRef {
        let sig = self.module.functions[index]
            .signature
            .unwrap_module_type_index();
        let wasm_func_ty = self.types[sig].unwrap_func();
        let sig = crate::wasm_call_signature(self.isa, wasm_func_ty, &self.tunables);
        let signature = func.import_signature(sig);
        self.sig_ref_to_ty[signature] = Some(wasm_func_ty);
        let name =
            ir::ExternalName::User(func.declare_imported_user_function(ir::UserExternalName {
                namespace: crate::NS_WASM_FUNC,
                index: index.as_u32(),
            }));
        func.import_function(ir::ExtFuncData {
            name,
            signature,

            // the value of this flag determines the codegen for calls to this
            // function. if this flag is `false` then absolute relocations will
            // be generated for references to the function, which requires
            // load-time relocation resolution. if this flag is set to `true`
            // then relative relocations are emitted which can be resolved at
            // object-link-time, just after all functions are compiled.
            //
            // this flag is set to `true` for functions defined in the object
            // we'll be defining in this compilation unit, or everything local
            // to the wasm module. this means that between functions in a wasm
            // module there's relative calls encoded. all calls external to a
            // wasm module (e.g. imports or libcalls) are either encoded through
            // the `vmcontext` as relative jumps (hence no relocations) or
            // they're libcalls with absolute relocations.
            colocated: self.module.defined_func_index(index).is_some()
                || self.translation.known_imported_functions[index].is_some(),
        })
    }

    fn make_heap(&mut self, func: &mut ir::Function, index: MemoryIndex) -> Heap {
        let pointer_type = self.pointer_type();
        let memory = self.module.memories[index];
        let is_shared = memory.shared;

        let (base_ptr, base_offset, current_length_offset, ptr_memtype) = {
            let vmctx = self.vmctx(func);
            if let Some(def_index) = self.module.defined_memory_index(index) {
                if is_shared {
                    // As with imported memory, the `VMMemoryDefinition` for a
                    // shared memory is stored elsewhere. We store a `*mut
                    // VMMemoryDefinition` to it and dereference that when
                    // atomically growing it.
                    let from_offset = self.offsets.vmctx_vmmemory_pointer(def_index);
                    let (memory, def_mt) = self.global_load_from_vmctx_with_memory_type(
                        func,
                        from_offset,
                        ir::MemFlags::trusted().with_readonly().with_can_move(),
                    );
                    let base_offset = i32::from(self.offsets.ptr.vmmemory_definition_base());
                    let current_length_offset =
                        i32::from(self.offsets.ptr.vmmemory_definition_current_length());
                    (memory, base_offset, current_length_offset, def_mt)
                } else {
                    let owned_index = self.module.owned_memory_index(def_index);
                    let owned_base_offset =
                        self.offsets.vmctx_vmmemory_definition_base(owned_index);
                    let owned_length_offset = self
                        .offsets
                        .vmctx_vmmemory_definition_current_length(owned_index);
                    let current_base_offset = i32::try_from(owned_base_offset).unwrap();
                    let current_length_offset = i32::try_from(owned_length_offset).unwrap();
                    (
                        vmctx,
                        current_base_offset,
                        current_length_offset,
                        self.pcc_vmctx_memtype,
                    )
                }
            } else {
                let from_offset = self.offsets.vmctx_vmmemory_import_from(index);
                let (memory, def_mt) = self.global_load_from_vmctx_with_memory_type(
                    func,
                    from_offset,
                    ir::MemFlags::trusted().with_readonly().with_can_move(),
                );
                let base_offset = i32::from(self.offsets.ptr.vmmemory_definition_base());
                let current_length_offset =
                    i32::from(self.offsets.ptr.vmmemory_definition_current_length());
                (memory, base_offset, current_length_offset, def_mt)
            }
        };

        let bound = func.create_global_value(ir::GlobalValueData::Load {
            base: base_ptr,
            offset: Offset32::new(current_length_offset),
            global_type: pointer_type,
            flags: MemFlags::trusted(),
        });

        let (base_fact, pcc_memory_type) = self.make_pcc_base_fact_and_type_for_memory(
            func,
            memory,
            base_offset,
            current_length_offset,
            ptr_memtype,
            bound,
        );

        let base = self.make_heap_base(func, memory, base_ptr, base_offset, base_fact);

        self.heaps.push(HeapData {
            base,
            bound,
            pcc_memory_type,
            memory,
        })
    }

    pub(crate) fn make_heap_base(
        &self,
        func: &mut Function,
        memory: Memory,
        ptr: ir::GlobalValue,
        offset: i32,
        fact: Option<Fact>,
    ) -> ir::GlobalValue {
        let pointer_type = self.pointer_type();

        let mut flags = ir::MemFlags::trusted().with_checked().with_can_move();
        if !memory.memory_may_move(self.tunables) {
            flags.set_readonly();
        }

        let heap_base = func.create_global_value(ir::GlobalValueData::Load {
            base: ptr,
            offset: Offset32::new(offset),
            global_type: pointer_type,
            flags,
        });
        func.global_value_facts[heap_base] = fact;
        heap_base
    }

    pub(crate) fn make_pcc_base_fact_and_type_for_memory(
        &mut self,
        func: &mut Function,
        memory: Memory,
        base_offset: i32,
        current_length_offset: i32,
        ptr_memtype: Option<ir::MemoryType>,
        heap_bound: ir::GlobalValue,
    ) -> (Option<Fact>, Option<ir::MemoryType>) {
        // If we have a declared maximum, we can make this a "static" heap, which is
        // allocated up front and never moved.
        let host_page_size_log2 = self.target_config().page_size_align_log2;
        let (base_fact, memory_type) = if !memory
            .can_elide_bounds_check(self.tunables, host_page_size_log2)
        {
            if let Some(ptr_memtype) = ptr_memtype {
                // Create a memtype representing the untyped memory region.
                let data_mt = func.create_memory_type(ir::MemoryTypeData::DynamicMemory {
                    gv: heap_bound,
                    size: self.tunables.memory_guard_size,
                });
                // This fact applies to any pointer to the start of the memory.
                let base_fact = ir::Fact::dynamic_base_ptr(data_mt);
                // This fact applies to the length.
                let length_fact = ir::Fact::global_value(
                    u16::try_from(self.isa.pointer_type().bits()).unwrap(),
                    heap_bound,
                );
                // Create a field in the vmctx for the base pointer.
                match &mut func.memory_types[ptr_memtype] {
                    ir::MemoryTypeData::Struct { size, fields } => {
                        let base_offset = u64::try_from(base_offset).unwrap();
                        fields.push(ir::MemoryTypeField {
                            offset: base_offset,
                            ty: self.isa.pointer_type(),
                            // Read-only field from the PoV of PCC checks:
                            // don't allow stores to this field. (Even if
                            // it is a dynamic memory whose base can
                            // change, that update happens inside the
                            // runtime, not in generated code.)
                            readonly: true,
                            fact: Some(base_fact.clone()),
                        });
                        let current_length_offset = u64::try_from(current_length_offset).unwrap();
                        fields.push(ir::MemoryTypeField {
                            offset: current_length_offset,
                            ty: self.isa.pointer_type(),
                            // As above, read-only; only the runtime modifies it.
                            readonly: true,
                            fact: Some(length_fact),
                        });

                        let pointer_size = u64::from(self.isa.pointer_type().bytes());
                        let fields_end = std::cmp::max(
                            base_offset + pointer_size,
                            current_length_offset + pointer_size,
                        );
                        *size = std::cmp::max(*size, fields_end);
                    }
                    _ => {
                        panic!("Bad memtype");
                    }
                }
                // Apply a fact to the base pointer.
                (Some(base_fact), Some(data_mt))
            } else {
                (None, None)
            }
        } else {
            if let Some(ptr_memtype) = ptr_memtype {
                // Create a memtype representing the untyped memory region.
                let data_mt = func.create_memory_type(ir::MemoryTypeData::Memory {
                    size: self
                        .tunables
                        .memory_reservation
                        .checked_add(self.tunables.memory_guard_size)
                        .expect("Memory plan has overflowing size plus guard"),
                });
                // This fact applies to any pointer to the start of the memory.
                let base_fact = Fact::Mem {
                    ty: data_mt,
                    min_offset: 0,
                    max_offset: 0,
                    nullable: false,
                };
                // Create a field in the vmctx for the base pointer.
                match &mut func.memory_types[ptr_memtype] {
                    ir::MemoryTypeData::Struct { size, fields } => {
                        let offset = u64::try_from(base_offset).unwrap();
                        fields.push(ir::MemoryTypeField {
                            offset,
                            ty: self.isa.pointer_type(),
                            // Read-only field from the PoV of PCC checks:
                            // don't allow stores to this field. (Even if
                            // it is a dynamic memory whose base can
                            // change, that update happens inside the
                            // runtime, not in generated code.)
                            readonly: true,
                            fact: Some(base_fact.clone()),
                        });
                        *size = std::cmp::max(
                            *size,
                            offset + u64::from(self.isa.pointer_type().bytes()),
                        );
                    }
                    _ => {
                        panic!("Bad memtype");
                    }
                }
                // Apply a fact to the base pointer.
                (Some(base_fact), Some(data_mt))
            } else {
                (None, None)
            }
        };
        (base_fact, memory_type)
    }

    fn make_table(&mut self, func: &mut ir::Function, index: TableIndex) -> TableData {
        let pointer_type = self.pointer_type();

        let (ptr, base_offset, current_elements_offset) = {
            let vmctx = self.vmctx(func);
            if let Some(def_index) = self.module.defined_table_index(index) {
                let base_offset =
                    i32::try_from(self.offsets.vmctx_vmtable_definition_base(def_index)).unwrap();
                let current_elements_offset = i32::try_from(
                    self.offsets
                        .vmctx_vmtable_definition_current_elements(def_index),
                )
                .unwrap();
                (vmctx, base_offset, current_elements_offset)
            } else {
                let from_offset = self.offsets.vmctx_vmtable_from(index);
                let table = func.create_global_value(ir::GlobalValueData::Load {
                    base: vmctx,
                    offset: Offset32::new(i32::try_from(from_offset).unwrap()),
                    global_type: pointer_type,
                    flags: MemFlags::trusted().with_readonly().with_can_move(),
                });
                let base_offset = i32::from(self.offsets.vmtable_definition_base());
                let current_elements_offset =
                    i32::from(self.offsets.vmtable_definition_current_elements());
                (table, base_offset, current_elements_offset)
            }
        };

        let table = &self.module.tables[index];
        let element_size = if table.ref_type.is_vmgcref_type() {
            // For GC-managed references, tables store `Option<VMGcRef>`s.
            ir::types::I32.bytes()
        } else {
            self.reference_type(table.ref_type.heap_type).0.bytes()
        };

        let base_gv = func.create_global_value(ir::GlobalValueData::Load {
            base: ptr,
            offset: Offset32::new(base_offset),
            global_type: pointer_type,
            flags: if Some(table.limits.min) == table.limits.max {
                // A fixed-size table can't be resized so its base address won't
                // change.
                MemFlags::trusted().with_readonly().with_can_move()
            } else {
                MemFlags::trusted()
            },
        });

        let bound = if Some(table.limits.min) == table.limits.max {
            TableSize::Static {
                bound: table.limits.min,
            }
        } else {
            TableSize::Dynamic {
                bound_gv: func.create_global_value(ir::GlobalValueData::Load {
                    base: ptr,
                    offset: Offset32::new(current_elements_offset),
                    global_type: ir::Type::int(
                        u16::from(self.offsets.size_of_vmtable_definition_current_elements()) * 8,
                    )
                    .unwrap(),
                    flags: MemFlags::trusted(),
                }),
            }
        };

        TableData {
            base_gv,
            bound,
            element_size,
        }
    }
}

struct Call<'a, 'func, 'module_env> {
    builder: &'a mut FunctionBuilder<'func>,
    env: &'a mut FuncEnvironment<'module_env>,
    tail: bool,
}

enum CheckIndirectCallTypeSignature {
    Runtime,
    StaticMatch {
        /// Whether or not the funcref may be null or if it's statically known
        /// to not be null.
        may_be_null: bool,
    },
    StaticTrap,
}

impl<'a, 'func, 'module_env> Call<'a, 'func, 'module_env> {
    /// Create a new `Call` site that will do regular, non-tail calls.
    pub fn new(
        builder: &'a mut FunctionBuilder<'func>,
        env: &'a mut FuncEnvironment<'module_env>,
    ) -> Self {
        Call {
            builder,
            env,
            tail: false,
        }
    }

    /// Create a new `Call` site that will perform tail calls.
    pub fn new_tail(
        builder: &'a mut FunctionBuilder<'func>,
        env: &'a mut FuncEnvironment<'module_env>,
    ) -> Self {
        Call {
            builder,
            env,
            tail: true,
        }
    }

    /// Do a direct call to the given callee function.
    pub fn direct_call(
        mut self,
        callee_index: FuncIndex,
        callee: ir::FuncRef,
        call_args: &[ir::Value],
    ) -> WasmResult<ir::Inst> {
        let mut real_call_args = Vec::with_capacity(call_args.len() + 2);
        let caller_vmctx = self
            .builder
            .func
            .special_param(ArgumentPurpose::VMContext)
            .unwrap();

        // Handle direct calls to locally-defined functions.
        if !self.env.module.is_imported_function(callee_index) {
            // First append the callee vmctx address, which is the same as the caller vmctx in
            // this case.
            real_call_args.push(caller_vmctx);

            // Then append the caller vmctx address.
            real_call_args.push(caller_vmctx);

            // Then append the regular call arguments.
            real_call_args.extend_from_slice(call_args);

            // Finally, make the direct call!
            return Ok(self.direct_call_inst(callee, &real_call_args));
        }

        // Handle direct calls to imported functions. We use an indirect call
        // so that we don't have to patch the code at runtime.
        let pointer_type = self.env.pointer_type();
        let sig_ref = self.builder.func.dfg.ext_funcs[callee].signature;
        let vmctx = self.env.vmctx(self.builder.func);
        let base = self.builder.ins().global_value(pointer_type, vmctx);

        let mem_flags = ir::MemFlags::trusted().with_readonly().with_can_move();

        // Load the callee address.
        let body_offset = i32::try_from(
            self.env
                .offsets
                .vmctx_vmfunction_import_wasm_call(callee_index),
        )
        .unwrap();

        // First append the callee vmctx address.
        let vmctx_offset =
            i32::try_from(self.env.offsets.vmctx_vmfunction_import_vmctx(callee_index)).unwrap();
        let callee_vmctx = self
            .builder
            .ins()
            .load(pointer_type, mem_flags, base, vmctx_offset);
        real_call_args.push(callee_vmctx);
        real_call_args.push(caller_vmctx);

        // Then append the regular call arguments.
        real_call_args.extend_from_slice(call_args);

        // If we statically know the imported function (e.g. this is a
        // component-to-component call where we statically know both components)
        // then we can actually still make a direct call (although we do have to
        // pass the callee's vmctx that we just loaded, not our own). Otherwise,
        // we really do an indirect call.
        if self.env.translation.known_imported_functions[callee_index].is_some() {
            Ok(self.direct_call_inst(callee, &real_call_args))
        } else {
            let func_addr = self
                .builder
                .ins()
                .load(pointer_type, mem_flags, base, body_offset);
            Ok(self.indirect_call_inst(sig_ref, func_addr, &real_call_args))
        }
    }

    /// Do an indirect call through the given funcref table.
    pub fn indirect_call(
        mut self,
        features: &WasmFeatures,
        table_index: TableIndex,
        ty_index: TypeIndex,
        sig_ref: ir::SigRef,
        callee: ir::Value,
        call_args: &[ir::Value],
    ) -> WasmResult<Option<ir::Inst>> {
        let (code_ptr, callee_vmctx) = match self.check_and_load_code_and_callee_vmctx(
            features,
            table_index,
            ty_index,
            callee,
            false,
        )? {
            Some(pair) => pair,
            None => return Ok(None),
        };

        self.unchecked_call_impl(sig_ref, code_ptr, callee_vmctx, call_args)
            .map(Some)
    }

    fn check_and_load_code_and_callee_vmctx(
        &mut self,
        features: &WasmFeatures,
        table_index: TableIndex,
        ty_index: TypeIndex,
        callee: ir::Value,
        cold_blocks: bool,
    ) -> WasmResult<Option<(ir::Value, ir::Value)>> {
        // Get the funcref pointer from the table.
        let funcref_ptr = self.env.get_or_init_func_ref_table_elem(
            self.builder,
            table_index,
            callee,
            cold_blocks,
        );

        // If necessary, check the signature.
        let check =
            self.check_indirect_call_type_signature(features, table_index, ty_index, funcref_ptr);

        let trap_code = match check {
            // `funcref_ptr` is checked at runtime that its type matches,
            // meaning that if code gets this far it's guaranteed to not be
            // null. That means nothing in `unchecked_call` can fail.
            CheckIndirectCallTypeSignature::Runtime => None,

            // No type check was performed on `funcref_ptr` because it's
            // statically known to have the right type. Note that whether or
            // not the function is null is not necessarily tested so far since
            // no type information was inspected.
            //
            // If the table may hold null functions, then further loads in
            // `unchecked_call` may fail. If the table only holds non-null
            // functions, though, then there's no possibility of a trap.
            CheckIndirectCallTypeSignature::StaticMatch { may_be_null } => {
                if may_be_null {
                    Some(crate::TRAP_INDIRECT_CALL_TO_NULL)
                } else {
                    None
                }
            }

            // Code has already trapped, so return nothing indicating that this
            // is now unreachable code.
            CheckIndirectCallTypeSignature::StaticTrap => return Ok(None),
        };

        Ok(Some(self.load_code_and_vmctx(funcref_ptr, trap_code)))
    }

    fn check_indirect_call_type_signature(
        &mut self,
        features: &WasmFeatures,
        table_index: TableIndex,
        ty_index: TypeIndex,
        funcref_ptr: ir::Value,
    ) -> CheckIndirectCallTypeSignature {
        let table = &self.env.module.tables[table_index];
        let sig_id_size = self.env.offsets.size_of_vmshared_type_index();
        let sig_id_type = Type::int(u16::from(sig_id_size) * 8).unwrap();

        // Test if a type check is necessary for this table. If this table is a
        // table of typed functions and that type matches `ty_index`, then
        // there's no need to perform a typecheck.
        match table.ref_type.heap_type {
            // Functions do not have a statically known type in the table, a
            // typecheck is required. Fall through to below to perform the
            // actual typecheck.
            WasmHeapType::Func => {}

            // Functions that have a statically known type are either going to
            // always succeed or always fail. Figure out by inspecting the types
            // further.
            WasmHeapType::ConcreteFunc(EngineOrModuleTypeIndex::Module(table_ty)) => {
                // If `ty_index` matches `table_ty`, then this call is
                // statically known to have the right type, so no checks are
                // necessary.
                let specified_ty = self.env.module.types[ty_index].unwrap_module_type_index();
                if specified_ty == table_ty {
                    return CheckIndirectCallTypeSignature::StaticMatch {
                        may_be_null: table.ref_type.nullable,
                    };
                }

                if features.gc() {
                    // If we are in the Wasm GC world, then we need to perform
                    // an actual subtype check at runtime. Fall through to below
                    // to do that.
                } else {
                    // Otherwise if the types don't match then either (a) this
                    // is a null pointer or (b) it's a pointer with the wrong
                    // type. Figure out which and trap here.
                    //
                    // If it's possible to have a null here then try to load the
                    // type information. If that fails due to the function being
                    // a null pointer, then this was a call to null. Otherwise
                    // if it succeeds then we know it won't match, so trap
                    // anyway.
                    if table.ref_type.nullable {
                        if self.env.clif_memory_traps_enabled() {
                            self.builder.ins().load(
                                sig_id_type,
                                ir::MemFlags::trusted()
                                    .with_readonly()
                                    .with_trap_code(Some(crate::TRAP_INDIRECT_CALL_TO_NULL)),
                                funcref_ptr,
                                i32::from(self.env.offsets.ptr.vm_func_ref_type_index()),
                            );
                        } else {
                            self.env.trapz(
                                self.builder,
                                funcref_ptr,
                                crate::TRAP_INDIRECT_CALL_TO_NULL,
                            );
                        }
                    }
                    self.env.trap(self.builder, crate::TRAP_BAD_SIGNATURE);
                    return CheckIndirectCallTypeSignature::StaticTrap;
                }
            }

            // Tables of `nofunc` can only be inhabited by null, so go ahead and
            // trap with that.
            WasmHeapType::NoFunc => {
                assert!(table.ref_type.nullable);
                self.env
                    .trap(self.builder, crate::TRAP_INDIRECT_CALL_TO_NULL);
                return CheckIndirectCallTypeSignature::StaticTrap;
            }

            WasmHeapType::Cont | WasmHeapType::ConcreteCont(_) | WasmHeapType::NoCont => todo!(), // FIXME: #10248 stack switching support.

            // Engine-indexed types don't show up until runtime and it's a Wasm
            // validation error to perform a call through a non-function table,
            // so these cases are dynamically not reachable.
            WasmHeapType::ConcreteFunc(EngineOrModuleTypeIndex::Engine(_))
            | WasmHeapType::ConcreteFunc(EngineOrModuleTypeIndex::RecGroup(_))
            | WasmHeapType::Extern
            | WasmHeapType::NoExtern
            | WasmHeapType::Any
            | WasmHeapType::Eq
            | WasmHeapType::I31
            | WasmHeapType::Array
            | WasmHeapType::ConcreteArray(_)
            | WasmHeapType::Struct
            | WasmHeapType::ConcreteStruct(_)
            | WasmHeapType::Exn
            | WasmHeapType::ConcreteExn(_)
            | WasmHeapType::NoExn
            | WasmHeapType::None => {
                unreachable!()
            }
        }

        // Load the caller's `VMSharedTypeIndex.
        let interned_ty = self.env.module.types[ty_index].unwrap_module_type_index();
        let caller_sig_id = self
            .env
            .module_interned_to_shared_ty(&mut self.builder.cursor(), interned_ty);

        // Load the callee's `VMSharedTypeIndex`.
        //
        // Note that the callee may be null in which case this load may
        // trap. If so use the `TRAP_INDIRECT_CALL_TO_NULL` trap code.
        let mut mem_flags = ir::MemFlags::trusted().with_readonly();
        if self.env.clif_memory_traps_enabled() {
            mem_flags = mem_flags.with_trap_code(Some(crate::TRAP_INDIRECT_CALL_TO_NULL));
        } else {
            self.env
                .trapz(self.builder, funcref_ptr, crate::TRAP_INDIRECT_CALL_TO_NULL);
        }
        let callee_sig_id =
            self.env
                .load_funcref_type_index(&mut self.builder.cursor(), mem_flags, funcref_ptr);

        // Check that they match: in the case of Wasm GC, this means doing a
        // full subtype check. Otherwise, we do a simple equality check.
        let matches = if features.gc() {
            #[cfg(feature = "gc")]
            {
                self.env
                    .is_subtype(self.builder, callee_sig_id, caller_sig_id)
            }
            #[cfg(not(feature = "gc"))]
            {
                unreachable!()
            }
        } else {
            self.builder
                .ins()
                .icmp(IntCC::Equal, callee_sig_id, caller_sig_id)
        };
        self.env
            .trapz(self.builder, matches, crate::TRAP_BAD_SIGNATURE);
        CheckIndirectCallTypeSignature::Runtime
    }

    /// Call a typed function reference.
    pub fn call_ref(
        mut self,
        sig_ref: ir::SigRef,
        callee: ir::Value,
        args: &[ir::Value],
    ) -> WasmResult<ir::Inst> {
        // FIXME: the wasm type system tracks enough information to know whether
        // `callee` is a null reference or not. In some situations it can be
        // statically known here that `callee` cannot be null in which case this
        // can be `None` instead. This requires feeding type information from
        // wasmparser's validator into this function, however, which is not
        // easily done at this time.
        let callee_load_trap_code = Some(crate::TRAP_NULL_REFERENCE);

        self.unchecked_call(sig_ref, callee, callee_load_trap_code, args)
    }

    /// This calls a function by reference without checking the signature.
    ///
    /// It gets the function address, sets relevant flags, and passes the
    /// special callee/caller vmctxs. It is used by both call_indirect (which
    /// checks the signature) and call_ref (which doesn't).
    fn unchecked_call(
        &mut self,
        sig_ref: ir::SigRef,
        callee: ir::Value,
        callee_load_trap_code: Option<ir::TrapCode>,
        call_args: &[ir::Value],
    ) -> WasmResult<ir::Inst> {
        let (func_addr, callee_vmctx) = self.load_code_and_vmctx(callee, callee_load_trap_code);
        self.unchecked_call_impl(sig_ref, func_addr, callee_vmctx, call_args)
    }

    fn load_code_and_vmctx(
        &mut self,
        callee: ir::Value,
        callee_load_trap_code: Option<ir::TrapCode>,
    ) -> (ir::Value, ir::Value) {
        let pointer_type = self.env.pointer_type();

        // Dereference callee pointer to get the function address.
        //
        // Note that this may trap if `callee` hasn't previously been verified
        // to be non-null. This means that this load is annotated with an
        // optional trap code provided by the caller of `unchecked_call` which
        // will handle the case where this is either already known to be
        // non-null or may trap.
        let mem_flags = ir::MemFlags::trusted().with_readonly();
        let mut callee_flags = mem_flags;
        if self.env.clif_memory_traps_enabled() {
            callee_flags = callee_flags.with_trap_code(callee_load_trap_code);
        } else {
            if let Some(trap) = callee_load_trap_code {
                self.env.trapz(self.builder, callee, trap);
            }
        }
        let func_addr = self.builder.ins().load(
            pointer_type,
            callee_flags,
            callee,
            i32::from(self.env.offsets.ptr.vm_func_ref_wasm_call()),
        );
        let callee_vmctx = self.builder.ins().load(
            pointer_type,
            mem_flags,
            callee,
            i32::from(self.env.offsets.ptr.vm_func_ref_vmctx()),
        );

        (func_addr, callee_vmctx)
    }

    /// This calls a function by reference without checking the
    /// signature, given the raw code pointer to the
    /// Wasm-calling-convention entry point and the callee vmctx.
    fn unchecked_call_impl(
        &mut self,
        sig_ref: ir::SigRef,
        func_addr: ir::Value,
        callee_vmctx: ir::Value,
        call_args: &[ir::Value],
    ) -> WasmResult<ir::Inst> {
        let mut real_call_args = Vec::with_capacity(call_args.len() + 2);
        let caller_vmctx = self
            .builder
            .func
            .special_param(ArgumentPurpose::VMContext)
            .unwrap();

        // First append the callee and caller vmctx addresses.
        real_call_args.push(callee_vmctx);
        real_call_args.push(caller_vmctx);

        // Then append the regular call arguments.
        real_call_args.extend_from_slice(call_args);

        Ok(self.indirect_call_inst(sig_ref, func_addr, &real_call_args))
    }

    fn direct_call_inst(&mut self, callee: ir::FuncRef, args: &[ir::Value]) -> ir::Inst {
        if self.tail {
            self.builder.ins().return_call(callee, args)
        } else {
            let inst = self.builder.ins().call(callee, args);
            let results: SmallVec<[_; 4]> = self
                .builder
                .func
                .dfg
                .inst_results(inst)
                .iter()
                .copied()
                .collect();
            for (i, val) in results.into_iter().enumerate() {
                if self
                    .env
                    .func_ref_result_needs_stack_map(&self.builder.func, callee, i)
                {
                    self.builder.declare_value_needs_stack_map(val);
                }
            }
            inst
        }
    }

    fn indirect_call_inst(
        &mut self,
        sig_ref: ir::SigRef,
        func_addr: ir::Value,
        args: &[ir::Value],
    ) -> ir::Inst {
        if self.tail {
            self.builder
                .ins()
                .return_call_indirect(sig_ref, func_addr, args)
        } else {
            let inst = self.builder.ins().call_indirect(sig_ref, func_addr, args);
            let results: SmallVec<[_; 4]> = self
                .builder
                .func
                .dfg
                .inst_results(inst)
                .iter()
                .copied()
                .collect();
            for (i, val) in results.into_iter().enumerate() {
                if self.env.sig_ref_result_needs_stack_map(sig_ref, i) {
                    self.builder.declare_value_needs_stack_map(val);
                }
            }
            inst
        }
    }
}

impl TypeConvert for FuncEnvironment<'_> {
    fn lookup_heap_type(&self, ty: wasmparser::UnpackedIndex) -> WasmHeapType {
        wasmtime_environ::WasmparserTypeConverter::new(self.types, |idx| {
            self.module.types[idx].unwrap_module_type_index()
        })
        .lookup_heap_type(ty)
    }

    fn lookup_type_index(&self, index: wasmparser::UnpackedIndex) -> EngineOrModuleTypeIndex {
        wasmtime_environ::WasmparserTypeConverter::new(self.types, |idx| {
            self.module.types[idx].unwrap_module_type_index()
        })
        .lookup_type_index(index)
    }
}

impl<'module_environment> TargetEnvironment for FuncEnvironment<'module_environment> {
    fn target_config(&self) -> TargetFrontendConfig {
        self.isa.frontend_config()
    }

    fn reference_type(&self, wasm_ty: WasmHeapType) -> (ir::Type, bool) {
        let ty = crate::reference_type(wasm_ty, self.pointer_type());
        let needs_stack_map = match wasm_ty.top() {
            WasmHeapTopType::Extern | WasmHeapTopType::Any | WasmHeapTopType::Exn => true,
            WasmHeapTopType::Func => false,
            WasmHeapTopType::Cont => todo!(), // FIXME: #10248 stack switching support.
        };
        (ty, needs_stack_map)
    }

    fn heap_access_spectre_mitigation(&self) -> bool {
        self.isa.flags().enable_heap_access_spectre_mitigation()
    }

    fn proof_carrying_code(&self) -> bool {
        self.isa.flags().enable_pcc()
    }

    fn tunables(&self) -> &Tunables {
        self.compiler.tunables()
    }
}

impl FuncEnvironment<'_> {
    pub fn heaps(&self) -> &PrimaryMap<Heap, HeapData> {
        &self.heaps
    }

    pub fn is_wasm_parameter(&self, _signature: &ir::Signature, index: usize) -> bool {
        // The first two parameters are the vmctx and caller vmctx. The rest are
        // the wasm parameters.
        index >= 2
    }

    pub fn param_needs_stack_map(&self, _signature: &ir::Signature, index: usize) -> bool {
        // Skip the caller and callee vmctx.
        if index < 2 {
            return false;
        }

        self.wasm_func_ty.params()[index - 2].is_vmgcref_type_and_not_i31()
    }

    pub fn sig_ref_result_needs_stack_map(&self, sig_ref: ir::SigRef, index: usize) -> bool {
        let wasm_func_ty = self.sig_ref_to_ty[sig_ref].as_ref().unwrap();
        wasm_func_ty.returns()[index].is_vmgcref_type_and_not_i31()
    }

    pub fn func_ref_result_needs_stack_map(
        &self,
        func: &ir::Function,
        func_ref: ir::FuncRef,
        index: usize,
    ) -> bool {
        let sig_ref = func.dfg.ext_funcs[func_ref].signature;
        let wasm_func_ty = self.sig_ref_to_ty[sig_ref].as_ref().unwrap();
        wasm_func_ty.returns()[index].is_vmgcref_type_and_not_i31()
    }

    pub fn translate_table_grow(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        table_index: TableIndex,
        delta: ir::Value,
        init_value: ir::Value,
    ) -> WasmResult<ir::Value> {
        let mut pos = builder.cursor();
        let table = self.table(table_index);
        let ty = table.ref_type.heap_type;
        let grow = if ty.is_vmgcref_type() {
            gc::builtins::table_grow_gc_ref(self, &mut pos.func)?
        } else {
            debug_assert_eq!(ty.top(), WasmHeapTopType::Func);
            self.builtin_functions.table_grow_func_ref(&mut pos.func)
        };

        let (table_vmctx, defined_table_index) =
            self.table_vmctx_and_defined_index(&mut pos, table_index);

        let index_type = table.idx_type;
        let delta = self.cast_index_to_i64(&mut pos, delta, index_type);
        let call_inst = pos
            .ins()
            .call(grow, &[table_vmctx, defined_table_index, delta, init_value]);
        let result = pos.func.dfg.first_result(call_inst);
        Ok(self.convert_pointer_to_index_type(builder.cursor(), result, index_type, false))
    }

    pub fn translate_table_get(
        &mut self,
        builder: &mut FunctionBuilder,
        table_index: TableIndex,
        index: ir::Value,
    ) -> WasmResult<ir::Value> {
        let table = self.module.tables[table_index];
        let table_data = self.get_or_create_table(builder.func, table_index);
        let heap_ty = table.ref_type.heap_type;
        match heap_ty.top() {
            // GC-managed types.
            WasmHeapTopType::Any | WasmHeapTopType::Extern | WasmHeapTopType::Exn => {
                let (src, flags) = table_data.prepare_table_addr(self, builder, index);
                gc::gc_compiler(self)?.translate_read_gc_reference(
                    self,
                    builder,
                    table.ref_type,
                    src,
                    flags,
                )
            }

            // Function types.
            WasmHeapTopType::Func => {
                Ok(self.get_or_init_func_ref_table_elem(builder, table_index, index, false))
            }

            // Continuation types.
            WasmHeapTopType::Cont => todo!(), // FIXME: #10248 stack switching support.
        }
    }

    pub fn translate_table_set(
        &mut self,
        builder: &mut FunctionBuilder,
        table_index: TableIndex,
        value: ir::Value,
        index: ir::Value,
    ) -> WasmResult<()> {
        let table = self.module.tables[table_index];
        let table_data = self.get_or_create_table(builder.func, table_index);
        let heap_ty = table.ref_type.heap_type;
        match heap_ty.top() {
            // GC-managed types.
            WasmHeapTopType::Any | WasmHeapTopType::Extern | WasmHeapTopType::Exn => {
                let (dst, flags) = table_data.prepare_table_addr(self, builder, index);
                gc::gc_compiler(self)?.translate_write_gc_reference(
                    self,
                    builder,
                    table.ref_type,
                    dst,
                    value,
                    flags,
                )
            }

            // Function types.
            WasmHeapTopType::Func => {
                let (elem_addr, flags) = table_data.prepare_table_addr(self, builder, index);
                // Set the "initialized bit". See doc-comment on
                // `FUNCREF_INIT_BIT` in
                // crates/environ/src/ref_bits.rs for details.
                let value_with_init_bit = if self.tunables.table_lazy_init {
                    builder
                        .ins()
                        .bor_imm(value, Imm64::from(FUNCREF_INIT_BIT as i64))
                } else {
                    value
                };
                builder
                    .ins()
                    .store(flags, value_with_init_bit, elem_addr, 0);
                Ok(())
            }

            // Continuation types.
            WasmHeapTopType::Cont => todo!(), // FIXME: #10248 stack switching support.
        }
    }

    pub fn translate_table_fill(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        table_index: TableIndex,
        dst: ir::Value,
        val: ir::Value,
        len: ir::Value,
    ) -> WasmResult<()> {
        let mut pos = builder.cursor();
        let table = self.table(table_index);
        let index_type = table.idx_type;
        let dst = self.cast_index_to_i64(&mut pos, dst, index_type);
        let len = self.cast_index_to_i64(&mut pos, len, index_type);
        let ty = table.ref_type.heap_type;
        let libcall = if ty.is_vmgcref_type() {
            gc::builtins::table_fill_gc_ref(self, &mut pos.func)?
        } else {
            debug_assert_eq!(ty.top(), WasmHeapTopType::Func);
            self.builtin_functions.table_fill_func_ref(&mut pos.func)
        };

        let (table_vmctx, table_index) = self.table_vmctx_and_defined_index(&mut pos, table_index);

        pos.ins()
            .call(libcall, &[table_vmctx, table_index, dst, val, len]);

        Ok(())
    }

    pub fn translate_ref_i31(
        &mut self,
        mut pos: FuncCursor,
        val: ir::Value,
    ) -> WasmResult<ir::Value> {
        debug_assert_eq!(pos.func.dfg.value_type(val), ir::types::I32);
        let shifted = pos.ins().ishl_imm(val, 1);
        let tagged = pos
            .ins()
            .bor_imm(shifted, i64::from(crate::I31_REF_DISCRIMINANT));
        let (ref_ty, _needs_stack_map) = self.reference_type(WasmHeapType::I31);
        debug_assert_eq!(ref_ty, ir::types::I32);
        Ok(tagged)
    }

    pub fn translate_i31_get_s(
        &mut self,
        builder: &mut FunctionBuilder,
        i31ref: ir::Value,
    ) -> WasmResult<ir::Value> {
        // TODO: If we knew we have a `(ref i31)` here, instead of maybe a `(ref
        // null i31)`, we could omit the `trapz`. But plumbing that type info
        // from `wasmparser` and through to here is a bit funky.
        self.trapz(builder, i31ref, crate::TRAP_NULL_REFERENCE);
        Ok(builder.ins().sshr_imm(i31ref, 1))
    }

    pub fn translate_i31_get_u(
        &mut self,
        builder: &mut FunctionBuilder,
        i31ref: ir::Value,
    ) -> WasmResult<ir::Value> {
        // TODO: If we knew we have a `(ref i31)` here, instead of maybe a `(ref
        // null i31)`, we could omit the `trapz`. But plumbing that type info
        // from `wasmparser` and through to here is a bit funky.
        self.trapz(builder, i31ref, crate::TRAP_NULL_REFERENCE);
        Ok(builder.ins().ushr_imm(i31ref, 1))
    }

    pub fn struct_fields_len(&mut self, struct_type_index: TypeIndex) -> WasmResult<usize> {
        let ty = self.module.types[struct_type_index].unwrap_module_type_index();
        match &self.types[ty].composite_type.inner {
            WasmCompositeInnerType::Struct(s) => Ok(s.fields.len()),
            _ => unreachable!(),
        }
    }

    pub fn translate_struct_new(
        &mut self,
        builder: &mut FunctionBuilder,
        struct_type_index: TypeIndex,
        fields: StructFieldsVec,
    ) -> WasmResult<ir::Value> {
        gc::translate_struct_new(self, builder, struct_type_index, &fields)
    }

    pub fn translate_struct_new_default(
        &mut self,
        builder: &mut FunctionBuilder,
        struct_type_index: TypeIndex,
    ) -> WasmResult<ir::Value> {
        gc::translate_struct_new_default(self, builder, struct_type_index)
    }

    pub fn translate_struct_get(
        &mut self,
        builder: &mut FunctionBuilder,
        struct_type_index: TypeIndex,
        field_index: u32,
        struct_ref: ir::Value,
        extension: Option<Extension>,
    ) -> WasmResult<ir::Value> {
        gc::translate_struct_get(
            self,
            builder,
            struct_type_index,
            field_index,
            struct_ref,
            extension,
        )
    }

    pub fn translate_struct_set(
        &mut self,
        builder: &mut FunctionBuilder,
        struct_type_index: TypeIndex,
        field_index: u32,
        struct_ref: ir::Value,
        value: ir::Value,
    ) -> WasmResult<()> {
        gc::translate_struct_set(
            self,
            builder,
            struct_type_index,
            field_index,
            struct_ref,
            value,
        )
    }

    pub fn translate_array_new(
        &mut self,
        builder: &mut FunctionBuilder,
        array_type_index: TypeIndex,
        elem: ir::Value,
        len: ir::Value,
    ) -> WasmResult<ir::Value> {
        gc::translate_array_new(self, builder, array_type_index, elem, len)
    }

    pub fn translate_array_new_default(
        &mut self,
        builder: &mut FunctionBuilder,
        array_type_index: TypeIndex,
        len: ir::Value,
    ) -> WasmResult<ir::Value> {
        gc::translate_array_new_default(self, builder, array_type_index, len)
    }

    pub fn translate_array_new_fixed(
        &mut self,
        builder: &mut FunctionBuilder,
        array_type_index: TypeIndex,
        elems: &[ir::Value],
    ) -> WasmResult<ir::Value> {
        gc::translate_array_new_fixed(self, builder, array_type_index, elems)
    }

    pub fn translate_array_new_data(
        &mut self,
        builder: &mut FunctionBuilder,
        array_type_index: TypeIndex,
        data_index: DataIndex,
        data_offset: ir::Value,
        len: ir::Value,
    ) -> WasmResult<ir::Value> {
        let libcall = gc::builtins::array_new_data(self, builder.func)?;
        let vmctx = self.vmctx_val(&mut builder.cursor());
        let interned_type_index = self.module.types[array_type_index].unwrap_module_type_index();
        let interned_type_index = builder
            .ins()
            .iconst(I32, i64::from(interned_type_index.as_u32()));
        let data_index = builder.ins().iconst(I32, i64::from(data_index.as_u32()));
        let call_inst = builder.ins().call(
            libcall,
            &[vmctx, interned_type_index, data_index, data_offset, len],
        );
        Ok(builder.func.dfg.first_result(call_inst))
    }

    pub fn translate_array_new_elem(
        &mut self,
        builder: &mut FunctionBuilder,
        array_type_index: TypeIndex,
        elem_index: ElemIndex,
        elem_offset: ir::Value,
        len: ir::Value,
    ) -> WasmResult<ir::Value> {
        let libcall = gc::builtins::array_new_elem(self, builder.func)?;
        let vmctx = self.vmctx_val(&mut builder.cursor());
        let interned_type_index = self.module.types[array_type_index].unwrap_module_type_index();
        let interned_type_index = builder
            .ins()
            .iconst(I32, i64::from(interned_type_index.as_u32()));
        let elem_index = builder.ins().iconst(I32, i64::from(elem_index.as_u32()));
        let call_inst = builder.ins().call(
            libcall,
            &[vmctx, interned_type_index, elem_index, elem_offset, len],
        );
        Ok(builder.func.dfg.first_result(call_inst))
    }

    pub fn translate_array_copy(
        &mut self,
        builder: &mut FunctionBuilder,
        _dst_array_type_index: TypeIndex,
        dst_array: ir::Value,
        dst_index: ir::Value,
        _src_array_type_index: TypeIndex,
        src_array: ir::Value,
        src_index: ir::Value,
        len: ir::Value,
    ) -> WasmResult<()> {
        let libcall = gc::builtins::array_copy(self, builder.func)?;
        let vmctx = self.vmctx_val(&mut builder.cursor());
        builder.ins().call(
            libcall,
            &[vmctx, dst_array, dst_index, src_array, src_index, len],
        );
        Ok(())
    }

    pub fn translate_array_fill(
        &mut self,
        builder: &mut FunctionBuilder,
        array_type_index: TypeIndex,
        array: ir::Value,
        index: ir::Value,
        value: ir::Value,
        len: ir::Value,
    ) -> WasmResult<()> {
        gc::translate_array_fill(self, builder, array_type_index, array, index, value, len)
    }

    pub fn translate_array_init_data(
        &mut self,
        builder: &mut FunctionBuilder,
        array_type_index: TypeIndex,
        array: ir::Value,
        dst_index: ir::Value,
        data_index: DataIndex,
        data_offset: ir::Value,
        len: ir::Value,
    ) -> WasmResult<()> {
        let libcall = gc::builtins::array_init_data(self, builder.func)?;
        let vmctx = self.vmctx_val(&mut builder.cursor());
        let interned_type_index = self.module.types[array_type_index].unwrap_module_type_index();
        let interned_type_index = builder
            .ins()
            .iconst(I32, i64::from(interned_type_index.as_u32()));
        let data_index = builder.ins().iconst(I32, i64::from(data_index.as_u32()));
        builder.ins().call(
            libcall,
            &[
                vmctx,
                interned_type_index,
                array,
                dst_index,
                data_index,
                data_offset,
                len,
            ],
        );
        Ok(())
    }

    pub fn translate_array_init_elem(
        &mut self,
        builder: &mut FunctionBuilder,
        array_type_index: TypeIndex,
        array: ir::Value,
        dst_index: ir::Value,
        elem_index: ElemIndex,
        elem_offset: ir::Value,
        len: ir::Value,
    ) -> WasmResult<()> {
        let libcall = gc::builtins::array_init_elem(self, builder.func)?;
        let vmctx = self.vmctx_val(&mut builder.cursor());
        let interned_type_index = self.module.types[array_type_index].unwrap_module_type_index();
        let interned_type_index = builder
            .ins()
            .iconst(I32, i64::from(interned_type_index.as_u32()));
        let elem_index = builder.ins().iconst(I32, i64::from(elem_index.as_u32()));
        builder.ins().call(
            libcall,
            &[
                vmctx,
                interned_type_index,
                array,
                dst_index,
                elem_index,
                elem_offset,
                len,
            ],
        );
        Ok(())
    }

    pub fn translate_array_len(
        &mut self,
        builder: &mut FunctionBuilder,
        array: ir::Value,
    ) -> WasmResult<ir::Value> {
        gc::translate_array_len(self, builder, array)
    }

    pub fn translate_array_get(
        &mut self,
        builder: &mut FunctionBuilder,
        array_type_index: TypeIndex,
        array: ir::Value,
        index: ir::Value,
        extension: Option<Extension>,
    ) -> WasmResult<ir::Value> {
        gc::translate_array_get(self, builder, array_type_index, array, index, extension)
    }

    pub fn translate_array_set(
        &mut self,
        builder: &mut FunctionBuilder,
        array_type_index: TypeIndex,
        array: ir::Value,
        index: ir::Value,
        value: ir::Value,
    ) -> WasmResult<()> {
        gc::translate_array_set(self, builder, array_type_index, array, index, value)
    }

    pub fn translate_ref_test(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        test_ty: WasmRefType,
        gc_ref: ir::Value,
        gc_ref_ty: WasmRefType,
    ) -> WasmResult<ir::Value> {
        gc::translate_ref_test(self, builder, test_ty, gc_ref, gc_ref_ty)
    }

    pub fn translate_ref_null(
        &mut self,
        mut pos: cranelift_codegen::cursor::FuncCursor,
        ht: WasmHeapType,
    ) -> WasmResult<ir::Value> {
        Ok(match ht.top() {
            WasmHeapTopType::Func => pos.ins().iconst(self.pointer_type(), 0),
            // NB: null GC references don't need to be in stack maps.
            WasmHeapTopType::Any | WasmHeapTopType::Extern | WasmHeapTopType::Exn => {
                pos.ins().iconst(types::I32, 0)
            }
            WasmHeapTopType::Cont => todo!(), // FIXME: #10248 stack switching support.
        })
    }

    pub fn translate_ref_is_null(
        &mut self,
        mut pos: cranelift_codegen::cursor::FuncCursor,
        value: ir::Value,
        ty: WasmRefType,
    ) -> WasmResult<ir::Value> {
        // If we know the type is not nullable, then we don't actually need to
        // check for null.
        if !ty.nullable {
            return Ok(pos.ins().iconst(ir::types::I32, 0));
        }

        let byte_is_null =
            pos.ins()
                .icmp_imm(cranelift_codegen::ir::condcodes::IntCC::Equal, value, 0);
        Ok(pos.ins().uextend(ir::types::I32, byte_is_null))
    }

    pub fn translate_ref_func(
        &mut self,
        mut pos: cranelift_codegen::cursor::FuncCursor<'_>,
        func_index: FuncIndex,
    ) -> WasmResult<ir::Value> {
        let func_index = pos.ins().iconst(I32, func_index.as_u32() as i64);
        let ref_func = self.builtin_functions.ref_func(&mut pos.func);
        let vmctx = self.vmctx_val(&mut pos);

        let call_inst = pos.ins().call(ref_func, &[vmctx, func_index]);
        Ok(pos.func.dfg.first_result(call_inst))
    }

    pub(crate) fn translate_global_get(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        global_index: GlobalIndex,
    ) -> WasmResult<ir::Value> {
        match self.get_or_create_global(builder.func, global_index) {
            GlobalVariable::Memory { gv, offset, ty } => {
                let addr = builder.ins().global_value(self.pointer_type(), gv);
                let mut flags = ir::MemFlags::trusted();
                // Store vector globals in little-endian format to avoid
                // byte swaps on big-endian platforms since at-rest vectors
                // should already be in little-endian format anyway.
                if ty.is_vector() {
                    flags.set_endianness(ir::Endianness::Little);
                }
                // Put globals in the "table" abstract heap category as well.
                flags.set_alias_region(Some(ir::AliasRegion::Table));
                Ok(builder.ins().load(ty, flags, addr, offset))
            }
            GlobalVariable::Custom => {
                let global_ty = self.module.globals[global_index];
                let wasm_ty = global_ty.wasm_ty;
                debug_assert!(
                    wasm_ty.is_vmgcref_type(),
                    "We only use GlobalVariable::Custom for VMGcRef types"
                );
                let WasmValType::Ref(ref_ty) = wasm_ty else {
                    unreachable!()
                };

                let (gv, offset) = self.get_global_location(builder.func, global_index);
                let gv = builder.ins().global_value(self.pointer_type(), gv);
                let src = builder.ins().iadd_imm(gv, i64::from(offset));

                gc::gc_compiler(self)?.translate_read_gc_reference(
                    self,
                    builder,
                    ref_ty,
                    src,
                    if global_ty.mutability {
                        ir::MemFlags::trusted()
                    } else {
                        ir::MemFlags::trusted().with_readonly().with_can_move()
                    },
                )
            }
        }
    }

    pub(crate) fn translate_global_set(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        global_index: GlobalIndex,
        val: ir::Value,
    ) -> WasmResult<()> {
        match self.get_or_create_global(builder.func, global_index) {
            GlobalVariable::Memory { gv, offset, ty } => {
                let addr = builder.ins().global_value(self.pointer_type(), gv);
                let mut flags = ir::MemFlags::trusted();
                // Like `global.get`, store globals in little-endian format.
                if ty.is_vector() {
                    flags.set_endianness(ir::Endianness::Little);
                }
                // Put globals in the "table" abstract heap category as well.
                flags.set_alias_region(Some(ir::AliasRegion::Table));
                debug_assert_eq!(ty, builder.func.dfg.value_type(val));
                builder.ins().store(flags, val, addr, offset);
                self.update_global(builder, global_index, val);
            }
            GlobalVariable::Custom => {
                let ty = self.module.globals[global_index].wasm_ty;
                debug_assert!(
                    ty.is_vmgcref_type(),
                    "We only use GlobalVariable::Custom for VMGcRef types"
                );
                let WasmValType::Ref(ty) = ty else {
                    unreachable!()
                };

                let (gv, offset) = self.get_global_location(builder.func, global_index);
                let gv = builder.ins().global_value(self.pointer_type(), gv);
                let src = builder.ins().iadd_imm(gv, i64::from(offset));

                gc::gc_compiler(self)?.translate_write_gc_reference(
                    self,
                    builder,
                    ty,
                    src,
                    val,
                    ir::MemFlags::trusted(),
                )?
            }
        }
        Ok(())
    }

    pub fn translate_call_indirect(
        &mut self,
        builder: &mut FunctionBuilder,
        features: &WasmFeatures,
        table_index: TableIndex,
        ty_index: TypeIndex,
        sig_ref: ir::SigRef,
        callee: ir::Value,
        call_args: &[ir::Value],
    ) -> WasmResult<Option<ir::Inst>> {
        Call::new(builder, self).indirect_call(
            features,
            table_index,
            ty_index,
            sig_ref,
            callee,
            call_args,
        )
    }

    pub fn translate_call(
        &mut self,
        builder: &mut FunctionBuilder,
        callee_index: FuncIndex,
        callee: ir::FuncRef,
        call_args: &[ir::Value],
    ) -> WasmResult<ir::Inst> {
        Call::new(builder, self).direct_call(callee_index, callee, call_args)
    }

    pub fn translate_call_ref(
        &mut self,
        builder: &mut FunctionBuilder,
        sig_ref: ir::SigRef,
        callee: ir::Value,
        call_args: &[ir::Value],
    ) -> WasmResult<ir::Inst> {
        Call::new(builder, self).call_ref(sig_ref, callee, call_args)
    }

    pub fn translate_return_call(
        &mut self,
        builder: &mut FunctionBuilder,
        callee_index: FuncIndex,
        callee: ir::FuncRef,
        call_args: &[ir::Value],
    ) -> WasmResult<()> {
        Call::new_tail(builder, self).direct_call(callee_index, callee, call_args)?;
        Ok(())
    }

    pub fn translate_return_call_indirect(
        &mut self,
        builder: &mut FunctionBuilder,
        features: &WasmFeatures,
        table_index: TableIndex,
        ty_index: TypeIndex,
        sig_ref: ir::SigRef,
        callee: ir::Value,
        call_args: &[ir::Value],
    ) -> WasmResult<()> {
        Call::new_tail(builder, self).indirect_call(
            features,
            table_index,
            ty_index,
            sig_ref,
            callee,
            call_args,
        )?;
        Ok(())
    }

    pub fn translate_return_call_ref(
        &mut self,
        builder: &mut FunctionBuilder,
        sig_ref: ir::SigRef,
        callee: ir::Value,
        call_args: &[ir::Value],
    ) -> WasmResult<()> {
        Call::new_tail(builder, self).call_ref(sig_ref, callee, call_args)?;
        Ok(())
    }

    /// Returns two `ir::Value`s, the first of which is the vmctx for the memory
    /// `index` and the second of which is the `DefinedMemoryIndex` for `index`.
    ///
    /// Handles internally whether `index` is an imported memory or not.
    fn memory_vmctx_and_defined_index(
        &mut self,
        pos: &mut FuncCursor,
        index: MemoryIndex,
    ) -> (ir::Value, ir::Value) {
        let cur_vmctx = self.vmctx_val(pos);
        match self.module.defined_memory_index(index) {
            // This is a defined memory, so the vmctx is our own and the defined
            // index is `index` here.
            Some(index) => (cur_vmctx, pos.ins().iconst(I32, i64::from(index.as_u32()))),

            // This is an imported memory, so load the vmctx/defined index from
            // the import definition itself.
            None => {
                let vmimport = self.offsets.vmctx_vmmemory_import(index);

                let vmctx = pos.ins().load(
                    self.isa.pointer_type(),
                    ir::MemFlags::trusted(),
                    cur_vmctx,
                    i32::try_from(vmimport + u32::from(self.offsets.vmmemory_import_vmctx()))
                        .unwrap(),
                );
                let index = pos.ins().load(
                    ir::types::I32,
                    ir::MemFlags::trusted(),
                    cur_vmctx,
                    i32::try_from(vmimport + u32::from(self.offsets.vmmemory_import_index()))
                        .unwrap(),
                );
                (vmctx, index)
            }
        }
    }

    /// Returns two `ir::Value`s, the first of which is the vmctx for the table
    /// `index` and the second of which is the `DefinedTableIndex` for `index`.
    ///
    /// Handles internally whether `index` is an imported table or not.
    fn table_vmctx_and_defined_index(
        &mut self,
        pos: &mut FuncCursor,
        index: TableIndex,
    ) -> (ir::Value, ir::Value) {
        // NB: the body of this method is similar to
        // `memory_vmctx_and_defined_index` above.
        let cur_vmctx = self.vmctx_val(pos);
        match self.module.defined_table_index(index) {
            Some(index) => (cur_vmctx, pos.ins().iconst(I32, i64::from(index.as_u32()))),
            None => {
                let vmimport = self.offsets.vmctx_vmtable_import(index);

                let vmctx = pos.ins().load(
                    self.isa.pointer_type(),
                    ir::MemFlags::trusted(),
                    cur_vmctx,
                    i32::try_from(vmimport + u32::from(self.offsets.vmtable_import_vmctx()))
                        .unwrap(),
                );
                let index = pos.ins().load(
                    ir::types::I32,
                    ir::MemFlags::trusted(),
                    cur_vmctx,
                    i32::try_from(vmimport + u32::from(self.offsets.vmtable_import_index()))
                        .unwrap(),
                );
                (vmctx, index)
            }
        }
    }

    pub fn translate_memory_grow(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        index: MemoryIndex,
        val: ir::Value,
    ) -> WasmResult<ir::Value> {
        let mut pos = builder.cursor();
        let memory_grow = self.builtin_functions.memory_grow(&mut pos.func);

        let (memory_vmctx, defined_memory_index) =
            self.memory_vmctx_and_defined_index(&mut pos, index);

        let index_type = self.memory(index).idx_type;
        let val = self.cast_index_to_i64(&mut pos, val, index_type);
        let call_inst = pos
            .ins()
            .call(memory_grow, &[memory_vmctx, val, defined_memory_index]);
        let result = *pos.func.dfg.inst_results(call_inst).first().unwrap();
        let single_byte_pages = match self.memory(index).page_size_log2 {
            16 => false,
            0 => true,
            _ => unreachable!("only page sizes 2**0 and 2**16 are currently valid"),
        };
        Ok(self.convert_pointer_to_index_type(
            builder.cursor(),
            result,
            index_type,
            single_byte_pages,
        ))
    }

    pub fn translate_memory_size(
        &mut self,
        mut pos: FuncCursor<'_>,
        index: MemoryIndex,
    ) -> WasmResult<ir::Value> {
        let pointer_type = self.pointer_type();
        let vmctx = self.vmctx(&mut pos.func);
        let is_shared = self.module.memories[index].shared;
        let base = pos.ins().global_value(pointer_type, vmctx);
        let current_length_in_bytes = match self.module.defined_memory_index(index) {
            Some(def_index) => {
                if is_shared {
                    let offset =
                        i32::try_from(self.offsets.vmctx_vmmemory_pointer(def_index)).unwrap();
                    let vmmemory_ptr =
                        pos.ins()
                            .load(pointer_type, ir::MemFlags::trusted(), base, offset);
                    let vmmemory_definition_offset =
                        i64::from(self.offsets.ptr.vmmemory_definition_current_length());
                    let vmmemory_definition_ptr =
                        pos.ins().iadd_imm(vmmemory_ptr, vmmemory_definition_offset);
                    // This atomic access of the
                    // `VMMemoryDefinition::current_length` is direct; no bounds
                    // check is needed. This is possible because shared memory
                    // has a static size (the maximum is always known). Shared
                    // memory is thus built with a static memory plan and no
                    // bounds-checked version of this is implemented.
                    pos.ins().atomic_load(
                        pointer_type,
                        ir::MemFlags::trusted(),
                        vmmemory_definition_ptr,
                    )
                } else {
                    let owned_index = self.module.owned_memory_index(def_index);
                    let offset = i32::try_from(
                        self.offsets
                            .vmctx_vmmemory_definition_current_length(owned_index),
                    )
                    .unwrap();
                    pos.ins()
                        .load(pointer_type, ir::MemFlags::trusted(), base, offset)
                }
            }
            None => {
                let offset = i32::try_from(self.offsets.vmctx_vmmemory_import_from(index)).unwrap();
                let vmmemory_ptr =
                    pos.ins()
                        .load(pointer_type, ir::MemFlags::trusted(), base, offset);
                if is_shared {
                    let vmmemory_definition_offset =
                        i64::from(self.offsets.ptr.vmmemory_definition_current_length());
                    let vmmemory_definition_ptr =
                        pos.ins().iadd_imm(vmmemory_ptr, vmmemory_definition_offset);
                    pos.ins().atomic_load(
                        pointer_type,
                        ir::MemFlags::trusted(),
                        vmmemory_definition_ptr,
                    )
                } else {
                    pos.ins().load(
                        pointer_type,
                        ir::MemFlags::trusted(),
                        vmmemory_ptr,
                        i32::from(self.offsets.ptr.vmmemory_definition_current_length()),
                    )
                }
            }
        };

        let page_size_log2 = i64::from(self.module.memories[index].page_size_log2);
        let current_length_in_pages = pos.ins().ushr_imm(current_length_in_bytes, page_size_log2);
        let single_byte_pages = match page_size_log2 {
            16 => false,
            0 => true,
            _ => unreachable!("only page sizes 2**0 and 2**16 are currently valid"),
        };
        Ok(self.convert_pointer_to_index_type(
            pos,
            current_length_in_pages,
            self.memory(index).idx_type,
            single_byte_pages,
        ))
    }

    pub fn translate_memory_copy(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        src_index: MemoryIndex,
        dst_index: MemoryIndex,
        dst: ir::Value,
        src: ir::Value,
        len: ir::Value,
    ) -> WasmResult<()> {
        let mut pos = builder.cursor();
        let vmctx = self.vmctx_val(&mut pos);

        let memory_copy = self.builtin_functions.memory_copy(&mut pos.func);
        let dst = self.cast_index_to_i64(&mut pos, dst, self.memory(dst_index).idx_type);
        let src = self.cast_index_to_i64(&mut pos, src, self.memory(src_index).idx_type);
        // The length is 32-bit if either memory is 32-bit, but if they're both
        // 64-bit then it's 64-bit. Our intrinsic takes a 64-bit length for
        // compatibility across all memories, so make sure that it's cast
        // correctly here (this is a bit special so no generic helper unlike for
        // `dst`/`src` above)
        let len = if index_type_to_ir_type(self.memory(dst_index).idx_type) == I64
            && index_type_to_ir_type(self.memory(src_index).idx_type) == I64
        {
            len
        } else {
            pos.ins().uextend(I64, len)
        };
        let src_index = pos.ins().iconst(I32, i64::from(src_index.as_u32()));
        let dst_index = pos.ins().iconst(I32, i64::from(dst_index.as_u32()));
        pos.ins()
            .call(memory_copy, &[vmctx, dst_index, dst, src_index, src, len]);

        Ok(())
    }

    pub fn translate_memory_fill(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        memory_index: MemoryIndex,
        dst: ir::Value,
        val: ir::Value,
        len: ir::Value,
    ) -> WasmResult<()> {
        let mut pos = builder.cursor();
        let memory_fill = self.builtin_functions.memory_fill(&mut pos.func);
        let dst = self.cast_index_to_i64(&mut pos, dst, self.memory(memory_index).idx_type);
        let len = self.cast_index_to_i64(&mut pos, len, self.memory(memory_index).idx_type);
        let (memory_vmctx, defined_memory_index) =
            self.memory_vmctx_and_defined_index(&mut pos, memory_index);

        pos.ins().call(
            memory_fill,
            &[memory_vmctx, defined_memory_index, dst, val, len],
        );

        Ok(())
    }

    pub fn translate_memory_init(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        memory_index: MemoryIndex,
        seg_index: u32,
        dst: ir::Value,
        src: ir::Value,
        len: ir::Value,
    ) -> WasmResult<()> {
        let mut pos = builder.cursor();
        let memory_init = self.builtin_functions.memory_init(&mut pos.func);

        let memory_index_arg = pos.ins().iconst(I32, memory_index.index() as i64);
        let seg_index_arg = pos.ins().iconst(I32, seg_index as i64);

        let vmctx = self.vmctx_val(&mut pos);

        let dst = self.cast_index_to_i64(&mut pos, dst, self.memory(memory_index).idx_type);

        pos.ins().call(
            memory_init,
            &[vmctx, memory_index_arg, seg_index_arg, dst, src, len],
        );

        Ok(())
    }

    pub fn translate_data_drop(&mut self, mut pos: FuncCursor, seg_index: u32) -> WasmResult<()> {
        let data_drop = self.builtin_functions.data_drop(&mut pos.func);
        let seg_index_arg = pos.ins().iconst(I32, seg_index as i64);
        let vmctx = self.vmctx_val(&mut pos);
        pos.ins().call(data_drop, &[vmctx, seg_index_arg]);
        Ok(())
    }

    pub fn translate_table_size(
        &mut self,
        pos: FuncCursor,
        table_index: TableIndex,
    ) -> WasmResult<ir::Value> {
        let table_data = self.get_or_create_table(pos.func, table_index);
        let index_type = index_type_to_ir_type(self.table(table_index).idx_type);
        Ok(table_data.bound.bound(&*self.isa, pos, index_type))
    }

    pub fn translate_table_copy(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        dst_table_index: TableIndex,
        src_table_index: TableIndex,
        dst: ir::Value,
        src: ir::Value,
        len: ir::Value,
    ) -> WasmResult<()> {
        let (table_copy, dst_table_index_arg, src_table_index_arg) =
            self.get_table_copy_func(&mut builder.func, dst_table_index, src_table_index);

        let mut pos = builder.cursor();
        let dst = self.cast_index_to_i64(&mut pos, dst, self.table(dst_table_index).idx_type);
        let src = self.cast_index_to_i64(&mut pos, src, self.table(src_table_index).idx_type);
        let len = if index_type_to_ir_type(self.table(dst_table_index).idx_type) == I64
            && index_type_to_ir_type(self.table(src_table_index).idx_type) == I64
        {
            len
        } else {
            pos.ins().uextend(I64, len)
        };
        let dst_table_index_arg = pos.ins().iconst(I32, dst_table_index_arg as i64);
        let src_table_index_arg = pos.ins().iconst(I32, src_table_index_arg as i64);
        let vmctx = self.vmctx_val(&mut pos);
        pos.ins().call(
            table_copy,
            &[
                vmctx,
                dst_table_index_arg,
                src_table_index_arg,
                dst,
                src,
                len,
            ],
        );

        Ok(())
    }

    pub fn translate_table_init(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        seg_index: u32,
        table_index: TableIndex,
        dst: ir::Value,
        src: ir::Value,
        len: ir::Value,
    ) -> WasmResult<()> {
        let mut pos = builder.cursor();
        let table_init = self.builtin_functions.table_init(&mut pos.func);
        let table_index_arg = pos.ins().iconst(I32, i64::from(table_index.as_u32()));
        let seg_index_arg = pos.ins().iconst(I32, i64::from(seg_index));
        let vmctx = self.vmctx_val(&mut pos);
        let index_type = self.table(table_index).idx_type;
        let dst = self.cast_index_to_i64(&mut pos, dst, index_type);
        let src = pos.ins().uextend(I64, src);
        let len = pos.ins().uextend(I64, len);

        pos.ins().call(
            table_init,
            &[vmctx, table_index_arg, seg_index_arg, dst, src, len],
        );

        Ok(())
    }

    pub fn translate_elem_drop(&mut self, mut pos: FuncCursor, elem_index: u32) -> WasmResult<()> {
        let elem_drop = self.builtin_functions.elem_drop(&mut pos.func);
        let elem_index_arg = pos.ins().iconst(I32, elem_index as i64);
        let vmctx = self.vmctx_val(&mut pos);
        pos.ins().call(elem_drop, &[vmctx, elem_index_arg]);
        Ok(())
    }

    pub fn translate_atomic_wait(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        memory_index: MemoryIndex,
        _heap: Heap,
        addr: ir::Value,
        expected: ir::Value,
        timeout: ir::Value,
    ) -> WasmResult<ir::Value> {
        #[cfg(feature = "threads")]
        {
            let mut pos = builder.cursor();
            let addr = self.cast_index_to_i64(&mut pos, addr, self.memory(memory_index).idx_type);
            let implied_ty = pos.func.dfg.value_type(expected);
            let wait_func = self.get_memory_atomic_wait(&mut pos.func, implied_ty);

            let (memory_vmctx, defined_memory_index) =
                self.memory_vmctx_and_defined_index(&mut pos, memory_index);

            let call_inst = pos.ins().call(
                wait_func,
                &[memory_vmctx, defined_memory_index, addr, expected, timeout],
            );
            let ret = pos.func.dfg.inst_results(call_inst)[0];
            Ok(builder.ins().ireduce(ir::types::I32, ret))
        }
        #[cfg(not(feature = "threads"))]
        {
            let _ = (builder, memory_index, addr, expected, timeout);
            Err(wasmtime_environ::WasmError::Unsupported(
                "threads support disabled at compile time".to_string(),
            ))
        }
    }

    pub fn translate_atomic_notify(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        memory_index: MemoryIndex,
        _heap: Heap,
        addr: ir::Value,
        count: ir::Value,
    ) -> WasmResult<ir::Value> {
        #[cfg(feature = "threads")]
        {
            let mut pos = builder.cursor();
            let addr = self.cast_index_to_i64(&mut pos, addr, self.memory(memory_index).idx_type);
            let atomic_notify = self.builtin_functions.memory_atomic_notify(&mut pos.func);

            let (memory_vmctx, defined_memory_index) =
                self.memory_vmctx_and_defined_index(&mut pos, memory_index);
            let call_inst = pos.ins().call(
                atomic_notify,
                &[memory_vmctx, defined_memory_index, addr, count],
            );
            let ret = pos.func.dfg.inst_results(call_inst)[0];
            Ok(builder.ins().ireduce(ir::types::I32, ret))
        }
        #[cfg(not(feature = "threads"))]
        {
            let _ = (builder, memory_index, addr, count);
            Err(wasmtime_environ::WasmError::Unsupported(
                "threads support disabled at compile time".to_string(),
            ))
        }
    }

    pub fn translate_loop_header(&mut self, builder: &mut FunctionBuilder) -> WasmResult<()> {
        // Additionally if enabled check how much fuel we have remaining to see
        // if we've run out by this point.
        if self.tunables.consume_fuel {
            self.fuel_check(builder);
        }

        // If we are performing epoch-based interruption, check to see
        // if the epoch counter has changed.
        if self.tunables.epoch_interruption {
            self.epoch_check(builder);
        }

        Ok(())
    }

    pub fn before_translate_operator(
        &mut self,
        op: &Operator,
        _operand_types: Option<&[WasmValType]>,
        builder: &mut FunctionBuilder,
        state: &FuncTranslationStacks,
    ) -> WasmResult<()> {
        if self.tunables.consume_fuel {
            self.fuel_before_op(op, builder, state.reachable());
        }
        Ok(())
    }

    pub fn after_translate_operator(
        &mut self,
        op: &Operator,
        _operand_types: Option<&[WasmValType]>,
        builder: &mut FunctionBuilder,
        state: &FuncTranslationStacks,
    ) -> WasmResult<()> {
        if self.tunables.consume_fuel && state.reachable() {
            self.fuel_after_op(op, builder);
        }
        Ok(())
    }

    pub fn before_unconditionally_trapping_memory_access(&mut self, builder: &mut FunctionBuilder) {
        if self.tunables.consume_fuel {
            self.fuel_increment_var(builder);
            self.fuel_save_from_var(builder);
        }
    }

    pub fn before_translate_function(
        &mut self,
        builder: &mut FunctionBuilder,
        _state: &FuncTranslationStacks,
    ) -> WasmResult<()> {
        // If an explicit stack limit is requested, emit one here at the start
        // of the function.
        if let Some(gv) = self.stack_limit_at_function_entry {
            let limit = builder.ins().global_value(self.pointer_type(), gv);
            let sp = builder.ins().get_stack_pointer(self.pointer_type());
            let overflow = builder.ins().icmp(IntCC::UnsignedLessThan, sp, limit);
            self.conditionally_trap(builder, overflow, ir::TrapCode::STACK_OVERFLOW);
        }

        // Additionally we initialize `fuel_var` if it will get used.
        if self.tunables.consume_fuel {
            self.fuel_function_entry(builder);
        }

        // Initialize `epoch_var` with the current epoch.
        if self.tunables.epoch_interruption {
            self.epoch_function_entry(builder);
        }

        #[cfg(feature = "wmemcheck")]
        if self.compiler.wmemcheck {
            let func_name = self.current_func_name(builder);
            if func_name == Some("malloc") {
                self.check_malloc_start(builder);
            } else if func_name == Some("free") {
                self.check_free_start(builder);
            }
        }

        Ok(())
    }

    pub fn after_translate_function(
        &mut self,
        builder: &mut FunctionBuilder,
        state: &FuncTranslationStacks,
    ) -> WasmResult<()> {
        if self.tunables.consume_fuel && state.reachable() {
            self.fuel_function_exit(builder);
        }
        Ok(())
    }

    pub fn relaxed_simd_deterministic(&self) -> bool {
        self.tunables.relaxed_simd_deterministic
    }

    pub fn has_native_fma(&self) -> bool {
        self.isa.has_native_fma()
    }

    pub fn is_x86(&self) -> bool {
        self.isa.triple().architecture == target_lexicon::Architecture::X86_64
    }

    pub fn use_x86_blendv_for_relaxed_laneselect(&self, ty: Type) -> bool {
        self.isa.has_x86_blendv_lowering(ty)
    }

    pub fn use_x86_pmulhrsw_for_relaxed_q15mul(&self) -> bool {
        self.isa.has_x86_pmulhrsw_lowering()
    }

    pub fn use_x86_pmaddubsw_for_dot(&self) -> bool {
        self.isa.has_x86_pmaddubsw_lowering()
    }

    pub fn handle_before_return(&mut self, retvals: &[ir::Value], builder: &mut FunctionBuilder) {
        #[cfg(feature = "wmemcheck")]
        if self.compiler.wmemcheck {
            let func_name = self.current_func_name(builder);
            if func_name == Some("malloc") {
                self.hook_malloc_exit(builder, retvals);
            } else if func_name == Some("free") {
                self.hook_free_exit(builder);
            }
        }
        #[cfg(not(feature = "wmemcheck"))]
        let _ = (retvals, builder);
    }

    pub fn before_load(
        &mut self,
        builder: &mut FunctionBuilder,
        val_size: u8,
        addr: ir::Value,
        offset: u64,
    ) {
        #[cfg(feature = "wmemcheck")]
        if self.compiler.wmemcheck {
            let check_load = self.builtin_functions.check_load(builder.func);
            let vmctx = self.vmctx_val(&mut builder.cursor());
            let num_bytes = builder.ins().iconst(I32, val_size as i64);
            let offset_val = builder.ins().iconst(I64, offset as i64);
            builder
                .ins()
                .call(check_load, &[vmctx, num_bytes, addr, offset_val]);
        }
        #[cfg(not(feature = "wmemcheck"))]
        let _ = (builder, val_size, addr, offset);
    }

    pub fn before_store(
        &mut self,
        builder: &mut FunctionBuilder,
        val_size: u8,
        addr: ir::Value,
        offset: u64,
    ) {
        #[cfg(feature = "wmemcheck")]
        if self.compiler.wmemcheck {
            let check_store = self.builtin_functions.check_store(builder.func);
            let vmctx = self.vmctx_val(&mut builder.cursor());
            let num_bytes = builder.ins().iconst(I32, val_size as i64);
            let offset_val = builder.ins().iconst(I64, offset as i64);
            builder
                .ins()
                .call(check_store, &[vmctx, num_bytes, addr, offset_val]);
        }
        #[cfg(not(feature = "wmemcheck"))]
        let _ = (builder, val_size, addr, offset);
    }

    pub fn update_global(
        &mut self,
        builder: &mut FunctionBuilder,
        global_index: GlobalIndex,
        value: ir::Value,
    ) {
        #[cfg(feature = "wmemcheck")]
        if self.compiler.wmemcheck {
            if global_index.index() == 0 {
                // We are making the assumption that global 0 is the auxiliary stack pointer.
                let update_stack_pointer =
                    self.builtin_functions.update_stack_pointer(builder.func);
                let vmctx = self.vmctx_val(&mut builder.cursor());
                builder.ins().call(update_stack_pointer, &[vmctx, value]);
            }
        }
        #[cfg(not(feature = "wmemcheck"))]
        let _ = (builder, global_index, value);
    }

    pub fn before_memory_grow(
        &mut self,
        builder: &mut FunctionBuilder,
        num_pages: ir::Value,
        mem_index: MemoryIndex,
    ) {
        #[cfg(feature = "wmemcheck")]
        if self.compiler.wmemcheck && mem_index.as_u32() == 0 {
            let update_mem_size = self.builtin_functions.update_mem_size(builder.func);
            let vmctx = self.vmctx_val(&mut builder.cursor());
            builder.ins().call(update_mem_size, &[vmctx, num_pages]);
        }
        #[cfg(not(feature = "wmemcheck"))]
        let _ = (builder, num_pages, mem_index);
    }

    /// If the ISA has rounding instructions, let Cranelift use them. But if
    /// not, lower to a libcall here, rather than having Cranelift do it. We
    /// can pass our libcall the vmctx pointer, which we use for stack
    /// overflow checking.
    ///
    /// This helper is generic for all rounding instructions below, both for
    /// scalar and simd types. The `clif_round` argument is the CLIF-level
    /// rounding instruction to use if the ISA has the instruction, and the
    /// `round_builtin` helper is used to determine which element-level
    /// rounding operation builtin is used. Note that this handles the case
    /// when `value` is a vector by doing an element-wise libcall invocation.
    fn isa_round(
        &mut self,
        builder: &mut FunctionBuilder,
        value: ir::Value,
        clif_round: fn(FuncInstBuilder<'_, '_>, ir::Value) -> ir::Value,
        round_builtin: fn(&mut BuiltinFunctions, &mut Function) -> ir::FuncRef,
    ) -> ir::Value {
        if self.isa.has_round() {
            return clif_round(builder.ins(), value);
        }

        let vmctx = self.vmctx_val(&mut builder.cursor());
        let round = round_builtin(&mut self.builtin_functions, builder.func);
        let round_one = |builder: &mut FunctionBuilder, value: ir::Value| {
            let call = builder.ins().call(round, &[vmctx, value]);
            *builder.func.dfg.inst_results(call).first().unwrap()
        };

        let ty = builder.func.dfg.value_type(value);
        if !ty.is_vector() {
            return round_one(builder, value);
        }

        assert_eq!(ty.bits(), 128);
        let zero = builder.func.dfg.constants.insert(V128Imm([0; 16]).into());
        let mut result = builder.ins().vconst(ty, zero);
        for i in 0..u8::try_from(ty.lane_count()).unwrap() {
            let element = builder.ins().extractlane(value, i);
            let element_rounded = round_one(builder, element);
            result = builder.ins().insertlane(result, element_rounded, i);
        }
        result
    }

    pub fn ceil_f32(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.ceil(val),
            BuiltinFunctions::ceil_f32,
        )
    }

    pub fn ceil_f64(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.ceil(val),
            BuiltinFunctions::ceil_f64,
        )
    }

    pub fn ceil_f32x4(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.ceil(val),
            BuiltinFunctions::ceil_f32,
        )
    }

    pub fn ceil_f64x2(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.ceil(val),
            BuiltinFunctions::ceil_f64,
        )
    }

    pub fn floor_f32(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.floor(val),
            BuiltinFunctions::floor_f32,
        )
    }

    pub fn floor_f64(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.floor(val),
            BuiltinFunctions::floor_f64,
        )
    }

    pub fn floor_f32x4(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.floor(val),
            BuiltinFunctions::floor_f32,
        )
    }

    pub fn floor_f64x2(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.floor(val),
            BuiltinFunctions::floor_f64,
        )
    }

    pub fn trunc_f32(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.trunc(val),
            BuiltinFunctions::trunc_f32,
        )
    }

    pub fn trunc_f64(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.trunc(val),
            BuiltinFunctions::trunc_f64,
        )
    }

    pub fn trunc_f32x4(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.trunc(val),
            BuiltinFunctions::trunc_f32,
        )
    }

    pub fn trunc_f64x2(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.trunc(val),
            BuiltinFunctions::trunc_f64,
        )
    }

    pub fn nearest_f32(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.nearest(val),
            BuiltinFunctions::nearest_f32,
        )
    }

    pub fn nearest_f64(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.nearest(val),
            BuiltinFunctions::nearest_f64,
        )
    }

    pub fn nearest_f32x4(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.nearest(val),
            BuiltinFunctions::nearest_f32,
        )
    }

    pub fn nearest_f64x2(&mut self, builder: &mut FunctionBuilder, value: ir::Value) -> ir::Value {
        self.isa_round(
            builder,
            value,
            |ins, val| ins.nearest(val),
            BuiltinFunctions::nearest_f64,
        )
    }

    pub fn swizzle(
        &mut self,
        builder: &mut FunctionBuilder,
        a: ir::Value,
        b: ir::Value,
    ) -> ir::Value {
        // On x86, swizzle would typically be compiled to `pshufb`, except
        // that that's not available on CPUs that lack SSSE3. In that case,
        // fall back to a builtin function.
        if !self.is_x86() || self.isa.has_x86_pshufb_lowering() {
            builder.ins().swizzle(a, b)
        } else {
            let swizzle = self.builtin_functions.i8x16_swizzle(builder.func);
            let vmctx = self.vmctx_val(&mut builder.cursor());
            let call = builder.ins().call(swizzle, &[vmctx, a, b]);
            *builder.func.dfg.inst_results(call).first().unwrap()
        }
    }

    pub fn relaxed_swizzle(
        &mut self,
        builder: &mut FunctionBuilder,
        a: ir::Value,
        b: ir::Value,
    ) -> ir::Value {
        // As above, fall back to a builtin if we lack SSSE3.
        if !self.is_x86() || self.isa.has_x86_pshufb_lowering() {
            if !self.is_x86() || self.relaxed_simd_deterministic() {
                builder.ins().swizzle(a, b)
            } else {
                builder.ins().x86_pshufb(a, b)
            }
        } else {
            let swizzle = self.builtin_functions.i8x16_swizzle(builder.func);
            let vmctx = self.vmctx_val(&mut builder.cursor());
            let call = builder.ins().call(swizzle, &[vmctx, a, b]);
            *builder.func.dfg.inst_results(call).first().unwrap()
        }
    }

    pub fn i8x16_shuffle(
        &mut self,
        builder: &mut FunctionBuilder,
        a: ir::Value,
        b: ir::Value,
        lanes: &[u8; 16],
    ) -> ir::Value {
        // As with swizzle, i8x16.shuffle would also commonly be implemented
        // with pshufb, so if we lack SSSE3, fall back to a builtin.
        if !self.is_x86() || self.isa.has_x86_pshufb_lowering() {
            let lanes = ConstantData::from(&lanes[..]);
            let mask = builder.func.dfg.immediates.push(lanes);
            builder.ins().shuffle(a, b, mask)
        } else {
            let lanes = builder
                .func
                .dfg
                .constants
                .insert(ConstantData::from(&lanes[..]));
            let lanes = builder.ins().vconst(I8X16, lanes);
            let i8x16_shuffle = self.builtin_functions.i8x16_shuffle(builder.func);
            let vmctx = self.vmctx_val(&mut builder.cursor());
            let call = builder.ins().call(i8x16_shuffle, &[vmctx, a, b, lanes]);
            *builder.func.dfg.inst_results(call).first().unwrap()
        }
    }

    pub fn fma_f32x4(
        &mut self,
        builder: &mut FunctionBuilder,
        a: ir::Value,
        b: ir::Value,
        c: ir::Value,
    ) -> ir::Value {
        if self.has_native_fma() {
            builder.ins().fma(a, b, c)
        } else if self.relaxed_simd_deterministic() {
            // Deterministic semantics are "fused multiply and add".
            let fma = self.builtin_functions.fma_f32x4(builder.func);
            let vmctx = self.vmctx_val(&mut builder.cursor());
            let call = builder.ins().call(fma, &[vmctx, a, b, c]);
            *builder.func.dfg.inst_results(call).first().unwrap()
        } else {
            let mul = builder.ins().fmul(a, b);
            builder.ins().fadd(mul, c)
        }
    }

    pub fn fma_f64x2(
        &mut self,
        builder: &mut FunctionBuilder,
        a: ir::Value,
        b: ir::Value,
        c: ir::Value,
    ) -> ir::Value {
        if self.has_native_fma() {
            builder.ins().fma(a, b, c)
        } else if self.relaxed_simd_deterministic() {
            // Deterministic semantics are "fused multiply and add".
            let fma = self.builtin_functions.fma_f64x2(builder.func);
            let vmctx = self.vmctx_val(&mut builder.cursor());
            let call = builder.ins().call(fma, &[vmctx, a, b, c]);
            *builder.func.dfg.inst_results(call).first().unwrap()
        } else {
            let mul = builder.ins().fmul(a, b);
            builder.ins().fadd(mul, c)
        }
    }

    pub fn isa(&self) -> &dyn TargetIsa {
        &*self.isa
    }

    pub fn trap(&mut self, builder: &mut FunctionBuilder, trap: ir::TrapCode) {
        match (
            self.clif_instruction_traps_enabled(),
            crate::clif_trap_to_env_trap(trap),
        ) {
            // If libcall traps are disabled or there's no wasmtime-defined trap
            // code for this, then emit a native trap instruction.
            (true, _) | (_, None) => {
                builder.ins().trap(trap);
            }
            // ... otherwise with libcall traps explicitly enabled and a
            // wasmtime-based trap code invoke the libcall to raise a trap and
            // pass in our trap code. Leave a debug `unreachable` in place
            // afterwards as a defense-in-depth measure.
            (false, Some(trap)) => {
                let libcall = self.builtin_functions.trap(&mut builder.func);
                let vmctx = self.vmctx_val(&mut builder.cursor());
                let trap_code = builder.ins().iconst(I8, i64::from(trap as u8));
                builder.ins().call(libcall, &[vmctx, trap_code]);
                let raise = self.builtin_functions.raise(&mut builder.func);
                builder.ins().call(raise, &[vmctx]);
                builder.ins().trap(TRAP_INTERNAL_ASSERT);
            }
        }
    }

    pub fn trapz(&mut self, builder: &mut FunctionBuilder, value: ir::Value, trap: ir::TrapCode) {
        if self.clif_instruction_traps_enabled() {
            builder.ins().trapz(value, trap);
        } else {
            let ty = builder.func.dfg.value_type(value);
            let zero = builder.ins().iconst(ty, 0);
            let cmp = builder.ins().icmp(IntCC::Equal, value, zero);
            self.conditionally_trap(builder, cmp, trap);
        }
    }

    pub fn trapnz(&mut self, builder: &mut FunctionBuilder, value: ir::Value, trap: ir::TrapCode) {
        if self.clif_instruction_traps_enabled() {
            builder.ins().trapnz(value, trap);
        } else {
            let ty = builder.func.dfg.value_type(value);
            let zero = builder.ins().iconst(ty, 0);
            let cmp = builder.ins().icmp(IntCC::NotEqual, value, zero);
            self.conditionally_trap(builder, cmp, trap);
        }
    }

    pub fn uadd_overflow_trap(
        &mut self,
        builder: &mut FunctionBuilder,
        lhs: ir::Value,
        rhs: ir::Value,
        trap: ir::TrapCode,
    ) -> ir::Value {
        if self.clif_instruction_traps_enabled() {
            builder.ins().uadd_overflow_trap(lhs, rhs, trap)
        } else {
            let (ret, overflow) = builder.ins().uadd_overflow(lhs, rhs);
            self.conditionally_trap(builder, overflow, trap);
            ret
        }
    }

    pub fn translate_sdiv(
        &mut self,
        builder: &mut FunctionBuilder,
        lhs: ir::Value,
        rhs: ir::Value,
    ) -> ir::Value {
        self.guard_signed_divide(builder, lhs, rhs);
        builder.ins().sdiv(lhs, rhs)
    }

    pub fn translate_udiv(
        &mut self,
        builder: &mut FunctionBuilder,
        lhs: ir::Value,
        rhs: ir::Value,
    ) -> ir::Value {
        self.guard_zero_divisor(builder, rhs);
        builder.ins().udiv(lhs, rhs)
    }

    pub fn translate_srem(
        &mut self,
        builder: &mut FunctionBuilder,
        lhs: ir::Value,
        rhs: ir::Value,
    ) -> ir::Value {
        self.guard_zero_divisor(builder, rhs);
        builder.ins().srem(lhs, rhs)
    }

    pub fn translate_urem(
        &mut self,
        builder: &mut FunctionBuilder,
        lhs: ir::Value,
        rhs: ir::Value,
    ) -> ir::Value {
        self.guard_zero_divisor(builder, rhs);
        builder.ins().urem(lhs, rhs)
    }

    pub fn translate_fcvt_to_sint(
        &mut self,
        builder: &mut FunctionBuilder,
        ty: ir::Type,
        val: ir::Value,
    ) -> ir::Value {
        // NB: for now avoid translating this entire instruction to CLIF and
        // just do it in a libcall.
        if !self.clif_instruction_traps_enabled() {
            self.guard_fcvt_to_int(builder, ty, val, true);
        }
        builder.ins().fcvt_to_sint(ty, val)
    }

    pub fn translate_fcvt_to_uint(
        &mut self,
        builder: &mut FunctionBuilder,
        ty: ir::Type,
        val: ir::Value,
    ) -> ir::Value {
        if !self.clif_instruction_traps_enabled() {
            self.guard_fcvt_to_int(builder, ty, val, false);
        }
        builder.ins().fcvt_to_uint(ty, val)
    }

    /// Returns whether it's acceptable to rely on traps in CLIF memory-related
    /// instructions (e.g. loads and stores).
    ///
    /// This is enabled if `signals_based_traps` is `true` since signal handlers
    /// are available, but this is additionally forcibly disabled if Pulley is
    /// being targeted since the Pulley runtime doesn't catch segfaults for
    /// itself.
    pub fn clif_memory_traps_enabled(&self) -> bool {
        self.tunables.signals_based_traps && !self.is_pulley()
    }

    /// Returns whether it's acceptable to have CLIF instructions natively trap,
    /// such as division-by-zero.
    ///
    /// This enabled if `signals_based_traps` is `true` or on Pulley
    /// unconditionally since Pulley doesn't use hardware-based traps in its
    /// runtime.
    pub fn clif_instruction_traps_enabled(&self) -> bool {
        self.tunables.signals_based_traps || self.is_pulley()
    }

    /// Returns whether loads from the null address are allowed as signals of
    /// whether to trap or not.
    pub fn load_from_zero_allowed(&self) -> bool {
        // Pulley allows loads-from-zero and otherwise this is only allowed with
        // traps + spectre mitigations.
        self.is_pulley()
            || (self.clif_memory_traps_enabled() && self.heap_access_spectre_mitigation())
    }

    /// Returns whether translation is happening for Pulley bytecode.
    pub fn is_pulley(&self) -> bool {
        self.isa.triple().is_pulley()
    }
}

// Helper function to convert an `IndexType` to an `ir::Type`.
//
// Implementing From/Into trait for `IndexType` or `ir::Type` would
// introduce an extra dependency between `wasmtime_types` and `cranelift_codegen`.
fn index_type_to_ir_type(index_type: IndexType) -> ir::Type {
    match index_type {
        IndexType::I32 => I32,
        IndexType::I64 => I64,
    }
}

/// TODO(10248) This is removed in the next stack switching PR. It stops the
/// compiler from complaining about the stack switching libcalls being dead
/// code.
#[cfg(feature = "stack-switching")]
#[allow(
    dead_code,
    reason = "Dummy function to suppress more dead code warnings"
)]
pub fn use_stack_switching_libcalls() {
    let _ = BuiltinFunctions::cont_new;
    let _ = BuiltinFunctions::table_grow_cont_obj;
    let _ = BuiltinFunctions::table_fill_cont_obj;
}
