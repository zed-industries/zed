use crate::{
    abi::{ABIOperand, ABISig, RetArea, vmctx},
    codegen::BlockSig,
    isa::reg::{Reg, RegClass, writable},
    masm::{
        AtomicWaitKind, Extend, Imm, IntCmpKind, IntScratch, LaneSelector, LoadKind,
        MacroAssembler, OperandSize, RegImm, RmwOp, SPOffset, ShiftKind, StoreKind, TrapCode,
        UNTRUSTED_FLAGS, Zero,
    },
    stack::{TypedReg, Val},
};
use anyhow::{Result, anyhow, bail, ensure};
use cranelift_codegen::{
    binemit::CodeOffset,
    ir::{RelSourceLoc, SourceLoc},
};
use smallvec::SmallVec;
use std::marker::PhantomData;
use wasmparser::{
    BinaryReader, FuncValidator, MemArg, Operator, OperatorsReader, ValidatorResources,
    VisitOperator, VisitSimdOperator,
};
use wasmtime_cranelift::{TRAP_BAD_SIGNATURE, TRAP_HEAP_MISALIGNED, TRAP_TABLE_OUT_OF_BOUNDS};
use wasmtime_environ::{
    FUNCREF_MASK, GlobalIndex, MemoryIndex, PtrSize, TableIndex, Tunables, TypeIndex, WasmHeapType,
    WasmValType,
};

mod context;
pub(crate) use context::*;
mod env;
pub use env::*;
mod call;
pub(crate) use call::*;
mod control;
pub(crate) use control::*;
mod builtin;
pub use builtin::*;
pub(crate) mod bounds;

use bounds::{Bounds, ImmOffset, Index};

mod phase;
pub(crate) use phase::*;

mod error;
pub(crate) use error::*;

/// Branch states in the compiler, enabling the derivation of the
/// reachability state.
pub(crate) trait BranchState {
    /// Whether the compiler will enter in an unreachable state after
    /// the branch is emitted.
    fn unreachable_state_after_emission() -> bool;
}

/// A conditional branch state, with a fallthrough.
pub(crate) struct ConditionalBranch;

impl BranchState for ConditionalBranch {
    fn unreachable_state_after_emission() -> bool {
        false
    }
}

/// Unconditional branch state.
pub(crate) struct UnconditionalBranch;

impl BranchState for UnconditionalBranch {
    fn unreachable_state_after_emission() -> bool {
        true
    }
}

/// Holds metadata about the source code location and the machine code emission.
/// The fields of this struct are opaque and are not interpreted in any way.
/// They serve as a mapping between source code and machine code.
#[derive(Default)]
pub(crate) struct SourceLocation {
    /// The base source location.
    pub base: Option<SourceLoc>,
    /// The current relative source code location along with its associated
    /// machine code offset.
    pub current: (CodeOffset, RelSourceLoc),
}

/// The code generation abstraction.
pub(crate) struct CodeGen<'a, 'translation: 'a, 'data: 'translation, M, P>
where
    M: MacroAssembler,
    P: CodeGenPhase,
{
    /// The ABI-specific representation of the function signature, excluding results.
    pub sig: ABISig,

    /// The code generation context.
    pub context: CodeGenContext<'a, P>,

    /// A reference to the function compilation environment.
    pub env: FuncEnv<'a, 'translation, 'data, M::Ptr>,

    /// The MacroAssembler.
    pub masm: &'a mut M,

    /// Stack frames for control flow.
    // NB The 64 is set arbitrarily, we can adjust it as
    // we see fit.
    pub control_frames: SmallVec<[ControlStackFrame; 64]>,

    /// Information about the source code location.
    pub source_location: SourceLocation,

    /// Compilation settings for code generation.
    pub tunables: &'a Tunables,

    /// Local counter to track fuel consumption.
    pub fuel_consumed: i64,
    phase: PhantomData<P>,
}

impl<'a, 'translation, 'data, M> CodeGen<'a, 'translation, 'data, M, Prologue>
where
    M: MacroAssembler,
{
    pub fn new(
        tunables: &'a Tunables,
        masm: &'a mut M,
        context: CodeGenContext<'a, Prologue>,
        env: FuncEnv<'a, 'translation, 'data, M::Ptr>,
        sig: ABISig,
    ) -> CodeGen<'a, 'translation, 'data, M, Prologue> {
        Self {
            sig,
            context,
            masm,
            env,
            tunables,
            source_location: Default::default(),
            control_frames: Default::default(),
            // Empty functions should consume at least 1 fuel unit.
            fuel_consumed: 1,
            phase: PhantomData,
        }
    }

    /// Code generation prologue.
    pub fn emit_prologue(mut self) -> Result<CodeGen<'a, 'translation, 'data, M, Emission>> {
        let vmctx = self
            .sig
            .params()
            .first()
            .ok_or_else(|| anyhow!(CodeGenError::vmcontext_arg_expected()))?
            .unwrap_reg();

        self.masm.start_source_loc(Default::default())?;
        // We need to use the vmctx parameter before pinning it for stack checking.
        self.masm.prologue(vmctx)?;

        // Pin the `VMContext` pointer.
        self.masm.mov(
            writable!(vmctx!(M)),
            vmctx.into(),
            self.env.ptr_type().try_into()?,
        )?;

        self.masm.reserve_stack(self.context.frame.locals_size)?;
        self.spill_register_arguments()?;

        let defined_locals_range = &self.context.frame.defined_locals_range;
        self.masm.zero_mem_range(defined_locals_range.as_range())?;

        // Save the results base parameter register into its slot.

        if self.sig.params.has_retptr() {
            match self.sig.params.unwrap_results_area_operand() {
                ABIOperand::Reg { ty, reg, .. } => {
                    let results_base_slot = self.context.frame.results_base_slot.as_ref().unwrap();
                    ensure!(
                        results_base_slot.addressed_from_sp(),
                        CodeGenError::sp_addressing_expected(),
                    );
                    let addr = self.masm.local_address(results_base_slot)?;
                    self.masm.store((*reg).into(), addr, (*ty).try_into()?)?;
                }
                // The result base parameter is a stack parameter, addressed
                // from FP.
                _ => {}
            }
        }

        self.masm.end_source_loc()?;

        Ok(CodeGen {
            sig: self.sig,
            context: self.context.for_emission(),
            masm: self.masm,
            env: self.env,
            tunables: self.tunables,
            source_location: self.source_location,
            control_frames: self.control_frames,
            fuel_consumed: self.fuel_consumed,
            phase: PhantomData,
        })
    }

    fn spill_register_arguments(&mut self) -> Result<()> {
        use WasmValType::*;
        for (operand, slot) in self
            .sig
            .params_without_retptr()
            .iter()
            .zip(self.context.frame.locals())
        {
            match (operand, slot) {
                (ABIOperand::Reg { ty, reg, .. }, slot) => {
                    let addr = self.masm.local_address(slot)?;
                    match &ty {
                        I32 | I64 | F32 | F64 | V128 => {
                            self.masm.store((*reg).into(), addr, (*ty).try_into()?)?;
                        }
                        Ref(rt) => match rt.heap_type {
                            WasmHeapType::Func | WasmHeapType::Extern => {
                                self.masm.store_ptr(*reg, addr)?;
                            }
                            _ => bail!(CodeGenError::unsupported_wasm_type()),
                        },
                    }
                }
                // Skip non-register arguments
                _ => {}
            }
        }
        Ok(())
    }
}

impl<'a, 'translation, 'data, M> CodeGen<'a, 'translation, 'data, M, Emission>
where
    M: MacroAssembler,
{
    /// Emit the function body to machine code.
    pub fn emit(
        &mut self,
        body: BinaryReader<'a>,
        validator: &mut FuncValidator<ValidatorResources>,
    ) -> Result<()> {
        self.emit_body(body, validator)
            .and_then(|_| self.emit_end())?;

        Ok(())
    }

    /// Pops a control frame from the control frame stack.
    pub fn pop_control_frame(&mut self) -> Result<ControlStackFrame> {
        self.control_frames
            .pop()
            .ok_or_else(|| anyhow!(CodeGenError::control_frame_expected()))
    }

    /// Derives a [RelSourceLoc] from a [SourceLoc].
    pub fn source_loc_from(&mut self, loc: SourceLoc) -> RelSourceLoc {
        if self.source_location.base.is_none() && !loc.is_default() {
            self.source_location.base = Some(loc);
        }

        RelSourceLoc::from_base_offset(self.source_location.base.unwrap_or_default(), loc)
    }

    /// The following two helpers, handle else or end instructions when the
    /// compiler has entered into an unreachable code state. These instructions
    /// must be observed to determine if the reachability state should be
    /// restored.
    ///
    /// When the compiler is in an unreachable state, all the other instructions
    /// are not visited.
    pub fn handle_unreachable_else(&mut self) -> Result<()> {
        let frame = self
            .control_frames
            .last_mut()
            .ok_or_else(|| CodeGenError::control_frame_expected())?;
        ensure!(frame.is_if(), CodeGenError::if_control_frame_expected());
        if frame.is_next_sequence_reachable() {
            // We entered an unreachable state when compiling the
            // if-then branch, but if the `if` was reachable at
            // entry, the if-else branch will be reachable.
            self.context.reachable = true;
            frame.ensure_stack_state(self.masm, &mut self.context)?;
            frame.bind_else(self.masm, &mut self.context)?;
        }
        Ok(())
    }

    pub fn handle_unreachable_end(&mut self) -> Result<()> {
        let mut frame = self.pop_control_frame()?;
        // We just popped the outermost block.
        let is_outermost = self.control_frames.len() == 0;

        if frame.is_next_sequence_reachable() {
            self.context.reachable = true;
            frame.ensure_stack_state(self.masm, &mut self.context)?;
            frame.bind_end(self.masm, &mut self.context)
        } else if is_outermost {
            // If we reach the end of the function in an unreachable
            // state, perform the necessary cleanup to leave the stack
            // and SP in the expected state.  The compiler can enter
            // in this state through an infinite loop.
            frame.ensure_stack_state(self.masm, &mut self.context)
        } else {
            Ok(())
        }
    }

    fn emit_body(
        &mut self,
        body: BinaryReader<'a>,
        validator: &mut FuncValidator<ValidatorResources>,
    ) -> Result<()> {
        self.maybe_emit_fuel_check()?;

        self.maybe_emit_epoch_check()?;

        // Once we have emitted the epilogue and reserved stack space for the locals, we push the
        // base control flow block.
        self.control_frames.push(ControlStackFrame::block(
            BlockSig::from_sig(self.sig.clone()),
            self.masm,
            &mut self.context,
        )?);

        // Set the return area of the results *after* initializing the block. In
        // the function body block case, we'll treat the results as any other
        // case, addressed from the stack pointer, and when ending the function
        // the return area will be set to the return pointer.
        if self.sig.params.has_retptr() {
            self.sig
                .results
                .set_ret_area(RetArea::slot(self.context.frame.results_base_slot.unwrap()));
        }

        let mut ops = OperatorsReader::new(body);
        while !ops.eof() {
            let offset = ops.original_position();
            ops.visit_operator(&mut ValidateThenVisit(
                validator.simd_visitor(offset),
                self,
                offset,
            ))??;
        }
        ops.finish()?;
        return Ok(());

        struct ValidateThenVisit<'a, T, U>(T, &'a mut U, usize);

        macro_rules! validate_then_visit {
            ($( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident $ann:tt)*) => {
                $(
                    fn $visit(&mut self $($(,$arg: $argty)*)?) -> Self::Output {
                        self.0.$visit($($($arg.clone()),*)?)?;
                        let op = Operator::$op $({ $($arg: $arg.clone()),* })?;
                        if self.1.visit(&op) {
                            self.1.before_visit_op(&op, self.2)?;
                            let res = self.1.$visit($($($arg),*)?)?;
                            self.1.after_visit_op()?;
                            Ok(res)
                        } else {
                            Ok(())
                        }
                    }
                )*
            };
        }

        fn visit_op_when_unreachable(op: &Operator) -> bool {
            use Operator::*;
            match op {
                If { .. } | Block { .. } | Loop { .. } | Else | End => true,
                _ => false,
            }
        }

        /// Trait to handle hooks that must happen before and after visiting an
        /// operator.
        trait VisitorHooks {
            /// Hook prior to visiting an operator.
            fn before_visit_op(&mut self, operator: &Operator, offset: usize) -> Result<()>;
            /// Hook after visiting an operator.
            fn after_visit_op(&mut self) -> Result<()>;

            /// Returns `true` if the operator will be visited.
            ///
            /// Operators will be visited if the following invariants are met:
            /// * The compiler is in a reachable state.
            /// * The compiler is in an unreachable state, but the current
            ///   operator is a control flow operator. These operators need to be
            ///   visited in order to keep the control stack frames balanced and
            ///   to determine if the reachability state must be restored.
            fn visit(&self, op: &Operator) -> bool;
        }

        impl<'a, 'translation, 'data, M: MacroAssembler> VisitorHooks
            for CodeGen<'a, 'translation, 'data, M, Emission>
        {
            fn visit(&self, op: &Operator) -> bool {
                self.context.reachable || visit_op_when_unreachable(op)
            }

            fn before_visit_op(&mut self, operator: &Operator, offset: usize) -> Result<()> {
                // Handle source location mapping.
                self.source_location_before_visit_op(offset)?;

                // Handle fuel.
                if self.tunables.consume_fuel {
                    self.fuel_before_visit_op(operator)?;
                }
                Ok(())
            }

            fn after_visit_op(&mut self) -> Result<()> {
                // Handle source code location mapping.
                self.source_location_after_visit_op()
            }
        }

        impl<'a, T, U> VisitOperator<'a> for ValidateThenVisit<'_, T, U>
        where
            T: VisitSimdOperator<'a, Output = wasmparser::Result<()>>,
            U: VisitSimdOperator<'a, Output = Result<()>> + VisitorHooks,
        {
            type Output = U::Output;

            fn simd_visitor(
                &mut self,
            ) -> Option<&mut dyn VisitSimdOperator<'a, Output = Self::Output>>
            where
                T:,
            {
                Some(self)
            }

            wasmparser::for_each_visit_operator!(validate_then_visit);
        }

        impl<'a, T, U> VisitSimdOperator<'a> for ValidateThenVisit<'_, T, U>
        where
            T: VisitSimdOperator<'a, Output = wasmparser::Result<()>>,
            U: VisitSimdOperator<'a, Output = Result<()>> + VisitorHooks,
        {
            wasmparser::for_each_visit_simd_operator!(validate_then_visit);
        }
    }

    /// Emits a a series of instructions that will type check a function reference call.
    pub fn emit_typecheck_funcref(
        &mut self,
        funcref_ptr: Reg,
        type_index: TypeIndex,
    ) -> Result<()> {
        let ptr_size: OperandSize = self.env.ptr_type().try_into()?;
        let sig_index_bytes = self.env.vmoffsets.size_of_vmshared_type_index();
        let sig_size = OperandSize::from_bytes(sig_index_bytes);
        let sig_index = self.env.translation.module.types[type_index].unwrap_module_type_index();
        let sig_offset = sig_index
            .as_u32()
            .checked_mul(sig_index_bytes.into())
            .unwrap();
        let signatures_base_offset = self.env.vmoffsets.ptr.vmctx_type_ids_array();
        let funcref_sig_offset = self.env.vmoffsets.ptr.vm_func_ref_type_index();
        // Get the caller id.
        let caller_id = self.context.any_gpr(self.masm)?;

        self.masm.with_scratch::<IntScratch, _>(|masm, scratch| {
            // Load the signatures address into the scratch register.
            masm.load(
                masm.address_at_vmctx(signatures_base_offset.into())?,
                scratch.writable(),
                ptr_size,
            )?;

            masm.load(
                masm.address_at_reg(scratch.inner(), sig_offset)?,
                writable!(caller_id),
                sig_size,
            )
        })?;

        let callee_id = self.context.any_gpr(self.masm)?;
        self.masm.load(
            self.masm
                .address_at_reg(funcref_ptr, funcref_sig_offset.into())?,
            writable!(callee_id),
            sig_size,
        )?;

        // Typecheck.
        self.masm
            .cmp(caller_id, callee_id.into(), OperandSize::S32)?;
        self.masm.trapif(IntCmpKind::Ne, TRAP_BAD_SIGNATURE)?;
        self.context.free_reg(callee_id);
        self.context.free_reg(caller_id);
        anyhow::Ok(())
    }

    /// Emit the usual function end instruction sequence.
    fn emit_end(&mut self) -> Result<()> {
        // The implicit body block is treated a normal block (it pushes results
        // to the stack); so when reaching the end, we pop them taking as
        // reference the current function's signature.
        let base = SPOffset::from_u32(self.context.frame.locals_size);
        self.masm.start_source_loc(Default::default())?;
        if self.context.reachable {
            ControlStackFrame::pop_abi_results_impl(
                &mut self.sig.results,
                &mut self.context,
                self.masm,
                |results, _, _| Ok(results.ret_area().copied()),
            )?;
        } else {
            // If we reach the end of the function in an unreachable code state,
            // simply truncate to the expected values.
            // The compiler could enter this state through an infinite loop.
            self.context.truncate_stack_to(0)?;
            self.masm.reset_stack_pointer(base)?;
        }
        ensure!(
            self.context.stack.len() == 0,
            CodeGenError::unexpected_value_in_value_stack()
        );
        self.masm.free_stack(self.context.frame.locals_size)?;
        self.masm.epilogue()?;
        self.masm.end_source_loc()?;
        Ok(())
    }

    /// Pops the value at the stack top and assigns it to the local at
    /// the given index, returning the typed register holding the
    /// source value.
    pub fn emit_set_local(&mut self, index: u32) -> Result<TypedReg> {
        // Materialize any references to the same local index that are in the
        // value stack by spilling.
        if self.context.stack.contains_latent_local(index) {
            self.context.spill(self.masm)?;
        }
        let src = self.context.pop_to_reg(self.masm, None)?;
        // Need to get address of local after `pop_to_reg` since `pop_to_reg`
        // will pop the machine stack causing an incorrect address to be
        // calculated.
        let (ty, addr) = self.context.frame.get_local_address(index, self.masm)?;
        self.masm
            .store(RegImm::reg(src.reg), addr, ty.try_into()?)?;

        Ok(src)
    }

    /// Loads the address of the given global.
    pub fn emit_get_global_addr(&mut self, index: GlobalIndex) -> Result<(WasmValType, Reg, u32)> {
        let data = self.env.resolve_global(index);

        if data.imported {
            let global_base = self.masm.address_at_reg(vmctx!(M), data.offset)?;
            let dst = self.context.any_gpr(self.masm)?;
            self.masm.load_ptr(global_base, writable!(dst))?;
            Ok((data.ty, dst, 0))
        } else {
            Ok((data.ty, vmctx!(M), data.offset))
        }
    }

    pub fn emit_lazy_init_funcref(&mut self, table_index: TableIndex) -> Result<()> {
        assert!(self.tunables.table_lazy_init, "unsupported eager init");
        let table_data = self.env.resolve_table_data(table_index);
        let ptr_type = self.env.ptr_type();
        let builtin = self
            .env
            .builtins
            .table_get_lazy_init_func_ref::<M::ABI, M::Ptr>()?;

        // Request the builtin's  result register and use it to hold the table
        // element value. We preemptively spill and request this register to
        // avoid conflict at the control flow merge below. Requesting the result
        // register is safe since we know ahead-of-time the builtin's signature.
        self.context.spill(self.masm)?;
        let elem_value: Reg = self.context.reg(
            builtin.sig().results.unwrap_singleton().unwrap_reg(),
            self.masm,
        )?;

        let index = self.context.pop_to_reg(self.masm, None)?;
        let base = self.context.any_gpr(self.masm)?;

        let elem_addr = self.emit_compute_table_elem_addr(index.into(), base, &table_data)?;
        self.masm.load_ptr(elem_addr, writable!(elem_value))?;
        // Free the register used as base, once we have loaded the element
        // address into the element value register.
        self.context.free_reg(base);

        let (defined, cont) = (self.masm.get_label()?, self.masm.get_label()?);

        // Push the built-in arguments to the stack.
        self.context
            .stack
            .extend([table_index.as_u32().try_into().unwrap(), index.into()]);

        self.masm.branch(
            IntCmpKind::Ne,
            elem_value,
            elem_value.into(),
            defined,
            ptr_type.try_into()?,
        )?;
        // Free the element value register.
        // This is safe since the FnCall::emit call below, will ensure
        // that the result register is placed on the value stack.
        self.context.free_reg(elem_value);
        FnCall::emit::<M>(
            &mut self.env,
            self.masm,
            &mut self.context,
            Callee::Builtin(builtin.clone()),
        )?;

        // We know the signature of the libcall in this case, so we assert that there's
        // one element in the stack and that it's  the ABI signature's result register.
        let top = self
            .context
            .stack
            .peek()
            .ok_or_else(|| CodeGenError::missing_values_in_stack())?;
        let top = top.unwrap_reg();
        ensure!(
            top.reg == elem_value,
            CodeGenError::table_element_value_expected()
        );
        self.masm.jmp(cont)?;

        // In the defined case, mask the funcref address in place, by peeking into the
        // last element of the value stack, which was pushed by the `indirect` function
        // call above.
        //
        // Note that `FUNCREF_MASK` as type `usize` but here we want a 64-bit
        // value so assert its actual value and then use a `-2` literal.
        self.masm.bind(defined)?;
        assert_eq!(FUNCREF_MASK as isize, -2);
        let imm = RegImm::i64(-2);
        let dst = top.into();
        self.masm
            .and(writable!(dst), dst, imm, top.ty.try_into()?)?;

        self.masm.bind(cont)
    }

    /// Emits a series of instructions to bounds check and calculate the address
    /// of the given WebAssembly memory.
    /// This function returns a register containing the requested address.
    ///
    /// In essence, when computing the heap address for a WebAssembly load or
    /// store instruction the objective is to ensure that such access is safe,
    /// but also to perform the least amount of checks, and rely on the system to
    /// detect illegal memory accesses where applicable.
    ///
    /// Winch follows almost the same principles as Cranelift when it comes to
    /// bounds checks, for a more detailed explanation refer to
    /// prepare_addr in wasmtime-cranelift.
    ///
    /// Winch implementation differs in that, it defaults to the general case
    /// for dynamic heaps rather than optimizing for doing the least amount of
    /// work possible at runtime, this is done to align with Winch's principle
    /// of doing the least amount of work possible at compile time. For static
    /// heaps, Winch does a bit more of work, given that some of the cases that
    /// are checked against, can benefit compilation times, like for example,
    /// detecting an out of bounds access at compile time.
    pub fn emit_compute_heap_address(
        &mut self,
        memarg: &MemArg,
        access_size: OperandSize,
    ) -> Result<Option<Reg>> {
        let ptr_size: OperandSize = self.env.ptr_type().try_into()?;
        let enable_spectre_mitigation = self.env.heap_access_spectre_mitigation();
        let add_offset_and_access_size = |offset: ImmOffset, access_size: OperandSize| {
            (access_size.bytes() as u64) + (offset.as_u32() as u64)
        };

        let memory_index = MemoryIndex::from_u32(memarg.memory);
        let heap = self.env.resolve_heap(memory_index);
        let index = Index::from_typed_reg(self.context.pop_to_reg(self.masm, None)?);

        let offset = bounds::ensure_index_and_offset(
            self.masm,
            index,
            memarg.offset,
            heap.index_type().try_into()?,
        )?;
        let offset_with_access_size = add_offset_and_access_size(offset, access_size);

        let can_elide_bounds_check = heap
            .memory
            .can_elide_bounds_check(self.tunables, self.env.page_size_log2);

        let addr = if offset_with_access_size > heap.memory.maximum_byte_size().unwrap_or(u64::MAX)
        {
            // Detect at compile time if the access is out of bounds.
            // Doing so will put the compiler in an unreachable code state,
            // optimizing the work that the compiler has to do until the
            // reachability is restored or when reaching the end of the
            // function.

            self.emit_fuel_increment()?;
            self.masm.trap(TrapCode::HEAP_OUT_OF_BOUNDS)?;
            self.context.reachable = false;
            None
        } else if !can_elide_bounds_check {
            // Account for the general case for bounds-checked memories. The
            // access is out of bounds if:
            // * index + offset + access_size overflows
            //   OR
            // * index + offset + access_size > bound
            let bounds = bounds::load_dynamic_heap_bounds::<_>(
                &mut self.context,
                self.masm,
                &heap,
                ptr_size,
            )?;

            let index_reg = index.as_typed_reg().reg;
            // Allocate a temporary register to hold
            //      index + offset + access_size
            //  which will serve as the check condition.
            let index_offset_and_access_size = self.context.any_gpr(self.masm)?;

            // Move the value of the index to the
            // index_offset_and_access_size register to perform the overflow
            // check to avoid clobbering the initial index value.
            //
            // We derive size of the operation from the heap type since:
            //
            // * This is the first assignment to the
            // `index_offset_and_access_size` register
            //
            // * The memory64 proposal specifies that the index is bound to
            // the heap type instead of hardcoding it to 32-bits (i32).
            self.masm.mov(
                writable!(index_offset_and_access_size),
                index_reg.into(),
                heap.index_type().try_into()?,
            )?;
            // Perform
            // index = index + offset + access_size, trapping if the
            // addition overflows.
            //
            // We use the target's pointer size rather than depending on the heap
            // type since we want to check for overflow; even though the
            // offset and access size are guaranteed to be bounded by the heap
            // type, when added, if used with the wrong operand size, their
            // result could be clamped, resulting in an erroneous overflow
            // check.
            self.masm.checked_uadd(
                writable!(index_offset_and_access_size),
                index_offset_and_access_size,
                Imm::i64(offset_with_access_size as i64),
                ptr_size,
                TrapCode::HEAP_OUT_OF_BOUNDS,
            )?;

            let addr = bounds::load_heap_addr_checked(
                self.masm,
                &mut self.context,
                ptr_size,
                &heap,
                enable_spectre_mitigation,
                bounds,
                index,
                offset,
                |masm, bounds, _| {
                    let bounds_reg = bounds.as_typed_reg().reg;
                    masm.cmp(
                        index_offset_and_access_size,
                        bounds_reg.into(),
                        // We use the pointer size to keep the bounds
                        // comparison consistent with the result of the
                        // overflow check above.
                        ptr_size,
                    )?;
                    Ok(IntCmpKind::GtU)
                },
            )?;
            self.context.free_reg(bounds.as_typed_reg().reg);
            self.context.free_reg(index_offset_and_access_size);
            Some(addr)

        // Account for the case in which we can completely elide the bounds
        // checks.
        //
        // This case, makes use of the fact that if a memory access uses
        // a 32-bit index, then we be certain that
        //
        //      index <= u32::MAX
        //
        // Therefore if any 32-bit index access occurs in the region
        // represented by
        //
        //      bound + guard_size - (offset + access_size)
        //
        // We are certain that it's in bounds or that the underlying virtual
        // memory subsystem will report an illegal access at runtime.
        //
        // Note:
        //
        // * bound - (offset + access_size) cannot wrap, because it's checked
        // in the condition above.
        // * bound + heap.offset_guard_size is guaranteed to not overflow if
        // the heap configuration is correct, given that it's address must
        // fit in 64-bits.
        // * If the heap type is 32-bits, the offset is at most u32::MAX, so
        // no  adjustment is needed as part of
        // [bounds::ensure_index_and_offset].
        } else if u64::from(u32::MAX)
            <= self.tunables.memory_reservation + self.tunables.memory_guard_size
                - offset_with_access_size
        {
            assert!(can_elide_bounds_check);
            assert!(heap.index_type() == WasmValType::I32);
            let addr = self.context.any_gpr(self.masm)?;
            bounds::load_heap_addr_unchecked(self.masm, &heap, index, offset, addr, ptr_size)?;
            Some(addr)

        // Account for the all remaining cases, aka. The access is out
        // of bounds if:
        //
        // index > bound - (offset + access_size)
        //
        // bound - (offset + access_size) cannot wrap, because we already
        // checked that (offset + access_size) > bound, above.
        } else {
            assert!(can_elide_bounds_check);
            assert!(heap.index_type() == WasmValType::I32);
            let bounds = Bounds::from_u64(self.tunables.memory_reservation);
            let addr = bounds::load_heap_addr_checked(
                self.masm,
                &mut self.context,
                ptr_size,
                &heap,
                enable_spectre_mitigation,
                bounds,
                index,
                offset,
                |masm, bounds, index| {
                    let adjusted_bounds = bounds.as_u64() - offset_with_access_size;
                    let index_reg = index.as_typed_reg().reg;
                    masm.cmp(
                        index_reg,
                        RegImm::i64(adjusted_bounds as i64),
                        // Similar to the dynamic heap case, even though the
                        // offset and access size are bound through the heap
                        // type, when added they can overflow, resulting in
                        // an erroneous comparison, therfore we rely on the
                        // target pointer size.
                        ptr_size,
                    )?;
                    Ok(IntCmpKind::GtU)
                },
            )?;
            Some(addr)
        };

        self.context.free_reg(index.as_typed_reg().reg);
        Ok(addr)
    }

    /// Emit checks to ensure that the address at `memarg` is correctly aligned for `size`.
    fn emit_check_align(&mut self, memarg: &MemArg, size: OperandSize) -> Result<()> {
        if size.bytes() > 1 {
            // Peek addr from top of the stack by popping and pushing.
            let addr = *self
                .context
                .stack
                .peek()
                .ok_or_else(|| CodeGenError::missing_values_in_stack())?;
            let tmp = self.context.any_gpr(self.masm)?;
            self.context.move_val_to_reg(&addr, tmp, self.masm)?;

            if memarg.offset != 0 {
                self.masm.add(
                    writable!(tmp),
                    tmp,
                    RegImm::Imm(Imm::I64(memarg.offset)),
                    size,
                )?;
            }

            self.masm.and(
                writable!(tmp),
                tmp,
                RegImm::Imm(Imm::I32(size.bytes() - 1)),
                size,
            )?;

            self.masm.cmp(tmp, RegImm::Imm(Imm::i64(0)), size)?;
            self.masm.trapif(IntCmpKind::Ne, TRAP_HEAP_MISALIGNED)?;
            self.context.free_reg(tmp);
        }

        Ok(())
    }

    pub fn emit_compute_heap_address_align_checked(
        &mut self,
        memarg: &MemArg,
        access_size: OperandSize,
    ) -> Result<Option<Reg>> {
        self.emit_check_align(memarg, access_size)?;
        self.emit_compute_heap_address(memarg, access_size)
    }

    /// Emit a WebAssembly load.
    pub fn emit_wasm_load(
        &mut self,
        arg: &MemArg,
        target_type: WasmValType,
        kind: LoadKind,
    ) -> Result<()> {
        let emit_load = |this: &mut Self, dst, addr, kind| -> Result<()> {
            let src = this.masm.address_at_reg(addr, 0)?;
            this.masm.wasm_load(src, writable!(dst), kind)?;
            this.context
                .stack
                .push(TypedReg::new(target_type, dst).into());
            this.context.free_reg(addr);
            Ok(())
        };

        // Ensure that the destination register is not allocated if
        // `emit_compute_heap_address` does not return an address.
        match kind {
            LoadKind::VectorLane(_) => {
                // Destination vector register is at the top of the stack and
                // `emit_compute_heap_address` expects an integer register
                // containing the address to load to be at the top of the stack.
                let dst = self.context.pop_to_reg(self.masm, None)?;
                let addr = self.emit_compute_heap_address(&arg, kind.derive_operand_size())?;
                if let Some(addr) = addr {
                    emit_load(self, dst.reg, addr, kind)?;
                } else {
                    self.context.free_reg(dst);
                }
            }
            _ => {
                let maybe_addr = match kind {
                    LoadKind::Atomic(_, _) => self.emit_compute_heap_address_align_checked(
                        &arg,
                        kind.derive_operand_size(),
                    )?,
                    _ => self.emit_compute_heap_address(&arg, kind.derive_operand_size())?,
                };

                if let Some(addr) = maybe_addr {
                    let dst = match target_type {
                        WasmValType::I32 | WasmValType::I64 => self.context.any_gpr(self.masm)?,
                        WasmValType::F32 | WasmValType::F64 => self.context.any_fpr(self.masm)?,
                        WasmValType::V128 => self.context.reg_for_type(target_type, self.masm)?,
                        _ => bail!(CodeGenError::unsupported_wasm_type()),
                    };

                    emit_load(self, dst, addr, kind)?;
                }
            }
        }

        Ok(())
    }

    /// Emit a WebAssembly store.
    pub fn emit_wasm_store(&mut self, arg: &MemArg, kind: StoreKind) -> Result<()> {
        let src = self.context.pop_to_reg(self.masm, None)?;

        let maybe_addr = match kind {
            StoreKind::Atomic(size) => self.emit_compute_heap_address_align_checked(&arg, size)?,
            StoreKind::Operand(size) | StoreKind::VectorLane(LaneSelector { size, .. }) => {
                self.emit_compute_heap_address(&arg, size)?
            }
        };

        if let Some(addr) = maybe_addr {
            self.masm
                .wasm_store(src.reg, self.masm.address_at_reg(addr, 0)?, kind)?;

            self.context.free_reg(addr);
        }
        self.context.free_reg(src);

        Ok(())
    }

    /// Loads the address of the table element at a given index. Returns the
    /// address of the table element using the provided register as base.
    pub fn emit_compute_table_elem_addr(
        &mut self,
        index: Reg,
        base: Reg,
        table_data: &TableData,
    ) -> Result<M::Address> {
        let bound = self.context.any_gpr(self.masm)?;
        let tmp = self.context.any_gpr(self.masm)?;
        let ptr_size: OperandSize = self.env.ptr_type().try_into()?;

        if let Some(offset) = table_data.import_from {
            // If the table data declares a particular offset base,
            // load the address into a register to further use it as
            // the table address.
            self.masm
                .load_ptr(self.masm.address_at_vmctx(offset)?, writable!(base))?;
        } else {
            // Else, simply move the vmctx register into the addr register as
            // the base to calculate the table address.
            self.masm.mov(writable!(base), vmctx!(M).into(), ptr_size)?;
        };

        // OOB check.
        let bound_addr = self
            .masm
            .address_at_reg(base, table_data.current_elems_offset)?;
        let bound_size = table_data.current_elements_size;
        self.masm.load(bound_addr, writable!(bound), bound_size)?;
        self.masm.cmp(index, bound.into(), bound_size)?;
        self.masm
            .trapif(IntCmpKind::GeU, TRAP_TABLE_OUT_OF_BOUNDS)?;

        // Move the index into the scratch register to calculate the table
        // element address.
        // Moving the value of the index register to the scratch register
        // also avoids overwriting the context of the index register.
        self.masm.with_scratch::<IntScratch, _>(|masm, scratch| {
            masm.mov(scratch.writable(), index.into(), bound_size)?;
            masm.mul(
                scratch.writable(),
                scratch.inner(),
                RegImm::i32(table_data.element_size.bytes() as i32),
                table_data.element_size,
            )?;
            masm.load_ptr(
                masm.address_at_reg(base, table_data.offset)?,
                writable!(base),
            )?;
            // Copy the value of the table base into a temporary register
            // so that we can use it later in case of a misspeculation.
            masm.mov(writable!(tmp), base.into(), ptr_size)?;
            // Calculate the address of the table element.
            masm.add(writable!(base), base, scratch.inner().into(), ptr_size)
        })?;
        if self.env.table_access_spectre_mitigation() {
            // Perform a bounds check and override the value of the
            // table element address in case the index is out of bounds.
            self.masm.cmp(index, bound.into(), OperandSize::S32)?;
            self.masm
                .cmov(writable!(base), tmp, IntCmpKind::GeU, ptr_size)?;
        }
        self.context.free_reg(bound);
        self.context.free_reg(tmp);
        self.masm.address_at_reg(base, 0)
    }

    /// Retrieves the size of the table, pushing the result to the value stack.
    pub fn emit_compute_table_size(&mut self, table_data: &TableData) -> Result<()> {
        let size = self.context.any_gpr(self.masm)?;
        let ptr_size: OperandSize = self.env.ptr_type().try_into()?;

        self.masm.with_scratch::<IntScratch, _>(|masm, scratch| {
            if let Some(offset) = table_data.import_from {
                masm.load_ptr(masm.address_at_vmctx(offset)?, scratch.writable())?;
            } else {
                masm.mov(scratch.writable(), vmctx!(M).into(), ptr_size)?;
            };

            let size_addr =
                masm.address_at_reg(scratch.inner(), table_data.current_elems_offset)?;
            masm.load(size_addr, writable!(size), table_data.current_elements_size)
        })?;

        let dst = TypedReg::new(table_data.index_type(), size);
        self.context.stack.push(dst.into());
        Ok(())
    }

    /// Retrieves the size of the memory, pushing the result to the value stack.
    pub fn emit_compute_memory_size(&mut self, heap_data: &HeapData) -> Result<()> {
        let size_reg = self.context.any_gpr(self.masm)?;

        self.masm.with_scratch::<IntScratch, _>(|masm, scratch| {
            let base = if let Some(offset) = heap_data.import_from {
                masm.load_ptr(masm.address_at_vmctx(offset)?, scratch.writable())?;
                scratch.inner()
            } else {
                vmctx!(M)
            };

            let size_addr = masm.address_at_reg(base, heap_data.current_length_offset)?;
            masm.load_ptr(size_addr, writable!(size_reg))
        })?;
        // Emit a shift to get the size in pages rather than in bytes.
        let dst = TypedReg::new(heap_data.index_type(), size_reg);
        let pow = heap_data.memory.page_size_log2;
        self.masm.shift_ir(
            writable!(dst.reg),
            Imm::i32(pow as i32),
            dst.into(),
            ShiftKind::ShrU,
            heap_data.index_type().try_into()?,
        )?;
        self.context.stack.push(dst.into());
        Ok(())
    }

    /// Checks if fuel consumption is enabled and emits a series of instructions
    /// that check the current fuel usage by performing a zero-comparison with
    /// the number of units stored in `VMStoreContext`.
    pub fn maybe_emit_fuel_check(&mut self) -> Result<()> {
        if !self.tunables.consume_fuel {
            return Ok(());
        }

        self.emit_fuel_increment()?;
        let out_of_fuel = self.env.builtins.out_of_gas::<M::ABI, M::Ptr>()?;
        let fuel_reg = self.context.without::<Result<Reg>, M, _>(
            &out_of_fuel.sig().regs,
            self.masm,
            |cx, masm| cx.any_gpr(masm),
        )??;

        self.emit_load_fuel_consumed(fuel_reg)?;

        // The  continuation label if the current fuel is under the limit.
        let continuation = self.masm.get_label()?;

        // Spill locals and registers to avoid conflicts at the out-of-fuel
        // control flow merge.
        self.context.spill(self.masm)?;
        // Fuel is stored as a negative i64, so if the number is less than zero,
        // we're still under the fuel limits.
        self.masm.branch(
            IntCmpKind::LtS,
            fuel_reg,
            RegImm::i64(0),
            continuation,
            OperandSize::S64,
        )?;
        // Out-of-fuel branch.
        FnCall::emit::<M>(
            &mut self.env,
            self.masm,
            &mut self.context,
            Callee::Builtin(out_of_fuel.clone()),
        )?;
        self.context.pop_and_free(self.masm)?;

        // Under fuel limits branch.
        self.masm.bind(continuation)?;
        self.context.free_reg(fuel_reg);

        Ok(())
    }

    /// Emits a series of instructions that load the `fuel_consumed` field from
    /// `VMStoreContext`.
    fn emit_load_fuel_consumed(&mut self, fuel_reg: Reg) -> Result<()> {
        let store_context_offset = self.env.vmoffsets.ptr.vmctx_store_context();
        let fuel_offset = self.env.vmoffsets.ptr.vmstore_context_fuel_consumed();
        self.masm.load_ptr(
            self.masm
                .address_at_vmctx(u32::from(store_context_offset))?,
            writable!(fuel_reg),
        )?;

        self.masm.load(
            self.masm.address_at_reg(fuel_reg, u32::from(fuel_offset))?,
            writable!(fuel_reg),
            // Fuel is an i64.
            OperandSize::S64,
        )
    }

    /// Checks if epoch interruption is configured and emits a series of
    /// instructions that check the current epoch against its deadline.
    pub fn maybe_emit_epoch_check(&mut self) -> Result<()> {
        if !self.tunables.epoch_interruption {
            return Ok(());
        }

        // The continuation branch if the current epoch hasn't reached the
        // configured deadline.
        let cont = self.masm.get_label()?;
        let new_epoch = self.env.builtins.new_epoch::<M::ABI, M::Ptr>()?;

        // Checks for runtime limits (e.g., fuel, epoch) are special since they
        // require inserting arbitrary function calls and control flow.
        // Special care must be taken to ensure that all invariants are met. In
        // this case, since `new_epoch` takes an argument and returns a value,
        // we must ensure that any registers used to hold the current epoch
        // value and deadline are not going to be needed later on by the
        // function call.
        let (epoch_deadline_reg, epoch_counter_reg) =
            self.context.without::<Result<(Reg, Reg)>, M, _>(
                &new_epoch.sig().regs,
                self.masm,
                |cx, masm| Ok((cx.any_gpr(masm)?, cx.any_gpr(masm)?)),
            )??;

        self.emit_load_epoch_deadline_and_counter(epoch_deadline_reg, epoch_counter_reg)?;

        // Spill locals and registers to avoid conflicts at the control flow
        // merge below.
        self.context.spill(self.masm)?;
        self.masm.branch(
            IntCmpKind::LtU,
            epoch_counter_reg,
            RegImm::reg(epoch_deadline_reg),
            cont,
            OperandSize::S64,
        )?;
        // Epoch deadline reached branch.
        FnCall::emit::<M>(
            &mut self.env,
            self.masm,
            &mut self.context,
            Callee::Builtin(new_epoch.clone()),
        )?;
        // `new_epoch` returns the new deadline. However we don't
        // perform any caching, so we simply drop this value.
        self.visit_drop()?;

        // Under epoch deadline branch.
        self.masm.bind(cont)?;

        self.context.free_reg(epoch_deadline_reg);
        self.context.free_reg(epoch_counter_reg);
        Ok(())
    }

    fn emit_load_epoch_deadline_and_counter(
        &mut self,
        epoch_deadline_reg: Reg,
        epoch_counter_reg: Reg,
    ) -> Result<()> {
        let epoch_ptr_offset = self.env.vmoffsets.ptr.vmctx_epoch_ptr();
        let store_context_offset = self.env.vmoffsets.ptr.vmctx_store_context();
        let epoch_deadline_offset = self.env.vmoffsets.ptr.vmstore_context_epoch_deadline();

        // Load the current epoch value into `epoch_counter_var`.
        self.masm.load_ptr(
            self.masm.address_at_vmctx(u32::from(epoch_ptr_offset))?,
            writable!(epoch_counter_reg),
        )?;

        // `epoch_deadline_var` contains the address of the value, so we need
        // to extract it.
        self.masm.load(
            self.masm.address_at_reg(epoch_counter_reg, 0)?,
            writable!(epoch_counter_reg),
            OperandSize::S64,
        )?;

        // Load the `VMStoreContext`.
        self.masm.load_ptr(
            self.masm
                .address_at_vmctx(u32::from(store_context_offset))?,
            writable!(epoch_deadline_reg),
        )?;

        self.masm.load(
            self.masm
                .address_at_reg(epoch_deadline_reg, u32::from(epoch_deadline_offset))?,
            writable!(epoch_deadline_reg),
            // The deadline value is a u64.
            OperandSize::S64,
        )
    }

    /// Increments the fuel consumed in `VMStoreContext` by flushing
    /// `self.fuel_consumed` to memory.
    fn emit_fuel_increment(&mut self) -> Result<()> {
        let fuel_at_point = std::mem::replace(&mut self.fuel_consumed, 0);
        if fuel_at_point == 0 {
            return Ok(());
        }

        let store_context_offset = self.env.vmoffsets.ptr.vmctx_store_context();
        let fuel_offset = self.env.vmoffsets.ptr.vmstore_context_fuel_consumed();
        let limits_reg = self.context.any_gpr(self.masm)?;

        // Load `VMStoreContext` into the `limits_reg` reg.
        self.masm.load_ptr(
            self.masm
                .address_at_vmctx(u32::from(store_context_offset))?,
            writable!(limits_reg),
        )?;

        self.masm.with_scratch::<IntScratch, _>(|masm, scratch| {
            // Load the fuel consumed at point into the scratch register.
            masm.load(
                masm.address_at_reg(limits_reg, u32::from(fuel_offset))?,
                scratch.writable(),
                OperandSize::S64,
            )?;

            // Add the fuel consumed at point with the value in the scratch
            // register.
            masm.add(
                scratch.writable(),
                scratch.inner(),
                RegImm::i64(fuel_at_point),
                OperandSize::S64,
            )?;

            // Store the updated fuel consumed to `VMStoreContext`.
            masm.store(
                scratch.inner().into(),
                masm.address_at_reg(limits_reg, u32::from(fuel_offset))?,
                OperandSize::S64,
            )
        })?;

        self.context.free_reg(limits_reg);

        Ok(())
    }

    /// Hook to handle fuel before visiting an operator.
    fn fuel_before_visit_op(&mut self, op: &Operator) -> Result<()> {
        if !self.context.reachable {
            // `self.fuel_consumed` must be correctly flushed to memory when
            // entering an unreachable state.
            ensure!(self.fuel_consumed == 0, CodeGenError::illegal_fuel_state())
        }

        // Generally, most instructions require 1 fuel unit.
        //
        // However, there are exceptions, which are detailed in the code below.
        // Note that the fuel accounting semantics align with those of
        // Cranelift; for further information, refer to
        // `crates/cranelift/src/func_environ.rs`.
        //
        // The primary distinction between the two implementations is that Winch
        // does not utilize a local-based cache to track fuel consumption.
        // Instead, each increase in fuel necessitates loading from and storing
        // to memory.
        //
        // Memory traffic will undoubtedly impact runtime performance. One
        // potential optimization is to designate a register as non-allocatable,
        // when fuel consumption is enabled, effectively using it as a local
        // fuel cache.
        self.fuel_consumed += match op {
            Operator::Nop | Operator::Drop => 0,
            Operator::Block { .. }
            | Operator::Loop { .. }
            | Operator::Unreachable
            | Operator::Return
            | Operator::Else
            | Operator::End => 0,
            _ => 1,
        };

        match op {
            Operator::Unreachable
            | Operator::Loop { .. }
            | Operator::If { .. }
            | Operator::Else { .. }
            | Operator::Br { .. }
            | Operator::BrIf { .. }
            | Operator::BrTable { .. }
            | Operator::End
            | Operator::Return
            | Operator::CallIndirect { .. }
            | Operator::Call { .. }
            | Operator::ReturnCall { .. }
            | Operator::ReturnCallIndirect { .. } => self.emit_fuel_increment(),
            _ => Ok(()),
        }
    }

    // Hook to handle source location mapping before visiting an operator.
    fn source_location_before_visit_op(&mut self, offset: usize) -> Result<()> {
        let loc = SourceLoc::new(offset as u32);
        let rel = self.source_loc_from(loc);
        self.source_location.current = self.masm.start_source_loc(rel)?;
        Ok(())
    }

    // Hook to handle source location mapping after visiting an operator.
    fn source_location_after_visit_op(&mut self) -> Result<()> {
        // Because in Winch binary emission is done in a single pass
        // and because the MachBuffer performs optimizations during
        // emission, we have to be careful when calling
        // [`MacroAssembler::end_source_location`] to avoid breaking the
        // invariant that checks that the end [CodeOffset] must be equal
        // or greater than the start [CodeOffset].
        if self.masm.current_code_offset()? >= self.source_location.current.0 {
            self.masm.end_source_loc()?;
        }

        Ok(())
    }

    pub(crate) fn emit_atomic_rmw(
        &mut self,
        arg: &MemArg,
        op: RmwOp,
        size: OperandSize,
        extend: Option<Extend<Zero>>,
    ) -> Result<()> {
        // We need to pop-push the operand to compute the address before passing control over to
        // masm, because some architectures may have specific requirements for the registers used
        // in some atomic operations.
        let operand = self.context.pop_to_reg(self.masm, None)?;
        if let Some(addr) = self.emit_compute_heap_address_align_checked(arg, size)? {
            let src = self.masm.address_at_reg(addr, 0)?;
            self.context.stack.push(operand.into());
            self.masm
                .atomic_rmw(&mut self.context, src, size, op, UNTRUSTED_FLAGS, extend)?;
            self.context.free_reg(addr);
        }

        Ok(())
    }

    pub(crate) fn emit_atomic_cmpxchg(
        &mut self,
        arg: &MemArg,
        size: OperandSize,
        extend: Option<Extend<Zero>>,
    ) -> Result<()> {
        // Emission for this instruction is a bit trickier. The address for the CAS is the 3rd from
        // the top of the stack, and we must emit instruction to compute the actual address with
        // `emit_compute_heap_address_align_checked`, while we still have access to self. However,
        // some ISAs have requirements with regard to the registers used for some arguments, so we
        // need to pass the context to the masm. To solve this issue, we pop the two first
        // arguments from the stack, compute the address, push back the arguments, and hand over
        // the control to masm. The implementer of `atomic_cas` can expect to find `expected` and
        // `replacement` at the top the context's stack.

        // pop the args
        let replacement = self.context.pop_to_reg(self.masm, None)?;
        let expected = self.context.pop_to_reg(self.masm, None)?;

        if let Some(addr) = self.emit_compute_heap_address_align_checked(arg, size)? {
            // push back the args
            self.context.stack.push(expected.into());
            self.context.stack.push(replacement.into());

            let src = self.masm.address_at_reg(addr, 0)?;
            self.masm
                .atomic_cas(&mut self.context, src, size, UNTRUSTED_FLAGS, extend)?;

            self.context.free_reg(addr);
        }
        Ok(())
    }

    #[cfg(not(feature = "threads"))]
    pub fn emit_atomic_wait(&mut self, _arg: &MemArg, _kind: AtomicWaitKind) -> Result<()> {
        Err(CodeGenError::unimplemented_wasm_instruction().into())
    }

    /// Emit the sequence of instruction for a `memory.atomic.wait*`.
    #[cfg(feature = "threads")]
    pub fn emit_atomic_wait(&mut self, arg: &MemArg, kind: AtomicWaitKind) -> Result<()> {
        // The `memory_atomic_wait*` builtins expect the following arguments:
        // - `memory`, as u32
        // - `address`, as u64
        // - `expected`, as either u64 or u32
        // - `timeout`, as u64
        // At this point our stack only contains the `timeout`, the `expected` and the address, so
        // we need to:
        // - insert the memory as the first argument
        // - compute the actual memory offset from the `MemArg`, if necessary.
        // Note that the builtin function performs the alignment and bounds checks for us, so we
        // don't need to emit that.

        let timeout = self.context.pop_to_reg(self.masm, None)?;
        let expected = self.context.pop_to_reg(self.masm, None)?;
        let addr = self.context.pop_to_reg(self.masm, None)?;

        // Put the target memory index as the first argument.
        let stack_len = self.context.stack.len();
        let builtin = match kind {
            AtomicWaitKind::Wait32 => self.env.builtins.memory_atomic_wait32::<M::ABI, M::Ptr>()?,
            AtomicWaitKind::Wait64 => self.env.builtins.memory_atomic_wait64::<M::ABI, M::Ptr>()?,
        };
        let builtin = self.prepare_builtin_defined_memory_arg(
            MemoryIndex::from_u32(arg.memory),
            stack_len,
            builtin,
        )?;

        if arg.offset != 0 {
            self.masm.add(
                writable!(addr.reg),
                addr.reg,
                RegImm::i64(arg.offset as i64),
                OperandSize::S64,
            )?;
        }

        self.context
            .stack
            .push(TypedReg::new(WasmValType::I64, addr.reg).into());
        self.context.stack.push(expected.into());
        self.context.stack.push(timeout.into());

        FnCall::emit::<M>(&mut self.env, self.masm, &mut self.context, builtin)?;

        Ok(())
    }

    #[cfg(not(feature = "threads"))]
    pub fn emit_atomic_notify(&mut self, _arg: &MemArg) -> Result<()> {
        Err(CodeGenError::unimplemented_wasm_instruction().into())
    }

    #[cfg(feature = "threads")]
    pub fn emit_atomic_notify(&mut self, arg: &MemArg) -> Result<()> {
        // The memory `memory_atomic_notify` builtin expects the following arguments:
        // - `memory`, as u32
        // - `address`, as u64
        // - `count`: as u32
        // At this point our stack only contains the `count` and the `address`, so we need to:
        // - insert the memory as the first argument
        // - compute the actual memory offset from the `MemArg`, if necessary.
        // Note that the builtin function performs the alignment and bounds checks for us, so we
        // don't need to emit that.

        // pop the arguments from the stack.
        let count = self.context.pop_to_reg(self.masm, None)?;
        let addr = self.context.pop_to_reg(self.masm, None)?;

        // Put the target memory index as the first argument.
        let builtin = self.env.builtins.memory_atomic_notify::<M::ABI, M::Ptr>()?;
        let stack_len = self.context.stack.len();
        let builtin = self.prepare_builtin_defined_memory_arg(
            MemoryIndex::from_u32(arg.memory),
            stack_len,
            builtin,
        )?;

        if arg.offset != 0 {
            self.masm.add(
                writable!(addr.reg),
                addr.reg,
                RegImm::i64(arg.offset as i64),
                OperandSize::S64,
            )?;
        }

        // push remaining arguments.
        self.context
            .stack
            .push(TypedReg::new(WasmValType::I64, addr.reg).into());
        self.context.stack.push(count.into());

        FnCall::emit::<M>(&mut self.env, self.masm, &mut self.context, builtin)?;

        Ok(())
    }

    pub fn prepare_builtin_defined_memory_arg(
        &mut self,
        mem: MemoryIndex,
        defined_index_at: usize,
        builtin: BuiltinFunction,
    ) -> Result<Callee> {
        match self.env.translation.module.defined_memory_index(mem) {
            // This memory is defined in this module, so the vmctx is this
            // module's vmctx and the memory index is `defined` as returned here.
            Some(defined) => {
                self.context
                    .stack
                    .insert_many(defined_index_at, &[defined.as_u32().try_into()?]);
                Ok(Callee::Builtin(builtin))
            }

            // This memory is not defined in this module, so the defined index
            // is loaded from the `VMMemoryImport` and the vmctx is loaded from
            // the vmctx itself.
            None => {
                let vmimport = self.env.vmoffsets.vmctx_vmmemory_import(mem);
                let vmctx_offset = vmimport + u32::from(self.env.vmoffsets.vmmemory_import_vmctx());
                let index_offset = vmimport + u32::from(self.env.vmoffsets.vmmemory_import_index());
                let index_addr = self.masm.address_at_vmctx(index_offset)?;
                let index_dst = self.context.reg_for_class(RegClass::Int, self.masm)?;
                self.masm
                    .load(index_addr, writable!(index_dst), OperandSize::S32)?;
                self.context
                    .stack
                    .insert_many(defined_index_at, &[Val::reg(index_dst, WasmValType::I32)]);
                Ok(Callee::BuiltinWithDifferentVmctx(builtin, vmctx_offset))
            }
        }
    }

    /// Same as `prepare_builtin_defined_memory_arg`, but for tables.
    pub fn prepare_builtin_defined_table_arg(
        &mut self,
        table: TableIndex,
        defined_index_at: usize,
        builtin: BuiltinFunction,
    ) -> Result<Callee> {
        match self.env.translation.module.defined_table_index(table) {
            Some(defined) => {
                self.context
                    .stack
                    .insert_many(defined_index_at, &[defined.as_u32().try_into()?]);
                Ok(Callee::Builtin(builtin))
            }
            None => {
                let vmimport = self.env.vmoffsets.vmctx_vmtable_import(table);
                let vmctx_offset = vmimport + u32::from(self.env.vmoffsets.vmtable_import_vmctx());
                let index_offset = vmimport + u32::from(self.env.vmoffsets.vmtable_import_index());
                let index_addr = self.masm.address_at_vmctx(index_offset)?;
                let index_dst = self.context.reg_for_class(RegClass::Int, self.masm)?;
                self.masm
                    .load(index_addr, writable!(index_dst), OperandSize::S32)?;
                self.context
                    .stack
                    .insert_many(defined_index_at, &[Val::reg(index_dst, WasmValType::I32)]);
                Ok(Callee::BuiltinWithDifferentVmctx(builtin, vmctx_offset))
            }
        }
    }
}

/// Returns the index of the [`ControlStackFrame`] for the given
/// depth.
pub fn control_index(depth: u32, control_length: usize) -> Result<usize> {
    (control_length - 1)
        .checked_sub(depth as usize)
        .ok_or_else(|| anyhow!(CodeGenError::control_frame_expected()))
}
