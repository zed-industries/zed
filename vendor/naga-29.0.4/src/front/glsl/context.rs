use alloc::{format, string::String, vec::Vec};
use core::ops::Index;

use super::{
    ast::{
        GlobalLookup, GlobalLookupKind, HirExpr, HirExprKind, ParameterInfo, ParameterQualifier,
        VariableReference,
    },
    error::{Error, ErrorKind},
    types::{scalar_components, type_power},
    Frontend, Result,
};
use crate::{
    front::Typifier, proc::Emitter, proc::Layouter, AddressSpace, Arena, BinaryOperator, Block,
    Expression, FastHashMap, FunctionArgument, Handle, Literal, LocalVariable, RelationalFunction,
    Scalar, Span, Statement, Type, TypeInner, VectorSize,
};

/// The position at which an expression is, used while lowering
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExprPos {
    /// The expression is in the left hand side of an assignment
    Lhs,
    /// The expression is in the right hand side of an assignment
    Rhs,
    /// The expression is an array being indexed, needed to allow constant
    /// arrays to be dynamically indexed
    AccessBase {
        /// The index is a constant
        constant_index: bool,
    },
}

impl ExprPos {
    /// Returns an lhs position if the current position is lhs otherwise AccessBase
    const fn maybe_access_base(&self, constant_index: bool) -> Self {
        match *self {
            ExprPos::Lhs
            | ExprPos::AccessBase {
                constant_index: false,
            } => *self,
            _ => ExprPos::AccessBase { constant_index },
        }
    }
}

#[derive(Debug)]
pub(crate) struct Context<'a> {
    pub expressions: Arena<Expression>,
    pub locals: Arena<LocalVariable>,

    /// The [`FunctionArgument`]s for the final [`crate::Function`].
    ///
    /// Parameters with the `out` and `inout` qualifiers have [`Pointer`] types
    /// here. For example, an `inout vec2 a` argument would be a [`Pointer`] to
    /// a [`Vector`].
    ///
    /// [`Pointer`]: crate::TypeInner::Pointer
    /// [`Vector`]: crate::TypeInner::Vector
    pub arguments: Vec<FunctionArgument>,

    /// The parameter types given in the source code.
    ///
    /// The `out` and `inout` qualifiers don't affect the types that appear
    /// here. For example, an `inout vec2 a` argument would simply be a
    /// [`Vector`], not a pointer to one.
    ///
    /// [`Vector`]: crate::TypeInner::Vector
    pub parameters: Vec<Handle<Type>>,
    pub parameters_info: Vec<ParameterInfo>,

    pub symbol_table: crate::front::SymbolTable<String, VariableReference>,
    pub samplers: FastHashMap<Handle<Expression>, Handle<Expression>>,

    pub const_typifier: Typifier,
    pub typifier: Typifier,
    layouter: Layouter,
    emitter: Emitter,
    stmt_ctx: Option<StmtContext>,
    pub body: Block,
    pub module: &'a mut crate::Module,
    pub is_const: bool,
    /// Tracks the expression kind of `Expression`s residing in `self.expressions`
    pub local_expression_kind_tracker: crate::proc::ExpressionKindTracker,
    /// Tracks the expression kind of `Expression`s residing in `self.module.global_expressions`
    pub global_expression_kind_tracker: &'a mut crate::proc::ExpressionKindTracker,
}

impl<'a> Context<'a> {
    pub fn new(
        frontend: &Frontend,
        module: &'a mut crate::Module,
        is_const: bool,
        global_expression_kind_tracker: &'a mut crate::proc::ExpressionKindTracker,
    ) -> Result<Self> {
        let mut this = Context {
            expressions: Arena::new(),
            locals: Arena::new(),
            arguments: Vec::new(),

            parameters: Vec::new(),
            parameters_info: Vec::new(),

            symbol_table: crate::front::SymbolTable::default(),
            samplers: FastHashMap::default(),

            const_typifier: Typifier::new(),
            typifier: Typifier::new(),
            layouter: Layouter::default(),
            emitter: Emitter::default(),
            stmt_ctx: Some(StmtContext::new()),
            body: Block::new(),
            module,
            is_const: false,
            local_expression_kind_tracker: crate::proc::ExpressionKindTracker::new(),
            global_expression_kind_tracker,
        };

        this.emit_start();

        for &(ref name, lookup) in frontend.global_variables.iter() {
            this.add_global(name, lookup)?
        }
        this.is_const = is_const;

        Ok(this)
    }

    pub fn new_body<F>(&mut self, cb: F) -> Result<Block>
    where
        F: FnOnce(&mut Self) -> Result<()>,
    {
        self.new_body_with_ret(cb).map(|(b, _)| b)
    }

    pub fn new_body_with_ret<F, R>(&mut self, cb: F) -> Result<(Block, R)>
    where
        F: FnOnce(&mut Self) -> Result<R>,
    {
        self.emit_restart();
        let old_body = core::mem::replace(&mut self.body, Block::new());
        let res = cb(self);
        self.emit_restart();
        let new_body = core::mem::replace(&mut self.body, old_body);
        res.map(|r| (new_body, r))
    }

    pub fn with_body<F>(&mut self, body: Block, cb: F) -> Result<Block>
    where
        F: FnOnce(&mut Self) -> Result<()>,
    {
        self.emit_restart();
        let old_body = core::mem::replace(&mut self.body, body);
        let res = cb(self);
        self.emit_restart();
        let body = core::mem::replace(&mut self.body, old_body);
        res.map(|_| body)
    }

    pub fn add_global(
        &mut self,
        name: &str,
        GlobalLookup {
            kind,
            entry_arg,
            mutable,
        }: GlobalLookup,
    ) -> Result<()> {
        let (expr, load, constant) = match kind {
            GlobalLookupKind::Variable(v) => {
                let span = self.module.global_variables.get_span(v);
                (
                    self.add_expression(Expression::GlobalVariable(v), span)?,
                    self.module.global_variables[v].space != AddressSpace::Handle,
                    None,
                )
            }
            GlobalLookupKind::BlockSelect(handle, index) => {
                let span = self.module.global_variables.get_span(handle);
                let base = self.add_expression(Expression::GlobalVariable(handle), span)?;
                let expr = self.add_expression(Expression::AccessIndex { base, index }, span)?;

                (
                    expr,
                    {
                        let ty = self.module.global_variables[handle].ty;

                        match self.module.types[ty].inner {
                            TypeInner::Struct { ref members, .. } => {
                                if let TypeInner::Array {
                                    size: crate::ArraySize::Dynamic,
                                    ..
                                } = self.module.types[members[index as usize].ty].inner
                                {
                                    false
                                } else {
                                    true
                                }
                            }
                            _ => true,
                        }
                    },
                    None,
                )
            }
            GlobalLookupKind::Constant(v, ty) => {
                let span = self.module.constants.get_span(v);
                (
                    self.add_expression(Expression::Constant(v), span)?,
                    false,
                    Some((v, ty)),
                )
            }
            GlobalLookupKind::Override(v, _ty) => {
                let span = self.module.overrides.get_span(v);
                (
                    self.add_expression(Expression::Override(v), span)?,
                    false,
                    None,
                )
            }
        };

        let var = VariableReference {
            expr,
            load,
            mutable,
            constant,
            entry_arg,
        };

        self.symbol_table.add(name.into(), var);

        Ok(())
    }

    /// Starts the expression emitter
    ///
    /// # Panics
    ///
    /// - If called twice in a row without calling [`emit_end`][Self::emit_end].
    #[inline]
    pub fn emit_start(&mut self) {
        self.emitter.start(&self.expressions)
    }

    /// Emits all the expressions captured by the emitter to the current body
    ///
    /// # Panics
    ///
    /// - If called before calling [`emit_start`].
    /// - If called twice in a row without calling [`emit_start`].
    ///
    /// [`emit_start`]: Self::emit_start
    pub fn emit_end(&mut self) {
        self.body.extend(self.emitter.finish(&self.expressions))
    }

    /// Emits all the expressions captured by the emitter to the current body
    /// and starts the emitter again
    ///
    /// # Panics
    ///
    /// - If called before calling [`emit_start`][Self::emit_start].
    pub fn emit_restart(&mut self) {
        self.emit_end();
        self.emit_start()
    }

    pub fn add_expression(&mut self, expr: Expression, meta: Span) -> Result<Handle<Expression>> {
        let mut eval = if self.is_const {
            crate::proc::ConstantEvaluator::for_glsl_module(
                self.module,
                self.global_expression_kind_tracker,
                &mut self.layouter,
            )
        } else {
            crate::proc::ConstantEvaluator::for_glsl_function(
                self.module,
                &mut self.expressions,
                &mut self.local_expression_kind_tracker,
                &mut self.layouter,
                &mut self.emitter,
                &mut self.body,
            )
        };

        eval.try_eval_and_append(expr, meta).map_err(|e| Error {
            kind: e.into(),
            meta,
        })
    }

    /// Add variable to current scope
    ///
    /// Returns a variable if a variable with the same name was already defined,
    /// otherwise returns `None`
    pub fn add_local_var(
        &mut self,
        name: String,
        expr: Handle<Expression>,
        mutable: bool,
    ) -> Option<VariableReference> {
        let var = VariableReference {
            expr,
            load: true,
            mutable,
            constant: None,
            entry_arg: None,
        };

        self.symbol_table.add(name, var)
    }

    /// Add function argument to current scope
    pub fn add_function_arg(
        &mut self,
        name_meta: Option<(String, Span)>,
        ty: Handle<Type>,
        qualifier: ParameterQualifier,
    ) -> Result<()> {
        let index = self.arguments.len();
        let mut arg = FunctionArgument {
            name: name_meta.as_ref().map(|&(ref name, _)| name.clone()),
            ty,
            binding: None,
        };
        self.parameters.push(ty);

        let opaque = match self.module.types[ty].inner {
            TypeInner::Image { .. } | TypeInner::Sampler { .. } => true,
            _ => false,
        };

        if qualifier.is_lhs() {
            let span = self.module.types.get_span(arg.ty);
            arg.ty = self.module.types.insert(
                Type {
                    name: None,
                    inner: TypeInner::Pointer {
                        base: arg.ty,
                        space: AddressSpace::Function,
                    },
                },
                span,
            )
        }

        self.arguments.push(arg);

        self.parameters_info.push(ParameterInfo {
            qualifier,
            depth: false,
        });

        if let Some((name, meta)) = name_meta {
            let expr = self.add_expression(Expression::FunctionArgument(index as u32), meta)?;
            let mutable = qualifier != ParameterQualifier::Const && !opaque;
            let load = qualifier.is_lhs();

            let var = if mutable && !load {
                let handle = self.locals.append(
                    LocalVariable {
                        name: Some(name.clone()),
                        ty,
                        init: None,
                    },
                    meta,
                );
                let local_expr = self.add_expression(Expression::LocalVariable(handle), meta)?;

                self.emit_restart();

                self.body.push(
                    Statement::Store {
                        pointer: local_expr,
                        value: expr,
                    },
                    meta,
                );

                VariableReference {
                    expr: local_expr,
                    load: true,
                    mutable,
                    constant: None,
                    entry_arg: None,
                }
            } else {
                VariableReference {
                    expr,
                    load,
                    mutable,
                    constant: None,
                    entry_arg: None,
                }
            };

            self.symbol_table.add(name, var);
        }

        Ok(())
    }

    /// Returns a [`StmtContext`] to be used in parsing and lowering
    ///
    /// # Panics
    ///
    /// - If more than one [`StmtContext`] are active at the same time or if the
    ///   previous call didn't use it in lowering.
    #[must_use]
    pub const fn stmt_ctx(&mut self) -> StmtContext {
        self.stmt_ctx.take().unwrap()
    }

    /// Lowers a [`HirExpr`] which might produce a [`Expression`].
    ///
    /// consumes a [`StmtContext`] returning it to the context so that it can be
    /// used again later.
    pub fn lower(
        &mut self,
        mut stmt: StmtContext,
        frontend: &mut Frontend,
        expr: Handle<HirExpr>,
        pos: ExprPos,
    ) -> Result<(Option<Handle<Expression>>, Span)> {
        let res = self.lower_inner(&stmt, frontend, expr, pos);

        stmt.hir_exprs.clear();
        self.stmt_ctx = Some(stmt);

        res
    }

    /// Similar to [`lower`](Self::lower) but returns an error if the expression
    /// returns void (ie. doesn't produce a [`Expression`]).
    ///
    /// consumes a [`StmtContext`] returning it to the context so that it can be
    /// used again later.
    pub fn lower_expect(
        &mut self,
        mut stmt: StmtContext,
        frontend: &mut Frontend,
        expr: Handle<HirExpr>,
        pos: ExprPos,
    ) -> Result<(Handle<Expression>, Span)> {
        let res = self.lower_expect_inner(&stmt, frontend, expr, pos);

        stmt.hir_exprs.clear();
        self.stmt_ctx = Some(stmt);

        res
    }

    /// internal implementation of [`lower_expect`](Self::lower_expect)
    ///
    /// this method is only public because it's used in
    /// [`function_call`](Frontend::function_call), unless you know what
    /// you're doing use [`lower_expect`](Self::lower_expect)
    pub fn lower_expect_inner(
        &mut self,
        stmt: &StmtContext,
        frontend: &mut Frontend,
        expr: Handle<HirExpr>,
        pos: ExprPos,
    ) -> Result<(Handle<Expression>, Span)> {
        let (maybe_expr, meta) = self.lower_inner(stmt, frontend, expr, pos)?;

        let expr = match maybe_expr {
            Some(e) => e,
            None => {
                return Err(Error {
                    kind: ErrorKind::SemanticError("Expression returns void".into()),
                    meta,
                })
            }
        };

        Ok((expr, meta))
    }

    fn lower_store(
        &mut self,
        pointer: Handle<Expression>,
        value: Handle<Expression>,
        meta: Span,
    ) -> Result<()> {
        if let Expression::Swizzle {
            size,
            mut vector,
            pattern,
        } = self.expressions[pointer]
        {
            // Stores to swizzled values are not directly supported,
            // lower them as series of per-component stores.
            let size = match size {
                VectorSize::Bi => 2,
                VectorSize::Tri => 3,
                VectorSize::Quad => 4,
            };

            if let Expression::Load { pointer } = self.expressions[vector] {
                vector = pointer;
            }

            #[allow(clippy::needless_range_loop)]
            for index in 0..size {
                let dst = self.add_expression(
                    Expression::AccessIndex {
                        base: vector,
                        index: pattern[index].index(),
                    },
                    meta,
                )?;
                let src = self.add_expression(
                    Expression::AccessIndex {
                        base: value,
                        index: index as u32,
                    },
                    meta,
                )?;

                self.emit_restart();

                self.body.push(
                    Statement::Store {
                        pointer: dst,
                        value: src,
                    },
                    meta,
                );
            }
        } else {
            self.emit_restart();

            self.body.push(Statement::Store { pointer, value }, meta);
        }

        Ok(())
    }

    /// Internal implementation of [`lower`](Self::lower)
    fn lower_inner(
        &mut self,
        stmt: &StmtContext,
        frontend: &mut Frontend,
        expr: Handle<HirExpr>,
        pos: ExprPos,
    ) -> Result<(Option<Handle<Expression>>, Span)> {
        let HirExpr { ref kind, meta } = stmt.hir_exprs[expr];

        log::debug!("Lowering {expr:?} (kind {kind:?}, pos {pos:?})");

        let handle = match *kind {
            HirExprKind::Access { base, index } => {
                let (index, _) = self.lower_expect_inner(stmt, frontend, index, ExprPos::Rhs)?;
                let maybe_constant_index = match pos {
                    // Don't try to generate `AccessIndex` if in a LHS position, since it
                    // wouldn't produce a pointer.
                    ExprPos::Lhs => None,
                    _ => self
                        .module
                        .to_ctx()
                        .get_const_val_from(index, &self.expressions)
                        .ok(),
                };

                let base = self
                    .lower_expect_inner(
                        stmt,
                        frontend,
                        base,
                        pos.maybe_access_base(maybe_constant_index.is_some()),
                    )?
                    .0;

                let pointer = maybe_constant_index
                    .map(|index| self.add_expression(Expression::AccessIndex { base, index }, meta))
                    .unwrap_or_else(|| {
                        self.add_expression(Expression::Access { base, index }, meta)
                    })?;

                if ExprPos::Rhs == pos {
                    let resolved = self.resolve_type(pointer, meta)?;
                    if resolved.pointer_space().is_some() {
                        return Ok((
                            Some(self.add_expression(Expression::Load { pointer }, meta)?),
                            meta,
                        ));
                    }
                }

                pointer
            }
            HirExprKind::Select { base, ref field } => {
                let base = self.lower_expect_inner(stmt, frontend, base, pos)?.0;

                frontend.field_selection(self, pos, base, field, meta)?
            }
            HirExprKind::Literal(literal) if pos != ExprPos::Lhs => {
                self.add_expression(Expression::Literal(literal), meta)?
            }
            HirExprKind::Binary { left, op, right } if pos != ExprPos::Lhs => {
                let (mut left, left_meta) =
                    self.lower_expect_inner(stmt, frontend, left, ExprPos::Rhs)?;
                let (mut right, right_meta) =
                    self.lower_expect_inner(stmt, frontend, right, ExprPos::Rhs)?;

                match op {
                    BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight => {
                        self.implicit_conversion(&mut right, right_meta, Scalar::U32)?
                    }
                    _ => self
                        .binary_implicit_conversion(&mut left, left_meta, &mut right, right_meta)?,
                }

                self.typifier_grow(left, left_meta)?;
                self.typifier_grow(right, right_meta)?;

                let left_inner = self.get_type(left);
                let right_inner = self.get_type(right);

                match (left_inner, right_inner) {
                    (
                        &TypeInner::Matrix {
                            columns: left_columns,
                            rows: left_rows,
                            scalar: left_scalar,
                        },
                        &TypeInner::Matrix {
                            columns: right_columns,
                            rows: right_rows,
                            scalar: right_scalar,
                        },
                    ) => {
                        let dimensions_ok = if op == BinaryOperator::Multiply {
                            left_columns == right_rows
                        } else {
                            left_columns == right_columns && left_rows == right_rows
                        };

                        // Check that the two arguments have the same dimensions
                        if !dimensions_ok || left_scalar != right_scalar {
                            frontend.errors.push(Error {
                                kind: ErrorKind::SemanticError(
                                    format!(
                                        "Cannot apply operation to {left_inner:?} and {right_inner:?}"
                                    )
                                    .into(),
                                ),
                                meta,
                            })
                        }

                        match op {
                            BinaryOperator::Divide => {
                                // Naga IR doesn't support matrix division so we need to
                                // divide the columns individually and reassemble the matrix
                                let mut components = Vec::with_capacity(left_columns as usize);

                                for index in 0..left_columns as u32 {
                                    // Get the column vectors
                                    let left_vector = self.add_expression(
                                        Expression::AccessIndex { base: left, index },
                                        meta,
                                    )?;
                                    let right_vector = self.add_expression(
                                        Expression::AccessIndex { base: right, index },
                                        meta,
                                    )?;

                                    // Divide the vectors
                                    let column = self.add_expression(
                                        Expression::Binary {
                                            op,
                                            left: left_vector,
                                            right: right_vector,
                                        },
                                        meta,
                                    )?;

                                    components.push(column)
                                }

                                let ty = self.module.types.insert(
                                    Type {
                                        name: None,
                                        inner: TypeInner::Matrix {
                                            columns: left_columns,
                                            rows: left_rows,
                                            scalar: left_scalar,
                                        },
                                    },
                                    Span::default(),
                                );

                                // Rebuild the matrix from the divided vectors
                                self.add_expression(Expression::Compose { ty, components }, meta)?
                            }
                            BinaryOperator::Equal | BinaryOperator::NotEqual => {
                                // Naga IR doesn't support matrix comparisons so we need to
                                // compare the columns individually and then fold them together
                                //
                                // The folding is done using a logical and for equality and
                                // a logical or for inequality
                                let equals = op == BinaryOperator::Equal;

                                let (op, combine, fun) = match equals {
                                    true => (
                                        BinaryOperator::Equal,
                                        BinaryOperator::LogicalAnd,
                                        RelationalFunction::All,
                                    ),
                                    false => (
                                        BinaryOperator::NotEqual,
                                        BinaryOperator::LogicalOr,
                                        RelationalFunction::Any,
                                    ),
                                };

                                let mut root = None;

                                for index in 0..left_columns as u32 {
                                    // Get the column vectors
                                    let left_vector = self.add_expression(
                                        Expression::AccessIndex { base: left, index },
                                        meta,
                                    )?;
                                    let right_vector = self.add_expression(
                                        Expression::AccessIndex { base: right, index },
                                        meta,
                                    )?;

                                    let argument = self.add_expression(
                                        Expression::Binary {
                                            op,
                                            left: left_vector,
                                            right: right_vector,
                                        },
                                        meta,
                                    )?;

                                    // The result of comparing two vectors is a boolean vector
                                    // so use a relational function like all to get a single
                                    // boolean value
                                    let compare = self.add_expression(
                                        Expression::Relational { fun, argument },
                                        meta,
                                    )?;

                                    // Fold the result
                                    root = Some(match root {
                                        Some(right) => self.add_expression(
                                            Expression::Binary {
                                                op: combine,
                                                left: compare,
                                                right,
                                            },
                                            meta,
                                        )?,
                                        None => compare,
                                    });
                                }

                                root.unwrap()
                            }
                            _ => {
                                self.add_expression(Expression::Binary { left, op, right }, meta)?
                            }
                        }
                    }
                    (&TypeInner::Vector { .. }, &TypeInner::Vector { .. }) => match op {
                        BinaryOperator::Equal | BinaryOperator::NotEqual => {
                            let equals = op == BinaryOperator::Equal;

                            let (op, fun) = match equals {
                                true => (BinaryOperator::Equal, RelationalFunction::All),
                                false => (BinaryOperator::NotEqual, RelationalFunction::Any),
                            };

                            let argument =
                                self.add_expression(Expression::Binary { op, left, right }, meta)?;

                            self.add_expression(Expression::Relational { fun, argument }, meta)?
                        }
                        _ => self.add_expression(Expression::Binary { left, op, right }, meta)?,
                    },
                    (&TypeInner::Vector { size, .. }, &TypeInner::Scalar { .. }) => match op {
                        BinaryOperator::Add
                        | BinaryOperator::Subtract
                        | BinaryOperator::Divide
                        | BinaryOperator::And
                        | BinaryOperator::ExclusiveOr
                        | BinaryOperator::InclusiveOr
                        | BinaryOperator::ShiftLeft
                        | BinaryOperator::ShiftRight => {
                            let scalar_vector = self
                                .add_expression(Expression::Splat { size, value: right }, meta)?;

                            self.add_expression(
                                Expression::Binary {
                                    op,
                                    left,
                                    right: scalar_vector,
                                },
                                meta,
                            )?
                        }
                        _ => self.add_expression(Expression::Binary { left, op, right }, meta)?,
                    },
                    (&TypeInner::Scalar { .. }, &TypeInner::Vector { size, .. }) => match op {
                        BinaryOperator::Add
                        | BinaryOperator::Subtract
                        | BinaryOperator::Divide
                        | BinaryOperator::And
                        | BinaryOperator::ExclusiveOr
                        | BinaryOperator::InclusiveOr => {
                            let scalar_vector =
                                self.add_expression(Expression::Splat { size, value: left }, meta)?;

                            self.add_expression(
                                Expression::Binary {
                                    op,
                                    left: scalar_vector,
                                    right,
                                },
                                meta,
                            )?
                        }
                        _ => self.add_expression(Expression::Binary { left, op, right }, meta)?,
                    },
                    (
                        &TypeInner::Scalar(left_scalar),
                        &TypeInner::Matrix {
                            rows,
                            columns,
                            scalar: right_scalar,
                        },
                    ) => {
                        // Check that the two arguments have the same scalar type
                        if left_scalar != right_scalar {
                            frontend.errors.push(Error {
                                kind: ErrorKind::SemanticError(
                                    format!(
                                        "Cannot apply operation to {left_inner:?} and {right_inner:?}"
                                    )
                                    .into(),
                                ),
                                meta,
                            })
                        }

                        match op {
                            BinaryOperator::Divide
                            | BinaryOperator::Add
                            | BinaryOperator::Subtract => {
                                // Naga IR doesn't support all matrix by scalar operations so
                                // we need for some to turn the scalar into a vector by
                                // splatting it and then for each column vector apply the
                                // operation and finally reconstruct the matrix
                                let scalar_vector = self.add_expression(
                                    Expression::Splat {
                                        size: rows,
                                        value: left,
                                    },
                                    meta,
                                )?;

                                let mut components = Vec::with_capacity(columns as usize);

                                for index in 0..columns as u32 {
                                    // Get the column vector
                                    let matrix_column = self.add_expression(
                                        Expression::AccessIndex { base: right, index },
                                        meta,
                                    )?;

                                    // Apply the operation to the splatted vector and
                                    // the column vector
                                    let column = self.add_expression(
                                        Expression::Binary {
                                            op,
                                            left: scalar_vector,
                                            right: matrix_column,
                                        },
                                        meta,
                                    )?;

                                    components.push(column)
                                }

                                let ty = self.module.types.insert(
                                    Type {
                                        name: None,
                                        inner: TypeInner::Matrix {
                                            columns,
                                            rows,
                                            scalar: left_scalar,
                                        },
                                    },
                                    Span::default(),
                                );

                                // Rebuild the matrix from the operation result vectors
                                self.add_expression(Expression::Compose { ty, components }, meta)?
                            }
                            _ => {
                                self.add_expression(Expression::Binary { left, op, right }, meta)?
                            }
                        }
                    }
                    (
                        &TypeInner::Matrix {
                            rows,
                            columns,
                            scalar: left_scalar,
                        },
                        &TypeInner::Scalar(right_scalar),
                    ) => {
                        // Check that the two arguments have the same scalar type
                        if left_scalar != right_scalar {
                            frontend.errors.push(Error {
                                kind: ErrorKind::SemanticError(
                                    format!(
                                        "Cannot apply operation to {left_inner:?} and {right_inner:?}"
                                    )
                                    .into(),
                                ),
                                meta,
                            })
                        }

                        match op {
                            BinaryOperator::Divide
                            | BinaryOperator::Add
                            | BinaryOperator::Subtract => {
                                // Naga IR doesn't support all matrix by scalar operations so
                                // we need for some to turn the scalar into a vector by
                                // splatting it and then for each column vector apply the
                                // operation and finally reconstruct the matrix

                                let scalar_vector = self.add_expression(
                                    Expression::Splat {
                                        size: rows,
                                        value: right,
                                    },
                                    meta,
                                )?;

                                let mut components = Vec::with_capacity(columns as usize);

                                for index in 0..columns as u32 {
                                    // Get the column vector
                                    let matrix_column = self.add_expression(
                                        Expression::AccessIndex { base: left, index },
                                        meta,
                                    )?;

                                    // Apply the operation to the splatted vector and
                                    // the column vector
                                    let column = self.add_expression(
                                        Expression::Binary {
                                            op,
                                            left: matrix_column,
                                            right: scalar_vector,
                                        },
                                        meta,
                                    )?;

                                    components.push(column)
                                }

                                let ty = self.module.types.insert(
                                    Type {
                                        name: None,
                                        inner: TypeInner::Matrix {
                                            columns,
                                            rows,
                                            scalar: left_scalar,
                                        },
                                    },
                                    Span::default(),
                                );

                                // Rebuild the matrix from the operation result vectors
                                self.add_expression(Expression::Compose { ty, components }, meta)?
                            }
                            _ => {
                                self.add_expression(Expression::Binary { left, op, right }, meta)?
                            }
                        }
                    }
                    _ => self.add_expression(Expression::Binary { left, op, right }, meta)?,
                }
            }
            HirExprKind::Unary { op, expr } if pos != ExprPos::Lhs => {
                let expr = self
                    .lower_expect_inner(stmt, frontend, expr, ExprPos::Rhs)?
                    .0;

                if let TypeInner::Matrix { scalar, .. } = *self.resolve_type(expr, meta)? {
                    // Naga IR doesn't support matrix negation, so we need to turn it into
                    // multiplication by scalar -1.
                    let minus_one = Literal::minus_one(scalar).ok_or_else(|| Error {
                        kind: ErrorKind::SemanticError(
                            format!("Cannot apply operator {op:?} to type {scalar:?}").into(),
                        ),
                        meta,
                    })?;
                    let lhs = self.add_expression(Expression::Literal(minus_one), meta)?;
                    self.add_expression(
                        Expression::Binary {
                            op: BinaryOperator::Multiply,
                            left: lhs,
                            right: expr,
                        },
                        meta,
                    )?
                } else {
                    self.add_expression(Expression::Unary { op, expr }, meta)?
                }
            }
            HirExprKind::Variable(ref var) => match pos {
                ExprPos::Lhs => {
                    if !var.mutable {
                        frontend.errors.push(Error {
                            kind: ErrorKind::SemanticError(
                                "Variable cannot be used in LHS position".into(),
                            ),
                            meta,
                        })
                    }

                    var.expr
                }
                ExprPos::AccessBase { constant_index } => {
                    // If the index isn't constant all accesses backed by a constant base need
                    // to be done through a proxy local variable, since constants have a non
                    // pointer type which is required for dynamic indexing
                    if !constant_index {
                        if let Some((constant, ty)) = var.constant {
                            let init = self
                                .add_expression(Expression::Constant(constant), Span::default())?;
                            let local = self.locals.append(
                                LocalVariable {
                                    name: None,
                                    ty,
                                    init: Some(init),
                                },
                                Span::default(),
                            );

                            self.add_expression(Expression::LocalVariable(local), Span::default())?
                        } else {
                            var.expr
                        }
                    } else {
                        var.expr
                    }
                }
                _ if var.load => {
                    self.add_expression(Expression::Load { pointer: var.expr }, meta)?
                }
                ExprPos::Rhs => {
                    if let Some((constant, _)) = self.is_const.then_some(var.constant).flatten() {
                        self.add_expression(Expression::Constant(constant), meta)?
                    } else {
                        // Check if this is an Override expression in const context
                        if self.is_const {
                            if let Expression::Override(o) = self.expressions[var.expr] {
                                // Need to add the Override expression to the global arena
                                self.add_expression(Expression::Override(o), meta)?
                            } else {
                                var.expr
                            }
                        } else {
                            var.expr
                        }
                    }
                }
            },
            HirExprKind::Call(ref call) if pos != ExprPos::Lhs => {
                let maybe_expr = frontend.function_or_constructor_call(
                    self,
                    stmt,
                    call.kind.clone(),
                    &call.args,
                    meta,
                )?;
                return Ok((maybe_expr, meta));
            }
            // `HirExprKind::Conditional` represents the ternary operator in glsl (`:?`)
            //
            // The ternary operator is defined to only evaluate one of the two possible
            // expressions which means that it's behavior is that of an `if` statement,
            // and it's merely syntactic sugar for it.
            HirExprKind::Conditional {
                condition,
                accept,
                reject,
            } if ExprPos::Lhs != pos => {
                // Given an expression `a ? b : c`, we need to produce a Naga
                // statement roughly like:
                //
                //     var temp;
                //     if a {
                //         temp = convert(b);
                //     } else  {
                //         temp = convert(c);
                //     }
                //
                // where `convert` stands for type conversions to bring `b` and `c` to
                // the same type, and then use `temp` to represent the value of the whole
                // conditional expression in subsequent code.

                // Lower the condition first to the current bodyy
                let condition = self
                    .lower_expect_inner(stmt, frontend, condition, ExprPos::Rhs)?
                    .0;

                let (mut accept_body, (mut accept, accept_meta)) =
                    self.new_body_with_ret(|ctx| {
                        // Lower the `true` branch
                        ctx.lower_expect_inner(stmt, frontend, accept, pos)
                    })?;

                let (mut reject_body, (mut reject, reject_meta)) =
                    self.new_body_with_ret(|ctx| {
                        // Lower the `false` branch
                        ctx.lower_expect_inner(stmt, frontend, reject, pos)
                    })?;

                // We need to do some custom implicit conversions since the two target expressions
                // are in different bodies
                if let (Some((accept_power, accept_scalar)), Some((reject_power, reject_scalar))) = (
                    // Get the components of both branches and calculate the type power
                    self.expr_scalar_components(accept, accept_meta)?
                        .and_then(|scalar| Some((type_power(scalar)?, scalar))),
                    self.expr_scalar_components(reject, reject_meta)?
                        .and_then(|scalar| Some((type_power(scalar)?, scalar))),
                ) {
                    match accept_power.cmp(&reject_power) {
                        core::cmp::Ordering::Less => {
                            accept_body = self.with_body(accept_body, |ctx| {
                                ctx.conversion(&mut accept, accept_meta, reject_scalar)?;
                                Ok(())
                            })?;
                        }
                        core::cmp::Ordering::Equal => {}
                        core::cmp::Ordering::Greater => {
                            reject_body = self.with_body(reject_body, |ctx| {
                                ctx.conversion(&mut reject, reject_meta, accept_scalar)?;
                                Ok(())
                            })?;
                        }
                    }
                }

                // We need to get the type of the resulting expression to create the local,
                // this must be done after implicit conversions to ensure both branches have
                // the same type.
                let ty = self.resolve_type_handle(accept, accept_meta)?;

                // Add the local that will hold the result of our conditional
                let local = self.locals.append(
                    LocalVariable {
                        name: None,
                        ty,
                        init: None,
                    },
                    meta,
                );

                let local_expr = self.add_expression(Expression::LocalVariable(local), meta)?;

                // Add to each  the store to the result variable
                accept_body.push(
                    Statement::Store {
                        pointer: local_expr,
                        value: accept,
                    },
                    accept_meta,
                );
                reject_body.push(
                    Statement::Store {
                        pointer: local_expr,
                        value: reject,
                    },
                    reject_meta,
                );

                // Finally add the `If` to the main body with the `condition` we lowered
                // earlier and the branches we prepared.
                self.body.push(
                    Statement::If {
                        condition,
                        accept: accept_body,
                        reject: reject_body,
                    },
                    meta,
                );

                // Note: `Expression::Load` must be emitted before it's used so make
                // sure the emitter is active here.
                self.add_expression(
                    Expression::Load {
                        pointer: local_expr,
                    },
                    meta,
                )?
            }
            HirExprKind::Assign { tgt, value } if ExprPos::Lhs != pos => {
                let (pointer, ptr_meta) =
                    self.lower_expect_inner(stmt, frontend, tgt, ExprPos::Lhs)?;
                let (mut value, value_meta) =
                    self.lower_expect_inner(stmt, frontend, value, ExprPos::Rhs)?;

                let ty = match *self.resolve_type(pointer, ptr_meta)? {
                    TypeInner::Pointer { base, .. } => &self.module.types[base].inner,
                    ref ty => ty,
                };

                if let Some(scalar) = scalar_components(ty) {
                    self.implicit_conversion(&mut value, value_meta, scalar)?;
                }

                self.lower_store(pointer, value, meta)?;

                value
            }
            HirExprKind::PrePostfix { op, postfix, expr } if ExprPos::Lhs != pos => {
                let (pointer, _) = self.lower_expect_inner(stmt, frontend, expr, ExprPos::Lhs)?;
                let left = if let Expression::Swizzle { .. } = self.expressions[pointer] {
                    pointer
                } else {
                    self.add_expression(Expression::Load { pointer }, meta)?
                };

                let res = match *self.resolve_type(left, meta)? {
                    TypeInner::Scalar(scalar) => {
                        let ty = TypeInner::Scalar(scalar);
                        Literal::one(scalar).map(|i| (ty, i, None, None))
                    }
                    TypeInner::Vector { size, scalar } => {
                        let ty = TypeInner::Vector { size, scalar };
                        Literal::one(scalar).map(|i| (ty, i, Some(size), None))
                    }
                    TypeInner::Matrix {
                        columns,
                        rows,
                        scalar,
                    } => {
                        let ty = TypeInner::Matrix {
                            columns,
                            rows,
                            scalar,
                        };
                        Literal::one(scalar).map(|i| (ty, i, Some(rows), Some(columns)))
                    }
                    _ => None,
                };
                let (ty_inner, literal, rows, columns) = match res {
                    Some(res) => res,
                    None => {
                        frontend.errors.push(Error {
                            kind: ErrorKind::SemanticError(
                                "Increment/decrement only works on scalar/vector/matrix".into(),
                            ),
                            meta,
                        });
                        return Ok((Some(left), meta));
                    }
                };

                let mut right = self.add_expression(Expression::Literal(literal), meta)?;

                // Glsl allows pre/postfixes operations on vectors and matrices, so if the
                // target is either of them change the right side of the addition to be splatted
                // to the same size as the target, furthermore if the target is a matrix
                // use a composed matrix using the splatted value.
                if let Some(size) = rows {
                    right = self.add_expression(Expression::Splat { size, value: right }, meta)?;

                    if let Some(cols) = columns {
                        let ty = self.module.types.insert(
                            Type {
                                name: None,
                                inner: ty_inner,
                            },
                            meta,
                        );

                        right = self.add_expression(
                            Expression::Compose {
                                ty,
                                components: core::iter::repeat_n(right, cols as usize).collect(),
                            },
                            meta,
                        )?;
                    }
                }

                let value = self.add_expression(Expression::Binary { op, left, right }, meta)?;

                self.lower_store(pointer, value, meta)?;

                if postfix {
                    left
                } else {
                    value
                }
            }
            HirExprKind::Method {
                expr: object,
                ref name,
                ref args,
            } if ExprPos::Lhs != pos => {
                let args = args
                    .iter()
                    .map(|e| self.lower_expect_inner(stmt, frontend, *e, ExprPos::Rhs))
                    .collect::<Result<Vec<_>>>()?;
                match name.as_ref() {
                    "length" => {
                        if !args.is_empty() {
                            frontend.errors.push(Error {
                                kind: ErrorKind::SemanticError(
                                    ".length() doesn't take any arguments".into(),
                                ),
                                meta,
                            });
                        }
                        let lowered_array = self.lower_expect_inner(stmt, frontend, object, pos)?.0;
                        let array_type = self.resolve_type(lowered_array, meta)?;

                        match *array_type {
                            TypeInner::Array {
                                size: crate::ArraySize::Constant(size),
                                ..
                            } => {
                                let mut array_length = self.add_expression(
                                    Expression::Literal(Literal::U32(size.get())),
                                    meta,
                                )?;
                                self.forced_conversion(&mut array_length, meta, Scalar::I32)?;
                                array_length
                            }
                            // let the error be handled in type checking if it's not a dynamic array
                            _ => {
                                let mut array_length = self
                                    .add_expression(Expression::ArrayLength(lowered_array), meta)?;
                                self.conversion(&mut array_length, meta, Scalar::I32)?;
                                array_length
                            }
                        }
                    }

                    _ => {
                        return Err(Error {
                            kind: ErrorKind::SemanticError(
                                format!("unknown method '{name}'").into(),
                            ),
                            meta,
                        });
                    }
                }
            }
            HirExprKind::Sequence { ref exprs } if pos != ExprPos::Lhs => {
                let mut last_handle = None;
                for expr in exprs.iter() {
                    let (handle, _) =
                        self.lower_expect_inner(stmt, frontend, *expr, ExprPos::Rhs)?;
                    last_handle = Some(handle);
                }
                match last_handle {
                    Some(handle) => handle,
                    None => unreachable!(),
                }
            }
            _ => {
                return Err(Error {
                    kind: ErrorKind::SemanticError(
                        format!("{:?} cannot be in the left hand side", stmt.hir_exprs[expr])
                            .into(),
                    ),
                    meta,
                })
            }
        };

        log::trace!("Lowered {expr:?}\n\tKind = {kind:?}\n\tPos = {pos:?}\n\tResult = {handle:?}");

        Ok((Some(handle), meta))
    }

    pub fn expr_scalar_components(
        &mut self,
        expr: Handle<Expression>,
        meta: Span,
    ) -> Result<Option<Scalar>> {
        let ty = self.resolve_type(expr, meta)?;
        Ok(scalar_components(ty))
    }

    pub fn expr_power(&mut self, expr: Handle<Expression>, meta: Span) -> Result<Option<u32>> {
        Ok(self
            .expr_scalar_components(expr, meta)?
            .and_then(type_power))
    }

    pub fn conversion(
        &mut self,
        expr: &mut Handle<Expression>,
        meta: Span,
        scalar: Scalar,
    ) -> Result<()> {
        *expr = self.add_expression(
            Expression::As {
                expr: *expr,
                kind: scalar.kind,
                convert: Some(scalar.width),
            },
            meta,
        )?;

        Ok(())
    }

    pub fn implicit_conversion(
        &mut self,
        expr: &mut Handle<Expression>,
        meta: Span,
        scalar: Scalar,
    ) -> Result<()> {
        if let (Some(tgt_power), Some(expr_power)) =
            (type_power(scalar), self.expr_power(*expr, meta)?)
        {
            if tgt_power > expr_power {
                self.conversion(expr, meta, scalar)?;
            }
        }

        Ok(())
    }

    pub fn forced_conversion(
        &mut self,
        expr: &mut Handle<Expression>,
        meta: Span,
        scalar: Scalar,
    ) -> Result<()> {
        if let Some(expr_scalar) = self.expr_scalar_components(*expr, meta)? {
            if expr_scalar != scalar {
                self.conversion(expr, meta, scalar)?;
            }
        }

        Ok(())
    }

    pub fn binary_implicit_conversion(
        &mut self,
        left: &mut Handle<Expression>,
        left_meta: Span,
        right: &mut Handle<Expression>,
        right_meta: Span,
    ) -> Result<()> {
        let left_components = self.expr_scalar_components(*left, left_meta)?;
        let right_components = self.expr_scalar_components(*right, right_meta)?;

        if let (Some((left_power, left_scalar)), Some((right_power, right_scalar))) = (
            left_components.and_then(|scalar| Some((type_power(scalar)?, scalar))),
            right_components.and_then(|scalar| Some((type_power(scalar)?, scalar))),
        ) {
            match left_power.cmp(&right_power) {
                core::cmp::Ordering::Less => {
                    self.conversion(left, left_meta, right_scalar)?;
                }
                core::cmp::Ordering::Equal => {}
                core::cmp::Ordering::Greater => {
                    self.conversion(right, right_meta, left_scalar)?;
                }
            }
        }

        Ok(())
    }

    pub fn implicit_splat(
        &mut self,
        expr: &mut Handle<Expression>,
        meta: Span,
        vector_size: Option<VectorSize>,
    ) -> Result<()> {
        let expr_type = self.resolve_type(*expr, meta)?;

        if let (&TypeInner::Scalar { .. }, Some(size)) = (expr_type, vector_size) {
            *expr = self.add_expression(Expression::Splat { size, value: *expr }, meta)?
        }

        Ok(())
    }

    pub fn vector_resize(
        &mut self,
        size: VectorSize,
        vector: Handle<Expression>,
        meta: Span,
    ) -> Result<Handle<Expression>> {
        self.add_expression(
            Expression::Swizzle {
                size,
                vector,
                pattern: crate::SwizzleComponent::XYZW,
            },
            meta,
        )
    }
}

impl Index<Handle<Expression>> for Context<'_> {
    type Output = Expression;

    fn index(&self, index: Handle<Expression>) -> &Self::Output {
        if self.is_const {
            &self.module.global_expressions[index]
        } else {
            &self.expressions[index]
        }
    }
}

/// Helper struct passed when parsing expressions
///
/// This struct should only be obtained through [`stmt_ctx`](Context::stmt_ctx)
/// and only one of these may be active at any time per context.
#[derive(Debug)]
pub struct StmtContext {
    /// A arena of high level expressions which can be lowered through a
    /// [`Context`] to Naga's [`Expression`]s
    pub hir_exprs: Arena<HirExpr>,
}

impl StmtContext {
    const fn new() -> Self {
        StmtContext {
            hir_exprs: Arena::new(),
        }
    }
}
