//! A NaN-canonicalizing rewriting pass. Patch floating point arithmetic
//! instructions that may return a NaN result with a sequence of operations
//! that will replace nondeterministic NaN's with a single canonical NaN value.

use crate::cursor::{Cursor, FuncCursor};
use crate::ir::condcodes::FloatCC;
use crate::ir::immediates::{Ieee32, Ieee64};
use crate::ir::types::{self};
use crate::ir::{Function, Inst, InstBuilder, InstructionData, Opcode, Value};
use crate::opts::MemFlags;
use crate::timing;

/// Perform the NaN canonicalization pass.
pub fn do_nan_canonicalization(func: &mut Function, has_vector_support: bool) {
    let _tt = timing::canonicalize_nans();
    let mut pos = FuncCursor::new(func);
    while let Some(_block) = pos.next_block() {
        while let Some(inst) = pos.next_inst() {
            if is_fp_arith(&mut pos, inst) {
                add_nan_canon_seq(&mut pos, inst, has_vector_support);
            }
        }
    }
}

/// Returns true/false based on whether the instruction is a floating-point
/// arithmetic operation. This ignores operations like `fneg`, `fabs`, or
/// `fcopysign` that only operate on the sign bit of a floating point value.
fn is_fp_arith(pos: &mut FuncCursor, inst: Inst) -> bool {
    match pos.func.dfg.insts[inst] {
        InstructionData::Unary { opcode, .. } => {
            opcode == Opcode::Ceil
                || opcode == Opcode::Floor
                || opcode == Opcode::Nearest
                || opcode == Opcode::Sqrt
                || opcode == Opcode::Trunc
                || opcode == Opcode::Fdemote
                || opcode == Opcode::Fpromote
                || opcode == Opcode::FvpromoteLow
                || opcode == Opcode::Fvdemote
        }
        InstructionData::Binary { opcode, .. } => {
            opcode == Opcode::Fadd
                || opcode == Opcode::Fdiv
                || opcode == Opcode::Fmax
                || opcode == Opcode::Fmin
                || opcode == Opcode::Fmul
                || opcode == Opcode::Fsub
        }
        InstructionData::Ternary { opcode, .. } => opcode == Opcode::Fma,
        _ => false,
    }
}

/// Append a sequence of canonicalizing instructions after the given instruction.
fn add_nan_canon_seq(pos: &mut FuncCursor, inst: Inst, has_vector_support: bool) {
    // Select the instruction result, result type. Replace the instruction
    // result and step forward before inserting the canonicalization sequence.
    let val = pos.func.dfg.first_result(inst);
    let val_type = pos.func.dfg.value_type(val);
    let new_res = pos.func.dfg.replace_result(val, val_type);
    let _next_inst = pos.next_inst().expect("block missing terminator!");

    // Insert a comparison instruction, to check if `inst_res` is NaN (comparing
    // against NaN is always unordered). Select the canonical NaN value if `val`
    // is NaN, assign the result to `inst`.
    let comparison = FloatCC::Unordered;

    let vectorized_scalar_select = |pos: &mut FuncCursor, canon_nan: Value, ty: types::Type| {
        let canon_nan = pos.ins().scalar_to_vector(ty, canon_nan);
        let new_res = pos.ins().scalar_to_vector(ty, new_res);
        let is_nan = pos.ins().fcmp(comparison, new_res, new_res);
        let is_nan = pos.ins().bitcast(ty, MemFlags::new(), is_nan);
        let simd_result = pos.ins().bitselect(is_nan, canon_nan, new_res);
        pos.ins().with_result(val).extractlane(simd_result, 0);
    };
    let scalar_select = |pos: &mut FuncCursor, canon_nan: Value| {
        let is_nan = pos.ins().fcmp(comparison, new_res, new_res);
        pos.ins()
            .with_result(val)
            .select(is_nan, canon_nan, new_res);
    };

    let vector_select = |pos: &mut FuncCursor, canon_nan: Value| {
        let is_nan = pos.ins().fcmp(comparison, new_res, new_res);
        let is_nan = pos.ins().bitcast(val_type, MemFlags::new(), is_nan);
        pos.ins()
            .with_result(val)
            .bitselect(is_nan, canon_nan, new_res);
    };

    match val_type {
        types::F32 => {
            let canon_nan = pos.ins().f32const(Ieee32::NAN);
            if has_vector_support {
                vectorized_scalar_select(pos, canon_nan, types::F32X4);
            } else {
                scalar_select(pos, canon_nan);
            }
        }
        types::F64 => {
            let canon_nan = pos.ins().f64const(Ieee64::NAN);
            if has_vector_support {
                vectorized_scalar_select(pos, canon_nan, types::F64X2);
            } else {
                scalar_select(pos, canon_nan);
            }
        }
        types::F32X4 => {
            let canon_nan = pos.ins().f32const(Ieee32::NAN);
            let canon_nan = pos.ins().splat(types::F32X4, canon_nan);
            vector_select(pos, canon_nan);
        }
        types::F64X2 => {
            let canon_nan = pos.ins().f64const(Ieee64::NAN);
            let canon_nan = pos.ins().splat(types::F64X2, canon_nan);
            vector_select(pos, canon_nan);
        }
        _ => {
            // Panic if the type given was not an IEEE floating point type.
            panic!("Could not canonicalize NaN: Unexpected result type found.");
        }
    }

    pos.prev_inst(); // Step backwards so the pass does not skip instructions.
}
