/*
 * Released under the terms of the Apache 2.0 license with LLVM
 * exception. See `LICENSE` for details.
 */

//! Fast postorder computation.

use crate::{Block, RegAllocError, VecExt};
use alloc::vec::Vec;
use smallvec::{smallvec, SmallVec};

pub fn calculate<'a, SuccFn: Fn(Block) -> &'a [Block]>(
    num_blocks: usize,
    entry: Block,
    visited_scratch: &mut Vec<bool>,
    out: &mut Vec<Block>,
    succ_blocks: SuccFn,
) -> Result<(), RegAllocError> {
    // State: visited-block map, and explicit DFS stack.
    struct State<'a> {
        block: Block,
        succs: core::slice::Iter<'a, Block>,
    }

    let visited = visited_scratch.repopulate(num_blocks, false);
    let mut stack: SmallVec<[State; 64]> = smallvec![];
    out.clear();

    let entry_visit = visited
        .get_mut(entry.index())
        .ok_or(RegAllocError::BB(entry))?;
    *entry_visit = true;
    stack.push(State {
        block: entry,
        succs: succ_blocks(entry).iter(),
    });

    while let Some(ref mut state) = stack.last_mut() {
        // Perform one action: push to new succ, skip an already-visited succ, or pop.
        if let Some(&succ) = state.succs.next() {
            let succ_visit = visited
                .get_mut(succ.index())
                .ok_or(RegAllocError::BB(succ))?;
            if !*succ_visit {
                *succ_visit = true;
                stack.push(State {
                    block: succ,
                    succs: succ_blocks(succ).iter(),
                });
            }
        } else {
            out.push(state.block);
            stack.pop();
        }
    }
    Ok(())
}
