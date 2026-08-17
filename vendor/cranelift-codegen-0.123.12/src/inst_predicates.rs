//! Instruction predicates/properties, shared by various analyses.
use crate::ir::immediates::Offset32;
use crate::ir::{self, Block, Function, Inst, InstructionData, Opcode, Type, Value};

/// Test whether the given opcode is unsafe to even consider as side-effect-free.
#[inline(always)]
fn trivially_has_side_effects(opcode: Opcode) -> bool {
    opcode.is_call()
        || opcode.is_branch()
        || opcode.is_terminator()
        || opcode.is_return()
        || opcode.can_trap()
        || opcode.other_side_effects()
        || opcode.can_store()
}

/// Load instructions without the `notrap` flag are defined to trap when
/// operating on inaccessible memory, so we can't treat them as side-effect-free even if the loaded
/// value is unused.
#[inline(always)]
fn is_load_with_defined_trapping(opcode: Opcode, data: &InstructionData) -> bool {
    if !opcode.can_load() {
        return false;
    }
    match *data {
        InstructionData::StackLoad { .. } => false,
        InstructionData::Load { flags, .. } => !flags.notrap(),
        _ => true,
    }
}

/// Does the given instruction have any side-effect that would preclude it from being removed when
/// its value is unused?
#[inline(always)]
fn has_side_effect(func: &Function, inst: Inst) -> bool {
    let data = &func.dfg.insts[inst];
    let opcode = data.opcode();
    trivially_has_side_effects(opcode) || is_load_with_defined_trapping(opcode, data)
}

/// Does the given instruction behave as a "pure" node with respect to
/// aegraph semantics?
///
/// - Trivially pure nodes (bitwise arithmetic, etc)
/// - Loads with the `readonly`, `notrap`, and `can_move` flags set
pub fn is_pure_for_egraph(func: &Function, inst: Inst) -> bool {
    let is_pure_load = match func.dfg.insts[inst] {
        InstructionData::Load {
            opcode: Opcode::Load,
            flags,
            ..
        } => flags.readonly() && flags.notrap() && flags.can_move(),
        _ => false,
    };

    // Multi-value results do not play nicely with much of the egraph
    // infrastructure. They are in practice used only for multi-return
    // calls and some other odd instructions (e.g. uadd_overflow) which,
    // for now, we can afford to leave in place as opaque
    // side-effecting ops. So if more than one result, then the inst
    // is "not pure". Similarly, ops with zero results can be used
    // only for their side-effects, so are never pure. (Or if they
    // are, we can always trivially eliminate them with no effect.)
    let has_one_result = func.dfg.inst_results(inst).len() == 1;

    let op = func.dfg.insts[inst].opcode();

    has_one_result && (is_pure_load || (!op.can_load() && !trivially_has_side_effects(op)))
}

/// Can the given instruction be merged into another copy of itself?
/// These instructions may have side-effects, but as long as we retain
/// the first instance of the instruction, the second and further
/// instances are redundant if they would produce the same trap or
/// result.
pub fn is_mergeable_for_egraph(func: &Function, inst: Inst) -> bool {
    let op = func.dfg.insts[inst].opcode();
    // We can only merge zero- and one-result operators due to the way that GVN
    // is structured in the egraph implementation.
    func.dfg.inst_results(inst).len() <= 1
        // Loads/stores are handled by alias analysis and not
        // otherwise mergeable.
        && !op.can_load()
        && !op.can_store()
        // Can only have idempotent side-effects.
        && (!has_side_effect(func, inst) || op.side_effects_idempotent())
}

/// Does the given instruction have any side-effect as per [has_side_effect], or else is a load,
/// but not the get_pinned_reg opcode?
pub fn has_lowering_side_effect(func: &Function, inst: Inst) -> bool {
    let op = func.dfg.insts[inst].opcode();
    op != Opcode::GetPinnedReg && (has_side_effect(func, inst) || op.can_load())
}

/// Is the given instruction a constant value (`iconst`, `fconst`) that can be
/// represented in 64 bits?
pub fn is_constant_64bit(func: &Function, inst: Inst) -> Option<u64> {
    match &func.dfg.insts[inst] {
        &InstructionData::UnaryImm { imm, .. } => Some(imm.bits() as u64),
        &InstructionData::UnaryIeee16 { imm, .. } => Some(imm.bits() as u64),
        &InstructionData::UnaryIeee32 { imm, .. } => Some(imm.bits() as u64),
        &InstructionData::UnaryIeee64 { imm, .. } => Some(imm.bits()),
        _ => None,
    }
}

/// Get the address, offset, and access type from the given instruction, if any.
pub fn inst_addr_offset_type(func: &Function, inst: Inst) -> Option<(Value, Offset32, Type)> {
    match &func.dfg.insts[inst] {
        InstructionData::Load { arg, offset, .. } => {
            let ty = func.dfg.value_type(func.dfg.inst_results(inst)[0]);
            Some((*arg, *offset, ty))
        }
        InstructionData::LoadNoOffset { arg, .. } => {
            let ty = func.dfg.value_type(func.dfg.inst_results(inst)[0]);
            Some((*arg, 0.into(), ty))
        }
        InstructionData::Store { args, offset, .. } => {
            let ty = func.dfg.value_type(args[0]);
            Some((args[1], *offset, ty))
        }
        InstructionData::StoreNoOffset { args, .. } => {
            let ty = func.dfg.value_type(args[0]);
            Some((args[1], 0.into(), ty))
        }
        _ => None,
    }
}

/// Get the store data, if any, from an instruction.
pub fn inst_store_data(func: &Function, inst: Inst) -> Option<Value> {
    match &func.dfg.insts[inst] {
        InstructionData::Store { args, .. } | InstructionData::StoreNoOffset { args, .. } => {
            Some(args[0])
        }
        _ => None,
    }
}

/// Determine whether this opcode behaves as a memory fence, i.e.,
/// prohibits any moving of memory accesses across it.
pub fn has_memory_fence_semantics(op: Opcode) -> bool {
    match op {
        Opcode::AtomicRmw
        | Opcode::AtomicCas
        | Opcode::AtomicLoad
        | Opcode::AtomicStore
        | Opcode::Fence
        | Opcode::Debugtrap => true,
        Opcode::Call | Opcode::CallIndirect | Opcode::TryCall | Opcode::TryCallIndirect => true,
        op if op.can_trap() => true,
        _ => false,
    }
}

/// Visit all successors of a block with a given visitor closure. The closure
/// arguments are the branch instruction that is used to reach the successor,
/// the successor block itself, and a flag indicating whether the block is
/// branched to via a table entry.
pub(crate) fn visit_block_succs<F: FnMut(Inst, Block, bool)>(
    f: &Function,
    block: Block,
    mut visit: F,
) {
    if let Some(inst) = f.layout.last_inst(block) {
        match &f.dfg.insts[inst] {
            ir::InstructionData::Jump {
                destination: dest, ..
            } => {
                visit(inst, dest.block(&f.dfg.value_lists), false);
            }

            ir::InstructionData::Brif {
                blocks: [block_then, block_else],
                ..
            } => {
                visit(inst, block_then.block(&f.dfg.value_lists), false);
                visit(inst, block_else.block(&f.dfg.value_lists), false);
            }

            ir::InstructionData::BranchTable { table, .. } => {
                let pool = &f.dfg.value_lists;
                let table = &f.stencil.dfg.jump_tables[*table];

                // The default block is reached via a direct conditional branch,
                // so it is not part of the table. We visit the default block
                // first explicitly, to mirror the traversal order of
                // `JumpTableData::all_branches`, and transitively the order of
                // `InstructionData::branch_destination`.
                //
                // Additionally, this case is why we are unable to replace this
                // whole function with a loop over `branch_destination`: we need
                // to report which branch targets come from the table vs the
                // default.
                visit(inst, table.default_block().block(pool), false);

                for dest in table.as_slice() {
                    visit(inst, dest.block(pool), true);
                }
            }

            ir::InstructionData::TryCall { exception, .. }
            | ir::InstructionData::TryCallIndirect { exception, .. } => {
                let pool = &f.dfg.value_lists;
                let exdata = &f.stencil.dfg.exception_tables[*exception];

                for dest in exdata.all_branches() {
                    visit(inst, dest.block(pool), false);
                }
            }

            inst => debug_assert!(!inst.opcode().is_branch()),
        }
    }
}
