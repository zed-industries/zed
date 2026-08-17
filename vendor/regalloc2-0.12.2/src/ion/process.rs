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

//! Main allocation loop that processes bundles.

use super::{
    spill_weight_from_constraint, Env, LiveBundleIndex, LiveBundleVec, LiveRangeFlag,
    LiveRangeIndex, LiveRangeKey, LiveRangeList, LiveRangeListEntry, PRegIndex, RegTraversalIter,
    Requirement, SpillWeight, UseList, VRegIndex,
};
use crate::{
    ion::data_structures::{
        CodeRange, Use, BUNDLE_MAX_NORMAL_SPILL_WEIGHT, MAX_SPLITS_PER_SPILLSET,
        MINIMAL_BUNDLE_SPILL_WEIGHT, MINIMAL_FIXED_BUNDLE_SPILL_WEIGHT,
    },
    Allocation, Function, Inst, InstPosition, OperandConstraint, OperandKind, PReg, ProgPoint,
    RegAllocError,
};
use core::fmt::Debug;
use smallvec::{smallvec, SmallVec};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllocRegResult<'a> {
    Allocated(Allocation),
    Conflict(&'a [LiveBundleIndex], ProgPoint),
    ConflictWithFixed(u32, ProgPoint),
    ConflictHighCost,
}

impl<'a, F: Function> Env<'a, F> {
    pub fn process_bundles(&mut self) -> Result<(), RegAllocError> {
        while let Some((bundle, reg_hint)) = self.ctx.allocation_queue.pop() {
            self.ctx.output.stats.process_bundle_count += 1;
            self.process_bundle(bundle, reg_hint)?;
        }
        self.ctx.output.stats.final_liverange_count = self.ranges.len();
        self.ctx.output.stats.final_bundle_count = self.bundles.len();
        self.ctx.output.stats.spill_bundle_count = self.spilled_bundles.len();

        Ok(())
    }

    pub fn try_to_allocate_bundle_to_reg<'b>(
        &mut self,
        bundle: LiveBundleIndex,
        reg: PRegIndex,
        // if the max bundle weight in the conflict set exceeds this
        // cost (if provided), just return
        // `AllocRegResult::ConflictHighCost`.
        max_allowable_cost: Option<u32>,
        conflicts: &'b mut LiveBundleVec,
    ) -> AllocRegResult<'b> {
        trace!("try_to_allocate_bundle_to_reg: {:?} -> {:?}", bundle, reg);
        conflicts.clear();
        self.ctx.conflict_set.clear();
        let mut max_conflict_weight = 0;
        // Traverse the BTreeMap in order by requesting the whole
        // range spanned by the bundle and iterating over that
        // concurrently with our ranges. Because our ranges are in
        // order, and the BTreeMap is as well, this allows us to have
        // an overall O(n log n) + O(b) complexity, where the PReg has
        // n current ranges and the bundle has b ranges, rather than
        // O(b * n log n) with the simple probe-for-each-bundle-range
        // approach.
        //
        // Note that the comparator function on a CodeRange tests for
        // *overlap*, so we are checking whether the BTree contains
        // any preg range that *overlaps* with range `range`, not
        // literally the range `range`.
        let bundle_ranges = &self.ctx.bundles[bundle].ranges;
        let from_key = LiveRangeKey::from_range(&CodeRange {
            from: bundle_ranges.first().unwrap().range.from,
            to: bundle_ranges.first().unwrap().range.from,
        });
        let mut preg_range_iter = self.ctx.pregs[reg.index()]
            .allocations
            .btree
            .range(from_key..)
            .peekable();
        trace!(
            "alloc map for {:?} in range {:?}..: {:?}",
            reg,
            from_key,
            self.ctx.pregs[reg.index()].allocations.btree
        );
        let mut first_conflict: Option<ProgPoint> = None;

        'ranges: for entry in bundle_ranges {
            trace!(" -> range LR {:?}: {:?}", entry.index, entry.range);
            let key = LiveRangeKey::from_range(&entry.range);

            let mut skips = 0;
            'alloc: loop {
                trace!("  -> PReg range {:?}", preg_range_iter.peek());

                // Advance our BTree traversal until it is >= this bundle
                // range (i.e., skip PReg allocations in the BTree that
                // are completely before this bundle range).

                if preg_range_iter.peek().is_some() && *preg_range_iter.peek().unwrap().0 < key {
                    trace!(
                        "Skipping PReg range {:?}",
                        preg_range_iter.peek().unwrap().0
                    );
                    preg_range_iter.next();
                    skips += 1;
                    if skips >= 16 {
                        let from_pos = entry.range.from;
                        let from_key = LiveRangeKey::from_range(&CodeRange {
                            from: from_pos,
                            to: from_pos,
                        });
                        preg_range_iter = self.ctx.pregs[reg.index()]
                            .allocations
                            .btree
                            .range(from_key..)
                            .peekable();
                        skips = 0;
                    }
                    continue 'alloc;
                }
                skips = 0;

                // If there are no more PReg allocations, we're done!
                if preg_range_iter.peek().is_none() {
                    trace!(" -> no more PReg allocations; so no conflict possible!");
                    break 'ranges;
                }

                // If the current PReg range is beyond this range, there is no conflict; continue.
                if *preg_range_iter.peek().unwrap().0 > key {
                    trace!(
                        " -> next PReg allocation is at {:?}; moving to next VReg range",
                        preg_range_iter.peek().unwrap().0
                    );
                    break 'alloc;
                }

                // Otherwise, there is a conflict.
                let preg_key = *preg_range_iter.peek().unwrap().0;
                debug_assert_eq!(preg_key, key); // Assert that this range overlaps.
                let preg_range = preg_range_iter.next().unwrap().1;

                trace!(" -> btree contains range {:?} that overlaps", preg_range);
                if preg_range.is_valid() {
                    trace!("   -> from vreg {:?}", self.ctx.ranges[*preg_range].vreg);
                    // range from an allocated bundle: find the bundle and add to
                    // conflicts list.
                    let conflict_bundle = self.ctx.ranges[*preg_range].bundle;
                    trace!("   -> conflict bundle {:?}", conflict_bundle);
                    if self.ctx.conflict_set.insert(conflict_bundle) {
                        conflicts.push(conflict_bundle);
                        max_conflict_weight = core::cmp::max(
                            max_conflict_weight,
                            self.ctx.bundles[conflict_bundle].cached_spill_weight(),
                        );
                        if max_allowable_cost.is_some()
                            && max_conflict_weight > max_allowable_cost.unwrap()
                        {
                            trace!("   -> reached high cost, retrying early");
                            return AllocRegResult::ConflictHighCost;
                        }
                    }

                    if first_conflict.is_none() {
                        first_conflict = Some(ProgPoint::from_index(core::cmp::max(
                            preg_key.from,
                            key.from,
                        )));
                    }
                } else {
                    trace!("   -> conflict with fixed reservation");
                    // range from a direct use of the PReg (due to clobber).
                    return AllocRegResult::ConflictWithFixed(
                        max_conflict_weight,
                        ProgPoint::from_index(preg_key.from),
                    );
                }
            }
        }

        if conflicts.len() > 0 {
            return AllocRegResult::Conflict(conflicts, first_conflict.unwrap());
        }

        // We can allocate! Add our ranges to the preg's BTree.
        let preg = PReg::from_index(reg.index());
        trace!("  -> bundle {:?} assigned to preg {:?}", bundle, preg);
        self.ctx.bundles[bundle].allocation = Allocation::reg(preg);
        for entry in &self.ctx.bundles[bundle].ranges {
            let key = LiveRangeKey::from_range(&entry.range);
            let res = self.ctx.pregs[reg.index()]
                .allocations
                .btree
                .insert(key, entry.index);

            // We disallow LR overlap within bundles, so this should never be possible.
            debug_assert!(res.is_none());
        }

        AllocRegResult::Allocated(Allocation::reg(preg))
    }

    pub fn evict_bundle(&mut self, bundle: LiveBundleIndex) {
        trace!(
            "evicting bundle {:?}: alloc {:?}",
            bundle,
            self.ctx.bundles[bundle].allocation
        );
        let preg = match self.ctx.bundles[bundle].allocation.as_reg() {
            Some(preg) => preg,
            None => {
                trace!(
                    "  -> has no allocation! {:?}",
                    self.ctx.bundles[bundle].allocation
                );
                return;
            }
        };
        let preg_idx = PRegIndex::new(preg.index());
        self.ctx.bundles[bundle].allocation = Allocation::none();
        for entry in &self.ctx.bundles[bundle].ranges {
            trace!(" -> removing LR {:?} from reg {:?}", entry.index, preg_idx);
            self.ctx.pregs[preg_idx.index()]
                .allocations
                .btree
                .remove(&LiveRangeKey::from_range(&entry.range));
        }
        let prio = self.ctx.bundles[bundle].prio;
        trace!(" -> prio {}; back into queue", prio);
        self.ctx
            .allocation_queue
            .insert(bundle, prio as usize, PReg::invalid());
    }

    pub fn bundle_spill_weight(&self, bundle: LiveBundleIndex) -> u32 {
        self.ctx.bundles[bundle].cached_spill_weight()
    }

    pub fn maximum_spill_weight_in_bundle_set(&self, bundles: &[LiveBundleIndex]) -> u32 {
        trace!("maximum_spill_weight_in_bundle_set: {:?}", bundles);
        let m = bundles
            .iter()
            .map(|&b| {
                let w = self.ctx.bundles[b].cached_spill_weight();
                trace!("bundle{}: {}", b.index(), w);
                w
            })
            .max()
            .unwrap_or(0);
        trace!(" -> max: {}", m);
        m
    }

    pub fn recompute_bundle_properties(&mut self, bundle: LiveBundleIndex) {
        trace!("recompute bundle properties: bundle {:?}", bundle);

        let minimal;
        let mut fixed = false;
        let mut fixed_def = false;
        let bundledata = &self.ctx.bundles[bundle];
        let num_ranges = bundledata.ranges.len();
        let first_range = bundledata.ranges[0].index;
        let first_range_data = &self.ctx.ranges[first_range];

        self.ctx.bundles[bundle].prio = self.compute_bundle_prio(bundle);

        if first_range_data.vreg.is_invalid() {
            trace!("  -> no vreg; minimal and fixed");
            minimal = true;
            fixed = true;
        } else if num_ranges == 1 {
            for u in &first_range_data.uses {
                trace!("  -> use: {:?}", u);
                if let OperandConstraint::FixedReg(_) = u.operand.constraint() {
                    trace!("  -> fixed operand at {:?}: {:?}", u.pos, u.operand);
                    fixed = true;
                    if u.operand.kind() == OperandKind::Def {
                        trace!("  -> is fixed def");
                        fixed_def = true;
                    }

                    break;
                }
            }
            // Minimal if there is only one LR and only up to one use
            // in that LR, and extent of range is consistent with the
            // "minimal range for use" definition.
            minimal = match &first_range_data.uses[..] {
                [] => true,
                [only_use] => {
                    // We use a "contains" rather than "exact" range
                    // check because a smaller-than-min range is also
                    // OK, if some logic produced it for a valid
                    // reason -- for example, a dead def. We don't
                    // want to livelock on "smaller than minimal"
                    // ranges.
                    let min_range = minimal_range_for_use(only_use);
                    min_range.contains(&first_range_data.range)
                }
                _ => false,
            };
            trace!("  -> minimal: {}", minimal);
        } else {
            minimal = false;
        }

        let spill_weight = if minimal {
            if fixed {
                trace!("  -> fixed and minimal");
                MINIMAL_FIXED_BUNDLE_SPILL_WEIGHT
            } else {
                trace!("  -> non-fixed and minimal");
                MINIMAL_BUNDLE_SPILL_WEIGHT
            }
        } else {
            let mut total = SpillWeight::zero();
            for entry in &self.ctx.bundles[bundle].ranges {
                let range_data = &self.ctx.ranges[entry.index];
                trace!(
                    "  -> uses spill weight: +{:?}",
                    range_data.uses_spill_weight()
                );
                total = total + range_data.uses_spill_weight();
            }

            if self.ctx.bundles[bundle].prio > 0 {
                let final_weight = (total.to_f32() as u32) / self.ctx.bundles[bundle].prio;
                trace!(
                    " -> dividing by prio {}; final weight {}",
                    self.ctx.bundles[bundle].prio,
                    final_weight
                );
                core::cmp::min(BUNDLE_MAX_NORMAL_SPILL_WEIGHT, final_weight)
            } else {
                0
            }
        };

        self.ctx.bundles[bundle].set_cached_spill_weight_and_props(
            spill_weight,
            minimal,
            fixed,
            fixed_def,
        );
    }

    pub fn minimal_bundle(&self, bundle: LiveBundleIndex) -> bool {
        self.ctx.bundles[bundle].cached_minimal()
    }

    pub fn recompute_range_properties(&mut self, range: LiveRangeIndex) {
        let rangedata = &mut self.ctx.ranges[range];
        let mut w = SpillWeight::zero();
        for u in &rangedata.uses {
            w = w + SpillWeight::from_bits(u.weight);
            trace!("range{}: use {:?}", range.index(), u);
        }
        rangedata.set_uses_spill_weight(w);
        if rangedata.uses.len() > 0 && rangedata.uses[0].operand.kind() == OperandKind::Def {
            // Note that we *set* the flag here, but we never *clear*
            // it: it may be set by a progmove as well (which does not
            // create an explicit use or def), and we want to preserve
            // that. We will never split or trim ranges in a way that
            // removes a def at the front and requires the flag to be
            // cleared.
            rangedata.set_flag(LiveRangeFlag::StartsAtDef);
        }
    }

    pub fn get_or_create_spill_bundle(
        &mut self,
        bundle: LiveBundleIndex,
        create_if_absent: bool,
    ) -> Option<LiveBundleIndex> {
        let ssidx = self.ctx.bundles[bundle].spillset;
        let idx = self.ctx.spillsets[ssidx].spill_bundle;
        if idx.is_valid() {
            Some(idx)
        } else if create_if_absent {
            let idx = self.ctx.bundles.add(self.ctx.bump());
            self.ctx.spillsets[ssidx].spill_bundle = idx;
            self.ctx.bundles[idx].spillset = ssidx;
            self.ctx.spilled_bundles.push(idx);
            Some(idx)
        } else {
            None
        }
    }

    pub fn split_and_requeue_bundle(
        &mut self,
        bundle: LiveBundleIndex,
        mut split_at: ProgPoint,
        reg_hint: PReg,
        // Do we trim the parts around the split and put them in the
        // spill bundle?
        mut trim_ends_into_spill_bundle: bool,
    ) {
        self.ctx.output.stats.splits += 1;
        trace!(
            "split bundle {:?} at {:?} and requeue with reg hint (for first part) {:?}",
            bundle,
            split_at,
            reg_hint,
        );

        // Split `bundle` at `split_at`, creating new LiveRanges and
        // bundles (and updating vregs' linked lists appropriately),
        // and enqueue the new bundles.

        let spillset = self.ctx.bundles[bundle].spillset;

        // Have we reached the maximum split count? If so, fall back
        // to a "minimal bundles and spill bundle" setup for this
        // bundle. See the doc-comment on
        // `split_into_minimal_bundles()` above for more.
        if self.ctx.spillsets[spillset].splits >= MAX_SPLITS_PER_SPILLSET {
            self.split_into_minimal_bundles(bundle, reg_hint);
            return;
        }
        self.ctx.spillsets[spillset].splits += 1;

        debug_assert!(!self.ctx.bundles[bundle].ranges.is_empty());
        // Split point *at* start is OK; this means we peel off
        // exactly one use to create a minimal bundle.
        let bundle_start = self.ctx.bundles[bundle].ranges.first().unwrap().range.from;
        debug_assert!(split_at >= bundle_start);
        let bundle_end = self.ctx.bundles[bundle].ranges.last().unwrap().range.to;
        debug_assert!(split_at < bundle_end);

        trace!(
            "bundle_start = {:?} bundle_end = {:?}",
            bundle_start,
            bundle_end
        );

        // Is the bundle at most spanning one instruction? Then
        // there's nothing we can do with this logic (we will not
        // split in the middle of the instruction). Fall back to the
        // minimal-bundle splitting in this case as well.
        if bundle_end.prev().inst() == bundle_start.inst() {
            trace!(" -> spans only one inst; splitting into minimal bundles");
            self.split_into_minimal_bundles(bundle, reg_hint);
            return;
        }

        // Is the split point *at* the start? If so, peel off the
        // program point with the first use: set the split point just
        // after it, or just before it if it comes after the start of
        // the bundle.
        if split_at == bundle_start {
            // Find any uses; if none, just chop off one instruction.
            let mut first_use = None;
            'outer: for entry in &self.ctx.bundles[bundle].ranges {
                for u in &self.ctx.ranges[entry.index].uses {
                    first_use = Some(u.pos);
                    break 'outer;
                }
            }
            trace!(" -> first use loc is {:?}", first_use);
            split_at = match first_use {
                Some(pos) => {
                    if pos.inst() == bundle_start.inst() {
                        ProgPoint::before(pos.inst().next())
                    } else {
                        ProgPoint::before(pos.inst())
                    }
                }
                None => ProgPoint::before(
                    self.ctx.bundles[bundle]
                        .ranges
                        .first()
                        .unwrap()
                        .range
                        .from
                        .inst()
                        .next(),
                ),
            };
            trace!(
                "split point is at bundle start; advancing to {:?}",
                split_at
            );
        } else {
            // Don't split in the middle of an instruction -- this could
            // create impossible moves (we cannot insert a move between an
            // instruction's uses and defs).
            if split_at.pos() == InstPosition::After {
                split_at = split_at.next();
            }
            if split_at >= bundle_end {
                split_at = split_at.prev().prev();
            }
        }

        debug_assert!(split_at > bundle_start && split_at < bundle_end);

        // We need to find which LRs fall on each side of the split,
        // which LR we need to split down the middle, then update the
        // current bundle, create a new one, and (re)-queue both.

        trace!(" -> LRs: {:?}", self.ctx.bundles[bundle].ranges);

        let mut last_lr_in_old_bundle_idx = 0; // last LR-list index in old bundle
        let mut first_lr_in_new_bundle_idx = 0; // first LR-list index in new bundle
        for (i, entry) in self.ctx.bundles[bundle].ranges.iter().enumerate() {
            if split_at > entry.range.from {
                last_lr_in_old_bundle_idx = i;
                first_lr_in_new_bundle_idx = i;
            }
            if split_at < entry.range.to {
                first_lr_in_new_bundle_idx = i;

                // When the bundle contains a fixed constraint, we advance the split point to right
                // before the first instruction with a fixed use present.
                if self.ctx.bundles[bundle].cached_fixed() {
                    for u in &self.ctx.ranges[entry.index].uses {
                        if u.pos < split_at {
                            continue;
                        }

                        if matches!(u.operand.constraint(), OperandConstraint::FixedReg { .. }) {
                            split_at = ProgPoint::before(u.pos.inst());

                            if split_at > entry.range.from {
                                last_lr_in_old_bundle_idx = i;
                            }

                            trace!(" -> advancing split point to {split_at:?}");

                            trim_ends_into_spill_bundle = false;

                            break;
                        }
                    }
                }

                break;
            }
        }

        trace!(
            " -> last LR in old bundle: LR {:?}",
            self.ctx.bundles[bundle].ranges[last_lr_in_old_bundle_idx]
        );
        trace!(
            " -> first LR in new bundle: LR {:?}",
            self.ctx.bundles[bundle].ranges[first_lr_in_new_bundle_idx]
        );

        // Take the sublist of LRs that will go in the new bundle.
        let mut new_lr_list: LiveRangeList = LiveRangeList::new_in(self.ctx.bump());
        new_lr_list.extend(
            self.ctx.bundles[bundle]
                .ranges
                .iter()
                .cloned()
                .skip(first_lr_in_new_bundle_idx),
        );
        self.ctx.bundles[bundle]
            .ranges
            .truncate(last_lr_in_old_bundle_idx + 1);
        self.ctx.bundles[bundle].ranges.shrink_to_fit();

        // If the first entry in `new_lr_list` is a LR that is split
        // down the middle, replace it with a new LR and chop off the
        // end of the same LR in the original list.
        if split_at > new_lr_list[0].range.from {
            debug_assert_eq!(last_lr_in_old_bundle_idx, first_lr_in_new_bundle_idx);
            let orig_lr = new_lr_list[0].index;
            let new_lr = self.ctx.ranges.add(
                CodeRange {
                    from: split_at,
                    to: new_lr_list[0].range.to,
                },
                self.ctx.bump(),
            );
            self.ctx.ranges[new_lr].vreg = self.ranges[orig_lr].vreg;
            trace!(" -> splitting LR {:?} into {:?}", orig_lr, new_lr);
            let first_use = self.ctx.ranges[orig_lr]
                .uses
                .iter()
                .position(|u| u.pos >= split_at)
                .unwrap_or(self.ctx.ranges[orig_lr].uses.len());
            let mut rest_uses = UseList::new_in(self.ctx.bump());
            rest_uses.extend(
                self.ctx.ranges[orig_lr]
                    .uses
                    .iter()
                    .cloned()
                    .skip(first_use),
            );
            self.ctx.ranges[new_lr].uses = rest_uses;
            self.ctx.ranges[orig_lr].uses.truncate(first_use);
            self.ctx.ranges[orig_lr].uses.shrink_to_fit();
            self.recompute_range_properties(orig_lr);
            self.recompute_range_properties(new_lr);
            new_lr_list[0].index = new_lr;
            new_lr_list[0].range = self.ctx.ranges[new_lr].range;
            self.ctx.ranges[orig_lr].range.to = split_at;
            self.ctx.bundles[bundle].ranges[last_lr_in_old_bundle_idx].range =
                self.ctx.ranges[orig_lr].range;

            // Perform a lazy split in the VReg data. We just
            // append the new LR and its range; we will sort by
            // start of range, and fix up range ends, once when we
            // iterate over the VReg's ranges after allocation
            // completes (this is the only time when order
            // matters).
            self.ctx.vregs[self.ctx.ranges[new_lr].vreg]
                .ranges
                .push(LiveRangeListEntry {
                    range: self.ctx.ranges[new_lr].range,
                    index: new_lr,
                });
        }

        let new_bundle = self.ctx.bundles.add(self.ctx.bump());
        trace!(" -> creating new bundle {:?}", new_bundle);
        self.ctx.bundles[new_bundle].spillset = spillset;
        for entry in &new_lr_list {
            self.ctx.ranges[entry.index].bundle = new_bundle;
        }
        self.ctx.bundles[new_bundle].ranges = new_lr_list;

        if trim_ends_into_spill_bundle {
            // Finally, handle moving LRs to the spill bundle when
            // appropriate: If the first range in `new_bundle` or last
            // range in `bundle` has "empty space" beyond the first or
            // last use (respectively), trim it and put an empty LR into
            // the spill bundle.  (We are careful to treat the "starts at
            // def" flag as an implicit first def even if no def-type Use
            // is present.)
            while let Some(entry) = self.ctx.bundles[bundle].ranges.last().cloned() {
                let end = entry.range.to;
                let vreg = self.ctx.ranges[entry.index].vreg;
                let last_use = self.ctx.ranges[entry.index].uses.last().map(|u| u.pos);
                if last_use.is_none() {
                    let spill = self
                        .get_or_create_spill_bundle(bundle, /* create_if_absent = */ true)
                        .unwrap();
                    trace!(
                        " -> bundle {:?} range {:?}: no uses; moving to spill bundle {:?}",
                        bundle,
                        entry.index,
                        spill
                    );
                    self.ctx.bundles[spill].ranges.push(entry);
                    self.ctx.bundles[bundle].ranges.pop();
                    self.ctx.ranges[entry.index].bundle = spill;
                    continue;
                }
                let last_use = last_use.unwrap();
                let split = ProgPoint::before(last_use.inst().next());
                if split < end {
                    let spill = self
                        .get_or_create_spill_bundle(bundle, /* create_if_absent = */ true)
                        .unwrap();
                    self.ctx.bundles[bundle].ranges.last_mut().unwrap().range.to = split;
                    self.ctx.ranges[self.ctx.bundles[bundle].ranges.last().unwrap().index]
                        .range
                        .to = split;
                    let range = CodeRange {
                        from: split,
                        to: end,
                    };
                    let empty_lr = self.ctx.ranges.add(range, self.ctx.bump());
                    self.ctx.bundles[spill].ranges.push(LiveRangeListEntry {
                        range,
                        index: empty_lr,
                    });
                    self.ctx.ranges[empty_lr].bundle = spill;
                    self.ctx.vregs[vreg].ranges.push(LiveRangeListEntry {
                        range,
                        index: empty_lr,
                    });
                    trace!(
                        " -> bundle {:?} range {:?}: last use implies split point {:?}",
                        bundle,
                        entry.index,
                        split
                    );
                    trace!(
                    " -> moving trailing empty region to new spill bundle {:?} with new LR {:?}",
                    spill,
                    empty_lr
                );
                }
                break;
            }
            while let Some(entry) = self.ctx.bundles[new_bundle].ranges.first().cloned() {
                if self.ctx.ranges[entry.index].has_flag(LiveRangeFlag::StartsAtDef) {
                    break;
                }
                let start = entry.range.from;
                let vreg = self.ctx.ranges[entry.index].vreg;
                let first_use = self.ctx.ranges[entry.index].uses.first().map(|u| u.pos);
                if first_use.is_none() {
                    let spill = self
                        .get_or_create_spill_bundle(new_bundle, /* create_if_absent = */ true)
                        .unwrap();
                    trace!(
                        " -> bundle {:?} range {:?}: no uses; moving to spill bundle {:?}",
                        new_bundle,
                        entry.index,
                        spill
                    );
                    self.ctx.bundles[spill].ranges.push(entry);
                    self.ctx.bundles[new_bundle].ranges.drain(..1);
                    self.ctx.ranges[entry.index].bundle = spill;
                    continue;
                }
                let first_use = first_use.unwrap();
                let split = ProgPoint::before(first_use.inst());
                if split > start {
                    let spill = self
                        .get_or_create_spill_bundle(new_bundle, /* create_if_absent = */ true)
                        .unwrap();
                    self.ctx.bundles[new_bundle]
                        .ranges
                        .first_mut()
                        .unwrap()
                        .range
                        .from = split;
                    self.ctx.ranges[self.ctx.bundles[new_bundle].ranges.first().unwrap().index]
                        .range
                        .from = split;
                    let range = CodeRange {
                        from: start,
                        to: split,
                    };
                    let empty_lr = self.ctx.ranges.add(range, self.ctx.bump());
                    self.ctx.bundles[spill].ranges.push(LiveRangeListEntry {
                        range,
                        index: empty_lr,
                    });
                    self.ctx.ranges[empty_lr].bundle = spill;
                    self.ctx.vregs[vreg].ranges.push(LiveRangeListEntry {
                        range,
                        index: empty_lr,
                    });
                    trace!(
                        " -> bundle {:?} range {:?}: first use implies split point {:?}",
                        bundle,
                        entry.index,
                        first_use,
                    );
                    trace!(
                        " -> moving leading empty region to new spill bundle {:?} with new LR {:?}",
                        spill,
                        empty_lr
                    );
                }
                break;
            }
        }

        if self.ctx.bundles[bundle].ranges.len() > 0 {
            self.recompute_bundle_properties(bundle);
            let prio = self.ctx.bundles[bundle].prio;
            self.ctx
                .allocation_queue
                .insert(bundle, prio as usize, reg_hint);
        }
        if self.ctx.bundles[new_bundle].ranges.len() > 0 {
            self.recompute_bundle_properties(new_bundle);
            let prio = self.ctx.bundles[new_bundle].prio;
            self.ctx
                .allocation_queue
                .insert(new_bundle, prio as usize, reg_hint);
        }
    }

    /// Splits the given bundle into minimal bundles per Use, falling
    /// back onto the spill bundle. This must work for any bundle no
    /// matter how many conflicts.
    ///
    /// This is meant to solve a quadratic-cost problem that exists
    /// with "normal" splitting as implemented above. With that
    /// procedure, splitting a bundle produces two
    /// halves. Furthermore, it has cost linear in the length of the
    /// bundle, because the resulting half-bundles have their
    /// requirements recomputed with a new scan, and because we copy
    /// half the use-list over to the tail end sub-bundle.
    ///
    /// This works fine when a bundle has a handful of splits overall,
    /// but not when an input has a systematic pattern of conflicts
    /// that will require O(|bundle|) splits (e.g., every Use is
    /// constrained to a different fixed register than the last
    /// one). In such a case, we get quadratic behavior.
    ///
    /// This method implements a direct split into minimal bundles
    /// along the whole length of the bundle, putting the regions
    /// without uses in the spill bundle. We do this once the number
    /// of splits in an original bundle (tracked by spillset) reaches
    /// a pre-determined limit.
    ///
    /// This basically approximates what a non-splitting allocator
    /// would do: it "spills" the whole bundle to possibly a
    /// stackslot, or a second-chance register allocation at best, via
    /// the spill bundle; and then does minimal reservations of
    /// registers just at uses/defs and moves the "spilled" value
    /// into/out of them immediately.
    pub fn split_into_minimal_bundles(&mut self, bundle: LiveBundleIndex, reg_hint: PReg) {
        assert_eq!(self.ctx.scratch_removed_lrs_vregs.len(), 0);
        self.ctx.scratch_removed_lrs.clear();

        let mut new_lrs: SmallVec<[(VRegIndex, LiveRangeIndex); 16]> = smallvec![];
        let mut new_bundles: SmallVec<[LiveBundleIndex; 16]> = smallvec![];

        let spillset = self.ctx.bundles[bundle].spillset;
        let spill = self
            .get_or_create_spill_bundle(bundle, /* create_if_absent = */ true)
            .unwrap();

        trace!(
            "Splitting bundle {:?} into minimal bundles with reg hint {}",
            bundle,
            reg_hint
        );

        let mut spill_uses = UseList::new_in(self.ctx.bump());

        let empty_vec = LiveRangeList::new_in(self.ctx.bump());
        for entry in core::mem::replace(&mut self.ctx.bundles[bundle].ranges, empty_vec) {
            let vreg = self.ctx.ranges[entry.index].vreg;

            self.ctx.scratch_removed_lrs.insert(entry.index);
            self.ctx.scratch_removed_lrs_vregs.insert(vreg);

            trace!(" -> removing old LR {:?} for vreg {:?}", entry.index, vreg);

            let mut spill_range = entry.range;
            let mut spill_starts_def = false;

            let empty_vec = UseList::new_in(self.ctx.bump());
            for u in core::mem::replace(&mut self.ctx.ranges[entry.index].uses, empty_vec) {
                trace!("   -> use {:?}", u);

                let is_def = u.operand.kind() == OperandKind::Def;

                // If this use has an `any` constraint, eagerly migrate it to the spill range. The
                // reasoning here is that in the second-chance allocation for the spill bundle,
                // any-constrained uses will be easy to satisfy. Solving those constraints earlier
                // could create unnecessary conflicts with existing bundles that need to fit in a
                // register, more strict requirements, so we delay them eagerly.
                if u.operand.constraint() == OperandConstraint::Any {
                    trace!("    -> migrating this any-constrained use to the spill range");
                    spill_uses.push(u);

                    // Remember if we're moving the def of this vreg into the spill range, so that
                    // we can set the appropriate flags on it later.
                    spill_starts_def = spill_starts_def || is_def;

                    continue;
                }

                // If this is a def, make sure that the spill range
                // starts before the next instruction rather than at
                // this position: the value is available only *after*
                // this position.
                if is_def {
                    trace!("    -> moving the spill range forward by one");
                    spill_range.from = ProgPoint::before(u.pos.inst().next());
                }

                // Create a new LR.
                let cr = minimal_range_for_use(&u);
                let lr = self.ctx.ranges.add(cr, self.ctx.bump());
                new_lrs.push((vreg, lr));
                self.ctx.ranges[lr].uses.push(u);
                self.ctx.ranges[lr].vreg = vreg;

                // Create a new bundle that contains only this LR.
                let new_bundle = self.ctx.bundles.add(self.ctx.bump());
                self.ctx.ranges[lr].bundle = new_bundle;
                self.ctx.bundles[new_bundle].spillset = spillset;
                self.ctx.bundles[new_bundle]
                    .ranges
                    .push(LiveRangeListEntry {
                        range: cr,
                        index: lr,
                    });
                new_bundles.push(new_bundle);

                // If this use was a Def, set the StartsAtDef flag for the new LR.
                if is_def {
                    self.ctx.ranges[lr].set_flag(LiveRangeFlag::StartsAtDef);
                }

                trace!(
                    "    -> created new LR {:?} range {:?} with new bundle {:?} for this use",
                    lr,
                    cr,
                    new_bundle
                );
            }

            if !spill_range.is_empty() {
                // Make one entry in the spill bundle that covers the whole range.
                // TODO: it might be worth tracking enough state to only create this LR when there is
                // open space in the original LR.
                let spill_lr = self.ctx.ranges.add(spill_range, self.ctx.bump());
                self.ctx.ranges[spill_lr].vreg = vreg;
                self.ctx.ranges[spill_lr].bundle = spill;
                self.ctx.ranges[spill_lr].uses.extend(spill_uses.drain(..));
                new_lrs.push((vreg, spill_lr));

                if spill_starts_def {
                    self.ctx.ranges[spill_lr].set_flag(LiveRangeFlag::StartsAtDef);
                }

                self.ctx.bundles[spill].ranges.push(LiveRangeListEntry {
                    range: spill_range,
                    index: spill_lr,
                });
                self.ctx.ranges[spill_lr].bundle = spill;
                trace!(
                    "  -> added spill range {:?} in new LR {:?} in spill bundle {:?}",
                    spill_range,
                    spill_lr,
                    spill
                );
            } else {
                assert!(spill_uses.is_empty());
            }
        }

        // Remove all of the removed LRs from respective vregs' lists.
        for vreg in self.ctx.scratch_removed_lrs_vregs.drain() {
            let lrs = &mut self.ctx.scratch_removed_lrs;
            self.ctx.vregs[vreg]
                .ranges
                .retain(|entry| !lrs.contains(&entry.index));
        }

        // Add the new LRs to their respective vreg lists.
        for (vreg, lr) in new_lrs {
            let range = self.ctx.ranges[lr].range;
            let entry = LiveRangeListEntry { range, index: lr };
            self.ctx.vregs[vreg].ranges.push(entry);
        }

        // Recompute bundle properties for all new bundles and enqueue
        // them.
        for bundle in new_bundles {
            if self.ctx.bundles[bundle].ranges.len() > 0 {
                self.recompute_bundle_properties(bundle);
                let prio = self.ctx.bundles[bundle].prio;
                self.ctx
                    .allocation_queue
                    .insert(bundle, prio as usize, reg_hint);
            }
        }
    }

    pub fn process_bundle(
        &mut self,
        bundle: LiveBundleIndex,
        reg_hint: PReg,
    ) -> Result<(), RegAllocError> {
        let class = self.ctx.spillsets[self.bundles[bundle].spillset].class;
        // Grab a hint from either the queue or our spillset, if any.
        let mut hint_reg = if reg_hint != PReg::invalid() {
            reg_hint
        } else {
            self.ctx.spillsets[self.bundles[bundle].spillset].reg_hint
        };
        if self.ctx.pregs[hint_reg.index()].is_stack {
            hint_reg = PReg::invalid();
        }
        trace!("process_bundle: bundle {:?} hint {:?}", bundle, hint_reg,);

        let req = match self.compute_requirement(bundle) {
            Ok(req) => req,
            Err(conflict) => {
                trace!("conflict!: {:?}", conflict);
                // We have to split right away. We'll find a point to
                // split that would allow at least the first half of the
                // split to be conflict-free.
                debug_assert!(
                    !self.minimal_bundle(bundle),
                    "Minimal bundle with conflict!"
                );
                self.split_and_requeue_bundle(
                    bundle,
                    /* split_at_point = */ conflict.suggested_split_point(),
                    reg_hint,
                    /* trim_ends_into_spill_bundle = */
                    conflict.should_trim_edges_around_split(),
                );
                return Ok(());
            }
        };

        // If no requirement at all (because no uses), and *if* a
        // spill bundle is already present, then move the LRs over to
        // the spill bundle right away.
        match req {
            Requirement::Any => {
                if let Some(spill) =
                    self.get_or_create_spill_bundle(bundle, /* create_if_absent = */ false)
                {
                    let empty_vec = LiveRangeList::new_in(self.ctx.bump());
                    let mut list =
                        core::mem::replace(&mut self.ctx.bundles[bundle].ranges, empty_vec);
                    for entry in &list {
                        self.ctx.ranges[entry.index].bundle = spill;
                    }
                    self.ctx.bundles[spill].ranges.extend(list.drain(..));
                    return Ok(());
                }
            }
            _ => {}
        }

        // Try to allocate!
        let mut attempts = 0;
        let mut scratch = core::mem::take(&mut self.ctx.scratch_conflicts);
        let mut lowest_cost_evict_conflict_set = core::mem::take(&mut self.ctx.scratch_bundle);
        'outer: loop {
            attempts += 1;
            trace!("attempt {}, req {:?}", attempts, req);
            debug_assert!(attempts < 100 * self.func.num_insts());

            let fixed_preg = match req {
                Requirement::FixedReg(preg) | Requirement::FixedStack(preg) => Some(preg),
                Requirement::Register => None,

                Requirement::Any => {
                    self.ctx.spilled_bundles.push(bundle);
                    break;
                }
            };
            // Scan all pregs, or the one fixed preg, and attempt to allocate.

            let mut lowest_cost_evict_conflict_cost: Option<u32> = None;
            lowest_cost_evict_conflict_set.clear();

            let mut lowest_cost_split_conflict_cost: Option<u32> = None;
            let mut lowest_cost_split_conflict_point = ProgPoint::before(Inst::new(0));
            let mut lowest_cost_split_conflict_reg = PReg::invalid();

            // Heuristic: start the scan for an available
            // register at an offset influenced both by our
            // location in the code and by the bundle we're
            // considering. This has the effect of spreading
            // demand more evenly across registers.
            let scan_offset = self.ctx.ranges[self.bundles[bundle].ranges[0].index]
                .range
                .from
                .inst()
                .index()
                + bundle.index();

            self.ctx.output.stats.process_bundle_reg_probe_start_any += 1;
            for preg in RegTraversalIter::new(
                self.env,
                class,
                hint_reg,
                PReg::invalid(),
                scan_offset,
                fixed_preg,
            ) {
                self.ctx.output.stats.process_bundle_reg_probes_any += 1;
                let preg_idx = PRegIndex::new(preg.index());
                trace!("trying preg {:?}", preg_idx);

                let scan_limit_cost = match (
                    lowest_cost_evict_conflict_cost,
                    lowest_cost_split_conflict_cost,
                ) {
                    (Some(a), Some(b)) => Some(core::cmp::max(a, b)),
                    _ => None,
                };
                match self.try_to_allocate_bundle_to_reg(
                    bundle,
                    preg_idx,
                    scan_limit_cost,
                    &mut scratch,
                ) {
                    AllocRegResult::Allocated(alloc) => {
                        self.ctx.output.stats.process_bundle_reg_success_any += 1;
                        trace!(" -> allocated to any {:?}", preg_idx);
                        self.ctx.spillsets[self.ctx.bundles[bundle].spillset].reg_hint =
                            alloc.as_reg().unwrap();
                        // Success, return scratch memory to context and finish
                        break 'outer;
                    }
                    AllocRegResult::Conflict(bundles, first_conflict_point) => {
                        trace!(
                            " -> conflict with bundles {:?}, first conflict at {:?}",
                            bundles,
                            first_conflict_point
                        );

                        let conflict_cost = self.maximum_spill_weight_in_bundle_set(bundles);

                        if lowest_cost_evict_conflict_cost.is_none()
                            || conflict_cost < lowest_cost_evict_conflict_cost.unwrap()
                        {
                            lowest_cost_evict_conflict_cost = Some(conflict_cost);
                            lowest_cost_evict_conflict_set.clear();
                            lowest_cost_evict_conflict_set.extend(bundles);
                        }

                        let loop_depth =
                            self.ctx.cfginfo.approx_loop_depth[self.ctx.cfginfo.insn_block
                                [first_conflict_point.inst().index()]
                            .index()];
                        let move_cost = spill_weight_from_constraint(
                            OperandConstraint::Reg,
                            loop_depth as usize,
                            /* is_def = */ true,
                        )
                        .to_int();
                        if lowest_cost_split_conflict_cost.is_none()
                            || (conflict_cost + move_cost)
                                < lowest_cost_split_conflict_cost.unwrap()
                        {
                            lowest_cost_split_conflict_cost = Some(conflict_cost + move_cost);
                            lowest_cost_split_conflict_point = first_conflict_point;
                            lowest_cost_split_conflict_reg = preg;
                        }
                    }
                    AllocRegResult::ConflictWithFixed(max_cost, point) => {
                        trace!(" -> conflict with fixed alloc; cost of other bundles up to point is {}, conflict at {:?}", max_cost, point);

                        let loop_depth = self.ctx.cfginfo.approx_loop_depth
                            [self.ctx.cfginfo.insn_block[point.inst().index()].index()];
                        let move_cost = spill_weight_from_constraint(
                            OperandConstraint::Reg,
                            loop_depth as usize,
                            /* is_def = */ true,
                        )
                        .to_int();

                        if lowest_cost_split_conflict_cost.is_none()
                            || (max_cost + move_cost) < lowest_cost_split_conflict_cost.unwrap()
                        {
                            lowest_cost_split_conflict_cost = Some(max_cost + move_cost);
                            lowest_cost_split_conflict_point = point;
                            lowest_cost_split_conflict_reg = preg;
                        }
                    }
                    AllocRegResult::ConflictHighCost => {
                        // Simply don't consider -- we already have
                        // a lower-cost conflict bundle option
                        // to evict.
                        continue;
                    }
                }
            }

            // Otherwise, we *require* a register, but didn't fit into
            // any with current bundle assignments. Hence, we will need
            // to either split or attempt to evict some bundles.

            trace!(
                " -> lowest cost evict: set {:?}, cost {:?}",
                lowest_cost_evict_conflict_set,
                lowest_cost_evict_conflict_cost,
            );
            trace!(
                " -> lowest cost split: cost {:?}, point {:?}, reg {:?}",
                lowest_cost_split_conflict_cost,
                lowest_cost_split_conflict_point,
                lowest_cost_split_conflict_reg
            );

            // If we reach here, we *must* have an option either to split or evict.
            debug_assert!(
                lowest_cost_split_conflict_cost.is_some()
                    || lowest_cost_evict_conflict_cost.is_some()
            );

            let our_spill_weight = self.bundle_spill_weight(bundle);
            trace!(" -> our spill weight: {}", our_spill_weight);

            // We detect the "too-many-live-registers" case here and
            // return an error cleanly, rather than panicking, because
            // the regalloc.rs fuzzer depends on the register
            // allocator to correctly reject impossible-to-allocate
            // programs in order to discard invalid test cases.
            if self.minimal_bundle(bundle)
                && (attempts >= 2
                    || lowest_cost_evict_conflict_cost.is_none()
                    || lowest_cost_evict_conflict_cost.unwrap() >= our_spill_weight)
            {
                if let Requirement::Register = req {
                    // Check if this is a too-many-live-registers situation.
                    let range = self.ctx.bundles[bundle].ranges[0].range;
                    trace!("checking for too many live regs");
                    let mut min_bundles_assigned = 0;
                    let mut fixed_assigned = 0;
                    let mut total_regs = 0;
                    for preg in self.env.preferred_regs_by_class[class as u8 as usize]
                        .iter()
                        .chain(self.env.non_preferred_regs_by_class[class as u8 as usize].iter())
                    {
                        trace!(" -> PR {:?}", preg);
                        let start = LiveRangeKey::from_range(&CodeRange {
                            from: range.from.prev(),
                            to: range.from.prev(),
                        });
                        for (key, lr) in self.ctx.pregs[preg.index()]
                            .allocations
                            .btree
                            .range(start..)
                        {
                            let preg_range = key.to_range();
                            if preg_range.to <= range.from {
                                continue;
                            }
                            if preg_range.from >= range.to {
                                break;
                            }
                            if lr.is_valid() {
                                if self.minimal_bundle(self.ranges[*lr].bundle) {
                                    trace!("  -> min bundle {:?}", lr);
                                    min_bundles_assigned += 1;
                                } else {
                                    trace!("  -> non-min bundle {:?}", lr);
                                }
                            } else {
                                trace!("  -> fixed bundle");
                                fixed_assigned += 1;
                            }
                        }
                        total_regs += 1;
                    }
                    trace!(
                        " -> total {}, fixed {}, min {}",
                        total_regs,
                        fixed_assigned,
                        min_bundles_assigned
                    );
                    if min_bundles_assigned + fixed_assigned >= total_regs {
                        return Err(RegAllocError::TooManyLiveRegs);
                    }
                }

                panic!("Could not allocate minimal bundle, but the allocation problem should be possible to solve");
            }

            // If our bundle's weight is less than or equal to(*) the
            // evict cost, choose to split.  Also pick splitting if
            // we're on our second or more attempt and we didn't
            // allocate.  Also pick splitting if the conflict set is
            // empty, meaning a fixed conflict that can't be evicted.
            //
            // (*) the "equal to" part is very important: it prevents
            // an infinite loop where two bundles with equal spill
            // cost continually evict each other in an infinite
            // allocation loop. In such a case, the first bundle in
            // wins, and the other splits.
            //
            // Note that we don't split if the bundle is minimal.
            if !self.minimal_bundle(bundle)
                && (attempts >= 2
                    || lowest_cost_evict_conflict_cost.is_none()
                    || our_spill_weight <= lowest_cost_evict_conflict_cost.unwrap())
            {
                trace!(
                    " -> deciding to split: our spill weight is {}",
                    self.bundle_spill_weight(bundle)
                );
                let bundle_start = self.ctx.bundles[bundle].ranges[0].range.from;
                let mut split_at_point =
                    core::cmp::max(lowest_cost_split_conflict_point, bundle_start);
                let requeue_with_reg = lowest_cost_split_conflict_reg;

                // Adjust `split_at_point` if it is within a deeper loop
                // than the bundle start -- hoist it to just before the
                // first loop header it encounters.
                let bundle_start_depth = self.ctx.cfginfo.approx_loop_depth
                    [self.ctx.cfginfo.insn_block[bundle_start.inst().index()].index()];
                let split_at_depth = self.ctx.cfginfo.approx_loop_depth
                    [self.ctx.cfginfo.insn_block[split_at_point.inst().index()].index()];
                if split_at_depth > bundle_start_depth {
                    for block in (self.ctx.cfginfo.insn_block[bundle_start.inst().index()].index()
                        + 1)
                        ..=self.ctx.cfginfo.insn_block[split_at_point.inst().index()].index()
                    {
                        if self.ctx.cfginfo.approx_loop_depth[block] > bundle_start_depth {
                            split_at_point = self.ctx.cfginfo.block_entry[block];
                            break;
                        }
                    }
                }

                self.split_and_requeue_bundle(
                    bundle,
                    split_at_point,
                    requeue_with_reg,
                    /* should_trim = */ true,
                );

                // Success, return scratch memory to context and finish
                break 'outer;
            } else {
                // Evict all bundles in `conflicting bundles` and try again.
                self.ctx.output.stats.evict_bundle_event += 1;
                for &bundle in &lowest_cost_evict_conflict_set {
                    trace!(" -> evicting {:?}", bundle);
                    self.evict_bundle(bundle);
                    self.ctx.output.stats.evict_bundle_count += 1;
                }
            }
        }

        self.ctx.scratch_conflicts = scratch;
        self.ctx.scratch_bundle = lowest_cost_evict_conflict_set;
        return Ok(());
    }
}

/// Compute the minimal range that covers a given use in a minimal
/// bundle. This definition needs to be consistent between bundle
/// property computation and minimal-range splitting (fallback
/// splitting).
///
/// The cases are:
/// - early def: whole instruction
/// - late def: Late only
/// - early use: Early only
/// - late use: whole instruction
fn minimal_range_for_use(u: &Use) -> CodeRange {
    let early = ProgPoint::before(u.pos.inst());
    let late = ProgPoint::after(u.pos.inst());
    let next_early = ProgPoint::before(u.pos.inst().next());
    match (u.pos.pos(), u.operand.kind()) {
        (InstPosition::Before, OperandKind::Def) => CodeRange {
            from: early,
            to: next_early,
        },
        (InstPosition::Before, OperandKind::Use) => CodeRange {
            from: early,
            to: late,
        },
        (InstPosition::After, OperandKind::Def) => CodeRange {
            from: late,
            to: next_early,
        },
        (InstPosition::After, OperandKind::Use) => CodeRange {
            from: early,
            to: next_early,
        },
    }
}
