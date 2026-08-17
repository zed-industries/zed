use alloc::{
    borrow::ToOwned,
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::num::NonZeroU32;

use crate::front::wgsl::error::{Error, ExpectedToken, InvalidAssignmentType};
use crate::front::wgsl::index::Index;
use crate::front::wgsl::parse::directive::enable_extension::EnableExtensions;
use crate::front::wgsl::parse::number::Number;
use crate::front::wgsl::parse::{ast, conv};
use crate::front::wgsl::Result;
use crate::front::Typifier;
use crate::{
    common::wgsl::{TryToWgsl, TypeContext},
    compact::KeepUnused,
};
use crate::{common::ForDebugWithTypes, proc::LayoutErrorInner};
use crate::{ir, proc};
use crate::{Arena, FastHashMap, FastIndexMap, Handle, Span};

use construction::Constructor;
use template_list::TemplateListIter;

mod construction;
mod conversion;
mod template_list;

/// Resolves the inner type of a given expression.
///
/// Expects a &mut [`ExpressionContext`] and a [`Handle<Expression>`].
///
/// Returns a &[`ir::TypeInner`].
///
/// Ideally, we would simply have a function that takes a `&mut ExpressionContext`
/// and returns a `&TypeResolution`. Unfortunately, this leads the borrow checker
/// to conclude that the mutable borrow lasts for as long as we are using the
/// `&TypeResolution`, so we can't use the `ExpressionContext` for anything else -
/// like, say, resolving another operand's type. Using a macro that expands to
/// two separate calls, only the first of which needs a `&mut`,
/// lets the borrow checker see that the mutable borrow is over.
macro_rules! resolve_inner {
    ($ctx:ident, $expr:expr) => {{
        $ctx.grow_types($expr)?;
        $ctx.typifier()[$expr].inner_with(&$ctx.module.types)
    }};
}
pub(super) use resolve_inner;

/// Resolves the inner types of two given expressions.
///
/// Expects a &mut [`ExpressionContext`] and two [`Handle<Expression>`]s.
///
/// Returns a tuple containing two &[`ir::TypeInner`].
///
/// See the documentation of [`resolve_inner!`] for why this macro is necessary.
macro_rules! resolve_inner_binary {
    ($ctx:ident, $left:expr, $right:expr) => {{
        $ctx.grow_types($left)?;
        $ctx.grow_types($right)?;
        (
            $ctx.typifier()[$left].inner_with(&$ctx.module.types),
            $ctx.typifier()[$right].inner_with(&$ctx.module.types),
        )
    }};
}

/// Resolves the type of a given expression.
///
/// Expects a &mut [`ExpressionContext`] and a [`Handle<Expression>`].
///
/// Returns a &[`TypeResolution`].
///
/// See the documentation of [`resolve_inner!`] for why this macro is necessary.
///
/// [`TypeResolution`]: proc::TypeResolution
macro_rules! resolve {
    ($ctx:ident, $expr:expr) => {{
        let expr = $expr;
        $ctx.grow_types(expr)?;
        &$ctx.typifier()[expr]
    }};
}
pub(super) use resolve;

/// State for constructing a `ir::Module`.
pub struct GlobalContext<'source, 'temp, 'out> {
    enable_extensions: EnableExtensions,

    /// The `TranslationUnit`'s expressions arena.
    ast_expressions: &'temp Arena<ast::Expression<'source>>,

    // Naga IR values.
    /// The map from the names of module-scope declarations to the Naga IR
    /// `Handle`s we have built for them, owned by `Lowerer::lower`.
    globals: &'temp mut FastHashMap<&'source str, LoweredGlobalDecl>,

    /// The module we're constructing.
    module: &'out mut ir::Module,

    const_typifier: &'temp mut Typifier,

    layouter: &'temp mut proc::Layouter,

    global_expression_kind_tracker: &'temp mut proc::ExpressionKindTracker,
}

impl<'source> GlobalContext<'source, '_, '_> {
    const fn as_const(&mut self) -> ExpressionContext<'source, '_, '_> {
        ExpressionContext {
            enable_extensions: self.enable_extensions,
            ast_expressions: self.ast_expressions,
            globals: self.globals,
            module: self.module,
            const_typifier: self.const_typifier,
            layouter: self.layouter,
            expr_type: ExpressionContextType::Constant(None),
            global_expression_kind_tracker: self.global_expression_kind_tracker,
        }
    }

    const fn as_override(&mut self) -> ExpressionContext<'source, '_, '_> {
        ExpressionContext {
            enable_extensions: self.enable_extensions,
            ast_expressions: self.ast_expressions,
            globals: self.globals,
            module: self.module,
            const_typifier: self.const_typifier,
            layouter: self.layouter,
            expr_type: ExpressionContextType::Override,
            global_expression_kind_tracker: self.global_expression_kind_tracker,
        }
    }

    fn ensure_type_exists(
        &mut self,
        name: Option<String>,
        inner: ir::TypeInner,
    ) -> Handle<ir::Type> {
        self.module
            .types
            .insert(ir::Type { inner, name }, Span::UNDEFINED)
    }
}

/// State for lowering a statement within a function.
pub struct StatementContext<'source, 'temp, 'out> {
    enable_extensions: EnableExtensions,

    // WGSL AST values.
    /// A reference to [`TranslationUnit::expressions`] for the translation unit
    /// we're lowering.
    ///
    /// [`TranslationUnit::expressions`]: ast::TranslationUnit::expressions
    ast_expressions: &'temp Arena<ast::Expression<'source>>,

    // Naga IR values.
    /// The map from the names of module-scope declarations to the Naga IR
    /// `Handle`s we have built for them, owned by `Lowerer::lower`.
    globals: &'temp mut FastHashMap<&'source str, LoweredGlobalDecl>,

    /// A map from each `ast::Local` handle to the Naga expression
    /// we've built for it:
    ///
    /// - WGSL function arguments become Naga [`FunctionArgument`] expressions.
    ///
    /// - WGSL `var` declarations become Naga [`LocalVariable`] expressions.
    ///
    /// - WGSL `let` declararations become arbitrary Naga expressions.
    ///
    /// This always borrows the `local_table` local variable in
    /// [`Lowerer::function`].
    ///
    /// [`LocalVariable`]: ir::Expression::LocalVariable
    /// [`FunctionArgument`]: ir::Expression::FunctionArgument
    local_table:
        &'temp mut FastHashMap<Handle<ast::Local>, Declared<Typed<Handle<ir::Expression>>>>,

    const_typifier: &'temp mut Typifier,
    typifier: &'temp mut Typifier,
    layouter: &'temp mut proc::Layouter,
    function: &'out mut ir::Function,
    /// Stores the names of expressions that are assigned in `let` statement
    /// Also stores the spans of the names, for use in errors.
    named_expressions: &'out mut FastIndexMap<Handle<ir::Expression>, (String, Span)>,
    module: &'out mut ir::Module,

    /// Which `Expression`s in `self.naga_expressions` are const expressions, in
    /// the WGSL sense.
    ///
    /// According to the WGSL spec, a const expression must not refer to any
    /// `let` declarations, even if those declarations' initializers are
    /// themselves const expressions. So this tracker is not simply concerned
    /// with the form of the expressions; it is also tracking whether WGSL says
    /// we should consider them to be const. See the use of `force_non_const` in
    /// the code for lowering `let` bindings.
    local_expression_kind_tracker: &'temp mut proc::ExpressionKindTracker,
    global_expression_kind_tracker: &'temp mut proc::ExpressionKindTracker,
}

impl<'a, 'temp> StatementContext<'a, 'temp, '_> {
    const fn as_const<'t>(
        &'t mut self,
        block: &'t mut ir::Block,
        emitter: &'t mut proc::Emitter,
    ) -> ExpressionContext<'a, 't, 't>
    where
        'temp: 't,
    {
        ExpressionContext {
            enable_extensions: self.enable_extensions,
            globals: self.globals,
            ast_expressions: self.ast_expressions,
            const_typifier: self.const_typifier,
            layouter: self.layouter,
            global_expression_kind_tracker: self.global_expression_kind_tracker,
            module: self.module,
            expr_type: ExpressionContextType::Constant(Some(LocalExpressionContext {
                local_table: self.local_table,
                function: self.function,
                block,
                emitter,
                typifier: self.typifier,
                local_expression_kind_tracker: self.local_expression_kind_tracker,
            })),
        }
    }

    const fn as_expression<'t>(
        &'t mut self,
        block: &'t mut ir::Block,
        emitter: &'t mut proc::Emitter,
    ) -> ExpressionContext<'a, 't, 't>
    where
        'temp: 't,
    {
        ExpressionContext {
            enable_extensions: self.enable_extensions,
            globals: self.globals,
            ast_expressions: self.ast_expressions,
            const_typifier: self.const_typifier,
            layouter: self.layouter,
            global_expression_kind_tracker: self.global_expression_kind_tracker,
            module: self.module,
            expr_type: ExpressionContextType::Runtime(LocalExpressionContext {
                local_table: self.local_table,
                function: self.function,
                block,
                emitter,
                typifier: self.typifier,
                local_expression_kind_tracker: self.local_expression_kind_tracker,
            }),
        }
    }

    #[allow(dead_code)]
    const fn as_global(&mut self) -> GlobalContext<'a, '_, '_> {
        GlobalContext {
            enable_extensions: self.enable_extensions,
            ast_expressions: self.ast_expressions,
            globals: self.globals,
            module: self.module,
            const_typifier: self.const_typifier,
            layouter: self.layouter,
            global_expression_kind_tracker: self.global_expression_kind_tracker,
        }
    }

    fn invalid_assignment_type(&self, expr: Handle<ir::Expression>) -> InvalidAssignmentType {
        if let Some(&(_, span)) = self.named_expressions.get(&expr) {
            InvalidAssignmentType::ImmutableBinding(span)
        } else {
            match self.function.expressions[expr] {
                ir::Expression::Swizzle { .. } => InvalidAssignmentType::Swizzle,
                ir::Expression::Access { base, .. } => self.invalid_assignment_type(base),
                ir::Expression::AccessIndex { base, .. } => self.invalid_assignment_type(base),
                _ => InvalidAssignmentType::Other,
            }
        }
    }
}

pub struct LocalExpressionContext<'temp, 'out> {
    /// A map from [`ast::Local`] handles to the Naga expressions we've built for them.
    ///
    /// This is always [`StatementContext::local_table`] for the
    /// enclosing statement; see that documentation for details.
    local_table: &'temp FastHashMap<Handle<ast::Local>, Declared<Typed<Handle<ir::Expression>>>>,

    function: &'out mut ir::Function,
    block: &'temp mut ir::Block,
    emitter: &'temp mut proc::Emitter,
    typifier: &'temp mut Typifier,

    /// Which `Expression`s in `self.naga_expressions` are const expressions, in
    /// the WGSL sense.
    ///
    /// See [`StatementContext::local_expression_kind_tracker`] for details.
    local_expression_kind_tracker: &'temp mut proc::ExpressionKindTracker,
}

/// The type of Naga IR expression we are lowering an [`ast::Expression`] to.
pub enum ExpressionContextType<'temp, 'out> {
    /// We are lowering to an arbitrary runtime expression, to be
    /// included in a function's body.
    ///
    /// The given [`LocalExpressionContext`] holds information about local
    /// variables, arguments, and other definitions available only to runtime
    /// expressions, not constant or override expressions.
    Runtime(LocalExpressionContext<'temp, 'out>),

    /// We are lowering to a constant expression, to be included in the module's
    /// constant expression arena.
    ///
    /// Everything global constant expressions are allowed to refer to is
    /// available in the [`ExpressionContext`], but local constant expressions can
    /// also refer to other
    Constant(Option<LocalExpressionContext<'temp, 'out>>),

    /// We are lowering to an override expression, to be included in the module's
    /// constant expression arena.
    ///
    /// Everything override expressions are allowed to refer to is
    /// available in the [`ExpressionContext`], so this variant
    /// carries no further information.
    Override,
}

/// State for lowering an [`ast::Expression`] to Naga IR.
///
/// [`ExpressionContext`]s come in two kinds, distinguished by
/// the value of the [`expr_type`] field:
///
/// - A [`Runtime`] context contributes [`naga::Expression`]s to a [`naga::Function`]'s
///   runtime expression arena.
///
/// - A [`Constant`] context contributes [`naga::Expression`]s to a [`naga::Module`]'s
///   constant expression arena.
///
/// [`ExpressionContext`]s are constructed in restricted ways:
///
/// - To get a [`Runtime`] [`ExpressionContext`], call
///   [`StatementContext::as_expression`].
///
/// - To get a [`Constant`] [`ExpressionContext`], call
///   [`GlobalContext::as_const`].
///
/// - You can demote a [`Runtime`] context to a [`Constant`] context
///   by calling [`as_const`], but there's no way to go in the other
///   direction, producing a runtime context from a constant one. This
///   is because runtime expressions can refer to constant
///   expressions, via [`Expression::Constant`], but constant
///   expressions can't refer to a function's expressions.
///
/// Not to be confused with `wgsl::parse::ExpressionContext`, which is
/// for parsing the `ast::Expression` in the first place.
///
/// [`expr_type`]: ExpressionContext::expr_type
/// [`Runtime`]: ExpressionContextType::Runtime
/// [`naga::Expression`]: ir::Expression
/// [`naga::Function`]: ir::Function
/// [`Constant`]: ExpressionContextType::Constant
/// [`naga::Module`]: ir::Module
/// [`as_const`]: ExpressionContext::as_const
/// [`Expression::Constant`]: ir::Expression::Constant
pub struct ExpressionContext<'source, 'temp, 'out> {
    enable_extensions: EnableExtensions,

    // WGSL AST values.
    ast_expressions: &'temp Arena<ast::Expression<'source>>,

    // Naga IR values.
    /// The map from the names of module-scope declarations to the Naga IR
    /// `Handle`s we have built for them, owned by `Lowerer::lower`.
    globals: &'temp mut FastHashMap<&'source str, LoweredGlobalDecl>,

    /// The IR [`Module`] we're constructing.
    ///
    /// [`Module`]: ir::Module
    module: &'out mut ir::Module,

    /// Type judgments for [`module::global_expressions`].
    ///
    /// [`module::global_expressions`]: ir::Module::global_expressions
    const_typifier: &'temp mut Typifier,
    layouter: &'temp mut proc::Layouter,
    global_expression_kind_tracker: &'temp mut proc::ExpressionKindTracker,

    /// Whether we are lowering a constant expression or a general
    /// runtime expression, and the data needed in each case.
    expr_type: ExpressionContextType<'temp, 'out>,
}

impl TypeContext for ExpressionContext<'_, '_, '_> {
    fn lookup_type(&self, handle: Handle<ir::Type>) -> &ir::Type {
        &self.module.types[handle]
    }

    fn type_name(&self, handle: Handle<ir::Type>) -> &str {
        self.module.types[handle]
            .name
            .as_deref()
            .unwrap_or("{anonymous type}")
    }

    fn write_override<W: core::fmt::Write>(
        &self,
        handle: Handle<ir::Override>,
        out: &mut W,
    ) -> core::fmt::Result {
        match self.module.overrides[handle].name {
            Some(ref name) => out.write_str(name),
            None => write!(out, "{{anonymous override {handle:?}}}"),
        }
    }

    fn write_unnamed_struct<W: core::fmt::Write>(
        &self,
        _: &ir::TypeInner,
        _: &mut W,
    ) -> core::fmt::Result {
        unreachable!("the WGSL front end should always know the type name");
    }
}

impl<'source, 'temp, 'out> ExpressionContext<'source, 'temp, 'out> {
    const fn is_runtime(&self) -> bool {
        match self.expr_type {
            ExpressionContextType::Runtime(_) => true,
            ExpressionContextType::Constant(_) | ExpressionContextType::Override => false,
        }
    }

    #[allow(dead_code)]
    const fn as_const(&mut self) -> ExpressionContext<'source, '_, '_> {
        ExpressionContext {
            enable_extensions: self.enable_extensions,
            globals: self.globals,
            ast_expressions: self.ast_expressions,
            const_typifier: self.const_typifier,
            layouter: self.layouter,
            module: self.module,
            expr_type: ExpressionContextType::Constant(match self.expr_type {
                ExpressionContextType::Runtime(ref mut local_expression_context)
                | ExpressionContextType::Constant(Some(ref mut local_expression_context)) => {
                    Some(LocalExpressionContext {
                        local_table: local_expression_context.local_table,
                        function: local_expression_context.function,
                        block: local_expression_context.block,
                        emitter: local_expression_context.emitter,
                        typifier: local_expression_context.typifier,
                        local_expression_kind_tracker: local_expression_context
                            .local_expression_kind_tracker,
                    })
                }
                ExpressionContextType::Constant(None) | ExpressionContextType::Override => None,
            }),
            global_expression_kind_tracker: self.global_expression_kind_tracker,
        }
    }

    const fn as_global(&mut self) -> GlobalContext<'source, '_, '_> {
        GlobalContext {
            enable_extensions: self.enable_extensions,
            ast_expressions: self.ast_expressions,
            globals: self.globals,
            module: self.module,
            const_typifier: self.const_typifier,
            layouter: self.layouter,
            global_expression_kind_tracker: self.global_expression_kind_tracker,
        }
    }

    const fn as_const_evaluator(&mut self) -> proc::ConstantEvaluator<'_> {
        match self.expr_type {
            ExpressionContextType::Runtime(ref mut rctx) => {
                proc::ConstantEvaluator::for_wgsl_function(
                    self.module,
                    &mut rctx.function.expressions,
                    rctx.local_expression_kind_tracker,
                    self.layouter,
                    rctx.emitter,
                    rctx.block,
                    false,
                )
            }
            ExpressionContextType::Constant(Some(ref mut rctx)) => {
                proc::ConstantEvaluator::for_wgsl_function(
                    self.module,
                    &mut rctx.function.expressions,
                    rctx.local_expression_kind_tracker,
                    self.layouter,
                    rctx.emitter,
                    rctx.block,
                    true,
                )
            }
            ExpressionContextType::Constant(None) => proc::ConstantEvaluator::for_wgsl_module(
                self.module,
                self.global_expression_kind_tracker,
                self.layouter,
                false,
            ),
            ExpressionContextType::Override => proc::ConstantEvaluator::for_wgsl_module(
                self.module,
                self.global_expression_kind_tracker,
                self.layouter,
                true,
            ),
        }
    }

    /// Return a wrapper around `value` suitable for formatting.
    ///
    /// Return a wrapper around `value` that implements
    /// [`core::fmt::Display`] in a form suitable for use in
    /// diagnostic messages.
    const fn as_diagnostic_display<T>(
        &self,
        value: T,
    ) -> crate::common::DiagnosticDisplay<(T, proc::GlobalCtx<'_>)> {
        let ctx = self.module.to_ctx();
        crate::common::DiagnosticDisplay((value, ctx))
    }

    fn append_expression(
        &mut self,
        expr: ir::Expression,
        span: Span,
    ) -> Result<'source, Handle<ir::Expression>> {
        let mut eval = self.as_const_evaluator();
        eval.try_eval_and_append(expr, span)
            .map_err(|e| Box::new(Error::ConstantEvaluatorError(e.into(), span)))
    }

    fn get_const_val<T: TryFrom<crate::Literal, Error = proc::ConstValueError>>(
        &self,
        handle: Handle<ir::Expression>,
    ) -> core::result::Result<T, proc::ConstValueError> {
        match self.expr_type {
            ExpressionContextType::Runtime(ref ctx) => {
                if !ctx.local_expression_kind_tracker.is_const(handle) {
                    return Err(proc::ConstValueError::NonConst);
                }

                self.module
                    .to_ctx()
                    .get_const_val_from(handle, &ctx.function.expressions)
            }
            ExpressionContextType::Constant(Some(ref ctx)) => {
                assert!(ctx.local_expression_kind_tracker.is_const(handle));
                self.module
                    .to_ctx()
                    .get_const_val_from(handle, &ctx.function.expressions)
            }
            ExpressionContextType::Constant(None) => self.module.to_ctx().get_const_val(handle),
            ExpressionContextType::Override => Err(proc::ConstValueError::NonConst),
        }
    }

    /// Return `true` if `handle` is a constant expression.
    fn is_const(&self, handle: Handle<ir::Expression>) -> bool {
        use ExpressionContextType as Ect;
        match self.expr_type {
            Ect::Runtime(ref ctx) | Ect::Constant(Some(ref ctx)) => {
                ctx.local_expression_kind_tracker.is_const(handle)
            }
            Ect::Constant(None) | Ect::Override => {
                self.global_expression_kind_tracker.is_const(handle)
            }
        }
    }

    fn get_expression_span(&self, handle: Handle<ir::Expression>) -> Span {
        match self.expr_type {
            ExpressionContextType::Runtime(ref ctx)
            | ExpressionContextType::Constant(Some(ref ctx)) => {
                ctx.function.expressions.get_span(handle)
            }
            ExpressionContextType::Constant(None) | ExpressionContextType::Override => {
                self.module.global_expressions.get_span(handle)
            }
        }
    }

    const fn typifier(&self) -> &Typifier {
        match self.expr_type {
            ExpressionContextType::Runtime(ref ctx)
            | ExpressionContextType::Constant(Some(ref ctx)) => ctx.typifier,
            ExpressionContextType::Constant(None) | ExpressionContextType::Override => {
                self.const_typifier
            }
        }
    }

    fn get(&self, handle: Handle<crate::Expression>) -> &crate::Expression {
        match self.expr_type {
            ExpressionContextType::Runtime(ref ctx)
            | ExpressionContextType::Constant(Some(ref ctx)) => &ctx.function.expressions[handle],
            ExpressionContextType::Constant(None) | ExpressionContextType::Override => {
                &self.module.global_expressions[handle]
            }
        }
    }

    fn local(
        &mut self,
        local: &Handle<ast::Local>,
        span: Span,
    ) -> Result<'source, Typed<Handle<ir::Expression>>> {
        match self.expr_type {
            ExpressionContextType::Runtime(ref ctx) => Ok(ctx.local_table[local].runtime()),
            ExpressionContextType::Constant(Some(ref ctx)) => ctx.local_table[local]
                .const_time()
                .ok_or(Box::new(Error::UnexpectedOperationInConstContext(span))),
            _ => Err(Box::new(Error::UnexpectedOperationInConstContext(span))),
        }
    }

    fn runtime_expression_ctx(
        &mut self,
        span: Span,
    ) -> Result<'source, &mut LocalExpressionContext<'temp, 'out>> {
        match self.expr_type {
            ExpressionContextType::Runtime(ref mut ctx) => Ok(ctx),
            ExpressionContextType::Constant(_) | ExpressionContextType::Override => {
                Err(Box::new(Error::UnexpectedOperationInConstContext(span)))
            }
        }
    }

    fn with_nested_runtime_expression_ctx<'a, F, T>(
        &mut self,
        span: Span,
        f: F,
    ) -> Result<'source, (T, crate::Block)>
    where
        for<'t> F: FnOnce(&mut ExpressionContext<'source, 't, 't>) -> Result<'source, T>,
    {
        let mut block = crate::Block::new();
        let rctx = match self.expr_type {
            ExpressionContextType::Runtime(ref mut rctx) => Ok(rctx),
            ExpressionContextType::Constant(_) | ExpressionContextType::Override => {
                Err(Error::UnexpectedOperationInConstContext(span))
            }
        }?;

        rctx.block
            .extend(rctx.emitter.finish(&rctx.function.expressions));
        rctx.emitter.start(&rctx.function.expressions);

        let nested_rctx = LocalExpressionContext {
            local_table: rctx.local_table,
            function: rctx.function,
            block: &mut block,
            emitter: rctx.emitter,
            typifier: rctx.typifier,
            local_expression_kind_tracker: rctx.local_expression_kind_tracker,
        };
        let mut nested_ctx = ExpressionContext {
            enable_extensions: self.enable_extensions,
            expr_type: ExpressionContextType::Runtime(nested_rctx),
            ast_expressions: self.ast_expressions,
            globals: self.globals,
            module: self.module,
            const_typifier: self.const_typifier,
            layouter: self.layouter,
            global_expression_kind_tracker: self.global_expression_kind_tracker,
        };
        let ret = f(&mut nested_ctx)?;

        block.extend(rctx.emitter.finish(&rctx.function.expressions));
        rctx.emitter.start(&rctx.function.expressions);

        Ok((ret, block))
    }

    fn gather_component(
        &mut self,
        expr: Handle<ir::Expression>,
        component_span: Span,
        gather_span: Span,
    ) -> Result<'source, ir::SwizzleComponent> {
        match self.expr_type {
            ExpressionContextType::Runtime(ref rctx) => {
                if !rctx.local_expression_kind_tracker.is_const(expr) {
                    return Err(Box::new(Error::ExpectedConstExprConcreteIntegerScalar(
                        component_span,
                    )));
                }

                let index = self
                    .module
                    .to_ctx()
                    .get_const_val_from::<u32, _>(expr, &rctx.function.expressions)
                    .map_err(|err| match err {
                        proc::ConstValueError::NonConst | proc::ConstValueError::InvalidType => {
                            Error::ExpectedConstExprConcreteIntegerScalar(component_span)
                        }
                        proc::ConstValueError::Negative => {
                            Error::ExpectedNonNegative(component_span)
                        }
                    })?;
                ir::SwizzleComponent::XYZW
                    .get(index as usize)
                    .copied()
                    .ok_or(Box::new(Error::InvalidGatherComponent(component_span)))
            }
            // This means a `gather` operation appeared in a constant expression.
            // This error refers to the `gather` itself, not its "component" argument.
            ExpressionContextType::Constant(_) | ExpressionContextType::Override => Err(Box::new(
                Error::UnexpectedOperationInConstContext(gather_span),
            )),
        }
    }

    /// Determine the type of `handle`, and add it to the module's arena.
    ///
    /// If you just need a `TypeInner` for `handle`'s type, use the
    /// [`resolve_inner!`] macro instead. This function
    /// should only be used when the type of `handle` needs to appear
    /// in the module's final `Arena<Type>`, for example, if you're
    /// creating a [`LocalVariable`] whose type is inferred from its
    /// initializer.
    ///
    /// [`LocalVariable`]: ir::LocalVariable
    fn register_type(
        &mut self,
        handle: Handle<ir::Expression>,
    ) -> Result<'source, Handle<ir::Type>> {
        self.grow_types(handle)?;
        // This is equivalent to calling ExpressionContext::typifier(),
        // except that this lets the borrow checker see that it's okay
        // to also borrow self.module.types mutably below.
        let typifier = match self.expr_type {
            ExpressionContextType::Runtime(ref ctx)
            | ExpressionContextType::Constant(Some(ref ctx)) => ctx.typifier,
            ExpressionContextType::Constant(None) | ExpressionContextType::Override => {
                &*self.const_typifier
            }
        };
        Ok(typifier.register_type(handle, &mut self.module.types))
    }

    /// Resolve the types of all expressions up through `handle`.
    ///
    /// Ensure that [`self.typifier`] has a [`TypeResolution`] for
    /// every expression in `self.function.expressions`.
    ///
    /// This does not add types to any arena. The [`Typifier`]
    /// documentation explains the steps we take to avoid filling
    /// arenas with intermediate types.
    ///
    /// This function takes `&mut self`, so it can't conveniently
    /// return a shared reference to the resulting `TypeResolution`:
    /// the shared reference would extend the mutable borrow, and you
    /// wouldn't be able to use `self` for anything else. Instead, you
    /// should use [`register_type`] or one of [`resolve!`],
    /// [`resolve_inner!`] or [`resolve_inner_binary!`].
    ///
    /// [`self.typifier`]: ExpressionContext::typifier
    /// [`TypeResolution`]: proc::TypeResolution
    /// [`register_type`]: Self::register_type
    /// [`Typifier`]: Typifier
    fn grow_types(&mut self, handle: Handle<ir::Expression>) -> Result<'source, &mut Self> {
        let empty_arena = Arena::new();
        let resolve_ctx;
        let typifier;
        let expressions;
        match self.expr_type {
            ExpressionContextType::Runtime(ref mut ctx)
            | ExpressionContextType::Constant(Some(ref mut ctx)) => {
                resolve_ctx = proc::ResolveContext::with_locals(
                    self.module,
                    &ctx.function.local_variables,
                    &ctx.function.arguments,
                );
                typifier = &mut *ctx.typifier;
                expressions = &ctx.function.expressions;
            }
            ExpressionContextType::Constant(None) | ExpressionContextType::Override => {
                resolve_ctx = proc::ResolveContext::with_locals(self.module, &empty_arena, &[]);
                typifier = self.const_typifier;
                expressions = &self.module.global_expressions;
            }
        };
        typifier
            .grow(handle, expressions, &resolve_ctx)
            .map_err(Error::InvalidResolve)?;

        Ok(self)
    }

    fn image_data(
        &mut self,
        image: Handle<ir::Expression>,
        span: Span,
    ) -> Result<'source, (ir::ImageClass, bool)> {
        match *resolve_inner!(self, image) {
            ir::TypeInner::Image { class, arrayed, .. } => Ok((class, arrayed)),
            _ => Err(Box::new(Error::BadTexture(span))),
        }
    }

    fn prepare_args<'b>(
        &mut self,
        args: &'b [Handle<ast::Expression<'source>>],
        min_args: u32,
        span: Span,
    ) -> ArgumentContext<'b, 'source> {
        ArgumentContext {
            args: args.iter(),
            min_args,
            args_used: 0,
            total_args: args.len() as u32,
            span,
        }
    }

    /// Insert splats, if needed by the non-'*' operations.
    ///
    /// See the "Binary arithmetic expressions with mixed scalar and vector operands"
    /// table in the WebGPU Shading Language specification for relevant operators.
    ///
    /// Multiply is not handled here as backends are expected to handle vec*scalar
    /// operations, so inserting splats into the IR increases size needlessly.
    fn binary_op_splat(
        &mut self,
        op: ir::BinaryOperator,
        left: &mut Handle<ir::Expression>,
        right: &mut Handle<ir::Expression>,
    ) -> Result<'source, ()> {
        if matches!(
            op,
            ir::BinaryOperator::Add
                | ir::BinaryOperator::Subtract
                | ir::BinaryOperator::Divide
                | ir::BinaryOperator::Modulo
        ) {
            match resolve_inner_binary!(self, *left, *right) {
                (&ir::TypeInner::Vector { size, .. }, &ir::TypeInner::Scalar { .. }) => {
                    *right = self.append_expression(
                        ir::Expression::Splat {
                            size,
                            value: *right,
                        },
                        self.get_expression_span(*right),
                    )?;
                }
                (&ir::TypeInner::Scalar { .. }, &ir::TypeInner::Vector { size, .. }) => {
                    *left = self.append_expression(
                        ir::Expression::Splat { size, value: *left },
                        self.get_expression_span(*left),
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Add a single expression to the expression table that is not covered by `self.emitter`.
    ///
    /// This is useful for `CallResult` and `AtomicResult` expressions, which should not be covered by
    /// `Emit` statements.
    fn interrupt_emitter(
        &mut self,
        expression: ir::Expression,
        span: Span,
    ) -> Result<'source, Handle<ir::Expression>> {
        match self.expr_type {
            ExpressionContextType::Runtime(ref mut rctx)
            | ExpressionContextType::Constant(Some(ref mut rctx)) => {
                rctx.block
                    .extend(rctx.emitter.finish(&rctx.function.expressions));
            }
            ExpressionContextType::Constant(None) | ExpressionContextType::Override => {}
        }
        let result = self.append_expression(expression, span);
        match self.expr_type {
            ExpressionContextType::Runtime(ref mut rctx)
            | ExpressionContextType::Constant(Some(ref mut rctx)) => {
                rctx.emitter.start(&rctx.function.expressions);
            }
            ExpressionContextType::Constant(None) | ExpressionContextType::Override => {}
        }
        result
    }

    /// Apply the WGSL Load Rule to `expr`.
    ///
    /// If `expr` is has type `ref<SC, T, A>`, perform a load to produce a value of type
    /// `T`. Otherwise, return `expr` unchanged.
    fn apply_load_rule(
        &mut self,
        expr: Typed<Handle<ir::Expression>>,
    ) -> Result<'source, Handle<ir::Expression>> {
        match expr {
            Typed::Reference(pointer) => {
                let load = ir::Expression::Load { pointer };
                let span = self.get_expression_span(pointer);
                self.append_expression(load, span)
            }
            Typed::Plain(handle) => Ok(handle),
        }
    }

    fn ensure_type_exists(&mut self, inner: ir::TypeInner) -> Handle<ir::Type> {
        self.as_global().ensure_type_exists(None, inner)
    }

    /// Check that `expr` is an identifier resolving to a predeclared enumerant.
    ///
    /// The identifier must not have any template parameters.
    ///
    /// Return the name of the identifier, together with its span.
    ///
    /// Actually, this only checks that the identifier refers to some
    /// predeclared object, not necessarily an enumerant. This should be good
    /// enough, since the caller is going to compare the name against some list
    /// of permitted enumerants anyway.
    fn enumerant(
        &self,
        expr: Handle<ast::Expression<'source>>,
    ) -> Result<'source, (&'source str, Span)> {
        let span = self.ast_expressions.get_span(expr);
        let expr = &self.ast_expressions[expr];

        let ast::Expression::Ident(ref ident) = *expr else {
            return Err(Box::new(Error::UnexpectedExprForEnumerant(span)));
        };

        let ast::TemplateElaboratedIdent {
            ident: ast::IdentExpr::Unresolved(name),
            ref template_list,
            ..
        } = *ident
        else {
            return Err(Box::new(Error::UnexpectedIdentForEnumerant(span)));
        };

        if self.globals.get(name).is_some() {
            return Err(Box::new(Error::UnexpectedIdentForEnumerant(span)));
        }

        if !template_list.is_empty() {
            return Err(Box::new(Error::UnexpectedTemplate(span)));
        }

        Ok((name, span))
    }

    fn var_address_space(
        &self,
        template_list: &[Handle<ast::Expression<'source>>],
    ) -> Result<'source, ir::AddressSpace> {
        let mut tl = TemplateListIter::new(Span::UNDEFINED, template_list);
        let mut address_space = tl.maybe_address_space(self)?;
        if let Some(ref mut address_space) = address_space {
            tl.maybe_access_mode(address_space, self)?;
        }
        tl.finish(self)?;
        Ok(address_space.unwrap_or(ir::AddressSpace::Handle))
    }
}

struct ArgumentContext<'ctx, 'source> {
    args: core::slice::Iter<'ctx, Handle<ast::Expression<'source>>>,
    min_args: u32,
    args_used: u32,
    total_args: u32,
    span: Span,
}

impl<'source> ArgumentContext<'_, 'source> {
    pub fn finish(self) -> Result<'source, ()> {
        if self.args.len() == 0 {
            Ok(())
        } else {
            Err(Box::new(Error::WrongArgumentCount {
                found: self.total_args,
                expected: self.min_args..self.args_used + 1,
                span: self.span,
            }))
        }
    }

    pub fn next(&mut self) -> Result<'source, Handle<ast::Expression<'source>>> {
        match self.args.next().copied() {
            Some(arg) => {
                self.args_used += 1;
                Ok(arg)
            }
            None => Err(Box::new(Error::WrongArgumentCount {
                found: self.total_args,
                expected: self.min_args..self.args_used + 1,
                span: self.span,
            })),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Declared<T> {
    /// Value declared as const
    Const(T),

    /// Value declared as non-const
    Runtime(T),
}

impl<T> Declared<T> {
    fn runtime(self) -> T {
        match self {
            Declared::Const(t) | Declared::Runtime(t) => t,
        }
    }

    fn const_time(self) -> Option<T> {
        match self {
            Declared::Const(t) => Some(t),
            Declared::Runtime(_) => None,
        }
    }
}

/// WGSL type annotations on expressions, types, values, etc.
///
/// Naga and WGSL types are very close, but Naga lacks WGSL's `ref` types, which
/// we need to know to apply the Load Rule. This enum carries some WGSL or Naga
/// datum along with enough information to determine its corresponding WGSL
/// type.
///
/// The `T` type parameter can be any expression-like thing:
///
/// - `Typed<Handle<ir::Type>>` can represent a full WGSL type. For example,
///   given some Naga `Pointer` type `ptr`, a WGSL reference type is a
///   `Typed::Reference(ptr)` whereas a WGSL pointer type is a
///   `Typed::Plain(ptr)`.
///
/// - `Typed<ir::Expression>` or `Typed<Handle<ir::Expression>>` can
///   represent references similarly.
///
/// Use the `map` and `try_map` methods to convert from one expression
/// representation to another.
///
/// [`Expression`]: ir::Expression
#[derive(Debug, Copy, Clone)]
enum Typed<T> {
    /// A WGSL reference.
    Reference(T),

    /// A WGSL plain type.
    Plain(T),
}

impl<T> Typed<T> {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> Typed<U> {
        match self {
            Self::Reference(v) => Typed::Reference(f(v)),
            Self::Plain(v) => Typed::Plain(f(v)),
        }
    }

    fn try_map<U, E>(
        self,
        mut f: impl FnMut(T) -> core::result::Result<U, E>,
    ) -> core::result::Result<Typed<U>, E> {
        Ok(match self {
            Self::Reference(expr) => Typed::Reference(f(expr)?),
            Self::Plain(expr) => Typed::Plain(f(expr)?),
        })
    }

    fn ref_or<E>(self, error: E) -> core::result::Result<T, E> {
        match self {
            Self::Reference(v) => Ok(v),
            Self::Plain(_) => Err(error),
        }
    }
}

/// A single vector component or swizzle.
///
/// This represents the things that can appear after the `.` in a vector access
/// expression: either a single component name, or a series of them,
/// representing a swizzle.
enum Components {
    Single(u32),
    Swizzle {
        size: ir::VectorSize,
        pattern: [ir::SwizzleComponent; 4],
    },
}

impl Components {
    const fn letter_component(letter: char) -> Option<ir::SwizzleComponent> {
        use ir::SwizzleComponent as Sc;
        match letter {
            'x' | 'r' => Some(Sc::X),
            'y' | 'g' => Some(Sc::Y),
            'z' | 'b' => Some(Sc::Z),
            'w' | 'a' => Some(Sc::W),
            _ => None,
        }
    }

    fn single_component(name: &str, name_span: Span) -> Result<'_, u32> {
        let ch = name.chars().next().ok_or(Error::BadAccessor(name_span))?;
        match Self::letter_component(ch) {
            Some(sc) => Ok(sc as u32),
            None => Err(Box::new(Error::BadAccessor(name_span))),
        }
    }

    /// Construct a `Components` value from a 'member' name, like `"wzy"` or `"x"`.
    ///
    /// Use `name_span` for reporting errors in parsing the component string.
    fn new(name: &str, name_span: Span) -> Result<'_, Self> {
        let size = match name.len() {
            1 => return Ok(Components::Single(Self::single_component(name, name_span)?)),
            2 => ir::VectorSize::Bi,
            3 => ir::VectorSize::Tri,
            4 => ir::VectorSize::Quad,
            _ => return Err(Box::new(Error::BadAccessor(name_span))),
        };

        let mut pattern = [ir::SwizzleComponent::X; 4];
        for (comp, ch) in pattern.iter_mut().zip(name.chars()) {
            *comp = Self::letter_component(ch).ok_or(Error::BadAccessor(name_span))?;
        }

        if name.chars().all(|c| matches!(c, 'x' | 'y' | 'z' | 'w'))
            || name.chars().all(|c| matches!(c, 'r' | 'g' | 'b' | 'a'))
        {
            Ok(Components::Swizzle { size, pattern })
        } else {
            Err(Box::new(Error::BadAccessor(name_span)))
        }
    }
}

/// An `ast::GlobalDecl` for which we have built the Naga IR equivalent.
enum LoweredGlobalDecl {
    Function {
        handle: Handle<ir::Function>,
        must_use: bool,
    },
    Var(Handle<ir::GlobalVariable>),
    Const(Handle<ir::Constant>),
    Override(Handle<ir::Override>),
    Type(Handle<ir::Type>),
    EntryPoint(usize),
}

enum Texture {
    Gather,
    GatherCompare,

    Sample,
    SampleBias,
    SampleCompare,
    SampleCompareLevel,
    SampleGrad,
    SampleLevel,
    SampleBaseClampToEdge,
}

impl Texture {
    pub fn map(word: &str) -> Option<Self> {
        Some(match word {
            "textureGather" => Self::Gather,
            "textureGatherCompare" => Self::GatherCompare,

            "textureSample" => Self::Sample,
            "textureSampleBias" => Self::SampleBias,
            "textureSampleCompare" => Self::SampleCompare,
            "textureSampleCompareLevel" => Self::SampleCompareLevel,
            "textureSampleGrad" => Self::SampleGrad,
            "textureSampleLevel" => Self::SampleLevel,
            "textureSampleBaseClampToEdge" => Self::SampleBaseClampToEdge,
            _ => return None,
        })
    }

    pub const fn min_argument_count(&self) -> u32 {
        match *self {
            Self::Gather => 3,
            Self::GatherCompare => 4,

            Self::Sample => 3,
            Self::SampleBias => 5,
            Self::SampleCompare => 5,
            Self::SampleCompareLevel => 5,
            Self::SampleGrad => 6,
            Self::SampleLevel => 5,
            Self::SampleBaseClampToEdge => 3,
        }
    }
}

enum SubgroupGather {
    BroadcastFirst,
    Broadcast,
    Shuffle,
    ShuffleDown,
    ShuffleUp,
    ShuffleXor,
    QuadBroadcast,
}

impl SubgroupGather {
    pub fn map(word: &str) -> Option<Self> {
        Some(match word {
            "subgroupBroadcastFirst" => Self::BroadcastFirst,
            "subgroupBroadcast" => Self::Broadcast,
            "subgroupShuffle" => Self::Shuffle,
            "subgroupShuffleDown" => Self::ShuffleDown,
            "subgroupShuffleUp" => Self::ShuffleUp,
            "subgroupShuffleXor" => Self::ShuffleXor,
            "quadBroadcast" => Self::QuadBroadcast,
            _ => return None,
        })
    }
}

/// Whether a declaration accepts abstract types, or concretizes.
enum AbstractRule {
    /// This declaration concretizes its initialization expression.
    Concretize,

    /// This declaration can accept initializers with abstract types.
    Allow,
}

/// Whether `@must_use` applies to a call expression.
#[derive(Debug, Copy, Clone)]
enum MustUse {
    Yes,
    No,
}

impl From<bool> for MustUse {
    fn from(value: bool) -> Self {
        if value {
            MustUse::Yes
        } else {
            MustUse::No
        }
    }
}

pub struct Lowerer<'source, 'temp> {
    index: &'temp Index<'source>,
}

impl<'source, 'temp> Lowerer<'source, 'temp> {
    pub const fn new(index: &'temp Index<'source>) -> Self {
        Self { index }
    }

    pub fn lower(&mut self, tu: ast::TranslationUnit<'source>) -> Result<'source, ir::Module> {
        let mut module = ir::Module {
            diagnostic_filters: tu.diagnostic_filters,
            diagnostic_filter_leaf: tu.diagnostic_filter_leaf,
            ..Default::default()
        };

        let mut ctx = GlobalContext {
            enable_extensions: tu.enable_extensions,
            ast_expressions: &tu.expressions,
            globals: &mut FastHashMap::default(),
            module: &mut module,
            const_typifier: &mut Typifier::new(),
            layouter: &mut proc::Layouter::default(),
            global_expression_kind_tracker: &mut proc::ExpressionKindTracker::new(),
        };
        if !tu.doc_comments.is_empty() {
            ctx.module.get_or_insert_default_doc_comments().module =
                tu.doc_comments.iter().map(|s| s.to_string()).collect();
        }

        for decl_handle in self.index.visit_ordered() {
            let span = tu.decls.get_span(decl_handle);
            let decl = &tu.decls[decl_handle];

            match decl.kind {
                ast::GlobalDeclKind::Fn(ref f) => {
                    let lowered_decl = self.function(f, span, &mut ctx)?;
                    if !f.doc_comments.is_empty() {
                        match lowered_decl {
                            LoweredGlobalDecl::Function { handle, .. } => {
                                ctx.module
                                    .get_or_insert_default_doc_comments()
                                    .functions
                                    .insert(
                                        handle,
                                        f.doc_comments.iter().map(|s| s.to_string()).collect(),
                                    );
                            }
                            LoweredGlobalDecl::EntryPoint(index) => {
                                ctx.module
                                    .get_or_insert_default_doc_comments()
                                    .entry_points
                                    .insert(
                                        index,
                                        f.doc_comments.iter().map(|s| s.to_string()).collect(),
                                    );
                            }
                            _ => {}
                        }
                    }
                    ctx.globals.insert(f.name.name, lowered_decl);
                }
                ast::GlobalDeclKind::Var(ref v) => {
                    let explicit_ty =
                        v.ty.as_ref()
                            .map(|ast| self.resolve_ast_type(ast, &mut ctx.as_const()))
                            .transpose()?;

                    let (ty, initializer) = self.type_and_init(
                        v.name,
                        v.init,
                        explicit_ty,
                        AbstractRule::Concretize,
                        &mut ctx.as_override(),
                    )?;

                    let binding = if let Some(ref binding) = v.binding {
                        Some(ir::ResourceBinding {
                            group: self.const_u32(binding.group, &mut ctx.as_const())?.0,
                            binding: self.const_u32(binding.binding, &mut ctx.as_const())?.0,
                        })
                    } else {
                        None
                    };

                    let space = ctx.as_const().var_address_space(&v.template_list)?;

                    let handle = ctx.module.global_variables.append(
                        ir::GlobalVariable {
                            name: Some(v.name.name.to_string()),
                            space,
                            binding,
                            ty,
                            init: initializer,
                            memory_decorations: v.memory_decorations,
                        },
                        span,
                    );

                    if !v.doc_comments.is_empty() {
                        ctx.module
                            .get_or_insert_default_doc_comments()
                            .global_variables
                            .insert(
                                handle,
                                v.doc_comments.iter().map(|s| s.to_string()).collect(),
                            );
                    }
                    ctx.globals
                        .insert(v.name.name, LoweredGlobalDecl::Var(handle));
                }
                ast::GlobalDeclKind::Const(ref c) => {
                    let mut ectx = ctx.as_const();

                    let explicit_ty =
                        c.ty.as_ref()
                            .map(|ast| self.resolve_ast_type(ast, &mut ectx))
                            .transpose()?;

                    let (ty, init) = self.type_and_init(
                        c.name,
                        Some(c.init),
                        explicit_ty,
                        AbstractRule::Allow,
                        &mut ectx,
                    )?;
                    let init = init.expect("Global const must have init");

                    let handle = ctx.module.constants.append(
                        ir::Constant {
                            name: Some(c.name.name.to_string()),
                            ty,
                            init,
                        },
                        span,
                    );

                    ctx.globals
                        .insert(c.name.name, LoweredGlobalDecl::Const(handle));
                    if !c.doc_comments.is_empty() {
                        ctx.module
                            .get_or_insert_default_doc_comments()
                            .constants
                            .insert(
                                handle,
                                c.doc_comments.iter().map(|s| s.to_string()).collect(),
                            );
                    }
                }
                ast::GlobalDeclKind::Override(ref o) => {
                    let explicit_ty =
                        o.ty.as_ref()
                            .map(|ast| self.resolve_ast_type(ast, &mut ctx.as_const()))
                            .transpose()?;

                    let mut ectx = ctx.as_override();

                    let (ty, init) = self.type_and_init(
                        o.name,
                        o.init,
                        explicit_ty,
                        AbstractRule::Concretize,
                        &mut ectx,
                    )?;

                    let id =
                        o.id.map(|id| self.const_u32(id, &mut ctx.as_const()))
                            .transpose()?;

                    let id = if let Some((id, id_span)) = id {
                        Some(
                            u16::try_from(id)
                                .map_err(|_| Error::PipelineConstantIDValue(id_span))?,
                        )
                    } else {
                        None
                    };

                    let handle = ctx.module.overrides.append(
                        ir::Override {
                            name: Some(o.name.name.to_string()),
                            id,
                            ty,
                            init,
                        },
                        span,
                    );

                    ctx.globals
                        .insert(o.name.name, LoweredGlobalDecl::Override(handle));
                }
                ast::GlobalDeclKind::Struct(ref s) => {
                    let handle = self.r#struct(s, span, &mut ctx)?;
                    ctx.globals
                        .insert(s.name.name, LoweredGlobalDecl::Type(handle));
                    if !s.doc_comments.is_empty() {
                        ctx.module
                            .get_or_insert_default_doc_comments()
                            .types
                            .insert(
                                handle,
                                s.doc_comments.iter().map(|s| s.to_string()).collect(),
                            );
                    }
                }
                ast::GlobalDeclKind::Type(ref alias) => {
                    let ty = self.resolve_named_ast_type(
                        &alias.ty,
                        alias.name.name.to_string(),
                        &mut ctx.as_const(),
                    )?;
                    ctx.globals
                        .insert(alias.name.name, LoweredGlobalDecl::Type(ty));
                }
                ast::GlobalDeclKind::ConstAssert(condition) => {
                    let condition = self.expression(condition, &mut ctx.as_const())?;

                    let span = ctx.module.global_expressions.get_span(condition);
                    match ctx
                        .module
                        .to_ctx()
                        .get_const_val_from(condition, &ctx.module.global_expressions)
                    {
                        Ok(true) => Ok(()),
                        Ok(false) => Err(Error::ConstAssertFailed(span)),
                        Err(proc::ConstValueError::NonConst | proc::ConstValueError::Negative) => {
                            unreachable!()
                        }
                        Err(proc::ConstValueError::InvalidType) => Err(Error::NotBool(span)),
                    }?;
                }
            }
        }

        // Constant evaluation may leave abstract-typed literals and
        // compositions in expression arenas, so we need to compact the module
        // to remove unused expressions and types.
        crate::compact::compact(&mut module, KeepUnused::Yes);

        Ok(module)
    }

    /// Obtain (inferred) type and initializer after automatic conversion
    fn type_and_init(
        &mut self,
        name: ast::Ident<'source>,
        init: Option<Handle<ast::Expression<'source>>>,
        explicit_ty: Option<Handle<ir::Type>>,
        abstract_rule: AbstractRule,
        ectx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, (Handle<ir::Type>, Option<Handle<ir::Expression>>)> {
        let ty;
        let initializer;
        match (init, explicit_ty) {
            (Some(init), Some(explicit_ty)) => {
                let init = self.expression_for_abstract(init, ectx)?;
                let ty_res = proc::TypeResolution::Handle(explicit_ty);
                let init = ectx
                    .try_automatic_conversions(init, &ty_res, name.span)
                    .map_err(|error| match *error {
                        Error::AutoConversion(e) => Box::new(Error::InitializationTypeMismatch {
                            name: name.span,
                            expected: e.dest_type,
                            got: e.source_type,
                        }),
                        _ => error,
                    })?;

                let init_ty = ectx.register_type(init)?;
                if !ectx.module.compare_types(
                    &proc::TypeResolution::Handle(explicit_ty),
                    &proc::TypeResolution::Handle(init_ty),
                ) {
                    return Err(Box::new(Error::InitializationTypeMismatch {
                        name: name.span,
                        expected: ectx.type_to_string(explicit_ty),
                        got: ectx.type_to_string(init_ty),
                    }));
                }
                ty = explicit_ty;
                initializer = Some(init);
            }
            (Some(init), None) => {
                let mut init = self.expression_for_abstract(init, ectx)?;
                if let AbstractRule::Concretize = abstract_rule {
                    init = ectx.concretize(init)?;
                }
                ty = ectx.register_type(init)?;
                initializer = Some(init);
            }
            (None, Some(explicit_ty)) => {
                ty = explicit_ty;
                initializer = None;
            }
            (None, None) => return Err(Box::new(Error::DeclMissingTypeAndInit(name.span))),
        }
        Ok((ty, initializer))
    }

    fn function(
        &mut self,
        f: &ast::Function<'source>,
        span: Span,
        ctx: &mut GlobalContext<'source, '_, '_>,
    ) -> Result<'source, LoweredGlobalDecl> {
        let mut local_table = FastHashMap::default();
        let mut expressions = Arena::new();
        let mut named_expressions = FastIndexMap::default();
        let mut local_expression_kind_tracker = proc::ExpressionKindTracker::new();

        let arguments = f
            .arguments
            .iter()
            .enumerate()
            .map(|(i, arg)| -> Result<'_, _> {
                let ty = self.resolve_ast_type(&arg.ty, &mut ctx.as_const())?;
                let expr =
                    expressions.append(ir::Expression::FunctionArgument(i as u32), arg.name.span);
                local_table.insert(arg.handle, Declared::Runtime(Typed::Plain(expr)));
                named_expressions.insert(expr, (arg.name.name.to_string(), arg.name.span));
                local_expression_kind_tracker.insert(expr, proc::ExpressionKind::Runtime);

                Ok(ir::FunctionArgument {
                    name: Some(arg.name.name.to_string()),
                    ty,
                    binding: self.binding(&arg.binding, ty, ctx)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let result = f
            .result
            .as_ref()
            .map(|res| -> Result<'_, _> {
                let ty = self.resolve_ast_type(&res.ty, &mut ctx.as_const())?;
                Ok(ir::FunctionResult {
                    ty,
                    binding: self.binding(&res.binding, ty, ctx)?,
                })
            })
            .transpose()?;

        let mut function = ir::Function {
            name: Some(f.name.name.to_string()),
            arguments,
            result,
            local_variables: Arena::new(),
            expressions,
            named_expressions: crate::NamedExpressions::default(),
            body: ir::Block::default(),
            diagnostic_filter_leaf: f.diagnostic_filter_leaf,
        };

        let mut typifier = Typifier::default();
        let mut stmt_ctx = StatementContext {
            enable_extensions: ctx.enable_extensions,
            local_table: &mut local_table,
            globals: ctx.globals,
            ast_expressions: ctx.ast_expressions,
            const_typifier: ctx.const_typifier,
            typifier: &mut typifier,
            layouter: ctx.layouter,
            function: &mut function,
            named_expressions: &mut named_expressions,
            module: ctx.module,
            local_expression_kind_tracker: &mut local_expression_kind_tracker,
            global_expression_kind_tracker: ctx.global_expression_kind_tracker,
        };
        let mut body = self.block(&f.body, false, &mut stmt_ctx)?;
        proc::ensure_block_returns(&mut body);

        function.body = body;
        function.named_expressions = named_expressions
            .into_iter()
            .map(|(key, (name, _))| (key, name))
            .collect();

        if let Some(ref entry) = f.entry_point {
            let (workgroup_size, workgroup_size_overrides) =
                if let Some(workgroup_size) = entry.workgroup_size {
                    // TODO: replace with try_map once stabilized
                    let mut workgroup_size_out = [1; 3];
                    let mut workgroup_size_overrides_out = [None; 3];
                    for (i, size) in workgroup_size.into_iter().enumerate() {
                        if let Some(size_expr) = size {
                            match self.const_u32(size_expr, &mut ctx.as_const()) {
                                Ok(value) => {
                                    workgroup_size_out[i] = value.0;
                                }
                                Err(err) => {
                                    if let Error::ConstantEvaluatorError(ref ty, _) = *err {
                                        match **ty {
                                            proc::ConstantEvaluatorError::OverrideExpr => {
                                                workgroup_size_overrides_out[i] =
                                                    Some(self.workgroup_size_override(
                                                        size_expr,
                                                        &mut ctx.as_override(),
                                                    )?);
                                            }
                                            _ => {
                                                return Err(err);
                                            }
                                        }
                                    } else {
                                        return Err(err);
                                    }
                                }
                            }
                        }
                    }
                    if workgroup_size_overrides_out.iter().all(|x| x.is_none()) {
                        (workgroup_size_out, None)
                    } else {
                        (workgroup_size_out, Some(workgroup_size_overrides_out))
                    }
                } else {
                    ([0; 3], None)
                };

            let mesh_info = if let Some((var_name, var_span)) = entry.mesh_output_variable {
                let var = match ctx.globals.get(var_name) {
                    Some(&LoweredGlobalDecl::Var(handle)) => handle,
                    Some(_) => {
                        return Err(Box::new(Error::ExpectedGlobalVariable {
                            name_span: var_span,
                        }))
                    }
                    None => return Err(Box::new(Error::UnknownIdent(var_span, var_name))),
                };

                let mut info = ctx.module.analyze_mesh_shader_info(var);
                if let Some(h) = info.1[0] {
                    info.0.max_vertices_override = Some(
                        ctx.module
                            .global_expressions
                            .append(crate::Expression::Override(h), Span::UNDEFINED),
                    );
                }
                if let Some(h) = info.1[1] {
                    info.0.max_primitives_override = Some(
                        ctx.module
                            .global_expressions
                            .append(crate::Expression::Override(h), Span::UNDEFINED),
                    );
                }

                Some(info.0)
            } else {
                None
            };

            let task_payload = if let Some((var_name, var_span)) = entry.task_payload {
                Some(match ctx.globals.get(var_name) {
                    Some(&LoweredGlobalDecl::Var(handle)) => handle,
                    Some(_) => {
                        return Err(Box::new(Error::ExpectedGlobalVariable {
                            name_span: var_span,
                        }))
                    }
                    None => return Err(Box::new(Error::UnknownIdent(var_span, var_name))),
                })
            } else {
                None
            };

            let incoming_ray_payload =
                if let Some((var_name, var_span)) = entry.ray_incoming_payload {
                    Some(match ctx.globals.get(var_name) {
                        Some(&LoweredGlobalDecl::Var(handle)) => handle,
                        Some(_) => {
                            return Err(Box::new(Error::ExpectedGlobalVariable {
                                name_span: var_span,
                            }))
                        }
                        None => return Err(Box::new(Error::UnknownIdent(var_span, var_name))),
                    })
                } else {
                    None
                };

            ctx.module.entry_points.push(ir::EntryPoint {
                name: f.name.name.to_string(),
                stage: entry.stage,
                early_depth_test: entry.early_depth_test,
                workgroup_size,
                workgroup_size_overrides,
                function,
                mesh_info,
                task_payload,
                incoming_ray_payload,
            });
            Ok(LoweredGlobalDecl::EntryPoint(
                ctx.module.entry_points.len() - 1,
            ))
        } else {
            let handle = ctx.module.functions.append(function, span);
            Ok(LoweredGlobalDecl::Function {
                handle,
                must_use: f.result.as_ref().is_some_and(|res| res.must_use),
            })
        }
    }

    fn workgroup_size_override(
        &mut self,
        size_expr: Handle<ast::Expression<'source>>,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Handle<ir::Expression>> {
        let span = ctx.ast_expressions.get_span(size_expr);
        let expr = self.expression(size_expr, ctx)?;
        match resolve_inner!(ctx, expr).scalar_kind().ok_or(0) {
            Ok(ir::ScalarKind::Sint) | Ok(ir::ScalarKind::Uint) => Ok(expr),
            _ => Err(Box::new(Error::ExpectedConstExprConcreteIntegerScalar(
                span,
            ))),
        }
    }

    fn block(
        &mut self,
        b: &ast::Block<'source>,
        is_inside_loop: bool,
        ctx: &mut StatementContext<'source, '_, '_>,
    ) -> Result<'source, ir::Block> {
        let mut block = ir::Block::default();

        for stmt in b.stmts.iter() {
            self.statement(stmt, &mut block, is_inside_loop, ctx)?;
        }

        Ok(block)
    }

    fn statement(
        &mut self,
        stmt: &ast::Statement<'source>,
        block: &mut ir::Block,
        is_inside_loop: bool,
        ctx: &mut StatementContext<'source, '_, '_>,
    ) -> Result<'source, ()> {
        let out = match stmt.kind {
            ast::StatementKind::Block(ref block) => {
                let block = self.block(block, is_inside_loop, ctx)?;
                ir::Statement::Block(block)
            }
            ast::StatementKind::LocalDecl(ref decl) => match *decl {
                ast::LocalDecl::Let(ref l) => {
                    let mut emitter = proc::Emitter::default();
                    emitter.start(&ctx.function.expressions);

                    let explicit_ty = l
                        .ty
                        .as_ref()
                        .map(|ty| self.resolve_ast_type(ty, &mut ctx.as_const(block, &mut emitter)))
                        .transpose()?;

                    let mut ectx = ctx.as_expression(block, &mut emitter);

                    let (ty, initializer) = self.type_and_init(
                        l.name,
                        Some(l.init),
                        explicit_ty,
                        AbstractRule::Concretize,
                        &mut ectx,
                    )?;

                    // We have this special check here for `let` declarations because the
                    // validator doesn't check them (they are comingled with other things in
                    // `named_expressions`; see <https://github.com/gfx-rs/wgpu/issues/7393>).
                    // The check could go in `type_and_init`, but then we'd have to
                    // distinguish whether override-sized is allowed. The error ought to use
                    // the type's span, but `module.types.get_span(ty)` is `Span::UNDEFINED`
                    // (see <https://github.com/gfx-rs/wgpu/issues/7951>).
                    if ctx.module.types[ty]
                        .inner
                        .is_dynamically_sized(&ctx.module.types)
                    {
                        return Err(Box::new(Error::TypeNotConstructible(l.name.span)));
                    }

                    // We passed `Some()` to `type_and_init`, so we
                    // will get a lowered initializer expression back.
                    let initializer =
                        initializer.expect("type_and_init did not return an initializer");

                    // The WGSL spec says that any expression that refers to a
                    // `let`-bound variable is not a const expression. This
                    // affects when errors must be reported, so we can't even
                    // treat suitable `let` bindings as constant as an
                    // optimization.
                    ctx.local_expression_kind_tracker
                        .force_non_const(initializer);

                    block.extend(emitter.finish(&ctx.function.expressions));
                    ctx.local_table
                        .insert(l.handle, Declared::Runtime(Typed::Plain(initializer)));
                    ctx.named_expressions
                        .insert(initializer, (l.name.name.to_string(), l.name.span));

                    return Ok(());
                }
                ast::LocalDecl::Var(ref v) => {
                    let mut emitter = proc::Emitter::default();
                    emitter.start(&ctx.function.expressions);

                    let explicit_ty =
                        v.ty.as_ref()
                            .map(|ast| {
                                self.resolve_ast_type(ast, &mut ctx.as_const(block, &mut emitter))
                            })
                            .transpose()?;

                    let mut ectx = ctx.as_expression(block, &mut emitter);
                    let (ty, initializer) = self.type_and_init(
                        v.name,
                        v.init,
                        explicit_ty,
                        AbstractRule::Concretize,
                        &mut ectx,
                    )?;

                    let (const_initializer, initializer) = {
                        match initializer {
                            Some(init) => {
                                // It's not correct to hoist the initializer up
                                // to the top of the function if:
                                // - the initialization is inside a loop, and should
                                //   take place on every iteration, or
                                // - the initialization is not a constant
                                //   expression, so its value depends on the
                                //   state at the point of initialization.
                                if is_inside_loop
                                    || !ctx.local_expression_kind_tracker.is_const_or_override(init)
                                {
                                    (None, Some(init))
                                } else {
                                    (Some(init), None)
                                }
                            }
                            None => (None, None),
                        }
                    };

                    let var = ctx.function.local_variables.append(
                        ir::LocalVariable {
                            name: Some(v.name.name.to_string()),
                            ty,
                            init: const_initializer,
                        },
                        stmt.span,
                    );

                    let handle = ctx
                        .as_expression(block, &mut emitter)
                        .interrupt_emitter(ir::Expression::LocalVariable(var), Span::UNDEFINED)?;
                    block.extend(emitter.finish(&ctx.function.expressions));
                    ctx.local_table
                        .insert(v.handle, Declared::Runtime(Typed::Reference(handle)));

                    match initializer {
                        Some(initializer) => ir::Statement::Store {
                            pointer: handle,
                            value: initializer,
                        },
                        None => return Ok(()),
                    }
                }
                ast::LocalDecl::Const(ref c) => {
                    let mut emitter = proc::Emitter::default();
                    emitter.start(&ctx.function.expressions);

                    let ectx = &mut ctx.as_const(block, &mut emitter);

                    let explicit_ty =
                        c.ty.as_ref()
                            .map(|ast| self.resolve_ast_type(ast, &mut ectx.as_const()))
                            .transpose()?;

                    let (_ty, init) = self.type_and_init(
                        c.name,
                        Some(c.init),
                        explicit_ty,
                        AbstractRule::Allow,
                        &mut ectx.as_const(),
                    )?;
                    let init = init.expect("Local const must have init");

                    block.extend(emitter.finish(&ctx.function.expressions));
                    ctx.local_table
                        .insert(c.handle, Declared::Const(Typed::Plain(init)));
                    return Ok(());
                }
            },
            ast::StatementKind::If {
                condition,
                ref accept,
                ref reject,
            } => {
                let mut emitter = proc::Emitter::default();
                emitter.start(&ctx.function.expressions);

                let condition =
                    self.expression(condition, &mut ctx.as_expression(block, &mut emitter))?;
                block.extend(emitter.finish(&ctx.function.expressions));

                let accept = self.block(accept, is_inside_loop, ctx)?;
                let reject = self.block(reject, is_inside_loop, ctx)?;

                ir::Statement::If {
                    condition,
                    accept,
                    reject,
                }
            }
            ast::StatementKind::Switch {
                selector,
                ref cases,
            } => {
                let mut emitter = proc::Emitter::default();
                emitter.start(&ctx.function.expressions);

                let mut ectx = ctx.as_expression(block, &mut emitter);

                // Determine the scalar type of the selector and case expressions, find the
                // consensus type for automatic conversion, then convert them.
                let (mut exprs, spans) = core::iter::once(selector)
                    .chain(cases.iter().filter_map(|case| match case.value {
                        ast::SwitchValue::Expr(expr) => Some(expr),
                        ast::SwitchValue::Default => None,
                    }))
                    .enumerate()
                    .map(|(i, expr)| {
                        let span = ectx.ast_expressions.get_span(expr);
                        let expr = self.expression_for_abstract(expr, &mut ectx)?;
                        let ty = resolve_inner!(ectx, expr);
                        match *ty {
                            ir::TypeInner::Scalar(
                                ir::Scalar::I32 | ir::Scalar::U32 | ir::Scalar::ABSTRACT_INT,
                            ) => Ok((expr, span)),
                            _ => match i {
                                0 => Err(Box::new(Error::InvalidSwitchSelector { span })),
                                _ => Err(Box::new(Error::InvalidSwitchCase { span })),
                            },
                        }
                    })
                    .collect::<Result<(Vec<_>, Vec<_>)>>()?;

                let mut consensus =
                    ectx.automatic_conversion_consensus(None, &exprs)
                        .map_err(|span_idx| Error::SwitchCaseTypeMismatch {
                            span: spans[span_idx],
                        })?;
                // Concretize to I32 if the selector and all cases were abstract
                if consensus == ir::Scalar::ABSTRACT_INT {
                    consensus = ir::Scalar::I32;
                }
                for expr in &mut exprs {
                    ectx.convert_to_leaf_scalar(expr, consensus)?;
                }

                block.extend(emitter.finish(&ctx.function.expressions));

                let mut exprs = exprs.into_iter();
                let selector = exprs
                    .next()
                    .expect("First element should be selector expression");

                let cases = cases
                    .iter()
                    .map(|case| {
                        Ok(ir::SwitchCase {
                            value: match case.value {
                                ast::SwitchValue::Expr(expr) => {
                                    let span = ctx.ast_expressions.get_span(expr);
                                    let expr = exprs.next().expect(
                                        "Should yield expression for each SwitchValue::Expr case",
                                    );
                                    match ctx
                                        .module
                                        .to_ctx()
                                        .get_const_val_from(expr, &ctx.function.expressions)
                                    {
                                        Ok(ir::Literal::I32(value)) => ir::SwitchValue::I32(value),
                                        Ok(ir::Literal::U32(value)) => ir::SwitchValue::U32(value),
                                        _ => {
                                            return Err(Box::new(Error::InvalidSwitchCase {
                                                span,
                                            }));
                                        }
                                    }
                                }
                                ast::SwitchValue::Default => ir::SwitchValue::Default,
                            },
                            body: self.block(&case.body, is_inside_loop, ctx)?,
                            fall_through: case.fall_through,
                        })
                    })
                    .collect::<Result<_>>()?;

                ir::Statement::Switch { selector, cases }
            }
            ast::StatementKind::Loop {
                ref body,
                ref continuing,
                break_if,
            } => {
                let body = self.block(body, true, ctx)?;
                let mut continuing = self.block(continuing, true, ctx)?;

                let mut emitter = proc::Emitter::default();
                emitter.start(&ctx.function.expressions);
                let break_if = break_if
                    .map(|expr| {
                        self.expression(expr, &mut ctx.as_expression(&mut continuing, &mut emitter))
                    })
                    .transpose()?;
                continuing.extend(emitter.finish(&ctx.function.expressions));

                ir::Statement::Loop {
                    body,
                    continuing,
                    break_if,
                }
            }
            ast::StatementKind::Break => ir::Statement::Break,
            ast::StatementKind::Continue => ir::Statement::Continue,
            ast::StatementKind::Return { value: ast_value } => {
                let mut emitter = proc::Emitter::default();
                emitter.start(&ctx.function.expressions);

                let value;
                if let Some(ast_expr) = ast_value {
                    let result_ty = ctx.function.result.as_ref().map(|r| r.ty);
                    let mut ectx = ctx.as_expression(block, &mut emitter);
                    let expr = self.expression_for_abstract(ast_expr, &mut ectx)?;

                    if let Some(result_ty) = result_ty {
                        let mut ectx = ctx.as_expression(block, &mut emitter);
                        let resolution = proc::TypeResolution::Handle(result_ty);
                        let converted =
                            ectx.try_automatic_conversions(expr, &resolution, Span::default())?;
                        value = Some(converted);
                    } else {
                        value = Some(expr);
                    }
                } else {
                    value = None;
                }
                block.extend(emitter.finish(&ctx.function.expressions));

                ir::Statement::Return { value }
            }
            ast::StatementKind::Kill => ir::Statement::Kill,
            ast::StatementKind::Call(ref call_phrase) => {
                let mut emitter = proc::Emitter::default();
                emitter.start(&ctx.function.expressions);

                let _ = self.call(
                    call_phrase,
                    stmt.span,
                    &mut ctx.as_expression(block, &mut emitter),
                    true,
                )?;
                block.extend(emitter.finish(&ctx.function.expressions));
                return Ok(());
            }
            ast::StatementKind::Assign {
                target: ast_target,
                op,
                value,
            } => {
                let mut emitter = proc::Emitter::default();
                emitter.start(&ctx.function.expressions);
                let target_span = ctx.ast_expressions.get_span(ast_target);

                let mut ectx = ctx.as_expression(block, &mut emitter);
                let target = self.expression_for_reference(ast_target, &mut ectx)?;
                let target_handle = match target {
                    Typed::Reference(handle) => handle,
                    Typed::Plain(handle) => {
                        let ty = ctx.invalid_assignment_type(handle);
                        return Err(Box::new(Error::InvalidAssignment {
                            span: target_span,
                            ty,
                        }));
                    }
                };

                // Usually the value needs to be converted to match the type of
                // the memory view you're assigning it to. The bit shift
                // operators are exceptions, in that the right operand is always
                // a `u32` or `vecN<u32>`.
                let target_scalar = match op {
                    Some(ir::BinaryOperator::ShiftLeft | ir::BinaryOperator::ShiftRight) => {
                        Some(ir::Scalar::U32)
                    }
                    _ => resolve_inner!(ectx, target_handle)
                        .pointer_automatically_convertible_scalar(&ectx.module.types),
                };

                // Need to emit the LHS _before_ the RHS so that it is evaluated first.
                let op_assign = if let Some(op) = op {
                    Some((op, ectx.apply_load_rule(target)?))
                } else {
                    None
                };

                let value = self.expression_for_abstract(value, &mut ectx)?;
                let mut value = match target_scalar {
                    Some(target_scalar) => ectx.try_automatic_conversion_for_leaf_scalar(
                        value,
                        target_scalar,
                        target_span,
                    )?,
                    None => value,
                };

                let value = match op_assign {
                    Some((op, mut left)) => {
                        ectx.binary_op_splat(op, &mut left, &mut value)?;
                        ectx.append_expression(
                            ir::Expression::Binary {
                                op,
                                left,
                                right: value,
                            },
                            stmt.span,
                        )?
                    }
                    None => value,
                };
                block.extend(emitter.finish(&ctx.function.expressions));

                ir::Statement::Store {
                    pointer: target_handle,
                    value,
                }
            }
            ast::StatementKind::Increment(value) | ast::StatementKind::Decrement(value) => {
                let mut emitter = proc::Emitter::default();
                emitter.start(&ctx.function.expressions);

                let op = match stmt.kind {
                    ast::StatementKind::Increment(_) => ir::BinaryOperator::Add,
                    ast::StatementKind::Decrement(_) => ir::BinaryOperator::Subtract,
                    _ => unreachable!(),
                };

                let value_span = ctx.ast_expressions.get_span(value);
                let target = self
                    .expression_for_reference(value, &mut ctx.as_expression(block, &mut emitter))?;
                let target_handle = target.ref_or(Error::BadIncrDecrReferenceType(value_span))?;

                let mut ectx = ctx.as_expression(block, &mut emitter);
                let scalar = match *resolve_inner!(ectx, target_handle) {
                    ir::TypeInner::ValuePointer {
                        size: None, scalar, ..
                    } => scalar,
                    ir::TypeInner::Pointer { base, .. } => match ectx.module.types[base].inner {
                        ir::TypeInner::Scalar(scalar) => scalar,
                        _ => return Err(Box::new(Error::BadIncrDecrReferenceType(value_span))),
                    },
                    _ => return Err(Box::new(Error::BadIncrDecrReferenceType(value_span))),
                };
                let literal = match scalar.kind {
                    ir::ScalarKind::Sint | ir::ScalarKind::Uint => ir::Literal::one(scalar)
                        .ok_or(Error::BadIncrDecrReferenceType(value_span))?,
                    _ => return Err(Box::new(Error::BadIncrDecrReferenceType(value_span))),
                };

                let right =
                    ectx.interrupt_emitter(ir::Expression::Literal(literal), Span::UNDEFINED)?;
                let rctx = ectx.runtime_expression_ctx(stmt.span)?;
                let left = rctx.function.expressions.append(
                    ir::Expression::Load {
                        pointer: target_handle,
                    },
                    value_span,
                );
                let value = rctx
                    .function
                    .expressions
                    .append(ir::Expression::Binary { op, left, right }, stmt.span);
                rctx.local_expression_kind_tracker
                    .insert(left, proc::ExpressionKind::Runtime);
                rctx.local_expression_kind_tracker
                    .insert(value, proc::ExpressionKind::Runtime);

                block.extend(emitter.finish(&ctx.function.expressions));
                ir::Statement::Store {
                    pointer: target_handle,
                    value,
                }
            }
            ast::StatementKind::ConstAssert(condition) => {
                let mut emitter = proc::Emitter::default();
                emitter.start(&ctx.function.expressions);

                let condition =
                    self.expression(condition, &mut ctx.as_const(block, &mut emitter))?;

                let span = ctx.function.expressions.get_span(condition);
                match ctx
                    .module
                    .to_ctx()
                    .get_const_val_from(condition, &ctx.function.expressions)
                {
                    Ok(true) => Ok(()),
                    Ok(false) => Err(Error::ConstAssertFailed(span)),
                    Err(proc::ConstValueError::NonConst | proc::ConstValueError::Negative) => {
                        unreachable!()
                    }
                    Err(proc::ConstValueError::InvalidType) => Err(Error::NotBool(span)),
                }?;

                block.extend(emitter.finish(&ctx.function.expressions));

                return Ok(());
            }
            ast::StatementKind::Phony(expr) => {
                // Remembered the RHS of the phony assignment as a named expression. This
                // is important (1) to preserve the RHS for validation, (2) to track any
                // referenced globals.
                let mut emitter = proc::Emitter::default();
                emitter.start(&ctx.function.expressions);

                let value = self.expression(expr, &mut ctx.as_expression(block, &mut emitter))?;
                block.extend(emitter.finish(&ctx.function.expressions));
                ctx.named_expressions
                    .insert(value, ("phony".to_string(), stmt.span));
                return Ok(());
            }
        };

        block.push(out, stmt.span);

        Ok(())
    }

    /// Lower `expr` and apply the Load Rule if possible.
    ///
    /// For the time being, this concretizes abstract values, to support
    /// consumers that haven't been adapted to consume them yet. Consumers
    /// prepared for abstract values can call [`expression_for_abstract`].
    ///
    /// [`expression_for_abstract`]: Lowerer::expression_for_abstract
    fn expression(
        &mut self,
        expr: Handle<ast::Expression<'source>>,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Handle<ir::Expression>> {
        let expr = self.expression_for_abstract(expr, ctx)?;
        ctx.concretize(expr)
    }

    fn expression_for_abstract(
        &mut self,
        expr: Handle<ast::Expression<'source>>,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Handle<ir::Expression>> {
        let expr = self.expression_for_reference(expr, ctx)?;
        ctx.apply_load_rule(expr)
    }

    fn expression_with_leaf_scalar(
        &mut self,
        expr: Handle<ast::Expression<'source>>,
        scalar: ir::Scalar,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Handle<ir::Expression>> {
        let unconverted = self.expression_for_abstract(expr, ctx)?;
        ctx.try_automatic_conversion_for_leaf_scalar(unconverted, scalar, Span::default())
    }

    fn expression_for_reference(
        &mut self,
        expr: Handle<ast::Expression<'source>>,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Typed<Handle<ir::Expression>>> {
        let span = ctx.ast_expressions.get_span(expr);
        let expr = &ctx.ast_expressions[expr];

        let expr: Typed<ir::Expression> = match *expr {
            ast::Expression::Literal(literal) => {
                let literal = match literal {
                    ast::Literal::Number(Number::F16(f)) => ir::Literal::F16(f),
                    ast::Literal::Number(Number::F32(f)) => ir::Literal::F32(f),
                    ast::Literal::Number(Number::I32(i)) => ir::Literal::I32(i),
                    ast::Literal::Number(Number::U32(u)) => ir::Literal::U32(u),
                    ast::Literal::Number(Number::I64(i)) => ir::Literal::I64(i),
                    ast::Literal::Number(Number::U64(u)) => ir::Literal::U64(u),
                    ast::Literal::Number(Number::F64(f)) => ir::Literal::F64(f),
                    ast::Literal::Number(Number::AbstractInt(i)) => ir::Literal::AbstractInt(i),
                    ast::Literal::Number(Number::AbstractFloat(f)) => ir::Literal::AbstractFloat(f),
                    ast::Literal::Bool(b) => ir::Literal::Bool(b),
                };
                let handle = ctx.interrupt_emitter(ir::Expression::Literal(literal), span)?;
                return Ok(Typed::Plain(handle));
            }
            ast::Expression::Ident(ast::TemplateElaboratedIdent {
                ref template_list, ..
            }) if !template_list.is_empty() => {
                return Err(Box::new(Error::UnexpectedTemplate(span)))
            }
            ast::Expression::Ident(ast::TemplateElaboratedIdent {
                ident: ast::IdentExpr::Local(local),
                ..
            }) => {
                return ctx.local(&local, span);
            }
            ast::Expression::Ident(ast::TemplateElaboratedIdent {
                ident: ast::IdentExpr::Unresolved(name),
                ..
            }) => {
                let global = ctx
                    .globals
                    .get(name)
                    .ok_or(Error::UnknownIdent(span, name))?;
                let expr = match *global {
                    LoweredGlobalDecl::Var(handle) => {
                        let expr = ir::Expression::GlobalVariable(handle);
                        let v = &ctx.module.global_variables[handle];
                        match v.space {
                            ir::AddressSpace::Handle => Typed::Plain(expr),
                            _ => Typed::Reference(expr),
                        }
                    }
                    LoweredGlobalDecl::Const(handle) => {
                        Typed::Plain(ir::Expression::Constant(handle))
                    }
                    LoweredGlobalDecl::Override(handle) => {
                        Typed::Plain(ir::Expression::Override(handle))
                    }
                    LoweredGlobalDecl::Function { .. }
                    | LoweredGlobalDecl::Type(_)
                    | LoweredGlobalDecl::EntryPoint(_) => {
                        return Err(Box::new(Error::Unexpected(span, ExpectedToken::Variable)));
                    }
                };

                return expr.try_map(|handle| ctx.interrupt_emitter(handle, span));
            }
            ast::Expression::Unary { op, expr } => {
                let expr = self.expression_for_abstract(expr, ctx)?;
                Typed::Plain(ir::Expression::Unary { op, expr })
            }
            ast::Expression::AddrOf(expr) => {
                // The `&` operator simply converts a reference to a pointer. And since a
                // reference is required, the Load Rule is not applied.
                match self.expression_for_reference(expr, ctx)? {
                    Typed::Reference(handle) => {
                        let expr = &ctx.runtime_expression_ctx(span)?.function.expressions[handle];
                        if let &ir::Expression::Access { base, .. }
                        | &ir::Expression::AccessIndex { base, .. } = expr
                        {
                            if let Some(ty) = resolve_inner!(ctx, base).pointer_base_type() {
                                if matches!(
                                    *ty.inner_with(&ctx.module.types),
                                    ir::TypeInner::Vector { .. },
                                ) {
                                    return Err(Box::new(Error::InvalidAddrOfOperand(
                                        ctx.get_expression_span(handle),
                                    )));
                                }
                            }
                        }
                        // No code is generated. We just declare the reference a pointer now.
                        return Ok(Typed::Plain(handle));
                    }
                    Typed::Plain(_) => {
                        return Err(Box::new(Error::NotReference(
                            "the operand of the `&` operator",
                            span,
                        )));
                    }
                }
            }
            ast::Expression::Deref(expr) => {
                // The pointer we dereference must be loaded.
                let pointer = self.expression(expr, ctx)?;

                if resolve_inner!(ctx, pointer).pointer_space().is_none() {
                    return Err(Box::new(Error::NotPointer(span)));
                }

                // No code is generated. We just declare the pointer a reference now.
                return Ok(Typed::Reference(pointer));
            }
            ast::Expression::Binary { op, left, right } => {
                self.binary(op, left, right, span, ctx)?
            }
            ast::Expression::Call(ref call_phrase) => {
                let handle = self
                    .call(call_phrase, span, ctx, false)?
                    .ok_or(Error::FunctionReturnsVoid(span))?;
                return Ok(Typed::Plain(handle));
            }
            ast::Expression::Index { base, index } => {
                let mut lowered_base = self.expression_for_reference(base, ctx)?;
                let index = self.expression(index, ctx)?;

                // <https://www.w3.org/TR/WGSL/#language_extension-pointer_composite_access>
                // Declare pointer as reference
                if let Typed::Plain(handle) = lowered_base {
                    if resolve_inner!(ctx, handle).pointer_space().is_some() {
                        lowered_base = Typed::Reference(handle);
                    }
                }

                lowered_base.try_map(|base| match ctx.get_const_val(index).ok() {
                    Some(index) => Ok::<_, Box<Error>>(ir::Expression::AccessIndex { base, index }),
                    None => {
                        // When an abstract array value e is indexed by an expression
                        // that is not a const-expression, then the array is concretized
                        // before the index is applied.
                        // https://www.w3.org/TR/WGSL/#array-access-expr
                        // Also applies to vectors and matrices.
                        let base = ctx.concretize(base)?;
                        Ok(ir::Expression::Access { base, index })
                    }
                })?
            }
            ast::Expression::Member { base, ref field } => {
                let mut lowered_base = self.expression_for_reference(base, ctx)?;

                // <https://www.w3.org/TR/WGSL/#language_extension-pointer_composite_access>
                // Declare pointer as reference
                if let Typed::Plain(handle) = lowered_base {
                    if resolve_inner!(ctx, handle).pointer_space().is_some() {
                        lowered_base = Typed::Reference(handle);
                    }
                }

                let temp_ty;
                let composite_type: &ir::TypeInner = match lowered_base {
                    Typed::Reference(handle) => {
                        temp_ty = resolve_inner!(ctx, handle)
                            .pointer_base_type()
                            .expect("In Typed::Reference(handle), handle must be a Naga pointer");
                        temp_ty.inner_with(&ctx.module.types)
                    }

                    Typed::Plain(handle) => {
                        resolve_inner!(ctx, handle)
                    }
                };

                let access = match *composite_type {
                    ir::TypeInner::Struct { ref members, .. } => {
                        let index = members
                            .iter()
                            .position(|m| m.name.as_deref() == Some(field.name))
                            .ok_or(Error::BadAccessor(field.span))?
                            as u32;

                        lowered_base.map(|base| ir::Expression::AccessIndex { base, index })
                    }
                    ir::TypeInner::Vector { size: vec_size, .. } => {
                        match Components::new(field.name, field.span)? {
                            Components::Swizzle { size, pattern } => {
                                for &component in pattern[..size as usize].iter() {
                                    if component as u8 >= vec_size as u8 {
                                        return Err(Box::new(Error::BadAccessor(field.span)));
                                    }
                                }
                                Typed::Plain(ir::Expression::Swizzle {
                                    size,
                                    vector: ctx.apply_load_rule(lowered_base)?,
                                    pattern,
                                })
                            }
                            Components::Single(index) => {
                                if index >= vec_size as u32 {
                                    return Err(Box::new(Error::BadAccessor(field.span)));
                                }
                                lowered_base.map(|base| ir::Expression::AccessIndex { base, index })
                            }
                        }
                    }
                    _ => return Err(Box::new(Error::BadAccessor(field.span))),
                };

                access
            }
        };

        expr.try_map(|handle| ctx.append_expression(handle, span))
    }

    /// Generate IR for the short-circuiting operators `&&` and `||`.
    ///
    /// `binary` has already lowered the LHS expression and resolved its type.
    fn logical(
        &mut self,
        op: crate::BinaryOperator,
        left: Handle<crate::Expression>,
        right: Handle<ast::Expression<'source>>,
        span: Span,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Typed<crate::Expression>> {
        debug_assert!(
            op == crate::BinaryOperator::LogicalAnd || op == crate::BinaryOperator::LogicalOr
        );

        if ctx.is_runtime() {
            // To simulate short-circuiting behavior, we want to generate IR
            // like the following for `&&`. For `||`, the condition is `!_lhs`
            // and the else value is `true`.
            //
            // var _e0: bool;
            // if _lhs {
            //     _e0 = _rhs;
            // } else {
            //     _e0 = false;
            // }

            let (condition, else_val) = if op == crate::BinaryOperator::LogicalAnd {
                let condition = left;
                let else_val = ctx.append_expression(
                    crate::Expression::Literal(crate::Literal::Bool(false)),
                    span,
                )?;
                (condition, else_val)
            } else {
                let condition = ctx.append_expression(
                    crate::Expression::Unary {
                        op: crate::UnaryOperator::LogicalNot,
                        expr: left,
                    },
                    span,
                )?;
                let else_val = ctx.append_expression(
                    crate::Expression::Literal(crate::Literal::Bool(true)),
                    span,
                )?;
                (condition, else_val)
            };

            let bool_ty = ctx.ensure_type_exists(crate::TypeInner::Scalar(crate::Scalar::BOOL));

            let rctx = ctx.runtime_expression_ctx(span)?;
            let result_var = rctx.function.local_variables.append(
                crate::LocalVariable {
                    name: None,
                    ty: bool_ty,
                    init: None,
                },
                span,
            );
            let pointer =
                ctx.append_expression(crate::Expression::LocalVariable(result_var), span)?;

            let (right, mut accept) = ctx.with_nested_runtime_expression_ctx(span, |ctx| {
                let right = self.expression_for_abstract(right, ctx)?;
                ctx.grow_types(right)?;
                Ok(right)
            })?;

            accept.push(
                crate::Statement::Store {
                    pointer,
                    value: right,
                },
                span,
            );

            let mut reject = crate::Block::with_capacity(1);
            reject.push(
                crate::Statement::Store {
                    pointer,
                    value: else_val,
                },
                span,
            );

            let rctx = ctx.runtime_expression_ctx(span)?;
            rctx.block.push(
                crate::Statement::If {
                    condition,
                    accept,
                    reject,
                },
                span,
            );

            Ok(Typed::Reference(crate::Expression::LocalVariable(
                result_var,
            )))
        } else {
            let left_val: Option<bool> = ctx.get_const_val(left).ok();

            if left_val.is_some_and(|left_val| {
                op == crate::BinaryOperator::LogicalAnd && !left_val
                    || op == crate::BinaryOperator::LogicalOr && left_val
            }) {
                // Short-circuit behavior: don't evaluate the RHS.

                // TODO(https://github.com/gfx-rs/wgpu/issues/8440): We shouldn't ignore the
                // RHS completely, it should still be type-checked. Preserving it for type
                // checking is a bit tricky, because we're trying to produce an expression
                // for a const context, but the RHS is allowed to have things that aren't
                // const.

                Ok(Typed::Plain(ctx.get(left).clone()))
            } else {
                // Evaluate the RHS and construct the entire binary expression as we
                // normally would. This case applies to well-formed constant logical
                // expressions that don't short-circuit (handled by the constant evaluator
                // shortly), to override expressions (handled when overrides are processed)
                // and to non-well-formed expressions (rejected by type checking).
                let right = self.expression_for_abstract(right, ctx)?;
                ctx.grow_types(right)?;

                Ok(Typed::Plain(crate::Expression::Binary { op, left, right }))
            }
        }
    }

    fn type_expression(
        &mut self,
        expr: Handle<ast::Expression<'source>>,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Handle<ir::Type>> {
        let span = ctx.ast_expressions.get_span(expr);
        let expr = &ctx.ast_expressions[expr];

        let ident = match *expr {
            ast::Expression::Ident(ref ident) => ident,
            _ => return Err(Box::new(Error::UnexpectedExprForTypeExpression(span))),
        };

        self.type_specifier(ident, ctx, None)
    }

    fn type_specifier(
        &mut self,
        ident: &ast::TemplateElaboratedIdent<'source>,
        ctx: &mut ExpressionContext<'source, '_, '_>,
        alias_name: Option<String>,
    ) -> Result<'source, Handle<ir::Type>> {
        let &ast::TemplateElaboratedIdent {
            ref ident,
            ident_span,
            ref template_list,
            ..
        } = ident;

        let ident = match *ident {
            ast::IdentExpr::Unresolved(ident) => ident,
            ast::IdentExpr::Local(_) => {
                // Since WGSL only supports module-scope type definitions and
                // aliases, a local identifier can't possibly refer to a type.
                return Err(Box::new(Error::UnexpectedExprForTypeExpression(ident_span)));
            }
        };

        let mut tl = TemplateListIter::new(ident_span, template_list);

        if let Some(global) = ctx.globals.get(ident) {
            let &LoweredGlobalDecl::Type(handle) = global else {
                return Err(Box::new(Error::UnexpectedExprForTypeExpression(ident_span)));
            };

            // Type generators can only be predeclared, so since `ident` refers
            // to a module-scope declaration, the template parameter list should
            // be empty.
            tl.finish(ctx)?;
            return Ok(handle);
        }

        // If `ident` doesn't resolve to a module-scope declaration, then it
        // must resolve to a predeclared type or type generator.
        let ty = conv::map_predeclared_type(&ctx.enable_extensions, ident_span, ident)?
            .ok_or_else(|| Box::new(Error::UnknownIdent(ident_span, ident)))?;
        let ty = self.finalize_type(ctx, ty, &mut tl, alias_name)?;

        tl.finish(ctx)?;

        Ok(ty)
    }

    /// Construct an [`ir::Type`] from a [`conv::PredeclaredType`] and a list of
    /// template parameters.
    ///
    /// If we're processing a type alias, then `alias_name` is the name we
    /// should use in the new `ir::Type`.
    ///
    /// For example, when parsing `vec3<f32>`, the caller would pass:
    ///
    /// - for `ty`, [`TypeGenerator::Vector`], and
    ///
    /// - for `tl`, an iterator producing a single [`Expression::Ident`] representing `f32`.
    ///
    /// From those arguments this function will return a handle for the
    /// [`ir::Type`] representing `vec3<f32>`.
    ///
    /// [`TypeGenerator::Vector`]: conv::TypeGenerator::Vector
    /// [`Expression::Ident`]: crate::front::wgsl::parse::ast::Expression::Ident
    fn finalize_type(
        &mut self,
        ctx: &mut ExpressionContext<'source, '_, '_>,
        ty: conv::PredeclaredType,
        tl: &mut TemplateListIter<'_, 'source>,
        alias_name: Option<String>,
    ) -> Result<'source, Handle<ir::Type>> {
        let ty = match ty {
            conv::PredeclaredType::TypeInner(ty_inner) => {
                if let ir::TypeInner::Image {
                    class: ir::ImageClass::External,
                    ..
                } = ty_inner
                {
                    // Other than the WGSL backend, every backend that supports
                    // external textures does so by lowering them to a set of
                    // ordinary textures and some parameters saying how to
                    // sample from them. We don't know which backend will
                    // consume the `Module` we're building, but in case it's not
                    // WGSL, populate `SpecialTypes::external_texture_params`
                    // and `SpecialTypes::external_texture_transfer_function`
                    // with the types the backend will use for the parameter
                    // buffer.
                    //
                    // Neither of these are the type we are lowering here:
                    // that's an ordinary `TypeInner::Image`. But the fact we
                    // are lowering a `texture_external` implies the backends
                    // may need these additional types too.
                    ctx.module.generate_external_texture_types();
                }

                ctx.as_global().ensure_type_exists(alias_name, ty_inner)
            }
            conv::PredeclaredType::RayDesc => ctx.module.generate_ray_desc_type(),
            conv::PredeclaredType::RayIntersection => ctx.module.generate_ray_intersection_type(),
            conv::PredeclaredType::TypeGenerator(type_generator) => {
                let ty_inner = match type_generator {
                    conv::TypeGenerator::Vector { size } => {
                        let (scalar, _) = tl.scalar_ty(self, ctx)?;
                        ir::TypeInner::Vector { size, scalar }
                    }
                    conv::TypeGenerator::Matrix { columns, rows } => {
                        let (scalar, span) = tl.scalar_ty(self, ctx)?;
                        if scalar.kind != ir::ScalarKind::Float {
                            return Err(Box::new(Error::BadMatrixScalarKind(span, scalar)));
                        }
                        ir::TypeInner::Matrix {
                            columns,
                            rows,
                            scalar,
                        }
                    }
                    conv::TypeGenerator::Array => {
                        let base = tl.ty(self, ctx)?;
                        let size = tl.maybe_array_size(self, ctx)?;

                        // Determine the size of the base type, if needed.
                        ctx.layouter.update(ctx.module.to_ctx()).map_err(|err| {
                            let LayoutErrorInner::TooLarge = err.inner else {
                                unreachable!("unexpected layout error: {err:?}");
                            };
                            // Lots of type definitions don't get spans, so this error
                            // message may not be very useful.
                            Box::new(Error::TypeTooLarge {
                                span: ctx.module.types.get_span(err.ty),
                            })
                        })?;
                        let stride = ctx.layouter[base].to_stride();

                        ir::TypeInner::Array { base, size, stride }
                    }
                    conv::TypeGenerator::Atomic => {
                        let (scalar, _) = tl.scalar_ty(self, ctx)?;
                        ir::TypeInner::Atomic(scalar)
                    }
                    conv::TypeGenerator::Pointer => {
                        let mut space = tl.address_space(ctx)?;
                        let base = tl.ty(self, ctx)?;
                        tl.maybe_access_mode(&mut space, ctx)?;
                        ir::TypeInner::Pointer { base, space }
                    }
                    conv::TypeGenerator::SampledTexture {
                        dim,
                        arrayed,
                        multi,
                    } => {
                        let (scalar, span) = tl.scalar_ty(self, ctx)?;
                        let ir::Scalar { kind, width } = scalar;
                        if width != 4 {
                            return Err(Box::new(Error::BadTextureSampleType { span, scalar }));
                        }
                        ir::TypeInner::Image {
                            dim,
                            arrayed,
                            class: ir::ImageClass::Sampled { kind, multi },
                        }
                    }
                    conv::TypeGenerator::StorageTexture { dim, arrayed } => {
                        let format = tl.storage_format(ctx)?;
                        let access = tl.access_mode(ctx)?;
                        ir::TypeInner::Image {
                            dim,
                            arrayed,
                            class: ir::ImageClass::Storage { format, access },
                        }
                    }
                    conv::TypeGenerator::BindingArray => {
                        let base = tl.ty(self, ctx)?;
                        let size = tl.maybe_array_size(self, ctx)?;
                        ir::TypeInner::BindingArray { base, size }
                    }
                    conv::TypeGenerator::AccelerationStructure => {
                        let vertex_return = tl.maybe_vertex_return(ctx)?;
                        ir::TypeInner::AccelerationStructure { vertex_return }
                    }
                    conv::TypeGenerator::RayQuery => {
                        let vertex_return = tl.maybe_vertex_return(ctx)?;
                        ir::TypeInner::RayQuery { vertex_return }
                    }
                    conv::TypeGenerator::CooperativeMatrix { columns, rows } => {
                        let (ty, span) = tl.ty_with_span(self, ctx)?;
                        let ir::TypeInner::Scalar(scalar) = ctx.module.types[ty].inner else {
                            return Err(Box::new(Error::UnsupportedCooperativeScalar(span)));
                        };
                        let role = tl.cooperative_role(ctx)?;
                        ir::TypeInner::CooperativeMatrix {
                            columns,
                            rows,
                            scalar,
                            role,
                        }
                    }
                };
                ctx.as_global().ensure_type_exists(alias_name, ty_inner)
            }
        };
        Ok(ty)
    }

    fn binary(
        &mut self,
        op: ir::BinaryOperator,
        left: Handle<ast::Expression<'source>>,
        right: Handle<ast::Expression<'source>>,
        span: Span,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Typed<ir::Expression>> {
        if op == ir::BinaryOperator::LogicalAnd || op == ir::BinaryOperator::LogicalOr {
            let left = self.expression_for_abstract(left, ctx)?;
            ctx.grow_types(left)?;

            if !matches!(
                resolve_inner!(ctx, left),
                &ir::TypeInner::Scalar(ir::Scalar::BOOL)
            ) {
                // Pass it through as-is, will fail validation
                let right = self.expression_for_abstract(right, ctx)?;
                ctx.grow_types(right)?;
                Ok(Typed::Plain(crate::Expression::Binary { op, left, right }))
            } else {
                self.logical(op, left, right, span, ctx)
            }
        } else {
            // Load both operands.
            let mut left = self.expression_for_abstract(left, ctx)?;
            let mut right = self.expression_for_abstract(right, ctx)?;

            // Convert `scalar op vector` to `vector op vector` by introducing
            // `Splat` expressions.
            ctx.binary_op_splat(op, &mut left, &mut right)?;

            // Apply automatic conversions.
            match op {
                ir::BinaryOperator::ShiftLeft | ir::BinaryOperator::ShiftRight => {
                    // Shift operators require the right operand to be `u32` or
                    // `vecN<u32>`. We can let the validator sort out vector length
                    // issues, but the right operand must be, or convert to, a u32 leaf
                    // scalar.
                    right =
                        ctx.try_automatic_conversion_for_leaf_scalar(right, ir::Scalar::U32, span)?;

                    // Additionally, we must concretize the left operand if the right operand
                    // is not a const-expression.
                    // See https://www.w3.org/TR/WGSL/#overload-resolution-section.
                    //
                    // 2. Eliminate any candidate where one of its subexpressions resolves to
                    // an abstract type after feasible automatic conversions, but another of
                    // the candidate’s subexpressions is not a const-expression.
                    //
                    // We only have to explicitly do so for shifts as their operands may be
                    // of different types - for other binary ops this is achieved by finding
                    // the conversion consensus for both operands.
                    if !ctx.is_const(right) {
                        left = ctx.concretize(left)?;
                    }
                }

                // All other operators follow the same pattern: reconcile the
                // scalar leaf types. If there's no reconciliation possible,
                // leave the expressions as they are: validation will report the
                // problem.
                _ => {
                    ctx.grow_types(left)?;
                    ctx.grow_types(right)?;
                    if let Ok(consensus_scalar) =
                        ctx.automatic_conversion_consensus(None, [left, right].iter())
                    {
                        ctx.convert_to_leaf_scalar(&mut left, consensus_scalar)?;
                        ctx.convert_to_leaf_scalar(&mut right, consensus_scalar)?;
                    }
                }
            }

            Ok(Typed::Plain(ir::Expression::Binary { op, left, right }))
        }
    }

    /// Generate Naga IR for a call to a WGSL builtin function.
    #[allow(clippy::too_many_arguments)]
    fn call_builtin<'phrase>(
        &mut self,
        function_name: &'source str,
        function_span: Span,
        arguments: &[Handle<ast::Expression<'source>>],
        template_params: &mut TemplateListIter<'phrase, 'source>,
        call_span: Span,
        ctx: &mut ExpressionContext<'source, '_, '_>,
        is_statement: bool,
    ) -> Result<'source, Option<(Handle<ir::Expression>, MustUse)>> {
        let (expr, must_use) = if let Some(fun) = conv::map_relational_fun(function_name) {
            let mut args = ctx.prepare_args(arguments, 1, function_span);
            let argument = self.expression(args.next()?, ctx)?;
            args.finish()?;

            // Check for no-op all(bool) and any(bool):
            let argument_unmodified = matches!(
                fun,
                ir::RelationalFunction::All | ir::RelationalFunction::Any
            ) && {
                matches!(
                    resolve_inner!(ctx, argument),
                    &ir::TypeInner::Scalar(ir::Scalar {
                        kind: ir::ScalarKind::Bool,
                        ..
                    })
                )
            };

            if argument_unmodified {
                return Ok(Some((argument, MustUse::Yes)));
            } else {
                (ir::Expression::Relational { fun, argument }, MustUse::Yes)
            }
        } else if let Some((axis, ctrl)) = conv::map_derivative(function_name) {
            let mut args = ctx.prepare_args(arguments, 1, function_span);
            let expr = self.expression(args.next()?, ctx)?;
            args.finish()?;

            (
                ir::Expression::Derivative { axis, ctrl, expr },
                MustUse::Yes,
            )
        } else if let Some(fun) = conv::map_standard_fun(function_name) {
            (
                self.math_function_helper(function_span, fun, arguments, ctx)?,
                MustUse::Yes,
            )
        } else if let Some(fun) = Texture::map(function_name) {
            (
                self.texture_sample_helper(fun, arguments, function_span, ctx)?,
                MustUse::Yes,
            )
        } else if let Some((op, cop)) = conv::map_subgroup_operation(function_name) {
            return Ok(Some((
                self.subgroup_operation_helper(function_span, op, cop, arguments, ctx)?,
                MustUse::Yes,
            )));
        } else if let Some(mode) = SubgroupGather::map(function_name) {
            return Ok(Some((
                self.subgroup_gather_helper(function_span, mode, arguments, ctx)?,
                MustUse::Yes,
            )));
        } else if let Some(fun) = ir::AtomicFunction::map(function_name) {
            return Ok(self
                .atomic_helper(function_span, fun, arguments, is_statement, ctx)?
                .map(|result| (result, MustUse::No)));
        } else {
            match function_name {
                "bitcast" => {
                    let ty = template_params.ty(self, ctx)?;

                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let expr = self.expression(args.next()?, ctx)?;
                    args.finish()?;

                    let element_scalar = match ctx.module.types[ty].inner {
                        ir::TypeInner::Scalar(scalar) => scalar,
                        ir::TypeInner::Vector { scalar, .. } => scalar,
                        _ => {
                            let ty_resolution = resolve!(ctx, expr);
                            return Err(Box::new(Error::BadTypeCast {
                                from_type: ctx.type_resolution_to_string(ty_resolution),
                                span: function_span,
                                to_type: ctx.type_to_string(ty),
                            }));
                        }
                    };

                    (
                        ir::Expression::As {
                            expr,
                            kind: element_scalar.kind,
                            convert: None,
                        },
                        MustUse::Yes,
                    )
                }
                "coopLoad" | "coopLoadT" => {
                    let row_major = function_name.ends_with("T");
                    let (matrix_ty, matrix_span) = template_params.ty_with_span(self, ctx)?;

                    let mut args = ctx.prepare_args(arguments, 1, call_span);
                    let pointer = self.expression(args.next()?, ctx)?;
                    let (columns, rows, role) = match ctx.module.types[matrix_ty].inner {
                        ir::TypeInner::CooperativeMatrix {
                            columns,
                            rows,
                            role,
                            ..
                        } => (columns, rows, role),
                        _ => return Err(Box::new(Error::InvalidCooperativeLoadType(matrix_span))),
                    };
                    let stride = if args.total_args > 1 {
                        self.expression(args.next()?, ctx)?
                    } else {
                        // Infer the stride from the matrix type
                        let stride = if row_major {
                            columns as u32
                        } else {
                            rows as u32
                        };
                        ctx.append_expression(
                            ir::Expression::Literal(ir::Literal::U32(stride)),
                            Span::UNDEFINED,
                        )?
                    };
                    args.finish()?;

                    (
                        crate::Expression::CooperativeLoad {
                            columns,
                            rows,
                            role,
                            data: crate::CooperativeData {
                                pointer,
                                stride,
                                row_major,
                            },
                        },
                        MustUse::Yes,
                    )
                }
                "select" => {
                    let mut args = ctx.prepare_args(arguments, 3, function_span);

                    let reject_orig = args.next()?;
                    let accept_orig = args.next()?;
                    let mut values = [
                        self.expression_for_abstract(reject_orig, ctx)?,
                        self.expression_for_abstract(accept_orig, ctx)?,
                    ];
                    let condition = self.expression(args.next()?, ctx)?;

                    args.finish()?;

                    let diagnostic_details =
                        |ctx: &ExpressionContext<'_, '_, '_>,
                         ty_res: &proc::TypeResolution,
                         orig_expr| {
                            (
                                ctx.ast_expressions.get_span(orig_expr),
                                format!("`{}`", ctx.as_diagnostic_display(ty_res)),
                            )
                        };
                    for (&value, orig_value) in values.iter().zip([reject_orig, accept_orig]) {
                        let value_ty_res = resolve!(ctx, value);
                        if value_ty_res
                            .inner_with(&ctx.module.types)
                            .vector_size_and_scalar()
                            .is_none()
                        {
                            let (arg_span, arg_type) =
                                diagnostic_details(ctx, value_ty_res, orig_value);
                            return Err(Box::new(Error::SelectUnexpectedArgumentType {
                                arg_span,
                                arg_type,
                            }));
                        }
                    }
                    let mut consensus_scalar = ctx
                        .automatic_conversion_consensus(None, &values)
                        .map_err(|_idx| {
                            let [reject, accept] = values;
                            let [(reject_span, reject_type), (accept_span, accept_type)] =
                                [(reject_orig, reject), (accept_orig, accept)].map(
                                    |(orig_expr, expr)| {
                                        let ty_res = &ctx.typifier()[expr];
                                        diagnostic_details(ctx, ty_res, orig_expr)
                                    },
                                );
                            Error::SelectRejectAndAcceptHaveNoCommonType {
                                reject_span,
                                reject_type,
                                accept_span,
                                accept_type,
                            }
                        })?;
                    if !ctx.is_const(condition) {
                        consensus_scalar = consensus_scalar.concretize();
                    }

                    ctx.convert_slice_to_common_leaf_scalar(&mut values, consensus_scalar)?;

                    let [reject, accept] = values;

                    (
                        ir::Expression::Select {
                            reject,
                            accept,
                            condition,
                        },
                        MustUse::Yes,
                    )
                }
                "arrayLength" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let expr = self.expression(args.next()?, ctx)?;
                    args.finish()?;

                    (ir::Expression::ArrayLength(expr), MustUse::Yes)
                }
                "atomicLoad" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let (pointer, _scalar) = self.atomic_pointer(args.next()?, ctx)?;
                    args.finish()?;

                    (ir::Expression::Load { pointer }, MustUse::No)
                }
                "atomicStore" => {
                    let mut args = ctx.prepare_args(arguments, 2, function_span);
                    let (pointer, scalar) = self.atomic_pointer(args.next()?, ctx)?;
                    let value = self.expression_with_leaf_scalar(args.next()?, scalar, ctx)?;
                    args.finish()?;

                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block
                        .extend(rctx.emitter.finish(&rctx.function.expressions));
                    rctx.emitter.start(&rctx.function.expressions);
                    rctx.block
                        .push(ir::Statement::Store { pointer, value }, function_span);
                    return Ok(None);
                }
                "atomicCompareExchangeWeak" => {
                    let mut args = ctx.prepare_args(arguments, 3, function_span);

                    let (pointer, scalar) = self.atomic_pointer(args.next()?, ctx)?;

                    let compare = self.expression_with_leaf_scalar(args.next()?, scalar, ctx)?;

                    let value = args.next()?;
                    let value_span = ctx.ast_expressions.get_span(value);
                    let value = self.expression_with_leaf_scalar(value, scalar, ctx)?;

                    args.finish()?;

                    let expression = match *resolve_inner!(ctx, value) {
                        ir::TypeInner::Scalar(scalar) => ir::Expression::AtomicResult {
                            ty: ctx.module.generate_predeclared_type(
                                ir::PredeclaredType::AtomicCompareExchangeWeakResult(scalar),
                            ),
                            comparison: true,
                        },
                        _ => return Err(Box::new(Error::InvalidAtomicOperandType(value_span))),
                    };

                    let result = ctx.interrupt_emitter(expression, function_span)?;
                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block.push(
                        ir::Statement::Atomic {
                            pointer,
                            fun: ir::AtomicFunction::Exchange {
                                compare: Some(compare),
                            },
                            value,
                            result: Some(result),
                        },
                        function_span,
                    );
                    return Ok(Some((result, MustUse::No)));
                }
                "textureAtomicMin" | "textureAtomicMax" | "textureAtomicAdd"
                | "textureAtomicAnd" | "textureAtomicOr" | "textureAtomicXor" => {
                    let mut args = ctx.prepare_args(arguments, 3, function_span);

                    let image = args.next()?;
                    let image_span = ctx.ast_expressions.get_span(image);
                    let image = self.expression(image, ctx)?;

                    let coordinate = self.expression(args.next()?, ctx)?;

                    let (_, arrayed) = ctx.image_data(image, image_span)?;
                    let array_index = arrayed
                        .then(|| {
                            args.min_args += 1;
                            self.expression(args.next()?, ctx)
                        })
                        .transpose()?;

                    let value = self.expression(args.next()?, ctx)?;

                    args.finish()?;

                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block
                        .extend(rctx.emitter.finish(&rctx.function.expressions));
                    rctx.emitter.start(&rctx.function.expressions);
                    let stmt = ir::Statement::ImageAtomic {
                        image,
                        coordinate,
                        array_index,
                        fun: match function_name {
                            "textureAtomicMin" => ir::AtomicFunction::Min,
                            "textureAtomicMax" => ir::AtomicFunction::Max,
                            "textureAtomicAdd" => ir::AtomicFunction::Add,
                            "textureAtomicAnd" => ir::AtomicFunction::And,
                            "textureAtomicOr" => ir::AtomicFunction::InclusiveOr,
                            "textureAtomicXor" => ir::AtomicFunction::ExclusiveOr,
                            _ => unreachable!(),
                        },
                        value,
                    };
                    rctx.block.push(stmt, function_span);
                    return Ok(None);
                }
                "storageBarrier" => {
                    ctx.prepare_args(arguments, 0, function_span).finish()?;

                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block.push(
                        ir::Statement::ControlBarrier(ir::Barrier::STORAGE),
                        function_span,
                    );
                    return Ok(None);
                }
                "workgroupBarrier" => {
                    ctx.prepare_args(arguments, 0, function_span).finish()?;

                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block.push(
                        ir::Statement::ControlBarrier(ir::Barrier::WORK_GROUP),
                        function_span,
                    );
                    return Ok(None);
                }
                "subgroupBarrier" => {
                    ctx.prepare_args(arguments, 0, function_span).finish()?;

                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block.push(
                        ir::Statement::ControlBarrier(ir::Barrier::SUB_GROUP),
                        function_span,
                    );
                    return Ok(None);
                }
                "textureBarrier" => {
                    ctx.prepare_args(arguments, 0, function_span).finish()?;

                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block.push(
                        ir::Statement::ControlBarrier(ir::Barrier::TEXTURE),
                        function_span,
                    );
                    return Ok(None);
                }
                "workgroupUniformLoad" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let expr = args.next()?;
                    args.finish()?;

                    let pointer = self.expression(expr, ctx)?;
                    let result_ty = match *resolve_inner!(ctx, pointer) {
                        ir::TypeInner::Pointer {
                            base,
                            space: ir::AddressSpace::WorkGroup,
                        } => match ctx.module.types[base].inner {
                            // Match `Expression::Load` semantics:
                            // loading through a pointer to `atomic<T>` produces a `T`.
                            ir::TypeInner::Atomic(scalar) => ctx.module.types.insert(
                                ir::Type {
                                    name: None,
                                    inner: ir::TypeInner::Scalar(scalar),
                                },
                                function_span,
                            ),
                            _ => base,
                        },
                        ir::TypeInner::ValuePointer {
                            size,
                            scalar,
                            space: ir::AddressSpace::WorkGroup,
                        } => ctx.module.types.insert(
                            ir::Type {
                                name: None,
                                inner: match size {
                                    Some(size) => ir::TypeInner::Vector { size, scalar },
                                    None => ir::TypeInner::Scalar(scalar),
                                },
                            },
                            function_span,
                        ),
                        _ => {
                            let span = ctx.ast_expressions.get_span(expr);
                            return Err(Box::new(Error::InvalidWorkGroupUniformLoad(span)));
                        }
                    };
                    let result = ctx.interrupt_emitter(
                        ir::Expression::WorkGroupUniformLoadResult { ty: result_ty },
                        function_span,
                    )?;
                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block.push(
                        ir::Statement::WorkGroupUniformLoad { pointer, result },
                        function_span,
                    );

                    return Ok(Some((result, MustUse::Yes)));
                }
                "textureStore" => {
                    let mut args = ctx.prepare_args(arguments, 3, function_span);

                    let image = args.next()?;
                    let image_span = ctx.ast_expressions.get_span(image);
                    let image = self.expression(image, ctx)?;

                    let coordinate = self.expression(args.next()?, ctx)?;

                    let (class, arrayed) = ctx.image_data(image, image_span)?;
                    let array_index = arrayed
                        .then(|| {
                            args.min_args += 1;
                            self.expression(args.next()?, ctx)
                        })
                        .transpose()?;
                    let scalar = if let ir::ImageClass::Storage { format, .. } = class {
                        format.into()
                    } else {
                        return Err(Box::new(Error::NotStorageTexture(image_span)));
                    };

                    let value = self.expression_with_leaf_scalar(args.next()?, scalar, ctx)?;

                    args.finish()?;

                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block
                        .extend(rctx.emitter.finish(&rctx.function.expressions));
                    rctx.emitter.start(&rctx.function.expressions);
                    let stmt = ir::Statement::ImageStore {
                        image,
                        coordinate,
                        array_index,
                        value,
                    };
                    rctx.block.push(stmt, function_span);
                    return Ok(None);
                }
                "textureLoad" => {
                    let mut args = ctx.prepare_args(arguments, 2, function_span);

                    let image = args.next()?;
                    let image_span = ctx.ast_expressions.get_span(image);
                    let image = self.expression(image, ctx)?;

                    let coordinate = self.expression(args.next()?, ctx)?;

                    let (class, arrayed) = ctx.image_data(image, image_span)?;
                    let array_index = arrayed
                        .then(|| {
                            args.min_args += 1;
                            self.expression(args.next()?, ctx)
                        })
                        .transpose()?;

                    let level = class
                        .is_mipmapped()
                        .then(|| {
                            args.min_args += 1;
                            self.expression(args.next()?, ctx)
                        })
                        .transpose()?;

                    let sample = class
                        .is_multisampled()
                        .then(|| self.expression(args.next()?, ctx))
                        .transpose()?;

                    args.finish()?;

                    (
                        ir::Expression::ImageLoad {
                            image,
                            coordinate,
                            array_index,
                            level,
                            sample,
                        },
                        MustUse::Yes,
                    )
                }
                "textureDimensions" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let image = self.expression(args.next()?, ctx)?;
                    let level = args
                        .next()
                        .map(|arg| self.expression(arg, ctx))
                        .ok()
                        .transpose()?;
                    args.finish()?;

                    (
                        ir::Expression::ImageQuery {
                            image,
                            query: ir::ImageQuery::Size { level },
                        },
                        MustUse::Yes,
                    )
                }
                "textureNumLevels" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let image = self.expression(args.next()?, ctx)?;
                    args.finish()?;

                    (
                        ir::Expression::ImageQuery {
                            image,
                            query: ir::ImageQuery::NumLevels,
                        },
                        MustUse::Yes,
                    )
                }
                "textureNumLayers" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let image = self.expression(args.next()?, ctx)?;
                    args.finish()?;

                    (
                        ir::Expression::ImageQuery {
                            image,
                            query: ir::ImageQuery::NumLayers,
                        },
                        MustUse::Yes,
                    )
                }
                "textureNumSamples" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let image = self.expression(args.next()?, ctx)?;
                    args.finish()?;

                    (
                        ir::Expression::ImageQuery {
                            image,
                            query: ir::ImageQuery::NumSamples,
                        },
                        MustUse::Yes,
                    )
                }
                "rayQueryInitialize" => {
                    let mut args = ctx.prepare_args(arguments, 3, function_span);
                    let query = self.ray_query_pointer(args.next()?, ctx)?;
                    let acceleration_structure = self.expression(args.next()?, ctx)?;
                    let descriptor = self.expression(args.next()?, ctx)?;
                    args.finish()?;

                    let _ = ctx.module.generate_ray_desc_type();
                    let fun = ir::RayQueryFunction::Initialize {
                        acceleration_structure,
                        descriptor,
                    };

                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block
                        .extend(rctx.emitter.finish(&rctx.function.expressions));
                    rctx.emitter.start(&rctx.function.expressions);
                    rctx.block
                        .push(ir::Statement::RayQuery { query, fun }, function_span);
                    return Ok(None);
                }
                "getCommittedHitVertexPositions" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let query = self.ray_query_pointer(args.next()?, ctx)?;
                    args.finish()?;

                    let _ = ctx.module.generate_vertex_return_type();

                    (
                        ir::Expression::RayQueryVertexPositions {
                            query,
                            committed: true,
                        },
                        MustUse::No,
                    )
                }
                "getCandidateHitVertexPositions" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let query = self.ray_query_pointer(args.next()?, ctx)?;
                    args.finish()?;

                    let _ = ctx.module.generate_vertex_return_type();

                    (
                        ir::Expression::RayQueryVertexPositions {
                            query,
                            committed: false,
                        },
                        MustUse::No,
                    )
                }
                "rayQueryProceed" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let query = self.ray_query_pointer(args.next()?, ctx)?;
                    args.finish()?;

                    let result = ctx
                        .interrupt_emitter(ir::Expression::RayQueryProceedResult, function_span)?;
                    let fun = ir::RayQueryFunction::Proceed { result };
                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block
                        .push(ir::Statement::RayQuery { query, fun }, function_span);
                    return Ok(Some((result, MustUse::No)));
                }
                "rayQueryGenerateIntersection" => {
                    let mut args = ctx.prepare_args(arguments, 2, function_span);
                    let query = self.ray_query_pointer(args.next()?, ctx)?;
                    let hit_t = self.expression(args.next()?, ctx)?;
                    args.finish()?;

                    let fun = ir::RayQueryFunction::GenerateIntersection { hit_t };
                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block
                        .push(ir::Statement::RayQuery { query, fun }, function_span);
                    return Ok(None);
                }
                "rayQueryConfirmIntersection" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let query = self.ray_query_pointer(args.next()?, ctx)?;
                    args.finish()?;

                    let fun = ir::RayQueryFunction::ConfirmIntersection;
                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block
                        .push(ir::Statement::RayQuery { query, fun }, function_span);
                    return Ok(None);
                }
                "rayQueryTerminate" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let query = self.ray_query_pointer(args.next()?, ctx)?;
                    args.finish()?;

                    let fun = ir::RayQueryFunction::Terminate;
                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block
                        .push(ir::Statement::RayQuery { query, fun }, function_span);
                    return Ok(None);
                }
                "rayQueryGetCommittedIntersection" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let query = self.ray_query_pointer(args.next()?, ctx)?;
                    args.finish()?;

                    let _ = ctx.module.generate_ray_intersection_type();
                    (
                        ir::Expression::RayQueryGetIntersection {
                            query,
                            committed: true,
                        },
                        MustUse::No,
                    )
                }
                "rayQueryGetCandidateIntersection" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);
                    let query = self.ray_query_pointer(args.next()?, ctx)?;
                    args.finish()?;

                    let _ = ctx.module.generate_ray_intersection_type();
                    (
                        ir::Expression::RayQueryGetIntersection {
                            query,
                            committed: false,
                        },
                        MustUse::No,
                    )
                }
                "subgroupBallot" => {
                    let mut args = ctx.prepare_args(arguments, 0, function_span);
                    let predicate = if arguments.len() == 1 {
                        Some(self.expression(args.next()?, ctx)?)
                    } else {
                        None
                    };
                    args.finish()?;

                    let result =
                        ctx.interrupt_emitter(ir::Expression::SubgroupBallotResult, function_span)?;
                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block.push(
                        ir::Statement::SubgroupBallot { result, predicate },
                        function_span,
                    );
                    return Ok(Some((result, MustUse::Yes)));
                }
                "quadSwapX" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);

                    let argument = self.expression(args.next()?, ctx)?;
                    args.finish()?;

                    let ty = ctx.register_type(argument)?;

                    let result = ctx.interrupt_emitter(
                        crate::Expression::SubgroupOperationResult { ty },
                        function_span,
                    )?;
                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block.push(
                        crate::Statement::SubgroupGather {
                            mode: crate::GatherMode::QuadSwap(crate::Direction::X),
                            argument,
                            result,
                        },
                        function_span,
                    );
                    return Ok(Some((result, MustUse::Yes)));
                }
                "quadSwapY" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);

                    let argument = self.expression(args.next()?, ctx)?;
                    args.finish()?;

                    let ty = ctx.register_type(argument)?;

                    let result = ctx.interrupt_emitter(
                        crate::Expression::SubgroupOperationResult { ty },
                        function_span,
                    )?;
                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block.push(
                        crate::Statement::SubgroupGather {
                            mode: crate::GatherMode::QuadSwap(crate::Direction::Y),
                            argument,
                            result,
                        },
                        function_span,
                    );
                    return Ok(Some((result, MustUse::Yes)));
                }
                "quadSwapDiagonal" => {
                    let mut args = ctx.prepare_args(arguments, 1, function_span);

                    let argument = self.expression(args.next()?, ctx)?;
                    args.finish()?;

                    let ty = ctx.register_type(argument)?;

                    let result = ctx.interrupt_emitter(
                        crate::Expression::SubgroupOperationResult { ty },
                        function_span,
                    )?;
                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block.push(
                        crate::Statement::SubgroupGather {
                            mode: crate::GatherMode::QuadSwap(crate::Direction::Diagonal),
                            argument,
                            result,
                        },
                        function_span,
                    );
                    return Ok(Some((result, MustUse::Yes)));
                }
                "coopStore" | "coopStoreT" => {
                    let row_major = function_name.ends_with("T");

                    let mut args = ctx.prepare_args(arguments, 2, function_span);
                    let target = self.expression(args.next()?, ctx)?;
                    let pointer = self.expression(args.next()?, ctx)?;
                    let stride = if args.total_args > 2 {
                        self.expression(args.next()?, ctx)?
                    } else {
                        // Infer the stride from the matrix type
                        let stride = match *resolve_inner!(ctx, target) {
                            ir::TypeInner::CooperativeMatrix { columns, rows, .. } => {
                                if row_major {
                                    columns as u32
                                } else {
                                    rows as u32
                                }
                            }
                            _ => 0,
                        };
                        ctx.append_expression(
                            ir::Expression::Literal(ir::Literal::U32(stride)),
                            Span::UNDEFINED,
                        )?
                    };
                    args.finish()?;

                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block.push(
                        crate::Statement::CooperativeStore {
                            target,
                            data: crate::CooperativeData {
                                pointer,
                                stride,
                                row_major,
                            },
                        },
                        function_span,
                    );
                    return Ok(None);
                }
                "coopMultiplyAdd" => {
                    let mut args = ctx.prepare_args(arguments, 3, function_span);
                    let a = self.expression(args.next()?, ctx)?;
                    let b = self.expression(args.next()?, ctx)?;
                    let c = self.expression(args.next()?, ctx)?;
                    args.finish()?;

                    (
                        ir::Expression::CooperativeMultiplyAdd { a, b, c },
                        MustUse::Yes,
                    )
                }
                "traceRay" => {
                    let mut args = ctx.prepare_args(arguments, 3, function_span);
                    let acceleration_structure = self.expression(args.next()?, ctx)?;
                    let descriptor = self.expression(args.next()?, ctx)?;
                    let payload = self.expression(args.next()?, ctx)?;
                    args.finish()?;

                    let _ = ctx.module.generate_ray_desc_type();
                    let fun = ir::RayPipelineFunction::TraceRay {
                        acceleration_structure,
                        descriptor,
                        payload,
                    };

                    let rctx = ctx.runtime_expression_ctx(function_span)?;
                    rctx.block
                        .extend(rctx.emitter.finish(&rctx.function.expressions));
                    rctx.emitter.start(&rctx.function.expressions);
                    rctx.block
                        .push(ir::Statement::RayPipelineFunction(fun), function_span);
                    return Ok(None);
                }
                _ => return Err(Box::new(Error::UnknownIdent(function_span, function_name))),
            }
        };

        let expr = ctx.append_expression(expr, function_span)?;
        Ok(Some((expr, must_use)))
    }

    /// Generate Naga IR for call expressions and statements, and type
    /// constructor expressions.
    ///
    /// The "function" being called is simply an `Ident` that we know refers to
    /// some module-scope definition.
    ///
    /// - If it is the name of a type, then the expression is a type constructor
    ///   expression: either constructing a value from components, a conversion
    ///   expression, or a zero value expression.
    ///
    /// - If it is the name of a function, then we're generating a [`Call`]
    ///   statement. We may be in the midst of generating code for an
    ///   expression, in which case we must generate an `Emit` statement to
    ///   force evaluation of the IR expressions we've generated so far, add the
    ///   `Call` statement to the current block, and then resume generating
    ///   expressions.
    ///
    /// [`Call`]: ir::Statement::Call
    fn call(
        &mut self,
        call_phrase: &ast::CallPhrase<'source>,
        span: Span,
        ctx: &mut ExpressionContext<'source, '_, '_>,
        is_statement: bool,
    ) -> Result<'source, Option<Handle<ir::Expression>>> {
        let function_name = match call_phrase.function.ident {
            ast::IdentExpr::Unresolved(name) => name,
            ast::IdentExpr::Local(_) => {
                return Err(Box::new(Error::CalledLocalDecl(
                    call_phrase.function.ident_span,
                )))
            }
        };
        let mut function_span = call_phrase.function.ident_span;
        function_span.subsume(call_phrase.function.template_list_span);
        let arguments = call_phrase.arguments.as_slice();

        let mut tl = TemplateListIter::new(function_span, &call_phrase.function.template_list);

        let result = match ctx.globals.get(function_name) {
            Some(&LoweredGlobalDecl::Type(ty)) => {
                // user-declared types can't make use of template lists
                tl.finish(ctx)?;

                let handle =
                    self.construct(span, Constructor::Type(ty), function_span, arguments, ctx)?;
                Some((handle, MustUse::Yes))
            }
            Some(
                &LoweredGlobalDecl::Const(_)
                | &LoweredGlobalDecl::Override(_)
                | &LoweredGlobalDecl::Var(_),
            ) => {
                return Err(Box::new(Error::Unexpected(
                    function_span,
                    ExpectedToken::Function,
                )))
            }
            Some(&LoweredGlobalDecl::EntryPoint(_)) => {
                return Err(Box::new(Error::CalledEntryPoint(function_span)));
            }
            Some(&LoweredGlobalDecl::Function {
                handle: function,
                must_use,
            }) => {
                // user-declared functions can't make use of template lists
                tl.finish(ctx)?;

                let arguments = arguments
                    .iter()
                    .enumerate()
                    .map(|(i, &arg)| {
                        // Try to convert abstract values to the known argument types
                        let Some(&ir::FunctionArgument {
                            ty: parameter_ty, ..
                        }) = ctx.module.functions[function].arguments.get(i)
                        else {
                            // Wrong number of arguments... just concretize the type here
                            // and let the validator report the error.
                            return self.expression(arg, ctx);
                        };

                        let expr = self.expression_for_abstract(arg, ctx)?;
                        ctx.try_automatic_conversions(
                            expr,
                            &proc::TypeResolution::Handle(parameter_ty),
                            ctx.ast_expressions.get_span(arg),
                        )
                    })
                    .collect::<Result<Vec<_>>>()?;

                let has_result = ctx.module.functions[function].result.is_some();

                let rctx = ctx.runtime_expression_ctx(span)?;
                // we need to always do this before a fn call since all arguments need to be emitted before the fn call
                rctx.block
                    .extend(rctx.emitter.finish(&rctx.function.expressions));
                let result = has_result.then(|| {
                    let result = rctx
                        .function
                        .expressions
                        .append(ir::Expression::CallResult(function), span);
                    rctx.local_expression_kind_tracker
                        .insert(result, proc::ExpressionKind::Runtime);
                    (result, must_use.into())
                });
                rctx.emitter.start(&rctx.function.expressions);
                rctx.block.push(
                    ir::Statement::Call {
                        function,
                        arguments,
                        result: result.map(|(expr, _)| expr),
                    },
                    span,
                );

                result
            }
            None => {
                // If the name refers to a predeclared type, this is a construction expression.
                let ty = conv::map_predeclared_type(
                    &ctx.enable_extensions,
                    function_span,
                    function_name,
                )?;
                if let Some(ty) = ty {
                    let empty_template_list = call_phrase.function.template_list.is_empty();
                    let constructor_ty = match ty {
                        conv::PredeclaredType::TypeGenerator(conv::TypeGenerator::Vector {
                            size,
                        }) if empty_template_list => Constructor::PartialVector { size },
                        conv::PredeclaredType::TypeGenerator(conv::TypeGenerator::Matrix {
                            columns,
                            rows,
                        }) if empty_template_list => Constructor::PartialMatrix { columns, rows },
                        conv::PredeclaredType::TypeGenerator(conv::TypeGenerator::Array)
                            if empty_template_list =>
                        {
                            Constructor::PartialArray
                        }
                        conv::PredeclaredType::TypeGenerator(
                            conv::TypeGenerator::CooperativeMatrix { .. },
                        ) if empty_template_list => {
                            return Err(Box::new(Error::UnderspecifiedCooperativeMatrix));
                        }
                        _ => Constructor::Type(self.finalize_type(ctx, ty, &mut tl, None)?),
                    };
                    tl.finish(ctx)?;
                    let handle =
                        self.construct(span, constructor_ty, function_span, arguments, ctx)?;
                    Some((handle, MustUse::Yes))
                } else {
                    // Otherwise, it must be a call to a builtin function.
                    let result = self.call_builtin(
                        function_name,
                        function_span,
                        arguments,
                        &mut tl,
                        span,
                        ctx,
                        is_statement,
                    )?;
                    tl.finish(ctx)?;
                    result
                }
            }
        };

        let result_used = !is_statement;
        if matches!(result, Some((_, MustUse::Yes))) && !result_used {
            return Err(Box::new(Error::FunctionMustUseUnused(function_span)));
        }
        Ok(result.map(|(expr, _)| expr))
    }

    /// Generate a Naga IR [`Math`] expression.
    ///
    /// Generate Naga IR for a call to the [`MathFunction`] `fun`, whose
    /// unlowered arguments are `ast_arguments`.
    ///
    /// The `span` argument should give the span of the function name in the
    /// call expression.
    ///
    /// [`Math`]: ir::Expression::Math
    /// [`MathFunction`]: ir::MathFunction
    fn math_function_helper(
        &mut self,
        span: Span,
        fun: ir::MathFunction,
        ast_arguments: &[Handle<ast::Expression<'source>>],
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, ir::Expression> {
        let mut lowered_arguments = Vec::with_capacity(ast_arguments.len());
        for &arg in ast_arguments {
            let lowered = self.expression_for_abstract(arg, ctx)?;
            ctx.grow_types(lowered)?;
            lowered_arguments.push(lowered);
        }

        let fun_overloads = fun.overloads();
        let rule = self.resolve_overloads(span, fun, fun_overloads, &lowered_arguments, ctx)?;
        self.apply_automatic_conversions_for_call(&rule, &mut lowered_arguments, ctx)?;

        // If this function returns a predeclared type, register it
        // in `Module::special_types`. The typifier will expect to
        // be able to find it there.
        if let proc::Conclusion::Predeclared(predeclared) = rule.conclusion {
            ctx.module.generate_predeclared_type(predeclared);
        }

        Ok(ir::Expression::Math {
            fun,
            arg: lowered_arguments[0],
            arg1: lowered_arguments.get(1).cloned(),
            arg2: lowered_arguments.get(2).cloned(),
            arg3: lowered_arguments.get(3).cloned(),
        })
    }

    /// Choose the right overload for a function call.
    ///
    /// Return a [`Rule`] representing the most preferred overload in
    /// `overloads` to apply to `arguments`, or return an error explaining why
    /// the call is not valid.
    ///
    /// Use `fun` to identify the function being called in error messages;
    /// `span` should be the span of the function name in the call expression.
    ///
    /// [`Rule`]: proc::Rule
    fn resolve_overloads<O, F>(
        &self,
        span: Span,
        fun: F,
        overloads: O,
        arguments: &[Handle<ir::Expression>],
        ctx: &ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, proc::Rule>
    where
        O: proc::OverloadSet,
        F: TryToWgsl + core::fmt::Debug + Copy,
    {
        let mut remaining_overloads = overloads.clone();
        let min_arguments = remaining_overloads.min_arguments();
        let max_arguments = remaining_overloads.max_arguments();
        if arguments.len() < min_arguments {
            return Err(Box::new(Error::WrongArgumentCount {
                span,
                expected: min_arguments as u32..max_arguments as u32,
                found: arguments.len() as u32,
            }));
        }
        if arguments.len() > max_arguments {
            return Err(Box::new(Error::TooManyArguments {
                function: fun.to_wgsl_for_diagnostics(),
                call_span: span,
                arg_span: ctx.get_expression_span(arguments[max_arguments]),
                max_arguments: max_arguments as _,
            }));
        }

        log::debug!(
            "Initial overloads: {:#?}",
            remaining_overloads.for_debug(&ctx.module.types)
        );

        for (arg_index, &arg) in arguments.iter().enumerate() {
            let arg_type_resolution = &ctx.typifier()[arg];
            let arg_inner = arg_type_resolution.inner_with(&ctx.module.types);
            log::debug!(
                "Supplying argument {arg_index} of type {:?}",
                arg_type_resolution.for_debug(&ctx.module.types)
            );
            let next_remaining_overloads =
                remaining_overloads.arg(arg_index, arg_inner, &ctx.module.types);

            // If any argument is not a constant expression, then no overloads
            // that accept abstract values should be considered.
            // (`OverloadSet::concrete_only` is supposed to help impose this
            // restriction.) However, no `MathFunction` accepts a mix of
            // abstract and concrete arguments, so we don't need to worry
            // about that here.

            log::debug!(
                "Remaining overloads: {:#?}",
                next_remaining_overloads.for_debug(&ctx.module.types)
            );

            // If the set of remaining overloads is empty, then this argument's type
            // was unacceptable. Diagnose the problem and produce an error message.
            if next_remaining_overloads.is_empty() {
                let function = fun.to_wgsl_for_diagnostics();
                let call_span = span;
                let arg_span = ctx.get_expression_span(arg);
                let arg_ty = ctx.as_diagnostic_display(arg_type_resolution).to_string();

                // Is this type *ever* permitted for the arg_index'th argument?
                // For example, `bool` is never permitted for `max`.
                let only_this_argument = overloads.arg(arg_index, arg_inner, &ctx.module.types);
                if only_this_argument.is_empty() {
                    // No overload of `fun` accepts this type as the
                    // arg_index'th argument. Determine the set of types that
                    // would ever be allowed there.
                    let allowed: Vec<String> = overloads
                        .allowed_args(arg_index, &ctx.module.to_ctx())
                        .iter()
                        .map(|ty| ctx.type_resolution_to_string(ty))
                        .collect();

                    if allowed.is_empty() {
                        // No overload of `fun` accepts any argument at this
                        // index, so it's a simple case of excess arguments.
                        // However, since each `MathFunction`'s overloads all
                        // have the same arity, we should have detected this
                        // earlier.
                        unreachable!("expected all overloads to have the same arity");
                    }

                    // Some overloads of `fun` do accept this many arguments,
                    // but none accept one of this type.
                    return Err(Box::new(Error::WrongArgumentType {
                        function,
                        call_span,
                        arg_span,
                        arg_index: arg_index as u32,
                        arg_ty,
                        allowed,
                    }));
                }

                // This argument's type is accepted by some overloads---just
                // not those overloads that remain, given the prior arguments.
                // For example, `max` accepts `f32` as its second argument -
                // but not if the first was `i32`.

                // Build a list of the types that would have been accepted here,
                // given the prior arguments.
                let allowed: Vec<String> = remaining_overloads
                    .allowed_args(arg_index, &ctx.module.to_ctx())
                    .iter()
                    .map(|ty| ctx.type_resolution_to_string(ty))
                    .collect();

                // Re-run the argument list to determine which prior argument
                // made this one unacceptable.
                let mut remaining_overloads = overloads;
                for (prior_index, &prior_expr) in arguments.iter().enumerate() {
                    let prior_type_resolution = &ctx.typifier()[prior_expr];
                    let prior_ty = prior_type_resolution.inner_with(&ctx.module.types);
                    remaining_overloads =
                        remaining_overloads.arg(prior_index, prior_ty, &ctx.module.types);
                    if remaining_overloads
                        .arg(arg_index, arg_inner, &ctx.module.types)
                        .is_empty()
                    {
                        // This is the argument that killed our dreams.
                        let inconsistent_span = ctx.get_expression_span(arguments[prior_index]);
                        let inconsistent_ty =
                            ctx.as_diagnostic_display(prior_type_resolution).to_string();

                        if allowed.is_empty() {
                            // Some overloads did accept `ty` at `arg_index`, but
                            // given the arguments up through `prior_expr`, we see
                            // no types acceptable at `arg_index`. This means that some
                            // overloads expect fewer arguments than others. However,
                            // each `MathFunction`'s overloads have the same arity, so this
                            // should be impossible.
                            unreachable!("expected all overloads to have the same arity");
                        }

                        // Report `arg`'s type as inconsistent with `prior_expr`'s
                        return Err(Box::new(Error::InconsistentArgumentType {
                            function,
                            call_span,
                            arg_span,
                            arg_index: arg_index as u32,
                            arg_ty,
                            inconsistent_span,
                            inconsistent_index: prior_index as u32,
                            inconsistent_ty,
                            allowed,
                        }));
                    }
                }
                unreachable!("Failed to eliminate argument type when re-tried");
            }
            remaining_overloads = next_remaining_overloads;
        }

        // Select the most preferred type rule for this call,
        // given the argument types supplied above.
        Ok(remaining_overloads.most_preferred())
    }

    /// Apply automatic type conversions for a function call.
    ///
    /// Apply whatever automatic conversions are needed to pass `arguments` to
    /// the function overload described by `rule`. Update `arguments` to refer
    /// to the converted arguments.
    fn apply_automatic_conversions_for_call(
        &self,
        rule: &proc::Rule,
        arguments: &mut [Handle<ir::Expression>],
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, ()> {
        for (i, argument) in arguments.iter_mut().enumerate() {
            let goal_inner = rule.arguments[i].inner_with(&ctx.module.types);
            let converted = match goal_inner.scalar_for_conversions(&ctx.module.types) {
                Some(goal_scalar) => {
                    let arg_span = ctx.get_expression_span(*argument);
                    ctx.try_automatic_conversion_for_leaf_scalar(*argument, goal_scalar, arg_span)?
                }
                // No conversion is necessary.
                None => *argument,
            };

            *argument = converted;
        }

        Ok(())
    }

    fn atomic_pointer(
        &mut self,
        expr: Handle<ast::Expression<'source>>,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, (Handle<ir::Expression>, ir::Scalar)> {
        let span = ctx.ast_expressions.get_span(expr);
        let pointer = self.expression(expr, ctx)?;

        match *resolve_inner!(ctx, pointer) {
            ir::TypeInner::Pointer { base, .. } => match ctx.module.types[base].inner {
                ir::TypeInner::Atomic(scalar) => Ok((pointer, scalar)),
                ref other => {
                    log::error!("Pointer type to {other:?} passed to atomic op");
                    Err(Box::new(Error::InvalidAtomicPointer(span)))
                }
            },
            ref other => {
                log::error!("Type {other:?} passed to atomic op");
                Err(Box::new(Error::InvalidAtomicPointer(span)))
            }
        }
    }

    fn atomic_helper(
        &mut self,
        span: Span,
        fun: ir::AtomicFunction,
        args: &[Handle<ast::Expression<'source>>],
        is_statement: bool,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Option<Handle<ir::Expression>>> {
        let mut args = ctx.prepare_args(args, 2, span);

        let (pointer, scalar) = self.atomic_pointer(args.next()?, ctx)?;
        let value = self.expression_with_leaf_scalar(args.next()?, scalar, ctx)?;
        let value_inner = resolve_inner!(ctx, value);
        args.finish()?;

        // If we don't use the return value of a 64-bit `min` or `max`
        // operation, generate a no-result form of the `Atomic` statement, so
        // that we can pass validation with only `SHADER_INT64_ATOMIC_MIN_MAX`
        // whenever possible.
        let is_64_bit_min_max = matches!(fun, ir::AtomicFunction::Min | ir::AtomicFunction::Max)
            && matches!(
                *value_inner,
                ir::TypeInner::Scalar(ir::Scalar { width: 8, .. })
            );
        let result = if is_64_bit_min_max && is_statement {
            let rctx = ctx.runtime_expression_ctx(span)?;
            rctx.block
                .extend(rctx.emitter.finish(&rctx.function.expressions));
            rctx.emitter.start(&rctx.function.expressions);
            None
        } else {
            let ty = ctx.register_type(value)?;
            Some(ctx.interrupt_emitter(
                ir::Expression::AtomicResult {
                    ty,
                    comparison: false,
                },
                span,
            )?)
        };
        let rctx = ctx.runtime_expression_ctx(span)?;
        rctx.block.push(
            ir::Statement::Atomic {
                pointer,
                fun,
                value,
                result,
            },
            span,
        );
        Ok(result)
    }

    fn texture_sample_helper(
        &mut self,
        fun: Texture,
        args: &[Handle<ast::Expression<'source>>],
        span: Span,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, ir::Expression> {
        let mut args = ctx.prepare_args(args, fun.min_argument_count(), span);

        fn get_image_and_span<'source>(
            lowerer: &mut Lowerer<'source, '_>,
            args: &mut ArgumentContext<'_, 'source>,
            ctx: &mut ExpressionContext<'source, '_, '_>,
        ) -> Result<'source, (Handle<ir::Expression>, Span)> {
            let image = args.next()?;
            let image_span = ctx.ast_expressions.get_span(image);
            let image = lowerer.expression_for_abstract(image, ctx)?;
            Ok((image, image_span))
        }

        let image;
        let image_span;
        let gather;
        match fun {
            Texture::Gather => {
                let image_or_component = args.next()?;
                let image_or_component_span = ctx.ast_expressions.get_span(image_or_component);
                // Gathers from depth textures don't take an initial `component` argument.
                let lowered_image_or_component = self.expression(image_or_component, ctx)?;

                match *resolve_inner!(ctx, lowered_image_or_component) {
                    ir::TypeInner::Image {
                        class: ir::ImageClass::Depth { .. },
                        ..
                    } => {
                        image = lowered_image_or_component;
                        image_span = image_or_component_span;
                        gather = Some(ir::SwizzleComponent::X);
                    }
                    _ => {
                        (image, image_span) = get_image_and_span(self, &mut args, ctx)?;
                        gather = Some(ctx.gather_component(
                            lowered_image_or_component,
                            image_or_component_span,
                            span,
                        )?);
                    }
                }
            }
            Texture::GatherCompare => {
                (image, image_span) = get_image_and_span(self, &mut args, ctx)?;
                gather = Some(ir::SwizzleComponent::X);
            }

            _ => {
                (image, image_span) = get_image_and_span(self, &mut args, ctx)?;
                gather = None;
            }
        };

        let sampler = self.expression_for_abstract(args.next()?, ctx)?;

        let coordinate = self.expression_with_leaf_scalar(args.next()?, ir::Scalar::F32, ctx)?;
        let clamp_to_edge = matches!(fun, Texture::SampleBaseClampToEdge);

        let (class, arrayed) = ctx.image_data(image, image_span)?;
        let array_index = arrayed
            .then(|| self.expression(args.next()?, ctx))
            .transpose()?;

        let level;
        let depth_ref;
        match fun {
            Texture::Gather => {
                level = ir::SampleLevel::Zero;
                depth_ref = None;
            }
            Texture::GatherCompare => {
                let reference =
                    self.expression_with_leaf_scalar(args.next()?, ir::Scalar::F32, ctx)?;
                level = ir::SampleLevel::Zero;
                depth_ref = Some(reference);
            }

            Texture::Sample => {
                level = ir::SampleLevel::Auto;
                depth_ref = None;
            }
            Texture::SampleBias => {
                let bias = self.expression_with_leaf_scalar(args.next()?, ir::Scalar::F32, ctx)?;
                level = ir::SampleLevel::Bias(bias);
                depth_ref = None;
            }
            Texture::SampleCompare => {
                let reference =
                    self.expression_with_leaf_scalar(args.next()?, ir::Scalar::F32, ctx)?;
                level = ir::SampleLevel::Auto;
                depth_ref = Some(reference);
            }
            Texture::SampleCompareLevel => {
                let reference =
                    self.expression_with_leaf_scalar(args.next()?, ir::Scalar::F32, ctx)?;
                level = ir::SampleLevel::Zero;
                depth_ref = Some(reference);
            }
            Texture::SampleGrad => {
                let x = self.expression_with_leaf_scalar(args.next()?, ir::Scalar::F32, ctx)?;
                let y = self.expression_with_leaf_scalar(args.next()?, ir::Scalar::F32, ctx)?;
                level = ir::SampleLevel::Gradient { x, y };
                depth_ref = None;
            }
            Texture::SampleLevel => {
                let exact = match class {
                    // When applied to depth textures, `textureSampleLevel`'s
                    // `level` argument is an `i32` or `u32`.
                    ir::ImageClass::Depth { .. } => self.expression(args.next()?, ctx)?,

                    // When applied to other sampled types, its `level` argument
                    // is an `f32`.
                    ir::ImageClass::Sampled { .. } => {
                        self.expression_with_leaf_scalar(args.next()?, ir::Scalar::F32, ctx)?
                    }

                    // Sampling `External` textures with a specified level isn't
                    // allowed, and sampling `Storage` textures isn't allowed at
                    // all. Let the validator report the error.
                    ir::ImageClass::Storage { .. } | ir::ImageClass::External => {
                        self.expression(args.next()?, ctx)?
                    }
                };
                level = ir::SampleLevel::Exact(exact);
                depth_ref = None;
            }
            Texture::SampleBaseClampToEdge => {
                level = crate::SampleLevel::Zero;
                depth_ref = None;
            }
        };

        let offset = args
            .next()
            .map(|arg| self.expression_with_leaf_scalar(arg, ir::Scalar::I32, &mut ctx.as_const()))
            .ok()
            .transpose()?;

        args.finish()?;

        Ok(ir::Expression::ImageSample {
            image,
            sampler,
            gather,
            coordinate,
            array_index,
            offset,
            level,
            depth_ref,
            clamp_to_edge,
        })
    }

    fn subgroup_operation_helper(
        &mut self,
        span: Span,
        op: ir::SubgroupOperation,
        collective_op: ir::CollectiveOperation,
        arguments: &[Handle<ast::Expression<'source>>],
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Handle<ir::Expression>> {
        let mut args = ctx.prepare_args(arguments, 1, span);

        let argument = self.expression(args.next()?, ctx)?;
        args.finish()?;

        let ty = ctx.register_type(argument)?;

        let result = ctx.interrupt_emitter(ir::Expression::SubgroupOperationResult { ty }, span)?;
        let rctx = ctx.runtime_expression_ctx(span)?;
        rctx.block.push(
            ir::Statement::SubgroupCollectiveOperation {
                op,
                collective_op,
                argument,
                result,
            },
            span,
        );
        Ok(result)
    }

    fn subgroup_gather_helper(
        &mut self,
        span: Span,
        mode: SubgroupGather,
        arguments: &[Handle<ast::Expression<'source>>],
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Handle<ir::Expression>> {
        let mut args = ctx.prepare_args(arguments, 2, span);

        let argument = self.expression(args.next()?, ctx)?;

        use SubgroupGather as Sg;
        let mode = if let Sg::BroadcastFirst = mode {
            ir::GatherMode::BroadcastFirst
        } else {
            let index = self.expression(args.next()?, ctx)?;
            match mode {
                Sg::BroadcastFirst => unreachable!(),
                Sg::Broadcast => ir::GatherMode::Broadcast(index),
                Sg::Shuffle => ir::GatherMode::Shuffle(index),
                Sg::ShuffleDown => ir::GatherMode::ShuffleDown(index),
                Sg::ShuffleUp => ir::GatherMode::ShuffleUp(index),
                Sg::ShuffleXor => ir::GatherMode::ShuffleXor(index),
                Sg::QuadBroadcast => ir::GatherMode::QuadBroadcast(index),
            }
        };

        args.finish()?;

        let ty = ctx.register_type(argument)?;

        let result = ctx.interrupt_emitter(ir::Expression::SubgroupOperationResult { ty }, span)?;
        let rctx = ctx.runtime_expression_ctx(span)?;
        rctx.block.push(
            ir::Statement::SubgroupGather {
                mode,
                argument,
                result,
            },
            span,
        );
        Ok(result)
    }

    fn r#struct(
        &mut self,
        s: &ast::Struct<'source>,
        span: Span,
        ctx: &mut GlobalContext<'source, '_, '_>,
    ) -> Result<'source, Handle<ir::Type>> {
        let mut offset = 0;
        let mut struct_alignment = proc::Alignment::ONE;
        let mut members = Vec::with_capacity(s.members.len());

        let mut doc_comments: Vec<Option<Vec<String>>> = Vec::new();

        for member in s.members.iter() {
            let ty = self.resolve_ast_type(&member.ty, &mut ctx.as_const())?;

            ctx.layouter.update(ctx.module.to_ctx()).map_err(|err| {
                let LayoutErrorInner::TooLarge = err.inner else {
                    unreachable!("unexpected layout error: {err:?}");
                };
                // Since anonymous types of struct members don't get a span,
                // associate the error with the member. The layouter could have
                // failed on any type that was pending layout, but if it wasn't
                // the current struct member, it wasn't a struct member at all,
                // because we resolve struct members one-by-one.
                if ty == err.ty {
                    Box::new(Error::StructMemberTooLarge {
                        member_name_span: member.name.span,
                    })
                } else {
                    // Lots of type definitions don't get spans, so this error
                    // message may not be very useful.
                    Box::new(Error::TypeTooLarge {
                        span: ctx.module.types.get_span(err.ty),
                    })
                }
            })?;

            let member_min_size = ctx.layouter[ty].size;
            let member_min_alignment = ctx.layouter[ty].alignment;

            let member_size = if let Some(size_expr) = member.size {
                let (size, span) = self.const_u32(size_expr, &mut ctx.as_const())?;
                if size < member_min_size {
                    return Err(Box::new(Error::SizeAttributeTooLow(span, member_min_size)));
                } else {
                    size
                }
            } else {
                member_min_size
            };

            let member_alignment = if let Some(align_expr) = member.align {
                let (align, span) = self.const_u32(align_expr, &mut ctx.as_const())?;
                if let Some(alignment) = proc::Alignment::new(align) {
                    if alignment < member_min_alignment {
                        return Err(Box::new(Error::AlignAttributeTooLow(
                            span,
                            member_min_alignment,
                        )));
                    } else {
                        alignment
                    }
                } else {
                    return Err(Box::new(Error::NonPowerOfTwoAlignAttribute(span)));
                }
            } else {
                member_min_alignment
            };

            let binding = self.binding(&member.binding, ty, ctx)?;

            offset = member_alignment.round_up(offset);
            struct_alignment = struct_alignment.max(member_alignment);

            if !member.doc_comments.is_empty() {
                doc_comments.push(Some(
                    member.doc_comments.iter().map(|s| s.to_string()).collect(),
                ));
            }
            members.push(ir::StructMember {
                name: Some(member.name.name.to_owned()),
                ty,
                binding,
                offset,
            });

            offset += member_size;
            if offset > crate::valid::MAX_TYPE_SIZE {
                return Err(Box::new(Error::TypeTooLarge { span }));
            }
        }

        let size = struct_alignment.round_up(offset);
        let inner = ir::TypeInner::Struct {
            members,
            span: size,
        };

        let handle = ctx.module.types.insert(
            ir::Type {
                name: Some(s.name.name.to_string()),
                inner,
            },
            span,
        );
        for (i, c) in doc_comments.drain(..).enumerate() {
            if let Some(comment) = c {
                ctx.module
                    .get_or_insert_default_doc_comments()
                    .struct_members
                    .insert((handle, i), comment);
            }
        }
        Ok(handle)
    }

    fn const_u32(
        &mut self,
        expr: Handle<ast::Expression<'source>>,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, (u32, Span)> {
        let span = ctx.ast_expressions.get_span(expr);
        let expr = self.expression(expr, ctx)?;
        let value = ctx
            .module
            .to_ctx()
            .get_const_val(expr)
            .map_err(|err| match err {
                proc::ConstValueError::NonConst | proc::ConstValueError::InvalidType => {
                    Error::ExpectedConstExprConcreteIntegerScalar(span)
                }
                proc::ConstValueError::Negative => Error::ExpectedNonNegative(span),
            })?;
        Ok((value, span))
    }

    fn array_size(
        &mut self,
        expr: Handle<ast::Expression<'source>>,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, ir::ArraySize> {
        let span = ctx.ast_expressions.get_span(expr);
        let const_ctx = &mut ctx.as_const();
        let const_expr = self.expression(expr, const_ctx);
        match const_expr {
            Ok(value) => {
                let len = const_ctx.get_const_val(value).map_err(|err| {
                    Box::new(match err {
                        proc::ConstValueError::NonConst | proc::ConstValueError::InvalidType => {
                            Error::ExpectedConstExprConcreteIntegerScalar(span)
                        }
                        proc::ConstValueError::Negative => Error::ExpectedPositiveArrayLength(span),
                    })
                })?;
                let size = NonZeroU32::new(len).ok_or(Error::ExpectedPositiveArrayLength(span))?;
                Ok(ir::ArraySize::Constant(size))
            }
            Err(err) => {
                // If the error is simply that `expr` was an override expression, then we
                // can represent that as an array length.
                let Error::ConstantEvaluatorError(ref ty, _) = *err else {
                    return Err(err);
                };

                let proc::ConstantEvaluatorError::OverrideExpr = **ty else {
                    return Err(err);
                };

                Ok(ir::ArraySize::Pending(self.array_size_override(
                    expr,
                    &mut ctx.as_global().as_override(),
                    span,
                )?))
            }
        }
    }

    fn array_size_override(
        &mut self,
        size_expr: Handle<ast::Expression<'source>>,
        ctx: &mut ExpressionContext<'source, '_, '_>,
        span: Span,
    ) -> Result<'source, Handle<ir::Override>> {
        let expr = self.expression(size_expr, ctx)?;
        match resolve_inner!(ctx, expr).scalar_kind().ok_or(0) {
            Ok(ir::ScalarKind::Sint) | Ok(ir::ScalarKind::Uint) => Ok({
                if let ir::Expression::Override(handle) = ctx.module.global_expressions[expr] {
                    handle
                } else {
                    let ty = ctx.register_type(expr)?;
                    ctx.module.overrides.append(
                        ir::Override {
                            name: None,
                            id: None,
                            ty,
                            init: Some(expr),
                        },
                        span,
                    )
                }
            }),
            _ => Err(Box::new(Error::ExpectedConstExprConcreteIntegerScalar(
                span,
            ))),
        }
    }

    /// Build the Naga equivalent of a named AST type.
    ///
    /// Return a Naga `Handle<Type>` representing the front-end type
    /// `handle`, which should be named `name`, if given.
    ///
    /// If `handle` refers to a type cached in [`SpecialTypes`],
    /// `name` may be ignored.
    ///
    /// [`SpecialTypes`]: ir::SpecialTypes
    fn resolve_named_ast_type(
        &mut self,
        ident: &ast::TemplateElaboratedIdent<'source>,
        name: String,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Handle<ir::Type>> {
        self.type_specifier(ident, ctx, Some(name))
    }

    /// Return a Naga `Handle<Type>` representing the front-end type `handle`.
    fn resolve_ast_type(
        &mut self,
        ident: &ast::TemplateElaboratedIdent<'source>,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Handle<ir::Type>> {
        self.type_specifier(ident, ctx, None)
    }

    fn binding(
        &mut self,
        binding: &Option<ast::Binding<'source>>,
        ty: Handle<ir::Type>,
        ctx: &mut GlobalContext<'source, '_, '_>,
    ) -> Result<'source, Option<ir::Binding>> {
        Ok(match *binding {
            Some(ast::Binding::BuiltIn(b)) => Some(ir::Binding::BuiltIn(b)),
            Some(ast::Binding::Location {
                location,
                interpolation,
                sampling,
                blend_src,
                per_primitive,
            }) => {
                let blend_src = if let Some(blend_src) = blend_src {
                    Some(self.const_u32(blend_src, &mut ctx.as_const())?.0)
                } else {
                    None
                };

                let mut binding = ir::Binding::Location {
                    location: self.const_u32(location, &mut ctx.as_const())?.0,
                    interpolation,
                    sampling,
                    blend_src,
                    per_primitive,
                };
                binding.apply_default_interpolation(&ctx.module.types[ty].inner);
                Some(binding)
            }
            None => None,
        })
    }

    fn ray_query_pointer(
        &mut self,
        expr: Handle<ast::Expression<'source>>,
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Handle<ir::Expression>> {
        let span = ctx.ast_expressions.get_span(expr);
        let pointer = self.expression(expr, ctx)?;

        match *resolve_inner!(ctx, pointer) {
            ir::TypeInner::Pointer { base, .. } => match ctx.module.types[base].inner {
                ir::TypeInner::RayQuery { .. } => Ok(pointer),
                ref other => {
                    log::error!("Pointer type to {other:?} passed to ray query op");
                    Err(Box::new(Error::InvalidRayQueryPointer(span)))
                }
            },
            ref other => {
                log::error!("Type {other:?} passed to ray query op");
                Err(Box::new(Error::InvalidRayQueryPointer(span)))
            }
        }
    }
}

impl ir::AtomicFunction {
    pub fn map(word: &str) -> Option<Self> {
        Some(match word {
            "atomicAdd" => ir::AtomicFunction::Add,
            "atomicSub" => ir::AtomicFunction::Subtract,
            "atomicAnd" => ir::AtomicFunction::And,
            "atomicOr" => ir::AtomicFunction::InclusiveOr,
            "atomicXor" => ir::AtomicFunction::ExclusiveOr,
            "atomicMin" => ir::AtomicFunction::Min,
            "atomicMax" => ir::AtomicFunction::Max,
            "atomicExchange" => ir::AtomicFunction::Exchange { compare: None },
            _ => return None,
        })
    }
}
