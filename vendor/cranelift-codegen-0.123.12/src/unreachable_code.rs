//! Unreachable code elimination.

use cranelift_entity::EntitySet;

use crate::cursor::{Cursor, FuncCursor};
use crate::dominator_tree::DominatorTree;
use crate::flowgraph::ControlFlowGraph;
use crate::timing;
use crate::{ir, trace};

/// Eliminate unreachable code.
///
/// This pass deletes whole blocks that can't be reached from the entry block. It does not delete
/// individual instructions whose results are unused.
///
/// The reachability analysis is performed by the dominator tree analysis.
pub fn eliminate_unreachable_code(
    func: &mut ir::Function,
    cfg: &mut ControlFlowGraph,
    domtree: &DominatorTree,
) {
    let _tt = timing::unreachable_code();
    let mut pos = FuncCursor::new(func);
    let mut used_tables = EntitySet::with_capacity(pos.func.stencil.dfg.jump_tables.len());
    let mut used_exception_tables =
        EntitySet::with_capacity(pos.func.stencil.dfg.exception_tables.len());
    while let Some(block) = pos.next_block() {
        if domtree.is_reachable(block) {
            let inst = pos.func.layout.last_inst(block).unwrap();
            match pos.func.dfg.insts[inst] {
                ir::InstructionData::BranchTable { table, .. } => {
                    used_tables.insert(table);
                }
                ir::InstructionData::TryCall { exception, .. }
                | ir::InstructionData::TryCallIndirect { exception, .. } => {
                    used_exception_tables.insert(exception);
                }
                _ => (),
            }
            continue;
        }

        trace!("Eliminating unreachable {}", block);
        // Move the cursor out of the way and make sure the next lop iteration goes to the right
        // block.
        pos.prev_block();

        // Remove all instructions from `block`.
        while let Some(inst) = pos.func.layout.first_inst(block) {
            trace!(" - {}", pos.func.dfg.display_inst(inst));
            pos.func.layout.remove_inst(inst);
        }

        // Once the block is completely empty, we can update the CFG which removes it from any
        // predecessor lists.
        cfg.recompute_block(pos.func, block);

        // Finally, remove the block from the layout.
        pos.func.layout.remove_block(block);
    }

    for (table, jt_data) in func.stencil.dfg.jump_tables.iter_mut() {
        if !used_tables.contains(table) {
            jt_data.clear();
        }
    }

    for (exception, exception_data) in func.stencil.dfg.exception_tables.iter_mut() {
        if !used_exception_tables.contains(exception) {
            exception_data.clear();
        }
    }
}
