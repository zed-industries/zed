//! Implementation of `Validator::validate_module_handles`.

use core::{convert::TryInto, hash::Hash};

use super::{TypeError, ValidationError};
use crate::non_max_u32::NonMaxU32;
use crate::{
    arena::{BadHandle, BadRangeError},
    diagnostic_filter::DiagnosticFilterNode,
    EntryPoint, Handle,
};
use crate::{Arena, UniqueArena};

use alloc::string::ToString;

impl super::Validator {
    /// Validates that all handles within `module` are:
    ///
    /// * Valid, in the sense that they contain indices within each arena structure inside the
    ///   [`crate::Module`] type.
    /// * No arena contents contain any items that have forward dependencies; that is, the value
    ///   associated with a handle only may contain references to handles in the same arena that
    ///   were constructed before it.
    ///
    /// By validating the above conditions, we free up subsequent logic to assume that handle
    /// accesses are infallible.
    ///
    /// # Errors
    ///
    /// Errors returned by this method are intentionally sparse, for simplicity of implementation.
    /// It is expected that only buggy frontends or fuzzers should ever emit IR that fails this
    /// validation pass.
    pub(super) fn validate_module_handles(module: &crate::Module) -> Result<(), ValidationError> {
        let &crate::Module {
            ref constants,
            ref overrides,
            ref entry_points,
            ref functions,
            ref global_variables,
            ref types,
            ref special_types,
            ref global_expressions,
            ref diagnostic_filters,
            ref diagnostic_filter_leaf,
            ref doc_comments,
        } = module;

        // Because types can refer to global expressions and vice versa, to
        // ensure the overall structure is free of cycles, we must traverse them
        // both in tandem.
        //
        // Try to visit all types and global expressions in an order such that
        // each item refers only to previously visited items. If we succeed,
        // that shows that there cannot be any cycles, since walking any edge
        // advances you towards the beginning of the visiting order.
        //
        // Validate all the handles in types and expressions as we traverse the
        // arenas.
        let mut global_exprs_iter = global_expressions.iter().peekable();
        for (th, t) in types.iter() {
            // Imagine the `for` loop and `global_exprs_iter` as two fingers
            // walking the type and global expression arenas. They don't visit
            // elements at the same rate: sometimes one processes a bunch of
            // elements while the other one stays still. But at each point, they
            // check that the two ranges of elements they've visited only refer
            // to other elements in those ranges.
            //
            // For brevity, we'll say 'handles behind `global_exprs_iter`' to
            // mean handles that have already been produced by
            // `global_exprs_iter`. Once `global_exprs_iter` returns `None`, all
            // global expression handles are 'behind' it.
            //
            // At this point:
            //
            // - All types visited by prior iterations (that is, before
            //   `th`/`t`) refer only to expressions behind `global_exprs_iter`.
            //
            //   On the first iteration, this is obviously true: there are no
            //   prior iterations, and `global_exprs_iter` hasn't produced
            //   anything yet. At the bottom of the loop, we'll claim that it's
            //   true for `th`/`t` as well, so the condition remains true when
            //   we advance to the next type.
            //
            // - All expressions behind `global_exprs_iter` refer only to
            //   previously visited types.
            //
            //   Again, trivially true at the start, and we'll show it's true
            //   about each expression that `global_exprs_iter` produces.
            //
            // Once we also check that arena elements only refer to prior
            // elements in that arena, we can see that `th`/`t` does not
            // participate in a cycle: it only refers to previously visited
            // types and expressions behind `global_exprs_iter`, and none of
            // those refer to `th`/`t`, because they passed the same checks
            // before we reached `th`/`t`.
            if let Some(max_expr) = Self::validate_type_handles((th, t), overrides)? {
                max_expr.check_valid_for(global_expressions)?;
                // Since `t` refers to `max_expr`, if we want our invariants to
                // remain true, we must advance `global_exprs_iter` beyond
                // `max_expr`.
                while let Some((eh, e)) = global_exprs_iter.next_if(|&(eh, _)| eh <= max_expr) {
                    if let Some(max_type) =
                        Self::validate_const_expression_handles((eh, e), constants, overrides)?
                    {
                        // Show that `eh` refers only to previously visited types.
                        th.check_dep(max_type)?;
                    }
                    // We've advanced `global_exprs_iter` past `eh` already. But
                    // since we now know that `eh` refers only to previously
                    // visited types, it is again true that all expressions
                    // behind `global_exprs_iter` refer only to previously
                    // visited types. So we can continue to the next expression.
                }
            }

            // Here we know that if `th` refers to any expressions at all,
            // `max_expr` is the latest one. And we know that `max_expr` is
            // behind `global_exprs_iter`. So `th` refers only to expressions
            // behind `global_exprs_iter`, and the invariants will still be
            // true on the next iteration.
        }

        // Since we also enforced the usual intra-arena rules that expressions
        // refer only to prior expressions, expressions can only form cycles if
        // they include types. But we've shown that all types are acyclic, so
        // all expressions must be acyclic as well.
        //
        // Validate the remaining expressions normally.
        for handle_and_expr in global_exprs_iter {
            Self::validate_const_expression_handles(handle_and_expr, constants, overrides)?;
        }

        let validate_type = |handle| Self::validate_type_handle(handle, types);
        let validate_const_expr =
            |handle| Self::validate_expression_handle(handle, global_expressions);

        for (_handle, constant) in constants.iter() {
            let &crate::Constant { name: _, ty, init } = constant;
            validate_type(ty)?;
            validate_const_expr(init)?;
        }

        for (_handle, r#override) in overrides.iter() {
            let &crate::Override {
                name: _,
                id: _,
                ty,
                init,
            } = r#override;
            validate_type(ty)?;
            if let Some(init_expr) = init {
                validate_const_expr(init_expr)?;
            }
        }

        for (_handle, global_variable) in global_variables.iter() {
            let &crate::GlobalVariable {
                name: _,
                space: _,
                binding: _,
                ty,
                init,
                memory_decorations: _,
            } = global_variable;
            validate_type(ty)?;
            if let Some(init_expr) = init {
                validate_const_expr(init_expr)?;
            }
        }

        let validate_function = |function_handle, function: &_| -> Result<_, InvalidHandleError> {
            let &crate::Function {
                name: _,
                ref arguments,
                ref result,
                ref local_variables,
                ref expressions,
                ref named_expressions,
                ref body,
                ref diagnostic_filter_leaf,
            } = function;

            for arg in arguments.iter() {
                let &crate::FunctionArgument {
                    name: _,
                    ty,
                    binding: _,
                } = arg;
                validate_type(ty)?;
            }

            if let &Some(crate::FunctionResult { ty, binding: _ }) = result {
                validate_type(ty)?;
            }

            for (_handle, local_variable) in local_variables.iter() {
                let &crate::LocalVariable { name: _, ty, init } = local_variable;
                validate_type(ty)?;
                if let Some(init) = init {
                    Self::validate_expression_handle(init, expressions)?;
                }
            }

            for handle in named_expressions.keys().copied() {
                Self::validate_expression_handle(handle, expressions)?;
            }

            for handle_and_expr in expressions.iter() {
                Self::validate_expression_handles(
                    handle_and_expr,
                    constants,
                    overrides,
                    types,
                    local_variables,
                    global_variables,
                    functions,
                    function_handle,
                )?;
            }

            Self::validate_block_handles(body, expressions, functions)?;

            if let Some(handle) = *diagnostic_filter_leaf {
                handle.check_valid_for(diagnostic_filters)?;
            }

            Ok(())
        };

        for entry_point in entry_points.iter() {
            validate_function(None, &entry_point.function)?;
            if let Some(sizes) = entry_point.workgroup_size_overrides {
                for size in sizes.iter().filter_map(|x| *x) {
                    validate_const_expr(size)?;
                }
            }
            if let Some(task_payload) = entry_point.task_payload {
                Self::validate_global_variable_handle(task_payload, global_variables)?;
            }
            if let Some(ref mesh_info) = entry_point.mesh_info {
                Self::validate_global_variable_handle(mesh_info.output_variable, global_variables)?;
                validate_type(mesh_info.vertex_output_type)?;
                validate_type(mesh_info.primitive_output_type)?;
                for ov in mesh_info
                    .max_vertices_override
                    .iter()
                    .chain(mesh_info.max_primitives_override.iter())
                {
                    validate_const_expr(*ov)?;
                }
            }
        }

        for (function_handle, function) in functions.iter() {
            validate_function(Some(function_handle), function)?;
        }

        if let Some(ty) = special_types.ray_desc {
            validate_type(ty)?;
        }
        if let Some(ty) = special_types.ray_intersection {
            validate_type(ty)?;
        }
        if let Some(ty) = special_types.ray_vertex_return {
            validate_type(ty)?;
        }

        for (handle, _node) in diagnostic_filters.iter() {
            let DiagnosticFilterNode { inner: _, parent } = diagnostic_filters[handle];
            handle.check_dep_opt(parent)?;
        }
        if let Some(handle) = *diagnostic_filter_leaf {
            handle.check_valid_for(diagnostic_filters)?;
        }

        if let Some(doc_comments) = doc_comments.as_ref() {
            let crate::DocComments {
                module: _,
                types: ref doc_comments_for_types,
                struct_members: ref doc_comments_for_struct_members,
                entry_points: ref doc_comments_for_entry_points,
                functions: ref doc_comments_for_functions,
                constants: ref doc_comments_for_constants,
                global_variables: ref doc_comments_for_global_variables,
            } = **doc_comments;

            for (&ty, _) in doc_comments_for_types.iter() {
                validate_type(ty)?;
            }

            for (&(ty, struct_member_index), _) in doc_comments_for_struct_members.iter() {
                validate_type(ty)?;
                let struct_type = types.get_handle(ty).unwrap();
                match struct_type.inner {
                    crate::TypeInner::Struct {
                        ref members,
                        span: ref _span,
                    } => {
                        (0..members.len())
                            .contains(&struct_member_index)
                            .then_some(())
                            // TODO: what errors should this be?
                            .ok_or_else(|| ValidationError::Type {
                                handle: ty,
                                name: struct_type.name.as_ref().map_or_else(
                                    || "members length incorrect".to_string(),
                                    |name| name.to_string(),
                                ),
                                source: TypeError::InvalidData(ty),
                            })?;
                    }
                    _ => {
                        // TODO: internal error ? We should never get here.
                        // If entering there, it's probably that we forgot to adjust a handle in the compact phase.
                        return Err(ValidationError::Type {
                            handle: ty,
                            name: struct_type
                                .name
                                .as_ref()
                                .map_or_else(|| "Unknown".to_string(), |name| name.to_string()),
                            source: TypeError::InvalidData(ty),
                        });
                    }
                }
                for (&function, _) in doc_comments_for_functions.iter() {
                    Self::validate_function_handle(function, functions)?;
                }
                for (&entry_point_index, _) in doc_comments_for_entry_points.iter() {
                    Self::validate_entry_point_index(entry_point_index, entry_points)?;
                }
                for (&constant, _) in doc_comments_for_constants.iter() {
                    Self::validate_constant_handle(constant, constants)?;
                }
                for (&global_variable, _) in doc_comments_for_global_variables.iter() {
                    Self::validate_global_variable_handle(global_variable, global_variables)?;
                }
            }
        }

        Ok(())
    }

    fn validate_type_handle(
        handle: Handle<crate::Type>,
        types: &UniqueArena<crate::Type>,
    ) -> Result<(), InvalidHandleError> {
        handle.check_valid_for_uniq(types).map(|_| ())
    }

    fn validate_constant_handle(
        handle: Handle<crate::Constant>,
        constants: &Arena<crate::Constant>,
    ) -> Result<(), InvalidHandleError> {
        handle.check_valid_for(constants).map(|_| ())
    }

    fn validate_global_variable_handle(
        handle: Handle<crate::GlobalVariable>,
        global_variables: &Arena<crate::GlobalVariable>,
    ) -> Result<(), InvalidHandleError> {
        handle.check_valid_for(global_variables).map(|_| ())
    }

    fn validate_override_handle(
        handle: Handle<crate::Override>,
        overrides: &Arena<crate::Override>,
    ) -> Result<(), InvalidHandleError> {
        handle.check_valid_for(overrides).map(|_| ())
    }

    fn validate_expression_handle(
        handle: Handle<crate::Expression>,
        expressions: &Arena<crate::Expression>,
    ) -> Result<(), InvalidHandleError> {
        handle.check_valid_for(expressions).map(|_| ())
    }

    fn validate_function_handle(
        handle: Handle<crate::Function>,
        functions: &Arena<crate::Function>,
    ) -> Result<(), InvalidHandleError> {
        handle.check_valid_for(functions).map(|_| ())
    }

    /// Validate all handles that occur in `ty`, whose handle is `handle`.
    ///
    /// If `ty` refers to any expressions, return the highest-indexed expression
    /// handle that it uses. This is used for detecting cycles between the
    /// expression and type arenas.
    fn validate_type_handles(
        (handle, ty): (Handle<crate::Type>, &crate::Type),
        overrides: &Arena<crate::Override>,
    ) -> Result<Option<Handle<crate::Expression>>, InvalidHandleError> {
        let max_expr = match ty.inner {
            crate::TypeInner::Scalar { .. }
            | crate::TypeInner::Vector { .. }
            | crate::TypeInner::Matrix { .. }
            | crate::TypeInner::CooperativeMatrix { .. }
            | crate::TypeInner::ValuePointer { .. }
            | crate::TypeInner::Atomic { .. }
            | crate::TypeInner::Image { .. }
            | crate::TypeInner::Sampler { .. }
            | crate::TypeInner::AccelerationStructure { .. }
            | crate::TypeInner::RayQuery { .. } => None,
            crate::TypeInner::Pointer { base, space: _ } => {
                handle.check_dep(base)?;
                None
            }
            crate::TypeInner::Array { base, size, .. }
            | crate::TypeInner::BindingArray { base, size, .. } => {
                handle.check_dep(base)?;
                match size {
                    crate::ArraySize::Pending(h) => {
                        Self::validate_override_handle(h, overrides)?;
                        let r#override = &overrides[h];
                        handle.check_dep(r#override.ty)?;
                        r#override.init
                    }
                    crate::ArraySize::Constant(_) | crate::ArraySize::Dynamic => None,
                }
            }
            crate::TypeInner::Struct {
                ref members,
                span: _,
            } => {
                handle.check_dep_iter(members.iter().map(|m| m.ty))?;
                None
            }
        };

        Ok(max_expr)
    }

    fn validate_entry_point_index(
        entry_point_index: usize,
        entry_points: &[EntryPoint],
    ) -> Result<(), InvalidHandleError> {
        (0..entry_points.len())
            .contains(&entry_point_index)
            .then_some(())
            .ok_or_else(|| {
                BadHandle {
                    kind: "EntryPoint",
                    index: entry_point_index,
                }
                .into()
            })
    }

    /// Validate all handles that occur in `expression`, whose handle is `handle`.
    ///
    /// If `expression` refers to any `Type`s, return the highest-indexed type
    /// handle that it uses. This is used for detecting cycles between the
    /// expression and type arenas.
    fn validate_const_expression_handles(
        (handle, expression): (Handle<crate::Expression>, &crate::Expression),
        constants: &Arena<crate::Constant>,
        overrides: &Arena<crate::Override>,
    ) -> Result<Option<Handle<crate::Type>>, InvalidHandleError> {
        let validate_constant = |handle| Self::validate_constant_handle(handle, constants);
        let validate_override = |handle| Self::validate_override_handle(handle, overrides);

        let max_type = match *expression {
            crate::Expression::Literal(_) => None,
            crate::Expression::Constant(constant) => {
                validate_constant(constant)?;
                handle.check_dep(constants[constant].init)?;
                None
            }
            crate::Expression::Override(r#override) => {
                validate_override(r#override)?;
                if let Some(init) = overrides[r#override].init {
                    handle.check_dep(init)?;
                }
                None
            }
            crate::Expression::ZeroValue(ty) => Some(ty),
            crate::Expression::Compose { ty, ref components } => {
                handle.check_dep_iter(components.iter().copied())?;
                Some(ty)
            }
            _ => None,
        };
        Ok(max_type)
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_expression_handles(
        (handle, expression): (Handle<crate::Expression>, &crate::Expression),
        constants: &Arena<crate::Constant>,
        overrides: &Arena<crate::Override>,
        types: &UniqueArena<crate::Type>,
        local_variables: &Arena<crate::LocalVariable>,
        global_variables: &Arena<crate::GlobalVariable>,
        functions: &Arena<crate::Function>,
        // The handle of the current function or `None` if it's an entry point
        current_function: Option<Handle<crate::Function>>,
    ) -> Result<(), InvalidHandleError> {
        let validate_constant = |handle| Self::validate_constant_handle(handle, constants);
        let validate_override = |handle| Self::validate_override_handle(handle, overrides);
        let validate_type = |handle| Self::validate_type_handle(handle, types);

        match *expression {
            crate::Expression::Access { base, index } => {
                handle.check_dep(base)?.check_dep(index)?;
            }
            crate::Expression::AccessIndex { base, .. } => {
                handle.check_dep(base)?;
            }
            crate::Expression::Splat { value, .. } => {
                handle.check_dep(value)?;
            }
            crate::Expression::Swizzle { vector, .. } => {
                handle.check_dep(vector)?;
            }
            crate::Expression::Literal(_) => {}
            crate::Expression::Constant(constant) => {
                validate_constant(constant)?;
            }
            crate::Expression::Override(r#override) => {
                validate_override(r#override)?;
            }
            crate::Expression::ZeroValue(ty) => {
                validate_type(ty)?;
            }
            crate::Expression::Compose { ty, ref components } => {
                validate_type(ty)?;
                handle.check_dep_iter(components.iter().copied())?;
            }
            crate::Expression::FunctionArgument(_arg_idx) => (),
            crate::Expression::GlobalVariable(global_variable) => {
                global_variable.check_valid_for(global_variables)?;
            }
            crate::Expression::LocalVariable(local_variable) => {
                local_variable.check_valid_for(local_variables)?;
            }
            crate::Expression::Load { pointer } => {
                handle.check_dep(pointer)?;
            }
            crate::Expression::ImageSample {
                image,
                sampler,
                gather: _,
                coordinate,
                array_index,
                offset,
                level,
                depth_ref,
                clamp_to_edge: _,
            } => {
                handle
                    .check_dep(image)?
                    .check_dep(sampler)?
                    .check_dep(coordinate)?
                    .check_dep_opt(array_index)?
                    .check_dep_opt(offset)?;

                match level {
                    crate::SampleLevel::Auto | crate::SampleLevel::Zero => (),
                    crate::SampleLevel::Exact(expr) => {
                        handle.check_dep(expr)?;
                    }
                    crate::SampleLevel::Bias(expr) => {
                        handle.check_dep(expr)?;
                    }
                    crate::SampleLevel::Gradient { x, y } => {
                        handle.check_dep(x)?.check_dep(y)?;
                    }
                };

                handle.check_dep_opt(depth_ref)?;
            }
            crate::Expression::ImageLoad {
                image,
                coordinate,
                array_index,
                sample,
                level,
            } => {
                handle
                    .check_dep(image)?
                    .check_dep(coordinate)?
                    .check_dep_opt(array_index)?
                    .check_dep_opt(sample)?
                    .check_dep_opt(level)?;
            }
            crate::Expression::ImageQuery { image, query } => {
                handle.check_dep(image)?;
                match query {
                    crate::ImageQuery::Size { level } => {
                        handle.check_dep_opt(level)?;
                    }
                    crate::ImageQuery::NumLevels
                    | crate::ImageQuery::NumLayers
                    | crate::ImageQuery::NumSamples => (),
                };
            }
            crate::Expression::Unary {
                op: _,
                expr: operand,
            } => {
                handle.check_dep(operand)?;
            }
            crate::Expression::Binary { op: _, left, right } => {
                handle.check_dep(left)?.check_dep(right)?;
            }
            crate::Expression::Select {
                condition,
                accept,
                reject,
            } => {
                handle
                    .check_dep(condition)?
                    .check_dep(accept)?
                    .check_dep(reject)?;
            }
            crate::Expression::Derivative { expr: argument, .. } => {
                handle.check_dep(argument)?;
            }
            crate::Expression::Relational { fun: _, argument } => {
                handle.check_dep(argument)?;
            }
            crate::Expression::Math {
                fun: _,
                arg,
                arg1,
                arg2,
                arg3,
            } => {
                handle
                    .check_dep(arg)?
                    .check_dep_opt(arg1)?
                    .check_dep_opt(arg2)?
                    .check_dep_opt(arg3)?;
            }
            crate::Expression::As {
                expr: input,
                kind: _,
                convert: _,
            } => {
                handle.check_dep(input)?;
            }
            crate::Expression::CallResult(function) => {
                Self::validate_function_handle(function, functions)?;
                if let Some(handle) = current_function {
                    handle.check_dep(function)?;
                }
            }
            crate::Expression::AtomicResult { .. }
            | crate::Expression::RayQueryProceedResult
            | crate::Expression::SubgroupBallotResult
            | crate::Expression::SubgroupOperationResult { .. }
            | crate::Expression::WorkGroupUniformLoadResult { .. } => (),
            crate::Expression::ArrayLength(array) => {
                handle.check_dep(array)?;
            }
            crate::Expression::RayQueryGetIntersection {
                query,
                committed: _,
            }
            | crate::Expression::RayQueryVertexPositions {
                query,
                committed: _,
            } => {
                handle.check_dep(query)?;
            }
            crate::Expression::CooperativeLoad { ref data, .. } => {
                handle.check_dep(data.pointer)?.check_dep(data.stride)?;
            }
            crate::Expression::CooperativeMultiplyAdd { a, b, c } => {
                handle.check_dep(a)?.check_dep(b)?.check_dep(c)?;
            }
        }
        Ok(())
    }

    fn validate_block_handles(
        block: &crate::Block,
        expressions: &Arena<crate::Expression>,
        functions: &Arena<crate::Function>,
    ) -> Result<(), InvalidHandleError> {
        let validate_block = |block| Self::validate_block_handles(block, expressions, functions);
        let validate_expr = |handle| Self::validate_expression_handle(handle, expressions);
        let validate_expr_opt = |handle_opt| {
            if let Some(handle) = handle_opt {
                validate_expr(handle)?;
            }
            Ok(())
        };

        block.iter().try_for_each(|stmt| match *stmt {
            crate::Statement::Emit(ref expr_range) => {
                expr_range.check_valid_for(expressions)?;
                Ok(())
            }
            crate::Statement::Block(ref block) => {
                validate_block(block)?;
                Ok(())
            }
            crate::Statement::If {
                condition,
                ref accept,
                ref reject,
            } => {
                validate_expr(condition)?;
                validate_block(accept)?;
                validate_block(reject)?;
                Ok(())
            }
            crate::Statement::Switch {
                selector,
                ref cases,
            } => {
                validate_expr(selector)?;
                for &crate::SwitchCase {
                    value: _,
                    ref body,
                    fall_through: _,
                } in cases
                {
                    validate_block(body)?;
                }
                Ok(())
            }
            crate::Statement::Loop {
                ref body,
                ref continuing,
                break_if,
            } => {
                validate_block(body)?;
                validate_block(continuing)?;
                validate_expr_opt(break_if)?;
                Ok(())
            }
            crate::Statement::Return { value } => validate_expr_opt(value),
            crate::Statement::Store { pointer, value } => {
                validate_expr(pointer)?;
                validate_expr(value)?;
                Ok(())
            }
            crate::Statement::ImageStore {
                image,
                coordinate,
                array_index,
                value,
            } => {
                validate_expr(image)?;
                validate_expr(coordinate)?;
                validate_expr_opt(array_index)?;
                validate_expr(value)?;
                Ok(())
            }
            crate::Statement::Atomic {
                pointer,
                fun,
                value,
                result,
            } => {
                validate_expr(pointer)?;
                match fun {
                    crate::AtomicFunction::Add
                    | crate::AtomicFunction::Subtract
                    | crate::AtomicFunction::And
                    | crate::AtomicFunction::ExclusiveOr
                    | crate::AtomicFunction::InclusiveOr
                    | crate::AtomicFunction::Min
                    | crate::AtomicFunction::Max => (),
                    crate::AtomicFunction::Exchange { compare } => validate_expr_opt(compare)?,
                };
                validate_expr(value)?;
                if let Some(result) = result {
                    validate_expr(result)?;
                }
                Ok(())
            }
            crate::Statement::ImageAtomic {
                image,
                coordinate,
                array_index,
                fun: _,
                value,
            } => {
                validate_expr(image)?;
                validate_expr(coordinate)?;
                validate_expr_opt(array_index)?;
                validate_expr(value)?;
                Ok(())
            }
            crate::Statement::WorkGroupUniformLoad { pointer, result } => {
                validate_expr(pointer)?;
                validate_expr(result)?;
                Ok(())
            }
            crate::Statement::Call {
                function,
                ref arguments,
                result,
            } => {
                Self::validate_function_handle(function, functions)?;
                for arg in arguments.iter().copied() {
                    validate_expr(arg)?;
                }
                validate_expr_opt(result)?;
                Ok(())
            }
            crate::Statement::RayQuery { query, ref fun } => {
                validate_expr(query)?;
                match *fun {
                    crate::RayQueryFunction::Initialize {
                        acceleration_structure,
                        descriptor,
                    } => {
                        validate_expr(acceleration_structure)?;
                        validate_expr(descriptor)?;
                    }
                    crate::RayQueryFunction::Proceed { result } => {
                        validate_expr(result)?;
                    }
                    crate::RayQueryFunction::GenerateIntersection { hit_t } => {
                        validate_expr(hit_t)?;
                    }
                    crate::RayQueryFunction::ConfirmIntersection => {}
                    crate::RayQueryFunction::Terminate => {}
                }
                Ok(())
            }
            crate::Statement::SubgroupBallot { result, predicate } => {
                validate_expr_opt(predicate)?;
                validate_expr(result)?;
                Ok(())
            }
            crate::Statement::SubgroupCollectiveOperation {
                op: _,
                collective_op: _,
                argument,
                result,
            } => {
                validate_expr(argument)?;
                validate_expr(result)?;
                Ok(())
            }
            crate::Statement::SubgroupGather {
                mode,
                argument,
                result,
            } => {
                validate_expr(argument)?;
                match mode {
                    crate::GatherMode::BroadcastFirst => {}
                    crate::GatherMode::Broadcast(index)
                    | crate::GatherMode::Shuffle(index)
                    | crate::GatherMode::ShuffleDown(index)
                    | crate::GatherMode::ShuffleUp(index)
                    | crate::GatherMode::ShuffleXor(index)
                    | crate::GatherMode::QuadBroadcast(index) => validate_expr(index)?,
                    crate::GatherMode::QuadSwap(_) => {}
                }
                validate_expr(result)?;
                Ok(())
            }
            crate::Statement::CooperativeStore { target, ref data } => {
                validate_expr(target)?;
                validate_expr(data.pointer)?;
                validate_expr(data.stride)?;
                Ok(())
            }
            crate::Statement::RayPipelineFunction(fun) => match fun {
                crate::RayPipelineFunction::TraceRay {
                    acceleration_structure,
                    descriptor,
                    payload,
                } => {
                    validate_expr(acceleration_structure)?;
                    validate_expr(descriptor)?;
                    validate_expr(payload)?;
                    Ok(())
                }
            },
            crate::Statement::Break
            | crate::Statement::Continue
            | crate::Statement::Kill
            | crate::Statement::ControlBarrier(_)
            | crate::Statement::MemoryBarrier(_) => Ok(()),
        })
    }
}

impl From<BadHandle> for ValidationError {
    fn from(source: BadHandle) -> Self {
        Self::InvalidHandle(source.into())
    }
}

impl From<FwdDepError> for ValidationError {
    fn from(source: FwdDepError) -> Self {
        Self::InvalidHandle(source.into())
    }
}

impl From<BadRangeError> for ValidationError {
    fn from(source: BadRangeError) -> Self {
        Self::InvalidHandle(source.into())
    }
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum InvalidHandleError {
    #[error(transparent)]
    BadHandle(#[from] BadHandle),
    #[error(transparent)]
    ForwardDependency(#[from] FwdDepError),
    #[error(transparent)]
    BadRange(#[from] BadRangeError),
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
#[error(
    "{subject:?} of kind {subject_kind:?} depends on {depends_on:?} of kind {depends_on_kind}, \
    which has not been processed yet"
)]
pub struct FwdDepError {
    // This error is used for many `Handle` types, but there's no point in making this generic, so
    // we just flatten them all to `Handle<()>` here.
    subject: Handle<()>,
    subject_kind: &'static str,
    depends_on: Handle<()>,
    depends_on_kind: &'static str,
}

impl<T> Handle<T> {
    /// Check that `self` is valid within `arena` using [`Arena::check_contains_handle`].
    pub(self) fn check_valid_for(self, arena: &Arena<T>) -> Result<(), InvalidHandleError> {
        arena.check_contains_handle(self)?;
        Ok(())
    }

    /// Check that `self` is valid within `arena` using [`UniqueArena::check_contains_handle`].
    pub(self) fn check_valid_for_uniq(
        self,
        arena: &UniqueArena<T>,
    ) -> Result<(), InvalidHandleError>
    where
        T: Eq + Hash,
    {
        arena.check_contains_handle(self)?;
        Ok(())
    }

    /// Check that `depends_on` was constructed before `self` by comparing handle indices.
    ///
    /// If `self` is a valid handle (i.e., it has been validated using [`Self::check_valid_for`])
    /// and this function returns [`Ok`], then it may be assumed that `depends_on` is also valid.
    /// In [`naga`](crate)'s current arena-based implementation, this is useful for validating
    /// recursive definitions of arena-based values in linear time.
    ///
    /// # Errors
    ///
    /// If `depends_on`'s handle is from the same [`Arena`] as `self'`s, but not constructed earlier
    /// than `self`'s, this function returns an error.
    pub(self) fn check_dep(self, depends_on: Self) -> Result<Self, FwdDepError> {
        if depends_on < self {
            Ok(self)
        } else {
            let erase_handle_type = |handle: Handle<_>| {
                Handle::new(NonMaxU32::new((handle.index()).try_into().unwrap()).unwrap())
            };
            Err(FwdDepError {
                subject: erase_handle_type(self),
                subject_kind: core::any::type_name::<T>(),
                depends_on: erase_handle_type(depends_on),
                depends_on_kind: core::any::type_name::<T>(),
            })
        }
    }

    /// Like [`Self::check_dep`], except for [`Option`]al handle values.
    pub(self) fn check_dep_opt(self, depends_on: Option<Self>) -> Result<Self, FwdDepError> {
        self.check_dep_iter(depends_on.into_iter())
    }

    /// Like [`Self::check_dep`], except for [`Iterator`]s over handle values.
    pub(self) fn check_dep_iter(
        self,
        depends_on: impl Iterator<Item = Self>,
    ) -> Result<Self, FwdDepError> {
        for handle in depends_on {
            self.check_dep(handle)?;
        }
        Ok(self)
    }
}

impl<T> crate::arena::Range<T> {
    pub(self) fn check_valid_for(&self, arena: &Arena<T>) -> Result<(), BadRangeError> {
        arena.check_contains_range(self)
    }
}

#[test]
fn constant_deps() {
    use crate::{Constant, Expression, Literal, Span, Type, TypeInner};

    let nowhere = Span::default();

    let mut types = UniqueArena::new();
    let mut const_exprs = Arena::new();
    let mut fun_exprs = Arena::new();
    let mut constants = Arena::new();
    let overrides = Arena::new();

    let i32_handle = types.insert(
        Type {
            name: None,
            inner: TypeInner::Scalar(crate::Scalar::I32),
        },
        nowhere,
    );

    // Construct a self-referential constant by misusing a handle to
    // fun_exprs as a constant initializer.
    let fun_expr = fun_exprs.append(Expression::Literal(Literal::I32(42)), nowhere);
    let self_referential_const = constants.append(
        Constant {
            name: None,
            ty: i32_handle,
            init: fun_expr,
        },
        nowhere,
    );
    let _self_referential_expr =
        const_exprs.append(Expression::Constant(self_referential_const), nowhere);

    for handle_and_expr in const_exprs.iter() {
        assert!(super::Validator::validate_const_expression_handles(
            handle_and_expr,
            &constants,
            &overrides,
        )
        .is_err());
    }
}

#[test]
fn array_size_deps() {
    use super::Validator;
    use crate::{ArraySize, Expression, Override, Scalar, Span, Type, TypeInner};

    let nowhere = Span::default();

    let mut m = crate::Module::default();

    let ty_u32 = m.types.insert(
        Type {
            name: Some("u32".to_string()),
            inner: TypeInner::Scalar(Scalar::U32),
        },
        nowhere,
    );
    let ex_zero = m
        .global_expressions
        .append(Expression::ZeroValue(ty_u32), nowhere);
    let ty_handle = m.overrides.append(
        Override {
            name: None,
            id: None,
            ty: ty_u32,
            init: Some(ex_zero),
        },
        nowhere,
    );
    let ty_arr = m.types.insert(
        Type {
            name: Some("bad_array".to_string()),
            inner: TypeInner::Array {
                base: ty_u32,
                size: ArraySize::Pending(ty_handle),
                stride: 4,
            },
        },
        nowhere,
    );

    // Everything should be okay now.
    assert!(Validator::validate_module_handles(&m).is_ok());

    // Mutate `ex_zero`'s type to `ty_arr`, introducing a cycle.
    // Validation should catch the cycle.
    m.global_expressions[ex_zero] = Expression::ZeroValue(ty_arr);
    assert!(Validator::validate_module_handles(&m).is_err());
}

#[test]
fn array_size_override() {
    use super::Validator;
    use crate::{ArraySize, Override, Scalar, Span, Type, TypeInner};

    let nowhere = Span::default();

    let mut m = crate::Module::default();

    let ty_u32 = m.types.insert(
        Type {
            name: Some("u32".to_string()),
            inner: TypeInner::Scalar(Scalar::U32),
        },
        nowhere,
    );

    let bad_override: Handle<Override> = Handle::new(NonMaxU32::new(1000).unwrap());
    let _ty_arr = m.types.insert(
        Type {
            name: Some("bad_array".to_string()),
            inner: TypeInner::Array {
                base: ty_u32,
                size: ArraySize::Pending(bad_override),
                stride: 4,
            },
        },
        nowhere,
    );

    assert!(Validator::validate_module_handles(&m).is_err());
}

#[test]
fn override_init_deps() {
    use super::Validator;
    use crate::{ArraySize, Expression, Override, Scalar, Span, Type, TypeInner};

    let nowhere = Span::default();

    let mut m = crate::Module::default();

    let ty_u32 = m.types.insert(
        Type {
            name: Some("u32".to_string()),
            inner: TypeInner::Scalar(Scalar::U32),
        },
        nowhere,
    );
    let ex_zero = m
        .global_expressions
        .append(Expression::ZeroValue(ty_u32), nowhere);
    let r#override = m.overrides.append(
        Override {
            name: Some("bad_override".into()),
            id: None,
            ty: ty_u32,
            init: Some(ex_zero),
        },
        nowhere,
    );
    let ty_arr = m.types.insert(
        Type {
            name: Some("bad_array".to_string()),
            inner: TypeInner::Array {
                base: ty_u32,
                size: ArraySize::Pending(r#override),
                stride: 4,
            },
        },
        nowhere,
    );
    let ex_arr = m
        .global_expressions
        .append(Expression::ZeroValue(ty_arr), nowhere);

    assert!(Validator::validate_module_handles(&m).is_ok());

    // Mutate `r#override`'s initializer to `ex_arr`, introducing a cycle.
    // Validation should catch the cycle.
    m.overrides[r#override].init = Some(ex_arr);
    assert!(Validator::validate_module_handles(&m).is_err());
}
