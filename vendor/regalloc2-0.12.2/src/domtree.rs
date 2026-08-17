/*
 * Derives from the dominator tree implementation in regalloc.rs, which is
 * licensed under the Apache Public License 2.0 with LLVM Exception. See:
 * https://github.com/bytecodealliance/regalloc.rs
 */

// This is an implementation of the algorithm described in
//
//   A Simple, Fast Dominance Algorithm
//   Keith D. Cooper, Timothy J. Harvey, and Ken Kennedy
//   Department of Computer Science, Rice University, Houston, Texas, USA
//   TR-06-33870
//   https://www.cs.rice.edu/~keith/EMBED/dom.pdf

use core::u32;

use alloc::vec::Vec;

use crate::{Block, VecExt};

// Helper
fn merge_sets(
    idom: &[Block], // map from Block to Block
    block_to_rpo: &[Option<u32>],
    mut node1: Block,
    mut node2: Block,
) -> Block {
    while node1 != node2 {
        if node1.is_invalid() || node2.is_invalid() {
            return Block::invalid();
        }
        let rpo1 = block_to_rpo[node1.index()].unwrap();
        let rpo2 = block_to_rpo[node2.index()].unwrap();
        if rpo1 > rpo2 {
            node1 = idom[node1.index()];
        } else if rpo2 > rpo1 {
            node2 = idom[node2.index()];
        }
    }
    debug_assert!(node1 == node2);
    node1
}

pub fn calculate<'a, PredFn: Fn(Block) -> &'a [Block]>(
    num_blocks: usize,
    preds: PredFn,
    post_ord: &[Block],
    block_to_rpo_scratch: &mut Vec<Option<u32>>,
    out: &mut Vec<Block>,
    start: Block,
) {
    // We have post_ord, which is the postorder sequence.
    // Compute maps from RPO to block number and vice-versa.
    let block_to_rpo = block_to_rpo_scratch.repopulate(num_blocks, None);
    for (i, rpo_block) in post_ord.iter().rev().enumerate() {
        block_to_rpo[rpo_block.index()] = Some(i as u32);
    }

    let idom = out.repopulate(num_blocks, Block::invalid());
    // The start node must have itself as a parent.
    idom[start.index()] = start;

    let mut changed = true;
    while changed {
        changed = false;
        // Consider blocks in reverse postorder. Skip any that are unreachable.
        for &node in post_ord.iter().rev() {
            let rponum = block_to_rpo[node.index()].unwrap();

            let mut parent = Block::invalid();
            for &pred in preds(node).iter() {
                let pred_rpo = match block_to_rpo[pred.index()] {
                    None => {
                        // Skip unreachable preds.
                        continue;
                    }
                    Some(r) => r,
                };
                if pred_rpo < rponum {
                    parent = pred;
                    break;
                }
            }

            if parent.is_valid() {
                for &pred in preds(node).iter() {
                    if pred == parent {
                        continue;
                    }
                    if idom[pred.index()].is_invalid() {
                        continue;
                    }
                    parent = merge_sets(&idom, &block_to_rpo[..], parent, pred);
                }
            }

            if parent.is_valid() && parent != idom[node.index()] {
                idom[node.index()] = parent;
                changed = true;
            }
        }
    }

    // Now set the start node's dominator-tree parent to "invalid";
    // this allows the loop in `dominates` to terminate.
    idom[start.index()] = Block::invalid();
}

pub fn dominates(idom: &[Block], a: Block, mut b: Block) -> bool {
    loop {
        if a == b {
            return true;
        }
        if b.is_invalid() {
            return false;
        }
        b = idom[b.index()];
    }
}
