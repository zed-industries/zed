use cranelift_srcgen::error::Error;
use std::path::Path;

struct Inst<'a> {
    snake_name: &'a str,
    name: &'a str,
    fields: &'a [(&'a str, &'a str)],
}

macro_rules! define {
    (
        $(
            $( #[$attr:meta] )*
            $snake_name:ident = $name:ident $( { $( $field:ident : $field_ty:ty ),* } )? ;
        )*
    ) => {
        &[$(Inst {
            snake_name: stringify!($snake_name),
            name: stringify!($name),
            fields: &[$($( (stringify!($field), stringify!($field_ty)), )*)?],
        }),*]
        // helpers.push_str(concat!("(define pulley_", stringify!($snake_name), " ("));
    };
}

const OPS: &[Inst<'_>] = pulley_interpreter::for_each_op!(define);
const EXTENDED_OPS: &[Inst<'_>] = pulley_interpreter::for_each_extended_op!(define);

enum Operand<'a> {
    Normal {
        name: &'a str,
        ty: &'a str,
    },
    Writable {
        name: &'a str,
        ty: &'a str,
    },
    TrapCode {
        name: &'a str,
        ty: &'a str,
    },
    Binop {
        dst: &'a str,
        src1: &'a str,
        src2: &'a str,
    },
}

impl Inst<'_> {
    fn operands(&self) -> impl Iterator<Item = Operand<'_>> + use<'_> {
        self.fields
            .iter()
            .map(|(name, ty)| match (*name, *ty) {
                ("operands", binop) => {
                    // Parse "BinaryOperands < A >"` as A/A/A
                    // Parse "BinaryOperands < A, B >"` as A/B/A
                    // Parse "BinaryOperands < A, B, C >"` as A/B/C
                    let mut parts = binop
                        .strip_prefix("BinaryOperands <")
                        .unwrap()
                        .strip_suffix(">")
                        .unwrap()
                        .trim()
                        .split(',')
                        .map(|x| x.trim());
                    let dst = parts.next().unwrap();
                    let src1 = parts.next().unwrap_or(dst);
                    let src2 = parts.next().unwrap_or(dst);
                    Operand::Binop { dst, src1, src2 }
                }
                (name, ty) if name.starts_with("dst") => Operand::Writable { name, ty },
                (name, "UpperRegSet < XReg >") => Operand::Normal {
                    name,
                    ty: "UpperXRegSet",
                },
                (name, ty) => Operand::Normal { name, ty },
            })
            .chain(if self.name.contains("Trap") {
                Some(Operand::TrapCode {
                    name: "code",
                    ty: "TrapCode",
                })
            } else {
                None
            })
    }

    fn skip(&self) -> bool {
        match self.name {
            // Skip instructions related to control-flow as those require
            // special handling with `MachBuffer`.
            "Jump" => true,
            n if n.starts_with("Call") => true,

            // Skip special instructions not used in Cranelift.
            "XPush32Many" | "XPush64Many" | "XPop32Many" | "XPop64Many" => true,

            // Skip more branching-related instructions.
            n => n.starts_with("Br"),
        }
    }
}

pub fn generate_rust(filename: &str, out_dir: &Path) -> Result<(), Error> {
    let mut rust = String::new();

    // Generate a pretty-printing method for debugging.
    rust.push_str("pub fn print(inst: &RawInst) -> String {\n");
    rust.push_str("match inst {\n");
    for inst @ Inst { name, .. } in OPS.iter().chain(EXTENDED_OPS) {
        if inst.skip() {
            continue;
        }

        let mut pat = String::new();
        let mut locals = String::new();
        let mut format_string = String::new();
        format_string.push_str(inst.snake_name);
        for (i, op) in inst.operands().enumerate() {
            match op {
                Operand::Normal { name, ty } | Operand::Writable { name, ty } => {
                    pat.push_str(name);
                    pat.push_str(",");

                    if i > 0 {
                        format_string.push_str(",");
                    }

                    if ty == "UpperXRegSet" {
                        format_string.push_str(" {");
                        format_string.push_str(name);
                        format_string.push_str(":?}");
                        continue;
                    }

                    format_string.push_str(" {");
                    format_string.push_str(name);
                    format_string.push_str("}");
                    if ty.contains("Reg") {
                        if matches!(op, Operand::Writable { .. }) {
                            locals.push_str(&format!("let {name} = reg_name(*{name}.to_reg());\n"));
                        } else {
                            locals.push_str(&format!("let {name} = reg_name(**{name});\n"));
                        }
                    }
                }
                Operand::TrapCode { name, ty: _ } => {
                    pat.push_str(name);
                    pat.push_str(",");
                    format_string.push_str(&format!(" // trap={{{name}:?}}"));
                }
                Operand::Binop { src2, .. } => {
                    pat.push_str("dst, src1, src2,");
                    format_string.push_str(" {dst}, {src1}, {src2}");
                    locals.push_str(&format!("let dst = reg_name(*dst.to_reg());\n"));
                    locals.push_str(&format!("let src1 = reg_name(**src1);\n"));
                    if src2.contains("Reg") {
                        locals.push_str(&format!("let src2 = reg_name(**src2);\n"));
                    }
                }
            }
        }

        rust.push_str(&format!(
            "
        RawInst::{name} {{ {pat} }} => {{
            {locals}
            format!(\"{format_string}\")
        }}
        "
        ));
    }
    rust.push_str("}\n");
    rust.push_str("}\n");

    // Generate `get_operands` to feed information to regalloc
    rust.push_str(
        "pub fn get_operands(inst: &mut RawInst, collector: &mut impl OperandVisitor) {\n",
    );
    rust.push_str("match inst {\n");
    for inst @ Inst { name, .. } in OPS.iter().chain(EXTENDED_OPS) {
        if inst.skip() {
            continue;
        }

        let mut pat = String::new();
        let mut uses = Vec::new();
        let mut defs = Vec::new();
        let mut addrs = Vec::new();
        for op in inst.operands() {
            match op {
                // `{Push,Pop}Frame{Save,Restore}` doesn't participate in
                // register allocation.
                Operand::Normal {
                    name: _,
                    ty: "UpperXRegSet",
                } if *name == "PushFrameSave" || *name == "PopFrameRestore" => {}

                Operand::Normal { name, ty } => {
                    if ty.contains("Reg") {
                        uses.push(name);
                        pat.push_str(name);
                        pat.push_str(",");
                    } else if ty.starts_with("Addr") {
                        addrs.push(name);
                        pat.push_str(name);
                        pat.push_str(",");
                    }
                }
                Operand::Writable { name, ty } => {
                    if ty.contains("Reg") {
                        defs.push(name);
                        pat.push_str(name);
                        pat.push_str(",");
                    }
                }
                Operand::TrapCode { .. } => {}
                Operand::Binop { src2, .. } => {
                    pat.push_str("dst, src1,");
                    uses.push("src1");
                    defs.push("dst");
                    if src2.contains("Reg") {
                        pat.push_str("src2,");
                        uses.push("src2");
                    }
                }
            }
        }

        let uses = uses
            .iter()
            .map(|u| format!("collector.reg_use({u});\n"))
            .collect::<String>();
        let defs = defs
            .iter()
            .map(|u| format!("collector.reg_def({u});\n"))
            .collect::<String>();
        let addrs = addrs
            .iter()
            .map(|u| format!("{u}.collect_operands(collector);\n"))
            .collect::<String>();

        rust.push_str(&format!(
            "
        RawInst::{name} {{ {pat} .. }} => {{
            {uses}
            {defs}
            {addrs}
        }}
        "
        ));
    }
    rust.push_str("}\n");
    rust.push_str("}\n");

    // Generate an emission method
    rust.push_str("pub fn emit<P>(inst: &RawInst, sink: &mut MachBuffer<InstAndKind<P>>)\n");
    rust.push_str("  where P: PulleyTargetKind,\n");
    rust.push_str("{\n");
    rust.push_str("match *inst {\n");
    for inst @ Inst {
        name, snake_name, ..
    } in OPS.iter().chain(EXTENDED_OPS)
    {
        if inst.skip() {
            continue;
        }

        let mut pat = String::new();
        let mut args = String::new();
        let mut trap = String::new();
        for op in inst.operands() {
            match op {
                Operand::Normal { name, ty: _ } | Operand::Writable { name, ty: _ } => {
                    pat.push_str(name);
                    pat.push_str(",");

                    args.push_str(name);
                    args.push_str(",");
                }
                Operand::TrapCode { name, ty: _ } => {
                    pat.push_str(name);
                    pat.push_str(",");
                    trap.push_str(&format!("sink.add_trap({name});\n"));
                }
                Operand::Binop { .. } => {
                    pat.push_str("dst, src1, src2,");
                    args.push_str(
                        "pulley_interpreter::regs::BinaryOperands::new(dst, src1, src2),",
                    );
                }
            }
        }

        rust.push_str(&format!(
            "
        RawInst::{name} {{ {pat} }} => {{
            {trap}
            pulley_interpreter::encode::{snake_name}(sink, {args})
        }}
        "
        ));
    }
    rust.push_str("}\n");
    rust.push_str("}\n");

    std::fs::write(out_dir.join(filename), rust)?;
    Ok(())
}

pub fn generate_isle(filename: &str, out_dir: &Path) -> Result<(), Error> {
    let mut isle = String::new();

    // Generate the `RawInst` enum
    isle.push_str("(type RawInst (enum\n");
    for inst in OPS.iter().chain(EXTENDED_OPS) {
        if inst.skip() {
            continue;
        }
        isle.push_str("  (");
        isle.push_str(inst.name);
        for op in inst.operands() {
            match op {
                Operand::Normal { name, ty } | Operand::TrapCode { name, ty } => {
                    isle.push_str(&format!("\n    ({name} {ty})"));
                }
                Operand::Writable { name, ty } => {
                    isle.push_str(&format!("\n    ({name} Writable{ty})"));
                }
                Operand::Binop { dst, src1, src2 } => {
                    isle.push_str(&format!("\n    (dst Writable{dst})"));
                    isle.push_str(&format!("\n    (src1 {src1})"));
                    isle.push_str(&format!("\n    (src2 {src2})"));
                }
            }
        }
        isle.push_str(")\n");
    }
    isle.push_str("))\n");

    // Generate the `pulley_*` constructors with a `decl` and a `rule`.
    for inst @ Inst {
        name, snake_name, ..
    } in OPS.iter().chain(EXTENDED_OPS)
    {
        if inst.skip() {
            continue;
        }
        // generate `decl` and `rule` at the same time, placing the `rule` in
        // temporary storage on the side. Makes generation a bit easier to read
        // as opposed to doing the decl first then the rule.
        let mut rule = String::new();
        isle.push_str(&format!("(decl pulley_{snake_name} ("));
        rule.push_str(&format!("(rule (pulley_{snake_name} "));
        let mut results = Vec::new();
        let mut ops = Vec::new();
        for op in inst.operands() {
            match op {
                Operand::Normal { name, ty } | Operand::TrapCode { name, ty } => {
                    isle.push_str(ty);
                    rule.push_str(name);
                    ops.push(name);
                }
                Operand::Writable { name: _, ty } => {
                    results.push(ty);
                }
                Operand::Binop { dst, src1, src2 } => {
                    isle.push_str(&format!("{src1} {src2}"));
                    rule.push_str("src1 src2");
                    ops.push("src1");
                    ops.push("src2");
                    results.push(dst);
                }
            }
            isle.push_str(" ");
            rule.push_str(" ");
        }
        isle.push_str(") ");
        rule.push_str(")");
        let ops = ops.join(" ");
        match &results[..] {
            [result] => {
                isle.push_str(result);
                rule.push_str(&format!(
                    "
  (let (
      (dst Writable{result} (temp_writable_{}))
      (_ Unit (emit (RawInst.{name} dst {ops})))
    )
    dst))\
\n",
                    result.to_lowercase()
                ));
            }
            [a, b] => {
                isle.push_str("ValueRegs");
                rule.push_str(&format!(
                    "
  (let (
      (dst1 Writable{a} (temp_writable_{}))
      (dst2 Writable{b} (temp_writable_{}))
      (_ Unit (emit (RawInst.{name} dst1 dst2 {ops})))
    )
    (value_regs dst1 dst2)))\
\n",
                    a.to_lowercase(),
                    b.to_lowercase(),
                ));
            }
            [] => {
                isle.push_str("SideEffectNoResult");
                rule.push_str(&format!(
                    "  (SideEffectNoResult.Inst (RawInst.{name} {ops})))\n",
                ));
            }
            other => panic!("cannot codegen results {other:?}"),
        }
        isle.push_str(")\n");

        isle.push_str(&rule);
    }

    std::fs::write(out_dir.join(filename), isle)?;
    Ok(())
}
