use alloc::{format, vec, vec::Vec};

use super::{Error, Instruction, LookupExpression, LookupHelper as _};
use crate::proc::Emitter;
use crate::{
    arena::{Arena, Handle},
    front::spv::{BlockContext, BodyIndex},
};

pub type BlockId = u32;

impl<I: Iterator<Item = u32>> super::Frontend<I> {
    // Registers a function call. It will generate a dummy handle to call, which
    // gets resolved after all the functions are processed.
    pub(super) fn add_call(
        &mut self,
        from: spirv::Word,
        to: spirv::Word,
    ) -> Handle<crate::Function> {
        let dummy_handle = self
            .dummy_functions
            .append(crate::Function::default(), Default::default());
        self.deferred_function_calls.push(to);
        self.function_call_graph.add_edge(from, to, ());
        dummy_handle
    }

    pub(super) fn parse_function(&mut self, module: &mut crate::Module) -> Result<(), Error> {
        let start = self.data_offset;
        self.lookup_expression.clear();
        self.lookup_load_override.clear();
        self.lookup_sampled_image.clear();

        let result_type_id = self.next()?;
        let fun_id = self.next()?;
        let _fun_control = self.next()?;
        let fun_type_id = self.next()?;

        let mut fun = {
            let ft = self.lookup_function_type.lookup(fun_type_id)?;
            if ft.return_type_id != result_type_id {
                return Err(Error::WrongFunctionResultType(result_type_id));
            }
            crate::Function {
                name: self.future_decor.remove(&fun_id).and_then(|dec| dec.name),
                arguments: Vec::with_capacity(ft.parameter_type_ids.len()),
                result: if self.lookup_void_type == Some(result_type_id) {
                    None
                } else {
                    let lookup_result_ty = self.lookup_type.lookup(result_type_id)?;
                    Some(crate::FunctionResult {
                        ty: lookup_result_ty.handle,
                        binding: None,
                    })
                },
                local_variables: Arena::new(),
                expressions: self.make_expression_storage(
                    &module.global_variables,
                    &module.constants,
                    &module.overrides,
                ),
                named_expressions: crate::NamedExpressions::default(),
                body: crate::Block::new(),
                diagnostic_filter_leaf: None,
            }
        };

        // read parameters
        for i in 0..fun.arguments.capacity() {
            let start = self.data_offset;
            match self.next_inst()? {
                Instruction {
                    op: spirv::Op::FunctionParameter,
                    wc: 3,
                } => {
                    let type_id = self.next()?;
                    let id = self.next()?;
                    let handle = fun.expressions.append(
                        crate::Expression::FunctionArgument(i as u32),
                        self.span_from(start),
                    );
                    self.lookup_expression.insert(
                        id,
                        LookupExpression {
                            handle,
                            type_id,
                            // Setting this to an invalid id will cause get_expr_handle
                            // to default to the main body making sure no load/stores
                            // are added.
                            block_id: 0,
                        },
                    );
                    //Note: we redo the lookup in order to work around `self` borrowing

                    if type_id
                        != self
                            .lookup_function_type
                            .lookup(fun_type_id)?
                            .parameter_type_ids[i]
                    {
                        return Err(Error::WrongFunctionArgumentType(type_id));
                    }
                    let ty = self.lookup_type.lookup(type_id)?.handle;
                    let decor = self.future_decor.remove(&id).unwrap_or_default();
                    fun.arguments.push(crate::FunctionArgument {
                        name: decor.name,
                        ty,
                        binding: None,
                    });
                }
                Instruction { op, .. } => return Err(Error::InvalidParameter(op)),
            }
        }

        // Note the index this function's handle will be assigned, for tracing.
        let function_index = module.functions.len();

        // Read body
        self.function_call_graph.add_node(fun_id);
        let mut parameters_sampling =
            vec![super::image::SamplingFlags::empty(); fun.arguments.len()];

        let mut block_ctx = BlockContext {
            phis: Default::default(),
            blocks: Default::default(),
            body_for_label: Default::default(),
            mergers: Default::default(),
            bodies: Default::default(),
            module,
            function_id: fun_id,
            expressions: &mut fun.expressions,
            local_arena: &mut fun.local_variables,
            arguments: &fun.arguments,
            parameter_sampling: &mut parameters_sampling,
        };
        // Insert the main body whose parent is also himself
        block_ctx.bodies.push(super::Body::with_parent(0));

        // Scan the blocks and add them as nodes
        loop {
            let fun_inst = self.next_inst()?;
            log::debug!("{:?}", fun_inst.op);
            match fun_inst.op {
                spirv::Op::Line => {
                    fun_inst.expect(4)?;
                    let _file_id = self.next()?;
                    let _row_id = self.next()?;
                    let _col_id = self.next()?;
                }
                spirv::Op::Label => {
                    // Read the label ID
                    fun_inst.expect(2)?;
                    let block_id = self.next()?;

                    self.next_block(block_id, &mut block_ctx)?;
                }
                spirv::Op::FunctionEnd => {
                    fun_inst.expect(1)?;
                    break;
                }
                spirv::Op::ExtInst => {
                    let _ = self.next()?;
                    let _ = self.next()?;
                    let set_id = self.next()?;
                    if Some(set_id) == self.ext_non_semantic_id {
                        for _ in 0..fun_inst.wc - 4 {
                            self.next()?;
                        }
                    } else {
                        return Err(Error::UnsupportedInstruction(self.state, fun_inst.op));
                    }
                }
                _ => {
                    return Err(Error::UnsupportedInstruction(self.state, fun_inst.op));
                }
            }
        }

        if let Some(ref prefix) = self.options.block_ctx_dump_prefix {
            let dump_suffix = match self.lookup_entry_point.get(&fun_id) {
                Some(ep) => format!("block_ctx.{:?}-{}.txt", ep.stage, ep.name),
                None => format!("block_ctx.Fun-{function_index}.txt"),
            };

            cfg_if::cfg_if! {
                if #[cfg(feature = "fs")] {
                    let prefix: &std::path::Path = prefix.as_ref();
                    let dest = prefix.join(dump_suffix);
                    let dump = format!("{block_ctx:#?}");
                    if let Err(e) = std::fs::write(&dest, dump) {
                        log::error!("Unable to dump the block context into {dest:?}: {e}");
                    }
                } else {
                    log::error!("Unable to dump the block context into {prefix:?}/{dump_suffix}: file system integration was not enabled with the `fs` feature");
                }
            }
        }

        // Emit `Store` statements to properly initialize all the local variables we
        // created for `phi` expressions.
        //
        // Note that get_expr_handle also contributes slightly odd entries to this table,
        // to get the spill.
        for phi in block_ctx.phis.iter() {
            // Get a pointer to the local variable for the phi's value.
            let phi_pointer: Handle<crate::Expression> = block_ctx.expressions.append(
                crate::Expression::LocalVariable(phi.local),
                crate::Span::default(),
            );

            // At the end of each of `phi`'s predecessor blocks, store the corresponding
            // source value in the phi's local variable.
            for &(source, predecessor) in phi.expressions.iter() {
                let source_lexp = &self.lookup_expression[&source];
                let predecessor_body_idx = block_ctx.body_for_label[&predecessor];
                // If the expression is a global/argument it will have a 0 block
                // id so we must use a default value instead of panicking
                let source_body_idx = block_ctx
                    .body_for_label
                    .get(&source_lexp.block_id)
                    .copied()
                    .unwrap_or(0);

                // If the Naga `Expression` generated for `source` is in scope, then we
                // can simply store that in the phi's local variable.
                //
                // Otherwise, spill the source value to a local variable in the block that
                // defines it. (We know this store dominates the predecessor; otherwise,
                // the phi wouldn't have been able to refer to that source expression in
                // the first place.) Then, the predecessor block can count on finding the
                // source's value in that local variable.
                let value = if super::is_parent(predecessor_body_idx, source_body_idx, &block_ctx) {
                    source_lexp.handle
                } else {
                    // The source SPIR-V expression is not defined in the phi's
                    // predecessor block, nor is it a globally available expression. So it
                    // must be defined off in some other block that merely dominates the
                    // predecessor. This means that the corresponding Naga `Expression`
                    // may not be in scope in the predecessor block.
                    //
                    // In the block that defines `source`, spill it to a fresh local
                    // variable, to ensure we can still use it at the end of the
                    // predecessor.
                    let ty = self.lookup_type[&source_lexp.type_id].handle;
                    let local = block_ctx.local_arena.append(
                        crate::LocalVariable {
                            name: None,
                            ty,
                            init: None,
                        },
                        crate::Span::default(),
                    );

                    let pointer = block_ctx.expressions.append(
                        crate::Expression::LocalVariable(local),
                        crate::Span::default(),
                    );

                    // Get the spilled value of the source expression.
                    let start = block_ctx.expressions.len();
                    let expr = block_ctx
                        .expressions
                        .append(crate::Expression::Load { pointer }, crate::Span::default());
                    let range = block_ctx.expressions.range_from(start);

                    block_ctx
                        .blocks
                        .get_mut(&predecessor)
                        .unwrap()
                        .push(crate::Statement::Emit(range), crate::Span::default());

                    // At the end of the block that defines it, spill the source
                    // expression's value.
                    block_ctx
                        .blocks
                        .get_mut(&source_lexp.block_id)
                        .unwrap()
                        .push(
                            crate::Statement::Store {
                                pointer,
                                value: source_lexp.handle,
                            },
                            crate::Span::default(),
                        );

                    expr
                };

                // At the end of the phi predecessor block, store the source
                // value in the phi's value.
                block_ctx.blocks.get_mut(&predecessor).unwrap().push(
                    crate::Statement::Store {
                        pointer: phi_pointer,
                        value,
                    },
                    crate::Span::default(),
                )
            }
        }

        fun.body = block_ctx.lower();

        // done
        let fun_handle = module.functions.append(fun, self.span_from_with_op(start));
        self.lookup_function.insert(
            fun_id,
            super::LookupFunction {
                handle: fun_handle,
                parameters_sampling,
            },
        );

        if let Some(ep) = self.lookup_entry_point.remove(&fun_id) {
            self.deferred_entry_points.push((ep, fun_id));
        }

        Ok(())
    }

    pub(super) fn process_entry_point(
        &mut self,
        module: &mut crate::Module,
        ep: super::EntryPoint,
        fun_id: u32,
    ) -> Result<(), Error> {
        // create a wrapping function
        let mut function = crate::Function {
            name: Some(format!("{}_wrap", ep.name)),
            arguments: Vec::new(),
            result: None,
            local_variables: Arena::new(),
            expressions: Arena::new(),
            named_expressions: crate::NamedExpressions::default(),
            body: crate::Block::new(),
            diagnostic_filter_leaf: None,
        };

        // 1. copy the inputs from arguments to privates
        for &v_id in ep.variable_ids.iter() {
            let lvar = self.lookup_variable.lookup(v_id)?;
            if let super::Variable::Input(ref arg) = lvar.inner {
                let span = module.global_variables.get_span(lvar.handle);
                let arg_expr = function.expressions.append(
                    crate::Expression::FunctionArgument(function.arguments.len() as u32),
                    span,
                );
                let load_expr = if arg.ty == module.global_variables[lvar.handle].ty {
                    arg_expr
                } else {
                    // The only case where the type is different is if we need to treat
                    // unsigned integer as signed.
                    let mut emitter = Emitter::default();
                    emitter.start(&function.expressions);
                    let handle = function.expressions.append(
                        crate::Expression::As {
                            expr: arg_expr,
                            kind: crate::ScalarKind::Sint,
                            convert: Some(4),
                        },
                        span,
                    );
                    function.body.extend(emitter.finish(&function.expressions));
                    handle
                };
                function.body.push(
                    crate::Statement::Store {
                        pointer: function
                            .expressions
                            .append(crate::Expression::GlobalVariable(lvar.handle), span),
                        value: load_expr,
                    },
                    span,
                );

                let mut arg = arg.clone();
                if ep.stage == crate::ShaderStage::Fragment {
                    if let Some(ref mut binding) = arg.binding {
                        binding.apply_default_interpolation(&module.types[arg.ty].inner);
                    }
                }
                function.arguments.push(arg);
            }
        }
        // 2. call the wrapped function
        let fake_id = !(module.entry_points.len() as u32); // doesn't matter, as long as it's not a collision
        let dummy_handle = self.add_call(fake_id, fun_id);
        function.body.push(
            crate::Statement::Call {
                function: dummy_handle,
                arguments: Vec::new(),
                result: None,
            },
            crate::Span::default(),
        );

        // 3. copy the outputs from privates to the result
        //
        // It would be nice to share struct layout code here with `parse_type_struct`,
        // but that case needs to take into account offset decorations, which makes an
        // abstraction harder to follow than just writing out what we mean. `Layouter`
        // and `Alignment` cover the worst parts already.
        let mut members = Vec::new();
        self.layouter.update(module.to_ctx()).unwrap();
        let mut next_member_offset = 0;
        let mut struct_alignment = crate::proc::Alignment::ONE;
        let mut components = Vec::new();
        for &v_id in ep.variable_ids.iter() {
            let lvar = self.lookup_variable.lookup(v_id)?;
            if let super::Variable::Output(ref result) = lvar.inner {
                let span = module.global_variables.get_span(lvar.handle);
                let expr_handle = function
                    .expressions
                    .append(crate::Expression::GlobalVariable(lvar.handle), span);

                // Cull problematic builtins of gl_PerVertex.
                // See the docs for `Frontend::gl_per_vertex_builtin_access`.
                {
                    let ty = &module.types[result.ty];
                    if let crate::TypeInner::Struct {
                        members: ref original_members,
                        span,
                    } = ty.inner
                    {
                        let mut new_members = None;
                        for (idx, member) in original_members.iter().enumerate() {
                            if let Some(crate::Binding::BuiltIn(built_in)) = member.binding {
                                if !self.gl_per_vertex_builtin_access.contains(&built_in) {
                                    new_members.get_or_insert_with(|| original_members.clone())
                                        [idx]
                                        .binding = None;
                                }
                            }
                        }
                        if let Some(new_members) = new_members {
                            module.types.replace(
                                result.ty,
                                crate::Type {
                                    name: ty.name.clone(),
                                    inner: crate::TypeInner::Struct {
                                        members: new_members,
                                        span,
                                    },
                                },
                            );
                        }
                    }
                }

                match module.types[result.ty].inner {
                    crate::TypeInner::Struct {
                        members: ref sub_members,
                        ..
                    } => {
                        for (index, sm) in sub_members.iter().enumerate() {
                            if sm.binding.is_none() {
                                continue;
                            }
                            let mut sm = sm.clone();

                            if let Some(ref mut binding) = sm.binding {
                                if ep.stage == crate::ShaderStage::Vertex {
                                    binding.apply_default_interpolation(&module.types[sm.ty].inner);
                                }
                            }

                            let member_alignment = self.layouter[sm.ty].alignment;
                            next_member_offset = member_alignment.round_up(next_member_offset);
                            sm.offset = next_member_offset;
                            struct_alignment = struct_alignment.max(member_alignment);
                            next_member_offset += self.layouter[sm.ty].size;
                            members.push(sm);

                            components.push(function.expressions.append(
                                crate::Expression::AccessIndex {
                                    base: expr_handle,
                                    index: index as u32,
                                },
                                span,
                            ));
                        }
                    }
                    ref inner => {
                        let mut binding = result.binding.clone();
                        if let Some(ref mut binding) = binding {
                            if ep.stage == crate::ShaderStage::Vertex {
                                binding.apply_default_interpolation(inner);
                            }
                        }

                        let member_alignment = self.layouter[result.ty].alignment;
                        next_member_offset = member_alignment.round_up(next_member_offset);
                        members.push(crate::StructMember {
                            name: None,
                            ty: result.ty,
                            binding,
                            offset: next_member_offset,
                        });
                        struct_alignment = struct_alignment.max(member_alignment);
                        next_member_offset += self.layouter[result.ty].size;
                        // populate just the globals first, then do `Load` in a
                        // separate step, so that we can get a range.
                        components.push(expr_handle);
                    }
                }
            }
        }

        for (member_index, member) in members.iter().enumerate() {
            match member.binding {
                Some(crate::Binding::BuiltIn(crate::BuiltIn::Position { .. }))
                    if self.options.adjust_coordinate_space =>
                {
                    let mut emitter = Emitter::default();
                    emitter.start(&function.expressions);
                    let global_expr = components[member_index];
                    let span = function.expressions.get_span(global_expr);
                    let access_expr = function.expressions.append(
                        crate::Expression::AccessIndex {
                            base: global_expr,
                            index: 1,
                        },
                        span,
                    );
                    let load_expr = function.expressions.append(
                        crate::Expression::Load {
                            pointer: access_expr,
                        },
                        span,
                    );
                    let neg_expr = function.expressions.append(
                        crate::Expression::Unary {
                            op: crate::UnaryOperator::Negate,
                            expr: load_expr,
                        },
                        span,
                    );
                    function.body.extend(emitter.finish(&function.expressions));
                    function.body.push(
                        crate::Statement::Store {
                            pointer: access_expr,
                            value: neg_expr,
                        },
                        span,
                    );
                }
                _ => {}
            }
        }

        let mut emitter = Emitter::default();
        emitter.start(&function.expressions);
        for component in components.iter_mut() {
            let load_expr = crate::Expression::Load {
                pointer: *component,
            };
            let span = function.expressions.get_span(*component);
            *component = function.expressions.append(load_expr, span);
        }

        match members[..] {
            [] => {}
            [ref member] => {
                function.body.extend(emitter.finish(&function.expressions));
                let span = function.expressions.get_span(components[0]);
                function.body.push(
                    crate::Statement::Return {
                        value: components.first().cloned(),
                    },
                    span,
                );
                function.result = Some(crate::FunctionResult {
                    ty: member.ty,
                    binding: member.binding.clone(),
                });
            }
            _ => {
                let span = crate::Span::total_span(
                    components.iter().map(|h| function.expressions.get_span(*h)),
                );
                let ty = module.types.insert(
                    crate::Type {
                        name: None,
                        inner: crate::TypeInner::Struct {
                            members,
                            span: struct_alignment.round_up(next_member_offset),
                        },
                    },
                    span,
                );
                let result_expr = function
                    .expressions
                    .append(crate::Expression::Compose { ty, components }, span);
                function.body.extend(emitter.finish(&function.expressions));
                function.body.push(
                    crate::Statement::Return {
                        value: Some(result_expr),
                    },
                    span,
                );
                function.result = Some(crate::FunctionResult { ty, binding: None });
            }
        }

        module.entry_points.push(crate::EntryPoint {
            name: ep.name,
            stage: ep.stage,
            early_depth_test: ep.early_depth_test,
            workgroup_size: ep.workgroup_size,
            workgroup_size_overrides: None,
            function,
            mesh_info: None,
            task_payload: None,
            incoming_ray_payload: None,
        });

        Ok(())
    }
}

impl BlockContext<'_> {
    pub(super) const fn gctx(&self) -> crate::proc::GlobalCtx<'_> {
        crate::proc::GlobalCtx {
            types: &self.module.types,
            constants: &self.module.constants,
            overrides: &self.module.overrides,
            global_expressions: &self.module.global_expressions,
        }
    }

    /// Consumes the `BlockContext` producing a Ir [`Block`](crate::Block)
    fn lower(mut self) -> crate::Block {
        fn lower_impl(
            blocks: &mut crate::FastHashMap<spirv::Word, crate::Block>,
            bodies: &[super::Body],
            body_idx: BodyIndex,
        ) -> crate::Block {
            let mut block = crate::Block::new();

            for item in bodies[body_idx].data.iter() {
                match *item {
                    super::BodyFragment::BlockId(id) => block.append(blocks.get_mut(&id).unwrap()),
                    super::BodyFragment::If {
                        condition,
                        accept,
                        reject,
                    } => {
                        let accept = lower_impl(blocks, bodies, accept);
                        let reject = lower_impl(blocks, bodies, reject);

                        block.push(
                            crate::Statement::If {
                                condition,
                                accept,
                                reject,
                            },
                            crate::Span::default(),
                        )
                    }
                    super::BodyFragment::Loop {
                        body,
                        continuing,
                        break_if,
                    } => {
                        let body = lower_impl(blocks, bodies, body);
                        let continuing = lower_impl(blocks, bodies, continuing);

                        block.push(
                            crate::Statement::Loop {
                                body,
                                continuing,
                                break_if,
                            },
                            crate::Span::default(),
                        )
                    }
                    super::BodyFragment::Switch {
                        selector,
                        ref cases,
                        default,
                    } => {
                        let mut ir_cases: Vec<_> = cases
                            .iter()
                            .map(|&(value, body_idx)| {
                                let body = lower_impl(blocks, bodies, body_idx);

                                // Handle simple cases that would make a fallthrough statement unreachable code
                                let fall_through = body.last().is_none_or(|s| !s.is_terminator());

                                crate::SwitchCase {
                                    value: crate::SwitchValue::I32(value),
                                    body,
                                    fall_through,
                                }
                            })
                            .collect();
                        ir_cases.push(crate::SwitchCase {
                            value: crate::SwitchValue::Default,
                            body: lower_impl(blocks, bodies, default),
                            fall_through: false,
                        });

                        block.push(
                            crate::Statement::Switch {
                                selector,
                                cases: ir_cases,
                            },
                            crate::Span::default(),
                        )
                    }
                    super::BodyFragment::Break => {
                        block.push(crate::Statement::Break, crate::Span::default())
                    }
                    super::BodyFragment::Continue => {
                        block.push(crate::Statement::Continue, crate::Span::default())
                    }
                }
            }

            block
        }

        lower_impl(&mut self.blocks, &self.bodies, 0)
    }
}
