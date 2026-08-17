/*
 * This file was initially derived from the files
 * `js/src/jit/BacktrackingAllocator.h` and
 * `js/src/jit/BacktrackingAllocator.cpp` in Mozilla Firefox, and was
 * originally licensed under the Mozilla Public License 2.0. We
 * subsequently relicensed it to Apache-2.0 WITH LLVM-exception (see
 * https://github.com/bytecodealliance/regalloc2/issues/7).
 *
 * Since the initial port, the design has been substantially evolved
 * and optimized.
 */

//! Live-range computation.

use super::{
    CodeRange, Env, LiveRangeFlag, LiveRangeIndex, LiveRangeKey, LiveRangeList, LiveRangeListEntry,
    LiveRangeSet, PRegData, PRegIndex, RegClass, Use, VRegData, VRegIndex,
};
use crate::indexset::IndexSet;
use crate::ion::data_structures::{
    BlockparamIn, BlockparamOut, FixedRegFixupLevel, MultiFixedRegFixup,
};
use crate::{
    Allocation, Block, Function, Inst, InstPosition, Operand, OperandConstraint, OperandKind,
    OperandPos, PReg, ProgPoint, RegAllocError, VReg, VecExt,
};
use core::convert::TryFrom;
use smallvec::{smallvec, SmallVec};

/// A spill weight computed for a certain Use.
#[derive(Clone, Copy, Debug)]
pub struct SpillWeight(f32);

#[inline(always)]
pub fn spill_weight_from_constraint(
    constraint: OperandConstraint,
    loop_depth: usize,
    is_def: bool,
) -> SpillWeight {
    // A bonus of 1000 for one loop level, 4000 for two loop levels,
    // 16000 for three loop levels, etc. Avoids exponentiation.
    let loop_depth = core::cmp::min(10, loop_depth);
    let hot_bonus: f32 = (0..loop_depth).fold(1000.0, |a, _| a * 4.0);
    let def_bonus: f32 = if is_def { 2000.0 } else { 0.0 };
    let constraint_bonus: f32 = match constraint {
        OperandConstraint::Any => 1000.0,
        OperandConstraint::Reg | OperandConstraint::FixedReg(_) => 2000.0,
        _ => 0.0,
    };
    SpillWeight(hot_bonus + def_bonus + constraint_bonus)
}

impl SpillWeight {
    /// Convert a floating-point weight to a u16 that can be compactly
    /// stored in a `Use`. We simply take the top 16 bits of the f32; this
    /// is equivalent to the bfloat16 format
    /// (https://en.wikipedia.org/wiki/Bfloat16_floating-point_format).
    pub fn to_bits(self) -> u16 {
        (self.0.to_bits() >> 15) as u16
    }

    /// Convert a value that was returned from
    /// `SpillWeight::to_bits()` back into a `SpillWeight`. Note that
    /// some precision may be lost when round-tripping from a spill
    /// weight to packed bits and back.
    pub fn from_bits(bits: u16) -> SpillWeight {
        let x = f32::from_bits((bits as u32) << 15);
        SpillWeight(x)
    }

    /// Get a zero spill weight.
    pub fn zero() -> SpillWeight {
        SpillWeight(0.0)
    }

    /// Convert to a raw floating-point value.
    pub fn to_f32(self) -> f32 {
        self.0
    }

    /// Create a `SpillWeight` from a raw floating-point value.
    pub fn from_f32(x: f32) -> SpillWeight {
        SpillWeight(x)
    }

    pub fn to_int(self) -> u32 {
        self.0 as u32
    }
}

impl core::ops::Add<SpillWeight> for SpillWeight {
    type Output = SpillWeight;
    fn add(self, other: SpillWeight) -> Self {
        SpillWeight(self.0 + other.0)
    }
}

fn slot_idx(i: usize) -> Result<u16, RegAllocError> {
    u16::try_from(i).map_err(|_| RegAllocError::TooManyOperands)
}

impl<'a, F: Function> Env<'a, F> {
    pub fn create_pregs_and_vregs(&mut self) {
        // Create PRegs from the env.
        self.pregs.resize(
            PReg::NUM_INDEX,
            PRegData {
                allocations: LiveRangeSet::new(),
                is_stack: false,
            },
        );
        for &preg in &self.env.fixed_stack_slots {
            self.pregs[preg.index()].is_stack = true;
        }
        for class in 0..self.preferred_victim_by_class.len() {
            self.preferred_victim_by_class[class] = self.env.non_preferred_regs_by_class[class]
                .last()
                .or(self.env.preferred_regs_by_class[class].last())
                .cloned()
                .unwrap_or(PReg::invalid());
        }
        // Create VRegs from the vreg count.
        for idx in 0..self.func.num_vregs() {
            // We'll fill in the real details when we see the def.
            self.ctx.vregs.add(
                VReg::new(idx, RegClass::Int),
                VRegData {
                    ranges: LiveRangeList::new_in(self.ctx.bump()),
                    blockparam: Block::invalid(),
                    // We'll learn the RegClass as we scan the code.
                    class: None,
                },
            );
        }
        // Create allocations too.
        for inst in 0..self.func.num_insts() {
            let start = self.output.allocs.len() as u32;
            self.output.inst_alloc_offsets.push(start);
            for _ in 0..self.func.inst_operands(Inst::new(inst)).len() {
                self.output.allocs.push(Allocation::none());
            }
        }
    }

    /// Mark `range` as live for the given `vreg`.
    ///
    /// Returns the liverange that contains the given range.
    pub fn add_liverange_to_vreg(
        &mut self,
        vreg: VRegIndex,
        mut range: CodeRange,
    ) -> LiveRangeIndex {
        trace!("add_liverange_to_vreg: vreg {:?} range {:?}", vreg, range);

        // Invariant: as we are building liveness information, we
        // *always* process instructions bottom-to-top, and as a
        // consequence, new liveranges are always created before any
        // existing liveranges for a given vreg. We assert this here,
        // then use it to avoid an O(n) merge step (which would lead
        // to O(n^2) liveness construction cost overall).
        //
        // We store liveranges in reverse order in the `.ranges`
        // array, then reverse them at the end of
        // `compute_liveness()`.

        if !self.vregs[vreg].ranges.is_empty() {
            let last_range_index = self.vregs[vreg].ranges.last().unwrap().index;
            let last_range = self.ranges[last_range_index].range;
            if self.func.allow_multiple_vreg_defs() {
                if last_range.contains(&range) {
                    // Special case (may occur when multiple defs of pinned
                    // physical regs occur): if this new range overlaps the
                    // existing range, return it.
                    return last_range_index;
                }
                // If this range's end falls in the middle of the last
                // range, truncate it to be contiguous so we can merge
                // below.
                if range.to >= last_range.from && range.to <= last_range.to {
                    range.to = last_range.from;
                }
            }
            debug_assert!(
                range.to <= last_range.from,
                "range {:?}, last_range {:?}",
                range,
                last_range
            );
        }

        if self.vregs[vreg].ranges.is_empty()
            || range.to
                < self.ranges[self.vregs[vreg].ranges.last().unwrap().index]
                    .range
                    .from
        {
            // Is not contiguous with previously-added (immediately
            // following) range; create a new range.
            let lr = self.ctx.ranges.add(range, self.ctx.bump());
            self.ranges[lr].vreg = vreg;
            self.vregs[vreg]
                .ranges
                .push(LiveRangeListEntry { range, index: lr });
            lr
        } else {
            // Is contiguous with previously-added range; just extend
            // its range and return it.
            let lr = self.vregs[vreg].ranges.last().unwrap().index;
            debug_assert!(range.to == self.ranges[lr].range.from);
            self.ranges[lr].range.from = range.from;
            lr
        }
    }

    pub fn insert_use_into_liverange(&mut self, into: LiveRangeIndex, mut u: Use) {
        let operand = u.operand;
        let constraint = operand.constraint();
        let block = self.cfginfo.insn_block[u.pos.inst().index()];
        let loop_depth = self.cfginfo.approx_loop_depth[block.index()] as usize;
        let weight = spill_weight_from_constraint(
            constraint,
            loop_depth,
            operand.kind() != OperandKind::Use,
        );
        u.weight = weight.to_bits();

        trace!(
            "insert use {:?} into lr {:?} with weight {:?}",
            u,
            into,
            weight,
        );

        // N.B.: we do *not* update `requirement` on the range,
        // because those will be computed during the multi-fixed-reg
        // fixup pass later (after all uses are inserted).

        self.ranges[into].uses.push(u);

        // Update stats.
        let range_weight = self.ranges[into].uses_spill_weight() + weight;
        self.ranges[into].set_uses_spill_weight(range_weight);
        trace!(
            "  -> now range has weight {:?}",
            self.ranges[into].uses_spill_weight(),
        );
    }

    pub fn find_vreg_liverange_for_pos(
        &self,
        vreg: VRegIndex,
        pos: ProgPoint,
    ) -> Option<LiveRangeIndex> {
        for entry in &self.vregs[vreg].ranges {
            if entry.range.contains_point(pos) {
                return Some(entry.index);
            }
        }
        None
    }

    pub fn add_liverange_to_preg(&mut self, range: CodeRange, reg: PReg) {
        trace!("adding liverange to preg: {:?} to {}", range, reg);
        let preg_idx = PRegIndex::new(reg.index());
        let res = self.pregs[preg_idx.index()]
            .allocations
            .btree
            .insert(LiveRangeKey::from_range(&range), LiveRangeIndex::invalid());
        debug_assert!(res.is_none());
    }

    pub fn is_live_in(&mut self, block: Block, vreg: VRegIndex) -> bool {
        self.liveins[block.index()].get(vreg.index())
    }

    pub fn compute_liveness(&mut self) -> Result<(), RegAllocError> {
        // Create initial LiveIn and LiveOut bitsets.
        for _ in 0..self.func.num_blocks() {
            self.liveins.push(IndexSet::new());
            self.liveouts.push(IndexSet::new());
        }

        // Run a worklist algorithm to precisely compute liveins and
        // liveouts.
        let mut workqueue = core::mem::take(&mut self.ctx.scratch_workqueue);
        let mut workqueue_set = core::mem::take(&mut self.ctx.scratch_workqueue_set);
        workqueue_set.clear();
        // Initialize workqueue with postorder traversal.
        for &block in &self.cfginfo.postorder[..] {
            workqueue.push_back(block);
            workqueue_set.insert(block);
        }

        while let Some(block) = workqueue.pop_front() {
            workqueue_set.remove(&block);
            let insns = self.func.block_insns(block);

            trace!("computing liveins for block{}", block.index());

            self.output.stats.livein_iterations += 1;

            let mut live = self.liveouts[block.index()].clone();
            trace!(" -> initial liveout set: {:?}", live);

            // Include outgoing blockparams in the initial live set.
            if self.func.is_branch(insns.last()) {
                for i in 0..self.func.block_succs(block).len() {
                    for &param in self.func.branch_blockparams(block, insns.last(), i) {
                        live.set(param.vreg(), true);
                        self.observe_vreg_class(param);
                    }
                }
            }

            for inst in insns.iter().rev() {
                for pos in &[OperandPos::Late, OperandPos::Early] {
                    for op in self.func.inst_operands(inst) {
                        if op.as_fixed_nonallocatable().is_some() {
                            continue;
                        }
                        if op.pos() == *pos {
                            let was_live = live.get(op.vreg().vreg());
                            trace!("op {:?} was_live = {}", op, was_live);
                            match op.kind() {
                                OperandKind::Use => {
                                    live.set(op.vreg().vreg(), true);
                                }
                                OperandKind::Def => {
                                    live.set(op.vreg().vreg(), false);
                                }
                            }
                            self.observe_vreg_class(op.vreg());
                        }
                    }
                }
            }
            for &blockparam in self.func.block_params(block) {
                live.set(blockparam.vreg(), false);
                self.observe_vreg_class(blockparam);
            }

            for &pred in self.func.block_preds(block) {
                if self.ctx.liveouts[pred.index()].union_with(&live) {
                    if !workqueue_set.contains(&pred) {
                        workqueue_set.insert(pred);
                        workqueue.push_back(pred);
                    }
                }
            }

            trace!("computed liveins at block{}: {:?}", block.index(), live);
            self.liveins[block.index()] = live;
        }

        // Check that there are no liveins to the entry block.
        if !self.liveins[self.func.entry_block().index()].is_empty() {
            trace!(
                "non-empty liveins to entry block: {:?}",
                self.liveins[self.func.entry_block().index()]
            );
            return Err(RegAllocError::EntryLivein);
        }

        self.ctx.scratch_workqueue = workqueue;
        self.ctx.scratch_workqueue_set = workqueue_set;

        Ok(())
    }

    pub fn build_liveranges(&mut self) -> Result<(), RegAllocError> {
        // Create Uses and Defs referring to VRegs, and place the Uses
        // in LiveRanges.
        //
        // We already computed precise liveouts and liveins for every
        // block above, so we don't need to run an iterative algorithm
        // here; instead, every block's computation is purely local,
        // from end to start.

        // Track current LiveRange for each vreg.
        //
        // Invariant: a stale range may be present here; ranges are
        // only valid if `live.get(vreg)` is true.
        let mut vreg_ranges = core::mem::take(&mut self.ctx.scratch_vreg_ranges);
        vreg_ranges.repopulate(self.func.num_vregs(), LiveRangeIndex::invalid());
        let mut operand_rewrites = core::mem::take(&mut self.ctx.scratch_operand_rewrites);

        for i in (0..self.func.num_blocks()).rev() {
            let block = Block::new(i);
            let insns = self.func.block_insns(block);

            self.output.stats.livein_blocks += 1;

            // Init our local live-in set.
            let mut live = self.liveouts[block.index()].clone();

            // If the last instruction is a branch (rather than
            // return), create blockparam_out entries.
            if self.func.is_branch(insns.last()) {
                for (i, &succ) in self.func.block_succs(block).iter().enumerate() {
                    let blockparams_in = self.func.block_params(succ);
                    let blockparams_out = self.func.branch_blockparams(block, insns.last(), i);
                    for (&blockparam_in, &blockparam_out) in
                        blockparams_in.iter().zip(blockparams_out)
                    {
                        let blockparam_out = VRegIndex::new(blockparam_out.vreg());
                        let blockparam_in = VRegIndex::new(blockparam_in.vreg());
                        self.blockparam_outs.push(BlockparamOut {
                            to_vreg: blockparam_in,
                            to_block: succ,
                            from_block: block,
                            from_vreg: blockparam_out,
                        });

                        // Include outgoing blockparams in the initial live set.
                        live.set(blockparam_out.index(), true);
                    }
                }
            }

            // Initially, registers are assumed live for the whole block.
            for vreg in live.iter() {
                let range = CodeRange {
                    from: self.cfginfo.block_entry[block.index()],
                    to: self.cfginfo.block_exit[block.index()].next(),
                };
                trace!(
                    "vreg {:?} live at end of block --> create range {:?}",
                    VRegIndex::new(vreg),
                    range
                );
                let lr = self.add_liverange_to_vreg(VRegIndex::new(vreg), range);
                vreg_ranges[vreg] = lr;
            }

            // Create vreg data for blockparams.
            for &param in self.func.block_params(block) {
                self.vregs[param].blockparam = block;
            }

            // For each instruction, in reverse order, process
            // operands and clobbers.
            for inst in insns.iter().rev() {
                // Mark clobbers with CodeRanges on PRegs.
                for clobber in self.func.inst_clobbers(inst) {
                    // Clobber range is at After point only: an
                    // instruction can still take an input in a reg
                    // that it later clobbers. (In other words, the
                    // clobber is like a normal def that never gets
                    // used.)
                    let range = CodeRange {
                        from: ProgPoint::after(inst),
                        to: ProgPoint::before(inst.next()),
                    };
                    self.add_liverange_to_preg(range, clobber);
                }

                // Does the instruction have any input-reusing
                // outputs? This is important below to establish
                // proper interference wrt other inputs. We note the
                // *vreg* that is reused, not the index.
                let mut reused_input = None;
                for op in self.func.inst_operands(inst) {
                    if let OperandConstraint::Reuse(i) = op.constraint() {
                        debug_assert!(self.func.inst_operands(inst)[i]
                            .as_fixed_nonallocatable()
                            .is_none());
                        reused_input = Some(self.func.inst_operands(inst)[i].vreg());
                        break;
                    }
                }

                // Preprocess defs and uses. Specifically, if there
                // are any fixed-reg-constrained defs at Late position
                // and fixed-reg-constrained uses at Early position
                // with the same preg, we need to (i) add a fixup move
                // for the use, (ii) rewrite the use to have an Any
                // constraint, and (ii) move the def to Early position
                // to reserve the register for the whole instruction.
                //
                // We don't touch any fixed-early-def or fixed-late-use
                // constraints: the only situation where the same physical
                // register can be used multiple times in the same
                // instruction is with an early-use and a late-def. Anything
                // else is a user error.
                operand_rewrites.clear();
                let mut late_def_fixed: SmallVec<[PReg; 8]> = smallvec![];
                for &operand in self.func.inst_operands(inst) {
                    if let OperandConstraint::FixedReg(preg) = operand.constraint() {
                        match (operand.pos(), operand.kind()) {
                            (OperandPos::Late, OperandKind::Def) => {
                                late_def_fixed.push(preg);
                            }
                            _ => {}
                        }
                    }
                }
                for (i, &operand) in self.func.inst_operands(inst).iter().enumerate() {
                    if operand.as_fixed_nonallocatable().is_some() {
                        continue;
                    }
                    if let OperandConstraint::FixedReg(preg) = operand.constraint() {
                        match (operand.pos(), operand.kind()) {
                            (OperandPos::Early, OperandKind::Use)
                                if live.get(operand.vreg().vreg()) =>
                            {
                                // If we have a use constraint at the
                                // Early point for a fixed preg, and
                                // this preg is also constrained with
                                // a *separate* def at Late or is
                                // clobbered, and *if* the vreg is
                                // live downward, we have to use the
                                // multi-fixed-reg mechanism for a
                                // fixup and rewrite here without the
                                // constraint. See #53.
                                //
                                // We adjust the def liverange and Use
                                // to an "early" position to reserve
                                // the register, it still must not be
                                // used by some other vreg at the
                                // use-site.
                                //
                                // Note that we handle multiple
                                // conflicting constraints for the
                                // same vreg in a separate pass (see
                                // `fixup_multi_fixed_vregs` below).
                                if late_def_fixed.contains(&preg)
                                    || self.func.inst_clobbers(inst).contains(preg)
                                {
                                    trace!(
                                        concat!(
                                            "-> operand {:?} is fixed to preg {:?}, ",
                                            "is downward live, and there is also a ",
                                            "def or clobber at this preg"
                                        ),
                                        operand,
                                        preg
                                    );
                                    let pos = ProgPoint::before(inst);
                                    self.multi_fixed_reg_fixups.push(MultiFixedRegFixup {
                                        pos,
                                        from_slot: slot_idx(i)?,
                                        to_slot: slot_idx(i)?,
                                        to_preg: PRegIndex::new(preg.index()),
                                        vreg: VRegIndex::new(operand.vreg().vreg()),
                                        level: FixedRegFixupLevel::Initial,
                                    });

                                    // We need to insert a reservation
                                    // at the before-point to reserve
                                    // the reg for the use too.
                                    let range = CodeRange::singleton(pos);
                                    self.add_liverange_to_preg(range, preg);

                                    // Remove the fixed-preg
                                    // constraint from the Use.
                                    operand_rewrites.insert(
                                        i,
                                        Operand::new(
                                            operand.vreg(),
                                            OperandConstraint::Any,
                                            operand.kind(),
                                            operand.pos(),
                                        ),
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                }

                // Process defs and uses.
                for &cur_pos in &[InstPosition::After, InstPosition::Before] {
                    for i in 0..self.func.inst_operands(inst).len() {
                        // don't borrow `self`
                        let operand = operand_rewrites
                            .get(&i)
                            .cloned()
                            .unwrap_or(self.func.inst_operands(inst)[i]);
                        let pos = match (operand.kind(), operand.pos()) {
                            (OperandKind::Def, OperandPos::Early) => ProgPoint::before(inst),
                            (OperandKind::Def, OperandPos::Late) => ProgPoint::after(inst),
                            (OperandKind::Use, OperandPos::Late) => ProgPoint::after(inst),
                            // If there are any reused inputs in this
                            // instruction, and this is *not* the
                            // reused vreg, force `pos` to
                            // `After`. This ensures that we correctly
                            // account for the interference between
                            // the other inputs and the
                            // input-that-is-reused/output.
                            (OperandKind::Use, OperandPos::Early)
                                if reused_input.is_some()
                                    && reused_input.unwrap() != operand.vreg() =>
                            {
                                ProgPoint::after(inst)
                            }
                            (OperandKind::Use, OperandPos::Early) => ProgPoint::before(inst),
                        };

                        if pos.pos() != cur_pos {
                            continue;
                        }

                        trace!(
                            "processing inst{} operand at {:?}: {:?}",
                            inst.index(),
                            pos,
                            operand
                        );

                        // If this is a "fixed non-allocatable
                        // register" operand, set the alloc
                        // immediately and then ignore the operand
                        // hereafter.
                        if let Some(preg) = operand.as_fixed_nonallocatable() {
                            self.set_alloc(inst, i, Allocation::reg(preg));
                            continue;
                        }

                        match operand.kind() {
                            OperandKind::Def => {
                                trace!("Def of {} at {:?}", operand.vreg(), pos);

                                // Get or create the LiveRange.
                                let mut lr = vreg_ranges[operand.vreg().vreg()];
                                trace!(" -> has existing LR {:?}", lr);
                                // If there was no liverange (dead def), create a trivial one.
                                if !live.get(operand.vreg().vreg()) {
                                    let from = pos;
                                    // We want to we want to span
                                    // until Before of the next
                                    // inst. This ensures that early
                                    // defs used for temps on an
                                    // instruction are reserved across
                                    // the whole instruction.
                                    let to = ProgPoint::before(pos.inst().next());
                                    lr = self.add_liverange_to_vreg(
                                        VRegIndex::new(operand.vreg().vreg()),
                                        CodeRange { from, to },
                                    );
                                    trace!(" -> invalid; created {:?}", lr);
                                    vreg_ranges[operand.vreg().vreg()] = lr;
                                    live.set(operand.vreg().vreg(), true);
                                }
                                // Create the use in the LiveRange.
                                self.insert_use_into_liverange(
                                    lr,
                                    Use::new(operand, pos, slot_idx(i)?),
                                );
                                // If def (not mod), this reg is now dead,
                                // scanning backward; make it so.
                                if operand.kind() == OperandKind::Def {
                                    // Trim the range for this vreg to start
                                    // at `pos` if it previously ended at the
                                    // start of this block (i.e. was not
                                    // merged into some larger LiveRange due
                                    // to out-of-order blocks).
                                    if self.ranges[lr].range.from
                                        == self.cfginfo.block_entry[block.index()]
                                    {
                                        trace!(" -> started at block start; trimming to {:?}", pos);
                                        self.ranges[lr].range.from = pos;
                                    }

                                    self.ranges[lr].set_flag(LiveRangeFlag::StartsAtDef);

                                    // Remove from live-set.
                                    live.set(operand.vreg().vreg(), false);
                                    vreg_ranges[operand.vreg().vreg()] = LiveRangeIndex::invalid();
                                }
                            }
                            OperandKind::Use => {
                                // Create/extend the LiveRange if it
                                // doesn't already exist, and add the use
                                // to the range.
                                let mut lr = vreg_ranges[operand.vreg().vreg()];
                                if !live.get(operand.vreg().vreg()) {
                                    let range = CodeRange {
                                        from: self.cfginfo.block_entry[block.index()],
                                        to: pos.next(),
                                    };
                                    lr = self.add_liverange_to_vreg(
                                        VRegIndex::new(operand.vreg().vreg()),
                                        range,
                                    );
                                    vreg_ranges[operand.vreg().vreg()] = lr;
                                }
                                debug_assert!(lr.is_valid());

                                trace!("Use of {:?} at {:?} -> {:?}", operand, pos, lr,);

                                self.insert_use_into_liverange(
                                    lr,
                                    Use::new(operand, pos, slot_idx(i)?),
                                );

                                // Add to live-set.
                                live.set(operand.vreg().vreg(), true);
                            }
                        }
                    }
                }
            }

            // Block parameters define vregs at the very beginning of
            // the block. Remove their live vregs from the live set
            // here.
            for vreg in self.func.block_params(block) {
                if live.get(vreg.vreg()) {
                    live.set(vreg.vreg(), false);
                } else {
                    // Create trivial liverange if blockparam is dead.
                    let start = self.cfginfo.block_entry[block.index()];
                    self.add_liverange_to_vreg(
                        VRegIndex::new(vreg.vreg()),
                        CodeRange {
                            from: start,
                            to: start.next(),
                        },
                    );
                }
                // add `blockparam_ins` entries.
                let vreg_idx = VRegIndex::new(vreg.vreg());
                for &pred in self.func.block_preds(block) {
                    self.blockparam_ins.push(BlockparamIn {
                        to_vreg: vreg_idx,
                        to_block: block,
                        from_block: pred,
                    });
                }
            }
        }

        // Make ranges in each vreg and uses in each range appear in
        // sorted order. We built them in reverse order above, so this
        // is a simple reversal, *not* a full sort.
        //
        // The ordering invariant is always maintained for uses and
        // always for ranges in bundles (which are initialized later),
        // but not always for ranges in vregs; those are sorted only
        // when needed, here and then again at the end of allocation
        // when resolving moves.

        for vreg in &mut self.ctx.vregs {
            vreg.ranges.reverse();
            let mut last = None;
            for entry in &mut vreg.ranges {
                // Ranges may have been truncated above at defs. We
                // need to update with the final range here.
                entry.range = self.ctx.ranges[entry.index].range;
                // Assert in-order and non-overlapping.
                debug_assert!(last.is_none() || last.unwrap() <= entry.range.from);
                last = Some(entry.range.to);
            }
        }

        for range in &mut self.ranges {
            range.uses.reverse();
            debug_assert!(range.uses.windows(2).all(|win| win[0].pos <= win[1].pos));
        }

        self.blockparam_ins.sort_unstable_by_key(|x| x.key());
        self.blockparam_outs.sort_unstable_by_key(|x| x.key());

        self.output.stats.initial_liverange_count = self.ranges.len();
        self.output.stats.blockparam_ins_count = self.blockparam_ins.len();
        self.output.stats.blockparam_outs_count = self.blockparam_outs.len();
        self.ctx.scratch_vreg_ranges = vreg_ranges;
        self.ctx.scratch_operand_rewrites = operand_rewrites;

        Ok(())
    }

    pub fn fixup_multi_fixed_vregs(&mut self) {
        // Do a fixed-reg cleanup pass: if there are any LiveRanges with
        // multiple uses at the same ProgPoint and there is
        // more than one FixedReg constraint at that ProgPoint, we
        // need to record all but one of them in a special fixup list
        // and handle them later; otherwise, bundle-splitting to
        // create minimal bundles becomes much more complex (we would
        // have to split the multiple uses at the same progpoint into
        // different bundles, which breaks invariants related to
        // disjoint ranges and bundles).
        let mut extra_clobbers: SmallVec<[(PReg, ProgPoint); 8]> = smallvec![];
        for vreg in 0..self.vregs.len() {
            let vreg = VRegIndex::new(vreg);
            for range_idx in 0..self.vregs[vreg].ranges.len() {
                let entry = self.vregs[vreg].ranges[range_idx];
                let range = entry.index;
                trace!("multi-fixed-reg cleanup: vreg {:?} range {:?}", vreg, range,);

                // Find groups of uses that occur in at the same program point.
                for uses in self.ctx.ranges[range]
                    .uses
                    .chunk_by_mut(|a, b| a.pos == b.pos)
                {
                    if uses.len() < 2 {
                        continue;
                    }

                    // Search for conflicting constraints in the uses.
                    let mut requires_reg = false;
                    let mut num_fixed_reg = 0;
                    let mut num_fixed_stack = 0;
                    let mut first_reg_slot = None;
                    let mut first_stack_slot = None;
                    for u in uses.iter() {
                        match u.operand.constraint() {
                            OperandConstraint::Any => {
                                first_reg_slot.get_or_insert(u.slot);
                                first_stack_slot.get_or_insert(u.slot);
                            }
                            OperandConstraint::Reg | OperandConstraint::Reuse(_) => {
                                first_reg_slot.get_or_insert(u.slot);
                                requires_reg = true;
                            }
                            OperandConstraint::FixedReg(preg) => {
                                if self.ctx.pregs[preg.index()].is_stack {
                                    num_fixed_stack += 1;
                                    first_stack_slot.get_or_insert(u.slot);
                                } else {
                                    requires_reg = true;
                                    num_fixed_reg += 1;
                                    first_reg_slot.get_or_insert(u.slot);
                                }
                            }
                        }
                    }

                    // Fast path if there are no conflicts.
                    if num_fixed_reg + num_fixed_stack <= 1
                        && !(requires_reg && num_fixed_stack != 0)
                    {
                        continue;
                    }

                    // We pick one constraint (in order: FixedReg, Reg, FixedStack)
                    // and then rewrite any incompatible constraints to Any.
                    // This allows register allocation to succeed and we will
                    // later insert moves to satisfy the rewritten constraints.
                    let source_slot = if requires_reg {
                        first_reg_slot.unwrap()
                    } else {
                        first_stack_slot.unwrap()
                    };
                    let mut first_preg = None;
                    for u in uses.iter_mut() {
                        if let OperandConstraint::FixedReg(preg) = u.operand.constraint() {
                            let vreg_idx = VRegIndex::new(u.operand.vreg().vreg());
                            let preg_idx = PRegIndex::new(preg.index());
                            trace!(
                                "at pos {:?}, vreg {:?} has fixed constraint to preg {:?}",
                                u.pos,
                                vreg_idx,
                                preg_idx
                            );

                            // FixedStack is incompatible if there are any
                            // Reg/FixedReg constraints. FixedReg is
                            // incompatible if there already is a different
                            // FixedReg constraint. If either condition is true,
                            // we edit the constraint below; otherwise, we can
                            // skip this edit.
                            if !(requires_reg && self.ctx.pregs[preg.index()].is_stack)
                                && *first_preg.get_or_insert(preg) == preg
                            {
                                continue;
                            }

                            trace!(" -> duplicate; switching to constraint Any");
                            self.ctx.multi_fixed_reg_fixups.push(MultiFixedRegFixup {
                                pos: u.pos,
                                from_slot: source_slot,
                                to_slot: u.slot,
                                to_preg: preg_idx,
                                vreg: vreg_idx,
                                level: FixedRegFixupLevel::Secondary,
                            });
                            u.operand = Operand::new(
                                u.operand.vreg(),
                                OperandConstraint::Any,
                                u.operand.kind(),
                                u.operand.pos(),
                            );
                            trace!(" -> extra clobber {} at inst{}", preg, u.pos.inst().index());
                            extra_clobbers.push((preg, u.pos));
                        }
                    }
                }

                for (clobber, pos) in extra_clobbers.drain(..) {
                    let range = CodeRange {
                        from: pos,
                        to: pos.next(),
                    };
                    self.add_liverange_to_preg(range, clobber);
                }
            }
        }
    }
}
