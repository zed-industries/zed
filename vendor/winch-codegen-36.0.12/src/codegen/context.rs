use anyhow::{Result, bail, ensure};
use wasmparser::{Ieee32, Ieee64};
use wasmtime_environ::{VMOffsets, WasmHeapType, WasmValType};

use super::ControlStackFrame;
use crate::{
    abi::{ABIOperand, ABIResults, RetArea, vmctx},
    codegen::{BranchState, CodeGenError, CodeGenPhase, Emission, Prologue},
    frame::Frame,
    isa::reg::RegClass,
    masm::{
        ExtractLaneKind, Imm, IntScratch, MacroAssembler, MemMoveDirection, OperandSize, RegImm,
        ReplaceLaneKind, SPOffset, ShiftKind, StackSlot,
    },
    reg::{Reg, WritableReg, writable},
    regalloc::RegAlloc,
    stack::{Stack, TypedReg, Val},
};

/// The code generation context.
/// The code generation context is made up of three
/// essential data structures:
///
/// * The register allocator, in charge of keeping the inventory of register
///   availability.
/// * The value stack, which keeps track of the state of the values
///   after each operation.
/// * The current function's frame.
///
/// These data structures normally require cooperating with each other
/// to perform most of the operations needed during the code
/// generation process. The code generation context should
/// be generally used as the single entry point to access
/// the compound functionality provided by its elements.
pub(crate) struct CodeGenContext<'a, P: CodeGenPhase> {
    /// The register allocator.
    pub regalloc: RegAlloc,
    /// The value stack.
    pub stack: Stack,
    /// The current function's frame.
    pub frame: Frame<P>,
    /// Reachability state.
    pub reachable: bool,
    /// A reference to the VMOffsets.
    pub vmoffsets: &'a VMOffsets<u8>,
}

impl<'a> CodeGenContext<'a, Emission> {
    /// Prepares arguments for emitting an i32 shift operation.
    pub fn i32_shift<M>(&mut self, masm: &mut M, kind: ShiftKind) -> Result<()>
    where
        M: MacroAssembler,
    {
        let top = self
            .stack
            .peek()
            .ok_or_else(|| CodeGenError::missing_values_in_stack())?;

        if top.is_i32_const() {
            let val = self
                .stack
                .pop_i32_const()
                .ok_or_else(|| CodeGenError::missing_values_in_stack())?;
            let typed_reg = self.pop_to_reg(masm, None)?;
            masm.shift_ir(
                writable!(typed_reg.reg),
                Imm::i32(val),
                typed_reg.reg,
                kind,
                OperandSize::S32,
            )?;
            self.stack.push(typed_reg.into());
        } else {
            masm.shift(self, kind, OperandSize::S32)?;
        }
        Ok(())
    }

    /// Prepares arguments for emitting an i64 binary operation.
    pub fn i64_shift<M>(&mut self, masm: &mut M, kind: ShiftKind) -> Result<()>
    where
        M: MacroAssembler,
    {
        let top = self
            .stack
            .peek()
            .ok_or_else(|| CodeGenError::missing_values_in_stack())?;
        if top.is_i64_const() {
            let val = self
                .stack
                .pop_i64_const()
                .ok_or_else(|| CodeGenError::missing_values_in_stack())?;
            let typed_reg = self.pop_to_reg(masm, None)?;
            masm.shift_ir(
                writable!(typed_reg.reg),
                Imm::i64(val),
                typed_reg.reg,
                kind,
                OperandSize::S64,
            )?;
            self.stack.push(typed_reg.into());
        } else {
            masm.shift(self, kind, OperandSize::S64)?;
        };

        Ok(())
    }
}

impl<'a> CodeGenContext<'a, Prologue> {
    /// Create a new code generation context.
    pub fn new(
        regalloc: RegAlloc,
        stack: Stack,
        frame: Frame<Prologue>,
        vmoffsets: &'a VMOffsets<u8>,
    ) -> Self {
        Self {
            regalloc,
            stack,
            frame,
            reachable: true,
            vmoffsets,
        }
    }

    /// Prepares the frame for the [`Emission`] code generation phase.
    pub fn for_emission(self) -> CodeGenContext<'a, Emission> {
        CodeGenContext {
            regalloc: self.regalloc,
            stack: self.stack,
            reachable: self.reachable,
            vmoffsets: self.vmoffsets,
            frame: self.frame.for_emission(),
        }
    }
}

impl<'a> CodeGenContext<'a, Emission> {
    /// Request a specific register to the register allocator,
    /// spilling if not available.
    pub fn reg<M: MacroAssembler>(&mut self, named: Reg, masm: &mut M) -> Result<Reg> {
        self.regalloc.reg(named, |regalloc| {
            Self::spill_impl(&mut self.stack, regalloc, &self.frame, masm)
        })
    }

    /// Allocate a register for the given WebAssembly type.
    pub fn reg_for_type<M: MacroAssembler>(
        &mut self,
        ty: WasmValType,
        masm: &mut M,
    ) -> Result<Reg> {
        use WasmValType::*;
        match ty {
            I32 | I64 => self.reg_for_class(RegClass::Int, masm),
            F32 | F64 => self.reg_for_class(RegClass::Float, masm),
            // All of our supported architectures use the float registers for vector operations.
            V128 => self.reg_for_class(RegClass::Float, masm),
            Ref(rt) => match rt.heap_type {
                WasmHeapType::Func | WasmHeapType::Extern => {
                    self.reg_for_class(RegClass::Int, masm)
                }
                _ => bail!(CodeGenError::unsupported_wasm_type()),
            },
        }
    }

    /// Request the register allocator to provide the next available
    /// register of the specified class.
    pub fn reg_for_class<M: MacroAssembler>(
        &mut self,
        class: RegClass,
        masm: &mut M,
    ) -> Result<Reg> {
        self.regalloc.reg_for_class(class, &mut |regalloc| {
            Self::spill_impl(&mut self.stack, regalloc, &self.frame, masm)
        })
    }

    /// Convenience wrapper around `CodeGenContext::reg_for_class`, to
    /// request the next available general purpose register.
    pub fn any_gpr<M: MacroAssembler>(&mut self, masm: &mut M) -> Result<Reg> {
        self.reg_for_class(RegClass::Int, masm)
    }

    /// Convenience wrapper around `CodeGenContext::reg_for_class`, to
    /// request the next available floating point register.
    pub fn any_fpr<M: MacroAssembler>(&mut self, masm: &mut M) -> Result<Reg> {
        self.reg_for_class(RegClass::Float, masm)
    }

    /// Executes the provided function, guaranteeing that the specified set of
    /// registers, if any, remain unallocatable throughout the function's
    /// execution.
    pub fn without<'r, T, M, F>(
        &mut self,
        regs: impl IntoIterator<Item = &'r Reg> + Copy,
        masm: &mut M,
        mut f: F,
    ) -> Result<T>
    where
        M: MacroAssembler,
        F: FnMut(&mut Self, &mut M) -> T,
    {
        for r in regs {
            self.reg(*r, masm)?;
        }

        let result = f(self, masm);

        for r in regs {
            self.free_reg(*r);
        }

        Ok(result)
    }

    /// Free the given register.
    pub fn free_reg(&mut self, reg: impl Into<Reg>) {
        let reg: Reg = reg.into();
        self.regalloc.free(reg);
    }

    /// Loads the stack top value into the next available register, if
    /// it isn't already one; spilling if there are no registers
    /// available.  Optionally the caller may specify a specific
    /// destination register.
    /// When a named register is requested and it's not at the top of the
    /// stack a move from register to register might happen, in which case
    /// the source register will be freed.
    pub fn pop_to_reg<M: MacroAssembler>(
        &mut self,
        masm: &mut M,
        named: Option<Reg>,
    ) -> Result<TypedReg> {
        let typed_reg = if let Some(dst) = named {
            self.stack.pop_named_reg(dst)
        } else {
            self.stack.pop_reg()
        };

        if let Some(dst) = typed_reg {
            return Ok(dst);
        }

        let val = self.stack.pop().expect("a value at stack top");
        let reg = if let Some(r) = named {
            self.reg(r, masm)?
        } else {
            self.reg_for_type(val.ty(), masm)?
        };

        if val.is_mem() {
            let mem = val.unwrap_mem();
            let curr_offset = masm.sp_offset()?.as_u32();
            let slot_offset = mem.slot.offset.as_u32();
            ensure!(
                curr_offset == slot_offset,
                CodeGenError::invalid_sp_offset(),
            );
            masm.pop(writable!(reg), val.ty().try_into()?)?;
        } else {
            self.move_val_to_reg(&val, reg, masm)?;
            // Free the source value if it is a register.
            if val.is_reg() {
                self.free_reg(val.unwrap_reg());
            }
        }

        Ok(TypedReg::new(val.ty(), reg))
    }

    /// Pops the value stack top and stores it at the specified address.
    pub fn pop_to_addr<M: MacroAssembler>(&mut self, masm: &mut M, addr: M::Address) -> Result<()> {
        let val = self.stack.pop().expect("a value at stack top");
        let ty = val.ty();
        let size: OperandSize = ty.try_into()?;
        match val {
            Val::Reg(tr) => {
                masm.store(tr.reg.into(), addr, size)?;
                self.free_reg(tr.reg);
            }
            Val::I32(v) => masm.store(RegImm::i32(v), addr, size)?,
            Val::I64(v) => masm.store(RegImm::i64(v), addr, size)?,
            Val::F32(v) => masm.store(RegImm::f32(v.bits()), addr, size)?,
            Val::F64(v) => masm.store(RegImm::f64(v.bits()), addr, size)?,
            Val::V128(v) => masm.store(RegImm::v128(v), addr, size)?,
            Val::Local(local) => {
                let slot = self.frame.get_wasm_local(local.index);
                let local_addr = masm.local_address(&slot)?;
                masm.with_scratch::<IntScratch, _>(|masm, scratch| {
                    masm.load(local_addr, scratch.writable(), size)?;
                    masm.store(scratch.inner().into(), addr, size)
                })?;
            }
            Val::Memory(_) => {
                masm.with_scratch_for(ty, |masm, scratch| {
                    masm.pop(scratch.writable(), size)?;
                    masm.store(scratch.inner().into(), addr, size)
                })?;
            }
        }

        Ok(())
    }

    /// Move a stack value to the given register.
    pub fn move_val_to_reg<M: MacroAssembler>(
        &self,
        src: &Val,
        dst: Reg,
        masm: &mut M,
    ) -> Result<()> {
        let size: OperandSize = src.ty().try_into()?;
        match src {
            Val::Reg(tr) => masm.mov(writable!(dst), RegImm::reg(tr.reg), size),
            Val::I32(imm) => masm.mov(writable!(dst), RegImm::i32(*imm), size),
            Val::I64(imm) => masm.mov(writable!(dst), RegImm::i64(*imm), size),
            Val::F32(imm) => masm.mov(writable!(dst), RegImm::f32(imm.bits()), size),
            Val::F64(imm) => masm.mov(writable!(dst), RegImm::f64(imm.bits()), size),
            Val::V128(imm) => masm.mov(writable!(dst), RegImm::v128(*imm), size),
            Val::Local(local) => {
                let slot = self.frame.get_wasm_local(local.index);
                let addr = masm.local_address(&slot)?;
                masm.load(addr, writable!(dst), size)
            }
            Val::Memory(mem) => {
                let addr = masm.address_from_sp(mem.slot.offset)?;
                masm.load(addr, writable!(dst), size)
            }
        }
    }

    /// Prepares arguments for emitting a unary operation.
    ///
    /// The `emit` function returns the `TypedReg` to put on the value stack.
    pub fn unop<F, M>(&mut self, masm: &mut M, emit: F) -> Result<()>
    where
        F: FnOnce(&mut M, Reg) -> Result<TypedReg>,
        M: MacroAssembler,
    {
        let typed_reg = self.pop_to_reg(masm, None)?;
        let dst = emit(masm, typed_reg.reg)?;
        self.stack.push(dst.into());

        Ok(())
    }

    /// Prepares arguments for emitting a binary operation.
    ///
    /// The `emit` function returns the `TypedReg` to put on the value stack.
    pub fn binop<F, M>(&mut self, masm: &mut M, size: OperandSize, emit: F) -> Result<()>
    where
        F: FnOnce(&mut M, Reg, Reg, OperandSize) -> Result<TypedReg>,
        M: MacroAssembler,
    {
        let src = self.pop_to_reg(masm, None)?;
        let dst = self.pop_to_reg(masm, None)?;
        let dst = emit(masm, dst.reg, src.reg, size)?;
        self.free_reg(src);
        self.stack.push(dst.into());

        Ok(())
    }

    /// Prepares arguments for emitting an f32 or f64 comparison operation.
    pub fn float_cmp_op<F, M>(&mut self, masm: &mut M, size: OperandSize, emit: F) -> Result<()>
    where
        F: FnOnce(&mut M, Reg, Reg, Reg, OperandSize) -> Result<()>,
        M: MacroAssembler,
    {
        let src2 = self.pop_to_reg(masm, None)?;
        let src1 = self.pop_to_reg(masm, None)?;
        let dst = self.any_gpr(masm)?;
        emit(masm, dst, src1.reg, src2.reg, size)?;
        self.free_reg(src1);
        self.free_reg(src2);

        let dst = match size {
            // Float comparison operators are defined as
            // [f64 f64] -> i32
            // https://webassembly.github.io/spec/core/appendix/index-instructions.html
            OperandSize::S32 | OperandSize::S64 => TypedReg::i32(dst),
            OperandSize::S8 | OperandSize::S16 | OperandSize::S128 => {
                bail!(CodeGenError::unexpected_operand_size())
            }
        };
        self.stack.push(dst.into());

        Ok(())
    }

    /// Prepares arguments for emitting an i32 binary operation.
    ///
    /// The `emit` function returns the `TypedReg` to put on the value stack.
    pub fn i32_binop<F, M>(&mut self, masm: &mut M, mut emit: F) -> Result<()>
    where
        F: FnMut(&mut M, Reg, RegImm, OperandSize) -> Result<TypedReg>,
        M: MacroAssembler,
    {
        match self.pop_i32_const() {
            Some(val) => {
                let typed_reg = self.pop_to_reg(masm, None)?;
                let dst = emit(masm, typed_reg.reg, RegImm::i32(val), OperandSize::S32)?;
                self.stack.push(dst.into());
            }
            None => self.binop(masm, OperandSize::S32, |masm, dst, src, size| {
                emit(masm, dst, src.into(), size)
            })?,
        }
        Ok(())
    }

    /// Prepares arguments for emitting an i64 binary operation.
    ///
    /// The `emit` function returns the `TypedReg` to put on the value stack.
    pub fn i64_binop<F, M>(&mut self, masm: &mut M, emit: F) -> Result<()>
    where
        F: FnOnce(&mut M, Reg, RegImm, OperandSize) -> Result<TypedReg>,
        M: MacroAssembler,
    {
        match self.pop_i64_const() {
            Some(val) => {
                let typed_reg = self.pop_to_reg(masm, None)?;
                let dst = emit(masm, typed_reg.reg, RegImm::i64(val), OperandSize::S64)?;
                self.stack.push(dst.into());
            }
            None => self.binop(masm, OperandSize::S64, |masm, dst, src, size| {
                emit(masm, dst, src.into(), size)
            })?,
        }
        Ok(())
    }

    /// Returns the i32 const on top of the stack or None if there isn't one.
    pub fn pop_i32_const(&mut self) -> Option<i32> {
        let top = self.stack.peek().expect("value at stack top");

        if top.is_i32_const() {
            let val = self
                .stack
                .pop_i32_const()
                .expect("i32 const value at stack top");
            Some(val)
        } else {
            None
        }
    }

    /// Returns the i64 const on top of the stack or None if there isn't one.
    pub fn pop_i64_const(&mut self) -> Option<i64> {
        let top = self.stack.peek().expect("value at stack top");

        if top.is_i64_const() {
            let val = self
                .stack
                .pop_i64_const()
                .expect("i64 const value at stack top");
            Some(val)
        } else {
            None
        }
    }

    /// Returns the f32 const on top of the stack or None if there isn't one.
    pub fn pop_f32_const(&mut self) -> Option<Ieee32> {
        let top = self.stack.peek().expect("value at stack top");

        if top.is_f32_const() {
            let val = self
                .stack
                .pop_f32_const()
                .expect("f32 const value at stack top");
            Some(val)
        } else {
            None
        }
    }

    /// Returns the f64 const on top of the stack or None if there isn't one.
    pub fn pop_f64_const(&mut self) -> Option<Ieee64> {
        let top = self.stack.peek().expect("value at stack top");

        if top.is_f64_const() {
            let val = self
                .stack
                .pop_f64_const()
                .expect("f64 const value at stack top");
            Some(val)
        } else {
            None
        }
    }

    /// Prepares arguments for emitting a convert operation.
    pub fn convert_op<F, M>(&mut self, masm: &mut M, dst_ty: WasmValType, emit: F) -> Result<()>
    where
        F: FnOnce(&mut M, Reg, Reg, OperandSize) -> Result<()>,
        M: MacroAssembler,
    {
        let src = self.pop_to_reg(masm, None)?;
        let dst = self.reg_for_type(dst_ty, masm)?;
        let dst_size = match dst_ty {
            WasmValType::I32 => OperandSize::S32,
            WasmValType::I64 => OperandSize::S64,
            WasmValType::F32 => OperandSize::S32,
            WasmValType::F64 => OperandSize::S64,
            WasmValType::V128 => bail!(CodeGenError::unsupported_wasm_type()),
            WasmValType::Ref(_) => bail!(CodeGenError::unsupported_wasm_type()),
        };

        emit(masm, dst, src.into(), dst_size)?;

        self.free_reg(src);
        self.stack.push(TypedReg::new(dst_ty, dst).into());
        Ok(())
    }

    /// Prepares arguments for emitting a convert operation with a temporary
    /// register.
    pub fn convert_op_with_tmp_reg<F, M>(
        &mut self,
        masm: &mut M,
        dst_ty: WasmValType,
        tmp_reg_class: RegClass,
        emit: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut M, Reg, Reg, Reg, OperandSize) -> Result<()>,
        M: MacroAssembler,
    {
        let tmp_gpr = self.reg_for_class(tmp_reg_class, masm)?;
        self.convert_op(masm, dst_ty, |masm, dst, src, dst_size| {
            emit(masm, dst, src, tmp_gpr, dst_size)
        })?;
        self.free_reg(tmp_gpr);
        Ok(())
    }

    /// Prepares arguments for emitting an extract lane operation.
    pub fn extract_lane_op<F, M>(
        &mut self,
        masm: &mut M,
        kind: ExtractLaneKind,
        emit: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut M, Reg, WritableReg, ExtractLaneKind) -> Result<()>,
        M: MacroAssembler,
    {
        let src = self.pop_to_reg(masm, None)?;
        let dst = writable!(match kind {
            ExtractLaneKind::I8x16S
            | ExtractLaneKind::I8x16U
            | ExtractLaneKind::I16x8S
            | ExtractLaneKind::I16x8U
            | ExtractLaneKind::I32x4
            | ExtractLaneKind::I64x2 => self.any_gpr(masm)?,
            ExtractLaneKind::F32x4 | ExtractLaneKind::F64x2 => src.reg,
        });

        emit(masm, src.reg, dst, kind)?;

        match kind {
            ExtractLaneKind::I8x16S
            | ExtractLaneKind::I8x16U
            | ExtractLaneKind::I16x8S
            | ExtractLaneKind::I16x8U
            | ExtractLaneKind::I32x4
            | ExtractLaneKind::I64x2 => self.free_reg(src),
            _ => (),
        }

        let dst = dst.to_reg();
        let dst = match kind {
            ExtractLaneKind::I8x16S
            | ExtractLaneKind::I8x16U
            | ExtractLaneKind::I16x8S
            | ExtractLaneKind::I16x8U
            | ExtractLaneKind::I32x4 => TypedReg::i32(dst),
            ExtractLaneKind::I64x2 => TypedReg::i64(dst),
            ExtractLaneKind::F32x4 => TypedReg::f32(dst),
            ExtractLaneKind::F64x2 => TypedReg::f64(dst),
        };

        self.stack.push(Val::Reg(dst));
        Ok(())
    }

    /// Prepares arguments for emitting a replace lane operation.
    pub fn replace_lane_op<F, M>(
        &mut self,
        masm: &mut M,
        kind: ReplaceLaneKind,
        emit: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut M, RegImm, WritableReg, ReplaceLaneKind) -> Result<()>,
        M: MacroAssembler,
    {
        let src = match kind {
            ReplaceLaneKind::I8x16 | ReplaceLaneKind::I16x8 | ReplaceLaneKind::I32x4 => {
                self.pop_i32_const().map(RegImm::i32)
            }
            ReplaceLaneKind::I64x2 => self.pop_i64_const().map(RegImm::i64),
            ReplaceLaneKind::F32x4 => self.pop_f32_const().map(|v| RegImm::f32(v.bits())),
            ReplaceLaneKind::F64x2 => self.pop_f64_const().map(|v| RegImm::f64(v.bits())),
        }
        .map_or_else(
            || Ok(RegImm::reg(self.pop_to_reg(masm, None)?.into())),
            Ok::<_, anyhow::Error>,
        )?;

        let dst = self.pop_to_reg(masm, None)?;

        emit(masm, src, writable!(dst.into()), kind)?;

        if let RegImm::Reg(reg) = src {
            self.free_reg(reg);
        }
        self.stack.push(dst.into());

        Ok(())
    }

    /// Drops the last `n` elements of the stack, calling the provided
    /// function for each `n` stack value.
    /// The values are dropped in top-to-bottom order.
    pub fn drop_last<F>(&mut self, last: usize, mut f: F) -> Result<()>
    where
        F: FnMut(&mut RegAlloc, &Val) -> Result<()>,
    {
        if last > 0 {
            let len = self.stack.len();
            ensure!(last <= len, CodeGenError::unexpected_value_stack_index(),);
            let truncate = self.stack.len() - last;
            let stack_mut = self.stack.inner_mut();

            // Invoke the callback in top-to-bottom order.
            for v in stack_mut[truncate..].into_iter().rev() {
                f(&mut self.regalloc, v)?
            }
            stack_mut.truncate(truncate);
        }

        Ok(())
    }

    /// Convenience wrapper around [`Self::spill_callback`].
    ///
    /// This function exists for cases in which triggering an unconditional
    /// spill is needed, like before entering control flow.
    pub fn spill<M: MacroAssembler>(&mut self, masm: &mut M) -> Result<()> {
        Self::spill_impl(&mut self.stack, &mut self.regalloc, &self.frame, masm)
    }

    /// Prepares the compiler to branch to the given destination
    /// frame.
    ///  This process involves:
    /// * Balancing the machine stack pointer and value stack by
    ///   popping it to match the destination branch.
    /// * Updating the reachability state.
    /// * Marking the destination frame as a destination target.
    pub fn br<M, F, B>(
        &mut self,
        dest: &mut ControlStackFrame,
        masm: &mut M,
        mut maybe_pop_results: F,
    ) -> Result<()>
    where
        M: MacroAssembler,
        F: FnMut(&mut M, &mut Self, &mut ControlStackFrame) -> Result<()>,
        B: BranchState,
    {
        let state = dest.stack_state();
        let target_offset = state.target_offset;
        let base_offset = state.base_offset;
        let results_size = dest.results::<M>()?.size();

        maybe_pop_results(masm, self, dest)?;
        // After calling `maybe_pop_results`, the stack pointer plus
        // any result space needed, must be greater or equal to the
        // destination frame base stack pointer offset.
        //
        // We check
        //   current_sp + results >= base_offset
        // as opposed to
        //   current_sp >= base_offset
        //
        // To:
        //  - Verify that `maybe_pop_results` popped exactly the right
        //    amount relative to the base offset.
        //  - Accommodate for multi-branch cases (i.e., `br_table`) in which
        //    result handling happens only once and _could_ happen outside of
        //    `maybe_pop_results` callback.
        //
        //
        // Ensuring that the current stack pointer offset plus any
        // result space is equal to or greater than the target branch
        // base offset is the the most deterministic check at branch
        // emission time since we can be certain that the base offset
        // is the value recorded when a new control frame was pushed,
        // upon which the expected target offset is calculated.
        ensure!(
            (masm.sp_offset()?.as_u32() + results_size) >= base_offset.as_u32(),
            CodeGenError::invalid_sp_offset()
        );

        // At jump sites, the machine stack might be left unbalanced,
        // due to register spills.
        // The following snippet, pops the stack pointer to ensure
        // that it is correctly placed according to the expectations
        // of the destination branch.
        //
        // Note that in most branch cases (`return`, ` br`) the stack
        // pointer will be already balanced, by virtue of calling
        // [`ControlStackFrame::pop_abi_results`] through the
        // callback.
        //
        // More generally speaking the current stack pointer will be
        // less than the destination frame stack pointer offset in
        // cases in which the top value in the value stack is a memory
        // entry which needs to be popped into the return location
        // according to the ABI (a register for single value returns
        // and a memory slot for 1+ returns).
        //
        // Stack balancing is mostly required for WebAssembly
        // instructions that deal with multiple destination branches
        // (e.g., `br_table`) or fall-through scenarios (e.g.,
        // `br_if`). In order to ensure that multi-value returns are
        // handled correctly we ensure that correct placing of stack
        // results by emitting a [`MacroAssembler::memmove`]
        // instruction, prior to claiming any excess stack space.
        //
        // Depending on the branch state, the compiler might enter in an
        // unreachable state; instead of immediately truncating the value stack
        // to the expected length of the destination branch, we let the
        // reachability analysis code decide what should happen with the length
        // of the value stack once reachability is actually restored. At that
        // point, the right stack pointer offset will also be restored, which
        // should match the contents of the value stack.
        if dest.unbalanced::<M>(masm)? {
            masm.memmove(
                masm.sp_offset()?,
                target_offset,
                results_size,
                MemMoveDirection::LowToHigh,
            )?;
        }
        masm.ensure_sp_for_jump(target_offset)?;
        dest.set_as_target();
        masm.jmp(*dest.label())?;
        if B::unreachable_state_after_emission() {
            self.reachable = false;
        }
        Ok(())
    }

    /// Push the ABI representation of the results stack.
    pub fn push_abi_results<M, F>(
        &mut self,
        results: &ABIResults,
        masm: &mut M,
        mut calculate_ret_area: F,
    ) -> Result<()>
    where
        M: MacroAssembler,
        F: FnMut(&ABIResults, &mut CodeGenContext<Emission>, &mut M) -> Option<RetArea>,
    {
        let area = results
            .on_stack()
            .then(|| calculate_ret_area(&results, self, masm).unwrap());

        for operand in results.operands().iter() {
            match operand {
                ABIOperand::Reg { reg, ty, .. } => {
                    ensure!(
                        self.regalloc.reg_available(*reg),
                        CodeGenError::expected_register_to_be_available(),
                    );

                    let typed_reg = TypedReg::new(*ty, self.reg(*reg, masm)?);
                    self.stack.push(typed_reg.into());
                }
                ABIOperand::Stack { ty, offset, size } => match area.unwrap() {
                    RetArea::SP(sp_offset) => {
                        let slot =
                            StackSlot::new(SPOffset::from_u32(sp_offset.as_u32() - offset), *size);
                        self.stack.push(Val::mem(*ty, slot));
                    }
                    // This function is only expected to be called when dealing
                    // with control flow and when calling functions; as a
                    // callee, only [Self::pop_abi_results] is needed when
                    // finalizing the function compilation.
                    _ => bail!(CodeGenError::unexpected_function_call()),
                },
            }
        }

        Ok(())
    }

    /// Truncates the value stack to the specified target.
    /// This function is intended to only be used when restoring the code
    /// generation's reachability state, when handling an unreachable end or
    /// else.
    pub fn truncate_stack_to(&mut self, target: usize) -> Result<()> {
        if self.stack.len() > target {
            self.drop_last(self.stack.len() - target, |regalloc, val| match val {
                Val::Reg(tr) => Ok(regalloc.free(tr.reg)),
                _ => Ok(()),
            })
        } else {
            Ok(())
        }
    }

    /// Load the [VMContext] pointer into the designated pinned register.
    pub fn load_vmctx<M>(&mut self, masm: &mut M) -> Result<()>
    where
        M: MacroAssembler,
    {
        let addr = masm.local_address(&self.frame.vmctx_slot())?;
        masm.load_ptr(addr, writable!(vmctx!(M)))
    }

    /// Spill locals and registers to memory.
    // TODO: optimize the spill range;
    // At any point in the program, the stack might already contain memory
    // entries; we could effectively ignore that range; only focusing on the
    // range that contains spillable values.
    fn spill_impl<M: MacroAssembler>(
        stack: &mut Stack,
        regalloc: &mut RegAlloc,
        frame: &Frame<Emission>,
        masm: &mut M,
    ) -> Result<()> {
        for v in stack.inner_mut() {
            match v {
                Val::Reg(r) => {
                    let slot = masm.push(r.reg, r.ty.try_into()?)?;
                    regalloc.free(r.reg);
                    *v = Val::mem(r.ty, slot);
                }
                Val::Local(local) => {
                    let slot = frame.get_wasm_local(local.index);
                    let addr = masm.local_address(&slot)?;
                    masm.with_scratch_for(slot.ty, |masm, scratch| {
                        masm.load(addr, scratch.writable(), slot.ty.try_into()?)?;
                        let stack_slot = masm.push(scratch.inner(), slot.ty.try_into()?)?;
                        *v = Val::mem(slot.ty, stack_slot);
                        anyhow::Ok(())
                    })?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Prepares for emitting a binary operation where four 64-bit operands are
    /// used to produce two 64-bit operands, e.g. a 128-bit binop.
    pub fn binop128<F, M>(&mut self, masm: &mut M, emit: F) -> Result<()>
    where
        F: FnOnce(&mut M, Reg, Reg, Reg, Reg) -> Result<(TypedReg, TypedReg)>,
        M: MacroAssembler,
    {
        let rhs_hi = self.pop_to_reg(masm, None)?;
        let rhs_lo = self.pop_to_reg(masm, None)?;
        let lhs_hi = self.pop_to_reg(masm, None)?;
        let lhs_lo = self.pop_to_reg(masm, None)?;
        let (lo, hi) = emit(masm, lhs_lo.reg, lhs_hi.reg, rhs_lo.reg, rhs_hi.reg)?;
        self.free_reg(rhs_hi);
        self.free_reg(rhs_lo);
        self.stack.push(lo.into());
        self.stack.push(hi.into());

        Ok(())
    }

    /// Prepares to emit a vector `all_true` operation.
    pub fn v128_all_true_op<F, M>(&mut self, masm: &mut M, emit: F) -> Result<()>
    where
        F: FnOnce(&mut M, Reg, Reg) -> Result<()>,
        M: MacroAssembler,
    {
        let src = self.pop_to_reg(masm, None)?;
        let dst = self.any_gpr(masm)?;
        emit(masm, src.reg, dst)?;
        self.free_reg(src);
        self.stack.push(TypedReg::i32(dst).into());

        Ok(())
    }

    /// Prepares to emit a vector `bitmask` operation.
    pub fn v128_bitmask_op<F, M>(&mut self, masm: &mut M, emit: F) -> Result<()>
    where
        F: FnOnce(&mut M, Reg, Reg) -> Result<()>,
        M: MacroAssembler,
    {
        let src = self.pop_to_reg(masm, None)?;
        let dst = self.any_gpr(masm)?;
        emit(masm, src.reg, dst)?;
        self.free_reg(src);
        self.stack.push(TypedReg::i32(dst).into());

        Ok(())
    }

    /// Pops a register from the stack and then immediately frees it. Used to
    /// discard values from the last operation, for example.
    pub fn pop_and_free<M: MacroAssembler>(&mut self, masm: &mut M) -> Result<()> {
        let reg = self.pop_to_reg(masm, None)?;
        self.free_reg(reg.reg);
        Ok(())
    }
}
