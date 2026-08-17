//! Harvest left-hand side superoptimization candidates.
//!
//! Given a clif function, harvest all its integer subexpressions, so that they
//! can be fed into [Souper](https://github.com/google/souper) as candidates for
//! superoptimization. For some of these candidates, Souper will successfully
//! synthesize a right-hand side that is equivalent but has lower cost than the
//! left-hand side. Then, we can combine these left- and right-hand sides into a
//! complete optimization, and add it to our peephole passes.
//!
//! To harvest the expression that produced a given value `x`, we do a
//! post-order traversal of the dataflow graph starting from `x`. As we do this
//! traversal, we maintain a map from clif values to their translated Souper
//! values. We stop traversing when we reach anything that can't be translated
//! into Souper IR: a memory load, a float-to-int conversion, a block parameter,
//! etc. For values produced by these instructions, we create a Souper `var`,
//! which is an input variable to the optimization. For instructions that have a
//! direct mapping into Souper IR, we get the Souper version of each of its
//! operands and then create the Souper version of the instruction itself. It
//! should now be clear why we do a post-order traversal: we need an
//! instruction's translated operands in order to translate the instruction
//! itself. Once this instruction is translated, we update the clif-to-souper
//! map with this new translation so that any other instruction that uses this
//! result as an operand has access to the translated value. When the traversal
//! is complete we return the translation of `x` as the root of left-hand side
//! candidate.

use crate::ir;
use souper_ir::ast;
use std::collections::{HashMap, HashSet};
use std::string::String;
use std::sync::mpsc;
use std::vec::Vec;

/// Harvest Souper left-hand side candidates from the given function.
///
/// Candidates are reported through the given MPSC sender.
pub fn do_souper_harvest(func: &ir::Function, out: &mut mpsc::Sender<String>) {
    let mut allocs = Allocs::default();

    // Iterate over each instruction in each block and try and harvest a
    // left-hand side from its result.
    for block in func.layout.blocks() {
        let mut option_inst = func.layout.first_inst(block);
        while let Some(inst) = option_inst {
            let results = func.dfg.inst_results(inst);
            if results.len() == 1 {
                let val = results[0];
                let ty = func.dfg.value_type(val);
                if ty.is_int() && ty.lane_count() == 1 {
                    harvest_candidate_lhs(&mut allocs, func, val, out);
                }
            }
            option_inst = func.layout.next_inst(inst);
        }
    }
}

/// Allocations that we reuse across many LHS candidate harvests.
#[derive(Default)]
struct Allocs {
    /// A map from cranelift IR to souper IR for values that we've already
    /// translated into souper IR.
    ir_to_souper_val: HashMap<ir::Value, ast::ValueId>,

    /// Stack of to-visit and to-trace values for the post-order DFS.
    dfs_stack: Vec<StackEntry>,

    /// Set of values we've already seen in our post-order DFS.
    dfs_seen: HashSet<ir::Value>,
}

impl Allocs {
    /// Reset the collections to their empty state (without deallocating their
    /// backing data).
    fn reset(&mut self) {
        self.ir_to_souper_val.clear();
        self.dfs_stack.clear();
        self.dfs_seen.clear();
    }
}

/// Harvest a candidate LHS for `val` from the dataflow graph.
fn harvest_candidate_lhs(
    allocs: &mut Allocs,
    func: &ir::Function,
    val: ir::Value,
    out: &mut mpsc::Sender<String>,
) {
    allocs.reset();
    let mut lhs = ast::LeftHandSideBuilder::default();
    let mut non_var_count = 0;

    // Should we keep tracing through the given `val`? Only if it is defined
    // by an instruction that we can translate to Souper IR.
    let should_trace = |val| match func.dfg.value_def(val) {
        ir::ValueDef::Result(inst, 0) => match func.dfg.insts[inst].opcode() {
                ir::Opcode::Iadd
                | ir::Opcode::IaddImm
                | ir::Opcode::IrsubImm
                | ir::Opcode::Imul
                | ir::Opcode::ImulImm
                | ir::Opcode::Udiv
                | ir::Opcode::UdivImm
                | ir::Opcode::Sdiv
                | ir::Opcode::SdivImm
                | ir::Opcode::Urem
                | ir::Opcode::UremImm
                | ir::Opcode::Srem
                | ir::Opcode::SremImm
                | ir::Opcode::Band
                | ir::Opcode::BandImm
                | ir::Opcode::Bor
                | ir::Opcode::BorImm
                | ir::Opcode::Bxor
                | ir::Opcode::BxorImm
                | ir::Opcode::Ishl
                | ir::Opcode::IshlImm
                | ir::Opcode::Sshr
                | ir::Opcode::SshrImm
                | ir::Opcode::Ushr
                | ir::Opcode::UshrImm
                | ir::Opcode::Select
                | ir::Opcode::Uextend
                | ir::Opcode::Sextend
                | ir::Opcode::Trunc
                | ir::Opcode::Icmp
                | ir::Opcode::Popcnt
                | ir::Opcode::Bitrev
                | ir::Opcode::Clz
                | ir::Opcode::Ctz
                // TODO: ir::Opcode::IaddCarry
                | ir::Opcode::SaddSat
                | ir::Opcode::SsubSat
                | ir::Opcode::UsubSat => true,
                _ => false,
            },
        _ => false,
    };

    post_order_dfs(allocs, &func.dfg, val, should_trace, |allocs, val| {
        let souper_assignment_rhs = match func.dfg.value_def(val) {
            ir::ValueDef::Result(inst, 0) => {
                let args = func.dfg.inst_args(inst);

                // Get the n^th argument as a souper operand.
                let arg = |allocs: &mut Allocs, n| {
                    let arg = args[n];
                    if let Some(a) = allocs.ir_to_souper_val.get(&arg).copied() {
                        a.into()
                    } else {
                        // The only arguments we get that we haven't already
                        // converted into a souper instruction are `iconst`s.
                        // This is because souper only allows
                        // constants as operands, and it doesn't allow assigning
                        // constants to a variable name. So we lazily convert
                        // `iconst`s into souper operands here,
                        // when they are actually used.
                        match func.dfg.value_def(arg) {
                            ir::ValueDef::Result(inst, 0) => match func.dfg.insts[inst] {
                                ir::InstructionData::UnaryImm { opcode, imm } => {
                                    debug_assert_eq!(opcode, ir::Opcode::Iconst);
                                    let imm: i64 = imm.into();
                                    ast::Operand::Constant(ast::Constant {
                                        value: imm.into(),
                                        r#type: souper_type_of(&func.dfg, arg),
                                    })
                                }
                                _ => unreachable!(
                                    "only iconst instructions \
                                     aren't in `ir_to_souper_val`"
                                ),
                            },
                            _ => unreachable!(
                                "only iconst instructions \
                                 aren't in `ir_to_souper_val`"
                            ),
                        }
                    }
                };

                match (func.dfg.insts[inst].opcode(), &func.dfg.insts[inst]) {
                    (ir::Opcode::Iadd, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::Add { a, b }.into()
                    }
                    (ir::Opcode::IaddImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let a = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let b = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::Add { a, b }.into()
                    }
                    (ir::Opcode::IrsubImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let b = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let a = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::Sub { a, b }.into()
                    }
                    (ir::Opcode::Imul, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::Mul { a, b }.into()
                    }
                    (ir::Opcode::ImulImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let a = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let b = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::Mul { a, b }.into()
                    }
                    (ir::Opcode::Udiv, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::Udiv { a, b }.into()
                    }
                    (ir::Opcode::UdivImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let a = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let b = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::Udiv { a, b }.into()
                    }
                    (ir::Opcode::Sdiv, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::Sdiv { a, b }.into()
                    }
                    (ir::Opcode::SdivImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let a = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let b = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::Sdiv { a, b }.into()
                    }
                    (ir::Opcode::Urem, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::Urem { a, b }.into()
                    }
                    (ir::Opcode::UremImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let a = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let b = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::Urem { a, b }.into()
                    }
                    (ir::Opcode::Srem, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::Srem { a, b }.into()
                    }
                    (ir::Opcode::SremImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let a = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let b = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::Srem { a, b }.into()
                    }
                    (ir::Opcode::Band, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::And { a, b }.into()
                    }
                    (ir::Opcode::BandImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let a = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let b = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::And { a, b }.into()
                    }
                    (ir::Opcode::Bor, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::Or { a, b }.into()
                    }
                    (ir::Opcode::BorImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let a = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let b = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::Or { a, b }.into()
                    }
                    (ir::Opcode::Bxor, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::Xor { a, b }.into()
                    }
                    (ir::Opcode::BxorImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let a = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let b = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::Xor { a, b }.into()
                    }
                    (ir::Opcode::Ishl, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::Shl { a, b }.into()
                    }
                    (ir::Opcode::IshlImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let a = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let b = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::Shl { a, b }.into()
                    }
                    (ir::Opcode::Sshr, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::Ashr { a, b }.into()
                    }
                    (ir::Opcode::SshrImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let a = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let b = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::Ashr { a, b }.into()
                    }
                    (ir::Opcode::Ushr, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::Lshr { a, b }.into()
                    }
                    (ir::Opcode::UshrImm, ir::InstructionData::BinaryImm64 { imm, .. }) => {
                        let a = arg(allocs, 0);
                        let value: i64 = (*imm).into();
                        let value: i128 = value.into();
                        let b = ast::Constant {
                            value,
                            r#type: souper_type_of(&func.dfg, val),
                        }
                        .into();
                        ast::Instruction::Lshr { a, b }.into()
                    }
                    (ir::Opcode::Select, _) => {
                        let a = arg(allocs, 0);

                        // While Cranelift allows any width condition for
                        // `select` and checks it against `0`, Souper requires
                        // an `i1`. So insert a `ne %x, 0` as needed.
                        let a = match a {
                            ast::Operand::Value(id) => match lhs.get_value(id).r#type {
                                Some(ast::Type { width: 1 }) => a,
                                _ => lhs
                                    .assignment(
                                        None,
                                        Some(ast::Type { width: 1 }),
                                        ast::Instruction::Ne {
                                            a,
                                            b: ast::Constant {
                                                value: 0,
                                                r#type: None,
                                            }
                                            .into(),
                                        },
                                        vec![],
                                    )
                                    .into(),
                            },
                            ast::Operand::Constant(ast::Constant { value, .. }) => ast::Constant {
                                value: (value != 0) as _,
                                r#type: Some(ast::Type { width: 1 }),
                            }
                            .into(),
                        };

                        let b = arg(allocs, 1);
                        let c = arg(allocs, 2);
                        ast::Instruction::Select { a, b, c }.into()
                    }
                    (ir::Opcode::Uextend, _) => {
                        let a = arg(allocs, 0);
                        ast::Instruction::Zext { a }.into()
                    }
                    (ir::Opcode::Sextend, _) => {
                        let a = arg(allocs, 0);
                        ast::Instruction::Sext { a }.into()
                    }
                    (ir::Opcode::Trunc, _) => {
                        let a = arg(allocs, 0);
                        ast::Instruction::Trunc { a }.into()
                    }
                    (ir::Opcode::Icmp, ir::InstructionData::IntCompare { cond, .. })
                    | (ir::Opcode::IcmpImm, ir::InstructionData::IntCompare { cond, .. }) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        match cond {
                            ir::condcodes::IntCC::Equal => ast::Instruction::Eq { a, b }.into(),
                            ir::condcodes::IntCC::NotEqual => ast::Instruction::Ne { a, b }.into(),
                            ir::condcodes::IntCC::UnsignedLessThan => {
                                ast::Instruction::Ult { a, b }.into()
                            }
                            ir::condcodes::IntCC::SignedLessThan => {
                                ast::Instruction::Slt { a, b }.into()
                            }
                            ir::condcodes::IntCC::UnsignedLessThanOrEqual => {
                                ast::Instruction::Sle { a, b }.into()
                            }
                            ir::condcodes::IntCC::SignedLessThanOrEqual => {
                                ast::Instruction::Sle { a, b }.into()
                            }
                            _ => ast::AssignmentRhs::Var,
                        }
                    }
                    (ir::Opcode::Popcnt, _) => {
                        let a = arg(allocs, 0);
                        ast::Instruction::Ctpop { a }.into()
                    }
                    (ir::Opcode::Bitrev, _) => {
                        let a = arg(allocs, 0);
                        ast::Instruction::BitReverse { a }.into()
                    }
                    (ir::Opcode::Clz, _) => {
                        let a = arg(allocs, 0);
                        ast::Instruction::Ctlz { a }.into()
                    }
                    (ir::Opcode::Ctz, _) => {
                        let a = arg(allocs, 0);
                        ast::Instruction::Cttz { a }.into()
                    }
                    // TODO: ir::Opcode::IaddCarry
                    (ir::Opcode::SaddSat, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::SaddSat { a, b }.into()
                    }
                    (ir::Opcode::SsubSat, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::SsubSat { a, b }.into()
                    }
                    (ir::Opcode::UsubSat, _) => {
                        let a = arg(allocs, 0);
                        let b = arg(allocs, 1);
                        ast::Instruction::UsubSat { a, b }.into()
                    }
                    // Because Souper doesn't allow constants to be on the right
                    // hand side of an assignment (i.e. `%0:i32 = 1234` is
                    // disallowed) we have to ignore `iconst`
                    // instructions until we process them as operands for some
                    // other instruction. See the `arg` closure above for
                    // details.
                    (ir::Opcode::Iconst, _) => return,
                    _ => ast::AssignmentRhs::Var,
                }
            }
            _ => ast::AssignmentRhs::Var,
        };

        non_var_count += match souper_assignment_rhs {
            ast::AssignmentRhs::Var => 0,
            _ => 1,
        };
        let souper_ty = souper_type_of(&func.dfg, val);
        let souper_val = lhs.assignment(None, souper_ty, souper_assignment_rhs, vec![]);
        let old_value = allocs.ir_to_souper_val.insert(val, souper_val);
        assert!(old_value.is_none());
    });

    // We end up harvesting a lot of candidates like:
    //
    //     %0:i32 = var
    //     infer %0
    //
    // and
    //
    //     %0:i32 = var
    //     %1:i32 = var
    //     %2:i32 = add %0, %1
    //
    // Both of these are useless. Only actually harvest the candidate if there
    // are at least two actual operations.
    if non_var_count >= 2 {
        let lhs = lhs.finish(allocs.ir_to_souper_val[&val], None);
        out.send(format!(
            ";; Harvested from `{}` in `{}`\n{}\n",
            val, func.name, lhs
        ))
        .unwrap();
    }
}

fn souper_type_of(dfg: &ir::DataFlowGraph, val: ir::Value) -> Option<ast::Type> {
    let ty = dfg.value_type(val);
    assert!(ty.is_int());
    assert_eq!(ty.lane_count(), 1);
    let width = match dfg.value_def(val).inst() {
        Some(inst)
            if dfg.insts[inst].opcode() == ir::Opcode::IcmpImm
                || dfg.insts[inst].opcode() == ir::Opcode::Icmp =>
        {
            1
        }
        _ => ty.bits().try_into().unwrap(),
    };
    Some(ast::Type { width })
}

#[derive(Debug)]
enum StackEntry {
    Visit(ir::Value),
    Trace(ir::Value),
}

fn post_order_dfs(
    allocs: &mut Allocs,
    dfg: &ir::DataFlowGraph,
    val: ir::Value,
    should_trace: impl Fn(ir::Value) -> bool,
    mut visit: impl FnMut(&mut Allocs, ir::Value),
) {
    allocs.dfs_stack.push(StackEntry::Trace(val));

    while let Some(entry) = allocs.dfs_stack.pop() {
        match entry {
            StackEntry::Visit(val) => {
                let is_new = allocs.dfs_seen.insert(val);
                if is_new {
                    visit(allocs, val);
                }
            }
            StackEntry::Trace(val) => {
                if allocs.dfs_seen.contains(&val) {
                    continue;
                }

                allocs.dfs_stack.push(StackEntry::Visit(val));
                if should_trace(val) {
                    if let ir::ValueDef::Result(inst, 0) = dfg.value_def(val) {
                        let args = dfg.inst_args(inst);
                        for v in args.iter().rev().copied() {
                            allocs.dfs_stack.push(StackEntry::Trace(v));
                        }
                    }
                }
            }
        }
    }
}
