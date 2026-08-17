//! Generate CLIF instruction data (including opcodes, formats, builders, etc.).

use crate::cdsl::camel_case;
use crate::cdsl::formats::InstructionFormat;
use crate::cdsl::instructions::{AllInstructions, Instruction};
use crate::cdsl::operands::{Operand, OperandKindFields};
use crate::cdsl::typevar::{TypeSet, TypeVar};
use crate::unique_table::{UniqueSeqTable, UniqueTable};
use cranelift_codegen_shared::constant_hash;
use cranelift_srcgen::{Formatter, Language, Match, error, fmtln};
use std::fmt;
use std::rc::Rc;

// TypeSet indexes are encoded in 8 bits, with `0xff` reserved.
const TYPESET_LIMIT: usize = 0xff;

/// Generate an instruction format enumeration.
fn gen_formats(formats: &[Rc<InstructionFormat>], fmt: &mut Formatter) {
    fmt.doc_comment(
        r#"
        An instruction format

        Every opcode has a corresponding instruction format
        which is represented by both the `InstructionFormat`
        and the `InstructionData` enums.
    "#,
    );
    fmt.line("#[derive(Copy, Clone, PartialEq, Eq, Debug)]");
    fmt.add_block("pub enum InstructionFormat", |fmt| {
        for format in formats {
            fmt.doc_comment(format.to_string());
            fmtln!(fmt, "{},", format.name);
        }
    });
    fmt.empty_line();

    // Emit a From<InstructionData> which also serves to verify that
    // InstructionFormat and InstructionData are in sync.
    fmt.add_block(
        "impl<'a> From<&'a InstructionData> for InstructionFormat",
        |fmt| {
            fmt.add_block("fn from(inst: &'a InstructionData) -> Self", |fmt| {
                let mut m = Match::new("*inst");
                for format in formats {
                    m.arm(
                        format!("InstructionData::{}", format.name),
                        vec![".."],
                        format!("Self::{}", format.name),
                    );
                }
                fmt.add_match(m);
            });
        },
    );
    fmt.empty_line();
}

/// Generate the InstructionData enum.
///
/// Every variant must contain an `opcode` field. The size of `InstructionData` should be kept at
/// 16 bytes on 64-bit architectures. If more space is needed to represent an instruction, use a
/// `ValueList` to store the additional information out of line.
fn gen_instruction_data(formats: &[Rc<InstructionFormat>], fmt: &mut Formatter) {
    fmt.line("#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]");
    fmt.line(r#"#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]"#);
    fmt.line("#[allow(missing_docs, reason = \"generated code\")]");
    fmt.add_block("pub enum InstructionData", |fmt| {
        for format in formats {
            fmt.add_block(&format!("{}", format.name), |fmt| {
                fmt.line("opcode: Opcode,");
                if format.has_value_list {
                    fmt.line("args: ValueList,");
                } else if format.num_value_operands == 1 {
                    fmt.line("arg: Value,");
                } else if format.num_value_operands > 0 {
                    fmtln!(fmt, "args: [Value; {}],", format.num_value_operands);
                }

                match format.num_block_operands {
                    0 => (),
                    1 => fmt.line("destination: ir::BlockCall,"),
                    2 => fmtln!(
                        fmt,
                        "blocks: [ir::BlockCall; {}],",
                        format.num_block_operands
                    ),
                    n => panic!("Too many block operands in instruction: {n}"),
                }

                for field in &format.imm_fields {
                    fmtln!(fmt, "{}: {},", field.member, field.kind.rust_type);
                }
            });
            fmtln!(fmt, ",");
        }
    });
}

fn gen_arguments_method(formats: &[Rc<InstructionFormat>], fmt: &mut Formatter, is_mut: bool) {
    let (method, mut_, rslice, as_slice) = if is_mut {
        (
            "arguments_mut",
            "mut ",
            "core::slice::from_mut",
            "as_mut_slice",
        )
    } else {
        ("arguments", "", "core::slice::from_ref", "as_slice")
    };

    fmt.add_block(&format!(
        "pub fn {method}<'a>(&'a {mut_}self, pool: &'a {mut_}ir::ValueListPool) -> &'a {mut_}[Value]"),

    |fmt| {
        let mut m = Match::new("*self");
        for format in formats {
            let name = format!("Self::{}", format.name);

            // Formats with a value list put all of their arguments in the list. We don't split
            // them up, just return it all as variable arguments. (I expect the distinction to go
            // away).
            if format.has_value_list {
                m.arm(
                    name,
                    vec![format!("ref {}args", mut_), "..".to_string()],
                    format!("args.{as_slice}(pool)"),
                );
                continue;
            }

            // Fixed args.
            let mut fields = Vec::new();
            let arg = if format.num_value_operands == 0 {
                format!("&{mut_}[]")
            } else if format.num_value_operands == 1 {
                fields.push(format!("ref {mut_}arg"));
                format!("{rslice}(arg)")
            } else {
                let arg = format!("args_arity{}", format.num_value_operands);
                fields.push(format!("args: ref {mut_}{arg}"));
                arg
            };
            fields.push("..".into());

            m.arm(name, fields, arg);
        }
        fmt.add_match(m);
    });
}

/// Generate the boring parts of the InstructionData implementation.
///
/// These methods in `impl InstructionData` can be generated automatically from the instruction
/// formats:
///
/// - `pub fn opcode(&self) -> Opcode`
/// - `pub fn arguments(&self, &pool) -> &[Value]`
/// - `pub fn arguments_mut(&mut self, &pool) -> &mut [Value]`
/// - `pub fn eq(&self, &other: Self, &pool) -> bool`
/// - `pub fn hash<H: Hasher>(&self, state: &mut H, &pool)`
fn gen_instruction_data_impl(formats: &[Rc<InstructionFormat>], fmt: &mut Formatter) {
    fmt.add_block("impl InstructionData", |fmt| {
        fmt.doc_comment("Get the opcode of this instruction.");
        fmt.add_block("pub fn opcode(&self) -> Opcode",|fmt| {
            let mut m = Match::new("*self");
            for format in formats {
                m.arm(format!("Self::{}", format.name), vec!["opcode", ".."],
                      "opcode".to_string());
            }
            fmt.add_match(m);
        });
                fmt.empty_line();

        fmt.doc_comment("Get the controlling type variable operand.");
        fmt.add_block("pub fn typevar_operand(&self, pool: &ir::ValueListPool) -> Option<Value>",|fmt| {
            let mut m = Match::new("*self");
            for format in formats {
                let name = format!("Self::{}", format.name);
                if format.typevar_operand.is_none() {
                    m.arm(name, vec![".."], "None".to_string());
                } else if format.has_value_list {
                    // We keep all arguments in a value list.
                    m.arm(name, vec!["ref args", ".."], format!("args.get({}, pool)", format.typevar_operand.unwrap()));
                } else if format.num_value_operands == 1 {
                    m.arm(name, vec!["arg", ".."], "Some(arg)".to_string());
                } else {
                    // We have multiple value operands and an array `args`.
                    // Which `args` index to use?
                    let args = format!("args_arity{}", format.num_value_operands);
                    m.arm(name, vec![format!("args: ref {}", args), "..".to_string()],
                        format!("Some({}[{}])", args, format.typevar_operand.unwrap()));
                }
            }
            fmt.add_match(m);
        });
                fmt.empty_line();

        fmt.doc_comment("Get the value arguments to this instruction.");
        gen_arguments_method(formats, fmt, false);
        fmt.empty_line();

        fmt.doc_comment(r#"Get mutable references to the value arguments to this
                        instruction."#);
        gen_arguments_method(formats, fmt, true);
        fmt.empty_line();

        fmt.doc_comment(r#"
            Compare two `InstructionData` for equality.

            This operation requires a reference to a `ValueListPool` to
            determine if the contents of any `ValueLists` are equal.

            This operation takes a closure that is allowed to map each
            argument value to some other value before the instructions
            are compared. This allows various forms of canonicalization.
        "#);
        fmt.add_block("pub fn eq(&self, other: &Self, pool: &ir::ValueListPool) -> bool", |fmt| {
            fmt.add_block("if ::core::mem::discriminant(self) != ::core::mem::discriminant(other)", |fmt| {
                fmt.line("return false;");
            });

            fmt.add_block("match (self, other)",|fmt| {
                for format in formats {
                    let name = format!("&Self::{}", format.name);
                    let mut members = vec!["opcode"];

                    let args_eq = if format.has_value_list {
                        members.push("args");
                        Some("args1.as_slice(pool).iter().zip(args2.as_slice(pool).iter()).all(|(a, b)| a == b)")
                    } else if format.num_value_operands == 1 {
                        members.push("arg");
                        Some("arg1 == arg2")
                    } else if format.num_value_operands > 0 {
                        members.push("args");
                        Some("args1.iter().zip(args2.iter()).all(|(a, b)| a == b)")
                    } else {
                        None
                    };

                    let blocks_eq = match format.num_block_operands {
                        0 => None,
                        1 => {
                            members.push("destination");
                            Some("destination1 == destination2")
                        },
                        _ => {
                            members.push("blocks");
                            Some("blocks1.iter().zip(blocks2.iter()).all(|(a, b)| a.block(pool) == b.block(pool))")
                        }
                    };

                    for field in &format.imm_fields {
                        members.push(field.member);
                    }

                    let pat1 = members.iter().map(|x| format!("{x}: ref {x}1")).collect::<Vec<_>>().join(", ");
                    let pat2 = members.iter().map(|x| format!("{x}: ref {x}2")).collect::<Vec<_>>().join(", ");
                    fmt.add_block(&format!("({name} {{ {pat1} }}, {name} {{ {pat2} }}) => "), |fmt| {
                        fmt.line("opcode1 == opcode2");
                        for field in &format.imm_fields {
                            fmtln!(fmt, "&& {}1 == {}2", field.member, field.member);
                        }
                        if let Some(args_eq) = args_eq {
                            fmtln!(fmt, "&& {}", args_eq);
                        }
                        if let Some(blocks_eq) = blocks_eq {
                            fmtln!(fmt, "&& {}", blocks_eq);
                        }
                    });
                }
                fmt.line("_ => unreachable!()");
            });
                    });
                fmt.empty_line();

        fmt.doc_comment(r#"
            Hash an `InstructionData`.

            This operation requires a reference to a `ValueListPool` to
            hash the contents of any `ValueLists`.

            This operation takes a closure that is allowed to map each
            argument value to some other value before it is hashed. This
            allows various forms of canonicalization.
        "#);
        fmt.add_block("pub fn hash<H: ::core::hash::Hasher>(&self, state: &mut H, pool: &ir::ValueListPool)",|fmt| {
            fmt.add_block("match *self",|fmt| {
                for format in formats {
                    let name = format!("Self::{}", format.name);
                    let mut members = vec!["opcode"];

                    let (args, len) = if format.has_value_list {
                        members.push("ref args");
                        (Some("args.as_slice(pool)"), "args.len(pool)")
                    } else if format.num_value_operands == 1 {
                        members.push("ref arg");
                        (Some("std::slice::from_ref(arg)"), "1")
                    } else if format.num_value_operands > 0 {
                        members.push("ref args");
                        (Some("args"), "args.len()")
                    } else {
                        (None, "0")
                    };

                    let blocks = match format.num_block_operands {
                        0 => None,
                        1 => {
                            members.push("ref destination");
                            Some(("std::slice::from_ref(destination)", "1"))
                        }
                        _ => {
                            members.push("ref blocks");
                            Some(("blocks", "blocks.len()"))
                        }
                    };

                    for field in &format.imm_fields {
                        members.push(field.member);
                    }
                    let members = members.join(", ");

                    fmt.add_block(&format!("{name}{{{members}}} => "), |fmt| {
                        fmt.line("::core::hash::Hash::hash( &::core::mem::discriminant(self), state);");
                        fmt.line("::core::hash::Hash::hash(&opcode, state);");
                        for field in &format.imm_fields {
                            fmtln!(fmt, "::core::hash::Hash::hash(&{}, state);", field.member);
                        }
                        fmtln!(fmt, "::core::hash::Hash::hash(&{}, state);", len);
                        if let Some(args) = args {
                            fmt.add_block(&format!("for &arg in {args}"), |fmt| {
                                fmtln!(fmt, "::core::hash::Hash::hash(&arg, state);");
                            });
                        }

                        if let Some((blocks, len)) = blocks {
                            fmtln!(fmt, "::core::hash::Hash::hash(&{len}, state);");
                            fmt.add_block(&format!("for &block in {blocks}"), |fmt| {
                                fmtln!(fmt, "::core::hash::Hash::hash(&block.block(pool), state);");
                                fmt.add_block("for arg in block.args(pool)", |fmt| {
                                    fmtln!(fmt, "::core::hash::Hash::hash(&arg, state);");
                                });
                            });
                        }
                    });
                }
            });
                    });

                fmt.empty_line();

        fmt.doc_comment(r#"
            Deep-clone an `InstructionData`, including any referenced lists.

            This operation requires a reference to a `ValueListPool` to
            clone the `ValueLists`.
        "#);
        fmt.add_block("pub fn deep_clone(&self, pool: &mut ir::ValueListPool) -> Self",|fmt| {
            fmt.add_block("match *self",|fmt| {
                for format in formats {
                    let name = format!("Self::{}", format.name);
                    let mut members = vec!["opcode"];

                    if format.has_value_list {
                        members.push("ref args");
                    } else if format.num_value_operands == 1 {
                        members.push("arg");
                    } else if format.num_value_operands > 0 {
                        members.push("args");
                    }

                    match format.num_block_operands {
                        0 => {}
                        1 => {
                            members.push("destination");
                        }
                        _ => {
                            members.push("blocks");
                        }
                    };

                    for field in &format.imm_fields {
                        members.push(field.member);
                    }
                    let members = members.join(", ");

                    fmt.add_block(&format!("{name}{{{members}}} => "),|fmt| {
                        fmt.add_block(&format!("Self::{}", format.name), |fmt| {
                            fmtln!(fmt, "opcode,");

                            if format.has_value_list {
                                fmtln!(fmt, "args: args.deep_clone(pool),");
                            } else if format.num_value_operands == 1 {
                                fmtln!(fmt, "arg,");
                            } else if format.num_value_operands > 0 {
                                fmtln!(fmt, "args,");
                            }

                            match format.num_block_operands {
                                0 => {}
                                1 => {
                                    fmtln!(fmt, "destination: destination.deep_clone(pool),");
                                }
                                2 => {
                                    fmtln!(fmt, "blocks: [blocks[0].deep_clone(pool), blocks[1].deep_clone(pool)],");
                                }
                                _ => panic!("Too many block targets in instruction"),
                            }

                            for field in &format.imm_fields {
                                fmtln!(fmt, "{},", field.member);
                            }
                        });
                    });
                }
            });
        });
        fmt.doc_comment(r#"
            Map some functions, described by the given `InstructionMapper`, over each of the
            entities within this instruction, producing a new `InstructionData`.
        "#);
        fmt.add_block("pub fn map(&self, mut mapper: impl crate::ir::instructions::InstructionMapper) -> Self", |fmt| {
            fmt.add_block("match *self",|fmt| {
                for format in formats {
                    let name = format!("Self::{}", format.name);
                    let mut members = vec!["opcode"];

                    if format.has_value_list {
                        members.push("args");
                    } else if format.num_value_operands == 1 {
                        members.push("arg");
                    } else if format.num_value_operands > 0 {
                        members.push("args");
                    }

                    match format.num_block_operands {
                        0 => {}
                        1 => {
                            members.push("destination");
                        }
                        _ => {
                            members.push("blocks");
                        }
                    };

                    for field in &format.imm_fields {
                        members.push(field.member);
                    }
                    let members = members.join(", ");

                    fmt.add_block(&format!("{name}{{{members}}} => "), |fmt| {
                        fmt.add_block(&format!("Self::{}", format.name), |fmt| {
                            fmtln!(fmt, "opcode,");

                            if format.has_value_list {
                                fmtln!(fmt, "args: mapper.map_value_list(args),");
                            } else if format.num_value_operands == 1 {
                                fmtln!(fmt, "arg: mapper.map_value(arg),");
                            } else if format.num_value_operands > 0 {
                                let maps = (0..format.num_value_operands)
                                    .map(|i| format!("mapper.map_value(args[{i}])"))
                                    .collect::<Box<[_]>>()
                                    .join(", ");
                                fmtln!(fmt, "args: [{maps}],");
                            }

                            match format.num_block_operands {
                                0 => {}
                                1 => {
                                    fmtln!(fmt, "destination: mapper.map_block_call(destination),");
                                }
                                2 => {
                                    fmtln!(fmt, "blocks: [mapper.map_block_call(blocks[0]), mapper.map_block_call(blocks[1])],");
                                }
                                _ => panic!("Too many block targets in instruction"),
                            }

                            for field in &format.imm_fields {
                                let member = field.member;
                                match &field.kind.fields {
                                    OperandKindFields::EntityRef => {
                                        let mut kind = heck::ToSnakeCase::to_snake_case(
                                            field
                                                .kind
                                                .rust_type
                                                .split("::")
                                                .last()
                                                .unwrap_or(field.kind.rust_type),
                                        );
                                        if kind == "block" {
                                            kind.push_str("_call");
                                        }
                                        fmtln!(fmt, "{member}: mapper.map_{kind}({member}),");
                                    }
                                    OperandKindFields::VariableArgs => {
                                        fmtln!(fmt, "{member}: mapper.map_value_list({member}),");
                                    }
                                    OperandKindFields::ImmValue |
                                    OperandKindFields::ImmEnum(_) |
                                    OperandKindFields::TypeVar(_) => fmtln!(fmt, "{member},"),
                                }
                            }
                        });
                    });
                }
            });
        });
    });
}

fn gen_bool_accessor<T: Fn(&Instruction) -> bool>(
    all_inst: &AllInstructions,
    get_attr: T,
    name: &'static str,
    doc: &'static str,
    fmt: &mut Formatter,
) {
    fmt.doc_comment(doc);
    fmt.add_block(&format!("pub fn {name}(self) -> bool"), |fmt| {
        let mut m = Match::new("self");
        for inst in all_inst.iter() {
            if get_attr(inst) {
                m.arm_no_fields(format!("Self::{}", inst.camel_name), "true");
            }
        }
        m.arm_no_fields("_", "false");
        fmt.add_match(m);
    });
    fmt.empty_line();
}

fn gen_opcodes(all_inst: &AllInstructions, fmt: &mut Formatter) {
    fmt.doc_comment(
        r#"
        An instruction opcode.

        All instructions from all supported ISAs are present.
    "#,
    );
    fmt.line("#[repr(u8)]");
    fmt.line("#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]");
    fmt.line(
        r#"#[cfg_attr(
            feature = "enable-serde",
            derive(serde_derive::Serialize, serde_derive::Deserialize)
        )]"#,
    );

    // We explicitly set the discriminant of the first variant to 1, which allows us to take
    // advantage of the NonZero optimization, meaning that wrapping enums can use the 0
    // discriminant instead of increasing the size of the whole type, and so the size of
    // Option<Opcode> is the same as Opcode's.
    fmt.add_block("pub enum Opcode", |fmt| {
        let mut is_first_opcode = true;
        for inst in all_inst.iter() {
            fmt.doc_comment(format!("`{}`. ({})", inst, inst.format.name));

            // Document polymorphism.
            if let Some(poly) = &inst.polymorphic_info {
                if poly.use_typevar_operand {
                    let op_num = inst.value_opnums[inst.format.typevar_operand.unwrap()];
                    fmt.doc_comment(format!(
                        "Type inferred from `{}`.",
                        inst.operands_in[op_num].name
                    ));
                }
            }

            // Enum variant itself.
            if is_first_opcode {
                fmtln!(fmt, "{} = 1,", inst.camel_name);
                is_first_opcode = false;
            } else {
                fmtln!(fmt, "{},", inst.camel_name)
            }
        }
    });
    fmt.empty_line();

    fmt.add_block("impl Opcode", |fmt| {
        gen_bool_accessor(
            all_inst,
            |inst| inst.is_terminator,
            "is_terminator",
            "True for instructions that terminate the block",
            fmt,
        );
        gen_bool_accessor(
            all_inst,
            |inst| inst.is_branch,
            "is_branch",
            "True for all branch or jump instructions.",
            fmt,
        );
        gen_bool_accessor(
            all_inst,
            |inst| inst.is_call,
            "is_call",
            "Is this a call instruction?",
            fmt,
        );
        gen_bool_accessor(
            all_inst,
            |inst| inst.is_return,
            "is_return",
            "Is this a return instruction?",
            fmt,
        );
        gen_bool_accessor(
            all_inst,
            |inst| inst.can_load,
            "can_load",
            "Can this instruction read from memory?",
            fmt,
        );
        gen_bool_accessor(
            all_inst,
            |inst| inst.can_store,
            "can_store",
            "Can this instruction write to memory?",
            fmt,
        );
        gen_bool_accessor(
            all_inst,
            |inst| inst.can_trap,
            "can_trap",
            "Can this instruction cause a trap?",
            fmt,
        );
        gen_bool_accessor(
            all_inst,
            |inst| inst.other_side_effects,
            "other_side_effects",
            "Does this instruction have other side effects besides can_* flags?",
            fmt,
        );
        gen_bool_accessor(
            all_inst,
            |inst| inst.side_effects_idempotent,
            "side_effects_idempotent",
            "Despite having side effects, is this instruction okay to GVN?",
            fmt,
        );

        // Generate an opcode list, for iterating over all known opcodes.
        fmt.doc_comment("All cranelift opcodes.");
        fmt.add_block("pub fn all() -> &'static [Opcode]", |fmt| {
            fmt.line("return &[");
            for inst in all_inst {
                fmt.indent(|fmt| {
                    fmtln!(fmt, "Opcode::{},", inst.camel_name);
                });
            }
            fmt.line("];");
        });
        fmt.empty_line();
    });
    fmt.empty_line();

    // Generate a private opcode_format table.
    fmtln!(
        fmt,
        "const OPCODE_FORMAT: [InstructionFormat; {}] = [",
        all_inst.len()
    );
    fmt.indent(|fmt| {
        for inst in all_inst.iter() {
            fmtln!(
                fmt,
                "InstructionFormat::{}, // {}",
                inst.format.name,
                inst.name
            );
        }
    });
    fmtln!(fmt, "];");
    fmt.empty_line();

    // Generate a private opcode_name function.
    fmt.add_block("fn opcode_name(opc: Opcode) -> &\'static str", |fmt| {
        let mut m = Match::new("opc");
        for inst in all_inst.iter() {
            m.arm_no_fields(
                format!("Opcode::{}", inst.camel_name),
                format!("\"{}\"", inst.name),
            );
        }
        fmt.add_match(m);
    });
    fmt.empty_line();

    // Generate an opcode hash table for looking up opcodes by name.
    let hash_table =
        crate::constant_hash::generate_table(all_inst.iter(), all_inst.len(), |inst| {
            constant_hash::simple_hash(&inst.name)
        });
    fmtln!(
        fmt,
        "const OPCODE_HASH_TABLE: [Option<Opcode>; {}] = [",
        hash_table.len()
    );
    fmt.indent(|fmt| {
        for i in hash_table {
            match i {
                Some(i) => fmtln!(fmt, "Some(Opcode::{}),", i.camel_name),
                None => fmtln!(fmt, "None,"),
            }
        }
    });
    fmtln!(fmt, "];");
    fmt.empty_line();
}

/// Get the value type constraint for an SSA value operand, where
/// `ctrl_typevar` is the controlling type variable.
///
/// Each operand constraint is represented as a string, one of:
/// - `Concrete(vt)`, where `vt` is a value type name.
/// - `Free(idx)` where `idx` is an index into `type_sets`.
/// - `Same`, `Lane`, `AsTruthy` for controlling typevar-derived constraints.
fn get_constraint<'entries, 'table>(
    operand: &'entries Operand,
    ctrl_typevar: Option<&TypeVar>,
    type_sets: &'table mut UniqueTable<'entries, TypeSet>,
) -> String {
    assert!(operand.is_value());
    let type_var = operand.type_var().unwrap();

    if let Some(typ) = type_var.singleton_type() {
        return format!("Concrete({})", typ.rust_name());
    }

    if let Some(free_typevar) = type_var.free_typevar() {
        if ctrl_typevar.is_some() && free_typevar != *ctrl_typevar.unwrap() {
            assert!(type_var.base.is_none());
            return format!("Free({})", type_sets.add(type_var.get_raw_typeset()));
        }
    }

    if let Some(base) = &type_var.base {
        assert!(base.type_var == *ctrl_typevar.unwrap());
        return camel_case(base.derived_func.name());
    }

    assert!(type_var == ctrl_typevar.unwrap());
    "Same".into()
}

fn gen_bitset<'a, T: IntoIterator<Item = &'a u16>>(
    iterable: T,
    name: &'static str,
    field_size: u8,
    fmt: &mut Formatter,
) {
    let bits = iterable.into_iter().fold(0, |acc, x| {
        assert!(x.is_power_of_two());
        assert!(u32::from(*x) < (1 << u32::from(field_size)));
        acc | x
    });
    fmtln!(fmt, "{}: ScalarBitSet::<u{}>({}),", name, field_size, bits);
}

fn iterable_to_string<I: fmt::Display, T: IntoIterator<Item = I>>(iterable: T) -> String {
    let elems = iterable
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    format!("{{{elems}}}")
}

fn typeset_to_string(ts: &TypeSet) -> String {
    let mut result = format!("TypeSet(lanes={}", iterable_to_string(&ts.lanes));
    if !ts.ints.is_empty() {
        result += &format!(", ints={}", iterable_to_string(&ts.ints));
    }
    if !ts.floats.is_empty() {
        result += &format!(", floats={}", iterable_to_string(&ts.floats));
    }
    result += ")";
    result
}

/// Generate the table of ValueTypeSets described by type_sets.
pub(crate) fn gen_typesets_table(type_sets: &UniqueTable<TypeSet>, fmt: &mut Formatter) {
    if type_sets.len() == 0 {
        return;
    }

    fmt.comment("Table of value type sets.");
    assert!(type_sets.len() <= TYPESET_LIMIT, "Too many type sets!");
    fmtln!(
        fmt,
        "const TYPE_SETS: [ir::instructions::ValueTypeSet; {}] = [",
        type_sets.len()
    );
    fmt.indent(|fmt| {
        for ts in type_sets.iter() {
            fmt.add_block("ir::instructions::ValueTypeSet", |fmt| {
                fmt.comment(typeset_to_string(ts));
                gen_bitset(&ts.lanes, "lanes", 16, fmt);
                gen_bitset(&ts.dynamic_lanes, "dynamic_lanes", 16, fmt);
                gen_bitset(&ts.ints, "ints", 8, fmt);
                gen_bitset(&ts.floats, "floats", 8, fmt);
            });
            fmt.line(",");
        }
    });
    fmtln!(fmt, "];");
}

/// Generate value type constraints for all instructions.
/// - Emit a compact constant table of ValueTypeSet objects.
/// - Emit a compact constant table of OperandConstraint objects.
/// - Emit an opcode-indexed table of instruction constraints.
fn gen_type_constraints(all_inst: &AllInstructions, fmt: &mut Formatter) {
    // Table of TypeSet instances.
    let mut type_sets = UniqueTable::new();

    // Table of operand constraint sequences (as tuples). Each operand
    // constraint is represented as a string, one of:
    // - `Concrete(vt)`, where `vt` is a value type name.
    // - `Free(idx)` where `idx` is an index into `type_sets`.
    // - `Same`, `Lane`, `AsTruthy` for controlling typevar-derived constraints.
    let mut operand_seqs = UniqueSeqTable::new();

    // Preload table with constraints for typical binops.
    operand_seqs.add(&vec!["Same".to_string(); 3]);

    fmt.comment("Table of opcode constraints.");
    fmtln!(
        fmt,
        "const OPCODE_CONSTRAINTS: [OpcodeConstraints; {}] = [",
        all_inst.len()
    );
    fmt.indent(|fmt| {
        for inst in all_inst.iter() {
            let (ctrl_typevar, ctrl_typeset) = if let Some(poly) = &inst.polymorphic_info {
                let index = type_sets.add(poly.ctrl_typevar.get_raw_typeset());
                (Some(&poly.ctrl_typevar), index)
            } else {
                (None, TYPESET_LIMIT)
            };

            // Collect constraints for the value results, not including `variable_args` results
            // which are always special cased.
            let mut constraints = Vec::new();
            for &index in &inst.value_results {
                constraints.push(get_constraint(&inst.operands_out[index], ctrl_typevar, &mut type_sets));
            }
            for &index in &inst.value_opnums {
                constraints.push(get_constraint(&inst.operands_in[index], ctrl_typevar, &mut type_sets));
            }

            let constraint_offset = operand_seqs.add(&constraints);

            let fixed_results = inst.value_results.len();
            let fixed_values = inst.value_opnums.len();

            // Can the controlling type variable be inferred from the designated operand?
            let use_typevar_operand = if let Some(poly) = &inst.polymorphic_info {
                poly.use_typevar_operand
            } else {
                false
            };

            // Can the controlling type variable be inferred from the result?
            let use_result = fixed_results > 0 && inst.operands_out[inst.value_results[0]].type_var() == ctrl_typevar;

            // Are we required to use the designated operand instead of the result?
            let requires_typevar_operand = use_typevar_operand && !use_result;

            fmt.comment(
                format!("{}: fixed_results={}, use_typevar_operand={}, requires_typevar_operand={}, fixed_values={}",
                inst.camel_name,
                fixed_results,
                use_typevar_operand,
                requires_typevar_operand,
                fixed_values)
            );
            fmt.comment(format!("Constraints=[{}]", constraints
                .iter()
                .map(|x| format!("'{x}'"))
                .collect::<Vec<_>>()
                .join(", ")));
            if let Some(poly) = &inst.polymorphic_info {
                fmt.comment(format!("Polymorphic over {}", typeset_to_string(poly.ctrl_typevar.get_raw_typeset())));
            }

            // Compute the bit field encoding, c.f. instructions.rs.
            assert!(fixed_results < 8 && fixed_values < 8, "Bit field encoding too tight");
            let mut flags = fixed_results; // 3 bits
            if use_typevar_operand {
                flags |= 1<<3; // 4th bit
            }
            if requires_typevar_operand {
                flags |= 1<<4; // 5th bit
            }
            flags |= fixed_values << 5; // 6th bit and more

            fmt.add_block("OpcodeConstraints",|fmt| {
                fmtln!(fmt, "flags: {:#04x},", flags);
                fmtln!(fmt, "typeset_offset: {},", ctrl_typeset);
                fmtln!(fmt, "constraint_offset: {},", constraint_offset);
            });
            fmt.line(",");
        }
    });
    fmtln!(fmt, "];");
    fmt.empty_line();

    gen_typesets_table(&type_sets, fmt);
    fmt.empty_line();

    fmt.comment("Table of operand constraint sequences.");
    fmtln!(
        fmt,
        "const OPERAND_CONSTRAINTS: [OperandConstraint; {}] = [",
        operand_seqs.len()
    );
    fmt.indent(|fmt| {
        for constraint in operand_seqs.iter() {
            fmtln!(fmt, "OperandConstraint::{},", constraint);
        }
    });
    fmtln!(fmt, "];");
}

/// Emit member initializers for an instruction format.
fn gen_member_inits(format: &InstructionFormat, fmt: &mut Formatter) {
    // Immediate operands.
    // We have local variables with the same names as the members.
    for f in &format.imm_fields {
        fmtln!(fmt, "{},", f.member);
    }

    // Value operands.
    if format.has_value_list {
        fmt.line("args,");
    } else if format.num_value_operands == 1 {
        fmt.line("arg: arg0,");
    } else if format.num_value_operands > 1 {
        let mut args = Vec::new();
        for i in 0..format.num_value_operands {
            args.push(format!("arg{i}"));
        }
        fmtln!(fmt, "args: [{}],", args.join(", "));
    }

    // Block operands
    match format.num_block_operands {
        0 => (),
        1 => fmt.line("destination: block0"),
        n => {
            let mut blocks = Vec::new();
            for i in 0..n {
                blocks.push(format!("block{i}"));
            }
            fmtln!(fmt, "blocks: [{}],", blocks.join(", "));
        }
    }
}

/// Emit a method for creating and inserting an instruction format.
///
/// All instruction formats take an `opcode` argument and a `ctrl_typevar` argument for deducing
/// the result types.
fn gen_format_constructor(format: &InstructionFormat, fmt: &mut Formatter) {
    // Construct method arguments.
    let mut args = vec![
        "self".to_string(),
        "opcode: Opcode".into(),
        "ctrl_typevar: Type".into(),
    ];

    // Normal operand arguments. Start with the immediate operands.
    for f in &format.imm_fields {
        args.push(format!("{}: {}", f.member, f.kind.rust_type));
    }

    // Then the block operands.
    args.extend((0..format.num_block_operands).map(|i| format!("block{i}: ir::BlockCall")));

    // Then the value operands.
    if format.has_value_list {
        // Take all value arguments as a finished value list. The value lists
        // are created by the individual instruction constructors.
        args.push("args: ir::ValueList".into());
    } else {
        // Take a fixed number of value operands.
        for i in 0..format.num_value_operands {
            args.push(format!("arg{i}: Value"));
        }
    }

    let proto = format!(
        "{}({}) -> (Inst, &'f mut ir::DataFlowGraph)",
        format.name,
        args.join(", ")
    );

    let imms_need_masking = format
        .imm_fields
        .iter()
        .any(|f| f.kind.rust_type == "ir::immediates::Imm64");

    fmt.doc_comment(format.to_string());
    fmt.line("#[allow(non_snake_case, reason = \"generated code\")]");
    fmt.add_block(&format!("fn {proto}"), |fmt| {
        // Generate the instruction data.
        fmt.add_block(&format!(
                "let{} data = ir::InstructionData::{}",
                if imms_need_masking { " mut" } else { "" },
                format.name
            ), |fmt| {
            fmt.line("opcode,");
            gen_member_inits(format, fmt);
        });
        fmtln!(fmt, ";");

        if imms_need_masking {
            fmtln!(fmt, "data.mask_immediates(ctrl_typevar);");
        }

        // Assert that this opcode belongs to this format
        fmtln!(fmt, "debug_assert_eq!(opcode.format(), InstructionFormat::from(&data), \"Wrong InstructionFormat for Opcode: {{opcode}}\");");

        fmt.line("self.build(data, ctrl_typevar)");
    });
}

/// Emit a method for generating the instruction `inst`.
///
/// The method will create and insert an instruction, then return the result values, or the
/// instruction reference itself for instructions that don't have results.
fn gen_inst_builder(inst: &Instruction, format: &InstructionFormat, fmt: &mut Formatter) {
    // Construct method arguments.
    let mut args = vec![String::new()];

    let mut args_doc = Vec::new();
    let mut rets_doc = Vec::new();

    // The controlling type variable will be inferred from the input values if
    // possible. Otherwise, it is the first method argument.
    if let Some(poly) = &inst.polymorphic_info {
        if !poly.use_typevar_operand {
            args.push(format!("{}: crate::ir::Type", poly.ctrl_typevar.name));
            args_doc.push(format!(
                "- {} (controlling type variable): {}",
                poly.ctrl_typevar.name, poly.ctrl_typevar.doc
            ));
        }
    }

    let mut tmpl_types = Vec::new();
    let mut into_args = Vec::new();
    let mut block_args = Vec::new();
    let mut lifetime_param = None;
    for op in &inst.operands_in {
        if op.kind.is_block() {
            args.push(format!("{}_label: {}", op.name, "ir::Block"));
            args_doc.push(format!(
                "- {}_label: {}",
                op.name, "Destination basic block"
            ));

            let lifetime = *lifetime_param.get_or_insert_with(|| {
                tmpl_types.insert(0, "'a".to_string());
                "'a"
            });
            args.push(format!(
                "{}_args: impl IntoIterator<Item = &{} BlockArg>",
                op.name, lifetime,
            ));
            args_doc.push(format!("- {}_args: {}", op.name, "Block arguments"));

            block_args.push(op);
        } else {
            let t = if op.is_immediate() {
                let t = format!("T{}", tmpl_types.len() + 1);
                tmpl_types.push(format!("{}: Into<{}>", t, op.kind.rust_type));
                into_args.push(op.name);
                t
            } else {
                op.kind.rust_type.to_string()
            };
            args.push(format!("{}: {}", op.name, t));
            args_doc.push(format!("- {}: {}", op.name, op.doc()));
        }
    }

    // We need to mutate `self` if this instruction accepts a value list, or will construct
    // BlockCall values.
    if format.has_value_list || !block_args.is_empty() {
        args[0].push_str("mut self");
    } else {
        args[0].push_str("self");
    }

    for op in &inst.operands_out {
        rets_doc.push(format!("- {}: {}", op.name, op.doc()));
    }

    let rtype = match inst.value_results.len() {
        0 => "Inst".into(),
        1 => "Value".into(),
        _ => format!("({})", vec!["Value"; inst.value_results.len()].join(", ")),
    };

    let tmpl = if !tmpl_types.is_empty() {
        format!("<{}>", tmpl_types.join(", "))
    } else {
        "".into()
    };

    let proto = format!(
        "{}{}({}) -> {}",
        inst.snake_name(),
        tmpl,
        args.join(", "),
        rtype
    );

    fmt.doc_comment(&inst.doc);
    if !args_doc.is_empty() {
        fmt.line("///");
        fmt.doc_comment("Inputs:");
        fmt.line("///");
        for doc_line in args_doc {
            fmt.doc_comment(doc_line);
        }
    }
    if !rets_doc.is_empty() {
        fmt.line("///");
        fmt.doc_comment("Outputs:");
        fmt.line("///");
        for doc_line in rets_doc {
            fmt.doc_comment(doc_line);
        }
    }

    fmt.line("#[allow(non_snake_case, reason = \"generated code\")]");
    fmt.add_block(&format!("fn {proto}"), |fmt| {
        // Convert all of the `Into<>` arguments.
        for arg in into_args {
            fmtln!(fmt, "let {} = {}.into();", arg, arg);
        }

        // Convert block references
        for op in block_args {
            fmtln!(
                fmt,
                "let {0} = self.data_flow_graph_mut().block_call({0}_label, {0}_args);",
                op.name
            );
        }

        // Arguments for instruction constructor.
        let first_arg = format!("Opcode::{}", inst.camel_name);
        let mut args = vec![first_arg.as_str()];
        if let Some(poly) = &inst.polymorphic_info {
            if poly.use_typevar_operand {
                // Infer the controlling type variable from the input operands.
                let op_num = inst.value_opnums[format.typevar_operand.unwrap()];
                fmtln!(
                    fmt,
                    "let ctrl_typevar = self.data_flow_graph().value_type({});",
                    inst.operands_in[op_num].name
                );

                // The format constructor will resolve the result types from the type var.
                args.push("ctrl_typevar");
            } else {
                // This was an explicit method argument.
                args.push(&poly.ctrl_typevar.name);
            }
        } else {
            // No controlling type variable needed.
            args.push("types::INVALID");
        }

        // Now add all of the immediate operands to the constructor arguments.
        for &op_num in &inst.imm_opnums {
            args.push(inst.operands_in[op_num].name);
        }

        // Finally, the value operands.
        if format.has_value_list {
            // We need to build a value list with all the arguments.
            fmt.line("let mut vlist = ir::ValueList::default();");
            args.push("vlist");
            fmt.line("{");
            fmt.indent(|fmt| {
                fmt.line("let pool = &mut self.data_flow_graph_mut().value_lists;");
                for op in &inst.operands_in {
                    if op.is_value() {
                        fmtln!(fmt, "vlist.push({}, pool);", op.name);
                    } else if op.is_varargs() {
                        fmtln!(fmt, "vlist.extend({}.iter().cloned(), pool);", op.name);
                    }
                }
            });
            fmt.line("}");
        } else {
            // With no value list, we're guaranteed to just have a set of fixed value operands.
            for &op_num in &inst.value_opnums {
                args.push(inst.operands_in[op_num].name);
            }
        }

        // Call to the format constructor,
        let fcall = format!("self.{}({})", format.name, args.join(", "));

        fmtln!(fmt, "let (inst, dfg) = {};", fcall);
        fmtln!(
            fmt,
            "crate::trace!(\"inserted {{inst:?}}: {{}}\", dfg.display_inst(inst));"
        );

        if inst.value_results.is_empty() {
            fmtln!(fmt, "inst");
            return;
        }

        if inst.value_results.len() == 1 {
            fmt.line("dfg.first_result(inst)");
        } else {
            fmtln!(
                fmt,
                "let results = &dfg.inst_results(inst)[0..{}];",
                inst.value_results.len()
            );
            fmtln!(
                fmt,
                "({})",
                inst.value_results
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("results[{i}]"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    });
}

/// Generate a Builder trait with methods for all instructions.
fn gen_builder(
    instructions: &AllInstructions,
    formats: &[Rc<InstructionFormat>],
    fmt: &mut Formatter,
) {
    fmt.doc_comment(
        r#"
        Convenience methods for building instructions.

        The `InstBuilder` trait has one method per instruction opcode for
        conveniently constructing the instruction with minimum arguments.
        Polymorphic instructions infer their result types from the input
        arguments when possible. In some cases, an explicit `ctrl_typevar`
        argument is required.

        The opcode methods return the new instruction's result values, or
        the `Inst` itself for instructions that don't have any results.

        There is also a method per instruction format. These methods all
        return an `Inst`.

        When an address to a load or store is specified, its integer
        size is required to be equal to the platform's pointer width.
    "#,
    );
    fmt.add_block("pub trait InstBuilder<'f>: InstBuilderBase<'f>", |fmt| {
        for inst in instructions.iter() {
            gen_inst_builder(inst, &inst.format, fmt);
            fmt.empty_line();
        }
        for (i, format) in formats.iter().enumerate() {
            gen_format_constructor(format, fmt);
            if i + 1 != formats.len() {
                fmt.empty_line();
            }
        }
    });
}

pub(crate) fn generate(
    formats: &[Rc<InstructionFormat>],
    all_inst: &AllInstructions,
    opcode_filename: &str,
    inst_builder_filename: &str,
    out_dir: &std::path::Path,
) -> Result<(), error::Error> {
    // Opcodes.
    let mut fmt = Formatter::new(Language::Rust);
    gen_formats(&formats, &mut fmt);
    gen_instruction_data(&formats, &mut fmt);
    fmt.empty_line();
    gen_instruction_data_impl(&formats, &mut fmt);
    fmt.empty_line();
    gen_opcodes(all_inst, &mut fmt);
    fmt.empty_line();
    gen_type_constraints(all_inst, &mut fmt);
    fmt.write(opcode_filename, out_dir)?;

    // Instruction builder.
    let mut fmt = Formatter::new(Language::Rust);
    gen_builder(all_inst, &formats, &mut fmt);
    fmt.write(inst_builder_filename, out_dir)?;

    Ok(())
}
