//! Support for safepoints and stack maps.

use super::*;
use crate::{HashMap, HashSet};
use core::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
#[repr(u8)]
enum SlotSize {
    Size8 = 0,
    Size16 = 1,
    Size32 = 2,
    Size64 = 3,
    Size128 = 4,
    // If adding support for more slot sizes, update `SLOT_SIZE_LEN` below.
}
const SLOT_SIZE_LEN: usize = 5;

impl TryFrom<ir::Type> for SlotSize {
    type Error = &'static str;

    fn try_from(ty: ir::Type) -> Result<Self, Self::Error> {
        Self::new(ty.bytes()).ok_or("type is not supported in stack maps")
    }
}

impl SlotSize {
    fn new(bytes: u32) -> Option<Self> {
        match bytes {
            1 => Some(Self::Size8),
            2 => Some(Self::Size16),
            4 => Some(Self::Size32),
            8 => Some(Self::Size64),
            16 => Some(Self::Size128),
            _ => None,
        }
    }

    fn unwrap_new(bytes: u32) -> Self {
        Self::new(bytes).unwrap_or_else(|| panic!("cannot create a `SlotSize` for {bytes} bytes"))
    }
}

/// A map from every `SlotSize` to a `T`.
struct SlotSizeMap<T>([T; SLOT_SIZE_LEN]);

impl<T> Default for SlotSizeMap<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<SlotSize> for SlotSizeMap<T> {
    type Output = T;
    fn index(&self, index: SlotSize) -> &Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<SlotSize> for SlotSizeMap<T> {
    fn index_mut(&mut self, index: SlotSize) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<T> SlotSizeMap<T> {
    fn new() -> Self
    where
        T: Default,
    {
        Self([
            T::default(),
            T::default(),
            T::default(),
            T::default(),
            T::default(),
        ])
    }

    fn clear(&mut self)
    where
        T: Default,
    {
        *self = Self::new();
    }

    fn get(&self, size: SlotSize) -> &T {
        let index = size as u8 as usize;
        &self.0[index]
    }

    fn get_mut(&mut self, size: SlotSize) -> &mut T {
        let index = size as u8 as usize;
        &mut self.0[index]
    }
}

/// A set of live values.
///
/// Make sure to copy to a vec and sort, or something, before iterating over the
/// values to ensure deterministic output.
type LiveSet = HashSet<ir::Value>;

/// A worklist of blocks' post-order indices that we need to process.
#[derive(Default)]
struct Worklist {
    /// Stack of blocks to process.
    stack: Vec<u32>,

    /// The set of blocks that need to be updated.
    ///
    /// This is a subset of the elements present in `self.stack`, *not* the
    /// exact same elements. `self.stack` is allowed to have duplicates, and
    /// once we pop the first occurrence of a duplicate, we remove it from this
    /// set, since it no longer needs updates at that point. This potentially
    /// uses more stack space than necessary, but prefers processing immediate
    /// predecessors, and therefore inner loop bodies before continuing to
    /// process outer loop bodies. This ultimately results in fewer iterations
    /// required to reach a fixed point.
    need_updates: HashSet<u32>,
}

impl Extend<u32> for Worklist {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = u32>,
    {
        for block_index in iter {
            self.push(block_index);
        }
    }
}

impl Worklist {
    fn clear(&mut self) {
        let Worklist {
            stack,
            need_updates,
        } = self;
        stack.clear();
        need_updates.clear();
    }

    fn reserve(&mut self, capacity: usize) {
        let Worklist {
            stack,
            need_updates,
        } = self;
        stack.reserve(capacity);
        need_updates.reserve(capacity);
    }

    fn push(&mut self, block_index: u32) {
        // Mark this block as needing an update. If it wasn't in `self.stack`,
        // now it is and it needs an update. If it was already in `self.stack`,
        // then pushing this copy logically hoists it to the top of the
        // stack. See the above note about processing inner-most loops first.
        self.need_updates.insert(block_index);
        self.stack.push(block_index);
    }

    fn pop(&mut self) -> Option<u32> {
        while let Some(block_index) = self.stack.pop() {
            // If this block was pushed multiple times, we only need to update
            // it once, so remove it from the need-updates set. In other words
            // it was logically hoisted up to the top of the stack, while this
            // entry was left behind, and we already popped the hoisted
            // copy. See the above note about processing inner-most loops first.
            if self.need_updates.remove(&block_index) {
                return Some(block_index);
            }
        }
        None
    }
}

/// A simple liveness analysis.
///
/// This analysis is used to determine which needs-stack-map values are live
/// across safepoint instructions.
///
/// This is a backwards analysis, from uses (which mark values live) to defs
/// (which remove values from the live set) and from successor blocks to
/// predecessor blocks.
///
/// We compute two live sets for each block:
///
/// 1. The live-in set, which is the set of values that are live when control
///    enters the block.
///
/// 2. The live-out set, which is the set of values that are live when control
///    exits the block.
///
/// A block's live-out set is the union of its successors' live-in sets. A
/// block's live-in set is the set of values that are still live after the
/// block's instructions have been processed.
///
/// ```text
/// live_in(block) = union(live_out(s) for s in successors(block))
/// live_out(block) = live_in(block) - defs(block) + uses(block)
/// ```
///
/// Whenever we update a block's live-in set, we must reprocess all of its
/// predecessors, because those predecessors' live-out sets depend on this
/// block's live-in set. Processing continues until the live sets stop changing
/// and we've reached a fixed-point. Each time we process a block, its live sets
/// can only grow monotonically, and therefore we know that the computation will
/// reach its fixed-point and terminate. This fixed-point is implemented with a
/// classic worklist algorithm.
///
/// The worklist is seeded such that we initially process blocks in post-order,
/// which ensures that, when we have a loop-free control-flow graph, we only
/// process each block once. We pop a block off the worklist for
/// processing. Whenever a block's live-in set is updated during processing, we
/// push its predecessors onto the worklist so that their live-in sets can be
/// updated. Once the worklist is empty, there are no more blocks needing
/// updates, and we've reached the fixed-point.
///
/// Note: For simplicity, we do not flow liveness from block parameters back to
/// branch arguments, and instead always consider branch arguments live.
///
/// Furthermore, we do not differentiate between uses of a needs-stack-map value
/// that ultimately flow into a side-effecting operation versus uses that
/// themselves are not live. This could be tightened up in the future, but we're
/// starting with the easiest, simplest thing. It also means that we do not need
/// `O(all values)` space, only `O(needs-stack-map values)`. Finally, none of
/// our mid-end optimization passes have run at this point in time yet, so there
/// probably isn't much, if any, dead code.
///
/// After we've computed the live-in and -out sets for each block, we pass once
/// more over each block, processing its instructions again. This time, we
/// record the precise set of needs-stack-map values that are live across each
/// safepoint instruction inside the block, which is the final output of this
/// analysis.
pub(crate) struct LivenessAnalysis {
    /// Reusable depth-first search state for traversing a function's blocks.
    dfs: Dfs,

    /// The cached post-order traversal of the function's blocks.
    post_order: Vec<ir::Block>,

    /// A secondary map from each block to its index in `post_order`.
    block_to_index: SecondaryMap<ir::Block, u32>,

    /// A mapping from each block's post-order index to the post-order indices
    /// of its direct (non-transitive) predecessors.
    predecessors: Vec<SmallVec<[u32; 4]>>,

    /// A worklist of blocks to process. Used to determine which blocks need
    /// updates cascaded to them and when we reach a fixed-point.
    worklist: Worklist,

    /// A map from a block's post-order index to its live-in set.
    live_ins: Vec<LiveSet>,

    /// A map from a block's post-order index to its live-out set.
    live_outs: Vec<LiveSet>,

    /// The set of each needs-stack-map value that is currently live while
    /// processing a block.
    currently_live: LiveSet,

    /// A mapping from each safepoint instruction to the set of needs-stack-map
    /// values that are live across it.
    safepoints: HashMap<ir::Inst, SmallVec<[ir::Value; 4]>>,

    /// The set of values that are live across *any* safepoint in the function,
    /// i.e. the union of all the values in the `safepoints` map.
    live_across_any_safepoint: EntitySet<ir::Value>,
}

impl Default for LivenessAnalysis {
    fn default() -> Self {
        Self {
            dfs: Default::default(),
            post_order: Default::default(),
            block_to_index: SecondaryMap::with_default(u32::MAX),
            predecessors: Default::default(),
            worklist: Default::default(),
            live_ins: Default::default(),
            live_outs: Default::default(),
            currently_live: Default::default(),
            safepoints: Default::default(),
            live_across_any_safepoint: Default::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RecordSafepoints {
    Yes,
    No,
}

impl LivenessAnalysis {
    /// Clear and reset all internal state such that this analysis is ready for
    /// reuse with a new function.
    pub fn clear(&mut self) {
        let LivenessAnalysis {
            dfs,
            post_order,
            block_to_index,
            predecessors,
            worklist,
            live_ins,
            live_outs,
            currently_live,
            safepoints,
            live_across_any_safepoint,
        } = self;
        dfs.clear();
        post_order.clear();
        block_to_index.clear();
        predecessors.clear();
        worklist.clear();
        live_ins.clear();
        live_outs.clear();
        currently_live.clear();
        safepoints.clear();
        live_across_any_safepoint.clear();
    }

    /// Given that we've initialized `self.post_order`, reserve capacity for the
    /// various data structures we use during our analysis.
    fn reserve_capacity(&mut self, func: &Function) {
        let LivenessAnalysis {
            dfs: _,
            post_order,
            block_to_index,
            predecessors,
            worklist,
            live_ins,
            live_outs,
            currently_live: _,
            safepoints: _,
            live_across_any_safepoint: _,
        } = self;

        block_to_index.resize(func.dfg.num_blocks());

        let capacity = post_order.len();
        worklist.reserve(capacity);
        predecessors.resize(capacity, Default::default());
        live_ins.resize(capacity, Default::default());
        live_outs.resize(capacity, Default::default());
    }

    fn initialize_block_to_index_map(&mut self) {
        for (block_index, block) in self.post_order.iter().enumerate() {
            self.block_to_index[*block] = u32::try_from(block_index).unwrap();
        }
    }

    fn initialize_predecessors_map(&mut self, func: &Function) {
        for (block_index, block) in self.post_order.iter().enumerate() {
            let block_index = u32::try_from(block_index).unwrap();
            for succ in func.block_successors(*block) {
                let succ_index = self.block_to_index[succ];
                debug_assert_ne!(succ_index, u32::MAX);
                let succ_index = usize::try_from(succ_index).unwrap();
                self.predecessors[succ_index].push(block_index);
            }
        }
    }

    /// Process a value's definition, removing it from the currently-live set.
    fn process_def(&mut self, val: ir::Value) {
        if self.currently_live.remove(&val) {
            log::trace!("liveness:   defining {val:?}, removing it from the live set");
        }
    }

    /// Record the live set of needs-stack-map values at the given safepoint.
    fn record_safepoint(&mut self, func: &Function, inst: Inst) {
        log::trace!(
            "liveness:   found safepoint: {inst:?}: {}",
            func.dfg.display_inst(inst)
        );
        log::trace!("liveness:     live set = {:?}", self.currently_live);

        let mut live = self.currently_live.iter().copied().collect::<SmallVec<_>>();
        // Keep order deterministic since we add stack map entries in this
        // order.
        live.sort();

        self.live_across_any_safepoint.extend(live.iter().copied());
        self.safepoints.insert(inst, live);
    }

    /// Process a use of a needs-stack-map value, inserting it into the
    /// currently-live set.
    fn process_use(&mut self, func: &Function, inst: Inst, val: Value) {
        if self.currently_live.insert(val) {
            log::trace!(
                "liveness:   found use of {val:?}, marking it live: {inst:?}: {}",
                func.dfg.display_inst(inst)
            );
        }
    }

    /// Process all the instructions in a block in reverse order.
    fn process_block(
        &mut self,
        func: &mut Function,
        stack_map_values: &EntitySet<ir::Value>,
        block_index: usize,
        record_safepoints: RecordSafepoints,
    ) {
        let block = self.post_order[block_index];
        log::trace!("liveness: traversing {block:?}");

        // Reset the currently-live set to this block's live-out set.
        self.currently_live.clear();
        self.currently_live
            .extend(self.live_outs[block_index].iter().copied());

        // Now process this block's instructions, incrementally building its
        // live-in set inside the currently-live set.
        let mut option_inst = func.layout.last_inst(block);
        while let Some(inst) = option_inst {
            // Process any needs-stack-map values defined by this instruction.
            for val in func.dfg.inst_results(inst) {
                self.process_def(*val);
            }

            // If this instruction is a safepoint and we've been asked to record
            // safepoints, then do so.
            let opcode = func.dfg.insts[inst].opcode();
            if record_safepoints == RecordSafepoints::Yes && opcode.is_safepoint() {
                self.record_safepoint(func, inst);
            }

            // Process any needs-stack-map values used by this instruction.
            for val in func.dfg.inst_values(inst) {
                let val = func.dfg.resolve_aliases(val);
                if stack_map_values.contains(val) {
                    self.process_use(func, inst, val);
                }
            }

            option_inst = func.layout.prev_inst(inst);
        }

        // After we've processed this block's instructions, remove its
        // parameters from the live set. This is part of step (1).
        for val in func.dfg.block_params(block) {
            self.process_def(*val);
        }
    }

    /// Run the liveness analysis on the given function.
    pub fn run(&mut self, func: &mut Function, stack_map_values: &EntitySet<ir::Value>) {
        self.clear();
        self.post_order.extend(self.dfs.post_order_iter(func));
        self.reserve_capacity(func);
        self.initialize_block_to_index_map();
        self.initialize_predecessors_map(func);

        // Initially enqueue all blocks for processing. We push them in reverse
        // post-order (which yields them in post-order when popped) because if
        // there are no back-edges in the control-flow graph, post-order will
        // result in only a single pass over the blocks.
        self.worklist
            .extend((0..u32::try_from(self.post_order.len()).unwrap()).rev());

        // Pump the worklist until we reach a fixed-point.
        while let Some(block_index) = self.worklist.pop() {
            let block_index = usize::try_from(block_index).unwrap();

            // Because our live sets grow monotonically, we just need to see if
            // the size changed to determine whether the whole set changed.
            let initial_live_in_len = self.live_ins[block_index].len();

            // The live-out set for a block is the union of the live-in sets of
            // its successors.
            for successor in func.block_successors(self.post_order[block_index]) {
                let successor_index = self.block_to_index[successor];
                debug_assert_ne!(successor_index, u32::MAX);
                let successor_index = usize::try_from(successor_index).unwrap();
                self.live_outs[block_index].extend(self.live_ins[successor_index].iter().copied());
            }

            // Process the block to compute its live-in set, but do not record
            // safepoints yet, as we haven't yet processed loop back edges (see
            // below).
            self.process_block(func, stack_map_values, block_index, RecordSafepoints::No);

            // The live-in set for a block is the set of values that are still
            // live after the block's instructions have been processed.
            self.live_ins[block_index].extend(self.currently_live.iter().copied());

            // If the live-in set changed, then we need to revisit all this
            // block's predecessors.
            if self.live_ins[block_index].len() != initial_live_in_len {
                self.worklist
                    .extend(self.predecessors[block_index].iter().copied());
            }
        }

        // Once we've reached a fixed-point, compute the actual live set for
        // each safepoint instruction in each block, backwards from the block's
        // live-out set.
        for block_index in 0..self.post_order.len() {
            self.process_block(func, stack_map_values, block_index, RecordSafepoints::Yes);

            debug_assert_eq!(
                self.currently_live, self.live_ins[block_index],
                "when we recompute the live-in set for a block as part of \
                 computing live sets at each safepoint, we should get the same \
                 result we computed in the fixed-point"
            );
        }
    }
}

/// A mapping from each needs-stack-map value to its associated stack slot.
///
/// Internally maintains free lists for stack slots that won't be used again, so
/// that we can reuse them and minimize the number of stack slots we need to
/// allocate.
#[derive(Default)]
struct StackSlots {
    /// A mapping from each needs-stack-map value that is live across some
    /// safepoint to the stack slot that it resides within. Note that if a
    /// needs-stack-map value is never live across a safepoint, then we won't
    /// ever add it to this map, it can remain in a virtual register for the
    /// duration of its lifetime, and we won't replace all its uses with reloads
    /// and all that stuff.
    stack_slots: HashMap<ir::Value, ir::StackSlot>,

    /// A map from slot size to free stack slots that are not being used
    /// anymore. This allows us to reuse stack slots across multiple values
    /// helps cut down on the ultimate size of our stack frames.
    free_stack_slots: SlotSizeMap<SmallVec<[ir::StackSlot; 4]>>,
}

impl StackSlots {
    fn clear(&mut self) {
        let StackSlots {
            stack_slots,
            free_stack_slots,
        } = self;
        stack_slots.clear();
        free_stack_slots.clear();
    }

    fn get(&self, val: ir::Value) -> Option<ir::StackSlot> {
        self.stack_slots.get(&val).copied()
    }

    fn get_or_create_stack_slot(&mut self, func: &mut Function, val: ir::Value) -> ir::StackSlot {
        *self.stack_slots.entry(val).or_insert_with(|| {
            log::trace!("rewriting:     {val:?} needs a stack slot");
            let ty = func.dfg.value_type(val);
            let size = ty.bytes();
            match self.free_stack_slots[SlotSize::unwrap_new(size)].pop() {
                Some(slot) => {
                    log::trace!("rewriting:       reusing free stack slot {slot:?} for {val:?}");
                    slot
                }
                None => {
                    debug_assert!(size.is_power_of_two());
                    let log2_size = size.ilog2();
                    let slot = func.create_sized_stack_slot(ir::StackSlotData::new(
                        ir::StackSlotKind::ExplicitSlot,
                        size,
                        log2_size.try_into().unwrap(),
                    ));
                    log::trace!("rewriting:       created new stack slot {slot:?} for {val:?}");
                    slot
                }
            }
        })
    }

    fn free_stack_slot(&mut self, size: SlotSize, slot: ir::StackSlot) {
        log::trace!("rewriting:     returning {slot:?} to the free list");
        self.free_stack_slots[size].push(slot);
    }
}

/// A pass to rewrite a function's instructions to spill and reload values that
/// are live across safepoints.
///
/// A single `SafepointSpiller` instance may be reused to rewrite many
/// functions, amortizing the cost of its internal allocations and avoiding
/// repeated `malloc` and `free` calls.
#[derive(Default)]
pub(super) struct SafepointSpiller {
    liveness: LivenessAnalysis,
    stack_slots: StackSlots,
}

impl SafepointSpiller {
    /// Clear and reset all internal state such that this pass is ready to run
    /// on a new function.
    pub fn clear(&mut self) {
        let SafepointSpiller {
            liveness,
            stack_slots,
        } = self;
        liveness.clear();
        stack_slots.clear();
    }

    /// Identify needs-stack-map values that are live across safepoints, and
    /// rewrite the function's instructions to spill and reload them as
    /// necessary.
    pub fn run(&mut self, func: &mut Function, stack_map_values: &EntitySet<ir::Value>) {
        log::trace!("values needing inclusion in stack maps: {stack_map_values:?}");
        log::trace!(
            "before inserting safepoint spills and reloads:\n{}",
            func.display()
        );

        self.clear();
        self.liveness.run(func, stack_map_values);
        self.rewrite(func);

        log::trace!(
            "after inserting safepoint spills and reloads:\n{}",
            func.display()
        );
    }

    /// Spill this value to a stack slot if it has been declared that it must be
    /// included in stack maps and is live across any safepoints.
    ///
    /// The given cursor must point just after this value's definition.
    fn rewrite_def(&mut self, pos: &mut FuncCursor<'_>, val: ir::Value) {
        if let Some(slot) = self.stack_slots.get(val) {
            let i = pos.ins().stack_store(val, slot, 0);
            log::trace!(
                "rewriting:   spilling {val:?} to {slot:?}: {}",
                pos.func.dfg.display_inst(i)
            );

            // Now that we've defined this value, there cannot be any more uses
            // of it, and therefore this stack slot is now available for reuse.
            let ty = pos.func.dfg.value_type(val);
            let size = SlotSize::try_from(ty).unwrap();
            self.stack_slots.free_stack_slot(size, slot);
        }
    }

    /// Add a stack map entry for each needs-stack-map value that is live across
    /// the given safepoint instruction.
    ///
    /// This will additionally assign stack slots to needs-stack-map values, if
    /// no such assignment has already been made.
    fn rewrite_safepoint(&mut self, func: &mut Function, inst: ir::Inst) {
        log::trace!(
            "rewriting:   found safepoint: {inst:?}: {}",
            func.dfg.display_inst(inst)
        );

        let live = self
            .liveness
            .safepoints
            .get(&inst)
            .expect("should only call `rewrite_safepoint` on safepoint instructions");

        for val in live {
            // Get or create the stack slot for this live needs-stack-map value.
            let slot = self.stack_slots.get_or_create_stack_slot(func, *val);

            log::trace!(
                "rewriting:     adding stack map entry for {val:?} at {slot:?}: {}",
                func.dfg.display_inst(inst)
            );
            let ty = func.dfg.value_type(*val);
            func.dfg.append_user_stack_map_entry(
                inst,
                ir::UserStackMapEntry {
                    ty,
                    slot,
                    offset: 0,
                },
            );
        }
    }

    /// If `val` is a needs-stack-map value that has been spilled to a stack
    /// slot, then rewrite `val` to be a load from its associated stack
    /// slot.
    ///
    /// Returns `true` if `val` was rewritten, `false` if not.
    ///
    /// The given cursor must point just before the use of the value that we are
    /// replacing.
    fn rewrite_use(&mut self, pos: &mut FuncCursor<'_>, val: &mut ir::Value) -> bool {
        if !self.liveness.live_across_any_safepoint.contains(*val) {
            return false;
        }

        let old_val = *val;
        log::trace!("rewriting:     found use of {old_val:?}");

        let ty = pos.func.dfg.value_type(*val);
        let slot = self.stack_slots.get_or_create_stack_slot(pos.func, *val);
        *val = pos.ins().stack_load(ty, slot, 0);

        log::trace!(
            "rewriting:     reloading {old_val:?}: {}",
            pos.func
                .dfg
                .display_inst(pos.func.dfg.value_def(*val).unwrap_inst())
        );

        true
    }

    /// Rewrite the function's instructions to spill and reload values that are
    /// live across safepoints:
    ///
    /// 1. Definitions of needs-stack-map values that are live across some
    ///    safepoint need to be spilled to their assigned stack slot.
    ///
    /// 2. Instructions that are themselves safepoints must have stack map
    ///    entries added for the needs-stack-map values that are live across
    ///    them.
    ///
    /// 3. Uses of needs-stack-map values that have been spilled to a stack slot
    ///    need to be replaced with reloads from the slot.
    fn rewrite(&mut self, func: &mut Function) {
        // Shared temporary storage for operand and result lists.
        let mut vals: SmallVec<[_; 8]> = Default::default();

        // Rewrite the function's instructions in post-order. This ensures that
        // we rewrite uses before defs, and therefore once we see a def we know
        // its stack slot will never be used for that value again. Therefore,
        // the slot can be reappropriated for a new needs-stack-map value with a
        // non-overlapping live range. See `rewrite_def` and `free_stack_slots`
        // for more details.
        for block_index in 0..self.liveness.post_order.len() {
            let block = self.liveness.post_order[block_index];
            log::trace!("rewriting: processing {block:?}");

            let mut option_inst = func.layout.last_inst(block);
            while let Some(inst) = option_inst {
                // If this instruction defines a needs-stack-map value that is
                // live across a safepoint, then spill the value to its stack
                // slot.
                let mut pos = FuncCursor::new(func).after_inst(inst);
                vals.extend_from_slice(pos.func.dfg.inst_results(inst));
                for val in vals.drain(..) {
                    self.rewrite_def(&mut pos, val);
                }

                // If this instruction is a safepoint, then we must add stack
                // map entries for the needs-stack-map values that are live
                // across it.
                if self.liveness.safepoints.contains_key(&inst) {
                    self.rewrite_safepoint(func, inst);
                }

                // Replace all uses of needs-stack-map values with loads from
                // the value's associated stack slot.
                let mut pos = FuncCursor::new(func).at_inst(inst);
                vals.extend(pos.func.dfg.inst_values(inst));
                let mut replaced_any = false;
                for val in &mut vals {
                    replaced_any |= self.rewrite_use(&mut pos, val);
                }
                if replaced_any {
                    pos.func.dfg.overwrite_inst_values(inst, vals.drain(..));
                    log::trace!(
                        "rewriting:     updated {inst:?} operands with reloaded values: {}",
                        pos.func.dfg.display_inst(inst)
                    );
                } else {
                    vals.clear();
                }

                option_inst = func.layout.prev_inst(inst);
            }

            // Spill needs-stack-map values defined by block parameters to their
            // associated stack slots.
            let mut pos = FuncCursor::new(func).at_position(CursorPosition::Before(block));
            pos.next_inst(); // Advance to the first instruction in the block.
            vals.clear();
            vals.extend_from_slice(pos.func.dfg.block_params(block));
            for val in vals.drain(..) {
                self.rewrite_def(&mut pos, val);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use cranelift_codegen::isa::CallConv;

    #[test]
    fn needs_stack_map_and_loop() {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));
        sig.params.push(AbiParam::new(ir::types::I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));
        let signature = builder.func.import_signature(sig);
        let func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // Here the value `v1` is technically not live but our single-pass liveness
        // analysis treats every branch argument to a block as live to avoid
        // needing to do a fixed-point loop.
        //
        //     block0(v0, v1):
        //       call $foo(v0)
        //       jump block0(v0, v1)
        let block0 = builder.create_block();
        builder.append_block_params_for_function_params(block0);
        let a = builder.func.dfg.block_params(block0)[0];
        let b = builder.func.dfg.block_params(block0)[1];
        builder.declare_value_needs_stack_map(a);
        builder.declare_value_needs_stack_map(b);
        builder.switch_to_block(block0);
        builder.ins().call(func_ref, &[a]);
        builder.ins().jump(block0, &[a.into(), b.into()]);
        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample(i32, i32) system_v {
    ss0 = explicit_slot 4, align = 4
    ss1 = explicit_slot 4, align = 4
    sig0 = (i32) system_v
    fn0 = colocated u0:0 sig0

block0(v0: i32, v1: i32):
    stack_store v0, ss0
    stack_store v1, ss1
    v4 = stack_load.i32 ss0
    call fn0(v4), stack_map=[i32 @ ss0+0, i32 @ ss1+0]
    v2 = stack_load.i32 ss0
    v3 = stack_load.i32 ss1
    jump block0(v2, v3)
}
            "#
        );
    }

    #[test]
    fn needs_stack_map_simple() {
        let _ = env_logger::try_init();

        let sig = Signature::new(CallConv::SystemV);

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));
        let signature = builder.func.import_signature(sig);
        let func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // At each `call` we are losing one more value as no longer live, so
        // each stack map should be one smaller than the last. `v3` is never
        // live across a safepoint, so should never appear in a stack map. Note
        // that a value that is an argument to the call, but is not live after
        // the call, should not appear in the stack map. This is why `v0`
        // appears in the first call's stack map, but not the second call's
        // stack map.
        //
        //     block0:
        //       v0 = needs stack map
        //       v1 = needs stack map
        //       v2 = needs stack map
        //       v3 = needs stack map
        //       call $foo(v3)
        //       call $foo(v0)
        //       call $foo(v1)
        //       call $foo(v2)
        //       return
        let block0 = builder.create_block();
        builder.append_block_params_for_function_params(block0);
        builder.switch_to_block(block0);
        let v0 = builder.ins().iconst(ir::types::I32, 0);
        builder.declare_value_needs_stack_map(v0);
        let v1 = builder.ins().iconst(ir::types::I32, 1);
        builder.declare_value_needs_stack_map(v1);
        let v2 = builder.ins().iconst(ir::types::I32, 2);
        builder.declare_value_needs_stack_map(v2);
        let v3 = builder.ins().iconst(ir::types::I32, 3);
        builder.declare_value_needs_stack_map(v3);
        builder.ins().call(func_ref, &[v3]);
        builder.ins().call(func_ref, &[v0]);
        builder.ins().call(func_ref, &[v1]);
        builder.ins().call(func_ref, &[v2]);
        builder.ins().return_(&[]);
        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample() system_v {
    ss0 = explicit_slot 4, align = 4
    ss1 = explicit_slot 4, align = 4
    ss2 = explicit_slot 4, align = 4
    sig0 = (i32) system_v
    fn0 = colocated u0:0 sig0

block0:
    v0 = iconst.i32 0
    stack_store v0, ss2  ; v0 = 0
    v1 = iconst.i32 1
    stack_store v1, ss1  ; v1 = 1
    v2 = iconst.i32 2
    stack_store v2, ss0  ; v2 = 2
    v3 = iconst.i32 3
    call fn0(v3), stack_map=[i32 @ ss2+0, i32 @ ss1+0, i32 @ ss0+0]  ; v3 = 3
    v6 = stack_load.i32 ss2
    call fn0(v6), stack_map=[i32 @ ss1+0, i32 @ ss0+0]
    v5 = stack_load.i32 ss1
    call fn0(v5), stack_map=[i32 @ ss0+0]
    v4 = stack_load.i32 ss0
    call fn0(v4)
    return
}
            "#
        );
    }

    #[test]
    fn needs_stack_map_and_post_order_early_return() {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // Here we rely on the post-order to make sure that we never visit block
        // 4 and add `v1` to our live set, then visit block 2 and add `v1` to
        // its stack map even though `v1` is not in scope. Thanksfully, that
        // sequence is impossible because it would be an invalid post-order
        // traversal. The only valid post-order traversals are [3, 1, 2, 0] and
        // [2, 3, 1, 0].
        //
        //     block0(v0):
        //       brif v0, block1, block2
        //
        //     block1:
        //       <stuff>
        //       v1 = get some gc ref
        //       jump block3
        //
        //     block2:
        //       call $needs_safepoint_accidentally
        //       return
        //
        //     block3:
        //       stuff keeping v1 live
        //       return
        let block0 = builder.create_block();
        let block1 = builder.create_block();
        let block2 = builder.create_block();
        let block3 = builder.create_block();
        builder.append_block_params_for_function_params(block0);

        builder.switch_to_block(block0);
        let v0 = builder.func.dfg.block_params(block0)[0];
        builder.ins().brif(v0, block1, &[], block2, &[]);

        builder.switch_to_block(block1);
        let v1 = builder.ins().iconst(ir::types::I64, 0x12345678);
        builder.declare_value_needs_stack_map(v1);
        builder.ins().jump(block3, &[]);

        builder.switch_to_block(block2);
        builder.ins().call(func_ref, &[]);
        builder.ins().return_(&[]);

        builder.switch_to_block(block3);
        // NB: Our simplistic liveness analysis conservatively treats any use of
        // a value as keeping it live, regardless if the use has side effects or
        // is otherwise itself live, so an `iadd_imm` suffices to keep `v1` live
        // here.
        builder.ins().iadd_imm(v1, 0);
        builder.ins().return_(&[]);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample(i32) system_v {
    sig0 = () system_v
    fn0 = colocated u0:0 sig0

block0(v0: i32):
    brif v0, block1, block2

block1:
    v1 = iconst.i64 0x1234_5678
    jump block3

block2:
    call fn0()
    return

block3:
    v2 = iadd_imm.i64 v1, 0  ; v1 = 0x1234_5678
    return
}
            "#
        );
    }

    #[test]
    fn needs_stack_map_conditional_branches_and_liveness() {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // We should not have a stack map entry for `v1` in block 1 because it
        // is not live across the call.
        //
        //     block0(v0):
        //       v1 = needs stack map
        //       brif v0, block1, block2
        //
        //     block1:
        //       call $foo()
        //       return
        //
        //     block2:
        //       keep v1 alive
        //       return
        let block0 = builder.create_block();
        let block1 = builder.create_block();
        let block2 = builder.create_block();
        builder.append_block_params_for_function_params(block0);

        builder.switch_to_block(block0);
        let v0 = builder.func.dfg.block_params(block0)[0];
        let v1 = builder.ins().iconst(ir::types::I64, 0x12345678);
        builder.declare_value_needs_stack_map(v1);
        builder.ins().brif(v0, block1, &[], block2, &[]);

        builder.switch_to_block(block1);
        builder.ins().call(func_ref, &[]);
        builder.ins().return_(&[]);

        builder.switch_to_block(block2);
        // NB: Our simplistic liveness analysis conservatively treats any use of
        // a value as keeping it live, regardless if the use has side effects or
        // is otherwise itself live, so an `iadd_imm` suffices to keep `v1` live
        // here.
        builder.ins().iadd_imm(v1, 0);
        builder.ins().return_(&[]);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample(i32) system_v {
    sig0 = () system_v
    fn0 = colocated u0:0 sig0

block0(v0: i32):
    v1 = iconst.i64 0x1234_5678
    brif v0, block1, block2

block1:
    call fn0()
    return

block2:
    v2 = iadd_imm.i64 v1, 0  ; v1 = 0x1234_5678
    return
}
            "#
        );

        // Now do the same test but with block 1 and 2 swapped so that we
        // exercise what we are trying to exercise, regardless of which
        // post-order traversal we happen to take.
        func.clear();
        fn_ctx.clear();

        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));

        func.signature = sig;
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        let block0 = builder.create_block();
        let block1 = builder.create_block();
        let block2 = builder.create_block();
        builder.append_block_params_for_function_params(block0);

        builder.switch_to_block(block0);
        let v0 = builder.func.dfg.block_params(block0)[0];
        let v1 = builder.ins().iconst(ir::types::I64, 0x12345678);
        builder.declare_value_needs_stack_map(v1);
        builder.ins().brif(v0, block1, &[], block2, &[]);

        builder.switch_to_block(block1);
        builder.ins().iadd_imm(v1, 0);
        builder.ins().return_(&[]);

        builder.switch_to_block(block2);
        builder.ins().call(func_ref, &[]);
        builder.ins().return_(&[]);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function u0:0(i32) system_v {
    sig0 = () system_v
    fn0 = colocated u0:0 sig0

block0(v0: i32):
    v1 = iconst.i64 0x1234_5678
    brif v0, block1, block2

block1:
    v2 = iadd_imm.i64 v1, 0  ; v1 = 0x1234_5678
    return

block2:
    call fn0()
    return
}
            "#
        );
    }

    #[test]
    fn needs_stack_map_and_tail_calls() {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // Depending on which post-order traversal we take, we might consider
        // `v1` live inside `block1`. But nothing is live after a tail call so
        // we shouldn't spill `v1` either way here.
        //
        //     block0(v0):
        //       v1 = needs stack map
        //       brif v0, block1, block2
        //
        //     block1:
        //       return_call $foo()
        //
        //     block2:
        //       keep v1 alive
        //       return
        let block0 = builder.create_block();
        let block1 = builder.create_block();
        let block2 = builder.create_block();
        builder.append_block_params_for_function_params(block0);

        builder.switch_to_block(block0);
        let v0 = builder.func.dfg.block_params(block0)[0];
        let v1 = builder.ins().iconst(ir::types::I64, 0x12345678);
        builder.declare_value_needs_stack_map(v1);
        builder.ins().brif(v0, block1, &[], block2, &[]);

        builder.switch_to_block(block1);
        builder.ins().return_call(func_ref, &[]);

        builder.switch_to_block(block2);
        // NB: Our simplistic liveness analysis conservatively treats any use of
        // a value as keeping it live, regardless if the use has side effects or
        // is otherwise itself live, so an `iadd_imm` suffices to keep `v1` live
        // here.
        builder.ins().iadd_imm(v1, 0);
        builder.ins().return_(&[]);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample(i32) system_v {
    sig0 = () system_v
    fn0 = colocated u0:0 sig0

block0(v0: i32):
    v1 = iconst.i64 0x1234_5678
    brif v0, block1, block2

block1:
    return_call fn0()

block2:
    v2 = iadd_imm.i64 v1, 0  ; v1 = 0x1234_5678
    return
}
            "#
        );

        // Do the same test but with block 1 and 2 swapped so that we exercise
        // what we are trying to exercise, regardless of which post-order
        // traversal we happen to take.
        func.clear();
        fn_ctx.clear();

        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));
        func.signature = sig;

        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        let block0 = builder.create_block();
        let block1 = builder.create_block();
        let block2 = builder.create_block();
        builder.append_block_params_for_function_params(block0);

        builder.switch_to_block(block0);
        let v0 = builder.func.dfg.block_params(block0)[0];
        let v1 = builder.ins().iconst(ir::types::I64, 0x12345678);
        builder.declare_value_needs_stack_map(v1);
        builder.ins().brif(v0, block1, &[], block2, &[]);

        builder.switch_to_block(block1);
        builder.ins().iadd_imm(v1, 0);
        builder.ins().return_(&[]);

        builder.switch_to_block(block2);
        builder.ins().return_call(func_ref, &[]);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function u0:0(i32) system_v {
    sig0 = () system_v
    fn0 = colocated u0:0 sig0

block0(v0: i32):
    v1 = iconst.i64 0x1234_5678
    brif v0, block1, block2

block1:
    v2 = iadd_imm.i64 v1, 0  ; v1 = 0x1234_5678
    return

block2:
    return_call fn0()
}
            "#
        );
    }

    #[test]
    fn needs_stack_map_and_cfg_diamond() {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // Create an if/else CFG diamond that and check that various things get
        // spilled as needed.
        //
        //     block0(v0):
        //       brif v0, block1, block2
        //
        //     block1:
        //       v1 = needs stack map
        //       v2 = needs stack map
        //       call $foo()
        //       jump block3(v1, v2)
        //
        //     block2:
        //       v3 = needs stack map
        //       v4 = needs stack map
        //       call $foo()
        //       jump block3(v3, v3)  ;; Note: v4 is not live
        //
        //     block3(v5, v6):
        //       call $foo()
        //       keep v5 alive, but not v6
        let block0 = builder.create_block();
        let block1 = builder.create_block();
        let block2 = builder.create_block();
        let block3 = builder.create_block();
        builder.append_block_params_for_function_params(block0);

        builder.switch_to_block(block0);
        let v0 = builder.func.dfg.block_params(block0)[0];
        builder.ins().brif(v0, block1, &[], block2, &[]);

        builder.switch_to_block(block1);
        let v1 = builder.ins().iconst(ir::types::I64, 1);
        builder.declare_value_needs_stack_map(v1);
        let v2 = builder.ins().iconst(ir::types::I64, 2);
        builder.declare_value_needs_stack_map(v2);
        builder.ins().call(func_ref, &[]);
        builder.ins().jump(block3, &[v1.into(), v2.into()]);

        builder.switch_to_block(block2);
        let v3 = builder.ins().iconst(ir::types::I64, 3);
        builder.declare_value_needs_stack_map(v3);
        let v4 = builder.ins().iconst(ir::types::I64, 4);
        builder.declare_value_needs_stack_map(v4);
        builder.ins().call(func_ref, &[]);
        builder.ins().jump(block3, &[v3.into(), v3.into()]);

        builder.switch_to_block(block3);
        builder.append_block_param(block3, ir::types::I64);
        builder.append_block_param(block3, ir::types::I64);
        builder.ins().call(func_ref, &[]);
        // NB: Our simplistic liveness analysis conservatively treats any use of
        // a value as keeping it live, regardless if the use has side effects or
        // is otherwise itself live, so an `iadd_imm` suffices to keep `v1` live
        // here.
        builder.ins().iadd_imm(v1, 0);
        builder.ins().return_(&[]);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample(i32) system_v {
    ss0 = explicit_slot 8, align = 8
    ss1 = explicit_slot 8, align = 8
    sig0 = () system_v
    fn0 = colocated u0:0 sig0

block0(v0: i32):
    brif v0, block1, block2

block1:
    v1 = iconst.i64 1
    stack_store v1, ss0  ; v1 = 1
    v2 = iconst.i64 2
    stack_store v2, ss1  ; v2 = 2
    call fn0(), stack_map=[i64 @ ss0+0, i64 @ ss1+0]
    v9 = stack_load.i64 ss0
    v10 = stack_load.i64 ss1
    jump block3(v9, v10)

block2:
    v3 = iconst.i64 3
    stack_store v3, ss0  ; v3 = 3
    v4 = iconst.i64 4
    call fn0(), stack_map=[i64 @ ss0+0, i64 @ ss0+0]
    v11 = stack_load.i64 ss0
    v12 = stack_load.i64 ss0
    jump block3(v11, v12)

block3(v5: i64, v6: i64):
    call fn0(), stack_map=[i64 @ ss0+0]
    v8 = stack_load.i64 ss0
    v7 = iadd_imm v8, 0
    return
}
            "#
        );
    }

    #[test]
    fn needs_stack_map_and_heterogeneous_types() {
        let _ = env_logger::try_init();

        let mut sig = Signature::new(CallConv::SystemV);
        for ty in [
            ir::types::I8,
            ir::types::I16,
            ir::types::I32,
            ir::types::I64,
            ir::types::I128,
            ir::types::F32,
            ir::types::F64,
            ir::types::I8X16,
            ir::types::I16X8,
        ] {
            sig.params.push(AbiParam::new(ty));
            sig.returns.push(AbiParam::new(ty));
        }

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // Test that we support stack maps of heterogeneous types and properly
        // coalesce types into stack slots based on their size.
        //
        //     block0(v0, v1, v2, v3, v4, v5, v6, v7, v8):
        //       call $foo()
        //       return v0, v1, v2, v3, v4, v5, v6, v7, v8
        let block0 = builder.create_block();
        builder.append_block_params_for_function_params(block0);

        builder.switch_to_block(block0);
        let params = builder.func.dfg.block_params(block0).to_vec();
        for val in &params {
            builder.declare_value_needs_stack_map(*val);
        }
        builder.ins().call(func_ref, &[]);
        builder.ins().return_(&params);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample(i8, i16, i32, i64, i128, f32, f64, i8x16, i16x8) -> i8, i16, i32, i64, i128, f32, f64, i8x16, i16x8 system_v {
    ss0 = explicit_slot 1
    ss1 = explicit_slot 2, align = 2
    ss2 = explicit_slot 4, align = 4
    ss3 = explicit_slot 8, align = 8
    ss4 = explicit_slot 16, align = 16
    ss5 = explicit_slot 4, align = 4
    ss6 = explicit_slot 8, align = 8
    ss7 = explicit_slot 16, align = 16
    ss8 = explicit_slot 16, align = 16
    sig0 = () system_v
    fn0 = colocated u0:0 sig0

block0(v0: i8, v1: i16, v2: i32, v3: i64, v4: i128, v5: f32, v6: f64, v7: i8x16, v8: i16x8):
    stack_store v0, ss0
    stack_store v1, ss1
    stack_store v2, ss2
    stack_store v3, ss3
    stack_store v4, ss4
    stack_store v5, ss5
    stack_store v6, ss6
    stack_store v7, ss7
    stack_store v8, ss8
    call fn0(), stack_map=[i8 @ ss0+0, i16 @ ss1+0, i32 @ ss2+0, i64 @ ss3+0, i128 @ ss4+0, f32 @ ss5+0, f64 @ ss6+0, i8x16 @ ss7+0, i16x8 @ ss8+0]
    v9 = stack_load.i8 ss0
    v10 = stack_load.i16 ss1
    v11 = stack_load.i32 ss2
    v12 = stack_load.i64 ss3
    v13 = stack_load.i128 ss4
    v14 = stack_load.f32 ss5
    v15 = stack_load.f64 ss6
    v16 = stack_load.i8x16 ss7
    v17 = stack_load.i16x8 ss8
    return v9, v10, v11, v12, v13, v14, v15, v16, v17
}
            "#
        );
    }

    #[test]
    fn series_of_non_overlapping_live_ranges_needs_stack_map() {
        let sig = Signature::new(CallConv::SystemV);

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let foo_func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 1,
            });
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));
        let signature = builder.func.import_signature(sig);
        let consume_func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // Create a series of needs-stack-map values that do not have
        // overlapping live ranges, but which do appear in stack maps for calls
        // to `$foo`:
        //
        //     block0:
        //       v0 = needs stack map
        //       call $foo()
        //       call consume(v0)
        //       v1 = needs stack map
        //       call $foo()
        //       call consume(v1)
        //       v2 = needs stack map
        //       call $foo()
        //       call consume(v2)
        //       v3 = needs stack map
        //       call $foo()
        //       call consume(v3)
        //       return
        let block0 = builder.create_block();
        builder.append_block_params_for_function_params(block0);
        builder.switch_to_block(block0);
        let v0 = builder.ins().iconst(ir::types::I32, 0);
        builder.declare_value_needs_stack_map(v0);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().call(consume_func_ref, &[v0]);
        let v1 = builder.ins().iconst(ir::types::I32, 1);
        builder.declare_value_needs_stack_map(v1);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().call(consume_func_ref, &[v1]);
        let v2 = builder.ins().iconst(ir::types::I32, 2);
        builder.declare_value_needs_stack_map(v2);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().call(consume_func_ref, &[v2]);
        let v3 = builder.ins().iconst(ir::types::I32, 3);
        builder.declare_value_needs_stack_map(v3);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().call(consume_func_ref, &[v3]);
        builder.ins().return_(&[]);
        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample() system_v {
    ss0 = explicit_slot 4, align = 4
    sig0 = () system_v
    sig1 = (i32) system_v
    fn0 = colocated u0:0 sig0
    fn1 = colocated u0:1 sig1

block0:
    v0 = iconst.i32 0
    stack_store v0, ss0  ; v0 = 0
    call fn0(), stack_map=[i32 @ ss0+0]
    v7 = stack_load.i32 ss0
    call fn1(v7)
    v1 = iconst.i32 1
    stack_store v1, ss0  ; v1 = 1
    call fn0(), stack_map=[i32 @ ss0+0]
    v6 = stack_load.i32 ss0
    call fn1(v6)
    v2 = iconst.i32 2
    stack_store v2, ss0  ; v2 = 2
    call fn0(), stack_map=[i32 @ ss0+0]
    v5 = stack_load.i32 ss0
    call fn1(v5)
    v3 = iconst.i32 3
    stack_store v3, ss0  ; v3 = 3
    call fn0(), stack_map=[i32 @ ss0+0]
    v4 = stack_load.i32 ss0
    call fn1(v4)
    return
}
            "#
        );
    }

    #[test]
    fn vars_block_params_and_needs_stack_map() {
        let _ = env_logger::try_init();

        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));
        sig.returns.push(AbiParam::new(ir::types::I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));
        let signature = builder.func.import_signature(sig);
        let func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // Use a variable, create a control flow diamond so that the variable
        // forces a block parameter on the control join point, and make sure
        // that we get stack maps for all the appropriate uses of the variable
        // in all blocks, as well as that we are reusing stack slots for each of
        // the values.
        //
        //                        block0:
        //                          x := needs stack map
        //                          call $foo(x)
        //                          br_if v0, block1, block2
        //
        //
        //     block1:                                     block2:
        //       call $foo(x)                                call $foo(x)
        //       call $foo(x)                                call $foo(x)
        //       x := new needs stack map                    x := new needs stack map
        //       call $foo(x)                                call $foo(x)
        //       jump block3                                 jump block3
        //
        //
        //                        block3:
        //                          call $foo(x)
        //                          return x

        let x = builder.declare_var(ir::types::I32);
        builder.declare_var_needs_stack_map(x);

        let block0 = builder.create_block();
        let block1 = builder.create_block();
        let block2 = builder.create_block();
        let block3 = builder.create_block();

        builder.append_block_params_for_function_params(block0);
        builder.switch_to_block(block0);
        let v0 = builder.func.dfg.block_params(block0)[0];
        let val = builder.ins().iconst(ir::types::I32, 42);
        builder.def_var(x, val);
        {
            let x = builder.use_var(x);
            builder.ins().call(func_ref, &[x]);
        }
        builder.ins().brif(v0, block1, &[], block2, &[]);

        builder.switch_to_block(block1);
        {
            let x = builder.use_var(x);
            builder.ins().call(func_ref, &[x]);
            builder.ins().call(func_ref, &[x]);
        }
        let val = builder.ins().iconst(ir::types::I32, 36);
        builder.def_var(x, val);
        {
            let x = builder.use_var(x);
            builder.ins().call(func_ref, &[x]);
        }
        builder.ins().jump(block3, &[]);

        builder.switch_to_block(block2);
        {
            let x = builder.use_var(x);
            builder.ins().call(func_ref, &[x]);
            builder.ins().call(func_ref, &[x]);
        }
        let val = builder.ins().iconst(ir::types::I32, 36);
        builder.def_var(x, val);
        {
            let x = builder.use_var(x);
            builder.ins().call(func_ref, &[x]);
        }
        builder.ins().jump(block3, &[]);

        builder.switch_to_block(block3);
        let x = builder.use_var(x);
        builder.ins().call(func_ref, &[x]);
        builder.ins().return_(&[x]);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample(i32) -> i32 system_v {
    ss0 = explicit_slot 4, align = 4
    ss1 = explicit_slot 4, align = 4
    sig0 = (i32) system_v
    fn0 = colocated u0:0 sig0

block0(v0: i32):
    v1 = iconst.i32 42
    v2 -> v1
    v4 -> v1
    stack_store v1, ss0  ; v1 = 42
    v13 = stack_load.i32 ss0
    call fn0(v13), stack_map=[i32 @ ss0+0]
    brif v0, block1, block2

block1:
    call fn0(v2), stack_map=[i32 @ ss0+0]  ; v2 = 42
    call fn0(v2)  ; v2 = 42
    v3 = iconst.i32 36
    stack_store v3, ss0  ; v3 = 36
    v10 = stack_load.i32 ss0
    call fn0(v10), stack_map=[i32 @ ss0+0]
    v9 = stack_load.i32 ss0
    jump block3(v9)

block2:
    call fn0(v4), stack_map=[i32 @ ss0+0]  ; v4 = 42
    call fn0(v4)  ; v4 = 42
    v5 = iconst.i32 36
    stack_store v5, ss1  ; v5 = 36
    v12 = stack_load.i32 ss1
    call fn0(v12), stack_map=[i32 @ ss1+0]
    v11 = stack_load.i32 ss1
    jump block3(v11)

block3(v6: i32):
    stack_store v6, ss0
    v8 = stack_load.i32 ss0
    call fn0(v8), stack_map=[i32 @ ss0+0]
    v7 = stack_load.i32 ss0
    return v7
}
            "#
        );
    }

    #[test]
    fn var_needs_stack_map() {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params
            .push(AbiParam::new(cranelift_codegen::ir::types::I32));
        sig.returns
            .push(AbiParam::new(cranelift_codegen::ir::types::I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let var = builder.declare_var(cranelift_codegen::ir::types::I32);
        builder.declare_var_needs_stack_map(var);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        let block0 = builder.create_block();
        builder.append_block_params_for_function_params(block0);
        builder.switch_to_block(block0);

        let arg = builder.func.dfg.block_params(block0)[0];
        builder.def_var(var, arg);

        builder.ins().call(func_ref, &[]);

        let val = builder.use_var(var);
        builder.ins().return_(&[val]);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample(i32) -> i32 system_v {
    ss0 = explicit_slot 4, align = 4
    sig0 = () system_v
    fn0 = colocated u0:0 sig0

block0(v0: i32):
    stack_store v0, ss0
    call fn0(), stack_map=[i32 @ ss0+0]
    v1 = stack_load.i32 ss0
    return v1
}
            "#
        );
    }

    #[test]
    fn first_inst_defines_needs_stack_map() {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params
            .push(AbiParam::new(cranelift_codegen::ir::types::I32));
        sig.returns
            .push(AbiParam::new(cranelift_codegen::ir::types::I32));
        sig.returns
            .push(AbiParam::new(cranelift_codegen::ir::types::I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // Regression test found via fuzzing in
        // https://github.com/bytecodealliance/wasmtime/pull/8941 involving the
        // combination of cursor positions after we have block parameters that
        // need inclusion in stack maps and when the first instruction in a
        // block defines a value that needs inclusion in stack maps.
        //
        // block0(v0: i32):
        //   v1 = iconst.i32 42
        //   call $foo()
        //   return v0, v1

        let block0 = builder.create_block();
        builder.append_block_params_for_function_params(block0);
        builder.switch_to_block(block0);

        let arg = builder.func.dfg.block_params(block0)[0];
        builder.declare_value_needs_stack_map(arg);

        let val = builder.ins().iconst(ir::types::I32, 42);
        builder.declare_value_needs_stack_map(val);

        builder.ins().call(func_ref, &[]);

        builder.ins().return_(&[arg, val]);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample(i32) -> i32, i32 system_v {
    ss0 = explicit_slot 4, align = 4
    ss1 = explicit_slot 4, align = 4
    sig0 = () system_v
    fn0 = colocated u0:0 sig0

block0(v0: i32):
    stack_store v0, ss0
    v1 = iconst.i32 42
    stack_store v1, ss1  ; v1 = 42
    call fn0(), stack_map=[i32 @ ss0+0, i32 @ ss1+0]
    v2 = stack_load.i32 ss0
    v3 = stack_load.i32 ss1
    return v2, v3
}
            "#
        );
    }

    #[test]
    fn needs_stack_map_and_loops_and_partially_live_values() {
        let _ = env_logger::try_init();

        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func =
            Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig.clone());
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let foo_func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 1,
                index: 1,
            });
        let signature = builder.func.import_signature(sig);
        let bar_func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // Test that we support stack maps in loops and that we properly handle
        // value that are only live for part of the loop body on each iteration,
        // but are live across the whole loop because they will be used again
        // the next iteration. Note that `v0` below, which is a GC value, is not
        // live *within a single iteration of the loop* after the call to `bar`,
        // but is actually live across the whole loop because it will be used
        // again in the *next iteration of the loop*:
        //
        //     block0(v0: i32):
        //       jump block1
        //
        //     block1:
        //       call $foo()
        //       call $bar(v0)
        //       call $foo()
        //       jump block1
        let block0 = builder.create_block();
        let block1 = builder.create_block();
        builder.append_block_params_for_function_params(block0);

        builder.switch_to_block(block0);
        builder.ins().jump(block1, &[]);

        builder.switch_to_block(block1);
        let v0 = builder.func.dfg.block_params(block0)[0];
        builder.declare_value_needs_stack_map(v0);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().call(bar_func_ref, &[v0]);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().jump(block1, &[]);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample(i32) system_v {
    ss0 = explicit_slot 4, align = 4
    sig0 = () system_v
    sig1 = (i32) system_v
    fn0 = colocated u0:0 sig0
    fn1 = colocated u1:1 sig1

block0(v0: i32):
    stack_store v0, ss0
    jump block1

block1:
    call fn0(), stack_map=[i32 @ ss0+0]
    v1 = stack_load.i32 ss0
    call fn1(v1), stack_map=[i32 @ ss0+0]
    call fn0(), stack_map=[i32 @ ss0+0]
    jump block1
}
            "#,
        );
    }

    #[test]
    fn needs_stack_map_and_irreducible_loops() {
        let _ = env_logger::try_init();

        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));
        sig.params.push(AbiParam::new(ir::types::I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let foo_func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 1,
                index: 1,
            });
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));
        let signature = builder.func.import_signature(sig);
        let bar_func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // Test an irreducible loop with multiple entry points, both block1 and
        // block2, in this case:
        //
        //     block0(v0: i32, v1: i32):
        //       brif v0, block1, block2
        //
        //     block1:
        //       jump block3
        //
        //     block2:
        //       jump block4
        //
        //     block3:
        //       call $foo()
        //       call $bar(v1)
        //       call $foo()
        //       jump block2
        //
        //     block4:
        //       call $foo()
        //       call $bar(v1)
        //       call $foo()
        //       jump block1
        let block0 = builder.create_block();
        let block1 = builder.create_block();
        let block2 = builder.create_block();
        let block3 = builder.create_block();
        let block4 = builder.create_block();
        builder.append_block_params_for_function_params(block0);

        builder.switch_to_block(block0);
        let v0 = builder.func.dfg.block_params(block0)[0];
        let v1 = builder.func.dfg.block_params(block0)[1];
        builder.declare_value_needs_stack_map(v1);
        builder.ins().brif(v0, block1, &[], block2, &[]);

        builder.switch_to_block(block1);
        builder.ins().jump(block3, &[]);

        builder.switch_to_block(block2);
        builder.ins().jump(block4, &[]);

        builder.switch_to_block(block3);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().call(bar_func_ref, &[v1]);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().jump(block2, &[]);

        builder.switch_to_block(block4);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().call(bar_func_ref, &[v1]);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().jump(block1, &[]);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample(i32, i32) system_v {
    ss0 = explicit_slot 4, align = 4
    sig0 = () system_v
    sig1 = (i32) system_v
    fn0 = colocated u0:0 sig0
    fn1 = colocated u1:1 sig1

block0(v0: i32, v1: i32):
    stack_store v1, ss0
    brif v0, block1, block2

block1:
    jump block3

block2:
    jump block4

block3:
    call fn0(), stack_map=[i32 @ ss0+0]
    v3 = stack_load.i32 ss0
    call fn1(v3), stack_map=[i32 @ ss0+0]
    call fn0(), stack_map=[i32 @ ss0+0]
    jump block2

block4:
    call fn0(), stack_map=[i32 @ ss0+0]
    v2 = stack_load.i32 ss0
    call fn1(v2), stack_map=[i32 @ ss0+0]
    call fn0(), stack_map=[i32 @ ss0+0]
    jump block1
}
            "#,
        );
    }

    #[test]
    fn needs_stack_map_and_back_edge_to_back_edge() {
        let _ = env_logger::try_init();

        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));
        sig.params.push(AbiParam::new(ir::types::I32));
        sig.params.push(AbiParam::new(ir::types::I32));
        sig.params.push(AbiParam::new(ir::types::I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("sample"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index: 0,
            });
        let signature = builder
            .func
            .import_signature(Signature::new(CallConv::SystemV));
        let foo_func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 1,
                index: 1,
            });
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ir::types::I32));
        let signature = builder.func.import_signature(sig);
        let bar_func_ref = builder.import_function(ir::ExtFuncData {
            name: ir::ExternalName::user(name),
            signature,
            colocated: true,
        });

        // Test that we detect the `block1 -> block2 -> block3 -> block2 ->
        // block1` loop in our liveness analysis and keep `v{0,1,2}` live across
        // the whole loop body.
        //
        //     block0(v0, v1, v2, v3):
        //       jump block1(v3)
        //
        //     block1(v4):
        //       call foo_func_ref()
        //       call bar_func_ref(v0)
        //       call foo_func_ref()
        //       jump block2
        //
        //     block2:
        //       call foo_func_ref()
        //       call bar_func_ref(v1)
        //       call foo_func_ref()
        //       v5 = iadd_imm v4, -1
        //       brif v4, block1(v5), block3
        //
        //     block3:
        //       call foo_func_ref()
        //       call bar_func_ref(v2)
        //       call foo_func_ref()
        //       jump block2

        let block0 = builder.create_block();
        let block1 = builder.create_block();
        let block2 = builder.create_block();
        let block3 = builder.create_block();

        builder.append_block_params_for_function_params(block0);

        builder.switch_to_block(block0);

        let v0 = builder.func.dfg.block_params(block0)[0];
        builder.declare_value_needs_stack_map(v0);
        let v1 = builder.func.dfg.block_params(block0)[1];
        builder.declare_value_needs_stack_map(v1);
        let v2 = builder.func.dfg.block_params(block0)[2];
        builder.declare_value_needs_stack_map(v2);
        let v3 = builder.func.dfg.block_params(block0)[3];

        builder.ins().jump(block1, &[v3.into()]);

        builder.switch_to_block(block1);
        let v4 = builder.append_block_param(block1, ir::types::I32);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().call(bar_func_ref, &[v0]);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().jump(block2, &[]);

        builder.switch_to_block(block2);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().call(bar_func_ref, &[v1]);
        builder.ins().call(foo_func_ref, &[]);
        let v5 = builder.ins().iadd_imm(v4, -1);
        builder.ins().brif(v4, block1, &[v5.into()], block3, &[]);

        builder.switch_to_block(block3);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().call(bar_func_ref, &[v2]);
        builder.ins().call(foo_func_ref, &[]);
        builder.ins().jump(block2, &[]);

        builder.seal_all_blocks();
        builder.finalize();

        assert_eq_output!(
            func.display().to_string(),
            r#"
function %sample(i32, i32, i32, i32) system_v {
    ss0 = explicit_slot 4, align = 4
    ss1 = explicit_slot 4, align = 4
    ss2 = explicit_slot 4, align = 4
    sig0 = () system_v
    sig1 = (i32) system_v
    fn0 = colocated u0:0 sig0
    fn1 = colocated u1:1 sig1

block0(v0: i32, v1: i32, v2: i32, v3: i32):
    stack_store v0, ss0
    stack_store v1, ss1
    stack_store v2, ss2
    jump block1(v3)

block1(v4: i32):
    call fn0(), stack_map=[i32 @ ss0+0, i32 @ ss1+0, i32 @ ss2+0]
    v8 = stack_load.i32 ss0
    call fn1(v8), stack_map=[i32 @ ss0+0, i32 @ ss1+0, i32 @ ss2+0]
    call fn0(), stack_map=[i32 @ ss0+0, i32 @ ss1+0, i32 @ ss2+0]
    jump block2

block2:
    call fn0(), stack_map=[i32 @ ss0+0, i32 @ ss1+0, i32 @ ss2+0]
    v7 = stack_load.i32 ss1
    call fn1(v7), stack_map=[i32 @ ss0+0, i32 @ ss1+0, i32 @ ss2+0]
    call fn0(), stack_map=[i32 @ ss0+0, i32 @ ss1+0, i32 @ ss2+0]
    v5 = iadd_imm.i32 v4, -1
    brif.i32 v4, block1(v5), block3

block3:
    call fn0(), stack_map=[i32 @ ss0+0, i32 @ ss1+0, i32 @ ss2+0]
    v6 = stack_load.i32 ss2
    call fn1(v6), stack_map=[i32 @ ss0+0, i32 @ ss1+0, i32 @ ss2+0]
    call fn0(), stack_map=[i32 @ ss0+0, i32 @ ss1+0, i32 @ ss2+0]
    jump block2
}
            "#,
        );
    }

    fn import_func(
        builder: &mut FunctionBuilder,
        params: impl IntoIterator<Item = ir::Type>,
        results: impl IntoIterator<Item = ir::Type>,
    ) -> ir::FuncRef {
        let index = u32::try_from(builder.func.dfg.ext_funcs.len()).unwrap();

        let name = builder
            .func
            .declare_imported_user_function(ir::UserExternalName {
                namespace: 0,
                index,
            });
        let name = ir::ExternalName::user(name);

        let mut signature = Signature::new(CallConv::SystemV);
        signature
            .params
            .extend(params.into_iter().map(|ty| AbiParam::new(ty)));
        signature
            .returns
            .extend(results.into_iter().map(|ty| AbiParam::new(ty)));
        let signature = builder.func.import_signature(signature);

        builder.import_function(ir::ExtFuncData {
            name,
            signature,
            colocated: true,
        })
    }

    #[test]
    fn issue_10397_stack_map_vars_and_indirect_block_params() {
        let _ = env_logger::try_init();

        let mut sig = Signature::new(CallConv::SystemV);
        if false {
            sig.params.push(AbiParam::new(ir::types::I32));
        }

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(ir::UserFuncName::testcase("f"), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let alloc_struct = import_func(&mut builder, None, Some(ir::types::I32));
        let alloc_array = import_func(&mut builder, None, Some(ir::types::I32));
        let array_init_elem = import_func(&mut builder, Some(ir::types::I32), None);
        let type_of = import_func(&mut builder, Some(ir::types::I32), Some(ir::types::I32));
        let ref_test = import_func(
            &mut builder,
            vec![ir::types::I32, ir::types::I32],
            Some(ir::types::I32),
        );
        let access_array = import_func(&mut builder, Some(ir::types::I32), None);
        let should_continue_inner_loop = import_func(&mut builder, None, Some(ir::types::I32));
        let access_struct = import_func(&mut builder, Some(ir::types::I32), None);
        let should_return = import_func(&mut builder, None, Some(ir::types::I32));

        // This test exercises the combination of declaring stack maps and when
        // we need to insert block params during SSA construction. The following
        // CLIF uses vars in such a way that it will ultimately require that we
        // insert block params. However, some of the inserted block params are
        // only ever used in such a way that they are passed as arguments to
        // *other* blocks. That is, there are regions where we never use them
        // directly, and therefore, if we are only declaring that the variable's
        // values need inclusion in stack maps on direct uses, we will
        // completely miss these inserted block params. That leads to incomplete
        // stack maps, which leads to collecting objects too early, which leads
        // to use-after-free bugs.
        //
        // After inserting safepoint spills and stack map annotations to the
        // following pseudo-CLIF, we should have stack map entries for the
        // `live: {...}` annotations (or an over-approximation of that
        // annotation).
        //
        //     block_entry:
        //       v0 = call alloc_struct()                          ;; live: {}
        //       define var_struct = v0
        //       jump block_outer_loop_head
        //
        //     block_outer_loop_head:
        //       v1 = call alloc_array()                           ;; live: {struct}
        //       define var_array = v1
        //       v2 = iconst.i32 0
        //       jump block_array_init_loop_head(v2)
        //
        //     block_array_init_loop_head(v3):
        //       v4 = iconst.i32 1
        //       v5 = icmp ult v3, v4
        //       br_if v5, block_array_init_loop_body, block_array_init_loop_done
        //
        //     block_array_init_loop_body:
        //       call array_init_elem(v1, v4)                      ;; live: {struct, array}
        //       v6 = iconst.i32 1
        //       v7 = iadd v4, v6
        //       jump block_array_init_loop_head(v7)
        //
        //     block_array_init_loop_done:
        //       jump block_inner_loop_head
        //
        //     block_inner_loop_head:
        //       v8 = use var_array
        //       v9 = iconst.i32 0
        //       v10 = icmp eq v8, v9
        //       br_if v10, block_ref_test_done(v9), block_ref_test_non_null
        //
        //     block_ref_test_non_null:
        //       v11 = call type_of(v8)                            ;; live: {struct, array}
        //       v12 = iconst.i32 0xbeefbeef
        //       v13 = icmp eq v11, v12
        //       v14 = iconst.i31 1
        //       br_if v13, block_ref_test_done(v14), block_ref_test_slow
        //
        //     block_ref_test_slow:
        //       v15 = call ref_test(v8, v12)                      ;; live: {struct, array}
        //       jump block_ref_test_done(v15)
        //
        //     block_ref_test_done(v16):
        //       trapz v16, user1
        //       define var_array = v8
        //       call access_array(v8)                             ;; live: {struct}
        //       v17 = call should_continue_inner_loop()           ;; live: {struct}
        //       br_if v17, block_inner_loop_head, block_after_inner_loop
        //
        //     block_after_inner_loop:
        //       v18 = use var_struct
        //       call access_struct(v18)                           ;; live: {struct}
        //       v19 = call should_return()                        ;; live: {struct}
        //       br_if v19, block_return, block_outer_loop_head
        //
        //     block_return:
        //       return

        let var_struct = builder.declare_var(cranelift_codegen::ir::types::I32);
        builder.declare_var_needs_stack_map(var_struct);

        let var_array = builder.declare_var(cranelift_codegen::ir::types::I32);
        builder.declare_var_needs_stack_map(var_array);

        let block_entry = builder.create_block();
        let block_outer_loop_head = builder.create_block();
        let block_array_init_loop_head = builder.create_block();
        let block_array_init_loop_body = builder.create_block();
        let block_array_init_loop_done = builder.create_block();
        let block_inner_loop_head = builder.create_block();
        let block_ref_test_non_null = builder.create_block();
        let block_ref_test_slow = builder.create_block();
        let block_ref_test_done = builder.create_block();
        let block_after_inner_loop = builder.create_block();
        let block_return = builder.create_block();

        builder.append_block_params_for_function_params(block_entry);
        builder.switch_to_block(block_entry);
        builder.seal_block(block_entry);
        let call_inst = builder.ins().call(alloc_struct, &[]);
        let v0 = builder.func.dfg.first_result(call_inst);
        builder.def_var(var_struct, v0);
        builder.ins().jump(block_outer_loop_head, &[]);

        builder.switch_to_block(block_outer_loop_head);
        let call_inst = builder.ins().call(alloc_array, &[]);
        let v1 = builder.func.dfg.first_result(call_inst);
        builder.def_var(var_array, v1);
        let v2 = builder.ins().iconst(ir::types::I32, 0);
        builder.ins().jump(block_array_init_loop_head, &[v2.into()]);

        builder.switch_to_block(block_array_init_loop_head);
        let v3 = builder.append_block_param(block_array_init_loop_head, ir::types::I32);
        let v4 = builder.ins().iconst(ir::types::I32, 1);
        let v5 = builder
            .ins()
            .icmp(ir::condcodes::IntCC::UnsignedLessThan, v3, v4);
        builder.ins().brif(
            v5,
            block_array_init_loop_body,
            &[],
            block_array_init_loop_done,
            &[],
        );

        builder.switch_to_block(block_array_init_loop_body);
        builder.seal_block(block_array_init_loop_body);
        builder.ins().call(array_init_elem, &[v1, v4]);
        let v6 = builder.ins().iconst(ir::types::I32, 1);
        let v7 = builder.ins().iadd(v4, v6);
        builder.ins().jump(block_array_init_loop_head, &[v7.into()]);
        builder.seal_block(block_array_init_loop_head);

        builder.switch_to_block(block_array_init_loop_done);
        builder.seal_block(block_array_init_loop_done);
        builder.ins().jump(block_inner_loop_head, &[]);

        builder.switch_to_block(block_inner_loop_head);
        let v8 = builder.use_var(var_array);
        let v9 = builder.ins().iconst(ir::types::I32, 0);
        let v10 = builder.ins().icmp(ir::condcodes::IntCC::Equal, v8, v9);
        builder.ins().brif(
            v10,
            block_ref_test_done,
            &[v9.into()],
            block_ref_test_non_null,
            &[],
        );

        builder.switch_to_block(block_ref_test_non_null);
        builder.seal_block(block_ref_test_non_null);
        let call_inst = builder.ins().call(type_of, &[v8]);
        let v11 = builder.func.dfg.first_result(call_inst);
        let v12 = builder.ins().iconst(ir::types::I32, 0xbeefbeef);
        let v13 = builder.ins().icmp(ir::condcodes::IntCC::Equal, v11, v12);
        let v14 = builder.ins().iconst(ir::types::I32, 1);
        builder.ins().brif(
            v13,
            block_ref_test_done,
            &[v14.into()],
            block_ref_test_slow,
            &[],
        );

        builder.switch_to_block(block_ref_test_slow);
        builder.seal_block(block_ref_test_slow);
        let call_inst = builder.ins().call(ref_test, &[v8, v12]);
        let v15 = builder.func.dfg.first_result(call_inst);
        builder.ins().jump(block_ref_test_done, &[v15.into()]);

        builder.switch_to_block(block_ref_test_done);
        let v16 = builder.append_block_param(block_ref_test_done, ir::types::I32);
        builder.seal_block(block_ref_test_done);
        builder.ins().trapz(v16, ir::TrapCode::user(1).unwrap());
        builder.def_var(var_array, v8);
        builder.ins().call(access_array, &[v8]);
        let call_inst = builder.ins().call(should_continue_inner_loop, &[]);
        let v17 = builder.func.dfg.first_result(call_inst);
        builder
            .ins()
            .brif(v17, block_inner_loop_head, &[], block_after_inner_loop, &[]);
        builder.seal_block(block_inner_loop_head);

        builder.switch_to_block(block_after_inner_loop);
        builder.seal_block(block_after_inner_loop);
        let v18 = builder.use_var(var_struct);
        builder.ins().call(access_struct, &[v18]);
        let call_inst = builder.ins().call(should_return, &[]);
        let v19 = builder.func.dfg.first_result(call_inst);
        builder
            .ins()
            .brif(v19, block_return, &[], block_outer_loop_head, &[]);
        builder.seal_block(block_outer_loop_head);

        builder.switch_to_block(block_return);
        builder.seal_block(block_return);
        builder.ins().return_(&[]);

        builder.finalize();
        assert_eq_output!(
            func.display().to_string(),
            r#"
function %f() system_v {
    ss0 = explicit_slot 4, align = 4
    ss1 = explicit_slot 4, align = 4
    ss2 = explicit_slot 4, align = 4
    sig0 = () -> i32 system_v
    sig1 = () -> i32 system_v
    sig2 = (i32) system_v
    sig3 = (i32) -> i32 system_v
    sig4 = (i32, i32) -> i32 system_v
    sig5 = (i32) system_v
    sig6 = () -> i32 system_v
    sig7 = (i32) system_v
    sig8 = () -> i32 system_v
    fn0 = colocated u0:0 sig0
    fn1 = colocated u0:1 sig1
    fn2 = colocated u0:2 sig2
    fn3 = colocated u0:3 sig3
    fn4 = colocated u0:4 sig4
    fn5 = colocated u0:5 sig5
    fn6 = colocated u0:6 sig6
    fn7 = colocated u0:7 sig7
    fn8 = colocated u0:8 sig8

block0:
    v0 = call fn0()
    jump block1(v0)

block1(v22: i32):
    v21 -> v22
    stack_store v22, ss1
    v1 = call fn1(), stack_map=[i32 @ ss1+0]
    v8 -> v1
    v18 -> v1
    stack_store v1, ss0
    v2 = iconst.i32 0
    jump block2(v2)  ; v2 = 0

block2(v3: i32):
    v4 = iconst.i32 1
    v5 = icmp ult v3, v4  ; v4 = 1
    brif v5, block3, block4

block3:
    v24 = stack_load.i32 ss0
    call fn2(v24, v4), stack_map=[i32 @ ss0+0, i32 @ ss1+0]  ; v4 = 1
    v6 = iconst.i32 1
    v7 = iadd.i32 v4, v6  ; v4 = 1, v6 = 1
    jump block2(v7)

block4:
    v26 = stack_load.i32 ss1
    jump block5(v26)

block5(v20: i32):
    v19 -> v20
    stack_store v20, ss2
    v9 = iconst.i32 0
    v10 = icmp.i32 eq v8, v9  ; v9 = 0
    brif v10, block8(v9), block6  ; v9 = 0

block6:
    v11 = call fn3(v8), stack_map=[i32 @ ss0+0, i32 @ ss2+0]
    v12 = iconst.i32 -1091584273
    v13 = icmp eq v11, v12  ; v12 = -1091584273
    v14 = iconst.i32 1
    brif v13, block8(v14), block7  ; v14 = 1

block7:
    v15 = call fn4(v8, v12), stack_map=[i32 @ ss0+0, i32 @ ss2+0]  ; v12 = -1091584273
    jump block8(v15)

block8(v16: i32):
    trapz v16, user1
    call fn5(v8), stack_map=[i32 @ ss0+0, i32 @ ss2+0]
    v17 = call fn6(), stack_map=[i32 @ ss0+0, i32 @ ss2+0]
    brif v17, block5(v19), block9

block9:
    v25 = stack_load.i32 ss2
    call fn7(v25), stack_map=[i32 @ ss2+0]
    v23 = call fn8(), stack_map=[i32 @ ss2+0]
    brif v23, block10, block1(v19)

block10:
    return
}
            "#,
        );
    }
}
