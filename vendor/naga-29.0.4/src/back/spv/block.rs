/*!
Implementations for `BlockContext` methods.
*/

use alloc::vec::Vec;

use arrayvec::ArrayVec;
use spirv::Word;

use super::{
    helpers::map_storage_class, index::BoundsCheckResult, selection::Selection, Block,
    BlockContext, Dimension, Error, IdGenerator, Instruction, LocalType, LookupType, NumericType,
    ResultMember, WrappedFunction, Writer, WriterFlags,
};
use crate::{
    arena::Handle, back::spv::helpers::is_uniform_matcx2_struct_member_access,
    proc::index::GuardedIndex, Statement,
};

fn get_dimension(type_inner: &crate::TypeInner) -> Dimension {
    match *type_inner {
        crate::TypeInner::Scalar(_) => Dimension::Scalar,
        crate::TypeInner::Vector { .. } => Dimension::Vector,
        crate::TypeInner::Matrix { .. } => Dimension::Matrix,
        crate::TypeInner::CooperativeMatrix { .. } => Dimension::CooperativeMatrix,
        _ => unreachable!(),
    }
}

/// How to derive the type of `OpAccessChain` instructions from Naga IR.
///
/// Most of the time, we compile Naga IR to SPIR-V instructions whose result
/// types are simply the direct SPIR-V analog of the Naga IR's. But in some
/// cases, the Naga IR and SPIR-V types need to diverge.
///
/// This enum specifies how [`BlockContext::write_access_chain`] should
/// choose a SPIR-V result type for the `OpAccessChain` it generates, based on
/// the type of the given Naga IR [`Expression`] it's generating code for.
///
/// [`Expression`]: crate::Expression
#[derive(Copy, Clone)]
enum AccessTypeAdjustment {
    /// No adjustment needed: the SPIR-V type should be the direct
    /// analog of the Naga IR expression type.
    ///
    /// For most access chains, this is the right thing: the Naga IR access
    /// expression produces a [`Pointer`] to the element / component, and the
    /// SPIR-V `OpAccessChain` instruction does the same.
    ///
    /// [`Pointer`]: crate::TypeInner::Pointer
    None,

    /// The SPIR-V type should be an `OpPointer` to the direct analog of the
    /// Naga IR expression's type.
    ///
    /// This is necessary for indexing binding arrays in the [`Handle`] address
    /// space:
    ///
    /// - In Naga IR, referencing a binding array [`GlobalVariable`] in the
    ///   [`Handle`] address space produces a value of type [`BindingArray`],
    ///   not a pointer to such. And [`Access`] and [`AccessIndex`] expressions
    ///   operate on handle binding arrays by value, and produce handle values,
    ///   not pointers.
    ///
    /// - In SPIR-V, a binding array `OpVariable` produces a pointer to an
    ///   array, and `OpAccessChain` instructions operate on pointers,
    ///   regardless of whether the elements are opaque types or not.
    ///
    /// See also the documentation for [`BindingArray`].
    ///
    /// [`Handle`]: crate::AddressSpace::Handle
    /// [`GlobalVariable`]: crate::GlobalVariable
    /// [`BindingArray`]: crate::TypeInner::BindingArray
    /// [`Access`]: crate::Expression::Access
    /// [`AccessIndex`]: crate::Expression::AccessIndex
    IntroducePointer(spirv::StorageClass),

    /// The SPIR-V type should be an `OpPointer` to the std140 layout
    /// compatible variant of the Naga IR expression's base type.
    ///
    /// This is used when accessing a type through an [`AddressSpace::Uniform`]
    /// pointer in cases where the original type is incompatible with std140
    /// layout requirements and we have therefore declared the uniform to be of
    /// an alternative std140 compliant type.
    ///
    /// [`AddressSpace::Uniform`]: crate::AddressSpace::Uniform
    UseStd140CompatType,
}

/// The results of emitting code for a left-hand-side expression.
///
/// On success, `write_access_chain` returns one of these.
enum ExpressionPointer {
    /// The pointer to the expression's value is available, as the value of the
    /// expression with the given id.
    Ready { pointer_id: Word },

    /// The access expression must be conditional on the value of `condition`, a boolean
    /// expression that is true if all indices are in bounds. If `condition` is true, then
    /// `access` is an `OpAccessChain` instruction that will compute a pointer to the
    /// expression's value. If `condition` is false, then executing `access` would be
    /// undefined behavior.
    Conditional {
        condition: Word,
        access: Instruction,
    },
}

/// The termination statement to be added to the end of the block
enum BlockExit {
    /// Generates an OpReturn (void return)
    Return,
    /// Generates an OpBranch to the specified block
    Branch {
        /// The branch target block
        target: Word,
    },
    /// Translates a loop `break if` into an `OpBranchConditional` to the
    /// merge block if true (the merge block is passed through [`LoopContext::break_id`]
    /// or else to the loop header (passed through [`preamble_id`])
    ///
    /// [`preamble_id`]: Self::BreakIf::preamble_id
    BreakIf {
        /// The condition of the `break if`
        condition: Handle<crate::Expression>,
        /// The loop header block id
        preamble_id: Word,
    },
}

/// What code generation did with a provided [`BlockExit`] value.
///
/// A function that accepts a [`BlockExit`] argument should return a value of
/// this type, to indicate whether the code it generated ended up using the
/// provided exit, or ignored it and did a non-local exit of some other kind
/// (say, [`Break`] or [`Continue`]). Some callers must use this information to
/// decide whether to generate the target block at all.
///
/// [`Break`]: Statement::Break
/// [`Continue`]: Statement::Continue
#[must_use]
enum BlockExitDisposition {
    /// The generated code used the provided `BlockExit` value. If it included a
    /// block label, the caller should be sure to actually emit the block it
    /// refers to.
    Used,

    /// The generated code did not use the provided `BlockExit` value. If it
    /// included a block label, the caller should not bother to actually emit
    /// the block it refers to, unless it knows the block is needed for
    /// something else.
    Discarded,
}

#[derive(Clone, Copy, Default)]
struct LoopContext {
    continuing_id: Option<Word>,
    break_id: Option<Word>,
}

#[derive(Debug)]
pub(crate) struct DebugInfoInner<'a> {
    pub source_code: &'a str,
    pub source_file_id: Word,
}

impl Writer {
    // Flip Y coordinate to adjust for coordinate space difference
    // between SPIR-V and our IR.
    // The `position_id` argument is a pointer to a `vecN<f32>`,
    // whose `y` component we will negate.
    fn write_epilogue_position_y_flip(
        &mut self,
        position_id: Word,
        body: &mut Vec<Instruction>,
    ) -> Result<(), Error> {
        let float_ptr_type_id = self.get_f32_pointer_type_id(spirv::StorageClass::Output);
        let index_y_id = self.get_index_constant(1);
        let access_id = self.id_gen.next();
        body.push(Instruction::access_chain(
            float_ptr_type_id,
            access_id,
            position_id,
            &[index_y_id],
        ));

        let float_type_id = self.get_f32_type_id();
        let load_id = self.id_gen.next();
        body.push(Instruction::load(float_type_id, load_id, access_id, None));

        let neg_id = self.id_gen.next();
        body.push(Instruction::unary(
            spirv::Op::FNegate,
            float_type_id,
            neg_id,
            load_id,
        ));

        body.push(Instruction::store(access_id, neg_id, None));
        Ok(())
    }

    // Clamp fragment depth between 0 and 1.
    fn write_epilogue_frag_depth_clamp(
        &mut self,
        frag_depth_id: Word,
        body: &mut Vec<Instruction>,
    ) -> Result<(), Error> {
        let float_type_id = self.get_f32_type_id();
        let zero_scalar_id = self.get_constant_scalar(crate::Literal::F32(0.0));
        let one_scalar_id = self.get_constant_scalar(crate::Literal::F32(1.0));

        let original_id = self.id_gen.next();
        body.push(Instruction::load(
            float_type_id,
            original_id,
            frag_depth_id,
            None,
        ));

        let clamp_id = self.id_gen.next();
        body.push(Instruction::ext_inst_gl_op(
            self.gl450_ext_inst_id,
            spirv::GlslStd450Op::FClamp,
            float_type_id,
            clamp_id,
            &[original_id, zero_scalar_id, one_scalar_id],
        ));

        body.push(Instruction::store(frag_depth_id, clamp_id, None));
        Ok(())
    }

    fn write_entry_point_return(
        &mut self,
        value_id: Word,
        ir_result: &crate::FunctionResult,
        result_members: &[ResultMember],
        body: &mut Vec<Instruction>,
    ) -> Result<Instruction, Error> {
        for (index, res_member) in result_members.iter().enumerate() {
            // This isn't a real builtin, and is handled elsewhere
            if res_member.built_in == Some(crate::BuiltIn::MeshTaskSize) {
                return Ok(Instruction::return_value(value_id));
            }
            let member_value_id = match ir_result.binding {
                Some(_) => value_id,
                None => {
                    let member_value_id = self.id_gen.next();
                    body.push(Instruction::composite_extract(
                        res_member.type_id,
                        member_value_id,
                        value_id,
                        &[index as u32],
                    ));
                    member_value_id
                }
            };

            self.store_io_with_f16_polyfill(body, res_member.id, member_value_id);

            match res_member.built_in {
                Some(crate::BuiltIn::Position { .. })
                    if self.flags.contains(WriterFlags::ADJUST_COORDINATE_SPACE) =>
                {
                    self.write_epilogue_position_y_flip(res_member.id, body)?;
                }
                Some(crate::BuiltIn::FragDepth)
                    if self.flags.contains(WriterFlags::CLAMP_FRAG_DEPTH) =>
                {
                    self.write_epilogue_frag_depth_clamp(res_member.id, body)?;
                }
                _ => {}
            }
        }
        Ok(Instruction::return_void())
    }
}

impl BlockContext<'_> {
    /// Generates code to ensure that a loop is bounded. Should be called immediately
    /// after adding the OpLoopMerge instruction to `block`. This function will
    /// [`consume()`](crate::back::spv::Function::consume) `block` and append its
    /// instructions to a new [`Block`], which will be returned to the caller for it to
    /// consumed prior to writing the loop body.
    ///
    /// Additionally this function will populate [`force_loop_bounding_vars`](crate::back::spv::Function::force_loop_bounding_vars),
    /// ensuring that [`Function::to_words()`](crate::back::spv::Function::to_words) will
    /// declare the required variables.
    ///
    /// See [`crate::back::msl::Writer::gen_force_bounded_loop_statements`] for details
    /// of why this is required.
    fn write_force_bounded_loop_instructions(&mut self, mut block: Block, merge_id: Word) -> Block {
        let uint_type_id = self.writer.get_u32_type_id();
        let uint2_type_id = self.writer.get_vec2u_type_id();
        let uint2_ptr_type_id = self
            .writer
            .get_vec2u_pointer_type_id(spirv::StorageClass::Function);
        let bool_type_id = self.writer.get_bool_type_id();
        let bool2_type_id = self.writer.get_vec2_bool_type_id();
        let zero_uint_const_id = self.writer.get_constant_scalar(crate::Literal::U32(0));
        let zero_uint2_const_id = self.writer.get_constant_composite(
            LookupType::Local(LocalType::Numeric(NumericType::Vector {
                size: crate::VectorSize::Bi,
                scalar: crate::Scalar::U32,
            })),
            &[zero_uint_const_id, zero_uint_const_id],
        );
        let one_uint_const_id = self.writer.get_constant_scalar(crate::Literal::U32(1));
        let max_uint_const_id = self
            .writer
            .get_constant_scalar(crate::Literal::U32(u32::MAX));
        let max_uint2_const_id = self.writer.get_constant_composite(
            LookupType::Local(LocalType::Numeric(NumericType::Vector {
                size: crate::VectorSize::Bi,
                scalar: crate::Scalar::U32,
            })),
            &[max_uint_const_id, max_uint_const_id],
        );

        let loop_counter_var_id = self.gen_id();
        if self.writer.flags.contains(WriterFlags::DEBUG) {
            self.writer
                .debugs
                .push(Instruction::name(loop_counter_var_id, "loop_bound"));
        }
        let var = super::LocalVariable {
            id: loop_counter_var_id,
            instruction: Instruction::variable(
                uint2_ptr_type_id,
                loop_counter_var_id,
                spirv::StorageClass::Function,
                Some(max_uint2_const_id),
            ),
        };
        self.function.force_loop_bounding_vars.push(var);

        let break_if_block = self.gen_id();

        self.function
            .consume(block, Instruction::branch(break_if_block));
        block = Block::new(break_if_block);

        // Load the current loop counter value from its variable. We use a vec2<u32> to
        // simulate a 64-bit counter.
        let load_id = self.gen_id();
        block.body.push(Instruction::load(
            uint2_type_id,
            load_id,
            loop_counter_var_id,
            None,
        ));

        // If both the high and low u32s have reached 0 then break. ie
        // if (all(eq(loop_counter, vec2(0)))) { break; }
        let eq_id = self.gen_id();
        block.body.push(Instruction::binary(
            spirv::Op::IEqual,
            bool2_type_id,
            eq_id,
            zero_uint2_const_id,
            load_id,
        ));
        let all_eq_id = self.gen_id();
        block.body.push(Instruction::relational(
            spirv::Op::All,
            bool_type_id,
            all_eq_id,
            eq_id,
        ));

        let inc_counter_block_id = self.gen_id();
        block.body.push(Instruction::selection_merge(
            inc_counter_block_id,
            spirv::SelectionControl::empty(),
        ));
        self.function.consume(
            block,
            Instruction::branch_conditional(all_eq_id, merge_id, inc_counter_block_id),
        );
        block = Block::new(inc_counter_block_id);

        // To simulate a 64-bit counter we always decrement the low u32, and decrement
        // the high u32 when the low u32 overflows. ie
        // counter -= vec2(select(0u, 1u, counter.y == 0), 1u);
        // Count down from u32::MAX rather than up from 0 to avoid hang on
        // certain Intel drivers. See <https://github.com/gfx-rs/wgpu/issues/7319>.
        let low_id = self.gen_id();
        block.body.push(Instruction::composite_extract(
            uint_type_id,
            low_id,
            load_id,
            &[1],
        ));
        let low_overflow_id = self.gen_id();
        block.body.push(Instruction::binary(
            spirv::Op::IEqual,
            bool_type_id,
            low_overflow_id,
            low_id,
            zero_uint_const_id,
        ));
        let carry_bit_id = self.gen_id();
        block.body.push(Instruction::select(
            uint_type_id,
            carry_bit_id,
            low_overflow_id,
            one_uint_const_id,
            zero_uint_const_id,
        ));
        let decrement_id = self.gen_id();
        block.body.push(Instruction::composite_construct(
            uint2_type_id,
            decrement_id,
            &[carry_bit_id, one_uint_const_id],
        ));
        let result_id = self.gen_id();
        block.body.push(Instruction::binary(
            spirv::Op::ISub,
            uint2_type_id,
            result_id,
            load_id,
            decrement_id,
        ));
        block
            .body
            .push(Instruction::store(loop_counter_var_id, result_id, None));

        block
    }

    /// If `pointer` refers to an access chain that contains a dynamic indexing
    /// of a two-row matrix in the [`Uniform`] address space, write code to
    /// access the value returning the ID of the result. Else return None.
    ///
    /// Two-row matrices in the uniform address space will have been declared
    /// using a alternative std140 layout compatible type, where each column is
    /// a member of a containing struct. As a result, SPIR-V is unable to access
    /// its columns with a non-constant index. To work around this limitation
    /// this function will call [`Self::write_checked_load()`] to load the
    /// matrix itself, which handles conversion from the std140 compatible type
    /// to the real matrix type. It then calls a [`wrapper function`] to obtain
    /// the correct column from the matrix, and possibly extracts a component
    /// from the vector too.
    ///
    /// [`Uniform`]: crate::AddressSpace::Uniform
    /// [`wrapper function`]: super::Writer::write_wrapped_matcx2_get_column
    fn maybe_write_uniform_matcx2_dynamic_access(
        &mut self,
        pointer: Handle<crate::Expression>,
        block: &mut Block,
    ) -> Result<Option<Word>, Error> {
        // If this access chain contains a dynamic matrix access, `pointer` is
        // either a pointer to a vector (the column) or a scalar (a component
        // within the column). In either case grab the pointer to the column,
        // and remember the component index if there is one. If `pointer`
        // points to any other type we're not interested.
        let (column_pointer, component_index) = match self.fun_info[pointer]
            .ty
            .inner_with(&self.ir_module.types)
            .pointer_base_type()
        {
            Some(resolution) => match *resolution.inner_with(&self.ir_module.types) {
                crate::TypeInner::Scalar(_) => match self.ir_function.expressions[pointer] {
                    crate::Expression::Access { base, index } => {
                        (base, Some(GuardedIndex::Expression(index)))
                    }
                    crate::Expression::AccessIndex { base, index } => {
                        (base, Some(GuardedIndex::Known(index)))
                    }
                    _ => return Ok(None),
                },
                crate::TypeInner::Vector { .. } => (pointer, None),
                _ => return Ok(None),
            },
            None => return Ok(None),
        };

        // Ensure the column is accessed with a dynamic index (i.e.
        // `Expression::Access`), and grab the pointer to the matrix.
        let crate::Expression::Access {
            base: matrix_pointer,
            index: column_index,
        } = self.ir_function.expressions[column_pointer]
        else {
            return Ok(None);
        };

        // Ensure the matrix pointer is in the uniform address space.
        let crate::TypeInner::Pointer {
            base: matrix_pointer_base_type,
            space: crate::AddressSpace::Uniform,
        } = *self.fun_info[matrix_pointer]
            .ty
            .inner_with(&self.ir_module.types)
        else {
            return Ok(None);
        };

        // Ensure the matrix pointer actually points to a Cx2 matrix.
        let crate::TypeInner::Matrix {
            columns,
            rows: rows @ crate::VectorSize::Bi,
            scalar,
        } = self.ir_module.types[matrix_pointer_base_type].inner
        else {
            return Ok(None);
        };

        let matrix_type_id = self.get_numeric_type_id(NumericType::Matrix {
            columns,
            rows,
            scalar,
        });
        let column_type_id = self.get_numeric_type_id(NumericType::Vector { size: rows, scalar });
        let component_type_id = self.get_numeric_type_id(NumericType::Scalar(scalar));
        let get_column_function_id = self.writer.wrapped_functions
            [&WrappedFunction::MatCx2GetColumn {
                r#type: matrix_pointer_base_type,
            }];

        let matrix_load_id = self.write_checked_load(
            matrix_pointer,
            block,
            AccessTypeAdjustment::None,
            matrix_type_id,
        )?;

        // Naga IR allows the index to be either an I32 or U32 but our wrapper
        // function expects a U32 argument, so convert it if required.
        let column_index_id = match *self.fun_info[column_index]
            .ty
            .inner_with(&self.ir_module.types)
        {
            crate::TypeInner::Scalar(crate::Scalar {
                kind: crate::ScalarKind::Uint,
                ..
            }) => self.cached[column_index],
            crate::TypeInner::Scalar(crate::Scalar {
                kind: crate::ScalarKind::Sint,
                ..
            }) => {
                let cast_id = self.gen_id();
                let u32_type_id = self.writer.get_u32_type_id();
                block.body.push(Instruction::unary(
                    spirv::Op::Bitcast,
                    u32_type_id,
                    cast_id,
                    self.cached[column_index],
                ));
                cast_id
            }
            _ => return Err(Error::Validation("Matrix access index must be u32 or i32")),
        };
        let column_id = self.gen_id();
        block.body.push(Instruction::function_call(
            column_type_id,
            column_id,
            get_column_function_id,
            &[matrix_load_id, column_index_id],
        ));
        let result_id = match component_index {
            Some(index) => self.write_vector_access(
                component_type_id,
                column_pointer,
                Some(column_id),
                index,
                block,
            )?,
            None => column_id,
        };

        Ok(Some(result_id))
    }

    /// If `pointer` refers to two-row matrix that is a member of a struct in
    /// the [`Uniform`] address space, write code to load the matrix returning
    /// the ID of the result. Else return None.
    ///
    /// Two-row matrices that are struct members in the uniform address space
    /// will have been decomposed such that the struct contains a separate
    /// vector member for each column of the matrix. This function will load
    /// each column separately from the containing struct, then composite them
    /// into the real matrix type.
    ///
    /// [`Uniform`]: crate::AddressSpace::Uniform
    fn maybe_write_load_uniform_matcx2_struct_member(
        &mut self,
        pointer: Handle<crate::Expression>,
        block: &mut Block,
    ) -> Result<Option<Word>, Error> {
        // Check this is a uniform address space pointer to a two-row matrix.
        let crate::TypeInner::Pointer {
            base: matrix_type,
            space: space @ crate::AddressSpace::Uniform,
        } = *self.fun_info[pointer].ty.inner_with(&self.ir_module.types)
        else {
            return Ok(None);
        };

        let crate::TypeInner::Matrix {
            columns,
            rows: rows @ crate::VectorSize::Bi,
            scalar,
        } = self.ir_module.types[matrix_type].inner
        else {
            return Ok(None);
        };

        // Check this is a struct member. Note struct members can only be
        // accessed with `AccessIndex`.
        let crate::Expression::AccessIndex {
            base: struct_pointer,
            index: member_index,
        } = self.ir_function.expressions[pointer]
        else {
            return Ok(None);
        };

        let crate::TypeInner::Pointer {
            base: struct_type, ..
        } = *self.fun_info[struct_pointer]
            .ty
            .inner_with(&self.ir_module.types)
        else {
            return Ok(None);
        };

        let crate::TypeInner::Struct { .. } = self.ir_module.types[struct_type].inner else {
            return Ok(None);
        };

        let matrix_type_id = self.get_numeric_type_id(NumericType::Matrix {
            columns,
            rows,
            scalar,
        });
        let column_type_id = self.get_numeric_type_id(NumericType::Vector { size: rows, scalar });
        let column_pointer_type_id =
            self.get_pointer_type_id(column_type_id, map_storage_class(space));
        let column0_index = self.writer.std140_compat_uniform_types[&struct_type].member_indices
            [member_index as usize];
        let column_indices = (0..columns as u32)
            .map(|c| self.get_index_constant(column0_index + c))
            .collect::<ArrayVec<_, 4>>();

        // Load each column from the struct, then composite into the real
        // matrix type.
        let load_mat_from_struct =
            |struct_pointer_id: Word, id_gen: &mut IdGenerator, block: &mut Block| -> Word {
                let mut column_ids: ArrayVec<Word, 4> = ArrayVec::new();
                for index in &column_indices {
                    let column_pointer_id = id_gen.next();
                    block.body.push(Instruction::access_chain(
                        column_pointer_type_id,
                        column_pointer_id,
                        struct_pointer_id,
                        &[*index],
                    ));
                    let column_id = id_gen.next();
                    block.body.push(Instruction::load(
                        column_type_id,
                        column_id,
                        column_pointer_id,
                        None,
                    ));
                    column_ids.push(column_id);
                }
                let result_id = id_gen.next();
                block.body.push(Instruction::composite_construct(
                    matrix_type_id,
                    result_id,
                    &column_ids,
                ));
                result_id
            };

        let result_id = match self.write_access_chain(
            struct_pointer,
            block,
            AccessTypeAdjustment::UseStd140CompatType,
        )? {
            ExpressionPointer::Ready { pointer_id } => {
                load_mat_from_struct(pointer_id, &mut self.writer.id_gen, block)
            }
            ExpressionPointer::Conditional { condition, access } => self
                .write_conditional_indexed_load(
                    matrix_type_id,
                    condition,
                    block,
                    |id_gen, block| {
                        let pointer_id = access.result_id.unwrap();
                        block.body.push(access);
                        load_mat_from_struct(pointer_id, id_gen, block)
                    },
                ),
        };

        Ok(Some(result_id))
    }

    /// Cache an expression for a value.
    pub(super) fn cache_expression_value(
        &mut self,
        expr_handle: Handle<crate::Expression>,
        block: &mut Block,
    ) -> Result<(), Error> {
        let is_named_expression = self
            .ir_function
            .named_expressions
            .contains_key(&expr_handle);

        if self.fun_info[expr_handle].ref_count == 0 && !is_named_expression {
            return Ok(());
        }

        let result_type_id = self.get_expression_type_id(&self.fun_info[expr_handle].ty);
        let id = match self.ir_function.expressions[expr_handle] {
            crate::Expression::Literal(literal) => self.writer.get_constant_scalar(literal),
            crate::Expression::Constant(handle) => {
                let init = self.ir_module.constants[handle].init;
                self.writer.constant_ids[init]
            }
            crate::Expression::Override(_) => return Err(Error::Override),
            crate::Expression::ZeroValue(_) => self.writer.get_constant_null(result_type_id),
            crate::Expression::Compose { ty, ref components } => {
                self.temp_list.clear();
                if self.expression_constness.is_const(expr_handle) {
                    self.temp_list.extend(
                        crate::proc::flatten_compose(
                            ty,
                            components,
                            &self.ir_function.expressions,
                            &self.ir_module.types,
                        )
                        .map(|component| self.cached[component]),
                    );
                    self.writer
                        .get_constant_composite(LookupType::Handle(ty), &self.temp_list)
                } else {
                    self.temp_list
                        .extend(components.iter().map(|&component| self.cached[component]));

                    let id = self.gen_id();
                    block.body.push(Instruction::composite_construct(
                        result_type_id,
                        id,
                        &self.temp_list,
                    ));
                    id
                }
            }
            crate::Expression::Splat { size, value } => {
                let value_id = self.cached[value];
                let components = &[value_id; 4][..size as usize];

                if self.expression_constness.is_const(expr_handle) {
                    let ty = self
                        .writer
                        .get_expression_lookup_type(&self.fun_info[expr_handle].ty);
                    self.writer.get_constant_composite(ty, components)
                } else {
                    let id = self.gen_id();
                    block.body.push(Instruction::composite_construct(
                        result_type_id,
                        id,
                        components,
                    ));
                    id
                }
            }
            crate::Expression::Access { base, index } => {
                let base_ty_inner = self.fun_info[base].ty.inner_with(&self.ir_module.types);
                match *base_ty_inner {
                    crate::TypeInner::Pointer { .. } | crate::TypeInner::ValuePointer { .. } => {
                        // When we have a chain of `Access` and `AccessIndex` expressions
                        // operating on pointers, we want to generate a single
                        // `OpAccessChain` instruction for the whole chain. Put off
                        // generating any code for this until we find the `Expression`
                        // that actually dereferences the pointer.
                        0
                    }
                    _ if self.function.spilled_accesses.contains(base) => {
                        // As far as Naga IR is concerned, this expression does not yield
                        // a pointer (we just checked, above), but this backend spilled it
                        // to a temporary variable, so SPIR-V thinks we're accessing it
                        // via a pointer.

                        // Since the base expression was spilled, mark this access to it
                        // as spilled, too.
                        self.function.spilled_accesses.insert(expr_handle);
                        self.maybe_access_spilled_composite(expr_handle, block, result_type_id)?
                    }
                    crate::TypeInner::Vector { .. } => self.write_vector_access(
                        result_type_id,
                        base,
                        None,
                        GuardedIndex::Expression(index),
                        block,
                    )?,
                    crate::TypeInner::Array { .. } | crate::TypeInner::Matrix { .. } => {
                        // See if `index` is known at compile time.
                        match GuardedIndex::from_expression(
                            index,
                            &self.ir_function.expressions,
                            self.ir_module,
                        ) {
                            GuardedIndex::Known(value) => {
                                // If `index` is known and in bounds, we can just use
                                // `OpCompositeExtract`.
                                //
                                // At the moment, validation rejects programs if this
                                // index is out of bounds, so we don't need bounds checks.
                                // However, that rejection is incorrect, since WGSL says
                                // that `let` bindings are not constant expressions
                                // (#6396). So eventually we will need to emulate bounds
                                // checks here.
                                let id = self.gen_id();
                                let base_id = self.cached[base];
                                block.body.push(Instruction::composite_extract(
                                    result_type_id,
                                    id,
                                    base_id,
                                    &[value],
                                ));
                                id
                            }
                            GuardedIndex::Expression(_) => {
                                // We are subscripting an array or matrix that is not
                                // behind a pointer, using an index computed at runtime.
                                // SPIR-V has no instructions that do this, so the best we
                                // can do is spill the value to a new temporary variable,
                                // at which point we can get a pointer to that and just
                                // use `OpAccessChain` in the usual way.
                                self.spill_to_internal_variable(base, block);

                                // Since the base was spilled, mark this access to it as
                                // spilled, too.
                                self.function.spilled_accesses.insert(expr_handle);
                                self.maybe_access_spilled_composite(
                                    expr_handle,
                                    block,
                                    result_type_id,
                                )?
                            }
                        }
                    }
                    crate::TypeInner::BindingArray {
                        base: binding_type, ..
                    } => {
                        // Only binding arrays in the `Handle` address space will take
                        // this path, since we handled the `Pointer` case above.
                        let result_id = match self.write_access_chain(
                            expr_handle,
                            block,
                            AccessTypeAdjustment::IntroducePointer(
                                spirv::StorageClass::UniformConstant,
                            ),
                        )? {
                            ExpressionPointer::Ready { pointer_id } => pointer_id,
                            ExpressionPointer::Conditional { .. } => {
                                return Err(Error::FeatureNotImplemented(
                                    "Texture array out-of-bounds handling",
                                ));
                            }
                        };

                        let binding_type_id = self.get_handle_type_id(binding_type);

                        let load_id = self.gen_id();
                        block.body.push(Instruction::load(
                            binding_type_id,
                            load_id,
                            result_id,
                            None,
                        ));

                        // Subsequent image operations require the image/sampler to be decorated as NonUniform
                        // if the image/sampler binding array was accessed with a non-uniform index
                        // see VUID-RuntimeSpirv-NonUniform-06274
                        if self.fun_info[index].uniformity.non_uniform_result.is_some() {
                            self.writer
                                .decorate_non_uniform_binding_array_access(load_id)?;
                        }

                        load_id
                    }
                    ref other => {
                        log::error!(
                            "Unable to access base {:?} of type {:?}",
                            self.ir_function.expressions[base],
                            other
                        );
                        return Err(Error::Validation(
                            "only vectors and arrays may be dynamically indexed by value",
                        ));
                    }
                }
            }
            crate::Expression::AccessIndex { base, index } => {
                match *self.fun_info[base].ty.inner_with(&self.ir_module.types) {
                    crate::TypeInner::Pointer { .. } | crate::TypeInner::ValuePointer { .. } => {
                        // When we have a chain of `Access` and `AccessIndex` expressions
                        // operating on pointers, we want to generate a single
                        // `OpAccessChain` instruction for the whole chain. Put off
                        // generating any code for this until we find the `Expression`
                        // that actually dereferences the pointer.
                        0
                    }
                    _ if self.function.spilled_accesses.contains(base) => {
                        // As far as Naga IR is concerned, this expression does not yield
                        // a pointer (we just checked, above), but this backend spilled it
                        // to a temporary variable, so SPIR-V thinks we're accessing it
                        // via a pointer.

                        // Since the base expression was spilled, mark this access to it
                        // as spilled, too.
                        self.function.spilled_accesses.insert(expr_handle);
                        self.maybe_access_spilled_composite(expr_handle, block, result_type_id)?
                    }
                    crate::TypeInner::Vector { .. }
                    | crate::TypeInner::Matrix { .. }
                    | crate::TypeInner::Array { .. }
                    | crate::TypeInner::Struct { .. } => {
                        // We never need bounds checks here: dynamically sized arrays can
                        // only appear behind pointers, and are thus handled by the
                        // `is_intermediate` case above. Everything else's size is
                        // statically known and checked in validation.
                        let id = self.gen_id();
                        let base_id = self.cached[base];
                        block.body.push(Instruction::composite_extract(
                            result_type_id,
                            id,
                            base_id,
                            &[index],
                        ));
                        id
                    }
                    crate::TypeInner::BindingArray {
                        base: binding_type, ..
                    } => {
                        // Only binding arrays in the `Handle` address space will take
                        // this path, since we handled the `Pointer` case above.
                        let result_id = match self.write_access_chain(
                            expr_handle,
                            block,
                            AccessTypeAdjustment::IntroducePointer(
                                spirv::StorageClass::UniformConstant,
                            ),
                        )? {
                            ExpressionPointer::Ready { pointer_id } => pointer_id,
                            ExpressionPointer::Conditional { .. } => {
                                return Err(Error::FeatureNotImplemented(
                                    "Texture array out-of-bounds handling",
                                ));
                            }
                        };

                        let binding_type_id = self.get_handle_type_id(binding_type);

                        let load_id = self.gen_id();
                        block.body.push(Instruction::load(
                            binding_type_id,
                            load_id,
                            result_id,
                            None,
                        ));

                        load_id
                    }
                    ref other => {
                        log::error!("Unable to access index of {other:?}");
                        return Err(Error::FeatureNotImplemented("access index for type"));
                    }
                }
            }
            crate::Expression::GlobalVariable(handle) => {
                self.writer.global_variables[handle].access_id
            }
            crate::Expression::Swizzle {
                size,
                vector,
                pattern,
            } => {
                let vector_id = self.cached[vector];
                self.temp_list.clear();
                for &sc in pattern[..size as usize].iter() {
                    self.temp_list.push(sc as Word);
                }
                let id = self.gen_id();
                block.body.push(Instruction::vector_shuffle(
                    result_type_id,
                    id,
                    vector_id,
                    vector_id,
                    &self.temp_list,
                ));
                id
            }
            crate::Expression::Unary { op, expr } => {
                let id = self.gen_id();
                let expr_id = self.cached[expr];
                let expr_ty_inner = self.fun_info[expr].ty.inner_with(&self.ir_module.types);

                let spirv_op = match op {
                    crate::UnaryOperator::Negate => match expr_ty_inner.scalar_kind() {
                        Some(crate::ScalarKind::Float) => spirv::Op::FNegate,
                        Some(crate::ScalarKind::Sint) => spirv::Op::SNegate,
                        _ => return Err(Error::Validation("Unexpected kind for negation")),
                    },
                    crate::UnaryOperator::LogicalNot => spirv::Op::LogicalNot,
                    crate::UnaryOperator::BitwiseNot => spirv::Op::Not,
                };

                block
                    .body
                    .push(Instruction::unary(spirv_op, result_type_id, id, expr_id));
                id
            }
            crate::Expression::Binary { op, left, right } => {
                let id = self.gen_id();
                let left_id = self.cached[left];
                let right_id = self.cached[right];
                let left_type_id = self.get_expression_type_id(&self.fun_info[left].ty);
                let right_type_id = self.get_expression_type_id(&self.fun_info[right].ty);

                if let Some(function_id) =
                    self.writer
                        .wrapped_functions
                        .get(&WrappedFunction::BinaryOp {
                            op,
                            left_type_id,
                            right_type_id,
                        })
                {
                    block.body.push(Instruction::function_call(
                        result_type_id,
                        id,
                        *function_id,
                        &[left_id, right_id],
                    ));
                } else {
                    let left_ty_inner = self.fun_info[left].ty.inner_with(&self.ir_module.types);
                    let right_ty_inner = self.fun_info[right].ty.inner_with(&self.ir_module.types);

                    let left_dimension = get_dimension(left_ty_inner);
                    let right_dimension = get_dimension(right_ty_inner);

                    let mut reverse_operands = false;

                    let spirv_op = match op {
                        crate::BinaryOperator::Add => match *left_ty_inner {
                            crate::TypeInner::Scalar(scalar)
                            | crate::TypeInner::Vector { scalar, .. } => match scalar.kind {
                                crate::ScalarKind::Float => spirv::Op::FAdd,
                                _ => spirv::Op::IAdd,
                            },
                            crate::TypeInner::Matrix {
                                columns,
                                rows,
                                scalar,
                            } => {
                                //TODO: why not just rely on `Fadd` for matrices?
                                self.write_matrix_matrix_column_op(
                                    block,
                                    id,
                                    result_type_id,
                                    left_id,
                                    right_id,
                                    columns,
                                    rows,
                                    scalar.width,
                                    spirv::Op::FAdd,
                                );

                                self.cached[expr_handle] = id;
                                return Ok(());
                            }
                            crate::TypeInner::CooperativeMatrix { .. } => spirv::Op::FAdd,
                            _ => unimplemented!(),
                        },
                        crate::BinaryOperator::Subtract => match *left_ty_inner {
                            crate::TypeInner::Scalar(scalar)
                            | crate::TypeInner::Vector { scalar, .. } => match scalar.kind {
                                crate::ScalarKind::Float => spirv::Op::FSub,
                                _ => spirv::Op::ISub,
                            },
                            crate::TypeInner::Matrix {
                                columns,
                                rows,
                                scalar,
                            } => {
                                self.write_matrix_matrix_column_op(
                                    block,
                                    id,
                                    result_type_id,
                                    left_id,
                                    right_id,
                                    columns,
                                    rows,
                                    scalar.width,
                                    spirv::Op::FSub,
                                );

                                self.cached[expr_handle] = id;
                                return Ok(());
                            }
                            crate::TypeInner::CooperativeMatrix { .. } => spirv::Op::FSub,
                            _ => unimplemented!(),
                        },
                        crate::BinaryOperator::Multiply => {
                            match (left_dimension, right_dimension) {
                                (Dimension::Scalar, Dimension::Vector) => {
                                    self.write_vector_scalar_mult(
                                        block,
                                        id,
                                        result_type_id,
                                        right_id,
                                        left_id,
                                        right_ty_inner,
                                    );

                                    self.cached[expr_handle] = id;
                                    return Ok(());
                                }
                                (Dimension::Vector, Dimension::Scalar) => {
                                    self.write_vector_scalar_mult(
                                        block,
                                        id,
                                        result_type_id,
                                        left_id,
                                        right_id,
                                        left_ty_inner,
                                    );

                                    self.cached[expr_handle] = id;
                                    return Ok(());
                                }
                                (Dimension::Vector, Dimension::Matrix) => {
                                    spirv::Op::VectorTimesMatrix
                                }
                                (Dimension::Matrix, Dimension::Scalar)
                                | (Dimension::CooperativeMatrix, Dimension::Scalar) => {
                                    spirv::Op::MatrixTimesScalar
                                }
                                (Dimension::Scalar, Dimension::Matrix)
                                | (Dimension::Scalar, Dimension::CooperativeMatrix) => {
                                    reverse_operands = true;
                                    spirv::Op::MatrixTimesScalar
                                }
                                (Dimension::Matrix, Dimension::Vector) => {
                                    spirv::Op::MatrixTimesVector
                                }
                                (Dimension::Matrix, Dimension::Matrix) => {
                                    spirv::Op::MatrixTimesMatrix
                                }
                                (Dimension::Vector, Dimension::Vector)
                                | (Dimension::Scalar, Dimension::Scalar)
                                    if left_ty_inner.scalar_kind()
                                        == Some(crate::ScalarKind::Float) =>
                                {
                                    spirv::Op::FMul
                                }
                                (Dimension::Vector, Dimension::Vector)
                                | (Dimension::Scalar, Dimension::Scalar) => spirv::Op::IMul,
                                (Dimension::CooperativeMatrix, Dimension::CooperativeMatrix)
                                //Note: technically can do `FMul` but IR doesn't have matrix per-component multiplication
                                | (Dimension::CooperativeMatrix, _)
                                | (_, Dimension::CooperativeMatrix) => {
                                    unimplemented!()
                                }
                            }
                        }
                        crate::BinaryOperator::Divide => match left_ty_inner.scalar_kind() {
                            Some(crate::ScalarKind::Sint) => spirv::Op::SDiv,
                            Some(crate::ScalarKind::Uint) => spirv::Op::UDiv,
                            Some(crate::ScalarKind::Float) => spirv::Op::FDiv,
                            _ => unimplemented!(),
                        },
                        crate::BinaryOperator::Modulo => match left_ty_inner.scalar_kind() {
                            // TODO: handle undefined behavior
                            // if right == 0 return ? see https://github.com/gpuweb/gpuweb/issues/2798
                            Some(crate::ScalarKind::Float) => spirv::Op::FRem,
                            Some(crate::ScalarKind::Sint | crate::ScalarKind::Uint) => {
                                unreachable!("Should have been handled by wrapped function")
                            }
                            _ => unimplemented!(),
                        },
                        crate::BinaryOperator::Equal => match left_ty_inner.scalar_kind() {
                            Some(crate::ScalarKind::Sint | crate::ScalarKind::Uint) => {
                                spirv::Op::IEqual
                            }
                            Some(crate::ScalarKind::Float) => spirv::Op::FOrdEqual,
                            Some(crate::ScalarKind::Bool) => spirv::Op::LogicalEqual,
                            _ => unimplemented!(),
                        },
                        crate::BinaryOperator::NotEqual => match left_ty_inner.scalar_kind() {
                            Some(crate::ScalarKind::Sint | crate::ScalarKind::Uint) => {
                                spirv::Op::INotEqual
                            }
                            Some(crate::ScalarKind::Float) => spirv::Op::FOrdNotEqual,
                            Some(crate::ScalarKind::Bool) => spirv::Op::LogicalNotEqual,
                            _ => unimplemented!(),
                        },
                        crate::BinaryOperator::Less => match left_ty_inner.scalar_kind() {
                            Some(crate::ScalarKind::Sint) => spirv::Op::SLessThan,
                            Some(crate::ScalarKind::Uint) => spirv::Op::ULessThan,
                            Some(crate::ScalarKind::Float) => spirv::Op::FOrdLessThan,
                            _ => unimplemented!(),
                        },
                        crate::BinaryOperator::LessEqual => match left_ty_inner.scalar_kind() {
                            Some(crate::ScalarKind::Sint) => spirv::Op::SLessThanEqual,
                            Some(crate::ScalarKind::Uint) => spirv::Op::ULessThanEqual,
                            Some(crate::ScalarKind::Float) => spirv::Op::FOrdLessThanEqual,
                            _ => unimplemented!(),
                        },
                        crate::BinaryOperator::Greater => match left_ty_inner.scalar_kind() {
                            Some(crate::ScalarKind::Sint) => spirv::Op::SGreaterThan,
                            Some(crate::ScalarKind::Uint) => spirv::Op::UGreaterThan,
                            Some(crate::ScalarKind::Float) => spirv::Op::FOrdGreaterThan,
                            _ => unimplemented!(),
                        },
                        crate::BinaryOperator::GreaterEqual => match left_ty_inner.scalar_kind() {
                            Some(crate::ScalarKind::Sint) => spirv::Op::SGreaterThanEqual,
                            Some(crate::ScalarKind::Uint) => spirv::Op::UGreaterThanEqual,
                            Some(crate::ScalarKind::Float) => spirv::Op::FOrdGreaterThanEqual,
                            _ => unimplemented!(),
                        },
                        crate::BinaryOperator::And => match left_ty_inner.scalar_kind() {
                            Some(crate::ScalarKind::Bool) => spirv::Op::LogicalAnd,
                            _ => spirv::Op::BitwiseAnd,
                        },
                        crate::BinaryOperator::ExclusiveOr => spirv::Op::BitwiseXor,
                        crate::BinaryOperator::InclusiveOr => match left_ty_inner.scalar_kind() {
                            Some(crate::ScalarKind::Bool) => spirv::Op::LogicalOr,
                            _ => spirv::Op::BitwiseOr,
                        },
                        crate::BinaryOperator::LogicalAnd => spirv::Op::LogicalAnd,
                        crate::BinaryOperator::LogicalOr => spirv::Op::LogicalOr,
                        crate::BinaryOperator::ShiftLeft => spirv::Op::ShiftLeftLogical,
                        crate::BinaryOperator::ShiftRight => match left_ty_inner.scalar_kind() {
                            Some(crate::ScalarKind::Sint) => spirv::Op::ShiftRightArithmetic,
                            Some(crate::ScalarKind::Uint) => spirv::Op::ShiftRightLogical,
                            _ => unimplemented!(),
                        },
                    };

                    block.body.push(Instruction::binary(
                        spirv_op,
                        result_type_id,
                        id,
                        if reverse_operands { right_id } else { left_id },
                        if reverse_operands { left_id } else { right_id },
                    ));
                }
                id
            }
            crate::Expression::Math {
                fun,
                arg,
                arg1,
                arg2,
                arg3,
            } => {
                use crate::MathFunction as Mf;
                enum MathOp {
                    Ext(spirv::GlslStd450Op),
                    Custom(Instruction),
                }

                let arg0_id = self.cached[arg];
                let arg_ty = self.fun_info[arg].ty.inner_with(&self.ir_module.types);
                let arg_scalar_kind = arg_ty.scalar_kind();
                let arg1_id = match arg1 {
                    Some(handle) => self.cached[handle],
                    None => 0,
                };
                let arg2_id = match arg2 {
                    Some(handle) => self.cached[handle],
                    None => 0,
                };
                let arg3_id = match arg3 {
                    Some(handle) => self.cached[handle],
                    None => 0,
                };

                let id = self.gen_id();
                let math_op = match fun {
                    // comparison
                    Mf::Abs => {
                        match arg_scalar_kind {
                            Some(crate::ScalarKind::Float) => {
                                MathOp::Ext(spirv::GlslStd450Op::FAbs)
                            }
                            Some(crate::ScalarKind::Sint) => MathOp::Ext(spirv::GlslStd450Op::SAbs),
                            Some(crate::ScalarKind::Uint) => {
                                MathOp::Custom(Instruction::unary(
                                    spirv::Op::CopyObject, // do nothing
                                    result_type_id,
                                    id,
                                    arg0_id,
                                ))
                            }
                            other => unimplemented!("Unexpected abs({:?})", other),
                        }
                    }
                    Mf::Min => MathOp::Ext(match arg_scalar_kind {
                        Some(crate::ScalarKind::Float) => spirv::GlslStd450Op::FMin,
                        Some(crate::ScalarKind::Sint) => spirv::GlslStd450Op::SMin,
                        Some(crate::ScalarKind::Uint) => spirv::GlslStd450Op::UMin,
                        other => unimplemented!("Unexpected min({:?})", other),
                    }),
                    Mf::Max => MathOp::Ext(match arg_scalar_kind {
                        Some(crate::ScalarKind::Float) => spirv::GlslStd450Op::FMax,
                        Some(crate::ScalarKind::Sint) => spirv::GlslStd450Op::SMax,
                        Some(crate::ScalarKind::Uint) => spirv::GlslStd450Op::UMax,
                        other => unimplemented!("Unexpected max({:?})", other),
                    }),
                    Mf::Clamp => match arg_scalar_kind {
                        // Clamp is undefined if min > max. In practice this means it can use a median-of-three
                        // instruction to determine the value. This is fine according to the WGSL spec for float
                        // clamp, but integer clamp _must_ use min-max. As such we write out min/max.
                        Some(crate::ScalarKind::Float) => MathOp::Ext(spirv::GlslStd450Op::FClamp),
                        Some(_) => {
                            let (min_op, max_op) = match arg_scalar_kind {
                                Some(crate::ScalarKind::Sint) => {
                                    (spirv::GlslStd450Op::SMin, spirv::GlslStd450Op::SMax)
                                }
                                Some(crate::ScalarKind::Uint) => {
                                    (spirv::GlslStd450Op::UMin, spirv::GlslStd450Op::UMax)
                                }
                                _ => unreachable!(),
                            };

                            let max_id = self.gen_id();
                            block.body.push(Instruction::ext_inst_gl_op(
                                self.writer.gl450_ext_inst_id,
                                max_op,
                                result_type_id,
                                max_id,
                                &[arg0_id, arg1_id],
                            ));

                            MathOp::Custom(Instruction::ext_inst_gl_op(
                                self.writer.gl450_ext_inst_id,
                                min_op,
                                result_type_id,
                                id,
                                &[max_id, arg2_id],
                            ))
                        }
                        other => unimplemented!("Unexpected max({:?})", other),
                    },
                    Mf::Saturate => {
                        let (maybe_size, scalar) = match *arg_ty {
                            crate::TypeInner::Vector { size, scalar } => (Some(size), scalar),
                            crate::TypeInner::Scalar(scalar) => (None, scalar),
                            ref other => unimplemented!("Unexpected saturate({:?})", other),
                        };
                        let scalar = crate::Scalar::float(scalar.width);
                        let mut arg1_id = self.writer.get_constant_scalar_with(0, scalar)?;
                        let mut arg2_id = self.writer.get_constant_scalar_with(1, scalar)?;

                        if let Some(size) = maybe_size {
                            let ty =
                                LocalType::Numeric(NumericType::Vector { size, scalar }).into();

                            self.temp_list.clear();
                            self.temp_list.resize(size as _, arg1_id);

                            arg1_id = self.writer.get_constant_composite(ty, &self.temp_list);

                            self.temp_list.fill(arg2_id);

                            arg2_id = self.writer.get_constant_composite(ty, &self.temp_list);
                        }

                        MathOp::Custom(Instruction::ext_inst_gl_op(
                            self.writer.gl450_ext_inst_id,
                            spirv::GlslStd450Op::FClamp,
                            result_type_id,
                            id,
                            &[arg0_id, arg1_id, arg2_id],
                        ))
                    }
                    // trigonometry
                    Mf::Sin => MathOp::Ext(spirv::GlslStd450Op::Sin),
                    Mf::Sinh => MathOp::Ext(spirv::GlslStd450Op::Sinh),
                    Mf::Asin => MathOp::Ext(spirv::GlslStd450Op::Asin),
                    Mf::Cos => MathOp::Ext(spirv::GlslStd450Op::Cos),
                    Mf::Cosh => MathOp::Ext(spirv::GlslStd450Op::Cosh),
                    Mf::Acos => MathOp::Ext(spirv::GlslStd450Op::Acos),
                    Mf::Tan => MathOp::Ext(spirv::GlslStd450Op::Tan),
                    Mf::Tanh => MathOp::Ext(spirv::GlslStd450Op::Tanh),
                    Mf::Atan => MathOp::Ext(spirv::GlslStd450Op::Atan),
                    Mf::Atan2 => MathOp::Ext(spirv::GlslStd450Op::Atan2),
                    Mf::Asinh => MathOp::Ext(spirv::GlslStd450Op::Asinh),
                    Mf::Acosh => MathOp::Ext(spirv::GlslStd450Op::Acosh),
                    Mf::Atanh => MathOp::Ext(spirv::GlslStd450Op::Atanh),
                    Mf::Radians => MathOp::Ext(spirv::GlslStd450Op::Radians),
                    Mf::Degrees => MathOp::Ext(spirv::GlslStd450Op::Degrees),
                    // decomposition
                    Mf::Ceil => MathOp::Ext(spirv::GlslStd450Op::Ceil),
                    Mf::Round => MathOp::Ext(spirv::GlslStd450Op::RoundEven),
                    Mf::Floor => MathOp::Ext(spirv::GlslStd450Op::Floor),
                    Mf::Fract => MathOp::Ext(spirv::GlslStd450Op::Fract),
                    Mf::Trunc => MathOp::Ext(spirv::GlslStd450Op::Trunc),
                    Mf::Modf => MathOp::Ext(spirv::GlslStd450Op::ModfStruct),
                    Mf::Frexp => MathOp::Ext(spirv::GlslStd450Op::FrexpStruct),
                    Mf::Ldexp => MathOp::Ext(spirv::GlslStd450Op::Ldexp),
                    // geometry
                    Mf::Dot => match *self.fun_info[arg].ty.inner_with(&self.ir_module.types) {
                        crate::TypeInner::Vector {
                            scalar:
                                crate::Scalar {
                                    kind: crate::ScalarKind::Float,
                                    ..
                                },
                            ..
                        } => MathOp::Custom(Instruction::binary(
                            spirv::Op::Dot,
                            result_type_id,
                            id,
                            arg0_id,
                            arg1_id,
                        )),
                        // TODO: consider using integer dot product if VK_KHR_shader_integer_dot_product is available
                        crate::TypeInner::Vector { size, .. } => {
                            self.write_dot_product(
                                id,
                                result_type_id,
                                arg0_id,
                                arg1_id,
                                size as u32,
                                block,
                                |result_id, composite_id, index| {
                                    Instruction::composite_extract(
                                        result_type_id,
                                        result_id,
                                        composite_id,
                                        &[index],
                                    )
                                },
                            );
                            self.cached[expr_handle] = id;
                            return Ok(());
                        }
                        _ => unreachable!(
                            "Correct TypeInner for dot product should be already validated"
                        ),
                    },
                    fun @ (Mf::Dot4I8Packed | Mf::Dot4U8Packed) => {
                        if self
                            .writer
                            .require_all(&[
                                spirv::Capability::DotProduct,
                                spirv::Capability::DotProductInput4x8BitPacked,
                            ])
                            .is_ok()
                        {
                            // Write optimized code using `PackedVectorFormat4x8Bit`.
                            if self.writer.lang_version() < (1, 6) {
                                // SPIR-V 1.6 supports the required capabilities natively, so the extension
                                // is only required for earlier versions. See right column of
                                // <https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#OpSDot>.
                                self.writer.use_extension("SPV_KHR_integer_dot_product");
                            }

                            let op = match fun {
                                Mf::Dot4I8Packed => spirv::Op::SDot,
                                Mf::Dot4U8Packed => spirv::Op::UDot,
                                _ => unreachable!(),
                            };

                            block.body.push(Instruction::ternary(
                                op,
                                result_type_id,
                                id,
                                arg0_id,
                                arg1_id,
                                spirv::PackedVectorFormat::PackedVectorFormat4x8Bit as Word,
                            ));
                        } else {
                            // Fall back to a polyfill since `PackedVectorFormat4x8Bit` is not available.
                            let (extract_op, arg0_id, arg1_id) = match fun {
                                Mf::Dot4U8Packed => (spirv::Op::BitFieldUExtract, arg0_id, arg1_id),
                                Mf::Dot4I8Packed => {
                                    // Convert both packed arguments to signed integers so that we can apply the
                                    // `BitFieldSExtract` operation on them in `write_dot_product` below.
                                    let new_arg0_id = self.gen_id();
                                    block.body.push(Instruction::unary(
                                        spirv::Op::Bitcast,
                                        result_type_id,
                                        new_arg0_id,
                                        arg0_id,
                                    ));

                                    let new_arg1_id = self.gen_id();
                                    block.body.push(Instruction::unary(
                                        spirv::Op::Bitcast,
                                        result_type_id,
                                        new_arg1_id,
                                        arg1_id,
                                    ));

                                    (spirv::Op::BitFieldSExtract, new_arg0_id, new_arg1_id)
                                }
                                _ => unreachable!(),
                            };

                            let eight = self.writer.get_constant_scalar(crate::Literal::U32(8));

                            const VEC_LENGTH: u8 = 4;
                            let bit_shifts: [_; VEC_LENGTH as usize] =
                                core::array::from_fn(|index| {
                                    self.writer
                                        .get_constant_scalar(crate::Literal::U32(index as u32 * 8))
                                });

                            self.write_dot_product(
                                id,
                                result_type_id,
                                arg0_id,
                                arg1_id,
                                VEC_LENGTH as Word,
                                block,
                                |result_id, composite_id, index| {
                                    Instruction::ternary(
                                        extract_op,
                                        result_type_id,
                                        result_id,
                                        composite_id,
                                        bit_shifts[index as usize],
                                        eight,
                                    )
                                },
                            );
                        }

                        self.cached[expr_handle] = id;
                        return Ok(());
                    }
                    Mf::Outer => MathOp::Custom(Instruction::binary(
                        spirv::Op::OuterProduct,
                        result_type_id,
                        id,
                        arg0_id,
                        arg1_id,
                    )),
                    Mf::Cross => MathOp::Ext(spirv::GlslStd450Op::Cross),
                    Mf::Distance => MathOp::Ext(spirv::GlslStd450Op::Distance),
                    Mf::Length => MathOp::Ext(spirv::GlslStd450Op::Length),
                    Mf::Normalize => MathOp::Ext(spirv::GlslStd450Op::Normalize),
                    Mf::FaceForward => MathOp::Ext(spirv::GlslStd450Op::FaceForward),
                    Mf::Reflect => MathOp::Ext(spirv::GlslStd450Op::Reflect),
                    Mf::Refract => MathOp::Ext(spirv::GlslStd450Op::Refract),
                    // exponent
                    Mf::Exp => MathOp::Ext(spirv::GlslStd450Op::Exp),
                    Mf::Exp2 => MathOp::Ext(spirv::GlslStd450Op::Exp2),
                    Mf::Log => MathOp::Ext(spirv::GlslStd450Op::Log),
                    Mf::Log2 => MathOp::Ext(spirv::GlslStd450Op::Log2),
                    Mf::Pow => MathOp::Ext(spirv::GlslStd450Op::Pow),
                    // computational
                    Mf::Sign => MathOp::Ext(match arg_scalar_kind {
                        Some(crate::ScalarKind::Float) => spirv::GlslStd450Op::FSign,
                        Some(crate::ScalarKind::Sint) => spirv::GlslStd450Op::SSign,
                        other => unimplemented!("Unexpected sign({:?})", other),
                    }),
                    Mf::Fma => MathOp::Ext(spirv::GlslStd450Op::Fma),
                    Mf::Mix => {
                        let selector = arg2.unwrap();
                        let selector_ty =
                            self.fun_info[selector].ty.inner_with(&self.ir_module.types);
                        match (arg_ty, selector_ty) {
                            // if the selector is a scalar, we need to splat it
                            (
                                &crate::TypeInner::Vector { size, .. },
                                &crate::TypeInner::Scalar(scalar),
                            ) => {
                                let selector_type_id =
                                    self.get_numeric_type_id(NumericType::Vector { size, scalar });
                                self.temp_list.clear();
                                self.temp_list.resize(size as usize, arg2_id);

                                let selector_id = self.gen_id();
                                block.body.push(Instruction::composite_construct(
                                    selector_type_id,
                                    selector_id,
                                    &self.temp_list,
                                ));

                                MathOp::Custom(Instruction::ext_inst_gl_op(
                                    self.writer.gl450_ext_inst_id,
                                    spirv::GlslStd450Op::FMix,
                                    result_type_id,
                                    id,
                                    &[arg0_id, arg1_id, selector_id],
                                ))
                            }
                            _ => MathOp::Ext(spirv::GlslStd450Op::FMix),
                        }
                    }
                    Mf::Step => MathOp::Ext(spirv::GlslStd450Op::Step),
                    Mf::SmoothStep => MathOp::Ext(spirv::GlslStd450Op::SmoothStep),
                    Mf::Sqrt => MathOp::Ext(spirv::GlslStd450Op::Sqrt),
                    Mf::InverseSqrt => MathOp::Ext(spirv::GlslStd450Op::InverseSqrt),
                    Mf::Inverse => MathOp::Ext(spirv::GlslStd450Op::MatrixInverse),
                    Mf::Transpose => MathOp::Custom(Instruction::unary(
                        spirv::Op::Transpose,
                        result_type_id,
                        id,
                        arg0_id,
                    )),
                    Mf::Determinant => MathOp::Ext(spirv::GlslStd450Op::Determinant),
                    Mf::QuantizeToF16 => MathOp::Custom(Instruction::unary(
                        spirv::Op::QuantizeToF16,
                        result_type_id,
                        id,
                        arg0_id,
                    )),
                    Mf::ReverseBits => MathOp::Custom(Instruction::unary(
                        spirv::Op::BitReverse,
                        result_type_id,
                        id,
                        arg0_id,
                    )),
                    Mf::CountTrailingZeros => {
                        let uint_id = match *arg_ty {
                            crate::TypeInner::Vector { size, scalar } => {
                                let ty =
                                    LocalType::Numeric(NumericType::Vector { size, scalar }).into();

                                self.temp_list.clear();
                                self.temp_list.resize(
                                    size as _,
                                    self.writer
                                        .get_constant_scalar_with(scalar.width * 8, scalar)?,
                                );

                                self.writer.get_constant_composite(ty, &self.temp_list)
                            }
                            crate::TypeInner::Scalar(scalar) => self
                                .writer
                                .get_constant_scalar_with(scalar.width * 8, scalar)?,
                            _ => unreachable!(),
                        };

                        let lsb_id = self.gen_id();
                        block.body.push(Instruction::ext_inst_gl_op(
                            self.writer.gl450_ext_inst_id,
                            spirv::GlslStd450Op::FindILsb,
                            result_type_id,
                            lsb_id,
                            &[arg0_id],
                        ));

                        MathOp::Custom(Instruction::ext_inst_gl_op(
                            self.writer.gl450_ext_inst_id,
                            spirv::GlslStd450Op::UMin,
                            result_type_id,
                            id,
                            &[uint_id, lsb_id],
                        ))
                    }
                    Mf::CountLeadingZeros => {
                        let (int_type_id, int_id, width) = match *arg_ty {
                            crate::TypeInner::Vector { size, scalar } => {
                                let ty =
                                    LocalType::Numeric(NumericType::Vector { size, scalar }).into();

                                self.temp_list.clear();
                                self.temp_list.resize(
                                    size as _,
                                    self.writer
                                        .get_constant_scalar_with(scalar.width * 8 - 1, scalar)?,
                                );

                                (
                                    self.get_type_id(ty),
                                    self.writer.get_constant_composite(ty, &self.temp_list),
                                    scalar.width,
                                )
                            }
                            crate::TypeInner::Scalar(scalar) => (
                                self.get_numeric_type_id(NumericType::Scalar(scalar)),
                                self.writer
                                    .get_constant_scalar_with(scalar.width * 8 - 1, scalar)?,
                                scalar.width,
                            ),
                            _ => unreachable!(),
                        };

                        if width != 4 {
                            unreachable!("This is validated out until a polyfill is implemented. https://github.com/gfx-rs/wgpu/issues/5276");
                        };

                        let msb_id = self.gen_id();
                        block.body.push(Instruction::ext_inst_gl_op(
                            self.writer.gl450_ext_inst_id,
                            if width != 4 {
                                spirv::GlslStd450Op::FindILsb
                            } else {
                                spirv::GlslStd450Op::FindUMsb
                            },
                            int_type_id,
                            msb_id,
                            &[arg0_id],
                        ));

                        MathOp::Custom(Instruction::binary(
                            spirv::Op::ISub,
                            result_type_id,
                            id,
                            int_id,
                            msb_id,
                        ))
                    }
                    Mf::CountOneBits => MathOp::Custom(Instruction::unary(
                        spirv::Op::BitCount,
                        result_type_id,
                        id,
                        arg0_id,
                    )),
                    Mf::ExtractBits => {
                        let op = match arg_scalar_kind {
                            Some(crate::ScalarKind::Uint) => spirv::Op::BitFieldUExtract,
                            Some(crate::ScalarKind::Sint) => spirv::Op::BitFieldSExtract,
                            other => unimplemented!("Unexpected sign({:?})", other),
                        };

                        // The behavior of ExtractBits is undefined when offset + count > bit_width. We need
                        // to first sanitize the offset and count first. If we don't do this, AMD and Intel
                        // will return out-of-spec values if the extracted range is not within the bit width.
                        //
                        // This encodes the exact formula specified by the wgsl spec:
                        // https://gpuweb.github.io/gpuweb/wgsl/#extractBits-unsigned-builtin
                        //
                        // w = sizeof(x) * 8
                        // o = min(offset, w)
                        // tmp = w - o
                        // c = min(count, tmp)
                        //
                        // bitfieldExtract(x, o, c)

                        let bit_width = arg_ty.scalar_width().unwrap() * 8;
                        let width_constant = self
                            .writer
                            .get_constant_scalar(crate::Literal::U32(bit_width as u32));

                        let u32_type =
                            self.get_numeric_type_id(NumericType::Scalar(crate::Scalar::U32));

                        // o = min(offset, w)
                        let offset_id = self.gen_id();
                        block.body.push(Instruction::ext_inst_gl_op(
                            self.writer.gl450_ext_inst_id,
                            spirv::GlslStd450Op::UMin,
                            u32_type,
                            offset_id,
                            &[arg1_id, width_constant],
                        ));

                        // tmp = w - o
                        let max_count_id = self.gen_id();
                        block.body.push(Instruction::binary(
                            spirv::Op::ISub,
                            u32_type,
                            max_count_id,
                            width_constant,
                            offset_id,
                        ));

                        // c = min(count, tmp)
                        let count_id = self.gen_id();
                        block.body.push(Instruction::ext_inst_gl_op(
                            self.writer.gl450_ext_inst_id,
                            spirv::GlslStd450Op::UMin,
                            u32_type,
                            count_id,
                            &[arg2_id, max_count_id],
                        ));

                        MathOp::Custom(Instruction::ternary(
                            op,
                            result_type_id,
                            id,
                            arg0_id,
                            offset_id,
                            count_id,
                        ))
                    }
                    Mf::InsertBits => {
                        // The behavior of InsertBits has the same undefined behavior as ExtractBits.

                        let bit_width = arg_ty.scalar_width().unwrap() * 8;
                        let width_constant = self
                            .writer
                            .get_constant_scalar(crate::Literal::U32(bit_width as u32));

                        let u32_type =
                            self.get_numeric_type_id(NumericType::Scalar(crate::Scalar::U32));

                        // o = min(offset, w)
                        let offset_id = self.gen_id();
                        block.body.push(Instruction::ext_inst_gl_op(
                            self.writer.gl450_ext_inst_id,
                            spirv::GlslStd450Op::UMin,
                            u32_type,
                            offset_id,
                            &[arg2_id, width_constant],
                        ));

                        // tmp = w - o
                        let max_count_id = self.gen_id();
                        block.body.push(Instruction::binary(
                            spirv::Op::ISub,
                            u32_type,
                            max_count_id,
                            width_constant,
                            offset_id,
                        ));

                        // c = min(count, tmp)
                        let count_id = self.gen_id();
                        block.body.push(Instruction::ext_inst_gl_op(
                            self.writer.gl450_ext_inst_id,
                            spirv::GlslStd450Op::UMin,
                            u32_type,
                            count_id,
                            &[arg3_id, max_count_id],
                        ));

                        MathOp::Custom(Instruction::quaternary(
                            spirv::Op::BitFieldInsert,
                            result_type_id,
                            id,
                            arg0_id,
                            arg1_id,
                            offset_id,
                            count_id,
                        ))
                    }
                    Mf::FirstTrailingBit => MathOp::Ext(spirv::GlslStd450Op::FindILsb),
                    Mf::FirstLeadingBit => {
                        if arg_ty.scalar_width() == Some(4) {
                            let thing = match arg_scalar_kind {
                                Some(crate::ScalarKind::Uint) => spirv::GlslStd450Op::FindUMsb,
                                Some(crate::ScalarKind::Sint) => spirv::GlslStd450Op::FindSMsb,
                                other => unimplemented!("Unexpected firstLeadingBit({:?})", other),
                            };
                            MathOp::Ext(thing)
                        } else {
                            unreachable!("This is validated out until a polyfill is implemented. https://github.com/gfx-rs/wgpu/issues/5276");
                        }
                    }
                    Mf::Pack4x8unorm => MathOp::Ext(spirv::GlslStd450Op::PackUnorm4x8),
                    Mf::Pack4x8snorm => MathOp::Ext(spirv::GlslStd450Op::PackSnorm4x8),
                    Mf::Pack2x16float => MathOp::Ext(spirv::GlslStd450Op::PackHalf2x16),
                    Mf::Pack2x16unorm => MathOp::Ext(spirv::GlslStd450Op::PackUnorm2x16),
                    Mf::Pack2x16snorm => MathOp::Ext(spirv::GlslStd450Op::PackSnorm2x16),
                    fun @ (Mf::Pack4xI8 | Mf::Pack4xU8 | Mf::Pack4xI8Clamp | Mf::Pack4xU8Clamp) => {
                        let is_signed = matches!(fun, Mf::Pack4xI8 | Mf::Pack4xI8Clamp);
                        let should_clamp = matches!(fun, Mf::Pack4xI8Clamp | Mf::Pack4xU8Clamp);

                        let last_instruction =
                            if self.writer.require_all(&[spirv::Capability::Int8]).is_ok() {
                                self.write_pack4x8_optimized(
                                    block,
                                    result_type_id,
                                    arg0_id,
                                    id,
                                    is_signed,
                                    should_clamp,
                                )
                            } else {
                                self.write_pack4x8_polyfill(
                                    block,
                                    result_type_id,
                                    arg0_id,
                                    id,
                                    is_signed,
                                    should_clamp,
                                )
                            };

                        MathOp::Custom(last_instruction)
                    }
                    Mf::Unpack4x8unorm => MathOp::Ext(spirv::GlslStd450Op::UnpackUnorm4x8),
                    Mf::Unpack4x8snorm => MathOp::Ext(spirv::GlslStd450Op::UnpackSnorm4x8),
                    Mf::Unpack2x16float => MathOp::Ext(spirv::GlslStd450Op::UnpackHalf2x16),
                    Mf::Unpack2x16unorm => MathOp::Ext(spirv::GlslStd450Op::UnpackUnorm2x16),
                    Mf::Unpack2x16snorm => MathOp::Ext(spirv::GlslStd450Op::UnpackSnorm2x16),
                    fun @ (Mf::Unpack4xI8 | Mf::Unpack4xU8) => {
                        let is_signed = matches!(fun, Mf::Unpack4xI8);

                        let last_instruction =
                            if self.writer.require_all(&[spirv::Capability::Int8]).is_ok() {
                                self.write_unpack4x8_optimized(
                                    block,
                                    result_type_id,
                                    arg0_id,
                                    id,
                                    is_signed,
                                )
                            } else {
                                self.write_unpack4x8_polyfill(
                                    block,
                                    result_type_id,
                                    arg0_id,
                                    id,
                                    is_signed,
                                )
                            };

                        MathOp::Custom(last_instruction)
                    }
                };

                block.body.push(match math_op {
                    MathOp::Ext(op) => Instruction::ext_inst_gl_op(
                        self.writer.gl450_ext_inst_id,
                        op,
                        result_type_id,
                        id,
                        &[arg0_id, arg1_id, arg2_id, arg3_id][..fun.argument_count()],
                    ),
                    MathOp::Custom(inst) => inst,
                });
                id
            }
            crate::Expression::LocalVariable(variable) => {
                if let Some(rq_tracker) = self
                    .function
                    .ray_query_initialization_tracker_variables
                    .get(&variable)
                {
                    self.ray_query_tracker_expr.insert(
                        expr_handle,
                        super::RayQueryTrackers {
                            initialized_tracker: rq_tracker.id,
                            t_max_tracker: self
                                .function
                                .ray_query_t_max_tracker_variables
                                .get(&variable)
                                .expect("Both trackers are set at the same time.")
                                .id,
                        },
                    );
                }
                self.function.variables[&variable].id
            }
            crate::Expression::Load { pointer } => {
                self.write_checked_load(pointer, block, AccessTypeAdjustment::None, result_type_id)?
            }
            crate::Expression::FunctionArgument(index) => self.function.parameter_id(index),
            crate::Expression::CallResult(_)
            | crate::Expression::AtomicResult { .. }
            | crate::Expression::WorkGroupUniformLoadResult { .. }
            | crate::Expression::RayQueryProceedResult
            | crate::Expression::SubgroupBallotResult
            | crate::Expression::SubgroupOperationResult { .. } => self.cached[expr_handle],
            crate::Expression::As {
                expr,
                kind,
                convert,
            } => self.write_as_expression(expr, convert, kind, block, result_type_id)?,
            crate::Expression::ImageLoad {
                image,
                coordinate,
                array_index,
                sample,
                level,
            } => self.write_image_load(
                result_type_id,
                image,
                coordinate,
                array_index,
                level,
                sample,
                block,
            )?,
            crate::Expression::ImageSample {
                image,
                sampler,
                gather,
                coordinate,
                array_index,
                offset,
                level,
                depth_ref,
                clamp_to_edge,
            } => self.write_image_sample(
                result_type_id,
                image,
                sampler,
                gather,
                coordinate,
                array_index,
                offset,
                level,
                depth_ref,
                clamp_to_edge,
                block,
            )?,
            crate::Expression::Select {
                condition,
                accept,
                reject,
            } => {
                let id = self.gen_id();
                let mut condition_id = self.cached[condition];
                let accept_id = self.cached[accept];
                let reject_id = self.cached[reject];

                let condition_ty = self.fun_info[condition]
                    .ty
                    .inner_with(&self.ir_module.types);
                let object_ty = self.fun_info[accept].ty.inner_with(&self.ir_module.types);

                if let (
                    &crate::TypeInner::Scalar(
                        condition_scalar @ crate::Scalar {
                            kind: crate::ScalarKind::Bool,
                            ..
                        },
                    ),
                    &crate::TypeInner::Vector { size, .. },
                ) = (condition_ty, object_ty)
                {
                    self.temp_list.clear();
                    self.temp_list.resize(size as usize, condition_id);

                    let bool_vector_type_id = self.get_numeric_type_id(NumericType::Vector {
                        size,
                        scalar: condition_scalar,
                    });

                    let id = self.gen_id();
                    block.body.push(Instruction::composite_construct(
                        bool_vector_type_id,
                        id,
                        &self.temp_list,
                    ));
                    condition_id = id
                }

                let instruction =
                    Instruction::select(result_type_id, id, condition_id, accept_id, reject_id);
                block.body.push(instruction);
                id
            }
            crate::Expression::Derivative { axis, ctrl, expr } => {
                use crate::{DerivativeAxis as Axis, DerivativeControl as Ctrl};
                match ctrl {
                    Ctrl::Coarse | Ctrl::Fine => {
                        self.writer.require_any(
                            "DerivativeControl",
                            &[spirv::Capability::DerivativeControl],
                        )?;
                    }
                    Ctrl::None => {}
                }
                let id = self.gen_id();
                let expr_id = self.cached[expr];
                let op = match (axis, ctrl) {
                    (Axis::X, Ctrl::Coarse) => spirv::Op::DPdxCoarse,
                    (Axis::X, Ctrl::Fine) => spirv::Op::DPdxFine,
                    (Axis::X, Ctrl::None) => spirv::Op::DPdx,
                    (Axis::Y, Ctrl::Coarse) => spirv::Op::DPdyCoarse,
                    (Axis::Y, Ctrl::Fine) => spirv::Op::DPdyFine,
                    (Axis::Y, Ctrl::None) => spirv::Op::DPdy,
                    (Axis::Width, Ctrl::Coarse) => spirv::Op::FwidthCoarse,
                    (Axis::Width, Ctrl::Fine) => spirv::Op::FwidthFine,
                    (Axis::Width, Ctrl::None) => spirv::Op::Fwidth,
                };
                block
                    .body
                    .push(Instruction::derivative(op, result_type_id, id, expr_id));
                id
            }
            crate::Expression::ImageQuery { image, query } => {
                self.write_image_query(result_type_id, image, query, block)?
            }
            crate::Expression::Relational { fun, argument } => {
                use crate::RelationalFunction as Rf;
                let arg_id = self.cached[argument];
                let op = match fun {
                    Rf::All => spirv::Op::All,
                    Rf::Any => spirv::Op::Any,
                    Rf::IsNan => spirv::Op::IsNan,
                    Rf::IsInf => spirv::Op::IsInf,
                };
                let id = self.gen_id();
                block
                    .body
                    .push(Instruction::relational(op, result_type_id, id, arg_id));
                id
            }
            crate::Expression::ArrayLength(expr) => self.write_runtime_array_length(expr, block)?,
            crate::Expression::RayQueryGetIntersection { query, committed } => {
                let query_id = self.cached[query];
                let init_tracker_id = *self
                    .ray_query_tracker_expr
                    .get(&query)
                    .expect("not a cached ray query");
                let func_id = self
                    .writer
                    .write_ray_query_get_intersection_function(committed, self.ir_module);
                let ray_intersection = self.ir_module.special_types.ray_intersection.unwrap();
                let intersection_type_id = self.get_handle_type_id(ray_intersection);
                let id = self.gen_id();
                block.body.push(Instruction::function_call(
                    intersection_type_id,
                    id,
                    func_id,
                    &[query_id, init_tracker_id.initialized_tracker],
                ));
                id
            }
            crate::Expression::RayQueryVertexPositions { query, committed } => {
                self.writer.require_any(
                    "RayQueryVertexPositions",
                    &[spirv::Capability::RayQueryPositionFetchKHR],
                )?;
                self.write_ray_query_return_vertex_position(query, block, committed)
            }
            crate::Expression::CooperativeLoad { ref data, .. } => {
                self.writer.require_any(
                    "CooperativeMatrix",
                    &[spirv::Capability::CooperativeMatrixKHR],
                )?;
                let layout = if data.row_major {
                    spirv::CooperativeMatrixLayout::RowMajorKHR
                } else {
                    spirv::CooperativeMatrixLayout::ColumnMajorKHR
                };
                let layout_id = self.get_index_constant(layout as u32);
                let stride_id = self.cached[data.stride];
                match self.write_access_chain(data.pointer, block, AccessTypeAdjustment::None)? {
                    ExpressionPointer::Ready { pointer_id } => {
                        let id = self.gen_id();
                        block.body.push(Instruction::coop_load(
                            result_type_id,
                            id,
                            pointer_id,
                            layout_id,
                            stride_id,
                        ));
                        id
                    }
                    ExpressionPointer::Conditional { condition, access } => self
                        .write_conditional_indexed_load(
                            result_type_id,
                            condition,
                            block,
                            |id_gen, block| {
                                let pointer_id = access.result_id.unwrap();
                                block.body.push(access);
                                let id = id_gen.next();
                                block.body.push(Instruction::coop_load(
                                    result_type_id,
                                    id,
                                    pointer_id,
                                    layout_id,
                                    stride_id,
                                ));
                                id
                            },
                        ),
                }
            }
            crate::Expression::CooperativeMultiplyAdd { a, b, c } => {
                self.writer.require_any(
                    "CooperativeMatrix",
                    &[spirv::Capability::CooperativeMatrixKHR],
                )?;
                let a_id = self.cached[a];
                let b_id = self.cached[b];
                let c_id = self.cached[c];
                let id = self.gen_id();
                block.body.push(Instruction::coop_mul_add(
                    result_type_id,
                    id,
                    a_id,
                    b_id,
                    c_id,
                ));
                id
            }
        };

        self.cached[expr_handle] = id;
        Ok(())
    }

    /// Helper which focuses on generating the `As` expressions and the various conversions
    /// that need to happen because of that.
    fn write_as_expression(
        &mut self,
        expr: Handle<crate::Expression>,
        convert: Option<u8>,
        kind: crate::ScalarKind,

        block: &mut Block,
        result_type_id: u32,
    ) -> Result<u32, Error> {
        use crate::ScalarKind as Sk;
        let expr_id = self.cached[expr];
        let ty = self.fun_info[expr].ty.inner_with(&self.ir_module.types);

        // Matrix casts needs special treatment in SPIR-V, as the cast functions
        // can take vectors or scalars, but not matrices. In order to cast a matrix
        // we need to cast each column of the matrix individually and construct a new
        // matrix from the converted columns.
        if let crate::TypeInner::Matrix {
            columns,
            rows,
            scalar,
        } = *ty
        {
            let Some(convert) = convert else {
                // No conversion needs to be done, passes through.
                return Ok(expr_id);
            };

            if convert == scalar.width {
                // No conversion needs to be done, passes through.
                return Ok(expr_id);
            }

            if kind != Sk::Float {
                // Only float conversions are supported for matrices.
                return Err(Error::Validation("Matrices must be floats"));
            }

            // Type of each extracted column
            let column_src_ty =
                self.get_type_id(LookupType::Local(LocalType::Numeric(NumericType::Vector {
                    size: rows,
                    scalar,
                })));

            // Type of the column after conversion
            let column_dst_ty =
                self.get_type_id(LookupType::Local(LocalType::Numeric(NumericType::Vector {
                    size: rows,
                    scalar: crate::Scalar {
                        kind,
                        width: convert,
                    },
                })));

            let mut components = ArrayVec::<Word, 4>::new();

            for column in 0..columns as usize {
                let column_id = self.gen_id();
                block.body.push(Instruction::composite_extract(
                    column_src_ty,
                    column_id,
                    expr_id,
                    &[column as u32],
                ));

                let column_conv_id = self.gen_id();
                block.body.push(Instruction::unary(
                    spirv::Op::FConvert,
                    column_dst_ty,
                    column_conv_id,
                    column_id,
                ));

                components.push(column_conv_id);
            }

            let construct_id = self.gen_id();

            block.body.push(Instruction::composite_construct(
                result_type_id,
                construct_id,
                &components,
            ));

            return Ok(construct_id);
        }

        let (src_scalar, src_size) = match *ty {
            crate::TypeInner::Scalar(scalar) => (scalar, None),
            crate::TypeInner::Vector { scalar, size } => (scalar, Some(size)),
            ref other => {
                log::error!("As source {other:?}");
                return Err(Error::Validation("Unexpected Expression::As source"));
            }
        };

        enum Cast {
            Identity(Word),
            Unary(spirv::Op, Word),
            Binary(spirv::Op, Word, Word),
            Ternary(spirv::Op, Word, Word, Word),
        }
        let cast = match (src_scalar.kind, kind, convert) {
            // Filter out identity casts. Some Adreno drivers are
            // confused by no-op OpBitCast instructions.
            (src_kind, kind, convert)
                if src_kind == kind
                    && convert.filter(|&width| width != src_scalar.width).is_none() =>
            {
                Cast::Identity(expr_id)
            }
            (Sk::Bool, Sk::Bool, _) => Cast::Unary(spirv::Op::CopyObject, expr_id),
            (_, _, None) => Cast::Unary(spirv::Op::Bitcast, expr_id),
            // casting to a bool - generate `OpXxxNotEqual`
            (_, Sk::Bool, Some(_)) => {
                let op = match src_scalar.kind {
                    Sk::Sint | Sk::Uint => spirv::Op::INotEqual,
                    Sk::Float => spirv::Op::FUnordNotEqual,
                    Sk::Bool | Sk::AbstractInt | Sk::AbstractFloat => unreachable!(),
                };
                let zero_scalar_id = self.writer.get_constant_scalar_with(0, src_scalar)?;
                let zero_id = match src_size {
                    Some(size) => {
                        let ty = LocalType::Numeric(NumericType::Vector {
                            size,
                            scalar: src_scalar,
                        })
                        .into();

                        self.temp_list.clear();
                        self.temp_list.resize(size as _, zero_scalar_id);

                        self.writer.get_constant_composite(ty, &self.temp_list)
                    }
                    None => zero_scalar_id,
                };

                Cast::Binary(op, expr_id, zero_id)
            }
            // casting from a bool - generate `OpSelect`
            (Sk::Bool, _, Some(dst_width)) => {
                let dst_scalar = crate::Scalar {
                    kind,
                    width: dst_width,
                };
                let zero_scalar_id = self.writer.get_constant_scalar_with(0, dst_scalar)?;
                let one_scalar_id = self.writer.get_constant_scalar_with(1, dst_scalar)?;
                let (accept_id, reject_id) = match src_size {
                    Some(size) => {
                        let ty = LocalType::Numeric(NumericType::Vector {
                            size,
                            scalar: dst_scalar,
                        })
                        .into();

                        self.temp_list.clear();
                        self.temp_list.resize(size as _, zero_scalar_id);

                        let vec0_id = self.writer.get_constant_composite(ty, &self.temp_list);

                        self.temp_list.fill(one_scalar_id);

                        let vec1_id = self.writer.get_constant_composite(ty, &self.temp_list);

                        (vec1_id, vec0_id)
                    }
                    None => (one_scalar_id, zero_scalar_id),
                };

                Cast::Ternary(spirv::Op::Select, expr_id, accept_id, reject_id)
            }
            // Avoid undefined behaviour when casting from a float to integer
            // when the value is out of range for the target type. Additionally
            // ensure we clamp to the correct value as per the WGSL spec.
            //
            // https://www.w3.org/TR/WGSL/#floating-point-conversion:
            // * If X is exactly representable in the target type T, then the
            //   result is that value.
            // * Otherwise, the result is the value in T closest to
            //   truncate(X) and also exactly representable in the original
            //   floating point type.
            (Sk::Float, Sk::Sint | Sk::Uint, Some(width)) => {
                let dst_scalar = crate::Scalar { kind, width };
                let (min, max) =
                    crate::proc::min_max_float_representable_by(src_scalar, dst_scalar);
                let expr_type_id = self.get_expression_type_id(&self.fun_info[expr].ty);

                let maybe_splat_const = |writer: &mut Writer, const_id| match src_size {
                    None => const_id,
                    Some(size) => {
                        let constituent_ids = [const_id; crate::VectorSize::MAX];
                        writer.get_constant_composite(
                            LookupType::Local(LocalType::Numeric(NumericType::Vector {
                                size,
                                scalar: src_scalar,
                            })),
                            &constituent_ids[..size as usize],
                        )
                    }
                };
                let min_const_id = self.writer.get_constant_scalar(min);
                let min_const_id = maybe_splat_const(self.writer, min_const_id);
                let max_const_id = self.writer.get_constant_scalar(max);
                let max_const_id = maybe_splat_const(self.writer, max_const_id);

                let clamp_id = self.gen_id();
                block.body.push(Instruction::ext_inst_gl_op(
                    self.writer.gl450_ext_inst_id,
                    spirv::GlslStd450Op::FClamp,
                    expr_type_id,
                    clamp_id,
                    &[expr_id, min_const_id, max_const_id],
                ));

                let op = match dst_scalar.kind {
                    crate::ScalarKind::Sint => spirv::Op::ConvertFToS,
                    crate::ScalarKind::Uint => spirv::Op::ConvertFToU,
                    _ => unreachable!(),
                };
                Cast::Unary(op, clamp_id)
            }
            (Sk::Float, Sk::Float, Some(dst_width)) if src_scalar.width != dst_width => {
                Cast::Unary(spirv::Op::FConvert, expr_id)
            }
            (Sk::Sint, Sk::Float, Some(_)) => Cast::Unary(spirv::Op::ConvertSToF, expr_id),
            (Sk::Sint, Sk::Sint, Some(dst_width)) if src_scalar.width != dst_width => {
                Cast::Unary(spirv::Op::SConvert, expr_id)
            }
            (Sk::Uint, Sk::Float, Some(_)) => Cast::Unary(spirv::Op::ConvertUToF, expr_id),
            (Sk::Uint, Sk::Uint, Some(dst_width)) if src_scalar.width != dst_width => {
                Cast::Unary(spirv::Op::UConvert, expr_id)
            }
            (Sk::Uint, Sk::Sint, Some(dst_width)) if src_scalar.width != dst_width => {
                Cast::Unary(spirv::Op::SConvert, expr_id)
            }
            (Sk::Sint, Sk::Uint, Some(dst_width)) if src_scalar.width != dst_width => {
                Cast::Unary(spirv::Op::UConvert, expr_id)
            }
            // We assume it's either an identity cast, or int-uint.
            _ => Cast::Unary(spirv::Op::Bitcast, expr_id),
        };
        Ok(match cast {
            Cast::Identity(expr) => expr,
            Cast::Unary(op, op1) => {
                let id = self.gen_id();
                block
                    .body
                    .push(Instruction::unary(op, result_type_id, id, op1));
                id
            }
            Cast::Binary(op, op1, op2) => {
                let id = self.gen_id();
                block
                    .body
                    .push(Instruction::binary(op, result_type_id, id, op1, op2));
                id
            }
            Cast::Ternary(op, op1, op2, op3) => {
                let id = self.gen_id();
                block
                    .body
                    .push(Instruction::ternary(op, result_type_id, id, op1, op2, op3));
                id
            }
        })
    }

    /// Build an `OpAccessChain` instruction.
    ///
    /// Emit any needed bounds-checking expressions to `block`.
    ///
    /// Give the `OpAccessChain` a result type based on `expr_handle`, adjusted
    /// according to `type_adjustment`; see the documentation for
    /// [`AccessTypeAdjustment`] for details.
    ///
    /// On success, the return value is an [`ExpressionPointer`] value; see the
    /// documentation for that type.
    fn write_access_chain(
        &mut self,
        mut expr_handle: Handle<crate::Expression>,
        block: &mut Block,
        type_adjustment: AccessTypeAdjustment,
    ) -> Result<ExpressionPointer, Error> {
        let result_type_id = {
            let resolution = &self.fun_info[expr_handle].ty;
            match type_adjustment {
                AccessTypeAdjustment::None => self.writer.get_expression_type_id(resolution),
                AccessTypeAdjustment::IntroducePointer(class) => {
                    self.writer.get_resolution_pointer_id(resolution, class)
                }
                AccessTypeAdjustment::UseStd140CompatType => {
                    match *resolution.inner_with(&self.ir_module.types) {
                        crate::TypeInner::Pointer {
                            base,
                            space: space @ crate::AddressSpace::Uniform,
                        } => self.writer.get_pointer_type_id(
                            self.writer.std140_compat_uniform_types[&base].type_id,
                            map_storage_class(space),
                        ),
                        _ => unreachable!(
                            "`UseStd140CompatType` must only be used with uniform pointer types"
                        ),
                    }
                }
            }
        };

        // The id of the boolean `and` of all dynamic bounds checks up to this point.
        //
        // See `extend_bounds_check_condition_chain` for a full explanation.
        let mut accumulated_checks = None;

        // Is true if we are accessing into a binding array with a non-uniform index.
        let mut is_non_uniform_binding_array = false;

        // The index value if the previously encountered expression was an
        // `AccessIndex` of a matrix which has been decomposed into individual
        // column vectors directly in the containing struct. The subsequent
        // iteration will append the correct index to the list for accessing
        // said column from the containing struct.
        let mut prev_decomposed_matrix_index = None;

        self.temp_list.clear();
        let root_id = loop {
            // If `expr_handle` was spilled, then the temporary variable has exactly
            // the value we want to start from.
            if let Some(spilled) = self.function.spilled_composites.get(&expr_handle) {
                // The root id of the `OpAccessChain` instruction is the temporary
                // variable we spilled the composite to.
                break spilled.id;
            }

            expr_handle = match self.ir_function.expressions[expr_handle] {
                crate::Expression::Access { base, index } => {
                    is_non_uniform_binding_array |=
                        self.is_nonuniform_binding_array_access(base, index);

                    let index = GuardedIndex::Expression(index);
                    let index_id =
                        self.write_access_chain_index(base, index, &mut accumulated_checks, block)?;
                    self.temp_list.push(index_id);

                    base
                }
                crate::Expression::AccessIndex { base, index } => {
                    // Decide whether we're indexing a struct (bounds checks
                    // forbidden) or anything else (bounds checks required).
                    let mut base_ty = self.fun_info[base].ty.inner_with(&self.ir_module.types);
                    let mut base_ty_handle = self.fun_info[base].ty.handle();
                    let mut pointer_space = None;
                    if let crate::TypeInner::Pointer { base, space } = *base_ty {
                        base_ty = &self.ir_module.types[base].inner;
                        base_ty_handle = Some(base);
                        pointer_space = Some(space);
                    }
                    match *base_ty {
                        // When indexing a struct bounds checks are forbidden. If accessing the
                        // struct through a uniform address space pointer, where the struct has
                        // been declared with an alternative std140 compatible layout, we must use
                        // the remapped member index. Additionally if the previous iteration was
                        // accessing a column of a matrix member which has been decomposed directly
                        // into the struct, we must ensure we access the correct column.
                        crate::TypeInner::Struct { .. } => {
                            let index = match base_ty_handle.and_then(|handle| {
                                self.writer.std140_compat_uniform_types.get(&handle)
                            }) {
                                Some(std140_type_info)
                                    if pointer_space == Some(crate::AddressSpace::Uniform) =>
                                {
                                    std140_type_info.member_indices[index as usize]
                                        + prev_decomposed_matrix_index.take().unwrap_or(0)
                                }
                                _ => index,
                            };
                            let index_id = self.get_index_constant(index);
                            self.temp_list.push(index_id);
                        }
                        // Bounds checks are not required when indexing a matrix. If indexing a
                        // two-row matrix contained within a struct through a uniform address space
                        // pointer then the matrix' columns will have been decomposed directly into
                        // the containing struct. We skip adding an index to the list on this
                        // iteration and instead adjust the index on the next iteration when
                        // accessing the struct member.
                        _ if is_uniform_matcx2_struct_member_access(
                            self.ir_function,
                            self.fun_info,
                            self.ir_module,
                            base,
                        ) =>
                        {
                            assert!(prev_decomposed_matrix_index.is_none());
                            prev_decomposed_matrix_index = Some(index);
                        }
                        _ => {
                            // `index` is constant, so this can't possibly require
                            // setting `is_nonuniform_binding_array_access`.

                            // Even though the index value is statically known, `base`
                            // may be a runtime-sized array, so we still need to go
                            // through the bounds check process.
                            let index_id = self.write_access_chain_index(
                                base,
                                GuardedIndex::Known(index),
                                &mut accumulated_checks,
                                block,
                            )?;
                            self.temp_list.push(index_id);
                        }
                    }
                    base
                }
                crate::Expression::GlobalVariable(handle) => {
                    let gv = &self.writer.global_variables[handle];
                    break gv.access_id;
                }
                crate::Expression::LocalVariable(variable) => {
                    let local_var = &self.function.variables[&variable];
                    break local_var.id;
                }
                crate::Expression::FunctionArgument(index) => {
                    break self.function.parameter_id(index);
                }
                ref other => unimplemented!("Unexpected pointer expression {:?}", other),
            }
        };

        let (pointer_id, expr_pointer) = if self.temp_list.is_empty() {
            (
                root_id,
                ExpressionPointer::Ready {
                    pointer_id: root_id,
                },
            )
        } else {
            self.temp_list.reverse();
            let pointer_id = self.gen_id();
            let access =
                Instruction::access_chain(result_type_id, pointer_id, root_id, &self.temp_list);

            // If we generated some bounds checks, we need to leave it to our
            // caller to generate the branch, the access, the load or store, and
            // the zero value (for loads). Otherwise, we can emit the access
            // ourselves, and just hand them the id of the pointer.
            let expr_pointer = match accumulated_checks {
                Some(condition) => ExpressionPointer::Conditional { condition, access },
                None => {
                    block.body.push(access);
                    ExpressionPointer::Ready { pointer_id }
                }
            };
            (pointer_id, expr_pointer)
        };
        // Subsequent load, store and atomic operations require the pointer to be decorated as NonUniform
        // if the binding array was accessed with a non-uniform index
        // see VUID-RuntimeSpirv-NonUniform-06274
        if is_non_uniform_binding_array {
            self.writer
                .decorate_non_uniform_binding_array_access(pointer_id)?;
        }

        Ok(expr_pointer)
    }

    fn is_nonuniform_binding_array_access(
        &mut self,
        base: Handle<crate::Expression>,
        index: Handle<crate::Expression>,
    ) -> bool {
        let crate::Expression::GlobalVariable(var_handle) = self.ir_function.expressions[base]
        else {
            return false;
        };

        // The access chain needs to be decorated as NonUniform
        // see VUID-RuntimeSpirv-NonUniform-06274
        let gvar = &self.ir_module.global_variables[var_handle];
        let crate::TypeInner::BindingArray { .. } = self.ir_module.types[gvar.ty].inner else {
            return false;
        };

        self.fun_info[index].uniformity.non_uniform_result.is_some()
    }

    /// Compute a single index operand to an `OpAccessChain` instruction.
    ///
    /// Given that we are indexing `base` with `index`, apply the appropriate
    /// bounds check policies, emitting code to `block` to clamp `index` or
    /// determine whether it's in bounds. Return the SPIR-V instruction id of
    /// the index value we should actually use.
    ///
    /// Extend `accumulated_checks` to include the results of any needed bounds
    /// checks. See [`BlockContext::extend_bounds_check_condition_chain`].
    fn write_access_chain_index(
        &mut self,
        base: Handle<crate::Expression>,
        index: GuardedIndex,
        accumulated_checks: &mut Option<Word>,
        block: &mut Block,
    ) -> Result<Word, Error> {
        match self.write_bounds_check(base, index, block)? {
            BoundsCheckResult::KnownInBounds(known_index) => {
                // Even if the index is known, `OpAccessChain`
                // requires expression operands, not literals.
                let scalar = crate::Literal::U32(known_index);
                Ok(self.writer.get_constant_scalar(scalar))
            }
            BoundsCheckResult::Computed(computed_index_id) => Ok(computed_index_id),
            BoundsCheckResult::Conditional {
                condition_id: condition,
                index_id: index,
            } => {
                self.extend_bounds_check_condition_chain(accumulated_checks, condition, block);

                // Use the index from the `Access` expression unchanged.
                Ok(index)
            }
        }
    }

    /// Add a condition to a chain of bounds checks.
    ///
    /// As we build an `OpAccessChain` instruction govered by
    /// [`BoundsCheckPolicy::ReadZeroSkipWrite`], we accumulate a chain of
    /// dynamic bounds checks, one for each index in the chain, which must all
    /// be true for that `OpAccessChain`'s execution to be well-defined. This
    /// function adds the boolean instruction id `comparison_id` to `chain`.
    ///
    /// If `chain` is `None`, that means there are no bounds checks in the chain
    /// yet. If chain is `Some(id)`, then `id` is the conjunction of all the
    /// bounds checks in the chain.
    ///
    /// When we have multiple bounds checks, we combine them with
    /// `OpLogicalAnd`, not a short-circuit branch. This means we might do
    /// comparisons we don't need to, but we expect these checks to almost
    /// always succeed, and keeping branches to a minimum is essential.
    ///
    /// [`BoundsCheckPolicy::ReadZeroSkipWrite`]: crate::proc::BoundsCheckPolicy
    fn extend_bounds_check_condition_chain(
        &mut self,
        chain: &mut Option<Word>,
        comparison_id: Word,
        block: &mut Block,
    ) {
        match *chain {
            Some(ref mut prior_checks) => {
                let combined = self.gen_id();
                block.body.push(Instruction::binary(
                    spirv::Op::LogicalAnd,
                    self.writer.get_bool_type_id(),
                    combined,
                    *prior_checks,
                    comparison_id,
                ));
                *prior_checks = combined;
            }
            None => {
                // Start a fresh chain of checks.
                *chain = Some(comparison_id);
            }
        }
    }

    fn write_checked_load(
        &mut self,
        pointer: Handle<crate::Expression>,
        block: &mut Block,
        access_type_adjustment: AccessTypeAdjustment,
        result_type_id: Word,
    ) -> Result<Word, Error> {
        if let Some(result_id) = self.maybe_write_uniform_matcx2_dynamic_access(pointer, block)? {
            Ok(result_id)
        } else if let Some(result_id) =
            self.maybe_write_load_uniform_matcx2_struct_member(pointer, block)?
        {
            Ok(result_id)
        } else {
            // If `pointer` refers to a uniform address space pointer to a type
            // which was declared using a std140 compatible type variant (i.e.
            // is a two-row matrix, or a struct or array containing such a
            // matrix) we must ensure the access chain and the type of the load
            // instruction use the std140 compatible type variant.
            struct WrappedLoad {
                access_type_adjustment: AccessTypeAdjustment,
                r#type: Handle<crate::Type>,
            }
            let mut wrapped_load = None;
            if let crate::TypeInner::Pointer {
                base: pointer_base_type,
                space: crate::AddressSpace::Uniform,
            } = *self.fun_info[pointer].ty.inner_with(&self.ir_module.types)
            {
                if self
                    .writer
                    .std140_compat_uniform_types
                    .contains_key(&pointer_base_type)
                {
                    wrapped_load = Some(WrappedLoad {
                        access_type_adjustment: AccessTypeAdjustment::UseStd140CompatType,
                        r#type: pointer_base_type,
                    });
                };
            };

            let (load_type_id, access_type_adjustment) = match wrapped_load {
                Some(ref wrapped_load) => (
                    self.writer.std140_compat_uniform_types[&wrapped_load.r#type].type_id,
                    wrapped_load.access_type_adjustment,
                ),
                None => (result_type_id, access_type_adjustment),
            };

            let load_id = match self.write_access_chain(pointer, block, access_type_adjustment)? {
                ExpressionPointer::Ready { pointer_id } => {
                    let id = self.gen_id();
                    let atomic_space =
                        match *self.fun_info[pointer].ty.inner_with(&self.ir_module.types) {
                            crate::TypeInner::Pointer { base, space } => {
                                match self.ir_module.types[base].inner {
                                    crate::TypeInner::Atomic { .. } => Some(space),
                                    _ => None,
                                }
                            }
                            _ => None,
                        };
                    let instruction = if let Some(space) = atomic_space {
                        let (semantics, scope) = space.to_spirv_semantics_and_scope();
                        let scope_constant_id = self.get_scope_constant(scope as u32);
                        let semantics_id = self.get_index_constant(semantics.bits());
                        Instruction::atomic_load(
                            result_type_id,
                            id,
                            pointer_id,
                            scope_constant_id,
                            semantics_id,
                        )
                    } else {
                        Instruction::load(load_type_id, id, pointer_id, None)
                    };
                    block.body.push(instruction);
                    id
                }
                ExpressionPointer::Conditional { condition, access } => {
                    //TODO: support atomics?
                    self.write_conditional_indexed_load(
                        load_type_id,
                        condition,
                        block,
                        move |id_gen, block| {
                            // The in-bounds path. Perform the access and the load.
                            let pointer_id = access.result_id.unwrap();
                            let value_id = id_gen.next();
                            block.body.push(access);
                            block.body.push(Instruction::load(
                                load_type_id,
                                value_id,
                                pointer_id,
                                None,
                            ));
                            value_id
                        },
                    )
                }
            };

            match wrapped_load {
                Some(ref wrapped_load) => {
                    // If we loaded a std140 compat type then we must call the
                    // function to convert the loaded value to the regular type.
                    let result_id = self.gen_id();
                    let function_id = self.writer.wrapped_functions
                        [&WrappedFunction::ConvertFromStd140CompatType {
                            r#type: wrapped_load.r#type,
                        }];
                    block.body.push(Instruction::function_call(
                        result_type_id,
                        result_id,
                        function_id,
                        &[load_id],
                    ));
                    Ok(result_id)
                }
                None => Ok(load_id),
            }
        }
    }

    fn spill_to_internal_variable(&mut self, base: Handle<crate::Expression>, block: &mut Block) {
        use indexmap::map::Entry;

        // Make sure we have an internal variable to spill `base` to.
        let spill_variable_id = match self.function.spilled_composites.entry(base) {
            Entry::Occupied(preexisting) => preexisting.get().id,
            Entry::Vacant(vacant) => {
                // Generate a new internal variable of the appropriate
                // type for `base`.
                let pointer_type_id = self.writer.get_resolution_pointer_id(
                    &self.fun_info[base].ty,
                    spirv::StorageClass::Function,
                );
                let id = self.writer.id_gen.next();
                vacant.insert(super::LocalVariable {
                    id,
                    instruction: Instruction::variable(
                        pointer_type_id,
                        id,
                        spirv::StorageClass::Function,
                        None,
                    ),
                });
                id
            }
        };

        // Perform the store even if we already had a spill variable for `base`.
        // Consider this code:
        //
        // var x = ...;
        // var y = ...;
        // var z = ...;
        // for (i = 0; i<2; i++) {
        //     let a = array(i, i, i);
        //     if (i == 0) {
        //         x += a[y];
        //     } else [
        //         x += a[z];
        //     }
        // }
        //
        // The value of `a` needs to be spilled so we can subscript it with `y` and `z`.
        //
        // When we generate SPIR-V for `a[y]`, we will create the spill
        // variable, and store `a`'s value in it.
        //
        // When we generate SPIR-V for `a[z]`, we will notice that the spill
        // variable for `a` has already been declared, but it is still essential
        // that we store `a` into it, so that `a[z]` sees this iteration's value
        // of `a`.
        let base_id = self.cached[base];
        block
            .body
            .push(Instruction::store(spill_variable_id, base_id, None));
    }

    /// Generate an access to a spilled temporary, if necessary.
    ///
    /// Given `access`, an [`Access`] or [`AccessIndex`] expression that refers
    /// to a component of a composite value that has been spilled to a temporary
    /// variable, determine whether other expressions are going to use
    /// `access`'s value:
    ///
    /// - If so, perform the access and cache that as the value of `access`.
    ///
    /// - Otherwise, generate no code and cache no value for `access`.
    ///
    /// Return `Ok(0)` if no value was fetched, or `Ok(id)` if we loaded it into
    /// the instruction given by `id`.
    ///
    /// [`Access`]: crate::Expression::Access
    /// [`AccessIndex`]: crate::Expression::AccessIndex
    fn maybe_access_spilled_composite(
        &mut self,
        access: Handle<crate::Expression>,
        block: &mut Block,
        result_type_id: Word,
    ) -> Result<Word, Error> {
        let access_uses = self.function.access_uses.get(&access).map_or(0, |r| *r);
        if access_uses == self.fun_info[access].ref_count {
            // This expression is only used by other `Access` and
            // `AccessIndex` expressions, so we don't need to cache a
            // value for it yet.
            Ok(0)
        } else {
            // There are other expressions that are going to expect this
            // expression's value to be cached, not just other `Access` or
            // `AccessIndex` expressions. We must actually perform the
            // access on the spill variable now.
            self.write_checked_load(
                access,
                block,
                AccessTypeAdjustment::IntroducePointer(spirv::StorageClass::Function),
                result_type_id,
            )
        }
    }

    /// Build the instructions for matrix - matrix column operations
    #[allow(clippy::too_many_arguments)]
    fn write_matrix_matrix_column_op(
        &mut self,
        block: &mut Block,
        result_id: Word,
        result_type_id: Word,
        left_id: Word,
        right_id: Word,
        columns: crate::VectorSize,
        rows: crate::VectorSize,
        width: u8,
        op: spirv::Op,
    ) {
        self.temp_list.clear();

        let vector_type_id = self.get_numeric_type_id(NumericType::Vector {
            size: rows,
            scalar: crate::Scalar::float(width),
        });

        for index in 0..columns as u32 {
            let column_id_left = self.gen_id();
            let column_id_right = self.gen_id();
            let column_id_res = self.gen_id();

            block.body.push(Instruction::composite_extract(
                vector_type_id,
                column_id_left,
                left_id,
                &[index],
            ));
            block.body.push(Instruction::composite_extract(
                vector_type_id,
                column_id_right,
                right_id,
                &[index],
            ));
            block.body.push(Instruction::binary(
                op,
                vector_type_id,
                column_id_res,
                column_id_left,
                column_id_right,
            ));

            self.temp_list.push(column_id_res);
        }

        block.body.push(Instruction::composite_construct(
            result_type_id,
            result_id,
            &self.temp_list,
        ));
    }

    /// Build the instructions for vector - scalar multiplication
    fn write_vector_scalar_mult(
        &mut self,
        block: &mut Block,
        result_id: Word,
        result_type_id: Word,
        vector_id: Word,
        scalar_id: Word,
        vector: &crate::TypeInner,
    ) {
        let (size, kind) = match *vector {
            crate::TypeInner::Vector {
                size,
                scalar: crate::Scalar { kind, .. },
            } => (size, kind),
            _ => unreachable!(),
        };

        let (op, operand_id) = match kind {
            crate::ScalarKind::Float => (spirv::Op::VectorTimesScalar, scalar_id),
            _ => {
                let operand_id = self.gen_id();
                self.temp_list.clear();
                self.temp_list.resize(size as usize, scalar_id);
                block.body.push(Instruction::composite_construct(
                    result_type_id,
                    operand_id,
                    &self.temp_list,
                ));
                (spirv::Op::IMul, operand_id)
            }
        };

        block.body.push(Instruction::binary(
            op,
            result_type_id,
            result_id,
            vector_id,
            operand_id,
        ));
    }

    /// Build the instructions for the arithmetic expression of a dot product
    ///
    /// The argument `extractor` is a function that maps `(result_id,
    /// composite_id, index)` to an instruction that extracts the `index`th
    /// entry of the value with ID `composite_id` and assigns it to the slot
    /// with id `result_id` (which must have type `result_type_id`).
    #[expect(clippy::too_many_arguments)]
    fn write_dot_product(
        &mut self,
        result_id: Word,
        result_type_id: Word,
        arg0_id: Word,
        arg1_id: Word,
        size: u32,
        block: &mut Block,
        extractor: impl Fn(Word, Word, Word) -> Instruction,
    ) {
        let mut partial_sum = self.writer.get_constant_null(result_type_id);
        let last_component = size - 1;
        for index in 0..=last_component {
            // compute the product of the current components
            let a_id = self.gen_id();
            block.body.push(extractor(a_id, arg0_id, index));
            let b_id = self.gen_id();
            block.body.push(extractor(b_id, arg1_id, index));
            let prod_id = self.gen_id();
            block.body.push(Instruction::binary(
                spirv::Op::IMul,
                result_type_id,
                prod_id,
                a_id,
                b_id,
            ));

            // choose the id for the next sum, depending on current index
            let id = if index == last_component {
                result_id
            } else {
                self.gen_id()
            };

            // sum the computed product with the partial sum
            block.body.push(Instruction::binary(
                spirv::Op::IAdd,
                result_type_id,
                id,
                partial_sum,
                prod_id,
            ));
            // set the id of the result as the previous partial sum
            partial_sum = id;
        }
    }

    /// Emit code for `pack4x{I,U}8[Clamp]` if capability "Int8" is available.
    fn write_pack4x8_optimized(
        &mut self,
        block: &mut Block,
        result_type_id: u32,
        arg0_id: u32,
        id: u32,
        is_signed: bool,
        should_clamp: bool,
    ) -> Instruction {
        let int_type = if is_signed {
            crate::ScalarKind::Sint
        } else {
            crate::ScalarKind::Uint
        };
        let wide_vector_type = NumericType::Vector {
            size: crate::VectorSize::Quad,
            scalar: crate::Scalar {
                kind: int_type,
                width: 4,
            },
        };
        let wide_vector_type_id = self.get_numeric_type_id(wide_vector_type);
        let packed_vector_type_id = self.get_numeric_type_id(NumericType::Vector {
            size: crate::VectorSize::Quad,
            scalar: crate::Scalar {
                kind: crate::ScalarKind::Uint,
                width: 1,
            },
        });

        let mut wide_vector = arg0_id;
        if should_clamp {
            let (min, max, clamp_op) = if is_signed {
                (
                    crate::Literal::I32(-128),
                    crate::Literal::I32(127),
                    spirv::GlslStd450Op::SClamp,
                )
            } else {
                (
                    crate::Literal::U32(0),
                    crate::Literal::U32(255),
                    spirv::GlslStd450Op::UClamp,
                )
            };
            let [min, max] = [min, max].map(|lit| {
                let scalar = self.writer.get_constant_scalar(lit);
                self.writer.get_constant_composite(
                    LookupType::Local(LocalType::Numeric(wide_vector_type)),
                    &[scalar; 4],
                )
            });

            let clamp_id = self.gen_id();
            block.body.push(Instruction::ext_inst_gl_op(
                self.writer.gl450_ext_inst_id,
                clamp_op,
                wide_vector_type_id,
                clamp_id,
                &[wide_vector, min, max],
            ));

            wide_vector = clamp_id;
        }

        let packed_vector = self.gen_id();
        block.body.push(Instruction::unary(
            spirv::Op::UConvert, // We truncate, so `UConvert` and `SConvert` behave identically.
            packed_vector_type_id,
            packed_vector,
            wide_vector,
        ));

        // The SPIR-V spec [1] defines the bit order for bit casting between a vector
        // and a scalar precisely as required by the WGSL spec [2].
        // [1]: https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#OpBitcast
        // [2]: https://www.w3.org/TR/WGSL/#pack4xI8-builtin
        Instruction::unary(spirv::Op::Bitcast, result_type_id, id, packed_vector)
    }

    /// Emit code for `pack4x{I,U}8[Clamp]` if capability "Int8" is not available.
    fn write_pack4x8_polyfill(
        &mut self,
        block: &mut Block,
        result_type_id: u32,
        arg0_id: u32,
        id: u32,
        is_signed: bool,
        should_clamp: bool,
    ) -> Instruction {
        let int_type = if is_signed {
            crate::ScalarKind::Sint
        } else {
            crate::ScalarKind::Uint
        };
        let uint_type_id = self.get_numeric_type_id(NumericType::Scalar(crate::Scalar::U32));
        let int_type_id = self.get_numeric_type_id(NumericType::Scalar(crate::Scalar {
            kind: int_type,
            width: 4,
        }));

        let mut last_instruction = Instruction::new(spirv::Op::Nop);

        let zero = self.writer.get_constant_scalar(crate::Literal::U32(0));
        let mut preresult = zero;
        block
            .body
            .reserve(usize::from(VEC_LENGTH) * (2 + usize::from(is_signed)));

        let eight = self.writer.get_constant_scalar(crate::Literal::U32(8));
        const VEC_LENGTH: u8 = 4;
        for i in 0..u32::from(VEC_LENGTH) {
            let offset = self.writer.get_constant_scalar(crate::Literal::U32(i * 8));
            let mut extracted = self.gen_id();
            block.body.push(Instruction::binary(
                spirv::Op::CompositeExtract,
                int_type_id,
                extracted,
                arg0_id,
                i,
            ));
            if is_signed {
                let casted = self.gen_id();
                block.body.push(Instruction::unary(
                    spirv::Op::Bitcast,
                    uint_type_id,
                    casted,
                    extracted,
                ));
                extracted = casted;
            }
            if should_clamp {
                let (min, max, clamp_op) = if is_signed {
                    (
                        crate::Literal::I32(-128),
                        crate::Literal::I32(127),
                        spirv::GlslStd450Op::SClamp,
                    )
                } else {
                    (
                        crate::Literal::U32(0),
                        crate::Literal::U32(255),
                        spirv::GlslStd450Op::UClamp,
                    )
                };
                let [min, max] = [min, max].map(|lit| self.writer.get_constant_scalar(lit));

                let clamp_id = self.gen_id();
                block.body.push(Instruction::ext_inst_gl_op(
                    self.writer.gl450_ext_inst_id,
                    clamp_op,
                    result_type_id,
                    clamp_id,
                    &[extracted, min, max],
                ));

                extracted = clamp_id;
            }
            let is_last = i == u32::from(VEC_LENGTH - 1);
            if is_last {
                last_instruction = Instruction::quaternary(
                    spirv::Op::BitFieldInsert,
                    result_type_id,
                    id,
                    preresult,
                    extracted,
                    offset,
                    eight,
                )
            } else {
                let new_preresult = self.gen_id();
                block.body.push(Instruction::quaternary(
                    spirv::Op::BitFieldInsert,
                    result_type_id,
                    new_preresult,
                    preresult,
                    extracted,
                    offset,
                    eight,
                ));
                preresult = new_preresult;
            }
        }
        last_instruction
    }

    /// Emit code for `unpack4x{I,U}8` if capability "Int8" is available.
    fn write_unpack4x8_optimized(
        &mut self,
        block: &mut Block,
        result_type_id: u32,
        arg0_id: u32,
        id: u32,
        is_signed: bool,
    ) -> Instruction {
        let (int_type, convert_op) = if is_signed {
            (crate::ScalarKind::Sint, spirv::Op::SConvert)
        } else {
            (crate::ScalarKind::Uint, spirv::Op::UConvert)
        };

        let packed_vector_type_id = self.get_numeric_type_id(NumericType::Vector {
            size: crate::VectorSize::Quad,
            scalar: crate::Scalar {
                kind: int_type,
                width: 1,
            },
        });

        // The SPIR-V spec [1] defines the bit order for bit casting between a vector
        // and a scalar precisely as required by the WGSL spec [2].
        // [1]: https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#OpBitcast
        // [2]: https://www.w3.org/TR/WGSL/#pack4xI8-builtin
        let packed_vector = self.gen_id();
        block.body.push(Instruction::unary(
            spirv::Op::Bitcast,
            packed_vector_type_id,
            packed_vector,
            arg0_id,
        ));

        Instruction::unary(convert_op, result_type_id, id, packed_vector)
    }

    /// Emit code for `unpack4x{I,U}8` if capability "Int8" is not available.
    fn write_unpack4x8_polyfill(
        &mut self,
        block: &mut Block,
        result_type_id: u32,
        arg0_id: u32,
        id: u32,
        is_signed: bool,
    ) -> Instruction {
        let (int_type, extract_op) = if is_signed {
            (crate::ScalarKind::Sint, spirv::Op::BitFieldSExtract)
        } else {
            (crate::ScalarKind::Uint, spirv::Op::BitFieldUExtract)
        };

        let sint_type_id = self.get_numeric_type_id(NumericType::Scalar(crate::Scalar::I32));

        let eight = self.writer.get_constant_scalar(crate::Literal::U32(8));
        let int_type_id = self.get_numeric_type_id(NumericType::Scalar(crate::Scalar {
            kind: int_type,
            width: 4,
        }));
        block
            .body
            .reserve(usize::from(VEC_LENGTH) * 2 + usize::from(is_signed));
        let arg_id = if is_signed {
            let new_arg_id = self.gen_id();
            block.body.push(Instruction::unary(
                spirv::Op::Bitcast,
                sint_type_id,
                new_arg_id,
                arg0_id,
            ));
            new_arg_id
        } else {
            arg0_id
        };

        const VEC_LENGTH: u8 = 4;
        let parts: [_; VEC_LENGTH as usize] = core::array::from_fn(|_| self.gen_id());
        for (i, part_id) in parts.into_iter().enumerate() {
            let index = self
                .writer
                .get_constant_scalar(crate::Literal::U32(i as u32 * 8));
            block.body.push(Instruction::ternary(
                extract_op,
                int_type_id,
                part_id,
                arg_id,
                index,
                eight,
            ));
        }

        Instruction::composite_construct(result_type_id, id, &parts)
    }

    /// Generate one or more SPIR-V blocks for `naga_block`.
    ///
    /// Use `label_id` as the label for the SPIR-V entry point block.
    ///
    /// If control reaches the end of the SPIR-V block, terminate it according
    /// to `exit`. This function's return value indicates whether it acted on
    /// this parameter or not; see [`BlockExitDisposition`].
    ///
    /// If the block contains [`Break`] or [`Continue`] statements,
    /// `loop_context` supplies the labels of the SPIR-V blocks to jump to. If
    /// either of these labels are `None`, then it should have been a Naga
    /// validation error for the corresponding statement to occur in this
    /// context.
    ///
    /// [`Break`]: Statement::Break
    /// [`Continue`]: Statement::Continue
    fn write_block(
        &mut self,
        label_id: Word,
        naga_block: &crate::Block,
        exit: BlockExit,
        loop_context: LoopContext,
        debug_info: Option<&DebugInfoInner>,
    ) -> Result<BlockExitDisposition, Error> {
        let mut block = Block::new(label_id);
        for (statement, span) in naga_block.span_iter() {
            if let (Some(debug_info), false) = (
                debug_info,
                matches!(
                    statement,
                    &(Statement::Block(..)
                        | Statement::Break
                        | Statement::Continue
                        | Statement::Kill
                        | Statement::Return { .. }
                        | Statement::Loop { .. })
                ),
            ) {
                let loc: crate::SourceLocation = span.location(debug_info.source_code);
                block.body.push(Instruction::line(
                    debug_info.source_file_id,
                    loc.line_number,
                    loc.line_position,
                ));
            };
            match *statement {
                Statement::Emit(ref range) => {
                    for handle in range.clone() {
                        // omit const expressions as we've already cached those
                        if !self.expression_constness.is_const(handle) {
                            self.cache_expression_value(handle, &mut block)?;
                        }
                    }
                }
                Statement::Block(ref block_statements) => {
                    let scope_id = self.gen_id();
                    self.function.consume(block, Instruction::branch(scope_id));

                    let merge_id = self.gen_id();
                    let merge_used = self.write_block(
                        scope_id,
                        block_statements,
                        BlockExit::Branch { target: merge_id },
                        loop_context,
                        debug_info,
                    )?;

                    match merge_used {
                        BlockExitDisposition::Used => {
                            block = Block::new(merge_id);
                        }
                        BlockExitDisposition::Discarded => {
                            return Ok(BlockExitDisposition::Discarded);
                        }
                    }
                }
                Statement::If {
                    condition,
                    ref accept,
                    ref reject,
                } => {
                    // In spirv 1.6, in a conditional branch the two block ids
                    // of the branches can't have the same label. If `accept`
                    // and `reject` are both empty (e.g. in `if (condition) {}`)
                    // merge id will be both labels. Because both branches are
                    // empty, we can skip the if statement.
                    if !(accept.is_empty() && reject.is_empty()) {
                        let condition_id = self.cached[condition];

                        let merge_id = self.gen_id();
                        block.body.push(Instruction::selection_merge(
                            merge_id,
                            spirv::SelectionControl::NONE,
                        ));

                        let accept_id = if accept.is_empty() {
                            None
                        } else {
                            Some(self.gen_id())
                        };
                        let reject_id = if reject.is_empty() {
                            None
                        } else {
                            Some(self.gen_id())
                        };

                        self.function.consume(
                            block,
                            Instruction::branch_conditional(
                                condition_id,
                                accept_id.unwrap_or(merge_id),
                                reject_id.unwrap_or(merge_id),
                            ),
                        );

                        if let Some(block_id) = accept_id {
                            // We can ignore the `BlockExitDisposition` returned here because,
                            // even if `merge_id` is not actually reachable, it is always
                            // referred to by the `OpSelectionMerge` instruction we emitted
                            // earlier.
                            let _ = self.write_block(
                                block_id,
                                accept,
                                BlockExit::Branch { target: merge_id },
                                loop_context,
                                debug_info,
                            )?;
                        }
                        if let Some(block_id) = reject_id {
                            // We can ignore the `BlockExitDisposition` returned here because,
                            // even if `merge_id` is not actually reachable, it is always
                            // referred to by the `OpSelectionMerge` instruction we emitted
                            // earlier.
                            let _ = self.write_block(
                                block_id,
                                reject,
                                BlockExit::Branch { target: merge_id },
                                loop_context,
                                debug_info,
                            )?;
                        }

                        block = Block::new(merge_id);
                    }
                }
                Statement::Switch {
                    selector,
                    ref cases,
                } => {
                    let selector_id = self.cached[selector];

                    let merge_id = self.gen_id();
                    block.body.push(Instruction::selection_merge(
                        merge_id,
                        spirv::SelectionControl::NONE,
                    ));

                    let mut default_id = None;
                    // id of previous empty fall-through case
                    let mut last_id = None;

                    let mut raw_cases = Vec::with_capacity(cases.len());
                    let mut case_ids = Vec::with_capacity(cases.len());
                    for case in cases.iter() {
                        // take id of previous empty fall-through case or generate a new one
                        let label_id = last_id.take().unwrap_or_else(|| self.gen_id());

                        if case.fall_through && case.body.is_empty() {
                            last_id = Some(label_id);
                        }

                        case_ids.push(label_id);

                        match case.value {
                            crate::SwitchValue::I32(value) => {
                                raw_cases.push(super::instructions::Case {
                                    value: value as Word,
                                    label_id,
                                });
                            }
                            crate::SwitchValue::U32(value) => {
                                raw_cases.push(super::instructions::Case { value, label_id });
                            }
                            crate::SwitchValue::Default => {
                                default_id = Some(label_id);
                            }
                        }
                    }

                    let default_id = default_id.unwrap();

                    self.function.consume(
                        block,
                        Instruction::switch(selector_id, default_id, &raw_cases),
                    );

                    let inner_context = LoopContext {
                        break_id: Some(merge_id),
                        ..loop_context
                    };

                    for (i, (case, label_id)) in cases
                        .iter()
                        .zip(case_ids.iter())
                        .filter(|&(case, _)| !(case.fall_through && case.body.is_empty()))
                        .enumerate()
                    {
                        let case_finish_id = if case.fall_through {
                            case_ids[i + 1]
                        } else {
                            merge_id
                        };
                        // We can ignore the `BlockExitDisposition` returned here because
                        // `case_finish_id` is always referred to by either:
                        //
                        // - the `OpSwitch`, if it's the next case's label for a
                        //   fall-through, or
                        //
                        // - the `OpSelectionMerge`, if it's the switch's overall merge
                        //   block because there's no fall-through.
                        let _ = self.write_block(
                            *label_id,
                            &case.body,
                            BlockExit::Branch {
                                target: case_finish_id,
                            },
                            inner_context,
                            debug_info,
                        )?;
                    }

                    block = Block::new(merge_id);
                }
                Statement::Loop {
                    ref body,
                    ref continuing,
                    break_if,
                } => {
                    let preamble_id = self.gen_id();
                    self.function
                        .consume(block, Instruction::branch(preamble_id));

                    let merge_id = self.gen_id();
                    let body_id = self.gen_id();
                    let continuing_id = self.gen_id();

                    // SPIR-V requires the continuing to the `OpLoopMerge`,
                    // so we have to start a new block with it.
                    block = Block::new(preamble_id);
                    // HACK the loop statement is begin with branch instruction,
                    // so we need to put `OpLine` debug info before merge instruction
                    if let Some(debug_info) = debug_info {
                        let loc: crate::SourceLocation = span.location(debug_info.source_code);
                        block.body.push(Instruction::line(
                            debug_info.source_file_id,
                            loc.line_number,
                            loc.line_position,
                        ))
                    }
                    block.body.push(Instruction::loop_merge(
                        merge_id,
                        continuing_id,
                        spirv::SelectionControl::NONE,
                    ));

                    if self.force_loop_bounding {
                        block = self.write_force_bounded_loop_instructions(block, merge_id);
                    }
                    self.function.consume(block, Instruction::branch(body_id));

                    // We can ignore the `BlockExitDisposition` returned here because,
                    // even if `continuing_id` is not actually reachable, it is always
                    // referred to by the `OpLoopMerge` instruction we emitted earlier.
                    let _ = self.write_block(
                        body_id,
                        body,
                        BlockExit::Branch {
                            target: continuing_id,
                        },
                        LoopContext {
                            continuing_id: Some(continuing_id),
                            break_id: Some(merge_id),
                        },
                        debug_info,
                    )?;

                    let exit = match break_if {
                        Some(condition) => BlockExit::BreakIf {
                            condition,
                            preamble_id,
                        },
                        None => BlockExit::Branch {
                            target: preamble_id,
                        },
                    };

                    // We can ignore the `BlockExitDisposition` returned here because,
                    // even if `merge_id` is not actually reachable, it is always referred
                    // to by the `OpLoopMerge` instruction we emitted earlier.
                    let _ = self.write_block(
                        continuing_id,
                        continuing,
                        exit,
                        LoopContext {
                            continuing_id: None,
                            break_id: Some(merge_id),
                        },
                        debug_info,
                    )?;

                    block = Block::new(merge_id);
                }
                Statement::Break => {
                    self.function
                        .consume(block, Instruction::branch(loop_context.break_id.unwrap()));
                    return Ok(BlockExitDisposition::Discarded);
                }
                Statement::Continue => {
                    self.function.consume(
                        block,
                        Instruction::branch(loop_context.continuing_id.unwrap()),
                    );
                    return Ok(BlockExitDisposition::Discarded);
                }
                Statement::Return { value: Some(value) } => {
                    let value_id = self.cached[value];
                    let instruction = match self.function.entry_point_context {
                        // If this is an entry point, and we need to return anything,
                        // let's instead store the output variables and return `void`.
                        Some(ref context) => self.writer.write_entry_point_return(
                            value_id,
                            self.ir_function.result.as_ref().unwrap(),
                            &context.results,
                            &mut block.body,
                        )?,
                        None => Instruction::return_value(value_id),
                    };
                    self.function.consume(block, instruction);
                    return Ok(BlockExitDisposition::Discarded);
                }
                Statement::Return { value: None } => {
                    self.function.consume(block, Instruction::return_void());
                    return Ok(BlockExitDisposition::Discarded);
                }
                Statement::Kill => {
                    self.function.consume(block, Instruction::kill());
                    return Ok(BlockExitDisposition::Discarded);
                }
                Statement::ControlBarrier(flags) => {
                    self.writer.write_control_barrier(flags, &mut block.body);
                }
                Statement::MemoryBarrier(flags) => {
                    self.writer.write_memory_barrier(flags, &mut block);
                }
                Statement::Store { pointer, value } => {
                    let value_id = self.cached[value];
                    match self.write_access_chain(
                        pointer,
                        &mut block,
                        AccessTypeAdjustment::None,
                    )? {
                        ExpressionPointer::Ready { pointer_id } => {
                            let atomic_space = match *self.fun_info[pointer]
                                .ty
                                .inner_with(&self.ir_module.types)
                            {
                                crate::TypeInner::Pointer { base, space } => {
                                    match self.ir_module.types[base].inner {
                                        crate::TypeInner::Atomic { .. } => Some(space),
                                        _ => None,
                                    }
                                }
                                _ => None,
                            };
                            let instruction = if let Some(space) = atomic_space {
                                let (semantics, scope) = space.to_spirv_semantics_and_scope();
                                let scope_constant_id = self.get_scope_constant(scope as u32);
                                let semantics_id = self.get_index_constant(semantics.bits());
                                Instruction::atomic_store(
                                    pointer_id,
                                    scope_constant_id,
                                    semantics_id,
                                    value_id,
                                )
                            } else {
                                Instruction::store(pointer_id, value_id, None)
                            };
                            block.body.push(instruction);
                        }
                        ExpressionPointer::Conditional { condition, access } => {
                            let mut selection = Selection::start(&mut block, ());
                            selection.if_true(self, condition, ());

                            // The in-bounds path. Perform the access and the store.
                            let pointer_id = access.result_id.unwrap();
                            selection.block().body.push(access);
                            selection
                                .block()
                                .body
                                .push(Instruction::store(pointer_id, value_id, None));

                            // Finish the in-bounds block and start the merge block. This
                            // is the block we'll leave current on return.
                            selection.finish(self, ());
                        }
                    };
                }
                Statement::ImageStore {
                    image,
                    coordinate,
                    array_index,
                    value,
                } => self.write_image_store(image, coordinate, array_index, value, &mut block)?,
                Statement::Call {
                    function: local_function,
                    ref arguments,
                    result,
                } => {
                    let id = self.gen_id();
                    self.temp_list.clear();
                    for &argument in arguments {
                        self.temp_list.push(self.cached[argument]);
                    }

                    let type_id = match result {
                        Some(expr) => {
                            self.cached[expr] = id;
                            self.get_expression_type_id(&self.fun_info[expr].ty)
                        }
                        None => self.writer.void_type,
                    };

                    block.body.push(Instruction::function_call(
                        type_id,
                        id,
                        self.writer.lookup_function[&local_function],
                        &self.temp_list,
                    ));
                }
                Statement::Atomic {
                    pointer,
                    ref fun,
                    value,
                    result,
                } => {
                    let id = self.gen_id();
                    // Compare-and-exchange operations produce a struct result,
                    // so use `result`'s type if it is available. For no-result
                    // operations, fall back to `value`'s type.
                    let result_type_id =
                        self.get_expression_type_id(&self.fun_info[result.unwrap_or(value)].ty);

                    if let Some(result) = result {
                        self.cached[result] = id;
                    }

                    let pointer_id = match self.write_access_chain(
                        pointer,
                        &mut block,
                        AccessTypeAdjustment::None,
                    )? {
                        ExpressionPointer::Ready { pointer_id } => pointer_id,
                        ExpressionPointer::Conditional { .. } => {
                            return Err(Error::FeatureNotImplemented(
                                "Atomics out-of-bounds handling",
                            ));
                        }
                    };

                    let space = self.fun_info[pointer]
                        .ty
                        .inner_with(&self.ir_module.types)
                        .pointer_space()
                        .unwrap();
                    let (semantics, scope) = space.to_spirv_semantics_and_scope();
                    let scope_constant_id = self.get_scope_constant(scope as u32);
                    let semantics_id = self.get_index_constant(semantics.bits());
                    let value_id = self.cached[value];
                    let value_inner = self.fun_info[value].ty.inner_with(&self.ir_module.types);

                    let crate::TypeInner::Scalar(scalar) = *value_inner else {
                        return Err(Error::FeatureNotImplemented(
                            "Atomics with non-scalar values",
                        ));
                    };

                    let instruction = match *fun {
                        crate::AtomicFunction::Add => {
                            let spirv_op = match scalar.kind {
                                crate::ScalarKind::Sint | crate::ScalarKind::Uint => {
                                    spirv::Op::AtomicIAdd
                                }
                                crate::ScalarKind::Float => spirv::Op::AtomicFAddEXT,
                                _ => unimplemented!(),
                            };
                            Instruction::atomic_binary(
                                spirv_op,
                                result_type_id,
                                id,
                                pointer_id,
                                scope_constant_id,
                                semantics_id,
                                value_id,
                            )
                        }
                        crate::AtomicFunction::Subtract => {
                            let (spirv_op, value_id) = match scalar.kind {
                                crate::ScalarKind::Sint | crate::ScalarKind::Uint => {
                                    (spirv::Op::AtomicISub, value_id)
                                }
                                crate::ScalarKind::Float => {
                                    // HACK: SPIR-V doesn't have a atomic subtraction,
                                    // so we add the negated value instead.
                                    let neg_result_id = self.gen_id();
                                    block.body.push(Instruction::unary(
                                        spirv::Op::FNegate,
                                        result_type_id,
                                        neg_result_id,
                                        value_id,
                                    ));
                                    (spirv::Op::AtomicFAddEXT, neg_result_id)
                                }
                                _ => unimplemented!(),
                            };
                            Instruction::atomic_binary(
                                spirv_op,
                                result_type_id,
                                id,
                                pointer_id,
                                scope_constant_id,
                                semantics_id,
                                value_id,
                            )
                        }
                        crate::AtomicFunction::And => {
                            let spirv_op = match scalar.kind {
                                crate::ScalarKind::Sint | crate::ScalarKind::Uint => {
                                    spirv::Op::AtomicAnd
                                }
                                _ => unimplemented!(),
                            };
                            Instruction::atomic_binary(
                                spirv_op,
                                result_type_id,
                                id,
                                pointer_id,
                                scope_constant_id,
                                semantics_id,
                                value_id,
                            )
                        }
                        crate::AtomicFunction::InclusiveOr => {
                            let spirv_op = match scalar.kind {
                                crate::ScalarKind::Sint | crate::ScalarKind::Uint => {
                                    spirv::Op::AtomicOr
                                }
                                _ => unimplemented!(),
                            };
                            Instruction::atomic_binary(
                                spirv_op,
                                result_type_id,
                                id,
                                pointer_id,
                                scope_constant_id,
                                semantics_id,
                                value_id,
                            )
                        }
                        crate::AtomicFunction::ExclusiveOr => {
                            let spirv_op = match scalar.kind {
                                crate::ScalarKind::Sint | crate::ScalarKind::Uint => {
                                    spirv::Op::AtomicXor
                                }
                                _ => unimplemented!(),
                            };
                            Instruction::atomic_binary(
                                spirv_op,
                                result_type_id,
                                id,
                                pointer_id,
                                scope_constant_id,
                                semantics_id,
                                value_id,
                            )
                        }
                        crate::AtomicFunction::Min => {
                            let spirv_op = match scalar.kind {
                                crate::ScalarKind::Sint => spirv::Op::AtomicSMin,
                                crate::ScalarKind::Uint => spirv::Op::AtomicUMin,
                                _ => unimplemented!(),
                            };
                            Instruction::atomic_binary(
                                spirv_op,
                                result_type_id,
                                id,
                                pointer_id,
                                scope_constant_id,
                                semantics_id,
                                value_id,
                            )
                        }
                        crate::AtomicFunction::Max => {
                            let spirv_op = match scalar.kind {
                                crate::ScalarKind::Sint => spirv::Op::AtomicSMax,
                                crate::ScalarKind::Uint => spirv::Op::AtomicUMax,
                                _ => unimplemented!(),
                            };
                            Instruction::atomic_binary(
                                spirv_op,
                                result_type_id,
                                id,
                                pointer_id,
                                scope_constant_id,
                                semantics_id,
                                value_id,
                            )
                        }
                        crate::AtomicFunction::Exchange { compare: None } => {
                            Instruction::atomic_binary(
                                spirv::Op::AtomicExchange,
                                result_type_id,
                                id,
                                pointer_id,
                                scope_constant_id,
                                semantics_id,
                                value_id,
                            )
                        }
                        crate::AtomicFunction::Exchange { compare: Some(cmp) } => {
                            let scalar_type_id =
                                self.get_numeric_type_id(NumericType::Scalar(scalar));
                            let bool_type_id =
                                self.get_numeric_type_id(NumericType::Scalar(crate::Scalar::BOOL));

                            let cas_result_id = self.gen_id();
                            let equality_result_id = self.gen_id();
                            let equality_operator = match scalar.kind {
                                crate::ScalarKind::Sint | crate::ScalarKind::Uint => {
                                    spirv::Op::IEqual
                                }
                                _ => unimplemented!(),
                            };

                            let mut cas_instr = Instruction::new(spirv::Op::AtomicCompareExchange);
                            cas_instr.set_type(scalar_type_id);
                            cas_instr.set_result(cas_result_id);
                            cas_instr.add_operand(pointer_id);
                            cas_instr.add_operand(scope_constant_id);
                            cas_instr.add_operand(semantics_id); // semantics if equal
                            cas_instr.add_operand(semantics_id); // semantics if not equal
                            cas_instr.add_operand(value_id);
                            cas_instr.add_operand(self.cached[cmp]);
                            block.body.push(cas_instr);
                            block.body.push(Instruction::binary(
                                equality_operator,
                                bool_type_id,
                                equality_result_id,
                                cas_result_id,
                                self.cached[cmp],
                            ));
                            Instruction::composite_construct(
                                result_type_id,
                                id,
                                &[cas_result_id, equality_result_id],
                            )
                        }
                    };

                    block.body.push(instruction);
                }
                Statement::ImageAtomic {
                    image,
                    coordinate,
                    array_index,
                    fun,
                    value,
                } => {
                    self.write_image_atomic(
                        image,
                        coordinate,
                        array_index,
                        fun,
                        value,
                        &mut block,
                    )?;
                }
                Statement::WorkGroupUniformLoad { pointer, result } => {
                    self.writer
                        .write_control_barrier(crate::Barrier::WORK_GROUP, &mut block.body);
                    let result_type_id = self.get_expression_type_id(&self.fun_info[result].ty);
                    // Match `Expression::Load` behavior, including `OpAtomicLoad` when
                    // loading from a pointer to `atomic<T>`.
                    let id = self.write_checked_load(
                        pointer,
                        &mut block,
                        AccessTypeAdjustment::None,
                        result_type_id,
                    )?;
                    self.cached[result] = id;
                    self.writer
                        .write_control_barrier(crate::Barrier::WORK_GROUP, &mut block.body);
                }
                Statement::RayQuery { query, ref fun } => {
                    self.write_ray_query_function(query, fun, &mut block);
                }
                Statement::SubgroupBallot {
                    result,
                    ref predicate,
                } => {
                    self.write_subgroup_ballot(predicate, result, &mut block)?;
                }
                Statement::SubgroupCollectiveOperation {
                    ref op,
                    ref collective_op,
                    argument,
                    result,
                } => {
                    self.write_subgroup_operation(op, collective_op, argument, result, &mut block)?;
                }
                Statement::SubgroupGather {
                    ref mode,
                    argument,
                    result,
                } => {
                    self.write_subgroup_gather(mode, argument, result, &mut block)?;
                }
                Statement::CooperativeStore { target, ref data } => {
                    let target_id = self.cached[target];
                    let layout = if data.row_major {
                        spirv::CooperativeMatrixLayout::RowMajorKHR
                    } else {
                        spirv::CooperativeMatrixLayout::ColumnMajorKHR
                    };
                    let layout_id = self.get_index_constant(layout as u32);
                    let stride_id = self.cached[data.stride];
                    match self.write_access_chain(
                        data.pointer,
                        &mut block,
                        AccessTypeAdjustment::None,
                    )? {
                        ExpressionPointer::Ready { pointer_id } => {
                            block.body.push(Instruction::coop_store(
                                target_id, pointer_id, layout_id, stride_id,
                            ));
                        }
                        ExpressionPointer::Conditional { condition, access } => {
                            let mut selection = Selection::start(&mut block, ());
                            selection.if_true(self, condition, ());

                            // The in-bounds path. Perform the access and the store.
                            let pointer_id = access.result_id.unwrap();
                            selection.block().body.push(access);
                            selection.block().body.push(Instruction::coop_store(
                                target_id, pointer_id, layout_id, stride_id,
                            ));

                            // Finish the in-bounds block and start the merge block. This
                            // is the block we'll leave current on return.
                            selection.finish(self, ());
                        }
                    };
                }
                Statement::RayPipelineFunction(_) => unreachable!(),
            }
        }

        let termination = match exit {
            // We're generating code for the top-level Block of the function, so we
            // need to end it with some kind of return instruction.
            BlockExit::Return => match self.ir_function.result {
                Some(ref result) if self.function.entry_point_context.is_none() => {
                    let type_id = self.get_handle_type_id(result.ty);
                    let null_id = self.writer.get_constant_null(type_id);
                    Instruction::return_value(null_id)
                }
                _ => Instruction::return_void(),
            },
            BlockExit::Branch { target } => Instruction::branch(target),
            BlockExit::BreakIf {
                condition,
                preamble_id,
            } => {
                let condition_id = self.cached[condition];

                Instruction::branch_conditional(
                    condition_id,
                    loop_context.break_id.unwrap(),
                    preamble_id,
                )
            }
        };

        self.function.consume(block, termination);
        Ok(BlockExitDisposition::Used)
    }

    pub(super) fn write_function_body(
        &mut self,
        entry_id: Word,
        debug_info: Option<&DebugInfoInner>,
    ) -> Result<(), Error> {
        // We can ignore the `BlockExitDisposition` returned here because
        // `BlockExit::Return` doesn't refer to a block.
        let _ = self.write_block(
            entry_id,
            &self.ir_function.body,
            BlockExit::Return,
            LoopContext::default(),
            debug_info,
        )?;

        Ok(())
    }
}
