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

//! Bundle merging.

use super::{Env, LiveBundleIndex, SpillSet, SpillSlotIndex, VRegIndex};
use crate::{
    ion::{
        data_structures::{BlockparamOut, CodeRange},
        LiveRangeList,
    },
    Function, Inst, OperandConstraint, OperandKind, PReg, ProgPoint,
};
use alloc::format;

impl<'a, F: Function> Env<'a, F> {
    pub fn merge_bundles(&mut self, from: LiveBundleIndex, to: LiveBundleIndex) -> bool {
        if from == to {
            // Merge bundle into self -- trivial merge.
            return true;
        }
        trace!(
            "merging from bundle{} to bundle{}",
            from.index(),
            to.index()
        );

        // Both bundles must deal with the same RegClass.
        let from_rc = self.spillsets[self.bundles[from].spillset].class;
        let to_rc = self.spillsets[self.bundles[to].spillset].class;
        if from_rc != to_rc {
            trace!(" -> mismatching reg classes");
            return false;
        }

        // If either bundle is already assigned (due to a pinned vreg), don't merge.
        if self.bundles[from].allocation.is_some() || self.bundles[to].allocation.is_some() {
            trace!("one of the bundles is already assigned (pinned)");
            return false;
        }

        #[cfg(debug_assertions)]
        {
            // Sanity check: both bundles should contain only ranges with appropriate VReg classes.
            for entry in &self.bundles[from].ranges {
                let vreg = self.ranges[entry.index].vreg;
                debug_assert_eq!(from_rc, self.vreg(vreg).class());
            }
            for entry in &self.bundles[to].ranges {
                let vreg = self.ranges[entry.index].vreg;
                debug_assert_eq!(to_rc, self.vreg(vreg).class());
            }
        }

        // If a bundle has a fixed-reg def then we need to be careful to not
        // extend the bundle to include another use in the same instruction.
        // This could result in a minimal bundle that is impossible to split.
        //
        // This can only happen with an early use and a late def, so we round
        // the start of each range containing a fixed def up to the start of
        // its instruction to detect overlaps.
        let adjust_range_start = |bundle_idx, range: CodeRange| {
            if self.bundles[bundle_idx].cached_fixed_def() {
                ProgPoint::before(range.from.inst())
            } else {
                range.from
            }
        };

        // Check for overlap in LiveRanges and for conflicting
        // requirements.
        let ranges_from = &self.bundles[from].ranges[..];
        let ranges_to = &self.bundles[to].ranges[..];
        let mut idx_from = 0;
        let mut idx_to = 0;
        let mut range_count = 0;
        while idx_from < ranges_from.len() && idx_to < ranges_to.len() {
            range_count += 1;
            if range_count > 200 {
                trace!(
                    "reached merge complexity (range_count = {}); exiting",
                    range_count
                );
                // Limit merge complexity.
                return false;
            }

            if adjust_range_start(from, ranges_from[idx_from].range) >= ranges_to[idx_to].range.to {
                idx_to += 1;
            } else if adjust_range_start(to, ranges_to[idx_to].range)
                >= ranges_from[idx_from].range.to
            {
                idx_from += 1;
            } else {
                // Overlap -- cannot merge.
                trace!(
                    " -> overlap between {:?} and {:?}, exiting",
                    ranges_from[idx_from].index,
                    ranges_to[idx_to].index
                );
                return false;
            }
        }

        // Check for a requirements conflict.
        if self.bundles[from].cached_fixed() || self.bundles[to].cached_fixed() {
            if self.merge_bundle_requirements(from, to).is_err() {
                trace!(" -> conflicting requirements; aborting merge");
                return false;
            }
        }

        trace!(" -> committing to merge");

        // If we reach here, then the bundles do not overlap -- merge
        // them!  We do this with a merge-sort-like scan over both
        // lists, building a new range list and replacing the list on
        // `to` when we're done.
        if ranges_from.is_empty() {
            // `from` bundle is empty -- trivial merge.
            trace!(" -> from bundle{} is empty; trivial merge", from.index());
            return true;
        }
        if ranges_to.is_empty() {
            // `to` bundle is empty -- just move the list over from
            // `from` and set `bundle` up-link on all ranges.
            trace!(" -> to bundle{} is empty; trivial merge", to.index());
            let empty_vec = LiveRangeList::new_in(self.ctx.bump());
            let list = core::mem::replace(&mut self.bundles[from].ranges, empty_vec);
            for entry in &list {
                self.ranges[entry.index].bundle = to;

                if self.annotations_enabled {
                    self.annotate(
                        entry.range.from,
                        format!(
                            " MERGE range{} v{} from bundle{} to bundle{}",
                            entry.index.index(),
                            self.ranges[entry.index].vreg.index(),
                            from.index(),
                            to.index(),
                        ),
                    );
                }
            }
            self.bundles[to].ranges = list;

            if self.bundles[from].cached_fixed() {
                self.bundles[to].set_cached_fixed();
            }
            if self.bundles[from].cached_fixed_def() {
                self.bundles[to].set_cached_fixed_def();
            }

            return true;
        }

        trace!(
            "merging: ranges_from = {:?} ranges_to = {:?}",
            ranges_from,
            ranges_to
        );

        let empty_vec = LiveRangeList::new_in(self.ctx.bump());
        let mut from_list = core::mem::replace(&mut self.bundles[from].ranges, empty_vec);
        for entry in &from_list {
            self.ranges[entry.index].bundle = to;
        }

        if from_list.len() == 1 {
            // Optimize for the common case where `from_list` contains a single
            // item. Using a binary search to find the insertion point and then
            // calling `insert` is more efficient than re-sorting the entire
            // list, specially after the changes in sorting algorithms introduced
            // in rustc 1.81.
            // See: https://github.com/bytecodealliance/regalloc2/issues/203
            let single_entry = from_list.pop().unwrap();
            let pos = self.bundles[to]
                .ranges
                .binary_search_by_key(&single_entry.range.from, |entry| entry.range.from)
                .unwrap_or_else(|pos| pos);
            self.bundles[to].ranges.insert(pos, single_entry);
        } else {
            // Two non-empty lists of LiveRanges: concatenate and sort. This is
            // faster than a mergesort-like merge into a new list, empirically.
            self.bundles[to].ranges.extend_from_slice(&from_list[..]);
            self.bundles[to]
                .ranges
                .sort_unstable_by_key(|entry| entry.range.from);
        }

        if self.annotations_enabled {
            trace!("merging: merged = {:?}", self.bundles[to].ranges);
            let mut last_range = None;
            for i in 0..self.bundles[to].ranges.len() {
                let entry = self.bundles[to].ranges[i];
                if last_range.is_some() {
                    debug_assert!(last_range.unwrap() < entry.range);
                }
                last_range = Some(entry.range);

                if self.ranges[entry.index].bundle == from {
                    self.annotate(
                        entry.range.from,
                        format!(
                            " MERGE range{} v{} from bundle{} to bundle{}",
                            entry.index.index(),
                            self.ranges[entry.index].vreg.index(),
                            from.index(),
                            to.index(),
                        ),
                    );
                }

                trace!(
                    " -> merged result for bundle{}: range{}",
                    to.index(),
                    entry.index.index(),
                );
            }
        }

        if self.bundles[from].spillset != self.bundles[to].spillset {
            // Widen the range for the target spillset to include the one being merged in.
            let from_range = self.spillsets[self.bundles[from].spillset].range;
            let to_range = &mut self.ctx.spillsets[self.ctx.bundles[to].spillset].range;
            *to_range = to_range.join(from_range);
        }

        if self.bundles[from].cached_fixed() {
            self.bundles[to].set_cached_fixed();
        }
        if self.bundles[from].cached_fixed_def() {
            self.bundles[to].set_cached_fixed_def();
        }

        true
    }

    pub fn merge_vreg_bundles(&mut self) {
        // Create a bundle for every vreg, initially.
        trace!("merge_vreg_bundles: creating vreg bundles");
        for vreg in 0..self.vregs.len() {
            let vreg = VRegIndex::new(vreg);
            if self.vregs[vreg].ranges.is_empty() {
                continue;
            }

            let bundle = self.ctx.bundles.add(self.ctx.bump());
            let mut range = self.vregs[vreg].ranges.first().unwrap().range;

            self.bundles[bundle].ranges = self.vregs[vreg].ranges.clone();
            trace!("vreg v{} gets bundle{}", vreg.index(), bundle.index());
            for entry in &self.ctx.bundles[bundle].ranges {
                trace!(
                    " -> with LR range{}: {:?}",
                    entry.index.index(),
                    entry.range
                );
                range = range.join(entry.range);
                self.ctx.ranges[entry.index].bundle = bundle;
            }

            let mut fixed = false;
            let mut fixed_def = false;
            for entry in &self.bundles[bundle].ranges {
                for u in &self.ranges[entry.index].uses {
                    if let OperandConstraint::FixedReg(_) = u.operand.constraint() {
                        fixed = true;
                        if u.operand.kind() == OperandKind::Def {
                            fixed_def = true;
                        }
                    }
                    if fixed && fixed_def {
                        break;
                    }
                }
            }
            if fixed {
                self.bundles[bundle].set_cached_fixed();
            }
            if fixed_def {
                self.bundles[bundle].set_cached_fixed_def();
            }

            // Create a spillslot for this bundle.
            let reg = self.vreg(vreg);
            let ssidx = self.spillsets.push(SpillSet {
                slot: SpillSlotIndex::invalid(),
                required: false,
                class: reg.class(),
                reg_hint: PReg::invalid(),
                spill_bundle: LiveBundleIndex::invalid(),
                splits: 0,
                range,
            });
            self.bundles[bundle].spillset = ssidx;
        }

        for inst in 0..self.func.num_insts() {
            let inst = Inst::new(inst);

            // Attempt to merge Reuse-constraint operand outputs with the
            // corresponding inputs.
            for op in self.func.inst_operands(inst) {
                if let OperandConstraint::Reuse(reuse_idx) = op.constraint() {
                    let src_vreg = op.vreg();
                    let dst_vreg = self.func.inst_operands(inst)[reuse_idx].vreg();

                    trace!(
                        "trying to merge reused-input def: src {} to dst {}",
                        src_vreg,
                        dst_vreg
                    );
                    let src_bundle = self.ranges[self.vregs[src_vreg].ranges[0].index].bundle;
                    debug_assert!(src_bundle.is_valid());
                    let dest_bundle = self.ranges[self.vregs[dst_vreg].ranges[0].index].bundle;
                    debug_assert!(dest_bundle.is_valid());
                    self.merge_bundles(/* from */ dest_bundle, /* to */ src_bundle);
                }
            }
        }

        // Attempt to merge blockparams with their inputs.
        for i in 0..self.blockparam_outs.len() {
            let BlockparamOut {
                from_vreg, to_vreg, ..
            } = self.blockparam_outs[i];
            trace!(
                "trying to merge blockparam v{} with input v{}",
                to_vreg.index(),
                from_vreg.index()
            );
            let to_bundle = self.ranges[self.vregs[to_vreg].ranges[0].index].bundle;
            debug_assert!(to_bundle.is_valid());
            let from_bundle = self.ranges[self.vregs[from_vreg].ranges[0].index].bundle;
            debug_assert!(from_bundle.is_valid());
            trace!(
                " -> from bundle{} to bundle{}",
                from_bundle.index(),
                to_bundle.index()
            );
            self.merge_bundles(from_bundle, to_bundle);
        }

        trace!("done merging bundles");
    }

    pub fn compute_bundle_prio(&self, bundle: LiveBundleIndex) -> u32 {
        // The priority is simply the total "length" -- the number of
        // instructions covered by all LiveRanges.
        let mut total = 0;
        for entry in &self.bundles[bundle].ranges {
            total += entry.range.len() as u32;
        }
        total
    }

    pub fn queue_bundles(&mut self) {
        for bundle in 0..self.bundles.len() {
            trace!("enqueueing bundle{}", bundle);
            let bundle = LiveBundleIndex::new(bundle);
            if self.bundles[bundle].ranges.is_empty() {
                trace!(" -> no ranges; skipping");
                continue;
            }
            let prio = self.compute_bundle_prio(bundle);
            trace!(" -> prio {}", prio);
            self.bundles[bundle].prio = prio;
            self.recompute_bundle_properties(bundle);
            self.allocation_queue
                .insert(bundle, prio as usize, PReg::invalid());
        }
        self.output.stats.merged_bundle_count = self.allocation_queue.heap.len();
    }
}
