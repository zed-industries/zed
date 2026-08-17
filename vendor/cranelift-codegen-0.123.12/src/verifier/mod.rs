//! A verifier for ensuring that functions are well formed.
//! It verifies:
//!
//! block integrity
//!
//! - All instructions reached from the `block_insts` iterator must belong to
//!   the block as reported by `inst_block()`.
//! - Every block must end in a terminator instruction, and no other instruction
//!   can be a terminator.
//! - Every value in the `block_params` iterator belongs to the block as reported by `value_block`.
//!
//! Instruction integrity
//!
//! - The instruction format must match the opcode.
//! - All result values must be created for multi-valued instructions.
//! - All referenced entities must exist. (Values, blocks, stack slots, ...)
//! - Instructions must not reference (eg. branch to) the entry block.
//!
//! SSA form
//!
//! - Values must be defined by an instruction that exists and that is inserted in
//!   a block, or be an argument of an existing block.
//! - Values used by an instruction must dominate the instruction.
//!
//! Control flow graph and dominator tree integrity:
//!
//! - All predecessors in the CFG must be branches to the block.
//! - All branches to a block must be present in the CFG.
//! - A recomputed dominator tree is identical to the existing one.
//! - The entry block must not be a cold block.
//!
//! Type checking
//!
//! - Compare input and output values against the opcode's type constraints.
//!   For polymorphic opcodes, determine the controlling type variable first.
//! - Branches and jumps must pass arguments to destination blocks that match the
//!   expected types exactly. The number of arguments must match.
//! - All blocks in a jump table must take no arguments.
//! - Function calls are type checked against their signature.
//! - The entry block must take arguments that match the signature of the current
//!   function.
//! - All return instructions must have return value operands matching the current
//!   function signature.
//!
//! Global values
//!
//! - Detect cycles in global values.
//! - Detect use of 'vmctx' global value when no corresponding parameter is defined.
//!
//! Memory types
//!
//! - Ensure that struct fields are in offset order.
//! - Ensure that struct fields are completely within the overall
//!   struct size, and do not overlap.
//!
//! TODO:
//! Ad hoc checking
//!
//! - Stack slot loads and stores must be in-bounds.
//! - Immediate constraints for certain opcodes, like `udiv_imm v3, 0`.
//! - `Insertlane` and `extractlane` instructions have immediate lane numbers that must be in
//!   range for their polymorphic type.
//! - Swizzle and shuffle instructions take a variable number of lane arguments. The number
//!   of arguments must match the destination type, and the lane indexes must be in range.

use crate::dbg::DisplayList;
use crate::dominator_tree::DominatorTree;
use crate::dominator_tree::DominatorTreePreorder;
use crate::entity::SparseSet;
use crate::flowgraph::{BlockPredecessor, ControlFlowGraph};
use crate::ir::ExceptionTableItem;
use crate::ir::entities::AnyEntity;
use crate::ir::instructions::{CallInfo, InstructionFormat, ResolvedConstraint};
use crate::ir::{self, ArgumentExtension, BlockArg, ExceptionTable};
use crate::ir::{
    ArgumentPurpose, Block, Constant, DynamicStackSlot, FuncRef, Function, GlobalValue, Inst,
    JumpTable, MemFlags, MemoryTypeData, Opcode, SigRef, StackSlot, Type, Value, ValueDef,
    ValueList, types,
};
use crate::isa::TargetIsa;
use crate::print_errors::pretty_verifier_error;
use crate::settings::FlagsOrIsa;
use crate::timing;
use alloc::collections::BTreeSet;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{self, Display, Formatter};

/// A verifier error.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerifierError {
    /// The entity causing the verifier error.
    pub location: AnyEntity,
    /// Optionally provide some context for the given location; e.g., for `inst42` provide
    /// `Some("v3 = iconst.i32 0")` for more comprehensible errors.
    pub context: Option<String>,
    /// The error message.
    pub message: String,
}

// This is manually implementing Error and Display instead of using thiserror to reduce the amount
// of dependencies used by Cranelift.
impl std::error::Error for VerifierError {}

impl Display for VerifierError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.context {
            None => write!(f, "{}: {}", self.location, self.message),
            Some(context) => write!(f, "{} ({}): {}", self.location, context, self.message),
        }
    }
}

/// Convenience converter for making error-reporting less verbose.
///
/// Converts a tuple of `(location, context, message)` to a `VerifierError`.
/// ```
/// use cranelift_codegen::verifier::VerifierErrors;
/// use cranelift_codegen::ir::Inst;
/// let mut errors = VerifierErrors::new();
/// errors.report((Inst::from_u32(42), "v3 = iadd v1, v2", "iadd cannot be used with values of this type"));
/// // note the double parenthenses to use this syntax
/// ```
impl<L, C, M> From<(L, C, M)> for VerifierError
where
    L: Into<AnyEntity>,
    C: Into<String>,
    M: Into<String>,
{
    fn from(items: (L, C, M)) -> Self {
        let (location, context, message) = items;
        Self {
            location: location.into(),
            context: Some(context.into()),
            message: message.into(),
        }
    }
}

/// Convenience converter for making error-reporting less verbose.
///
/// Same as above but without `context`.
impl<L, M> From<(L, M)> for VerifierError
where
    L: Into<AnyEntity>,
    M: Into<String>,
{
    fn from(items: (L, M)) -> Self {
        let (location, message) = items;
        Self {
            location: location.into(),
            context: None,
            message: message.into(),
        }
    }
}

/// Result of a step in the verification process.
///
/// Functions that return `VerifierStepResult` should also take a
/// mutable reference to `VerifierErrors` as argument in order to report
/// errors.
///
/// Here, `Ok` represents a step that **did not lead to a fatal error**,
/// meaning that the verification process may continue. However, other (non-fatal)
/// errors might have been reported through the previously mentioned `VerifierErrors`
/// argument.
pub type VerifierStepResult = Result<(), ()>;

/// Result of a verification operation.
///
/// Unlike `VerifierStepResult` which may be `Ok` while still having reported
/// errors, this type always returns `Err` if an error (fatal or not) was reported.
pub type VerifierResult<T> = Result<T, VerifierErrors>;

/// List of verifier errors.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct VerifierErrors(pub Vec<VerifierError>);

// This is manually implementing Error and Display instead of using thiserror to reduce the amount
// of dependencies used by Cranelift.
impl std::error::Error for VerifierErrors {}

impl VerifierErrors {
    /// Return a new `VerifierErrors` struct.
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Return whether no errors were reported.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return whether one or more errors were reported.
    #[inline]
    pub fn has_error(&self) -> bool {
        !self.0.is_empty()
    }

    /// Return a `VerifierStepResult` that is fatal if at least one error was reported,
    /// and non-fatal otherwise.
    #[inline]
    pub fn as_result(&self) -> VerifierStepResult {
        if self.is_empty() { Ok(()) } else { Err(()) }
    }

    /// Report an error, adding it to the list of errors.
    pub fn report(&mut self, error: impl Into<VerifierError>) {
        self.0.push(error.into());
    }

    /// Report a fatal error and return `Err`.
    pub fn fatal(&mut self, error: impl Into<VerifierError>) -> VerifierStepResult {
        self.report(error);
        Err(())
    }

    /// Report a non-fatal error and return `Ok`.
    pub fn nonfatal(&mut self, error: impl Into<VerifierError>) -> VerifierStepResult {
        self.report(error);
        Ok(())
    }
}

impl From<Vec<VerifierError>> for VerifierErrors {
    fn from(v: Vec<VerifierError>) -> Self {
        Self(v)
    }
}

impl From<VerifierErrors> for Vec<VerifierError> {
    fn from(errors: VerifierErrors) -> Vec<VerifierError> {
        errors.0
    }
}

impl From<VerifierErrors> for VerifierResult<()> {
    fn from(errors: VerifierErrors) -> VerifierResult<()> {
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Display for VerifierErrors {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for err in &self.0 {
            writeln!(f, "- {err}")?;
        }
        Ok(())
    }
}

/// Verify `func`.
pub fn verify_function<'a, FOI: Into<FlagsOrIsa<'a>>>(
    func: &Function,
    fisa: FOI,
) -> VerifierResult<()> {
    let _tt = timing::verifier();
    let mut errors = VerifierErrors::default();
    let verifier = Verifier::new(func, fisa.into());
    let result = verifier.run(&mut errors);
    if errors.is_empty() {
        result.unwrap();
        Ok(())
    } else {
        Err(errors)
    }
}

/// Verify `func` after checking the integrity of associated context data structures `cfg` and
/// `domtree`.
pub fn verify_context<'a, FOI: Into<FlagsOrIsa<'a>>>(
    func: &Function,
    cfg: &ControlFlowGraph,
    domtree: &DominatorTree,
    fisa: FOI,
    errors: &mut VerifierErrors,
) -> VerifierStepResult {
    let _tt = timing::verifier();
    let verifier = Verifier::new(func, fisa.into());
    if cfg.is_valid() {
        verifier.cfg_integrity(cfg, errors)?;
    }
    if domtree.is_valid() {
        verifier.domtree_integrity(domtree, errors)?;
    }
    verifier.run(errors)
}

#[derive(Clone, Copy, Debug)]
enum BlockCallTargetType {
    Normal,
    ExNormalRet,
    Exception,
}

struct Verifier<'a> {
    func: &'a Function,
    expected_cfg: ControlFlowGraph,
    expected_domtree_preorder: DominatorTreePreorder,
    expected_domtree: DominatorTree,
    isa: Option<&'a dyn TargetIsa>,
}

impl<'a> Verifier<'a> {
    pub fn new(func: &'a Function, fisa: FlagsOrIsa<'a>) -> Self {
        let expected_cfg = ControlFlowGraph::with_function(func);
        let expected_domtree = DominatorTree::with_function(func, &expected_cfg);
        let mut expected_domtree_preorder = DominatorTreePreorder::new();
        expected_domtree_preorder.compute(&expected_domtree);
        Self {
            func,
            expected_cfg,
            expected_domtree,
            expected_domtree_preorder,
            isa: fisa.isa,
        }
    }

    /// Determine a contextual error string for an instruction.
    #[inline]
    fn context(&self, inst: Inst) -> String {
        self.func.dfg.display_inst(inst).to_string()
    }

    // Check for:
    //  - cycles in the global value declarations.
    //  - use of 'vmctx' when no special parameter declares it.
    fn verify_global_values(&self, errors: &mut VerifierErrors) -> VerifierStepResult {
        let mut cycle_seen = false;
        let mut seen = SparseSet::new();

        'gvs: for gv in self.func.global_values.keys() {
            seen.clear();
            seen.insert(gv);

            let mut cur = gv;
            loop {
                match self.func.global_values[cur] {
                    ir::GlobalValueData::Load { base, .. }
                    | ir::GlobalValueData::IAddImm { base, .. } => {
                        if seen.insert(base).is_some() {
                            if !cycle_seen {
                                errors.report((
                                    gv,
                                    format!("global value cycle: {}", DisplayList(seen.as_slice())),
                                ));
                                // ensures we don't report the cycle multiple times
                                cycle_seen = true;
                            }
                            continue 'gvs;
                        }

                        cur = base;
                    }
                    _ => break,
                }
            }

            match self.func.global_values[gv] {
                ir::GlobalValueData::VMContext { .. } => {
                    if self
                        .func
                        .special_param(ir::ArgumentPurpose::VMContext)
                        .is_none()
                    {
                        errors.report((gv, format!("undeclared vmctx reference {gv}")));
                    }
                }
                ir::GlobalValueData::IAddImm {
                    base, global_type, ..
                } => {
                    if !global_type.is_int() {
                        errors.report((
                            gv,
                            format!("iadd_imm global value with non-int type {global_type}"),
                        ));
                    } else if let Some(isa) = self.isa {
                        let base_type = self.func.global_values[base].global_type(isa);
                        if global_type != base_type {
                            errors.report((
                                gv,
                                format!(
                                    "iadd_imm type {global_type} differs from operand type {base_type}"
                                ),
                            ));
                        }
                    }
                }
                ir::GlobalValueData::Load { base, .. } => {
                    if let Some(isa) = self.isa {
                        let base_type = self.func.global_values[base].global_type(isa);
                        let pointer_type = isa.pointer_type();
                        if base_type != pointer_type {
                            errors.report((
                                gv,
                                format!(
                                    "base {base} has type {base_type}, which is not the pointer type {pointer_type}"
                                ),
                            ));
                        }
                    }
                }
                _ => {}
            }
        }

        // Invalid global values shouldn't stop us from verifying the rest of the function
        Ok(())
    }

    fn verify_memory_types(&self, errors: &mut VerifierErrors) -> VerifierStepResult {
        // Verify that all fields are statically-sized and lie within
        // the struct, do not overlap, and are in offset order
        for (mt, mt_data) in &self.func.memory_types {
            match mt_data {
                MemoryTypeData::Struct { size, fields } => {
                    let mut last_offset = 0;
                    for field in fields {
                        if field.offset < last_offset {
                            errors.report((
                                mt,
                                format!(
                                    "memory type {} has a field at offset {}, which is out-of-order",
                                    mt, field.offset
                                ),
                            ));
                        }
                        last_offset = match field.offset.checked_add(u64::from(field.ty.bytes())) {
                            Some(o) => o,
                            None => {
                                errors.report((
                                        mt,
                                        format!(
                                            "memory type {} has a field at offset {} of size {}; offset plus size overflows a u64",
                                            mt, field.offset, field.ty.bytes()),
                                ));
                                break;
                            }
                        };

                        if last_offset > *size {
                            errors.report((
                                        mt,
                                        format!(
                                            "memory type {} has a field at offset {} of size {} that overflows the struct size {}",
                                            mt, field.offset, field.ty.bytes(), *size),
                                          ));
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Check that the given block can be encoded as a BB, by checking that only
    /// branching instructions are ending the block.
    fn encodable_as_bb(&self, block: Block, errors: &mut VerifierErrors) -> VerifierStepResult {
        match self.func.is_block_basic(block) {
            Ok(()) => Ok(()),
            Err((inst, message)) => errors.fatal((inst, self.context(inst), message)),
        }
    }

    fn block_integrity(
        &self,
        block: Block,
        inst: Inst,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        let is_terminator = self.func.dfg.insts[inst].opcode().is_terminator();
        let is_last_inst = self.func.layout.last_inst(block) == Some(inst);

        if is_terminator && !is_last_inst {
            // Terminating instructions only occur at the end of blocks.
            return errors.fatal((
                inst,
                self.context(inst),
                format!("a terminator instruction was encountered before the end of {block}"),
            ));
        }
        if is_last_inst && !is_terminator {
            return errors.fatal((block, "block does not end in a terminator instruction"));
        }

        // Instructions belong to the correct block.
        let inst_block = self.func.layout.inst_block(inst);
        if inst_block != Some(block) {
            return errors.fatal((
                inst,
                self.context(inst),
                format!("should belong to {block} not {inst_block:?}"),
            ));
        }

        // Parameters belong to the correct block.
        for &arg in self.func.dfg.block_params(block) {
            match self.func.dfg.value_def(arg) {
                ValueDef::Param(arg_block, _) => {
                    if block != arg_block {
                        return errors.fatal((arg, format!("does not belong to {block}")));
                    }
                }
                _ => {
                    return errors.fatal((arg, "expected an argument, found a result"));
                }
            }
        }

        Ok(())
    }

    fn instruction_integrity(&self, inst: Inst, errors: &mut VerifierErrors) -> VerifierStepResult {
        let inst_data = &self.func.dfg.insts[inst];
        let dfg = &self.func.dfg;

        // The instruction format matches the opcode
        if inst_data.opcode().format() != InstructionFormat::from(inst_data) {
            return errors.fatal((
                inst,
                self.context(inst),
                "instruction opcode doesn't match instruction format",
            ));
        }

        let expected_num_results = dfg.num_expected_results_for_verifier(inst);

        // All result values for multi-valued instructions are created
        let got_results = dfg.inst_results(inst).len();
        if got_results != expected_num_results {
            return errors.fatal((
                inst,
                self.context(inst),
                format!("expected {expected_num_results} result values, found {got_results}"),
            ));
        }

        self.verify_entity_references(inst, errors)
    }

    fn verify_entity_references(
        &self,
        inst: Inst,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        use crate::ir::instructions::InstructionData::*;

        for arg in self.func.dfg.inst_values(inst) {
            self.verify_inst_arg(inst, arg, errors)?;

            // All used values must be attached to something.
            let original = self.func.dfg.resolve_aliases(arg);
            if !self.func.dfg.value_is_attached(original) {
                errors.report((
                    inst,
                    self.context(inst),
                    format!("argument {arg} -> {original} is not attached"),
                ));
            }
        }

        for &res in self.func.dfg.inst_results(inst) {
            self.verify_inst_result(inst, res, errors)?;
        }

        match self.func.dfg.insts[inst] {
            MultiAry { ref args, .. } => {
                self.verify_value_list(inst, args, errors)?;
            }
            Jump { destination, .. } => {
                self.verify_block(inst, destination.block(&self.func.dfg.value_lists), errors)?;
            }
            Brif {
                arg,
                blocks: [block_then, block_else],
                ..
            } => {
                self.verify_value(inst, arg, errors)?;
                self.verify_block(inst, block_then.block(&self.func.dfg.value_lists), errors)?;
                self.verify_block(inst, block_else.block(&self.func.dfg.value_lists), errors)?;
            }
            BranchTable { table, .. } => {
                self.verify_jump_table(inst, table, errors)?;
            }
            Call {
                func_ref, ref args, ..
            } => {
                self.verify_func_ref(inst, func_ref, errors)?;
                self.verify_value_list(inst, args, errors)?;
            }
            CallIndirect {
                sig_ref, ref args, ..
            } => {
                self.verify_sig_ref(inst, sig_ref, errors)?;
                self.verify_value_list(inst, args, errors)?;
            }
            TryCall {
                func_ref,
                ref args,
                exception,
                ..
            } => {
                self.verify_func_ref(inst, func_ref, errors)?;
                self.verify_value_list(inst, args, errors)?;
                self.verify_exception_table(inst, exception, errors)?;
                self.verify_exception_compatible_abi(inst, exception, errors)?;
            }
            TryCallIndirect {
                ref args,
                exception,
                ..
            } => {
                self.verify_value_list(inst, args, errors)?;
                self.verify_exception_table(inst, exception, errors)?;
                self.verify_exception_compatible_abi(inst, exception, errors)?;
            }
            FuncAddr { func_ref, .. } => {
                self.verify_func_ref(inst, func_ref, errors)?;
            }
            StackLoad { stack_slot, .. } | StackStore { stack_slot, .. } => {
                self.verify_stack_slot(inst, stack_slot, errors)?;
            }
            DynamicStackLoad {
                dynamic_stack_slot, ..
            }
            | DynamicStackStore {
                dynamic_stack_slot, ..
            } => {
                self.verify_dynamic_stack_slot(inst, dynamic_stack_slot, errors)?;
            }
            UnaryGlobalValue { global_value, .. } => {
                self.verify_global_value(inst, global_value, errors)?;
            }
            NullAry {
                opcode: Opcode::GetPinnedReg,
            }
            | Unary {
                opcode: Opcode::SetPinnedReg,
                ..
            } => {
                if let Some(isa) = &self.isa {
                    if !isa.flags().enable_pinned_reg() {
                        return errors.fatal((
                            inst,
                            self.context(inst),
                            "GetPinnedReg/SetPinnedReg cannot be used without enable_pinned_reg",
                        ));
                    }
                } else {
                    return errors.fatal((
                        inst,
                        self.context(inst),
                        "GetPinnedReg/SetPinnedReg need an ISA!",
                    ));
                }
            }
            NullAry {
                opcode: Opcode::GetFramePointer | Opcode::GetReturnAddress,
            } => {
                if let Some(isa) = &self.isa {
                    // Backends may already rely on this check implicitly, so do
                    // not relax it without verifying that it is safe to do so.
                    if !isa.flags().preserve_frame_pointers() {
                        return errors.fatal((
                            inst,
                            self.context(inst),
                            "`get_frame_pointer`/`get_return_address` cannot be used without \
                             enabling `preserve_frame_pointers`",
                        ));
                    }
                } else {
                    return errors.fatal((
                        inst,
                        self.context(inst),
                        "`get_frame_pointer`/`get_return_address` require an ISA!",
                    ));
                }
            }
            LoadNoOffset {
                opcode: Opcode::Bitcast,
                flags,
                arg,
            } => {
                self.verify_bitcast(inst, flags, arg, errors)?;
            }
            LoadNoOffset { opcode, arg, .. } if opcode.can_load() => {
                self.verify_is_address(inst, arg, errors)?;
            }
            Load { opcode, arg, .. } if opcode.can_load() => {
                self.verify_is_address(inst, arg, errors)?;
            }
            AtomicCas {
                opcode,
                args: [p, _, _],
                ..
            } if opcode.can_load() || opcode.can_store() => {
                self.verify_is_address(inst, p, errors)?;
            }
            AtomicRmw {
                opcode,
                args: [p, _],
                ..
            } if opcode.can_load() || opcode.can_store() => {
                self.verify_is_address(inst, p, errors)?;
            }
            Store {
                opcode,
                args: [_, p],
                ..
            } if opcode.can_store() => {
                self.verify_is_address(inst, p, errors)?;
            }
            StoreNoOffset {
                opcode,
                args: [_, p],
                ..
            } if opcode.can_store() => {
                self.verify_is_address(inst, p, errors)?;
            }
            UnaryConst {
                opcode: opcode @ (Opcode::Vconst | Opcode::F128const),
                constant_handle,
                ..
            } => {
                self.verify_constant_size(inst, opcode, constant_handle, errors)?;
            }

            // Exhaustive list so we can't forget to add new formats
            AtomicCas { .. }
            | AtomicRmw { .. }
            | LoadNoOffset { .. }
            | StoreNoOffset { .. }
            | Unary { .. }
            | UnaryConst { .. }
            | UnaryImm { .. }
            | UnaryIeee16 { .. }
            | UnaryIeee32 { .. }
            | UnaryIeee64 { .. }
            | Binary { .. }
            | BinaryImm8 { .. }
            | BinaryImm64 { .. }
            | Ternary { .. }
            | TernaryImm8 { .. }
            | Shuffle { .. }
            | IntAddTrap { .. }
            | IntCompare { .. }
            | IntCompareImm { .. }
            | FloatCompare { .. }
            | Load { .. }
            | Store { .. }
            | Trap { .. }
            | CondTrap { .. }
            | NullAry { .. } => {}
        }

        Ok(())
    }

    fn verify_block(
        &self,
        loc: impl Into<AnyEntity>,
        e: Block,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        if !self.func.dfg.block_is_valid(e) || !self.func.layout.is_block_inserted(e) {
            return errors.fatal((loc, format!("invalid block reference {e}")));
        }
        if let Some(entry_block) = self.func.layout.entry_block() {
            if e == entry_block {
                return errors.fatal((loc, format!("invalid reference to entry block {e}")));
            }
        }
        Ok(())
    }

    fn verify_sig_ref(
        &self,
        inst: Inst,
        s: SigRef,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        if !self.func.dfg.signatures.is_valid(s) {
            errors.fatal((
                inst,
                self.context(inst),
                format!("invalid signature reference {s}"),
            ))
        } else {
            Ok(())
        }
    }

    fn verify_func_ref(
        &self,
        inst: Inst,
        f: FuncRef,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        if !self.func.dfg.ext_funcs.is_valid(f) {
            errors.nonfatal((
                inst,
                self.context(inst),
                format!("invalid function reference {f}"),
            ))
        } else {
            Ok(())
        }
    }

    fn verify_stack_slot(
        &self,
        inst: Inst,
        ss: StackSlot,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        if !self.func.sized_stack_slots.is_valid(ss) {
            errors.nonfatal((inst, self.context(inst), format!("invalid stack slot {ss}")))
        } else {
            Ok(())
        }
    }

    fn verify_dynamic_stack_slot(
        &self,
        inst: Inst,
        ss: DynamicStackSlot,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        if !self.func.dynamic_stack_slots.is_valid(ss) {
            errors.nonfatal((
                inst,
                self.context(inst),
                format!("invalid dynamic stack slot {ss}"),
            ))
        } else {
            Ok(())
        }
    }

    fn verify_global_value(
        &self,
        inst: Inst,
        gv: GlobalValue,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        if !self.func.global_values.is_valid(gv) {
            errors.nonfatal((
                inst,
                self.context(inst),
                format!("invalid global value {gv}"),
            ))
        } else {
            Ok(())
        }
    }

    fn verify_value_list(
        &self,
        inst: Inst,
        l: &ValueList,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        if !l.is_valid(&self.func.dfg.value_lists) {
            errors.nonfatal((
                inst,
                self.context(inst),
                format!("invalid value list reference {l:?}"),
            ))
        } else {
            Ok(())
        }
    }

    fn verify_jump_table(
        &self,
        inst: Inst,
        j: JumpTable,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        if !self.func.stencil.dfg.jump_tables.is_valid(j) {
            errors.nonfatal((
                inst,
                self.context(inst),
                format!("invalid jump table reference {j}"),
            ))
        } else {
            let pool = &self.func.stencil.dfg.value_lists;
            for block in self.func.stencil.dfg.jump_tables[j].all_branches() {
                self.verify_block(inst, block.block(pool), errors)?;
            }
            Ok(())
        }
    }

    fn verify_exception_table(
        &self,
        inst: Inst,
        et: ExceptionTable,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        // Verify that the exception table reference itself is valid.
        if !self.func.stencil.dfg.exception_tables.is_valid(et) {
            errors.nonfatal((
                inst,
                self.context(inst),
                format!("invalid exception table reference {et}"),
            ))?;
        }

        let pool = &self.func.stencil.dfg.value_lists;
        let exdata = &self.func.stencil.dfg.exception_tables[et];

        // Verify that the exception table's signature reference
        // is valid.
        self.verify_sig_ref(inst, exdata.signature(), errors)?;

        // Verify that the exception table's block references are valid.
        for block in exdata.all_branches() {
            self.verify_block(inst, block.block(pool), errors)?;
        }
        Ok(())
    }

    fn verify_exception_compatible_abi(
        &self,
        inst: Inst,
        et: ExceptionTable,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        let callee_sig_ref = self.func.dfg.exception_tables[et].signature();
        let callee_sig = &self.func.dfg.signatures[callee_sig_ref];
        let callee_call_conv = callee_sig.call_conv;
        if !callee_call_conv.supports_exceptions() {
            errors.nonfatal((
                inst,
                self.context(inst),
                format!(
                    "calling convention `{callee_call_conv}` of callee does not support exceptions"
                ),
            ))?;
        }
        Ok(())
    }

    fn verify_value(
        &self,
        loc_inst: Inst,
        v: Value,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        let dfg = &self.func.dfg;
        if !dfg.value_is_valid(v) {
            errors.nonfatal((
                loc_inst,
                self.context(loc_inst),
                format!("invalid value reference {v}"),
            ))
        } else {
            Ok(())
        }
    }

    fn verify_inst_arg(
        &self,
        loc_inst: Inst,
        v: Value,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        self.verify_value(loc_inst, v, errors)?;

        let dfg = &self.func.dfg;
        let loc_block = self
            .func
            .layout
            .inst_block(loc_inst)
            .expect("Instruction not in layout.");
        let is_reachable = self.expected_domtree.is_reachable(loc_block);

        // SSA form
        match dfg.value_def(v) {
            ValueDef::Result(def_inst, _) => {
                // Value is defined by an instruction that exists.
                if !dfg.inst_is_valid(def_inst) {
                    return errors.fatal((
                        loc_inst,
                        self.context(loc_inst),
                        format!("{v} is defined by invalid instruction {def_inst}"),
                    ));
                }
                // Defining instruction is inserted in a block.
                if self.func.layout.inst_block(def_inst) == None {
                    return errors.fatal((
                        loc_inst,
                        self.context(loc_inst),
                        format!("{v} is defined by {def_inst} which has no block"),
                    ));
                }
                // Defining instruction dominates the instruction that uses the value.
                if is_reachable {
                    if !self.expected_domtree_preorder.dominates_inst(
                        def_inst,
                        loc_inst,
                        &self.func.layout,
                    ) {
                        return errors.fatal((
                            loc_inst,
                            self.context(loc_inst),
                            format!("uses value {v} from non-dominating {def_inst}"),
                        ));
                    }
                    if def_inst == loc_inst {
                        return errors.fatal((
                            loc_inst,
                            self.context(loc_inst),
                            format!("uses value {v} from itself"),
                        ));
                    }
                }
            }
            ValueDef::Param(block, _) => {
                // Value is defined by an existing block.
                if !dfg.block_is_valid(block) {
                    return errors.fatal((
                        loc_inst,
                        self.context(loc_inst),
                        format!("{v} is defined by invalid block {block}"),
                    ));
                }
                // Defining block is inserted in the layout
                if !self.func.layout.is_block_inserted(block) {
                    return errors.fatal((
                        loc_inst,
                        self.context(loc_inst),
                        format!("{v} is defined by {block} which is not in the layout"),
                    ));
                }
                let user_block = self.func.layout.inst_block(loc_inst).expect("Expected instruction to be in a block as we're traversing code already in layout");
                // The defining block dominates the instruction using this value.
                if is_reachable && !self.expected_domtree_preorder.dominates(block, user_block) {
                    return errors.fatal((
                        loc_inst,
                        self.context(loc_inst),
                        format!("uses value arg from non-dominating {block}"),
                    ));
                }
            }
            ValueDef::Union(_, _) => {
                // Nothing: union nodes themselves have no location,
                // so we cannot check any dominance properties.
            }
        }
        Ok(())
    }

    fn verify_inst_result(
        &self,
        loc_inst: Inst,
        v: Value,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        self.verify_value(loc_inst, v, errors)?;

        match self.func.dfg.value_def(v) {
            ValueDef::Result(def_inst, _) => {
                if def_inst != loc_inst {
                    errors.fatal((
                        loc_inst,
                        self.context(loc_inst),
                        format!("instruction result {v} is not defined by the instruction"),
                    ))
                } else {
                    Ok(())
                }
            }
            ValueDef::Param(_, _) => errors.fatal((
                loc_inst,
                self.context(loc_inst),
                format!("instruction result {v} is not defined by the instruction"),
            )),
            ValueDef::Union(_, _) => errors.fatal((
                loc_inst,
                self.context(loc_inst),
                format!("instruction result {v} is a union node"),
            )),
        }
    }

    fn verify_bitcast(
        &self,
        inst: Inst,
        flags: MemFlags,
        arg: Value,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        let typ = self.func.dfg.ctrl_typevar(inst);
        let value_type = self.func.dfg.value_type(arg);

        if typ.bits() != value_type.bits() {
            errors.fatal((
                inst,
                format!(
                    "The bitcast argument {} has a type of {} bits, which doesn't match an expected type of {} bits",
                    arg,
                    value_type.bits(),
                    typ.bits()
                ),
            ))
        } else if flags != MemFlags::new()
            && flags != MemFlags::new().with_endianness(ir::Endianness::Little)
            && flags != MemFlags::new().with_endianness(ir::Endianness::Big)
        {
            errors.fatal((
                inst,
                "The bitcast instruction only accepts the `big` or `little` memory flags",
            ))
        } else if flags == MemFlags::new() && typ.lane_count() != value_type.lane_count() {
            errors.fatal((
                inst,
                "Byte order specifier required for bitcast instruction changing lane count",
            ))
        } else {
            Ok(())
        }
    }

    fn verify_constant_size(
        &self,
        inst: Inst,
        opcode: Opcode,
        constant: Constant,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        let type_size = match opcode {
            Opcode::F128const => types::F128.bytes(),
            Opcode::Vconst => self.func.dfg.ctrl_typevar(inst).bytes(),
            _ => unreachable!("unexpected opcode {opcode:?}"),
        } as usize;
        let constant_size = self.func.dfg.constants.get(constant).len();
        if type_size != constant_size {
            errors.fatal((
                inst,
                format!(
                    "The instruction expects {constant} to have a size of {type_size} bytes but it has {constant_size}"
                ),
            ))
        } else {
            Ok(())
        }
    }

    fn verify_is_address(
        &self,
        loc_inst: Inst,
        v: Value,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        if let Some(isa) = self.isa {
            let pointer_width = isa.triple().pointer_width()?;
            let value_type = self.func.dfg.value_type(v);
            let expected_width = pointer_width.bits() as u32;
            let value_width = value_type.bits();
            if expected_width != value_width {
                errors.nonfatal((
                    loc_inst,
                    self.context(loc_inst),
                    format!("invalid pointer width (got {value_width}, expected {expected_width}) encountered {v}"),
                ))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn domtree_integrity(
        &self,
        domtree: &DominatorTree,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        // We consider two `DominatorTree`s to be equal if they return the same immediate
        // dominator for each block. Therefore the current domtree is valid if it matches the freshly
        // computed one.
        for block in self.func.layout.blocks() {
            let expected = self.expected_domtree.idom(block);
            let got = domtree.idom(block);
            if got != expected {
                return errors.fatal((
                    block,
                    format!("invalid domtree, expected idom({block}) = {expected:?}, got {got:?}"),
                ));
            }
        }
        // We also verify if the postorder defined by `DominatorTree` is sane
        if domtree.cfg_postorder().len() != self.expected_domtree.cfg_postorder().len() {
            return errors.fatal((
                AnyEntity::Function,
                "incorrect number of Blocks in postorder traversal",
            ));
        }
        for (index, (&test_block, &true_block)) in domtree
            .cfg_postorder()
            .iter()
            .zip(self.expected_domtree.cfg_postorder().iter())
            .enumerate()
        {
            if test_block != true_block {
                return errors.fatal((
                    test_block,
                    format!(
                        "invalid domtree, postorder block number {index} should be {true_block}, got {test_block}"
                    ),
                ));
            }
        }
        Ok(())
    }

    fn typecheck_entry_block_params(&self, errors: &mut VerifierErrors) -> VerifierStepResult {
        if let Some(block) = self.func.layout.entry_block() {
            let expected_types = &self.func.signature.params;
            let block_param_count = self.func.dfg.num_block_params(block);

            if block_param_count != expected_types.len() {
                return errors.fatal((
                    block,
                    format!(
                        "entry block parameters ({}) must match function signature ({})",
                        block_param_count,
                        expected_types.len()
                    ),
                ));
            }

            for (i, &arg) in self.func.dfg.block_params(block).iter().enumerate() {
                let arg_type = self.func.dfg.value_type(arg);
                if arg_type != expected_types[i].value_type {
                    errors.report((
                        block,
                        format!(
                            "entry block parameter {} expected to have type {}, got {}",
                            i, expected_types[i], arg_type
                        ),
                    ));
                }
            }
        }

        errors.as_result()
    }

    fn check_entry_not_cold(&self, errors: &mut VerifierErrors) -> VerifierStepResult {
        if let Some(entry_block) = self.func.layout.entry_block() {
            if self.func.layout.is_cold(entry_block) {
                return errors
                    .fatal((entry_block, format!("entry block cannot be marked as cold")));
            }
        }
        errors.as_result()
    }

    fn typecheck(&self, inst: Inst, errors: &mut VerifierErrors) -> VerifierStepResult {
        let inst_data = &self.func.dfg.insts[inst];
        let constraints = inst_data.opcode().constraints();

        let ctrl_type = if let Some(value_typeset) = constraints.ctrl_typeset() {
            // For polymorphic opcodes, determine the controlling type variable first.
            let ctrl_type = self.func.dfg.ctrl_typevar(inst);

            if !value_typeset.contains(ctrl_type) {
                errors.report((
                    inst,
                    self.context(inst),
                    format!(
                        "has an invalid controlling type {ctrl_type} (allowed set is {value_typeset:?})"
                    ),
                ));
            }

            ctrl_type
        } else {
            // Non-polymorphic instructions don't check the controlling type variable, so `Option`
            // is unnecessary and we can just make it `INVALID`.
            types::INVALID
        };

        // Typechecking instructions is never fatal
        let _ = self.typecheck_results(inst, ctrl_type, errors);
        let _ = self.typecheck_fixed_args(inst, ctrl_type, errors);
        let _ = self.typecheck_variable_args(inst, errors);
        let _ = self.typecheck_return(inst, errors);
        let _ = self.typecheck_special(inst, errors);

        Ok(())
    }

    fn typecheck_results(
        &self,
        inst: Inst,
        ctrl_type: Type,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        let mut i = 0;
        for &result in self.func.dfg.inst_results(inst) {
            let result_type = self.func.dfg.value_type(result);
            let expected_type = self.func.dfg.compute_result_type(inst, i, ctrl_type);
            if let Some(expected_type) = expected_type {
                if result_type != expected_type {
                    errors.report((
                        inst,
                        self.context(inst),
                        format!(
                            "expected result {i} ({result}) to have type {expected_type}, found {result_type}"
                        ),
                    ));
                }
            } else {
                return errors.nonfatal((
                    inst,
                    self.context(inst),
                    "has more result values than expected",
                ));
            }
            i += 1;
        }

        // There aren't any more result types left.
        if self.func.dfg.compute_result_type(inst, i, ctrl_type) != None {
            return errors.nonfatal((
                inst,
                self.context(inst),
                "has fewer result values than expected",
            ));
        }
        Ok(())
    }

    fn typecheck_fixed_args(
        &self,
        inst: Inst,
        ctrl_type: Type,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        let constraints = self.func.dfg.insts[inst].opcode().constraints();

        for (i, &arg) in self.func.dfg.inst_fixed_args(inst).iter().enumerate() {
            let arg_type = self.func.dfg.value_type(arg);
            match constraints.value_argument_constraint(i, ctrl_type) {
                ResolvedConstraint::Bound(expected_type) => {
                    if arg_type != expected_type {
                        errors.report((
                            inst,
                            self.context(inst),
                            format!(
                                "arg {i} ({arg}) has type {arg_type}, expected {expected_type}"
                            ),
                        ));
                    }
                }
                ResolvedConstraint::Free(type_set) => {
                    if !type_set.contains(arg_type) {
                        errors.report((
                            inst,
                            self.context(inst),
                            format!(
                                "arg {i} ({arg}) with type {arg_type} failed to satisfy type set {type_set:?}"
                            ),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Typecheck both instructions that contain variable arguments like calls, and those that
    /// include references to basic blocks with their arguments.
    fn typecheck_variable_args(
        &self,
        inst: Inst,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        match &self.func.dfg.insts[inst] {
            ir::InstructionData::Jump { destination, .. } => {
                self.typecheck_block_call(inst, destination, BlockCallTargetType::Normal, errors)?;
            }
            ir::InstructionData::Brif {
                blocks: [block_then, block_else],
                ..
            } => {
                self.typecheck_block_call(inst, block_then, BlockCallTargetType::Normal, errors)?;
                self.typecheck_block_call(inst, block_else, BlockCallTargetType::Normal, errors)?;
            }
            ir::InstructionData::BranchTable { table, .. } => {
                for block in self.func.stencil.dfg.jump_tables[*table].all_branches() {
                    self.typecheck_block_call(inst, block, BlockCallTargetType::Normal, errors)?;
                }
            }
            ir::InstructionData::TryCall { exception, .. }
            | ir::InstructionData::TryCallIndirect { exception, .. } => {
                let exdata = &self.func.dfg.exception_tables[*exception];
                self.typecheck_block_call(
                    inst,
                    exdata.normal_return(),
                    BlockCallTargetType::ExNormalRet,
                    errors,
                )?;
                for item in exdata.items() {
                    match item {
                        ExceptionTableItem::Tag(_, block_call)
                        | ExceptionTableItem::Default(block_call) => {
                            self.typecheck_block_call(
                                inst,
                                &block_call,
                                BlockCallTargetType::Exception,
                                errors,
                            )?;
                        }
                        ExceptionTableItem::Context(_) => {}
                    }
                }
            }
            inst => debug_assert!(!inst.opcode().is_branch()),
        }

        match self.func.dfg.insts[inst]
            .analyze_call(&self.func.dfg.value_lists, &self.func.dfg.exception_tables)
        {
            CallInfo::Direct(func_ref, args) => {
                let sig_ref = self.func.dfg.ext_funcs[func_ref].signature;
                let arg_types = self.func.dfg.signatures[sig_ref]
                    .params
                    .iter()
                    .map(|a| a.value_type);
                self.typecheck_variable_args_iterator(inst, arg_types, args, errors)?;
            }
            CallInfo::DirectWithSig(func_ref, sig_ref, args) => {
                let expected_sig_ref = self.func.dfg.ext_funcs[func_ref].signature;
                let sigdata = &self.func.dfg.signatures;
                // Compare signatures by value, not by ID -- any
                // equivalent signature ID is acceptable.
                if sigdata[sig_ref] != sigdata[expected_sig_ref] {
                    errors.nonfatal((
                        inst,
                        self.context(inst),
                        format!(
                            "exception table signature {sig_ref} did not match function {func_ref}'s signature {expected_sig_ref}"
                        ),
                    ))?;
                }
                let arg_types = self.func.dfg.signatures[sig_ref]
                    .params
                    .iter()
                    .map(|a| a.value_type);
                self.typecheck_variable_args_iterator(inst, arg_types, args, errors)?;
            }
            CallInfo::Indirect(sig_ref, args) => {
                let arg_types = self.func.dfg.signatures[sig_ref]
                    .params
                    .iter()
                    .map(|a| a.value_type);
                self.typecheck_variable_args_iterator(inst, arg_types, args, errors)?;
            }
            CallInfo::NotACall => {}
        }
        Ok(())
    }

    fn typecheck_block_call(
        &self,
        inst: Inst,
        block: &ir::BlockCall,
        target_type: BlockCallTargetType,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        let pool = &self.func.dfg.value_lists;
        let block_params = self.func.dfg.block_params(block.block(pool));
        let args = block.args(pool);
        if args.len() != block_params.len() {
            return errors.nonfatal((
                inst,
                self.context(inst),
                format!(
                    "mismatched argument count for `{}`: got {}, expected {}",
                    self.func.dfg.display_inst(inst),
                    args.len(),
                    block_params.len(),
                ),
            ));
        }
        for (arg, param) in args.zip(block_params.iter()) {
            let Some(arg_ty) = self.block_call_arg_ty(arg, inst, target_type, errors)? else {
                continue;
            };
            let param_ty = self.func.dfg.value_type(*param);
            if arg_ty != param_ty {
                errors.nonfatal((
                    inst,
                    self.context(inst),
                    format!("arg {arg} has type {arg_ty}, expected {param_ty}"),
                ))?;
            }
        }
        Ok(())
    }

    fn block_call_arg_ty(
        &self,
        arg: BlockArg,
        inst: Inst,
        target_type: BlockCallTargetType,
        errors: &mut VerifierErrors,
    ) -> Result<Option<Type>, ()> {
        match arg {
            BlockArg::Value(v) => Ok(Some(self.func.dfg.value_type(v))),
            BlockArg::TryCallRet(_) | BlockArg::TryCallExn(_) => {
                // Get the invoked signature.
                let et = match self.func.dfg.insts[inst].exception_table() {
                    Some(et) => et,
                    None => {
                        errors.fatal((
                            inst,
                            self.context(inst),
                            format!(
                                "`retN` block argument in block-call not on `try_call` instruction"
                            ),
                        ))?;
                        unreachable!()
                    }
                };
                let exdata = &self.func.dfg.exception_tables[et];
                let sig = &self.func.dfg.signatures[exdata.signature()];

                match (arg, target_type) {
                    (BlockArg::TryCallRet(i), BlockCallTargetType::ExNormalRet)
                        if (i as usize) < sig.returns.len() =>
                    {
                        Ok(Some(sig.returns[i as usize].value_type))
                    }
                    (BlockArg::TryCallRet(_), BlockCallTargetType::ExNormalRet) => {
                        errors.fatal((
                            inst,
                            self.context(inst),
                            format!("out-of-bounds `retN` block argument"),
                        ))?;
                        unreachable!()
                    }
                    (BlockArg::TryCallRet(_), _) => {
                        errors.fatal((
                            inst,
                            self.context(inst),
                            format!("`retN` block argument used outside normal-return target of `try_call`"),
                        ))?;
                        unreachable!()
                    }
                    (BlockArg::TryCallExn(i), BlockCallTargetType::Exception) => {
                        if let Some(isa) = self.isa {
                            match sig
                                .call_conv
                                .exception_payload_types(isa.pointer_type())
                                .get(i as usize)
                            {
                                Some(ty) => Ok(Some(*ty)),
                                None => {
                                    errors.fatal((
                                        inst,
                                        self.context(inst),
                                        format!("out-of-bounds `exnN` block argument"),
                                    ))?;
                                    unreachable!()
                                }
                            }
                        } else {
                            Ok(None)
                        }
                    }
                    (BlockArg::TryCallExn(_), _) => {
                        errors.fatal((
                            inst,
                            self.context(inst),
                            format!("`exnN` block argument used outside normal-return target of `try_call`"),
                        ))?;
                        unreachable!()
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn typecheck_variable_args_iterator(
        &self,
        inst: Inst,
        iter: impl ExactSizeIterator<Item = Type>,
        variable_args: &[Value],
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        let mut i = 0;

        for expected_type in iter {
            if i >= variable_args.len() {
                // Result count mismatch handled below, we want the full argument count first though
                i += 1;
                continue;
            }
            let arg = variable_args[i];
            let arg_type = self.func.dfg.value_type(arg);
            if expected_type != arg_type {
                errors.report((
                    inst,
                    self.context(inst),
                    format!(
                        "arg {} ({}) has type {}, expected {}",
                        i, variable_args[i], arg_type, expected_type
                    ),
                ));
            }
            i += 1;
        }
        if i != variable_args.len() {
            return errors.nonfatal((
                inst,
                self.context(inst),
                format!(
                    "mismatched argument count for `{}`: got {}, expected {}",
                    self.func.dfg.display_inst(inst),
                    variable_args.len(),
                    i,
                ),
            ));
        }
        Ok(())
    }

    fn typecheck_return(&self, inst: Inst, errors: &mut VerifierErrors) -> VerifierStepResult {
        match self.func.dfg.insts[inst] {
            ir::InstructionData::MultiAry {
                opcode: Opcode::Return,
                args,
            } => {
                let types = args
                    .as_slice(&self.func.dfg.value_lists)
                    .iter()
                    .map(|v| self.func.dfg.value_type(*v));
                self.typecheck_return_types(
                    inst,
                    types,
                    errors,
                    "arguments of return must match function signature",
                )?;
            }
            ir::InstructionData::Call {
                opcode: Opcode::ReturnCall,
                func_ref,
                ..
            } => {
                let sig_ref = self.func.dfg.ext_funcs[func_ref].signature;
                self.typecheck_tail_call(inst, sig_ref, errors)?;
            }
            ir::InstructionData::CallIndirect {
                opcode: Opcode::ReturnCallIndirect,
                sig_ref,
                ..
            } => {
                self.typecheck_tail_call(inst, sig_ref, errors)?;
            }
            inst => debug_assert!(!inst.opcode().is_return()),
        }
        Ok(())
    }

    fn typecheck_tail_call(
        &self,
        inst: Inst,
        sig_ref: SigRef,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        let signature = &self.func.dfg.signatures[sig_ref];
        let cc = signature.call_conv;
        if !cc.supports_tail_calls() {
            errors.report((
                inst,
                self.context(inst),
                format!("calling convention `{cc}` does not support tail calls"),
            ));
        }
        if cc != self.func.signature.call_conv {
            errors.report((
                inst,
                self.context(inst),
                "callee's calling convention must match caller",
            ));
        }
        let types = signature.returns.iter().map(|param| param.value_type);
        self.typecheck_return_types(inst, types, errors, "results of callee must match caller")?;
        Ok(())
    }

    fn typecheck_return_types(
        &self,
        inst: Inst,
        actual_types: impl ExactSizeIterator<Item = Type>,
        errors: &mut VerifierErrors,
        message: &str,
    ) -> VerifierStepResult {
        let expected_types = &self.func.signature.returns;
        if actual_types.len() != expected_types.len() {
            return errors.nonfatal((inst, self.context(inst), message));
        }
        for (i, (actual_type, &expected_type)) in actual_types.zip(expected_types).enumerate() {
            if actual_type != expected_type.value_type {
                errors.report((
                    inst,
                    self.context(inst),
                    format!(
                        "result {i} has type {actual_type}, must match function signature of \
                         {expected_type}"
                    ),
                ));
            }
        }
        Ok(())
    }

    // Check special-purpose type constraints that can't be expressed in the normal opcode
    // constraints.
    fn typecheck_special(&self, inst: Inst, errors: &mut VerifierErrors) -> VerifierStepResult {
        match self.func.dfg.insts[inst] {
            ir::InstructionData::UnaryGlobalValue { global_value, .. } => {
                if let Some(isa) = self.isa {
                    let inst_type = self.func.dfg.value_type(self.func.dfg.first_result(inst));
                    let global_type = self.func.global_values[global_value].global_type(isa);
                    if inst_type != global_type {
                        return errors.nonfatal((
                            inst, self.context(inst),
                            format!(
                                "global_value instruction with type {inst_type} references global value with type {global_type}"
                            )),
                        );
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn cfg_integrity(
        &self,
        cfg: &ControlFlowGraph,
        errors: &mut VerifierErrors,
    ) -> VerifierStepResult {
        let mut expected_succs = BTreeSet::<Block>::new();
        let mut got_succs = BTreeSet::<Block>::new();
        let mut expected_preds = BTreeSet::<Inst>::new();
        let mut got_preds = BTreeSet::<Inst>::new();

        for block in self.func.layout.blocks() {
            expected_succs.extend(self.expected_cfg.succ_iter(block));
            got_succs.extend(cfg.succ_iter(block));

            let missing_succs: Vec<Block> =
                expected_succs.difference(&got_succs).cloned().collect();
            if !missing_succs.is_empty() {
                errors.report((
                    block,
                    format!("cfg lacked the following successor(s) {missing_succs:?}"),
                ));
                continue;
            }

            let excess_succs: Vec<Block> = got_succs.difference(&expected_succs).cloned().collect();
            if !excess_succs.is_empty() {
                errors.report((
                    block,
                    format!("cfg had unexpected successor(s) {excess_succs:?}"),
                ));
                continue;
            }

            expected_preds.extend(
                self.expected_cfg
                    .pred_iter(block)
                    .map(|BlockPredecessor { inst, .. }| inst),
            );
            got_preds.extend(
                cfg.pred_iter(block)
                    .map(|BlockPredecessor { inst, .. }| inst),
            );

            let missing_preds: Vec<Inst> = expected_preds.difference(&got_preds).cloned().collect();
            if !missing_preds.is_empty() {
                errors.report((
                    block,
                    format!("cfg lacked the following predecessor(s) {missing_preds:?}"),
                ));
                continue;
            }

            let excess_preds: Vec<Inst> = got_preds.difference(&expected_preds).cloned().collect();
            if !excess_preds.is_empty() {
                errors.report((
                    block,
                    format!("cfg had unexpected predecessor(s) {excess_preds:?}"),
                ));
                continue;
            }

            expected_succs.clear();
            got_succs.clear();
            expected_preds.clear();
            got_preds.clear();
        }
        errors.as_result()
    }

    fn immediate_constraints(&self, inst: Inst, errors: &mut VerifierErrors) -> VerifierStepResult {
        let inst_data = &self.func.dfg.insts[inst];

        match *inst_data {
            ir::InstructionData::Store { flags, .. } => {
                if flags.readonly() {
                    errors.fatal((
                        inst,
                        self.context(inst),
                        "A store instruction cannot have the `readonly` MemFlag",
                    ))
                } else {
                    Ok(())
                }
            }
            ir::InstructionData::BinaryImm8 {
                opcode: ir::instructions::Opcode::Extractlane,
                imm: lane,
                arg,
                ..
            }
            | ir::InstructionData::TernaryImm8 {
                opcode: ir::instructions::Opcode::Insertlane,
                imm: lane,
                args: [arg, _],
                ..
            } => {
                // We must be specific about the opcodes above because other instructions are using
                // the same formats.
                let ty = self.func.dfg.value_type(arg);
                if lane as u32 >= ty.lane_count() {
                    errors.fatal((
                        inst,
                        self.context(inst),
                        format!("The lane {lane} does not index into the type {ty}",),
                    ))
                } else {
                    Ok(())
                }
            }
            ir::InstructionData::Shuffle {
                opcode: ir::instructions::Opcode::Shuffle,
                imm,
                ..
            } => {
                let imm = self.func.dfg.immediates.get(imm).unwrap().as_slice();
                if imm.len() != 16 {
                    errors.fatal((
                        inst,
                        self.context(inst),
                        format!("the shuffle immediate wasn't 16-bytes long"),
                    ))
                } else if let Some(i) = imm.iter().find(|i| **i >= 32) {
                    errors.fatal((
                        inst,
                        self.context(inst),
                        format!("shuffle immediate index {i} is larger than the maximum 31"),
                    ))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }

    fn iconst_bounds(&self, inst: Inst, errors: &mut VerifierErrors) -> VerifierStepResult {
        use crate::ir::instructions::InstructionData::UnaryImm;

        let inst_data = &self.func.dfg.insts[inst];
        if let UnaryImm {
            opcode: Opcode::Iconst,
            imm,
        } = inst_data
        {
            let ctrl_typevar = self.func.dfg.ctrl_typevar(inst);
            let bounds_mask = match ctrl_typevar {
                types::I8 => u8::MAX.into(),
                types::I16 => u16::MAX.into(),
                types::I32 => u32::MAX.into(),
                types::I64 => u64::MAX,
                _ => unreachable!(),
            };

            let value = imm.bits() as u64;
            if value & bounds_mask != value {
                errors.fatal((
                    inst,
                    self.context(inst),
                    "constant immediate is out of bounds",
                ))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn typecheck_function_signature(&self, errors: &mut VerifierErrors) -> VerifierStepResult {
        let params = self
            .func
            .signature
            .params
            .iter()
            .enumerate()
            .map(|p| (true, p));
        let returns = self
            .func
            .signature
            .returns
            .iter()
            .enumerate()
            .map(|p| (false, p));

        for (is_argument, (i, param)) in params.chain(returns) {
            let is_return = !is_argument;
            let item = if is_argument {
                "Parameter"
            } else {
                "Return value"
            };

            if param.value_type == types::INVALID {
                errors.report((
                    AnyEntity::Function,
                    format!("{item} at position {i} has an invalid type"),
                ));
            }

            if let ArgumentPurpose::StructArgument(_) = param.purpose {
                if is_return {
                    errors.report((
                        AnyEntity::Function,
                        format!("{item} at position {i} can't be an struct argument"),
                    ))
                }
            }

            let ty_allows_extension = param.value_type.is_int();
            let has_extension = param.extension != ArgumentExtension::None;
            if !ty_allows_extension && has_extension {
                errors.report((
                    AnyEntity::Function,
                    format!(
                        "{} at position {} has invalid extension {:?}",
                        item, i, param.extension
                    ),
                ));
            }
        }

        if errors.has_error() { Err(()) } else { Ok(()) }
    }

    pub fn run(&self, errors: &mut VerifierErrors) -> VerifierStepResult {
        self.verify_global_values(errors)?;
        self.verify_memory_types(errors)?;
        self.typecheck_entry_block_params(errors)?;
        self.check_entry_not_cold(errors)?;
        self.typecheck_function_signature(errors)?;

        for block in self.func.layout.blocks() {
            if self.func.layout.first_inst(block).is_none() {
                return errors.fatal((block, format!("{block} cannot be empty")));
            }
            for inst in self.func.layout.block_insts(block) {
                crate::trace!("verifying {inst:?}: {}", self.func.dfg.display_inst(inst));
                self.block_integrity(block, inst, errors)?;
                self.instruction_integrity(inst, errors)?;
                self.typecheck(inst, errors)?;
                self.immediate_constraints(inst, errors)?;
                self.iconst_bounds(inst, errors)?;
            }

            self.encodable_as_bb(block, errors)?;
        }

        if !errors.is_empty() {
            log::warn!(
                "Found verifier errors in function:\n{}",
                pretty_verifier_error(self.func, None, errors.clone())
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Verifier, VerifierError, VerifierErrors};
    use crate::ir::instructions::{InstructionData, Opcode};
    use crate::ir::{AbiParam, Function, Type, types};
    use crate::settings;

    macro_rules! assert_err_with_msg {
        ($e:expr, $msg:expr) => {
            match $e.0.get(0) {
                None => panic!("Expected an error"),
                Some(&VerifierError { ref message, .. }) => {
                    if !message.contains($msg) {
                        #[cfg(feature = "std")]
                        panic!("'{}' did not contain the substring '{}'", message, $msg);
                        #[cfg(not(feature = "std"))]
                        panic!("error message did not contain the expected substring");
                    }
                }
            }
        };
    }

    #[test]
    fn empty() {
        let func = Function::new();
        let flags = &settings::Flags::new(settings::builder());
        let verifier = Verifier::new(&func, flags.into());
        let mut errors = VerifierErrors::default();

        assert_eq!(verifier.run(&mut errors), Ok(()));
        assert!(errors.0.is_empty());
    }

    #[test]
    fn bad_instruction_format() {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        func.layout.append_block(block0);
        let nullary_with_bad_opcode = func.dfg.make_inst(InstructionData::UnaryImm {
            opcode: Opcode::F32const,
            imm: 0.into(),
        });
        func.layout.append_inst(nullary_with_bad_opcode, block0);
        let destination = func.dfg.block_call(block0, &[]);
        func.stencil.layout.append_inst(
            func.stencil.dfg.make_inst(InstructionData::Jump {
                opcode: Opcode::Jump,
                destination,
            }),
            block0,
        );
        let flags = &settings::Flags::new(settings::builder());
        let verifier = Verifier::new(&func, flags.into());
        let mut errors = VerifierErrors::default();

        let _ = verifier.run(&mut errors);

        assert_err_with_msg!(errors, "instruction format");
    }

    fn test_iconst_bounds(immediate: i64, ctrl_typevar: Type) -> VerifierErrors {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        func.layout.append_block(block0);

        let test_inst = func.dfg.make_inst(InstructionData::UnaryImm {
            opcode: Opcode::Iconst,
            imm: immediate.into(),
        });

        let end_inst = func.dfg.make_inst(InstructionData::MultiAry {
            opcode: Opcode::Return,
            args: Default::default(),
        });

        func.dfg.make_inst_results(test_inst, ctrl_typevar);
        func.layout.append_inst(test_inst, block0);
        func.layout.append_inst(end_inst, block0);

        let flags = &settings::Flags::new(settings::builder());
        let verifier = Verifier::new(&func, flags.into());
        let mut errors = VerifierErrors::default();

        let _ = verifier.run(&mut errors);
        errors
    }

    fn test_iconst_bounds_err(immediate: i64, ctrl_typevar: Type) {
        assert_err_with_msg!(
            test_iconst_bounds(immediate, ctrl_typevar),
            "constant immediate is out of bounds"
        );
    }

    fn test_iconst_bounds_ok(immediate: i64, ctrl_typevar: Type) {
        assert!(test_iconst_bounds(immediate, ctrl_typevar).is_empty());
    }

    #[test]
    fn negative_iconst_8() {
        test_iconst_bounds_err(-10, types::I8);
    }

    #[test]
    fn negative_iconst_32() {
        test_iconst_bounds_err(-1, types::I32);
    }

    #[test]
    fn large_iconst_8() {
        test_iconst_bounds_err(1 + u8::MAX as i64, types::I8);
    }

    #[test]
    fn large_iconst_16() {
        test_iconst_bounds_err(10 + u16::MAX as i64, types::I16);
    }

    #[test]
    fn valid_iconst_8() {
        test_iconst_bounds_ok(10, types::I8);
    }

    #[test]
    fn valid_iconst_32() {
        test_iconst_bounds_ok(u32::MAX as i64, types::I32);
    }

    #[test]
    fn test_function_invalid_param() {
        let mut func = Function::new();
        func.signature.params.push(AbiParam::new(types::INVALID));

        let mut errors = VerifierErrors::default();
        let flags = &settings::Flags::new(settings::builder());
        let verifier = Verifier::new(&func, flags.into());

        let _ = verifier.typecheck_function_signature(&mut errors);
        assert_err_with_msg!(errors, "Parameter at position 0 has an invalid type");
    }

    #[test]
    fn test_function_invalid_return_value() {
        let mut func = Function::new();
        func.signature.returns.push(AbiParam::new(types::INVALID));

        let mut errors = VerifierErrors::default();
        let flags = &settings::Flags::new(settings::builder());
        let verifier = Verifier::new(&func, flags.into());

        let _ = verifier.typecheck_function_signature(&mut errors);
        assert_err_with_msg!(errors, "Return value at position 0 has an invalid type");
    }

    #[test]
    fn test_printing_contextual_errors() {
        // Build function.
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        func.layout.append_block(block0);

        // Build instruction "f64const 0.0" (missing one required result)
        let inst = func.dfg.make_inst(InstructionData::UnaryIeee64 {
            opcode: Opcode::F64const,
            imm: 0.0.into(),
        });
        func.layout.append_inst(inst, block0);

        // Setup verifier.
        let mut errors = VerifierErrors::default();
        let flags = &settings::Flags::new(settings::builder());
        let verifier = Verifier::new(&func, flags.into());

        // Now the error message, when printed, should contain the instruction sequence causing the
        // error (i.e. f64const 0.0) and not only its entity value (i.e. inst0)
        let _ = verifier.typecheck_results(inst, types::I32, &mut errors);
        assert_eq!(
            format!("{}", errors.0[0]),
            "inst0 (f64const 0.0): has fewer result values than expected"
        )
    }

    #[test]
    fn test_empty_block() {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        func.layout.append_block(block0);

        let flags = &settings::Flags::new(settings::builder());
        let verifier = Verifier::new(&func, flags.into());
        let mut errors = VerifierErrors::default();
        let _ = verifier.run(&mut errors);

        assert_err_with_msg!(errors, "block0 cannot be empty");
    }
}
