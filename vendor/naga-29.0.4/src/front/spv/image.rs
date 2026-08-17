use alloc::vec::Vec;

use crate::{
    arena::{Handle, UniqueArena},
    Scalar,
};

use super::{Error, LookupExpression, LookupHelper as _};

#[derive(Clone, Debug)]
pub(super) struct LookupSampledImage {
    image: Handle<crate::Expression>,
    sampler: Handle<crate::Expression>,
}

bitflags::bitflags! {
    /// Flags describing sampling method.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct SamplingFlags: u32 {
        /// Regular sampling.
        const REGULAR = 0x1;
        /// Comparison sampling.
        const COMPARISON = 0x2;
    }
}

impl super::BlockContext<'_> {
    fn get_image_expr_ty(
        &self,
        handle: Handle<crate::Expression>,
    ) -> Result<Handle<crate::Type>, Error> {
        match self.expressions[handle] {
            crate::Expression::GlobalVariable(handle) => {
                Ok(self.module.global_variables[handle].ty)
            }
            crate::Expression::FunctionArgument(i) => Ok(self.arguments[i as usize].ty),
            crate::Expression::Access { base, .. } => Ok(self.get_image_expr_ty(base)?),
            ref other => Err(Error::InvalidImageExpression(other.clone())),
        }
    }
}

/// Options of a sampling operation.
#[derive(Debug)]
pub struct SamplingOptions {
    /// Projection sampling: the division by W is expected to happen
    /// in the texture unit.
    pub project: bool,
    /// Depth comparison sampling with a reference value.
    pub compare: bool,
    /// Gather sampling: Operates on four samples of one channel.
    pub gather: bool,
}

enum ExtraCoordinate {
    ArrayLayer,
    Projection,
    Garbage,
}

/// Return the texture coordinates separated from the array layer,
/// and/or divided by the projection term.
///
/// The Proj sampling ops expect an extra coordinate for the W.
/// The arrayed (can't be Proj!) images expect an extra coordinate for the layer.
fn extract_image_coordinates(
    image_dim: crate::ImageDimension,
    extra_coordinate: ExtraCoordinate,
    base: Handle<crate::Expression>,
    coordinate_ty: Handle<crate::Type>,
    ctx: &mut super::BlockContext,
) -> (Handle<crate::Expression>, Option<Handle<crate::Expression>>) {
    let (given_size, kind) = match ctx.module.types[coordinate_ty].inner {
        crate::TypeInner::Scalar(Scalar { kind, .. }) => (None, kind),
        crate::TypeInner::Vector {
            size,
            scalar: Scalar { kind, .. },
        } => (Some(size), kind),
        ref other => unreachable!("Unexpected texture coordinate {:?}", other),
    };

    let required_size = image_dim.required_coordinate_size();
    let required_ty = required_size.map(|size| {
        ctx.module
            .types
            .get(&crate::Type {
                name: None,
                inner: crate::TypeInner::Vector {
                    size,
                    scalar: Scalar { kind, width: 4 },
                },
            })
            .expect("Required coordinate type should have been set up by `parse_type_image`!")
    });
    let extra_expr = crate::Expression::AccessIndex {
        base,
        index: required_size.map_or(1, |size| size as u32),
    };

    let base_span = ctx.expressions.get_span(base);

    match extra_coordinate {
        ExtraCoordinate::ArrayLayer => {
            let extracted = match required_size {
                None => ctx
                    .expressions
                    .append(crate::Expression::AccessIndex { base, index: 0 }, base_span),
                Some(size) => {
                    let mut components = Vec::with_capacity(size as usize);
                    for index in 0..size as u32 {
                        let comp = ctx
                            .expressions
                            .append(crate::Expression::AccessIndex { base, index }, base_span);
                        components.push(comp);
                    }
                    ctx.expressions.append(
                        crate::Expression::Compose {
                            ty: required_ty.unwrap(),
                            components,
                        },
                        base_span,
                    )
                }
            };
            let array_index_f32 = ctx.expressions.append(extra_expr, base_span);
            let array_index = ctx.expressions.append(
                crate::Expression::As {
                    kind: crate::ScalarKind::Sint,
                    expr: array_index_f32,
                    convert: Some(4),
                },
                base_span,
            );
            (extracted, Some(array_index))
        }
        ExtraCoordinate::Projection => {
            let projection = ctx.expressions.append(extra_expr, base_span);
            let divided = match required_size {
                None => {
                    let temp = ctx
                        .expressions
                        .append(crate::Expression::AccessIndex { base, index: 0 }, base_span);
                    ctx.expressions.append(
                        crate::Expression::Binary {
                            op: crate::BinaryOperator::Divide,
                            left: temp,
                            right: projection,
                        },
                        base_span,
                    )
                }
                Some(size) => {
                    let mut components = Vec::with_capacity(size as usize);
                    for index in 0..size as u32 {
                        let temp = ctx
                            .expressions
                            .append(crate::Expression::AccessIndex { base, index }, base_span);
                        let comp = ctx.expressions.append(
                            crate::Expression::Binary {
                                op: crate::BinaryOperator::Divide,
                                left: temp,
                                right: projection,
                            },
                            base_span,
                        );
                        components.push(comp);
                    }
                    ctx.expressions.append(
                        crate::Expression::Compose {
                            ty: required_ty.unwrap(),
                            components,
                        },
                        base_span,
                    )
                }
            };
            (divided, None)
        }
        ExtraCoordinate::Garbage if given_size == required_size => (base, None),
        ExtraCoordinate::Garbage => {
            use crate::SwizzleComponent as Sc;
            let cut_expr = match required_size {
                None => crate::Expression::AccessIndex { base, index: 0 },
                Some(size) => crate::Expression::Swizzle {
                    size,
                    vector: base,
                    pattern: [Sc::X, Sc::Y, Sc::Z, Sc::W],
                },
            };
            (ctx.expressions.append(cut_expr, base_span), None)
        }
    }
}

pub(super) fn patch_comparison_type(
    flags: SamplingFlags,
    var: &mut crate::GlobalVariable,
    arena: &mut UniqueArena<crate::Type>,
) -> bool {
    if !flags.contains(SamplingFlags::COMPARISON) {
        return true;
    }
    if flags == SamplingFlags::all() {
        return false;
    }

    log::debug!("Flipping comparison for {var:?}");
    let original_ty = &arena[var.ty];
    let original_ty_span = arena.get_span(var.ty);
    let ty_inner = match original_ty.inner {
        crate::TypeInner::Image {
            class: crate::ImageClass::Sampled { multi, .. },
            dim,
            arrayed,
        } => crate::TypeInner::Image {
            class: crate::ImageClass::Depth { multi },
            dim,
            arrayed,
        },
        crate::TypeInner::Sampler { .. } => crate::TypeInner::Sampler { comparison: true },
        ref other => unreachable!("Unexpected type for comparison mutation: {:?}", other),
    };

    let name = original_ty.name.clone();
    var.ty = arena.insert(
        crate::Type {
            name,
            inner: ty_inner,
        },
        original_ty_span,
    );
    true
}

impl<I: Iterator<Item = u32>> super::Frontend<I> {
    pub(super) fn parse_image_couple(&mut self) -> Result<(), Error> {
        let _result_type_id = self.next()?;
        let result_id = self.next()?;
        let image_id = self.next()?;
        let sampler_id = self.next()?;
        let image_lexp = self.lookup_expression.lookup(image_id)?;
        let sampler_lexp = self.lookup_expression.lookup(sampler_id)?;
        self.lookup_sampled_image.insert(
            result_id,
            LookupSampledImage {
                image: image_lexp.handle,
                sampler: sampler_lexp.handle,
            },
        );
        Ok(())
    }

    pub(super) fn parse_image_uncouple(&mut self, block_id: spirv::Word) -> Result<(), Error> {
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let sampled_image_id = self.next()?;
        self.lookup_expression.insert(
            result_id,
            LookupExpression {
                handle: self.lookup_sampled_image.lookup(sampled_image_id)?.image,
                type_id: result_type_id,
                block_id,
            },
        );
        Ok(())
    }

    pub(super) fn parse_image_write(
        &mut self,
        words_left: u16,
        ctx: &mut super::BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        body_idx: usize,
    ) -> Result<crate::Statement, Error> {
        let image_id = self.next()?;
        let coordinate_id = self.next()?;
        let value_id = self.next()?;

        let image_ops = if words_left != 0 { self.next()? } else { 0 };

        if image_ops != 0 {
            let other = spirv::ImageOperands::from_bits_truncate(image_ops);
            log::warn!("Unknown image write ops {other:?}");
            for _ in 1..words_left {
                self.next()?;
            }
        }

        let image_lexp = self.lookup_expression.lookup(image_id)?;
        let image_ty = ctx.get_image_expr_ty(image_lexp.handle)?;

        let coord_lexp = self.lookup_expression.lookup(coordinate_id)?;
        let coord_handle =
            self.get_expr_handle(coordinate_id, coord_lexp, ctx, emitter, block, body_idx);
        let coord_type_handle = self.lookup_type.lookup(coord_lexp.type_id)?.handle;
        let (coordinate, array_index) = match ctx.module.types[image_ty].inner {
            crate::TypeInner::Image {
                dim,
                arrayed,
                class: _,
            } => extract_image_coordinates(
                dim,
                if arrayed {
                    ExtraCoordinate::ArrayLayer
                } else {
                    ExtraCoordinate::Garbage
                },
                coord_handle,
                coord_type_handle,
                ctx,
            ),
            _ => return Err(Error::InvalidImage(image_ty)),
        };

        let value_lexp = self.lookup_expression.lookup(value_id)?;
        let value = self.get_expr_handle(value_id, value_lexp, ctx, emitter, block, body_idx);
        let value_type = self.lookup_type.lookup(value_lexp.type_id)?.handle;

        // In hlsl etc, the write value may not be the vector 4.
        let expanded_value = match ctx.module.types[value_type].inner {
            crate::TypeInner::Scalar(_) => Some(crate::Expression::Splat {
                value,
                size: crate::VectorSize::Quad,
            }),
            crate::TypeInner::Vector { size, .. } => match size {
                crate::VectorSize::Bi => Some(crate::Expression::Swizzle {
                    size: crate::VectorSize::Quad,
                    vector: value,
                    pattern: [
                        crate::SwizzleComponent::X,
                        crate::SwizzleComponent::Y,
                        crate::SwizzleComponent::Y,
                        crate::SwizzleComponent::Y,
                    ],
                }),
                crate::VectorSize::Tri => Some(crate::Expression::Swizzle {
                    size: crate::VectorSize::Quad,
                    vector: value,
                    pattern: [
                        crate::SwizzleComponent::X,
                        crate::SwizzleComponent::Y,
                        crate::SwizzleComponent::Z,
                        crate::SwizzleComponent::Z,
                    ],
                }),
                crate::VectorSize::Quad => None,
            },
            _ => return Err(Error::InvalidVectorType(value_type)),
        };

        let value_patched = if let Some(s) = expanded_value {
            ctx.expressions.append(s, crate::Span::default())
        } else {
            value
        };

        Ok(crate::Statement::ImageStore {
            image: image_lexp.handle,
            coordinate,
            array_index,
            value: value_patched,
        })
    }

    pub(super) fn parse_image_load(
        &mut self,
        mut words_left: u16,
        ctx: &mut super::BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        block_id: spirv::Word,
        body_idx: usize,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let image_id = self.next()?;
        let coordinate_id = self.next()?;

        let mut image_ops = if words_left != 0 {
            words_left -= 1;
            self.next()?
        } else {
            0
        };

        let mut sample = None;
        let mut level = None;
        while image_ops != 0 {
            let bit = 1 << image_ops.trailing_zeros();
            match spirv::ImageOperands::from_bits_truncate(bit) {
                spirv::ImageOperands::LOD => {
                    let lod_expr = self.next()?;
                    let lod_lexp = self.lookup_expression.lookup(lod_expr)?;
                    let lod_handle =
                        self.get_expr_handle(lod_expr, lod_lexp, ctx, emitter, block, body_idx);
                    level = Some(lod_handle);
                    words_left -= 1;
                }
                spirv::ImageOperands::SAMPLE => {
                    let sample_expr = self.next()?;
                    let sample_handle = self.lookup_expression.lookup(sample_expr)?.handle;
                    sample = Some(sample_handle);
                    words_left -= 1;
                }
                other => {
                    log::warn!("Unknown image load op {other:?}");
                    for _ in 0..words_left {
                        self.next()?;
                    }
                    break;
                }
            }
            image_ops ^= bit;
        }

        // No need to call get_expr_handle here since only globals/arguments are
        // allowed as images and they are always in the root scope
        let image_lexp = self.lookup_expression.lookup(image_id)?;
        let image_ty = ctx.get_image_expr_ty(image_lexp.handle)?;

        let coord_lexp = self.lookup_expression.lookup(coordinate_id)?;
        let coord_handle =
            self.get_expr_handle(coordinate_id, coord_lexp, ctx, emitter, block, body_idx);
        let coord_type_handle = self.lookup_type.lookup(coord_lexp.type_id)?.handle;
        let (coordinate, array_index, is_depth) = match ctx.module.types[image_ty].inner {
            crate::TypeInner::Image {
                dim,
                arrayed,
                class,
            } => {
                let (coord, array_index) = extract_image_coordinates(
                    dim,
                    if arrayed {
                        ExtraCoordinate::ArrayLayer
                    } else {
                        ExtraCoordinate::Garbage
                    },
                    coord_handle,
                    coord_type_handle,
                    ctx,
                );
                (coord, array_index, class.is_depth())
            }
            _ => return Err(Error::InvalidImage(image_ty)),
        };

        let image_load_expr = crate::Expression::ImageLoad {
            image: image_lexp.handle,
            coordinate,
            array_index,
            sample,
            level,
        };
        let image_load_handle = ctx
            .expressions
            .append(image_load_expr, self.span_from_with_op(start));

        let handle = if is_depth {
            let result_ty = self.lookup_type.lookup(result_type_id)?;
            // The return type of `OpImageRead` can be a scalar or vector.
            match ctx.module.types[result_ty.handle].inner {
                crate::TypeInner::Vector { size, .. } => {
                    let splat_expr = crate::Expression::Splat {
                        size,
                        value: image_load_handle,
                    };
                    ctx.expressions
                        .append(splat_expr, self.span_from_with_op(start))
                }
                _ => image_load_handle,
            }
        } else {
            image_load_handle
        };

        self.lookup_expression.insert(
            result_id,
            LookupExpression {
                handle,
                type_id: result_type_id,
                block_id,
            },
        );
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn parse_image_sample(
        &mut self,
        mut words_left: u16,
        options: SamplingOptions,
        ctx: &mut super::BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        block_id: spirv::Word,
        body_idx: usize,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let sampled_image_id = self.next()?;
        let coordinate_id = self.next()?;
        let (component_id, dref_id) = match (options.gather, options.compare) {
            (true, false) => (Some(self.next()?), None),
            (_, true) => (None, Some(self.next()?)),
            (_, _) => (None, None),
        };
        let span = self.span_from_with_op(start);

        let mut image_ops = if words_left != 0 {
            words_left -= 1;
            self.next()?
        } else {
            0
        };

        let mut level = crate::SampleLevel::Auto;
        let mut offset = None;
        while image_ops != 0 {
            let bit = 1 << image_ops.trailing_zeros();
            match spirv::ImageOperands::from_bits_truncate(bit) {
                spirv::ImageOperands::BIAS => {
                    let bias_expr = self.next()?;
                    let bias_lexp = self.lookup_expression.lookup(bias_expr)?;
                    let bias_handle =
                        self.get_expr_handle(bias_expr, bias_lexp, ctx, emitter, block, body_idx);
                    level = crate::SampleLevel::Bias(bias_handle);
                    words_left -= 1;
                }
                spirv::ImageOperands::LOD => {
                    let lod_expr = self.next()?;
                    let lod_lexp = self.lookup_expression.lookup(lod_expr)?;
                    let lod_handle =
                        self.get_expr_handle(lod_expr, lod_lexp, ctx, emitter, block, body_idx);

                    let is_depth_image = {
                        let image_lexp = self.lookup_sampled_image.lookup(sampled_image_id)?;
                        let image_ty = ctx.get_image_expr_ty(image_lexp.image)?;
                        matches!(
                            ctx.module.types[image_ty].inner,
                            crate::TypeInner::Image {
                                class: crate::ImageClass::Depth { .. },
                                ..
                            }
                        )
                    };

                    level = if options.compare {
                        log::debug!("Assuming {lod_handle:?} is zero");
                        crate::SampleLevel::Zero
                    } else if is_depth_image {
                        log::debug!(
                            "Assuming level {lod_handle:?} converts losslessly to an integer"
                        );
                        let expr = crate::Expression::As {
                            expr: lod_handle,
                            kind: crate::ScalarKind::Sint,
                            convert: Some(4),
                        };
                        let s32_lod_handle = ctx.expressions.append(expr, span);
                        crate::SampleLevel::Exact(s32_lod_handle)
                    } else {
                        crate::SampleLevel::Exact(lod_handle)
                    };
                    words_left -= 1;
                }
                spirv::ImageOperands::GRAD => {
                    let grad_x_expr = self.next()?;
                    let grad_x_lexp = self.lookup_expression.lookup(grad_x_expr)?;
                    let grad_x_handle = self.get_expr_handle(
                        grad_x_expr,
                        grad_x_lexp,
                        ctx,
                        emitter,
                        block,
                        body_idx,
                    );
                    let grad_y_expr = self.next()?;
                    let grad_y_lexp = self.lookup_expression.lookup(grad_y_expr)?;
                    let grad_y_handle = self.get_expr_handle(
                        grad_y_expr,
                        grad_y_lexp,
                        ctx,
                        emitter,
                        block,
                        body_idx,
                    );
                    level = if options.compare {
                        log::debug!(
                            "Assuming gradients {grad_x_handle:?} and {grad_y_handle:?} are not greater than 1"
                        );
                        crate::SampleLevel::Zero
                    } else {
                        crate::SampleLevel::Gradient {
                            x: grad_x_handle,
                            y: grad_y_handle,
                        }
                    };
                    words_left -= 2;
                }
                spirv::ImageOperands::CONST_OFFSET => {
                    let offset_expr = self.next()?;
                    let offset_lexp = self.lookup_expression.lookup(offset_expr)?;
                    let offset_handle = self.get_expr_handle(
                        offset_expr,
                        offset_lexp,
                        ctx,
                        emitter,
                        block,
                        body_idx,
                    );
                    offset = Some(offset_handle);
                    words_left -= 1;
                }
                other => {
                    log::warn!("Unknown image sample operand {other:?}");
                    for _ in 0..words_left {
                        self.next()?;
                    }
                    break;
                }
            }
            image_ops ^= bit;
        }

        let si_lexp = self.lookup_sampled_image.lookup(sampled_image_id)?;
        let coord_lexp = self.lookup_expression.lookup(coordinate_id)?;
        let coord_handle =
            self.get_expr_handle(coordinate_id, coord_lexp, ctx, emitter, block, body_idx);
        let coord_type_handle = self.lookup_type.lookup(coord_lexp.type_id)?.handle;

        let gather = match (options.gather, component_id) {
            (true, Some(component_id)) => {
                let component_lexp = self.lookup_expression.lookup(component_id)?;

                let component_value = match ctx.expressions[component_lexp.handle] {
                    // VUID-StandaloneSpirv-OpImageGather-04664:
                    // The “Component” operand of OpImageGather, and OpImageSparseGather must be the
                    // <id> of a constant instruction.
                    crate::Expression::Constant(const_handle) => {
                        let constant = &ctx.module.constants[const_handle];
                        match ctx.module.global_expressions[constant.init] {
                            // SPIR-V specification: "It must be a 32-bit integer type scalar."
                            crate::Expression::Literal(crate::Literal::U32(value)) => value,
                            crate::Expression::Literal(crate::Literal::I32(value)) => value as u32,
                            _ => {
                                log::error!(
                                    "Image gather component constant must be a 32-bit integer literal"
                                );
                                return Err(Error::InvalidOperand);
                            }
                        }
                    }
                    _ => {
                        log::error!("Image gather component must be a constant");
                        return Err(Error::InvalidOperand);
                    }
                };

                debug_assert_eq!(level, crate::SampleLevel::Auto);
                level = crate::SampleLevel::Zero;

                // SPIR-V specification: "Behavior is undefined if its value is not 0, 1, 2 or 3."
                match component_value {
                    0 => Some(crate::SwizzleComponent::X),
                    1 => Some(crate::SwizzleComponent::Y),
                    2 => Some(crate::SwizzleComponent::Z),
                    3 => Some(crate::SwizzleComponent::W),
                    other => {
                        log::error!("Invalid gather component operand: {other}");
                        return Err(Error::InvalidOperand);
                    }
                }
            }
            (true, None) => {
                debug_assert_eq!(level, crate::SampleLevel::Auto);
                level = crate::SampleLevel::Zero;

                Some(crate::SwizzleComponent::X)
            }
            (_, _) => None,
        };

        let sampling_bit = if options.compare {
            SamplingFlags::COMPARISON
        } else {
            SamplingFlags::REGULAR
        };

        let image_ty = match ctx.expressions[si_lexp.image] {
            crate::Expression::GlobalVariable(handle) => {
                if let Some(flags) = self.handle_sampling.get_mut(&handle) {
                    *flags |= sampling_bit;
                }

                ctx.module.global_variables[handle].ty
            }

            crate::Expression::FunctionArgument(i) => {
                ctx.parameter_sampling[i as usize] |= sampling_bit;
                ctx.arguments[i as usize].ty
            }

            crate::Expression::Access { base, .. } => match ctx.expressions[base] {
                crate::Expression::GlobalVariable(handle) => {
                    if let Some(flags) = self.handle_sampling.get_mut(&handle) {
                        *flags |= sampling_bit;
                    }

                    match ctx.module.types[ctx.module.global_variables[handle].ty].inner {
                        crate::TypeInner::BindingArray { base, .. } => base,
                        _ => return Err(Error::InvalidGlobalVar(ctx.expressions[base].clone())),
                    }
                }

                ref other => return Err(Error::InvalidGlobalVar(other.clone())),
            },

            ref other => return Err(Error::InvalidGlobalVar(other.clone())),
        };

        match ctx.expressions[si_lexp.sampler] {
            crate::Expression::GlobalVariable(handle) => {
                *self.handle_sampling.get_mut(&handle).unwrap() |= sampling_bit;
            }

            crate::Expression::FunctionArgument(i) => {
                ctx.parameter_sampling[i as usize] |= sampling_bit;
            }

            crate::Expression::Access { base, .. } => match ctx.expressions[base] {
                crate::Expression::GlobalVariable(handle) => {
                    *self.handle_sampling.get_mut(&handle).unwrap() |= sampling_bit;
                }

                ref other => return Err(Error::InvalidGlobalVar(other.clone())),
            },

            ref other => return Err(Error::InvalidGlobalVar(other.clone())),
        }

        let ((coordinate, array_index), depth_ref, is_depth) =
            match ctx.module.types[image_ty].inner {
                crate::TypeInner::Image {
                    dim,
                    arrayed,
                    class,
                } => (
                    extract_image_coordinates(
                        dim,
                        if options.project {
                            ExtraCoordinate::Projection
                        } else if arrayed {
                            ExtraCoordinate::ArrayLayer
                        } else {
                            ExtraCoordinate::Garbage
                        },
                        coord_handle,
                        coord_type_handle,
                        ctx,
                    ),
                    {
                        match dref_id {
                            Some(id) => {
                                let expr_lexp = self.lookup_expression.lookup(id)?;
                                let mut expr = self
                                    .get_expr_handle(id, expr_lexp, ctx, emitter, block, body_idx);

                                if options.project {
                                    let required_size = dim.required_coordinate_size();
                                    let right = ctx.expressions.append(
                                        crate::Expression::AccessIndex {
                                            base: coord_handle,
                                            index: required_size.map_or(1, |size| size as u32),
                                        },
                                        crate::Span::default(),
                                    );
                                    expr = ctx.expressions.append(
                                        crate::Expression::Binary {
                                            op: crate::BinaryOperator::Divide,
                                            left: expr,
                                            right,
                                        },
                                        crate::Span::default(),
                                    )
                                };
                                Some(expr)
                            }
                            None => None,
                        }
                    },
                    class.is_depth(),
                ),
                _ => return Err(Error::InvalidImage(image_ty)),
            };

        let expr = crate::Expression::ImageSample {
            image: si_lexp.image,
            sampler: si_lexp.sampler,
            gather,
            coordinate,
            array_index,
            offset,
            level,
            depth_ref,
            clamp_to_edge: false,
        };
        let image_sample_handle = ctx.expressions.append(expr, self.span_from_with_op(start));
        let handle = if is_depth && depth_ref.is_none() {
            let splat_expr = crate::Expression::Splat {
                size: crate::VectorSize::Quad,
                value: image_sample_handle,
            };
            ctx.expressions
                .append(splat_expr, self.span_from_with_op(start))
        } else {
            image_sample_handle
        };
        self.lookup_expression.insert(
            result_id,
            LookupExpression {
                handle,
                type_id: result_type_id,
                block_id,
            },
        );
        Ok(())
    }

    pub(super) fn parse_image_query_size(
        &mut self,
        at_level: bool,
        ctx: &mut super::BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        block_id: spirv::Word,
        body_idx: usize,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let image_id = self.next()?;
        let level = if at_level {
            let level_id = self.next()?;
            let level_lexp = self.lookup_expression.lookup(level_id)?;
            Some(self.get_expr_handle(level_id, level_lexp, ctx, emitter, block, body_idx))
        } else {
            None
        };

        // No need to call get_expr_handle here since only globals/arguments are
        // allowed as images and they are always in the root scope
        //TODO: handle arrays and cubes
        let image_lexp = self.lookup_expression.lookup(image_id)?;

        let expr = crate::Expression::ImageQuery {
            image: image_lexp.handle,
            query: crate::ImageQuery::Size { level },
        };

        let result_type_handle = self.lookup_type.lookup(result_type_id)?.handle;
        let maybe_scalar_kind = ctx.module.types[result_type_handle].inner.scalar_kind();

        let expr = if maybe_scalar_kind == Some(crate::ScalarKind::Sint) {
            crate::Expression::As {
                expr: ctx.expressions.append(expr, self.span_from_with_op(start)),
                kind: crate::ScalarKind::Sint,
                convert: Some(4),
            }
        } else {
            expr
        };

        self.lookup_expression.insert(
            result_id,
            LookupExpression {
                handle: ctx.expressions.append(expr, self.span_from_with_op(start)),
                type_id: result_type_id,
                block_id,
            },
        );

        Ok(())
    }

    pub(super) fn parse_image_query_other(
        &mut self,
        query: crate::ImageQuery,
        ctx: &mut super::BlockContext,
        block_id: spirv::Word,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let image_id = self.next()?;

        // No need to call get_expr_handle here since only globals/arguments are
        // allowed as images and they are always in the root scope
        let image_lexp = self.lookup_expression.lookup(image_id)?.clone();

        let expr = crate::Expression::ImageQuery {
            image: image_lexp.handle,
            query,
        };

        let result_type_handle = self.lookup_type.lookup(result_type_id)?.handle;
        let maybe_scalar_kind = ctx.module.types[result_type_handle].inner.scalar_kind();

        let expr = if maybe_scalar_kind == Some(crate::ScalarKind::Sint) {
            crate::Expression::As {
                expr: ctx.expressions.append(expr, self.span_from_with_op(start)),
                kind: crate::ScalarKind::Sint,
                convert: Some(4),
            }
        } else {
            expr
        };

        self.lookup_expression.insert(
            result_id,
            LookupExpression {
                handle: ctx.expressions.append(expr, self.span_from_with_op(start)),
                type_id: result_type_id,
                block_id,
            },
        );

        Ok(())
    }
}
