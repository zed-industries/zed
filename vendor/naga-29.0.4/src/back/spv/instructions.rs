use alloc::{vec, vec::Vec};

use spirv::{Op, Word};

use super::{block::DebugInfoInner, helpers};

pub(super) enum Signedness {
    Unsigned = 0,
    Signed = 1,
}

pub(super) enum SampleLod {
    Explicit,
    Implicit,
}

pub(super) struct Case {
    pub value: Word,
    pub label_id: Word,
}

impl super::Instruction {
    //
    //  Debug Instructions
    //

    pub(super) fn string(name: &str, id: Word) -> Self {
        let mut instruction = Self::new(Op::String);
        instruction.set_result(id);
        instruction.add_operands(helpers::string_to_words(name));
        instruction
    }

    pub(super) fn source(
        source_language: spirv::SourceLanguage,
        version: u32,
        source: &Option<DebugInfoInner>,
    ) -> Self {
        let mut instruction = Self::new(Op::Source);
        instruction.add_operand(source_language as u32);
        instruction.add_operands(helpers::bytes_to_words(&version.to_le_bytes()));
        if let Some(source) = source.as_ref() {
            instruction.add_operand(source.source_file_id);
            instruction.add_operands(helpers::string_to_words(source.source_code));
        }
        instruction
    }

    pub(super) fn source_continued(source: &[u8]) -> Self {
        let mut instruction = Self::new(Op::SourceContinued);
        instruction.add_operands(helpers::str_bytes_to_words(source));
        instruction
    }

    pub(super) fn source_auto_continued(
        source_language: spirv::SourceLanguage,
        version: u32,
        source: &Option<DebugInfoInner>,
    ) -> Vec<Self> {
        let mut instructions = vec![];

        let with_continue = source.as_ref().and_then(|debug_info| {
            (debug_info.source_code.len() > u16::MAX as usize).then_some(debug_info)
        });
        if let Some(debug_info) = with_continue {
            let mut instruction = Self::new(Op::Source);
            instruction.add_operand(source_language as u32);
            instruction.add_operands(helpers::bytes_to_words(&version.to_le_bytes()));

            let words = helpers::string_to_byte_chunks(debug_info.source_code, u16::MAX as usize);
            instruction.add_operand(debug_info.source_file_id);
            instruction.add_operands(helpers::str_bytes_to_words(words[0]));
            instructions.push(instruction);
            for word_bytes in words[1..].iter() {
                let instruction_continue = Self::source_continued(word_bytes);
                instructions.push(instruction_continue);
            }
        } else {
            let instruction = Self::source(source_language, version, source);
            instructions.push(instruction);
        }
        instructions
    }

    pub(super) fn name(target_id: Word, name: &str) -> Self {
        let mut instruction = Self::new(Op::Name);
        instruction.add_operand(target_id);
        instruction.add_operands(helpers::string_to_words(name));
        instruction
    }

    pub(super) fn member_name(target_id: Word, member: Word, name: &str) -> Self {
        let mut instruction = Self::new(Op::MemberName);
        instruction.add_operand(target_id);
        instruction.add_operand(member);
        instruction.add_operands(helpers::string_to_words(name));
        instruction
    }

    pub(super) fn line(file: Word, line: Word, column: Word) -> Self {
        let mut instruction = Self::new(Op::Line);
        instruction.add_operand(file);
        instruction.add_operand(line);
        instruction.add_operand(column);
        instruction
    }

    //
    //  Annotation Instructions
    //

    pub(super) fn decorate(
        target_id: Word,
        decoration: spirv::Decoration,
        operands: &[Word],
    ) -> Self {
        let mut instruction = Self::new(Op::Decorate);
        instruction.add_operand(target_id);
        instruction.add_operand(decoration as u32);
        for operand in operands {
            instruction.add_operand(*operand)
        }
        instruction
    }

    pub(super) fn member_decorate(
        target_id: Word,
        member_index: Word,
        decoration: spirv::Decoration,
        operands: &[Word],
    ) -> Self {
        let mut instruction = Self::new(Op::MemberDecorate);
        instruction.add_operand(target_id);
        instruction.add_operand(member_index);
        instruction.add_operand(decoration as u32);
        for operand in operands {
            instruction.add_operand(*operand)
        }
        instruction
    }

    //
    //  Extension Instructions
    //

    pub(super) fn extension(name: &str) -> Self {
        let mut instruction = Self::new(Op::Extension);
        instruction.add_operands(helpers::string_to_words(name));
        instruction
    }

    pub(super) fn ext_inst_import(id: Word, name: &str) -> Self {
        let mut instruction = Self::new(Op::ExtInstImport);
        instruction.set_result(id);
        instruction.add_operands(helpers::string_to_words(name));
        instruction
    }

    pub(super) fn ext_inst_gl_op(
        set_id: Word,
        op: spirv::GlslStd450Op,
        result_type_id: Word,
        id: Word,
        operands: &[Word],
    ) -> Self {
        Self::ext_inst(set_id, op as u32, result_type_id, id, operands)
    }

    pub(super) fn ext_inst(
        set_id: Word,
        op: u32,
        result_type_id: Word,
        id: Word,
        operands: &[Word],
    ) -> Self {
        let mut instruction = Self::new(Op::ExtInst);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(set_id);
        instruction.add_operand(op);
        for operand in operands {
            instruction.add_operand(*operand)
        }
        instruction
    }

    //
    //  Mode-Setting Instructions
    //

    pub(super) fn memory_model(
        addressing_model: spirv::AddressingModel,
        memory_model: spirv::MemoryModel,
    ) -> Self {
        let mut instruction = Self::new(Op::MemoryModel);
        instruction.add_operand(addressing_model as u32);
        instruction.add_operand(memory_model as u32);
        instruction
    }

    pub(super) fn entry_point(
        execution_model: spirv::ExecutionModel,
        entry_point_id: Word,
        name: &str,
        interface_ids: &[Word],
    ) -> Self {
        let mut instruction = Self::new(Op::EntryPoint);
        instruction.add_operand(execution_model as u32);
        instruction.add_operand(entry_point_id);
        instruction.add_operands(helpers::string_to_words(name));

        for interface_id in interface_ids {
            instruction.add_operand(*interface_id);
        }

        instruction
    }

    pub(super) fn execution_mode(
        entry_point_id: Word,
        execution_mode: spirv::ExecutionMode,
        args: &[Word],
    ) -> Self {
        let mut instruction = Self::new(Op::ExecutionMode);
        instruction.add_operand(entry_point_id);
        instruction.add_operand(execution_mode as u32);
        for arg in args {
            instruction.add_operand(*arg);
        }
        instruction
    }

    pub(super) fn capability(capability: spirv::Capability) -> Self {
        let mut instruction = Self::new(Op::Capability);
        instruction.add_operand(capability as u32);
        instruction
    }

    //
    //  Type-Declaration Instructions
    //

    pub(super) fn type_void(id: Word) -> Self {
        let mut instruction = Self::new(Op::TypeVoid);
        instruction.set_result(id);
        instruction
    }

    pub(super) fn type_bool(id: Word) -> Self {
        let mut instruction = Self::new(Op::TypeBool);
        instruction.set_result(id);
        instruction
    }

    pub(super) fn type_int(id: Word, width: Word, signedness: Signedness) -> Self {
        let mut instruction = Self::new(Op::TypeInt);
        instruction.set_result(id);
        instruction.add_operand(width);
        instruction.add_operand(signedness as u32);
        instruction
    }

    pub(super) fn type_float(id: Word, width: Word) -> Self {
        let mut instruction = Self::new(Op::TypeFloat);
        instruction.set_result(id);
        instruction.add_operand(width);
        instruction
    }

    pub(super) fn type_vector(
        id: Word,
        component_type_id: Word,
        component_count: crate::VectorSize,
    ) -> Self {
        let mut instruction = Self::new(Op::TypeVector);
        instruction.set_result(id);
        instruction.add_operand(component_type_id);
        instruction.add_operand(component_count as u32);
        instruction
    }

    pub(super) fn type_matrix(
        id: Word,
        column_type_id: Word,
        column_count: crate::VectorSize,
    ) -> Self {
        let mut instruction = Self::new(Op::TypeMatrix);
        instruction.set_result(id);
        instruction.add_operand(column_type_id);
        instruction.add_operand(column_count as u32);
        instruction
    }

    pub(super) fn type_coop_matrix(
        id: Word,
        scalar_type_id: Word,
        scope_id: Word,
        row_count_id: Word,
        column_count_id: Word,
        matrix_use_id: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::TypeCooperativeMatrixKHR);
        instruction.set_result(id);
        instruction.add_operand(scalar_type_id);
        instruction.add_operand(scope_id);
        instruction.add_operand(row_count_id);
        instruction.add_operand(column_count_id);
        instruction.add_operand(matrix_use_id);
        instruction
    }

    pub(super) fn type_image(
        id: Word,
        sampled_type_id: Word,
        dim: spirv::Dim,
        flags: super::ImageTypeFlags,
        image_format: spirv::ImageFormat,
    ) -> Self {
        let mut instruction = Self::new(Op::TypeImage);
        instruction.set_result(id);
        instruction.add_operand(sampled_type_id);
        instruction.add_operand(dim as u32);
        instruction.add_operand(flags.contains(super::ImageTypeFlags::DEPTH) as u32);
        instruction.add_operand(flags.contains(super::ImageTypeFlags::ARRAYED) as u32);
        instruction.add_operand(flags.contains(super::ImageTypeFlags::MULTISAMPLED) as u32);
        instruction.add_operand(if flags.contains(super::ImageTypeFlags::SAMPLED) {
            1
        } else {
            2
        });
        instruction.add_operand(image_format as u32);
        instruction
    }

    pub(super) fn type_sampler(id: Word) -> Self {
        let mut instruction = Self::new(Op::TypeSampler);
        instruction.set_result(id);
        instruction
    }

    pub(super) fn type_acceleration_structure(id: Word) -> Self {
        let mut instruction = Self::new(Op::TypeAccelerationStructureKHR);
        instruction.set_result(id);
        instruction
    }

    pub(super) fn type_ray_query(id: Word) -> Self {
        let mut instruction = Self::new(Op::TypeRayQueryKHR);
        instruction.set_result(id);
        instruction
    }

    pub(super) fn type_sampled_image(id: Word, image_type_id: Word) -> Self {
        let mut instruction = Self::new(Op::TypeSampledImage);
        instruction.set_result(id);
        instruction.add_operand(image_type_id);
        instruction
    }

    pub(super) fn type_array(id: Word, element_type_id: Word, length_id: Word) -> Self {
        let mut instruction = Self::new(Op::TypeArray);
        instruction.set_result(id);
        instruction.add_operand(element_type_id);
        instruction.add_operand(length_id);
        instruction
    }

    pub(super) fn type_runtime_array(id: Word, element_type_id: Word) -> Self {
        let mut instruction = Self::new(Op::TypeRuntimeArray);
        instruction.set_result(id);
        instruction.add_operand(element_type_id);
        instruction
    }

    pub(super) fn type_struct(id: Word, member_ids: &[Word]) -> Self {
        let mut instruction = Self::new(Op::TypeStruct);
        instruction.set_result(id);

        for member_id in member_ids {
            instruction.add_operand(*member_id)
        }

        instruction
    }

    pub(super) fn type_pointer(
        id: Word,
        storage_class: spirv::StorageClass,
        type_id: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::TypePointer);
        instruction.set_result(id);
        instruction.add_operand(storage_class as u32);
        instruction.add_operand(type_id);
        instruction
    }

    pub(super) fn type_function(id: Word, return_type_id: Word, parameter_ids: &[Word]) -> Self {
        let mut instruction = Self::new(Op::TypeFunction);
        instruction.set_result(id);
        instruction.add_operand(return_type_id);

        for parameter_id in parameter_ids {
            instruction.add_operand(*parameter_id);
        }

        instruction
    }

    //
    //  Constant-Creation Instructions
    //

    pub(super) fn constant_null(result_type_id: Word, id: Word) -> Self {
        let mut instruction = Self::new(Op::ConstantNull);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction
    }

    pub(super) fn constant_true(result_type_id: Word, id: Word) -> Self {
        let mut instruction = Self::new(Op::ConstantTrue);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction
    }

    pub(super) fn constant_false(result_type_id: Word, id: Word) -> Self {
        let mut instruction = Self::new(Op::ConstantFalse);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction
    }

    pub(super) fn constant_16bit(result_type_id: Word, id: Word, low: Word) -> Self {
        Self::constant(result_type_id, id, &[low])
    }

    pub(super) fn constant_32bit(result_type_id: Word, id: Word, value: Word) -> Self {
        Self::constant(result_type_id, id, &[value])
    }

    pub(super) fn constant_64bit(result_type_id: Word, id: Word, low: Word, high: Word) -> Self {
        Self::constant(result_type_id, id, &[low, high])
    }

    pub(super) fn constant(result_type_id: Word, id: Word, values: &[Word]) -> Self {
        let mut instruction = Self::new(Op::Constant);
        instruction.set_type(result_type_id);
        instruction.set_result(id);

        for value in values {
            instruction.add_operand(*value);
        }

        instruction
    }

    pub(super) fn constant_composite(
        result_type_id: Word,
        id: Word,
        constituent_ids: &[Word],
    ) -> Self {
        let mut instruction = Self::new(Op::ConstantComposite);
        instruction.set_type(result_type_id);
        instruction.set_result(id);

        for constituent_id in constituent_ids {
            instruction.add_operand(*constituent_id);
        }

        instruction
    }

    //
    //  Memory Instructions
    //

    pub(super) fn variable(
        result_type_id: Word,
        id: Word,
        storage_class: spirv::StorageClass,
        initializer_id: Option<Word>,
    ) -> Self {
        let mut instruction = Self::new(Op::Variable);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(storage_class as u32);

        if let Some(initializer_id) = initializer_id {
            instruction.add_operand(initializer_id);
        }

        instruction
    }

    pub(super) fn load(
        result_type_id: Word,
        id: Word,
        pointer_id: Word,
        memory_access: Option<spirv::MemoryAccess>,
    ) -> Self {
        let mut instruction = Self::new(Op::Load);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(pointer_id);

        if let Some(memory_access) = memory_access {
            instruction.add_operand(memory_access.bits());
        }

        instruction
    }

    pub(super) fn atomic_load(
        result_type_id: Word,
        id: Word,
        pointer_id: Word,
        scope_id: Word,
        semantics_id: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::AtomicLoad);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(pointer_id);
        instruction.add_operand(scope_id);
        instruction.add_operand(semantics_id);
        instruction
    }

    pub(super) fn store(
        pointer_id: Word,
        value_id: Word,
        memory_access: Option<spirv::MemoryAccess>,
    ) -> Self {
        let mut instruction = Self::new(Op::Store);
        instruction.add_operand(pointer_id);
        instruction.add_operand(value_id);

        if let Some(memory_access) = memory_access {
            instruction.add_operand(memory_access.bits());
        }

        instruction
    }

    pub(super) fn atomic_store(
        pointer_id: Word,
        scope_id: Word,
        semantics_id: Word,
        value_id: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::AtomicStore);
        instruction.add_operand(pointer_id);
        instruction.add_operand(scope_id);
        instruction.add_operand(semantics_id);
        instruction.add_operand(value_id);
        instruction
    }

    pub(super) fn access_chain(
        result_type_id: Word,
        id: Word,
        base_id: Word,
        index_ids: &[Word],
    ) -> Self {
        let mut instruction = Self::new(Op::AccessChain);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(base_id);

        for index_id in index_ids {
            instruction.add_operand(*index_id);
        }

        instruction
    }

    pub(super) fn array_length(
        result_type_id: Word,
        id: Word,
        structure_id: Word,
        array_member: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::ArrayLength);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(structure_id);
        instruction.add_operand(array_member);
        instruction
    }

    //
    //  Function Instructions
    //

    pub(super) fn function(
        return_type_id: Word,
        id: Word,
        function_control: spirv::FunctionControl,
        function_type_id: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::Function);
        instruction.set_type(return_type_id);
        instruction.set_result(id);
        instruction.add_operand(function_control.bits());
        instruction.add_operand(function_type_id);
        instruction
    }

    pub(super) fn function_parameter(result_type_id: Word, id: Word) -> Self {
        let mut instruction = Self::new(Op::FunctionParameter);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction
    }

    pub(super) const fn function_end() -> Self {
        Self::new(Op::FunctionEnd)
    }

    pub(super) fn function_call(
        result_type_id: Word,
        id: Word,
        function_id: Word,
        argument_ids: &[Word],
    ) -> Self {
        let mut instruction = Self::new(Op::FunctionCall);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(function_id);

        for argument_id in argument_ids {
            instruction.add_operand(*argument_id);
        }

        instruction
    }

    //
    //  Image Instructions
    //

    pub(super) fn sampled_image(
        result_type_id: Word,
        id: Word,
        image: Word,
        sampler: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::SampledImage);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(image);
        instruction.add_operand(sampler);
        instruction
    }

    pub(super) fn image_sample(
        result_type_id: Word,
        id: Word,
        lod: SampleLod,
        sampled_image: Word,
        coordinates: Word,
        depth_ref: Option<Word>,
    ) -> Self {
        let op = match (lod, depth_ref) {
            (SampleLod::Explicit, None) => Op::ImageSampleExplicitLod,
            (SampleLod::Implicit, None) => Op::ImageSampleImplicitLod,
            (SampleLod::Explicit, Some(_)) => Op::ImageSampleDrefExplicitLod,
            (SampleLod::Implicit, Some(_)) => Op::ImageSampleDrefImplicitLod,
        };

        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(sampled_image);
        instruction.add_operand(coordinates);
        if let Some(dref) = depth_ref {
            instruction.add_operand(dref);
        }

        instruction
    }

    pub(super) fn image_gather(
        result_type_id: Word,
        id: Word,
        sampled_image: Word,
        coordinates: Word,
        component_id: Word,
        depth_ref: Option<Word>,
    ) -> Self {
        let op = match depth_ref {
            None => Op::ImageGather,
            Some(_) => Op::ImageDrefGather,
        };

        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(sampled_image);
        instruction.add_operand(coordinates);
        if let Some(dref) = depth_ref {
            instruction.add_operand(dref);
        } else {
            instruction.add_operand(component_id);
        }

        instruction
    }

    pub(super) fn image_fetch_or_read(
        op: Op,
        result_type_id: Word,
        id: Word,
        image: Word,
        coordinates: Word,
    ) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(image);
        instruction.add_operand(coordinates);
        instruction
    }

    pub(super) fn image_write(image: Word, coordinates: Word, value: Word) -> Self {
        let mut instruction = Self::new(Op::ImageWrite);
        instruction.add_operand(image);
        instruction.add_operand(coordinates);
        instruction.add_operand(value);
        instruction
    }

    pub(super) fn image_texel_pointer(
        result_type_id: Word,
        id: Word,
        image: Word,
        coordinates: Word,
        sample: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::ImageTexelPointer);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(image);
        instruction.add_operand(coordinates);
        instruction.add_operand(sample);
        instruction
    }

    pub(super) fn image_atomic(
        op: Op,
        result_type_id: Word,
        id: Word,
        pointer: Word,
        scope_id: Word,
        semantics_id: Word,
        value: Word,
    ) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(pointer);
        instruction.add_operand(scope_id);
        instruction.add_operand(semantics_id);
        instruction.add_operand(value);
        instruction
    }

    pub(super) fn image_query(op: Op, result_type_id: Word, id: Word, image: Word) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(image);
        instruction
    }

    //
    //  Ray Query Instructions
    //
    #[allow(clippy::too_many_arguments)]
    pub(super) fn ray_query_initialize(
        query: Word,
        acceleration_structure: Word,
        ray_flags: Word,
        cull_mask: Word,
        ray_origin: Word,
        ray_tmin: Word,
        ray_dir: Word,
        ray_tmax: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::RayQueryInitializeKHR);
        instruction.add_operand(query);
        instruction.add_operand(acceleration_structure);
        instruction.add_operand(ray_flags);
        instruction.add_operand(cull_mask);
        instruction.add_operand(ray_origin);
        instruction.add_operand(ray_tmin);
        instruction.add_operand(ray_dir);
        instruction.add_operand(ray_tmax);
        instruction
    }

    pub(super) fn ray_query_proceed(result_type_id: Word, id: Word, query: Word) -> Self {
        let mut instruction = Self::new(Op::RayQueryProceedKHR);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(query);
        instruction
    }

    pub(super) fn ray_query_generate_intersection(query: Word, hit: Word) -> Self {
        let mut instruction = Self::new(Op::RayQueryGenerateIntersectionKHR);
        instruction.add_operand(query);
        instruction.add_operand(hit);
        instruction
    }

    pub(super) fn ray_query_confirm_intersection(query: Word) -> Self {
        let mut instruction = Self::new(Op::RayQueryConfirmIntersectionKHR);
        instruction.add_operand(query);
        instruction
    }

    pub(super) fn ray_query_return_vertex_position(
        result_type_id: Word,
        id: Word,
        query: Word,
        intersection: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::RayQueryGetIntersectionTriangleVertexPositionsKHR);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(query);
        instruction.add_operand(intersection);
        instruction
    }

    pub(super) fn ray_query_get_intersection(
        op: Op,
        result_type_id: Word,
        id: Word,
        query: Word,
        intersection: Word,
    ) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(query);
        instruction.add_operand(intersection);
        instruction
    }

    pub(super) fn ray_query_get_t_min(result_type_id: Word, id: Word, query: Word) -> Self {
        let mut instruction = Self::new(Op::RayQueryGetRayTMinKHR);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(query);
        instruction
    }

    pub(super) fn ray_query_terminate(query: Word) -> Self {
        let mut instruction = Self::new(Op::RayQueryTerminateKHR);
        instruction.add_operand(query);
        instruction
    }

    //
    //  Conversion Instructions
    //
    pub(super) fn unary(op: Op, result_type_id: Word, id: Word, value: Word) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(value);
        instruction
    }

    //
    //  Composite Instructions
    //

    pub(super) fn composite_construct(
        result_type_id: Word,
        id: Word,
        constituent_ids: &[Word],
    ) -> Self {
        let mut instruction = Self::new(Op::CompositeConstruct);
        instruction.set_type(result_type_id);
        instruction.set_result(id);

        for constituent_id in constituent_ids {
            instruction.add_operand(*constituent_id);
        }

        instruction
    }

    pub(super) fn composite_extract(
        result_type_id: Word,
        id: Word,
        composite_id: Word,
        indices: &[Word],
    ) -> Self {
        let mut instruction = Self::new(Op::CompositeExtract);
        instruction.set_type(result_type_id);
        instruction.set_result(id);

        instruction.add_operand(composite_id);
        for index in indices {
            instruction.add_operand(*index);
        }

        instruction
    }

    pub(super) fn vector_extract_dynamic(
        result_type_id: Word,
        id: Word,
        vector_id: Word,
        index_id: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::VectorExtractDynamic);
        instruction.set_type(result_type_id);
        instruction.set_result(id);

        instruction.add_operand(vector_id);
        instruction.add_operand(index_id);

        instruction
    }

    pub(super) fn vector_shuffle(
        result_type_id: Word,
        id: Word,
        v1_id: Word,
        v2_id: Word,
        components: &[Word],
    ) -> Self {
        let mut instruction = Self::new(Op::VectorShuffle);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(v1_id);
        instruction.add_operand(v2_id);

        for &component in components {
            instruction.add_operand(component);
        }

        instruction
    }

    //
    // Arithmetic Instructions
    //
    pub(super) fn binary(
        op: Op,
        result_type_id: Word,
        id: Word,
        operand_1: Word,
        operand_2: Word,
    ) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(operand_1);
        instruction.add_operand(operand_2);
        instruction
    }

    pub(super) fn ternary(
        op: Op,
        result_type_id: Word,
        id: Word,
        operand_1: Word,
        operand_2: Word,
        operand_3: Word,
    ) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(operand_1);
        instruction.add_operand(operand_2);
        instruction.add_operand(operand_3);
        instruction
    }

    pub(super) fn quaternary(
        op: Op,
        result_type_id: Word,
        id: Word,
        operand_1: Word,
        operand_2: Word,
        operand_3: Word,
        operand_4: Word,
    ) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(operand_1);
        instruction.add_operand(operand_2);
        instruction.add_operand(operand_3);
        instruction.add_operand(operand_4);
        instruction
    }

    pub(super) fn relational(op: Op, result_type_id: Word, id: Word, expr_id: Word) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(expr_id);
        instruction
    }

    pub(super) fn atomic_binary(
        op: Op,
        result_type_id: Word,
        id: Word,
        pointer: Word,
        scope_id: Word,
        semantics_id: Word,
        value: Word,
    ) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(pointer);
        instruction.add_operand(scope_id);
        instruction.add_operand(semantics_id);
        instruction.add_operand(value);
        instruction
    }

    //
    // Bit Instructions
    //

    //
    // Relational and Logical Instructions
    //

    //
    // Derivative Instructions
    //

    pub(super) fn derivative(op: Op, result_type_id: Word, id: Word, expr_id: Word) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(expr_id);
        instruction
    }

    //
    // Control-Flow Instructions
    //

    pub(super) fn phi(
        result_type_id: Word,
        result_id: Word,
        var_parent_pairs: &[(Word, Word)],
    ) -> Self {
        let mut instruction = Self::new(Op::Phi);
        instruction.add_operand(result_type_id);
        instruction.add_operand(result_id);
        for &(variable, parent) in var_parent_pairs {
            instruction.add_operand(variable);
            instruction.add_operand(parent);
        }
        instruction
    }

    pub(super) fn selection_merge(
        merge_id: Word,
        selection_control: spirv::SelectionControl,
    ) -> Self {
        let mut instruction = Self::new(Op::SelectionMerge);
        instruction.add_operand(merge_id);
        instruction.add_operand(selection_control.bits());
        instruction
    }

    pub(super) fn loop_merge(
        merge_id: Word,
        continuing_id: Word,
        selection_control: spirv::SelectionControl,
    ) -> Self {
        let mut instruction = Self::new(Op::LoopMerge);
        instruction.add_operand(merge_id);
        instruction.add_operand(continuing_id);
        instruction.add_operand(selection_control.bits());
        instruction
    }

    pub(super) fn label(id: Word) -> Self {
        let mut instruction = Self::new(Op::Label);
        instruction.set_result(id);
        instruction
    }

    pub(super) fn branch(id: Word) -> Self {
        let mut instruction = Self::new(Op::Branch);
        instruction.add_operand(id);
        instruction
    }

    // TODO Branch Weights not implemented.
    pub(super) fn branch_conditional(
        condition_id: Word,
        true_label: Word,
        false_label: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::BranchConditional);
        instruction.add_operand(condition_id);
        instruction.add_operand(true_label);
        instruction.add_operand(false_label);
        instruction
    }

    pub(super) fn switch(selector_id: Word, default_id: Word, cases: &[Case]) -> Self {
        let mut instruction = Self::new(Op::Switch);
        instruction.add_operand(selector_id);
        instruction.add_operand(default_id);
        for case in cases {
            instruction.add_operand(case.value);
            instruction.add_operand(case.label_id);
        }
        instruction
    }

    pub(super) fn select(
        result_type_id: Word,
        id: Word,
        condition_id: Word,
        accept_id: Word,
        reject_id: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::Select);
        instruction.add_operand(result_type_id);
        instruction.add_operand(id);
        instruction.add_operand(condition_id);
        instruction.add_operand(accept_id);
        instruction.add_operand(reject_id);
        instruction
    }

    pub(super) const fn kill() -> Self {
        Self::new(Op::Kill)
    }

    pub(super) const fn return_void() -> Self {
        Self::new(Op::Return)
    }

    pub(super) fn return_value(value_id: Word) -> Self {
        let mut instruction = Self::new(Op::ReturnValue);
        instruction.add_operand(value_id);
        instruction
    }

    //
    //  Atomic Instructions
    //

    //
    //  Primitive Instructions
    //

    // Barriers

    pub(super) fn control_barrier(
        exec_scope_id: Word,
        mem_scope_id: Word,
        semantics_id: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::ControlBarrier);
        instruction.add_operand(exec_scope_id);
        instruction.add_operand(mem_scope_id);
        instruction.add_operand(semantics_id);
        instruction
    }
    pub(super) fn memory_barrier(mem_scope_id: Word, semantics_id: Word) -> Self {
        let mut instruction = Self::new(Op::MemoryBarrier);
        instruction.add_operand(mem_scope_id);
        instruction.add_operand(semantics_id);
        instruction
    }

    // Group Instructions

    pub(super) fn group_non_uniform_ballot(
        result_type_id: Word,
        id: Word,
        exec_scope_id: Word,
        predicate: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::GroupNonUniformBallot);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(exec_scope_id);
        instruction.add_operand(predicate);

        instruction
    }
    pub(super) fn group_non_uniform_broadcast_first(
        result_type_id: Word,
        id: Word,
        exec_scope_id: Word,
        value: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::GroupNonUniformBroadcastFirst);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(exec_scope_id);
        instruction.add_operand(value);

        instruction
    }
    pub(super) fn group_non_uniform_gather(
        op: Op,
        result_type_id: Word,
        id: Word,
        exec_scope_id: Word,
        value: Word,
        index: Word,
    ) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(exec_scope_id);
        instruction.add_operand(value);
        instruction.add_operand(index);

        instruction
    }
    pub(super) fn group_non_uniform_arithmetic(
        op: Op,
        result_type_id: Word,
        id: Word,
        exec_scope_id: Word,
        group_op: Option<spirv::GroupOperation>,
        value: Word,
    ) -> Self {
        let mut instruction = Self::new(op);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(exec_scope_id);
        if let Some(group_op) = group_op {
            instruction.add_operand(group_op as u32);
        }
        instruction.add_operand(value);

        instruction
    }
    pub(super) fn group_non_uniform_quad_swap(
        result_type_id: Word,
        id: Word,
        exec_scope_id: Word,
        value: Word,
        direction: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::GroupNonUniformQuadSwap);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(exec_scope_id);
        instruction.add_operand(value);
        instruction.add_operand(direction);

        instruction
    }

    // Cooperative operations
    pub(super) fn coop_load(
        result_type_id: Word,
        id: Word,
        pointer_id: Word,
        layout_id: Word,
        stride_id: Word,
    ) -> Self {
        let mut instruction = Self::new(Op::CooperativeMatrixLoadKHR);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(pointer_id);
        instruction.add_operand(layout_id);
        instruction.add_operand(stride_id);
        instruction
    }
    pub(super) fn coop_store(id: Word, pointer_id: Word, layout_id: Word, stride_id: Word) -> Self {
        let mut instruction = Self::new(Op::CooperativeMatrixStoreKHR);
        instruction.add_operand(pointer_id);
        instruction.add_operand(id);
        instruction.add_operand(layout_id);
        instruction.add_operand(stride_id);
        instruction
    }
    pub(super) fn coop_mul_add(result_type_id: Word, id: Word, a: Word, b: Word, c: Word) -> Self {
        let mut instruction = Self::new(Op::CooperativeMatrixMulAddKHR);
        instruction.set_type(result_type_id);
        instruction.set_result(id);
        instruction.add_operand(a);
        instruction.add_operand(b);
        instruction.add_operand(c);

        instruction
    }
}

impl From<crate::StorageFormat> for spirv::ImageFormat {
    fn from(format: crate::StorageFormat) -> Self {
        use crate::StorageFormat as Sf;
        match format {
            Sf::R8Unorm => Self::R8,
            Sf::R8Snorm => Self::R8Snorm,
            Sf::R8Uint => Self::R8ui,
            Sf::R8Sint => Self::R8i,
            Sf::R16Uint => Self::R16ui,
            Sf::R16Sint => Self::R16i,
            Sf::R16Float => Self::R16f,
            Sf::Rg8Unorm => Self::Rg8,
            Sf::Rg8Snorm => Self::Rg8Snorm,
            Sf::Rg8Uint => Self::Rg8ui,
            Sf::Rg8Sint => Self::Rg8i,
            Sf::R32Uint => Self::R32ui,
            Sf::R32Sint => Self::R32i,
            Sf::R32Float => Self::R32f,
            Sf::Rg16Uint => Self::Rg16ui,
            Sf::Rg16Sint => Self::Rg16i,
            Sf::Rg16Float => Self::Rg16f,
            Sf::Rgba8Unorm => Self::Rgba8,
            Sf::Rgba8Snorm => Self::Rgba8Snorm,
            Sf::Rgba8Uint => Self::Rgba8ui,
            Sf::Rgba8Sint => Self::Rgba8i,
            Sf::Bgra8Unorm => Self::Unknown,
            Sf::Rgb10a2Uint => Self::Rgb10a2ui,
            Sf::Rgb10a2Unorm => Self::Rgb10A2,
            Sf::Rg11b10Ufloat => Self::R11fG11fB10f,
            Sf::R64Uint => Self::R64ui,
            Sf::Rg32Uint => Self::Rg32ui,
            Sf::Rg32Sint => Self::Rg32i,
            Sf::Rg32Float => Self::Rg32f,
            Sf::Rgba16Uint => Self::Rgba16ui,
            Sf::Rgba16Sint => Self::Rgba16i,
            Sf::Rgba16Float => Self::Rgba16f,
            Sf::Rgba32Uint => Self::Rgba32ui,
            Sf::Rgba32Sint => Self::Rgba32i,
            Sf::Rgba32Float => Self::Rgba32f,
            Sf::R16Unorm => Self::R16,
            Sf::R16Snorm => Self::R16Snorm,
            Sf::Rg16Unorm => Self::Rg16,
            Sf::Rg16Snorm => Self::Rg16Snorm,
            Sf::Rgba16Unorm => Self::Rgba16,
            Sf::Rgba16Snorm => Self::Rgba16Snorm,
        }
    }
}

impl From<crate::ImageDimension> for spirv::Dim {
    fn from(dim: crate::ImageDimension) -> Self {
        use crate::ImageDimension as Id;
        match dim {
            Id::D1 => Self::Dim1D,
            Id::D2 => Self::Dim2D,
            Id::D3 => Self::Dim3D,
            Id::Cube => Self::DimCube,
        }
    }
}

impl From<crate::CooperativeRole> for spirv::CooperativeMatrixUse {
    fn from(role: crate::CooperativeRole) -> Self {
        match role {
            crate::CooperativeRole::A => Self::MatrixAKHR,
            crate::CooperativeRole::B => Self::MatrixBKHR,
            crate::CooperativeRole::C => Self::MatrixAccumulatorKHR,
        }
    }
}
