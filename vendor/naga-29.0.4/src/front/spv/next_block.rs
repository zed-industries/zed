//! Implementation of [`Frontend::next_block()`].
//!
//! This method is split out into its own module purely because it is so long.

use alloc::{format, vec, vec::Vec};

use crate::front::spv::{
    convert::{map_binary_operator, map_relational_fun},
    image, resolve_constant, BlockContext, Body, BodyFragment, Constant, Error, Frontend,
    LookupExpression, LookupHelper as _, LookupLoadOverride, MergeBlockInformation, PhiExpression,
    SignAnchor,
};
use crate::Handle;

impl<I: Iterator<Item = u32>> Frontend<I> {
    /// Add the next SPIR-V block's contents to `block_ctx`.
    ///
    /// Except for the function's entry block, `block_id` should be the label of
    /// a block we've seen mentioned before, with an entry in
    /// `block_ctx.body_for_label` to tell us which `Body` it contributes to.
    pub(in crate::front::spv) fn next_block(
        &mut self,
        block_id: spirv::Word,
        ctx: &mut BlockContext,
    ) -> Result<(), Error> {
        // Extend `body` with the correct form for a branch to `target`.
        fn merger(body: &mut Body, target: &MergeBlockInformation) {
            body.data.push(match *target {
                MergeBlockInformation::LoopContinue => BodyFragment::Continue,
                MergeBlockInformation::LoopMerge | MergeBlockInformation::SwitchMerge => {
                    BodyFragment::Break
                }

                // Finishing a selection merge means just falling off the end of
                // the `accept` or `reject` block of the `If` statement.
                MergeBlockInformation::SelectionMerge => return,
            })
        }

        let mut emitter = crate::proc::Emitter::default();
        emitter.start(ctx.expressions);

        // Find the `Body` to which this block contributes.
        //
        // If this is some SPIR-V structured control flow construct's merge
        // block, then `body_idx` will refer to the same `Body` as the header,
        // so that we simply pick up accumulating the `Body` where the header
        // left off. Each of the statements in a block dominates the next, so
        // we're sure to encounter their SPIR-V blocks in order, ensuring that
        // the `Body` will be assembled in the proper order.
        //
        // Note that, unlike every other kind of SPIR-V block, we don't know the
        // function's first block's label in advance. Thus, we assume that if
        // this block has no entry in `ctx.body_for_label`, it must be the
        // function's first block. This always has body index zero.
        let mut body_idx = *ctx.body_for_label.entry(block_id).or_default();

        // The Naga IR block this call builds. This will end up as
        // `ctx.blocks[&block_id]`, and `ctx.bodies[body_idx]` will refer to it
        // via a `BodyFragment::BlockId`.
        let mut block = crate::Block::new();

        // Stores the merge block as defined by a `OpSelectionMerge` otherwise is `None`
        //
        // This is used in `OpSwitch` to promote the `MergeBlockInformation` from
        // `SelectionMerge` to `SwitchMerge` to allow `Break`s this isn't desirable for
        // `LoopMerge`s because otherwise `Continue`s wouldn't be allowed
        let mut selection_merge_block = None;

        macro_rules! get_expr_handle {
            ($id:expr, $lexp:expr) => {
                self.get_expr_handle($id, $lexp, ctx, &mut emitter, &mut block, body_idx)
            };
        }
        macro_rules! parse_expr_op {
            ($op:expr, BINARY) => {
                self.parse_expr_binary_op(ctx, &mut emitter, &mut block, block_id, body_idx, $op)
            };

            ($op:expr, SHIFT) => {
                self.parse_expr_shift_op(ctx, &mut emitter, &mut block, block_id, body_idx, $op)
            };
            ($op:expr, UNARY) => {
                self.parse_expr_unary_op(ctx, &mut emitter, &mut block, block_id, body_idx, $op)
            };
            ($axis:expr, $ctrl:expr, DERIVATIVE) => {
                self.parse_expr_derivative(
                    ctx,
                    &mut emitter,
                    &mut block,
                    block_id,
                    body_idx,
                    ($axis, $ctrl),
                )
            };
        }

        let terminator = loop {
            use spirv::Op;
            let start = self.data_offset;
            let inst = self.next_inst()?;
            let span = crate::Span::from(start..(start + 4 * (inst.wc as usize)));
            log::debug!("\t\t{:?} [{}]", inst.op, inst.wc);

            match inst.op {
                Op::Line => {
                    inst.expect(4)?;
                    let _file_id = self.next()?;
                    let _row_id = self.next()?;
                    let _col_id = self.next()?;
                }
                Op::NoLine => inst.expect(1)?,
                Op::Undef => {
                    inst.expect(3)?;
                    let type_id = self.next()?;
                    let id = self.next()?;
                    let type_lookup = self.lookup_type.lookup(type_id)?;
                    let ty = type_lookup.handle;

                    self.lookup_expression.insert(
                        id,
                        LookupExpression {
                            handle: ctx
                                .expressions
                                .append(crate::Expression::ZeroValue(ty), span),
                            type_id,
                            block_id,
                        },
                    );
                }
                Op::Variable => {
                    inst.expect_at_least(4)?;
                    block.extend(emitter.finish(ctx.expressions));

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let _storage_class = self.next()?;
                    let init = if inst.wc > 4 {
                        inst.expect(5)?;
                        let init_id = self.next()?;
                        let lconst = self.lookup_constant.lookup(init_id)?;
                        Some(ctx.expressions.append(lconst.inner.to_expr(), span))
                    } else {
                        None
                    };

                    let name = self
                        .future_decor
                        .remove(&result_id)
                        .and_then(|decor| decor.name);
                    if let Some(ref name) = name {
                        log::debug!("\t\t\tid={result_id} name={name}");
                    }
                    let lookup_ty = self.lookup_type.lookup(result_type_id)?;
                    let var_handle = ctx.local_arena.append(
                        crate::LocalVariable {
                            name,
                            ty: match ctx.module.types[lookup_ty.handle].inner {
                                crate::TypeInner::Pointer { base, .. } => base,
                                _ => lookup_ty.handle,
                            },
                            init,
                        },
                        span,
                    );

                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx
                                .expressions
                                .append(crate::Expression::LocalVariable(var_handle), span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                    emitter.start(ctx.expressions);
                }
                Op::Phi => {
                    inst.expect_at_least(3)?;
                    block.extend(emitter.finish(ctx.expressions));

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;

                    let name = format!("phi_{result_id}");
                    let local = ctx.local_arena.append(
                        crate::LocalVariable {
                            name: Some(name),
                            ty: self.lookup_type.lookup(result_type_id)?.handle,
                            init: None,
                        },
                        self.span_from(start),
                    );
                    let pointer = ctx
                        .expressions
                        .append(crate::Expression::LocalVariable(local), span);

                    let in_count = (inst.wc - 3) / 2;
                    let mut phi = PhiExpression {
                        local,
                        expressions: Vec::with_capacity(in_count as usize),
                    };
                    for _ in 0..in_count {
                        let expr = self.next()?;
                        let block = self.next()?;
                        phi.expressions.push((expr, block));
                    }

                    ctx.phis.push(phi);
                    emitter.start(ctx.expressions);

                    // Associate the lookup with an actual value, which is emitted
                    // into the current block.
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx
                                .expressions
                                .append(crate::Expression::Load { pointer }, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::AccessChain | Op::InBoundsAccessChain => {
                    struct AccessExpression {
                        base_handle: Handle<crate::Expression>,
                        type_id: spirv::Word,
                        load_override: Option<LookupLoadOverride>,
                    }

                    inst.expect_at_least(4)?;

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let base_id = self.next()?;
                    log::trace!("\t\t\tlooking up expr {base_id:?}");

                    let mut acex = {
                        let lexp = self.lookup_expression.lookup(base_id)?;
                        let lty = self.lookup_type.lookup(lexp.type_id)?;

                        // HACK `OpAccessChain` and `OpInBoundsAccessChain`
                        // require for the result type to be a pointer, but if
                        // we're given a pointer to an image / sampler, it will
                        // be *already* dereferenced, since we do that early
                        // during `parse_type_pointer()`.
                        //
                        // This can happen only through `BindingArray`, since
                        // that's the only case where one can obtain a pointer
                        // to an image / sampler, and so let's match on that:
                        let dereference = match ctx.module.types[lty.handle].inner {
                            crate::TypeInner::BindingArray { .. } => false,
                            _ => true,
                        };

                        let type_id = if dereference {
                            lty.base_id.ok_or(Error::InvalidAccessType(lexp.type_id))?
                        } else {
                            lexp.type_id
                        };

                        AccessExpression {
                            base_handle: get_expr_handle!(base_id, lexp),
                            type_id,
                            load_override: self.lookup_load_override.get(&base_id).cloned(),
                        }
                    };

                    for _ in 4..inst.wc {
                        let access_id = self.next()?;
                        log::trace!("\t\t\tlooking up index expr {access_id:?}");
                        let index_expr = self.lookup_expression.lookup(access_id)?.clone();
                        let index_expr_handle = get_expr_handle!(access_id, &index_expr);
                        let index_expr_data = &ctx.expressions[index_expr.handle];
                        let index_maybe = match *index_expr_data {
                            crate::Expression::Constant(const_handle) => Some(
                                ctx.gctx()
                                    .get_const_val(ctx.module.constants[const_handle].init)
                                    .map_err(|_| {
                                        Error::InvalidAccess(crate::Expression::Constant(
                                            const_handle,
                                        ))
                                    })?,
                            ),
                            _ => None,
                        };

                        log::trace!("\t\t\tlooking up type {:?}", acex.type_id);
                        let type_lookup = self.lookup_type.lookup(acex.type_id)?;
                        let ty = &ctx.module.types[type_lookup.handle];
                        acex = match ty.inner {
                            // can only index a struct with a constant
                            crate::TypeInner::Struct { ref members, .. } => {
                                let index = index_maybe
                                    .ok_or_else(|| Error::InvalidAccess(index_expr_data.clone()))?;

                                let lookup_member = self
                                    .lookup_member
                                    .get(&(type_lookup.handle, index))
                                    .ok_or(Error::InvalidAccessType(acex.type_id))?;
                                let base_handle = ctx.expressions.append(
                                    crate::Expression::AccessIndex {
                                        base: acex.base_handle,
                                        index,
                                    },
                                    span,
                                );

                                if let Some(crate::Binding::BuiltIn(built_in)) =
                                    members[index as usize].binding
                                {
                                    self.gl_per_vertex_builtin_access.insert(built_in);
                                }

                                AccessExpression {
                                    base_handle,
                                    type_id: lookup_member.type_id,
                                    load_override: if lookup_member.row_major {
                                        debug_assert!(acex.load_override.is_none());
                                        let sub_type_lookup =
                                            self.lookup_type.lookup(lookup_member.type_id)?;
                                        Some(match ctx.module.types[sub_type_lookup.handle].inner {
                                            // load it transposed, to match column major expectations
                                            crate::TypeInner::Matrix { .. } => {
                                                let loaded = ctx.expressions.append(
                                                    crate::Expression::Load {
                                                        pointer: base_handle,
                                                    },
                                                    span,
                                                );
                                                let transposed = ctx.expressions.append(
                                                    crate::Expression::Math {
                                                        fun: crate::MathFunction::Transpose,
                                                        arg: loaded,
                                                        arg1: None,
                                                        arg2: None,
                                                        arg3: None,
                                                    },
                                                    span,
                                                );
                                                LookupLoadOverride::Loaded(transposed)
                                            }
                                            _ => LookupLoadOverride::Pending,
                                        })
                                    } else {
                                        None
                                    },
                                }
                            }
                            crate::TypeInner::Matrix { .. } => {
                                let load_override = match acex.load_override {
                                    // We are indexing inside a row-major matrix
                                    Some(LookupLoadOverride::Loaded(load_expr)) => {
                                        let index = index_maybe.ok_or_else(|| {
                                            Error::InvalidAccess(index_expr_data.clone())
                                        })?;
                                        let sub_handle = ctx.expressions.append(
                                            crate::Expression::AccessIndex {
                                                base: load_expr,
                                                index,
                                            },
                                            span,
                                        );
                                        Some(LookupLoadOverride::Loaded(sub_handle))
                                    }
                                    _ => None,
                                };
                                let sub_expr = match index_maybe {
                                    Some(index) => crate::Expression::AccessIndex {
                                        base: acex.base_handle,
                                        index,
                                    },
                                    None => crate::Expression::Access {
                                        base: acex.base_handle,
                                        index: index_expr_handle,
                                    },
                                };
                                AccessExpression {
                                    base_handle: ctx.expressions.append(sub_expr, span),
                                    type_id: type_lookup
                                        .base_id
                                        .ok_or(Error::InvalidAccessType(acex.type_id))?,
                                    load_override,
                                }
                            }
                            // This must be a vector or an array.
                            _ => {
                                let base_handle = ctx.expressions.append(
                                    crate::Expression::Access {
                                        base: acex.base_handle,
                                        index: index_expr_handle,
                                    },
                                    span,
                                );
                                let load_override = match acex.load_override {
                                    // If there is a load override in place, then we always end up
                                    // with a side-loaded value here.
                                    Some(lookup_load_override) => {
                                        let sub_expr = match lookup_load_override {
                                            // We must be indexing into the array of row-major matrices.
                                            // Let's load the result of indexing and transpose it.
                                            LookupLoadOverride::Pending => {
                                                let loaded = ctx.expressions.append(
                                                    crate::Expression::Load {
                                                        pointer: base_handle,
                                                    },
                                                    span,
                                                );
                                                ctx.expressions.append(
                                                    crate::Expression::Math {
                                                        fun: crate::MathFunction::Transpose,
                                                        arg: loaded,
                                                        arg1: None,
                                                        arg2: None,
                                                        arg3: None,
                                                    },
                                                    span,
                                                )
                                            }
                                            // We are indexing inside a row-major matrix.
                                            LookupLoadOverride::Loaded(load_expr) => {
                                                ctx.expressions.append(
                                                    crate::Expression::Access {
                                                        base: load_expr,
                                                        index: index_expr_handle,
                                                    },
                                                    span,
                                                )
                                            }
                                        };
                                        Some(LookupLoadOverride::Loaded(sub_expr))
                                    }
                                    None => None,
                                };
                                AccessExpression {
                                    base_handle,
                                    type_id: type_lookup
                                        .base_id
                                        .ok_or(Error::InvalidAccessType(acex.type_id))?,
                                    load_override,
                                }
                            }
                        };
                    }

                    if let Some(load_expr) = acex.load_override {
                        self.lookup_load_override.insert(result_id, load_expr);
                    }
                    let lookup_expression = LookupExpression {
                        handle: acex.base_handle,
                        type_id: result_type_id,
                        block_id,
                    };
                    self.lookup_expression.insert(result_id, lookup_expression);
                }
                Op::VectorExtractDynamic => {
                    inst.expect(5)?;

                    let result_type_id = self.next()?;
                    let id = self.next()?;
                    let composite_id = self.next()?;
                    let index_id = self.next()?;

                    let root_lexp = self.lookup_expression.lookup(composite_id)?;
                    let root_handle = get_expr_handle!(composite_id, root_lexp);
                    let root_type_lookup = self.lookup_type.lookup(root_lexp.type_id)?;
                    let index_lexp = self.lookup_expression.lookup(index_id)?;
                    let index_handle = get_expr_handle!(index_id, index_lexp);
                    let index_type = self.lookup_type.lookup(index_lexp.type_id)?.handle;

                    let num_components = match ctx.module.types[root_type_lookup.handle].inner {
                        crate::TypeInner::Vector { size, .. } => size as u32,
                        _ => return Err(Error::InvalidVectorType(root_type_lookup.handle)),
                    };

                    let mut make_index = |ctx: &mut BlockContext, index: u32| {
                        make_index_literal(
                            ctx,
                            index,
                            &mut block,
                            &mut emitter,
                            index_type,
                            index_lexp.type_id,
                            span,
                        )
                    };

                    let index_expr = make_index(ctx, 0)?;
                    let mut handle = ctx.expressions.append(
                        crate::Expression::Access {
                            base: root_handle,
                            index: index_expr,
                        },
                        span,
                    );
                    for index in 1..num_components {
                        let index_expr = make_index(ctx, index)?;
                        let access_expr = ctx.expressions.append(
                            crate::Expression::Access {
                                base: root_handle,
                                index: index_expr,
                            },
                            span,
                        );
                        let cond = ctx.expressions.append(
                            crate::Expression::Binary {
                                op: crate::BinaryOperator::Equal,
                                left: index_expr,
                                right: index_handle,
                            },
                            span,
                        );
                        handle = ctx.expressions.append(
                            crate::Expression::Select {
                                condition: cond,
                                accept: access_expr,
                                reject: handle,
                            },
                            span,
                        );
                    }

                    self.lookup_expression.insert(
                        id,
                        LookupExpression {
                            handle,
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::VectorInsertDynamic => {
                    inst.expect(6)?;

                    let result_type_id = self.next()?;
                    let id = self.next()?;
                    let composite_id = self.next()?;
                    let object_id = self.next()?;
                    let index_id = self.next()?;

                    let object_lexp = self.lookup_expression.lookup(object_id)?;
                    let object_handle = get_expr_handle!(object_id, object_lexp);
                    let root_lexp = self.lookup_expression.lookup(composite_id)?;
                    let root_handle = get_expr_handle!(composite_id, root_lexp);
                    let root_type_lookup = self.lookup_type.lookup(root_lexp.type_id)?;
                    let index_lexp = self.lookup_expression.lookup(index_id)?;
                    let index_handle = get_expr_handle!(index_id, index_lexp);
                    let index_type = self.lookup_type.lookup(index_lexp.type_id)?.handle;

                    let num_components = match ctx.module.types[root_type_lookup.handle].inner {
                        crate::TypeInner::Vector { size, .. } => size as u32,
                        _ => return Err(Error::InvalidVectorType(root_type_lookup.handle)),
                    };

                    let mut components = Vec::with_capacity(num_components as usize);
                    for index in 0..num_components {
                        let index_expr = make_index_literal(
                            ctx,
                            index,
                            &mut block,
                            &mut emitter,
                            index_type,
                            index_lexp.type_id,
                            span,
                        )?;
                        let access_expr = ctx.expressions.append(
                            crate::Expression::Access {
                                base: root_handle,
                                index: index_expr,
                            },
                            span,
                        );
                        let cond = ctx.expressions.append(
                            crate::Expression::Binary {
                                op: crate::BinaryOperator::Equal,
                                left: index_expr,
                                right: index_handle,
                            },
                            span,
                        );
                        let handle = ctx.expressions.append(
                            crate::Expression::Select {
                                condition: cond,
                                accept: object_handle,
                                reject: access_expr,
                            },
                            span,
                        );
                        components.push(handle);
                    }
                    let handle = ctx.expressions.append(
                        crate::Expression::Compose {
                            ty: root_type_lookup.handle,
                            components,
                        },
                        span,
                    );

                    self.lookup_expression.insert(
                        id,
                        LookupExpression {
                            handle,
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::CompositeExtract => {
                    inst.expect_at_least(4)?;

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let base_id = self.next()?;
                    log::trace!("\t\t\tlooking up expr {base_id:?}");
                    let mut lexp = self.lookup_expression.lookup(base_id)?.clone();
                    lexp.handle = get_expr_handle!(base_id, &lexp);
                    for _ in 4..inst.wc {
                        let index = self.next()?;
                        log::trace!("\t\t\tlooking up type {:?}", lexp.type_id);
                        let type_lookup = self.lookup_type.lookup(lexp.type_id)?;
                        let type_id = match ctx.module.types[type_lookup.handle].inner {
                            crate::TypeInner::Struct { .. } => {
                                self.lookup_member
                                    .get(&(type_lookup.handle, index))
                                    .ok_or(Error::InvalidAccessType(lexp.type_id))?
                                    .type_id
                            }
                            crate::TypeInner::Array { .. }
                            | crate::TypeInner::Vector { .. }
                            | crate::TypeInner::Matrix { .. } => type_lookup
                                .base_id
                                .ok_or(Error::InvalidAccessType(lexp.type_id))?,
                            ref other => {
                                log::warn!("composite type {other:?}");
                                return Err(Error::UnsupportedType(type_lookup.handle));
                            }
                        };
                        lexp = LookupExpression {
                            handle: ctx.expressions.append(
                                crate::Expression::AccessIndex {
                                    base: lexp.handle,
                                    index,
                                },
                                span,
                            ),
                            type_id,
                            block_id,
                        };
                    }

                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: lexp.handle,
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::CompositeInsert => {
                    inst.expect_at_least(5)?;

                    let result_type_id = self.next()?;
                    let id = self.next()?;
                    let object_id = self.next()?;
                    let composite_id = self.next()?;
                    let mut selections = Vec::with_capacity(inst.wc as usize - 5);
                    for _ in 5..inst.wc {
                        selections.push(self.next()?);
                    }

                    let object_lexp = self.lookup_expression.lookup(object_id)?.clone();
                    let object_handle = get_expr_handle!(object_id, &object_lexp);
                    let root_lexp = self.lookup_expression.lookup(composite_id)?.clone();
                    let root_handle = get_expr_handle!(composite_id, &root_lexp);
                    let handle = self.insert_composite(
                        root_handle,
                        result_type_id,
                        object_handle,
                        &selections,
                        &ctx.module.types,
                        ctx.expressions,
                        span,
                    )?;

                    self.lookup_expression.insert(
                        id,
                        LookupExpression {
                            handle,
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::CompositeConstruct => {
                    inst.expect_at_least(3)?;

                    let result_type_id = self.next()?;
                    let id = self.next()?;
                    let mut components = Vec::with_capacity(inst.wc as usize - 2);
                    for _ in 3..inst.wc {
                        let comp_id = self.next()?;
                        log::trace!("\t\t\tlooking up expr {comp_id:?}");
                        let lexp = self.lookup_expression.lookup(comp_id)?;
                        let handle = get_expr_handle!(comp_id, lexp);
                        components.push(handle);
                    }
                    let ty = self.lookup_type.lookup(result_type_id)?.handle;
                    let first = components[0];
                    let expr = match ctx.module.types[ty].inner {
                        // this is an optimization to detect the splat
                        crate::TypeInner::Vector { size, .. }
                            if components.len() == size as usize
                                && components[1..].iter().all(|&c| c == first) =>
                        {
                            crate::Expression::Splat { size, value: first }
                        }
                        _ => crate::Expression::Compose { ty, components },
                    };
                    self.lookup_expression.insert(
                        id,
                        LookupExpression {
                            handle: ctx.expressions.append(expr, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::Load => {
                    inst.expect_at_least(4)?;

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let pointer_id = self.next()?;
                    if inst.wc != 4 {
                        inst.expect(5)?;
                        let _memory_access = self.next()?;
                    }

                    let base_lexp = self.lookup_expression.lookup(pointer_id)?;
                    let base_handle = get_expr_handle!(pointer_id, base_lexp);
                    let type_lookup = self.lookup_type.lookup(base_lexp.type_id)?;
                    let handle = match ctx.module.types[type_lookup.handle].inner {
                        crate::TypeInner::Image { .. } | crate::TypeInner::Sampler { .. } => {
                            base_handle
                        }
                        _ => match self.lookup_load_override.get(&pointer_id) {
                            Some(&LookupLoadOverride::Loaded(handle)) => handle,
                            //Note: we aren't handling `LookupLoadOverride::Pending` properly here
                            _ => ctx.expressions.append(
                                crate::Expression::Load {
                                    pointer: base_handle,
                                },
                                span,
                            ),
                        },
                    };

                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle,
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::Store => {
                    inst.expect_at_least(3)?;

                    let pointer_id = self.next()?;
                    let value_id = self.next()?;
                    if inst.wc != 3 {
                        inst.expect(4)?;
                        let _memory_access = self.next()?;
                    }
                    let base_expr = self.lookup_expression.lookup(pointer_id)?;
                    let base_handle = get_expr_handle!(pointer_id, base_expr);
                    let value_expr = self.lookup_expression.lookup(value_id)?;
                    let value_handle = get_expr_handle!(value_id, value_expr);

                    block.extend(emitter.finish(ctx.expressions));
                    block.push(
                        crate::Statement::Store {
                            pointer: base_handle,
                            value: value_handle,
                        },
                        span,
                    );
                    emitter.start(ctx.expressions);
                }
                // Arithmetic Instructions +, -, *, /, %
                Op::SNegate | Op::FNegate => {
                    inst.expect(4)?;
                    self.parse_expr_unary_op_sign_adjusted(
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                        crate::UnaryOperator::Negate,
                    )?;
                }
                Op::IAdd
                | Op::ISub
                | Op::IMul
                | Op::BitwiseOr
                | Op::BitwiseXor
                | Op::BitwiseAnd
                | Op::SDiv
                | Op::SRem => {
                    inst.expect(5)?;
                    let operator = map_binary_operator(inst.op)?;
                    self.parse_expr_binary_op_sign_adjusted(
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                        operator,
                        SignAnchor::Result,
                    )?;
                }
                Op::IEqual | Op::INotEqual => {
                    inst.expect(5)?;
                    let operator = map_binary_operator(inst.op)?;
                    self.parse_expr_binary_op_sign_adjusted(
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                        operator,
                        SignAnchor::Operand,
                    )?;
                }
                Op::FAdd => {
                    inst.expect(5)?;
                    parse_expr_op!(crate::BinaryOperator::Add, BINARY)?;
                }
                Op::FSub => {
                    inst.expect(5)?;
                    parse_expr_op!(crate::BinaryOperator::Subtract, BINARY)?;
                }
                Op::FMul => {
                    inst.expect(5)?;
                    parse_expr_op!(crate::BinaryOperator::Multiply, BINARY)?;
                }
                Op::UDiv | Op::FDiv => {
                    inst.expect(5)?;
                    parse_expr_op!(crate::BinaryOperator::Divide, BINARY)?;
                }
                Op::UMod | Op::FRem => {
                    inst.expect(5)?;
                    parse_expr_op!(crate::BinaryOperator::Modulo, BINARY)?;
                }
                Op::SMod => {
                    inst.expect(5)?;

                    // x - y * int(floor(float(x) / float(y)))

                    let start = self.data_offset;
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let p1_id = self.next()?;
                    let p2_id = self.next()?;
                    let span = self.span_from_with_op(start);

                    let p1_lexp = self.lookup_expression.lookup(p1_id)?;
                    let left = self.get_expr_handle(
                        p1_id,
                        p1_lexp,
                        ctx,
                        &mut emitter,
                        &mut block,
                        body_idx,
                    );
                    let p2_lexp = self.lookup_expression.lookup(p2_id)?;
                    let right = self.get_expr_handle(
                        p2_id,
                        p2_lexp,
                        ctx,
                        &mut emitter,
                        &mut block,
                        body_idx,
                    );

                    let result_ty = self.lookup_type.lookup(result_type_id)?;
                    let inner = &ctx.module.types[result_ty.handle].inner;
                    let kind = inner.scalar_kind().unwrap();
                    let size = inner.size(ctx.gctx()) as u8;

                    let left_cast = ctx.expressions.append(
                        crate::Expression::As {
                            expr: left,
                            kind: crate::ScalarKind::Float,
                            convert: Some(size),
                        },
                        span,
                    );
                    let right_cast = ctx.expressions.append(
                        crate::Expression::As {
                            expr: right,
                            kind: crate::ScalarKind::Float,
                            convert: Some(size),
                        },
                        span,
                    );
                    let div = ctx.expressions.append(
                        crate::Expression::Binary {
                            op: crate::BinaryOperator::Divide,
                            left: left_cast,
                            right: right_cast,
                        },
                        span,
                    );
                    let floor = ctx.expressions.append(
                        crate::Expression::Math {
                            fun: crate::MathFunction::Floor,
                            arg: div,
                            arg1: None,
                            arg2: None,
                            arg3: None,
                        },
                        span,
                    );
                    let cast = ctx.expressions.append(
                        crate::Expression::As {
                            expr: floor,
                            kind,
                            convert: Some(size),
                        },
                        span,
                    );
                    let mult = ctx.expressions.append(
                        crate::Expression::Binary {
                            op: crate::BinaryOperator::Multiply,
                            left: cast,
                            right,
                        },
                        span,
                    );
                    let sub = ctx.expressions.append(
                        crate::Expression::Binary {
                            op: crate::BinaryOperator::Subtract,
                            left,
                            right: mult,
                        },
                        span,
                    );
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: sub,
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::FMod => {
                    inst.expect(5)?;

                    // x - y * floor(x / y)

                    let start = self.data_offset;
                    let span = self.span_from_with_op(start);

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let p1_id = self.next()?;
                    let p2_id = self.next()?;

                    let p1_lexp = self.lookup_expression.lookup(p1_id)?;
                    let left = self.get_expr_handle(
                        p1_id,
                        p1_lexp,
                        ctx,
                        &mut emitter,
                        &mut block,
                        body_idx,
                    );
                    let p2_lexp = self.lookup_expression.lookup(p2_id)?;
                    let right = self.get_expr_handle(
                        p2_id,
                        p2_lexp,
                        ctx,
                        &mut emitter,
                        &mut block,
                        body_idx,
                    );

                    let div = ctx.expressions.append(
                        crate::Expression::Binary {
                            op: crate::BinaryOperator::Divide,
                            left,
                            right,
                        },
                        span,
                    );
                    let floor = ctx.expressions.append(
                        crate::Expression::Math {
                            fun: crate::MathFunction::Floor,
                            arg: div,
                            arg1: None,
                            arg2: None,
                            arg3: None,
                        },
                        span,
                    );
                    let mult = ctx.expressions.append(
                        crate::Expression::Binary {
                            op: crate::BinaryOperator::Multiply,
                            left: floor,
                            right,
                        },
                        span,
                    );
                    let sub = ctx.expressions.append(
                        crate::Expression::Binary {
                            op: crate::BinaryOperator::Subtract,
                            left,
                            right: mult,
                        },
                        span,
                    );
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: sub,
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::VectorTimesScalar
                | Op::VectorTimesMatrix
                | Op::MatrixTimesScalar
                | Op::MatrixTimesVector
                | Op::MatrixTimesMatrix => {
                    inst.expect(5)?;
                    parse_expr_op!(crate::BinaryOperator::Multiply, BINARY)?;
                }
                Op::Transpose => {
                    inst.expect(4)?;

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let matrix_id = self.next()?;
                    let matrix_lexp = self.lookup_expression.lookup(matrix_id)?;
                    let matrix_handle = get_expr_handle!(matrix_id, matrix_lexp);
                    let expr = crate::Expression::Math {
                        fun: crate::MathFunction::Transpose,
                        arg: matrix_handle,
                        arg1: None,
                        arg2: None,
                        arg3: None,
                    };
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx.expressions.append(expr, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::Dot => {
                    inst.expect(5)?;

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let left_id = self.next()?;
                    let right_id = self.next()?;
                    let left_lexp = self.lookup_expression.lookup(left_id)?;
                    let left_handle = get_expr_handle!(left_id, left_lexp);
                    let right_lexp = self.lookup_expression.lookup(right_id)?;
                    let right_handle = get_expr_handle!(right_id, right_lexp);
                    let expr = crate::Expression::Math {
                        fun: crate::MathFunction::Dot,
                        arg: left_handle,
                        arg1: Some(right_handle),
                        arg2: None,
                        arg3: None,
                    };
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx.expressions.append(expr, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::BitFieldInsert => {
                    inst.expect(7)?;

                    let start = self.data_offset;
                    let span = self.span_from_with_op(start);

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let base_id = self.next()?;
                    let insert_id = self.next()?;
                    let offset_id = self.next()?;
                    let count_id = self.next()?;
                    let base_lexp = self.lookup_expression.lookup(base_id)?;
                    let base_handle = get_expr_handle!(base_id, base_lexp);
                    let insert_lexp = self.lookup_expression.lookup(insert_id)?;
                    let insert_handle = get_expr_handle!(insert_id, insert_lexp);
                    let offset_lexp = self.lookup_expression.lookup(offset_id)?;
                    let offset_handle = get_expr_handle!(offset_id, offset_lexp);
                    let offset_lookup_ty = self.lookup_type.lookup(offset_lexp.type_id)?;
                    let count_lexp = self.lookup_expression.lookup(count_id)?;
                    let count_handle = get_expr_handle!(count_id, count_lexp);
                    let count_lookup_ty = self.lookup_type.lookup(count_lexp.type_id)?;

                    let offset_kind = ctx.module.types[offset_lookup_ty.handle]
                        .inner
                        .scalar_kind()
                        .unwrap();
                    let count_kind = ctx.module.types[count_lookup_ty.handle]
                        .inner
                        .scalar_kind()
                        .unwrap();

                    let offset_cast_handle = if offset_kind != crate::ScalarKind::Uint {
                        ctx.expressions.append(
                            crate::Expression::As {
                                expr: offset_handle,
                                kind: crate::ScalarKind::Uint,
                                convert: None,
                            },
                            span,
                        )
                    } else {
                        offset_handle
                    };

                    let count_cast_handle = if count_kind != crate::ScalarKind::Uint {
                        ctx.expressions.append(
                            crate::Expression::As {
                                expr: count_handle,
                                kind: crate::ScalarKind::Uint,
                                convert: None,
                            },
                            span,
                        )
                    } else {
                        count_handle
                    };

                    let expr = crate::Expression::Math {
                        fun: crate::MathFunction::InsertBits,
                        arg: base_handle,
                        arg1: Some(insert_handle),
                        arg2: Some(offset_cast_handle),
                        arg3: Some(count_cast_handle),
                    };
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx.expressions.append(expr, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::BitFieldSExtract | Op::BitFieldUExtract => {
                    inst.expect(6)?;

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let base_id = self.next()?;
                    let offset_id = self.next()?;
                    let count_id = self.next()?;
                    let base_lexp = self.lookup_expression.lookup(base_id)?;
                    let base_handle = get_expr_handle!(base_id, base_lexp);
                    let offset_lexp = self.lookup_expression.lookup(offset_id)?;
                    let offset_handle = get_expr_handle!(offset_id, offset_lexp);
                    let offset_lookup_ty = self.lookup_type.lookup(offset_lexp.type_id)?;
                    let count_lexp = self.lookup_expression.lookup(count_id)?;
                    let count_handle = get_expr_handle!(count_id, count_lexp);
                    let count_lookup_ty = self.lookup_type.lookup(count_lexp.type_id)?;

                    let offset_kind = ctx.module.types[offset_lookup_ty.handle]
                        .inner
                        .scalar_kind()
                        .unwrap();
                    let count_kind = ctx.module.types[count_lookup_ty.handle]
                        .inner
                        .scalar_kind()
                        .unwrap();

                    let offset_cast_handle = if offset_kind != crate::ScalarKind::Uint {
                        ctx.expressions.append(
                            crate::Expression::As {
                                expr: offset_handle,
                                kind: crate::ScalarKind::Uint,
                                convert: None,
                            },
                            span,
                        )
                    } else {
                        offset_handle
                    };

                    let count_cast_handle = if count_kind != crate::ScalarKind::Uint {
                        ctx.expressions.append(
                            crate::Expression::As {
                                expr: count_handle,
                                kind: crate::ScalarKind::Uint,
                                convert: None,
                            },
                            span,
                        )
                    } else {
                        count_handle
                    };

                    let expr = crate::Expression::Math {
                        fun: crate::MathFunction::ExtractBits,
                        arg: base_handle,
                        arg1: Some(offset_cast_handle),
                        arg2: Some(count_cast_handle),
                        arg3: None,
                    };
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx.expressions.append(expr, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::BitReverse | Op::BitCount => {
                    inst.expect(4)?;

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let base_id = self.next()?;
                    let base_lexp = self.lookup_expression.lookup(base_id)?;
                    let base_handle = get_expr_handle!(base_id, base_lexp);
                    let expr = crate::Expression::Math {
                        fun: match inst.op {
                            Op::BitReverse => crate::MathFunction::ReverseBits,
                            Op::BitCount => crate::MathFunction::CountOneBits,
                            _ => unreachable!(),
                        },
                        arg: base_handle,
                        arg1: None,
                        arg2: None,
                        arg3: None,
                    };
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx.expressions.append(expr, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::OuterProduct => {
                    inst.expect(5)?;

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let left_id = self.next()?;
                    let right_id = self.next()?;
                    let left_lexp = self.lookup_expression.lookup(left_id)?;
                    let left_handle = get_expr_handle!(left_id, left_lexp);
                    let right_lexp = self.lookup_expression.lookup(right_id)?;
                    let right_handle = get_expr_handle!(right_id, right_lexp);
                    let expr = crate::Expression::Math {
                        fun: crate::MathFunction::Outer,
                        arg: left_handle,
                        arg1: Some(right_handle),
                        arg2: None,
                        arg3: None,
                    };
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx.expressions.append(expr, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                // Bitwise instructions
                Op::Not => {
                    inst.expect(4)?;
                    self.parse_expr_unary_op_sign_adjusted(
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                        crate::UnaryOperator::BitwiseNot,
                    )?;
                }
                Op::ShiftRightLogical => {
                    inst.expect(5)?;
                    //TODO: convert input and result to unsigned
                    parse_expr_op!(crate::BinaryOperator::ShiftRight, SHIFT)?;
                }
                Op::ShiftRightArithmetic => {
                    inst.expect(5)?;
                    //TODO: convert input and result to signed
                    parse_expr_op!(crate::BinaryOperator::ShiftRight, SHIFT)?;
                }
                Op::ShiftLeftLogical => {
                    inst.expect(5)?;
                    parse_expr_op!(crate::BinaryOperator::ShiftLeft, SHIFT)?;
                }
                // Sampling
                Op::Image => {
                    inst.expect(4)?;
                    self.parse_image_uncouple(block_id)?;
                }
                Op::SampledImage => {
                    inst.expect(5)?;
                    self.parse_image_couple()?;
                }
                Op::ImageWrite => {
                    let extra = inst.expect_at_least(4)?;
                    let stmt =
                        self.parse_image_write(extra, ctx, &mut emitter, &mut block, body_idx)?;
                    block.extend(emitter.finish(ctx.expressions));
                    block.push(stmt, span);
                    emitter.start(ctx.expressions);
                }
                Op::ImageFetch | Op::ImageRead => {
                    let extra = inst.expect_at_least(5)?;
                    self.parse_image_load(
                        extra,
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                    )?;
                }
                Op::ImageSampleImplicitLod | Op::ImageSampleExplicitLod => {
                    let extra = inst.expect_at_least(5)?;
                    let options = image::SamplingOptions {
                        compare: false,
                        project: false,
                        gather: false,
                    };
                    self.parse_image_sample(
                        extra,
                        options,
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                    )?;
                }
                Op::ImageSampleProjImplicitLod | Op::ImageSampleProjExplicitLod => {
                    let extra = inst.expect_at_least(5)?;
                    let options = image::SamplingOptions {
                        compare: false,
                        project: true,
                        gather: false,
                    };
                    self.parse_image_sample(
                        extra,
                        options,
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                    )?;
                }
                Op::ImageSampleDrefImplicitLod | Op::ImageSampleDrefExplicitLod => {
                    let extra = inst.expect_at_least(6)?;
                    let options = image::SamplingOptions {
                        compare: true,
                        project: false,
                        gather: false,
                    };
                    self.parse_image_sample(
                        extra,
                        options,
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                    )?;
                }
                Op::ImageSampleProjDrefImplicitLod | Op::ImageSampleProjDrefExplicitLod => {
                    let extra = inst.expect_at_least(6)?;
                    let options = image::SamplingOptions {
                        compare: true,
                        project: true,
                        gather: false,
                    };
                    self.parse_image_sample(
                        extra,
                        options,
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                    )?;
                }
                Op::ImageGather => {
                    let extra = inst.expect_at_least(6)?;
                    let options = image::SamplingOptions {
                        compare: false,
                        project: false,
                        gather: true,
                    };
                    self.parse_image_sample(
                        extra,
                        options,
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                    )?;
                }
                Op::ImageDrefGather => {
                    let extra = inst.expect_at_least(6)?;
                    let options = image::SamplingOptions {
                        compare: true,
                        project: false,
                        gather: true,
                    };
                    self.parse_image_sample(
                        extra,
                        options,
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                    )?;
                }
                Op::ImageQuerySize => {
                    inst.expect(4)?;
                    self.parse_image_query_size(
                        false,
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                    )?;
                }
                Op::ImageQuerySizeLod => {
                    inst.expect(5)?;
                    self.parse_image_query_size(
                        true,
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                    )?;
                }
                Op::ImageQueryLevels => {
                    inst.expect(4)?;
                    self.parse_image_query_other(crate::ImageQuery::NumLevels, ctx, block_id)?;
                }
                Op::ImageQuerySamples => {
                    inst.expect(4)?;
                    self.parse_image_query_other(crate::ImageQuery::NumSamples, ctx, block_id)?;
                }
                // other ops
                Op::Select => {
                    inst.expect(6)?;
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let condition = self.next()?;
                    let o1_id = self.next()?;
                    let o2_id = self.next()?;

                    let cond_lexp = self.lookup_expression.lookup(condition)?;
                    let cond_handle = get_expr_handle!(condition, cond_lexp);
                    let o1_lexp = self.lookup_expression.lookup(o1_id)?;
                    let o1_handle = get_expr_handle!(o1_id, o1_lexp);
                    let o2_lexp = self.lookup_expression.lookup(o2_id)?;
                    let o2_handle = get_expr_handle!(o2_id, o2_lexp);

                    let expr = crate::Expression::Select {
                        condition: cond_handle,
                        accept: o1_handle,
                        reject: o2_handle,
                    };
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx.expressions.append(expr, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::VectorShuffle => {
                    inst.expect_at_least(5)?;
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let v1_id = self.next()?;
                    let v2_id = self.next()?;

                    let v1_lexp = self.lookup_expression.lookup(v1_id)?;
                    let v1_lty = self.lookup_type.lookup(v1_lexp.type_id)?;
                    let v1_handle = get_expr_handle!(v1_id, v1_lexp);
                    let n1 = match ctx.module.types[v1_lty.handle].inner {
                        crate::TypeInner::Vector { size, .. } => size as u32,
                        _ => return Err(Error::InvalidInnerType(v1_lexp.type_id)),
                    };
                    let v2_lexp = self.lookup_expression.lookup(v2_id)?;
                    let v2_lty = self.lookup_type.lookup(v2_lexp.type_id)?;
                    let v2_handle = get_expr_handle!(v2_id, v2_lexp);
                    let n2 = match ctx.module.types[v2_lty.handle].inner {
                        crate::TypeInner::Vector { size, .. } => size as u32,
                        _ => return Err(Error::InvalidInnerType(v2_lexp.type_id)),
                    };

                    self.temp_bytes.clear();
                    let mut max_component = 0;
                    for _ in 5..inst.wc as usize {
                        let mut index = self.next()?;
                        if index == u32::MAX {
                            // treat Undefined as X
                            index = 0;
                        }
                        max_component = max_component.max(index);
                        self.temp_bytes.push(index as u8);
                    }

                    // Check for swizzle first.
                    let expr = if max_component < n1 {
                        use crate::SwizzleComponent as Sc;
                        let size = match self.temp_bytes.len() {
                            2 => crate::VectorSize::Bi,
                            3 => crate::VectorSize::Tri,
                            _ => crate::VectorSize::Quad,
                        };
                        let mut pattern = [Sc::X; 4];
                        for (pat, index) in pattern.iter_mut().zip(self.temp_bytes.drain(..)) {
                            *pat = match index {
                                0 => Sc::X,
                                1 => Sc::Y,
                                2 => Sc::Z,
                                _ => Sc::W,
                            };
                        }
                        crate::Expression::Swizzle {
                            size,
                            vector: v1_handle,
                            pattern,
                        }
                    } else {
                        // Fall back to access + compose
                        let mut components = Vec::with_capacity(self.temp_bytes.len());
                        for index in self.temp_bytes.drain(..).map(|i| i as u32) {
                            let expr = if index < n1 {
                                crate::Expression::AccessIndex {
                                    base: v1_handle,
                                    index,
                                }
                            } else if index < n1 + n2 {
                                crate::Expression::AccessIndex {
                                    base: v2_handle,
                                    index: index - n1,
                                }
                            } else {
                                return Err(Error::InvalidAccessIndex(index));
                            };
                            components.push(ctx.expressions.append(expr, span));
                        }
                        crate::Expression::Compose {
                            ty: self.lookup_type.lookup(result_type_id)?.handle,
                            components,
                        }
                    };

                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx.expressions.append(expr, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::Bitcast
                | Op::ConvertSToF
                | Op::ConvertUToF
                | Op::ConvertFToU
                | Op::ConvertFToS
                | Op::FConvert
                | Op::UConvert
                | Op::SConvert => {
                    inst.expect(4)?;
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let value_id = self.next()?;

                    let value_lexp = self.lookup_expression.lookup(value_id)?;
                    let ty_lookup = self.lookup_type.lookup(result_type_id)?;
                    let scalar = match ctx.module.types[ty_lookup.handle].inner {
                        crate::TypeInner::Scalar(scalar)
                        | crate::TypeInner::Vector { scalar, .. }
                        | crate::TypeInner::Matrix { scalar, .. } => scalar,
                        _ => return Err(Error::InvalidAsType(ty_lookup.handle)),
                    };

                    let expr = crate::Expression::As {
                        expr: get_expr_handle!(value_id, value_lexp),
                        kind: scalar.kind,
                        convert: if scalar.kind == crate::ScalarKind::Bool {
                            Some(crate::BOOL_WIDTH)
                        } else if inst.op == Op::Bitcast {
                            None
                        } else {
                            Some(scalar.width)
                        },
                    };
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx.expressions.append(expr, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::FunctionCall => {
                    inst.expect_at_least(4)?;

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let func_id = self.next()?;

                    let mut arguments = Vec::with_capacity(inst.wc as usize - 4);
                    for _ in 0..arguments.capacity() {
                        let arg_id = self.next()?;
                        let lexp = self.lookup_expression.lookup(arg_id)?;
                        arguments.push(get_expr_handle!(arg_id, lexp));
                    }

                    block.extend(emitter.finish(ctx.expressions));

                    // We just need an unique handle here, nothing more.
                    let function = self.add_call(ctx.function_id, func_id);

                    let result = if self.lookup_void_type == Some(result_type_id) {
                        None
                    } else {
                        let expr_handle = ctx
                            .expressions
                            .append(crate::Expression::CallResult(function), span);
                        self.lookup_expression.insert(
                            result_id,
                            LookupExpression {
                                handle: expr_handle,
                                type_id: result_type_id,
                                block_id,
                            },
                        );
                        Some(expr_handle)
                    };
                    block.push(
                        crate::Statement::Call {
                            function,
                            arguments,
                            result,
                        },
                        span,
                    );
                    emitter.start(ctx.expressions);
                }
                Op::ExtInst => {
                    use crate::MathFunction as Mf;
                    use spirv::GlslStd450Op as Glo;

                    let base_wc = 5;
                    inst.expect_at_least(base_wc)?;

                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let set_id = self.next()?;
                    if Some(set_id) == self.ext_non_semantic_id {
                        for _ in 0..inst.wc - 4 {
                            self.next()?;
                        }
                        continue;
                    } else if Some(set_id) != self.ext_glsl_id {
                        return Err(Error::UnsupportedExtInstSet(set_id));
                    }
                    let inst_id = self.next()?;
                    let gl_op = Glo::from_u32(inst_id).ok_or(Error::UnsupportedExtInst(inst_id))?;

                    let fun = match gl_op {
                        Glo::Round => Mf::Round,
                        Glo::RoundEven => Mf::Round,
                        Glo::Trunc => Mf::Trunc,
                        Glo::FAbs | Glo::SAbs => Mf::Abs,
                        Glo::FSign | Glo::SSign => Mf::Sign,
                        Glo::Floor => Mf::Floor,
                        Glo::Ceil => Mf::Ceil,
                        Glo::Fract => Mf::Fract,
                        Glo::Sin => Mf::Sin,
                        Glo::Cos => Mf::Cos,
                        Glo::Tan => Mf::Tan,
                        Glo::Asin => Mf::Asin,
                        Glo::Acos => Mf::Acos,
                        Glo::Atan => Mf::Atan,
                        Glo::Sinh => Mf::Sinh,
                        Glo::Cosh => Mf::Cosh,
                        Glo::Tanh => Mf::Tanh,
                        Glo::Atan2 => Mf::Atan2,
                        Glo::Asinh => Mf::Asinh,
                        Glo::Acosh => Mf::Acosh,
                        Glo::Atanh => Mf::Atanh,
                        Glo::Radians => Mf::Radians,
                        Glo::Degrees => Mf::Degrees,
                        Glo::Pow => Mf::Pow,
                        Glo::Exp => Mf::Exp,
                        Glo::Log => Mf::Log,
                        Glo::Exp2 => Mf::Exp2,
                        Glo::Log2 => Mf::Log2,
                        Glo::Sqrt => Mf::Sqrt,
                        Glo::InverseSqrt => Mf::InverseSqrt,
                        Glo::MatrixInverse => Mf::Inverse,
                        Glo::Determinant => Mf::Determinant,
                        Glo::ModfStruct => Mf::Modf,
                        Glo::FMin | Glo::UMin | Glo::SMin | Glo::NMin => Mf::Min,
                        Glo::FMax | Glo::UMax | Glo::SMax | Glo::NMax => Mf::Max,
                        Glo::FClamp | Glo::UClamp | Glo::SClamp | Glo::NClamp => Mf::Clamp,
                        Glo::FMix => Mf::Mix,
                        Glo::Step => Mf::Step,
                        Glo::SmoothStep => Mf::SmoothStep,
                        Glo::Fma => Mf::Fma,
                        Glo::FrexpStruct => Mf::Frexp,
                        Glo::Ldexp => Mf::Ldexp,
                        Glo::Length => Mf::Length,
                        Glo::Distance => Mf::Distance,
                        Glo::Cross => Mf::Cross,
                        Glo::Normalize => Mf::Normalize,
                        Glo::FaceForward => Mf::FaceForward,
                        Glo::Reflect => Mf::Reflect,
                        Glo::Refract => Mf::Refract,
                        Glo::PackUnorm4x8 => Mf::Pack4x8unorm,
                        Glo::PackSnorm4x8 => Mf::Pack4x8snorm,
                        Glo::PackHalf2x16 => Mf::Pack2x16float,
                        Glo::PackUnorm2x16 => Mf::Pack2x16unorm,
                        Glo::PackSnorm2x16 => Mf::Pack2x16snorm,
                        Glo::UnpackUnorm4x8 => Mf::Unpack4x8unorm,
                        Glo::UnpackSnorm4x8 => Mf::Unpack4x8snorm,
                        Glo::UnpackHalf2x16 => Mf::Unpack2x16float,
                        Glo::UnpackUnorm2x16 => Mf::Unpack2x16unorm,
                        Glo::UnpackSnorm2x16 => Mf::Unpack2x16snorm,
                        Glo::FindILsb => Mf::FirstTrailingBit,
                        Glo::FindUMsb | Glo::FindSMsb => Mf::FirstLeadingBit,
                        // TODO: https://github.com/gfx-rs/naga/issues/2526
                        Glo::Modf | Glo::Frexp => return Err(Error::UnsupportedExtInst(inst_id)),
                        Glo::IMix
                        | Glo::PackDouble2x32
                        | Glo::UnpackDouble2x32
                        | Glo::InterpolateAtCentroid
                        | Glo::InterpolateAtSample
                        | Glo::InterpolateAtOffset => {
                            return Err(Error::UnsupportedExtInst(inst_id))
                        }
                    };

                    let arg_count = fun.argument_count();
                    inst.expect(base_wc + arg_count as u16)?;
                    let arg = {
                        let arg_id = self.next()?;
                        let lexp = self.lookup_expression.lookup(arg_id)?;
                        get_expr_handle!(arg_id, lexp)
                    };
                    let arg1 = if arg_count > 1 {
                        let arg_id = self.next()?;
                        let lexp = self.lookup_expression.lookup(arg_id)?;
                        Some(get_expr_handle!(arg_id, lexp))
                    } else {
                        None
                    };
                    let arg2 = if arg_count > 2 {
                        let arg_id = self.next()?;
                        let lexp = self.lookup_expression.lookup(arg_id)?;
                        Some(get_expr_handle!(arg_id, lexp))
                    } else {
                        None
                    };
                    let arg3 = if arg_count > 3 {
                        let arg_id = self.next()?;
                        let lexp = self.lookup_expression.lookup(arg_id)?;
                        Some(get_expr_handle!(arg_id, lexp))
                    } else {
                        None
                    };

                    let expr = crate::Expression::Math {
                        fun,
                        arg,
                        arg1,
                        arg2,
                        arg3,
                    };
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx.expressions.append(expr, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                // Relational and Logical Instructions
                Op::LogicalNot => {
                    inst.expect(4)?;
                    parse_expr_op!(crate::UnaryOperator::LogicalNot, UNARY)?;
                }
                Op::LogicalOr => {
                    inst.expect(5)?;
                    parse_expr_op!(crate::BinaryOperator::LogicalOr, BINARY)?;
                }
                Op::LogicalAnd => {
                    inst.expect(5)?;
                    parse_expr_op!(crate::BinaryOperator::LogicalAnd, BINARY)?;
                }
                Op::SGreaterThan | Op::SGreaterThanEqual | Op::SLessThan | Op::SLessThanEqual => {
                    inst.expect(5)?;
                    self.parse_expr_int_comparison(
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                        map_binary_operator(inst.op)?,
                        crate::ScalarKind::Sint,
                    )?;
                }
                Op::UGreaterThan | Op::UGreaterThanEqual | Op::ULessThan | Op::ULessThanEqual => {
                    inst.expect(5)?;
                    self.parse_expr_int_comparison(
                        ctx,
                        &mut emitter,
                        &mut block,
                        block_id,
                        body_idx,
                        map_binary_operator(inst.op)?,
                        crate::ScalarKind::Uint,
                    )?;
                }
                Op::FOrdEqual
                | Op::FUnordEqual
                | Op::FOrdNotEqual
                | Op::FUnordNotEqual
                | Op::FOrdLessThan
                | Op::FUnordLessThan
                | Op::FOrdGreaterThan
                | Op::FUnordGreaterThan
                | Op::FOrdLessThanEqual
                | Op::FUnordLessThanEqual
                | Op::FOrdGreaterThanEqual
                | Op::FUnordGreaterThanEqual
                | Op::LogicalEqual
                | Op::LogicalNotEqual => {
                    inst.expect(5)?;
                    let operator = map_binary_operator(inst.op)?;
                    parse_expr_op!(operator, BINARY)?;
                }
                Op::Any | Op::All | Op::IsNan | Op::IsInf | Op::IsFinite | Op::IsNormal => {
                    inst.expect(4)?;
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let arg_id = self.next()?;

                    let arg_lexp = self.lookup_expression.lookup(arg_id)?;
                    let arg_handle = get_expr_handle!(arg_id, arg_lexp);

                    let expr = crate::Expression::Relational {
                        fun: map_relational_fun(inst.op)?,
                        argument: arg_handle,
                    };
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: ctx.expressions.append(expr, span),
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::Kill => {
                    inst.expect(1)?;
                    break Some(crate::Statement::Kill);
                }
                Op::Unreachable => {
                    inst.expect(1)?;
                    break None;
                }
                Op::Return => {
                    inst.expect(1)?;
                    break Some(crate::Statement::Return { value: None });
                }
                Op::ReturnValue => {
                    inst.expect(2)?;
                    let value_id = self.next()?;
                    let value_lexp = self.lookup_expression.lookup(value_id)?;
                    let value_handle = get_expr_handle!(value_id, value_lexp);
                    break Some(crate::Statement::Return {
                        value: Some(value_handle),
                    });
                }
                Op::Branch => {
                    inst.expect(2)?;
                    let target_id = self.next()?;

                    // If this is a branch to a merge or continue block, then
                    // that ends the current body.
                    //
                    // Why can we count on finding an entry here when it's
                    // needed? SPIR-V requires dominators to appear before
                    // blocks they dominate, so we will have visited a
                    // structured control construct's header block before
                    // anything that could exit it.
                    if let Some(info) = ctx.mergers.get(&target_id) {
                        block.extend(emitter.finish(ctx.expressions));
                        ctx.blocks.insert(block_id, block);
                        let body = &mut ctx.bodies[body_idx];
                        body.data.push(BodyFragment::BlockId(block_id));

                        merger(body, info);

                        return Ok(());
                    }

                    // If `target_id` has no entry in `ctx.body_for_label`, then
                    // this must be the only branch to it:
                    //
                    // - We've already established that it's not anybody's merge
                    //   block.
                    //
                    // - It can't be a switch case. Only switch header blocks
                    //   and other switch cases can branch to a switch case.
                    //   Switch header blocks must dominate all their cases, so
                    //   they must appear in the file before them, and when we
                    //   see `Op::Switch` we populate `ctx.body_for_label` for
                    //   every switch case.
                    //
                    // Thus, `target_id` must be a simple extension of the
                    // current block, which we dominate, so we know we'll
                    // encounter it later in the file.
                    ctx.body_for_label.entry(target_id).or_insert(body_idx);

                    break None;
                }
                Op::BranchConditional => {
                    inst.expect_at_least(4)?;

                    let condition = {
                        let condition_id = self.next()?;
                        let lexp = self.lookup_expression.lookup(condition_id)?;
                        get_expr_handle!(condition_id, lexp)
                    };

                    // HACK(eddyb) Naga doesn't seem to have this helper,
                    // so it's declared on the fly here for convenience.
                    #[derive(Copy, Clone)]
                    struct BranchTarget {
                        label_id: spirv::Word,
                        merge_info: Option<MergeBlockInformation>,
                    }
                    let branch_target = |label_id| BranchTarget {
                        label_id,
                        merge_info: ctx.mergers.get(&label_id).copied(),
                    };

                    let true_target = branch_target(self.next()?);
                    let false_target = branch_target(self.next()?);

                    // Consume branch weights
                    for _ in 4..inst.wc {
                        let _ = self.next()?;
                    }

                    // Handle `OpBranchConditional`s used at the end of a loop
                    // body's "continuing" section as a "conditional backedge",
                    // i.e. a `do`-`while` condition, or `break if` in WGSL.

                    // HACK(eddyb) this has to go to the parent *twice*, because
                    // `OpLoopMerge` left the "continuing" section nested in the
                    // loop body in terms of `parent`, but not `BodyFragment`.
                    let parent_body_idx = ctx.bodies[body_idx].parent;
                    let parent_parent_body_idx = ctx.bodies[parent_body_idx].parent;
                    match ctx.bodies[parent_parent_body_idx].data[..] {
                        // The `OpLoopMerge`'s `continuing` block and the loop's
                        // backedge block may not be the same, but they'll both
                        // belong to the same body.
                        [.., BodyFragment::Loop {
                            body: loop_body_idx,
                            continuing: loop_continuing_idx,
                            break_if: ref mut break_if_slot @ None,
                        }] if body_idx == loop_continuing_idx => {
                            // Try both orderings of break-vs-backedge, because
                            // SPIR-V is symmetrical here, unlike WGSL `break if`.
                            let break_if_cond = [true, false].into_iter().find_map(|true_breaks| {
                                let (break_candidate, backedge_candidate) = if true_breaks {
                                    (true_target, false_target)
                                } else {
                                    (false_target, true_target)
                                };

                                if break_candidate.merge_info
                                    != Some(MergeBlockInformation::LoopMerge)
                                {
                                    return None;
                                }

                                // HACK(eddyb) since Naga doesn't explicitly track
                                // backedges, this is checking for the outcome of
                                // `OpLoopMerge` below (even if it looks weird).
                                let backedge_candidate_is_backedge =
                                    backedge_candidate.merge_info.is_none()
                                        && ctx.body_for_label.get(&backedge_candidate.label_id)
                                            == Some(&loop_body_idx);
                                if !backedge_candidate_is_backedge {
                                    return None;
                                }

                                Some(if true_breaks {
                                    condition
                                } else {
                                    ctx.expressions.append(
                                        crate::Expression::Unary {
                                            op: crate::UnaryOperator::LogicalNot,
                                            expr: condition,
                                        },
                                        span,
                                    )
                                })
                            });

                            if let Some(break_if_cond) = break_if_cond {
                                *break_if_slot = Some(break_if_cond);

                                // This `OpBranchConditional` ends the "continuing"
                                // section of the loop body as normal, with the
                                // `break if` condition having been stashed above.
                                break None;
                            }
                        }
                        _ => {}
                    }

                    block.extend(emitter.finish(ctx.expressions));
                    ctx.blocks.insert(block_id, block);
                    let body = &mut ctx.bodies[body_idx];
                    body.data.push(BodyFragment::BlockId(block_id));

                    let same_target = true_target.label_id == false_target.label_id;

                    // Start a body block for the `accept` branch.
                    let accept = ctx.bodies.len();
                    let mut accept_block = Body::with_parent(body_idx);

                    // If the `OpBranchConditional` target is somebody else's
                    // merge or continue block, then put a `Break` or `Continue`
                    // statement in this new body block.
                    if let Some(info) = true_target.merge_info {
                        merger(
                            match same_target {
                                true => &mut ctx.bodies[body_idx],
                                false => &mut accept_block,
                            },
                            &info,
                        )
                    } else {
                        // Note the body index for the block we're branching to.
                        let prev = ctx.body_for_label.insert(
                            true_target.label_id,
                            match same_target {
                                true => body_idx,
                                false => accept,
                            },
                        );
                        debug_assert!(prev.is_none());
                    }

                    if same_target {
                        return Ok(());
                    }

                    ctx.bodies.push(accept_block);

                    // Handle the `reject` branch just like the `accept` block.
                    let reject = ctx.bodies.len();
                    let mut reject_block = Body::with_parent(body_idx);

                    if let Some(info) = false_target.merge_info {
                        merger(&mut reject_block, &info)
                    } else {
                        let prev = ctx.body_for_label.insert(false_target.label_id, reject);
                        debug_assert!(prev.is_none());
                    }

                    ctx.bodies.push(reject_block);

                    let body = &mut ctx.bodies[body_idx];
                    body.data.push(BodyFragment::If {
                        condition,
                        accept,
                        reject,
                    });

                    return Ok(());
                }
                Op::Switch => {
                    inst.expect_at_least(3)?;
                    let selector = self.next()?;
                    let default_id = self.next()?;

                    // If the previous instruction was a `OpSelectionMerge` then we must
                    // promote the `MergeBlockInformation` to a `SwitchMerge`
                    if let Some(merge) = selection_merge_block {
                        ctx.mergers
                            .insert(merge, MergeBlockInformation::SwitchMerge);
                    }

                    let default = ctx.bodies.len();
                    ctx.bodies.push(Body::with_parent(body_idx));
                    ctx.body_for_label.entry(default_id).or_insert(default);

                    let selector_lexp = &self.lookup_expression[&selector];
                    let selector_lty = self.lookup_type.lookup(selector_lexp.type_id)?;
                    let selector_handle = get_expr_handle!(selector, selector_lexp);
                    let selector = match ctx.module.types[selector_lty.handle].inner {
                        crate::TypeInner::Scalar(crate::Scalar {
                            kind: crate::ScalarKind::Uint,
                            width: _,
                        }) => {
                            // IR expects a signed integer, so do a bitcast
                            ctx.expressions.append(
                                crate::Expression::As {
                                    kind: crate::ScalarKind::Sint,
                                    expr: selector_handle,
                                    convert: None,
                                },
                                span,
                            )
                        }
                        crate::TypeInner::Scalar(crate::Scalar {
                            kind: crate::ScalarKind::Sint,
                            width: _,
                        }) => selector_handle,
                        ref other => unimplemented!("Unexpected selector {:?}", other),
                    };

                    // Clear past switch cases to prevent them from entering this one
                    self.switch_cases.clear();

                    for _ in 0..(inst.wc - 3) / 2 {
                        let literal = self.next()?;
                        let target = self.next()?;

                        let case_body_idx = ctx.bodies.len();

                        // Check if any previous case already used this target block id, if so
                        // group them together to reorder them later so that no weird
                        // fallthrough cases happen.
                        if let Some(&mut (_, ref mut literals)) = self.switch_cases.get_mut(&target)
                        {
                            literals.push(literal as i32);
                            continue;
                        }

                        let mut body = Body::with_parent(body_idx);

                        if let Some(info) = ctx.mergers.get(&target) {
                            merger(&mut body, info);
                        }

                        ctx.bodies.push(body);
                        ctx.body_for_label.entry(target).or_insert(case_body_idx);

                        // Register this target block id as already having been processed and
                        // the respective body index assigned and the first case value
                        self.switch_cases
                            .insert(target, (case_body_idx, vec![literal as i32]));
                    }

                    // Loop through the collected target blocks creating a new case for each
                    // literal pointing to it, only one case will have the true body and all the
                    // others will be empty fallthrough so that they all execute the same body
                    // without duplicating code.
                    //
                    // Since `switch_cases` is an indexmap the order of insertion is preserved
                    // this is needed because spir-v defines fallthrough order in the switch
                    // instruction.
                    let mut cases = Vec::with_capacity((inst.wc as usize - 3) / 2);
                    for &(case_body_idx, ref literals) in self.switch_cases.values() {
                        let value = literals[0];

                        for &literal in literals.iter().skip(1) {
                            let empty_body_idx = ctx.bodies.len();
                            let body = Body::with_parent(body_idx);

                            ctx.bodies.push(body);

                            cases.push((literal, empty_body_idx));
                        }

                        cases.push((value, case_body_idx));
                    }

                    block.extend(emitter.finish(ctx.expressions));

                    let body = &mut ctx.bodies[body_idx];
                    ctx.blocks.insert(block_id, block);
                    // Make sure the vector has space for at least two more allocations
                    body.data.reserve(2);
                    body.data.push(BodyFragment::BlockId(block_id));
                    body.data.push(BodyFragment::Switch {
                        selector,
                        cases,
                        default,
                    });

                    return Ok(());
                }
                Op::SelectionMerge => {
                    inst.expect(3)?;
                    let merge_block_id = self.next()?;
                    // TODO: Selection Control Mask
                    let _selection_control = self.next()?;

                    // Indicate that the merge block is a continuation of the
                    // current `Body`.
                    ctx.body_for_label.entry(merge_block_id).or_insert(body_idx);

                    // Let subsequent branches to the merge block know that
                    // they've reached the end of the selection construct.
                    ctx.mergers
                        .insert(merge_block_id, MergeBlockInformation::SelectionMerge);

                    selection_merge_block = Some(merge_block_id);
                }
                Op::LoopMerge => {
                    inst.expect_at_least(4)?;
                    let merge_block_id = self.next()?;
                    let continuing = self.next()?;

                    // TODO: Loop Control Parameters
                    for _ in 0..inst.wc - 3 {
                        self.next()?;
                    }

                    // Indicate that the merge block is a continuation of the
                    // current `Body`.
                    ctx.body_for_label.entry(merge_block_id).or_insert(body_idx);
                    // Let subsequent branches to the merge block know that
                    // they're `Break` statements.
                    ctx.mergers
                        .insert(merge_block_id, MergeBlockInformation::LoopMerge);

                    let loop_body_idx = ctx.bodies.len();
                    ctx.bodies.push(Body::with_parent(body_idx));

                    let continue_idx = ctx.bodies.len();
                    // The continue block inherits the scope of the loop body
                    ctx.bodies.push(Body::with_parent(loop_body_idx));
                    ctx.body_for_label.entry(continuing).or_insert(continue_idx);
                    // Let subsequent branches to the continue block know that
                    // they're `Continue` statements.
                    ctx.mergers
                        .insert(continuing, MergeBlockInformation::LoopContinue);

                    // The loop header always belongs to the loop body
                    ctx.body_for_label.insert(block_id, loop_body_idx);

                    let parent_body = &mut ctx.bodies[body_idx];
                    parent_body.data.push(BodyFragment::Loop {
                        body: loop_body_idx,
                        continuing: continue_idx,
                        break_if: None,
                    });
                    body_idx = loop_body_idx;
                }
                Op::DPdxCoarse => {
                    parse_expr_op!(
                        crate::DerivativeAxis::X,
                        crate::DerivativeControl::Coarse,
                        DERIVATIVE
                    )?;
                }
                Op::DPdyCoarse => {
                    parse_expr_op!(
                        crate::DerivativeAxis::Y,
                        crate::DerivativeControl::Coarse,
                        DERIVATIVE
                    )?;
                }
                Op::FwidthCoarse => {
                    parse_expr_op!(
                        crate::DerivativeAxis::Width,
                        crate::DerivativeControl::Coarse,
                        DERIVATIVE
                    )?;
                }
                Op::DPdxFine => {
                    parse_expr_op!(
                        crate::DerivativeAxis::X,
                        crate::DerivativeControl::Fine,
                        DERIVATIVE
                    )?;
                }
                Op::DPdyFine => {
                    parse_expr_op!(
                        crate::DerivativeAxis::Y,
                        crate::DerivativeControl::Fine,
                        DERIVATIVE
                    )?;
                }
                Op::FwidthFine => {
                    parse_expr_op!(
                        crate::DerivativeAxis::Width,
                        crate::DerivativeControl::Fine,
                        DERIVATIVE
                    )?;
                }
                Op::DPdx => {
                    parse_expr_op!(
                        crate::DerivativeAxis::X,
                        crate::DerivativeControl::None,
                        DERIVATIVE
                    )?;
                }
                Op::DPdy => {
                    parse_expr_op!(
                        crate::DerivativeAxis::Y,
                        crate::DerivativeControl::None,
                        DERIVATIVE
                    )?;
                }
                Op::Fwidth => {
                    parse_expr_op!(
                        crate::DerivativeAxis::Width,
                        crate::DerivativeControl::None,
                        DERIVATIVE
                    )?;
                }
                Op::ArrayLength => {
                    inst.expect(5)?;
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let structure_id = self.next()?;
                    let member_index = self.next()?;

                    // We're assuming that the validation pass, if it's run, will catch if the
                    // wrong types or parameters are supplied here.

                    let structure_ptr = self.lookup_expression.lookup(structure_id)?;
                    let structure_handle = get_expr_handle!(structure_id, structure_ptr);

                    let member_ptr = ctx.expressions.append(
                        crate::Expression::AccessIndex {
                            base: structure_handle,
                            index: member_index,
                        },
                        span,
                    );

                    let length = ctx
                        .expressions
                        .append(crate::Expression::ArrayLength(member_ptr), span);

                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: length,
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::CopyMemory => {
                    inst.expect_at_least(3)?;
                    let target_id = self.next()?;
                    let source_id = self.next()?;
                    let _memory_access = if inst.wc != 3 {
                        inst.expect(4)?;
                        spirv::MemoryAccess::from_bits(self.next()?)
                            .ok_or(Error::InvalidParameter(Op::CopyMemory))?
                    } else {
                        spirv::MemoryAccess::NONE
                    };

                    // TODO: check if the source and target types are the same?
                    let target = self.lookup_expression.lookup(target_id)?;
                    let target_handle = get_expr_handle!(target_id, target);
                    let source = self.lookup_expression.lookup(source_id)?;
                    let source_handle = get_expr_handle!(source_id, source);

                    // This operation is practically the same as loading and then storing, I think.
                    let value_expr = ctx.expressions.append(
                        crate::Expression::Load {
                            pointer: source_handle,
                        },
                        span,
                    );

                    block.extend(emitter.finish(ctx.expressions));
                    block.push(
                        crate::Statement::Store {
                            pointer: target_handle,
                            value: value_expr,
                        },
                        span,
                    );

                    emitter.start(ctx.expressions);
                }
                Op::ControlBarrier => {
                    inst.expect(4)?;
                    let exec_scope_id = self.next()?;
                    let _mem_scope_raw = self.next()?;
                    let semantics_id = self.next()?;
                    let exec_scope_const = self.lookup_constant.lookup(exec_scope_id)?;
                    let semantics_const = self.lookup_constant.lookup(semantics_id)?;

                    let exec_scope = resolve_constant(ctx.gctx(), &exec_scope_const.inner)
                        .ok_or(Error::InvalidBarrierScope(exec_scope_id))?;
                    let semantics = resolve_constant(ctx.gctx(), &semantics_const.inner)
                        .ok_or(Error::InvalidBarrierMemorySemantics(semantics_id))?;

                    if exec_scope == spirv::Scope::Workgroup as u32
                        || exec_scope == spirv::Scope::Subgroup as u32
                    {
                        let mut flags = crate::Barrier::empty();
                        flags.set(
                            crate::Barrier::STORAGE,
                            semantics & spirv::MemorySemantics::UNIFORM_MEMORY.bits() != 0,
                        );
                        flags.set(
                            crate::Barrier::WORK_GROUP,
                            semantics & (spirv::MemorySemantics::WORKGROUP_MEMORY).bits() != 0,
                        );
                        flags.set(
                            crate::Barrier::SUB_GROUP,
                            semantics & spirv::MemorySemantics::SUBGROUP_MEMORY.bits() != 0,
                        );
                        flags.set(
                            crate::Barrier::TEXTURE,
                            semantics & spirv::MemorySemantics::IMAGE_MEMORY.bits() != 0,
                        );

                        block.extend(emitter.finish(ctx.expressions));
                        block.push(crate::Statement::ControlBarrier(flags), span);
                        emitter.start(ctx.expressions);
                    } else {
                        log::warn!("Unsupported barrier execution scope: {exec_scope}");
                    }
                }
                Op::MemoryBarrier => {
                    inst.expect(3)?;
                    let mem_scope_id = self.next()?;
                    let semantics_id = self.next()?;
                    let mem_scope_const = self.lookup_constant.lookup(mem_scope_id)?;
                    let semantics_const = self.lookup_constant.lookup(semantics_id)?;

                    let mem_scope = resolve_constant(ctx.gctx(), &mem_scope_const.inner)
                        .ok_or(Error::InvalidBarrierScope(mem_scope_id))?;
                    let semantics = resolve_constant(ctx.gctx(), &semantics_const.inner)
                        .ok_or(Error::InvalidBarrierMemorySemantics(semantics_id))?;

                    let mut flags = if mem_scope == spirv::Scope::Device as u32 {
                        crate::Barrier::STORAGE
                    } else if mem_scope == spirv::Scope::Workgroup as u32 {
                        crate::Barrier::WORK_GROUP
                    } else if mem_scope == spirv::Scope::Subgroup as u32 {
                        crate::Barrier::SUB_GROUP
                    } else {
                        crate::Barrier::empty()
                    };
                    flags.set(
                        crate::Barrier::STORAGE,
                        semantics & spirv::MemorySemantics::UNIFORM_MEMORY.bits() != 0,
                    );
                    flags.set(
                        crate::Barrier::WORK_GROUP,
                        semantics & (spirv::MemorySemantics::WORKGROUP_MEMORY).bits() != 0,
                    );
                    flags.set(
                        crate::Barrier::SUB_GROUP,
                        semantics & spirv::MemorySemantics::SUBGROUP_MEMORY.bits() != 0,
                    );
                    flags.set(
                        crate::Barrier::TEXTURE,
                        semantics & spirv::MemorySemantics::IMAGE_MEMORY.bits() != 0,
                    );

                    block.extend(emitter.finish(ctx.expressions));
                    block.push(crate::Statement::MemoryBarrier(flags), span);
                    emitter.start(ctx.expressions);
                }
                Op::CopyObject => {
                    inst.expect(4)?;
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let operand_id = self.next()?;

                    let lookup = self.lookup_expression.lookup(operand_id)?;
                    let handle = get_expr_handle!(operand_id, lookup);

                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle,
                            type_id: result_type_id,
                            block_id,
                        },
                    );
                }
                Op::GroupNonUniformBallot => {
                    inst.expect(5)?;
                    block.extend(emitter.finish(ctx.expressions));
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let exec_scope_id = self.next()?;
                    let predicate_id = self.next()?;

                    let exec_scope_const = self.lookup_constant.lookup(exec_scope_id)?;
                    let _exec_scope = resolve_constant(ctx.gctx(), &exec_scope_const.inner)
                        .filter(|exec_scope| *exec_scope == spirv::Scope::Subgroup as u32)
                        .ok_or(Error::InvalidBarrierScope(exec_scope_id))?;

                    let predicate = if self
                        .lookup_constant
                        .lookup(predicate_id)
                        .ok()
                        .filter(|predicate_const| match predicate_const.inner {
                            Constant::Constant(constant) => matches!(
                                ctx.gctx().global_expressions[ctx.gctx().constants[constant].init],
                                crate::Expression::Literal(crate::Literal::Bool(true)),
                            ),
                            Constant::Override(_) => false,
                        })
                        .is_some()
                    {
                        None
                    } else {
                        let predicate_lookup = self.lookup_expression.lookup(predicate_id)?;
                        let predicate_handle = get_expr_handle!(predicate_id, predicate_lookup);
                        Some(predicate_handle)
                    };

                    let result_handle = ctx
                        .expressions
                        .append(crate::Expression::SubgroupBallotResult, span);
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: result_handle,
                            type_id: result_type_id,
                            block_id,
                        },
                    );

                    block.push(
                        crate::Statement::SubgroupBallot {
                            result: result_handle,
                            predicate,
                        },
                        span,
                    );
                    emitter.start(ctx.expressions);
                }
                Op::GroupNonUniformAll
                | Op::GroupNonUniformAny
                | Op::GroupNonUniformIAdd
                | Op::GroupNonUniformFAdd
                | Op::GroupNonUniformIMul
                | Op::GroupNonUniformFMul
                | Op::GroupNonUniformSMax
                | Op::GroupNonUniformUMax
                | Op::GroupNonUniformFMax
                | Op::GroupNonUniformSMin
                | Op::GroupNonUniformUMin
                | Op::GroupNonUniformFMin
                | Op::GroupNonUniformBitwiseAnd
                | Op::GroupNonUniformBitwiseOr
                | Op::GroupNonUniformBitwiseXor
                | Op::GroupNonUniformLogicalAnd
                | Op::GroupNonUniformLogicalOr
                | Op::GroupNonUniformLogicalXor => {
                    block.extend(emitter.finish(ctx.expressions));
                    inst.expect(
                        if matches!(inst.op, Op::GroupNonUniformAll | Op::GroupNonUniformAny) {
                            5
                        } else {
                            6
                        },
                    )?;
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let exec_scope_id = self.next()?;
                    let collective_op_id = match inst.op {
                        Op::GroupNonUniformAll | Op::GroupNonUniformAny => {
                            crate::CollectiveOperation::Reduce
                        }
                        _ => {
                            let group_op_id = self.next()?;
                            match spirv::GroupOperation::from_u32(group_op_id) {
                                Some(spirv::GroupOperation::Reduce) => {
                                    crate::CollectiveOperation::Reduce
                                }
                                Some(spirv::GroupOperation::InclusiveScan) => {
                                    crate::CollectiveOperation::InclusiveScan
                                }
                                Some(spirv::GroupOperation::ExclusiveScan) => {
                                    crate::CollectiveOperation::ExclusiveScan
                                }
                                _ => return Err(Error::UnsupportedGroupOperation(group_op_id)),
                            }
                        }
                    };
                    let argument_id = self.next()?;

                    let argument_lookup = self.lookup_expression.lookup(argument_id)?;
                    let argument_handle = get_expr_handle!(argument_id, argument_lookup);

                    let exec_scope_const = self.lookup_constant.lookup(exec_scope_id)?;
                    let _exec_scope = resolve_constant(ctx.gctx(), &exec_scope_const.inner)
                        .filter(|exec_scope| *exec_scope == spirv::Scope::Subgroup as u32)
                        .ok_or(Error::InvalidBarrierScope(exec_scope_id))?;

                    let op_id = match inst.op {
                        Op::GroupNonUniformAll => crate::SubgroupOperation::All,
                        Op::GroupNonUniformAny => crate::SubgroupOperation::Any,
                        Op::GroupNonUniformIAdd | Op::GroupNonUniformFAdd => {
                            crate::SubgroupOperation::Add
                        }
                        Op::GroupNonUniformIMul | Op::GroupNonUniformFMul => {
                            crate::SubgroupOperation::Mul
                        }
                        Op::GroupNonUniformSMax
                        | Op::GroupNonUniformUMax
                        | Op::GroupNonUniformFMax => crate::SubgroupOperation::Max,
                        Op::GroupNonUniformSMin
                        | Op::GroupNonUniformUMin
                        | Op::GroupNonUniformFMin => crate::SubgroupOperation::Min,
                        Op::GroupNonUniformBitwiseAnd | Op::GroupNonUniformLogicalAnd => {
                            crate::SubgroupOperation::And
                        }
                        Op::GroupNonUniformBitwiseOr | Op::GroupNonUniformLogicalOr => {
                            crate::SubgroupOperation::Or
                        }
                        Op::GroupNonUniformBitwiseXor | Op::GroupNonUniformLogicalXor => {
                            crate::SubgroupOperation::Xor
                        }
                        _ => unreachable!(),
                    };

                    let result_type = self.lookup_type.lookup(result_type_id)?;

                    let result_handle = ctx.expressions.append(
                        crate::Expression::SubgroupOperationResult {
                            ty: result_type.handle,
                        },
                        span,
                    );
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: result_handle,
                            type_id: result_type_id,
                            block_id,
                        },
                    );

                    block.push(
                        crate::Statement::SubgroupCollectiveOperation {
                            result: result_handle,
                            op: op_id,
                            collective_op: collective_op_id,
                            argument: argument_handle,
                        },
                        span,
                    );
                    emitter.start(ctx.expressions);
                }
                Op::GroupNonUniformBroadcastFirst
                | Op::GroupNonUniformBroadcast
                | Op::GroupNonUniformShuffle
                | Op::GroupNonUniformShuffleDown
                | Op::GroupNonUniformShuffleUp
                | Op::GroupNonUniformShuffleXor
                | Op::GroupNonUniformQuadBroadcast => {
                    inst.expect(if matches!(inst.op, Op::GroupNonUniformBroadcastFirst) {
                        5
                    } else {
                        6
                    })?;
                    block.extend(emitter.finish(ctx.expressions));
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let exec_scope_id = self.next()?;
                    let argument_id = self.next()?;

                    let argument_lookup = self.lookup_expression.lookup(argument_id)?;
                    let argument_handle = get_expr_handle!(argument_id, argument_lookup);

                    let exec_scope_const = self.lookup_constant.lookup(exec_scope_id)?;
                    let _exec_scope = resolve_constant(ctx.gctx(), &exec_scope_const.inner)
                        .filter(|exec_scope| *exec_scope == spirv::Scope::Subgroup as u32)
                        .ok_or(Error::InvalidBarrierScope(exec_scope_id))?;

                    let mode = if matches!(inst.op, Op::GroupNonUniformBroadcastFirst) {
                        crate::GatherMode::BroadcastFirst
                    } else {
                        let index_id = self.next()?;
                        let index_lookup = self.lookup_expression.lookup(index_id)?;
                        let index_handle = get_expr_handle!(index_id, index_lookup);
                        match inst.op {
                            Op::GroupNonUniformBroadcast => {
                                crate::GatherMode::Broadcast(index_handle)
                            }
                            Op::GroupNonUniformShuffle => crate::GatherMode::Shuffle(index_handle),
                            Op::GroupNonUniformShuffleDown => {
                                crate::GatherMode::ShuffleDown(index_handle)
                            }
                            Op::GroupNonUniformShuffleUp => {
                                crate::GatherMode::ShuffleUp(index_handle)
                            }
                            Op::GroupNonUniformShuffleXor => {
                                crate::GatherMode::ShuffleXor(index_handle)
                            }
                            Op::GroupNonUniformQuadBroadcast => {
                                crate::GatherMode::QuadBroadcast(index_handle)
                            }
                            _ => unreachable!(),
                        }
                    };

                    let result_type = self.lookup_type.lookup(result_type_id)?;

                    let result_handle = ctx.expressions.append(
                        crate::Expression::SubgroupOperationResult {
                            ty: result_type.handle,
                        },
                        span,
                    );
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: result_handle,
                            type_id: result_type_id,
                            block_id,
                        },
                    );

                    block.push(
                        crate::Statement::SubgroupGather {
                            result: result_handle,
                            mode,
                            argument: argument_handle,
                        },
                        span,
                    );
                    emitter.start(ctx.expressions);
                }
                Op::GroupNonUniformQuadSwap => {
                    inst.expect(6)?;
                    block.extend(emitter.finish(ctx.expressions));
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let exec_scope_id = self.next()?;
                    let argument_id = self.next()?;
                    let direction_id = self.next()?;

                    let argument_lookup = self.lookup_expression.lookup(argument_id)?;
                    let argument_handle = get_expr_handle!(argument_id, argument_lookup);

                    let exec_scope_const = self.lookup_constant.lookup(exec_scope_id)?;
                    let _exec_scope = resolve_constant(ctx.gctx(), &exec_scope_const.inner)
                        .filter(|exec_scope| *exec_scope == spirv::Scope::Subgroup as u32)
                        .ok_or(Error::InvalidBarrierScope(exec_scope_id))?;

                    let direction_const = self.lookup_constant.lookup(direction_id)?;
                    let direction_const = resolve_constant(ctx.gctx(), &direction_const.inner)
                        .ok_or(Error::InvalidOperand)?;
                    let direction = match direction_const {
                        0 => crate::Direction::X,
                        1 => crate::Direction::Y,
                        2 => crate::Direction::Diagonal,
                        _ => unreachable!(),
                    };

                    let result_type = self.lookup_type.lookup(result_type_id)?;

                    let result_handle = ctx.expressions.append(
                        crate::Expression::SubgroupOperationResult {
                            ty: result_type.handle,
                        },
                        span,
                    );
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle: result_handle,
                            type_id: result_type_id,
                            block_id,
                        },
                    );

                    block.push(
                        crate::Statement::SubgroupGather {
                            mode: crate::GatherMode::QuadSwap(direction),
                            result: result_handle,
                            argument: argument_handle,
                        },
                        span,
                    );
                    emitter.start(ctx.expressions);
                }
                Op::AtomicLoad => {
                    inst.expect(6)?;
                    let start = self.data_offset;
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let pointer_id = self.next()?;
                    let _scope_id = self.next()?;
                    let _memory_semantics_id = self.next()?;
                    let span = self.span_from_with_op(start);

                    log::trace!("\t\t\tlooking up expr {pointer_id:?}");
                    let p_lexp_handle =
                        get_expr_handle!(pointer_id, self.lookup_expression.lookup(pointer_id)?);

                    // Create an expression for our result
                    let expr = crate::Expression::Load {
                        pointer: p_lexp_handle,
                    };
                    let handle = ctx.expressions.append(expr, span);
                    self.lookup_expression.insert(
                        result_id,
                        LookupExpression {
                            handle,
                            type_id: result_type_id,
                            block_id,
                        },
                    );

                    // Store any associated global variables so we can upgrade their types later
                    self.record_atomic_access(ctx, p_lexp_handle)?;
                }
                Op::AtomicStore => {
                    inst.expect(5)?;
                    let start = self.data_offset;
                    let pointer_id = self.next()?;
                    let _scope_id = self.next()?;
                    let _memory_semantics_id = self.next()?;
                    let value_id = self.next()?;
                    let span = self.span_from_with_op(start);

                    log::trace!("\t\t\tlooking up pointer expr {pointer_id:?}");
                    let p_lexp_handle =
                        get_expr_handle!(pointer_id, self.lookup_expression.lookup(pointer_id)?);

                    log::trace!("\t\t\tlooking up value expr {pointer_id:?}");
                    let v_lexp_handle =
                        get_expr_handle!(value_id, self.lookup_expression.lookup(value_id)?);

                    block.extend(emitter.finish(ctx.expressions));
                    // Create a statement for the op itself
                    let stmt = crate::Statement::Store {
                        pointer: p_lexp_handle,
                        value: v_lexp_handle,
                    };
                    block.push(stmt, span);
                    emitter.start(ctx.expressions);

                    // Store any associated global variables so we can upgrade their types later
                    self.record_atomic_access(ctx, p_lexp_handle)?;
                }
                Op::AtomicIIncrement | Op::AtomicIDecrement => {
                    inst.expect(6)?;
                    let start = self.data_offset;
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let pointer_id = self.next()?;
                    let _scope_id = self.next()?;
                    let _memory_semantics_id = self.next()?;
                    let span = self.span_from_with_op(start);

                    let (p_exp_h, p_base_ty_h) = self.get_exp_and_base_ty_handles(
                        pointer_id,
                        ctx,
                        &mut emitter,
                        &mut block,
                        body_idx,
                    )?;

                    block.extend(emitter.finish(ctx.expressions));
                    // Create an expression for our result
                    let r_lexp_handle = {
                        let expr = crate::Expression::AtomicResult {
                            ty: p_base_ty_h,
                            comparison: false,
                        };
                        let handle = ctx.expressions.append(expr, span);
                        self.lookup_expression.insert(
                            result_id,
                            LookupExpression {
                                handle,
                                type_id: result_type_id,
                                block_id,
                            },
                        );
                        handle
                    };
                    emitter.start(ctx.expressions);

                    // Create a literal "1" to use as our value
                    let one_lexp_handle = make_index_literal(
                        ctx,
                        1,
                        &mut block,
                        &mut emitter,
                        p_base_ty_h,
                        result_type_id,
                        span,
                    )?;

                    // Create a statement for the op itself
                    let stmt = crate::Statement::Atomic {
                        pointer: p_exp_h,
                        fun: match inst.op {
                            Op::AtomicIIncrement => crate::AtomicFunction::Add,
                            _ => crate::AtomicFunction::Subtract,
                        },
                        value: one_lexp_handle,
                        result: Some(r_lexp_handle),
                    };
                    block.push(stmt, span);

                    // Store any associated global variables so we can upgrade their types later
                    self.record_atomic_access(ctx, p_exp_h)?;
                }
                Op::AtomicCompareExchange => {
                    inst.expect(9)?;

                    let start = self.data_offset;
                    let span = self.span_from_with_op(start);
                    let result_type_id = self.next()?;
                    let result_id = self.next()?;
                    let pointer_id = self.next()?;
                    let _memory_scope_id = self.next()?;
                    let _equal_memory_semantics_id = self.next()?;
                    let _unequal_memory_semantics_id = self.next()?;
                    let value_id = self.next()?;
                    let comparator_id = self.next()?;

                    let (p_exp_h, p_base_ty_h) = self.get_exp_and_base_ty_handles(
                        pointer_id,
                        ctx,
                        &mut emitter,
                        &mut block,
                        body_idx,
                    )?;

                    log::trace!("\t\t\tlooking up value expr {value_id:?}");
                    let v_lexp_handle =
                        get_expr_handle!(value_id, self.lookup_expression.lookup(value_id)?);

                    log::trace!("\t\t\tlooking up comparator expr {value_id:?}");
                    let c_lexp_handle = get_expr_handle!(
                        comparator_id,
                        self.lookup_expression.lookup(comparator_id)?
                    );

                    // We know from the SPIR-V spec that the result type must be an integer
                    // scalar, and we'll need the type itself to get a handle to the atomic
                    // result struct.
                    let crate::TypeInner::Scalar(scalar) = ctx.module.types[p_base_ty_h].inner
                    else {
                        return Err(
                            crate::front::atomic_upgrade::Error::CompareExchangeNonScalarBaseType
                                .into(),
                        );
                    };

                    // Get a handle to the atomic result struct type.
                    let atomic_result_struct_ty_h = ctx.module.generate_predeclared_type(
                        crate::PredeclaredType::AtomicCompareExchangeWeakResult(scalar),
                    );

                    block.extend(emitter.finish(ctx.expressions));

                    // Create an expression for our atomic result
                    let atomic_lexp_handle = {
                        let expr = crate::Expression::AtomicResult {
                            ty: atomic_result_struct_ty_h,
                            comparison: true,
                        };
                        ctx.expressions.append(expr, span)
                    };

                    // Create an dot accessor to extract the value from the
                    // result struct __atomic_compare_exchange_result<T> and use that
                    // as the expression for the result_id
                    {
                        let expr = crate::Expression::AccessIndex {
                            base: atomic_lexp_handle,
                            index: 0,
                        };
                        let handle = ctx.expressions.append(expr, span);
                        // Use this dot accessor as the result id's expression
                        let _ = self.lookup_expression.insert(
                            result_id,
                            LookupExpression {
                                handle,
                                type_id: result_type_id,
                                block_id,
                            },
                        );
                    }

                    emitter.start(ctx.expressions);

                    // Create a statement for the op itself
                    let stmt = crate::Statement::Atomic {
                        pointer: p_exp_h,
                        fun: crate::AtomicFunction::Exchange {
                            compare: Some(c_lexp_handle),
                        },
                        value: v_lexp_handle,
                        result: Some(atomic_lexp_handle),
                    };
                    block.push(stmt, span);

                    // Store any associated global variables so we can upgrade their types later
                    self.record_atomic_access(ctx, p_exp_h)?;
                }
                Op::AtomicExchange
                | Op::AtomicIAdd
                | Op::AtomicISub
                | Op::AtomicSMin
                | Op::AtomicUMin
                | Op::AtomicSMax
                | Op::AtomicUMax
                | Op::AtomicAnd
                | Op::AtomicOr
                | Op::AtomicXor
                | Op::AtomicFAddEXT => self.parse_atomic_expr_with_value(
                    inst,
                    &mut emitter,
                    ctx,
                    &mut block,
                    block_id,
                    body_idx,
                    match inst.op {
                        Op::AtomicExchange => crate::AtomicFunction::Exchange { compare: None },
                        Op::AtomicIAdd | Op::AtomicFAddEXT => crate::AtomicFunction::Add,
                        Op::AtomicISub => crate::AtomicFunction::Subtract,
                        Op::AtomicSMin => crate::AtomicFunction::Min,
                        Op::AtomicUMin => crate::AtomicFunction::Min,
                        Op::AtomicSMax => crate::AtomicFunction::Max,
                        Op::AtomicUMax => crate::AtomicFunction::Max,
                        Op::AtomicAnd => crate::AtomicFunction::And,
                        Op::AtomicOr => crate::AtomicFunction::InclusiveOr,
                        Op::AtomicXor => crate::AtomicFunction::ExclusiveOr,
                        _ => unreachable!(),
                    },
                )?,

                _ => {
                    return Err(Error::UnsupportedInstruction(self.state, inst.op));
                }
            }
        };

        block.extend(emitter.finish(ctx.expressions));
        if let Some(stmt) = terminator {
            block.push(stmt, crate::Span::default());
        }

        // Save this block fragment in `block_ctx.blocks`, and mark it to be
        // incorporated into the current body at `Statement` assembly time.
        ctx.blocks.insert(block_id, block);
        let body = &mut ctx.bodies[body_idx];
        body.data.push(BodyFragment::BlockId(block_id));
        Ok(())
    }
}

fn make_index_literal(
    ctx: &mut BlockContext,
    index: u32,
    block: &mut crate::Block,
    emitter: &mut crate::proc::Emitter,
    index_type: Handle<crate::Type>,
    index_type_id: spirv::Word,
    span: crate::Span,
) -> Result<Handle<crate::Expression>, Error> {
    block.extend(emitter.finish(ctx.expressions));

    let literal = match ctx.module.types[index_type].inner.scalar_kind() {
        Some(crate::ScalarKind::Uint) => crate::Literal::U32(index),
        Some(crate::ScalarKind::Sint) => crate::Literal::I32(index as i32),
        _ => return Err(Error::InvalidIndexType(index_type_id)),
    };
    let expr = ctx
        .expressions
        .append(crate::Expression::Literal(literal), span);

    emitter.start(ctx.expressions);
    Ok(expr)
}
