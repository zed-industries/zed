//! Redundant-move elimination.

use crate::{Allocation, FxHashMap, VReg};
use smallvec::{smallvec, SmallVec};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RedundantMoveState {
    Copy(Allocation, Option<VReg>),
    Orig(VReg),
    None,
}
#[derive(Clone, Debug, Default)]
pub struct RedundantMoveEliminator {
    allocs: FxHashMap<Allocation, RedundantMoveState>,
    reverse_allocs: FxHashMap<Allocation, SmallVec<[Allocation; 4]>>,
}
#[derive(Copy, Clone, Debug)]
pub struct RedundantMoveAction {
    pub elide: bool,
}

impl RedundantMoveEliminator {
    pub fn process_move(
        &mut self,
        from: Allocation,
        to: Allocation,
        to_vreg: Option<VReg>,
    ) -> RedundantMoveAction {
        // Look up the src and dest.
        let from_state = self
            .allocs
            .get(&from)
            .map(|&p| p)
            .unwrap_or(RedundantMoveState::None);
        let to_state = self
            .allocs
            .get(&to)
            .map(|&p| p)
            .unwrap_or(RedundantMoveState::None);

        trace!(
            "     -> redundant move tracker: from {} to {} to_vreg {:?}",
            from,
            to,
            to_vreg
        );
        trace!(
            "       -> from_state {:?} to_state {:?}",
            from_state,
            to_state
        );

        if from == to && to_vreg.is_some() {
            self.clear_alloc(to);
            self.allocs
                .insert(to, RedundantMoveState::Orig(to_vreg.unwrap()));
            return RedundantMoveAction { elide: true };
        }

        let src_vreg = match from_state {
            RedundantMoveState::Copy(_, opt_r) => opt_r,
            RedundantMoveState::Orig(r) => Some(r),
            _ => None,
        };
        trace!("      -> src_vreg {:?}", src_vreg);
        let dst_vreg = to_vreg.or(src_vreg);
        trace!("      -> dst_vreg {:?}", dst_vreg);
        let existing_dst_vreg = match to_state {
            RedundantMoveState::Copy(_, opt_r) => opt_r,
            RedundantMoveState::Orig(r) => Some(r),
            _ => None,
        };
        trace!("      -> existing_dst_vreg {:?}", existing_dst_vreg);

        let elide = match (from_state, to_state) {
            (_, RedundantMoveState::Copy(orig_alloc, _)) if orig_alloc == from => true,
            (RedundantMoveState::Copy(new_alloc, _), _) if new_alloc == to => true,
            _ => false,
        };
        trace!("      -> elide {}", elide);

        // Invalidate all existing copies of `to` if `to` actually changed value.
        if !elide {
            self.clear_alloc(to);
        }

        // Set up forward and reverse mapping. Don't track stack-to-stack copies.
        if from.is_reg() || to.is_reg() {
            self.allocs
                .insert(to, RedundantMoveState::Copy(from, dst_vreg));
            trace!(
                "     -> create mapping {} -> {:?}",
                to,
                RedundantMoveState::Copy(from, dst_vreg)
            );
            self.reverse_allocs
                .entry(from)
                .or_insert_with(|| smallvec![])
                .push(to);
        }

        RedundantMoveAction { elide }
    }

    pub fn clear(&mut self) {
        trace!("   redundant move eliminator cleared");
        self.allocs.clear();
        self.reverse_allocs.clear();
    }

    pub fn clear_alloc(&mut self, alloc: Allocation) {
        trace!("   redundant move eliminator: clear {:?}", alloc);
        if let Some(ref mut existing_copies) = self.reverse_allocs.get_mut(&alloc) {
            for to_inval in existing_copies.drain(..) {
                trace!("     -> clear existing copy: {:?}", to_inval);
                if let Some(val) = self.allocs.get_mut(&to_inval) {
                    match val {
                        RedundantMoveState::Copy(_, Some(vreg)) => {
                            *val = RedundantMoveState::Orig(*vreg);
                        }
                        _ => *val = RedundantMoveState::None,
                    }
                }
                self.allocs.remove(&to_inval);
            }
        }
        self.allocs.remove(&alloc);
    }
}
