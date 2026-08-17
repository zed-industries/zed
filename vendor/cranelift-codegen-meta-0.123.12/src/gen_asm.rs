//! Generate the Cranelift-specific integration of the x64 assembler.

use cranelift_assembler_x64_meta::dsl::{
    Feature, Format, Inst, Location, Mutability, Operand, OperandKind, RegClass,
};
use cranelift_srcgen::{Formatter, fmtln};

/// This factors out use of the assembler crate name.
const ASM: &str = "cranelift_assembler_x64";

fn include_inst(inst: &Inst) -> bool {
    // No need to worry about this instruction shape in ISLE as it's generated
    // in ABI code, not ISLE.
    if inst.mnemonic.starts_with("push") {
        return false;
    }

    true
}

/// Returns the Rust type used for the `IsleConstructorRaw` variants.
fn rust_param_raw(op: &Operand) -> String {
    match op.location.kind() {
        OperandKind::Imm(loc) => {
            let bits = loc.bits();
            if op.extension.is_sign_extended() {
                format!("i{bits}")
            } else {
                format!("u{bits}")
            }
        }
        OperandKind::RegMem(rm) => {
            let reg = rm.reg_class().unwrap();
            let aligned = if op.align { "Aligned" } else { "" };
            format!("&{reg}Mem{aligned}")
        }
        OperandKind::Mem(_) => {
            format!("&SyntheticAmode")
        }
        OperandKind::Reg(r) | OperandKind::FixedReg(r) => r.reg_class().unwrap().to_string(),
    }
}

/// Returns the conversion function, if any, when converting the ISLE type for
/// this parameter to the assembler type for this parameter. Effectively
/// converts `self.rust_param_raw()` to the assembler type.
fn rust_convert_isle_to_assembler(op: &Operand) -> String {
    match op.location.kind() {
        OperandKind::Imm(loc) => {
            let bits = loc.bits();
            let ty = if op.extension.is_sign_extended() {
                "Simm"
            } else {
                "Imm"
            };
            format!("{ASM}::{ty}{bits}::new({loc})")
        }
        OperandKind::FixedReg(r) => {
            let reg = r.reg_class().unwrap().to_string().to_lowercase();
            match op.mutability {
                Mutability::Read => format!("{ASM}::Fixed({r})"),
                Mutability::Write => {
                    format!("{ASM}::Fixed(self.temp_writable_{reg}())")
                }
                Mutability::ReadWrite => {
                    format!("self.convert_{reg}_to_assembler_fixed_read_write_{reg}({r})")
                }
            }
        }
        OperandKind::Reg(r) => {
            let reg = r.reg_class().unwrap();
            let reg_lower = reg.to_string().to_lowercase();
            match op.mutability {
                Mutability::Read => {
                    format!("{ASM}::{reg}::new({r})")
                }
                Mutability::Write => {
                    format!("{ASM}::{reg}::new(self.temp_writable_{reg_lower}())")
                }
                Mutability::ReadWrite => {
                    format!("self.convert_{reg_lower}_to_assembler_read_write_{reg_lower}({r})")
                }
            }
        }
        OperandKind::RegMem(rm) => {
            let reg = rm.reg_class().unwrap().to_string().to_lowercase();
            let mut_ = op.mutability.generate_snake_case();
            let align = if op.align { "_aligned" } else { "" };
            format!("self.convert_{reg}_mem_to_assembler_{mut_}_{reg}_mem{align}({rm})")
        }
        OperandKind::Mem(mem) => format!("self.convert_amode_to_assembler_amode({mem})"),
    }
}

/// `fn x64_<inst>(&mut self, <params>) -> Inst<R> { ... }`
///
/// # Panics
///
/// This function panics if the instruction has no operands.
fn generate_macro_inst_fn(f: &mut Formatter, inst: &Inst) {
    use OperandKind::*;

    let struct_name = inst.name();
    let operands = inst.format.operands.iter().cloned().collect::<Vec<_>>();
    let results = operands
        .iter()
        .filter(|o| o.mutability.is_write())
        .collect::<Vec<_>>();
    let rust_params = operands
        .iter()
        .filter(|o| is_raw_operand_param(o))
        .map(|o| format!("{}: {}", o.location, rust_param_raw(o)))
        .chain(if inst.has_trap {
            Some(format!("trap: &TrapCode"))
        } else {
            None
        })
        .collect::<Vec<_>>()
        .join(", ");
    f.add_block(
        &format!("fn x64_{struct_name}_raw(&mut self, {rust_params}) -> AssemblerOutputs"),
        |f| {
            f.comment("Convert ISLE types to assembler types.");
            for op in operands.iter() {
                let loc = op.location;
                let cvt = rust_convert_isle_to_assembler(op);
                fmtln!(f, "let {loc} = {cvt};");
            }
            let mut args = operands
                .iter()
                .map(|o| format!("{}.clone()", o.location))
                .collect::<Vec<_>>();
            if inst.has_trap {
                args.push(format!("{ASM}::TrapCode(trap.as_raw())"));
            }
            let args = args.join(", ");
            f.empty_line();

            f.comment("Build the instruction.");
            fmtln!(
                f,
                "let inst = {ASM}::inst::{struct_name}::new({args}).into();"
            );
            fmtln!(f, "let inst = MInst::External {{ inst }};");
            f.empty_line();

            // When an instruction writes to an operand, Cranelift expects a
            // returned value to use in other instructions: we return this
            // information in the `AssemblerOutputs` struct defined in ISLE
            // (below). The general rule here is that memory stores will create
            // a `SideEffect` whereas for write or read-write registers we will
            // return some form of `Ret*`.
            f.comment("Return a type ISLE can work with.");
            let access_reg = |op: &Operand| match op.mutability {
                Mutability::Read => unreachable!(),
                Mutability::Write => "to_reg()",
                Mutability::ReadWrite => "write.to_reg()",
            };
            let ty_var_of_reg = |loc: Location| {
                let ty = loc.reg_class().unwrap().to_string();
                let var = ty.to_lowercase();
                (ty, var)
            };
            match results.as_slice() {
                [] => fmtln!(f, "AssemblerOutputs::SideEffect {{ inst }}"),
                [op] => match op.location.kind() {
                    Imm(_) => unreachable!(),
                    Reg(r) | FixedReg(r) => {
                        let (ty, var) = ty_var_of_reg(r);
                        fmtln!(f, "let {var} = {r}.as_ref().{};", access_reg(op));
                        fmtln!(f, "AssemblerOutputs::Ret{ty} {{ inst, {var} }}");
                    }
                    Mem(_) => {
                        fmtln!(f, "AssemblerOutputs::SideEffect {{ inst }}")
                    }
                    RegMem(rm) => {
                        let (ty, var) = ty_var_of_reg(rm);
                        f.add_block(&format!("match {rm}"), |f| {
                            f.add_block(&format!("{ASM}::{ty}Mem::{ty}(reg) => "), |f| {
                                fmtln!(f, "let {var} = reg.{};", access_reg(op));
                                fmtln!(f, "AssemblerOutputs::Ret{ty} {{ inst, {var} }} ");
                            });
                            f.add_block(&format!("{ASM}::{ty}Mem::Mem(_) => "), |f| {
                                fmtln!(f, "AssemblerOutputs::SideEffect {{ inst }} ");
                            });
                        });
                    }
                },
                // For now, we assume that if there are two results, they are
                // coming from a register-writing instruction like `mul`. The
                // `match` below can be expanded as needed.
                [op1, op2] => match (op1.location.kind(), op2.location.kind()) {
                    (FixedReg(loc1) | Reg(loc1), FixedReg(loc2) | Reg(loc2)) => {
                        fmtln!(f, "let one = {loc1}.as_ref().{}.to_reg();", access_reg(op1));
                        fmtln!(f, "let two = {loc2}.as_ref().{}.to_reg();", access_reg(op2));
                        fmtln!(f, "let regs = ValueRegs::two(one, two);");
                        fmtln!(f, "AssemblerOutputs::RetValueRegs {{ inst, regs }}");
                    }
                    (Reg(reg), Mem(_)) | (Mem(_) | RegMem(_), Reg(reg) | FixedReg(reg)) => {
                        let (ty, var) = ty_var_of_reg(reg);
                        fmtln!(f, "let {var} = {reg}.as_ref().{};", access_reg(op2));
                        fmtln!(f, "AssemblerOutputs::Ret{ty} {{ inst, {var} }}");
                    }
                    _ => unimplemented!("unhandled results: {results:?}"),
                },

                [op1, op2, op3] => match (
                    op1.location.kind(),
                    op2.location.kind(),
                    op3.location.kind(),
                ) {
                    (FixedReg(loc1), FixedReg(loc2), Mem(_)) => {
                        fmtln!(f, "let one = {loc1}.as_ref().{}.to_reg();", access_reg(op1));
                        fmtln!(f, "let two = {loc2}.as_ref().{}.to_reg();", access_reg(op2));
                        fmtln!(f, "let regs = ValueRegs::two(one, two);");
                        fmtln!(f, "AssemblerOutputs::RetValueRegs {{ inst, regs }}");
                    }
                    _ => unimplemented!("unhandled results: {results:?}"),
                },

                _ => panic!("instruction has more than one result"),
            }
        },
    );
}

/// Generate the `isle_assembler_methods!` macro.
pub fn generate_rust_macro(f: &mut Formatter, insts: &[Inst]) {
    fmtln!(f, "#[doc(hidden)]");
    fmtln!(f, "macro_rules! isle_assembler_methods {{");
    f.indent(|f| {
        fmtln!(f, "() => {{");
        f.indent(|f| {
            for inst in insts {
                if include_inst(inst) {
                    generate_macro_inst_fn(f, inst);
                }
            }
        });
        fmtln!(f, "}};");
    });
    fmtln!(f, "}}");
}

/// Returns the type of this operand in ISLE as a part of the ISLE "raw"
/// constructors.
fn isle_param_raw(op: &Operand) -> String {
    match op.location.kind() {
        OperandKind::Imm(loc) => {
            let bits = loc.bits();
            if op.extension.is_sign_extended() {
                format!("i{bits}")
            } else {
                format!("u{bits}")
            }
        }
        OperandKind::Reg(r) | OperandKind::FixedReg(r) => r.reg_class().unwrap().to_string(),
        OperandKind::Mem(_) => {
            if op.align {
                unimplemented!("no way yet to mark an SyntheticAmode as aligned")
            } else {
                "SyntheticAmode".to_string()
            }
        }
        OperandKind::RegMem(rm) => {
            let reg = rm.reg_class().unwrap();
            let aligned = if op.align { "Aligned" } else { "" };
            format!("{reg}Mem{aligned}")
        }
    }
}

/// Different kinds of ISLE constructors generated for a particular instruction.
///
/// One instruction may generate a single constructor or multiple constructors.
/// For example an instruction that writes its result to a register will
/// generate only a single constructor. An instruction where the destination
/// read/write operand is `GprMem` will generate two constructors though, one
/// for memory and one for in registers.
#[derive(Copy, Clone, Debug)]
enum IsleConstructor {
    /// This constructor only produces a side effect, meaning that the
    /// instruction does not produce results in registers. This may produce
    /// a result in memory, however.
    RetMemorySideEffect,

    /// This constructor produces a `Gpr` value, meaning that the instruction
    /// will write its result to a single GPR register.
    RetGpr,

    /// This is similar to `RetGpr`, but for XMM registers.
    RetXmm,

    /// This "special" constructor captures multiple written-to registers (e.g.
    /// `mul`).
    RetValueRegs,

    /// This constructor does not return any results, but produces a side effect affecting EFLAGs.
    NoReturnSideEffect,

    /// This constructor produces no results, but the flags register is written,
    /// so a `ProducesFlags` value is returned with a side effect.
    ProducesFlagsSideEffect,

    /// This instructions reads EFLAGS, and returns a single gpr, so this
    /// creates `ConsumesFlags.ConsumesFlagsReturnsReg`.
    ConsumesFlagsReturnsGpr,
}

impl IsleConstructor {
    /// Returns the result type, in ISLE, that this constructor generates.
    fn result_ty(&self) -> &'static str {
        match self {
            IsleConstructor::RetGpr => "Gpr",
            IsleConstructor::RetXmm => "Xmm",
            IsleConstructor::RetValueRegs => "ValueRegs",
            IsleConstructor::NoReturnSideEffect | IsleConstructor::RetMemorySideEffect => {
                "SideEffectNoResult"
            }
            IsleConstructor::ProducesFlagsSideEffect => "ProducesFlags",
            IsleConstructor::ConsumesFlagsReturnsGpr => "ConsumesFlags",
        }
    }

    /// Returns the constructor used to convert an `AssemblerOutput` into the
    /// type returned by [`Self::result_ty`].
    fn conversion_constructor(&self) -> &'static str {
        match self {
            IsleConstructor::NoReturnSideEffect | IsleConstructor::RetMemorySideEffect => {
                "defer_side_effect"
            }
            IsleConstructor::RetGpr => "emit_ret_gpr",
            IsleConstructor::RetXmm => "emit_ret_xmm",
            IsleConstructor::RetValueRegs => "emit_ret_value_regs",
            IsleConstructor::ProducesFlagsSideEffect => "asm_produce_flags_side_effect",
            IsleConstructor::ConsumesFlagsReturnsGpr => "asm_consumes_flags_returns_gpr",
        }
    }

    /// Returns the suffix used in the ISLE constructor name.
    fn suffix(&self) -> &'static str {
        match self {
            IsleConstructor::RetMemorySideEffect => "_mem",
            IsleConstructor::RetGpr
            | IsleConstructor::RetXmm
            | IsleConstructor::RetValueRegs
            | IsleConstructor::NoReturnSideEffect
            | IsleConstructor::ProducesFlagsSideEffect
            | IsleConstructor::ConsumesFlagsReturnsGpr => "",
        }
    }

    /// Returns whether this constructor will include a write-only `RegMem`
    /// operand as an argument to the constructor.
    ///
    /// Memory-based ctors take an `Amode`, but register-based ctors don't take
    /// the result as an argument and instead manufacture it internally.
    fn includes_write_only_reg_mem(&self) -> bool {
        match self {
            IsleConstructor::RetMemorySideEffect => true,
            IsleConstructor::RetGpr
            | IsleConstructor::RetXmm
            | IsleConstructor::RetValueRegs
            | IsleConstructor::NoReturnSideEffect
            | IsleConstructor::ProducesFlagsSideEffect
            | IsleConstructor::ConsumesFlagsReturnsGpr => false,
        }
    }
}

/// Returns the parameter type used for the `IsleConstructor` variant
/// provided.
fn isle_param_for_ctor(op: &Operand, ctor: IsleConstructor) -> String {
    match op.location.kind() {
        // Writable `RegMem` operands are special here: in one constructor
        // it's operating on memory so the argument is `Amode` and in the
        // other constructor it's operating on registers so the argument is
        // a `Gpr`.
        OperandKind::RegMem(_) if op.mutability.is_write() => match ctor {
            IsleConstructor::RetMemorySideEffect => "SyntheticAmode".to_string(),
            IsleConstructor::NoReturnSideEffect => "".to_string(),
            IsleConstructor::RetGpr | IsleConstructor::ConsumesFlagsReturnsGpr => "Gpr".to_string(),
            IsleConstructor::RetXmm => "Xmm".to_string(),
            IsleConstructor::RetValueRegs => "ValueRegs".to_string(),
            IsleConstructor::ProducesFlagsSideEffect => todo!(),
        },

        // everything else is the same as the "raw" variant
        _ => isle_param_raw(op),
    }
}

/// Returns the ISLE constructors that are going to be used when generating
/// this instruction.
///
/// Note that one instruction might need multiple constructors, such as one
/// for operating on memory and one for operating on registers.
fn isle_constructors(format: &Format) -> Vec<IsleConstructor> {
    use Mutability::*;
    use OperandKind::*;

    let write_operands = format
        .operands
        .iter()
        .filter(|o| o.mutability.is_write())
        .collect::<Vec<_>>();
    match &write_operands[..] {
        [] => {
            if format.eflags.is_write() {
                vec![IsleConstructor::ProducesFlagsSideEffect]
            } else {
                vec![IsleConstructor::NoReturnSideEffect]
            }
        }
        [one] => match one.mutability {
            Read => unreachable!(),
            ReadWrite | Write => match one.location.kind() {
                Imm(_) => unreachable!(),
                // One read/write register output? Output the instruction
                // and that register.
                Reg(r) | FixedReg(r) => match r.reg_class().unwrap() {
                    RegClass::Xmm => {
                        assert!(!format.eflags.is_read());
                        vec![IsleConstructor::RetXmm]
                    }
                    RegClass::Gpr => {
                        if format.eflags.is_read() {
                            vec![IsleConstructor::ConsumesFlagsReturnsGpr]
                        } else {
                            vec![IsleConstructor::RetGpr]
                        }
                    }
                },
                // One read/write memory operand? Output a side effect.
                Mem(_) => {
                    assert!(!format.eflags.is_read());
                    vec![IsleConstructor::RetMemorySideEffect]
                }
                // One read/write reg-mem output? We need constructors for
                // both variants.
                RegMem(rm) => match rm.reg_class().unwrap() {
                    RegClass::Xmm => {
                        assert!(!format.eflags.is_read());
                        vec![
                            IsleConstructor::RetXmm,
                            IsleConstructor::RetMemorySideEffect,
                        ]
                    }
                    RegClass::Gpr => {
                        if format.eflags.is_read() {
                            // FIXME: should expand this to include "consumes
                            // flags plus side effect" to model the
                            // memory-writing variant too. For example this
                            // means there's no memory-writing variant of
                            // `setcc` instructions generated.
                            vec![IsleConstructor::ConsumesFlagsReturnsGpr]
                        } else {
                            vec![
                                IsleConstructor::RetGpr,
                                IsleConstructor::RetMemorySideEffect,
                            ]
                        }
                    }
                },
            },
        },
        [one, two] => {
            assert!(!format.eflags.is_read());
            match (one.location.kind(), two.location.kind()) {
                (FixedReg(_) | Reg(_), FixedReg(_) | Reg(_)) => {
                    vec![IsleConstructor::RetValueRegs]
                }
                (Reg(r), Mem(_)) | (Mem(_) | RegMem(_), Reg(r) | FixedReg(r)) => {
                    assert!(matches!(r.reg_class().unwrap(), RegClass::Gpr));
                    vec![IsleConstructor::RetGpr]
                }
                other => panic!("unsupported number of write operands {other:?}"),
            }
        }
        [one, two, three] => {
            assert!(!format.eflags.is_read());
            match (
                one.location.kind(),
                two.location.kind(),
                three.location.kind(),
            ) {
                (FixedReg(_), FixedReg(_), Mem(_)) => {
                    vec![IsleConstructor::RetValueRegs]
                }
                other => panic!("unsupported number of write operands {other:?}"),
            }
        }

        other => panic!("unsupported number of write operands {other:?}"),
    }
}

/// Generate a "raw" constructor that simply constructs, but does not emit
/// the assembly instruction:
///
/// ```text
/// (decl x64_<inst>_raw (<params>) AssemblerOutputs)
/// (extern constructor x64_<inst>_raw x64_<inst>_raw)
/// ```
///
/// Using the "raw" constructor, we also generate "emitter" constructors
/// (see [`IsleConstructor`]). E.g., instructions that write to a register
/// will return the register:
///
/// ```text
/// (decl x64_<inst> (<params>) Gpr)
/// (rule (x64_<inst> <params>) (emit_ret_gpr (x64_<inst>_raw <params>)))
/// ```
///
/// For instructions that write to memory, we also generate an "emitter"
/// constructor with the `_mem` suffix:
///
/// ```text
/// (decl x64_<inst>_mem (<params>) SideEffectNoResult)
/// (rule (x64_<inst>_mem <params>) (defer_side_effect (x64_<inst>_raw <params>)))
/// ```
///
/// # Panics
///
/// This function panics if the instruction has no operands.
fn generate_isle_inst_decls(f: &mut Formatter, inst: &Inst) {
    let (trap_type, trap_name) = if inst.has_trap {
        (Some("TrapCode".to_string()), Some("trap".to_string()))
    } else {
        (None, None)
    };

    // First declare the "raw" constructor which is implemented in Rust
    // with `generate_isle_macro` above. This is an "extern" constructor
    // with relatively raw types. This is not intended to be used by
    // general lowering rules in ISLE.
    let struct_name = inst.name();
    let raw_name = format!("x64_{struct_name}_raw");
    let params = inst
        .format
        .operands
        .iter()
        .filter(|o| is_raw_operand_param(o))
        .collect::<Vec<_>>();
    let raw_param_tys = params
        .iter()
        .map(|o| isle_param_raw(o))
        .chain(trap_type.clone())
        .collect::<Vec<_>>()
        .join(" ");
    fmtln!(f, "(decl {raw_name} ({raw_param_tys}) AssemblerOutputs)");
    fmtln!(f, "(extern constructor {raw_name} {raw_name})");

    // Next, for each "emitter" ISLE constructor being generated, synthesize
    // a pure-ISLE constructor which delegates appropriately to the `*_raw`
    // constructor above.
    //
    // The main purpose of these constructors is to have faithful type
    // signatures for the SSA nature of VCode/ISLE, effectively translating
    // x64's type system to ISLE/VCode's type system.
    //
    // Note that the `params` from above are partitioned into explicit/implicit
    // parameters based on the `ctor` we're generating here. That means, for
    // example, that a write-only `RegMem` will have one ctor which produces a
    // register that takes no argument, but one ctors will take an `Amode` which
    // is the address to write to.
    for ctor in isle_constructors(&inst.format) {
        let suffix = ctor.suffix();
        let rule_name = format!("x64_{struct_name}{suffix}");
        let result_ty = ctor.result_ty();
        let mut explicit_params = Vec::new();
        let mut implicit_params = Vec::new();
        for param in params.iter() {
            if param.mutability.is_read() || ctor.includes_write_only_reg_mem() {
                explicit_params.push(param);
            } else {
                implicit_params.push(param);
            }
        }
        assert!(implicit_params.len() <= 1);
        let param_tys = explicit_params
            .iter()
            .map(|o| isle_param_for_ctor(o, ctor))
            .chain(trap_type.clone())
            .collect::<Vec<_>>()
            .join(" ");
        let param_names = explicit_params
            .iter()
            .map(|o| o.location.to_string())
            .chain(trap_name.clone())
            .collect::<Vec<_>>()
            .join(" ");
        let convert = ctor.conversion_constructor();

        // Generate implicit parameters to the `*_raw` constructor. Currently
        // this is only destination gpr/xmm temps if the result of this entire
        // constructor is a gpr/xmm register.
        let implicit_params = implicit_params
            .iter()
            .map(|o| {
                assert!(matches!(o.location.kind(), OperandKind::RegMem(_)));
                match ctor {
                    IsleConstructor::RetMemorySideEffect | IsleConstructor::NoReturnSideEffect => {
                        unreachable!()
                    }
                    IsleConstructor::RetGpr | IsleConstructor::ConsumesFlagsReturnsGpr => {
                        "(temp_writable_gpr)"
                    }
                    IsleConstructor::RetXmm => "(temp_writable_xmm)",
                    IsleConstructor::RetValueRegs | IsleConstructor::ProducesFlagsSideEffect => {
                        todo!()
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        fmtln!(f, "(decl {rule_name} ({param_tys}) {result_ty})");
        fmtln!(
            f,
            "(rule ({rule_name} {param_names}) ({convert} ({raw_name} {implicit_params} {param_names})))"
        );

        if let Some(alternate) = &inst.alternate {
            // We currently plan to use alternate instructions for SSE/AVX
            // pairs, so we expect the one of the registers to be an XMM
            // register. In the future we could relax this, but would need to
            // handle more cases below.
            assert!(
                inst.format
                    .operands
                    .iter()
                    .any(|o| matches!(o.location.reg_class(), Some(RegClass::Xmm)))
            );
            let param_tys = if alternate.feature == Feature::avx {
                param_tys.replace("Aligned", "")
            } else {
                param_tys
            };
            let alt_feature = alternate.feature.to_string();
            let alt_name = &alternate.name;
            let rule_name_or_feat = format!("{rule_name}_or_{alt_feature}");
            fmtln!(f, "(decl {rule_name_or_feat} ({param_tys}) {result_ty})");
            fmtln!(f, "(rule 1 ({rule_name_or_feat} {param_names})");
            f.indent(|f| {
                fmtln!(f, "(if-let true (use_{alt_feature}))");
                fmtln!(f, "(x64_{alt_name}{suffix} {param_names}))");
            });
            fmtln!(
                f,
                "(rule 0 ({rule_name_or_feat} {param_names}) ({rule_name} {param_names}))"
            );
        }
    }
}

/// Generate the ISLE definitions that match the `isle_assembler_methods!` macro
/// above.
pub fn generate_isle(f: &mut Formatter, insts: &[Inst]) {
    fmtln!(f, "(type AssemblerOutputs (enum");
    fmtln!(f, "    ;; Used for instructions that have ISLE");
    fmtln!(f, "    ;; `SideEffect`s (memory stores, traps,");
    fmtln!(f, "    ;; etc.) and do not return a `Value`.");
    fmtln!(f, "    (SideEffect (inst MInst))");
    fmtln!(f, "    ;; Used for instructions that return a");
    fmtln!(f, "    ;; GPR (including `GprMem` variants with");
    fmtln!(f, "    ;; a GPR as the first argument).");
    fmtln!(f, "    (RetGpr (inst MInst) (gpr Gpr))");
    fmtln!(f, "    ;; Used for instructions that return an");
    fmtln!(f, "    ;; XMM register.");
    fmtln!(f, "    (RetXmm (inst MInst) (xmm Xmm))");
    fmtln!(f, "    ;; Used for multi-return instructions.");
    fmtln!(f, "    (RetValueRegs (inst MInst) (regs ValueRegs))");
    fmtln!(
        f,
        "    ;; https://github.com/bytecodealliance/wasmtime/pull/10276"
    );
    fmtln!(f, "))");
    f.empty_line();

    fmtln!(f, ";; Directly emit instructions that return a GPR.");
    fmtln!(f, "(decl emit_ret_gpr (AssemblerOutputs) Gpr)");
    fmtln!(f, "(rule (emit_ret_gpr (AssemblerOutputs.RetGpr inst gpr))");
    fmtln!(f, "    (let ((_ Unit (emit inst))) gpr))");
    f.empty_line();

    fmtln!(f, ";; Directly emit instructions that return an");
    fmtln!(f, ";; XMM register.");
    fmtln!(f, "(decl emit_ret_xmm (AssemblerOutputs) Xmm)");
    fmtln!(f, "(rule (emit_ret_xmm (AssemblerOutputs.RetXmm inst xmm))");
    fmtln!(f, "    (let ((_ Unit (emit inst))) xmm))");
    f.empty_line();

    fmtln!(f, ";; Directly emit instructions that return multiple");
    fmtln!(f, ";; registers (e.g. `mul`).");
    fmtln!(f, "(decl emit_ret_value_regs (AssemblerOutputs) ValueRegs)");
    fmtln!(
        f,
        "(rule (emit_ret_value_regs (AssemblerOutputs.RetValueRegs inst regs))"
    );
    fmtln!(f, "    (let ((_ Unit (emit inst))) regs))");
    f.empty_line();

    fmtln!(f, ";; Pass along the side-effecting instruction");
    fmtln!(f, ";; for later emission.");
    fmtln!(
        f,
        "(decl defer_side_effect (AssemblerOutputs) SideEffectNoResult)"
    );
    fmtln!(
        f,
        "(rule (defer_side_effect (AssemblerOutputs.SideEffect inst))"
    );
    fmtln!(f, "    (SideEffectNoResult.Inst inst))");
    f.empty_line();

    for inst in insts {
        if include_inst(inst) {
            generate_isle_inst_decls(f, inst);
            f.empty_line();
        }
    }
}

/// Returns whether `o` is included in the `*_raw` constructor generated in
/// ISLE/Rust.
///
/// This notably includes all operands that are read as those are the
/// data-dependencies of an instruction. This additionally includes, though,
/// write-only `RegMem` operands. In this situation the `RegMem` operand is
/// dynamically a `RegMem::Reg`, a temp register synthesized in ISLE, or a
/// `RegMem::Mem`, an operand from the constructor of the original entrypoint
/// itself.
fn is_raw_operand_param(o: &Operand) -> bool {
    o.mutability.is_read()
        || matches!(
            o.location.kind(),
            OperandKind::RegMem(_) | OperandKind::Mem(_)
        )
}
