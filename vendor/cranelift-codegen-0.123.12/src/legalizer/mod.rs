//! Legalize instructions.
//!
//! A legal instruction is one that can be mapped directly to a machine code instruction for the
//! target ISA. The `legalize_function()` function takes as input any function and transforms it
//! into an equivalent function using only legal instructions.
//!
//! The characteristics of legal instructions depend on the target ISA, so any given instruction
//! can be legal for one ISA and illegal for another.
//!
//! Besides transforming instructions, the legalizer also fills out the `function.encodings` map
//! which provides a legal encoding recipe for every instruction.
//!
//! The legalizer does not deal with register allocation constraints. These constraints are derived
//! from the encoding recipes, and solved later by the register allocator.

use crate::cursor::{Cursor, FuncCursor};
use crate::ir::immediates::Imm64;
use crate::ir::types::{self, I64, I128};
use crate::ir::{self, InstBuilder, InstructionData, MemFlags, Value};
use crate::isa::TargetIsa;
use crate::trace;
use cranelift_entity::EntitySet;
use smallvec::SmallVec;

mod branch_to_trap;
mod globalvalue;

use self::branch_to_trap::BranchToTrap;
use self::globalvalue::expand_global_value;

fn imm_const(pos: &mut FuncCursor, arg: Value, imm: Imm64, is_signed: bool) -> Value {
    let ty = pos.func.dfg.value_type(arg);
    match (ty, is_signed) {
        (I128, true) => {
            let imm = pos.ins().iconst(I64, imm);
            pos.ins().sextend(I128, imm)
        }
        (I128, false) => {
            let imm = pos.ins().iconst(I64, imm);
            pos.ins().uextend(I128, imm)
        }
        _ => {
            let bits = imm.bits();
            let unsigned = match ty.lane_type() {
                types::I8 => bits as u8 as i64,
                types::I16 => bits as u16 as i64,
                types::I32 => bits as u32 as i64,
                types::I64 => bits,
                _ => unreachable!(),
            };
            pos.ins().iconst(ty.lane_type(), unsigned)
        }
    }
}

/// A command describing how the walk over instructions should proceed.
enum WalkCommand {
    /// Continue walking to the next instruction, if any.
    Continue,
    /// Revisit the current instruction (presumably because it was legalized
    /// into a new instruction that may also require further legalization).
    Revisit,
}

/// A simple, naive backwards walk over every instruction in every block in the
/// function's layout.
///
/// This does not guarantee any kind of reverse post-order visitation or
/// anything like that, it is just iterating over blocks in reverse layout
/// order, not any kind of control-flow graph visitation order.
///
/// The `f` visitor closure controls how the walk proceeds via its `WalkCommand`
/// result.
fn backward_walk(
    func: &mut ir::Function,
    mut f: impl FnMut(&mut ir::Function, ir::Block, ir::Inst) -> WalkCommand,
) {
    let mut pos = FuncCursor::new(func);
    while let Some(block) = pos.prev_block() {
        let mut prev_pos;
        while let Some(inst) = {
            prev_pos = pos.position();
            pos.prev_inst()
        } {
            match f(pos.func, block, inst) {
                WalkCommand::Continue => continue,
                WalkCommand::Revisit => pos.set_position(prev_pos),
            }
        }
    }
}

/// Perform a simple legalization by expansion of the function, without
/// platform-specific transforms.
pub fn simple_legalize(func: &mut ir::Function, isa: &dyn TargetIsa) {
    trace!("Pre-legalization function:\n{}", func.display());

    let mut branch_to_trap = BranchToTrap::default();

    // We walk the IR backwards because in practice, given the way that
    // frontends tend to produce CLIF, this means we will visit in roughly
    // reverse post order, which is helpful for getting the most optimizations
    // out of the `branch-to-trap` pass that we can (it must analyze trapping
    // blocks before it can rewrite branches to them) but the order does not
    // actually affect correctness.
    backward_walk(func, |func, block, inst| match func.dfg.insts[inst] {
        InstructionData::Trap {
            opcode: ir::Opcode::Trap,
            code: _,
        } => {
            branch_to_trap.analyze_trapping_block(func, block);
            WalkCommand::Continue
        }
        InstructionData::Brif {
            opcode: ir::Opcode::Brif,
            arg,
            blocks,
        } => {
            branch_to_trap.process_brif(func, inst, arg, blocks);
            WalkCommand::Continue
        }

        InstructionData::UnaryGlobalValue {
            opcode: ir::Opcode::GlobalValue,
            global_value,
        } => expand_global_value(inst, func, isa, global_value),

        InstructionData::StackLoad {
            opcode: ir::Opcode::StackLoad,
            stack_slot,
            offset,
        } => expand_stack_load(isa, func, inst, stack_slot, offset),

        InstructionData::StackStore {
            opcode: ir::Opcode::StackStore,
            arg,
            stack_slot,
            offset,
        } => expand_stack_store(isa, func, inst, arg, stack_slot, offset),

        InstructionData::DynamicStackLoad {
            opcode: ir::Opcode::DynamicStackLoad,
            dynamic_stack_slot,
        } => expand_dynamic_stack_load(isa, func, inst, dynamic_stack_slot),

        InstructionData::DynamicStackStore {
            opcode: ir::Opcode::DynamicStackStore,
            arg,
            dynamic_stack_slot,
        } => expand_dynamic_stack_store(isa, func, inst, arg, dynamic_stack_slot),

        InstructionData::BinaryImm64 { opcode, arg, imm } => {
            expand_binary_imm64(func, inst, opcode, arg, imm)
        }

        InstructionData::IntCompareImm {
            opcode: ir::Opcode::IcmpImm,
            cond,
            arg,
            imm,
        } => expand_icmp_imm(func, inst, cond, arg, imm),

        InstructionData::Binary { opcode, args } => expand_binary(func, inst, opcode, args),

        _ => WalkCommand::Continue,
    });

    trace!("Post-legalization function:\n{}", func.display());
}

fn expand_binary(
    func: &mut ir::Function,
    inst: ir::Inst,
    opcode: ir::Opcode,
    args: [ir::Value; 2],
) -> WalkCommand {
    let mut pos = FuncCursor::new(func);
    pos.goto_inst(inst);

    // Legalize the fused bitwise-plus-not instructions into simpler
    // instructions to assist with optimizations. Lowering will pattern match
    // this sequence regardless when architectures support the instruction
    // natively.
    match opcode {
        ir::Opcode::BandNot => {
            let neg = pos.ins().bnot(args[1]);
            pos.func.dfg.replace(inst).band(args[0], neg);
        }
        ir::Opcode::BorNot => {
            let neg = pos.ins().bnot(args[1]);
            pos.func.dfg.replace(inst).bor(args[0], neg);
        }
        ir::Opcode::BxorNot => {
            let neg = pos.ins().bnot(args[1]);
            pos.func.dfg.replace(inst).bxor(args[0], neg);
        }
        _ => {}
    }

    WalkCommand::Continue
}

fn expand_icmp_imm(
    func: &mut ir::Function,
    inst: ir::Inst,
    cond: ir::condcodes::IntCC,
    arg: Value,
    imm: Imm64,
) -> WalkCommand {
    let mut pos = FuncCursor::new(func);
    pos.goto_inst(inst);

    let imm = imm_const(&mut pos, arg, imm, true);
    pos.func.dfg.replace(inst).icmp(cond, arg, imm);

    WalkCommand::Continue
}

fn expand_binary_imm64(
    func: &mut ir::Function,
    inst: ir::Inst,
    opcode: ir::Opcode,
    arg: Value,
    imm: Imm64,
) -> WalkCommand {
    let mut pos = FuncCursor::new(func);
    pos.goto_inst(inst);

    let is_signed = match opcode {
        ir::Opcode::IaddImm
        | ir::Opcode::IrsubImm
        | ir::Opcode::ImulImm
        | ir::Opcode::SdivImm
        | ir::Opcode::SremImm => true,
        _ => false,
    };

    let imm = imm_const(&mut pos, arg, imm, is_signed);

    let replace = pos.func.dfg.replace(inst);
    match opcode {
        // bitops
        ir::Opcode::BandImm => {
            replace.band(arg, imm);
        }
        ir::Opcode::BorImm => {
            replace.bor(arg, imm);
        }
        ir::Opcode::BxorImm => {
            replace.bxor(arg, imm);
        }
        // bitshifting
        ir::Opcode::IshlImm => {
            replace.ishl(arg, imm);
        }
        ir::Opcode::RotlImm => {
            replace.rotl(arg, imm);
        }
        ir::Opcode::RotrImm => {
            replace.rotr(arg, imm);
        }
        ir::Opcode::SshrImm => {
            replace.sshr(arg, imm);
        }
        ir::Opcode::UshrImm => {
            replace.ushr(arg, imm);
        }
        // math
        ir::Opcode::IaddImm => {
            replace.iadd(arg, imm);
        }
        ir::Opcode::IrsubImm => {
            // note: arg order reversed
            replace.isub(imm, arg);
        }
        ir::Opcode::ImulImm => {
            replace.imul(arg, imm);
        }
        ir::Opcode::SdivImm => {
            replace.sdiv(arg, imm);
        }
        ir::Opcode::SremImm => {
            replace.srem(arg, imm);
        }
        ir::Opcode::UdivImm => {
            replace.udiv(arg, imm);
        }
        ir::Opcode::UremImm => {
            replace.urem(arg, imm);
        }
        _ => {}
    }

    WalkCommand::Continue
}

fn expand_dynamic_stack_store(
    isa: &dyn TargetIsa,
    func: &mut ir::Function,
    inst: ir::Inst,
    arg: Value,
    dynamic_stack_slot: ir::DynamicStackSlot,
) -> WalkCommand {
    let mut pos = FuncCursor::new(func);
    pos.goto_inst(inst);
    pos.use_srcloc(inst);

    let vector_ty = pos.func.dfg.value_type(arg);
    assert!(vector_ty.is_dynamic_vector());

    let addr_ty = isa.pointer_type();
    let addr = pos.ins().dynamic_stack_addr(addr_ty, dynamic_stack_slot);

    let mut mflags = MemFlags::new();
    // Stack slots are required to be accessible and aligned.
    mflags.set_notrap();
    mflags.set_aligned();

    pos.func.dfg.replace(inst).store(mflags, arg, addr, 0);

    WalkCommand::Continue
}

fn expand_dynamic_stack_load(
    isa: &dyn TargetIsa,
    func: &mut ir::Function,
    inst: ir::Inst,
    dynamic_stack_slot: ir::DynamicStackSlot,
) -> WalkCommand {
    let mut pos = FuncCursor::new(func).at_inst(inst);
    pos.use_srcloc(inst);

    let ty = pos.func.dfg.value_type(pos.func.dfg.first_result(inst));
    assert!(ty.is_dynamic_vector());

    let addr_ty = isa.pointer_type();
    let addr = pos.ins().dynamic_stack_addr(addr_ty, dynamic_stack_slot);

    // Stack slots are required to be accessible and aligned.
    let mflags = MemFlags::trusted();

    pos.func.dfg.replace(inst).load(ty, mflags, addr, 0);

    WalkCommand::Continue
}

fn expand_stack_store(
    isa: &dyn TargetIsa,
    func: &mut ir::Function,
    inst: ir::Inst,
    arg: ir::Value,
    stack_slot: ir::StackSlot,
    offset: ir::immediates::Offset32,
) -> WalkCommand {
    let mut pos = FuncCursor::new(func).at_inst(inst);
    pos.use_srcloc(inst);

    let addr_ty = isa.pointer_type();
    let addr = pos.ins().stack_addr(addr_ty, stack_slot, offset);

    // Stack slots are required to be accessible.
    // We can't currently ensure that they are aligned.
    let mut mflags = MemFlags::new();
    mflags.set_notrap();

    pos.func.dfg.replace(inst).store(mflags, arg, addr, 0);

    WalkCommand::Continue
}

fn expand_stack_load(
    isa: &dyn TargetIsa,
    func: &mut ir::Function,
    inst: ir::Inst,
    stack_slot: ir::StackSlot,
    offset: ir::immediates::Offset32,
) -> WalkCommand {
    let mut pos = FuncCursor::new(func).at_inst(inst);
    pos.use_srcloc(inst);

    let ty = pos.func.dfg.value_type(pos.func.dfg.first_result(inst));
    let addr_ty = isa.pointer_type();

    let addr = pos.ins().stack_addr(addr_ty, stack_slot, offset);

    // Stack slots are required to be accessible.
    // We can't currently ensure that they are aligned.
    let mut mflags = MemFlags::new();
    mflags.set_notrap();

    pos.func.dfg.replace(inst).load(ty, mflags, addr, 0);

    WalkCommand::Continue
}
