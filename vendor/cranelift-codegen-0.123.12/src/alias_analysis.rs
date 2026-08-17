//! Alias analysis, consisting of a "last store" pass and a "memory
//! values" pass. These two passes operate as one fused pass, and so
//! are implemented together here.
//!
//! We partition memory state into several *disjoint pieces* of
//! "abstract state". There are a finite number of such pieces:
//! currently, we call them "heap", "table", "vmctx", and "other".Any
//! given address in memory belongs to exactly one disjoint piece.
//!
//! One never tracks which piece a concrete address belongs to at
//! runtime; this is a purely static concept. Instead, all
//! memory-accessing instructions (loads and stores) are labeled with
//! one of these four categories in the `MemFlags`. It is forbidden
//! for a load or store to access memory under one category and a
//! later load or store to access the same memory under a different
//! category. This is ensured to be true by construction during
//! frontend translation into CLIF and during legalization.
//!
//! Given that this non-aliasing property is ensured by the producer
//! of CLIF, we can compute a *may-alias* property: one load or store
//! may-alias another load or store if both access the same category
//! of abstract state.
//!
//! The "last store" pass helps to compute this aliasing: it scans the
//! code, finding at each program point the last instruction that
//! *might have* written to a given part of abstract state.
//!
//! We can't say for sure that the "last store" *did* actually write
//! that state, but we know for sure that no instruction *later* than
//! it (up to the current instruction) did. However, we can get a
//! must-alias property from this: if at a given load or store, we
//! look backward to the "last store", *AND* we find that it has
//! exactly the same address expression and type, then we know that
//! the current instruction's access *must* be to the same memory
//! location.
//!
//! To get this must-alias property, we compute a sparse table of
//! "memory values": these are known equivalences between SSA `Value`s
//! and particular locations in memory. The memory-values table is a
//! mapping from (last store, address expression, type) to SSA
//! value. At a store, we can insert into this table directly. At a
//! load, we can also insert, if we don't already have a value (from
//! the store that produced the load's value).
//!
//! Then we can do two optimizations at once given this table. If a
//! load accesses a location identified by a (last store, address,
//! type) key already in the table, we replace it with the SSA value
//! for that memory location. This is usually known as "redundant load
//! elimination" if the value came from an earlier load of the same
//! location, or "store-to-load forwarding" if the value came from an
//! earlier store to the same location.
//!
//! In theory we could also do *dead-store elimination*, where if a
//! store overwrites a key in the table, *and* if no other load/store
//! to the abstract state category occurred, *and* no other trapping
//! instruction occurred (at which point we need an up-to-date memory
//! state because post-trap-termination memory state can be observed),
//! *and* we can prove the original store could not have trapped, then
//! we can eliminate the original store. Because this is so complex,
//! and the conditions for doing it correctly when post-trap state
//! must be correct likely reduce the potential benefit, we don't yet
//! do this.

use crate::{
    cursor::{Cursor, FuncCursor},
    dominator_tree::DominatorTreePreorder,
    inst_predicates::{
        has_memory_fence_semantics, inst_addr_offset_type, inst_store_data, visit_block_succs,
    },
    ir::{AliasRegion, Block, Function, Inst, Opcode, Type, Value, immediates::Offset32},
    trace,
};
use cranelift_entity::{EntityRef, packed_option::PackedOption};
use rustc_hash::{FxHashMap, FxHashSet};

/// For a given program point, the vector of last-store instruction
/// indices for each disjoint category of abstract state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LastStores {
    heap: PackedOption<Inst>,
    table: PackedOption<Inst>,
    vmctx: PackedOption<Inst>,
    other: PackedOption<Inst>,
}

impl LastStores {
    fn update(&mut self, func: &Function, inst: Inst) {
        let opcode = func.dfg.insts[inst].opcode();
        if has_memory_fence_semantics(opcode) {
            self.heap = inst.into();
            self.table = inst.into();
            self.vmctx = inst.into();
            self.other = inst.into();
        } else if opcode.can_store() {
            if let Some(memflags) = func.dfg.insts[inst].memflags() {
                match memflags.alias_region() {
                    None => self.other = inst.into(),
                    Some(AliasRegion::Heap) => self.heap = inst.into(),
                    Some(AliasRegion::Table) => self.table = inst.into(),
                    Some(AliasRegion::Vmctx) => self.vmctx = inst.into(),
                }
            } else {
                self.heap = inst.into();
                self.table = inst.into();
                self.vmctx = inst.into();
                self.other = inst.into();
            }
        }
    }

    fn get_last_store(&self, func: &Function, inst: Inst) -> PackedOption<Inst> {
        if let Some(memflags) = func.dfg.insts[inst].memflags() {
            match memflags.alias_region() {
                None => self.other,
                Some(AliasRegion::Heap) => self.heap,
                Some(AliasRegion::Table) => self.table,
                Some(AliasRegion::Vmctx) => self.vmctx,
            }
        } else if func.dfg.insts[inst].opcode().can_load()
            || func.dfg.insts[inst].opcode().can_store()
        {
            inst.into()
        } else {
            PackedOption::default()
        }
    }

    fn meet_from(&mut self, other: &LastStores, loc: Inst) {
        let meet = |a: PackedOption<Inst>, b: PackedOption<Inst>| -> PackedOption<Inst> {
            match (a.into(), b.into()) {
                (None, None) => None.into(),
                (Some(a), None) => a,
                (None, Some(b)) => b,
                (Some(a), Some(b)) if a == b => a,
                _ => loc.into(),
            }
        };

        self.heap = meet(self.heap, other.heap);
        self.table = meet(self.table, other.table);
        self.vmctx = meet(self.vmctx, other.vmctx);
        self.other = meet(self.other, other.other);
    }
}

/// A key identifying a unique memory location.
///
/// For the result of a load to be equivalent to the result of another
/// load, or the store data from a store, we need for (i) the
/// "version" of memory (here ensured by having the same last store
/// instruction to touch the disjoint category of abstract state we're
/// accessing); (ii) the address must be the same (here ensured by
/// having the same SSA value, which doesn't change after computed);
/// (iii) the offset must be the same; and (iv) the accessed type and
/// extension mode (e.g., 8-to-32, signed) must be the same.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct MemoryLoc {
    last_store: PackedOption<Inst>,
    address: Value,
    offset: Offset32,
    ty: Type,
    /// We keep the *opcode* of the instruction that produced the
    /// value we record at this key if the opcode is anything other
    /// than an ordinary load or store. This is needed when we
    /// consider loads that extend the value: e.g., an 8-to-32
    /// sign-extending load will produce a 32-bit value from an 8-bit
    /// value in memory, so we can only reuse that (as part of RLE)
    /// for another load with the same extending opcode.
    ///
    /// We could improve the transform to insert explicit extend ops
    /// in place of extending loads when we know the memory value, but
    /// we haven't yet done this.
    extending_opcode: Option<Opcode>,
}

/// An alias-analysis pass.
pub struct AliasAnalysis<'a> {
    /// The domtree for the function.
    domtree: &'a DominatorTreePreorder,

    /// Input state to a basic block.
    block_input: FxHashMap<Block, LastStores>,

    /// Known memory-value equivalences. This is the result of the
    /// analysis. This is a mapping from (last store, address
    /// expression, offset, type) to SSA `Value`.
    ///
    /// We keep the defining inst around for quick dominance checks.
    mem_values: FxHashMap<MemoryLoc, (Inst, Value)>,
}

impl<'a> AliasAnalysis<'a> {
    /// Perform an alias analysis pass.
    pub fn new(func: &Function, domtree: &'a DominatorTreePreorder) -> AliasAnalysis<'a> {
        trace!("alias analysis: input is:\n{:?}", func);
        let mut analysis = AliasAnalysis {
            domtree,
            block_input: FxHashMap::default(),
            mem_values: FxHashMap::default(),
        };

        analysis.compute_block_input_states(func);
        analysis
    }

    fn compute_block_input_states(&mut self, func: &Function) {
        let mut queue = vec![];
        let mut queue_set = FxHashSet::default();
        let entry = func.layout.entry_block().unwrap();
        queue.push(entry);
        queue_set.insert(entry);

        while let Some(block) = queue.pop() {
            queue_set.remove(&block);
            let mut state = *self
                .block_input
                .entry(block)
                .or_insert_with(|| LastStores::default());

            trace!(
                "alias analysis: input to block{} is {:?}",
                block.index(),
                state
            );

            for inst in func.layout.block_insts(block) {
                state.update(func, inst);
                trace!("after inst{}: state is {:?}", inst.index(), state);
            }

            visit_block_succs(func, block, |_inst, succ, _from_table| {
                let succ_first_inst = func.layout.block_insts(succ).next().unwrap();
                let updated = match self.block_input.get_mut(&succ) {
                    Some(succ_state) => {
                        let old = *succ_state;
                        succ_state.meet_from(&state, succ_first_inst);
                        *succ_state != old
                    }
                    None => {
                        self.block_input.insert(succ, state);
                        true
                    }
                };

                if updated && queue_set.insert(succ) {
                    queue.push(succ);
                }
            });
        }
    }

    /// Get the starting state for a block.
    pub fn block_starting_state(&self, block: Block) -> LastStores {
        self.block_input
            .get(&block)
            .cloned()
            .unwrap_or_else(|| LastStores::default())
    }

    /// Process one instruction. Meant to be invoked in program order
    /// within a block, and ideally in RPO or at least some domtree
    /// preorder for maximal reuse.
    ///
    /// Returns `true` if instruction was removed.
    pub fn process_inst(
        &mut self,
        func: &mut Function,
        state: &mut LastStores,
        inst: Inst,
    ) -> Option<Value> {
        trace!(
            "alias analysis: scanning at inst{} with state {:?} ({:?})",
            inst.index(),
            state,
            func.dfg.insts[inst],
        );

        let replacing_value = if let Some((address, offset, ty)) = inst_addr_offset_type(func, inst)
        {
            let address = func.dfg.resolve_aliases(address);
            let opcode = func.dfg.insts[inst].opcode();

            if opcode.can_store() {
                let store_data = inst_store_data(func, inst).unwrap();
                let store_data = func.dfg.resolve_aliases(store_data);
                let mem_loc = MemoryLoc {
                    last_store: inst.into(),
                    address,
                    offset,
                    ty,
                    extending_opcode: get_ext_opcode(opcode),
                };
                trace!(
                    "alias analysis: at inst{}: store with data v{} at loc {:?}",
                    inst.index(),
                    store_data.index(),
                    mem_loc
                );
                self.mem_values.insert(mem_loc, (inst, store_data));

                None
            } else if opcode.can_load() {
                let last_store = state.get_last_store(func, inst);
                let load_result = func.dfg.inst_results(inst)[0];
                let mem_loc = MemoryLoc {
                    last_store,
                    address,
                    offset,
                    ty,
                    extending_opcode: get_ext_opcode(opcode),
                };
                trace!(
                    "alias analysis: at inst{}: load with last_store inst{} at loc {:?}",
                    inst.index(),
                    last_store.map(|inst| inst.index()).unwrap_or(usize::MAX),
                    mem_loc
                );

                // Is there a Value already known to be stored
                // at this specific memory location?  If so,
                // we can alias the load result to this
                // already-known Value.
                //
                // Check if the definition dominates this
                // location; it might not, if it comes from a
                // load (stores will always dominate though if
                // their `last_store` survives through
                // meet-points to this use-site).
                let aliased =
                    if let Some((def_inst, value)) = self.mem_values.get(&mem_loc).cloned() {
                        trace!(
                            " -> sees known value v{} from inst{}",
                            value.index(),
                            def_inst.index()
                        );
                        if self.domtree.dominates_inst(def_inst, inst, &func.layout) {
                            trace!(
                                " -> dominates; value equiv from v{} to v{} inserted",
                                load_result.index(),
                                value.index()
                            );
                            Some(value)
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                // Otherwise, we can keep *this* load around
                // as a new equivalent value.
                if aliased.is_none() {
                    trace!(
                        " -> inserting load result v{} at loc {:?}",
                        load_result.index(),
                        mem_loc
                    );
                    self.mem_values.insert(mem_loc, (inst, load_result));
                }

                aliased
            } else {
                None
            }
        } else {
            None
        };

        state.update(func, inst);

        replacing_value
    }

    /// Make a pass and update known-redundant loads to aliased
    /// values. We interleave the updates with the memory-location
    /// tracking because resolving some aliases may expose others
    /// (e.g. in cases of double-indirection with two separate chains
    /// of loads).
    pub fn compute_and_update_aliases(&mut self, func: &mut Function) {
        let mut pos = FuncCursor::new(func);

        while let Some(block) = pos.next_block() {
            let mut state = self.block_starting_state(block);
            while let Some(inst) = pos.next_inst() {
                if let Some(replaced_result) = self.process_inst(pos.func, &mut state, inst) {
                    let result = pos.func.dfg.inst_results(inst)[0];
                    pos.func.dfg.clear_results(inst);
                    pos.func.dfg.change_to_alias(result, replaced_result);
                    pos.remove_inst_and_step_back();
                }
            }
        }
    }
}

fn get_ext_opcode(op: Opcode) -> Option<Opcode> {
    debug_assert!(op.can_load() || op.can_store());
    match op {
        Opcode::Load | Opcode::Store => None,
        _ => Some(op),
    }
}
