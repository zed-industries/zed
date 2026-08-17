/*!
Generating SPIR-V for image operations.
*/

use spirv::Word;

use super::{
    selection::{MergeTuple, Selection},
    Block, BlockContext, Error, IdGenerator, Instruction, LocalType, LookupType, NumericType,
};
use crate::arena::Handle;

/// Information about a vector of coordinates.
///
/// The coordinate vectors expected by SPIR-V `OpImageRead` and `OpImageFetch`
/// supply the array index for arrayed images as an additional component at
/// the end, whereas Naga's `ImageLoad`, `ImageStore`, and `ImageSample` carry
/// the array index as a separate field.
///
/// In the process of generating code to compute the combined vector, we also
/// produce SPIR-V types and vector lengths that are useful elsewhere. This
/// struct gathers that information into one place, with standard names.
struct ImageCoordinates {
    /// The SPIR-V id of the combined coordinate/index vector value.
    ///
    /// Note: when indexing a non-arrayed 1D image, this will be a scalar.
    value_id: Word,

    /// The SPIR-V id of the type of `value`.
    type_id: Word,

    /// The number of components in `value`, if it is a vector, or `None` if it
    /// is a scalar.
    size: Option<crate::VectorSize>,
}

/// A trait for image access (load or store) code generators.
///
/// Types implementing this trait hold information about an `ImageStore` or
/// `ImageLoad` operation that is not affected by the bounds check policy. The
/// `generate` method emits code for the access, given the results of bounds
/// checking.
///
/// The [`image`] bounds checks policy affects access coordinates, level of
/// detail, and sample index, but never the image id, result type (if any), or
/// the specific SPIR-V instruction used. Types that implement this trait gather
/// together the latter category, so we don't have to plumb them through the
/// bounds-checking code.
///
/// [`image`]: crate::proc::BoundsCheckPolicies::index
trait Access {
    /// The Rust type that represents SPIR-V values and types for this access.
    ///
    /// For operations like loads, this is `Word`. For operations like stores,
    /// this is `()`.
    ///
    /// For `ReadZeroSkipWrite`, this will be the type of the selection
    /// construct that performs the bounds checks, so it must implement
    /// `MergeTuple`.
    type Output: MergeTuple + Copy + Clone;

    /// Write an image access to `block`.
    ///
    /// Access the texel at `coordinates_id`. The optional `level_id` indicates
    /// the level of detail, and `sample_id` is the index of the sample to
    /// access in a multisampled texel.
    ///
    /// This method assumes that `coordinates_id` has already had the image array
    /// index, if any, folded in, as done by `write_image_coordinates`.
    ///
    /// Return the value id produced by the instruction, if any.
    ///
    /// Use `id_gen` to generate SPIR-V ids as necessary.
    fn generate(
        &self,
        id_gen: &mut IdGenerator,
        coordinates_id: Word,
        level_id: Option<Word>,
        sample_id: Option<Word>,
        block: &mut Block,
    ) -> Self::Output;

    /// Return the SPIR-V type of the value produced by the code written by
    /// `generate`. If the access does not produce a value, `Self::Output`
    /// should be `()`.
    fn result_type(&self) -> Self::Output;

    /// Construct the SPIR-V 'zero' value to be returned for an out-of-bounds
    /// access under the `ReadZeroSkipWrite` policy. If the access does not
    /// produce a value, `Self::Output` should be `()`.
    fn out_of_bounds_value(&self, ctx: &mut BlockContext<'_>) -> Self::Output;
}

/// Texel access information for an [`ImageLoad`] expression.
///
/// [`ImageLoad`]: crate::Expression::ImageLoad
struct Load {
    /// The specific opcode we'll use to perform the fetch. Storage images
    /// require `OpImageRead`, while sampled images require `OpImageFetch`.
    opcode: spirv::Op,

    /// The type id produced by the actual image access instruction.
    type_id: Word,

    /// The id of the image being accessed.
    image_id: Word,
}

impl Load {
    fn from_image_expr(
        ctx: &mut BlockContext<'_>,
        image_id: Word,
        image_class: crate::ImageClass,
        result_type_id: Word,
    ) -> Result<Load, Error> {
        let opcode = match image_class {
            crate::ImageClass::Storage { .. } => spirv::Op::ImageRead,
            crate::ImageClass::Depth { .. } | crate::ImageClass::Sampled { .. } => {
                spirv::Op::ImageFetch
            }
            crate::ImageClass::External => unimplemented!(),
        };

        // `OpImageRead` and `OpImageFetch` instructions produce vec4<f32>
        // values. Most of the time, we can just use `result_type_id` for
        // this. The exception is that `Expression::ImageLoad` from a depth
        // image produces a scalar `f32`, so in that case we need to find
        // the right SPIR-V type for the access instruction here.
        let type_id = match image_class {
            crate::ImageClass::Depth { .. } => ctx.get_numeric_type_id(NumericType::Vector {
                size: crate::VectorSize::Quad,
                scalar: crate::Scalar::F32,
            }),
            _ => result_type_id,
        };

        Ok(Load {
            opcode,
            type_id,
            image_id,
        })
    }
}

impl Access for Load {
    type Output = Word;

    /// Write an instruction to access a given texel of this image.
    fn generate(
        &self,
        id_gen: &mut IdGenerator,
        coordinates_id: Word,
        level_id: Option<Word>,
        sample_id: Option<Word>,
        block: &mut Block,
    ) -> Word {
        let texel_id = id_gen.next();
        let mut instruction = Instruction::image_fetch_or_read(
            self.opcode,
            self.type_id,
            texel_id,
            self.image_id,
            coordinates_id,
        );

        match (level_id, sample_id) {
            (None, None) => {}
            (Some(level_id), None) => {
                instruction.add_operand(spirv::ImageOperands::LOD.bits());
                instruction.add_operand(level_id);
            }
            (None, Some(sample_id)) => {
                instruction.add_operand(spirv::ImageOperands::SAMPLE.bits());
                instruction.add_operand(sample_id);
            }
            // There's no such thing as a multi-sampled mipmap.
            (Some(_), Some(_)) => unreachable!(),
        }

        block.body.push(instruction);

        texel_id
    }

    fn result_type(&self) -> Word {
        self.type_id
    }

    fn out_of_bounds_value(&self, ctx: &mut BlockContext<'_>) -> Word {
        ctx.writer.get_constant_null(self.type_id)
    }
}

/// Texel access information for a [`Store`] statement.
///
/// [`Store`]: crate::Statement::Store
struct Store {
    /// The id of the image being written to.
    image_id: Word,

    /// The value we're going to write to the texel.
    value_id: Word,
}

impl Access for Store {
    /// Stores don't generate any value.
    type Output = ();

    fn generate(
        &self,
        _id_gen: &mut IdGenerator,
        coordinates_id: Word,
        _level_id: Option<Word>,
        _sample_id: Option<Word>,
        block: &mut Block,
    ) {
        block.body.push(Instruction::image_write(
            self.image_id,
            coordinates_id,
            self.value_id,
        ));
    }

    /// Stores don't generate any value, so this just returns `()`.
    fn result_type(&self) {}

    /// Stores don't generate any value, so this just returns `()`.
    fn out_of_bounds_value(&self, _ctx: &mut BlockContext<'_>) {}
}

impl BlockContext<'_> {
    /// Extend image coordinates with an array index, if necessary.
    ///
    /// Whereas [`Expression::ImageLoad`] and [`ImageSample`] treat the array
    /// index as a separate operand from the coordinates, SPIR-V image access
    /// instructions include the array index in the `coordinates` operand. This
    /// function builds a SPIR-V coordinate vector from a Naga coordinate vector
    /// and array index, if one is supplied, and returns a `ImageCoordinates`
    /// struct describing what it built.
    ///
    /// If `array_index` is `Some(expr)`, then this function constructs a new
    /// vector that is `coordinates` with `array_index` concatenated onto the
    /// end: a `vec2` becomes a `vec3`, a scalar becomes a `vec2`, and so on.
    ///
    /// If `array_index` is `None`, then the return value uses `coordinates`
    /// unchanged. Note that, when indexing a non-arrayed 1D image, this will be
    /// a scalar value.
    ///
    /// If needed, this function generates code to convert the array index,
    /// always an integer scalar, to match the component type of `coordinates`.
    /// Naga's `ImageLoad` and SPIR-V's `OpImageRead`, `OpImageFetch`, and
    /// `OpImageWrite` all use integer coordinates, while Naga's `ImageSample`
    /// and SPIR-V's `OpImageSample...` instructions all take floating-point
    /// coordinate vectors.
    ///
    /// [`Expression::ImageLoad`]: crate::Expression::ImageLoad
    /// [`ImageSample`]: crate::Expression::ImageSample
    fn write_image_coordinates(
        &mut self,
        coordinates: Handle<crate::Expression>,
        array_index: Option<Handle<crate::Expression>>,
        block: &mut Block,
    ) -> Result<ImageCoordinates, Error> {
        use crate::TypeInner as Ti;
        use crate::VectorSize as Vs;

        let coordinates_id = self.cached[coordinates];
        let ty = &self.fun_info[coordinates].ty;
        let inner_ty = ty.inner_with(&self.ir_module.types);

        // If there's no array index, the image coordinates are exactly the
        // `coordinate` field of the `Expression::ImageLoad`. No work is needed.
        let array_index = match array_index {
            None => {
                let value_id = coordinates_id;
                let type_id = self.get_expression_type_id(ty);
                let size = match *inner_ty {
                    Ti::Scalar { .. } => None,
                    Ti::Vector { size, .. } => Some(size),
                    _ => return Err(Error::Validation("coordinate type")),
                };
                return Ok(ImageCoordinates {
                    value_id,
                    type_id,
                    size,
                });
            }
            Some(ix) => ix,
        };

        // Find the component type of `coordinates`, and figure out the size the
        // combined coordinate vector will have.
        let (component_scalar, size) = match *inner_ty {
            Ti::Scalar(scalar @ crate::Scalar { width: 4, .. }) => (scalar, Vs::Bi),
            Ti::Vector {
                scalar: scalar @ crate::Scalar { width: 4, .. },
                size: Vs::Bi,
            } => (scalar, Vs::Tri),
            Ti::Vector {
                scalar: scalar @ crate::Scalar { width: 4, .. },
                size: Vs::Tri,
            } => (scalar, Vs::Quad),
            Ti::Vector { size: Vs::Quad, .. } => {
                return Err(Error::Validation("extending vec4 coordinate"));
            }
            ref other => {
                log::error!("wrong coordinate type {other:?}");
                return Err(Error::Validation("coordinate type"));
            }
        };

        // Convert the index to the coordinate component type, if necessary.
        let array_index_id = self.cached[array_index];
        let ty = &self.fun_info[array_index].ty;
        let inner_ty = ty.inner_with(&self.ir_module.types);
        let array_index_scalar = match *inner_ty {
            Ti::Scalar(
                scalar @ crate::Scalar {
                    kind: crate::ScalarKind::Sint | crate::ScalarKind::Uint,
                    width: 4,
                },
            ) => scalar,
            _ => unreachable!("we only allow i32 and u32"),
        };
        let cast = match (component_scalar.kind, array_index_scalar.kind) {
            (crate::ScalarKind::Sint, crate::ScalarKind::Sint)
            | (crate::ScalarKind::Uint, crate::ScalarKind::Uint) => None,
            (crate::ScalarKind::Sint, crate::ScalarKind::Uint)
            | (crate::ScalarKind::Uint, crate::ScalarKind::Sint) => Some(spirv::Op::Bitcast),
            (crate::ScalarKind::Float, crate::ScalarKind::Sint) => Some(spirv::Op::ConvertSToF),
            (crate::ScalarKind::Float, crate::ScalarKind::Uint) => Some(spirv::Op::ConvertUToF),
            (crate::ScalarKind::Bool, _) => unreachable!("we don't allow bool for component"),
            (_, crate::ScalarKind::Bool | crate::ScalarKind::Float) => {
                unreachable!("we don't allow bool or float for array index")
            }
            (crate::ScalarKind::AbstractInt | crate::ScalarKind::AbstractFloat, _)
            | (_, crate::ScalarKind::AbstractInt | crate::ScalarKind::AbstractFloat) => {
                unreachable!("abstract types should never reach backends")
            }
        };
        let reconciled_array_index_id = if let Some(cast) = cast {
            let component_ty_id = self.get_numeric_type_id(NumericType::Scalar(component_scalar));
            let reconciled_id = self.gen_id();
            block.body.push(Instruction::unary(
                cast,
                component_ty_id,
                reconciled_id,
                array_index_id,
            ));
            reconciled_id
        } else {
            array_index_id
        };

        // Find the SPIR-V type for the combined coordinates/index vector.
        let type_id = self.get_numeric_type_id(NumericType::Vector {
            size,
            scalar: component_scalar,
        });

        // Schmear the coordinates and index together.
        let value_id = self.gen_id();
        block.body.push(Instruction::composite_construct(
            type_id,
            value_id,
            &[coordinates_id, reconciled_array_index_id],
        ));
        Ok(ImageCoordinates {
            value_id,
            type_id,
            size: Some(size),
        })
    }

    pub(super) fn get_handle_id(&mut self, expr_handle: Handle<crate::Expression>) -> Word {
        let id = match self.ir_function.expressions[expr_handle] {
            crate::Expression::GlobalVariable(handle) => {
                self.writer.global_variables[handle].handle_id
            }
            crate::Expression::FunctionArgument(i) => {
                self.function.parameters[i as usize].handle_id
            }
            crate::Expression::Access { .. } | crate::Expression::AccessIndex { .. } => {
                self.cached[expr_handle]
            }
            ref other => unreachable!("Unexpected image expression {:?}", other),
        };

        if id == 0 {
            unreachable!(
                "Image expression {:?} doesn't have a handle ID",
                expr_handle
            );
        }

        id
    }

    /// Generate a vector or scalar 'one' for arithmetic on `coordinates`.
    ///
    /// If `coordinates` is a scalar, return a scalar one. Otherwise, return
    /// a vector of ones.
    fn write_coordinate_one(&mut self, coordinates: &ImageCoordinates) -> Result<Word, Error> {
        let one = self.get_scope_constant(1);
        match coordinates.size {
            None => Ok(one),
            Some(vector_size) => {
                let ones = [one; 4];
                let id = self.gen_id();
                Instruction::constant_composite(
                    coordinates.type_id,
                    id,
                    &ones[..vector_size as usize],
                )
                .to_words(&mut self.writer.logical_layout.declarations);
                Ok(id)
            }
        }
    }

    /// Generate code to restrict `input` to fall between zero and one less than
    /// `size_id`.
    ///
    /// Both must be 32-bit scalar integer values, whose type is given by
    /// `type_id`. The computed value is also of type `type_id`.
    fn restrict_scalar(
        &mut self,
        type_id: Word,
        input_id: Word,
        size_id: Word,
        block: &mut Block,
    ) -> Result<Word, Error> {
        let i32_one_id = self.get_scope_constant(1);

        // Subtract one from `size` to get the largest valid value.
        let limit_id = self.gen_id();
        block.body.push(Instruction::binary(
            spirv::Op::ISub,
            type_id,
            limit_id,
            size_id,
            i32_one_id,
        ));

        // Use an unsigned minimum, to handle both positive out-of-range values
        // and negative values in a single instruction: negative values of
        // `input_id` get treated as very large positive values.
        let restricted_id = self.gen_id();
        block.body.push(Instruction::ext_inst_gl_op(
            self.writer.gl450_ext_inst_id,
            spirv::GlslStd450Op::UMin,
            type_id,
            restricted_id,
            &[input_id, limit_id],
        ));

        Ok(restricted_id)
    }

    /// Write instructions to query the size of an image.
    ///
    /// This takes care of selecting the right instruction depending on whether
    /// a level of detail parameter is present.
    fn write_coordinate_bounds(
        &mut self,
        type_id: Word,
        image_id: Word,
        level_id: Option<Word>,
        block: &mut Block,
    ) -> Word {
        let coordinate_bounds_id = self.gen_id();
        match level_id {
            Some(level_id) => {
                // A level of detail was provided, so fetch the image size for
                // that level.
                let mut inst = Instruction::image_query(
                    spirv::Op::ImageQuerySizeLod,
                    type_id,
                    coordinate_bounds_id,
                    image_id,
                );
                inst.add_operand(level_id);
                block.body.push(inst);
            }
            _ => {
                // No level of detail was given.
                block.body.push(Instruction::image_query(
                    spirv::Op::ImageQuerySize,
                    type_id,
                    coordinate_bounds_id,
                    image_id,
                ));
            }
        }

        coordinate_bounds_id
    }

    /// Write code to restrict coordinates for an image reference.
    ///
    /// First, clamp the level of detail or sample index to fall within bounds.
    /// Then, obtain the image size, possibly using the clamped level of detail.
    /// Finally, use an unsigned minimum instruction to force all coordinates
    /// into range.
    ///
    /// Return a triple `(COORDS, LEVEL, SAMPLE)`, where `COORDS` is a coordinate
    /// vector (including the array index, if any), `LEVEL` is an optional level
    /// of detail, and `SAMPLE` is an optional sample index, all guaranteed to
    /// be in-bounds for `image_id`.
    ///
    /// The result is usually a vector, but it is a scalar when indexing
    /// non-arrayed 1D images.
    fn write_restricted_coordinates(
        &mut self,
        image_id: Word,
        coordinates: ImageCoordinates,
        level_id: Option<Word>,
        sample_id: Option<Word>,
        block: &mut Block,
    ) -> Result<(Word, Option<Word>, Option<Word>), Error> {
        self.writer.require_any(
            "the `Restrict` image bounds check policy",
            &[spirv::Capability::ImageQuery],
        )?;

        let i32_type_id = self.get_numeric_type_id(NumericType::Scalar(crate::Scalar::I32));

        // If `level` is `Some`, clamp it to fall within bounds. This must
        // happen first, because we'll use it to query the image size for
        // clamping the actual coordinates.
        let level_id = level_id
            .map(|level_id| {
                // Find the number of mipmap levels in this image.
                let num_levels_id = self.gen_id();
                block.body.push(Instruction::image_query(
                    spirv::Op::ImageQueryLevels,
                    i32_type_id,
                    num_levels_id,
                    image_id,
                ));

                self.restrict_scalar(i32_type_id, level_id, num_levels_id, block)
            })
            .transpose()?;

        // If `sample_id` is `Some`, clamp it to fall within bounds.
        let sample_id = sample_id
            .map(|sample_id| {
                // Find the number of samples per texel.
                let num_samples_id = self.gen_id();
                block.body.push(Instruction::image_query(
                    spirv::Op::ImageQuerySamples,
                    i32_type_id,
                    num_samples_id,
                    image_id,
                ));

                self.restrict_scalar(i32_type_id, sample_id, num_samples_id, block)
            })
            .transpose()?;

        // Obtain the image bounds, including the array element count.
        let coordinate_bounds_id =
            self.write_coordinate_bounds(coordinates.type_id, image_id, level_id, block);

        // Compute maximum valid values from the bounds.
        let ones = self.write_coordinate_one(&coordinates)?;
        let coordinate_limit_id = self.gen_id();
        block.body.push(Instruction::binary(
            spirv::Op::ISub,
            coordinates.type_id,
            coordinate_limit_id,
            coordinate_bounds_id,
            ones,
        ));

        // Restrict the coordinates to fall within those bounds.
        //
        // Use an unsigned minimum, to handle both positive out-of-range values
        // and negative values in a single instruction: negative values of
        // `coordinates` get treated as very large positive values.
        let restricted_coordinates_id = self.gen_id();
        block.body.push(Instruction::ext_inst_gl_op(
            self.writer.gl450_ext_inst_id,
            spirv::GlslStd450Op::UMin,
            coordinates.type_id,
            restricted_coordinates_id,
            &[coordinates.value_id, coordinate_limit_id],
        ));

        Ok((restricted_coordinates_id, level_id, sample_id))
    }

    fn write_conditional_image_access<A: Access>(
        &mut self,
        image_id: Word,
        coordinates: ImageCoordinates,
        level_id: Option<Word>,
        sample_id: Option<Word>,
        block: &mut Block,
        access: &A,
    ) -> Result<A::Output, Error> {
        self.writer.require_any(
            "the `ReadZeroSkipWrite` image bounds check policy",
            &[spirv::Capability::ImageQuery],
        )?;

        let bool_type_id = self.writer.get_bool_type_id();
        let i32_type_id = self.get_numeric_type_id(NumericType::Scalar(crate::Scalar::I32));

        let null_id = access.out_of_bounds_value(self);

        let mut selection = Selection::start(block, access.result_type());

        // If `level_id` is `Some`, check whether it is within bounds. This must
        // happen first, because we'll be supplying this as an argument when we
        // query the image size.
        if let Some(level_id) = level_id {
            // Find the number of mipmap levels in this image.
            let num_levels_id = self.gen_id();
            selection.block().body.push(Instruction::image_query(
                spirv::Op::ImageQueryLevels,
                i32_type_id,
                num_levels_id,
                image_id,
            ));

            let lod_cond_id = self.gen_id();
            selection.block().body.push(Instruction::binary(
                spirv::Op::ULessThan,
                bool_type_id,
                lod_cond_id,
                level_id,
                num_levels_id,
            ));

            selection.if_true(self, lod_cond_id, null_id);
        }

        // If `sample_id` is `Some`, check whether it is in bounds.
        if let Some(sample_id) = sample_id {
            // Find the number of samples per texel.
            let num_samples_id = self.gen_id();
            selection.block().body.push(Instruction::image_query(
                spirv::Op::ImageQuerySamples,
                i32_type_id,
                num_samples_id,
                image_id,
            ));

            let samples_cond_id = self.gen_id();
            selection.block().body.push(Instruction::binary(
                spirv::Op::ULessThan,
                bool_type_id,
                samples_cond_id,
                sample_id,
                num_samples_id,
            ));

            selection.if_true(self, samples_cond_id, null_id);
        }

        // Obtain the image bounds, including any array element count.
        let coordinate_bounds_id = self.write_coordinate_bounds(
            coordinates.type_id,
            image_id,
            level_id,
            selection.block(),
        );

        // Compare the coordinates against the bounds.
        let coords_numeric_type = match coordinates.size {
            Some(size) => NumericType::Vector {
                size,
                scalar: crate::Scalar::BOOL,
            },
            None => NumericType::Scalar(crate::Scalar::BOOL),
        };
        let coords_bool_type_id = self.get_numeric_type_id(coords_numeric_type);
        let coords_conds_id = self.gen_id();
        selection.block().body.push(Instruction::binary(
            spirv::Op::ULessThan,
            coords_bool_type_id,
            coords_conds_id,
            coordinates.value_id,
            coordinate_bounds_id,
        ));

        // If the comparison above was a vector comparison, then we need to
        // check that all components of the comparison are true.
        let coords_cond_id = if coords_bool_type_id != bool_type_id {
            let id = self.gen_id();
            selection.block().body.push(Instruction::relational(
                spirv::Op::All,
                bool_type_id,
                id,
                coords_conds_id,
            ));
            id
        } else {
            coords_conds_id
        };

        selection.if_true(self, coords_cond_id, null_id);

        // All conditions are met. We can carry out the access.
        let texel_id = access.generate(
            &mut self.writer.id_gen,
            coordinates.value_id,
            level_id,
            sample_id,
            selection.block(),
        );

        // This, then, is the value of the 'true' branch.
        Ok(selection.finish(self, texel_id))
    }

    /// Generate code for an `ImageLoad` expression.
    ///
    /// The arguments are the components of an `Expression::ImageLoad` variant.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn write_image_load(
        &mut self,
        result_type_id: Word,
        image: Handle<crate::Expression>,
        coordinate: Handle<crate::Expression>,
        array_index: Option<Handle<crate::Expression>>,
        level: Option<Handle<crate::Expression>>,
        sample: Option<Handle<crate::Expression>>,
        block: &mut Block,
    ) -> Result<Word, Error> {
        let image_id = self.get_handle_id(image);
        let image_type = self.fun_info[image].ty.inner_with(&self.ir_module.types);
        let image_class = match *image_type {
            crate::TypeInner::Image { class, .. } => class,
            _ => return Err(Error::Validation("image type")),
        };

        let access = Load::from_image_expr(self, image_id, image_class, result_type_id)?;
        let coordinates = self.write_image_coordinates(coordinate, array_index, block)?;

        let level_id = level.map(|expr| self.cached[expr]);
        let sample_id = sample.map(|expr| self.cached[expr]);

        // Perform the access, according to the bounds check policy.
        let access_id = match self.writer.bounds_check_policies.image_load {
            crate::proc::BoundsCheckPolicy::Restrict => {
                let (coords, level_id, sample_id) = self.write_restricted_coordinates(
                    image_id,
                    coordinates,
                    level_id,
                    sample_id,
                    block,
                )?;
                access.generate(&mut self.writer.id_gen, coords, level_id, sample_id, block)
            }
            crate::proc::BoundsCheckPolicy::ReadZeroSkipWrite => self
                .write_conditional_image_access(
                    image_id,
                    coordinates,
                    level_id,
                    sample_id,
                    block,
                    &access,
                )?,
            crate::proc::BoundsCheckPolicy::Unchecked => access.generate(
                &mut self.writer.id_gen,
                coordinates.value_id,
                level_id,
                sample_id,
                block,
            ),
        };

        // For depth images, `ImageLoad` expressions produce a single f32,
        // whereas the SPIR-V instructions always produce a vec4. So we may have
        // to pull out the component we need.
        let result_id = if result_type_id == access.result_type() {
            // The instruction produced the type we expected. We can use
            // its result as-is.
            access_id
        } else {
            // For `ImageClass::Depth` images, SPIR-V gave us four components,
            // but we only want the first one.
            let component_id = self.gen_id();
            block.body.push(Instruction::composite_extract(
                result_type_id,
                component_id,
                access_id,
                &[0],
            ));
            component_id
        };

        Ok(result_id)
    }

    /// Generate code for an `ImageSample` expression.
    ///
    /// The arguments are the components of an `Expression::ImageSample` variant.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn write_image_sample(
        &mut self,
        result_type_id: Word,
        image: Handle<crate::Expression>,
        sampler: Handle<crate::Expression>,
        gather: Option<crate::SwizzleComponent>,
        coordinate: Handle<crate::Expression>,
        array_index: Option<Handle<crate::Expression>>,
        offset: Option<Handle<crate::Expression>>,
        level: crate::SampleLevel,
        depth_ref: Option<Handle<crate::Expression>>,
        clamp_to_edge: bool,
        block: &mut Block,
    ) -> Result<Word, Error> {
        use super::instructions::SampleLod;
        // image
        let image_id = self.get_handle_id(image);
        let image_type = self.fun_info[image].ty.handle().unwrap();
        // SPIR-V doesn't know about our `Depth` class, and it returns
        // `vec4<f32>`, so we need to grab the first component out of it.
        let needs_sub_access = match self.ir_module.types[image_type].inner {
            crate::TypeInner::Image {
                class: crate::ImageClass::Depth { .. },
                ..
            } => depth_ref.is_none() && gather.is_none(),
            _ => false,
        };
        let sample_result_type_id = if needs_sub_access {
            self.get_numeric_type_id(NumericType::Vector {
                size: crate::VectorSize::Quad,
                scalar: crate::Scalar::F32,
            })
        } else {
            result_type_id
        };

        // OpTypeSampledImage
        let image_type_id = self.get_handle_type_id(image_type);
        let sampled_image_type_id =
            self.get_type_id(LookupType::Local(LocalType::SampledImage { image_type_id }));

        let sampler_id = self.get_handle_id(sampler);

        let coordinates = self.write_image_coordinates(coordinate, array_index, block)?;
        let coordinates_id = if clamp_to_edge {
            self.writer.require_any(
                "clamp sample coordinates to edge",
                &[spirv::Capability::ImageQuery],
            )?;

            // clamp_to_edge can only be used with Level 0, and no array offset, offset,
            // depth_ref or gather. This should have been caught by validation. Rather
            // than entirely duplicate validation code here just ensure the level is
            // zero, as we rely on that to query the texture size in order to calculate
            // the clamped coordinates.
            if level != crate::SampleLevel::Zero {
                return Err(Error::Validation(
                    "ImageSample::clamp_to_edge requires SampleLevel::Zero",
                ));
            }

            // Query the size of level 0 of the texture.
            let image_size_id = self.gen_id();
            let vec2u_type_id = self.writer.get_vec2u_type_id();
            let const_zero_uint_id = self.writer.get_constant_scalar(crate::Literal::U32(0));
            let mut query_inst = Instruction::image_query(
                spirv::Op::ImageQuerySizeLod,
                vec2u_type_id,
                image_size_id,
                image_id,
            );
            query_inst.add_operand(const_zero_uint_id);
            block.body.push(query_inst);

            let image_size_f_id = self.gen_id();
            let vec2f_type_id = self.writer.get_vec2f_type_id();
            block.body.push(Instruction::unary(
                spirv::Op::ConvertUToF,
                vec2f_type_id,
                image_size_f_id,
                image_size_id,
            ));

            // Calculate the top-left and bottom-right margin for clamping to. I.e. a
            // half-texel from each side.
            let const_0_5_f32_id = self.writer.get_constant_scalar(crate::Literal::F32(0.5));
            let const_0_5_vec2f_id = self.writer.get_constant_composite(
                LookupType::Local(LocalType::Numeric(NumericType::Vector {
                    size: crate::VectorSize::Bi,
                    scalar: crate::Scalar::F32,
                })),
                &[const_0_5_f32_id, const_0_5_f32_id],
            );

            let margin_left_id = self.gen_id();
            block.body.push(Instruction::binary(
                spirv::Op::FDiv,
                vec2f_type_id,
                margin_left_id,
                const_0_5_vec2f_id,
                image_size_f_id,
            ));

            let const_1_f32_id = self.writer.get_constant_scalar(crate::Literal::F32(1.0));
            let const_1_vec2f_id = self.writer.get_constant_composite(
                LookupType::Local(LocalType::Numeric(NumericType::Vector {
                    size: crate::VectorSize::Bi,
                    scalar: crate::Scalar::F32,
                })),
                &[const_1_f32_id, const_1_f32_id],
            );

            let margin_right_id = self.gen_id();
            block.body.push(Instruction::binary(
                spirv::Op::FSub,
                vec2f_type_id,
                margin_right_id,
                const_1_vec2f_id,
                margin_left_id,
            ));

            // Clamp the coords to the calculated margins
            let clamped_coords_id = self.gen_id();
            block.body.push(Instruction::ext_inst_gl_op(
                self.writer.gl450_ext_inst_id,
                spirv::GlslStd450Op::NClamp,
                vec2f_type_id,
                clamped_coords_id,
                &[coordinates.value_id, margin_left_id, margin_right_id],
            ));

            clamped_coords_id
        } else {
            coordinates.value_id
        };

        let sampled_image_id = self.gen_id();
        block.body.push(Instruction::sampled_image(
            sampled_image_type_id,
            sampled_image_id,
            image_id,
            sampler_id,
        ));
        let id = self.gen_id();

        let depth_id = depth_ref.map(|handle| self.cached[handle]);
        let mut mask = spirv::ImageOperands::empty();
        mask.set(spirv::ImageOperands::CONST_OFFSET, offset.is_some());

        let mut main_instruction = match (level, gather) {
            (_, Some(component)) => {
                let component_id = self.get_index_constant(component as u32);
                let mut inst = Instruction::image_gather(
                    sample_result_type_id,
                    id,
                    sampled_image_id,
                    coordinates_id,
                    component_id,
                    depth_id,
                );
                if !mask.is_empty() {
                    inst.add_operand(mask.bits());
                }
                inst
            }
            (crate::SampleLevel::Zero, None) => {
                let mut inst = Instruction::image_sample(
                    sample_result_type_id,
                    id,
                    SampleLod::Explicit,
                    sampled_image_id,
                    coordinates_id,
                    depth_id,
                );

                let zero_id = self.writer.get_constant_scalar(crate::Literal::F32(0.0));

                mask |= spirv::ImageOperands::LOD;
                inst.add_operand(mask.bits());
                inst.add_operand(zero_id);

                inst
            }
            (crate::SampleLevel::Auto, None) => {
                let mut inst = Instruction::image_sample(
                    sample_result_type_id,
                    id,
                    SampleLod::Implicit,
                    sampled_image_id,
                    coordinates_id,
                    depth_id,
                );
                if !mask.is_empty() {
                    inst.add_operand(mask.bits());
                }
                inst
            }
            (crate::SampleLevel::Exact(lod_handle), None) => {
                let mut inst = Instruction::image_sample(
                    sample_result_type_id,
                    id,
                    SampleLod::Explicit,
                    sampled_image_id,
                    coordinates_id,
                    depth_id,
                );

                let mut lod_id = self.cached[lod_handle];
                // SPIR-V expects the LOD to be a float for all image classes.
                // lod_id, however, will be an integer for depth images,
                // therefore we must do a conversion.
                if matches!(
                    self.ir_module.types[image_type].inner,
                    crate::TypeInner::Image {
                        class: crate::ImageClass::Depth { .. },
                        ..
                    }
                ) {
                    let lod_f32_id = self.gen_id();
                    let f32_type_id =
                        self.get_numeric_type_id(NumericType::Scalar(crate::Scalar::F32));
                    let convert_op = match *self.fun_info[lod_handle]
                        .ty
                        .inner_with(&self.ir_module.types)
                    {
                        crate::TypeInner::Scalar(crate::Scalar {
                            kind: crate::ScalarKind::Uint,
                            width: 4,
                        }) => spirv::Op::ConvertUToF,
                        crate::TypeInner::Scalar(crate::Scalar {
                            kind: crate::ScalarKind::Sint,
                            width: 4,
                        }) => spirv::Op::ConvertSToF,
                        _ => unreachable!(),
                    };
                    block.body.push(Instruction::unary(
                        convert_op,
                        f32_type_id,
                        lod_f32_id,
                        lod_id,
                    ));
                    lod_id = lod_f32_id;
                }
                mask |= spirv::ImageOperands::LOD;
                inst.add_operand(mask.bits());
                inst.add_operand(lod_id);

                inst
            }
            (crate::SampleLevel::Bias(bias_handle), None) => {
                let mut inst = Instruction::image_sample(
                    sample_result_type_id,
                    id,
                    SampleLod::Implicit,
                    sampled_image_id,
                    coordinates_id,
                    depth_id,
                );

                let bias_id = self.cached[bias_handle];
                mask |= spirv::ImageOperands::BIAS;
                inst.add_operand(mask.bits());
                inst.add_operand(bias_id);

                inst
            }
            (crate::SampleLevel::Gradient { x, y }, None) => {
                let mut inst = Instruction::image_sample(
                    sample_result_type_id,
                    id,
                    SampleLod::Explicit,
                    sampled_image_id,
                    coordinates_id,
                    depth_id,
                );

                let x_id = self.cached[x];
                let y_id = self.cached[y];
                mask |= spirv::ImageOperands::GRAD;
                inst.add_operand(mask.bits());
                inst.add_operand(x_id);
                inst.add_operand(y_id);

                inst
            }
        };

        if let Some(offset_const) = offset {
            let offset_id = self.cached[offset_const];
            main_instruction.add_operand(offset_id);
        }

        block.body.push(main_instruction);

        let id = if needs_sub_access {
            let sub_id = self.gen_id();
            block.body.push(Instruction::composite_extract(
                result_type_id,
                sub_id,
                id,
                &[0],
            ));
            sub_id
        } else {
            id
        };

        Ok(id)
    }

    /// Generate code for an `ImageQuery` expression.
    ///
    /// The arguments are the components of an `Expression::ImageQuery` variant.
    pub(super) fn write_image_query(
        &mut self,
        result_type_id: Word,
        image: Handle<crate::Expression>,
        query: crate::ImageQuery,
        block: &mut Block,
    ) -> Result<Word, Error> {
        use crate::{ImageClass as Ic, ImageDimension as Id, ImageQuery as Iq};

        let image_id = self.get_handle_id(image);
        let image_type = self.fun_info[image].ty.handle().unwrap();
        let (dim, arrayed, class) = match self.ir_module.types[image_type].inner {
            crate::TypeInner::Image {
                dim,
                arrayed,
                class,
            } => (dim, arrayed, class),
            _ => {
                return Err(Error::Validation("image type"));
            }
        };

        self.writer
            .require_any("image queries", &[spirv::Capability::ImageQuery])?;

        let id = match query {
            Iq::Size { level } => {
                let dim_coords = match dim {
                    Id::D1 => 1,
                    Id::D2 | Id::Cube => 2,
                    Id::D3 => 3,
                };
                let array_coords = usize::from(arrayed);
                let vector_size = match dim_coords + array_coords {
                    2 => Some(crate::VectorSize::Bi),
                    3 => Some(crate::VectorSize::Tri),
                    4 => Some(crate::VectorSize::Quad),
                    _ => None,
                };
                let vector_numeric_type = match vector_size {
                    Some(size) => NumericType::Vector {
                        size,
                        scalar: crate::Scalar::U32,
                    },
                    None => NumericType::Scalar(crate::Scalar::U32),
                };

                let extended_size_type_id = self.get_numeric_type_id(vector_numeric_type);

                let (query_op, level_id) = match class {
                    Ic::Sampled { multi: true, .. }
                    | Ic::Depth { multi: true }
                    | Ic::Storage { .. } => (spirv::Op::ImageQuerySize, None),
                    _ => {
                        let level_id = match level {
                            Some(expr) => self.cached[expr],
                            None => self.get_index_constant(0),
                        };
                        (spirv::Op::ImageQuerySizeLod, Some(level_id))
                    }
                };

                // The ID of the vector returned by SPIR-V, which contains the dimensions
                // as well as the layer count.
                let id_extended = self.gen_id();
                let mut inst = Instruction::image_query(
                    query_op,
                    extended_size_type_id,
                    id_extended,
                    image_id,
                );
                if let Some(expr_id) = level_id {
                    inst.add_operand(expr_id);
                }
                block.body.push(inst);

                if result_type_id != extended_size_type_id {
                    let id = self.gen_id();
                    let components = match dim {
                        // always pick the first component, and duplicate it for all 3 dimensions
                        Id::Cube => &[0u32, 0][..],
                        _ => &[0u32, 1, 2, 3][..dim_coords],
                    };
                    block.body.push(Instruction::vector_shuffle(
                        result_type_id,
                        id,
                        id_extended,
                        id_extended,
                        components,
                    ));

                    id
                } else {
                    id_extended
                }
            }
            Iq::NumLevels => {
                let query_id = self.gen_id();
                block.body.push(Instruction::image_query(
                    spirv::Op::ImageQueryLevels,
                    result_type_id,
                    query_id,
                    image_id,
                ));

                query_id
            }
            Iq::NumLayers => {
                let vec_size = match dim {
                    Id::D1 => crate::VectorSize::Bi,
                    Id::D2 | Id::Cube => crate::VectorSize::Tri,
                    Id::D3 => crate::VectorSize::Quad,
                };
                let extended_size_type_id = self.get_numeric_type_id(NumericType::Vector {
                    size: vec_size,
                    scalar: crate::Scalar::U32,
                });
                let id_extended = self.gen_id();
                let mut inst = Instruction::image_query(
                    spirv::Op::ImageQuerySizeLod,
                    extended_size_type_id,
                    id_extended,
                    image_id,
                );
                inst.add_operand(self.get_index_constant(0));
                block.body.push(inst);

                let extract_id = self.gen_id();
                block.body.push(Instruction::composite_extract(
                    result_type_id,
                    extract_id,
                    id_extended,
                    &[vec_size as u32 - 1],
                ));

                extract_id
            }
            Iq::NumSamples => {
                let query_id = self.gen_id();
                block.body.push(Instruction::image_query(
                    spirv::Op::ImageQuerySamples,
                    result_type_id,
                    query_id,
                    image_id,
                ));

                query_id
            }
        };

        Ok(id)
    }

    pub(super) fn write_image_store(
        &mut self,
        image: Handle<crate::Expression>,
        coordinate: Handle<crate::Expression>,
        array_index: Option<Handle<crate::Expression>>,
        value: Handle<crate::Expression>,
        block: &mut Block,
    ) -> Result<(), Error> {
        let image_id = self.get_handle_id(image);
        let coordinates = self.write_image_coordinates(coordinate, array_index, block)?;
        let value_id = self.cached[value];

        let write = Store { image_id, value_id };

        match *self.fun_info[image].ty.inner_with(&self.ir_module.types) {
            crate::TypeInner::Image {
                class:
                    crate::ImageClass::Storage {
                        format: crate::StorageFormat::Bgra8Unorm,
                        ..
                    },
                ..
            } => self.writer.require_any(
                "Bgra8Unorm storage write",
                &[spirv::Capability::StorageImageWriteWithoutFormat],
            )?,
            _ => {}
        }

        write.generate(
            &mut self.writer.id_gen,
            coordinates.value_id,
            None,
            None,
            block,
        );

        Ok(())
    }

    pub(super) fn write_image_atomic(
        &mut self,
        image: Handle<crate::Expression>,
        coordinate: Handle<crate::Expression>,
        array_index: Option<Handle<crate::Expression>>,
        fun: crate::AtomicFunction,
        value: Handle<crate::Expression>,
        block: &mut Block,
    ) -> Result<(), Error> {
        let image_id = match self.ir_function.originating_global(image) {
            Some(handle) => self.writer.global_variables[handle].var_id,
            _ => return Err(Error::Validation("Unexpected image type")),
        };
        let crate::TypeInner::Image { class, .. } =
            *self.fun_info[image].ty.inner_with(&self.ir_module.types)
        else {
            return Err(Error::Validation("Invalid image type"));
        };
        let crate::ImageClass::Storage { format, .. } = class else {
            return Err(Error::Validation("Invalid image class"));
        };
        let scalar = format.into();
        let scalar_type_id = self.get_numeric_type_id(NumericType::Scalar(scalar));
        let pointer_type_id = self.get_pointer_type_id(scalar_type_id, spirv::StorageClass::Image);
        let signed = scalar.kind == crate::ScalarKind::Sint;
        if scalar.width == 8 {
            self.writer
                .require_any("64 bit image atomics", &[spirv::Capability::Int64Atomics])?;
        }
        let pointer_id = self.gen_id();
        let coordinates = self.write_image_coordinates(coordinate, array_index, block)?;
        let sample_id = self.writer.get_constant_scalar(crate::Literal::U32(0));
        block.body.push(Instruction::image_texel_pointer(
            pointer_type_id,
            pointer_id,
            image_id,
            coordinates.value_id,
            sample_id,
        ));

        let op = match fun {
            crate::AtomicFunction::Add => spirv::Op::AtomicIAdd,
            crate::AtomicFunction::Subtract => spirv::Op::AtomicISub,
            crate::AtomicFunction::And => spirv::Op::AtomicAnd,
            crate::AtomicFunction::ExclusiveOr => spirv::Op::AtomicXor,
            crate::AtomicFunction::InclusiveOr => spirv::Op::AtomicOr,
            crate::AtomicFunction::Min if signed => spirv::Op::AtomicSMin,
            crate::AtomicFunction::Min => spirv::Op::AtomicUMin,
            crate::AtomicFunction::Max if signed => spirv::Op::AtomicSMax,
            crate::AtomicFunction::Max => spirv::Op::AtomicUMax,
            crate::AtomicFunction::Exchange { .. } => {
                return Err(Error::Validation("Exchange atomics are not supported yet"))
            }
        };
        let result_type_id = self.get_expression_type_id(&self.fun_info[value].ty);
        let id = self.gen_id();
        let space = crate::AddressSpace::Handle;
        let (semantics, scope) = space.to_spirv_semantics_and_scope();
        let scope_constant_id = self.get_scope_constant(scope as u32);
        let semantics_id = self.get_index_constant(semantics.bits());
        let value_id = self.cached[value];

        block.body.push(Instruction::image_atomic(
            op,
            result_type_id,
            id,
            pointer_id,
            scope_constant_id,
            semantics_id,
            value_id,
        ));

        Ok(())
    }
}
