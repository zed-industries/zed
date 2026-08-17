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

//! Data structures for backtracking allocator.

use super::liveranges::SpillWeight;
use crate::cfg::{CFGInfo, CFGInfoCtx};
use crate::index::ContainerComparator;
use crate::indexset::IndexSet;
use crate::Vec2;
use crate::{
    define_index, Allocation, Block, Bump, Edit, Function, FxHashMap, FxHashSet, MachineEnv,
    Operand, Output, PReg, ProgPoint, RegClass, VReg,
};
use alloc::collections::BTreeMap;
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt::Debug;
use core::ops::{Deref, DerefMut};
use smallvec::SmallVec;

/// A range from `from` (inclusive) to `to` (exclusive).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeRange {
    pub from: ProgPoint,
    pub to: ProgPoint,
}

impl CodeRange {
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.from >= self.to
    }
    #[inline(always)]
    pub fn contains(&self, other: &Self) -> bool {
        other.from >= self.from && other.to <= self.to
    }
    #[inline(always)]
    pub fn contains_point(&self, other: ProgPoint) -> bool {
        other >= self.from && other < self.to
    }
    #[inline(always)]
    pub fn overlaps(&self, other: &Self) -> bool {
        other.to > self.from && other.from < self.to
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.to.inst().index() - self.from.inst().index()
    }
    /// Returns the range covering just one program point.
    #[inline(always)]
    pub fn singleton(pos: ProgPoint) -> CodeRange {
        CodeRange {
            from: pos,
            to: pos.next(),
        }
    }

    /// Join two [CodeRange] values together, producing a [CodeRange] that includes both.
    #[inline(always)]
    pub fn join(&self, other: CodeRange) -> Self {
        CodeRange {
            from: self.from.min(other.from),
            to: self.to.max(other.to),
        }
    }
}

impl core::cmp::PartialOrd for CodeRange {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl core::cmp::Ord for CodeRange {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.to <= other.from {
            Ordering::Less
        } else if self.from >= other.to {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

define_index!(LiveBundleIndex, LiveBundles, LiveBundle);
define_index!(LiveRangeIndex, LiveRanges, LiveRange);
define_index!(SpillSetIndex, SpillSets, SpillSet);
define_index!(UseIndex);
define_index!(VRegIndex, VRegs, VRegData);
define_index!(PRegIndex);
define_index!(SpillSlotIndex);

/// Used to carry small sets of bundles, e.g. for conflict sets.
pub type LiveBundleVec = Vec<LiveBundleIndex>;

#[derive(Clone, Copy, Debug)]
pub struct LiveRangeListEntry {
    pub range: CodeRange,
    pub index: LiveRangeIndex,
}

pub type LiveRangeList = Vec2<LiveRangeListEntry, Bump>;
pub type UseList = Vec2<Use, Bump>;

#[derive(Clone, Debug)]
pub struct LiveRange {
    pub range: CodeRange,

    pub vreg: VRegIndex,
    pub bundle: LiveBundleIndex,
    pub uses_spill_weight_and_flags: u32,
    pub(crate) uses: UseList,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum LiveRangeFlag {
    StartsAtDef = 1,
}

impl LiveRange {
    #[inline(always)]
    pub fn set_flag(&mut self, flag: LiveRangeFlag) {
        self.uses_spill_weight_and_flags |= (flag as u32) << 29;
    }
    #[inline(always)]
    pub fn clear_flag(&mut self, flag: LiveRangeFlag) {
        self.uses_spill_weight_and_flags &= !((flag as u32) << 29);
    }
    #[inline(always)]
    pub fn assign_flag(&mut self, flag: LiveRangeFlag, val: bool) {
        let bit = if val { (flag as u32) << 29 } else { 0 };
        self.uses_spill_weight_and_flags &= 0xe000_0000;
        self.uses_spill_weight_and_flags |= bit;
    }
    #[inline(always)]
    pub fn has_flag(&self, flag: LiveRangeFlag) -> bool {
        self.uses_spill_weight_and_flags & ((flag as u32) << 29) != 0
    }
    #[inline(always)]
    pub fn flag_word(&self) -> u32 {
        self.uses_spill_weight_and_flags & 0xe000_0000
    }
    #[inline(always)]
    pub fn merge_flags(&mut self, flag_word: u32) {
        self.uses_spill_weight_and_flags |= flag_word;
    }
    #[inline(always)]
    pub fn uses_spill_weight(&self) -> SpillWeight {
        // NOTE: the spill weight is technically stored in 29 bits, but we ignore the sign bit as
        // we will always be dealing with positive values. Thus we mask out the top 3 bits to
        // ensure that the sign bit is clear, then shift left by only two.
        let bits = (self.uses_spill_weight_and_flags & 0x1fff_ffff) << 2;
        SpillWeight::from_f32(f32::from_bits(bits))
    }
    #[inline(always)]
    pub fn set_uses_spill_weight(&mut self, weight: SpillWeight) {
        let weight_bits = (weight.to_f32().to_bits() >> 2) & 0x1fff_ffff;
        self.uses_spill_weight_and_flags =
            (self.uses_spill_weight_and_flags & 0xe000_0000) | weight_bits;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Use {
    pub operand: Operand,
    pub pos: ProgPoint,
    pub slot: u16,
    pub weight: u16,
}

impl Use {
    #[inline(always)]
    pub fn new(operand: Operand, pos: ProgPoint, slot: u16) -> Self {
        Self {
            operand,
            pos,
            slot,
            // Weight is updated on insertion into LR.
            weight: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LiveBundle {
    pub(crate) ranges: LiveRangeList,
    pub spillset: SpillSetIndex,
    pub allocation: Allocation,
    pub prio: u32, // recomputed after every bulk update
    pub spill_weight_and_props: u32,
}

pub const BUNDLE_MAX_SPILL_WEIGHT: u32 = (1 << 29) - 1;
pub const MINIMAL_FIXED_BUNDLE_SPILL_WEIGHT: u32 = BUNDLE_MAX_SPILL_WEIGHT;
pub const MINIMAL_BUNDLE_SPILL_WEIGHT: u32 = BUNDLE_MAX_SPILL_WEIGHT - 1;
pub const BUNDLE_MAX_NORMAL_SPILL_WEIGHT: u32 = BUNDLE_MAX_SPILL_WEIGHT - 2;

impl LiveBundle {
    #[inline(always)]
    pub fn set_cached_spill_weight_and_props(
        &mut self,
        spill_weight: u32,
        minimal: bool,
        fixed: bool,
        fixed_def: bool,
    ) {
        debug_assert!(spill_weight <= BUNDLE_MAX_SPILL_WEIGHT);
        self.spill_weight_and_props = spill_weight
            | (if minimal { 1 << 31 } else { 0 })
            | (if fixed { 1 << 30 } else { 0 })
            | (if fixed_def { 1 << 29 } else { 0 });
    }

    #[inline(always)]
    pub fn cached_minimal(&self) -> bool {
        self.spill_weight_and_props & (1 << 31) != 0
    }

    #[inline(always)]
    pub fn cached_fixed(&self) -> bool {
        self.spill_weight_and_props & (1 << 30) != 0
    }

    #[inline(always)]
    pub fn cached_fixed_def(&self) -> bool {
        self.spill_weight_and_props & (1 << 29) != 0
    }

    #[inline(always)]
    pub fn set_cached_fixed(&mut self) {
        self.spill_weight_and_props |= 1 << 30;
    }

    #[inline(always)]
    pub fn set_cached_fixed_def(&mut self) {
        self.spill_weight_and_props |= 1 << 29;
    }

    #[inline(always)]
    pub fn cached_spill_weight(&self) -> u32 {
        self.spill_weight_and_props & BUNDLE_MAX_SPILL_WEIGHT
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BundleProperties {
    pub minimal: bool,
    pub fixed: bool,
}

/// Calculate the maximum `N` inline capacity for a `SmallVec<[T; N]>` we can
/// have without bloating its size to be larger than a `Vec<T>`.
const fn no_bloat_capacity<T>() -> usize {
    // `Vec<T>` is three words: `(pointer, capacity, length)`.
    //
    // A `SmallVec<[T; N]>` replaces the first two members with the following:
    //
    //     union {
    //         Inline([T; N]),
    //         Heap(pointer, capacity),
    //     }
    //
    // So if `size_of([T; N]) == size_of(pointer) + size_of(capacity)` then we
    // get the maximum inline capacity without bloat.
    core::mem::size_of::<usize>() * 2 / core::mem::size_of::<T>()
}

#[derive(Clone, Debug)]
pub struct SpillSet {
    pub slot: SpillSlotIndex,
    pub reg_hint: PReg,
    pub class: RegClass,
    pub spill_bundle: LiveBundleIndex,
    pub required: bool,
    pub splits: u8,

    /// The aggregate [`CodeRange`] of all involved [`LiveRange`]s. The effect of this abstraction
    /// is that we attempt to allocate one spill slot for the extent of a bundle. For fragmented
    /// bundles with lots of open space this abstraction is pessimistic, but when bundles are small
    /// or dense this yields similar results to tracking individual live ranges.
    pub range: CodeRange,
}

pub(crate) const MAX_SPLITS_PER_SPILLSET: u8 = 2;

#[derive(Clone, Debug)]
pub struct VRegData {
    pub(crate) ranges: LiveRangeList,
    pub blockparam: Block,
    // We don't initially know the RegClass until we observe a use of the VReg.
    pub class: Option<RegClass>,
}

#[derive(Clone, Debug)]
pub struct PRegData {
    pub allocations: LiveRangeSet,
    pub is_stack: bool,
}

#[derive(Clone, Debug)]
pub struct MultiFixedRegFixup {
    pub pos: ProgPoint,
    pub from_slot: u16,
    pub to_slot: u16,
    pub level: FixedRegFixupLevel,
    pub to_preg: PRegIndex,
    pub vreg: VRegIndex,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FixedRegFixupLevel {
    /// A fixup copy for the initial fixed reg; must come first.
    Initial,
    /// A fixup copy from the first fixed reg to other fixed regs for
    /// the same vreg; must come second.
    Secondary,
}

/// The field order is significant: these are sorted so that a
/// scan over vregs, then blocks in each range, can scan in
/// order through this (sorted) list and add allocs to the
/// half-move list.
///
/// The fields in this struct are reversed in sort order so that the entire
/// struct can be treated as a u128 for sorting purposes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct BlockparamOut {
    pub to_vreg: VRegIndex,
    pub to_block: Block,
    pub from_block: Block,
    pub from_vreg: VRegIndex,
}
impl BlockparamOut {
    #[inline(always)]
    pub fn key(&self) -> u128 {
        u128_key(
            self.from_vreg.raw_u32(),
            self.from_block.raw_u32(),
            self.to_block.raw_u32(),
            self.to_vreg.raw_u32(),
        )
    }
}

/// As above for `BlockparamIn`, field order is significant.
///
/// The fields in this struct are reversed in sort order so that the entire
/// struct can be treated as a u128 for sorting purposes.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct BlockparamIn {
    pub from_block: Block,
    pub to_block: Block,
    pub to_vreg: VRegIndex,
}
impl BlockparamIn {
    #[inline(always)]
    pub fn key(&self) -> u128 {
        u128_key(
            self.to_vreg.raw_u32(),
            self.to_block.raw_u32(),
            self.from_block.raw_u32(),
            0,
        )
    }
}

impl LiveRanges {
    pub(crate) fn add(&mut self, range: CodeRange, bump: Bump) -> LiveRangeIndex {
        self.push(LiveRange {
            range,
            vreg: VRegIndex::invalid(),
            bundle: LiveBundleIndex::invalid(),
            uses_spill_weight_and_flags: 0,
            uses: UseList::new_in(bump),
        })
    }
}

impl LiveBundles {
    pub(crate) fn add(&mut self, bump: Bump) -> LiveBundleIndex {
        self.push(LiveBundle {
            allocation: Allocation::none(),
            ranges: LiveRangeList::new_in(bump),
            spillset: SpillSetIndex::invalid(),
            prio: 0,
            spill_weight_and_props: 0,
        })
    }
}

impl VRegs {
    pub fn add(&mut self, reg: VReg, data: VRegData) -> VRegIndex {
        let idx = self.push(data);
        debug_assert_eq!(reg.vreg(), idx.index());
        idx
    }
}

impl core::ops::Index<VReg> for VRegs {
    type Output = VRegData;

    #[inline(always)]
    fn index(&self, idx: VReg) -> &Self::Output {
        &self.storage[idx.vreg()]
    }
}

impl core::ops::IndexMut<VReg> for VRegs {
    #[inline(always)]
    fn index_mut(&mut self, idx: VReg) -> &mut Self::Output {
        &mut self.storage[idx.vreg()]
    }
}

#[derive(Default)]
pub struct Ctx {
    pub(crate) cfginfo: CFGInfo,
    pub(crate) cfginfo_ctx: CFGInfoCtx,
    pub(crate) liveins: Vec<IndexSet>,
    pub(crate) liveouts: Vec<IndexSet>,
    pub(crate) blockparam_outs: Vec<BlockparamOut>,
    pub(crate) blockparam_ins: Vec<BlockparamIn>,

    pub(crate) ranges: LiveRanges,
    pub(crate) bundles: LiveBundles,
    pub(crate) spillsets: SpillSets,
    pub(crate) vregs: VRegs,
    pub(crate) pregs: Vec<PRegData>,
    pub(crate) allocation_queue: PrioQueue,

    pub(crate) spilled_bundles: Vec<LiveBundleIndex>,
    pub(crate) spillslots: Vec<SpillSlotData>,
    pub(crate) slots_by_class: [SpillSlotList; 3],

    pub(crate) extra_spillslots_by_class: [SmallVec<[Allocation; 2]>; 3],
    pub(crate) preferred_victim_by_class: [PReg; 3],

    // When multiple fixed-register constraints are present on a
    // single VReg at a single program point (this can happen for,
    // e.g., call args that use the same value multiple times), we
    // remove all but one of the fixed-register constraints, make a
    // note here, and add a clobber with that PReg instread to keep
    // the register available. When we produce the final edit-list, we
    // will insert a copy from wherever the VReg's primary allocation
    // was to the approprate PReg.
    pub(crate) multi_fixed_reg_fixups: Vec<MultiFixedRegFixup>,

    pub(crate) allocated_bundle_count: usize,

    // For debug output only: a list of textual annotations at every
    // ProgPoint to insert into the final allocated program listing.
    pub(crate) debug_annotations: FxHashMap<ProgPoint, Vec<String>>,
    pub(crate) annotations_enabled: bool,

    // Cached allocation for `try_to_allocate_bundle_to_reg` to avoid allocating
    // a new HashSet on every call.
    pub(crate) conflict_set: FxHashSet<LiveBundleIndex>,

    // Output:
    pub output: Output,

    pub(crate) scratch_conflicts: LiveBundleVec,
    pub(crate) scratch_bundle: LiveBundleVec,
    pub(crate) scratch_vreg_ranges: Vec<LiveRangeIndex>,
    pub(crate) scratch_spillset_pool: Vec<SpillSetRanges>,

    pub(crate) scratch_workqueue: VecDeque<Block>,

    pub(crate) scratch_operand_rewrites: FxHashMap<usize, Operand>,
    pub(crate) scratch_removed_lrs: FxHashSet<LiveRangeIndex>,
    pub(crate) scratch_removed_lrs_vregs: FxHashSet<VRegIndex>,
    pub(crate) scratch_workqueue_set: FxHashSet<Block>,

    pub(crate) scratch_bump: Bump,
}

impl Ctx {
    pub(crate) fn bump(&self) -> Bump {
        self.scratch_bump.clone()
    }
}

pub struct Env<'a, F: Function> {
    pub func: &'a F,
    pub env: &'a MachineEnv,
    pub ctx: &'a mut Ctx,
}

impl<'a, F: Function> Deref for Env<'a, F> {
    type Target = Ctx;

    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl<'a, F: Function> DerefMut for Env<'a, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ctx
    }
}

impl<'a, F: Function> Env<'a, F> {
    /// Get the VReg (with bundled RegClass) from a vreg index.
    #[inline]
    pub fn vreg(&self, index: VRegIndex) -> VReg {
        let class = self.vregs[index]
            .class
            .expect("trying to get a VReg before observing its class");
        VReg::new(index.index(), class)
    }

    /// Record the class of a VReg. We learn this only when we observe
    /// the VRegs in use.
    pub fn observe_vreg_class(&mut self, vreg: VReg) {
        let old_class = self.vregs[vreg].class.replace(vreg.class());
        // We should never observe two different classes for two
        // mentions of a VReg in the source program.
        debug_assert!(old_class == None || old_class == Some(vreg.class()));
    }

    /// Is this vreg actually used in the source program?
    pub fn is_vreg_used(&self, index: VRegIndex) -> bool {
        self.vregs[index].class.is_some()
    }
}

#[derive(Clone, Debug, Default)]
pub struct SpillSetRanges {
    pub btree: BTreeMap<LiveRangeKey, SpillSetIndex>,
}

#[derive(Clone, Debug)]
pub struct SpillSlotData {
    pub ranges: SpillSetRanges,
    pub slots: u32,
    pub alloc: Allocation,
}

#[derive(Clone, Debug, Default)]
pub struct SpillSlotList {
    pub slots: SmallVec<[SpillSlotIndex; 32]>,
    pub probe_start: usize,
}

impl SpillSlotList {
    /// Get the next spillslot index in probing order, wrapping around
    /// at the end of the slots list.
    pub(crate) fn next_index(&self, index: usize) -> usize {
        debug_assert!(index < self.slots.len());
        if index == self.slots.len() - 1 {
            0
        } else {
            index + 1
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PrioQueue {
    pub heap: alloc::collections::BinaryHeap<PrioQueueEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrioQueueEntry {
    pub prio: u32,
    pub bundle: LiveBundleIndex,
    pub reg_hint: PReg,
}

#[derive(Clone, Debug)]
pub struct LiveRangeSet {
    pub btree: BTreeMap<LiveRangeKey, LiveRangeIndex>,
}

#[derive(Clone, Copy, Debug)]
pub struct LiveRangeKey {
    pub from: u32,
    pub to: u32,
}

impl LiveRangeKey {
    #[inline(always)]
    pub fn from_range(range: &CodeRange) -> Self {
        Self {
            from: range.from.to_index(),
            to: range.to.to_index(),
        }
    }

    #[inline(always)]
    pub fn to_range(&self) -> CodeRange {
        CodeRange {
            from: ProgPoint::from_index(self.from),
            to: ProgPoint::from_index(self.to),
        }
    }
}

impl core::cmp::PartialEq for LiveRangeKey {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.to > other.from && self.from < other.to
    }
}
impl core::cmp::Eq for LiveRangeKey {}
impl core::cmp::PartialOrd for LiveRangeKey {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl core::cmp::Ord for LiveRangeKey {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.to <= other.from {
            core::cmp::Ordering::Less
        } else if self.from >= other.to {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

pub struct PrioQueueComparator<'a> {
    pub prios: &'a [usize],
}
impl<'a> ContainerComparator for PrioQueueComparator<'a> {
    type Ix = LiveBundleIndex;
    fn compare(&self, a: Self::Ix, b: Self::Ix) -> core::cmp::Ordering {
        self.prios[a.index()].cmp(&self.prios[b.index()])
    }
}

impl PrioQueue {
    #[inline(always)]
    pub fn insert(&mut self, bundle: LiveBundleIndex, prio: usize, reg_hint: PReg) {
        self.heap.push(PrioQueueEntry {
            prio: prio as u32,
            bundle,
            reg_hint,
        });
    }

    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.heap.is_empty()
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<(LiveBundleIndex, PReg)> {
        self.heap.pop().map(|entry| (entry.bundle, entry.reg_hint))
    }
}

impl LiveRangeSet {
    pub(crate) fn new() -> Self {
        Self {
            btree: BTreeMap::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InsertedMove {
    pub pos_prio: PosWithPrio,
    pub from_alloc: Allocation,
    pub to_alloc: Allocation,
    pub to_vreg: VReg,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InsertMovePrio {
    InEdgeMoves,
    Regular,
    MultiFixedRegInitial,
    MultiFixedRegSecondary,
    ReusedInput,
    OutEdgeMoves,
}

#[derive(Debug, Default)]
pub struct InsertedMoves {
    pub moves: Vec<InsertedMove>,
}

impl InsertedMoves {
    pub fn push(
        &mut self,
        pos: ProgPoint,
        prio: InsertMovePrio,
        from_alloc: Allocation,
        to_alloc: Allocation,
        to_vreg: VReg,
    ) {
        trace!(
            "insert_move: pos {:?} prio {:?} from_alloc {:?} to_alloc {:?} to_vreg {:?}",
            pos,
            prio,
            from_alloc,
            to_alloc,
            to_vreg
        );
        if from_alloc == to_alloc {
            trace!(" -> skipping move with same source and  dest");
            return;
        }
        if let Some(from) = from_alloc.as_reg() {
            debug_assert_eq!(from.class(), to_vreg.class());
        }
        if let Some(to) = to_alloc.as_reg() {
            debug_assert_eq!(to.class(), to_vreg.class());
        }
        self.moves.push(InsertedMove {
            pos_prio: PosWithPrio {
                pos,
                prio: prio as u32,
            },
            from_alloc,
            to_alloc,
            to_vreg,
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct Edits {
    edits: Vec<(PosWithPrio, Edit)>,
}

impl Edits {
    #[inline(always)]
    pub fn with_capacity(n: usize) -> Self {
        Self {
            edits: Vec::with_capacity(n),
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.edits.len()
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &(PosWithPrio, Edit)> {
        self.edits.iter()
    }

    #[inline(always)]
    pub fn drain_edits(&mut self) -> impl Iterator<Item = (ProgPoint, Edit)> + '_ {
        self.edits.drain(..).map(|(pos, edit)| (pos.pos, edit))
    }

    /// Sort edits by the combination of their program position and priority. This is a stable sort
    /// to preserve the order of the moves the parallel move resolver inserts.
    #[inline(always)]
    pub fn sort(&mut self) {
        self.edits.sort_by_key(|&(pos_prio, _)| pos_prio.key());
    }

    pub fn add(&mut self, pos_prio: PosWithPrio, from: Allocation, to: Allocation) {
        if from != to {
            if from.is_reg() && to.is_reg() {
                debug_assert_eq!(from.as_reg().unwrap().class(), to.as_reg().unwrap().class());
            }
            self.edits.push((pos_prio, Edit::Move { from, to }));
        }
    }
}

/// The fields in this struct are reversed in sort order so that the entire
/// struct can be treated as a u64 for sorting purposes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct PosWithPrio {
    pub prio: u32,
    pub pos: ProgPoint,
}

impl PosWithPrio {
    #[inline]
    pub fn key(self) -> u64 {
        u64_key(self.pos.to_index(), self.prio)
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "enable-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stats {
    pub livein_blocks: usize,
    pub livein_iterations: usize,
    pub initial_liverange_count: usize,
    pub merged_bundle_count: usize,
    pub process_bundle_count: usize,
    pub process_bundle_reg_probes_fixed: usize,
    pub process_bundle_reg_success_fixed: usize,
    pub process_bundle_bounding_range_probe_start_any: usize,
    pub process_bundle_bounding_range_probes_any: usize,
    pub process_bundle_bounding_range_success_any: usize,
    pub process_bundle_reg_probe_start_any: usize,
    pub process_bundle_reg_probes_any: usize,
    pub process_bundle_reg_success_any: usize,
    pub evict_bundle_event: usize,
    pub evict_bundle_count: usize,
    pub splits: usize,
    pub splits_clobbers: usize,
    pub splits_hot: usize,
    pub splits_conflicts: usize,
    pub splits_defs: usize,
    pub splits_all: usize,
    pub final_liverange_count: usize,
    pub final_bundle_count: usize,
    pub spill_bundle_count: usize,
    pub spill_bundle_reg_probes: usize,
    pub spill_bundle_reg_success: usize,
    pub blockparam_ins_count: usize,
    pub blockparam_outs_count: usize,
    pub halfmoves_count: usize,
    pub edits_count: usize,
}

// Helper function for generating sorting keys. The order of arguments is from
// the most significant field to the least significant one.
//
// These work best when the fields are stored in reverse order in memory so that
// they can be loaded with a single u64 load on little-endian machines.
#[inline(always)]
pub fn u64_key(b: u32, a: u32) -> u64 {
    a as u64 | (b as u64) << 32
}
#[inline(always)]
pub fn u128_key(d: u32, c: u32, b: u32, a: u32) -> u128 {
    a as u128 | (b as u128) << 32 | (c as u128) << 64 | (d as u128) << 96
}
