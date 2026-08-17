//! Data structures for control flow emission.
//!
//! Winch currently doesn't apply any sort of optimizations to control flow, but
//! as a future optimization, for starters, we could perform a look ahead to the
//! next instruction when reaching any of the comparison instructions. If the
//! next instruction is a control instruction, we could avoid emitting
//! a [`crate::masm::MacroAssembler::cmp_with_set`] and instead emit
//! a conditional jump inline when emitting the control flow instruction.
use super::{CodeGenContext, CodeGenError, Emission, OperandSize, Reg, TypedReg};
use crate::{
    CallingConvention,
    abi::{ABI, ABIOperand, ABIResults, ABISig, RetArea},
    masm::{IntCmpKind, MacroAssembler, MemMoveDirection, RegImm, SPOffset},
    reg::writable,
    stack::Val,
};
use anyhow::{Result, anyhow, bail, ensure};
use cranelift_codegen::MachLabel;
use wasmtime_environ::{WasmFuncType, WasmValType};

/// Categorization of the type of the block.
#[derive(Debug, Clone)]
pub(crate) enum BlockType {
    /// Doesn't produce or consume any values.
    Void,
    /// Produces a single value.
    Single(WasmValType),
    /// Consumes multiple values and produces multiple values.
    Func(WasmFuncType),
    /// An already resolved ABI signature.
    ABISig(ABISig),
}

/// Holds all the information about the signature of the block.
#[derive(Debug, Clone)]
pub(crate) struct BlockSig {
    /// The type of the block.
    pub ty: BlockType,
    /// ABI representation of the results of the block.
    results: Option<ABIResults>,
    /// ABI representation of the params of the block interpreted as results.
    params: Option<ABIResults>,
}

impl BlockSig {
    /// Create a new [BlockSig].
    pub fn new(ty: BlockType) -> Self {
        Self {
            ty,
            results: None,
            params: None,
        }
    }

    /// Create a new [BlockSig] from an [ABISig].
    pub fn from_sig(sig: ABISig) -> Self {
        Self {
            ty: BlockType::sig(sig),
            results: None,
            params: None,
        }
    }

    /// Return the ABI representation of the results of the block.
    /// This method will lazily initialize the results if not present.
    pub fn results<M>(&mut self) -> Result<&mut ABIResults>
    where
        M: MacroAssembler,
    {
        if self.ty.is_sig() {
            return match &mut self.ty {
                BlockType::ABISig(sig) => Ok(&mut sig.results),
                _ => unreachable!(),
            };
        }

        if self.results.is_some() {
            return Ok(self.results.as_mut().unwrap());
        }

        let results = match &self.ty {
            BlockType::Void => <M::ABI as ABI>::abi_results(&[], &CallingConvention::Default),
            BlockType::Single(ty) => {
                <M::ABI as ABI>::abi_results(&[*ty], &CallingConvention::Default)
            }
            BlockType::Func(f) => {
                <M::ABI as ABI>::abi_results(f.returns(), &CallingConvention::Default)
            }
            BlockType::ABISig(_) => unreachable!(),
        };

        self.results = Some(results?);
        Ok(self.results.as_mut().unwrap())
    }

    /// Construct an ABI result representation of the params of the block.
    /// This is needed for loops and for handling cases in which params flow as
    /// the block's results, i.e. in the presence of an empty then or else.
    pub fn params<M>(&mut self) -> Result<&mut ABIResults>
    where
        M: MacroAssembler,
    {
        if self.params.is_some() {
            return Ok(self.params.as_mut().unwrap());
        }

        let params_as_results = match &self.ty {
            BlockType::Void | BlockType::Single(_) => {
                <M::ABI as ABI>::abi_results(&[], &CallingConvention::Default)
            }
            BlockType::Func(f) => {
                <M::ABI as ABI>::abi_results(f.params(), &CallingConvention::Default)
            }
            // Once we have created a block type from a known signature, we
            // can't modify its meaning. This should only be used for the
            // function body block, in which case there's no need for treating
            // params as results.
            BlockType::ABISig(_) => unreachable!(),
        };

        self.params = Some(params_as_results?);
        Ok(self.params.as_mut().unwrap())
    }

    /// Returns the signature param count.
    pub fn param_count(&self) -> usize {
        match &self.ty {
            BlockType::Void | BlockType::Single(_) => 0,
            BlockType::Func(f) => f.params().len(),
            BlockType::ABISig(sig) => sig.params_without_retptr().len(),
        }
    }

    /// Returns the signature return count.
    pub fn return_count(&self) -> usize {
        match &self.ty {
            BlockType::Void => 0,
            BlockType::Single(_) => 1,
            BlockType::Func(f) => f.returns().len(),
            BlockType::ABISig(sig) => sig.results().len(),
        }
    }
}

impl BlockType {
    /// Create a [BlockType::Void].
    pub fn void() -> Self {
        Self::Void
    }

    /// Create a [BlockType::Single] from the given [WasmType].
    pub fn single(ty: WasmValType) -> Self {
        Self::Single(ty)
    }

    /// Create a [BlockType::Func] from the given [WasmFuncType].
    pub fn func(ty: WasmFuncType) -> Self {
        Self::Func(ty)
    }

    /// Create a [BlockType::ABISig].
    pub fn sig(sig: ABISig) -> Self {
        Self::ABISig(sig)
    }

    /// Returns true if the type of the block is [BlockType::ABISig].
    pub fn is_sig(&self) -> bool {
        match self {
            Self::ABISig(_) => true,
            _ => false,
        }
    }
}

/// The expected value and machine stack state when entering and exiting the block.
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct StackState {
    /// The base stack pointer offset.
    /// This offset is set when entering the block, after saving any live
    /// registers and locals.
    /// It is calculated by subtracting the size, in bytes, of any block params
    /// to the current stack pointer offset.
    pub base_offset: SPOffset,
    /// The target stack pointer offset.
    /// This offset is calculated by adding the size of the stack results
    /// to the base stack pointer offset.
    pub target_offset: SPOffset,
    /// The base length of the value stack when entering the block.
    /// Which is the current length of the value stack minus any block parameters.
    pub base_len: usize,
    /// The target length of the value stack when exiting the block.
    /// Calculate by adding the number of results to the base value stack
    /// length.
    pub target_len: usize,
}

/// Holds the all the metadata to support the emission
/// of control flow instructions.
#[derive(Debug)]
pub(crate) enum ControlStackFrame {
    If {
        /// The if continuation label.
        cont: MachLabel,
        /// The exit label of the block.
        exit: MachLabel,
        /// The signature of the block.
        sig: BlockSig,
        /// The stack state of the block.
        stack_state: StackState,
        /// Local reachability state when entering the block.
        reachable: bool,
    },
    Else {
        /// The exit label of the block.
        exit: MachLabel,
        /// The signature of the block.
        sig: BlockSig,
        /// The stack state of the block.
        stack_state: StackState,
        /// Local reachability state when entering the block.
        reachable: bool,
    },
    Block {
        /// The block exit label.
        exit: MachLabel,
        /// The signature of the block.
        sig: BlockSig,
        /// The stack state of the block.
        stack_state: StackState,
        /// Exit state of the block.
        ///
        /// This flag is used to determine if a block is a branch
        /// target. By default, this is false, and it's updated when
        /// emitting a `br` or `br_if`.
        is_branch_target: bool,
    },
    Loop {
        /// The start of the Loop.
        head: MachLabel,
        /// The stack state of the block.
        stack_state: StackState,
        /// The signature of the block.
        sig: BlockSig,
    },
}

impl ControlStackFrame {
    /// Returns [`ControlStackFrame`] for an if.
    pub fn r#if<M: MacroAssembler>(
        sig: BlockSig,
        masm: &mut M,
        context: &mut CodeGenContext<Emission>,
    ) -> Result<Self> {
        let mut control = Self::If {
            cont: masm.get_label()?,
            exit: masm.get_label()?,
            sig,
            reachable: context.reachable,
            stack_state: Default::default(),
        };

        control.emit(masm, context)?;
        Ok(control)
    }

    /// Returns [`ControlStackFrame`] for a block.
    pub fn block<M: MacroAssembler>(
        sig: BlockSig,
        masm: &mut M,
        context: &mut CodeGenContext<Emission>,
    ) -> Result<Self> {
        let mut control = Self::Block {
            sig,
            is_branch_target: false,
            exit: masm.get_label()?,
            stack_state: Default::default(),
        };

        control.emit(masm, context)?;
        Ok(control)
    }

    /// Returns [`ControlStackFrame`] for a loop.
    pub fn r#loop<M: MacroAssembler>(
        sig: BlockSig,
        masm: &mut M,
        context: &mut CodeGenContext<Emission>,
    ) -> Result<Self> {
        let mut control = Self::Loop {
            stack_state: Default::default(),
            sig,
            head: masm.get_label()?,
        };

        control.emit(masm, context)?;
        Ok(control)
    }

    fn init<M: MacroAssembler>(
        &mut self,
        masm: &mut M,
        context: &mut CodeGenContext<Emission>,
    ) -> Result<()> {
        self.calculate_stack_state(context, masm)?;
        // If the block has stack results, immediately resolve the return area
        // base.
        if self.results::<M>()?.on_stack() {
            let results_base = self.stack_state().target_offset;
            self.results::<M>()?.set_ret_area(RetArea::sp(results_base));
        }

        if self.is_if() || self.is_loop() {
            // Preemptively handle block params as results so that the params
            // are correctly placed in memory. This is especially
            // important for control flow joins with empty blocks:
            //
            //(module
            //  (func (export "params") (param i32) (result i32)
            //       (i32.const 2)
            //       (if (param i32) (result i32) (local.get 0)
            //       (then))
            //     (i32.const 3)
            //     (i32.add)
            //   )
            //)
            let base_offset = self.stack_state().base_offset;
            if self.params::<M>()?.on_stack() {
                let offset = base_offset.as_u32() + self.params::<M>()?.size();
                self.params::<M>()?
                    .set_ret_area(RetArea::sp(SPOffset::from_u32(offset)));
            }
            Self::top_abi_results_impl(
                self.params::<M>()?,
                context,
                masm,
                |params: &ABIResults, _, _| Ok(params.ret_area().copied()),
            )?;
        }
        Ok(())
    }

    /// Calculates the [StackState] of the block.
    fn calculate_stack_state<M: MacroAssembler>(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
    ) -> Result<()> {
        use ControlStackFrame::*;
        let sig = self.sig();
        // If the block type contains a full [ABISig], do not take into account
        // the params, since these are the params of the function that is
        // currently being compiled and the value stack doesn't currently
        // contain any values anyway.
        let param_count = if sig.ty.is_sig() {
            0
        } else {
            sig.param_count()
        };
        let return_count = sig.return_count();
        ensure!(
            context.stack.len() >= param_count,
            CodeGenError::missing_values_in_stack()
        );
        let results_size = self.results::<M>()?.size();

        // Save any live registers and locals.
        context.spill(masm)?;

        let base_len = context.stack.len() - param_count;
        let stack_consumed = context.stack.sizeof(param_count);
        let current_sp = masm.sp_offset()?;
        let base_offset = SPOffset::from_u32(current_sp.as_u32() - stack_consumed);

        match self {
            If { stack_state, .. } | Block { stack_state, .. } | Loop { stack_state, .. } => {
                stack_state.base_offset = base_offset;
                stack_state.base_len = base_len;
                stack_state.target_offset = SPOffset::from_u32(base_offset.as_u32() + results_size);
                stack_state.target_len = base_len + return_count;
            }
            _ => {}
        }
        Ok(())
    }

    /// This function ensures that the state of the -- machine and value --
    /// stack  is the right one when reaching a control frame branch in which
    /// reachability is restored or when reaching the end of a function in an
    /// unreachable state. This function is intended to be called when handling
    /// an unreachable else or end.
    //
    /// This function will truncate the value stack to the base length of
    /// the control frame and will also set the stack pointer offset to reflect
    /// the offset expected by the target branch.
    ///
    // NB: This method is assumed to be called *before* pushing any block
    // results to the value stack, so that any excess values are cleaned up.
    pub fn ensure_stack_state<M: MacroAssembler>(
        &mut self,
        masm: &mut M,
        context: &mut CodeGenContext<Emission>,
    ) -> Result<()> {
        let state = self.stack_state();
        // This assumes that at jump sites, the machine stack pointer will be
        // adjusted to match the expectations of the target branch (e.g.
        // `target_offset`); after performing the jump, the MacroAssembler
        // implementation will soft-reset the stack pointer offset to its
        // original offset, ensure that other parts of the program have access
        // to the right offset, this is especially important in conditional
        // branches.
        // When restoring reachability we ensure that the MacroAssembler offset
        // is set to match the expectations of the target branch, similar to how
        // the machine stack pointer was adjusted at jump sites.
        masm.reset_stack_pointer(state.target_offset)?;
        // We use the base length, because this function is assumed to be called
        // *before* pushing any results to the value stack. This way, any excess
        // values will be discarded.
        context.truncate_stack_to(state.base_len)
    }

    /// Return the type information of the block.
    pub fn sig(&self) -> &BlockSig {
        use ControlStackFrame::*;
        match self {
            If { sig, .. } | Else { sig, .. } | Loop { sig, .. } | Block { sig, .. } => sig,
        }
    }

    fn emit<M: MacroAssembler>(
        &mut self,
        masm: &mut M,
        context: &mut CodeGenContext<Emission>,
    ) -> Result<()> {
        use ControlStackFrame::*;

        // Do not perform any emissions if we are in an unreachable state.
        if !context.reachable {
            return Ok(());
        }

        match *self {
            If { cont, .. } => {
                // Pop the condition value.
                // Because in the case of Self::If, Self::init, will top the
                // branch params, we exclude any result registers from being
                // used as the branch test.
                let top = context.without::<Result<TypedReg>, _, _>(
                    self.params::<M>()?.regs(),
                    masm,
                    |cx, masm| cx.pop_to_reg(masm, None),
                )??;
                self.init(masm, context)?;
                masm.branch(
                    IntCmpKind::Eq,
                    top.reg,
                    top.reg.into(),
                    cont,
                    OperandSize::S32,
                )?;
                context.free_reg(top);
                Ok(())
            }
            Block { .. } => self.init(masm, context),
            Loop { head, .. } => {
                self.init(masm, context)?;
                masm.bind(head)?;
                Ok(())
            }
            _ => Err(anyhow!(CodeGenError::if_control_frame_expected())),
        }
    }

    /// Handles the else branch if the current control stack frame is
    /// [`ControlStackFrame::If`].
    pub fn emit_else<M: MacroAssembler>(
        &mut self,
        masm: &mut M,
        context: &mut CodeGenContext<Emission>,
    ) -> Result<()> {
        ensure!(self.is_if(), CodeGenError::if_control_frame_expected());
        let state = self.stack_state();

        ensure!(
            state.target_len == context.stack.len(),
            CodeGenError::control_frame_state_mismatch()
        );
        self.pop_abi_results(context, masm, |results, _, _| {
            Ok(results.ret_area().copied())
        })?;
        masm.jmp(*self.exit_label().unwrap())?;
        self.bind_else(masm, context)?;
        Ok(())
    }

    /// Binds the else branch label and converts `self` to
    /// [`ControlStackFrame::Else`].
    pub fn bind_else<M: MacroAssembler>(
        &mut self,
        masm: &mut M,
        context: &mut CodeGenContext<Emission>,
    ) -> Result<()> {
        use ControlStackFrame::*;
        match self {
            If {
                cont,
                sig,
                stack_state,
                exit,
                ..
            } => {
                // Bind the else branch.
                masm.bind(*cont)?;

                // Push the abi results to the value stack, so that they are
                // used as params for the else branch. At the beginning of the
                // if block, any params are preemptively resolved as results;
                // when reaching the else all params are already materialized as
                // stack results. As part of ensuring the right state when
                // entering the else branch, the following snippet also soft
                // resets the stack pointer so that it matches the expectations
                // of the else branch: the stack pointer is expected to be at
                // the base stack pointer, plus the params stack size in bytes.
                let params_size = sig.params::<M>()?.size();
                context.push_abi_results::<M, _>(sig.params::<M>()?, masm, |params, _, _| {
                    params.ret_area().copied()
                })?;
                masm.reset_stack_pointer(SPOffset::from_u32(
                    stack_state.base_offset.as_u32() + params_size,
                ))?;

                // Update the stack control frame with an else control frame.
                *self = ControlStackFrame::Else {
                    exit: *exit,
                    stack_state: *stack_state,
                    reachable: context.reachable,
                    sig: sig.clone(),
                };
            }
            _ => bail!(CodeGenError::if_control_frame_expected()),
        }
        Ok(())
    }

    /// Handles the end of a control stack frame.
    pub fn emit_end<M: MacroAssembler>(
        &mut self,
        masm: &mut M,
        context: &mut CodeGenContext<Emission>,
    ) -> Result<()> {
        use ControlStackFrame::*;
        match self {
            If { stack_state, .. } | Else { stack_state, .. } | Block { stack_state, .. } => {
                ensure!(
                    stack_state.target_len == context.stack.len(),
                    CodeGenError::control_frame_state_mismatch()
                );
                // Before binding the exit label, we handle the block results.
                self.pop_abi_results(context, masm, |results, _, _| {
                    Ok(results.ret_area().copied())
                })?;
                self.bind_end(masm, context)?;
            }
            Loop { stack_state, .. } => {
                ensure!(
                    stack_state.target_len == context.stack.len(),
                    CodeGenError::control_frame_state_mismatch()
                );
            }
        };

        Ok(())
    }

    /// Binds the exit label of the current control stack frame and pushes the
    /// ABI results to the value stack.
    pub fn bind_end<M: MacroAssembler>(
        &mut self,
        masm: &mut M,
        context: &mut CodeGenContext<Emission>,
    ) -> Result<()> {
        self.push_abi_results(context, masm)?;
        self.bind_exit_label(masm)
    }

    /// Binds the exit label of the control stack frame.
    pub fn bind_exit_label<M: MacroAssembler>(&self, masm: &mut M) -> Result<()> {
        use ControlStackFrame::*;
        match self {
            // We use an explicit label to track the exit of an if block. In case there's no
            // else, we bind the if's continuation block to make sure that any jumps from the if
            // condition are reachable and we bind the explicit exit label as well to ensure that any
            // branching instructions are able to correctly reach the block's end.
            If { cont, .. } => masm.bind(*cont)?,
            _ => {}
        }
        if let Some(label) = self.exit_label() {
            masm.bind(*label)?;
        }
        Ok(())
    }

    /// Returns the continuation label of the current control stack frame.
    pub fn label(&self) -> &MachLabel {
        use ControlStackFrame::*;

        match self {
            If { exit, .. } | Else { exit, .. } | Block { exit, .. } => exit,
            Loop { head, .. } => head,
        }
    }

    /// Returns the exit label of the current control stack frame. Note that
    /// this is similar to [`ControlStackFrame::label`], with the only difference that it
    /// returns `None` for `Loop` since its label doesn't represent an exit.
    pub fn exit_label(&self) -> Option<&MachLabel> {
        use ControlStackFrame::*;

        match self {
            If { exit, .. } | Else { exit, .. } | Block { exit, .. } => Some(exit),
            Loop { .. } => None,
        }
    }

    /// Set the current control stack frame as a branch target.
    pub fn set_as_target(&mut self) {
        match self {
            ControlStackFrame::Block {
                is_branch_target, ..
            } => {
                *is_branch_target = true;
            }
            _ => {}
        }
    }

    /// Returns [`crate::abi::ABIResults`] of the control stack frame
    /// block.
    pub fn results<M>(&mut self) -> Result<&mut ABIResults>
    where
        M: MacroAssembler,
    {
        use ControlStackFrame::*;

        match self {
            If { sig, .. } | Else { sig, .. } | Block { sig, .. } => sig.results::<M>(),
            Loop { sig, .. } => sig.params::<M>(),
        }
    }

    /// Returns the block params interpreted as [crate::abi::ABIResults].
    pub fn params<M>(&mut self) -> Result<&mut ABIResults>
    where
        M: MacroAssembler,
    {
        use ControlStackFrame::*;
        match self {
            If { sig, .. } | Else { sig, .. } | Block { sig, .. } | Loop { sig, .. } => {
                sig.params::<M>()
            }
        }
    }

    /// Orchestrates how block results are handled.
    /// Results are handled in reverse order, starting from register results
    /// continuing to memory values. This guarantees that the stack ordering
    /// invariant is maintained. See [ABIResults] for more details.
    ///
    /// This function will iterate through each result and invoke the provided
    /// callback if there are results on the stack.
    ///
    /// Calculating the return area involves ensuring that there's enough stack
    /// space to store the block's results. To make the process of handling
    /// multiple results easier, this function will save all live registers and
    /// locals right after handling any register results. This will ensure that
    /// the top `n` values in the value stack are correctly placed in the memory
    /// locations corresponding to multiple stack results. Once the iteration
    /// over all the results is done, the stack result area of the block will be
    /// updated.
    pub fn pop_abi_results<M, F>(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
        calculate_ret_area: F,
    ) -> Result<()>
    where
        M: MacroAssembler,
        F: FnMut(&ABIResults, &mut CodeGenContext<Emission>, &mut M) -> Result<Option<RetArea>>,
    {
        Self::pop_abi_results_impl(self.results::<M>()?, context, masm, calculate_ret_area)
    }

    /// Shared implementation for popping the ABI results.
    /// This is needed because, in some cases, params must be interpreted and
    /// used as the results of the block. When emitting code at control flow
    /// joins, the block params are interpreted as results, to ensure that they
    /// can correctly "flow" as the results of the block. This is especially
    /// important in the presence of empty then, else and loop blocks. This
    /// interpretation is an internal detail of the control module, and having
    /// a shared implementation allows the caller to decide how the
    /// results should be interpreted.
    pub fn pop_abi_results_impl<M, F>(
        results: &mut ABIResults,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
        mut calculate_ret_area: F,
    ) -> Result<()>
    where
        M: MacroAssembler,
        F: FnMut(&ABIResults, &mut CodeGenContext<Emission>, &mut M) -> Result<Option<RetArea>>,
    {
        let mut iter = results.operands().iter().rev().peekable();

        while let Some(ABIOperand::Reg { reg, .. }) = iter.peek() {
            let TypedReg { reg, .. } = context.pop_to_reg(masm, Some(*reg))?;
            context.free_reg(reg);
            iter.next().unwrap();
        }

        let ret_area = calculate_ret_area(results, context, masm)?;

        let retptr = Self::maybe_load_retptr(ret_area.as_ref(), &results, context, masm)?;
        if let Some(area) = ret_area {
            if area.is_sp() {
                Self::ensure_ret_area(&area, context, masm)?;
            }
        }

        if let Some(retptr) = retptr {
            while let Some(ABIOperand::Stack { offset, .. }) = iter.peek() {
                let addr = masm.address_at_reg(retptr, *offset)?;
                context.pop_to_addr(masm, addr)?;
                iter.next().unwrap();
            }
            context.free_reg(retptr);
        }

        if let Some(area) = ret_area {
            if area.is_sp() {
                Self::adjust_stack_results(area, results, context, masm)?;
            }
        }

        Ok(())
    }

    /// Convenience wrapper around [CodeGenContext::push_abi_results] using the
    /// results of the current frame.
    fn push_abi_results<M>(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
    ) -> Result<()>
    where
        M: MacroAssembler,
    {
        context.push_abi_results(self.results::<M>()?, masm, |results, _, _| {
            results.ret_area().copied()
        })
    }

    /// Preemptively handles the ABI results of the current frame.
    /// This function is meant to be used when emitting control flow with joins,
    /// in which it's not possible to know at compile time which branch will be
    /// taken.
    pub fn top_abi_results<M, F>(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
        calculate_ret_area: F,
    ) -> Result<()>
    where
        M: MacroAssembler,
        F: FnMut(&ABIResults, &mut CodeGenContext<Emission>, &mut M) -> Result<Option<RetArea>>,
    {
        Self::top_abi_results_impl::<M, _>(self.results::<M>()?, context, masm, calculate_ret_area)
    }

    /// Internal implementation of [Self::top_abi_results].
    /// See [Self::pop_abi_results_impl] on why an internal implementation is
    /// needed.
    fn top_abi_results_impl<M, F>(
        results: &mut ABIResults,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
        mut calculate_ret_area: F,
    ) -> Result<()>
    where
        M: MacroAssembler,
        F: FnMut(&ABIResults, &mut CodeGenContext<Emission>, &mut M) -> Result<Option<RetArea>>,
    {
        let mut area = None;
        Self::pop_abi_results_impl::<M, _>(results, context, masm, |r, context, masm| {
            area = calculate_ret_area(r, context, masm)?;
            Ok(area)
        })?;
        // Use the previously calculated area to ensure that the ret area is
        // kept in sync between both operations.
        context.push_abi_results::<M, _>(results, masm, |_, _, _| area)
    }

    // If the results on the stack are handled via the stack pointer, ensure
    // that the stack results are correctly located. In general, since values in
    // the value stack are spilled when exiting the block, the top `n` entries
    // in the value stack, representing the `n` stack results of the block are
    // almost correctly located. However, since constants are not
    // spilled, their presence complicate block exits. For this reason, the
    // last step for finalizing multiple block results involves:
    // * Scanning the value stack from oldest to newest memory values and
    // calculating the source and destination of each value, if the source
    // is closer to the stack pointer (greater) than the destination,
    // perform a memory move of the bytes to its destination, else stop,
    // because the memory values are in place.
    // * Scanning the value stack from newest to oldest and calculating the
    // source and destination of each value, if the source is closer to the
    // frame pointer (less) than the destination, perform a memory move of
    // the bytes to its destination, else stop, because the memory values
    // are in place.
    // * Lastly, iterate over the top `n` elements of the value stack,
    // and spill any constant values, placing them in their respective
    // memory location.
    //
    // The implementation in Winch is inspired by how this is handled in
    // SpiderMonkey's WebAssembly Baseline Compiler:
    // https://wingolog.org/archives/2020/04/03/multi-value-webassembly-in-firefox-from-1-to-n
    fn adjust_stack_results<M>(
        ret_area: RetArea,
        results: &ABIResults,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
    ) -> Result<()>
    where
        M: MacroAssembler,
    {
        ensure!(ret_area.is_sp(), CodeGenError::sp_addressing_expected());
        let results_offset = ret_area.unwrap_sp();

        // Start iterating from memory values that are closer to the
        // frame pointer (oldest entries first).
        for (i, operand) in results.operands().iter().enumerate() {
            if operand.is_reg() {
                break;
            }

            let value_index = (context.stack.len() - results.stack_operands_len()) + i;
            let val = context.stack.inner()[value_index];

            match (val, operand) {
                (Val::Memory(mem), ABIOperand::Stack { offset, size, .. }) => {
                    let dst = results_offset.as_u32() - *offset;
                    let src = mem.slot.offset;

                    // Values are moved from lower (SP) to higher (FP)
                    // addresses.
                    if src.as_u32() <= dst {
                        break;
                    }

                    masm.memmove(
                        src,
                        SPOffset::from_u32(dst),
                        *size,
                        MemMoveDirection::LowToHigh,
                    )?;
                }
                _ => {}
            }
        }

        // Start iterating from memory values that are closer to the
        // stack pointer (newest entries first).
        for (i, operand) in results
            .operands()
            .iter()
            .rev()
            // Skip any register results.
            .skip(results.regs().len())
            .enumerate()
        {
            let value_index = context.stack.len() - i - 1;
            let val = context.stack.inner()[value_index];
            match (val, operand) {
                (Val::Memory(mem), ABIOperand::Stack { offset, size, .. }) => {
                    let dst = results_offset.as_u32() - *offset;
                    let src = mem.slot.offset;

                    // Values are moved from higher (FP) to lower (SP)
                    // addresses.
                    if src.as_u32() >= dst {
                        break;
                    }

                    masm.memmove(
                        src,
                        SPOffset::from_u32(dst),
                        *size,
                        MemMoveDirection::HighToLow,
                    )?;
                }
                _ => {}
            }
        }

        // Finally store any constants in the value stack in their respective
        // locations.
        for operand in results
            .operands()
            .iter()
            .take(results.stack_operands_len())
            .rev()
        {
            // If we want to do this, we should start from newest, essentially from top to
            // bottom in the iteration of the operands.
            match (operand, context.stack.peek().unwrap()) {
                (ABIOperand::Stack { ty, offset, .. }, Val::I32(v)) => {
                    let addr = masm
                        .address_from_sp(SPOffset::from_u32(results_offset.as_u32() - *offset))?;
                    masm.store(RegImm::i32(*v), addr, (*ty).try_into()?)?;
                }
                (ABIOperand::Stack { ty, offset, .. }, Val::I64(v)) => {
                    let addr = masm
                        .address_from_sp(SPOffset::from_u32(results_offset.as_u32() - *offset))?;
                    masm.store(RegImm::i64(*v), addr, (*ty).try_into()?)?;
                }
                (ABIOperand::Stack { ty, offset, .. }, Val::F32(v)) => {
                    let addr = masm
                        .address_from_sp(SPOffset::from_u32(results_offset.as_u32() - *offset))?;
                    masm.store(RegImm::f32(v.bits()), addr, (*ty).try_into()?)?;
                }
                (ABIOperand::Stack { ty, offset, .. }, Val::F64(v)) => {
                    let addr = masm
                        .address_from_sp(SPOffset::from_u32(results_offset.as_u32() - *offset))?;
                    masm.store(RegImm::f64(v.bits()), addr, (*ty).try_into()?)?;
                }
                (ABIOperand::Stack { ty, offset, .. }, Val::V128(v)) => {
                    let addr = masm
                        .address_from_sp(SPOffset::from_u32(results_offset.as_u32() - *offset))?;
                    masm.store(RegImm::v128(*v), addr, (*ty).try_into()?)?;
                }
                (_, v) => debug_assert!(v.is_mem()),
            }

            let _ = context.stack.pop().unwrap();
        }

        // Adjust any excess stack space: the stack space after handling the
        // block's results should be the exact amount needed by the return area.
        ensure!(
            masm.sp_offset()?.as_u32() >= results_offset.as_u32(),
            CodeGenError::invalid_sp_offset()
        );
        masm.free_stack(masm.sp_offset()?.as_u32() - results_offset.as_u32())?;
        Ok(())
    }

    /// Ensures that there is enough space for return values on the stack.
    /// This function is called at the end of all blocks and when branching from
    /// within blocks.
    fn ensure_ret_area<M>(
        ret_area: &RetArea,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
    ) -> Result<()>
    where
        M: MacroAssembler,
    {
        ensure!(ret_area.is_sp(), CodeGenError::sp_addressing_expected());
        // Save any live registers and locals when exiting the block to ensure
        // that the respective values are correctly located in memory.
        // See [Self::adjust_stack_results] for more details.
        context.spill(masm)?;
        if ret_area.unwrap_sp() > masm.sp_offset()? {
            masm.reserve_stack(ret_area.unwrap_sp().as_u32() - masm.sp_offset()?.as_u32())?
        }

        Ok(())
    }

    /// Loads the return pointer, if it exists, into the next available register.
    fn maybe_load_retptr<M>(
        ret_area: Option<&RetArea>,
        results: &ABIResults,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
    ) -> Result<Option<Reg>>
    where
        M: MacroAssembler,
    {
        if let Some(area) = ret_area {
            match area {
                RetArea::Slot(slot) => {
                    let base = context.without::<Result<Reg>, M, _>(
                        results.regs(),
                        masm,
                        |cx, masm| cx.any_gpr(masm),
                    )??;
                    let local_addr = masm.local_address(&slot)?;
                    masm.load_ptr(local_addr, writable!(base))?;
                    Ok(Some(base))
                }
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    /// This function is used at the end of unreachable code handling
    /// to determine if the reachability status should be updated.
    pub fn is_next_sequence_reachable(&self) -> bool {
        use ControlStackFrame::*;

        match self {
            // For if/else, the reachability of the next sequence is determined
            // by the reachability state at the start of the block. An else
            // block will be reachable if the if block is also reachable at
            // entry.
            If { reachable, .. } | Else { reachable, .. } => *reachable,
            // For blocks, the reachability of the next sequence is determined
            // if they're a branch target.
            Block {
                is_branch_target, ..
            } => *is_branch_target,
            // Loops are not used for reachability analysis,
            // given that they don't have exit branches.
            Loop { .. } => false,
        }
    }

    /// Returns a reference to the [StackState] of the block.
    pub fn stack_state(&self) -> &StackState {
        use ControlStackFrame::*;
        match self {
            If { stack_state, .. }
            | Else { stack_state, .. }
            | Block { stack_state, .. }
            | Loop { stack_state, .. } => stack_state,
        }
    }

    /// Returns true if the current frame is [ControlStackFrame::If].
    pub fn is_if(&self) -> bool {
        match self {
            Self::If { .. } => true,
            _ => false,
        }
    }

    /// Returns true if the current frame is [ControlStackFrame::Loop].
    pub fn is_loop(&self) -> bool {
        match self {
            Self::Loop { .. } => true,
            _ => false,
        }
    }

    /// Returns true if the current stack pointer is unbalanced
    /// relative to the the expected control frame stack pointer
    /// offset. The stack pointer is considered unbalanced relative
    /// to the control frame if the stack pointer is greater than the
    /// the target stack pointer offset expected by the control frame.
    pub fn unbalanced<M: MacroAssembler>(&self, masm: &mut M) -> Result<bool> {
        Ok(masm.sp_offset()? > self.stack_state().target_offset)
    }
}
