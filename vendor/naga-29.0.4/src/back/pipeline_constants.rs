use alloc::{
    borrow::Cow,
    string::{String, ToString},
    vec::Vec,
};
use core::mem;

use hashbrown::HashSet;
use thiserror::Error;

use super::PipelineConstants;
use crate::{
    arena::HandleVec,
    compact::{compact, KeepUnused},
    ir,
    proc::{ConstantEvaluator, ConstantEvaluatorError, Emitter},
    valid::{Capabilities, ModuleInfo, ValidationError, ValidationFlags, Validator},
    Arena, Block, Constant, Expression, Function, Handle, Literal, Module, Override, Range, Scalar,
    Span, Statement, TypeInner, WithSpan,
};

// Possibly unused if not compiled with no_std
#[allow(unused_imports)]
use num_traits::float::FloatCore as _;

#[derive(Error, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum PipelineConstantError {
    #[error("Missing value for pipeline-overridable constant with identifier string: '{0}'")]
    MissingValue(String),
    #[error("pipeline-overridable constant '{0}' not found in the shader")]
    NotFound(String),
    #[error(
        "Source f64 value needs to be finite ({}) for number destinations",
        "NaNs and Inifinites are not allowed"
    )]
    SrcNeedsToBeFinite,
    #[error("Source f64 value doesn't fit in destination")]
    DstRangeTooSmall,
    #[error(transparent)]
    ConstantEvaluatorError(#[from] ConstantEvaluatorError),
    #[error(transparent)]
    ValidationError(#[from] WithSpan<ValidationError>),
    #[error("workgroup_size override isn't strictly positive")]
    NegativeWorkgroupSize,
    #[error("max vertices or max primitives is negative")]
    NegativeMeshOutputMax,
}

/// Compact `module` and replace all overrides with constants.
///
/// `module` must be valid. Both compaction and constant evaluation may produce
/// invalid results (e.g. replace an invalid expression with a constant) for
/// invalid modules.
///
/// If no changes are needed, this just returns `Cow::Borrowed` references to
/// `module` and `module_info`. Otherwise, it clones `module`, retains only the
/// selected entry point, compacts the module, edits its [`global_expressions`]
/// arena to contain only fully-evaluated expressions, and returns the
/// simplified module and its validation results.
///
/// The module returned has an empty `overrides` arena, and the
/// `global_expressions` arena contains only fully-evaluated expressions.
///
/// [`global_expressions`]: Module::global_expressions
pub fn process_overrides<'a>(
    module: &'a Module,
    module_info: &'a ModuleInfo,
    entry_point: Option<(ir::ShaderStage, &str)>,
    pipeline_constants: &PipelineConstants,
) -> Result<(Cow<'a, Module>, Cow<'a, ModuleInfo>), PipelineConstantError> {
    let mut handles = module
        .overrides
        .iter()
        .map(|(handle, _)| handle)
        .collect::<Vec<_>>();
    for c in pipeline_constants.keys() {
        let c_id = c.parse().ok();
        if let Some((i, _)) = handles.iter().enumerate().find(|&(_, handle)| {
            let o = &module.overrides[*handle];
            if o.id.is_some() {
                o.id == c_id
            } else {
                o.name.as_deref() == Some(c.as_str())
            }
        }) {
            handles.swap_remove(i);
        } else {
            return Err(PipelineConstantError::NotFound(c.clone()));
        }
    }

    if (entry_point.is_none() || module.entry_points.len() <= 1) && module.overrides.is_empty() {
        // We skip compacting the module here mostly to reduce the risk of
        // hitting corner cases like https://github.com/gfx-rs/wgpu/issues/7793.
        // Compaction doesn't cost very much [1], so it would also be reasonable
        // to do it unconditionally. Even when there is a single entry point or
        // when no entry point is specified, it is still possible that there
        // are unreferenced items in the module that would be removed by this
        // compaction.
        //
        // [1]: https://github.com/gfx-rs/wgpu/pull/7703#issuecomment-2902153760
        return Ok((Cow::Borrowed(module), Cow::Borrowed(module_info)));
    }

    let mut module = module.clone();
    if let Some((ep_stage, ep_name)) = entry_point {
        module
            .entry_points
            .retain(|ep| ep.stage == ep_stage && ep.name == ep_name);
    }

    // Compact the module to remove anything not reachable from an entry point.
    // This is necessary because we may not have values for overrides that are
    // not reachable from the/an entry point.
    compact(&mut module, KeepUnused::No);

    // If there are no overrides in the module, then we can skip the rest.
    if module.overrides.is_empty() {
        return revalidate(module);
    }

    // A map from override handles to the handles of the constants
    // we've replaced them with.
    let mut override_map = HandleVec::with_capacity(module.overrides.len());

    // A map from `module`'s original global expression handles to
    // handles in the new, simplified global expression arena.
    let mut adjusted_global_expressions = HandleVec::with_capacity(module.global_expressions.len());

    // The set of constants whose initializer handles we've already
    // updated to refer to the newly built global expression arena.
    //
    // All constants in `module` must have their `init` handles
    // updated to point into the new, simplified global expression
    // arena. Some of these we can most easily handle as a side effect
    // during the simplification process, but we must handle the rest
    // in a final fixup pass, guided by `adjusted_global_expressions`. We
    // add their handles to this set, so that the final fixup step can
    // leave them alone.
    let mut adjusted_constant_initializers = HashSet::with_capacity(module.constants.len());

    let mut global_expression_kind_tracker = crate::proc::ExpressionKindTracker::new();
    let mut layouter = crate::proc::Layouter::default();

    // An iterator through the original overrides table, consumed in
    // approximate tandem with the global expressions.
    let mut overrides = mem::take(&mut module.overrides);
    let mut override_iter = overrides.iter_mut_span();

    // Do two things in tandem:
    //
    // - Rebuild the global expression arena from scratch, fully
    //   evaluating all expressions, and replacing each `Override`
    //   expression in `module.global_expressions` with a `Constant`
    //   expression.
    //
    // - Build a new `Constant` in `module.constants` to take the
    //   place of each `Override`.
    //
    // Build a map from old global expression handles to their
    // fully-evaluated counterparts in `adjusted_global_expressions` as we
    // go.
    //
    // Why in tandem? Overrides refer to expressions, and expressions
    // refer to overrides, so we can't disentangle the two into
    // separate phases. However, we can take advantage of the fact
    // that the overrides and expressions must form a DAG, and work
    // our way from the leaves to the roots, replacing and evaluating
    // as we go.
    //
    // Although the two loops are nested, this is really two
    // alternating phases: we adjust and evaluate constant expressions
    // until we hit an `Override` expression, at which point we switch
    // to building `Constant`s for `Overrides` until we've handled the
    // one used by the expression. Then we switch back to processing
    // expressions. Because we know they form a DAG, we know the
    // `Override` expressions we encounter can only have initializers
    // referring to global expressions we've already simplified.
    for (old_h, expr, span) in module.global_expressions.drain() {
        let mut expr = match expr {
            Expression::Override(h) => {
                let c_h = if let Some(new_h) = override_map.get(h) {
                    *new_h
                } else {
                    let mut new_h = None;
                    for entry in override_iter.by_ref() {
                        let stop = entry.0 == h;
                        new_h = Some(process_override(
                            entry,
                            pipeline_constants,
                            &mut module,
                            &mut override_map,
                            &adjusted_global_expressions,
                            &mut adjusted_constant_initializers,
                            &mut global_expression_kind_tracker,
                        )?);
                        if stop {
                            break;
                        }
                    }
                    new_h.unwrap()
                };
                Expression::Constant(c_h)
            }
            Expression::Constant(c_h) => {
                if adjusted_constant_initializers.insert(c_h) {
                    let init = &mut module.constants[c_h].init;
                    *init = adjusted_global_expressions[*init];
                }
                expr
            }
            expr => expr,
        };
        let mut evaluator = ConstantEvaluator::for_wgsl_module(
            &mut module,
            &mut global_expression_kind_tracker,
            &mut layouter,
            false,
        );
        adjust_expr(&adjusted_global_expressions, &mut expr);
        let h = evaluator.try_eval_and_append(expr, span)?;
        adjusted_global_expressions.insert(old_h, h);
    }

    // Finish processing any overrides we didn't visit in the loop above.
    for entry in override_iter {
        match *entry.1 {
            Override { name: Some(_), .. } | Override { id: Some(_), .. } => {
                process_override(
                    entry,
                    pipeline_constants,
                    &mut module,
                    &mut override_map,
                    &adjusted_global_expressions,
                    &mut adjusted_constant_initializers,
                    &mut global_expression_kind_tracker,
                )?;
            }
            Override {
                init: Some(ref mut init),
                ..
            } => {
                *init = adjusted_global_expressions[*init];
            }
            _ => {}
        }
    }

    // Update the initialization expression handles of all `Constant`s
    // and `GlobalVariable`s. Skip `Constant`s we'd already updated en
    // passant.
    for (_, c) in module
        .constants
        .iter_mut()
        .filter(|&(c_h, _)| !adjusted_constant_initializers.contains(&c_h))
    {
        c.init = adjusted_global_expressions[c.init];
    }

    for (_, v) in module.global_variables.iter_mut() {
        if let Some(ref mut init) = v.init {
            *init = adjusted_global_expressions[*init];
        }
    }

    let mut functions = mem::take(&mut module.functions);
    for (_, function) in functions.iter_mut() {
        process_function(&mut module, &override_map, &mut layouter, function)?;
    }
    module.functions = functions;

    let mut entry_points = mem::take(&mut module.entry_points);
    for ep in entry_points.iter_mut() {
        process_function(&mut module, &override_map, &mut layouter, &mut ep.function)?;
        process_workgroup_size_override(&mut module, &adjusted_global_expressions, ep)?;
        process_mesh_shader_overrides(&mut module, &adjusted_global_expressions, ep)?;
    }
    module.entry_points = entry_points;
    module.overrides = overrides;

    // Now that we've rewritten all the expressions, we need to
    // recompute their types and other metadata. For the time being,
    // do a full re-validation.
    revalidate(module)
}

fn revalidate(
    module: Module,
) -> Result<(Cow<'static, Module>, Cow<'static, ModuleInfo>), PipelineConstantError> {
    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());
    let module_info = validator.validate_resolved_overrides(&module)?;
    Ok((Cow::Owned(module), Cow::Owned(module_info)))
}

fn process_workgroup_size_override(
    module: &mut Module,
    adjusted_global_expressions: &HandleVec<Expression, Handle<Expression>>,
    ep: &mut crate::EntryPoint,
) -> Result<(), PipelineConstantError> {
    match ep.workgroup_size_overrides {
        None => {}
        Some(overrides) => {
            overrides.iter().enumerate().try_for_each(
                |(i, overridden)| -> Result<(), PipelineConstantError> {
                    match *overridden {
                        None => Ok(()),
                        Some(h) => {
                            ep.workgroup_size[i] = module
                                .to_ctx()
                                .get_const_val(adjusted_global_expressions[h])
                                .map(|n| {
                                    if n == 0 {
                                        Err(PipelineConstantError::NegativeWorkgroupSize)
                                    } else {
                                        Ok(n)
                                    }
                                })
                                .map_err(|_| PipelineConstantError::NegativeWorkgroupSize)??;
                            Ok(())
                        }
                    }
                },
            )?;
            ep.workgroup_size_overrides = None;
        }
    }
    Ok(())
}

fn process_mesh_shader_overrides(
    module: &mut Module,
    adjusted_global_expressions: &HandleVec<Expression, Handle<Expression>>,
    ep: &mut crate::EntryPoint,
) -> Result<(), PipelineConstantError> {
    if let Some(ref mut mesh_info) = ep.mesh_info {
        if let Some(r#override) = mesh_info.max_vertices_override {
            mesh_info.max_vertices = module
                .to_ctx()
                .get_const_val(adjusted_global_expressions[r#override])
                .map_err(|_| PipelineConstantError::NegativeMeshOutputMax)?;
        }
        if let Some(r#override) = mesh_info.max_primitives_override {
            mesh_info.max_primitives = module
                .to_ctx()
                .get_const_val(adjusted_global_expressions[r#override])
                .map_err(|_| PipelineConstantError::NegativeMeshOutputMax)?;
        }
    }
    Ok(())
}

/// Add a [`Constant`] to `module` for the override `old_h`.
///
/// Add the new `Constant` to `override_map` and `adjusted_constant_initializers`.
fn process_override(
    (old_h, r#override, span): (Handle<Override>, &mut Override, &Span),
    pipeline_constants: &PipelineConstants,
    module: &mut Module,
    override_map: &mut HandleVec<Override, Handle<Constant>>,
    adjusted_global_expressions: &HandleVec<Expression, Handle<Expression>>,
    adjusted_constant_initializers: &mut HashSet<Handle<Constant>>,
    global_expression_kind_tracker: &mut crate::proc::ExpressionKindTracker,
) -> Result<Handle<Constant>, PipelineConstantError> {
    // Determine which key to use for `r#override` in `pipeline_constants`.
    let key = if let Some(id) = r#override.id {
        Cow::Owned(id.to_string())
    } else if let Some(ref name) = r#override.name {
        Cow::Borrowed(name)
    } else {
        unreachable!();
    };

    // Generate a global expression for `r#override`'s value, either
    // from the provided `pipeline_constants` table or its initializer
    // in the module.
    let init = if let Some(value) = pipeline_constants.get::<str>(&key) {
        let literal = match module.types[r#override.ty].inner {
            TypeInner::Scalar(scalar) => map_value_to_literal(*value, scalar)?,
            _ => unreachable!(),
        };
        let expr = module
            .global_expressions
            .append(Expression::Literal(literal), Span::UNDEFINED);
        global_expression_kind_tracker.insert(expr, crate::proc::ExpressionKind::Const);
        expr
    } else if let Some(init) = r#override.init {
        adjusted_global_expressions[init]
    } else {
        return Err(PipelineConstantError::MissingValue(key.to_string()));
    };

    // Generate a new `Constant` to represent the override's value.
    let constant = Constant {
        name: r#override.name.clone(),
        ty: r#override.ty,
        init,
    };
    let h = module.constants.append(constant, *span);
    override_map.insert(old_h, h);
    adjusted_constant_initializers.insert(h);
    r#override.init = Some(init);
    Ok(h)
}

/// Replace all override expressions in `function` with fully-evaluated constants.
///
/// Replace all `Expression::Override`s in `function`'s expression arena with
/// the corresponding `Expression::Constant`s, as given in `override_map`.
/// Replace any expressions whose values are now known with their fully
/// evaluated form.
///
/// If `h` is a `Handle<Override>`, then `override_map[h]` is the
/// `Handle<Constant>` for the override's final value.
fn process_function(
    module: &mut Module,
    override_map: &HandleVec<Override, Handle<Constant>>,
    layouter: &mut crate::proc::Layouter,
    function: &mut Function,
) -> Result<(), ConstantEvaluatorError> {
    // A map from original local expression handles to
    // handles in the new, local expression arena.
    let mut adjusted_local_expressions = HandleVec::with_capacity(function.expressions.len());

    let mut local_expression_kind_tracker = crate::proc::ExpressionKindTracker::new();

    let mut expressions = mem::take(&mut function.expressions);

    // Dummy `emitter` and `block` for the constant evaluator.
    // We can ignore the concept of emitting expressions here since
    // expressions have already been covered by a `Statement::Emit`
    // in the frontend.
    // The only thing we might have to do is remove some expressions
    // that have been covered by a `Statement::Emit`. See the docs of
    // `filter_emits_in_block` for the reasoning.
    let mut emitter = Emitter::default();
    let mut block = Block::new();

    let mut evaluator = ConstantEvaluator::for_wgsl_function(
        module,
        &mut function.expressions,
        &mut local_expression_kind_tracker,
        layouter,
        &mut emitter,
        &mut block,
        false,
    );

    for (old_h, mut expr, span) in expressions.drain() {
        if let Expression::Override(h) = expr {
            expr = Expression::Constant(override_map[h]);
        }
        adjust_expr(&adjusted_local_expressions, &mut expr);
        let h = evaluator.try_eval_and_append(expr, span)?;
        adjusted_local_expressions.insert(old_h, h);
    }

    adjust_block(&adjusted_local_expressions, &mut function.body);

    filter_emits_in_block(&mut function.body, &function.expressions);

    // Update local expression initializers.
    for (_, local) in function.local_variables.iter_mut() {
        if let &mut Some(ref mut init) = &mut local.init {
            *init = adjusted_local_expressions[*init];
        }
    }

    // We've changed the keys of `function.named_expression`, so we have to
    // rebuild it from scratch.
    let named_expressions = mem::take(&mut function.named_expressions);
    for (expr_h, name) in named_expressions {
        function
            .named_expressions
            .insert(adjusted_local_expressions[expr_h], name);
    }

    Ok(())
}

/// Replace every expression handle in `expr` with its counterpart
/// given by `new_pos`.
fn adjust_expr(new_pos: &HandleVec<Expression, Handle<Expression>>, expr: &mut Expression) {
    let adjust = |expr: &mut Handle<Expression>| {
        *expr = new_pos[*expr];
    };
    match *expr {
        Expression::Compose {
            ref mut components,
            ty: _,
        } => {
            for c in components.iter_mut() {
                adjust(c);
            }
        }
        Expression::Access {
            ref mut base,
            ref mut index,
        } => {
            adjust(base);
            adjust(index);
        }
        Expression::AccessIndex {
            ref mut base,
            index: _,
        } => {
            adjust(base);
        }
        Expression::Splat {
            ref mut value,
            size: _,
        } => {
            adjust(value);
        }
        Expression::Swizzle {
            ref mut vector,
            size: _,
            pattern: _,
        } => {
            adjust(vector);
        }
        Expression::Load { ref mut pointer } => {
            adjust(pointer);
        }
        Expression::ImageSample {
            ref mut image,
            ref mut sampler,
            ref mut coordinate,
            ref mut array_index,
            ref mut offset,
            ref mut level,
            ref mut depth_ref,
            gather: _,
            clamp_to_edge: _,
        } => {
            adjust(image);
            adjust(sampler);
            adjust(coordinate);
            if let Some(e) = array_index.as_mut() {
                adjust(e);
            }
            if let Some(e) = offset.as_mut() {
                adjust(e);
            }
            match *level {
                crate::SampleLevel::Exact(ref mut expr)
                | crate::SampleLevel::Bias(ref mut expr) => {
                    adjust(expr);
                }
                crate::SampleLevel::Gradient {
                    ref mut x,
                    ref mut y,
                } => {
                    adjust(x);
                    adjust(y);
                }
                _ => {}
            }
            if let Some(e) = depth_ref.as_mut() {
                adjust(e);
            }
        }
        Expression::ImageLoad {
            ref mut image,
            ref mut coordinate,
            ref mut array_index,
            ref mut sample,
            ref mut level,
        } => {
            adjust(image);
            adjust(coordinate);
            if let Some(e) = array_index.as_mut() {
                adjust(e);
            }
            if let Some(e) = sample.as_mut() {
                adjust(e);
            }
            if let Some(e) = level.as_mut() {
                adjust(e);
            }
        }
        Expression::ImageQuery {
            ref mut image,
            ref mut query,
        } => {
            adjust(image);
            match *query {
                crate::ImageQuery::Size { ref mut level } => {
                    if let Some(e) = level.as_mut() {
                        adjust(e);
                    }
                }
                crate::ImageQuery::NumLevels
                | crate::ImageQuery::NumLayers
                | crate::ImageQuery::NumSamples => {}
            }
        }
        Expression::Unary {
            ref mut expr,
            op: _,
        } => {
            adjust(expr);
        }
        Expression::Binary {
            ref mut left,
            ref mut right,
            op: _,
        } => {
            adjust(left);
            adjust(right);
        }
        Expression::Select {
            ref mut condition,
            ref mut accept,
            ref mut reject,
        } => {
            adjust(condition);
            adjust(accept);
            adjust(reject);
        }
        Expression::Derivative {
            ref mut expr,
            axis: _,
            ctrl: _,
        } => {
            adjust(expr);
        }
        Expression::Relational {
            ref mut argument,
            fun: _,
        } => {
            adjust(argument);
        }
        Expression::Math {
            ref mut arg,
            ref mut arg1,
            ref mut arg2,
            ref mut arg3,
            fun: _,
        } => {
            adjust(arg);
            if let Some(e) = arg1.as_mut() {
                adjust(e);
            }
            if let Some(e) = arg2.as_mut() {
                adjust(e);
            }
            if let Some(e) = arg3.as_mut() {
                adjust(e);
            }
        }
        Expression::As {
            ref mut expr,
            kind: _,
            convert: _,
        } => {
            adjust(expr);
        }
        Expression::ArrayLength(ref mut expr) => {
            adjust(expr);
        }
        Expression::RayQueryGetIntersection {
            ref mut query,
            committed: _,
        } => {
            adjust(query);
        }
        Expression::Literal(_)
        | Expression::FunctionArgument(_)
        | Expression::GlobalVariable(_)
        | Expression::LocalVariable(_)
        | Expression::CallResult(_)
        | Expression::RayQueryProceedResult
        | Expression::Constant(_)
        | Expression::Override(_)
        | Expression::ZeroValue(_)
        | Expression::AtomicResult {
            ty: _,
            comparison: _,
        }
        | Expression::WorkGroupUniformLoadResult { ty: _ }
        | Expression::SubgroupBallotResult
        | Expression::SubgroupOperationResult { .. } => {}
        Expression::RayQueryVertexPositions {
            ref mut query,
            committed: _,
        } => {
            adjust(query);
        }
        Expression::CooperativeLoad { ref mut data, .. } => {
            adjust(&mut data.pointer);
            adjust(&mut data.stride);
        }
        Expression::CooperativeMultiplyAdd {
            ref mut a,
            ref mut b,
            ref mut c,
        } => {
            adjust(a);
            adjust(b);
            adjust(c);
        }
    }
}

/// Replace every expression handle in `block` with its counterpart
/// given by `new_pos`.
fn adjust_block(new_pos: &HandleVec<Expression, Handle<Expression>>, block: &mut Block) {
    for stmt in block.iter_mut() {
        adjust_stmt(new_pos, stmt);
    }
}

/// Replace every expression handle in `stmt` with its counterpart
/// given by `new_pos`.
fn adjust_stmt(new_pos: &HandleVec<Expression, Handle<Expression>>, stmt: &mut Statement) {
    let adjust = |expr: &mut Handle<Expression>| {
        *expr = new_pos[*expr];
    };
    match *stmt {
        Statement::Emit(ref mut range) => {
            if let Some((mut first, mut last)) = range.first_and_last() {
                adjust(&mut first);
                adjust(&mut last);
                *range = Range::new_from_bounds(first, last);
            }
        }
        Statement::Block(ref mut block) => {
            adjust_block(new_pos, block);
        }
        Statement::If {
            ref mut condition,
            ref mut accept,
            ref mut reject,
        } => {
            adjust(condition);
            adjust_block(new_pos, accept);
            adjust_block(new_pos, reject);
        }
        Statement::Switch {
            ref mut selector,
            ref mut cases,
        } => {
            adjust(selector);
            for case in cases.iter_mut() {
                adjust_block(new_pos, &mut case.body);
            }
        }
        Statement::Loop {
            ref mut body,
            ref mut continuing,
            ref mut break_if,
        } => {
            adjust_block(new_pos, body);
            adjust_block(new_pos, continuing);
            if let Some(e) = break_if.as_mut() {
                adjust(e);
            }
        }
        Statement::Return { ref mut value } => {
            if let Some(e) = value.as_mut() {
                adjust(e);
            }
        }
        Statement::Store {
            ref mut pointer,
            ref mut value,
        } => {
            adjust(pointer);
            adjust(value);
        }
        Statement::ImageStore {
            ref mut image,
            ref mut coordinate,
            ref mut array_index,
            ref mut value,
        } => {
            adjust(image);
            adjust(coordinate);
            if let Some(e) = array_index.as_mut() {
                adjust(e);
            }
            adjust(value);
        }
        Statement::Atomic {
            ref mut pointer,
            ref mut value,
            ref mut result,
            ref mut fun,
        } => {
            adjust(pointer);
            adjust(value);
            if let Some(ref mut result) = *result {
                adjust(result);
            }
            match *fun {
                crate::AtomicFunction::Exchange {
                    compare: Some(ref mut compare),
                } => {
                    adjust(compare);
                }
                crate::AtomicFunction::Add
                | crate::AtomicFunction::Subtract
                | crate::AtomicFunction::And
                | crate::AtomicFunction::ExclusiveOr
                | crate::AtomicFunction::InclusiveOr
                | crate::AtomicFunction::Min
                | crate::AtomicFunction::Max
                | crate::AtomicFunction::Exchange { compare: None } => {}
            }
        }
        Statement::ImageAtomic {
            ref mut image,
            ref mut coordinate,
            ref mut array_index,
            fun: _,
            ref mut value,
        } => {
            adjust(image);
            adjust(coordinate);
            if let Some(ref mut array_index) = *array_index {
                adjust(array_index);
            }
            adjust(value);
        }
        Statement::WorkGroupUniformLoad {
            ref mut pointer,
            ref mut result,
        } => {
            adjust(pointer);
            adjust(result);
        }
        Statement::SubgroupBallot {
            ref mut result,
            ref mut predicate,
        } => {
            if let Some(ref mut predicate) = *predicate {
                adjust(predicate);
            }
            adjust(result);
        }
        Statement::SubgroupCollectiveOperation {
            ref mut argument,
            ref mut result,
            ..
        } => {
            adjust(argument);
            adjust(result);
        }
        Statement::SubgroupGather {
            ref mut mode,
            ref mut argument,
            ref mut result,
        } => {
            match *mode {
                crate::GatherMode::BroadcastFirst => {}
                crate::GatherMode::Broadcast(ref mut index)
                | crate::GatherMode::Shuffle(ref mut index)
                | crate::GatherMode::ShuffleDown(ref mut index)
                | crate::GatherMode::ShuffleUp(ref mut index)
                | crate::GatherMode::ShuffleXor(ref mut index)
                | crate::GatherMode::QuadBroadcast(ref mut index) => {
                    adjust(index);
                }
                crate::GatherMode::QuadSwap(_) => {}
            }
            adjust(argument);
            adjust(result)
        }
        Statement::Call {
            ref mut arguments,
            ref mut result,
            function: _,
        } => {
            for argument in arguments.iter_mut() {
                adjust(argument);
            }
            if let Some(e) = result.as_mut() {
                adjust(e);
            }
        }
        Statement::RayQuery {
            ref mut query,
            ref mut fun,
        } => {
            adjust(query);
            match *fun {
                crate::RayQueryFunction::Initialize {
                    ref mut acceleration_structure,
                    ref mut descriptor,
                } => {
                    adjust(acceleration_structure);
                    adjust(descriptor);
                }
                crate::RayQueryFunction::Proceed { ref mut result } => {
                    adjust(result);
                }
                crate::RayQueryFunction::GenerateIntersection { ref mut hit_t } => {
                    adjust(hit_t);
                }
                crate::RayQueryFunction::ConfirmIntersection => {}
                crate::RayQueryFunction::Terminate => {}
            }
        }
        Statement::CooperativeStore {
            ref mut target,
            ref mut data,
        } => {
            adjust(target);
            adjust(&mut data.pointer);
            adjust(&mut data.stride);
        }
        Statement::RayPipelineFunction(ref mut func) => match *func {
            crate::RayPipelineFunction::TraceRay {
                ref mut acceleration_structure,
                ref mut descriptor,
                ref mut payload,
            } => {
                adjust(acceleration_structure);
                adjust(descriptor);
                adjust(payload);
            }
        },
        Statement::Break
        | Statement::Continue
        | Statement::Kill
        | Statement::ControlBarrier(_)
        | Statement::MemoryBarrier(_) => {}
    }
}

/// Adjust [`Emit`] statements in `block` to skip [`needs_pre_emit`] expressions we have introduced.
///
/// According to validation, [`Emit`] statements must not cover any expressions
/// for which [`Expression::needs_pre_emit`] returns true. All expressions built
/// by successful constant evaluation fall into that category, meaning that
/// `process_function` will usually rewrite [`Override`] expressions and those
/// that use their values into pre-emitted expressions, leaving any [`Emit`]
/// statements that cover them invalid.
///
/// This function rewrites all [`Emit`] statements into zero or more new
/// [`Emit`] statements covering only those expressions in the original range
/// that are not pre-emitted.
///
/// [`Emit`]: Statement::Emit
/// [`needs_pre_emit`]: Expression::needs_pre_emit
/// [`Override`]: Expression::Override
fn filter_emits_in_block(block: &mut Block, expressions: &Arena<Expression>) {
    let original = mem::replace(block, Block::with_capacity(block.len()));
    for (stmt, span) in original.span_into_iter() {
        match stmt {
            Statement::Emit(range) => {
                let mut current = None;
                for expr_h in range {
                    if expressions[expr_h].needs_pre_emit() {
                        if let Some((first, last)) = current {
                            block.push(Statement::Emit(Range::new_from_bounds(first, last)), span);
                        }

                        current = None;
                    } else if let Some((_, ref mut last)) = current {
                        *last = expr_h;
                    } else {
                        current = Some((expr_h, expr_h));
                    }
                }
                if let Some((first, last)) = current {
                    block.push(Statement::Emit(Range::new_from_bounds(first, last)), span);
                }
            }
            Statement::Block(mut child) => {
                filter_emits_in_block(&mut child, expressions);
                block.push(Statement::Block(child), span);
            }
            Statement::If {
                condition,
                mut accept,
                mut reject,
            } => {
                filter_emits_in_block(&mut accept, expressions);
                filter_emits_in_block(&mut reject, expressions);
                block.push(
                    Statement::If {
                        condition,
                        accept,
                        reject,
                    },
                    span,
                );
            }
            Statement::Switch {
                selector,
                mut cases,
            } => {
                for case in &mut cases {
                    filter_emits_in_block(&mut case.body, expressions);
                }
                block.push(Statement::Switch { selector, cases }, span);
            }
            Statement::Loop {
                mut body,
                mut continuing,
                break_if,
            } => {
                filter_emits_in_block(&mut body, expressions);
                filter_emits_in_block(&mut continuing, expressions);
                block.push(
                    Statement::Loop {
                        body,
                        continuing,
                        break_if,
                    },
                    span,
                );
            }
            stmt => block.push(stmt.clone(), span),
        }
    }
}

fn map_value_to_literal(value: f64, scalar: Scalar) -> Result<Literal, PipelineConstantError> {
    // note that in rust 0.0 == -0.0
    match scalar {
        Scalar::BOOL => {
            // https://webidl.spec.whatwg.org/#js-boolean
            let value = value != 0.0 && !value.is_nan();
            Ok(Literal::Bool(value))
        }
        Scalar::I32 => {
            // https://webidl.spec.whatwg.org/#js-long
            if !value.is_finite() {
                return Err(PipelineConstantError::SrcNeedsToBeFinite);
            }

            let value = value.trunc();
            if value < f64::from(i32::MIN) || value > f64::from(i32::MAX) {
                return Err(PipelineConstantError::DstRangeTooSmall);
            }

            let value = value as i32;
            Ok(Literal::I32(value))
        }
        Scalar::U32 => {
            // https://webidl.spec.whatwg.org/#js-unsigned-long
            if !value.is_finite() {
                return Err(PipelineConstantError::SrcNeedsToBeFinite);
            }

            let value = value.trunc();
            if value < f64::from(u32::MIN) || value > f64::from(u32::MAX) {
                return Err(PipelineConstantError::DstRangeTooSmall);
            }

            let value = value as u32;
            Ok(Literal::U32(value))
        }
        Scalar::F16 => {
            // https://webidl.spec.whatwg.org/#js-float
            if !value.is_finite() {
                return Err(PipelineConstantError::SrcNeedsToBeFinite);
            }

            let value = half::f16::from_f64(value);
            if !value.is_finite() {
                return Err(PipelineConstantError::DstRangeTooSmall);
            }

            Ok(Literal::F16(value))
        }
        Scalar::F32 => {
            // https://webidl.spec.whatwg.org/#js-float
            if !value.is_finite() {
                return Err(PipelineConstantError::SrcNeedsToBeFinite);
            }

            let value = value as f32;
            if !value.is_finite() {
                return Err(PipelineConstantError::DstRangeTooSmall);
            }

            Ok(Literal::F32(value))
        }
        Scalar::F64 => {
            // https://webidl.spec.whatwg.org/#js-double
            if !value.is_finite() {
                return Err(PipelineConstantError::SrcNeedsToBeFinite);
            }

            Ok(Literal::F64(value))
        }
        Scalar::ABSTRACT_FLOAT | Scalar::ABSTRACT_INT => {
            unreachable!("abstract values should not be validated out of override processing")
        }
        _ => unreachable!("unrecognized scalar type for override"),
    }
}

#[test]
fn test_map_value_to_literal() {
    let bool_test_cases = [
        (0.0, false),
        (-0.0, false),
        (f64::NAN, false),
        (1.0, true),
        (f64::INFINITY, true),
        (f64::NEG_INFINITY, true),
    ];
    for (value, out) in bool_test_cases {
        let res = Ok(Literal::Bool(out));
        assert_eq!(map_value_to_literal(value, Scalar::BOOL), res);
    }

    for scalar in [Scalar::I32, Scalar::U32, Scalar::F32, Scalar::F64] {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let res = Err(PipelineConstantError::SrcNeedsToBeFinite);
            assert_eq!(map_value_to_literal(value, scalar), res);
        }
    }

    // i32
    assert_eq!(
        map_value_to_literal(f64::from(i32::MIN), Scalar::I32),
        Ok(Literal::I32(i32::MIN))
    );
    assert_eq!(
        map_value_to_literal(f64::from(i32::MAX), Scalar::I32),
        Ok(Literal::I32(i32::MAX))
    );
    assert_eq!(
        map_value_to_literal(f64::from(i32::MIN) - 1.0, Scalar::I32),
        Err(PipelineConstantError::DstRangeTooSmall)
    );
    assert_eq!(
        map_value_to_literal(f64::from(i32::MAX) + 1.0, Scalar::I32),
        Err(PipelineConstantError::DstRangeTooSmall)
    );

    // u32
    assert_eq!(
        map_value_to_literal(f64::from(u32::MIN), Scalar::U32),
        Ok(Literal::U32(u32::MIN))
    );
    assert_eq!(
        map_value_to_literal(f64::from(u32::MAX), Scalar::U32),
        Ok(Literal::U32(u32::MAX))
    );
    assert_eq!(
        map_value_to_literal(f64::from(u32::MIN) - 1.0, Scalar::U32),
        Err(PipelineConstantError::DstRangeTooSmall)
    );
    assert_eq!(
        map_value_to_literal(f64::from(u32::MAX) + 1.0, Scalar::U32),
        Err(PipelineConstantError::DstRangeTooSmall)
    );

    // f32
    assert_eq!(
        map_value_to_literal(f64::from(f32::MIN), Scalar::F32),
        Ok(Literal::F32(f32::MIN))
    );
    assert_eq!(
        map_value_to_literal(f64::from(f32::MAX), Scalar::F32),
        Ok(Literal::F32(f32::MAX))
    );
    assert_eq!(
        map_value_to_literal(-f64::from_bits(0x47efffffefffffff), Scalar::F32),
        Ok(Literal::F32(f32::MIN))
    );
    assert_eq!(
        map_value_to_literal(f64::from_bits(0x47efffffefffffff), Scalar::F32),
        Ok(Literal::F32(f32::MAX))
    );
    assert_eq!(
        map_value_to_literal(-f64::from_bits(0x47effffff0000000), Scalar::F32),
        Err(PipelineConstantError::DstRangeTooSmall)
    );
    assert_eq!(
        map_value_to_literal(f64::from_bits(0x47effffff0000000), Scalar::F32),
        Err(PipelineConstantError::DstRangeTooSmall)
    );

    // f64
    assert_eq!(
        map_value_to_literal(f64::MIN, Scalar::F64),
        Ok(Literal::F64(f64::MIN))
    );
    assert_eq!(
        map_value_to_literal(f64::MAX, Scalar::F64),
        Ok(Literal::F64(f64::MAX))
    );
}
