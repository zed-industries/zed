//! Abstract syntax tree (AST) created from parsed ISLE.

#![expect(missing_docs, reason = "fields mostly self-describing")]

use crate::lexer::Pos;
use crate::log;

/// One toplevel form in an ISLE file.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Def {
    Pragma(Pragma),
    Type(Type),
    Rule(Rule),
    Extractor(Extractor),
    Decl(Decl),
    Spec(Spec),
    Model(Model),
    Form(Form),
    Instantiation(Instantiation),
    Extern(Extern),
    Converter(Converter),
}

/// An identifier -- a variable, term symbol, or type.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident(pub String, pub Pos);

/// Pragmas parsed with the `(pragma <ident>)` syntax.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Pragma {
    // currently, no pragmas are defined, but the infrastructure is useful to keep around
}

/// A declaration of a type.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Type {
    pub name: Ident,
    pub is_extern: bool,
    pub is_nodebug: bool,
    pub ty: TypeValue,
    pub pos: Pos,
}

/// The actual type-value: a primitive or an enum with variants.
///
/// TODO: add structs as well?
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TypeValue {
    Primitive(Ident, Pos),
    Enum(Vec<Variant>, Pos),
}

/// One variant of an enum type.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Variant {
    pub name: Ident,
    pub fields: Vec<Field>,
    pub pos: Pos,
}

/// One field of an enum variant.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Field {
    pub name: Ident,
    pub ty: Ident,
    pub pos: Pos,
}

/// A declaration of a term with its argument and return types.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Decl {
    pub term: Ident,
    pub arg_tys: Vec<Ident>,
    pub ret_ty: Ident,
    /// Whether this term's constructor is pure.
    pub pure: bool,
    /// Whether this term can exist with some multiplicity: an
    /// extractor or a constructor that matches multiple times, or
    /// produces multiple values.
    pub multi: bool,
    /// Whether this term's constructor can fail to match.
    pub partial: bool,
    pub pos: Pos,
}

/// An expression used to specify term semantics, similar to SMT-LIB syntax.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SpecExpr {
    /// An operator that matches a constant integer value.
    ConstInt {
        val: i128,
        pos: Pos,
    },
    /// An operator that matches a constant bitvector value.
    ConstBitVec {
        val: i128,
        width: i8,
        pos: Pos,
    },
    /// An operator that matches a constant boolean value.
    ConstBool {
        val: bool,
        pos: Pos,
    },
    /// The Unit constant value.
    ConstUnit {
        pos: Pos,
    },
    // A variable
    Var {
        var: Ident,
        pos: Pos,
    },
    /// An application of a type variant or term.
    Op {
        op: SpecOp,
        args: Vec<SpecExpr>,
        pos: Pos,
    },
    /// Pairs, currently used for switch statements.
    Pair {
        l: Box<SpecExpr>,
        r: Box<SpecExpr>,
    },
    /// Enums variant values (enums defined by model)
    Enum {
        name: Ident,
    },
}

/// An operation used to specify term semantics, similar to SMT-LIB syntax.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SpecOp {
    // Boolean operations
    Eq,
    And,
    Or,
    Not,
    Imp,

    // Integer comparisons
    Lt,
    Lte,
    Gt,
    Gte,

    // Bitwise bitvector operations (directly SMT-LIB)
    BVNot,
    BVAnd,
    BVOr,
    BVXor,

    // Bitvector arithmetic operations  (directly SMT-LIB)
    BVNeg,
    BVAdd,
    BVSub,
    BVMul,
    BVUdiv,
    BVUrem,
    BVSdiv,
    BVSrem,
    BVShl,
    BVLshr,
    BVAshr,

    // Bitvector comparison operations  (directly SMT-LIB)
    BVUle,
    BVUlt,
    BVUgt,
    BVUge,
    BVSlt,
    BVSle,
    BVSgt,
    BVSge,

    // Bitvector overflow checks (SMT-LIB pending standardization)
    BVSaddo,

    // Desugared bitvector arithmetic operations
    Rotr,
    Rotl,
    Extract,
    ZeroExt,
    SignExt,
    Concat,

    // Custom encodings
    Subs,
    Popcnt,
    Clz,
    Cls,
    Rev,

    // Conversion operations
    ConvTo,
    Int2BV,
    BV2Int,
    WidthOf,

    // Control operations
    If,
    Switch,

    LoadEffect,
    StoreEffect,
}

/// A specification of the semantics of a term.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Spec {
    /// The term name (must match a (decl ...))
    pub term: Ident,
    /// Argument names
    pub args: Vec<Ident>,
    /// Provide statements, which give the semantics of the produces value
    pub provides: Vec<SpecExpr>,
    /// Require statements, which express preconditions on the term
    pub requires: Vec<SpecExpr>,
}

/// A model of an SMT-LIB type.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ModelType {
    /// SMT-LIB Int
    Int,
    /// SMT-LIB Bool
    Bool,
    /// SMT-LIB bitvector, but with a potentially-polymorphic width
    BitVec(Option<usize>),
    /// Unit (removed before conversion to SMT-LIB)
    Unit,
}

/// A construct's value in SMT-LIB
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ModelValue {
    /// Correspond to ISLE types
    TypeValue(ModelType),
    /// Correspond to ISLE enums, identifier is the enum variant name
    EnumValues(Vec<(Ident, SpecExpr)>),
}

/// A model of a construct into SMT-LIB (currently, types or enums)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Model {
    /// The name of the type or enum
    pub name: Ident,
    /// The value of the type or enum (potentially multiple values)
    pub val: ModelValue,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Signature {
    pub args: Vec<ModelType>,
    pub ret: ModelType,
    pub canonical: ModelType,
    pub pos: Pos,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Form {
    pub name: Ident,
    pub signatures: Vec<Signature>,
    pub pos: Pos,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Instantiation {
    pub term: Ident,
    pub form: Option<Ident>,
    pub signatures: Vec<Signature>,
    pub pos: Pos,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Rule {
    pub pattern: Pattern,
    pub iflets: Vec<IfLet>,
    pub expr: Expr,
    pub pos: Pos,
    pub prio: Option<i64>,
    pub name: Option<Ident>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IfLet {
    pub pattern: Pattern,
    pub expr: Expr,
    pub pos: Pos,
}

/// An extractor macro: (A x y) becomes (B x _ y ...). Expanded during
/// ast-to-sema pass.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Extractor {
    pub term: Ident,
    pub args: Vec<Ident>,
    pub template: Pattern,
    pub pos: Pos,
}

/// A pattern: the left-hand side of a rule.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Pattern {
    /// A mention of a variable.
    ///
    /// Equivalent either to a binding (which can be emulated with
    /// `BindPattern` with a `Pattern::Wildcard` subpattern), if this
    /// is the first mention of the variable, in order to capture its
    /// value; or else a match of the already-captured value. This
    /// disambiguation happens when we lower `ast` nodes to `sema`
    /// nodes as we resolve bound variable names.
    Var { var: Ident, pos: Pos },
    /// An operator that binds a variable to a subterm and matches the
    /// subpattern.
    BindPattern {
        var: Ident,
        subpat: Box<Pattern>,
        pos: Pos,
    },
    /// An operator that matches a constant boolean value.
    ConstBool { val: bool, pos: Pos },
    /// An operator that matches a constant integer value.
    ConstInt { val: i128, pos: Pos },
    /// An operator that matches an external constant value.
    ConstPrim { val: Ident, pos: Pos },
    /// An application of a type variant or term.
    Term {
        sym: Ident,
        args: Vec<Pattern>,
        pos: Pos,
    },
    /// An operator that matches anything.
    Wildcard { pos: Pos },
    /// N sub-patterns that must all match.
    And { subpats: Vec<Pattern>, pos: Pos },
    /// Internal use only: macro argument in a template.
    MacroArg { index: usize, pos: Pos },
}

impl Pattern {
    pub fn root_term(&self) -> Option<&Ident> {
        match self {
            &Pattern::Term { ref sym, .. } => Some(sym),
            _ => None,
        }
    }

    /// Call `f` for each of the terms in this pattern.
    pub fn terms(&self, f: &mut dyn FnMut(Pos, &Ident)) {
        match self {
            Pattern::Term { sym, args, pos } => {
                f(*pos, sym);
                for arg in args {
                    arg.terms(f);
                }
            }
            Pattern::And { subpats, .. } => {
                for p in subpats {
                    p.terms(f);
                }
            }
            Pattern::BindPattern { subpat, .. } => {
                subpat.terms(f);
            }
            Pattern::Var { .. }
            | Pattern::ConstBool { .. }
            | Pattern::ConstInt { .. }
            | Pattern::ConstPrim { .. }
            | Pattern::Wildcard { .. }
            | Pattern::MacroArg { .. } => {}
        }
    }

    pub fn make_macro_template(&self, macro_args: &[Ident]) -> Pattern {
        log!("make_macro_template: {:?} with {:?}", self, macro_args);
        match self {
            &Pattern::BindPattern {
                ref var,
                ref subpat,
                pos,
                ..
            } if matches!(&**subpat, &Pattern::Wildcard { .. }) => {
                if let Some(i) = macro_args.iter().position(|arg| arg.0 == var.0) {
                    Pattern::MacroArg { index: i, pos }
                } else {
                    self.clone()
                }
            }
            &Pattern::BindPattern {
                ref var,
                ref subpat,
                pos,
            } => Pattern::BindPattern {
                var: var.clone(),
                subpat: Box::new(subpat.make_macro_template(macro_args)),
                pos,
            },
            &Pattern::Var { ref var, pos } => {
                if let Some(i) = macro_args.iter().position(|arg| arg.0 == var.0) {
                    Pattern::MacroArg { index: i, pos }
                } else {
                    self.clone()
                }
            }
            &Pattern::And { ref subpats, pos } => {
                let subpats = subpats
                    .iter()
                    .map(|subpat| subpat.make_macro_template(macro_args))
                    .collect::<Vec<_>>();
                Pattern::And { subpats, pos }
            }
            &Pattern::Term {
                ref sym,
                ref args,
                pos,
            } => {
                let args = args
                    .iter()
                    .map(|arg| arg.make_macro_template(macro_args))
                    .collect::<Vec<_>>();
                Pattern::Term {
                    sym: sym.clone(),
                    args,
                    pos,
                }
            }

            &Pattern::Wildcard { .. }
            | &Pattern::ConstBool { .. }
            | &Pattern::ConstInt { .. }
            | &Pattern::ConstPrim { .. } => self.clone(),
            &Pattern::MacroArg { .. } => unreachable!(),
        }
    }

    pub fn subst_macro_args(&self, macro_args: &[Pattern]) -> Option<Pattern> {
        log!("subst_macro_args: {:?} with {:?}", self, macro_args);
        match self {
            &Pattern::BindPattern {
                ref var,
                ref subpat,
                pos,
            } => Some(Pattern::BindPattern {
                var: var.clone(),
                subpat: Box::new(subpat.subst_macro_args(macro_args)?),
                pos,
            }),
            &Pattern::And { ref subpats, pos } => {
                let subpats = subpats
                    .iter()
                    .map(|subpat| subpat.subst_macro_args(macro_args))
                    .collect::<Option<Vec<_>>>()?;
                Some(Pattern::And { subpats, pos })
            }
            &Pattern::Term {
                ref sym,
                ref args,
                pos,
            } => {
                let args = args
                    .iter()
                    .map(|arg| arg.subst_macro_args(macro_args))
                    .collect::<Option<Vec<_>>>()?;
                Some(Pattern::Term {
                    sym: sym.clone(),
                    args,
                    pos,
                })
            }

            &Pattern::Var { .. }
            | &Pattern::Wildcard { .. }
            | &Pattern::ConstBool { .. }
            | &Pattern::ConstInt { .. }
            | &Pattern::ConstPrim { .. } => Some(self.clone()),
            &Pattern::MacroArg { index, .. } => macro_args.get(index).cloned(),
        }
    }

    pub fn pos(&self) -> Pos {
        match self {
            &Pattern::ConstBool { pos, .. }
            | &Pattern::ConstInt { pos, .. }
            | &Pattern::ConstPrim { pos, .. }
            | &Pattern::And { pos, .. }
            | &Pattern::Term { pos, .. }
            | &Pattern::BindPattern { pos, .. }
            | &Pattern::Var { pos, .. }
            | &Pattern::Wildcard { pos, .. }
            | &Pattern::MacroArg { pos, .. } => pos,
        }
    }
}

/// An expression: the right-hand side of a rule.
///
/// Note that this *almost* looks like a core Lisp or lambda calculus,
/// except that there is no abstraction (lambda). This first-order
/// limit is what makes it analyzable.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expr {
    /// A term: `(sym args...)`.
    Term {
        sym: Ident,
        args: Vec<Expr>,
        pos: Pos,
    },
    /// A variable use.
    Var { name: Ident, pos: Pos },
    /// A constant boolean.
    ConstBool { val: bool, pos: Pos },
    /// A constant integer.
    ConstInt { val: i128, pos: Pos },
    /// A constant of some other primitive type.
    ConstPrim { val: Ident, pos: Pos },
    /// The `(let ((var ty val)*) body)` form.
    Let {
        defs: Vec<LetDef>,
        body: Box<Expr>,
        pos: Pos,
    },
}

impl Expr {
    pub fn pos(&self) -> Pos {
        match self {
            &Expr::Term { pos, .. }
            | &Expr::Var { pos, .. }
            | &Expr::ConstBool { pos, .. }
            | &Expr::ConstInt { pos, .. }
            | &Expr::ConstPrim { pos, .. }
            | &Expr::Let { pos, .. } => pos,
        }
    }

    /// Call `f` for each of the terms in this expression.
    pub fn terms(&self, f: &mut dyn FnMut(Pos, &Ident)) {
        match self {
            Expr::Term { sym, args, pos } => {
                f(*pos, sym);
                for arg in args {
                    arg.terms(f);
                }
            }
            Expr::Let { defs, body, .. } => {
                for def in defs {
                    def.val.terms(f);
                }
                body.terms(f);
            }
            Expr::Var { .. }
            | Expr::ConstBool { .. }
            | Expr::ConstInt { .. }
            | Expr::ConstPrim { .. } => {}
        }
    }
}

/// One variable locally bound in a `(let ...)` expression.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LetDef {
    pub var: Ident,
    pub ty: Ident,
    pub val: Box<Expr>,
    pub pos: Pos,
}

/// An external binding: an extractor or constructor function attached
/// to a term.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Extern {
    /// An external extractor: `(extractor Term rustfunc)` form.
    Extractor {
        /// The term to which this external extractor is attached.
        term: Ident,
        /// The Rust function name.
        func: Ident,
        /// The position of this decl.
        pos: Pos,
        /// Infallibility: if an external extractor returns `(T1, T2,
        /// ...)` rather than `Option<(T1, T2, ...)>`, and hence can
        /// never fail, it is declared as such and allows for slightly
        /// better code to be generated.
        infallible: bool,
    },
    /// An external constructor: `(constructor Term rustfunc)` form.
    Constructor {
        /// The term to which this external constructor is attached.
        term: Ident,
        /// The Rust function name.
        func: Ident,
        /// The position of this decl.
        pos: Pos,
    },
    /// An external constant: `(const $IDENT type)` form.
    Const { name: Ident, ty: Ident, pos: Pos },
}

/// An implicit converter: the given term, which must have type
/// (inner_ty) -> outer_ty, is used either in extractor or constructor
/// position as appropriate when a type mismatch with the given pair
/// of types would otherwise occur.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Converter {
    /// The term name.
    pub term: Ident,
    /// The "inner type": the type to convert *from*, on the
    /// right-hand side, or *to*, on the left-hand side. Must match
    /// the singular argument type of the term.
    pub inner_ty: Ident,
    /// The "outer type": the type to convert *to*, on the right-hand
    /// side, or *from*, on the left-hand side. Must match the ret_ty
    /// of the term.
    pub outer_ty: Ident,
    /// The position of this converter decl.
    pub pos: Pos,
}
