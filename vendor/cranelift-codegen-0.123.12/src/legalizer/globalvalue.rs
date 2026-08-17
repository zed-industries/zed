//! Legalization of global values.
//!
//! This module exports the `expand_global_value` function which transforms a `global_value`
//! instruction into code that depends on the kind of global value referenced.

use super::WalkCommand;
use crate::cursor::{Cursor, FuncCursor};
use crate::ir::{self, InstBuilder, pcc::Fact};
use crate::isa::TargetIsa;

/// Expand a `global_value` instruction according to the definition of the global value.
pub fn expand_global_value(
    inst: ir::Inst,
    func: &mut ir::Function,
    isa: &dyn TargetIsa,
    global_value: ir::GlobalValue,
) -> WalkCommand {
    crate::trace!(
        "expanding global value: {:?}: {}",
        inst,
        func.dfg.display_inst(inst)
    );

    match func.global_values[global_value] {
        ir::GlobalValueData::VMContext => vmctx_addr(global_value, inst, func),
        ir::GlobalValueData::IAddImm {
            base,
            offset,
            global_type,
        } => iadd_imm_addr(inst, func, base, offset.into(), global_type),
        ir::GlobalValueData::Load {
            base,
            offset,
            global_type,
            flags,
        } => load_addr(inst, func, base, offset, global_type, flags, isa),
        ir::GlobalValueData::Symbol { tls, .. } => symbol(inst, func, global_value, isa, tls),
        ir::GlobalValueData::DynScaleTargetConst { vector_type } => {
            const_vector_scale(inst, func, vector_type, isa)
        }
    }
}

fn const_vector_scale(
    inst: ir::Inst,
    func: &mut ir::Function,
    ty: ir::Type,
    isa: &dyn TargetIsa,
) -> WalkCommand {
    assert!(ty.bytes() <= 16);

    // Use a minimum of 128-bits for the base type.
    let base_bytes = std::cmp::max(ty.bytes(), 16);
    let scale = (isa.dynamic_vector_bytes(ty) / base_bytes) as i64;
    assert!(scale > 0);
    let pos = FuncCursor::new(func).at_inst(inst);
    pos.func.dfg.replace(inst).iconst(isa.pointer_type(), scale);

    WalkCommand::Continue
}

/// Expand a `global_value` instruction for a vmctx global.
fn vmctx_addr(
    global_value: ir::GlobalValue,
    inst: ir::Inst,
    func: &mut ir::Function,
) -> WalkCommand {
    // Get the value representing the `vmctx` argument.
    let vmctx = func
        .special_param(ir::ArgumentPurpose::VMContext)
        .expect("Missing vmctx parameter");

    // Replace the `global_value` instruction's value with an alias to the vmctx arg.
    let result = func.dfg.first_result(inst);
    func.dfg.clear_results(inst);
    func.dfg.change_to_alias(result, vmctx);
    func.layout.remove_inst(inst);

    // If there was a fact on the GV, then copy it to the vmctx arg
    // blockparam def.
    if let Some(fact) = &func.global_value_facts[global_value] {
        if func.dfg.facts[vmctx].is_none() {
            let fact = fact.clone();
            func.dfg.facts[vmctx] = Some(fact);
        }
    }

    // We removed the instruction, so `cursor.next_inst()` will fail if we
    // return `WalkCommand::Continue`; instead "revisit" the current
    // instruction, which will be the next instruction since we removed the
    // current instruction.
    WalkCommand::Revisit
}

/// Expand a `global_value` instruction for an iadd_imm global.
fn iadd_imm_addr(
    inst: ir::Inst,
    func: &mut ir::Function,
    base: ir::GlobalValue,
    offset: i64,
    global_type: ir::Type,
) -> WalkCommand {
    let mut pos = FuncCursor::new(func).at_inst(inst);

    // Get the value for the lhs.
    let lhs = pos.ins().global_value(global_type, base);
    if let Some(fact) = &pos.func.global_value_facts[base] {
        pos.func.dfg.facts[lhs] = Some(fact.clone());
    }

    // Generate the constant and attach a fact to the constant if
    // there is a fact on the base.
    let constant = pos.ins().iconst(global_type, offset);
    if pos.func.global_value_facts[base].is_some() {
        let bits = u16::try_from(global_type.bits()).unwrap();
        let unsigned_offset = offset as u64; // Safety: reinterpret i64 bits as u64.
        pos.func.dfg.facts[constant] = Some(Fact::constant(bits, unsigned_offset));
    }

    // Simply replace the `global_value` instruction with an `iadd_imm`, reusing the result value.
    pos.func.dfg.replace(inst).iadd(lhs, constant);

    // Need to legalize the `global_value` that we emitted.
    WalkCommand::Revisit
}

/// Expand a `global_value` instruction for a load global.
fn load_addr(
    inst: ir::Inst,
    func: &mut ir::Function,
    base: ir::GlobalValue,
    offset: ir::immediates::Offset32,
    global_type: ir::Type,
    flags: ir::MemFlags,
    isa: &dyn TargetIsa,
) -> WalkCommand {
    // We need to load a pointer from the `base` global value, so insert a new `global_value`
    // instruction. This depends on the iterative legalization loop. Note that the IR verifier
    // detects any cycles in the `load` globals.
    let ptr_ty = isa.pointer_type();

    let mut pos = FuncCursor::new(func).at_inst(inst);
    pos.use_srcloc(inst);

    // Get the value for the base.
    let base_addr = pos.ins().global_value(ptr_ty, base);
    if let Some(fact) = &pos.func.global_value_facts[base] {
        pos.func.dfg.facts[base_addr] = Some(fact.clone());
    }

    // Perform the load.
    pos.func
        .dfg
        .replace(inst)
        .load(global_type, flags, base_addr, offset);

    // Need to legalize the `global_value` for the base address.
    WalkCommand::Revisit
}

/// Expand a `global_value` instruction for a symbolic name global.
fn symbol(
    inst: ir::Inst,
    func: &mut ir::Function,
    gv: ir::GlobalValue,
    isa: &dyn TargetIsa,
    tls: bool,
) -> WalkCommand {
    let ptr_ty = isa.pointer_type();

    if tls {
        func.dfg.replace(inst).tls_value(ptr_ty, gv);
    } else {
        func.dfg.replace(inst).symbol_value(ptr_ty, gv);
    }

    WalkCommand::Continue
}
