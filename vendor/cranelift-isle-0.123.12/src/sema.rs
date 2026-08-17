//! Semantic analysis.
//!
//! This module primarily contains the type environment and term environment.
//!
//! The type environment is constructed by analyzing an input AST. The type
//! environment records the types used in the input source and the types of our
//! various rules and symbols. ISLE's type system is intentionally easy to
//! check, only requires a single pass over the AST, and doesn't require any
//! unification or anything like that.
//!
//! The term environment is constructed from both the AST and type
//! environment. It is sort of a typed and reorganized AST that more directly
//! reflects ISLE semantics than the input ISLE source code (where as the AST is
//! the opposite).

use crate::ast;
use crate::error::*;
use crate::lexer::Pos;
use crate::log;
use crate::stablemapset::{StableMap, StableSet};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt;

declare_id!(
    /// The id of an interned symbol.
    Sym
);
declare_id!(
    /// The id of an interned type inside the `TypeEnv`.
    TypeId
);
declare_id!(
    /// The id of a variant inside an enum.
    VariantId
);
declare_id!(
    /// The id of a field inside a variant.
    FieldId
);
declare_id!(
    /// The id of an interned term inside the `TermEnv`.
    TermId
);
declare_id!(
    /// The id of an interned rule inside the `TermEnv`.
    RuleId
);
declare_id!(
    /// The id of a bound variable inside a `Bindings`.
    VarId
);

/// The type environment.
///
/// Keeps track of which symbols and rules have which types.
#[derive(Debug)]
pub struct TypeEnv {
    /// Arena of interned symbol names.
    ///
    /// Referred to indirectly via `Sym` indices.
    pub syms: Vec<String>,

    /// Map of already-interned symbol names to their `Sym` ids.
    pub sym_map: StableMap<String, Sym>,

    /// Arena of type definitions.
    ///
    /// Referred to indirectly via `TypeId`s.
    pub types: Vec<Type>,

    /// A map from a type name symbol to its `TypeId`.
    pub type_map: StableMap<Sym, TypeId>,

    /// The types of constant symbols.
    pub const_types: StableMap<Sym, TypeId>,

    /// Type errors that we've found so far during type checking.
    pub errors: Vec<Error>,
}

/// A built-in type.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BuiltinType {
    /// The type of booleans, with values `true` and `false`.
    Bool,
    /// The types of fixed-width integers.
    Int(IntType),
}

/// A built-in fixed-width integer type.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntType {
    /// Unsigned, 8 bits.
    U8,
    /// Unsigned, 16 bits.
    U16,
    /// Unsigned, 32 bits.
    U32,
    /// Unsigned, 64 bits.
    U64,
    /// Unsigned, 128 bits.
    U128,
    /// Unsigned, enough bits to hold a pointer.
    USize,
    /// Signed, 8 bits.
    I8,
    /// Signed, 16 bits.
    I16,
    /// Signed, 32 bits.
    I32,
    /// Signed, 64 bits.
    I64,
    /// Signed, 128 bits.
    I128,
    /// Unsigned, enough bits to hold a pointer.
    ISize,
}

impl IntType {
    /// Get the integer type's name.
    pub fn name(&self) -> &'static str {
        match self {
            IntType::U8 => "u8",
            IntType::U16 => "u16",
            IntType::U32 => "u32",
            IntType::U64 => "u64",
            IntType::U128 => "u128",
            IntType::USize => "usize",
            IntType::I8 => "i8",
            IntType::I16 => "i16",
            IntType::I32 => "i32",
            IntType::I64 => "i64",
            IntType::I128 => "i128",
            IntType::ISize => "isize",
        }
    }

    /// Is this integer type signed?
    pub fn is_signed(&self) -> bool {
        match self {
            IntType::U8
            | IntType::U16
            | IntType::U32
            | IntType::U64
            | IntType::U128
            | IntType::USize => false,

            IntType::I8
            | IntType::I16
            | IntType::I32
            | IntType::I64
            | IntType::I128
            | IntType::ISize => true,
        }
    }
}

impl fmt::Display for IntType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl BuiltinType {
    /// All the built-in types.
    pub const ALL: &'static [Self] = &[
        Self::Bool,
        Self::Int(IntType::U8),
        Self::Int(IntType::U16),
        Self::Int(IntType::U32),
        Self::Int(IntType::U64),
        Self::Int(IntType::U128),
        Self::Int(IntType::USize),
        Self::Int(IntType::I8),
        Self::Int(IntType::I16),
        Self::Int(IntType::I32),
        Self::Int(IntType::I64),
        Self::Int(IntType::I128),
        Self::Int(IntType::ISize),
    ];

    /// Get the built-in type's name.
    pub fn name(&self) -> &'static str {
        match self {
            BuiltinType::Bool => "bool",
            BuiltinType::Int(it) => it.name(),
        }
    }

    const fn to_usize(&self) -> usize {
        match self {
            Self::Bool => 0,
            Self::Int(ty) => *ty as usize + 1,
        }
    }
}

impl TypeId {
    const fn builtin(builtin: BuiltinType) -> Self {
        Self(builtin.to_usize())
    }

    /// TypeId for `bool`.
    pub const BOOL: Self = Self::builtin(BuiltinType::Bool);

    /// TypeId for `u8`.
    pub const U8: Self = Self::builtin(BuiltinType::Int(IntType::U8));
    /// TypeId for `u16`.
    pub const U16: Self = Self::builtin(BuiltinType::Int(IntType::U16));
    /// TypeId for `u32`.
    pub const U32: Self = Self::builtin(BuiltinType::Int(IntType::U32));
    /// TypeId for `u64`.
    pub const U64: Self = Self::builtin(BuiltinType::Int(IntType::U64));
    /// TypeId for `u128`.
    pub const U128: Self = Self::builtin(BuiltinType::Int(IntType::U128));
    /// TypeId for `usize`.
    pub const USIZE: Self = Self::builtin(BuiltinType::Int(IntType::USize));

    /// TypeId for `i8`.
    pub const I8: Self = Self::builtin(BuiltinType::Int(IntType::I8));
    /// TypeId for `i16`.
    pub const I16: Self = Self::builtin(BuiltinType::Int(IntType::I16));
    /// TypeId for `i32`.
    pub const I32: Self = Self::builtin(BuiltinType::Int(IntType::I32));
    /// TypeId for `i64`.
    pub const I64: Self = Self::builtin(BuiltinType::Int(IntType::I64));
    /// TypeId for `i128`.
    pub const I128: Self = Self::builtin(BuiltinType::Int(IntType::I128));
    /// TypeId for `isize`.
    pub const ISIZE: Self = Self::builtin(BuiltinType::Int(IntType::ISize));
}

/// A type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    /// Built-in types. Always in scope, not defined anywhere in source.
    Builtin(BuiltinType),

    /// A primitive, `Copy` type.
    ///
    /// These are always defined externally, and we allow literals of these
    /// types to pass through from ISLE source code to the emitted Rust code.
    Primitive(TypeId, Sym, Pos),

    /// A sum type.
    ///
    /// Note that enums with only one variant are equivalent to a "struct".
    Enum {
        /// The name of this enum.
        name: Sym,
        /// This `enum`'s type id.
        id: TypeId,
        /// Is this `enum` defined in external Rust code?
        ///
        /// If so, ISLE will not emit a definition for it. If not, then it will
        /// emit a Rust definition for it.
        is_extern: bool,
        /// Whether this type should *not* derive `Debug`.
        ///
        /// Incompatible with `is_extern`.
        is_nodebug: bool,
        /// The different variants for this enum.
        variants: Vec<Variant>,
        /// The ISLE source position where this `enum` is defined.
        pos: Pos,
    },
}

impl Type {
    /// Get the name of this `Type`.
    pub fn name<'a>(&self, tyenv: &'a TypeEnv) -> &'a str {
        match self {
            Self::Builtin(ty) => ty.name(),
            Self::Primitive(_, name, _) | Self::Enum { name, .. } => &tyenv.syms[name.index()],
        }
    }

    /// Get the position where this type was defined.
    pub fn pos(&self) -> Option<Pos> {
        match self {
            Self::Builtin(..) => None,
            Self::Primitive(_, _, pos) | Self::Enum { pos, .. } => Some(*pos),
        }
    }

    /// Is this a primitive type?
    pub fn is_prim(&self) -> bool {
        matches!(self, Type::Primitive(..))
    }

    /// Is this a built-in integer type?
    pub fn is_int(&self) -> bool {
        matches!(self, Self::Builtin(BuiltinType::Int(_)))
    }
}

/// A variant of an enum.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
    /// The name of this variant.
    pub name: Sym,

    /// The full, prefixed-with-the-enum's-name name of this variant.
    ///
    /// E.g. if the enum is `Foo` and this variant is `Bar`, then the
    /// `fullname` is `Foo.Bar`.
    pub fullname: Sym,

    /// The id of this variant, i.e. the index of this variant within its
    /// enum's `Type::Enum::variants`.
    pub id: VariantId,

    /// The data fields of this enum variant.
    pub fields: Vec<Field>,
}

/// A field of a `Variant`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    /// The name of this field.
    pub name: Sym,
    /// This field's id.
    pub id: FieldId,
    /// The type of this field.
    pub ty: TypeId,
}

/// The term environment.
///
/// This is sort of a typed and reorganized AST that more directly reflects ISLE
/// semantics than the input ISLE source code (where as the AST is the
/// opposite).
#[derive(Clone, Debug)]
pub struct TermEnv {
    /// Arena of interned terms defined in this ISLE program.
    ///
    /// This is indexed by `TermId`.
    pub terms: Vec<Term>,

    /// A map from am interned `Term`'s name to its `TermId`.
    pub term_map: StableMap<Sym, TermId>,

    /// Arena of interned rules defined in this ISLE program.
    ///
    /// This is indexed by `RuleId`.
    pub rules: Vec<Rule>,

    /// Map from (inner_ty, outer_ty) pairs to term IDs, giving the
    /// defined implicit type-converter terms we can try to use to fit
    /// types together.
    pub converters: StableMap<(TypeId, TypeId), TermId>,

    /// Flag for whether to expand internal extractors in the
    /// translation from the AST to sema.
    pub expand_internal_extractors: bool,
}

/// A term.
///
/// Maps parameter types to result types if this is a constructor term, or
/// result types to parameter types if this is an extractor term. Or both if
/// this term can be either a constructor or an extractor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Term {
    /// This term's id.
    pub id: TermId,
    /// The source position where this term was declared.
    pub decl_pos: Pos,
    /// The name of this term.
    pub name: Sym,
    /// The parameter types to this term.
    pub arg_tys: Vec<TypeId>,
    /// The result types of this term.
    pub ret_ty: TypeId,
    /// The kind of this term.
    pub kind: TermKind,
}

/// Flags from a term's declaration with `(decl ...)`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TermFlags {
    /// Whether the term is marked as `pure`.
    pub pure: bool,
    /// Whether the term is marked as `multi`.
    pub multi: bool,
    /// Whether the term is marked as `partial`.
    pub partial: bool,
}

impl TermFlags {
    /// Return a new `TermFlags` suitable for a term on the LHS of a rule.
    pub fn on_lhs(mut self) -> Self {
        self.pure = true;
        self.partial = true;
        self
    }
}

/// The kind of a term.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TermKind {
    /// An enum variant constructor or extractor.
    EnumVariant {
        /// Which variant of the enum: e.g. for enum type `A` if a term is
        /// `(A.A1 ...)` then the variant ID corresponds to `A1`.
        variant: VariantId,
    },
    /// A term declared via a `(decl ...)` form.
    Decl {
        /// Flags from the term's declaration.
        flags: TermFlags,
        /// The kind of this term's constructor, if any.
        constructor_kind: Option<ConstructorKind>,
        /// The kind of this term's extractor, if any.
        extractor_kind: Option<ExtractorKind>,
    },
}

/// The kind of a constructor for a term.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstructorKind {
    /// A term with "internal" rules that work in the forward direction. Becomes
    /// a compiled Rust function in the generated code.
    InternalConstructor,
    /// A term defined solely by an external constructor function.
    ExternalConstructor {
        /// The external name of the constructor function.
        name: Sym,
    },
}

/// The kind of an extractor for a term.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExtractorKind {
    /// A term that defines an "extractor macro" in the LHS of a pattern. Its
    /// arguments take patterns and are simply substituted with the given
    /// patterns when used.
    InternalExtractor {
        /// This extractor's pattern.
        template: ast::Pattern,
    },
    /// A term defined solely by an external extractor function.
    ExternalExtractor {
        /// The external name of the extractor function.
        name: Sym,
        /// Is the external extractor infallible?
        infallible: bool,
        /// The position where this external extractor was declared.
        pos: Pos,
    },
}

/// How many values a function can return.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReturnKind {
    /// Exactly one return value.
    Plain,
    /// Zero or one return values.
    Option,
    /// Zero or more return values.
    Iterator,
}

/// An external function signature.
#[derive(Clone, Debug)]
pub struct ExternalSig {
    /// The name of the external function.
    pub func_name: String,
    /// The name of the external function, prefixed with the context trait.
    pub full_name: String,
    /// The types of this function signature's parameters.
    pub param_tys: Vec<TypeId>,
    /// The types of this function signature's results.
    pub ret_tys: Vec<TypeId>,
    /// How many values can this function return?
    pub ret_kind: ReturnKind,
}

impl Term {
    /// Get this term's type.
    pub fn ty(&self) -> TypeId {
        self.ret_ty
    }

    fn check_args_count<T>(&self, args: &[T], tyenv: &mut TypeEnv, pos: Pos, sym: &ast::Ident) {
        if self.arg_tys.len() != args.len() {
            tyenv.report_error(
                pos,
                format!(
                    "Incorrect argument count for term '{}': got {}, expect {}",
                    sym.0,
                    args.len(),
                    self.arg_tys.len()
                ),
            );
        }
    }

    /// Is this term an enum variant?
    pub fn is_enum_variant(&self) -> bool {
        matches!(self.kind, TermKind::EnumVariant { .. })
    }

    /// Is this term partial?
    pub fn is_partial(&self) -> bool {
        matches!(
            self.kind,
            TermKind::Decl {
                flags: TermFlags { partial: true, .. },
                ..
            }
        )
    }

    /// Does this term have a constructor?
    pub fn has_constructor(&self) -> bool {
        matches!(
            self.kind,
            TermKind::EnumVariant { .. }
                | TermKind::Decl {
                    constructor_kind: Some(_),
                    ..
                }
        )
    }

    /// Does this term have an extractor?
    pub fn has_extractor(&self) -> bool {
        matches!(
            self.kind,
            TermKind::EnumVariant { .. }
                | TermKind::Decl {
                    extractor_kind: Some(_),
                    ..
                }
        )
    }

    /// Is this term's extractor external?
    pub fn has_external_extractor(&self) -> bool {
        matches!(
            self.kind,
            TermKind::Decl {
                extractor_kind: Some(ExtractorKind::ExternalExtractor { .. }),
                ..
            }
        )
    }

    /// Is this term's constructor external?
    pub fn has_external_constructor(&self) -> bool {
        matches!(
            self.kind,
            TermKind::Decl {
                constructor_kind: Some(ConstructorKind::ExternalConstructor { .. }),
                ..
            }
        )
    }

    /// Get this term's extractor's external function signature, if any.
    pub fn extractor_sig(&self, tyenv: &TypeEnv) -> Option<ExternalSig> {
        match &self.kind {
            TermKind::Decl {
                flags,
                extractor_kind:
                    Some(ExtractorKind::ExternalExtractor {
                        name, infallible, ..
                    }),
                ..
            } => {
                let ret_kind = if flags.multi {
                    ReturnKind::Iterator
                } else if *infallible {
                    ReturnKind::Plain
                } else {
                    ReturnKind::Option
                };
                Some(ExternalSig {
                    func_name: tyenv.syms[name.index()].clone(),
                    full_name: format!("C::{}", tyenv.syms[name.index()]),
                    param_tys: vec![self.ret_ty],
                    ret_tys: self.arg_tys.clone(),
                    ret_kind,
                })
            }
            _ => None,
        }
    }

    /// Get this term's constructor's external function signature, if any.
    pub fn constructor_sig(&self, tyenv: &TypeEnv) -> Option<ExternalSig> {
        match &self.kind {
            TermKind::Decl {
                constructor_kind: Some(kind),
                flags,
                ..
            } => {
                let (func_name, full_name) = match kind {
                    ConstructorKind::InternalConstructor => {
                        let name = format!("constructor_{}", tyenv.syms[self.name.index()]);
                        (name.clone(), name)
                    }
                    ConstructorKind::ExternalConstructor { name } => (
                        tyenv.syms[name.index()].clone(),
                        format!("C::{}", tyenv.syms[name.index()]),
                    ),
                };
                let ret_kind = if flags.multi {
                    ReturnKind::Iterator
                } else if flags.partial {
                    ReturnKind::Option
                } else {
                    ReturnKind::Plain
                };
                Some(ExternalSig {
                    func_name,
                    full_name,
                    param_tys: self.arg_tys.clone(),
                    ret_tys: vec![self.ret_ty],
                    ret_kind,
                })
            }
            _ => None,
        }
    }
}

/// A term rewrite rule.
#[derive(Clone, Debug)]
pub struct Rule {
    /// This rule's id.
    pub id: RuleId,
    /// The left-hand side pattern that this rule matches.
    pub root_term: TermId,
    /// Patterns to test against the root term's arguments.
    pub args: Vec<Pattern>,
    /// Any subpattern "if-let" clauses.
    pub iflets: Vec<IfLet>,
    /// The right-hand side expression that this rule evaluates upon successful
    /// match.
    pub rhs: Expr,
    /// Variable names used in this rule, indexed by [VarId].
    pub vars: Vec<BoundVar>,
    /// The priority of this rule, defaulted to 0 if it was missing in the source.
    pub prio: i64,
    /// The source position where this rule is defined.
    pub pos: Pos,
    /// The optional name for this rule.
    pub name: Option<Sym>,
}

/// A name bound in a pattern or let-expression.
#[derive(Clone, Debug)]
pub struct BoundVar {
    /// The identifier used for this variable within the scope of the current [Rule].
    pub id: VarId,
    /// The variable's name.
    pub name: Sym,
    /// The type of the value this variable is bound to.
    pub ty: TypeId,
    /// A counter used to check whether this variable is still in scope during
    /// semantic analysis. Not meaningful afterward.
    scope: usize,
}

/// An `if-let` clause with a subpattern match on an expr after the
/// main LHS matches.
#[derive(Clone, Debug)]
pub struct IfLet {
    /// The left-hand side pattern that this `if-let` clause matches
    /// against the expression below.
    pub lhs: Pattern,
    /// The right-hand side expression that this pattern
    /// evaluates. Must be pure.
    pub rhs: Expr,
}

/// A left-hand side pattern of some rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pattern {
    /// Bind a variable of the given type from the current value.
    ///
    /// Keep matching on the value with the subpattern.
    BindPattern(TypeId, VarId, Box<Pattern>),

    /// Match the current value against an already bound variable with the given
    /// type.
    Var(TypeId, VarId),

    /// Match the current value against a constant boolean.
    ConstBool(TypeId, bool),

    /// Match the current value against a constant integer of the given integer
    /// type.
    ConstInt(TypeId, i128),

    /// Match the current value against a constant primitive value of the given
    /// primitive type.
    ConstPrim(TypeId, Sym),

    /// Match the current value against the given extractor term with the given
    /// arguments.
    Term(TypeId, TermId, Vec<Pattern>),

    /// Match anything of the given type successfully.
    Wildcard(TypeId),

    /// Match all of the following patterns of the given type.
    And(TypeId, Vec<Pattern>),
}

/// A right-hand side expression of some rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    /// Invoke this term constructor with the given arguments.
    Term(TypeId, TermId, Vec<Expr>),
    /// Get the value of a variable that was bound in the left-hand side.
    Var(TypeId, VarId),
    /// Get a constant boolean.
    ConstBool(TypeId, bool),
    /// Get a constant integer.
    ConstInt(TypeId, i128),
    /// Get a constant primitive.
    ConstPrim(TypeId, Sym),
    /// Evaluate the nested expressions and bind their results to the given
    /// variables, then evaluate the body expression.
    Let {
        /// The type of the result of this let expression.
        ty: TypeId,
        /// The expressions that are evaluated and bound to the given variables.
        bindings: Vec<(VarId, TypeId, Box<Expr>)>,
        /// The body expression that is evaluated after the bindings.
        body: Box<Expr>,
    },
}

/// Visitor interface for [Pattern]s. Visitors can assign an arbitrary identifier to each
/// subpattern, which is threaded through to subsequent calls into the visitor.
pub trait PatternVisitor {
    /// The type of subpattern identifiers.
    type PatternId: Copy;

    /// Match if `a` and `b` have equal values.
    fn add_match_equal(&mut self, a: Self::PatternId, b: Self::PatternId, ty: TypeId);
    /// Match if `input` is the given boolean constant.
    fn add_match_bool(&mut self, input: Self::PatternId, ty: TypeId, bool_val: bool);
    /// Match if `input` is the given integer constant.
    fn add_match_int(&mut self, input: Self::PatternId, ty: TypeId, int_val: i128);
    /// Match if `input` is the given primitive constant.
    fn add_match_prim(&mut self, input: Self::PatternId, ty: TypeId, val: Sym);

    /// Match if `input` is the given enum variant. Returns an identifier for each field within the
    /// enum variant. The length of the return list must equal the length of `arg_tys`.
    fn add_match_variant(
        &mut self,
        input: Self::PatternId,
        input_ty: TypeId,
        arg_tys: &[TypeId],
        variant: VariantId,
    ) -> Vec<Self::PatternId>;

    /// Match if the given external extractor succeeds on `input`. Returns an identifier for each
    /// return value from the external extractor. The length of the return list must equal the
    /// length of `output_tys`.
    fn add_extract(
        &mut self,
        input: Self::PatternId,
        input_ty: TypeId,
        output_tys: Vec<TypeId>,
        term: TermId,
        infallible: bool,
        multi: bool,
    ) -> Vec<Self::PatternId>;
}

impl Pattern {
    /// Get this pattern's type.
    pub fn ty(&self) -> TypeId {
        match *self {
            Self::BindPattern(t, ..) => t,
            Self::Var(t, ..) => t,
            Self::ConstBool(t, ..) => t,
            Self::ConstInt(t, ..) => t,
            Self::ConstPrim(t, ..) => t,
            Self::Term(t, ..) => t,
            Self::Wildcard(t, ..) => t,
            Self::And(t, ..) => t,
        }
    }

    /// Recursively visit every sub-pattern.
    pub fn visit<V: PatternVisitor>(
        &self,
        visitor: &mut V,
        input: V::PatternId,
        termenv: &TermEnv,
        vars: &mut HashMap<VarId, V::PatternId>,
    ) {
        match *self {
            Pattern::BindPattern(_ty, var, ref subpat) => {
                // Bind the appropriate variable and recurse.
                assert!(!vars.contains_key(&var));
                vars.insert(var, input);
                subpat.visit(visitor, input, termenv, vars);
            }
            Pattern::Var(ty, var) => {
                // Assert that the value matches the existing bound var.
                let var_val = vars
                    .get(&var)
                    .copied()
                    .expect("Variable should already be bound");
                visitor.add_match_equal(input, var_val, ty);
            }
            Pattern::ConstBool(ty, value) => visitor.add_match_bool(input, ty, value),
            Pattern::ConstInt(ty, value) => visitor.add_match_int(input, ty, value),
            Pattern::ConstPrim(ty, value) => visitor.add_match_prim(input, ty, value),
            Pattern::Term(ty, term, ref args) => {
                // Determine whether the term has an external extractor or not.
                let termdata = &termenv.terms[term.index()];
                let arg_values = match &termdata.kind {
                    TermKind::EnumVariant { variant } => {
                        visitor.add_match_variant(input, ty, &termdata.arg_tys, *variant)
                    }
                    TermKind::Decl {
                        extractor_kind: None,
                        ..
                    } => {
                        panic!("Pattern invocation of undefined term body")
                    }
                    TermKind::Decl {
                        extractor_kind: Some(ExtractorKind::InternalExtractor { .. }),
                        ..
                    } => {
                        panic!("Should have been expanded away")
                    }
                    TermKind::Decl {
                        flags,
                        extractor_kind: Some(ExtractorKind::ExternalExtractor { infallible, .. }),
                        ..
                    } => {
                        // Evaluate all `input` args.
                        let output_tys = args.iter().map(|arg| arg.ty()).collect();

                        // Invoke the extractor.
                        visitor.add_extract(
                            input,
                            termdata.ret_ty,
                            output_tys,
                            term,
                            *infallible && !flags.multi,
                            flags.multi,
                        )
                    }
                };
                for (pat, val) in args.iter().zip(arg_values) {
                    pat.visit(visitor, val, termenv, vars);
                }
            }
            Pattern::And(_ty, ref children) => {
                for child in children {
                    child.visit(visitor, input, termenv, vars);
                }
            }
            Pattern::Wildcard(_ty) => {
                // Nothing!
            }
        }
    }
}

/// Visitor interface for [Expr]s. Visitors can return an arbitrary identifier for each
/// subexpression, which is threaded through to subsequent calls into the visitor.
pub trait ExprVisitor {
    /// The type of subexpression identifiers.
    type ExprId: Copy;

    /// Construct a constant boolean.
    fn add_const_bool(&mut self, ty: TypeId, val: bool) -> Self::ExprId;
    /// Construct a constant integer.
    fn add_const_int(&mut self, ty: TypeId, val: i128) -> Self::ExprId;
    /// Construct a primitive constant.
    fn add_const_prim(&mut self, ty: TypeId, val: Sym) -> Self::ExprId;

    /// Construct an enum variant with the given `inputs` assigned to the variant's fields in order.
    fn add_create_variant(
        &mut self,
        inputs: Vec<(Self::ExprId, TypeId)>,
        ty: TypeId,
        variant: VariantId,
    ) -> Self::ExprId;

    /// Call an external constructor with the given `inputs` as arguments.
    fn add_construct(
        &mut self,
        inputs: Vec<(Self::ExprId, TypeId)>,
        ty: TypeId,
        term: TermId,
        pure: bool,
        infallible: bool,
        multi: bool,
    ) -> Self::ExprId;
}

impl Expr {
    /// Get this expression's type.
    pub fn ty(&self) -> TypeId {
        match *self {
            Self::Term(t, ..) => t,
            Self::Var(t, ..) => t,
            Self::ConstBool(t, ..) => t,
            Self::ConstInt(t, ..) => t,
            Self::ConstPrim(t, ..) => t,
            Self::Let { ty: t, .. } => t,
        }
    }

    /// Recursively visit every subexpression.
    pub fn visit<V: ExprVisitor>(
        &self,
        visitor: &mut V,
        termenv: &TermEnv,
        vars: &HashMap<VarId, V::ExprId>,
    ) -> V::ExprId {
        log!("Expr::visit: expr {:?}", self);
        match *self {
            Expr::ConstBool(ty, val) => visitor.add_const_bool(ty, val),
            Expr::ConstInt(ty, val) => visitor.add_const_int(ty, val),
            Expr::ConstPrim(ty, val) => visitor.add_const_prim(ty, val),
            Expr::Let {
                ty: _ty,
                ref bindings,
                ref body,
            } => {
                let mut vars = vars.clone();
                for &(var, _var_ty, ref var_expr) in bindings {
                    let var_value = var_expr.visit(visitor, termenv, &vars);
                    vars.insert(var, var_value);
                }
                body.visit(visitor, termenv, &vars)
            }
            Expr::Var(_ty, var_id) => *vars.get(&var_id).unwrap(),
            Expr::Term(ty, term, ref arg_exprs) => {
                let termdata = &termenv.terms[term.index()];
                let arg_values_tys = arg_exprs
                    .iter()
                    .map(|arg_expr| arg_expr.visit(visitor, termenv, vars))
                    .zip(termdata.arg_tys.iter().copied())
                    .collect();
                match &termdata.kind {
                    TermKind::EnumVariant { variant } => {
                        visitor.add_create_variant(arg_values_tys, ty, *variant)
                    }
                    TermKind::Decl {
                        constructor_kind: Some(_),
                        flags,
                        ..
                    } => {
                        visitor.add_construct(
                            arg_values_tys,
                            ty,
                            term,
                            flags.pure,
                            /* infallible = */ !flags.partial,
                            flags.multi,
                        )
                    }
                    TermKind::Decl {
                        constructor_kind: None,
                        ..
                    } => panic!("Should have been caught by typechecking"),
                }
            }
        }
    }

    fn visit_in_rule<V: RuleVisitor>(
        &self,
        visitor: &mut V,
        termenv: &TermEnv,
        vars: &HashMap<VarId, <V::PatternVisitor as PatternVisitor>::PatternId>,
    ) -> V::Expr {
        let var_exprs = vars
            .iter()
            .map(|(&var, &val)| (var, visitor.pattern_as_expr(val)))
            .collect();
        visitor.add_expr(|visitor| VisitedExpr {
            ty: self.ty(),
            value: self.visit(visitor, termenv, &var_exprs),
        })
    }
}

/// Information about an expression after it has been fully visited in [RuleVisitor::add_expr].
#[derive(Clone, Copy)]
pub struct VisitedExpr<V: ExprVisitor> {
    /// The type of the top-level expression.
    pub ty: TypeId,
    /// The identifier returned by the visitor for the top-level expression.
    pub value: V::ExprId,
}

/// Visitor interface for [Rule]s. Visitors must be able to visit patterns by implementing
/// [PatternVisitor], and to visit expressions by providing a type that implements [ExprVisitor].
pub trait RuleVisitor {
    /// The type of pattern visitors constructed by [RuleVisitor::add_pattern].
    type PatternVisitor: PatternVisitor;
    /// The type of expression visitors constructed by [RuleVisitor::add_expr].
    type ExprVisitor: ExprVisitor;
    /// The type returned from [RuleVisitor::add_expr], which may be exchanged for a subpattern
    /// identifier using [RuleVisitor::expr_as_pattern].
    type Expr;

    /// Visit one of the arguments to the top-level pattern.
    fn add_arg(
        &mut self,
        index: usize,
        ty: TypeId,
    ) -> <Self::PatternVisitor as PatternVisitor>::PatternId;

    /// Visit a pattern, used once for the rule's left-hand side and once for each if-let. You can
    /// determine which part of the rule the pattern comes from based on whether the `PatternId`
    /// passed to the first call to this visitor came from `add_arg` or `expr_as_pattern`.
    fn add_pattern<F>(&mut self, visitor: F)
    where
        F: FnOnce(&mut Self::PatternVisitor);

    /// Visit an expression, used once for each if-let and once for the rule's right-hand side.
    fn add_expr<F>(&mut self, visitor: F) -> Self::Expr
    where
        F: FnOnce(&mut Self::ExprVisitor) -> VisitedExpr<Self::ExprVisitor>;

    /// Given an expression from [RuleVisitor::add_expr], return an identifier that can be used with
    /// a pattern visitor in [RuleVisitor::add_pattern].
    fn expr_as_pattern(
        &mut self,
        expr: Self::Expr,
    ) -> <Self::PatternVisitor as PatternVisitor>::PatternId;

    /// Given an identifier from the pattern visitor, return an identifier that can be used with
    /// the expression visitor.
    fn pattern_as_expr(
        &mut self,
        pattern: <Self::PatternVisitor as PatternVisitor>::PatternId,
    ) -> <Self::ExprVisitor as ExprVisitor>::ExprId;
}

impl Rule {
    /// Recursively visit every pattern and expression in this rule. Returns the [RuleVisitor::Expr]
    /// that was returned from [RuleVisitor::add_expr] when that function was called on the rule's
    /// right-hand side.
    pub fn visit<V: RuleVisitor>(&self, visitor: &mut V, termenv: &TermEnv) -> V::Expr {
        let mut vars = HashMap::new();

        // Visit the pattern, starting from the root input value.
        let termdata = &termenv.terms[self.root_term.index()];
        for (i, (subpat, &arg_ty)) in self.args.iter().zip(termdata.arg_tys.iter()).enumerate() {
            let value = visitor.add_arg(i, arg_ty);
            visitor.add_pattern(|visitor| subpat.visit(visitor, value, termenv, &mut vars));
        }

        // Visit the `if-let` clauses, using `V::ExprVisitor` for the sub-exprs (right-hand sides).
        for iflet in self.iflets.iter() {
            let subexpr = iflet.rhs.visit_in_rule(visitor, termenv, &vars);
            let value = visitor.expr_as_pattern(subexpr);
            visitor.add_pattern(|visitor| iflet.lhs.visit(visitor, value, termenv, &mut vars));
        }

        // Visit the rule's right-hand side, making use of the bound variables from the pattern.
        self.rhs.visit_in_rule(visitor, termenv, &vars)
    }
}

/// Given an `Option<T>`, unwrap the inner `T` value, or `continue` if it is
/// `None`.
///
/// Useful for when we encountered an error earlier in our analysis but kept
/// going to find more errors, and now we've run into some missing data that
/// would have been filled in if we didn't hit that original error, but we want
/// to keep going to find more errors.
macro_rules! unwrap_or_continue {
    ($e:expr) => {
        match $e {
            Some(x) => x,
            None => continue,
        }
    };
}

impl Default for TypeEnv {
    fn default() -> Self {
        Self {
            syms: BuiltinType::ALL
                .iter()
                .map(|bt| String::from(bt.name()))
                .collect(),
            sym_map: BuiltinType::ALL
                .iter()
                .enumerate()
                .map(|(idx, bt)| (String::from(bt.name()), Sym(idx)))
                .collect(),
            types: BuiltinType::ALL
                .iter()
                .map(|bt| Type::Builtin(*bt))
                .collect(),
            type_map: BuiltinType::ALL
                .iter()
                .enumerate()
                .map(|(idx, _)| (Sym(idx), TypeId(idx)))
                .collect(),
            const_types: StableMap::new(),
            errors: vec![],
        }
    }
}

impl TypeEnv {
    /// Construct the type environment from the AST.
    pub fn from_ast(defs: &[ast::Def]) -> Result<TypeEnv, Vec<Error>> {
        let mut tyenv = TypeEnv::default();

        // Traverse defs, assigning type IDs to type names. We'll fill
        // in types on a second pass.
        for def in defs {
            match def {
                &ast::Def::Type(ref td) => {
                    let tid = TypeId(tyenv.type_map.len());
                    let name = tyenv.intern_mut(&td.name);

                    if let Some(existing) = tyenv.type_map.get(&name).copied() {
                        tyenv.report_error(
                            td.pos,
                            format!("Type with name '{}' defined more than once", td.name.0),
                        );
                        let pos = unwrap_or_continue!(tyenv.types.get(existing.index())).pos();
                        match pos {
                            Some(pos) => tyenv.report_error(
                                pos,
                                format!("Type with name '{}' already defined here", td.name.0),
                            ),
                            None => tyenv.report_error(
                                td.pos,
                                format!("Type with name '{}' is a built-in type", td.name.0),
                            ),
                        }
                        continue;
                    }

                    tyenv.type_map.insert(name, tid);
                }
                _ => {}
            }
        }

        // Now lower AST nodes to type definitions, raising errors
        // where typenames of fields are undefined or field names are
        // duplicated.
        for def in defs {
            match def {
                &ast::Def::Type(ref td) => {
                    let tid = tyenv.types.len();
                    if let Some(ty) = tyenv.type_from_ast(TypeId(tid), td) {
                        tyenv.types.push(ty);
                    }
                }
                _ => {}
            }
        }

        // Now collect types for extern constants.
        for def in defs {
            if let &ast::Def::Extern(ast::Extern::Const {
                ref name,
                ref ty,
                pos,
            }) = def
            {
                let ty = match tyenv.get_type_by_name(ty) {
                    Some(ty) => ty,
                    None => {
                        tyenv.report_error(pos, "Unknown type for constant");
                        continue;
                    }
                };
                let name = tyenv.intern_mut(name);
                tyenv.const_types.insert(name, ty);
            }
        }

        tyenv.return_errors()?;

        Ok(tyenv)
    }

    fn return_errors(&mut self) -> Result<(), Vec<Error>> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    fn type_from_ast(&mut self, tid: TypeId, ty: &ast::Type) -> Option<Type> {
        let name = self.intern(&ty.name).unwrap();
        match &ty.ty {
            &ast::TypeValue::Primitive(ref id, ..) => {
                if ty.is_nodebug {
                    self.report_error(ty.pos, "primitive types cannot be marked `nodebug`");
                    return None;
                }
                if ty.is_extern {
                    self.report_error(ty.pos, "primitive types cannot be marked `extern`");
                    return None;
                }
                Some(Type::Primitive(tid, self.intern_mut(id), ty.pos))
            }
            &ast::TypeValue::Enum(ref ty_variants, ..) => {
                if ty.is_extern && ty.is_nodebug {
                    self.report_error(ty.pos, "external types cannot be marked `nodebug`");
                    return None;
                }

                let mut variants = vec![];
                for variant in ty_variants {
                    let combined_ident =
                        ast::Ident(format!("{}.{}", ty.name.0, variant.name.0), variant.name.1);
                    let fullname = self.intern_mut(&combined_ident);
                    let name = self.intern_mut(&variant.name);
                    let id = VariantId(variants.len());
                    if variants.iter().any(|v: &Variant| v.name == name) {
                        self.report_error(
                            variant.pos,
                            format!("Duplicate variant name in type: '{}'", variant.name.0),
                        );
                        return None;
                    }
                    let mut fields = vec![];
                    for field in &variant.fields {
                        let field_name = self.intern_mut(&field.name);
                        if fields.iter().any(|f: &Field| f.name == field_name) {
                            self.report_error(
                                field.pos,
                                format!(
                                    "Duplicate field name '{}' in variant '{}' of type",
                                    field.name.0, variant.name.0
                                ),
                            );
                            return None;
                        }
                        let field_tid = match self.get_type_by_name(&field.ty) {
                            Some(tid) => tid,
                            None => {
                                self.report_error(
                                    field.ty.1,
                                    format!(
                                        "Unknown type '{}' for field '{}' in variant '{}'",
                                        field.ty.0, field.name.0, variant.name.0
                                    ),
                                );
                                return None;
                            }
                        };
                        fields.push(Field {
                            name: field_name,
                            id: FieldId(fields.len()),
                            ty: field_tid,
                        });
                    }
                    variants.push(Variant {
                        name,
                        fullname,
                        id,
                        fields,
                    });
                }
                Some(Type::Enum {
                    name,
                    id: tid,
                    is_extern: ty.is_extern,
                    is_nodebug: ty.is_nodebug,
                    variants,
                    pos: ty.pos,
                })
            }
        }
    }

    fn error(&self, pos: Pos, msg: impl Into<String>) -> Error {
        Error::TypeError {
            msg: msg.into(),
            span: Span::new_single(pos),
        }
    }

    fn report_error(&mut self, pos: Pos, msg: impl Into<String>) {
        let err = self.error(pos, msg);
        self.errors.push(err);
    }

    fn intern_mut(&mut self, ident: &ast::Ident) -> Sym {
        if let Some(s) = self.sym_map.get(&ident.0).copied() {
            s
        } else {
            let s = Sym(self.syms.len());
            self.syms.push(ident.0.clone());
            self.sym_map.insert(ident.0.clone(), s);
            s
        }
    }

    fn intern(&self, ident: &ast::Ident) -> Option<Sym> {
        self.sym_map.get(&ident.0).copied()
    }

    /// Lookup type by name.
    pub fn get_type_by_name(&self, sym: &ast::Ident) -> Option<TypeId> {
        self.intern(sym)
            .and_then(|sym| self.type_map.get(&sym))
            .copied()
    }
}

#[derive(Clone, Debug, Default)]
struct Bindings {
    /// All bindings accumulated so far within the current rule, including let-
    /// bindings which have gone out of scope.
    seen: Vec<BoundVar>,
    /// Counter for unique scope IDs within this set of bindings.
    next_scope: usize,
    /// Stack of the scope IDs for bindings which are currently in scope.
    in_scope: Vec<usize>,
}

impl Bindings {
    fn enter_scope(&mut self) {
        self.in_scope.push(self.next_scope);
        self.next_scope += 1;
    }

    fn exit_scope(&mut self) {
        self.in_scope.pop();
    }

    fn add_var(&mut self, name: Sym, ty: TypeId) -> VarId {
        let id = VarId(self.seen.len());
        let var = BoundVar {
            id,
            name,
            ty,
            scope: *self
                .in_scope
                .last()
                .expect("enter_scope should be called before add_var"),
        };
        log!("binding var {:?}", var);
        self.seen.push(var);
        id
    }

    fn lookup(&self, name: Sym) -> Option<&BoundVar> {
        self.seen
            .iter()
            .rev()
            .find(|binding| binding.name == name && self.in_scope.contains(&binding.scope))
    }
}

impl TermEnv {
    /// Construct the term environment from the AST and the type environment.
    pub fn from_ast(
        tyenv: &mut TypeEnv,
        defs: &[ast::Def],
        expand_internal_extractors: bool,
    ) -> Result<TermEnv, Vec<Error>> {
        let mut env = TermEnv {
            terms: vec![],
            term_map: StableMap::new(),
            rules: vec![],
            converters: StableMap::new(),
            expand_internal_extractors,
        };

        env.collect_pragmas(defs);
        env.collect_term_sigs(tyenv, defs);
        env.collect_enum_variant_terms(tyenv);
        tyenv.return_errors()?;
        env.collect_constructors(tyenv, defs);
        env.collect_extractor_templates(tyenv, defs);
        tyenv.return_errors()?;
        env.collect_converters(tyenv, defs);
        tyenv.return_errors()?;
        env.collect_externs(tyenv, defs);
        tyenv.return_errors()?;
        env.collect_rules(tyenv, defs);
        env.check_for_undefined_decls(tyenv, defs);
        env.check_for_expr_terms_without_constructors(tyenv, defs);
        tyenv.return_errors()?;

        Ok(env)
    }

    fn collect_pragmas(&mut self, _: &[ast::Def]) {
        // currently, no pragmas are defined, but the infrastructure is useful to keep around
        return;
    }

    fn collect_term_sigs(&mut self, tyenv: &mut TypeEnv, defs: &[ast::Def]) {
        for def in defs {
            match def {
                &ast::Def::Decl(ref decl) => {
                    let name = tyenv.intern_mut(&decl.term);
                    if let Some(tid) = self.term_map.get(&name) {
                        tyenv.report_error(
                            decl.pos,
                            format!("Duplicate decl for '{}'", decl.term.0),
                        );
                        tyenv.report_error(
                            self.terms[tid.index()].decl_pos,
                            format!("Duplicate decl for '{}'", decl.term.0),
                        );
                    }

                    if decl.multi && decl.partial {
                        tyenv.report_error(
                            decl.pos,
                            format!("Term '{}' can't be both multi and partial", decl.term.0),
                        );
                    }

                    let arg_tys = decl
                        .arg_tys
                        .iter()
                        .map(|id| {
                            tyenv.get_type_by_name(id).ok_or_else(|| {
                                tyenv.report_error(id.1, format!("Unknown arg type: '{}'", id.0));
                            })
                        })
                        .collect::<Result<Vec<_>, _>>();
                    let arg_tys = match arg_tys {
                        Ok(a) => a,
                        Err(_) => {
                            continue;
                        }
                    };
                    let ret_ty = match tyenv.get_type_by_name(&decl.ret_ty) {
                        Some(t) => t,
                        None => {
                            tyenv.report_error(
                                decl.ret_ty.1,
                                format!("Unknown return type: '{}'", decl.ret_ty.0),
                            );
                            continue;
                        }
                    };

                    let tid = TermId(self.terms.len());
                    self.term_map.insert(name, tid);
                    let flags = TermFlags {
                        pure: decl.pure,
                        multi: decl.multi,
                        partial: decl.partial,
                    };
                    self.terms.push(Term {
                        id: tid,
                        decl_pos: decl.pos,
                        name,
                        arg_tys,
                        ret_ty,
                        kind: TermKind::Decl {
                            flags,
                            constructor_kind: None,
                            extractor_kind: None,
                        },
                    });
                }
                _ => {}
            }
        }
    }

    fn collect_enum_variant_terms(&mut self, tyenv: &mut TypeEnv) {
        'types: for i in 0..tyenv.types.len() {
            let ty = &tyenv.types[i];
            match ty {
                &Type::Enum {
                    pos,
                    id,
                    ref variants,
                    ..
                } => {
                    for variant in variants {
                        if self.term_map.contains_key(&variant.fullname) {
                            let variant_name = tyenv.syms[variant.fullname.index()].clone();
                            tyenv.report_error(
                                pos,
                                format!("Duplicate enum variant constructor: '{variant_name}'",),
                            );
                            continue 'types;
                        }
                        let tid = TermId(self.terms.len());
                        let arg_tys = variant.fields.iter().map(|fld| fld.ty).collect::<Vec<_>>();
                        let ret_ty = id;
                        self.terms.push(Term {
                            id: tid,
                            decl_pos: pos,
                            name: variant.fullname,
                            arg_tys,
                            ret_ty,
                            kind: TermKind::EnumVariant {
                                variant: variant.id,
                            },
                        });
                        self.term_map.insert(variant.fullname, tid);
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_constructors(&mut self, tyenv: &mut TypeEnv, defs: &[ast::Def]) {
        for def in defs {
            log!("collect_constructors from def: {:?}", def);
            match def {
                &ast::Def::Rule(ref rule) => {
                    let pos = rule.pos;
                    let term = match rule.pattern.root_term() {
                        Some(t) => t,
                        None => {
                            tyenv.report_error(
                                pos,
                                "Rule does not have a term at the LHS root".to_string(),
                            );
                            continue;
                        }
                    };
                    let term = match self.get_term_by_name(tyenv, &term) {
                        Some(tid) => tid,
                        None => {
                            tyenv
                                .report_error(pos, "Rule LHS root term is not defined".to_string());
                            continue;
                        }
                    };
                    let termdata = &mut self.terms[term.index()];
                    match &mut termdata.kind {
                        TermKind::Decl {
                            constructor_kind, ..
                        } => {
                            match constructor_kind {
                                None => {
                                    *constructor_kind = Some(ConstructorKind::InternalConstructor);
                                }
                                Some(ConstructorKind::InternalConstructor) => {
                                    // OK, no error; multiple rules can apply to
                                    // one internal constructor term.
                                }
                                Some(ConstructorKind::ExternalConstructor { .. }) => {
                                    tyenv.report_error(
                                        pos,
                                        "Rule LHS root term is incorrect kind; cannot \
                                         be external constructor"
                                            .to_string(),
                                    );
                                    continue;
                                }
                            }
                        }
                        TermKind::EnumVariant { .. } => {
                            tyenv.report_error(
                                pos,
                                "Rule LHS root term is incorrect kind; cannot be enum variant"
                                    .to_string(),
                            );
                            continue;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_extractor_templates(&mut self, tyenv: &mut TypeEnv, defs: &[ast::Def]) {
        let mut extractor_call_graph = BTreeMap::new();

        for def in defs {
            if let &ast::Def::Extractor(ref ext) = def {
                let term = match self.get_term_by_name(tyenv, &ext.term) {
                    Some(x) => x,
                    None => {
                        tyenv.report_error(
                            ext.pos,
                            "Extractor macro body definition on a non-existent term".to_string(),
                        );
                        return;
                    }
                };

                let template = ext.template.make_macro_template(&ext.args[..]);
                log!("extractor def: {:?} becomes template {:?}", def, template);

                let mut callees = BTreeSet::new();
                template.terms(&mut |pos, t| {
                    if let Some(term) = self.get_term_by_name(tyenv, t) {
                        callees.insert(term);
                    } else {
                        tyenv.report_error(
                            pos,
                            format!(
                                "`{}` extractor definition references unknown term `{}`",
                                ext.term.0, t.0
                            ),
                        );
                    }
                });
                extractor_call_graph.insert(term, callees);

                let termdata = &mut self.terms[term.index()];
                match &mut termdata.kind {
                    TermKind::EnumVariant { .. } => {
                        tyenv.report_error(
                            ext.pos,
                            "Extractor macro body defined on term of incorrect kind; cannot be an \
                             enum variant",
                        );
                        continue;
                    }
                    TermKind::Decl {
                        flags,
                        extractor_kind,
                        ..
                    } => match extractor_kind {
                        None => {
                            if flags.multi {
                                tyenv.report_error(
                                    ext.pos,
                                    "A term declared with `multi` cannot have an internal extractor.".to_string());
                                continue;
                            }
                            *extractor_kind = Some(ExtractorKind::InternalExtractor { template });
                        }
                        Some(ext_kind) => {
                            tyenv.report_error(
                                ext.pos,
                                "Duplicate extractor definition".to_string(),
                            );
                            let pos = match ext_kind {
                                ExtractorKind::InternalExtractor { template } => template.pos(),
                                ExtractorKind::ExternalExtractor { pos, .. } => *pos,
                            };
                            tyenv.report_error(
                                pos,
                                "Extractor was already defined here".to_string(),
                            );
                            continue;
                        }
                    },
                }
            }
        }

        // Check for cycles in the extractor call graph.
        let mut stack = vec![];
        'outer: for root in extractor_call_graph.keys().copied() {
            stack.clear();
            stack.push((root, vec![root], StableSet::new()));

            while let Some((caller, path, mut seen)) = stack.pop() {
                let is_new = seen.insert(caller);
                if is_new {
                    if let Some(callees) = extractor_call_graph.get(&caller) {
                        stack.extend(callees.iter().map(|callee| {
                            let mut path = path.clone();
                            path.push(*callee);
                            (*callee, path, seen.clone())
                        }));
                    }
                } else {
                    let pos = match &self.terms[caller.index()].kind {
                        TermKind::Decl {
                            extractor_kind: Some(ExtractorKind::InternalExtractor { template }),
                            ..
                        } => template.pos(),
                        _ => {
                            // There must have already been errors recorded.
                            assert!(!tyenv.errors.is_empty());
                            continue 'outer;
                        }
                    };

                    let path: Vec<_> = path
                        .iter()
                        .map(|sym| tyenv.syms[sym.index()].as_str())
                        .collect();
                    let msg = format!(
                        "`{}` extractor definition is recursive: {}",
                        tyenv.syms[root.index()],
                        path.join(" -> ")
                    );
                    tyenv.report_error(pos, msg);
                    continue 'outer;
                }
            }
        }
    }

    fn collect_converters(&mut self, tyenv: &mut TypeEnv, defs: &[ast::Def]) {
        for def in defs {
            match def {
                &ast::Def::Converter(ast::Converter {
                    ref term,
                    ref inner_ty,
                    ref outer_ty,
                    pos,
                }) => {
                    let inner_ty_id = match tyenv.get_type_by_name(inner_ty) {
                        Some(ty) => ty,
                        None => {
                            tyenv.report_error(
                                inner_ty.1,
                                format!("Unknown inner type for converter: '{}'", inner_ty.0),
                            );
                            continue;
                        }
                    };

                    let outer_ty_id = match tyenv.get_type_by_name(outer_ty) {
                        Some(ty) => ty,
                        None => {
                            tyenv.report_error(
                                outer_ty.1,
                                format!("Unknown outer type for converter: '{}'", outer_ty.0),
                            );
                            continue;
                        }
                    };

                    let term_id = match self.get_term_by_name(tyenv, term) {
                        Some(term_id) => term_id,
                        None => {
                            tyenv.report_error(
                                term.1,
                                format!("Unknown term for converter: '{}'", term.0),
                            );
                            continue;
                        }
                    };

                    match self.converters.entry((inner_ty_id, outer_ty_id)) {
                        Entry::Vacant(v) => {
                            v.insert(term_id);
                        }
                        Entry::Occupied(_) => {
                            tyenv.report_error(
                                pos,
                                format!(
                                    "Converter already exists for this type pair: '{}', '{}'",
                                    inner_ty.0, outer_ty.0
                                ),
                            );
                            continue;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_externs(&mut self, tyenv: &mut TypeEnv, defs: &[ast::Def]) {
        for def in defs {
            match def {
                &ast::Def::Extern(ast::Extern::Constructor {
                    ref term,
                    ref func,
                    pos,
                }) => {
                    let func_sym = tyenv.intern_mut(func);
                    let term_id = match self.get_term_by_name(tyenv, term) {
                        Some(term) => term,
                        None => {
                            tyenv.report_error(
                                pos,
                                format!("Constructor declared on undefined term '{}'", term.0),
                            );
                            continue;
                        }
                    };
                    let termdata = &mut self.terms[term_id.index()];
                    match &mut termdata.kind {
                        TermKind::Decl {
                            constructor_kind, ..
                        } => match constructor_kind {
                            None => {
                                *constructor_kind =
                                    Some(ConstructorKind::ExternalConstructor { name: func_sym });
                            }
                            Some(ConstructorKind::InternalConstructor) => {
                                tyenv.report_error(
                                    pos,
                                    format!(
                                        "External constructor declared on term that already has rules: {}",
                                        term.0,
                                    ),
                                );
                            }
                            Some(ConstructorKind::ExternalConstructor { .. }) => {
                                tyenv.report_error(
                                    pos,
                                    "Duplicate external constructor definition".to_string(),
                                );
                            }
                        },
                        TermKind::EnumVariant { .. } => {
                            tyenv.report_error(
                                pos,
                                format!(
                                    "External constructor cannot be defined on enum variant: {}",
                                    term.0,
                                ),
                            );
                        }
                    }
                }
                &ast::Def::Extern(ast::Extern::Extractor {
                    ref term,
                    ref func,
                    pos,
                    infallible,
                }) => {
                    let func_sym = tyenv.intern_mut(func);
                    let term_id = match self.get_term_by_name(tyenv, term) {
                        Some(term) => term,
                        None => {
                            tyenv.report_error(
                                pos,
                                format!("Extractor declared on undefined term '{}'", term.0),
                            );
                            continue;
                        }
                    };

                    let termdata = &mut self.terms[term_id.index()];

                    match &mut termdata.kind {
                        TermKind::Decl { extractor_kind, .. } => match extractor_kind {
                            None => {
                                *extractor_kind = Some(ExtractorKind::ExternalExtractor {
                                    name: func_sym,
                                    infallible,
                                    pos,
                                });
                            }
                            Some(ExtractorKind::ExternalExtractor { pos: pos2, .. }) => {
                                tyenv.report_error(
                                    pos,
                                    "Duplicate external extractor definition".to_string(),
                                );
                                tyenv.report_error(
                                    *pos2,
                                    "External extractor already defined".to_string(),
                                );
                                continue;
                            }
                            Some(ExtractorKind::InternalExtractor { template }) => {
                                tyenv.report_error(
                                    pos,
                                    "Cannot define external extractor for term that already has an \
                                     internal extractor macro body defined"
                                        .to_string(),
                                );
                                tyenv.report_error(
                                    template.pos(),
                                    "Internal extractor macro body already defined".to_string(),
                                );
                                continue;
                            }
                        },
                        TermKind::EnumVariant { .. } => {
                            tyenv.report_error(
                                pos,
                                format!("Cannot define extractor for enum variant '{}'", term.0),
                            );
                            continue;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_rules(&mut self, tyenv: &mut TypeEnv, defs: &[ast::Def]) {
        for def in defs {
            match def {
                &ast::Def::Rule(ref rule) => {
                    let pos = rule.pos;
                    let mut bindings = Bindings::default();
                    bindings.enter_scope();

                    let (sym, args) = if let ast::Pattern::Term { sym, args, .. } = &rule.pattern {
                        (sym, args)
                    } else {
                        tyenv.report_error(
                            pos,
                            "Rule does not have a term at the root of its left-hand side"
                                .to_string(),
                        );
                        continue;
                    };

                    let root_term = if let Some(term) = self.get_term_by_name(tyenv, sym) {
                        term
                    } else {
                        tyenv.report_error(
                            pos,
                            "Cannot define a rule for an unknown term".to_string(),
                        );
                        continue;
                    };

                    let termdata = &self.terms[root_term.index()];

                    let flags = match &termdata.kind {
                        TermKind::Decl { flags, .. } => *flags,
                        _ => {
                            tyenv.report_error(
                                pos,
                                "Cannot define a rule on a left-hand-side that is an enum variant"
                                    .to_string(),
                            );
                            continue;
                        }
                    };

                    termdata.check_args_count(args, tyenv, pos, sym);
                    let args = self.translate_args(args, termdata, tyenv, &mut bindings);

                    let iflets = rule
                        .iflets
                        .iter()
                        .filter_map(|iflet| {
                            self.translate_iflet(tyenv, iflet, &mut bindings, flags)
                        })
                        .collect();
                    let rhs = unwrap_or_continue!(self.translate_expr(
                        tyenv,
                        &rule.expr,
                        Some(termdata.ret_ty),
                        &mut bindings,
                        flags,
                    ));

                    bindings.exit_scope();

                    let prio = if let Some(prio) = rule.prio {
                        if flags.multi {
                            tyenv.report_error(
                                pos,
                                "Cannot set rule priorities in multi-terms".to_string(),
                            );
                        }
                        prio
                    } else {
                        0
                    };

                    let rid = RuleId(self.rules.len());
                    self.rules.push(Rule {
                        id: rid,
                        root_term,
                        args,
                        iflets,
                        rhs,
                        vars: bindings.seen,
                        prio,
                        pos,
                        name: rule.name.as_ref().map(|i| tyenv.intern_mut(i)),
                    });
                }
                _ => {}
            }
        }
    }

    fn check_for_undefined_decls(&self, tyenv: &mut TypeEnv, defs: &[ast::Def]) {
        for def in defs {
            if let ast::Def::Decl(decl) = def {
                let term = self.get_term_by_name(tyenv, &decl.term).unwrap();
                let term = &self.terms[term.index()];
                if !term.has_constructor() && !term.has_extractor() {
                    tyenv.report_error(
                        decl.pos,
                        format!(
                            "no rules, extractor, or external definition for declaration '{}'",
                            decl.term.0
                        ),
                    );
                }
            }
        }
    }

    fn check_for_expr_terms_without_constructors(&self, tyenv: &mut TypeEnv, defs: &[ast::Def]) {
        for def in defs {
            if let ast::Def::Rule(rule) = def {
                rule.expr.terms(&mut |pos, ident| {
                    let term = match self.get_term_by_name(tyenv, ident) {
                        None => {
                            debug_assert!(!tyenv.errors.is_empty());
                            return;
                        }
                        Some(t) => t,
                    };
                    let term = &self.terms[term.index()];
                    if !term.has_constructor() {
                        tyenv.report_error(
                            pos,
                            format!(
                                "term `{}` cannot be used in an expression because \
                                 it does not have a constructor",
                                ident.0
                            ),
                        )
                    }
                });
            }
        }
    }

    fn maybe_implicit_convert_pattern(
        &self,
        tyenv: &mut TypeEnv,
        pattern: &ast::Pattern,
        inner_ty: TypeId,
        outer_ty: TypeId,
    ) -> Option<ast::Pattern> {
        if let Some(converter_term) = self.converters.get(&(inner_ty, outer_ty)) {
            if self.terms[converter_term.index()].has_extractor() {
                // This is a little awkward: we have to
                // convert back to an Ident, to be
                // re-resolved. The pos doesn't matter
                // as it shouldn't result in a lookup
                // failure.
                let converter_term_ident = ast::Ident(
                    tyenv.syms[self.terms[converter_term.index()].name.index()].clone(),
                    pattern.pos(),
                );
                let expanded_pattern = ast::Pattern::Term {
                    sym: converter_term_ident,
                    pos: pattern.pos(),
                    args: vec![pattern.clone()],
                };

                return Some(expanded_pattern);
            }
        }
        None
    }

    fn translate_pattern(
        &self,
        tyenv: &mut TypeEnv,
        pat: &ast::Pattern,
        expected_ty: TypeId,
        bindings: &mut Bindings,
    ) -> Option<Pattern> {
        log!("translate_pattern: {:?}", pat);
        log!("translate_pattern: bindings = {:?}", bindings);
        match pat {
            // TODO: flag on primitive type decl indicating it's an integer type?
            &ast::Pattern::ConstInt { val, pos } => {
                let ty = &tyenv.types[expected_ty.index()];
                if !ty.is_int() && !ty.is_prim() {
                    tyenv.report_error(
                        pos,
                        format!(
                            "expected non-integer type {}, but found integer literal '{}'",
                            ty.name(tyenv),
                            val,
                        ),
                    );
                }
                Some(Pattern::ConstInt(expected_ty, val))
            }
            &ast::Pattern::ConstBool { val, pos } => {
                if expected_ty != TypeId::BOOL {
                    tyenv.report_error(
                        pos,
                        format!(
                            "Boolean literal '{val}' has type {} but we need {} in context",
                            BuiltinType::Bool.name(),
                            tyenv.types[expected_ty.index()].name(tyenv)
                        ),
                    )
                }
                Some(Pattern::ConstBool(TypeId::BOOL, val))
            }
            &ast::Pattern::ConstPrim { ref val, pos } => {
                let val = tyenv.intern_mut(val);
                let const_ty = match tyenv.const_types.get(&val) {
                    Some(ty) => *ty,
                    None => {
                        tyenv.report_error(pos, "Unknown constant");
                        return None;
                    }
                };
                if expected_ty != const_ty {
                    tyenv.report_error(pos, "Type mismatch for constant");
                }
                Some(Pattern::ConstPrim(const_ty, val))
            }
            &ast::Pattern::Wildcard { .. } => Some(Pattern::Wildcard(expected_ty)),
            &ast::Pattern::And { ref subpats, .. } => {
                // If any of the subpatterns fails to type-check, we'll report
                // an error at that point. Here, just skip it and keep looking
                // for more errors.
                let children = subpats
                    .iter()
                    .filter_map(|subpat| {
                        self.translate_pattern(tyenv, subpat, expected_ty, bindings)
                    })
                    .collect();
                Some(Pattern::And(expected_ty, children))
            }
            &ast::Pattern::BindPattern {
                ref var,
                ref subpat,
                pos,
            } => {
                let subpat = self.translate_pattern(tyenv, subpat, expected_ty, bindings)?;

                // The sub-pattern's type should be `expected_ty`. If it isn't,
                // we've already reported a type error about it, but continue
                // using the type we actually found in hopes that we'll
                // generate fewer follow-on error messages.
                let ty = subpat.ty();

                let name = tyenv.intern_mut(var);
                if bindings.lookup(name).is_some() {
                    tyenv.report_error(
                        pos,
                        format!("Re-bound variable name in LHS pattern: '{}'", var.0),
                    );
                    // Try to keep going.
                }
                let id = bindings.add_var(name, ty);
                Some(Pattern::BindPattern(ty, id, Box::new(subpat)))
            }
            &ast::Pattern::Var { ref var, pos } => {
                // Look up the variable; if it has already been bound,
                // then this becomes a `Var` node (which matches the
                // existing bound value), otherwise it becomes a
                // `BindPattern` with a wildcard subpattern to capture
                // at this location.
                let name = tyenv.intern_mut(var);
                match bindings.lookup(name) {
                    None => {
                        let id = bindings.add_var(name, expected_ty);
                        Some(Pattern::BindPattern(
                            expected_ty,
                            id,
                            Box::new(Pattern::Wildcard(expected_ty)),
                        ))
                    }
                    Some(bv) => {
                        if expected_ty != bv.ty {
                            tyenv.report_error(
                                pos,
                                format!(
                                    "Mismatched types: pattern expects type '{}' but already-bound var '{}' has type '{}'",
                                    tyenv.types[expected_ty.index()].name(tyenv),
                                    var.0,
                                    tyenv.types[bv.ty.index()].name(tyenv),
                                ),
                            );
                            // Try to keep going for more errors.
                        }
                        Some(Pattern::Var(bv.ty, bv.id))
                    }
                }
            }
            &ast::Pattern::Term {
                ref sym,
                ref args,
                pos,
            } => {
                // Look up the term.
                let tid = match self.get_term_by_name(tyenv, sym) {
                    Some(t) => t,
                    None => {
                        tyenv.report_error(pos, format!("Unknown term in pattern: '{}'", sym.0));
                        return None;
                    }
                };

                let termdata = &self.terms[tid.index()];

                // Get the return type and arg types. Verify the
                // expected type of this pattern, if any, against the
                // return type of the term. Insert an implicit
                // converter if needed.
                let ret_ty = termdata.ret_ty;
                if expected_ty != ret_ty {
                    // Can we do an implicit type conversion? Look
                    // up the converter term, if any. If one has
                    // been registered, and the term has an
                    // extractor, then build an expanded AST node
                    // right here and recurse on it.
                    if let Some(expanded_pattern) =
                        self.maybe_implicit_convert_pattern(tyenv, pat, ret_ty, expected_ty)
                    {
                        return self.translate_pattern(
                            tyenv,
                            &expanded_pattern,
                            expected_ty,
                            bindings,
                        );
                    }

                    tyenv.report_error(
                        pos,
                        format!(
                            "Mismatched types: pattern expects type '{}' but term has return type '{}'",
                            tyenv.types[expected_ty.index()].name(tyenv),
                            tyenv.types[ret_ty.index()].name(tyenv),
                        ),
                    );
                    // Try to keep going for more errors.
                }

                termdata.check_args_count(args, tyenv, pos, sym);

                // TODO: check that multi-extractors are only used in terms declared `multi`

                match &termdata.kind {
                    TermKind::EnumVariant { .. } => {}
                    TermKind::Decl {
                        extractor_kind: Some(ExtractorKind::ExternalExtractor { .. }),
                        ..
                    } => {}
                    TermKind::Decl {
                        extractor_kind: Some(ExtractorKind::InternalExtractor { template }),
                        ..
                    } => {
                        if self.expand_internal_extractors {
                            // Expand the extractor macro! We create a map
                            // from macro args to AST pattern trees and
                            // then evaluate the template with these
                            // substitutions.
                            log!("internal extractor macro args = {:?}", args);
                            let pat = template.subst_macro_args(&args)?;
                            return self.translate_pattern(tyenv, &pat, expected_ty, bindings);
                        }
                    }
                    TermKind::Decl {
                        extractor_kind: None,
                        ..
                    } => {
                        tyenv.report_error(
                            pos,
                            format!(
                                "Cannot use term '{}' that does not have a defined extractor in a \
                                 left-hand side pattern",
                                sym.0
                            ),
                        );
                    }
                }

                let subpats = self.translate_args(args, termdata, tyenv, bindings);
                Some(Pattern::Term(ret_ty, tid, subpats))
            }
            &ast::Pattern::MacroArg { .. } => unreachable!(),
        }
    }

    fn translate_args(
        &self,
        args: &Vec<ast::Pattern>,
        termdata: &Term,
        tyenv: &mut TypeEnv,
        bindings: &mut Bindings,
    ) -> Vec<Pattern> {
        args.iter()
            .zip(termdata.arg_tys.iter())
            .filter_map(|(arg, &arg_ty)| self.translate_pattern(tyenv, arg, arg_ty, bindings))
            .collect()
    }

    fn maybe_implicit_convert_expr(
        &self,
        tyenv: &mut TypeEnv,
        expr: &ast::Expr,
        inner_ty: TypeId,
        outer_ty: TypeId,
    ) -> Option<ast::Expr> {
        // Is there a converter for this type mismatch?
        if let Some(converter_term) = self.converters.get(&(inner_ty, outer_ty)) {
            if self.terms[converter_term.index()].has_constructor() {
                let converter_ident = ast::Ident(
                    tyenv.syms[self.terms[converter_term.index()].name.index()].clone(),
                    expr.pos(),
                );
                return Some(ast::Expr::Term {
                    sym: converter_ident,
                    pos: expr.pos(),
                    args: vec![expr.clone()],
                });
            }
        }
        None
    }

    fn translate_expr(
        &self,
        tyenv: &mut TypeEnv,
        expr: &ast::Expr,
        ty: Option<TypeId>,
        bindings: &mut Bindings,
        root_flags: TermFlags,
    ) -> Option<Expr> {
        log!("translate_expr: {:?}", expr);
        match expr {
            &ast::Expr::Term {
                ref sym,
                ref args,
                pos,
            } => {
                // Look up the term.
                let name = tyenv.intern_mut(&sym);
                let tid = match self.term_map.get(&name) {
                    Some(&t) => t,
                    None => {
                        // Maybe this was actually a variable binding and the user has placed
                        // parens around it by mistake? (See #4775.)
                        if bindings.lookup(name).is_some() {
                            tyenv.report_error(
                                pos,
                                format!(
                                    "Unknown term in expression: '{}'. Variable binding under this name exists; try removing the parens?", sym.0));
                        } else {
                            tyenv.report_error(
                                pos,
                                format!("Unknown term in expression: '{}'", sym.0),
                            );
                        }
                        return None;
                    }
                };
                let termdata = &self.terms[tid.index()];

                // Get the return type and arg types. Verify the
                // expected type of this pattern, if any, against the
                // return type of the term, and determine whether we
                // are doing an implicit conversion. Report an error
                // if types don't match and no conversion is possible.
                let ret_ty = termdata.ret_ty;
                let ty = if ty.is_some() && ret_ty != ty.unwrap() {
                    // Is there a converter for this type mismatch?
                    if let Some(expanded_expr) =
                        self.maybe_implicit_convert_expr(tyenv, expr, ret_ty, ty.unwrap())
                    {
                        return self.translate_expr(
                            tyenv,
                            &expanded_expr,
                            ty,
                            bindings,
                            root_flags,
                        );
                    }

                    tyenv.report_error(
                        pos,
                        format!("Mismatched types: expression expects type '{}' but term has return type '{}'",
                                tyenv.types[ty.unwrap().index()].name(tyenv),
                                tyenv.types[ret_ty.index()].name(tyenv)));

                    // Keep going, to discover more errors.
                    ret_ty
                } else {
                    ret_ty
                };

                if let TermKind::Decl { flags, .. } = &termdata.kind {
                    // On the left-hand side of a rule or in a pure term, only pure terms may be
                    // used.
                    let pure_required = root_flags.pure;
                    if pure_required && !flags.pure {
                        tyenv.report_error(
                            pos,
                            format!(
                                "Used non-pure constructor '{}' in pure expression context",
                                sym.0
                            ),
                        );
                    }

                    // Multi-terms may only be used inside other multi-terms.
                    if !root_flags.multi && flags.multi {
                        tyenv.report_error(
                            pos,
                            format!(
                                "Used multi-constructor '{}' but this rule is not in a multi-term",
                                sym.0
                            ),
                        );
                    }

                    // Partial terms may always be used on the left-hand side of a rule. On the
                    // right-hand side they may only be used inside other partial terms.
                    let partial_allowed = root_flags.partial;
                    if !partial_allowed && flags.partial {
                        tyenv.report_error(
                            pos,
                            format!(
                                "Rule can't use partial constructor '{}' on RHS; \
                                try moving it to if-let{}",
                                sym.0,
                                if root_flags.multi {
                                    ""
                                } else {
                                    " or make this rule's term partial too"
                                }
                            ),
                        );
                    }
                }

                termdata.check_args_count(args, tyenv, pos, sym);

                // Resolve subexpressions.
                let subexprs = args
                    .iter()
                    .zip(termdata.arg_tys.iter())
                    .filter_map(|(arg, &arg_ty)| {
                        self.translate_expr(tyenv, arg, Some(arg_ty), bindings, root_flags)
                    })
                    .collect();

                Some(Expr::Term(ty, tid, subexprs))
            }
            &ast::Expr::Var { ref name, pos } => {
                let sym = tyenv.intern_mut(name);
                // Look through bindings, innermost (most recent) first.
                let bv = match bindings.lookup(sym) {
                    None => {
                        tyenv.report_error(pos, format!("Unknown variable '{}'", name.0));
                        return None;
                    }
                    Some(bv) => bv,
                };

                // Verify type. Maybe do an implicit conversion.
                if ty.is_some() && bv.ty != ty.unwrap() {
                    // Is there a converter for this type mismatch?
                    if let Some(expanded_expr) =
                        self.maybe_implicit_convert_expr(tyenv, expr, bv.ty, ty.unwrap())
                    {
                        return self.translate_expr(
                            tyenv,
                            &expanded_expr,
                            ty,
                            bindings,
                            root_flags,
                        );
                    }

                    tyenv.report_error(
                        pos,
                        format!(
                            "Variable '{}' has type {} but we need {} in context",
                            name.0,
                            tyenv.types[bv.ty.index()].name(tyenv),
                            tyenv.types[ty.unwrap().index()].name(tyenv)
                        ),
                    );
                }

                Some(Expr::Var(bv.ty, bv.id))
            }
            &ast::Expr::ConstBool { val, pos } => {
                match ty {
                    Some(ty) if ty != TypeId::BOOL => tyenv.report_error(
                        pos,
                        format!(
                            "Boolean literal '{val}' has type {} but we need {} in context",
                            BuiltinType::Bool.name(),
                            tyenv.types[ty.index()].name(tyenv)
                        ),
                    ),
                    Some(..) | None => {}
                };
                Some(Expr::ConstBool(TypeId::BOOL, val))
            }
            &ast::Expr::ConstInt { val, pos } => {
                let Some(ty) = ty else {
                    tyenv.report_error(
                        pos,
                        "integer literal in a context that needs an explicit type".to_string(),
                    );
                    return None;
                };

                let typ = &tyenv.types[ty.index()];

                if !typ.is_int() && !typ.is_prim() {
                    tyenv.report_error(
                        pos,
                        format!(
                            "expected non-integer type {}, but found integer literal '{}'",
                            tyenv.types[ty.index()].name(tyenv),
                            val,
                        ),
                    );
                }
                Some(Expr::ConstInt(ty, val))
            }
            &ast::Expr::ConstPrim { ref val, pos } => {
                let val = tyenv.intern_mut(val);
                let const_ty = match tyenv.const_types.get(&val) {
                    Some(ty) => *ty,
                    None => {
                        tyenv.report_error(pos, "Unknown constant");
                        return None;
                    }
                };
                if ty.is_some() && const_ty != ty.unwrap() {
                    tyenv.report_error(
                        pos,
                        format!(
                            "Constant '{}' has wrong type: expected {}, but is actually {}",
                            tyenv.syms[val.index()],
                            tyenv.types[ty.unwrap().index()].name(tyenv),
                            tyenv.types[const_ty.index()].name(tyenv)
                        ),
                    );
                    return None;
                }
                Some(Expr::ConstPrim(const_ty, val))
            }
            &ast::Expr::Let {
                ref defs,
                ref body,
                pos,
            } => {
                bindings.enter_scope();

                // For each new binding...
                let mut let_defs = vec![];
                for def in defs {
                    // Check that the given variable name does not already exist.
                    let name = tyenv.intern_mut(&def.var);

                    // Look up the type.
                    let tid = match tyenv.get_type_by_name(&def.ty) {
                        Some(tid) => tid,
                        None => {
                            tyenv.report_error(
                                pos,
                                format!("Unknown type {} for variable '{}'", def.ty.0, def.var.0),
                            );
                            continue;
                        }
                    };

                    // Evaluate the variable's value.
                    let val = Box::new(unwrap_or_continue!(self.translate_expr(
                        tyenv,
                        &def.val,
                        Some(tid),
                        bindings,
                        root_flags,
                    )));

                    // Bind the var with the given type.
                    let id = bindings.add_var(name, tid);
                    let_defs.push((id, tid, val));
                }

                // Evaluate the body, expecting the type of the overall let-expr.
                let body = Box::new(self.translate_expr(tyenv, body, ty, bindings, root_flags)?);
                let body_ty = body.ty();

                // Pop the bindings.
                bindings.exit_scope();

                Some(Expr::Let {
                    ty: body_ty,
                    bindings: let_defs,
                    body,
                })
            }
        }
    }

    fn translate_iflet(
        &self,
        tyenv: &mut TypeEnv,
        iflet: &ast::IfLet,
        bindings: &mut Bindings,
        root_flags: TermFlags,
    ) -> Option<IfLet> {
        // Translate the expr first. The `if-let` and `if` forms are part of the left-hand side of
        // the rule.
        let rhs = self.translate_expr(tyenv, &iflet.expr, None, bindings, root_flags.on_lhs())?;
        let lhs = self.translate_pattern(tyenv, &iflet.pattern, rhs.ty(), bindings)?;

        Some(IfLet { lhs, rhs })
    }

    /// Lookup term by name.
    pub fn get_term_by_name(&self, tyenv: &TypeEnv, sym: &ast::Ident) -> Option<TermId> {
        tyenv
            .intern(sym)
            .and_then(|sym| self.term_map.get(&sym))
            .copied()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::Ident;
    use crate::lexer::Lexer;
    use crate::parser::parse;

    #[test]
    fn build_type_env() {
        let text = r"
            (type UImm8 (primitive UImm8))
            (type A extern (enum (B (f1 u32) (f2 u32)) (C (f1 u32))))
        ";
        let ast = parse(Lexer::new(0, text).unwrap()).expect("should parse");
        let tyenv = TypeEnv::from_ast(&ast).expect("should not have type-definition errors");

        let sym_a = tyenv
            .intern(&Ident("A".to_string(), Default::default()))
            .unwrap();
        let sym_b = tyenv
            .intern(&Ident("B".to_string(), Default::default()))
            .unwrap();
        let sym_c = tyenv
            .intern(&Ident("C".to_string(), Default::default()))
            .unwrap();
        let sym_a_b = tyenv
            .intern(&Ident("A.B".to_string(), Default::default()))
            .unwrap();
        let sym_a_c = tyenv
            .intern(&Ident("A.C".to_string(), Default::default()))
            .unwrap();
        let sym_uimm8 = tyenv
            .intern(&Ident("UImm8".to_string(), Default::default()))
            .unwrap();
        let sym_f1 = tyenv
            .intern(&Ident("f1".to_string(), Default::default()))
            .unwrap();
        let sym_f2 = tyenv
            .intern(&Ident("f2".to_string(), Default::default()))
            .unwrap();

        assert_eq!(tyenv.type_map.get(&sym_uimm8).unwrap(), &TypeId(13));
        assert_eq!(tyenv.type_map.get(&sym_a).unwrap(), &TypeId(14));

        let expected_types = vec![
            Type::Primitive(
                TypeId(13),
                sym_uimm8,
                Pos {
                    file: 0,
                    offset: 19,
                },
            ),
            Type::Enum {
                name: sym_a,
                id: TypeId(14),
                is_extern: true,
                is_nodebug: false,
                variants: vec![
                    Variant {
                        name: sym_b,
                        fullname: sym_a_b,
                        id: VariantId(0),
                        fields: vec![
                            Field {
                                name: sym_f1,
                                id: FieldId(0),
                                ty: TypeId::U32,
                            },
                            Field {
                                name: sym_f2,
                                id: FieldId(1),
                                ty: TypeId::U32,
                            },
                        ],
                    },
                    Variant {
                        name: sym_c,
                        fullname: sym_a_c,
                        id: VariantId(1),
                        fields: vec![Field {
                            name: sym_f1,
                            id: FieldId(0),
                            ty: TypeId::U32,
                        }],
                    },
                ],
                pos: Pos {
                    file: 0,
                    offset: 62,
                },
            },
        ];

        assert_eq!(
            tyenv.types.len(),
            expected_types.len() + BuiltinType::ALL.len()
        );
        for (i, (actual, expected)) in tyenv
            .types
            .iter()
            .skip(BuiltinType::ALL.len())
            .zip(&expected_types)
            .enumerate()
        {
            assert_eq!(expected, actual, "`{i}`th type is not equal!");
        }
    }
}
