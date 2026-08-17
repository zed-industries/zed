/*
 * The following code is derived from `lib/src/checker.rs` in the
 * regalloc.rs project
 * (https://github.com/bytecodealliance/regalloc.rs). regalloc.rs is
 * also licensed under Apache-2.0 with the LLVM exception, as the rest
 * of regalloc2 is.
 */

//! Checker: verifies that spills/reloads/moves retain equivalent
//! dataflow to original, VReg-based code.
//!
//! The basic idea is that we track symbolic values as they flow
//! through spills and reloads.  The symbolic values represent
//! particular virtual registers in the original function body
//! presented to the register allocator. Any instruction in the
//! original function body (i.e., not added by the allocator)
//! conceptually generates a symbolic value "Vn" when storing to (or
//! modifying) a virtual register.
//!
//! A symbolic value is logically a *set of virtual registers*,
//! representing all virtual registers equal to the value in the given
//! storage slot at a given program point. This representation (as
//! opposed to tracking just one virtual register) is necessary
//! because the regalloc may implement moves in the source program
//! (via move instructions or blockparam assignments on edges) in
//! "intelligent" ways, taking advantage of values that are already in
//! the right place, so we need to know *all* names for a value.
//!
//! These symbolic values are precise but partial: in other words, if
//! a physical register is described as containing a virtual register
//! at a program point, it must actually contain the value of this
//! register (modulo any analysis bugs); but it may describe fewer
//! virtual registers even in cases where one *could* statically prove
//! that it contains a certain register, because the analysis is not
//! perfectly path-sensitive or value-sensitive. However, all
//! assignments *produced by our register allocator* should be
//! analyzed fully precisely. (This last point is important and bears
//! repeating: we only need to verify the programs that we produce,
//! not arbitrary programs.)
//!
//! Operand constraints (fixed register, register, any) are also checked
//! at each operand.
//!
//! ## Formal Definition
//!
//! The analysis lattice consists of the elements of 𝒫(V), the
//! powerset (set of all subsets) of V (the set of all virtual
//! registers). The ⊤ (top) value in the lattice is V itself, and the
//! ⊥ (bottom) value in the lattice is ∅ (the empty set). The lattice
//! ordering relation is the subset relation: S ≤ U iff S ⊆ U. These
//! definitions imply that the lattice meet-function (greatest lower
//! bound) is set-intersection.
//!
//! (For efficiency, we represent ⊤ not by actually listing out all
//! virtual registers, but by representing a special "top" value, but
//! the semantics are the same.)
//!
//! The dataflow analysis state at each program point (each point
//! before or after an instruction) is:
//!
//!   - map of: Allocation -> lattice value
//!
//! And the transfer functions for instructions are (where `A` is the
//! above map from allocated physical registers to lattice values):
//!
//!   - `Edit::Move` inserted by RA:       [ alloc_d := alloc_s ]
//!
//!       A' = A[alloc_d → A\[alloc_s\]]
//!
//!   - statement in pre-regalloc function [ V_i := op V_j, V_k, ... ]
//!     with allocated form                [ A_i := op A_j, A_k, ... ]
//!
//!       A' = { A_k → A\[A_k\] \ { V_i } for k ≠ i } ∪
//!            { A_i -> { V_i } }
//!
//!     In other words, a statement, even after allocation, generates
//!     a symbol that corresponds to its original virtual-register
//!     def. Simultaneously, that same virtual register symbol is removed
//!     from all other allocs: they no longer carry the current value.
//!
//!   - Parallel moves or blockparam-assignments in original program
//!                                       [ V_d1 := V_s1, V_d2 := V_s2, ... ]
//!
//!       A' = { A_k → subst(A\[A_k\]) for all k }
//!            where subst(S) removes symbols for overwritten virtual
//!            registers (V_d1 .. V_dn) and then adds V_di whenever
//!            V_si appeared prior to the removals.
//!
//! To check correctness, we first find the dataflow fixpoint with the
//! above lattice and transfer/meet functions. Then, at each op, we
//! examine the dataflow solution at the preceding program point, and
//! check that the allocation for each op arg (input/use) contains the
//! symbol corresponding to the original virtual register specified
//! for this arg.

#![allow(dead_code)]

use crate::{
    Allocation, AllocationKind, Block, Edit, Function, FxHashMap, FxHashSet, Inst, InstOrEdit,
    InstPosition, MachineEnv, Operand, OperandConstraint, OperandKind, OperandPos, Output, PReg,
    PRegSet, VReg,
};
use alloc::vec::Vec;
use alloc::{format, vec};
use core::default::Default;
use core::hash::Hash;
use core::result::Result;
use smallvec::{smallvec, SmallVec};

/// A set of errors detected by the regalloc checker.
#[derive(Clone, Debug)]
pub struct CheckerErrors {
    errors: Vec<CheckerError>,
}

/// A single error detected by the regalloc checker.
#[derive(Clone, Debug)]
pub enum CheckerError {
    MissingAllocation {
        inst: Inst,
        op: Operand,
    },
    UnknownValueInAllocation {
        inst: Inst,
        op: Operand,
        alloc: Allocation,
    },
    ConflictedValueInAllocation {
        inst: Inst,
        op: Operand,
        alloc: Allocation,
    },
    IncorrectValuesInAllocation {
        inst: Inst,
        op: Operand,
        alloc: Allocation,
        actual: FxHashSet<VReg>,
    },
    ConstraintViolated {
        inst: Inst,
        op: Operand,
        alloc: Allocation,
    },
    AllocationIsNotReg {
        inst: Inst,
        op: Operand,
        alloc: Allocation,
    },
    AllocationIsNotFixedReg {
        inst: Inst,
        op: Operand,
        alloc: Allocation,
    },
    AllocationIsNotReuse {
        inst: Inst,
        op: Operand,
        alloc: Allocation,
        expected_alloc: Allocation,
    },
    AllocationIsNotStack {
        inst: Inst,
        op: Operand,
        alloc: Allocation,
    },
    StackToStackMove {
        into: Allocation,
        from: Allocation,
    },
}

/// Abstract state for an allocation.
///
/// Equivalent to a set of virtual register names, with the
/// universe-set as top and empty set as bottom lattice element. The
/// meet-function is thus set intersection.
#[derive(Clone, Debug, PartialEq, Eq)]
enum CheckerValue {
    /// The lattice top-value: this value could be equivalent to any
    /// vreg (i.e., the universe set).
    Universe,
    /// The set of VRegs that this value is equal to.
    VRegs(FxHashSet<VReg>),
}

impl CheckerValue {
    fn vregs(&self) -> Option<&FxHashSet<VReg>> {
        match self {
            CheckerValue::Universe => None,
            CheckerValue::VRegs(vregs) => Some(vregs),
        }
    }

    fn vregs_mut(&mut self) -> Option<&mut FxHashSet<VReg>> {
        match self {
            CheckerValue::Universe => None,
            CheckerValue::VRegs(vregs) => Some(vregs),
        }
    }
}

impl Default for CheckerValue {
    fn default() -> CheckerValue {
        CheckerValue::Universe
    }
}

impl CheckerValue {
    /// Meet function of the abstract-interpretation value
    /// lattice. Returns a boolean value indicating whether `self` was
    /// changed.
    fn meet_with(&mut self, other: &CheckerValue) {
        match (self, other) {
            (_, CheckerValue::Universe) => {
                // Nothing.
            }
            (this @ CheckerValue::Universe, _) => {
                *this = other.clone();
            }
            (CheckerValue::VRegs(my_vregs), CheckerValue::VRegs(other_vregs)) => {
                my_vregs.retain(|vreg| other_vregs.contains(vreg));
            }
        }
    }

    fn from_reg(reg: VReg) -> CheckerValue {
        CheckerValue::VRegs(core::iter::once(reg).collect())
    }

    fn remove_vreg(&mut self, reg: VReg) {
        match self {
            CheckerValue::Universe => {
                panic!("Cannot remove VReg from Universe set (we do not have the full list of vregs available");
            }
            CheckerValue::VRegs(vregs) => {
                vregs.remove(&reg);
            }
        }
    }

    fn copy_vreg(&mut self, src: VReg, dst: VReg) {
        match self {
            CheckerValue::Universe => {
                // Nothing.
            }
            CheckerValue::VRegs(vregs) => {
                if vregs.contains(&src) {
                    vregs.insert(dst);
                }
            }
        }
    }

    fn empty() -> CheckerValue {
        CheckerValue::VRegs(FxHashSet::default())
    }
}

fn visit_all_vregs<F: Function, V: FnMut(VReg)>(f: &F, mut v: V) {
    for block in 0..f.num_blocks() {
        let block = Block::new(block);
        for inst in f.block_insns(block).iter() {
            for op in f.inst_operands(inst) {
                v(op.vreg());
            }
            if f.is_branch(inst) {
                for succ_idx in 0..f.block_succs(block).len() {
                    for &param in f.branch_blockparams(block, inst, succ_idx) {
                        v(param);
                    }
                }
            }
        }
        for &vreg in f.block_params(block) {
            v(vreg);
        }
    }
}

/// State that steps through program points as we scan over the instruction stream.
#[derive(Clone, Debug, PartialEq, Eq)]
enum CheckerState {
    Top,
    Allocations(FxHashMap<Allocation, CheckerValue>),
}

impl CheckerState {
    fn get_value(&self, alloc: &Allocation) -> Option<&CheckerValue> {
        match self {
            CheckerState::Top => None,
            CheckerState::Allocations(allocs) => allocs.get(alloc),
        }
    }

    fn get_values_mut(&mut self) -> impl Iterator<Item = &mut CheckerValue> {
        match self {
            CheckerState::Top => panic!("Cannot get mutable values iterator on Top state"),
            CheckerState::Allocations(allocs) => allocs.values_mut(),
        }
    }

    fn get_mappings(&self) -> impl Iterator<Item = (&Allocation, &CheckerValue)> {
        match self {
            CheckerState::Top => panic!("Cannot get mappings iterator on Top state"),
            CheckerState::Allocations(allocs) => allocs.iter(),
        }
    }

    fn get_mappings_mut(&mut self) -> impl Iterator<Item = (&Allocation, &mut CheckerValue)> {
        match self {
            CheckerState::Top => panic!("Cannot get mutable mappings iterator on Top state"),
            CheckerState::Allocations(allocs) => allocs.iter_mut(),
        }
    }

    /// Transition from a "top" (undefined/unanalyzed) state to an empty set of allocations.
    fn become_defined(&mut self) {
        match self {
            CheckerState::Top => *self = CheckerState::Allocations(FxHashMap::default()),
            _ => {}
        }
    }

    fn set_value(&mut self, alloc: Allocation, value: CheckerValue) {
        match self {
            CheckerState::Top => {
                panic!("Cannot set value on Top state");
            }
            CheckerState::Allocations(allocs) => {
                allocs.insert(alloc, value);
            }
        }
    }

    fn copy_vreg(&mut self, src: VReg, dst: VReg) {
        match self {
            CheckerState::Top => {
                // Nothing.
            }
            CheckerState::Allocations(allocs) => {
                for value in allocs.values_mut() {
                    value.copy_vreg(src, dst);
                }
            }
        }
    }

    fn remove_value(&mut self, alloc: &Allocation) {
        match self {
            CheckerState::Top => {
                panic!("Cannot remove value on Top state");
            }
            CheckerState::Allocations(allocs) => {
                allocs.remove(alloc);
            }
        }
    }

    fn initial() -> Self {
        CheckerState::Allocations(FxHashMap::default())
    }
}

impl Default for CheckerState {
    fn default() -> CheckerState {
        CheckerState::Top
    }
}

impl core::fmt::Display for CheckerValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            CheckerValue::Universe => {
                write!(f, "top")
            }
            CheckerValue::VRegs(vregs) => {
                write!(f, "{{ ")?;
                for vreg in vregs {
                    write!(f, "{} ", vreg)?;
                }
                write!(f, "}}")?;
                Ok(())
            }
        }
    }
}

/// Meet function for analysis value: meet individual values at
/// matching allocations, and intersect keys (remove key-value pairs
/// only on one side). Returns boolean flag indicating whether `into`
/// changed.
fn merge_map<K: Copy + Clone + PartialEq + Eq + Hash>(
    into: &mut FxHashMap<K, CheckerValue>,
    from: &FxHashMap<K, CheckerValue>,
) {
    into.retain(|k, _| from.contains_key(k));
    for (k, into_v) in into.iter_mut() {
        let from_v = from.get(k).unwrap();
        into_v.meet_with(from_v);
    }
}

impl CheckerState {
    /// Create a new checker state.
    fn new() -> CheckerState {
        Default::default()
    }

    /// Merge this checker state with another at a CFG join-point.
    fn meet_with(&mut self, other: &CheckerState) {
        match (self, other) {
            (_, CheckerState::Top) => {
                // Nothing.
            }
            (this @ CheckerState::Top, _) => {
                *this = other.clone();
            }
            (
                CheckerState::Allocations(my_allocations),
                CheckerState::Allocations(other_allocations),
            ) => {
                merge_map(my_allocations, other_allocations);
            }
        }
    }

    fn check_val<'a, F: Function>(
        &self,
        inst: Inst,
        op: Operand,
        alloc: Allocation,
        val: &CheckerValue,
        allocs: &[Allocation],
        checker: &Checker<'a, F>,
    ) -> Result<(), CheckerError> {
        if alloc == Allocation::none() {
            return Err(CheckerError::MissingAllocation { inst, op });
        }

        if op.kind() == OperandKind::Use && op.as_fixed_nonallocatable().is_none() {
            match val {
                CheckerValue::Universe => {
                    return Err(CheckerError::UnknownValueInAllocation { inst, op, alloc });
                }
                CheckerValue::VRegs(vregs) if !vregs.contains(&op.vreg()) => {
                    return Err(CheckerError::IncorrectValuesInAllocation {
                        inst,
                        op,
                        alloc,
                        actual: vregs.clone(),
                    });
                }
                _ => {}
            }
        }

        self.check_constraint(inst, op, alloc, allocs, checker)?;

        Ok(())
    }

    /// Check an instruction against this state. This must be called
    /// twice: once with `InstPosition::Before`, and once with
    /// `InstPosition::After` (after updating state with defs).
    fn check<'a, F: Function>(
        &self,
        pos: InstPosition,
        checkinst: &CheckerInst,
        checker: &Checker<'a, F>,
    ) -> Result<(), CheckerError> {
        let default_val = Default::default();
        match checkinst {
            &CheckerInst::Op {
                inst,
                ref operands,
                ref allocs,
                ..
            } => {
                // Skip Use-checks at the After point if there are any
                // reused inputs: the Def which reuses the input
                // happens early.
                let has_reused_input = operands
                    .iter()
                    .any(|op| matches!(op.constraint(), OperandConstraint::Reuse(_)));
                if has_reused_input && pos == InstPosition::After {
                    return Ok(());
                }

                // For each operand, check (i) that the allocation
                // contains the expected vreg, and (ii) that it meets
                // the requirements of the OperandConstraint.
                for (op, alloc) in operands.iter().zip(allocs.iter()) {
                    let is_here = match (op.pos(), pos) {
                        (OperandPos::Early, InstPosition::Before) => true,
                        (OperandPos::Late, InstPosition::After) => true,
                        _ => false,
                    };
                    if !is_here {
                        continue;
                    }

                    let val = self.get_value(alloc).unwrap_or(&default_val);
                    trace!(
                        "checker: checkinst {:?}: op {:?}, alloc {:?}, checker value {:?}",
                        checkinst,
                        op,
                        alloc,
                        val
                    );
                    self.check_val(inst, *op, *alloc, val, allocs, checker)?;
                }
            }
            &CheckerInst::Move { into, from } => {
                // Ensure that the allocator never returns stack-to-stack moves.
                let is_stack = |alloc: Allocation| {
                    if let Some(reg) = alloc.as_reg() {
                        checker.stack_pregs.contains(reg)
                    } else {
                        alloc.is_stack()
                    }
                };
                if is_stack(into) && is_stack(from) {
                    return Err(CheckerError::StackToStackMove { into, from });
                }
            }
            &CheckerInst::ParallelMove { .. } => {
                // This doesn't need verification; we just update
                // according to the move semantics in the step
                // function below.
            }
        }
        Ok(())
    }

    /// Update according to instruction.
    fn update(&mut self, checkinst: &CheckerInst) {
        self.become_defined();

        match checkinst {
            &CheckerInst::Move { into, from } => {
                // Value may not be present if this move is part of
                // the parallel move resolver's fallback sequence that
                // saves a victim register elsewhere. (In other words,
                // that sequence saves an undefined value and restores
                // it, so has no effect.) The checker needs to avoid
                // putting Universe lattice values into the map.
                if let Some(val) = self.get_value(&from).cloned() {
                    trace!(
                        "checker: checkinst {:?} updating: move {:?} -> {:?} val {:?}",
                        checkinst,
                        from,
                        into,
                        val
                    );
                    self.set_value(into, val);
                }
            }
            &CheckerInst::ParallelMove { ref moves } => {
                // First, build map of actions for each vreg in an
                // alloc. If an alloc has a reg V_i before a parallel
                // move, then for each use of V_i as a source (V_i ->
                // V_j), we might add a new V_j wherever V_i appears;
                // and if V_i is used as a dest (at most once), then
                // it must be removed first from allocs' vreg sets.
                let mut additions: FxHashMap<VReg, SmallVec<[VReg; 2]>> = FxHashMap::default();
                let mut deletions: FxHashSet<VReg> = FxHashSet::default();

                for &(dest, src) in moves {
                    deletions.insert(dest);
                    additions
                        .entry(src)
                        .or_insert_with(|| smallvec![])
                        .push(dest);
                }

                // Now process each allocation's set of vreg labels,
                // first deleting those labels that were updated by
                // this parallel move, then adding back labels
                // redefined by the move.
                for value in self.get_values_mut() {
                    if let Some(vregs) = value.vregs_mut() {
                        let mut insertions: SmallVec<[VReg; 2]> = smallvec![];
                        for &vreg in vregs.iter() {
                            if let Some(additions) = additions.get(&vreg) {
                                insertions.extend(additions.iter().cloned());
                            }
                        }
                        for &d in &deletions {
                            vregs.remove(&d);
                        }
                        vregs.extend(insertions);
                    }
                }
            }
            &CheckerInst::Op {
                ref operands,
                ref allocs,
                ref clobbers,
                ..
            } => {
                // For each def, (i) update alloc to reflect defined
                // vreg (and only that vreg), and (ii) update all
                // other allocs in the checker state by removing this
                // vreg, if defined (other defs are now stale).
                for (op, alloc) in operands.iter().zip(allocs.iter()) {
                    if op.kind() != OperandKind::Def {
                        continue;
                    }
                    self.remove_vreg(op.vreg());
                    self.set_value(*alloc, CheckerValue::from_reg(op.vreg()));
                }
                for clobber in clobbers {
                    self.remove_value(&Allocation::reg(*clobber));
                }
            }
        }
    }

    fn remove_vreg(&mut self, vreg: VReg) {
        for (_, value) in self.get_mappings_mut() {
            value.remove_vreg(vreg);
        }
    }

    fn check_constraint<'a, F: Function>(
        &self,
        inst: Inst,
        op: Operand,
        alloc: Allocation,
        allocs: &[Allocation],
        checker: &Checker<'a, F>,
    ) -> Result<(), CheckerError> {
        match op.constraint() {
            OperandConstraint::Any => {}
            OperandConstraint::Reg => {
                if let Some(preg) = alloc.as_reg() {
                    // Reject pregs that represent a fixed stack slot.
                    if !checker.machine_env.fixed_stack_slots.contains(&preg) {
                        return Ok(());
                    }
                }
                return Err(CheckerError::AllocationIsNotReg { inst, op, alloc });
            }
            OperandConstraint::FixedReg(preg) => {
                if alloc != Allocation::reg(preg) {
                    return Err(CheckerError::AllocationIsNotFixedReg { inst, op, alloc });
                }
            }
            OperandConstraint::Reuse(idx) => {
                if alloc.kind() != AllocationKind::Reg {
                    return Err(CheckerError::AllocationIsNotReg { inst, op, alloc });
                }
                if alloc != allocs[idx] {
                    return Err(CheckerError::AllocationIsNotReuse {
                        inst,
                        op,
                        alloc,
                        expected_alloc: allocs[idx],
                    });
                }
            }
        }
        Ok(())
    }
}

/// An instruction representation in the checker's BB summary.
#[derive(Clone, Debug)]
pub(crate) enum CheckerInst {
    /// A move between allocations (these could be registers or
    /// spillslots).
    Move { into: Allocation, from: Allocation },

    /// A parallel move in the original program. Simultaneously moves
    /// from all source vregs to all corresponding dest vregs,
    /// permitting overlap in the src and dest sets and doing all
    /// reads before any writes.
    ParallelMove {
        /// Vector of (dest, src) moves.
        moves: Vec<(VReg, VReg)>,
    },

    /// A regular instruction with fixed use and def slots. Contains
    /// both the original operands (as given to the regalloc) and the
    /// allocation results.
    Op {
        inst: Inst,
        operands: Vec<Operand>,
        allocs: Vec<Allocation>,
        clobbers: Vec<PReg>,
    },
}

#[derive(Debug)]
pub struct Checker<'a, F: Function> {
    f: &'a F,
    bb_in: FxHashMap<Block, CheckerState>,
    bb_insts: FxHashMap<Block, Vec<CheckerInst>>,
    edge_insts: FxHashMap<(Block, Block), Vec<CheckerInst>>,
    machine_env: &'a MachineEnv,
    stack_pregs: PRegSet,
}

impl<'a, F: Function> Checker<'a, F> {
    /// Create a new checker for the given function, initializing CFG
    /// info immediately.  The client should call the `add_*()`
    /// methods to add abstract instructions to each BB before
    /// invoking `run()` to check for errors.
    pub fn new(f: &'a F, machine_env: &'a MachineEnv) -> Checker<'a, F> {
        let mut bb_in = FxHashMap::default();
        let mut bb_insts = FxHashMap::default();
        let mut edge_insts = FxHashMap::default();

        for block in 0..f.num_blocks() {
            let block = Block::new(block);
            bb_in.insert(block, Default::default());
            bb_insts.insert(block, vec![]);
            for &succ in f.block_succs(block) {
                edge_insts.insert((block, succ), vec![]);
            }
        }

        bb_in.insert(f.entry_block(), CheckerState::default());

        let mut stack_pregs = PRegSet::empty();
        for &preg in &machine_env.fixed_stack_slots {
            stack_pregs.add(preg);
        }

        Checker {
            f,
            bb_in,
            bb_insts,
            edge_insts,
            machine_env,
            stack_pregs,
        }
    }

    /// Build the list of checker instructions based on the given func
    /// and allocation results.
    pub fn prepare(&mut self, out: &Output) {
        trace!("checker: out = {:?}", out);
        let mut last_inst = None;
        for block in 0..self.f.num_blocks() {
            let block = Block::new(block);
            for inst_or_edit in out.block_insts_and_edits(self.f, block) {
                match inst_or_edit {
                    InstOrEdit::Inst(inst) => {
                        debug_assert!(last_inst.is_none() || inst > last_inst.unwrap());
                        last_inst = Some(inst);
                        self.handle_inst(block, inst, out);
                    }
                    InstOrEdit::Edit(edit) => self.handle_edit(block, edit),
                }
            }
        }
    }

    /// For each original instruction, create an `Op`.
    fn handle_inst(&mut self, block: Block, inst: Inst, out: &Output) {
        // Process uses, defs, and clobbers.
        let operands: Vec<_> = self.f.inst_operands(inst).iter().cloned().collect();
        let allocs: Vec<_> = out.inst_allocs(inst).iter().cloned().collect();
        let clobbers: Vec<_> = self.f.inst_clobbers(inst).into_iter().collect();
        let checkinst = CheckerInst::Op {
            inst,
            operands,
            allocs,
            clobbers,
        };
        trace!("checker: adding inst {:?}", checkinst);
        self.bb_insts.get_mut(&block).unwrap().push(checkinst);

        // If this is a branch, emit a ParallelMove on each outgoing
        // edge as necessary to handle blockparams.
        if self.f.is_branch(inst) {
            for (i, &succ) in self.f.block_succs(block).iter().enumerate() {
                let args = self.f.branch_blockparams(block, inst, i);
                let params = self.f.block_params(succ);
                assert_eq!(
                    args.len(),
                    params.len(),
                    "block{} has succ block{}; gave {} args for {} params",
                    block.index(),
                    succ.index(),
                    args.len(),
                    params.len()
                );
                if args.len() > 0 {
                    let moves = params.iter().cloned().zip(args.iter().cloned()).collect();
                    self.edge_insts
                        .get_mut(&(block, succ))
                        .unwrap()
                        .push(CheckerInst::ParallelMove { moves });
                }
            }
        }
    }

    fn handle_edit(&mut self, block: Block, edit: &Edit) {
        trace!("checker: adding edit {:?}", edit);
        match edit {
            &Edit::Move { from, to } => {
                self.bb_insts
                    .get_mut(&block)
                    .unwrap()
                    .push(CheckerInst::Move { into: to, from });
            }
        }
    }

    /// Perform the dataflow analysis to compute checker state at each BB entry.
    fn analyze(&mut self) {
        let mut queue = Vec::new();
        let mut queue_set = FxHashSet::default();

        // Put every block in the queue to start with, to ensure
        // everything is visited even if the initial state remains
        // `Top` after preds update it.
        //
        // We add blocks in reverse order so that when we process
        // back-to-front below, we do our initial pass in input block
        // order, which is (usually) RPO order or at least a
        // reasonable visit order.
        for block in (0..self.f.num_blocks()).rev() {
            let block = Block::new(block);
            queue.push(block);
            queue_set.insert(block);
        }

        while let Some(block) = queue.pop() {
            queue_set.remove(&block);
            let mut state = self.bb_in.get(&block).cloned().unwrap();
            trace!("analyze: block {} has state {:?}", block.index(), state);
            for inst in self.bb_insts.get(&block).unwrap() {
                state.update(inst);
                trace!("analyze: inst {:?} -> state {:?}", inst, state);
            }

            for &succ in self.f.block_succs(block) {
                let mut new_state = state.clone();
                for edge_inst in self.edge_insts.get(&(block, succ)).unwrap() {
                    new_state.update(edge_inst);
                    trace!(
                        "analyze: succ {:?}: inst {:?} -> state {:?}",
                        succ,
                        edge_inst,
                        new_state
                    );
                }

                let cur_succ_in = self.bb_in.get(&succ).unwrap();
                trace!(
                    "meeting state {:?} for block {} with state {:?} for block {}",
                    new_state,
                    block.index(),
                    cur_succ_in,
                    succ.index()
                );
                new_state.meet_with(cur_succ_in);
                let changed = &new_state != cur_succ_in;
                trace!(" -> {:?}, changed {}", new_state, changed);

                if changed {
                    trace!(
                        "analyze: block {} state changed from {:?} to {:?}; pushing onto queue",
                        succ.index(),
                        cur_succ_in,
                        new_state
                    );
                    self.bb_in.insert(succ, new_state);
                    if queue_set.insert(succ) {
                        queue.push(succ);
                    }
                }
            }
        }
    }

    /// Using BB-start state computed by `analyze()`, step the checker state
    /// through each BB and check each instruction's register allocations
    /// for errors.
    fn find_errors(&self) -> Result<(), CheckerErrors> {
        let mut errors = vec![];
        for (block, input) in &self.bb_in {
            let mut state = input.clone();
            for inst in self.bb_insts.get(block).unwrap() {
                if let Err(e) = state.check(InstPosition::Before, inst, self) {
                    trace!("Checker error: {:?}", e);
                    errors.push(e);
                }
                state.update(inst);
                if let Err(e) = state.check(InstPosition::After, inst, self) {
                    trace!("Checker error: {:?}", e);
                    errors.push(e);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(CheckerErrors { errors })
        }
    }

    /// Find any errors, returning `Err(CheckerErrors)` with all errors found
    /// or `Ok(())` otherwise.
    pub fn run(mut self) -> Result<(), CheckerErrors> {
        self.analyze();
        let result = self.find_errors();

        trace!("=== CHECKER RESULT ===");
        fn print_state(state: &CheckerState) {
            if !trace_enabled!() {
                return;
            }
            if let CheckerState::Allocations(allocs) = state {
                let mut s = vec![];
                for (alloc, state) in allocs {
                    s.push(format!("{} := {}", alloc, state));
                }
                trace!("    {{ {} }}", s.join(", "))
            }
        }
        for bb in 0..self.f.num_blocks() {
            let bb = Block::new(bb);
            trace!("block{}:", bb.index());
            let insts = self.bb_insts.get(&bb).unwrap();
            let mut state = self.bb_in.get(&bb).unwrap().clone();
            print_state(&state);
            for inst in insts {
                match inst {
                    &CheckerInst::Op {
                        inst,
                        ref operands,
                        ref allocs,
                        ref clobbers,
                    } => {
                        trace!(
                            "  inst{}: {:?} ({:?}) clobbers:{:?}",
                            inst.index(),
                            operands,
                            allocs,
                            clobbers
                        );
                    }
                    &CheckerInst::Move { from, into } => {
                        trace!("    {} -> {}", from, into);
                    }
                    &CheckerInst::ParallelMove { .. } => {
                        panic!("unexpected parallel_move in body (non-edge)")
                    }
                }
                state.update(inst);
                print_state(&state);
            }

            for &succ in self.f.block_succs(bb) {
                trace!("  succ {:?}:", succ);
                let mut state = state.clone();
                for edge_inst in self.edge_insts.get(&(bb, succ)).unwrap() {
                    match edge_inst {
                        &CheckerInst::ParallelMove { ref moves } => {
                            let moves = moves
                                .iter()
                                .map(|(dest, src)| format!("{} -> {}", src, dest))
                                .collect::<Vec<_>>();
                            trace!("    parallel_move {}", moves.join(", "));
                        }
                        _ => panic!("unexpected edge_inst: not a parallel move"),
                    }
                    state.update(edge_inst);
                    print_state(&state);
                }
            }
        }

        result
    }
}
