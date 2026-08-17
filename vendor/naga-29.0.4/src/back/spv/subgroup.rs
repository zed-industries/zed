use super::{Block, BlockContext, Error, Instruction, NumericType};
use crate::{arena::Handle, TypeInner};

impl BlockContext<'_> {
    pub(super) fn write_subgroup_ballot(
        &mut self,
        predicate: &Option<Handle<crate::Expression>>,
        result: Handle<crate::Expression>,
        block: &mut Block,
    ) -> Result<(), Error> {
        self.writer.require_any(
            "GroupNonUniformBallot",
            &[spirv::Capability::GroupNonUniformBallot],
        )?;
        let vec4_u32_type_id = self.get_numeric_type_id(NumericType::Vector {
            size: crate::VectorSize::Quad,
            scalar: crate::Scalar::U32,
        });
        let exec_scope_id = self.get_index_constant(spirv::Scope::Subgroup as u32);
        let predicate = if let Some(predicate) = *predicate {
            self.cached[predicate]
        } else {
            self.writer.get_constant_scalar(crate::Literal::Bool(true))
        };
        let id = self.gen_id();
        block.body.push(Instruction::group_non_uniform_ballot(
            vec4_u32_type_id,
            id,
            exec_scope_id,
            predicate,
        ));
        self.cached[result] = id;
        Ok(())
    }
    pub(super) fn write_subgroup_operation(
        &mut self,
        op: &crate::SubgroupOperation,
        collective_op: &crate::CollectiveOperation,
        argument: Handle<crate::Expression>,
        result: Handle<crate::Expression>,
        block: &mut Block,
    ) -> Result<(), Error> {
        use crate::SubgroupOperation as sg;
        match *op {
            sg::All | sg::Any => {
                self.writer.require_any(
                    "GroupNonUniformVote",
                    &[spirv::Capability::GroupNonUniformVote],
                )?;
            }
            _ => {
                self.writer.require_any(
                    "GroupNonUniformArithmetic",
                    &[spirv::Capability::GroupNonUniformArithmetic],
                )?;
            }
        }

        let id = self.gen_id();
        let result_ty = &self.fun_info[result].ty;
        let result_type_id = self.get_expression_type_id(result_ty);
        let result_ty_inner = result_ty.inner_with(&self.ir_module.types);

        let (is_scalar, scalar) = match *result_ty_inner {
            TypeInner::Scalar(kind) => (true, kind),
            TypeInner::Vector { scalar: kind, .. } => (false, kind),
            _ => unimplemented!(),
        };

        use crate::ScalarKind as sk;
        let spirv_op = match (scalar.kind, *op) {
            (sk::Bool, sg::All) if is_scalar => spirv::Op::GroupNonUniformAll,
            (sk::Bool, sg::Any) if is_scalar => spirv::Op::GroupNonUniformAny,
            (_, sg::All | sg::Any) => unimplemented!(),

            (sk::Sint | sk::Uint, sg::Add) => spirv::Op::GroupNonUniformIAdd,
            (sk::Float, sg::Add) => spirv::Op::GroupNonUniformFAdd,
            (sk::Sint | sk::Uint, sg::Mul) => spirv::Op::GroupNonUniformIMul,
            (sk::Float, sg::Mul) => spirv::Op::GroupNonUniformFMul,
            (sk::Sint, sg::Max) => spirv::Op::GroupNonUniformSMax,
            (sk::Uint, sg::Max) => spirv::Op::GroupNonUniformUMax,
            (sk::Float, sg::Max) => spirv::Op::GroupNonUniformFMax,
            (sk::Sint, sg::Min) => spirv::Op::GroupNonUniformSMin,
            (sk::Uint, sg::Min) => spirv::Op::GroupNonUniformUMin,
            (sk::Float, sg::Min) => spirv::Op::GroupNonUniformFMin,
            (_, sg::Add | sg::Mul | sg::Min | sg::Max) => unimplemented!(),

            (sk::Sint | sk::Uint, sg::And) => spirv::Op::GroupNonUniformBitwiseAnd,
            (sk::Sint | sk::Uint, sg::Or) => spirv::Op::GroupNonUniformBitwiseOr,
            (sk::Sint | sk::Uint, sg::Xor) => spirv::Op::GroupNonUniformBitwiseXor,
            (sk::Bool, sg::And) => spirv::Op::GroupNonUniformLogicalAnd,
            (sk::Bool, sg::Or) => spirv::Op::GroupNonUniformLogicalOr,
            (sk::Bool, sg::Xor) => spirv::Op::GroupNonUniformLogicalXor,
            (_, sg::And | sg::Or | sg::Xor) => unimplemented!(),
        };

        let exec_scope_id = self.get_index_constant(spirv::Scope::Subgroup as u32);

        use crate::CollectiveOperation as c;
        let group_op = match *op {
            sg::All | sg::Any => None,
            _ => Some(match *collective_op {
                c::Reduce => spirv::GroupOperation::Reduce,
                c::InclusiveScan => spirv::GroupOperation::InclusiveScan,
                c::ExclusiveScan => spirv::GroupOperation::ExclusiveScan,
            }),
        };

        let arg_id = self.cached[argument];
        block.body.push(Instruction::group_non_uniform_arithmetic(
            spirv_op,
            result_type_id,
            id,
            exec_scope_id,
            group_op,
            arg_id,
        ));
        self.cached[result] = id;
        Ok(())
    }
    pub(super) fn write_subgroup_gather(
        &mut self,
        mode: &crate::GatherMode,
        argument: Handle<crate::Expression>,
        result: Handle<crate::Expression>,
        block: &mut Block,
    ) -> Result<(), Error> {
        match *mode {
            crate::GatherMode::BroadcastFirst => {
                self.writer.require_any(
                    "GroupNonUniformBallot",
                    &[spirv::Capability::GroupNonUniformBallot],
                )?;
            }
            crate::GatherMode::Shuffle(_)
            | crate::GatherMode::ShuffleXor(_)
            | crate::GatherMode::Broadcast(_) => {
                self.writer.require_any(
                    "GroupNonUniformShuffle",
                    &[spirv::Capability::GroupNonUniformShuffle],
                )?;
            }
            crate::GatherMode::ShuffleDown(_) | crate::GatherMode::ShuffleUp(_) => {
                self.writer.require_any(
                    "GroupNonUniformShuffleRelative",
                    &[spirv::Capability::GroupNonUniformShuffleRelative],
                )?;
            }
            crate::GatherMode::QuadBroadcast(_) | crate::GatherMode::QuadSwap(_) => {
                self.writer.require_any(
                    "GroupNonUniformQuad",
                    &[spirv::Capability::GroupNonUniformQuad],
                )?;
            }
        }

        let id = self.gen_id();
        let result_ty = &self.fun_info[result].ty;
        let result_type_id = self.get_expression_type_id(result_ty);

        let exec_scope_id = self.get_index_constant(spirv::Scope::Subgroup as u32);

        let arg_id = self.cached[argument];
        match *mode {
            crate::GatherMode::BroadcastFirst => {
                block
                    .body
                    .push(Instruction::group_non_uniform_broadcast_first(
                        result_type_id,
                        id,
                        exec_scope_id,
                        arg_id,
                    ));
            }
            crate::GatherMode::Broadcast(index)
            | crate::GatherMode::Shuffle(index)
            | crate::GatherMode::ShuffleDown(index)
            | crate::GatherMode::ShuffleUp(index)
            | crate::GatherMode::ShuffleXor(index)
            | crate::GatherMode::QuadBroadcast(index) => {
                let index_id = self.cached[index];
                let op = match *mode {
                    crate::GatherMode::BroadcastFirst => unreachable!(),
                    // Use shuffle to emit broadcast to allow the index to
                    // be dynamically uniform on Vulkan 1.1. The argument to
                    // OpGroupNonUniformBroadcast must be a constant pre SPIR-V
                    // 1.5 (vulkan 1.2)
                    crate::GatherMode::Broadcast(_) => spirv::Op::GroupNonUniformShuffle,
                    crate::GatherMode::Shuffle(_) => spirv::Op::GroupNonUniformShuffle,
                    crate::GatherMode::ShuffleDown(_) => spirv::Op::GroupNonUniformShuffleDown,
                    crate::GatherMode::ShuffleUp(_) => spirv::Op::GroupNonUniformShuffleUp,
                    crate::GatherMode::ShuffleXor(_) => spirv::Op::GroupNonUniformShuffleXor,
                    crate::GatherMode::QuadBroadcast(_) => spirv::Op::GroupNonUniformQuadBroadcast,
                    crate::GatherMode::QuadSwap(_) => unreachable!(),
                };
                block.body.push(Instruction::group_non_uniform_gather(
                    op,
                    result_type_id,
                    id,
                    exec_scope_id,
                    arg_id,
                    index_id,
                ));
            }
            crate::GatherMode::QuadSwap(direction) => {
                let direction = self.get_index_constant(match direction {
                    crate::Direction::X => 0,
                    crate::Direction::Y => 1,
                    crate::Direction::Diagonal => 2,
                });
                block.body.push(Instruction::group_non_uniform_quad_swap(
                    result_type_id,
                    id,
                    exec_scope_id,
                    arg_id,
                    direction,
                ));
            }
        }
        self.cached[result] = id;
        Ok(())
    }
}
