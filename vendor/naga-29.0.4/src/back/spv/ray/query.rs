/*!
Generating SPIR-V for ray query operations.
*/

use alloc::{vec, vec::Vec};

use super::super::{
    Block, BlockContext, Function, FunctionArgument, Instruction, LocalType, LookupFunctionType,
    LookupRayQueryFunction, NumericType, Writer, WriterFlags,
};
use crate::{arena::Handle, back::RayQueryPoint};

/// helper function to check if a particular flag is set in a u32.
fn write_ray_flags_contains_flags(
    writer: &mut Writer,
    block: &mut Block,
    id: spirv::Word,
    flag: u32,
) -> spirv::Word {
    let bit_id = writer.get_constant_scalar(crate::Literal::U32(flag));
    let zero_id = writer.get_constant_scalar(crate::Literal::U32(0));
    let u32_type_id = writer.get_u32_type_id();
    let bool_ty = writer.get_bool_type_id();

    let and_id = writer.id_gen.next();
    block.body.push(Instruction::binary(
        spirv::Op::BitwiseAnd,
        u32_type_id,
        and_id,
        id,
        bit_id,
    ));

    let eq_id = writer.id_gen.next();
    block.body.push(Instruction::binary(
        spirv::Op::INotEqual,
        bool_ty,
        eq_id,
        and_id,
        zero_id,
    ));

    eq_id
}

impl Writer {
    /// writes a logical and of two scalar booleans
    fn write_logical_and(
        &mut self,
        block: &mut Block,
        one: spirv::Word,
        two: spirv::Word,
    ) -> spirv::Word {
        let id = self.id_gen.next();
        let bool_id = self.get_bool_type_id();
        block.body.push(Instruction::binary(
            spirv::Op::LogicalAnd,
            bool_id,
            id,
            one,
            two,
        ));
        id
    }

    fn write_reduce_and(&mut self, block: &mut Block, mut bools: Vec<spirv::Word>) -> spirv::Word {
        // The combined `and`ed together of all of the bools up to this point.
        let mut current_combined = bools.pop().unwrap();
        for boolean in bools {
            current_combined = self.write_logical_and(block, current_combined, boolean)
        }
        current_combined
    }

    // returns the id of the function, the function, and ids for its arguments.
    fn write_function_signature(
        &mut self,
        arg_types: &[spirv::Word],
        return_ty: spirv::Word,
    ) -> (spirv::Word, Function, Vec<spirv::Word>) {
        let func_ty = self.get_function_type(LookupFunctionType {
            parameter_type_ids: Vec::from(arg_types),
            return_type_id: return_ty,
        });

        let mut function = Function::default();
        let func_id = self.id_gen.next();
        function.signature = Some(Instruction::function(
            return_ty,
            func_id,
            spirv::FunctionControl::empty(),
            func_ty,
        ));

        let mut arg_ids = Vec::with_capacity(arg_types.len());

        for (idx, &arg_ty) in arg_types.iter().enumerate() {
            let id = self.id_gen.next();
            let instruction = Instruction::function_parameter(arg_ty, id);
            function.parameters.push(FunctionArgument {
                instruction,
                handle_id: idx as u32,
            });
            arg_ids.push(id);
        }
        (func_id, function, arg_ids)
    }

    pub(in super::super) fn write_ray_query_get_intersection_function(
        &mut self,
        is_committed: bool,
        ir_module: &crate::Module,
    ) -> spirv::Word {
        if let Some(&word) =
            self.ray_query_functions
                .get(&LookupRayQueryFunction::GetIntersection {
                    committed: is_committed,
                })
        {
            return word;
        }
        let ray_intersection = ir_module.special_types.ray_intersection.unwrap();
        let intersection_type_id = self.get_handle_type_id(ray_intersection);
        let intersection_pointer_type_id =
            self.get_pointer_type_id(intersection_type_id, spirv::StorageClass::Function);

        let flag_type_id = self.get_u32_type_id();
        let flag_pointer_type_id =
            self.get_pointer_type_id(flag_type_id, spirv::StorageClass::Function);

        let transform_type_id = self.get_numeric_type_id(NumericType::Matrix {
            columns: crate::VectorSize::Quad,
            rows: crate::VectorSize::Tri,
            scalar: crate::Scalar::F32,
        });
        let transform_pointer_type_id =
            self.get_pointer_type_id(transform_type_id, spirv::StorageClass::Function);

        let barycentrics_type_id = self.get_numeric_type_id(NumericType::Vector {
            size: crate::VectorSize::Bi,
            scalar: crate::Scalar::F32,
        });
        let barycentrics_pointer_type_id =
            self.get_pointer_type_id(barycentrics_type_id, spirv::StorageClass::Function);

        let bool_type_id = self.get_bool_type_id();
        let bool_pointer_type_id =
            self.get_pointer_type_id(bool_type_id, spirv::StorageClass::Function);

        let scalar_type_id = self.get_f32_type_id();
        let float_pointer_type_id = self.get_f32_pointer_type_id(spirv::StorageClass::Function);

        let argument_type_id = self.get_ray_query_pointer_id();

        let (func_id, mut function, arg_ids) = self.write_function_signature(
            &[argument_type_id, flag_pointer_type_id],
            intersection_type_id,
        );

        let query_id = arg_ids[0];
        let intersection_tracker_id = arg_ids[1];

        let label_id = self.id_gen.next();
        let mut block = Block::new(label_id);

        let blank_intersection = self.get_constant_null(intersection_type_id);
        let blank_intersection_id = self.id_gen.next();
        // This must be before everything else in the function.
        block.body.push(Instruction::variable(
            intersection_pointer_type_id,
            blank_intersection_id,
            spirv::StorageClass::Function,
            Some(blank_intersection),
        ));

        let intersection_id = self.get_constant_scalar(crate::Literal::U32(if is_committed {
            spirv::RayQueryIntersection::RayQueryCommittedIntersectionKHR
        } else {
            spirv::RayQueryIntersection::RayQueryCandidateIntersectionKHR
        } as _));

        let loaded_ray_query_tracker_id = self.id_gen.next();
        block.body.push(Instruction::load(
            flag_type_id,
            loaded_ray_query_tracker_id,
            intersection_tracker_id,
            None,
        ));
        let proceeded_id = write_ray_flags_contains_flags(
            self,
            &mut block,
            loaded_ray_query_tracker_id,
            RayQueryPoint::PROCEED.bits(),
        );
        let finished_proceed_id = write_ray_flags_contains_flags(
            self,
            &mut block,
            loaded_ray_query_tracker_id,
            RayQueryPoint::FINISHED_TRAVERSAL.bits(),
        );
        let proceed_finished_correct_id = if is_committed {
            finished_proceed_id
        } else {
            let not_finished_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::LogicalNot,
                bool_type_id,
                not_finished_id,
                finished_proceed_id,
            ));
            not_finished_id
        };

        let is_valid_id =
            self.write_logical_and(&mut block, proceed_finished_correct_id, proceeded_id);

        let valid_id = self.id_gen.next();
        let mut valid_block = Block::new(valid_id);

        let final_label_id = self.id_gen.next();
        let mut final_block = Block::new(final_label_id);

        block.body.push(Instruction::selection_merge(
            final_label_id,
            spirv::SelectionControl::NONE,
        ));
        function.consume(
            block,
            Instruction::branch_conditional(is_valid_id, valid_id, final_label_id),
        );

        let raw_kind_id = self.id_gen.next();
        valid_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionTypeKHR,
                flag_type_id,
                raw_kind_id,
                query_id,
                intersection_id,
            ));
        let kind_id = if is_committed {
            // Nothing to do: the IR value matches `spirv::RayQueryCommittedIntersectionType`
            raw_kind_id
        } else {
            // Remap from the candidate kind to IR
            let condition_id = self.id_gen.next();
            let committed_triangle_kind_id = self.get_constant_scalar(crate::Literal::U32(
                spirv::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionTriangleKHR
                    as _,
            ));
            valid_block.body.push(Instruction::binary(
                spirv::Op::IEqual,
                self.get_bool_type_id(),
                condition_id,
                raw_kind_id,
                committed_triangle_kind_id,
            ));
            let kind_id = self.id_gen.next();
            valid_block.body.push(Instruction::select(
                flag_type_id,
                kind_id,
                condition_id,
                self.get_constant_scalar(crate::Literal::U32(
                    crate::RayQueryIntersection::Triangle as _,
                )),
                self.get_constant_scalar(crate::Literal::U32(
                    crate::RayQueryIntersection::Aabb as _,
                )),
            ));
            kind_id
        };
        let idx_id = self.get_index_constant(0);
        let access_idx = self.id_gen.next();
        valid_block.body.push(Instruction::access_chain(
            flag_pointer_type_id,
            access_idx,
            blank_intersection_id,
            &[idx_id],
        ));
        valid_block
            .body
            .push(Instruction::store(access_idx, kind_id, None));

        let not_none_comp_id = self.id_gen.next();
        let none_id =
            self.get_constant_scalar(crate::Literal::U32(crate::RayQueryIntersection::None as _));
        valid_block.body.push(Instruction::binary(
            spirv::Op::INotEqual,
            self.get_bool_type_id(),
            not_none_comp_id,
            kind_id,
            none_id,
        ));

        let not_none_label_id = self.id_gen.next();
        let mut not_none_block = Block::new(not_none_label_id);

        let outer_merge_label_id = self.id_gen.next();
        let outer_merge_block = Block::new(outer_merge_label_id);

        valid_block.body.push(Instruction::selection_merge(
            outer_merge_label_id,
            spirv::SelectionControl::NONE,
        ));
        function.consume(
            valid_block,
            Instruction::branch_conditional(
                not_none_comp_id,
                not_none_label_id,
                outer_merge_label_id,
            ),
        );

        let instance_custom_index_id = self.id_gen.next();
        not_none_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionInstanceCustomIndexKHR,
                flag_type_id,
                instance_custom_index_id,
                query_id,
                intersection_id,
            ));
        let instance_id = self.id_gen.next();
        not_none_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionInstanceIdKHR,
                flag_type_id,
                instance_id,
                query_id,
                intersection_id,
            ));
        let sbt_record_offset_id = self.id_gen.next();
        not_none_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR,
                flag_type_id,
                sbt_record_offset_id,
                query_id,
                intersection_id,
            ));
        let geometry_index_id = self.id_gen.next();
        not_none_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionGeometryIndexKHR,
                flag_type_id,
                geometry_index_id,
                query_id,
                intersection_id,
            ));
        let primitive_index_id = self.id_gen.next();
        not_none_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionPrimitiveIndexKHR,
                flag_type_id,
                primitive_index_id,
                query_id,
                intersection_id,
            ));

        //Note: there is also `OpRayQueryGetIntersectionCandidateAABBOpaqueKHR`,
        // but it's not a property of an intersection.

        let object_to_world_id = self.id_gen.next();
        not_none_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionObjectToWorldKHR,
                transform_type_id,
                object_to_world_id,
                query_id,
                intersection_id,
            ));
        let world_to_object_id = self.id_gen.next();
        not_none_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionWorldToObjectKHR,
                transform_type_id,
                world_to_object_id,
                query_id,
                intersection_id,
            ));

        // instance custom index
        let idx_id = self.get_index_constant(2);
        let access_idx = self.id_gen.next();
        not_none_block.body.push(Instruction::access_chain(
            flag_pointer_type_id,
            access_idx,
            blank_intersection_id,
            &[idx_id],
        ));
        not_none_block.body.push(Instruction::store(
            access_idx,
            instance_custom_index_id,
            None,
        ));

        // instance
        let idx_id = self.get_index_constant(3);
        let access_idx = self.id_gen.next();
        not_none_block.body.push(Instruction::access_chain(
            flag_pointer_type_id,
            access_idx,
            blank_intersection_id,
            &[idx_id],
        ));
        not_none_block
            .body
            .push(Instruction::store(access_idx, instance_id, None));

        let idx_id = self.get_index_constant(4);
        let access_idx = self.id_gen.next();
        not_none_block.body.push(Instruction::access_chain(
            flag_pointer_type_id,
            access_idx,
            blank_intersection_id,
            &[idx_id],
        ));
        not_none_block
            .body
            .push(Instruction::store(access_idx, sbt_record_offset_id, None));

        let idx_id = self.get_index_constant(5);
        let access_idx = self.id_gen.next();
        not_none_block.body.push(Instruction::access_chain(
            flag_pointer_type_id,
            access_idx,
            blank_intersection_id,
            &[idx_id],
        ));
        not_none_block
            .body
            .push(Instruction::store(access_idx, geometry_index_id, None));

        let idx_id = self.get_index_constant(6);
        let access_idx = self.id_gen.next();
        not_none_block.body.push(Instruction::access_chain(
            flag_pointer_type_id,
            access_idx,
            blank_intersection_id,
            &[idx_id],
        ));
        not_none_block
            .body
            .push(Instruction::store(access_idx, primitive_index_id, None));

        let idx_id = self.get_index_constant(9);
        let access_idx = self.id_gen.next();
        not_none_block.body.push(Instruction::access_chain(
            transform_pointer_type_id,
            access_idx,
            blank_intersection_id,
            &[idx_id],
        ));
        not_none_block
            .body
            .push(Instruction::store(access_idx, object_to_world_id, None));

        let idx_id = self.get_index_constant(10);
        let access_idx = self.id_gen.next();
        not_none_block.body.push(Instruction::access_chain(
            transform_pointer_type_id,
            access_idx,
            blank_intersection_id,
            &[idx_id],
        ));
        not_none_block
            .body
            .push(Instruction::store(access_idx, world_to_object_id, None));

        let tri_comp_id = self.id_gen.next();
        let tri_id = self.get_constant_scalar(crate::Literal::U32(
            crate::RayQueryIntersection::Triangle as _,
        ));
        not_none_block.body.push(Instruction::binary(
            spirv::Op::IEqual,
            self.get_bool_type_id(),
            tri_comp_id,
            kind_id,
            tri_id,
        ));

        let tri_label_id = self.id_gen.next();
        let mut tri_block = Block::new(tri_label_id);

        let merge_label_id = self.id_gen.next();
        let merge_block = Block::new(merge_label_id);
        // t
        {
            let block = if is_committed {
                &mut not_none_block
            } else {
                &mut tri_block
            };
            let t_id = self.id_gen.next();
            block.body.push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionTKHR,
                scalar_type_id,
                t_id,
                query_id,
                intersection_id,
            ));
            let idx_id = self.get_index_constant(1);
            let access_idx = self.id_gen.next();
            block.body.push(Instruction::access_chain(
                float_pointer_type_id,
                access_idx,
                blank_intersection_id,
                &[idx_id],
            ));
            block.body.push(Instruction::store(access_idx, t_id, None));
        }
        not_none_block.body.push(Instruction::selection_merge(
            merge_label_id,
            spirv::SelectionControl::NONE,
        ));
        function.consume(
            not_none_block,
            Instruction::branch_conditional(not_none_comp_id, tri_label_id, merge_label_id),
        );

        let barycentrics_id = self.id_gen.next();
        tri_block.body.push(Instruction::ray_query_get_intersection(
            spirv::Op::RayQueryGetIntersectionBarycentricsKHR,
            barycentrics_type_id,
            barycentrics_id,
            query_id,
            intersection_id,
        ));

        let front_face_id = self.id_gen.next();
        tri_block.body.push(Instruction::ray_query_get_intersection(
            spirv::Op::RayQueryGetIntersectionFrontFaceKHR,
            bool_type_id,
            front_face_id,
            query_id,
            intersection_id,
        ));

        let idx_id = self.get_index_constant(7);
        let access_idx = self.id_gen.next();
        tri_block.body.push(Instruction::access_chain(
            barycentrics_pointer_type_id,
            access_idx,
            blank_intersection_id,
            &[idx_id],
        ));
        tri_block
            .body
            .push(Instruction::store(access_idx, barycentrics_id, None));

        let idx_id = self.get_index_constant(8);
        let access_idx = self.id_gen.next();
        tri_block.body.push(Instruction::access_chain(
            bool_pointer_type_id,
            access_idx,
            blank_intersection_id,
            &[idx_id],
        ));
        tri_block
            .body
            .push(Instruction::store(access_idx, front_face_id, None));
        function.consume(tri_block, Instruction::branch(merge_label_id));
        function.consume(merge_block, Instruction::branch(outer_merge_label_id));
        function.consume(outer_merge_block, Instruction::branch(final_label_id));

        let loaded_blank_intersection_id = self.id_gen.next();
        final_block.body.push(Instruction::load(
            intersection_type_id,
            loaded_blank_intersection_id,
            blank_intersection_id,
            None,
        ));
        function.consume(
            final_block,
            Instruction::return_value(loaded_blank_intersection_id),
        );

        function.to_words(&mut self.logical_layout.function_definitions);
        self.ray_query_functions.insert(
            LookupRayQueryFunction::GetIntersection {
                committed: is_committed,
            },
            func_id,
        );
        func_id
    }

    fn write_ray_query_initialize(&mut self, ir_module: &crate::Module) -> spirv::Word {
        if let Some(&word) = self
            .ray_query_functions
            .get(&LookupRayQueryFunction::Initialize)
        {
            return word;
        }

        let ray_query_type_id = self.get_ray_query_pointer_id();
        let acceleration_structure_type_id =
            self.get_localtype_id(LocalType::AccelerationStructure);
        let ray_desc_type_id = self.get_handle_type_id(
            ir_module
                .special_types
                .ray_desc
                .expect("ray desc should be set if ray queries are being initialized"),
        );

        let u32_ty = self.get_u32_type_id();
        let u32_ptr_ty = self.get_pointer_type_id(u32_ty, spirv::StorageClass::Function);

        let f32_type_id = self.get_f32_type_id();
        let f32_ptr_ty = self.get_pointer_type_id(f32_type_id, spirv::StorageClass::Function);

        let bool_type_id = self.get_bool_type_id();
        let bool_vec3_type_id = self.get_vec3_bool_type_id();

        let (func_id, mut function, arg_ids) = self.write_function_signature(
            &[
                ray_query_type_id,
                acceleration_structure_type_id,
                ray_desc_type_id,
                u32_ptr_ty,
                f32_ptr_ty,
            ],
            self.void_type,
        );

        let query_id = arg_ids[0];
        let acceleration_structure_id = arg_ids[1];
        let desc_id = arg_ids[2];
        let init_tracker_id = arg_ids[3];
        let t_max_tracker_id = arg_ids[4];

        let label_id = self.id_gen.next();
        let mut block = Block::new(label_id);

        let flag_type_id = self.get_numeric_type_id(NumericType::Scalar(crate::Scalar::U32));

        //Note: composite extract indices and types must match `generate_ray_desc_type`
        let ray_flags_id = self.id_gen.next();
        block.body.push(Instruction::composite_extract(
            flag_type_id,
            ray_flags_id,
            desc_id,
            &[0],
        ));
        let cull_mask_id = self.id_gen.next();
        block.body.push(Instruction::composite_extract(
            flag_type_id,
            cull_mask_id,
            desc_id,
            &[1],
        ));

        let tmin_id = self.id_gen.next();
        block.body.push(Instruction::composite_extract(
            f32_type_id,
            tmin_id,
            desc_id,
            &[2],
        ));
        let tmax_id = self.id_gen.next();
        block.body.push(Instruction::composite_extract(
            f32_type_id,
            tmax_id,
            desc_id,
            &[3],
        ));
        block
            .body
            .push(Instruction::store(t_max_tracker_id, tmax_id, None));

        let vector_type_id = self.get_numeric_type_id(NumericType::Vector {
            size: crate::VectorSize::Tri,
            scalar: crate::Scalar::F32,
        });
        let ray_origin_id = self.id_gen.next();
        block.body.push(Instruction::composite_extract(
            vector_type_id,
            ray_origin_id,
            desc_id,
            &[4],
        ));
        let ray_dir_id = self.id_gen.next();
        block.body.push(Instruction::composite_extract(
            vector_type_id,
            ray_dir_id,
            desc_id,
            &[5],
        ));

        let valid_id = self.ray_query_initialization_tracking.then(||{
            let tmin_le_tmax_id = self.id_gen.next();
            // Check both that tmin is less than or equal to tmax (https://docs.vulkan.org/spec/latest/appendices/spirvenv.html#VUID-RuntimeSpirv-OpRayQueryInitializeKHR-06350)
            // and implicitly that neither tmin or tmax are NaN (https://docs.vulkan.org/spec/latest/appendices/spirvenv.html#VUID-RuntimeSpirv-OpRayQueryInitializeKHR-06351)
            // because this checks if tmin and tmax are ordered too (i.e: not NaN).
            block.body.push(Instruction::binary(
                spirv::Op::FOrdLessThanEqual,
                bool_type_id,
                tmin_le_tmax_id,
                tmin_id,
                tmax_id,
            ));

            // Check that tmin is greater than or equal to 0 (and
            // therefore also tmax is too because it is greater than
            // or equal to tmin) (https://docs.vulkan.org/spec/latest/appendices/spirvenv.html#VUID-RuntimeSpirv-OpRayQueryInitializeKHR-06349).
            let tmin_ge_zero_id = self.id_gen.next();
            let zero_id = self.get_constant_scalar(crate::Literal::F32(0.0));
            block.body.push(Instruction::binary(
                spirv::Op::FOrdGreaterThanEqual,
                bool_type_id,
                tmin_ge_zero_id,
                tmin_id,
                zero_id,
            ));

            // Check that ray origin is finite (https://docs.vulkan.org/spec/latest/appendices/spirvenv.html#VUID-RuntimeSpirv-OpRayQueryInitializeKHR-06348)
            let ray_origin_infinite_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::IsInf,
                bool_vec3_type_id,
                ray_origin_infinite_id,
                ray_origin_id,
            ));
            let any_ray_origin_infinite_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::Any,
                bool_type_id,
                any_ray_origin_infinite_id,
                ray_origin_infinite_id,
            ));

            let ray_origin_nan_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::IsNan,
                bool_vec3_type_id,
                ray_origin_nan_id,
                ray_origin_id,
            ));
            let any_ray_origin_nan_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::Any,
                bool_type_id,
                any_ray_origin_nan_id,
                ray_origin_nan_id,
            ));

            let ray_origin_not_finite_id = self.id_gen.next();
            block.body.push(Instruction::binary(
                spirv::Op::LogicalOr,
                bool_type_id,
                ray_origin_not_finite_id,
                any_ray_origin_nan_id,
                any_ray_origin_infinite_id,
            ));

            let all_ray_origin_finite_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::LogicalNot,
                bool_type_id,
                all_ray_origin_finite_id,
                ray_origin_not_finite_id,
            ));

            // Check that ray direction is finite (https://docs.vulkan.org/spec/latest/appendices/spirvenv.html#VUID-RuntimeSpirv-OpRayQueryInitializeKHR-06348)
            let ray_dir_infinite_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::IsInf,
                bool_vec3_type_id,
                ray_dir_infinite_id,
                ray_dir_id,
            ));
            let any_ray_dir_infinite_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::Any,
                bool_type_id,
                any_ray_dir_infinite_id,
                ray_dir_infinite_id,
            ));

            let ray_dir_nan_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::IsNan,
                bool_vec3_type_id,
                ray_dir_nan_id,
                ray_dir_id,
            ));
            let any_ray_dir_nan_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::Any,
                bool_type_id,
                any_ray_dir_nan_id,
                ray_dir_nan_id,
            ));

            let ray_dir_not_finite_id = self.id_gen.next();
            block.body.push(Instruction::binary(
                spirv::Op::LogicalOr,
                bool_type_id,
                ray_dir_not_finite_id,
                any_ray_dir_nan_id,
                any_ray_dir_infinite_id,
            ));

            let all_ray_dir_finite_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::LogicalNot,
                bool_type_id,
                all_ray_dir_finite_id,
                ray_dir_not_finite_id,
            ));

            /// Writes spirv to check that less than two booleans are true
            ///
            /// For each boolean: removes it, `and`s it with all others (i.e for all possible combinations of two booleans in the list checks to see if both are true).
            /// Then `or`s all of these checks together. This produces whether two or more booleans are true.
            fn write_less_than_2_true(
                writer: &mut Writer,
                block: &mut Block,
                mut bools: Vec<spirv::Word>,
            ) -> spirv::Word {
                assert!(bools.len() > 1, "Must have multiple booleans!");
                let bool_ty = writer.get_bool_type_id();
                let mut each_two_true = Vec::new();
                while let Some(last_bool) = bools.pop() {
                    for &bool in &bools {
                        let both_true_id = writer.write_logical_and(
                            block,
                            last_bool,
                            bool,
                        );
                        each_two_true.push(both_true_id);
                    }
                }
                let mut all_or_id = each_two_true.pop().expect("since this must have multiple booleans, there must be at least one thing in `each_two_true`");
                for two_true in each_two_true {
                    let new_all_or_id = writer.id_gen.next();
                    block.body.push(Instruction::binary(
                        spirv::Op::LogicalOr,
                        bool_ty,
                        new_all_or_id,
                        all_or_id,
                        two_true,
                    ));
                    all_or_id = new_all_or_id;
                }

                let less_than_two_id = writer.id_gen.next();
                block.body.push(Instruction::unary(
                    spirv::Op::LogicalNot,
                    bool_ty,
                    less_than_two_id,
                    all_or_id,
                ));
                less_than_two_id
            }

            // Check that at most one of skip triangles and skip AABBs is
            // present (https://docs.vulkan.org/spec/latest/appendices/spirvenv.html#VUID-RuntimeSpirv-OpRayQueryInitializeKHR-06889)
            let contains_skip_triangles = write_ray_flags_contains_flags(
                self,
                &mut block,
                ray_flags_id,
                crate::RayFlag::SKIP_TRIANGLES.bits(),
            );
            let contains_skip_aabbs = write_ray_flags_contains_flags(
                self,
                &mut block,
                ray_flags_id,
                crate::RayFlag::SKIP_AABBS.bits(),
            );

            let not_contain_skip_triangles_aabbs = write_less_than_2_true(
                self,
                &mut block,
                vec![contains_skip_triangles, contains_skip_aabbs],
            );

            // Check that at most one of skip triangles (taken from above check),
            // cull back facing, and cull front face is present (https://docs.vulkan.org/spec/latest/appendices/spirvenv.html#VUID-RuntimeSpirv-OpRayQueryInitializeKHR-06890)
            let contains_cull_back = write_ray_flags_contains_flags(
                self,
                &mut block,
                ray_flags_id,
                crate::RayFlag::CULL_BACK_FACING.bits(),
            );
            let contains_cull_front = write_ray_flags_contains_flags(
                self,
                &mut block,
                ray_flags_id,
                crate::RayFlag::CULL_FRONT_FACING.bits(),
            );

            let not_contain_skip_triangles_cull = write_less_than_2_true(
                self,
                &mut block,
                vec![
                    contains_skip_triangles,
                    contains_cull_back,
                    contains_cull_front,
                ],
            );

            // Check that at most one of force opaque, force not opaque, cull opaque,
            // and cull not opaque are present (https://docs.vulkan.org/spec/latest/appendices/spirvenv.html#VUID-RuntimeSpirv-OpRayQueryInitializeKHR-06891)
            let contains_opaque = write_ray_flags_contains_flags(
                self,
                &mut block,
                ray_flags_id,
                crate::RayFlag::FORCE_OPAQUE.bits(),
            );
            let contains_no_opaque = write_ray_flags_contains_flags(
                self,
                &mut block,
                ray_flags_id,
                crate::RayFlag::FORCE_NO_OPAQUE.bits(),
            );
            let contains_cull_opaque = write_ray_flags_contains_flags(
                self,
                &mut block,
                ray_flags_id,
                crate::RayFlag::CULL_OPAQUE.bits(),
            );
            let contains_cull_no_opaque = write_ray_flags_contains_flags(
                self,
                &mut block,
                ray_flags_id,
                crate::RayFlag::CULL_NO_OPAQUE.bits(),
            );

            let not_contain_multiple_opaque = write_less_than_2_true(
                self,
                &mut block,
                vec![
                    contains_opaque,
                    contains_no_opaque,
                    contains_cull_opaque,
                    contains_cull_no_opaque,
                ],
            );

            // Combine all checks into a single flag saying whether the call is valid or not.
            self.write_reduce_and(
                &mut block,
                vec![
                    tmin_le_tmax_id,
                    tmin_ge_zero_id,
                    all_ray_origin_finite_id,
                    all_ray_dir_finite_id,
                    not_contain_skip_triangles_aabbs,
                    not_contain_skip_triangles_cull,
                    not_contain_multiple_opaque,
                ],
            )
        });

        let merge_label_id = self.id_gen.next();
        let merge_block = Block::new(merge_label_id);

        // NOTE: this block will be unreachable if initialization tracking is disabled.
        let invalid_label_id = self.id_gen.next();
        let mut invalid_block = Block::new(invalid_label_id);

        let valid_label_id = self.id_gen.next();
        let mut valid_block = Block::new(valid_label_id);

        match valid_id {
            Some(all_valid_id) => {
                block.body.push(Instruction::selection_merge(
                    merge_label_id,
                    spirv::SelectionControl::NONE,
                ));
                function.consume(
                    block,
                    Instruction::branch_conditional(all_valid_id, valid_label_id, invalid_label_id),
                );
            }
            None => {
                function.consume(block, Instruction::branch(valid_label_id));
            }
        }

        valid_block.body.push(Instruction::ray_query_initialize(
            query_id,
            acceleration_structure_id,
            ray_flags_id,
            cull_mask_id,
            ray_origin_id,
            tmin_id,
            ray_dir_id,
            tmax_id,
        ));

        let const_initialized =
            self.get_constant_scalar(crate::Literal::U32(RayQueryPoint::INITIALIZED.bits()));
        valid_block
            .body
            .push(Instruction::store(init_tracker_id, const_initialized, None));

        function.consume(valid_block, Instruction::branch(merge_label_id));

        if self
            .flags
            .contains(WriterFlags::PRINT_ON_RAY_QUERY_INITIALIZATION_FAIL)
        {
            self.write_debug_printf(
                &mut invalid_block,
                "Naga ignored invalid arguments to rayQueryInitialize with flags: %u t_min: %f t_max: %f origin: %v4f dir: %v4f",
                &[
                    ray_flags_id,
                    tmin_id,
                    tmax_id,
                    ray_origin_id,
                    ray_dir_id,
                ],
            );
        }

        function.consume(invalid_block, Instruction::branch(merge_label_id));

        function.consume(merge_block, Instruction::return_void());

        function.to_words(&mut self.logical_layout.function_definitions);

        self.ray_query_functions
            .insert(LookupRayQueryFunction::Initialize, func_id);
        func_id
    }

    fn write_ray_query_proceed(&mut self) -> spirv::Word {
        if let Some(&word) = self
            .ray_query_functions
            .get(&LookupRayQueryFunction::Proceed)
        {
            return word;
        }

        let ray_query_type_id = self.get_ray_query_pointer_id();

        let u32_ty = self.get_u32_type_id();
        let u32_ptr_ty = self.get_pointer_type_id(u32_ty, spirv::StorageClass::Function);

        let bool_type_id = self.get_bool_type_id();
        let bool_ptr_ty = self.get_pointer_type_id(bool_type_id, spirv::StorageClass::Function);

        let (func_id, mut function, arg_ids) =
            self.write_function_signature(&[ray_query_type_id, u32_ptr_ty], bool_type_id);

        let query_id = arg_ids[0];
        let init_tracker_id = arg_ids[1];

        let block_id = self.id_gen.next();
        let mut block = Block::new(block_id);

        // TODO: perhaps this could be replaced with an OpPhi?
        let proceeded_id = self.id_gen.next();
        let const_false = self.get_constant_scalar(crate::Literal::Bool(false));
        block.body.push(Instruction::variable(
            bool_ptr_ty,
            proceeded_id,
            spirv::StorageClass::Function,
            Some(const_false),
        ));

        let initialized_tracker_id = self.id_gen.next();
        block.body.push(Instruction::load(
            u32_ty,
            initialized_tracker_id,
            init_tracker_id,
            None,
        ));

        let merge_id = self.id_gen.next();
        let mut merge_block = Block::new(merge_id);

        let valid_block_id = self.id_gen.next();
        let mut valid_block = Block::new(valid_block_id);

        let instruction = if self.ray_query_initialization_tracking {
            let is_initialized = write_ray_flags_contains_flags(
                self,
                &mut block,
                initialized_tracker_id,
                RayQueryPoint::INITIALIZED.bits(),
            );

            block.body.push(Instruction::selection_merge(
                merge_id,
                spirv::SelectionControl::NONE,
            ));

            Instruction::branch_conditional(is_initialized, valid_block_id, merge_id)
        } else {
            Instruction::branch(valid_block_id)
        };

        function.consume(block, instruction);

        let has_proceeded = self.id_gen.next();
        valid_block.body.push(Instruction::ray_query_proceed(
            bool_type_id,
            has_proceeded,
            query_id,
        ));

        valid_block
            .body
            .push(Instruction::store(proceeded_id, has_proceeded, None));

        let add_flag_finished = self.get_constant_scalar(crate::Literal::U32(
            (RayQueryPoint::PROCEED | RayQueryPoint::FINISHED_TRAVERSAL).bits(),
        ));
        let add_flag_continuing =
            self.get_constant_scalar(crate::Literal::U32(RayQueryPoint::PROCEED.bits()));

        let add_flags_id = self.id_gen.next();
        valid_block.body.push(Instruction::select(
            u32_ty,
            add_flags_id,
            has_proceeded,
            add_flag_continuing,
            add_flag_finished,
        ));
        let final_flags = self.id_gen.next();
        valid_block.body.push(Instruction::binary(
            spirv::Op::BitwiseOr,
            u32_ty,
            final_flags,
            initialized_tracker_id,
            add_flags_id,
        ));
        valid_block
            .body
            .push(Instruction::store(init_tracker_id, final_flags, None));

        function.consume(valid_block, Instruction::branch(merge_id));

        let loaded_proceeded_id = self.id_gen.next();
        merge_block.body.push(Instruction::load(
            bool_type_id,
            loaded_proceeded_id,
            proceeded_id,
            None,
        ));

        function.consume(merge_block, Instruction::return_value(loaded_proceeded_id));

        function.to_words(&mut self.logical_layout.function_definitions);

        self.ray_query_functions
            .insert(LookupRayQueryFunction::Proceed, func_id);
        func_id
    }

    fn write_ray_query_generate_intersection(&mut self) -> spirv::Word {
        if let Some(&word) = self
            .ray_query_functions
            .get(&LookupRayQueryFunction::GenerateIntersection)
        {
            return word;
        }

        let ray_query_type_id = self.get_ray_query_pointer_id();

        let u32_ty = self.get_u32_type_id();
        let u32_ptr_ty = self.get_pointer_type_id(u32_ty, spirv::StorageClass::Function);

        let f32_type_id = self.get_f32_type_id();
        let f32_ptr_type_id = self.get_pointer_type_id(f32_type_id, spirv::StorageClass::Function);

        let bool_type_id = self.get_bool_type_id();

        let (func_id, mut function, arg_ids) = self.write_function_signature(
            &[ray_query_type_id, u32_ptr_ty, f32_type_id, f32_ptr_type_id],
            self.void_type,
        );

        let query_id = arg_ids[0];
        let init_tracker_id = arg_ids[1];
        let depth_id = arg_ids[2];
        let t_max_tracker_id = arg_ids[3];

        let block_id = self.id_gen.next();
        let mut block = Block::new(block_id);

        let current_t = self.id_gen.next();
        block.body.push(Instruction::variable(
            f32_ptr_type_id,
            current_t,
            spirv::StorageClass::Function,
            None,
        ));

        let current_t = self.id_gen.next();
        block.body.push(Instruction::variable(
            f32_ptr_type_id,
            current_t,
            spirv::StorageClass::Function,
            None,
        ));

        let valid_id = self.id_gen.next();
        let mut valid_block = Block::new(valid_id);

        let final_label_id = self.id_gen.next();
        let final_block = Block::new(final_label_id);

        let instruction = if self.ray_query_initialization_tracking {
            let initialized_tracker_id = self.id_gen.next();
            block.body.push(Instruction::load(
                u32_ty,
                initialized_tracker_id,
                init_tracker_id,
                None,
            ));

            let proceeded_id = write_ray_flags_contains_flags(
                self,
                &mut block,
                initialized_tracker_id,
                RayQueryPoint::PROCEED.bits(),
            );
            let finished_proceed_id = write_ray_flags_contains_flags(
                self,
                &mut block,
                initialized_tracker_id,
                RayQueryPoint::FINISHED_TRAVERSAL.bits(),
            );

            // Can't find anything to suggest double calling this function is invalid.

            let not_finished_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::LogicalNot,
                bool_type_id,
                not_finished_id,
                finished_proceed_id,
            ));

            let is_valid_id = self.write_logical_and(&mut block, not_finished_id, proceeded_id);

            block.body.push(Instruction::selection_merge(
                final_label_id,
                spirv::SelectionControl::NONE,
            ));

            Instruction::branch_conditional(is_valid_id, valid_id, final_label_id)
        } else {
            Instruction::branch(valid_id)
        };

        function.consume(block, instruction);

        let intersection_id = self.get_constant_scalar(crate::Literal::U32(
            spirv::RayQueryIntersection::RayQueryCandidateIntersectionKHR as _,
        ));
        let committed_intersection_id = self.get_constant_scalar(crate::Literal::U32(
            spirv::RayQueryIntersection::RayQueryCommittedIntersectionKHR as _,
        ));
        let raw_kind_id = self.id_gen.next();
        valid_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionTypeKHR,
                u32_ty,
                raw_kind_id,
                query_id,
                intersection_id,
            ));

        let candidate_aabb_id = self.get_constant_scalar(crate::Literal::U32(
            spirv::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionAABBKHR as _,
        ));
        let intersection_aabb_id = self.id_gen.next();
        valid_block.body.push(Instruction::binary(
            spirv::Op::IEqual,
            bool_type_id,
            intersection_aabb_id,
            raw_kind_id,
            candidate_aabb_id,
        ));

        // Check that the provided t value is between t min and the current committed
        // t value, (https://docs.vulkan.org/spec/latest/appendices/spirvenv.html#VUID-RuntimeSpirv-OpRayQueryGenerateIntersectionKHR-06353)

        // Get the tmin
        let t_min_id = self.id_gen.next();
        valid_block.body.push(Instruction::ray_query_get_t_min(
            f32_type_id,
            t_min_id,
            query_id,
        ));

        // Get the current committed t, or tmax if no hit.
        // Basically emulate HLSL's (easier) version
        // Pseudo-code:
        // ````wgsl
        // // start of function
        // var current_t:f32;
        // ...
        // let committed_type_id = RayQueryGetIntersectionTypeKHR<Committed>(query_id);
        // if committed_type_id == Committed_None {
        //     current_t = load(t_max_tracker);
        // } else {
        //     current_t = RayQueryGetIntersectionTKHR<Committed>(query_id);
        // }
        // ...
        // ````

        let committed_type_id = self.id_gen.next();
        valid_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionTypeKHR,
                u32_ty,
                committed_type_id,
                query_id,
                committed_intersection_id,
            ));

        let no_committed = self.id_gen.next();
        valid_block.body.push(Instruction::binary(
            spirv::Op::IEqual,
            bool_type_id,
            no_committed,
            committed_type_id,
            self.get_constant_scalar(crate::Literal::U32(
                spirv::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionNoneKHR as _,
            )),
        ));

        let next_valid_block_id = self.id_gen.next();
        let no_committed_block_id = self.id_gen.next();
        let mut no_committed_block = Block::new(no_committed_block_id);
        let committed_block_id = self.id_gen.next();
        let mut committed_block = Block::new(committed_block_id);
        valid_block.body.push(Instruction::selection_merge(
            next_valid_block_id,
            spirv::SelectionControl::NONE,
        ));
        function.consume(
            valid_block,
            Instruction::branch_conditional(
                no_committed,
                no_committed_block_id,
                committed_block_id,
            ),
        );

        // Assign t_max to current_t
        let t_max_id = self.id_gen.next();
        no_committed_block.body.push(Instruction::load(
            f32_type_id,
            t_max_id,
            t_max_tracker_id,
            None,
        ));
        no_committed_block
            .body
            .push(Instruction::store(current_t, t_max_id, None));
        function.consume(no_committed_block, Instruction::branch(next_valid_block_id));

        // Assign t_current to current_t
        let latest_t_id = self.id_gen.next();
        committed_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionTKHR,
                f32_type_id,
                latest_t_id,
                query_id,
                intersection_id,
            ));
        committed_block
            .body
            .push(Instruction::store(current_t, latest_t_id, None));
        function.consume(committed_block, Instruction::branch(next_valid_block_id));

        let mut valid_block = Block::new(next_valid_block_id);

        let t_ge_t_min = self.id_gen.next();
        valid_block.body.push(Instruction::binary(
            spirv::Op::FOrdGreaterThanEqual,
            bool_type_id,
            t_ge_t_min,
            depth_id,
            t_min_id,
        ));
        let t_current = self.id_gen.next();
        valid_block
            .body
            .push(Instruction::load(f32_type_id, t_current, current_t, None));
        let t_le_t_current = self.id_gen.next();
        valid_block.body.push(Instruction::binary(
            spirv::Op::FOrdLessThanEqual,
            bool_type_id,
            t_le_t_current,
            depth_id,
            t_current,
        ));

        let t_in_range = self.id_gen.next();
        valid_block.body.push(Instruction::binary(
            spirv::Op::LogicalAnd,
            bool_type_id,
            t_in_range,
            t_ge_t_min,
            t_le_t_current,
        ));

        let call_valid_id = self.id_gen.next();
        valid_block.body.push(Instruction::binary(
            spirv::Op::LogicalAnd,
            bool_type_id,
            call_valid_id,
            t_in_range,
            intersection_aabb_id,
        ));

        let generate_label_id = self.id_gen.next();
        let mut generate_block = Block::new(generate_label_id);

        let merge_label_id = self.id_gen.next();
        let merge_block = Block::new(merge_label_id);

        valid_block.body.push(Instruction::selection_merge(
            merge_label_id,
            spirv::SelectionControl::NONE,
        ));
        function.consume(
            valid_block,
            Instruction::branch_conditional(call_valid_id, generate_label_id, merge_label_id),
        );

        generate_block
            .body
            .push(Instruction::ray_query_generate_intersection(
                query_id, depth_id,
            ));

        function.consume(generate_block, Instruction::branch(merge_label_id));
        function.consume(merge_block, Instruction::branch(final_label_id));

        function.consume(final_block, Instruction::return_void());

        function.to_words(&mut self.logical_layout.function_definitions);

        self.ray_query_functions
            .insert(LookupRayQueryFunction::GenerateIntersection, func_id);
        func_id
    }

    fn write_ray_query_confirm_intersection(&mut self) -> spirv::Word {
        if let Some(&word) = self
            .ray_query_functions
            .get(&LookupRayQueryFunction::ConfirmIntersection)
        {
            return word;
        }

        let ray_query_type_id = self.get_ray_query_pointer_id();

        let u32_ty = self.get_u32_type_id();
        let u32_ptr_ty = self.get_pointer_type_id(u32_ty, spirv::StorageClass::Function);

        let bool_type_id = self.get_bool_type_id();

        let (func_id, mut function, arg_ids) =
            self.write_function_signature(&[ray_query_type_id, u32_ptr_ty], self.void_type);

        let query_id = arg_ids[0];
        let init_tracker_id = arg_ids[1];

        let block_id = self.id_gen.next();
        let mut block = Block::new(block_id);

        let valid_id = self.id_gen.next();
        let mut valid_block = Block::new(valid_id);

        let final_label_id = self.id_gen.next();
        let final_block = Block::new(final_label_id);

        let instruction = if self.ray_query_initialization_tracking {
            let initialized_tracker_id = self.id_gen.next();
            block.body.push(Instruction::load(
                u32_ty,
                initialized_tracker_id,
                init_tracker_id,
                None,
            ));

            let proceeded_id = write_ray_flags_contains_flags(
                self,
                &mut block,
                initialized_tracker_id,
                RayQueryPoint::PROCEED.bits(),
            );
            let finished_proceed_id = write_ray_flags_contains_flags(
                self,
                &mut block,
                initialized_tracker_id,
                RayQueryPoint::FINISHED_TRAVERSAL.bits(),
            );
            // Although it seems strange to call this twice, I (Vecvec) can't find anything to suggest double calling this function is invalid.
            let not_finished_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::LogicalNot,
                bool_type_id,
                not_finished_id,
                finished_proceed_id,
            ));

            let is_valid_id = self.write_logical_and(&mut block, not_finished_id, proceeded_id);

            block.body.push(Instruction::selection_merge(
                final_label_id,
                spirv::SelectionControl::NONE,
            ));

            Instruction::branch_conditional(is_valid_id, valid_id, final_label_id)
        } else {
            Instruction::branch(valid_id)
        };

        function.consume(block, instruction);

        let intersection_id = self.get_constant_scalar(crate::Literal::U32(
            spirv::RayQueryIntersection::RayQueryCandidateIntersectionKHR as _,
        ));
        let raw_kind_id = self.id_gen.next();
        valid_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionTypeKHR,
                u32_ty,
                raw_kind_id,
                query_id,
                intersection_id,
            ));

        let candidate_tri_id = self.get_constant_scalar(crate::Literal::U32(
            spirv::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionTriangleKHR as _,
        ));
        let intersection_tri_id = self.id_gen.next();
        valid_block.body.push(Instruction::binary(
            spirv::Op::IEqual,
            bool_type_id,
            intersection_tri_id,
            raw_kind_id,
            candidate_tri_id,
        ));

        let generate_label_id = self.id_gen.next();
        let mut generate_block = Block::new(generate_label_id);

        let merge_label_id = self.id_gen.next();
        let merge_block = Block::new(merge_label_id);

        valid_block.body.push(Instruction::selection_merge(
            merge_label_id,
            spirv::SelectionControl::NONE,
        ));
        function.consume(
            valid_block,
            Instruction::branch_conditional(intersection_tri_id, generate_label_id, merge_label_id),
        );

        generate_block
            .body
            .push(Instruction::ray_query_confirm_intersection(query_id));

        function.consume(generate_block, Instruction::branch(merge_label_id));
        function.consume(merge_block, Instruction::branch(final_label_id));

        function.consume(final_block, Instruction::return_void());

        self.ray_query_functions
            .insert(LookupRayQueryFunction::ConfirmIntersection, func_id);

        function.to_words(&mut self.logical_layout.function_definitions);

        func_id
    }

    fn write_ray_query_get_vertex_positions(
        &mut self,
        is_committed: bool,
        ir_module: &crate::Module,
    ) -> spirv::Word {
        if let Some(&word) =
            self.ray_query_functions
                .get(&LookupRayQueryFunction::GetVertexPositions {
                    committed: is_committed,
                })
        {
            return word;
        }

        let (committed_ty, committed_tri_ty) = if is_committed {
            (
                spirv::RayQueryIntersection::RayQueryCommittedIntersectionKHR as u32,
                spirv::RayQueryCommittedIntersectionType::RayQueryCommittedIntersectionTriangleKHR
                    as u32,
            )
        } else {
            (
                spirv::RayQueryIntersection::RayQueryCandidateIntersectionKHR as u32,
                spirv::RayQueryCandidateIntersectionType::RayQueryCandidateIntersectionTriangleKHR
                    as u32,
            )
        };

        let ray_query_type_id = self.get_ray_query_pointer_id();

        let u32_ty = self.get_u32_type_id();
        let u32_ptr_ty = self.get_pointer_type_id(u32_ty, spirv::StorageClass::Function);

        let rq_get_vertex_positions_ty_id = self.get_handle_type_id(
            *ir_module
                .special_types
                .ray_vertex_return
                .as_ref()
                .expect("must be generated when reading in get vertex position"),
        );
        let ptr_return_ty =
            self.get_pointer_type_id(rq_get_vertex_positions_ty_id, spirv::StorageClass::Function);

        let bool_type_id = self.get_bool_type_id();

        let (func_id, mut function, arg_ids) = self.write_function_signature(
            &[ray_query_type_id, u32_ptr_ty],
            rq_get_vertex_positions_ty_id,
        );

        let query_id = arg_ids[0];
        let init_tracker_id = arg_ids[1];

        let block_id = self.id_gen.next();
        let mut block = Block::new(block_id);

        let return_id = self.id_gen.next();
        block.body.push(Instruction::variable(
            ptr_return_ty,
            return_id,
            spirv::StorageClass::Function,
            Some(self.get_constant_null(rq_get_vertex_positions_ty_id)),
        ));

        let valid_id = self.id_gen.next();
        let mut valid_block = Block::new(valid_id);

        let final_label_id = self.id_gen.next();
        let mut final_block = Block::new(final_label_id);

        let instruction = if self.ray_query_initialization_tracking {
            let initialized_tracker_id = self.id_gen.next();
            block.body.push(Instruction::load(
                u32_ty,
                initialized_tracker_id,
                init_tracker_id,
                None,
            ));

            let proceeded_id = write_ray_flags_contains_flags(
                self,
                &mut block,
                initialized_tracker_id,
                RayQueryPoint::PROCEED.bits(),
            );
            let finished_proceed_id = write_ray_flags_contains_flags(
                self,
                &mut block,
                initialized_tracker_id,
                RayQueryPoint::FINISHED_TRAVERSAL.bits(),
            );

            let correct_finish_id = if is_committed {
                finished_proceed_id
            } else {
                let not_finished_id = self.id_gen.next();
                block.body.push(Instruction::unary(
                    spirv::Op::LogicalNot,
                    bool_type_id,
                    not_finished_id,
                    finished_proceed_id,
                ));
                not_finished_id
            };

            let is_valid_id = self.write_logical_and(&mut block, correct_finish_id, proceeded_id);
            block.body.push(Instruction::selection_merge(
                final_label_id,
                spirv::SelectionControl::NONE,
            ));
            Instruction::branch_conditional(is_valid_id, valid_id, final_label_id)
        } else {
            Instruction::branch(valid_id)
        };

        function.consume(block, instruction);

        let intersection_id = self.get_constant_scalar(crate::Literal::U32(committed_ty));
        let raw_kind_id = self.id_gen.next();
        valid_block
            .body
            .push(Instruction::ray_query_get_intersection(
                spirv::Op::RayQueryGetIntersectionTypeKHR,
                u32_ty,
                raw_kind_id,
                query_id,
                intersection_id,
            ));

        let candidate_tri_id = self.get_constant_scalar(crate::Literal::U32(committed_tri_ty));
        let intersection_tri_id = self.id_gen.next();
        valid_block.body.push(Instruction::binary(
            spirv::Op::IEqual,
            bool_type_id,
            intersection_tri_id,
            raw_kind_id,
            candidate_tri_id,
        ));

        let generate_label_id = self.id_gen.next();
        let mut vertex_return_block = Block::new(generate_label_id);

        let merge_label_id = self.id_gen.next();
        let merge_block = Block::new(merge_label_id);

        valid_block.body.push(Instruction::selection_merge(
            merge_label_id,
            spirv::SelectionControl::NONE,
        ));
        function.consume(
            valid_block,
            Instruction::branch_conditional(intersection_tri_id, generate_label_id, merge_label_id),
        );

        let vertices_id = self.id_gen.next();
        vertex_return_block
            .body
            .push(Instruction::ray_query_return_vertex_position(
                rq_get_vertex_positions_ty_id,
                vertices_id,
                query_id,
                intersection_id,
            ));
        vertex_return_block
            .body
            .push(Instruction::store(return_id, vertices_id, None));

        function.consume(vertex_return_block, Instruction::branch(merge_label_id));
        function.consume(merge_block, Instruction::branch(final_label_id));

        let loaded_pos_id = self.id_gen.next();
        final_block.body.push(Instruction::load(
            rq_get_vertex_positions_ty_id,
            loaded_pos_id,
            return_id,
            None,
        ));

        function.consume(final_block, Instruction::return_value(loaded_pos_id));

        self.ray_query_functions.insert(
            LookupRayQueryFunction::GetVertexPositions {
                committed: is_committed,
            },
            func_id,
        );

        function.to_words(&mut self.logical_layout.function_definitions);

        func_id
    }

    fn write_ray_query_terminate(&mut self) -> spirv::Word {
        if let Some(&word) = self
            .ray_query_functions
            .get(&LookupRayQueryFunction::Terminate)
        {
            return word;
        }

        let ray_query_type_id = self.get_ray_query_pointer_id();

        let u32_ty = self.get_u32_type_id();
        let u32_ptr_ty = self.get_pointer_type_id(u32_ty, spirv::StorageClass::Function);

        let bool_type_id = self.get_bool_type_id();

        let (func_id, mut function, arg_ids) =
            self.write_function_signature(&[ray_query_type_id, u32_ptr_ty], self.void_type);

        let query_id = arg_ids[0];
        let init_tracker_id = arg_ids[1];

        let block_id = self.id_gen.next();
        let mut block = Block::new(block_id);

        let initialized_tracker_id = self.id_gen.next();
        block.body.push(Instruction::load(
            u32_ty,
            initialized_tracker_id,
            init_tracker_id,
            None,
        ));

        let merge_id = self.id_gen.next();
        let merge_block = Block::new(merge_id);

        let valid_block_id = self.id_gen.next();
        let mut valid_block = Block::new(valid_block_id);

        let instruction = if self.ray_query_initialization_tracking {
            let has_proceeded = write_ray_flags_contains_flags(
                self,
                &mut block,
                initialized_tracker_id,
                RayQueryPoint::PROCEED.bits(),
            );

            let finished_proceed_id = write_ray_flags_contains_flags(
                self,
                &mut block,
                initialized_tracker_id,
                RayQueryPoint::FINISHED_TRAVERSAL.bits(),
            );

            let not_finished_id = self.id_gen.next();
            block.body.push(Instruction::unary(
                spirv::Op::LogicalNot,
                bool_type_id,
                not_finished_id,
                finished_proceed_id,
            ));

            let valid_call = self.write_logical_and(&mut block, not_finished_id, has_proceeded);

            block.body.push(Instruction::selection_merge(
                merge_id,
                spirv::SelectionControl::NONE,
            ));

            Instruction::branch_conditional(valid_call, valid_block_id, merge_id)
        } else {
            Instruction::branch(valid_block_id)
        };

        function.consume(block, instruction);

        valid_block
            .body
            .push(Instruction::ray_query_terminate(query_id));

        function.consume(valid_block, Instruction::branch(merge_id));

        function.consume(merge_block, Instruction::return_void());

        function.to_words(&mut self.logical_layout.function_definitions);

        self.ray_query_functions
            .insert(LookupRayQueryFunction::Proceed, func_id);
        func_id
    }
}

impl BlockContext<'_> {
    pub(in super::super) fn write_ray_query_function(
        &mut self,
        query: Handle<crate::Expression>,
        function: &crate::RayQueryFunction,
        block: &mut Block,
    ) {
        let query_id = self.cached[query];
        let tracker_ids = *self
            .ray_query_tracker_expr
            .get(&query)
            .expect("not a cached ray query");

        match *function {
            crate::RayQueryFunction::Initialize {
                acceleration_structure,
                descriptor,
            } => {
                let desc_id = self.cached[descriptor];
                let acc_struct_id = self.get_handle_id(acceleration_structure);

                let func = self.writer.write_ray_query_initialize(self.ir_module);

                let func_id = self.gen_id();
                block.body.push(Instruction::function_call(
                    self.writer.void_type,
                    func_id,
                    func,
                    &[
                        query_id,
                        acc_struct_id,
                        desc_id,
                        tracker_ids.initialized_tracker,
                        tracker_ids.t_max_tracker,
                    ],
                ));
            }
            crate::RayQueryFunction::Proceed { result } => {
                let id = self.gen_id();
                self.cached[result] = id;

                let bool_ty = self.writer.get_bool_type_id();

                let func_id = self.writer.write_ray_query_proceed();
                block.body.push(Instruction::function_call(
                    bool_ty,
                    id,
                    func_id,
                    &[query_id, tracker_ids.initialized_tracker],
                ));
            }
            crate::RayQueryFunction::GenerateIntersection { hit_t } => {
                let hit_id = self.cached[hit_t];

                let func_id = self.writer.write_ray_query_generate_intersection();

                let func_call_id = self.gen_id();
                block.body.push(Instruction::function_call(
                    self.writer.void_type,
                    func_call_id,
                    func_id,
                    &[
                        query_id,
                        tracker_ids.initialized_tracker,
                        hit_id,
                        tracker_ids.t_max_tracker,
                    ],
                ));
            }
            crate::RayQueryFunction::ConfirmIntersection => {
                let func_id = self.writer.write_ray_query_confirm_intersection();

                let func_call_id = self.gen_id();
                block.body.push(Instruction::function_call(
                    self.writer.void_type,
                    func_call_id,
                    func_id,
                    &[query_id, tracker_ids.initialized_tracker],
                ));
            }
            crate::RayQueryFunction::Terminate => {
                let id = self.gen_id();

                let func_id = self.writer.write_ray_query_terminate();
                block.body.push(Instruction::function_call(
                    self.writer.void_type,
                    id,
                    func_id,
                    &[query_id, tracker_ids.initialized_tracker],
                ));
            }
        }
    }

    pub(in super::super) fn write_ray_query_return_vertex_position(
        &mut self,
        query: Handle<crate::Expression>,
        block: &mut Block,
        is_committed: bool,
    ) -> spirv::Word {
        let fn_id = self
            .writer
            .write_ray_query_get_vertex_positions(is_committed, self.ir_module);

        let query_id = self.cached[query];
        let tracker_id = *self
            .ray_query_tracker_expr
            .get(&query)
            .expect("not a cached ray query");

        let rq_get_vertex_positions_ty_id = self.get_handle_type_id(
            *self
                .ir_module
                .special_types
                .ray_vertex_return
                .as_ref()
                .expect("must be generated when reading in get vertex position"),
        );

        let func_call_id = self.gen_id();
        block.body.push(Instruction::function_call(
            rq_get_vertex_positions_ty_id,
            func_call_id,
            fn_id,
            &[query_id, tracker_id.initialized_tracker],
        ));
        func_call_id
    }
}
