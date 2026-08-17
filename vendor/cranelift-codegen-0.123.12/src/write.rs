//! Converting Cranelift IR to text.
//!
//! The `write` module provides the `write_function` function which converts an IR `Function` to an
//! equivalent textual form. This textual form can be read back by the `cranelift-reader` crate.

use crate::entity::SecondaryMap;
use crate::ir::entities::AnyEntity;
use crate::ir::immediates::Ieee128;
use crate::ir::pcc::Fact;
use crate::ir::{Block, DataFlowGraph, Function, Inst, Opcode, SigRef, Type, Value, ValueDef};
use crate::packed_option::ReservedValue;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{self, Write};

/// A `FuncWriter` used to decorate functions during printing.
pub trait FuncWriter {
    /// Write the basic block header for the current function.
    fn write_block_header(
        &mut self,
        w: &mut dyn Write,
        func: &Function,
        block: Block,
        indent: usize,
    ) -> fmt::Result;

    /// Write the given `inst` to `w`.
    fn write_instruction(
        &mut self,
        w: &mut dyn Write,
        func: &Function,
        aliases: &SecondaryMap<Value, Vec<Value>>,
        inst: Inst,
        indent: usize,
    ) -> fmt::Result;

    /// Write the preamble to `w`. By default, this uses `write_entity_definition`.
    fn write_preamble(&mut self, w: &mut dyn Write, func: &Function) -> Result<bool, fmt::Error> {
        self.super_preamble(w, func)
    }

    /// Default impl of `write_preamble`
    fn super_preamble(&mut self, w: &mut dyn Write, func: &Function) -> Result<bool, fmt::Error> {
        let mut any = false;

        for (ss, slot) in func.dynamic_stack_slots.iter() {
            any = true;
            self.write_entity_definition(w, func, ss.into(), slot, None)?;
        }

        for (ss, slot) in func.sized_stack_slots.iter() {
            any = true;
            self.write_entity_definition(w, func, ss.into(), slot, None)?;
        }

        for (gv, gv_data) in &func.global_values {
            any = true;
            let maybe_fact = func.global_value_facts[gv].as_ref();
            self.write_entity_definition(w, func, gv.into(), gv_data, maybe_fact)?;
        }

        for (mt, mt_data) in &func.memory_types {
            any = true;
            self.write_entity_definition(w, func, mt.into(), mt_data, None)?;
        }

        // Write out all signatures before functions since function declarations can refer to
        // signatures.
        for (sig, sig_data) in &func.dfg.signatures {
            any = true;
            self.write_entity_definition(w, func, sig.into(), &sig_data, None)?;
        }

        for (fnref, ext_func) in &func.dfg.ext_funcs {
            if ext_func.signature != SigRef::reserved_value() {
                any = true;
                self.write_entity_definition(
                    w,
                    func,
                    fnref.into(),
                    &ext_func.display(Some(&func.params)),
                    None,
                )?;
            }
        }

        for (&cref, cval) in func.dfg.constants.iter() {
            any = true;
            self.write_entity_definition(w, func, cref.into(), cval, None)?;
        }

        if let Some(limit) = func.stack_limit {
            any = true;
            self.write_entity_definition(w, func, AnyEntity::StackLimit, &limit, None)?;
        }

        Ok(any)
    }

    /// Write an entity definition defined in the preamble to `w`.
    fn write_entity_definition(
        &mut self,
        w: &mut dyn Write,
        func: &Function,
        entity: AnyEntity,
        value: &dyn fmt::Display,
        maybe_fact: Option<&Fact>,
    ) -> fmt::Result {
        self.super_entity_definition(w, func, entity, value, maybe_fact)
    }

    /// Default impl of `write_entity_definition`
    fn super_entity_definition(
        &mut self,
        w: &mut dyn Write,
        _func: &Function,
        entity: AnyEntity,
        value: &dyn fmt::Display,
        maybe_fact: Option<&Fact>,
    ) -> fmt::Result {
        if let Some(fact) = maybe_fact {
            writeln!(w, "    {entity} ! {fact} = {value}")
        } else {
            writeln!(w, "    {entity} = {value}")
        }
    }
}

/// A `PlainWriter` that doesn't decorate the function.
pub struct PlainWriter;

impl FuncWriter for PlainWriter {
    fn write_instruction(
        &mut self,
        w: &mut dyn Write,
        func: &Function,
        aliases: &SecondaryMap<Value, Vec<Value>>,
        inst: Inst,
        indent: usize,
    ) -> fmt::Result {
        write_instruction(w, func, aliases, inst, indent)
    }

    fn write_block_header(
        &mut self,
        w: &mut dyn Write,
        func: &Function,
        block: Block,
        indent: usize,
    ) -> fmt::Result {
        write_block_header(w, func, block, indent)
    }
}

/// Write `func` to `w` as equivalent text.
/// Use `isa` to emit ISA-dependent annotations.
pub fn write_function(w: &mut dyn Write, func: &Function) -> fmt::Result {
    decorate_function(&mut PlainWriter, w, func)
}

/// Create a reverse-alias map from a value to all aliases having that value as a direct target
fn alias_map(func: &Function) -> SecondaryMap<Value, Vec<Value>> {
    let mut aliases = SecondaryMap::<_, Vec<_>>::new();
    for v in func.dfg.values() {
        // VADFS returns the immediate target of an alias
        if let Some(k) = func.dfg.value_alias_dest_for_serialization(v) {
            aliases[k].push(v);
        }
    }
    aliases
}

/// Writes `func` to `w` as text.
/// write_function_plain is passed as 'closure' to print instructions as text.
/// pretty_function_error is passed as 'closure' to add error decoration.
pub fn decorate_function<FW: FuncWriter>(
    func_w: &mut FW,
    w: &mut dyn Write,
    func: &Function,
) -> fmt::Result {
    write!(w, "function ")?;
    write_function_spec(w, func)?;
    writeln!(w, " {{")?;
    let aliases = alias_map(func);
    let mut any = func_w.write_preamble(w, func)?;
    for block in &func.layout {
        if any {
            writeln!(w)?;
        }
        decorate_block(func_w, w, func, &aliases, block)?;
        any = true;
    }
    writeln!(w, "}}")
}

//----------------------------------------------------------------------
//
// Function spec.

/// Writes the spec (name and signature) of 'func' to 'w' as text.
pub fn write_function_spec(w: &mut dyn Write, func: &Function) -> fmt::Result {
    write!(w, "{}{}", func.name, func.signature)
}

//----------------------------------------------------------------------
//
// Basic blocks

fn write_arg(w: &mut dyn Write, func: &Function, arg: Value) -> fmt::Result {
    let ty = func.dfg.value_type(arg);
    if let Some(f) = &func.dfg.facts[arg] {
        write!(w, "{arg} ! {f}: {ty}")
    } else {
        write!(w, "{arg}: {ty}")
    }
}

/// Write out the basic block header, outdented:
///
///    block1:
///    block1(v1: i32):
///    block10(v4: f64, v5: i8):
///
pub fn write_block_header(
    w: &mut dyn Write,
    func: &Function,
    block: Block,
    indent: usize,
) -> fmt::Result {
    let cold = if func.layout.is_cold(block) {
        " cold"
    } else {
        ""
    };

    // The `indent` is the instruction indentation. block headers are 4 spaces out from that.
    write!(w, "{1:0$}{2}", indent - 4, "", block)?;

    let mut args = func.dfg.block_params(block).iter().cloned();
    match args.next() {
        None => return writeln!(w, "{cold}:"),
        Some(arg) => {
            write!(w, "(")?;
            write_arg(w, func, arg)?;
        }
    }
    // Remaining arguments.
    for arg in args {
        write!(w, ", ")?;
        write_arg(w, func, arg)?;
    }
    writeln!(w, "){cold}:")
}

fn decorate_block<FW: FuncWriter>(
    func_w: &mut FW,
    w: &mut dyn Write,
    func: &Function,
    aliases: &SecondaryMap<Value, Vec<Value>>,
    block: Block,
) -> fmt::Result {
    // Indent all instructions if any srclocs are present.
    let indent = if func.rel_srclocs().is_empty() { 4 } else { 36 };

    func_w.write_block_header(w, func, block, indent)?;
    for a in func.dfg.block_params(block).iter().cloned() {
        write_value_aliases(w, aliases, a, indent)?;
    }

    for inst in func.layout.block_insts(block) {
        func_w.write_instruction(w, func, aliases, inst, indent)?;
    }

    Ok(())
}

//----------------------------------------------------------------------
//
// Instructions

// Should `inst` be printed with a type suffix?
//
// Polymorphic instructions may need a suffix indicating the value of the controlling type variable
// if it can't be trivially inferred.
//
fn type_suffix(func: &Function, inst: Inst) -> Option<Type> {
    let inst_data = &func.dfg.insts[inst];
    let constraints = inst_data.opcode().constraints();

    if !constraints.is_polymorphic() {
        return None;
    }

    // If the controlling type variable can be inferred from the type of the designated value input
    // operand, we don't need the type suffix.
    if constraints.use_typevar_operand() {
        let ctrl_var = inst_data.typevar_operand(&func.dfg.value_lists).unwrap();
        let def_block = match func.dfg.value_def(ctrl_var) {
            ValueDef::Result(instr, _) => func.layout.inst_block(instr),
            ValueDef::Param(block, _) => Some(block),
            ValueDef::Union(..) => None,
        };
        if def_block.is_some() && def_block == func.layout.inst_block(inst) {
            return None;
        }
    }

    let rtype = func.dfg.ctrl_typevar(inst);
    assert!(
        !rtype.is_invalid(),
        "Polymorphic instruction must produce a result"
    );
    Some(rtype)
}

/// Write out any aliases to the given target, including indirect aliases
fn write_value_aliases(
    w: &mut dyn Write,
    aliases: &SecondaryMap<Value, Vec<Value>>,
    target: Value,
    indent: usize,
) -> fmt::Result {
    let mut todo_stack = vec![target];
    while let Some(target) = todo_stack.pop() {
        for &a in &aliases[target] {
            writeln!(w, "{1:0$}{2} -> {3}", indent, "", a, target)?;
            todo_stack.push(a);
        }
    }

    Ok(())
}

fn write_instruction(
    w: &mut dyn Write,
    func: &Function,
    aliases: &SecondaryMap<Value, Vec<Value>>,
    inst: Inst,
    indent: usize,
) -> fmt::Result {
    // Prefix containing source location, encoding, and value locations.
    let mut s = String::with_capacity(16);

    // Source location goes first.
    let srcloc = func.srcloc(inst);
    if !srcloc.is_default() {
        write!(s, "{srcloc} ")?;
    }

    // Write out prefix and indent the instruction.
    write!(w, "{s:indent$}")?;

    // Write out the result values, if any.
    let mut has_results = false;
    for r in func.dfg.inst_results(inst) {
        if !has_results {
            has_results = true;
            write!(w, "{r}")?;
        } else {
            write!(w, ", {r}")?;
        }
        if let Some(f) = &func.dfg.facts[*r] {
            write!(w, " ! {f}")?;
        }
    }
    if has_results {
        write!(w, " = ")?;
    }

    // Then the opcode, possibly with a '.type' suffix.
    let opcode = func.dfg.insts[inst].opcode();

    match type_suffix(func, inst) {
        Some(suf) => write!(w, "{opcode}.{suf}")?,
        None => write!(w, "{opcode}")?,
    }

    write_operands(w, &func.dfg, inst)?;
    writeln!(w)?;

    // Value aliases come out on lines after the instruction defining the referent.
    for r in func.dfg.inst_results(inst) {
        write_value_aliases(w, aliases, *r, indent)?;
    }
    Ok(())
}

/// Write the operands of `inst` to `w` with a prepended space.
pub fn write_operands(w: &mut dyn Write, dfg: &DataFlowGraph, inst: Inst) -> fmt::Result {
    let pool = &dfg.value_lists;
    let jump_tables = &dfg.jump_tables;
    let exception_tables = &dfg.exception_tables;
    use crate::ir::instructions::InstructionData::*;
    let ctrl_ty = dfg.ctrl_typevar(inst);
    match dfg.insts[inst] {
        AtomicRmw { op, args, .. } => write!(w, " {} {}, {}", op, args[0], args[1]),
        AtomicCas { args, .. } => write!(w, " {}, {}, {}", args[0], args[1], args[2]),
        LoadNoOffset { flags, arg, .. } => write!(w, "{flags} {arg}"),
        StoreNoOffset { flags, args, .. } => write!(w, "{} {}, {}", flags, args[0], args[1]),
        Unary { arg, .. } => write!(w, " {arg}"),
        UnaryImm { imm, .. } => write!(w, " {}", {
            let mut imm = imm;
            if ctrl_ty.bits() != 0 {
                imm = imm.sign_extend_from_width(ctrl_ty.bits());
            }
            imm
        }),
        UnaryIeee16 { imm, .. } => write!(w, " {imm}"),
        UnaryIeee32 { imm, .. } => write!(w, " {imm}"),
        UnaryIeee64 { imm, .. } => write!(w, " {imm}"),
        UnaryGlobalValue { global_value, .. } => write!(w, " {global_value}"),
        UnaryConst {
            constant_handle, ..
        } => write!(w, " {constant_handle}"),
        Binary { args, .. } => write!(w, " {}, {}", args[0], args[1]),
        BinaryImm8 { arg, imm, .. } => write!(w, " {arg}, {imm}"),
        BinaryImm64 { arg, imm, .. } => write!(w, " {}, {}", arg, {
            let mut imm = imm;
            if ctrl_ty.bits() != 0 {
                imm = imm.sign_extend_from_width(ctrl_ty.bits());
            }
            imm
        }),
        Ternary { args, .. } => write!(w, " {}, {}, {}", args[0], args[1], args[2]),
        MultiAry { ref args, .. } => {
            if args.is_empty() {
                write!(w, "")
            } else {
                write!(w, " {}", DisplayValues(args.as_slice(pool)))
            }
        }
        NullAry { .. } => write!(w, " "),
        TernaryImm8 { imm, args, .. } => write!(w, " {}, {}, {}", args[0], args[1], imm),
        Shuffle { imm, args, .. } => {
            let data = dfg.immediates.get(imm).expect(
                "Expected the shuffle mask to already be inserted into the immediates table",
            );
            write!(w, " {}, {}, {}", args[0], args[1], data)
        }
        IntCompare { cond, args, .. } => write!(w, " {} {}, {}", cond, args[0], args[1]),
        IntCompareImm { cond, arg, imm, .. } => write!(w, " {} {}, {}", cond, arg, {
            let mut imm = imm;
            if ctrl_ty.bits() != 0 {
                imm = imm.sign_extend_from_width(ctrl_ty.bits());
            }
            imm
        }),
        IntAddTrap { args, code, .. } => write!(w, " {}, {}, {}", args[0], args[1], code),
        FloatCompare { cond, args, .. } => write!(w, " {} {}, {}", cond, args[0], args[1]),
        Jump { destination, .. } => {
            write!(w, " {}", destination.display(pool))
        }
        Brif {
            arg,
            blocks: [block_then, block_else],
            ..
        } => {
            write!(w, " {}, {}", arg, block_then.display(pool))?;
            write!(w, ", {}", block_else.display(pool))
        }
        BranchTable { arg, table, .. } => {
            write!(w, " {}, {}", arg, jump_tables[table].display(pool))
        }
        Call {
            func_ref, ref args, ..
        } => {
            write!(w, " {}({})", func_ref, DisplayValues(args.as_slice(pool)))?;
            write_user_stack_map_entries(w, dfg, inst)
        }
        CallIndirect {
            sig_ref, ref args, ..
        } => {
            let args = args.as_slice(pool);
            write!(
                w,
                " {}, {}({})",
                sig_ref,
                args[0],
                DisplayValues(&args[1..])
            )?;
            write_user_stack_map_entries(w, dfg, inst)
        }
        TryCall {
            func_ref,
            ref args,
            exception,
            ..
        } => {
            write!(
                w,
                " {}({}), {}",
                func_ref,
                DisplayValues(args.as_slice(pool)),
                exception_tables[exception].display(pool),
            )
        }
        TryCallIndirect {
            ref args,
            exception,
            ..
        } => {
            let args = args.as_slice(pool);
            write!(
                w,
                " {}({}), {}",
                args[0],
                DisplayValues(&args[1..]),
                exception_tables[exception].display(pool),
            )
        }
        FuncAddr { func_ref, .. } => write!(w, " {func_ref}"),
        StackLoad {
            stack_slot, offset, ..
        } => write!(w, " {stack_slot}{offset}"),
        StackStore {
            arg,
            stack_slot,
            offset,
            ..
        } => write!(w, " {arg}, {stack_slot}{offset}"),
        DynamicStackLoad {
            dynamic_stack_slot, ..
        } => write!(w, " {dynamic_stack_slot}"),
        DynamicStackStore {
            arg,
            dynamic_stack_slot,
            ..
        } => write!(w, " {arg}, {dynamic_stack_slot}"),
        Load {
            flags, arg, offset, ..
        } => write!(w, "{flags} {arg}{offset}"),
        Store {
            flags,
            args,
            offset,
            ..
        } => write!(w, "{} {}, {}{}", flags, args[0], args[1], offset),
        Trap { code, .. } => write!(w, " {code}"),
        CondTrap { arg, code, .. } => write!(w, " {arg}, {code}"),
    }?;

    let mut sep = "  ; ";
    for arg in dfg.inst_values(inst) {
        if let ValueDef::Result(src, _) = dfg.value_def(arg) {
            let imm = match dfg.insts[src] {
                UnaryImm { imm, .. } => {
                    let mut imm = imm;
                    if dfg.ctrl_typevar(src).bits() != 0 {
                        imm = imm.sign_extend_from_width(dfg.ctrl_typevar(src).bits());
                    }
                    imm.to_string()
                }
                UnaryIeee16 { imm, .. } => imm.to_string(),
                UnaryIeee32 { imm, .. } => imm.to_string(),
                UnaryIeee64 { imm, .. } => imm.to_string(),
                UnaryConst {
                    constant_handle,
                    opcode: Opcode::F128const,
                } => Ieee128::try_from(dfg.constants.get(constant_handle))
                    .expect("16-byte f128 constant")
                    .to_string(),
                UnaryConst {
                    constant_handle, ..
                } => constant_handle.to_string(),
                _ => continue,
            };
            write!(w, "{sep}{arg} = {imm}")?;
            sep = ", ";
        }
    }
    Ok(())
}

fn write_user_stack_map_entries(w: &mut dyn Write, dfg: &DataFlowGraph, inst: Inst) -> fmt::Result {
    let entries = match dfg.user_stack_map_entries(inst) {
        None => return Ok(()),
        Some(es) => es,
    };
    write!(w, ", stack_map=[")?;
    let mut need_comma = false;
    for entry in entries {
        if need_comma {
            write!(w, ", ")?;
        }
        write!(w, "{} @ {}+{}", entry.ty, entry.slot, entry.offset)?;
        need_comma = true;
    }
    write!(w, "]")?;
    Ok(())
}

/// Displayable slice of values.
struct DisplayValues<'a>(&'a [Value]);

impl<'a> fmt::Display for DisplayValues<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, val) in self.0.iter().enumerate() {
            if i == 0 {
                write!(f, "{val}")?;
            } else {
                write!(f, ", {val}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cursor::{Cursor, CursorPosition, FuncCursor};
    use crate::ir::types;
    use crate::ir::{Function, InstBuilder, StackSlotData, StackSlotKind, UserFuncName};
    use alloc::string::ToString;

    #[test]
    fn basic() {
        let mut f = Function::new();
        assert_eq!(f.to_string(), "function u0:0() fast {\n}\n");

        f.name = UserFuncName::testcase("foo");
        assert_eq!(f.to_string(), "function %foo() fast {\n}\n");

        f.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 4, 0));
        assert_eq!(
            f.to_string(),
            "function %foo() fast {\n    ss0 = explicit_slot 4\n}\n"
        );

        let block = f.dfg.make_block();
        f.layout.append_block(block);
        assert_eq!(
            f.to_string(),
            "function %foo() fast {\n    ss0 = explicit_slot 4\n\nblock0:\n}\n"
        );

        f.dfg.append_block_param(block, types::I8);
        assert_eq!(
            f.to_string(),
            "function %foo() fast {\n    ss0 = explicit_slot 4\n\nblock0(v0: i8):\n}\n"
        );

        f.dfg.append_block_param(block, types::F32.by(4).unwrap());
        assert_eq!(
            f.to_string(),
            "function %foo() fast {\n    ss0 = explicit_slot 4\n\nblock0(v0: i8, v1: f32x4):\n}\n"
        );

        {
            let mut cursor = FuncCursor::new(&mut f);
            cursor.set_position(CursorPosition::After(block));
            cursor.ins().return_(&[])
        };
        assert_eq!(
            f.to_string(),
            "function %foo() fast {\n    ss0 = explicit_slot 4\n\nblock0(v0: i8, v1: f32x4):\n    return\n}\n"
        );

        let mut f = Function::new();
        f.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 4, 2));
        assert_eq!(
            f.to_string(),
            "function u0:0() fast {\n    ss0 = explicit_slot 4, align = 4\n}\n"
        );
    }

    #[test]
    fn aliases() {
        use crate::ir::InstBuilder;

        let mut func = Function::new();
        {
            let block0 = func.dfg.make_block();
            let mut pos = FuncCursor::new(&mut func);
            pos.insert_block(block0);

            // make some detached values for change_to_alias
            let v0 = pos.func.dfg.append_block_param(block0, types::I32);
            let v1 = pos.func.dfg.append_block_param(block0, types::I32);
            let v2 = pos.func.dfg.append_block_param(block0, types::I32);
            pos.func.dfg.detach_block_params(block0);

            // alias to a param--will be printed at beginning of block defining param
            let v3 = pos.func.dfg.append_block_param(block0, types::I32);
            pos.func.dfg.change_to_alias(v0, v3);

            // alias to an alias--should print attached to alias, not ultimate target
            pos.func.dfg.make_value_alias_for_serialization(v0, v2); // v0 <- v2

            // alias to a result--will be printed after instruction producing result
            let _dummy0 = pos.ins().iconst(types::I32, 42);
            let v4 = pos.ins().iadd(v0, v0);
            pos.func.dfg.change_to_alias(v1, v4);
            let _dummy1 = pos.ins().iconst(types::I32, 23);
            let _v7 = pos.ins().iadd(v1, v1);
        }
        assert_eq!(
            func.to_string(),
            "function u0:0() fast {\nblock0(v3: i32):\n    v0 -> v3\n    v2 -> v0\n    v4 = iconst.i32 42\n    v5 = iadd v0, v0\n    v1 -> v5\n    v6 = iconst.i32 23\n    v7 = iadd v1, v1\n}\n"
        );
    }

    #[test]
    fn cold_blocks() {
        let mut func = Function::new();
        {
            let mut pos = FuncCursor::new(&mut func);

            let block0 = pos.func.dfg.make_block();
            pos.insert_block(block0);
            pos.func.layout.set_cold(block0);

            let block1 = pos.func.dfg.make_block();
            pos.insert_block(block1);
            pos.func.dfg.append_block_param(block1, types::I32);
            pos.func.layout.set_cold(block1);
        }

        assert_eq!(
            func.to_string(),
            "function u0:0() fast {\nblock0 cold:\n\nblock1(v0: i32) cold:\n}\n"
        );
    }
}
