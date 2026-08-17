use alloc::vec::Vec;
use core::hash::Hash;

use crate::diagnostic_filter::DiagnosticFilterNode;
use crate::front::wgsl::parse::directive::enable_extension::EnableExtensions;
use crate::front::wgsl::parse::number::Number;
use crate::{Arena, FastIndexSet, Handle, Span};

#[derive(Debug, Default)]
pub struct TranslationUnit<'a> {
    pub enable_extensions: EnableExtensions,
    pub decls: Arena<GlobalDecl<'a>>,
    /// The common expressions arena for the entire translation unit.
    ///
    /// All functions, global initializers, array lengths, etc. store their
    /// expressions here. We apportion these out to individual Naga
    /// [`Function`]s' expression arenas at lowering time. Keeping them all in a
    /// single arena simplifies handling of things like array lengths (which are
    /// effectively global and thus don't clearly belong to any function) and
    /// initializers (which can appear in both function-local and module-scope
    /// contexts).
    ///
    /// [`Function`]: crate::Function
    pub expressions: Arena<Expression<'a>>,

    /// Arena for all diagnostic filter rules parsed in this module, including those in functions.
    ///
    /// See [`DiagnosticFilterNode`] for details on how the tree is represented and used in
    /// validation.
    pub diagnostic_filters: Arena<DiagnosticFilterNode>,
    /// The leaf of all `diagnostic(…)` directives in this module.
    ///
    /// See [`DiagnosticFilterNode`] for details on how the tree is represented and used in
    /// validation.
    pub diagnostic_filter_leaf: Option<Handle<DiagnosticFilterNode>>,

    /// Doc comments appearing first in the file.
    /// This serves as documentation for the whole TranslationUnit.
    pub doc_comments: Vec<&'a str>,
}

#[derive(Debug, Clone, Copy)]
pub struct Ident<'a> {
    pub name: &'a str,
    pub span: Span,
}

/// An identifier that [resolves] to some declaration.
///
/// This does not cover context-dependent names: attributes, built-in values,
/// and so on. We map those to their Naga IR equivalents as soon as they're
/// parsed, so they never need to appear as identifiers in the AST.
///
/// [resolves]: https://gpuweb.github.io/gpuweb/wgsl/#resolves
#[derive(Debug)]
pub enum IdentExpr<'a> {
    /// An identifier referring to a module-scope declaration or predeclared
    /// object.
    ///
    /// We need to collect the entire module before we can resolve this, to
    /// distinguish between predeclared objects and module-scope declarations
    /// that appear after their uses.
    ///
    /// Whenever you create one of these values, you almost certainly want to
    /// insert the `&str` into [`ExpressionContext::unresolved`][ECu], to ensure
    /// that [indexing] knows that the name's declaration must be lowered before
    /// the one containing this use. Using [`Parser::ident_expr`][ie] to build
    /// `IdentExpr` will take care of that for you.
    ///
    /// [ECu]: super::ExpressionContext::unresolved
    /// [ie]: super::Parser::ident_expr
    /// [indexing]: crate::front::wgsl::index::Index::generate
    Unresolved(&'a str),

    /// An identifier that has been resolved to a non-module-scope declaration.
    Local(Handle<Local>),
}

/// An identifier with optional template parameters.
///
/// Following the WGSL specification (see the [`template_list`] non-terminal),
/// `TemplateElaboratedIdent` represents all template parameters as expressions:
/// even parameters to type generators, like the `f32` in `vec3<f32>`, are [Type
/// Expressions].
///
/// # Examples
///
/// - A use of a global variable `colors` would be an [`Expression::Ident(v)`][EI],
///   where `v` is an `TemplateElaboratedIdent` whose `ident` is
///   [`IdentExpr::Unresolved("colors")`][IEU]. Lowering will resolve this to a
///   reference to the global variable.
///
/// - The type `f32` in a variable declaration is represented as a
///   `TemplateElaboratedIdent` whose `ident` is
///   [`IdentExpr::Unresolved("f32")`][IEU]. Lowering will resolve this to
///   WGSL's predeclared `f32` type.
///
/// - The type `vec3<f32>` can be represented as a `TemplateElaboratedIdent`
///   whose `ident` is [`IdentExpr::Unresolved("vec3")`][IEU], and whose
///   `template_list` has one element: an [`ExpressionIdent(v)`][EI] where `v` is a
///   nested `TemplateElaboratedIdent` representing `f32` as described above.
///
/// - The type `array<vec3<f32>, 4>` has `"array"` as its `ident`, and then
///   a two-element `template_list`:
///
///     - `template_list[0]` is an [`Expression::Ident(v)`][EI] where `v` is a nested
///       `TemplateElaboratedIdent` representing `vec3<f32>` as described above.
///
///     - `template_list[1]` is an [`Expression`] representing `4`.
///
/// After [indexing] the module to ensure that declarations appear before uses,
/// lowering can see which declaration a given `TemplateElaboratedIdent`s
/// `ident` refers to. The declaration then determines how to interpret the
/// `template_list`.
///
/// [`template_list`]: https://gpuweb.github.io/gpuweb/wgsl/#syntax-template_list
/// [Type Expressions]: https://gpuweb.github.io/gpuweb/wgsl/#type-expr
/// [IEU]: IdentExpr::Unresolved
/// [EI]: Expression::Ident
/// [indexing]: crate::front::wgsl::index::Index::generate
#[derive(Debug)]
pub struct TemplateElaboratedIdent<'a> {
    pub ident: IdentExpr<'a>,
    pub ident_span: Span,

    /// If non-empty, the template parameters following the identifier.
    pub template_list: Vec<Handle<Expression<'a>>>,
    pub template_list_span: Span,
}

/// A function call or value constructor expression.
///
/// We can't tell whether an expression like `IDENTIFIER(EXPR, ...)` is a
/// construction expression or a function call until we know `IDENTIFIER`'s
/// definition, so we represent everything of that form as one of these
/// expressions until lowering. At that point, [`Lowerer::call`] has
/// everything's definition in hand, and can decide whether to emit a Naga
/// [`Constant`], [`As`], [`Splat`], or [`Compose`] expression.
///
/// [`Lowerer::call`]: Lowerer::call
/// [`Constant`]: crate::Expression::Constant
/// [`As`]: crate::Expression::As
/// [`Splat`]: crate::Expression::Splat
/// [`Compose`]: crate::Expression::Compose
#[derive(Debug)]
pub struct CallPhrase<'a> {
    pub function: TemplateElaboratedIdent<'a>,
    pub arguments: Vec<Handle<Expression<'a>>>,
}

/// A reference to a module-scope definition or predeclared object.
///
/// Each [`GlobalDecl`] holds a set of these values, to be resolved to
/// specific definitions later. To support de-duplication, `Eq` and
/// `Hash` on a `Dependency` value consider only the name, not the
/// source location at which the reference occurs.
#[derive(Debug)]
pub struct Dependency<'a> {
    /// The name referred to.
    pub ident: &'a str,

    /// The location at which the reference to that name occurs.
    pub usage: Span,
}

impl Hash for Dependency<'_> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
    }
}

impl PartialEq for Dependency<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl Eq for Dependency<'_> {}

/// A module-scope declaration.
#[derive(Debug)]
pub struct GlobalDecl<'a> {
    pub kind: GlobalDeclKind<'a>,

    /// Names of all module-scope or predeclared objects this
    /// declaration uses.
    pub dependencies: FastIndexSet<Dependency<'a>>,
}

#[derive(Debug)]
pub enum GlobalDeclKind<'a> {
    Fn(Function<'a>),
    Var(GlobalVariable<'a>),
    Const(Const<'a>),
    Override(Override<'a>),
    Struct(Struct<'a>),
    Type(TypeAlias<'a>),
    ConstAssert(Handle<Expression<'a>>),
}

#[derive(Debug)]
pub struct FunctionArgument<'a> {
    pub name: Ident<'a>,
    pub ty: TemplateElaboratedIdent<'a>,
    pub binding: Option<Binding<'a>>,
    pub handle: Handle<Local>,
}

#[derive(Debug)]
pub struct FunctionResult<'a> {
    pub ty: TemplateElaboratedIdent<'a>,
    pub binding: Option<Binding<'a>>,
    pub must_use: bool,
}

#[derive(Debug)]
pub struct EntryPoint<'a> {
    pub stage: crate::ShaderStage,
    pub early_depth_test: Option<crate::EarlyDepthTest>,
    pub workgroup_size: Option<[Option<Handle<Expression<'a>>>; 3]>,
    pub mesh_output_variable: Option<(&'a str, Span)>,
    pub task_payload: Option<(&'a str, Span)>,
    pub ray_incoming_payload: Option<(&'a str, Span)>,
}

#[cfg(doc)]
use crate::front::wgsl::lower::{LocalExpressionContext, StatementContext};

#[derive(Debug)]
pub struct Function<'a> {
    pub entry_point: Option<EntryPoint<'a>>,
    pub name: Ident<'a>,
    pub arguments: Vec<FunctionArgument<'a>>,
    pub result: Option<FunctionResult<'a>>,
    pub body: Block<'a>,
    pub diagnostic_filter_leaf: Option<Handle<DiagnosticFilterNode>>,
    pub doc_comments: Vec<&'a str>,
}

#[derive(Debug)]
pub enum Binding<'a> {
    BuiltIn(crate::BuiltIn),
    Location {
        location: Handle<Expression<'a>>,
        interpolation: Option<crate::Interpolation>,
        sampling: Option<crate::Sampling>,
        blend_src: Option<Handle<Expression<'a>>>,
        per_primitive: bool,
    },
}

#[derive(Debug)]
pub struct ResourceBinding<'a> {
    pub group: Handle<Expression<'a>>,
    pub binding: Handle<Expression<'a>>,
}

#[derive(Debug)]
pub struct GlobalVariable<'a> {
    pub name: Ident<'a>,

    /// The template list parameters for the `var`, giving the variable's
    /// address space and access mode, if present.
    pub template_list: Vec<Handle<Expression<'a>>>,

    /// The `@group` and `@binding` attributes, if present.
    pub binding: Option<ResourceBinding<'a>>,

    pub ty: Option<TemplateElaboratedIdent<'a>>,
    pub init: Option<Handle<Expression<'a>>>,
    pub doc_comments: Vec<&'a str>,

    /// Memory decorations for this variable (`@coherent`, `@volatile`).
    pub memory_decorations: crate::MemoryDecorations,
}

#[derive(Debug)]
pub struct StructMember<'a> {
    pub name: Ident<'a>,
    pub ty: TemplateElaboratedIdent<'a>,
    pub binding: Option<Binding<'a>>,
    pub align: Option<Handle<Expression<'a>>>,
    pub size: Option<Handle<Expression<'a>>>,
    pub doc_comments: Vec<&'a str>,
}

#[derive(Debug)]
pub struct Struct<'a> {
    pub name: Ident<'a>,
    pub members: Vec<StructMember<'a>>,
    pub doc_comments: Vec<&'a str>,
}

#[derive(Debug)]
pub struct TypeAlias<'a> {
    pub name: Ident<'a>,
    pub ty: TemplateElaboratedIdent<'a>,
}

#[derive(Debug)]
pub struct Const<'a> {
    pub name: Ident<'a>,
    pub ty: Option<TemplateElaboratedIdent<'a>>,
    pub init: Handle<Expression<'a>>,
    pub doc_comments: Vec<&'a str>,
}

#[derive(Debug)]
pub struct Override<'a> {
    pub name: Ident<'a>,
    pub id: Option<Handle<Expression<'a>>>,
    pub ty: Option<TemplateElaboratedIdent<'a>>,
    pub init: Option<Handle<Expression<'a>>>,
}

#[derive(Debug, Default)]
pub struct Block<'a> {
    pub stmts: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub struct Statement<'a> {
    pub kind: StatementKind<'a>,
    pub span: Span,
}

#[derive(Debug)]
pub enum StatementKind<'a> {
    LocalDecl(LocalDecl<'a>),
    Block(Block<'a>),
    If {
        condition: Handle<Expression<'a>>,
        accept: Block<'a>,
        reject: Block<'a>,
    },
    Switch {
        selector: Handle<Expression<'a>>,
        cases: Vec<SwitchCase<'a>>,
    },
    Loop {
        body: Block<'a>,
        continuing: Block<'a>,
        break_if: Option<Handle<Expression<'a>>>,
    },
    Break,
    Continue,
    Return {
        value: Option<Handle<Expression<'a>>>,
    },
    Kill,
    Call(CallPhrase<'a>),
    Assign {
        target: Handle<Expression<'a>>,
        op: Option<crate::BinaryOperator>,
        value: Handle<Expression<'a>>,
    },
    Increment(Handle<Expression<'a>>),
    Decrement(Handle<Expression<'a>>),
    Phony(Handle<Expression<'a>>),
    ConstAssert(Handle<Expression<'a>>),
}

#[derive(Debug)]
pub enum SwitchValue<'a> {
    Expr(Handle<Expression<'a>>),
    Default,
}

#[derive(Debug)]
pub struct SwitchCase<'a> {
    pub value: SwitchValue<'a>,
    pub body: Block<'a>,
    pub fall_through: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum Literal {
    Bool(bool),
    Number(Number),
}

#[cfg(doc)]
use crate::front::wgsl::lower::Lowerer;

#[derive(Debug)]
pub enum Expression<'a> {
    Literal(Literal),
    Ident(TemplateElaboratedIdent<'a>),
    Unary {
        op: crate::UnaryOperator,
        expr: Handle<Expression<'a>>,
    },
    AddrOf(Handle<Expression<'a>>),
    Deref(Handle<Expression<'a>>),
    Binary {
        op: crate::BinaryOperator,
        left: Handle<Expression<'a>>,
        right: Handle<Expression<'a>>,
    },
    Call(CallPhrase<'a>),
    Index {
        base: Handle<Expression<'a>>,
        index: Handle<Expression<'a>>,
    },
    Member {
        base: Handle<Expression<'a>>,
        field: Ident<'a>,
    },
}

#[derive(Debug)]
pub struct LocalVariable<'a> {
    pub name: Ident<'a>,
    pub ty: Option<TemplateElaboratedIdent<'a>>,
    pub init: Option<Handle<Expression<'a>>>,
    pub handle: Handle<Local>,
}

#[derive(Debug)]
pub struct Let<'a> {
    pub name: Ident<'a>,
    pub ty: Option<TemplateElaboratedIdent<'a>>,
    pub init: Handle<Expression<'a>>,
    pub handle: Handle<Local>,
}

#[derive(Debug)]
pub struct LocalConst<'a> {
    pub name: Ident<'a>,
    pub ty: Option<TemplateElaboratedIdent<'a>>,
    pub init: Handle<Expression<'a>>,
    pub handle: Handle<Local>,
}

#[derive(Debug)]
pub enum LocalDecl<'a> {
    Var(LocalVariable<'a>),
    Let(Let<'a>),
    Const(LocalConst<'a>),
}

#[derive(Debug)]
/// A placeholder for a local variable declaration.
///
/// See [`super::ExpressionContext::locals`] for more information.
pub struct Local;
