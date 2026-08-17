/*
 * Released under the terms of the Apache 2.0 license with LLVM
 * exception. See `LICENSE` for details.
 */

use crate::{ion::data_structures::u64_key, Allocation, PReg};
use core::fmt::Debug;
use smallvec::{smallvec, SmallVec};

/// A list of moves to be performed in sequence, with auxiliary data
/// attached to each.
pub type MoveVec<T> = SmallVec<[(Allocation, Allocation, T); 16]>;

/// A list of moves to be performance in sequence, like a
/// `MoveVec<T>`, except that an unchosen scratch space may occur as
/// well, represented by `Allocation::none()`.
#[derive(Clone, Debug)]
pub enum MoveVecWithScratch<T> {
    /// No scratch was actually used.
    NoScratch(MoveVec<T>),
    /// A scratch space was used.
    Scratch(MoveVec<T>),
}

/// A `ParallelMoves` represents a list of alloc-to-alloc moves that
/// must happen in parallel -- i.e., all reads of sources semantically
/// happen before all writes of destinations, and destinations are
/// allowed to overwrite sources. It can compute a list of sequential
/// moves that will produce the equivalent data movement, possibly
/// using a scratch register if one is necessary.
pub struct ParallelMoves<T: Clone + Copy + Default> {
    parallel_moves: MoveVec<T>,
}

impl<T: Clone + Copy + Default + PartialEq> ParallelMoves<T> {
    pub fn new() -> Self {
        Self {
            parallel_moves: smallvec![],
        }
    }

    pub fn add(&mut self, from: Allocation, to: Allocation, t: T) {
        self.parallel_moves.push((from, to, t));
    }

    fn sources_overlap_dests(&self) -> bool {
        // Assumes `parallel_moves` has already been sorted by `dst`
        // in `resolve()` below. The O(n log n) cost of this loop is no
        // worse than the sort we already did.
        for &(src, _, _) in &self.parallel_moves {
            if self
                .parallel_moves
                .binary_search_by_key(&src, |&(_, dst, _)| dst)
                .is_ok()
            {
                return true;
            }
        }
        false
    }

    /// Resolve the parallel-moves problem to a sequence of separate
    /// moves, such that the combined effect of the sequential moves
    /// is as-if all of the moves added to this `ParallelMoves`
    /// resolver happened in parallel.
    ///
    /// Sometimes, if there is a cycle, a scratch register is
    /// necessary to allow the moves to occur sequentially. In this
    /// case, `Allocation::none()` is returned to represent the
    /// scratch register. The caller may choose to always hold a
    /// separate scratch register unused to allow this to be trivially
    /// rewritten; or may dynamically search for or create a free
    /// register as needed, if none are available.
    pub fn resolve(mut self) -> MoveVecWithScratch<T> {
        // Easy case: zero or one move. Just return our vec.
        if self.parallel_moves.len() <= 1 {
            return MoveVecWithScratch::NoScratch(self.parallel_moves);
        }

        // Sort moves so that we can efficiently test for presence.
        // For that purpose it doesn't matter whether we sort by
        // source or destination, but later we'll want them sorted
        // by destination.
        self.parallel_moves
            .sort_by_key(|&(src, dst, _)| u64_key(dst.bits(), src.bits()));

        // Duplicate moves cannot change the semantics of this
        // parallel move set, so remove them. This is cheap since we
        // just sorted the list.
        self.parallel_moves.dedup();

        // General case: some moves overwrite dests that other moves
        // read as sources. We'll use a general algorithm.
        //
        // *Important property*: because we expect that each register
        // has only one writer (otherwise the effect of the parallel
        // move is undefined), each move can only block one other move
        // (with its one source corresponding to the one writer of
        // that source). Thus, we *can only have simple cycles* (those
        // that are a ring of nodes, i.e., with only one path from a
        // node back to itself); there are no SCCs that are more
        // complex than that. We leverage this fact below to avoid
        // having to do a full Tarjan SCC DFS (with lowest-index
        // computation, etc.): instead, as soon as we find a cycle, we
        // know we have the full cycle and we can do a cyclic move
        // sequence and continue.

        // Check that each destination has only one writer.
        if cfg!(debug_assertions) {
            let mut last_dst = None;
            for &(_, dst, _) in &self.parallel_moves {
                if last_dst.is_some() {
                    debug_assert!(last_dst.unwrap() != dst);
                }
                last_dst = Some(dst);
            }
        }

        // Moving an allocation into itself is technically a cycle but
        // should have no effect, as long as there are no other writes
        // into that destination.
        self.parallel_moves.retain(|&mut (src, dst, _)| src != dst);

        // Do any dests overlap sources? If not, we can also just
        // return the list.
        if !self.sources_overlap_dests() {
            return MoveVecWithScratch::NoScratch(self.parallel_moves);
        }

        // Construct a mapping from move indices to moves they must
        // come before. Any given move must come before a move that
        // overwrites its destination; we have moves sorted by dest
        // above so we can efficiently find such a move, if any.
        const NONE: usize = usize::MAX;
        let must_come_before: SmallVec<[usize; 16]> = self
            .parallel_moves
            .iter()
            .map(|&(src, _, _)| {
                self.parallel_moves
                    .binary_search_by_key(&src, |&(_, dst, _)| dst)
                    .unwrap_or(NONE)
            })
            .collect();

        // Do a simple stack-based DFS and emit moves in postorder,
        // then reverse at the end for RPO. Unlike Tarjan's SCC
        // algorithm, we can emit a cycle as soon as we find one, as
        // noted above.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum State {
            /// Not on stack, not visited
            ToDo,
            /// On stack, not yet visited
            Pending,
            /// Visited
            Done,
        }
        let mut ret: MoveVec<T> = smallvec![];
        let mut stack: SmallVec<[usize; 16]> = smallvec![];
        let mut state: SmallVec<[State; 16]> = smallvec![State::ToDo; self.parallel_moves.len()];
        let mut scratch_used = false;

        while let Some(next) = state.iter().position(|&state| state == State::ToDo) {
            stack.push(next);
            state[next] = State::Pending;

            while let Some(&top) = stack.last() {
                debug_assert_eq!(state[top], State::Pending);
                let next = must_come_before[top];
                if next == NONE || state[next] == State::Done {
                    ret.push(self.parallel_moves[top]);
                    state[top] = State::Done;
                    stack.pop();
                    while let Some(top) = stack.pop() {
                        ret.push(self.parallel_moves[top]);
                        state[top] = State::Done;
                    }
                } else if state[next] == State::ToDo {
                    stack.push(next);
                    state[next] = State::Pending;
                } else {
                    // Found a cycle -- emit a cyclic-move sequence
                    // for the cycle on the top of stack, then normal
                    // moves below it. Recall that these moves will be
                    // reversed in sequence, so from the original
                    // parallel move set
                    //
                    //     { B := A, C := B, A := B }
                    //
                    // we will generate something like:
                    //
                    //     A := scratch
                    //     B := A
                    //     C := B
                    //     scratch := C
                    //
                    // which will become:
                    //
                    //     scratch := C
                    //     C := B
                    //     B := A
                    //     A := scratch
                    debug_assert_ne!(top, next);
                    state[top] = State::Done;
                    stack.pop();

                    let (scratch_src, dst, dst_t) = self.parallel_moves[top];
                    scratch_used = true;

                    ret.push((Allocation::none(), dst, dst_t));
                    while let Some(move_idx) = stack.pop() {
                        state[move_idx] = State::Done;
                        ret.push(self.parallel_moves[move_idx]);

                        if move_idx == next {
                            break;
                        }
                    }
                    ret.push((scratch_src, Allocation::none(), T::default()));
                }
            }
        }

        ret.reverse();

        if scratch_used {
            MoveVecWithScratch::Scratch(ret)
        } else {
            MoveVecWithScratch::NoScratch(ret)
        }
    }
}

impl<T> MoveVecWithScratch<T> {
    /// Fills in the scratch space, if needed, with the given
    /// register/allocation and returns a final list of moves. The
    /// scratch register must not occur anywhere in the parallel-move
    /// problem given to the resolver that produced this
    /// `MoveVecWithScratch`.
    pub fn with_scratch(self, scratch: Allocation) -> MoveVec<T> {
        match self {
            MoveVecWithScratch::NoScratch(moves) => moves,
            MoveVecWithScratch::Scratch(mut moves) => {
                for (src, dst, _) in &mut moves {
                    debug_assert!(
                        *src != scratch && *dst != scratch,
                        "Scratch register should not also be an actual source or dest of moves"
                    );
                    debug_assert!(
                        !(src.is_none() && dst.is_none()),
                        "Move resolution should not have produced a scratch-to-scratch move"
                    );
                    if src.is_none() {
                        *src = scratch;
                    }
                    if dst.is_none() {
                        *dst = scratch;
                    }
                }
                moves
            }
        }
    }

    /// Unwrap without a scratch register.
    pub fn without_scratch(self) -> Option<MoveVec<T>> {
        match self {
            MoveVecWithScratch::NoScratch(moves) => Some(moves),
            MoveVecWithScratch::Scratch(..) => None,
        }
    }

    /// Do we need a scratch register?
    pub fn needs_scratch(&self) -> bool {
        match self {
            MoveVecWithScratch::NoScratch(..) => false,
            MoveVecWithScratch::Scratch(..) => true,
        }
    }
}

/// Final stage of move resolution: finding or using scratch
/// registers, creating them if necessary by using stackslots, and
/// ensuring that the final list of moves contains no stack-to-stack
/// moves.
///
/// The resolved list of moves may need one or two scratch registers,
/// and maybe a stackslot, to ensure these conditions. Our general
/// strategy is in two steps.
///
/// First, we find a scratch register, so we only have to worry about
/// a list of moves, all with real locations as src and dest. If we're
/// lucky and there are any registers not allocated at this
/// program-point, we can use a real register. Otherwise, we use an
/// extra stackslot. This is fine, because at this step,
/// stack-to-stack moves are OK.
///
/// Then, we resolve stack-to-stack moves into stack-to-reg /
/// reg-to-stack pairs. For this, we try to allocate a second free
/// register. If unavailable, we create a new scratch stackslot to
/// serve as a backup of one of the in-use registers, then borrow that
/// register as the scratch register in the middle of stack-to-stack
/// moves.
pub struct MoveAndScratchResolver<GetReg, GetStackSlot, IsStackAlloc>
where
    GetReg: FnMut() -> Option<Allocation>,
    GetStackSlot: FnMut() -> Allocation,
    IsStackAlloc: Fn(Allocation) -> bool,
{
    /// Closure that finds us a PReg at the current location.
    pub find_free_reg: GetReg,
    /// Closure that gets us a stackslot, if needed.
    pub get_stackslot: GetStackSlot,
    /// Closure to determine whether an `Allocation` refers to a stack slot.
    pub is_stack_alloc: IsStackAlloc,
    /// Use this register if no free register is available to use as a
    /// temporary in stack-to-stack moves. If we do use this register
    /// for that purpose, its value will be restored by the end of the
    /// move sequence. Provided by caller and statically chosen. This is
    /// a very last-ditch option, so static choice is OK.
    pub borrowed_scratch_reg: PReg,
}

impl<GetReg, GetStackSlot, IsStackAlloc> MoveAndScratchResolver<GetReg, GetStackSlot, IsStackAlloc>
where
    GetReg: FnMut() -> Option<Allocation>,
    GetStackSlot: FnMut() -> Allocation,
    IsStackAlloc: Fn(Allocation) -> bool,
{
    pub fn compute<T: Debug + Default + Copy>(
        mut self,
        moves: MoveVecWithScratch<T>,
    ) -> MoveVec<T> {
        let moves = if moves.needs_scratch() {
            // Now, find a scratch allocation in order to resolve cycles.
            let scratch = (self.find_free_reg)().unwrap_or_else(|| (self.get_stackslot)());
            trace!("scratch resolver: scratch alloc {:?}", scratch);

            moves.with_scratch(scratch)
        } else {
            moves.without_scratch().unwrap()
        };

        // Do we have any stack-to-stack moves? Fast return if not.
        let stack_to_stack = moves
            .iter()
            .any(|&(src, dst, _)| self.is_stack_to_stack_move(src, dst));
        if !stack_to_stack {
            return moves;
        }

        // Allocate a scratch register for stack-to-stack move expansion.
        let (scratch_reg, save_slot) = if let Some(reg) = (self.find_free_reg)() {
            trace!(
                "scratch resolver: have free stack-to-stack scratch preg: {:?}",
                reg
            );
            (reg, None)
        } else {
            let reg = Allocation::reg(self.borrowed_scratch_reg);
            // Stackslot into which we need to save the stack-to-stack
            // scratch reg before doing any stack-to-stack moves, if we stole
            // the reg.
            let save = (self.get_stackslot)();
            trace!(
                "scratch resolver: stack-to-stack borrowing {:?} with save stackslot {:?}",
                reg,
                save
            );
            (reg, Some(save))
        };

        // Mutually exclusive flags for whether either scratch_reg or
        // save_slot need to be restored from the other. Initially,
        // scratch_reg has a value we should preserve and save_slot
        // has garbage.
        let mut scratch_dirty = false;
        let mut save_dirty = true;

        let mut result = smallvec![];
        for &(src, dst, data) in &moves {
            // Do we have a stack-to-stack move? If so, resolve.
            if self.is_stack_to_stack_move(src, dst) {
                trace!("scratch resolver: stack to stack: {:?} -> {:?}", src, dst);

                // If the selected scratch register is stolen from the
                // set of in-use registers, then we need to save the
                // current contents of the scratch register before using
                // it as a temporary.
                if let Some(save_slot) = save_slot {
                    // However we may have already done so for an earlier
                    // stack-to-stack move in which case we don't need
                    // to do it again.
                    if save_dirty {
                        debug_assert!(!scratch_dirty);
                        result.push((scratch_reg, save_slot, T::default()));
                        save_dirty = false;
                    }
                }

                // We can't move directly from one stack slot to another
                // on any architecture we care about, so stack-to-stack
                // moves must go via a scratch register.
                result.push((src, scratch_reg, data));
                result.push((scratch_reg, dst, data));
                scratch_dirty = true;
            } else {
                // This is not a stack-to-stack move, but we need to
                // make sure that the scratch register is in the correct
                // state if this move interacts with that register.
                if src == scratch_reg && scratch_dirty {
                    // We're copying from the scratch register so if
                    // it was stolen for a stack-to-stack move then we
                    // need to make sure it has the correct contents,
                    // not whatever was temporarily copied into it. If
                    // we got scratch_reg from find_free_reg then it
                    // had better not have been used as the source of
                    // a move. So if we're here it's because we fell
                    // back to the caller-provided last-resort scratch
                    // register, and we must therefore have a save-slot
                    // allocated too.
                    debug_assert!(!save_dirty);
                    let save_slot = save_slot.expect("move source should not be a free register");
                    result.push((save_slot, scratch_reg, T::default()));
                    scratch_dirty = false;
                }
                if dst == scratch_reg {
                    // We are writing something to the scratch register
                    // so it doesn't matter what was there before. We
                    // can avoid restoring it, but we will need to save
                    // it again before the next stack-to-stack move.
                    scratch_dirty = false;
                    save_dirty = true;
                }
                result.push((src, dst, data));
            }
        }

        // Now that all the stack-to-stack moves are done, restore the
        // scratch register if necessary.
        if let Some(save_slot) = save_slot {
            if scratch_dirty {
                debug_assert!(!save_dirty);
                result.push((save_slot, scratch_reg, T::default()));
            }
        }

        trace!("scratch resolver: got {:?}", result);
        result
    }

    fn is_stack_to_stack_move(&self, src: Allocation, dst: Allocation) -> bool {
        (self.is_stack_alloc)(src) && (self.is_stack_alloc)(dst)
    }
}
