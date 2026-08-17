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

//! Move resolution.

use alloc::vec;

use super::{
    Env, InsertMovePrio, InsertedMove, InsertedMoves, LiveRangeFlag, LiveRangeIndex,
    RedundantMoveEliminator, VRegIndex,
};
use crate::ion::data_structures::{
    u64_key, BlockparamIn, BlockparamOut, CodeRange, Edits, FixedRegFixupLevel, LiveRangeKey,
    LiveRangeListEntry,
};
use crate::ion::reg_traversal::RegTraversalIter;
use crate::moves::{MoveAndScratchResolver, ParallelMoves};
use crate::{
    Allocation, Block, Edit, Function, FxHashMap, Inst, InstPosition, OperandConstraint,
    OperandKind, OperandPos, PReg, ProgPoint, RegClass, SpillSlot,
};
use alloc::format;
use alloc::vec::Vec;
use hashbrown::hash_map::Entry;
use smallvec::{smallvec, SmallVec};

impl<'a, F: Function> Env<'a, F> {
    pub fn is_start_of_block(&self, pos: ProgPoint) -> bool {
        let block = self.ctx.cfginfo.insn_block[pos.inst().index()];
        pos == self.ctx.cfginfo.block_entry[block.index()]
    }
    pub fn is_end_of_block(&self, pos: ProgPoint) -> bool {
        let block = self.ctx.cfginfo.insn_block[pos.inst().index()];
        pos == self.ctx.cfginfo.block_exit[block.index()]
    }

    pub fn get_alloc(&self, inst: Inst, slot: usize) -> Allocation {
        let inst_allocs =
            &self.ctx.output.allocs[self.ctx.output.inst_alloc_offsets[inst.index()] as usize..];
        inst_allocs[slot]
    }

    pub fn set_alloc(&mut self, inst: Inst, slot: usize, alloc: Allocation) {
        let inst_allocs = &mut self.ctx.output.allocs
            [self.ctx.output.inst_alloc_offsets[inst.index()] as usize..];
        inst_allocs[slot] = alloc;
    }

    pub fn get_alloc_for_range(&self, range: LiveRangeIndex) -> Allocation {
        trace!("get_alloc_for_range: {:?}", range);
        let bundle = self.ctx.ranges[range].bundle;
        trace!(" -> bundle: {:?}", bundle);
        let bundledata = &self.ctx.bundles[bundle];
        trace!(" -> allocation {:?}", bundledata.allocation);
        if bundledata.allocation != Allocation::none() {
            bundledata.allocation
        } else {
            trace!(" -> spillset {:?}", bundledata.spillset);
            trace!(
                " -> spill slot {:?}",
                self.ctx.spillsets[bundledata.spillset].slot
            );
            self.ctx.spillslots[self.ctx.spillsets[bundledata.spillset].slot.index()].alloc
        }
    }

    pub fn apply_allocations_and_insert_moves(&mut self) -> InsertedMoves {
        trace!("apply_allocations_and_insert_moves");
        trace!("blockparam_ins: {:?}", self.blockparam_ins);
        trace!("blockparam_outs: {:?}", self.blockparam_outs);

        let mut inserted_moves = InsertedMoves::default();

        // Now that all splits are done, we can pay the cost once to
        // sort VReg range lists and update with the final ranges.
        for vreg in &mut self.ctx.vregs {
            for entry in &mut vreg.ranges {
                entry.range = self.ctx.ranges[entry.index].range;
            }
            vreg.ranges.sort_unstable_by_key(|entry| entry.range.from);
        }

        /// Buffered information about the previous liverange that was processed.
        struct PrevBuffer {
            prev: Option<LiveRangeListEntry>,
            prev_ins_idx: usize,
            buffered: Option<LiveRangeListEntry>,
            buffered_ins_idx: usize,
        }

        impl PrevBuffer {
            fn new(prev_ins_idx: usize) -> Self {
                Self {
                    prev: None,
                    prev_ins_idx,
                    buffered: None,
                    buffered_ins_idx: prev_ins_idx,
                }
            }

            /// Returns the previous `LiveRangeListEntry` when it's present.
            #[inline(always)]
            fn is_valid(&self) -> Option<LiveRangeListEntry> {
                self.prev
            }

            /// Fetch the current index into the `Env::blockparam_ins` vector.
            #[inline(always)]
            fn blockparam_ins_idx(&self) -> usize {
                self.prev_ins_idx
            }

            /// Record this index as the next index to use when the previous liverange buffer
            /// anvances.
            #[inline(always)]
            fn update_blockparam_ins_idx(&mut self, idx: usize) {
                self.buffered_ins_idx = idx;
            }

            /// As overlapping liveranges might start at the same program point, we buffer the
            /// previous liverange used when determining where to take the last value from for
            /// intra-block moves. The liveranges we process are buffered until we encounter one
            /// that starts at a later program point, indicating that it's now safe to advance the
            /// previous LR buffer. We accumulate the longest-lived liverange in the buffer as a
            /// heuristic for finding the most stable source of a value.
            ///
            /// We also buffer the index into the `Env::blockparam_ins` vector, as we may see
            /// multiple uses of a blockparam within a single instruction, and as such may need to
            /// generate multiple blockparam move destinations by re-traversing that section of the
            /// vector.
            #[inline(always)]
            fn advance(&mut self, current: LiveRangeListEntry) {
                // Advance the `prev` pointer to the `next` pointer, as long as the `next` pointer
                // does not start at the same time as the current LR we're processing.
                if self
                    .buffered
                    .map(|entry| entry.range.from < current.range.from)
                    .unwrap_or(false)
                {
                    self.prev = self.buffered;
                    self.prev_ins_idx = self.buffered_ins_idx;
                }

                // Advance the `next` pointer to the currently processed LR, as long as it ends
                // later than the current `next`.
                if self
                    .buffered
                    .map(|entry| entry.range.to < current.range.to)
                    .unwrap_or(true)
                {
                    self.buffered = Some(current);
                }
            }
        }

        // Determine the ProgPoint where moves on this (from, to)
        // edge should go:
        // - If there is more than one in-edge to `to`, then
        //   `from` must have only one out-edge; moves go at tail of
        //   `from` just before last Branch/Ret.
        // - Otherwise, there must be at most one in-edge to `to`,
        //   and moves go at start of `to`.
        #[inline(always)]
        fn choose_move_location<'a, F: Function>(
            env: &Env<'a, F>,
            from: Block,
            to: Block,
        ) -> (ProgPoint, InsertMovePrio) {
            let from_last_insn = env.func.block_insns(from).last();
            let to_first_insn = env.func.block_insns(to).first();
            let from_is_ret = env.func.is_ret(from_last_insn);
            let to_is_entry = env.func.entry_block() == to;
            let from_outs = env.func.block_succs(from).len() + if from_is_ret { 1 } else { 0 };
            let to_ins = env.func.block_preds(to).len() + if to_is_entry { 1 } else { 0 };

            if to_ins > 1 && from_outs <= 1 {
                (
                    // N.B.: though semantically the edge moves happen
                    // after the branch, we must insert them before
                    // the branch because otherwise, of course, they
                    // would never execute. This is correct even in
                    // the presence of branches that read register
                    // inputs (e.g. conditional branches on some RISCs
                    // that branch on reg zero/not-zero, or any
                    // indirect branch), but for a very subtle reason:
                    // all cases of such branches will (or should)
                    // have multiple successors, and thus due to
                    // critical-edge splitting, their successors will
                    // have only the single predecessor, and we prefer
                    // to insert at the head of the successor in that
                    // case (rather than here). We make this a
                    // requirement, in fact: the user of this library
                    // shall not read registers in a branch
                    // instruction of there is only one successor per
                    // the given CFG information.
                    ProgPoint::before(from_last_insn),
                    InsertMovePrio::OutEdgeMoves,
                )
            } else if to_ins <= 1 {
                (
                    ProgPoint::before(to_first_insn),
                    InsertMovePrio::InEdgeMoves,
                )
            } else {
                panic!(
                    "Critical edge: can't insert moves between blocks {:?} and {:?}",
                    from, to
                );
            }
        }

        #[derive(PartialEq)]
        struct InterBlockDest {
            to: Block,
            from: Block,
            alloc: Allocation,
        }

        impl InterBlockDest {
            fn key(&self) -> u64 {
                u64_key(self.from.raw_u32(), self.to.raw_u32())
            }
        }

        let mut inter_block_sources: FxHashMap<Block, Allocation> = FxHashMap::default();
        let mut inter_block_dests = Vec::with_capacity(self.func.num_blocks());

        #[derive(Hash, Eq, PartialEq)]
        struct BlockparamSourceKey {
            bits: u64,
        }

        impl BlockparamSourceKey {
            fn new(from_block: Block, to_vreg: VRegIndex) -> Self {
                BlockparamSourceKey {
                    bits: u64_key(from_block.raw_u32(), to_vreg.raw_u32()),
                }
            }
        }

        struct BlockparamDest {
            from_block: Block,
            to_block: Block,
            to_vreg: VRegIndex,
            alloc: Allocation,
        }

        impl BlockparamDest {
            fn key(&self) -> u64 {
                u64_key(self.to_block.raw_u32(), self.from_block.raw_u32())
            }

            fn source(&self) -> BlockparamSourceKey {
                BlockparamSourceKey::new(self.from_block, self.to_vreg)
            }
        }

        let mut block_param_sources =
            FxHashMap::<BlockparamSourceKey, Allocation>::with_capacity_and_hasher(
                3 * self.func.num_insts(),
                Default::default(),
            );
        let mut block_param_dests = Vec::with_capacity(3 * self.func.num_insts());

        let debug_labels = self.func.debug_value_labels();

        let mut reuse_input_insts = Vec::with_capacity(self.func.num_insts() / 2);

        let mut blockparam_in_idx = 0;
        let mut blockparam_out_idx = 0;
        for vreg in 0..self.vregs.len() {
            let vreg = VRegIndex::new(vreg);
            if !self.is_vreg_used(vreg) {
                continue;
            }

            inter_block_sources.clear();

            // For each range in each vreg, insert moves or
            // half-moves.  We also scan over `blockparam_ins` and
            // `blockparam_outs`, which are sorted by (block, vreg),
            // to fill in allocations.
            let mut prev = PrevBuffer::new(blockparam_in_idx);
            for range_idx in 0..self.vregs[vreg].ranges.len() {
                let entry = self.vregs[vreg].ranges[range_idx];
                let alloc = self.get_alloc_for_range(entry.index);
                let range = entry.range;
                trace!(
                    "apply_allocations: vreg {:?} LR {:?} with range {:?} has alloc {:?}",
                    vreg,
                    entry.index,
                    range,
                    alloc,
                );
                debug_assert!(alloc != Allocation::none());

                if self.annotations_enabled {
                    self.annotate(
                        range.from,
                        format!(
                            " <<< start v{} in {} (range{}) (bundle{})",
                            vreg.index(),
                            alloc,
                            entry.index.index(),
                            self.ranges[entry.index].bundle.raw_u32(),
                        ),
                    );
                    self.annotate(
                        range.to,
                        format!(
                            "     end   v{} in {} (range{}) (bundle{}) >>>",
                            vreg.index(),
                            alloc,
                            entry.index.index(),
                            self.ranges[entry.index].bundle.raw_u32(),
                        ),
                    );
                }

                prev.advance(entry);

                // Does this range follow immediately after a prior
                // range in the same block? If so, insert a move (if
                // the allocs differ). We do this directly rather than
                // with half-moves because we eagerly know both sides
                // already (and also, half-moves are specific to
                // inter-block transfers).
                //
                // Note that we do *not* do this if there is also a
                // def as the first use in the new range: it's
                // possible that an old liverange covers the Before
                // pos of an inst, a new liverange covers the After
                // pos, and the def also happens at After. In this
                // case we don't want to an insert a move after the
                // instruction copying the old liverange.
                //
                // Note also that we assert that the new range has to
                // start at the Before-point of an instruction; we
                // can't insert a move that logically happens just
                // before After (i.e. in the middle of a single
                // instruction).
                if let Some(prev) = prev.is_valid() {
                    let prev_alloc = self.get_alloc_for_range(prev.index);
                    debug_assert!(prev_alloc != Allocation::none());

                    if prev.range.to >= range.from
                        && (prev.range.to > range.from || !self.is_start_of_block(range.from))
                        && !self.ranges[entry.index].has_flag(LiveRangeFlag::StartsAtDef)
                    {
                        trace!(
                            "prev LR {} abuts LR {} in same block; moving {} -> {} for v{}",
                            prev.index.index(),
                            entry.index.index(),
                            prev_alloc,
                            alloc,
                            vreg.index()
                        );
                        debug_assert_eq!(range.from.pos(), InstPosition::Before);
                        inserted_moves.push(
                            range.from,
                            InsertMovePrio::Regular,
                            prev_alloc,
                            alloc,
                            self.vreg(vreg),
                        );
                    }
                }

                // Scan over blocks whose ends are covered by this
                // range. For each, for each successor that is not
                // already in this range (hence guaranteed to have the
                // same allocation) and if the vreg is live, add a
                // Source half-move.
                let mut block = self.cfginfo.insn_block[range.from.inst().index()];
                while block.is_valid() && block.index() < self.func.num_blocks() {
                    if range.to < self.cfginfo.block_exit[block.index()].next() {
                        break;
                    }
                    trace!("examining block with end in range: block{}", block.index());

                    match inter_block_sources.entry(block) {
                        // If the entry is already present in the map, we'll try to prefer a
                        // register allocation.
                        Entry::Occupied(mut entry) => {
                            if !entry.get().is_reg() {
                                entry.insert(alloc);
                            }
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(alloc);
                        }
                    }

                    // Scan forward in `blockparam_outs`, adding all
                    // half-moves for outgoing values to blockparams
                    // in succs.
                    trace!(
                        "scanning blockparam_outs for v{} block{}: blockparam_out_idx = {}",
                        vreg.index(),
                        block.index(),
                        blockparam_out_idx,
                    );
                    while blockparam_out_idx < self.blockparam_outs.len() {
                        let BlockparamOut {
                            from_vreg,
                            from_block,
                            to_block,
                            to_vreg,
                        } = self.blockparam_outs[blockparam_out_idx];
                        if (from_vreg, from_block) > (vreg, block) {
                            break;
                        }
                        if (from_vreg, from_block) == (vreg, block) {
                            trace!(
                                " -> found: from v{} block{} to v{} block{}",
                                from_vreg.index(),
                                from_block.index(),
                                to_vreg.index(),
                                to_vreg.index()
                            );

                            let key = BlockparamSourceKey::new(from_block, to_vreg);
                            match block_param_sources.entry(key) {
                                // As with inter-block moves, if the entry is already present we'll
                                // try to prefer a register allocation.
                                Entry::Occupied(mut entry) => {
                                    if !entry.get().is_reg() {
                                        entry.insert(alloc);
                                    }
                                }
                                Entry::Vacant(entry) => {
                                    entry.insert(alloc);
                                }
                            }

                            if self.annotations_enabled {
                                self.annotate(
                                    self.cfginfo.block_exit[block.index()],
                                    format!(
                                        "blockparam-out: block{} to block{}: v{} to v{} in {}",
                                        from_block.index(),
                                        to_block.index(),
                                        from_vreg.index(),
                                        to_vreg.index(),
                                        alloc
                                    ),
                                );
                            }
                        }

                        blockparam_out_idx += 1;
                    }

                    block = block.next();
                }

                // Scan over blocks whose beginnings are covered by
                // this range and for which the vreg is live at the
                // start of the block. For each, for each predecessor,
                // add a Dest half-move.
                let mut block = self.cfginfo.insn_block[range.from.inst().index()];
                if self.cfginfo.block_entry[block.index()] < range.from {
                    block = block.next();
                }
                while block.is_valid() && block.index() < self.func.num_blocks() {
                    if self.cfginfo.block_entry[block.index()] >= range.to {
                        break;
                    }

                    // Add half-moves for blockparam inputs.
                    trace!(
                        "scanning blockparam_ins at vreg {} block {}: blockparam_in_idx = {}",
                        vreg.index(),
                        block.index(),
                        prev.prev_ins_idx,
                    );
                    let mut idx = prev.blockparam_ins_idx();
                    while idx < self.blockparam_ins.len() {
                        let BlockparamIn {
                            from_block,
                            to_block,
                            to_vreg,
                        } = self.blockparam_ins[idx];
                        if (to_vreg, to_block) > (vreg, block) {
                            break;
                        }
                        if (to_vreg, to_block) == (vreg, block) {
                            block_param_dests.push(BlockparamDest {
                                from_block,
                                to_block,
                                to_vreg,
                                alloc,
                            });
                            trace!(
                                "match: blockparam_in: v{} in block{} from block{} into {}",
                                to_vreg.index(),
                                to_block.index(),
                                from_block.index(),
                                alloc,
                            );
                            #[cfg(debug_assertions)]
                            if self.annotations_enabled {
                                self.annotate(
                                    self.cfginfo.block_entry[block.index()],
                                    format!(
                                        "blockparam-in: block{} to block{}:into v{} in {}",
                                        from_block.index(),
                                        to_block.index(),
                                        to_vreg.index(),
                                        alloc
                                    ),
                                );
                            }
                        }
                        idx += 1;
                    }

                    prev.update_blockparam_ins_idx(idx);

                    if !self.is_live_in(block, vreg) {
                        block = block.next();
                        continue;
                    }

                    trace!(
                        "scanning preds at vreg {} block {} for ends outside the range",
                        vreg.index(),
                        block.index()
                    );

                    // Now find any preds whose ends are not in the
                    // same range, and insert appropriate moves.
                    for &pred in self.func.block_preds(block) {
                        trace!(
                            "pred block {} has exit {:?}",
                            pred.index(),
                            self.cfginfo.block_exit[pred.index()]
                        );
                        if range.contains_point(self.cfginfo.block_exit[pred.index()]) {
                            continue;
                        }

                        inter_block_dests.push(InterBlockDest {
                            from: pred,
                            to: block,
                            alloc,
                        })
                    }

                    block = block.next();
                }

                // Scan over def/uses and apply allocations.
                for use_idx in 0..self.ranges[entry.index].uses.len() {
                    let usedata = self.ranges[entry.index].uses[use_idx];
                    trace!("applying to use: {:?}", usedata);
                    debug_assert!(range.contains_point(usedata.pos));
                    let inst = usedata.pos.inst();
                    let slot = usedata.slot;
                    let operand = usedata.operand;
                    self.set_alloc(inst, slot as usize, alloc);
                    if let OperandConstraint::Reuse(_) = operand.constraint() {
                        reuse_input_insts.push(inst);
                    }
                }

                // Scan debug-labels on this vreg that overlap with
                // this range, producing a debug-info output record
                // giving the allocation location for each label.
                if !debug_labels.is_empty() {
                    // Do a binary search to find the start of any
                    // labels for this vreg. Recall that we require
                    // debug-label requests to be sorted by vreg as a
                    // precondition (which we verified above).
                    let start = debug_labels
                        .binary_search_by(|&(label_vreg, _label_from, _label_to, _label)| {
                            // Search for the point just before the first
                            // tuple that could be for `vreg` overlapping
                            // with `range`. Never return
                            // `Ordering::Equal`; `binary_search_by` in
                            // this case returns the index of the first
                            // entry that is greater as an `Err`.
                            if label_vreg.vreg() < vreg.index() {
                                core::cmp::Ordering::Less
                            } else {
                                core::cmp::Ordering::Greater
                            }
                        })
                        .unwrap_err();

                    for &(label_vreg, label_from, label_to, label) in &debug_labels[start..] {
                        let label_from = ProgPoint::before(label_from);
                        let label_to = ProgPoint::before(label_to);
                        let label_range = CodeRange {
                            from: label_from,
                            to: label_to,
                        };
                        if label_vreg.vreg() != vreg.index() {
                            break;
                        }
                        if !range.overlaps(&label_range) {
                            continue;
                        }

                        let from = core::cmp::max(label_from, range.from);
                        let to = core::cmp::min(label_to, range.to);

                        self.ctx
                            .output
                            .debug_locations
                            .push((label, from, to, alloc));
                    }
                }
            }

            if !inter_block_dests.is_empty() {
                self.output.stats.halfmoves_count += inter_block_dests.len() * 2;

                inter_block_dests.sort_unstable_by_key(InterBlockDest::key);

                let vreg = self.vreg(vreg);
                trace!("processing inter-block moves for {}", vreg);
                for dest in inter_block_dests.drain(..) {
                    let src = inter_block_sources[&dest.from];

                    trace!(
                        " -> moving from {} to {} between {:?} and {:?}",
                        src,
                        dest.alloc,
                        dest.from,
                        dest.to
                    );

                    let (pos, prio) = choose_move_location(self, dest.from, dest.to);
                    inserted_moves.push(pos, prio, src, dest.alloc, vreg);
                }
            }

            blockparam_in_idx = prev.blockparam_ins_idx();
        }

        if !block_param_dests.is_empty() {
            self.output.stats.halfmoves_count += block_param_sources.len();
            self.output.stats.halfmoves_count += block_param_dests.len();

            trace!("processing block-param moves");
            for dest in block_param_dests {
                let src = dest.source();
                let src_alloc = block_param_sources.get(&src).unwrap();
                let (pos, prio) = choose_move_location(self, dest.from_block, dest.to_block);
                inserted_moves.push(pos, prio, *src_alloc, dest.alloc, self.vreg(dest.to_vreg));
            }
        }

        // Handle multi-fixed-reg constraints by copying.
        for fixup in core::mem::replace(&mut self.multi_fixed_reg_fixups, vec![]) {
            let from_alloc = self.get_alloc(fixup.pos.inst(), fixup.from_slot as usize);
            let to_alloc = Allocation::reg(PReg::from_index(fixup.to_preg.index()));
            trace!(
                "multi-fixed-move constraint at {:?} from {} to {} for v{}",
                fixup.pos,
                from_alloc,
                to_alloc,
                fixup.vreg.index(),
            );
            let prio = match fixup.level {
                FixedRegFixupLevel::Initial => InsertMovePrio::MultiFixedRegInitial,
                FixedRegFixupLevel::Secondary => InsertMovePrio::MultiFixedRegSecondary,
            };
            inserted_moves.push(fixup.pos, prio, from_alloc, to_alloc, self.vreg(fixup.vreg));
            self.set_alloc(
                fixup.pos.inst(),
                fixup.to_slot as usize,
                Allocation::reg(PReg::from_index(fixup.to_preg.index())),
            );
        }

        // Handle outputs that reuse inputs: copy beforehand, then set
        // input's alloc to output's.
        //
        // Note that the output's allocation may not *actually* be
        // valid until InstPosition::After, but the reused input may
        // occur at InstPosition::Before. This may appear incorrect,
        // but we make it work by ensuring that all *other* inputs are
        // extended to InstPosition::After so that the def will not
        // interfere. (The liveness computation code does this -- we
        // do not require the user to do so.)
        //
        // One might ask: why not insist that input-reusing defs occur
        // at InstPosition::Before? this would be correct, but would
        // mean that the reused input and the reusing output
        // interfere, *guaranteeing* that every such case would
        // require a move. This is really bad on ISAs (like x86) where
        // reused inputs are ubiquitous.
        //
        // Another approach might be to put the def at Before, and
        // trim the reused input's liverange back to the previous
        // instruction's After. This is kind of OK until (i) a block
        // boundary occurs between the prior inst and this one, or
        // (ii) any moves/spills/reloads occur between the two
        // instructions. We really do need the input to be live at
        // this inst's Before.
        //
        // In principle what we really need is a "BeforeBefore"
        // program point, but we don't want to introduce that
        // everywhere and pay the cost of twice as many ProgPoints
        // throughout the allocator.
        //
        // Or we could introduce a separate move instruction -- this
        // is the approach that regalloc.rs takes with "mod" operands
        // -- but that is also costly.
        //
        // So we take this approach (invented by IonMonkey -- somewhat
        // hard to discern, though see [0] for a comment that makes
        // this slightly less unclear) to avoid interference between
        // the actual reused input and reusing output, ensure
        // interference (hence no incorrectness) between other inputs
        // and the reusing output, and not require a separate explicit
        // move instruction.
        //
        // [0] https://searchfox.org/mozilla-central/rev/3a798ef9252896fb389679f06dd3203169565af0/js/src/jit/shared/Lowering-shared-inl.h#108-110
        for inst in reuse_input_insts {
            let mut input_reused: SmallVec<[usize; 4]> = smallvec![];
            for output_idx in 0..self.func.inst_operands(inst).len() {
                let operand = self.func.inst_operands(inst)[output_idx];
                if let OperandConstraint::Reuse(input_idx) = operand.constraint() {
                    debug_assert!(!input_reused.contains(&input_idx));
                    debug_assert_eq!(operand.pos(), OperandPos::Late);
                    input_reused.push(input_idx);
                    let input_alloc = self.get_alloc(inst, input_idx);
                    let output_alloc = self.get_alloc(inst, output_idx);
                    trace!(
                        "reuse-input inst {:?}: output {} has alloc {:?}, input {} has alloc {:?}",
                        inst,
                        output_idx,
                        output_alloc,
                        input_idx,
                        input_alloc
                    );
                    if input_alloc != output_alloc {
                        #[cfg(debug_assertions)]
                        if self.annotations_enabled {
                            self.annotate(
                                ProgPoint::before(inst),
                                format!(" reuse-input-copy: {} -> {}", input_alloc, output_alloc),
                            );
                        }
                        let input_operand = self.func.inst_operands(inst)[input_idx];
                        inserted_moves.push(
                            ProgPoint::before(inst),
                            InsertMovePrio::ReusedInput,
                            input_alloc,
                            output_alloc,
                            input_operand.vreg(),
                        );
                        self.set_alloc(inst, input_idx, output_alloc);
                    }
                }
            }
        }

        // Sort the debug-locations vector; we provide this
        // invariant to the client.
        self.output.debug_locations.sort_unstable();

        inserted_moves
    }

    pub fn resolve_inserted_moves(&mut self, mut inserted_moves: InsertedMoves) -> Edits {
        // For each program point, gather all moves together. Then
        // resolve (see cases below).
        let mut i = 0;
        inserted_moves
            .moves
            .sort_unstable_by_key(|m| m.pos_prio.key());

        // Redundant-move elimination state tracker.
        let mut redundant_moves = RedundantMoveEliminator::default();

        fn redundant_move_process_side_effects<'a, F: Function>(
            this: &Env<'a, F>,
            redundant_moves: &mut RedundantMoveEliminator,
            from: ProgPoint,
            to: ProgPoint,
        ) {
            // If we cross a block boundary, clear and return.
            if this.cfginfo.insn_block[from.inst().index()]
                != this.cfginfo.insn_block[to.inst().index()]
            {
                redundant_moves.clear();
                return;
            }

            let start_inst = if from.pos() == InstPosition::Before {
                from.inst()
            } else {
                from.inst().next()
            };
            let end_inst = if to.pos() == InstPosition::Before {
                to.inst()
            } else {
                to.inst().next()
            };
            for inst in start_inst.index()..end_inst.index() {
                let inst = Inst::new(inst);
                for (i, op) in this.func.inst_operands(inst).iter().enumerate() {
                    match op.kind() {
                        OperandKind::Def => {
                            let alloc = this.get_alloc(inst, i);
                            redundant_moves.clear_alloc(alloc);
                        }
                        _ => {}
                    }
                }
                for reg in this.func.inst_clobbers(inst) {
                    redundant_moves.clear_alloc(Allocation::reg(reg));
                }
                // The dedicated scratch registers may be clobbered by any
                // instruction.
                for reg in this.env.scratch_by_class {
                    if let Some(reg) = reg {
                        redundant_moves.clear_alloc(Allocation::reg(reg));
                    }
                }
            }
        }

        let mut last_pos = ProgPoint::before(Inst::new(0));
        let mut edits = Edits::with_capacity(self.func.num_insts());

        while i < inserted_moves.moves.len() {
            let start = i;
            let pos_prio = inserted_moves.moves[i].pos_prio;
            while i < inserted_moves.moves.len() && inserted_moves.moves[i].pos_prio == pos_prio {
                i += 1;
            }
            let moves = &inserted_moves.moves[start..i];

            redundant_move_process_side_effects(self, &mut redundant_moves, last_pos, pos_prio.pos);
            last_pos = pos_prio.pos;

            // Gather all the moves in each RegClass separately.
            // These cannot interact, so it is safe to have separate
            // ParallelMove instances. They need to be separate because
            // moves between the classes are impossible. (We could
            // enhance ParallelMoves to understand register classes, but
            // this seems simpler.)
            let mut int_moves: SmallVec<[InsertedMove; 8]> = smallvec![];
            let mut float_moves: SmallVec<[InsertedMove; 8]> = smallvec![];
            let mut vec_moves: SmallVec<[InsertedMove; 8]> = smallvec![];

            for m in moves {
                match m.to_vreg.class() {
                    RegClass::Int => {
                        int_moves.push(m.clone());
                    }
                    RegClass::Float => {
                        float_moves.push(m.clone());
                    }
                    RegClass::Vector => {
                        vec_moves.push(m.clone());
                    }
                }
            }

            for &(regclass, moves) in &[
                (RegClass::Int, &int_moves),
                (RegClass::Float, &float_moves),
                (RegClass::Vector, &vec_moves),
            ] {
                // All moves in `moves` semantically happen in
                // parallel. Let's resolve these to a sequence of moves
                // that can be done one at a time.
                let mut parallel_moves = ParallelMoves::new();
                trace!(
                    "parallel moves at pos {:?} prio {:?}",
                    pos_prio.pos,
                    pos_prio.prio
                );
                for m in moves {
                    trace!(" {} -> {}", m.from_alloc, m.to_alloc);
                    parallel_moves.add(m.from_alloc, m.to_alloc, Some(m.to_vreg));
                }

                let resolved = parallel_moves.resolve();
                let mut scratch_iter = RegTraversalIter::new(
                    self.env,
                    regclass,
                    PReg::invalid(),
                    PReg::invalid(),
                    0,
                    None,
                );
                let mut dedicated_scratch = self.env.scratch_by_class[regclass as usize];
                let key = LiveRangeKey::from_range(&CodeRange {
                    from: pos_prio.pos,
                    to: pos_prio.pos.next(),
                });
                let find_free_reg = || {
                    // Use the dedicated scratch register first if it is
                    // available.
                    if let Some(reg) = dedicated_scratch.take() {
                        return Some(Allocation::reg(reg));
                    }
                    while let Some(preg) = scratch_iter.next() {
                        if !self.pregs[preg.index()]
                            .allocations
                            .btree
                            .contains_key(&key)
                        {
                            let alloc = Allocation::reg(preg);
                            if moves
                                .iter()
                                .any(|m| m.from_alloc == alloc || m.to_alloc == alloc)
                            {
                                // Skip pregs used by moves in this
                                // parallel move set, even if not
                                // marked used at progpoint: edge move
                                // liveranges meet but don't overlap
                                // so otherwise we may incorrectly
                                // overwrite a source reg.
                                continue;
                            }
                            return Some(alloc);
                        }
                    }
                    None
                };
                let mut stackslot_idx = 0;
                let get_stackslot = || {
                    let idx = stackslot_idx;
                    stackslot_idx += 1;
                    // We can't borrow `self` as mutable, so we create
                    // these placeholders then allocate the actual
                    // slots if needed with `self.allocate_spillslot`
                    // below.
                    Allocation::stack(SpillSlot::new(SpillSlot::MAX - idx))
                };
                let is_stack_alloc = |alloc: Allocation| {
                    if let Some(preg) = alloc.as_reg() {
                        self.pregs[preg.index()].is_stack
                    } else {
                        alloc.is_stack()
                    }
                };
                let preferred_victim = self.preferred_victim_by_class[regclass as usize];

                let scratch_resolver = MoveAndScratchResolver {
                    find_free_reg,
                    get_stackslot,
                    is_stack_alloc,
                    borrowed_scratch_reg: preferred_victim,
                };

                let resolved = scratch_resolver.compute(resolved);

                let mut rewrites = FxHashMap::default();
                for i in 0..stackslot_idx {
                    if i >= self.extra_spillslots_by_class[regclass as usize].len() {
                        let slot =
                            self.allocate_spillslot(self.func.spillslot_size(regclass) as u32);
                        self.extra_spillslots_by_class[regclass as usize].push(slot);
                    }
                    rewrites.insert(
                        Allocation::stack(SpillSlot::new(SpillSlot::MAX - i)),
                        self.extra_spillslots_by_class[regclass as usize][i],
                    );
                }

                for (src, dst, to_vreg) in resolved {
                    let src = rewrites.get(&src).cloned().unwrap_or(src);
                    let dst = rewrites.get(&dst).cloned().unwrap_or(dst);
                    trace!("  resolved: {} -> {} ({:?})", src, dst, to_vreg);
                    let action = redundant_moves.process_move(src, dst, to_vreg);
                    if !action.elide {
                        edits.add(pos_prio, src, dst);
                    } else {
                        trace!("    -> redundant move elided");
                    }
                }
            }
        }

        // Ensure edits are in sorted ProgPoint order. N.B.: this must
        // be a stable sort! We have to keep the order produced by the
        // parallel-move resolver for all moves within a single sort
        // key.
        edits.sort();
        self.output.stats.edits_count = edits.len();

        // Add debug annotations.
        if self.annotations_enabled {
            for &(pos_prio, ref edit) in edits.iter() {
                match edit {
                    &Edit::Move { from, to } => {
                        self.annotate(pos_prio.pos, format!("move {} -> {}", from, to));
                    }
                }
            }
        }

        edits
    }
}
