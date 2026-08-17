//! Printer for ISLE language.

use std::{io::Write, vec};

use crate::ast::*;

/// Print ISLE definitions.
pub fn print<W: Write>(defs: &[Def], width: usize, out: &mut W) -> std::io::Result<()> {
    for (i, def) in defs.iter().enumerate() {
        if i > 0 {
            writeln!(out)?;
        }
        print_node(def, width, out)?;
        writeln!(out)?;
    }
    Ok(())
}

/// Print a single ISLE node.
pub fn print_node<N: ToSExpr, W: Write>(
    node: &N,
    width: usize,
    out: &mut W,
) -> std::io::Result<()> {
    let mut printer = Printer::new(out, width);
    let sexpr = node.to_sexpr();
    printer.print(&sexpr)
}

/// S-expression representation of ISLE source code prior to printing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SExpr {
    /// Atom is a plain string to be printed.
    Atom(String),
    /// A binding for an ISLE structure, e.g. `x @ (...)`.
    Binding(String, Box<SExpr>),
    /// A parenthesized list of S-expressions, e.g. `(x y z)`.
    List(Vec<SExpr>),
}

/// Trait for converting ISLE definitions to S-expressions.
pub trait ToSExpr {
    /// Convert the given value to an S-expression.
    fn to_sexpr(&self) -> SExpr;
}

impl SExpr {
    fn atom<S: ToString>(atom: S) -> Self {
        SExpr::Atom(atom.to_string())
    }

    fn list(items: &[impl ToSExpr]) -> Self {
        SExpr::List(items.into_iter().map(|i| i.to_sexpr()).collect())
    }

    fn tagged(tag: &str, items: &[impl ToSExpr]) -> Self {
        let mut parts = vec![SExpr::atom(tag)];
        parts.extend(items.iter().map(ToSExpr::to_sexpr));
        SExpr::List(parts)
    }
}

struct Printer<'a, W: Write> {
    out: &'a mut W,
    col: usize,
    indent: usize,
    width: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Wrapping {
    Wrap,
    SingleLine,
}

impl<'a, W: Write> Printer<'a, W> {
    fn new(out: &'a mut W, width: usize) -> Self {
        Self {
            out,
            col: 0,
            indent: 0,
            width,
        }
    }

    fn print(&mut self, sexpr: &SExpr) -> std::io::Result<()> {
        self.print_wrapped(sexpr, Wrapping::Wrap)
    }

    fn print_wrapped(&mut self, sexpr: &SExpr, wrapping: Wrapping) -> std::io::Result<()> {
        match sexpr {
            SExpr::Atom(atom) => self.put(atom),
            SExpr::Binding(name, sexpr) => {
                self.put(name)?;
                self.put(" @ ")?;
                self.print_wrapped(sexpr, wrapping)
            }
            SExpr::List(items) => {
                if wrapping == Wrapping::SingleLine || self.fits(sexpr) {
                    self.put("(")?;
                    for (i, item) in items.iter().enumerate() {
                        if i > 0 {
                            self.put(" ")?;
                        }
                        self.print_wrapped(item, Wrapping::SingleLine)?;
                    }
                    self.put(")")
                } else {
                    let (first, rest) = items.split_first().expect("non-empty list");
                    self.put("(")?;
                    self.print_wrapped(first, wrapping)?;
                    self.indent += 1;
                    for item in rest {
                        self.nl()?;
                        self.print_wrapped(item, wrapping)?;
                    }
                    self.indent -= 1;
                    self.nl()?;
                    self.put(")")?;
                    Ok(())
                }
            }
        }
    }

    // Would the expressions fit in the current line?
    fn fits(&self, sexpr: &SExpr) -> bool {
        let Some(mut remaining) = self.width.checked_sub(self.col) else {
            return false;
        };
        let mut stack = vec![sexpr];
        while let Some(sexpr) = stack.pop() {
            let consume = match sexpr {
                SExpr::Atom(atom) => atom.len(),
                SExpr::Binding(name, inner) => {
                    stack.push(inner);
                    name.len() + 3 // " @ "
                }
                SExpr::List(items) => {
                    stack.extend(items.iter().rev());
                    2 + items.len() - 1 // "(" + ")" + spaces
                }
            };
            if consume > remaining {
                return false;
            }
            remaining -= consume;
        }
        true
    }

    fn put(&mut self, s: &str) -> std::io::Result<()> {
        write!(self.out, "{s}")?;
        self.col += s.len();
        Ok(())
    }

    fn nl(&mut self) -> std::io::Result<()> {
        writeln!(self.out)?;
        self.col = 0;
        for _ in 0..self.indent {
            write!(self.out, "    ")?;
        }
        Ok(())
    }
}

impl ToSExpr for Def {
    fn to_sexpr(&self) -> SExpr {
        match self {
            Def::Pragma(_) => unimplemented!("pragmas not supported"),
            Def::Type(ty) => ty.to_sexpr(),
            Def::Rule(rule) => rule.to_sexpr(),
            Def::Extractor(extractor) => extractor.to_sexpr(),
            Def::Decl(decl) => decl.to_sexpr(),
            Def::Spec(spec) => spec.to_sexpr(),
            Def::Model(model) => model.to_sexpr(),
            Def::Form(form) => form.to_sexpr(),
            Def::Instantiation(instantiation) => instantiation.to_sexpr(),
            Def::Extern(ext) => ext.to_sexpr(),
            Def::Converter(converter) => converter.to_sexpr(),
        }
    }
}

impl ToSExpr for Type {
    fn to_sexpr(&self) -> SExpr {
        let Type {
            name,
            ty,
            is_extern,
            is_nodebug,
            pos: _,
        } = self;
        let mut parts = vec![SExpr::atom("type"), name.to_sexpr()];
        if *is_extern {
            parts.push(SExpr::atom("extern"));
        }
        if *is_nodebug {
            parts.push(SExpr::Atom("nodebug".to_string()));
        }
        parts.push(ty.to_sexpr());
        SExpr::List(parts)
    }
}

impl ToSExpr for Rule {
    fn to_sexpr(&self) -> SExpr {
        let Rule {
            name,
            prio,
            pattern,
            iflets,
            expr,
            pos: _,
        } = self;
        let mut parts = vec![SExpr::atom("rule")];
        if let Some(name) = name {
            parts.push(name.to_sexpr());
        }
        if let Some(prio) = prio {
            parts.push(SExpr::atom(prio.to_string()));
        }
        parts.push(pattern.to_sexpr());
        parts.extend(iflets.iter().map(ToSExpr::to_sexpr));
        parts.push(expr.to_sexpr());
        SExpr::List(parts)
    }
}

impl ToSExpr for Extractor {
    fn to_sexpr(&self) -> SExpr {
        let Extractor {
            term,
            args,
            template,
            pos: _,
        } = self;
        let mut sig = vec![term.to_sexpr()];
        sig.extend(args.iter().map(ToSExpr::to_sexpr));

        let mut parts = vec![SExpr::atom("extractor")];
        parts.push(SExpr::List(sig));
        parts.push(template.to_sexpr());
        SExpr::List(parts)
    }
}

impl ToSExpr for Decl {
    fn to_sexpr(&self) -> SExpr {
        let Decl {
            term,
            arg_tys,
            ret_ty,
            pure,
            multi,
            partial,
            pos: _,
        } = self;
        let mut parts = vec![SExpr::atom("decl")];
        if *pure {
            parts.push(SExpr::atom("pure"));
        }
        if *multi {
            parts.push(SExpr::atom("multi"));
        }
        if *partial {
            parts.push(SExpr::atom("partial"));
        }
        parts.push(term.to_sexpr());
        parts.push(SExpr::list(arg_tys));
        parts.push(ret_ty.to_sexpr());
        SExpr::List(parts)
    }
}

impl ToSExpr for Spec {
    fn to_sexpr(&self) -> SExpr {
        let Spec {
            term,
            args,
            provides,
            requires,
        } = self;
        let mut sig = vec![term.to_sexpr()];
        sig.extend(args.iter().map(ToSExpr::to_sexpr));

        let mut parts = vec![SExpr::atom("spec")];
        parts.push(SExpr::List(sig));
        if !provides.is_empty() {
            parts.push(SExpr::tagged("provide", &self.provides));
        }
        if !requires.is_empty() {
            parts.push(SExpr::tagged("require", &self.requires));
        }
        SExpr::List(parts)
    }
}

impl ToSExpr for Model {
    fn to_sexpr(&self) -> SExpr {
        let Model { name, val } = self;
        SExpr::List(vec![SExpr::atom("model"), name.to_sexpr(), val.to_sexpr()])
    }
}

impl ToSExpr for Form {
    fn to_sexpr(&self) -> SExpr {
        let Form {
            name,
            signatures,
            pos: _,
        } = self;
        let mut parts = vec![SExpr::atom("form"), name.to_sexpr()];
        parts.extend(signatures.iter().map(ToSExpr::to_sexpr));
        SExpr::List(parts)
    }
}

impl ToSExpr for Instantiation {
    fn to_sexpr(&self) -> SExpr {
        let Instantiation {
            term,
            form,
            signatures,
            pos: _,
        } = self;
        let mut parts = vec![SExpr::atom("instantiate"), term.to_sexpr()];
        if let Some(form) = form {
            parts.push(form.to_sexpr());
        } else {
            parts.extend(signatures.iter().map(ToSExpr::to_sexpr));
        }
        SExpr::List(parts)
    }
}

impl ToSExpr for Extern {
    fn to_sexpr(&self) -> SExpr {
        match self {
            Extern::Extractor {
                term,
                func,
                infallible,
                pos: _,
            } => {
                let mut parts = vec![SExpr::atom("extern"), SExpr::atom("extractor")];
                if *infallible {
                    parts.push(SExpr::atom("infallible"));
                }
                parts.push(term.to_sexpr());
                parts.push(func.to_sexpr());
                SExpr::List(parts)
            }
            Extern::Constructor { term, func, pos: _ } => SExpr::List(vec![
                SExpr::atom("extern"),
                SExpr::atom("constructor"),
                term.to_sexpr(),
                func.to_sexpr(),
            ]),
            Extern::Const { name, ty, pos: _ } => SExpr::List(vec![
                SExpr::atom("extern"),
                SExpr::atom("const"),
                SExpr::atom(format!("${}", name.0)),
                ty.to_sexpr(),
            ]),
        }
    }
}

impl ToSExpr for Converter {
    fn to_sexpr(&self) -> SExpr {
        let Converter {
            inner_ty,
            outer_ty,
            term,
            pos: _,
        } = self;
        SExpr::List(vec![
            SExpr::atom("convert"),
            inner_ty.to_sexpr(),
            outer_ty.to_sexpr(),
            term.to_sexpr(),
        ])
    }
}

impl ToSExpr for TypeValue {
    fn to_sexpr(&self) -> SExpr {
        match self {
            TypeValue::Primitive(name, _) => {
                SExpr::List(vec![SExpr::atom("primitive"), name.to_sexpr()])
            }
            TypeValue::Enum(variants, _) => {
                let mut parts = vec![SExpr::atom("enum")];
                parts.extend(variants.iter().map(ToSExpr::to_sexpr));
                SExpr::List(parts)
            }
        }
    }
}

impl ToSExpr for Variant {
    fn to_sexpr(&self) -> SExpr {
        let Variant {
            name,
            fields,
            pos: _,
        } = self;
        let mut parts = vec![name.to_sexpr()];
        parts.extend(fields.iter().map(ToSExpr::to_sexpr));
        SExpr::List(parts)
    }
}

impl ToSExpr for Field {
    fn to_sexpr(&self) -> SExpr {
        let Field { name, ty, pos: _ } = self;
        SExpr::List(vec![name.to_sexpr(), ty.to_sexpr()])
    }
}

impl ToSExpr for ModelValue {
    fn to_sexpr(&self) -> SExpr {
        match self {
            ModelValue::TypeValue(mt) => SExpr::List(vec![SExpr::atom("type"), mt.to_sexpr()]),
            ModelValue::EnumValues(enumerators) => {
                let mut parts = vec![SExpr::atom("enum")];
                for (variant, value) in enumerators {
                    parts.push(SExpr::List(vec![variant.to_sexpr(), value.to_sexpr()]));
                }
                SExpr::List(parts)
            }
        }
    }
}

impl ToSExpr for ModelType {
    fn to_sexpr(&self) -> SExpr {
        match self {
            ModelType::Unit => SExpr::atom("Unit"),
            ModelType::Int => SExpr::atom("Int"),
            ModelType::Bool => SExpr::atom("Bool"),
            ModelType::BitVec(Some(size)) => {
                SExpr::List(vec![SExpr::atom("bv"), SExpr::atom(size)])
            }
            ModelType::BitVec(None) => SExpr::List(vec![SExpr::atom("bv")]),
        }
    }
}

impl ToSExpr for Signature {
    fn to_sexpr(&self) -> SExpr {
        let Signature {
            args,
            ret,
            canonical,
            pos: _,
        } = self;
        SExpr::List(vec![
            SExpr::tagged("args", args),
            SExpr::tagged("ret", std::slice::from_ref(ret)),
            SExpr::tagged("canon", std::slice::from_ref(canonical)),
        ])
    }
}

impl ToSExpr for SpecExpr {
    fn to_sexpr(&self) -> SExpr {
        match self {
            SpecExpr::ConstInt { val, pos: _ } => SExpr::atom(val),
            SpecExpr::ConstBitVec { val, width, pos: _ } => SExpr::atom(if *width % 4 == 0 {
                format!("#x{val:0width$x}", width = *width as usize / 4)
            } else {
                format!("#b{val:0width$b}", width = *width as usize)
            }),
            SpecExpr::ConstBool { val, pos: _ } => SExpr::atom(if *val { "true" } else { "false" }),
            SpecExpr::ConstUnit { pos: _ } => SExpr::List(Vec::new()),
            SpecExpr::Var { var, pos: _ } => var.to_sexpr(),
            SpecExpr::Op { op, args, pos: _ } => {
                let mut parts = vec![op.to_sexpr()];
                parts.extend(args.iter().map(ToSExpr::to_sexpr));
                SExpr::List(parts)
            }
            SpecExpr::Pair { l, r } => SExpr::List(vec![l.to_sexpr(), r.to_sexpr()]),
            SpecExpr::Enum { name } => SExpr::List(vec![name.to_sexpr()]),
        }
    }
}

impl ToSExpr for SpecOp {
    fn to_sexpr(&self) -> SExpr {
        SExpr::atom(match self {
            SpecOp::Eq => "=",
            SpecOp::And => "and",
            SpecOp::Not => "not",
            SpecOp::Imp => "=>",
            SpecOp::Or => "or",
            SpecOp::Lte => "<=",
            SpecOp::Lt => "<",
            SpecOp::Gte => ">=",
            SpecOp::Gt => ">",
            SpecOp::BVNot => "bvnot",
            SpecOp::BVAnd => "bvand",
            SpecOp::BVOr => "bvor",
            SpecOp::BVXor => "bvxor",
            SpecOp::BVNeg => "bvneg",
            SpecOp::BVAdd => "bvadd",
            SpecOp::BVSub => "bvsub",
            SpecOp::BVMul => "bvmul",
            SpecOp::BVUdiv => "bvudiv",
            SpecOp::BVUrem => "bvurem",
            SpecOp::BVSdiv => "bvsdiv",
            SpecOp::BVSrem => "bvsrem",
            SpecOp::BVShl => "bvshl",
            SpecOp::BVLshr => "bvlshr",
            SpecOp::BVAshr => "bvashr",
            SpecOp::BVSaddo => "bvsaddo",
            SpecOp::BVUle => "bvule",
            SpecOp::BVUlt => "bvult",
            SpecOp::BVUgt => "bvugt",
            SpecOp::BVUge => "bvuge",
            SpecOp::BVSlt => "bvslt",
            SpecOp::BVSle => "bvsle",
            SpecOp::BVSgt => "bvsgt",
            SpecOp::BVSge => "bvsge",
            SpecOp::Rotr => "rotr",
            SpecOp::Rotl => "rotl",
            SpecOp::Extract => "extract",
            SpecOp::ZeroExt => "zero_ext",
            SpecOp::SignExt => "sign_ext",
            SpecOp::Concat => "concat",
            SpecOp::ConvTo => "conv_to",
            SpecOp::Int2BV => "int2bv",
            SpecOp::WidthOf => "widthof",
            SpecOp::If => "if",
            SpecOp::Switch => "switch",
            SpecOp::Popcnt => "popcnt",
            SpecOp::Rev => "rev",
            SpecOp::Cls => "cls",
            SpecOp::Clz => "clz",
            SpecOp::Subs => "subs",
            SpecOp::BV2Int => "bv2int",
            SpecOp::LoadEffect => "load_effect",
            SpecOp::StoreEffect => "store_effect",
        })
    }
}

impl ToSExpr for Pattern {
    fn to_sexpr(&self) -> SExpr {
        match self {
            Pattern::Var {
                var: Ident(var, _),
                pos: _,
            } => SExpr::atom(var.clone()),
            Pattern::BindPattern {
                var: Ident(var, _),
                subpat,
                pos: _,
            } => SExpr::Binding(var.clone(), Box::new(subpat.to_sexpr())),
            Pattern::ConstInt { val, pos: _ } => SExpr::atom(val),
            Pattern::ConstBool { val, pos: _ } => SExpr::atom(if *val { "true" } else { "false" }),
            Pattern::ConstPrim { val, pos: _ } => SExpr::atom(format!("${}", val.0)),
            Pattern::Wildcard { pos: _ } => SExpr::atom("_"),
            Pattern::Term { sym, args, pos: _ } => {
                let mut parts = vec![sym.to_sexpr()];
                parts.extend(args.iter().map(ToSExpr::to_sexpr));
                SExpr::List(parts)
            }
            Pattern::And { subpats, pos: _ } => {
                let mut parts = vec![SExpr::atom("and")];
                parts.extend(subpats.iter().map(ToSExpr::to_sexpr));
                SExpr::List(parts)
            }
            Pattern::MacroArg { .. } => unimplemented!("macro arguments are for internal use only"),
        }
    }
}

impl ToSExpr for IfLet {
    fn to_sexpr(&self) -> SExpr {
        let IfLet {
            pattern,
            expr,
            pos: _,
        } = self;
        SExpr::List(vec![
            SExpr::atom("if-let"),
            pattern.to_sexpr(),
            expr.to_sexpr(),
        ])
    }
}

impl ToSExpr for Expr {
    fn to_sexpr(&self) -> SExpr {
        match self {
            Expr::Term { sym, args, pos: _ } => {
                let mut parts = vec![sym.to_sexpr()];
                parts.extend(args.iter().map(ToSExpr::to_sexpr));
                SExpr::List(parts)
            }
            Expr::Var { name, pos: _ } => name.to_sexpr(),
            Expr::ConstInt { val, pos: _ } => SExpr::atom(val),
            Expr::ConstBool { val, pos: _ } => SExpr::atom(if *val { "true" } else { "false" }),
            Expr::ConstPrim { val, pos: _ } => SExpr::atom(format!("${}", val.0)),
            Expr::Let { defs, body, pos: _ } => {
                let mut parts = vec![SExpr::atom("let")];
                parts.push(SExpr::list(&defs));
                parts.push(body.to_sexpr());
                SExpr::List(parts)
            }
        }
    }
}

impl ToSExpr for LetDef {
    fn to_sexpr(&self) -> SExpr {
        let LetDef {
            var,
            ty,
            val,
            pos: _,
        } = self;
        SExpr::List(vec![var.to_sexpr(), ty.to_sexpr(), val.to_sexpr()])
    }
}

impl ToSExpr for Ident {
    fn to_sexpr(&self) -> SExpr {
        let Ident(name, _) = self;
        SExpr::atom(name.clone())
    }
}
