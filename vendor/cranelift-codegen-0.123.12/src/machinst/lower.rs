//! This module implements lowering (instruction selection) from Cranelift IR
//! to machine instructions with virtual registers. This is *almost* the final
//! machine code, except for register allocation.

// TODO: separate the IR-query core of `Lower` from the lowering logic built on
// top of it, e.g. the side-effect/coloring analysis and the scan support.

use crate::entity::SecondaryMap;
use crate::inst_predicates::{has_lowering_side_effect, is_constant_64bit};
use crate::ir::pcc::{Fact, FactContext, PccError, PccResult};
use crate::ir::{
    ArgumentPurpose, Block, BlockArg, Constant, ConstantData, DataFlowGraph, ExternalName,
    Function, GlobalValue, GlobalValueData, Immediate, Inst, InstructionData, MemFlags,
    RelSourceLoc, SigRef, Signature, Type, Value, ValueDef, ValueLabelAssignments, ValueLabelStart,
};
use crate::machinst::valueregs::InvalidSentinel;
use crate::machinst::{
    ABIMachineSpec, BackwardsInsnIndex, BlockIndex, BlockLoweringOrder, CallArgList, CallInfo,
    CallRetList, Callee, InsnIndex, LoweredBlock, MachLabel, Reg, Sig, SigSet, TryCallInfo, VCode,
    VCodeBuilder, VCodeConstant, VCodeConstantData, VCodeConstants, VCodeInst, ValueRegs, Writable,
    writable_value_regs,
};
use crate::settings::Flags;
use crate::{CodegenError, CodegenResult, trace};
use alloc::vec::Vec;
use cranelift_control::ControlPlane;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::{SmallVec, smallvec};
use std::fmt::Debug;

use super::{VCodeBuildDirection, VRegAllocator};

/// A vector of ValueRegs, used to represent the outputs of an instruction.
pub type InstOutput = SmallVec<[ValueRegs<Reg>; 2]>;

/// An "instruction color" partitions CLIF instructions by side-effecting ops.
/// All instructions with the same "color" are guaranteed not to be separated by
/// any side-effecting op (for this purpose, loads are also considered
/// side-effecting, to avoid subtle questions w.r.t. the memory model), and
/// furthermore, it is guaranteed that for any two instructions A and B such
/// that color(A) == color(B), either A dominates B and B postdominates A, or
/// vice-versa. (For now, in practice, only ops in the same basic block can ever
/// have the same color, trivially providing the second condition.) Intuitively,
/// this means that the ops of the same color must always execute "together", as
/// part of one atomic contiguous section of the dynamic execution trace, and
/// they can be freely permuted (modulo true dataflow dependencies) without
/// affecting program behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct InstColor(u32);
impl InstColor {
    fn new(n: u32) -> InstColor {
        InstColor(n)
    }

    /// Get an arbitrary index representing this color. The index is unique
    /// *within a single function compilation*, but indices may be reused across
    /// functions.
    pub fn get(self) -> u32 {
        self.0
    }
}

/// A representation of all of the ways in which a value is available, aside
/// from as a direct register.
///
/// - An instruction, if it would be allowed to occur at the current location
///   instead (see [Lower::get_input_as_source_or_const()] for more details).
///
/// - A constant, if the value is known to be a constant.
#[derive(Clone, Copy, Debug)]
pub struct NonRegInput {
    /// An instruction produces this value (as the given output), and its
    /// computation (and side-effect if applicable) could occur at the
    /// current instruction's location instead.
    ///
    /// If this instruction's operation is merged into the current instruction,
    /// the backend must call [Lower::sink_inst()].
    ///
    /// This enum indicates whether this use of the source instruction
    /// is unique or not.
    pub inst: InputSourceInst,
    /// The value is a known constant.
    pub constant: Option<u64>,
}

/// When examining an input to an instruction, this enum provides one
/// of several options: there is or isn't a single instruction (that
/// we can see and merge with) that produces that input's value, and
/// we are or aren't the single user of that instruction.
#[derive(Clone, Copy, Debug)]
pub enum InputSourceInst {
    /// The input in question is the single, unique use of the given
    /// instruction and output index, and it can be sunk to the
    /// location of this input.
    UniqueUse(Inst, usize),
    /// The input in question is one of multiple uses of the given
    /// instruction. It can still be sunk to the location of this
    /// input.
    Use(Inst, usize),
    /// We cannot determine which instruction produced the input, or
    /// it is one of several instructions (e.g., due to a control-flow
    /// merge and blockparam), or the source instruction cannot be
    /// allowed to sink to the current location due to side-effects.
    None,
}

impl InputSourceInst {
    /// Get the instruction and output index for this source, whether
    /// we are its single or one of many users.
    pub fn as_inst(&self) -> Option<(Inst, usize)> {
        match self {
            &InputSourceInst::UniqueUse(inst, output_idx)
            | &InputSourceInst::Use(inst, output_idx) => Some((inst, output_idx)),
            &InputSourceInst::None => None,
        }
    }
}

/// A machine backend.
pub trait LowerBackend {
    /// The machine instruction type.
    type MInst: VCodeInst;

    /// Lower a single instruction.
    ///
    /// For a branch, this function should not generate the actual branch
    /// instruction. However, it must force any values it needs for the branch
    /// edge (block-param actuals) into registers, because the actual branch
    /// generation (`lower_branch()`) happens *after* any possible merged
    /// out-edge.
    ///
    /// Returns `None` if no lowering for the instruction was found.
    fn lower(&self, ctx: &mut Lower<Self::MInst>, inst: Inst) -> Option<InstOutput>;

    /// Lower a block-terminating group of branches (which together can be seen
    /// as one N-way branch), given a vcode MachLabel for each target.
    ///
    /// Returns `None` if no lowering for the branch was found.
    fn lower_branch(
        &self,
        ctx: &mut Lower<Self::MInst>,
        inst: Inst,
        targets: &[MachLabel],
    ) -> Option<()>;

    /// A bit of a hack: give a fixed register that always holds the result of a
    /// `get_pinned_reg` instruction, if known.  This allows elision of moves
    /// into the associated vreg, instead using the real reg directly.
    fn maybe_pinned_reg(&self) -> Option<Reg> {
        None
    }

    /// The type of state carried between `check_fact` invocations.
    type FactFlowState: Default + Clone + Debug;

    /// Check any facts about an instruction, given VCode with facts
    /// on VRegs. Takes mutable `VCode` so that it can propagate some
    /// kinds of facts automatically.
    fn check_fact(
        &self,
        _ctx: &FactContext<'_>,
        _vcode: &mut VCode<Self::MInst>,
        _inst: InsnIndex,
        _state: &mut Self::FactFlowState,
    ) -> PccResult<()> {
        Err(PccError::UnimplementedBackend)
    }
}

/// Machine-independent lowering driver / machine-instruction container. Maintains a correspondence
/// from original Inst to MachInsts.
pub struct Lower<'func, I: VCodeInst> {
    /// The function to lower.
    pub(crate) f: &'func Function,

    /// Lowered machine instructions.
    vcode: VCodeBuilder<I>,

    /// VReg allocation context, given to the vcode field at build time to finalize the vcode.
    vregs: VRegAllocator<I>,

    /// Mapping from `Value` (SSA value in IR) to virtual register.
    value_regs: SecondaryMap<Value, ValueRegs<Reg>>,

    /// sret registers, if needed.
    sret_reg: Option<ValueRegs<Reg>>,

    /// Instruction colors at block exits. From this map, we can recover all
    /// instruction colors by scanning backward from the block end and
    /// decrementing on any color-changing (side-effecting) instruction.
    block_end_colors: SecondaryMap<Block, InstColor>,

    /// Instruction colors at side-effecting ops. This is the *entry* color,
    /// i.e., the version of global state that exists before an instruction
    /// executes.  For each side-effecting instruction, the *exit* color is its
    /// entry color plus one.
    side_effect_inst_entry_colors: FxHashMap<Inst, InstColor>,

    /// Current color as we scan during lowering. While we are lowering an
    /// instruction, this is equal to the color *at entry to* the instruction.
    cur_scan_entry_color: Option<InstColor>,

    /// Current instruction as we scan during lowering.
    cur_inst: Option<Inst>,

    /// Instruction constant values, if known.
    inst_constants: FxHashMap<Inst, u64>,

    /// Use-counts per SSA value, as counted in the input IR. These
    /// are "coarsened", in the abstract-interpretation sense: we only
    /// care about "0, 1, many" states, as this is all we need and
    /// this lets us do an efficient fixpoint analysis.
    ///
    /// See doc comment on `ValueUseState` for more details.
    value_ir_uses: SecondaryMap<Value, ValueUseState>,

    /// Actual uses of each SSA value so far, incremented while lowering.
    value_lowered_uses: SecondaryMap<Value, u32>,

    /// Effectful instructions that have been sunk; they are not codegen'd at
    /// their original locations.
    inst_sunk: FxHashSet<Inst>,

    /// Instructions collected for the CLIF inst in progress, in forward order.
    ir_insts: Vec<I>,

    /// Try-call block arg normal-return values, indexed by instruction.
    try_call_rets: FxHashMap<Inst, SmallVec<[ValueRegs<Writable<Reg>>; 2]>>,

    /// Try-call block arg exceptional-return payloads, indexed by
    /// instruction. Payloads are carried in registers per the ABI and
    /// can only be one register each.
    try_call_payloads: FxHashMap<Inst, SmallVec<[Writable<Reg>; 2]>>,

    /// The register to use for GetPinnedReg, if any, on this architecture.
    pinned_reg: Option<Reg>,

    /// Compilation flags.
    flags: Flags,
}

/// How is a value used in the IR?
///
/// This can be seen as a coarsening of an integer count. We only need
/// distinct states for zero, one, or many.
///
/// This analysis deserves further explanation. The basic idea is that
/// we want to allow instruction lowering to know whether a value that
/// an instruction references is *only* referenced by that one use, or
/// by others as well. This is necessary to know when we might want to
/// move a side-effect: we cannot, for example, duplicate a load, so
/// we cannot let instruction lowering match a load as part of a
/// subpattern and potentially incorporate it.
///
/// Note that a lot of subtlety comes into play once we have
/// *indirect* uses. The classical example of this in our development
/// history was the x86 compare instruction, which is incorporated
/// into flags users (e.g. `selectif`, `trueif`, branches) and can
/// subsequently incorporate loads, or at least we would like it
/// to. However, danger awaits: the compare might be the only user of
/// a load, so we might think we can just move the load (and nothing
/// is duplicated -- success!), except that the compare itself is
/// codegen'd in multiple places, where it is incorporated as a
/// subpattern itself.
///
/// So we really want a notion of "unique all the way along the
/// matching path". Rust's `&T` and `&mut T` offer a partial analogy
/// to the semantics that we want here: we want to know when we've
/// matched a unique use of an instruction, and that instruction's
/// unique use of another instruction, etc, just as `&mut T` can only
/// be obtained by going through a chain of `&mut T`. If one has a
/// `&T` to a struct containing `&mut T` (one of several uses of an
/// instruction that itself has a unique use of an instruction), one
/// can only get a `&T` (one can only get a "I am one of several users
/// of this instruction" result).
///
/// We could track these paths, either dynamically as one "looks up the operand
/// tree" or precomputed. But the former requires state and means that the
/// `Lower` API carries that state implicitly, which we'd like to avoid if we
/// can. And the latter implies O(n^2) storage: it is an all-pairs property (is
/// inst `i` unique from the point of view of `j`).
///
/// To make matters even a little more complex still, a value that is
/// not uniquely used when initially viewing the IR can *become*
/// uniquely used, at least as a root allowing further unique uses of
/// e.g. loads to merge, if no other instruction actually merges
/// it. To be more concrete, if we have `v1 := load; v2 := op v1; v3
/// := op v2; v4 := op v2` then `v2` is non-uniquely used, so from the
/// point of view of lowering `v4` or `v3`, we cannot merge the load
/// at `v1`. But if we decide just to use the assigned register for
/// `v2` at both `v3` and `v4`, then we only actually codegen `v2`
/// once, so it *is* a unique root at that point and we *can* merge
/// the load.
///
/// Note also that the color scheme is not sufficient to give us this
/// information, for various reasons: reasoning about side-effects
/// does not tell us about potential duplication of uses through pure
/// ops.
///
/// To keep things simple and avoid error-prone lowering APIs that
/// would extract more information about whether instruction merging
/// happens or not (we don't have that info now, and it would be
/// difficult to refactor to get it and make that refactor 100%
/// correct), we give up on the above "can become unique if not
/// actually merged" point. Instead, we compute a
/// transitive-uniqueness. That is what this enum represents.
///
/// There is one final caveat as well to the result of this analysis.  Notably,
/// we define some instructions to be "root" instructions, which means that we
/// assume they will always be codegen'd at the root of a matching tree, and not
/// matched. (This comes with the caveat that we actually enforce this property
/// by making them "opaque" to subtree matching in
/// `get_value_as_source_or_const`). Because they will always be codegen'd once,
/// they in some sense "reset" multiplicity: these root instructions can be used
/// many times, but because their result(s) are only computed once, they only
/// use their inputs once.
///
/// We currently define all multi-result instructions to be "root" instructions,
/// because it is too complex to reason about matching through them, and they
/// cause too-coarse-grained approximation of multiplicity otherwise: the
/// analysis would have to assume (as it used to!) that they are always
/// multiply-used, simply because they have multiple outputs even if those
/// outputs are used only once.
///
/// In the future we could define other instructions to be "root" instructions
/// as well, if we make the corresponding change to get_value_as_source_or_const
/// as well.
///
/// To define `ValueUseState` more plainly: a value is `Unused` if no references
/// exist to it; `Once` if only one other op refers to it, *and* that other op
/// is `Unused` or `Once`; and `Multiple` otherwise. In other words, `Multiple`
/// is contagious (except through root instructions): even if an op's result
/// value is directly used only once in the CLIF, that value is `Multiple` if
/// the op that uses it is itself used multiple times (hence could be codegen'd
/// multiple times). In brief, this analysis tells us whether, if every op
/// merged all of its operand tree, a given op could be codegen'd in more than
/// one place.
///
/// To compute this, we first consider direct uses. At this point
/// `Unused` answers are correct, `Multiple` answers are correct, but
/// some `Once`s may change to `Multiple`s. Then we propagate
/// `Multiple` transitively using a workqueue/fixpoint algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ValueUseState {
    /// Not used at all.
    Unused,
    /// Used exactly once.
    Once,
    /// Used multiple times.
    Multiple,
}

impl ValueUseState {
    /// Add one use.
    fn inc(&mut self) {
        let new = match self {
            Self::Unused => Self::Once,
            Self::Once | Self::Multiple => Self::Multiple,
        };
        *self = new;
    }
}

/// Notion of "relocation distance". This gives an estimate of how far away a symbol will be from a
/// reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelocDistance {
    /// Target of relocation is "nearby". The threshold for this is fuzzy but should be interpreted
    /// as approximately "within the compiled output of one module"; e.g., within AArch64's +/-
    /// 128MB offset. If unsure, use `Far` instead.
    Near,
    /// Target of relocation could be anywhere in the address space.
    Far,
}

impl<'func, I: VCodeInst> Lower<'func, I> {
    /// Prepare a new lowering context for the given IR function.
    pub fn new(
        f: &'func Function,
        abi: Callee<I::ABIMachineSpec>,
        emit_info: I::Info,
        block_order: BlockLoweringOrder,
        sigs: SigSet,
        flags: Flags,
    ) -> CodegenResult<Self> {
        let constants = VCodeConstants::with_capacity(f.dfg.constants.len());
        let vcode = VCodeBuilder::new(
            sigs,
            abi,
            emit_info,
            block_order,
            constants,
            VCodeBuildDirection::Backward,
            flags.log2_min_function_alignment(),
        );

        // We usually need two VRegs per instruction result, plus extras for
        // various temporaries, but two per Value is a good starting point.
        let mut vregs = VRegAllocator::with_capacity(f.dfg.num_values() * 2);

        let mut value_regs = SecondaryMap::with_default(ValueRegs::invalid());
        let mut try_call_rets = FxHashMap::default();
        let mut try_call_payloads = FxHashMap::default();

        // Assign a vreg to each block param, each inst result, and
        // each edge-defined block-call arg.
        for bb in f.layout.blocks() {
            for &param in f.dfg.block_params(bb) {
                let ty = f.dfg.value_type(param);
                if value_regs[param].is_invalid() {
                    let regs = vregs.alloc_with_maybe_fact(ty, f.dfg.facts[param].clone())?;
                    value_regs[param] = regs;
                    trace!("bb {} param {}: regs {:?}", bb, param, regs);
                }
            }
            for inst in f.layout.block_insts(bb) {
                for &result in f.dfg.inst_results(inst) {
                    let ty = f.dfg.value_type(result);
                    if value_regs[result].is_invalid() && !ty.is_invalid() {
                        let regs = vregs.alloc_with_maybe_fact(ty, f.dfg.facts[result].clone())?;
                        value_regs[result] = regs;
                        trace!(
                            "bb {} inst {} ({:?}): result {} regs {:?}",
                            bb, inst, f.dfg.insts[inst], result, regs,
                        );
                    }
                }

                if let Some(et) = f.dfg.insts[inst].exception_table() {
                    let exdata = &f.dfg.exception_tables[et];
                    let sig = &f.dfg.signatures[exdata.signature()];

                    let mut rets = smallvec![];
                    for ty in sig.returns.iter().map(|ret| ret.value_type) {
                        rets.push(vregs.alloc(ty)?.map(|r| Writable::from_reg(r)));
                    }
                    try_call_rets.insert(inst, rets);

                    let mut payloads = smallvec![];
                    for &ty in sig
                        .call_conv
                        .exception_payload_types(I::ABIMachineSpec::word_type())
                    {
                        payloads.push(Writable::from_reg(vregs.alloc(ty)?.only_reg().unwrap()));
                    }
                    try_call_payloads.insert(inst, payloads);
                }
            }
        }

        // Find the sret register, if it's used.
        let mut sret_param = None;
        for ret in vcode.abi().signature().returns.iter() {
            if ret.purpose == ArgumentPurpose::StructReturn {
                let entry_bb = f.stencil.layout.entry_block().unwrap();
                for (&param, sig_param) in f
                    .dfg
                    .block_params(entry_bb)
                    .iter()
                    .zip(vcode.abi().signature().params.iter())
                {
                    if sig_param.purpose == ArgumentPurpose::StructReturn {
                        assert!(sret_param.is_none());
                        sret_param = Some(param);
                    }
                }

                assert!(sret_param.is_some());
            }
        }

        let sret_reg = sret_param.map(|param| {
            let regs = value_regs[param];
            assert!(regs.len() == 1);
            regs
        });

        // Compute instruction colors, find constant instructions, and find instructions with
        // side-effects, in one combined pass.
        let mut cur_color = 0;
        let mut block_end_colors = SecondaryMap::with_default(InstColor::new(0));
        let mut side_effect_inst_entry_colors = FxHashMap::default();
        let mut inst_constants = FxHashMap::default();
        for bb in f.layout.blocks() {
            cur_color += 1;
            for inst in f.layout.block_insts(bb) {
                let side_effect = has_lowering_side_effect(f, inst);

                trace!("bb {} inst {} has color {}", bb, inst, cur_color);
                if side_effect {
                    side_effect_inst_entry_colors.insert(inst, InstColor::new(cur_color));
                    trace!(" -> side-effecting; incrementing color for next inst");
                    cur_color += 1;
                }

                // Determine if this is a constant; if so, add to the table.
                if let Some(c) = is_constant_64bit(f, inst) {
                    trace!(" -> constant: {}", c);
                    inst_constants.insert(inst, c);
                }
            }

            block_end_colors[bb] = InstColor::new(cur_color);
        }

        let value_ir_uses = compute_use_states(f, sret_param);

        Ok(Lower {
            f,
            vcode,
            vregs,
            value_regs,
            sret_reg,
            block_end_colors,
            side_effect_inst_entry_colors,
            inst_constants,
            value_ir_uses,
            value_lowered_uses: SecondaryMap::default(),
            inst_sunk: FxHashSet::default(),
            cur_scan_entry_color: None,
            cur_inst: None,
            ir_insts: vec![],
            try_call_rets,
            try_call_payloads,
            pinned_reg: None,
            flags,
        })
    }

    pub fn sigs(&self) -> &SigSet {
        self.vcode.sigs()
    }

    pub fn sigs_mut(&mut self) -> &mut SigSet {
        self.vcode.sigs_mut()
    }

    pub fn vregs_mut(&mut self) -> &mut VRegAllocator<I> {
        &mut self.vregs
    }

    fn gen_arg_setup(&mut self) {
        if let Some(entry_bb) = self.f.layout.entry_block() {
            trace!(
                "gen_arg_setup: entry BB {} args are:\n{:?}",
                entry_bb,
                self.f.dfg.block_params(entry_bb)
            );

            for (i, param) in self.f.dfg.block_params(entry_bb).iter().enumerate() {
                if self.value_ir_uses[*param] == ValueUseState::Unused {
                    continue;
                }
                let regs = writable_value_regs(self.value_regs[*param]);
                for insn in self
                    .vcode
                    .vcode
                    .abi
                    .gen_copy_arg_to_regs(&self.vcode.vcode.sigs, i, regs, &mut self.vregs)
                    .into_iter()
                {
                    self.emit(insn);
                }
            }
            if let Some(insn) = self
                .vcode
                .vcode
                .abi
                .gen_retval_area_setup(&self.vcode.vcode.sigs, &mut self.vregs)
            {
                self.emit(insn);
            }

            // The `args` instruction below must come first. Finish
            // the current "IR inst" (with a default source location,
            // as for other special instructions inserted during
            // lowering) and continue the scan backward.
            self.finish_ir_inst(Default::default());

            if let Some(insn) = self.vcode.vcode.abi.take_args() {
                self.emit(insn);
            }
        }
    }

    /// Generate the return instruction.
    pub fn gen_return(&mut self, rets: &[ValueRegs<Reg>]) {
        let mut out_rets = vec![];

        let mut rets = rets.into_iter();
        for (i, ret) in self
            .abi()
            .signature()
            .returns
            .clone()
            .into_iter()
            .enumerate()
        {
            let regs = if ret.purpose == ArgumentPurpose::StructReturn {
                self.sret_reg.unwrap()
            } else {
                *rets.next().unwrap()
            };

            let (regs, insns) = self.vcode.abi().gen_copy_regs_to_retval(
                self.vcode.sigs(),
                i,
                regs,
                &mut self.vregs,
            );
            out_rets.extend(regs);
            for insn in insns {
                self.emit(insn);
            }
        }

        // Hack: generate a virtual instruction that uses vmctx in
        // order to keep it alive for the duration of the function,
        // for the benefit of debuginfo.
        if self.f.dfg.values_labels.is_some() {
            if let Some(vmctx_val) = self.f.special_param(ArgumentPurpose::VMContext) {
                if self.value_ir_uses[vmctx_val] != ValueUseState::Unused {
                    let vmctx_reg = self.value_regs[vmctx_val].only_reg().unwrap();
                    self.emit(I::gen_dummy_use(vmctx_reg));
                }
            }
        }

        let inst = self.abi().gen_rets(out_rets);
        self.emit(inst);
    }

    /// Generate list of registers to hold the output of a call with
    /// signature `sig`.
    pub fn gen_call_output(&mut self, sig: &Signature) -> InstOutput {
        let mut rets = smallvec![];
        for ty in sig.returns.iter().map(|ret| ret.value_type) {
            rets.push(self.vregs.alloc_with_deferred_error(ty));
        }
        rets
    }

    /// Likewise, but for a `SigRef` instead.
    pub fn gen_call_output_from_sig_ref(&mut self, sig_ref: SigRef) -> InstOutput {
        self.gen_call_output(&self.f.dfg.signatures[sig_ref])
    }

    /// Set up arguments values `args` for a call with signature `sig`.
    pub fn gen_call_args(&mut self, sig: Sig, args: &[ValueRegs<Reg>]) -> CallArgList {
        let (uses, insts) = self.vcode.abi().gen_call_args(
            self.vcode.sigs(),
            sig,
            args,
            /* is_tail_call */ false,
            &self.flags,
            &mut self.vregs,
        );
        for insn in insts {
            self.emit(insn);
        }
        uses
    }

    /// Likewise, but for a `return_call`.
    pub fn gen_return_call_args(&mut self, sig: Sig, args: &[ValueRegs<Reg>]) -> CallArgList {
        let (uses, insts) = self.vcode.abi().gen_call_args(
            self.vcode.sigs(),
            sig,
            args,
            /* is_tail_call */ true,
            &self.flags,
            &mut self.vregs,
        );
        for insn in insts {
            self.emit(insn);
        }
        uses
    }

    /// Set up return values `outputs` for a call with signature `sig`.
    pub fn gen_call_rets(&mut self, sig: Sig, outputs: &[ValueRegs<Reg>]) -> CallRetList {
        self.vcode
            .abi()
            .gen_call_rets(self.vcode.sigs(), sig, outputs, None, &mut self.vregs)
    }

    /// Likewise, but for a `try_call`.
    pub fn gen_try_call_rets(&mut self, sig: Sig) -> CallRetList {
        let ir_inst = self.cur_inst.unwrap();
        let mut outputs: SmallVec<[ValueRegs<Reg>; 2]> = smallvec![];
        for return_def in self.try_call_rets.get(&ir_inst).unwrap() {
            outputs.push(return_def.map(|r| r.to_reg()));
        }
        let payloads = Some(&self.try_call_payloads.get(&ir_inst).unwrap()[..]);

        self.vcode
            .abi()
            .gen_call_rets(self.vcode.sigs(), sig, &outputs, payloads, &mut self.vregs)
    }

    /// Populate a `CallInfo` for a call with signature `sig`.
    pub fn gen_call_info<T>(
        &mut self,
        sig: Sig,
        dest: T,
        uses: CallArgList,
        defs: CallRetList,
        try_call_info: Option<TryCallInfo>,
    ) -> CallInfo<T> {
        self.vcode
            .abi()
            .gen_call_info(self.vcode.sigs(), sig, dest, uses, defs, try_call_info)
    }

    /// Has this instruction been sunk to a use-site (i.e., away from its
    /// original location)?
    fn is_inst_sunk(&self, inst: Inst) -> bool {
        self.inst_sunk.contains(&inst)
    }

    // Is any result of this instruction needed?
    fn is_any_inst_result_needed(&self, inst: Inst) -> bool {
        self.f
            .dfg
            .inst_results(inst)
            .iter()
            .any(|&result| self.value_lowered_uses[result] > 0)
    }

    fn lower_clif_block<B: LowerBackend<MInst = I>>(
        &mut self,
        backend: &B,
        block: Block,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<()> {
        self.cur_scan_entry_color = Some(self.block_end_colors[block]);
        // Lowering loop:
        // - For each non-branch instruction, in reverse order:
        //   - If side-effecting (load, store, branch/call/return,
        //     possible trap), or if used outside of this block, or if
        //     demanded by another inst, then lower.
        //
        // That's it! Lowering of side-effecting ops will force all *needed*
        // (live) non-side-effecting ops to be lowered at the right places, via
        // the `use_input_reg()` callback on the `Lower` (that's us). That's
        // because `use_input_reg()` sets the eager/demand bit for any insts
        // whose result registers are used.
        //
        // We set the VCodeBuilder to "backward" mode, so we emit
        // blocks in reverse order wrt the BlockIndex sequence, and
        // emit instructions in reverse order within blocks.  Because
        // the machine backend calls `ctx.emit()` in forward order, we
        // collect per-IR-inst lowered instructions in `ir_insts`,
        // then reverse these and append to the VCode at the end of
        // each IR instruction.
        for inst in self.f.layout.block_insts(block).rev() {
            let data = &self.f.dfg.insts[inst];
            let has_side_effect = has_lowering_side_effect(self.f, inst);
            // If  inst has been sunk to another location, skip it.
            if self.is_inst_sunk(inst) {
                continue;
            }
            // Are any outputs used at least once?
            let value_needed = self.is_any_inst_result_needed(inst);
            trace!(
                "lower_clif_block: block {} inst {} ({:?}) is_branch {} side_effect {} value_needed {}",
                block,
                inst,
                data,
                data.opcode().is_branch(),
                has_side_effect,
                value_needed,
            );

            // Update scan state to color prior to this inst (as we are scanning
            // backward).
            self.cur_inst = Some(inst);
            if has_side_effect {
                let entry_color = *self
                    .side_effect_inst_entry_colors
                    .get(&inst)
                    .expect("every side-effecting inst should have a color-map entry");
                self.cur_scan_entry_color = Some(entry_color);
            }

            // Skip lowering branches; these are handled separately
            // (see `lower_clif_branches()` below).
            if self.f.dfg.insts[inst].opcode().is_branch() {
                continue;
            }

            // Value defined by "inst" becomes live after it in normal
            // order, and therefore **before** in reversed order.
            self.emit_value_label_live_range_start_for_inst(inst);

            // Normal instruction: codegen if the instruction is side-effecting
            // or any of its outputs is used.
            if has_side_effect || value_needed {
                trace!("lowering: inst {}: {}", inst, self.f.dfg.display_inst(inst));
                let temp_regs = match backend.lower(self, inst) {
                    Some(regs) => regs,
                    None => {
                        let ty = if self.num_outputs(inst) > 0 {
                            Some(self.output_ty(inst, 0))
                        } else {
                            None
                        };
                        return Err(CodegenError::Unsupported(format!(
                            "should be implemented in ISLE: inst = `{}`, type = `{:?}`",
                            self.f.dfg.display_inst(inst),
                            ty
                        )));
                    }
                };

                // The ISLE generated code emits its own registers to define the
                // instruction's lowered values in. However, other instructions
                // that use this SSA value will be lowered assuming that the value
                // is generated into a pre-assigned, different, register.
                //
                // To connect the two, we set up "aliases" in the VCodeBuilder
                // that apply when it is building the Operand table for the
                // regalloc to use. These aliases effectively rewrite any use of
                // the pre-assigned register to the register that was returned by
                // the ISLE lowering logic.
                let results = self.f.dfg.inst_results(inst);
                debug_assert_eq!(temp_regs.len(), results.len());
                for (regs, &result) in temp_regs.iter().zip(results) {
                    let dsts = self.value_regs[result];
                    let mut regs = regs.regs().iter();
                    for &dst in dsts.regs().iter() {
                        let temp = regs.next().copied().unwrap_or(Reg::invalid_sentinel());
                        trace!("set vreg alias: {result:?} = {dst:?}, lowering = {temp:?}");
                        self.vregs.set_vreg_alias(dst, temp);
                    }
                }
            }

            let start = self.vcode.vcode.num_insts();
            let loc = self.srcloc(inst);
            self.finish_ir_inst(loc);

            // If the instruction had a user stack map, forward it from the CLIF
            // to the vcode.
            if let Some(entries) = self.f.dfg.user_stack_map_entries(inst) {
                let end = self.vcode.vcode.num_insts();
                debug_assert!(end > start);
                debug_assert_eq!(
                    (start..end)
                        .filter(|i| self.vcode.vcode[InsnIndex::new(*i)].is_safepoint())
                        .count(),
                    1
                );
                for i in start..end {
                    let iix = InsnIndex::new(i);
                    if self.vcode.vcode[iix].is_safepoint() {
                        trace!(
                            "Adding user stack map from clif\n\n\
                                 {inst:?} `{}`\n\n\
                             to vcode\n\n\
                                 {iix:?} `{}`",
                            self.f.dfg.display_inst(inst),
                            &self.vcode.vcode[iix].pretty_print_inst(&mut Default::default()),
                        );
                        self.vcode
                            .add_user_stack_map(BackwardsInsnIndex::new(iix.index()), entries);
                        break;
                    }
                }
            }

            // maybe insert random instruction
            if ctrl_plane.get_decision() {
                if ctrl_plane.get_decision() {
                    let imm: u64 = ctrl_plane.get_arbitrary();
                    let reg = self.alloc_tmp(crate::ir::types::I64).regs()[0];
                    I::gen_imm_u64(imm, reg).map(|inst| self.emit(inst));
                } else {
                    let imm: f64 = ctrl_plane.get_arbitrary();
                    let tmp = self.alloc_tmp(crate::ir::types::I64).regs()[0];
                    let reg = self.alloc_tmp(crate::ir::types::F64).regs()[0];
                    for inst in I::gen_imm_f64(imm, tmp, reg) {
                        self.emit(inst);
                    }
                }
            }
        }

        // Add the block params to this block.
        self.add_block_params(block)?;

        self.cur_scan_entry_color = None;
        Ok(())
    }

    fn add_block_params(&mut self, block: Block) -> CodegenResult<()> {
        for &param in self.f.dfg.block_params(block) {
            for &reg in self.value_regs[param].regs() {
                let vreg = reg.to_virtual_reg().unwrap();
                self.vcode.add_block_param(vreg);
            }
        }
        Ok(())
    }

    fn get_value_labels<'a>(&'a self, val: Value, depth: usize) -> Option<&'a [ValueLabelStart]> {
        if let Some(ref values_labels) = self.f.dfg.values_labels {
            debug_assert!(self.f.dfg.value_is_real(val));
            trace!(
                "get_value_labels: val {} -> {:?}",
                val,
                values_labels.get(&val)
            );
            match values_labels.get(&val) {
                Some(&ValueLabelAssignments::Starts(ref list)) => Some(&list[..]),
                Some(&ValueLabelAssignments::Alias { value, .. }) if depth < 10 => {
                    self.get_value_labels(value, depth + 1)
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn emit_value_label_marks_for_value(&mut self, val: Value) {
        let regs = self.value_regs[val];
        if regs.len() > 1 {
            return;
        }
        let reg = regs.only_reg().unwrap();

        if let Some(label_starts) = self.get_value_labels(val, 0) {
            let labels = label_starts
                .iter()
                .map(|&ValueLabelStart { label, .. }| label)
                .collect::<FxHashSet<_>>();
            for label in labels {
                trace!(
                    "value labeling: defines val {:?} -> reg {:?} -> label {:?}",
                    val, reg, label,
                );
                self.vcode.add_value_label(reg, label);
            }
        }
    }

    fn emit_value_label_live_range_start_for_inst(&mut self, inst: Inst) {
        if self.f.dfg.values_labels.is_none() {
            return;
        }

        trace!(
            "value labeling: srcloc {}: inst {}",
            self.srcloc(inst),
            inst
        );
        for &val in self.f.dfg.inst_results(inst) {
            self.emit_value_label_marks_for_value(val);
        }
    }

    fn emit_value_label_live_range_start_for_block_args(&mut self, block: Block) {
        if self.f.dfg.values_labels.is_none() {
            return;
        }

        trace!("value labeling: block {}", block);
        for &arg in self.f.dfg.block_params(block) {
            self.emit_value_label_marks_for_value(arg);
        }
        self.finish_ir_inst(Default::default());
    }

    fn finish_ir_inst(&mut self, loc: RelSourceLoc) {
        // The VCodeBuilder builds in reverse order (and reverses at
        // the end), but `ir_insts` is in forward order, so reverse
        // it.
        for inst in self.ir_insts.drain(..).rev() {
            self.vcode.push(inst, loc);
        }
    }

    fn finish_bb(&mut self) {
        self.vcode.end_bb();
    }

    fn lower_clif_branch<B: LowerBackend<MInst = I>>(
        &mut self,
        backend: &B,
        // Lowered block index:
        bindex: BlockIndex,
        // Original CLIF block:
        block: Block,
        branch: Inst,
        targets: &[MachLabel],
    ) -> CodegenResult<()> {
        trace!(
            "lower_clif_branch: block {} branch {:?} targets {:?}",
            block, branch, targets,
        );
        // When considering code-motion opportunities, consider the current
        // program point to be this branch.
        self.cur_inst = Some(branch);

        // Lower the branch in ISLE.
        backend
            .lower_branch(self, branch, targets)
            .unwrap_or_else(|| {
                panic!(
                    "should be implemented in ISLE: branch = `{}`",
                    self.f.dfg.display_inst(branch),
                )
            });
        let loc = self.srcloc(branch);
        self.finish_ir_inst(loc);
        // Add block param outputs for current block.
        self.lower_branch_blockparam_args(bindex);
        Ok(())
    }

    fn lower_branch_blockparam_args(&mut self, block: BlockIndex) {
        let mut branch_arg_vregs: SmallVec<[Reg; 16]> = smallvec![];

        // TODO: why not make `block_order` public?
        for succ_idx in 0..self.vcode.block_order().succ_indices(block).1.len() {
            branch_arg_vregs.clear();
            let (succ, args) = self.collect_block_call(block, succ_idx, &mut branch_arg_vregs);
            self.vcode.add_succ(succ, args);
        }
    }

    fn collect_branch_and_targets(
        &self,
        bindex: BlockIndex,
        _bb: Block,
        targets: &mut SmallVec<[MachLabel; 2]>,
    ) -> Option<Inst> {
        targets.clear();
        let (opt_inst, succs) = self.vcode.block_order().succ_indices(bindex);
        targets.extend(succs.iter().map(|succ| MachLabel::from_block(*succ)));
        opt_inst
    }

    /// Collect the outgoing block-call arguments for a given edge out
    /// of a lowered block.
    fn collect_block_call<'a>(
        &mut self,
        block: BlockIndex,
        succ_idx: usize,
        buffer: &'a mut SmallVec<[Reg; 16]>,
    ) -> (BlockIndex, &'a [Reg]) {
        let block_order = self.vcode.block_order();
        let (_, succs) = block_order.succ_indices(block);
        let succ = succs[succ_idx];
        let this_lb = block_order.lowered_order()[block.index()];
        let succ_lb = block_order.lowered_order()[succ.index()];

        let (branch_inst, succ_idx) = match (this_lb, succ_lb) {
            (_, LoweredBlock::CriticalEdge { .. }) => {
                // The successor is a split-critical-edge block. In this
                // case, this block-call has no arguments, and the
                // arguments go on the critical edge block's unconditional
                // branch instead.
                return (succ, &[]);
            }
            (LoweredBlock::CriticalEdge { pred, succ_idx, .. }, _) => {
                // This is a split-critical-edge block. In this case, our
                // block-call has the arguments that in the CLIF appear in
                // the predecessor's branch to this edge.
                let branch_inst = self.f.layout.last_inst(pred).unwrap();
                (branch_inst, succ_idx as usize)
            }

            (this, _) => {
                let block = this.orig_block().unwrap();
                // Ordinary block, with an ordinary block as
                // successor. Take the arguments from the branch.
                let branch_inst = self.f.layout.last_inst(block).unwrap();
                (branch_inst, succ_idx)
            }
        };

        let block_call = self.f.dfg.insts[branch_inst]
            .branch_destination(&self.f.dfg.jump_tables, &self.f.dfg.exception_tables)[succ_idx];
        for arg in block_call.args(&self.f.dfg.value_lists) {
            match arg {
                BlockArg::Value(arg) => {
                    debug_assert!(self.f.dfg.value_is_real(arg));
                    let regs = self.put_value_in_regs(arg);
                    buffer.extend_from_slice(regs.regs());
                }
                BlockArg::TryCallRet(i) => {
                    let regs = self.try_call_rets.get(&branch_inst).unwrap()[i as usize]
                        .map(|r| r.to_reg());
                    buffer.extend_from_slice(regs.regs());
                }
                BlockArg::TryCallExn(i) => {
                    let reg =
                        self.try_call_payloads.get(&branch_inst).unwrap()[i as usize].to_reg();
                    buffer.push(reg);
                }
            }
        }
        (succ, &buffer[..])
    }

    /// Lower the function.
    pub fn lower<B: LowerBackend<MInst = I>>(
        mut self,
        backend: &B,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<VCode<I>> {
        trace!("about to lower function: {:?}", self.f);

        self.vcode.init_retval_area(&mut self.vregs)?;

        // Get the pinned reg here (we only parameterize this function on `B`,
        // not the whole `Lower` impl).
        self.pinned_reg = backend.maybe_pinned_reg();

        self.vcode.set_entry(BlockIndex::new(0));

        // Reused vectors for branch lowering.
        let mut targets: SmallVec<[MachLabel; 2]> = SmallVec::new();

        // get a copy of the lowered order; we hold this separately because we
        // need a mut ref to the vcode to mutate it below.
        let lowered_order: SmallVec<[LoweredBlock; 64]> = self
            .vcode
            .block_order()
            .lowered_order()
            .iter()
            .cloned()
            .collect();

        // Main lowering loop over lowered blocks.
        for (bindex, lb) in lowered_order.iter().enumerate().rev() {
            let bindex = BlockIndex::new(bindex);

            // Lower the block body in reverse order (see comment in
            // `lower_clif_block()` for rationale).

            // End branch.
            if let Some(bb) = lb.orig_block() {
                if let Some(branch) = self.collect_branch_and_targets(bindex, bb, &mut targets) {
                    self.lower_clif_branch(backend, bindex, bb, branch, &targets)?;
                    self.finish_ir_inst(self.srcloc(branch));
                }
            } else {
                // If no orig block, this must be a pure edge block;
                // get the successor and emit a jump. This block has
                // no block params; and this jump's block-call args
                // will be filled in by
                // `lower_branch_blockparam_args`.
                let succ = self.vcode.block_order().succ_indices(bindex).1[0];
                self.emit(I::gen_jump(MachLabel::from_block(succ)));
                self.finish_ir_inst(Default::default());
                self.lower_branch_blockparam_args(bindex);
            }

            // Original block body.
            if let Some(bb) = lb.orig_block() {
                self.lower_clif_block(backend, bb, ctrl_plane)?;
                self.emit_value_label_live_range_start_for_block_args(bb);
            }

            if bindex.index() == 0 {
                // Set up the function with arg vreg inits.
                self.gen_arg_setup();
                self.finish_ir_inst(Default::default());
            }

            self.finish_bb();

            // Check for any deferred vreg-temp allocation errors, and
            // bubble one up at this time if it exists.
            if let Some(e) = self.vregs.take_deferred_error() {
                return Err(e);
            }
        }

        // Now that we've emitted all instructions into the
        // VCodeBuilder, let's build the VCode.
        trace!(
            "built vcode:\n{:?}Backwards {:?}",
            &self.vregs, &self.vcode.vcode
        );
        let vcode = self.vcode.build(self.vregs);

        Ok(vcode)
    }

    pub fn value_is_unused(&self, val: Value) -> bool {
        match self.value_ir_uses[val] {
            ValueUseState::Unused => true,
            _ => false,
        }
    }
}

/// Pre-analysis: compute `value_ir_uses`. See comment on
/// `ValueUseState` for a description of what this analysis
/// computes.
fn compute_use_states(
    f: &Function,
    sret_param: Option<Value>,
) -> SecondaryMap<Value, ValueUseState> {
    // We perform the analysis without recursion, so we don't
    // overflow the stack on long chains of ops in the input.
    //
    // This is sort of a hybrid of a "shallow use-count" pass and
    // a DFS. We iterate over all instructions and mark their args
    // as used. However when we increment a use-count to
    // "Multiple" we push its args onto the stack and do a DFS,
    // immediately marking the whole dependency tree as
    // Multiple. Doing both (shallow use-counting over all insts,
    // and deep Multiple propagation) lets us trim both
    // traversals, stopping recursion when a node is already at
    // the appropriate state.
    //
    // In particular, note that the *coarsening* into {Unused,
    // Once, Multiple} is part of what makes this pass more
    // efficient than a full indirect-use-counting pass.

    let mut value_ir_uses = SecondaryMap::with_default(ValueUseState::Unused);

    if let Some(sret_param) = sret_param {
        // There's an implicit use of the struct-return parameter in each
        // copy of the function epilogue, which we count here.
        value_ir_uses[sret_param] = ValueUseState::Multiple;
    }

    // Stack of iterators over Values as we do DFS to mark
    // Multiple-state subtrees. The iterator type is whatever is
    // returned by `uses` below.
    let mut stack: SmallVec<[_; 16]> = smallvec![];

    // Find the args for the inst corresponding to the given value.
    //
    // Note that "root" instructions are skipped here. This means that multiple
    // uses of any result of a multi-result instruction are not considered
    // multiple uses of the operands of a multi-result instruction. This
    // requires tight coupling with `get_value_as_source_or_const` above which
    // is the consumer of the map that this function is producing.
    let uses = |value| {
        trace!(" -> pushing args for {} onto stack", value);
        if let ValueDef::Result(src_inst, _) = f.dfg.value_def(value) {
            if is_value_use_root(f, src_inst) {
                None
            } else {
                Some(f.dfg.inst_values(src_inst))
            }
        } else {
            None
        }
    };

    // Do a DFS through `value_ir_uses` to mark a subtree as
    // Multiple.
    for inst in f
        .layout
        .blocks()
        .flat_map(|block| f.layout.block_insts(block))
    {
        // Iterate over all values used by all instructions, noting an
        // additional use on each operand.
        for arg in f.dfg.inst_values(inst) {
            debug_assert!(f.dfg.value_is_real(arg));
            let old = value_ir_uses[arg];
            value_ir_uses[arg].inc();
            let new = value_ir_uses[arg];
            trace!("arg {} used, old state {:?}, new {:?}", arg, old, new);

            // On transition to Multiple, do DFS.
            if old == ValueUseState::Multiple || new != ValueUseState::Multiple {
                continue;
            }
            if let Some(iter) = uses(arg) {
                stack.push(iter);
            }
            while let Some(iter) = stack.last_mut() {
                if let Some(value) = iter.next() {
                    debug_assert!(f.dfg.value_is_real(value));
                    trace!(" -> DFS reaches {}", value);
                    if value_ir_uses[value] == ValueUseState::Multiple {
                        // Truncate DFS here: no need to go further,
                        // as whole subtree must already be Multiple.
                        // With debug asserts, check one level of
                        // that invariant at least.
                        debug_assert!(uses(value).into_iter().flatten().all(|arg| {
                            debug_assert!(f.dfg.value_is_real(arg));
                            value_ir_uses[arg] == ValueUseState::Multiple
                        }));
                        continue;
                    }
                    value_ir_uses[value] = ValueUseState::Multiple;
                    trace!(" -> became Multiple");
                    if let Some(iter) = uses(value) {
                        stack.push(iter);
                    }
                } else {
                    // Empty iterator, discard.
                    stack.pop();
                }
            }
        }
    }

    value_ir_uses
}

/// Definition of a "root" instruction for the calculation of `ValueUseState`.
///
/// This function calculates whether `inst` is considered a "root" for value-use
/// information. This concept is used to forcibly prevent looking-through the
/// instruction during `get_value_as_source_or_const` as it additionally
/// prevents propagating `Multiple`-used results of the `inst` here to the
/// operands of the instruction.
///
/// Currently this is defined as multi-result instructions. That means that
/// lowerings are never allowed to look through a multi-result instruction to
/// generate patterns. Note that this isn't possible in ISLE today anyway so
/// this isn't currently much of a loss.
///
/// The main purpose of this function is to prevent the operands of a
/// multi-result instruction from being forcibly considered `Multiple`-used
/// regardless of circumstances.
fn is_value_use_root(f: &Function, inst: Inst) -> bool {
    f.dfg.inst_results(inst).len() > 1
}

/// Function-level queries.
impl<'func, I: VCodeInst> Lower<'func, I> {
    pub fn dfg(&self) -> &DataFlowGraph {
        &self.f.dfg
    }

    /// Get the `Callee`.
    pub fn abi(&self) -> &Callee<I::ABIMachineSpec> {
        self.vcode.abi()
    }

    /// Get the `Callee`.
    pub fn abi_mut(&mut self) -> &mut Callee<I::ABIMachineSpec> {
        self.vcode.abi_mut()
    }
}

/// Instruction input/output queries.
impl<'func, I: VCodeInst> Lower<'func, I> {
    /// Get the instdata for a given IR instruction.
    pub fn data(&self, ir_inst: Inst) -> &InstructionData {
        &self.f.dfg.insts[ir_inst]
    }

    /// Likewise, but starting with a GlobalValue identifier.
    pub fn symbol_value_data<'b>(
        &'b self,
        global_value: GlobalValue,
    ) -> Option<(&'b ExternalName, RelocDistance, i64)> {
        let gvdata = &self.f.global_values[global_value];
        match gvdata {
            &GlobalValueData::Symbol {
                ref name,
                ref offset,
                colocated,
                ..
            } => {
                let offset = offset.bits();
                let dist = if colocated {
                    RelocDistance::Near
                } else {
                    RelocDistance::Far
                };
                Some((name, dist, offset))
            }
            _ => None,
        }
    }

    /// Returns the memory flags of a given memory access.
    pub fn memflags(&self, ir_inst: Inst) -> Option<MemFlags> {
        match &self.f.dfg.insts[ir_inst] {
            &InstructionData::AtomicCas { flags, .. } => Some(flags),
            &InstructionData::AtomicRmw { flags, .. } => Some(flags),
            &InstructionData::Load { flags, .. }
            | &InstructionData::LoadNoOffset { flags, .. }
            | &InstructionData::Store { flags, .. } => Some(flags),
            &InstructionData::StoreNoOffset { flags, .. } => Some(flags),
            _ => None,
        }
    }

    /// Get the source location for a given instruction.
    pub fn srcloc(&self, ir_inst: Inst) -> RelSourceLoc {
        self.f.rel_srclocs()[ir_inst]
    }

    /// Get the number of inputs to the given IR instruction. This is a count only of the Value
    /// arguments to the instruction: block arguments will not be included in this count.
    pub fn num_inputs(&self, ir_inst: Inst) -> usize {
        self.f.dfg.inst_args(ir_inst).len()
    }

    /// Get the number of outputs to the given IR instruction.
    pub fn num_outputs(&self, ir_inst: Inst) -> usize {
        self.f.dfg.inst_results(ir_inst).len()
    }

    /// Get the type for an instruction's input.
    pub fn input_ty(&self, ir_inst: Inst, idx: usize) -> Type {
        self.value_ty(self.input_as_value(ir_inst, idx))
    }

    /// Get the type for a value.
    pub fn value_ty(&self, val: Value) -> Type {
        self.f.dfg.value_type(val)
    }

    /// Get the type for an instruction's output.
    pub fn output_ty(&self, ir_inst: Inst, idx: usize) -> Type {
        self.f.dfg.value_type(self.f.dfg.inst_results(ir_inst)[idx])
    }

    /// Get the value of a constant instruction (`iconst`, etc.) as a 64-bit
    /// value, if possible.
    pub fn get_constant(&self, ir_inst: Inst) -> Option<u64> {
        self.inst_constants.get(&ir_inst).map(|&c| {
            // The upper bits must be zero, enforced during legalization and by
            // the CLIF verifier.
            debug_assert_eq!(c, {
                let input_size = self.output_ty(ir_inst, 0).bits() as u64;
                let shift = 64 - input_size;
                (c << shift) >> shift
            });
            c
        })
    }

    /// Get the input as one of two options other than a direct register:
    ///
    /// - An instruction, given that it is effect-free or able to sink its
    ///   effect to the current instruction being lowered, and given it has only
    ///   one output, and if effect-ful, given that this is the only use;
    /// - A constant, if the value is a constant.
    ///
    /// The instruction input may be available in either of these forms.  It may
    /// be available in neither form, if the conditions are not met; if so, use
    /// `put_input_in_regs()` instead to get it in a register.
    ///
    /// If the backend merges the effect of a side-effecting instruction, it
    /// must call `sink_inst()`. When this is called, it indicates that the
    /// effect has been sunk to the current scan location. The sunk
    /// instruction's result(s) must have *no* uses remaining, because it will
    /// not be codegen'd (it has been integrated into the current instruction).
    pub fn input_as_value(&self, ir_inst: Inst, idx: usize) -> Value {
        let val = self.f.dfg.inst_args(ir_inst)[idx];
        debug_assert!(self.f.dfg.value_is_real(val));
        val
    }

    /// Resolves a particular input of an instruction to the `Value` that it is
    /// represented with.
    ///
    /// For more information see [`Lower::get_value_as_source_or_const`].
    pub fn get_input_as_source_or_const(&self, ir_inst: Inst, idx: usize) -> NonRegInput {
        let val = self.input_as_value(ir_inst, idx);
        self.get_value_as_source_or_const(val)
    }

    /// Resolves a `Value` definition to the source instruction it came from
    /// plus whether it's a unique-use of that instruction.
    ///
    /// This function is the workhorse of pattern-matching in ISLE which enables
    /// combining multiple instructions together. This is used implicitly in
    /// patterns such as `(iadd x (iconst y))` where this function is used to
    /// extract the `(iconst y)` operand.
    ///
    /// At its core this function is a wrapper around
    /// [`DataFlowGraph::value_def`]. This function applies a filter on top of
    /// that, however, to determine when it is actually safe to "look through"
    /// the `val` definition here and view the underlying instruction. This
    /// protects against duplicating side effects, such as loads, for example.
    ///
    /// Internally this uses the data computed from `compute_use_states` along
    /// with other instruction properties to know what to return.
    pub fn get_value_as_source_or_const(&self, val: Value) -> NonRegInput {
        trace!(
            "get_input_for_val: val {} at cur_inst {:?} cur_scan_entry_color {:?}",
            val, self.cur_inst, self.cur_scan_entry_color,
        );
        let inst = match self.f.dfg.value_def(val) {
            // OK to merge source instruction if we have a source
            // instruction, and one of these two conditions hold:
            //
            // - It has no side-effects and this instruction is not a "value-use
            //   root" instruction. Instructions which are considered "roots"
            //   for value-use calculations do not have accurate information
            //   known about the `ValueUseState` of their operands. This is
            //   currently done for multi-result instructions to prevent a use
            //   of each result from forcing all operands of the multi-result
            //   instruction to also be `Multiple`. This in turn means that the
            //   `ValueUseState` for operands of a "root" instruction to be a
            //   lie if pattern matching were to look through the multi-result
            //   instruction. As a result the "look through this instruction"
            //   logic only succeeds if it's not a root instruction.
            //
            // - It has a side-effect, has one output value, that one
            //   output has only one use, directly or indirectly (so
            //   cannot be duplicated -- see comment on
            //   `ValueUseState`), and the instruction's color is *one
            //   less than* the current scan color.
            //
            //   This latter set of conditions is testing whether a
            //   side-effecting instruction can sink to the current scan
            //   location; this is possible if the in-color of this inst is
            //   equal to the out-color of the producing inst, so no other
            //   side-effecting ops occur between them (which will only be true
            //   if they are in the same BB, because color increments at each BB
            //   start).
            //
            //   If it is actually sunk, then in `merge_inst()`, we update the
            //   scan color so that as we scan over the range past which the
            //   instruction was sunk, we allow other instructions (that came
            //   prior to the sunk instruction) to sink.
            ValueDef::Result(src_inst, result_idx) => {
                let src_side_effect = has_lowering_side_effect(self.f, src_inst);
                trace!(" -> src inst {}", self.f.dfg.display_inst(src_inst));
                trace!(" -> has lowering side effect: {}", src_side_effect);
                if is_value_use_root(self.f, src_inst) {
                    // If this instruction is a "root instruction" then it's
                    // required that we can't look through it to see the
                    // definition. This means that the `ValueUseState` for the
                    // operands of this result assume that this instruction is
                    // generated exactly once which might get violated were we
                    // to allow looking through it.
                    trace!(" -> is a root instruction");
                    InputSourceInst::None
                } else if !src_side_effect {
                    // Otherwise if this instruction has no side effects and the
                    // value is used only once then we can look through it with
                    // a "unique" tag. A non-unique `Use` can be shown for other
                    // values ensuring consumers know how it's computed but that
                    // it's not available to omit.
                    if self.value_ir_uses[val] == ValueUseState::Once {
                        InputSourceInst::UniqueUse(src_inst, result_idx)
                    } else {
                        InputSourceInst::Use(src_inst, result_idx)
                    }
                } else {
                    // Side-effect: test whether this is the only use of the
                    // only result of the instruction, and whether colors allow
                    // the code-motion.
                    trace!(
                        " -> side-effecting op {} for val {}: use state {:?}",
                        src_inst, val, self.value_ir_uses[val]
                    );
                    if self.cur_scan_entry_color.is_some()
                        && self.value_ir_uses[val] == ValueUseState::Once
                        && self.num_outputs(src_inst) == 1
                        && self
                            .side_effect_inst_entry_colors
                            .get(&src_inst)
                            .unwrap()
                            .get()
                            + 1
                            == self.cur_scan_entry_color.unwrap().get()
                    {
                        InputSourceInst::UniqueUse(src_inst, 0)
                    } else {
                        InputSourceInst::None
                    }
                }
            }
            _ => InputSourceInst::None,
        };
        let constant = inst.as_inst().and_then(|(inst, _)| self.get_constant(inst));

        NonRegInput { inst, constant }
    }

    /// Increment the reference count for the Value, ensuring that it gets lowered.
    pub fn increment_lowered_uses(&mut self, val: Value) {
        self.value_lowered_uses[val] += 1
    }

    /// Put the `idx`th input into register(s) and return the assigned register.
    pub fn put_input_in_regs(&mut self, ir_inst: Inst, idx: usize) -> ValueRegs<Reg> {
        let val = self.f.dfg.inst_args(ir_inst)[idx];
        self.put_value_in_regs(val)
    }

    /// Put the given value into register(s) and return the assigned register.
    pub fn put_value_in_regs(&mut self, val: Value) -> ValueRegs<Reg> {
        debug_assert!(self.f.dfg.value_is_real(val));
        trace!("put_value_in_regs: val {}", val);

        if let Some(inst) = self.f.dfg.value_def(val).inst() {
            assert!(!self.inst_sunk.contains(&inst));
        }

        let regs = self.value_regs[val];
        trace!(" -> regs {:?}", regs);
        assert!(regs.is_valid());

        self.value_lowered_uses[val] += 1;

        regs
    }

    /// Get the ValueRegs for the edge-defined values for special
    /// try-call-return block arguments.
    pub fn try_call_return_defs(&mut self, ir_inst: Inst) -> &[ValueRegs<Writable<Reg>>] {
        &self.try_call_rets.get(&ir_inst).unwrap()[..]
    }

    /// Get the Regs for the edge-defined values for special
    /// try-call-return exception payload arguments.
    pub fn try_call_exception_defs(&mut self, ir_inst: Inst) -> &[Writable<Reg>] {
        &self.try_call_payloads.get(&ir_inst).unwrap()[..]
    }
}

/// Codegen primitives: allocate temps, emit instructions, set result registers,
/// ask for an input to be gen'd into a register.
impl<'func, I: VCodeInst> Lower<'func, I> {
    /// Get a new temp.
    pub fn alloc_tmp(&mut self, ty: Type) -> ValueRegs<Writable<Reg>> {
        writable_value_regs(self.vregs.alloc_with_deferred_error(ty))
    }

    /// Get the current root instruction that we are lowering.
    pub fn cur_inst(&self) -> Inst {
        self.cur_inst.unwrap()
    }

    /// Emit a machine instruction.
    pub fn emit(&mut self, mach_inst: I) {
        trace!("emit: {:?}", mach_inst);
        self.ir_insts.push(mach_inst);
    }

    /// Indicate that the side-effect of an instruction has been sunk to the
    /// current scan location. This should only be done with the instruction's
    /// original results are not used (i.e., `put_input_in_regs` is not invoked
    /// for the input produced by the sunk instruction), otherwise the
    /// side-effect will occur twice.
    pub fn sink_inst(&mut self, ir_inst: Inst) {
        assert!(has_lowering_side_effect(self.f, ir_inst));
        assert!(self.cur_scan_entry_color.is_some());

        for result in self.dfg().inst_results(ir_inst) {
            assert!(self.value_lowered_uses[*result] == 0);
        }

        let sunk_inst_entry_color = self
            .side_effect_inst_entry_colors
            .get(&ir_inst)
            .cloned()
            .unwrap();
        let sunk_inst_exit_color = InstColor::new(sunk_inst_entry_color.get() + 1);
        assert!(sunk_inst_exit_color == self.cur_scan_entry_color.unwrap());
        self.cur_scan_entry_color = Some(sunk_inst_entry_color);
        self.inst_sunk.insert(ir_inst);
    }

    /// Retrieve immediate data given a handle.
    pub fn get_immediate_data(&self, imm: Immediate) -> &ConstantData {
        self.f.dfg.immediates.get(imm).unwrap()
    }

    /// Retrieve constant data given a handle.
    pub fn get_constant_data(&self, constant_handle: Constant) -> &ConstantData {
        self.f.dfg.constants.get(constant_handle)
    }

    /// Indicate that a constant should be emitted.
    pub fn use_constant(&mut self, constant: VCodeConstantData) -> VCodeConstant {
        self.vcode.constants().insert(constant)
    }

    /// Cause the value in `reg` to be in a virtual reg, by copying it into a
    /// new virtual reg if `reg` is a real reg. `ty` describes the type of the
    /// value in `reg`.
    pub fn ensure_in_vreg(&mut self, reg: Reg, ty: Type) -> Reg {
        if reg.to_virtual_reg().is_some() {
            reg
        } else {
            let new_reg = self.alloc_tmp(ty).only_reg().unwrap();
            self.emit(I::gen_move(new_reg, reg, ty));
            new_reg.to_reg()
        }
    }

    /// Add a range fact to a register, if no other fact is present.
    pub fn add_range_fact(&mut self, reg: Reg, bit_width: u16, min: u64, max: u64) {
        if self.flags.enable_pcc() {
            self.vregs.set_fact_if_missing(
                reg.to_virtual_reg().unwrap(),
                Fact::Range {
                    bit_width,
                    min,
                    max,
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ValueUseState;
    use crate::cursor::{Cursor, FuncCursor};
    use crate::ir::types;
    use crate::ir::{Function, InstBuilder};

    #[test]
    fn multi_result_use_once() {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        let mut pos = FuncCursor::new(&mut func);
        pos.insert_block(block0);
        let v1 = pos.ins().iconst(types::I64, 0);
        let v2 = pos.ins().iconst(types::I64, 1);
        let v3 = pos.ins().iconcat(v1, v2);
        let (v4, v5) = pos.ins().isplit(v3);
        pos.ins().return_(&[v4, v5]);
        let func = pos.func;

        let uses = super::compute_use_states(&func, None);
        assert_eq!(uses[v1], ValueUseState::Once);
        assert_eq!(uses[v2], ValueUseState::Once);
        assert_eq!(uses[v3], ValueUseState::Once);
        assert_eq!(uses[v4], ValueUseState::Once);
        assert_eq!(uses[v5], ValueUseState::Once);
    }

    #[test]
    fn results_used_twice_but_not_operands() {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        let mut pos = FuncCursor::new(&mut func);
        pos.insert_block(block0);
        let v1 = pos.ins().iconst(types::I64, 0);
        let v2 = pos.ins().iconst(types::I64, 1);
        let v3 = pos.ins().iconcat(v1, v2);
        let (v4, v5) = pos.ins().isplit(v3);
        pos.ins().return_(&[v4, v4]);
        let func = pos.func;

        let uses = super::compute_use_states(&func, None);
        assert_eq!(uses[v1], ValueUseState::Once);
        assert_eq!(uses[v2], ValueUseState::Once);
        assert_eq!(uses[v3], ValueUseState::Once);
        assert_eq!(uses[v4], ValueUseState::Multiple);
        assert_eq!(uses[v5], ValueUseState::Unused);
    }
}
