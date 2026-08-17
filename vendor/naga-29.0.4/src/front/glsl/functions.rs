use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::iter;

use super::{
    ast::*,
    builtins::{inject_builtin, sampled_to_depth},
    context::{Context, ExprPos, StmtContext},
    error::{Error, ErrorKind},
    types::scalar_components,
    Frontend, Result,
};
use crate::{
    front::glsl::types::type_power, proc::ensure_block_returns, AddressSpace, Block, EntryPoint,
    Expression, Function, FunctionArgument, FunctionResult, Handle, Literal, LocalVariable, Scalar,
    ScalarKind, Span, Statement, StructMember, Type, TypeInner,
};

/// Struct detailing a store operation that must happen after a function call
struct ProxyWrite {
    /// The store target
    target: Handle<Expression>,
    /// A pointer to read the value of the store
    value: Handle<Expression>,
    /// An optional conversion to be applied
    convert: Option<Scalar>,
}

impl Frontend {
    pub(crate) fn function_or_constructor_call(
        &mut self,
        ctx: &mut Context,
        stmt: &StmtContext,
        fc: FunctionCallKind,
        raw_args: &[Handle<HirExpr>],
        meta: Span,
    ) -> Result<Option<Handle<Expression>>> {
        let args: Vec<_> = raw_args
            .iter()
            .map(|e| ctx.lower_expect_inner(stmt, self, *e, ExprPos::Rhs))
            .collect::<Result<_>>()?;

        match fc {
            FunctionCallKind::TypeConstructor(ty) => {
                if args.len() == 1 {
                    self.constructor_single(ctx, ty, args[0], meta).map(Some)
                } else {
                    self.constructor_many(ctx, ty, args, meta).map(Some)
                }
            }
            FunctionCallKind::Function(name) => {
                self.function_call(ctx, stmt, name, args, raw_args, meta)
            }
        }
    }

    fn constructor_single(
        &mut self,
        ctx: &mut Context,
        ty: Handle<Type>,
        (mut value, expr_meta): (Handle<Expression>, Span),
        meta: Span,
    ) -> Result<Handle<Expression>> {
        let expr_type = ctx.resolve_type(value, expr_meta)?;

        let vector_size = match *expr_type {
            TypeInner::Vector { size, .. } => Some(size),
            _ => None,
        };

        let expr_is_bool = expr_type.scalar_kind() == Some(ScalarKind::Bool);

        // Special case: if casting from a bool, we need to use Select and not As.
        match ctx.module.types[ty].inner.scalar() {
            Some(result_scalar) if expr_is_bool && result_scalar.kind != ScalarKind::Bool => {
                let result_scalar = Scalar {
                    width: 4,
                    ..result_scalar
                };
                let l0 = Literal::zero(result_scalar).unwrap();
                let l1 = Literal::one(result_scalar).unwrap();
                let mut reject = ctx.add_expression(Expression::Literal(l0), expr_meta)?;
                let mut accept = ctx.add_expression(Expression::Literal(l1), expr_meta)?;

                ctx.implicit_splat(&mut reject, meta, vector_size)?;
                ctx.implicit_splat(&mut accept, meta, vector_size)?;

                let h = ctx.add_expression(
                    Expression::Select {
                        accept,
                        reject,
                        condition: value,
                    },
                    expr_meta,
                )?;

                return Ok(h);
            }
            _ => {}
        }

        Ok(match ctx.module.types[ty].inner {
            TypeInner::Vector { size, scalar } if vector_size.is_none() => {
                ctx.forced_conversion(&mut value, expr_meta, scalar)?;

                if let TypeInner::Scalar { .. } = *ctx.resolve_type(value, expr_meta)? {
                    ctx.add_expression(Expression::Splat { size, value }, meta)?
                } else {
                    self.vector_constructor(ctx, ty, size, scalar, &[(value, expr_meta)], meta)?
                }
            }
            TypeInner::Scalar(scalar) => {
                let mut expr = value;
                if let TypeInner::Vector { .. } | TypeInner::Matrix { .. } =
                    *ctx.resolve_type(value, expr_meta)?
                {
                    expr = ctx.add_expression(
                        Expression::AccessIndex {
                            base: expr,
                            index: 0,
                        },
                        meta,
                    )?;
                }

                if let TypeInner::Matrix { .. } = *ctx.resolve_type(value, expr_meta)? {
                    expr = ctx.add_expression(
                        Expression::AccessIndex {
                            base: expr,
                            index: 0,
                        },
                        meta,
                    )?;
                }

                ctx.add_expression(
                    Expression::As {
                        kind: scalar.kind,
                        expr,
                        convert: Some(scalar.width),
                    },
                    meta,
                )?
            }
            TypeInner::Vector { size, scalar } => {
                if vector_size != Some(size) {
                    value = ctx.vector_resize(size, value, expr_meta)?;
                }

                ctx.add_expression(
                    Expression::As {
                        kind: scalar.kind,
                        expr: value,
                        convert: Some(scalar.width),
                    },
                    meta,
                )?
            }
            TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } => self.matrix_one_arg(ctx, ty, columns, rows, scalar, (value, expr_meta), meta)?,
            TypeInner::Struct { ref members, .. } => {
                let scalar_components = members
                    .first()
                    .and_then(|member| scalar_components(&ctx.module.types[member.ty].inner));
                if let Some(scalar) = scalar_components {
                    ctx.implicit_conversion(&mut value, expr_meta, scalar)?;
                }

                ctx.add_expression(
                    Expression::Compose {
                        ty,
                        components: vec![value],
                    },
                    meta,
                )?
            }

            TypeInner::Array { base, .. } => {
                let scalar_components = scalar_components(&ctx.module.types[base].inner);
                if let Some(scalar) = scalar_components {
                    ctx.implicit_conversion(&mut value, expr_meta, scalar)?;
                }

                ctx.add_expression(
                    Expression::Compose {
                        ty,
                        components: vec![value],
                    },
                    meta,
                )?
            }
            _ => {
                self.errors.push(Error {
                    kind: ErrorKind::SemanticError("Bad type constructor".into()),
                    meta,
                });

                value
            }
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn matrix_one_arg(
        &mut self,
        ctx: &mut Context,
        ty: Handle<Type>,
        columns: crate::VectorSize,
        rows: crate::VectorSize,
        element_scalar: Scalar,
        (mut value, expr_meta): (Handle<Expression>, Span),
        meta: Span,
    ) -> Result<Handle<Expression>> {
        let mut components = Vec::with_capacity(columns as usize);
        // TODO: casts
        // `Expression::As` doesn't support matrix width
        // casts so we need to do some extra work for casts

        ctx.forced_conversion(&mut value, expr_meta, element_scalar)?;
        match *ctx.resolve_type(value, expr_meta)? {
            TypeInner::Scalar(_) => {
                // If a matrix is constructed with a single scalar value, then that
                // value is used to initialize all the values along the diagonal of
                // the matrix; the rest are given zeros.
                let vector_ty = ctx.module.types.insert(
                    Type {
                        name: None,
                        inner: TypeInner::Vector {
                            size: rows,
                            scalar: element_scalar,
                        },
                    },
                    meta,
                );

                let zero_literal = Literal::zero(element_scalar).unwrap();
                let zero = ctx.add_expression(Expression::Literal(zero_literal), meta)?;

                for i in 0..columns as u32 {
                    components.push(
                        ctx.add_expression(
                            Expression::Compose {
                                ty: vector_ty,
                                components: (0..rows as u32)
                                    .map(|r| match r == i {
                                        true => value,
                                        false => zero,
                                    })
                                    .collect(),
                            },
                            meta,
                        )?,
                    )
                }
            }
            TypeInner::Matrix {
                rows: ori_rows,
                columns: ori_cols,
                ..
            } => {
                // If a matrix is constructed from a matrix, then each component
                // (column i, row j) in the result that has a corresponding component
                // (column i, row j) in the argument will be initialized from there. All
                // other components will be initialized to the identity matrix.

                let zero_literal = Literal::zero(element_scalar).unwrap();
                let one_literal = Literal::one(element_scalar).unwrap();

                let zero = ctx.add_expression(Expression::Literal(zero_literal), meta)?;
                let one = ctx.add_expression(Expression::Literal(one_literal), meta)?;

                let vector_ty = ctx.module.types.insert(
                    Type {
                        name: None,
                        inner: TypeInner::Vector {
                            size: rows,
                            scalar: element_scalar,
                        },
                    },
                    meta,
                );

                for i in 0..columns as u32 {
                    if i < ori_cols as u32 {
                        use core::cmp::Ordering;

                        let vector = ctx.add_expression(
                            Expression::AccessIndex {
                                base: value,
                                index: i,
                            },
                            meta,
                        )?;

                        components.push(match ori_rows.cmp(&rows) {
                            Ordering::Less => {
                                let components = (0..rows as u32)
                                    .map(|r| {
                                        if r < ori_rows as u32 {
                                            ctx.add_expression(
                                                Expression::AccessIndex {
                                                    base: vector,
                                                    index: r,
                                                },
                                                meta,
                                            )
                                        } else if r == i {
                                            Ok(one)
                                        } else {
                                            Ok(zero)
                                        }
                                    })
                                    .collect::<Result<_>>()?;

                                ctx.add_expression(
                                    Expression::Compose {
                                        ty: vector_ty,
                                        components,
                                    },
                                    meta,
                                )?
                            }
                            Ordering::Equal => vector,
                            Ordering::Greater => ctx.vector_resize(rows, vector, meta)?,
                        })
                    } else {
                        let compose_expr = Expression::Compose {
                            ty: vector_ty,
                            components: (0..rows as u32)
                                .map(|r| match r == i {
                                    true => one,
                                    false => zero,
                                })
                                .collect(),
                        };

                        let vec = ctx.add_expression(compose_expr, meta)?;

                        components.push(vec)
                    }
                }
            }
            _ => {
                components = iter::repeat_n(value, columns as usize).collect();
            }
        }

        ctx.add_expression(Expression::Compose { ty, components }, meta)
    }

    fn vector_constructor(
        &mut self,
        ctx: &mut Context,
        ty: Handle<Type>,
        size: crate::VectorSize,
        scalar: Scalar,
        args: &[(Handle<Expression>, Span)],
        meta: Span,
    ) -> Result<Handle<Expression>> {
        let mut components = Vec::with_capacity(size as usize);

        for (mut arg, expr_meta) in args.iter().copied() {
            ctx.forced_conversion(&mut arg, expr_meta, scalar)?;

            if components.len() >= size as usize {
                break;
            }

            match *ctx.resolve_type(arg, expr_meta)? {
                TypeInner::Scalar { .. } => components.push(arg),
                TypeInner::Matrix { rows, columns, .. } => {
                    components.reserve(rows as usize * columns as usize);
                    for c in 0..(columns as u32) {
                        let base = ctx.add_expression(
                            Expression::AccessIndex {
                                base: arg,
                                index: c,
                            },
                            expr_meta,
                        )?;
                        for r in 0..(rows as u32) {
                            components.push(ctx.add_expression(
                                Expression::AccessIndex { base, index: r },
                                expr_meta,
                            )?)
                        }
                    }
                }
                TypeInner::Vector { size: ori_size, .. } => {
                    components.reserve(ori_size as usize);
                    for index in 0..(ori_size as u32) {
                        components.push(ctx.add_expression(
                            Expression::AccessIndex { base: arg, index },
                            expr_meta,
                        )?)
                    }
                }
                _ => components.push(arg),
            }
        }

        components.truncate(size as usize);

        ctx.add_expression(Expression::Compose { ty, components }, meta)
    }

    fn constructor_many(
        &mut self,
        ctx: &mut Context,
        ty: Handle<Type>,
        args: Vec<(Handle<Expression>, Span)>,
        meta: Span,
    ) -> Result<Handle<Expression>> {
        let mut components = Vec::with_capacity(args.len());

        let struct_member_data = match ctx.module.types[ty].inner {
            TypeInner::Matrix {
                columns,
                rows,
                scalar: element_scalar,
            } => {
                let mut flattened = Vec::with_capacity(columns as usize * rows as usize);

                for (mut arg, meta) in args.iter().copied() {
                    ctx.forced_conversion(&mut arg, meta, element_scalar)?;

                    match *ctx.resolve_type(arg, meta)? {
                        TypeInner::Vector { size, .. } => {
                            for i in 0..(size as u32) {
                                flattened.push(ctx.add_expression(
                                    Expression::AccessIndex {
                                        base: arg,
                                        index: i,
                                    },
                                    meta,
                                )?)
                            }
                        }
                        _ => flattened.push(arg),
                    }
                }

                let ty = ctx.module.types.insert(
                    Type {
                        name: None,
                        inner: TypeInner::Vector {
                            size: rows,
                            scalar: element_scalar,
                        },
                    },
                    meta,
                );

                for chunk in flattened.chunks(rows as usize) {
                    components.push(ctx.add_expression(
                        Expression::Compose {
                            ty,
                            components: Vec::from(chunk),
                        },
                        meta,
                    )?)
                }
                None
            }
            TypeInner::Vector { size, scalar } => {
                return self.vector_constructor(ctx, ty, size, scalar, &args, meta)
            }
            TypeInner::Array { base, .. } => {
                for (mut arg, meta) in args.iter().copied() {
                    let scalar_components = scalar_components(&ctx.module.types[base].inner);
                    if let Some(scalar) = scalar_components {
                        ctx.implicit_conversion(&mut arg, meta, scalar)?;
                    }

                    components.push(arg)
                }
                None
            }
            TypeInner::Struct { ref members, .. } => Some(
                members
                    .iter()
                    .map(|member| scalar_components(&ctx.module.types[member.ty].inner))
                    .collect::<Vec<_>>(),
            ),
            _ => {
                return Err(Error {
                    kind: ErrorKind::SemanticError("Constructor: Too many arguments".into()),
                    meta,
                })
            }
        };

        if let Some(struct_member_data) = struct_member_data {
            for ((mut arg, meta), scalar_components) in
                args.iter().copied().zip(struct_member_data.iter().copied())
            {
                if let Some(scalar) = scalar_components {
                    ctx.implicit_conversion(&mut arg, meta, scalar)?;
                }

                components.push(arg)
            }
        }

        ctx.add_expression(Expression::Compose { ty, components }, meta)
    }

    fn function_call(
        &mut self,
        ctx: &mut Context,
        stmt: &StmtContext,
        name: String,
        args: Vec<(Handle<Expression>, Span)>,
        raw_args: &[Handle<HirExpr>],
        meta: Span,
    ) -> Result<Option<Handle<Expression>>> {
        // Grow the typifier to be able to index it later without needing
        // to hold the context mutably
        for &(expr, span) in args.iter() {
            ctx.typifier_grow(expr, span)?;
        }

        // Check if the passed arguments require any special variations
        let mut variations =
            builtin_required_variations(args.iter().map(|&(expr, _)| ctx.get_type(expr)));

        // Initiate the declaration if it wasn't previously initialized and inject builtins
        let declaration = self.lookup_function.entry(name.clone()).or_insert_with(|| {
            variations |= BuiltinVariations::STANDARD;
            Default::default()
        });
        inject_builtin(declaration, ctx.module, &name, variations);

        // Borrow again but without mutability, at this point a declaration is guaranteed
        let declaration = self.lookup_function.get(&name).unwrap();

        // Possibly contains the overload to be used in the call
        let mut maybe_overload = None;
        // The conversions needed for the best analyzed overload, this is initialized all to
        // `NONE` to make sure that conversions always pass the first time without ambiguity
        let mut old_conversions = vec![Conversion::None; args.len()];
        // Tracks whether the comparison between overloads lead to an ambiguity
        let mut ambiguous = false;

        // Iterate over all the available overloads to select either an exact match or a
        // overload which has suitable implicit conversions
        'outer: for (overload_idx, overload) in declaration.overloads.iter().enumerate() {
            // If the overload and the function call don't have the same number of arguments
            // continue to the next overload
            if args.len() != overload.parameters.len() {
                continue;
            }

            log::trace!("Testing overload {overload_idx}");

            // Stores whether the current overload matches exactly the function call
            let mut exact = true;
            // State of the selection
            // If None we still don't know what is the best overload
            // If Some(true) the new overload is better
            // If Some(false) the old overload is better
            let mut superior = None;
            // Store the conversions for the current overload so that later they can replace the
            // conversions used for querying the best overload
            let mut new_conversions = vec![Conversion::None; args.len()];

            // Loop through the overload parameters and check if the current overload is better
            // compared to the previous best overload.
            for (i, overload_parameter) in overload.parameters.iter().enumerate() {
                let call_argument = &args[i];
                let parameter_info = &overload.parameters_info[i];

                // If the image is used in the overload as a depth texture convert it
                // before comparing, otherwise exact matches wouldn't be reported
                if parameter_info.depth {
                    sampled_to_depth(ctx, call_argument.0, call_argument.1, &mut self.errors);
                    ctx.invalidate_expression(call_argument.0, call_argument.1)?
                }

                ctx.typifier_grow(call_argument.0, call_argument.1)?;

                let overload_param_ty = &ctx.module.types[*overload_parameter].inner;
                let call_arg_ty = ctx.get_type(call_argument.0);

                log::trace!(
                    "Testing parameter {i}\n\tOverload = {overload_param_ty:?}\n\tCall = {call_arg_ty:?}"
                );

                // Storage images cannot be directly compared since while the access is part of the
                // type in naga's IR, in glsl they are a qualifier and don't enter in the match as
                // long as the access needed is satisfied.
                if let (
                    &TypeInner::Image {
                        class:
                            crate::ImageClass::Storage {
                                format: overload_format,
                                access: overload_access,
                            },
                        dim: overload_dim,
                        arrayed: overload_arrayed,
                    },
                    &TypeInner::Image {
                        class:
                            crate::ImageClass::Storage {
                                format: call_format,
                                access: call_access,
                            },
                        dim: call_dim,
                        arrayed: call_arrayed,
                    },
                ) = (overload_param_ty, call_arg_ty)
                {
                    // Images size must match otherwise the overload isn't what we want
                    let good_size = call_dim == overload_dim && call_arrayed == overload_arrayed;
                    // Glsl requires the formats to strictly match unless you are builtin
                    // function overload and have not been replaced, in which case we only
                    // check that the format scalar kind matches
                    let good_format = overload_format == call_format
                        || (overload.internal
                            && Scalar::from(overload_format) == Scalar::from(call_format));
                    if !(good_size && good_format) {
                        continue 'outer;
                    }

                    // While storage access mismatch is an error it isn't one that causes
                    // the overload matching to fail so we defer the error and consider
                    // that the images match exactly
                    if !call_access.contains(overload_access) {
                        self.errors.push(Error {
                            kind: ErrorKind::SemanticError(
                                format!(
                                    "'{name}': image needs {overload_access:?} access but only {call_access:?} was provided"
                                )
                                .into(),
                            ),
                            meta,
                        });
                    }

                    // The images satisfy the conditions to be considered as an exact match
                    new_conversions[i] = Conversion::Exact;
                    continue;
                } else if overload_param_ty == call_arg_ty {
                    // If the types match there's no need to check for conversions so continue
                    new_conversions[i] = Conversion::Exact;
                    continue;
                }

                // Glsl defines that inout follows both the conversions for input parameters and
                // output parameters, this means that the type must have a conversion from both the
                // call argument to the function parameter and the function parameter to the call
                // argument, the only way this is possible is for the conversion to be an identity
                // (i.e. call argument = function parameter)
                if let ParameterQualifier::InOut = parameter_info.qualifier {
                    continue 'outer;
                }

                // The function call argument and the function definition
                // parameter are not equal at this point, so we need to try
                // implicit conversions.
                //
                // Now there are two cases, the argument is defined as a normal
                // parameter (`in` or `const`), in this case an implicit
                // conversion is made from the calling argument to the
                // definition argument. If the parameter is `out` the
                // opposite needs to be done, so the implicit conversion is made
                // from the definition argument to the calling argument.
                let maybe_conversion = if parameter_info.qualifier.is_lhs() {
                    conversion(call_arg_ty, overload_param_ty)
                } else {
                    conversion(overload_param_ty, call_arg_ty)
                };

                let conversion = match maybe_conversion {
                    Some(info) => info,
                    None => continue 'outer,
                };

                // At this point a conversion will be needed so the overload no longer
                // exactly matches the call arguments
                exact = false;

                // Compare the conversions needed for this overload parameter to that of the
                // last overload analyzed respective parameter, the value is:
                // - `true` when the new overload argument has a better conversion
                // - `false` when the old overload argument has a better conversion
                let best_arg = match (conversion, old_conversions[i]) {
                    // An exact match is always better, we don't need to check this for the
                    // current overload since it was checked earlier
                    (_, Conversion::Exact) => false,
                    // No overload was yet analyzed so this one is the best yet
                    (_, Conversion::None) => true,
                    // A conversion from a float to a double is the best possible conversion
                    (Conversion::FloatToDouble, _) => true,
                    (_, Conversion::FloatToDouble) => false,
                    // A conversion from a float to an integer is preferred than one
                    // from double to an integer
                    (Conversion::IntToFloat, Conversion::IntToDouble) => true,
                    (Conversion::IntToDouble, Conversion::IntToFloat) => false,
                    // This case handles things like no conversion and exact which were already
                    // treated and other cases which no conversion is better than the other
                    _ => continue,
                };

                // Check if the best parameter corresponds to the current selected overload
                // to pass to the next comparison, if this isn't true mark it as ambiguous
                match best_arg {
                    true => match superior {
                        Some(false) => ambiguous = true,
                        _ => {
                            superior = Some(true);
                            new_conversions[i] = conversion
                        }
                    },
                    false => match superior {
                        Some(true) => ambiguous = true,
                        _ => superior = Some(false),
                    },
                }
            }

            // The overload matches exactly the function call so there's no ambiguity (since
            // repeated overload aren't allowed) and the current overload is selected, no
            // further querying is needed.
            if exact {
                maybe_overload = Some(overload);
                ambiguous = false;
                break;
            }

            match superior {
                // New overload is better keep it
                Some(true) => {
                    maybe_overload = Some(overload);
                    // Replace the conversions
                    old_conversions = new_conversions;
                }
                // Old overload is better do nothing
                Some(false) => {}
                // No overload was better than the other this can be caused
                // when all conversions are ambiguous in which the overloads themselves are
                // ambiguous.
                None => {
                    ambiguous = true;
                    // Assign the new overload, this helps ensures that in this case of
                    // ambiguity the parsing won't end immediately and allow for further
                    // collection of errors.
                    maybe_overload = Some(overload);
                }
            }
        }

        if ambiguous {
            self.errors.push(Error {
                kind: ErrorKind::SemanticError(
                    format!("Ambiguous best function for '{name}'").into(),
                ),
                meta,
            })
        }

        let overload = maybe_overload.ok_or_else(|| Error {
            kind: ErrorKind::SemanticError(format!("Unknown function '{name}'").into()),
            meta,
        })?;

        let parameters_info = overload.parameters_info.clone();
        let parameters = overload.parameters.clone();
        let is_void = overload.void;
        let kind = overload.kind;

        let mut arguments = Vec::with_capacity(args.len());
        let mut proxy_writes = Vec::new();

        // Iterate through the function call arguments applying transformations as needed
        for (((parameter_info, call_argument), expr), parameter) in parameters_info
            .iter()
            .zip(&args)
            .zip(raw_args)
            .zip(&parameters)
        {
            if parameter_info.qualifier.is_lhs() {
                // Reprocess argument in LHS position
                let (handle, meta) = ctx.lower_expect_inner(stmt, self, *expr, ExprPos::Lhs)?;

                self.process_lhs_argument(
                    ctx,
                    meta,
                    *parameter,
                    parameter_info,
                    handle,
                    call_argument,
                    &mut proxy_writes,
                    &mut arguments,
                )?;

                continue;
            }

            let (mut handle, meta) = *call_argument;

            let scalar_comps = scalar_components(&ctx.module.types[*parameter].inner);

            // Apply implicit conversions as needed
            if let Some(scalar) = scalar_comps {
                ctx.implicit_conversion(&mut handle, meta, scalar)?;
            }

            arguments.push(handle)
        }

        match kind {
            FunctionKind::Call(function) => {
                ctx.emit_end();

                let result = if !is_void {
                    Some(ctx.add_expression(Expression::CallResult(function), meta)?)
                } else {
                    None
                };

                ctx.body.push(
                    Statement::Call {
                        function,
                        arguments,
                        result,
                    },
                    meta,
                );

                ctx.emit_start();

                // Write back all the variables that were scheduled to their original place
                for proxy_write in proxy_writes {
                    let mut value = ctx.add_expression(
                        Expression::Load {
                            pointer: proxy_write.value,
                        },
                        meta,
                    )?;

                    if let Some(scalar) = proxy_write.convert {
                        ctx.conversion(&mut value, meta, scalar)?;
                    }

                    ctx.emit_restart();

                    ctx.body.push(
                        Statement::Store {
                            pointer: proxy_write.target,
                            value,
                        },
                        meta,
                    );
                }

                Ok(result)
            }
            FunctionKind::Macro(builtin) => builtin.call(self, ctx, arguments.as_mut_slice(), meta),
        }
    }

    /// Processes a function call argument that appears in place of an output
    /// parameter.
    #[allow(clippy::too_many_arguments)]
    fn process_lhs_argument(
        &mut self,
        ctx: &mut Context,
        meta: Span,
        parameter_ty: Handle<Type>,
        parameter_info: &ParameterInfo,
        original: Handle<Expression>,
        call_argument: &(Handle<Expression>, Span),
        proxy_writes: &mut Vec<ProxyWrite>,
        arguments: &mut Vec<Handle<Expression>>,
    ) -> Result<()> {
        let original_ty = ctx.resolve_type(original, meta)?;
        let original_pointer_space = original_ty.pointer_space();

        // The type of a possible spill variable needed for a proxy write
        let mut maybe_ty = match *original_ty {
            // If the argument is to be passed as a pointer but the type of the
            // expression returns a vector it must mean that it was for example
            // swizzled and it must be spilled into a local before calling
            TypeInner::Vector { size, scalar } => Some(ctx.module.types.insert(
                Type {
                    name: None,
                    inner: TypeInner::Vector { size, scalar },
                },
                Span::default(),
            )),
            // If the argument is a pointer whose address space isn't `Function`, an
            // indirection through a local variable is needed to align the address
            // spaces of the call argument and the overload parameter.
            TypeInner::Pointer { base, space } if space != AddressSpace::Function => Some(base),
            TypeInner::ValuePointer {
                size,
                scalar,
                space,
            } if space != AddressSpace::Function => {
                let inner = match size {
                    Some(size) => TypeInner::Vector { size, scalar },
                    None => TypeInner::Scalar(scalar),
                };

                Some(
                    ctx.module
                        .types
                        .insert(Type { name: None, inner }, Span::default()),
                )
            }
            _ => None,
        };

        // Since the original expression might be a pointer and we want a value
        // for the proxy writes, we might need to load the pointer.
        let value = if original_pointer_space.is_some() {
            ctx.add_expression(Expression::Load { pointer: original }, Span::default())?
        } else {
            original
        };

        ctx.typifier_grow(call_argument.0, call_argument.1)?;

        let overload_param_ty = &ctx.module.types[parameter_ty].inner;
        let call_arg_ty = ctx.get_type(call_argument.0);
        let needs_conversion = call_arg_ty != overload_param_ty;

        let arg_scalar_comps = scalar_components(call_arg_ty);

        // Since output parameters also allow implicit conversions from the
        // parameter to the argument, we need to spill the conversion to a
        // variable and create a proxy write for the original variable.
        if needs_conversion {
            maybe_ty = Some(parameter_ty);
        }

        if let Some(ty) = maybe_ty {
            // Create the spill variable
            let spill_var = ctx.locals.append(
                LocalVariable {
                    name: None,
                    ty,
                    init: None,
                },
                Span::default(),
            );
            let spill_expr =
                ctx.add_expression(Expression::LocalVariable(spill_var), Span::default())?;

            // If the argument is also copied in we must store the value of the
            // original variable to the spill variable.
            if let ParameterQualifier::InOut = parameter_info.qualifier {
                ctx.body.push(
                    Statement::Store {
                        pointer: spill_expr,
                        value,
                    },
                    Span::default(),
                );
            }

            // Add the spill variable as an argument to the function call
            arguments.push(spill_expr);

            let convert = if needs_conversion {
                arg_scalar_comps
            } else {
                None
            };

            // Register the temporary local to be written back to it's original
            // place after the function call
            if let Expression::Swizzle {
                size,
                mut vector,
                pattern,
            } = ctx.expressions[original]
            {
                if let Expression::Load { pointer } = ctx.expressions[vector] {
                    vector = pointer;
                }

                for (i, component) in pattern.iter().take(size as usize).enumerate() {
                    let original = ctx.add_expression(
                        Expression::AccessIndex {
                            base: vector,
                            index: *component as u32,
                        },
                        Span::default(),
                    )?;

                    let spill_component = ctx.add_expression(
                        Expression::AccessIndex {
                            base: spill_expr,
                            index: i as u32,
                        },
                        Span::default(),
                    )?;

                    proxy_writes.push(ProxyWrite {
                        target: original,
                        value: spill_component,
                        convert,
                    });
                }
            } else {
                proxy_writes.push(ProxyWrite {
                    target: original,
                    value: spill_expr,
                    convert,
                });
            }
        } else {
            arguments.push(original);
        }

        Ok(())
    }

    pub(crate) fn add_function(
        &mut self,
        mut ctx: Context,
        name: String,
        result: Option<FunctionResult>,
        meta: Span,
    ) {
        ensure_block_returns(&mut ctx.body);

        let void = result.is_none();

        // Check if the passed arguments require any special variations
        let mut variations = builtin_required_variations(
            ctx.parameters
                .iter()
                .map(|&arg| &ctx.module.types[arg].inner),
        );

        // Initiate the declaration if it wasn't previously initialized and inject builtins
        let declaration = self.lookup_function.entry(name.clone()).or_insert_with(|| {
            variations |= BuiltinVariations::STANDARD;
            Default::default()
        });
        inject_builtin(declaration, ctx.module, &name, variations);

        let Context {
            expressions,
            locals,
            arguments,
            parameters,
            parameters_info,
            body,
            module,
            ..
        } = ctx;

        let function = Function {
            name: Some(name),
            arguments,
            result,
            local_variables: locals,
            expressions,
            named_expressions: crate::NamedExpressions::default(),
            body,
            diagnostic_filter_leaf: None,
        };

        'outer: for decl in declaration.overloads.iter_mut() {
            if parameters.len() != decl.parameters.len() {
                continue;
            }

            for (new_parameter, old_parameter) in parameters.iter().zip(decl.parameters.iter()) {
                let new_inner = &module.types[*new_parameter].inner;
                let old_inner = &module.types[*old_parameter].inner;

                if new_inner != old_inner {
                    continue 'outer;
                }
            }

            if decl.defined {
                return self.errors.push(Error {
                    kind: ErrorKind::SemanticError("Function already defined".into()),
                    meta,
                });
            }

            decl.defined = true;
            decl.parameters_info = parameters_info;
            match decl.kind {
                FunctionKind::Call(handle) => *module.functions.get_mut(handle) = function,
                FunctionKind::Macro(_) => {
                    let handle = module.functions.append(function, meta);
                    decl.kind = FunctionKind::Call(handle)
                }
            }
            return;
        }

        let handle = module.functions.append(function, meta);
        declaration.overloads.push(Overload {
            parameters,
            parameters_info,
            kind: FunctionKind::Call(handle),
            defined: true,
            internal: false,
            void,
        });
    }

    pub(crate) fn add_prototype(
        &mut self,
        ctx: Context,
        name: String,
        result: Option<FunctionResult>,
        meta: Span,
    ) {
        let void = result.is_none();

        // Check if the passed arguments require any special variations
        let mut variations = builtin_required_variations(
            ctx.parameters
                .iter()
                .map(|&arg| &ctx.module.types[arg].inner),
        );

        // Initiate the declaration if it wasn't previously initialized and inject builtins
        let declaration = self.lookup_function.entry(name.clone()).or_insert_with(|| {
            variations |= BuiltinVariations::STANDARD;
            Default::default()
        });
        inject_builtin(declaration, ctx.module, &name, variations);

        let Context {
            arguments,
            parameters,
            parameters_info,
            module,
            ..
        } = ctx;

        let function = Function {
            name: Some(name),
            arguments,
            result,
            ..Default::default()
        };

        'outer: for decl in declaration.overloads.iter() {
            if parameters.len() != decl.parameters.len() {
                continue;
            }

            for (new_parameter, old_parameter) in parameters.iter().zip(decl.parameters.iter()) {
                let new_inner = &module.types[*new_parameter].inner;
                let old_inner = &module.types[*old_parameter].inner;

                if new_inner != old_inner {
                    continue 'outer;
                }
            }

            return self.errors.push(Error {
                kind: ErrorKind::SemanticError("Prototype already defined".into()),
                meta,
            });
        }

        let handle = module.functions.append(function, meta);
        declaration.overloads.push(Overload {
            parameters,
            parameters_info,
            kind: FunctionKind::Call(handle),
            defined: false,
            internal: false,
            void,
        });
    }

    /// Create a Naga [`EntryPoint`] that calls the GLSL `main` function.
    ///
    /// We compile the GLSL `main` function as an ordinary Naga [`Function`].
    /// This function synthesizes a Naga [`EntryPoint`] to call that.
    ///
    /// Each GLSL input and output variable (including builtins) becomes a Naga
    /// [`GlobalVariable`]s in the [`Private`] address space, which `main` can
    /// access in the usual way.
    ///
    /// The `EntryPoint` we synthesize here has an argument for each GLSL input
    /// variable, and returns a struct with a member for each GLSL output
    /// variable. The entry point contains code to:
    ///
    /// - copy its arguments into the Naga globals representing the GLSL input
    ///   variables,
    ///
    /// - call the Naga `Function` representing the GLSL `main` function, and then
    ///
    /// - build its return value from whatever values the GLSL `main` left in
    ///   the Naga globals representing GLSL `output` variables.
    ///
    /// Upon entry, [`ctx.body`] should contain code, accumulated by prior calls
    /// to [`ParsingContext::parse_external_declaration`][pxd], to initialize
    /// private global variables as needed. This code gets spliced into the
    /// entry point before the call to `main`.
    ///
    /// [`GlobalVariable`]: crate::GlobalVariable
    /// [`Private`]: crate::AddressSpace::Private
    /// [`ctx.body`]: Context::body
    /// [pxd]: super::ParsingContext::parse_external_declaration
    pub(crate) fn add_entry_point(
        &mut self,
        function: Handle<Function>,
        mut ctx: Context,
    ) -> Result<()> {
        let mut arguments = Vec::new();

        let body = Block::with_capacity(
            // global init body
            ctx.body.len() +
            // prologue and epilogue
            self.entry_args.len() * 2
            // Call, Emit for composing struct and return
            + 3,
        );

        let global_init_body = core::mem::replace(&mut ctx.body, body);

        for arg in self.entry_args.iter() {
            if arg.storage != StorageQualifier::Input {
                continue;
            }

            let pointer = ctx
                .expressions
                .append(Expression::GlobalVariable(arg.handle), Default::default());
            ctx.local_expression_kind_tracker
                .insert(pointer, crate::proc::ExpressionKind::Runtime);

            let ty = ctx.module.global_variables[arg.handle].ty;

            ctx.arg_type_walker(
                arg.name.clone(),
                arg.binding.clone(),
                pointer,
                ty,
                &mut |ctx, name, pointer, ty, binding| {
                    let idx = arguments.len() as u32;

                    arguments.push(FunctionArgument {
                        name,
                        ty,
                        binding: Some(binding),
                    });

                    let value = ctx
                        .expressions
                        .append(Expression::FunctionArgument(idx), Default::default());
                    ctx.local_expression_kind_tracker
                        .insert(value, crate::proc::ExpressionKind::Runtime);
                    ctx.body
                        .push(Statement::Store { pointer, value }, Default::default());
                },
            )?
        }

        ctx.body.extend_block(global_init_body);

        ctx.body.push(
            Statement::Call {
                function,
                arguments: Vec::new(),
                result: None,
            },
            Default::default(),
        );

        let mut span = 0;
        let mut members = Vec::new();
        let mut components = Vec::new();

        for arg in self.entry_args.iter() {
            if arg.storage != StorageQualifier::Output {
                continue;
            }

            let pointer = ctx
                .expressions
                .append(Expression::GlobalVariable(arg.handle), Default::default());
            ctx.local_expression_kind_tracker
                .insert(pointer, crate::proc::ExpressionKind::Runtime);

            let ty = ctx.module.global_variables[arg.handle].ty;

            ctx.arg_type_walker(
                arg.name.clone(),
                arg.binding.clone(),
                pointer,
                ty,
                &mut |ctx, name, pointer, ty, binding| {
                    members.push(StructMember {
                        name,
                        ty,
                        binding: Some(binding),
                        offset: span,
                    });

                    span += ctx.module.types[ty].inner.size(ctx.module.to_ctx());

                    let len = ctx.expressions.len();
                    let load = ctx
                        .expressions
                        .append(Expression::Load { pointer }, Default::default());
                    ctx.local_expression_kind_tracker
                        .insert(load, crate::proc::ExpressionKind::Runtime);
                    ctx.body.push(
                        Statement::Emit(ctx.expressions.range_from(len)),
                        Default::default(),
                    );
                    components.push(load)
                },
            )?
        }

        let (ty, value) = if !components.is_empty() {
            let ty = ctx.module.types.insert(
                Type {
                    name: None,
                    inner: TypeInner::Struct { members, span },
                },
                Default::default(),
            );

            let len = ctx.expressions.len();
            let res = ctx
                .expressions
                .append(Expression::Compose { ty, components }, Default::default());
            ctx.local_expression_kind_tracker
                .insert(res, crate::proc::ExpressionKind::Runtime);
            ctx.body.push(
                Statement::Emit(ctx.expressions.range_from(len)),
                Default::default(),
            );

            (Some(ty), Some(res))
        } else {
            (None, None)
        };

        ctx.body
            .push(Statement::Return { value }, Default::default());

        let Context {
            body, expressions, ..
        } = ctx;

        ctx.module.entry_points.push(EntryPoint {
            name: "main".to_string(),
            stage: self.meta.stage,
            early_depth_test: Some(crate::EarlyDepthTest::Force)
                .filter(|_| self.meta.early_fragment_tests),
            workgroup_size: self.meta.workgroup_size,
            workgroup_size_overrides: None,
            function: Function {
                arguments,
                expressions,
                body,
                result: ty.map(|ty| FunctionResult { ty, binding: None }),
                ..Default::default()
            },
            mesh_info: None,
            task_payload: None,
            incoming_ray_payload: None,
        });

        Ok(())
    }
}

impl Context<'_> {
    /// Helper function for building the input/output interface of the entry point
    ///
    /// Calls `f` with the data of the entry point argument, flattening composite types
    /// recursively
    ///
    /// The passed arguments to the callback are:
    /// - The ctx
    /// - The name
    /// - The pointer expression to the global storage
    /// - The handle to the type of the entry point argument
    /// - The binding of the entry point argument
    fn arg_type_walker(
        &mut self,
        name: Option<String>,
        binding: crate::Binding,
        pointer: Handle<Expression>,
        ty: Handle<Type>,
        f: &mut impl FnMut(
            &mut Context,
            Option<String>,
            Handle<Expression>,
            Handle<Type>,
            crate::Binding,
        ),
    ) -> Result<()> {
        match self.module.types[ty].inner {
            // TODO: Better error reporting
            // right now we just don't walk the array if the size isn't known at
            // compile time and let validation catch it
            TypeInner::Array {
                base,
                size: crate::ArraySize::Constant(size),
                ..
            } => {
                let mut location = match binding {
                    crate::Binding::Location { location, .. } => location,
                    crate::Binding::BuiltIn(_) => return Ok(()),
                };

                let interpolation =
                    self.module.types[base]
                        .inner
                        .scalar_kind()
                        .map(|kind| match kind {
                            ScalarKind::Float => crate::Interpolation::Perspective,
                            _ => crate::Interpolation::Flat,
                        });

                for index in 0..size.get() {
                    let member_pointer = self.add_expression(
                        Expression::AccessIndex {
                            base: pointer,
                            index,
                        },
                        Span::default(),
                    )?;

                    let binding = crate::Binding::Location {
                        location,
                        interpolation,
                        sampling: None,
                        blend_src: None,
                        per_primitive: false,
                    };
                    location += 1;

                    self.arg_type_walker(name.clone(), binding, member_pointer, base, f)?
                }
            }
            TypeInner::Struct { ref members, .. } => {
                let mut location = match binding {
                    crate::Binding::Location { location, .. } => location,
                    crate::Binding::BuiltIn(_) => return Ok(()),
                };

                for (i, member) in members.clone().into_iter().enumerate() {
                    let member_pointer = self.add_expression(
                        Expression::AccessIndex {
                            base: pointer,
                            index: i as u32,
                        },
                        Span::default(),
                    )?;

                    let binding = match member.binding {
                        Some(binding) => binding,
                        None => {
                            let interpolation = self.module.types[member.ty]
                                .inner
                                .scalar_kind()
                                .map(|kind| match kind {
                                    ScalarKind::Float => crate::Interpolation::Perspective,
                                    _ => crate::Interpolation::Flat,
                                });
                            let binding = crate::Binding::Location {
                                location,
                                interpolation,
                                sampling: None,
                                blend_src: None,
                                per_primitive: false,
                            };
                            location += 1;
                            binding
                        }
                    };

                    self.arg_type_walker(member.name, binding, member_pointer, member.ty, f)?
                }
            }
            _ => f(self, name, pointer, ty, binding),
        }

        Ok(())
    }
}

/// Helper enum containing the type of conversion need for a call
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Conversion {
    /// No conversion needed
    Exact,
    /// Float to double conversion needed
    FloatToDouble,
    /// Int or uint to float conversion needed
    IntToFloat,
    /// Int or uint to double conversion needed
    IntToDouble,
    /// Other type of conversion needed
    Other,
    /// No conversion was yet registered
    None,
}

/// Helper function, returns the type of conversion from `source` to `target`, if a
/// conversion is not possible returns None.
fn conversion(target: &TypeInner, source: &TypeInner) -> Option<Conversion> {
    use ScalarKind::*;

    // Gather the `ScalarKind` and scalar width from both the target and the source
    let (target_scalar, source_scalar) = match (target, source) {
        // Conversions between scalars are allowed
        (&TypeInner::Scalar(tgt_scalar), &TypeInner::Scalar(src_scalar)) => {
            (tgt_scalar, src_scalar)
        }
        // Conversions between vectors of the same size are allowed
        (
            &TypeInner::Vector {
                size: tgt_size,
                scalar: tgt_scalar,
            },
            &TypeInner::Vector {
                size: src_size,
                scalar: src_scalar,
            },
        ) if tgt_size == src_size => (tgt_scalar, src_scalar),
        // Conversions between matrices of the same size are allowed
        (
            &TypeInner::Matrix {
                rows: tgt_rows,
                columns: tgt_cols,
                scalar: tgt_scalar,
            },
            &TypeInner::Matrix {
                rows: src_rows,
                columns: src_cols,
                scalar: src_scalar,
            },
        ) if tgt_cols == src_cols && tgt_rows == src_rows => (tgt_scalar, src_scalar),
        _ => return None,
    };

    // Check if source can be converted into target, if this is the case then the type
    // power of target must be higher than that of source
    let target_power = type_power(target_scalar);
    let source_power = type_power(source_scalar);
    if target_power < source_power {
        return None;
    }

    Some(match (target_scalar, source_scalar) {
        // A conversion from a float to a double is special
        (Scalar::F64, Scalar::F32) => Conversion::FloatToDouble,
        // A conversion from an integer to a float is special
        (
            Scalar::F32,
            Scalar {
                kind: Sint | Uint,
                width: _,
            },
        ) => Conversion::IntToFloat,
        // A conversion from an integer to a double is special
        (
            Scalar::F64,
            Scalar {
                kind: Sint | Uint,
                width: _,
            },
        ) => Conversion::IntToDouble,
        _ => Conversion::Other,
    })
}

/// Helper method returning all the non standard builtin variations needed
/// to process the function call with the passed arguments
fn builtin_required_variations<'a>(args: impl Iterator<Item = &'a TypeInner>) -> BuiltinVariations {
    let mut variations = BuiltinVariations::empty();

    for ty in args {
        match *ty {
            TypeInner::ValuePointer { scalar, .. }
            | TypeInner::Scalar(scalar)
            | TypeInner::Vector { scalar, .. }
            | TypeInner::Matrix { scalar, .. } => {
                if scalar == Scalar::F64 {
                    variations |= BuiltinVariations::DOUBLE
                }
            }
            TypeInner::Image {
                dim,
                arrayed,
                class,
            } => {
                if dim == crate::ImageDimension::Cube && arrayed {
                    variations |= BuiltinVariations::CUBE_TEXTURES_ARRAY
                }

                if dim == crate::ImageDimension::D2 && arrayed && class.is_multisampled() {
                    variations |= BuiltinVariations::D2_MULTI_TEXTURES_ARRAY
                }
            }
            _ => {}
        }
    }

    variations
}
