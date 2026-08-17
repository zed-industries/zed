use alloc::vec::Vec;
use spirv::Word;

use crate::{
    back::spv::{
        helpers::BindingDecorations, writer::FunctionInterface, Block, EntryPointContext, Error,
        Instruction, WriterFlags,
    },
    non_max_u32::NonMaxU32,
    Handle,
};

#[derive(Clone)]
pub struct MeshReturnMember {
    pub ty_id: u32,
    pub binding: crate::Binding,
}

struct PerOutputTypeMeshReturnInfo {
    max_length_constant: Word,
    array_type_id: Word,
    struct_members: Vec<MeshReturnMember>,

    // * Most builtins must be in the same block.
    // * All bindings must be in their own unique block.
    // * The primitive indices builtin family needs its own block.
    // * Cull primitive doesn't care about having its own block, but
    //   some older validation layers didn't respect this.
    builtin_block: Option<Word>,
    bindings: Vec<Word>,
}

pub struct MeshReturnInfo {
    /// Id of the workgroup variable containing the data to be output
    out_variable_id: Word,
    /// All members of the output variable struct type
    out_members: Vec<MeshReturnMember>,
    /// Id of the input variable for local invocation id
    local_invocation_index_var_id: Word,
    /// Total workgroup size (product)
    workgroup_size: u32,

    /// Vertex-specific info
    vertex_info: PerOutputTypeMeshReturnInfo,
    /// Primitive-specific info
    primitive_info: PerOutputTypeMeshReturnInfo,
    /// Array variable for the primitive indices builtin
    primitive_indices: Option<Word>,
}

impl super::Writer {
    /// Sets up an output variable that will handle part of the mesh shader output
    pub(super) fn write_mesh_return_global_variable(
        &mut self,
        ty: u32,
        array_size_id: u32,
    ) -> Result<Word, Error> {
        let array_ty = self.id_gen.next();
        Instruction::type_array(array_ty, ty, array_size_id)
            .to_words(&mut self.logical_layout.declarations);
        let ptr_ty = self.get_pointer_type_id(array_ty, spirv::StorageClass::Output);
        let var_id = self.id_gen.next();
        Instruction::variable(ptr_ty, var_id, spirv::StorageClass::Output, None)
            .to_words(&mut self.logical_layout.declarations);
        Ok(var_id)
    }

    /// This does various setup things to allow mesh shader entry points
    /// to be properly written, such as creating the output variables
    pub(super) fn write_entry_point_mesh_shader_info(
        &mut self,
        iface: &mut FunctionInterface,
        local_invocation_index_id: Option<Word>,
        ir_module: &crate::Module,
        ep_context: &mut EntryPointContext,
    ) -> Result<(), Error> {
        let Some(ref mesh_info) = iface.mesh_info else {
            return Ok(());
        };
        // Collect the members in the output structs
        let out_members: Vec<MeshReturnMember> =
            match &ir_module.types[ir_module.global_variables[mesh_info.output_variable].ty] {
                &crate::Type {
                    inner: crate::TypeInner::Struct { ref members, .. },
                    ..
                } => members
                    .iter()
                    .map(|a| MeshReturnMember {
                        ty_id: self.get_handle_type_id(a.ty),
                        binding: a.binding.clone().unwrap(),
                    })
                    .collect(),
                _ => unreachable!(),
            };
        let vertex_array_type_id = out_members
            .iter()
            .find(|a| a.binding == crate::Binding::BuiltIn(crate::BuiltIn::Vertices))
            .unwrap()
            .ty_id;
        let primitive_array_type_id = out_members
            .iter()
            .find(|a| a.binding == crate::Binding::BuiltIn(crate::BuiltIn::Primitives))
            .unwrap()
            .ty_id;
        let vertex_members = match &ir_module.types[mesh_info.vertex_output_type] {
            &crate::Type {
                inner: crate::TypeInner::Struct { ref members, .. },
                ..
            } => members
                .iter()
                .map(|a| MeshReturnMember {
                    ty_id: self.get_handle_type_id(a.ty),
                    binding: a.binding.clone().unwrap(),
                })
                .collect(),
            _ => unreachable!(),
        };
        let primitive_members = match &ir_module.types[mesh_info.primitive_output_type] {
            &crate::Type {
                inner: crate::TypeInner::Struct { ref members, .. },
                ..
            } => members
                .iter()
                .map(|a| MeshReturnMember {
                    ty_id: self.get_handle_type_id(a.ty),
                    binding: a.binding.clone().unwrap(),
                })
                .collect(),
            _ => unreachable!(),
        };
        // In the final return, we do a giant memcpy, for which this is helpful
        let local_invocation_index_var_id = match local_invocation_index_id {
            Some(a) => a,
            None => {
                let u32_id = self.get_u32_type_id();
                let var = self.id_gen.next();
                Instruction::variable(
                    self.get_pointer_type_id(u32_id, spirv::StorageClass::Input),
                    var,
                    spirv::StorageClass::Input,
                    None,
                )
                .to_words(&mut self.logical_layout.declarations);
                Instruction::decorate(
                    var,
                    spirv::Decoration::BuiltIn,
                    &[spirv::BuiltIn::LocalInvocationIndex as u32],
                )
                .to_words(&mut self.logical_layout.annotations);
                iface.varying_ids.push(var);

                var
            }
        };
        // This is the information that is passed to the function writer
        // so that it can write the final return logic
        let mut mesh_return_info = MeshReturnInfo {
            out_variable_id: self.global_variables[mesh_info.output_variable].var_id,
            out_members,
            local_invocation_index_var_id,
            workgroup_size: self
                .get_constant_scalar(crate::Literal::U32(iface.workgroup_size.iter().product())),

            vertex_info: PerOutputTypeMeshReturnInfo {
                array_type_id: vertex_array_type_id,
                struct_members: vertex_members,
                max_length_constant: self
                    .get_constant_scalar(crate::Literal::U32(mesh_info.max_vertices)),
                bindings: Vec::new(),
                builtin_block: None,
            },
            primitive_info: PerOutputTypeMeshReturnInfo {
                array_type_id: primitive_array_type_id,
                struct_members: primitive_members,
                max_length_constant: self
                    .get_constant_scalar(crate::Literal::U32(mesh_info.max_primitives)),
                bindings: Vec::new(),
                builtin_block: None,
            },
            primitive_indices: None,
        };
        let vert_array_size_id =
            self.get_constant_scalar(crate::Literal::U32(mesh_info.max_vertices));
        let prim_array_size_id =
            self.get_constant_scalar(crate::Literal::U32(mesh_info.max_primitives));

        // Create the actual output variables and types.
        // According to SPIR-V,
        // * All builtins must be in the same output `Block` (except builtins for different output types like vertex/primitive)
        // * Each member with `location` must be in its own `Block` decorated `struct`
        // * Some builtins like CullPrimitiveEXT don't care as much (older validation layers don't know this! Wonderful!)
        // * Some builtins like the indices ones need to be in their own output variable without a struct wrapper

        // Write vertex builtin block
        if mesh_return_info
            .vertex_info
            .struct_members
            .iter()
            .any(|a| matches!(a.binding, crate::Binding::BuiltIn(..)))
        {
            let builtin_block_ty_id = self.id_gen.next();
            let mut ins = Instruction::type_struct(builtin_block_ty_id, &[]);
            let mut bi_index = 0;
            let mut decorations = Vec::new();
            for member in &mesh_return_info.vertex_info.struct_members {
                if let crate::Binding::BuiltIn(_) = member.binding {
                    ins.add_operand(member.ty_id);
                    let binding = self.map_binding(
                        ir_module,
                        iface.stage,
                        spirv::StorageClass::Output,
                        // Unused except in fragment shaders with other conditions, so we can pass null
                        Handle::new(NonMaxU32::new(0).unwrap()),
                        &member.binding,
                    )?;
                    match binding {
                        BindingDecorations::BuiltIn(bi, others) => {
                            decorations.push(Instruction::member_decorate(
                                builtin_block_ty_id,
                                bi_index,
                                spirv::Decoration::BuiltIn,
                                &[bi as Word],
                            ));
                            for other in others {
                                decorations.push(Instruction::member_decorate(
                                    builtin_block_ty_id,
                                    bi_index,
                                    other,
                                    &[],
                                ));
                            }
                        }
                        _ => unreachable!(),
                    }
                    bi_index += 1;
                }
            }
            ins.to_words(&mut self.logical_layout.declarations);
            decorations.push(Instruction::decorate(
                builtin_block_ty_id,
                spirv::Decoration::Block,
                &[],
            ));
            for dec in decorations {
                dec.to_words(&mut self.logical_layout.annotations);
            }
            let v =
                self.write_mesh_return_global_variable(builtin_block_ty_id, vert_array_size_id)?;
            iface.varying_ids.push(v);
            if self.flags.contains(WriterFlags::DEBUG) {
                self.debugs
                    .push(Instruction::name(v, "naga_vertex_builtin_outputs"));
            }
            mesh_return_info.vertex_info.builtin_block = Some(v);
        }
        // Write primitive builtin block
        if mesh_return_info
            .primitive_info
            .struct_members
            .iter()
            .any(|a| {
                !matches!(
                    a.binding,
                    crate::Binding::BuiltIn(
                        crate::BuiltIn::PointIndex
                            | crate::BuiltIn::LineIndices
                            | crate::BuiltIn::TriangleIndices
                    ) | crate::Binding::Location { .. }
                )
            })
        {
            let builtin_block_ty_id = self.id_gen.next();
            let mut ins = Instruction::type_struct(builtin_block_ty_id, &[]);
            let mut bi_index = 0;
            let mut decorations = Vec::new();
            for member in &mesh_return_info.primitive_info.struct_members {
                if let crate::Binding::BuiltIn(bi) = member.binding {
                    // These need to be in their own block, unlike other builtins
                    if matches!(
                        bi,
                        crate::BuiltIn::PointIndex
                            | crate::BuiltIn::LineIndices
                            | crate::BuiltIn::TriangleIndices,
                    ) {
                        continue;
                    }
                    ins.add_operand(member.ty_id);
                    let binding = self.map_binding(
                        ir_module,
                        iface.stage,
                        spirv::StorageClass::Output,
                        // Unused except in fragment shaders with other conditions, so we can pass null
                        Handle::new(NonMaxU32::new(0).unwrap()),
                        &member.binding,
                    )?;
                    match binding {
                        BindingDecorations::BuiltIn(bi, others) => {
                            decorations.push(Instruction::member_decorate(
                                builtin_block_ty_id,
                                bi_index,
                                spirv::Decoration::BuiltIn,
                                &[bi as Word],
                            ));
                            for other in others {
                                decorations.push(Instruction::member_decorate(
                                    builtin_block_ty_id,
                                    bi_index,
                                    other,
                                    &[],
                                ));
                            }
                        }
                        _ => unreachable!(),
                    }
                    bi_index += 1;
                }
            }
            ins.to_words(&mut self.logical_layout.declarations);
            decorations.push(Instruction::decorate(
                builtin_block_ty_id,
                spirv::Decoration::Block,
                &[],
            ));
            for dec in decorations {
                dec.to_words(&mut self.logical_layout.annotations);
            }
            let v =
                self.write_mesh_return_global_variable(builtin_block_ty_id, prim_array_size_id)?;
            Instruction::decorate(v, spirv::Decoration::PerPrimitiveEXT, &[])
                .to_words(&mut self.logical_layout.annotations);
            iface.varying_ids.push(v);
            if self.flags.contains(WriterFlags::DEBUG) {
                self.debugs
                    .push(Instruction::name(v, "naga_primitive_builtin_outputs"));
            }
            mesh_return_info.primitive_info.builtin_block = Some(v);
        }

        // Write vertex binding output blocks (1 array per output struct member)
        for member in &mesh_return_info.vertex_info.struct_members {
            match member.binding {
                crate::Binding::Location { location, .. } => {
                    // Create variable
                    let v =
                        self.write_mesh_return_global_variable(member.ty_id, vert_array_size_id)?;
                    // Decorate the variable with Location
                    Instruction::decorate(v, spirv::Decoration::Location, &[location])
                        .to_words(&mut self.logical_layout.annotations);
                    iface.varying_ids.push(v);
                    mesh_return_info.vertex_info.bindings.push(v);
                }
                crate::Binding::BuiltIn(_) => (),
            }
        }
        // Write primitive binding output blocks (1 array per output struct member)
        // Also write indices output block
        for member in &mesh_return_info.primitive_info.struct_members {
            match member.binding {
                crate::Binding::BuiltIn(
                    crate::BuiltIn::PointIndex
                    | crate::BuiltIn::LineIndices
                    | crate::BuiltIn::TriangleIndices,
                ) => {
                    // This is written here instead of as part of the builtin block
                    let v =
                        self.write_mesh_return_global_variable(member.ty_id, prim_array_size_id)?;
                    // This shouldn't be marked as PerPrimitiveEXT
                    Instruction::decorate(
                        v,
                        spirv::Decoration::BuiltIn,
                        &[match member.binding.to_built_in().unwrap() {
                            crate::BuiltIn::PointIndex => spirv::BuiltIn::PrimitivePointIndicesEXT,
                            crate::BuiltIn::LineIndices => spirv::BuiltIn::PrimitiveLineIndicesEXT,
                            crate::BuiltIn::TriangleIndices => {
                                spirv::BuiltIn::PrimitiveTriangleIndicesEXT
                            }
                            _ => unreachable!(),
                        } as Word],
                    )
                    .to_words(&mut self.logical_layout.annotations);
                    iface.varying_ids.push(v);
                    if self.flags.contains(WriterFlags::DEBUG) {
                        self.debugs
                            .push(Instruction::name(v, "naga_primitive_indices_outputs"));
                    }
                    mesh_return_info.primitive_indices = Some(v);
                }
                crate::Binding::Location { location, .. } => {
                    // Create variable
                    let v =
                        self.write_mesh_return_global_variable(member.ty_id, prim_array_size_id)?;
                    // Decorate the variable with Location
                    Instruction::decorate(v, spirv::Decoration::Location, &[location])
                        .to_words(&mut self.logical_layout.annotations);
                    // Decorate it with PerPrimitiveEXT
                    Instruction::decorate(v, spirv::Decoration::PerPrimitiveEXT, &[])
                        .to_words(&mut self.logical_layout.annotations);
                    iface.varying_ids.push(v);

                    mesh_return_info.primitive_info.bindings.push(v);
                }
                crate::Binding::BuiltIn(_) => (),
            }
        }

        // Store this where it can be read later during function write
        ep_context.mesh_state = Some(mesh_return_info);

        Ok(())
    }

    pub(super) fn write_entry_point_task_return(
        &mut self,
        value_id: Word,
        body: &mut Vec<Instruction>,
        task_payload: Word,
    ) -> Result<Instruction, Error> {
        // OpEmitMeshTasksEXT must be called right before exiting (after setting other
        // output variables if there are any)

        // Extract the vec3<u32> into 3 u32's
        let values = [self.id_gen.next(), self.id_gen.next(), self.id_gen.next()];
        for (i, &value) in values.iter().enumerate() {
            let instruction = Instruction::composite_extract(
                self.get_u32_type_id(),
                value,
                value_id,
                &[i as Word],
            );
            body.push(instruction);
        }
        let mut instruction = Instruction::new(spirv::Op::EmitMeshTasksEXT);
        for id in values {
            instruction.add_operand(id);
        }
        // We have to include the task payload in our call
        instruction.add_operand(task_payload);
        Ok(instruction)
    }

    /// This writes the actual loop
    #[allow(clippy::too_many_arguments)]
    fn write_mesh_copy_loop(
        &mut self,
        body: &mut Vec<Instruction>,
        mut loop_body_block: Vec<Instruction>,
        loop_header: u32,
        loop_merge: u32,
        count_id: u32,
        index_var: u32,
        return_info: &MeshReturnInfo,
    ) {
        let u32_id = self.get_u32_type_id();
        let condition_check = self.id_gen.next();
        let loop_continue = self.id_gen.next();
        let loop_body = self.id_gen.next();

        // Loop header
        {
            body.push(Instruction::label(loop_header));
            body.push(Instruction::loop_merge(
                loop_merge,
                loop_continue,
                spirv::SelectionControl::empty(),
            ));
            body.push(Instruction::branch(condition_check));
        }
        // Condition check - check if i is less than num vertices to copy
        {
            body.push(Instruction::label(condition_check));

            let val_i = self.id_gen.next();
            body.push(Instruction::load(u32_id, val_i, index_var, None));

            let cond = self.id_gen.next();
            body.push(Instruction::binary(
                spirv::Op::ULessThan,
                self.get_bool_type_id(),
                cond,
                val_i,
                count_id,
            ));
            body.push(Instruction::branch_conditional(cond, loop_body, loop_merge));
        }
        // Loop body
        {
            body.push(Instruction::label(loop_body));
            body.append(&mut loop_body_block);
            body.push(Instruction::branch(loop_continue));
        }
        // Loop continue - increment i
        {
            body.push(Instruction::label(loop_continue));

            let prev_val_i = self.id_gen.next();
            body.push(Instruction::load(u32_id, prev_val_i, index_var, None));
            let new_val_i = self.id_gen.next();
            body.push(Instruction::binary(
                spirv::Op::IAdd,
                u32_id,
                new_val_i,
                prev_val_i,
                return_info.workgroup_size,
            ));
            body.push(Instruction::store(index_var, new_val_i, None));

            body.push(Instruction::branch(loop_header));
        }
    }

    /// This generates the instructions used to copy all parts of a single output vertex/primitive
    /// to their individual output locations
    fn write_mesh_copy_body(
        &mut self,
        is_primitive: bool,
        return_info: &MeshReturnInfo,
        index_var: u32,
        vert_array_ptr: u32,
        prim_array_ptr: u32,
    ) -> Vec<Instruction> {
        let u32_type_id = self.get_u32_type_id();
        let mut body = Vec::new();
        // Current index to copy
        let val_i = self.id_gen.next();
        body.push(Instruction::load(u32_type_id, val_i, index_var, None));

        let info = if is_primitive {
            &return_info.primitive_info
        } else {
            &return_info.vertex_info
        };
        let array_ptr = if is_primitive {
            prim_array_ptr
        } else {
            vert_array_ptr
        };

        let mut builtin_index = 0;
        let mut binding_index = 0;
        // Write individual members of the vertex
        for (member_id, member) in info.struct_members.iter().enumerate() {
            let val_to_copy_ptr = self.id_gen.next();
            body.push(Instruction::access_chain(
                self.get_pointer_type_id(member.ty_id, spirv::StorageClass::Workgroup),
                val_to_copy_ptr,
                array_ptr,
                &[
                    val_i,
                    self.get_constant_scalar(crate::Literal::U32(member_id as u32)),
                ],
            ));
            let val_to_copy = self.id_gen.next();
            body.push(Instruction::load(
                member.ty_id,
                val_to_copy,
                val_to_copy_ptr,
                None,
            ));
            let mut needs_y_flip = false;
            let ptr_to_copy_to = self.id_gen.next();
            // Get a pointer to the struct member to copy
            match member.binding {
                crate::Binding::BuiltIn(
                    crate::BuiltIn::PointIndex
                    | crate::BuiltIn::LineIndices
                    | crate::BuiltIn::TriangleIndices,
                ) => {
                    body.push(Instruction::access_chain(
                        self.get_pointer_type_id(member.ty_id, spirv::StorageClass::Output),
                        ptr_to_copy_to,
                        return_info.primitive_indices.unwrap(),
                        &[val_i],
                    ));
                }
                crate::Binding::BuiltIn(bi) => {
                    body.push(Instruction::access_chain(
                        self.get_pointer_type_id(member.ty_id, spirv::StorageClass::Output),
                        ptr_to_copy_to,
                        info.builtin_block.unwrap(),
                        &[
                            val_i,
                            self.get_constant_scalar(crate::Literal::U32(builtin_index)),
                        ],
                    ));
                    needs_y_flip = matches!(bi, crate::BuiltIn::Position { .. })
                        && self.flags.contains(WriterFlags::ADJUST_COORDINATE_SPACE);
                    builtin_index += 1;
                }
                crate::Binding::Location { .. } => {
                    body.push(Instruction::access_chain(
                        self.get_pointer_type_id(member.ty_id, spirv::StorageClass::Output),
                        ptr_to_copy_to,
                        info.bindings[binding_index],
                        &[val_i],
                    ));
                    binding_index += 1;
                }
            }
            body.push(Instruction::store(ptr_to_copy_to, val_to_copy, None));
            // Flip the vertex position y coordinate in some cases
            // Can't use epilogue flip because can't read from this storage class
            if needs_y_flip {
                let prev_y = self.id_gen.next();
                body.push(Instruction::composite_extract(
                    self.get_f32_type_id(),
                    prev_y,
                    val_to_copy,
                    &[1],
                ));
                let new_y = self.id_gen.next();
                body.push(Instruction::unary(
                    spirv::Op::FNegate,
                    self.get_f32_type_id(),
                    new_y,
                    prev_y,
                ));
                let new_ptr_to_copy_to = self.id_gen.next();
                body.push(Instruction::access_chain(
                    self.get_f32_pointer_type_id(spirv::StorageClass::Output),
                    new_ptr_to_copy_to,
                    ptr_to_copy_to,
                    &[self.get_constant_scalar(crate::Literal::U32(1))],
                ));
                body.push(Instruction::store(new_ptr_to_copy_to, new_y, None));
            }
        }
        body
    }

    /// Writes the return call for a mesh shader, which involves copying previously
    /// written vertices/primitives into the actual output location.
    pub(super) fn write_mesh_shader_return(
        &mut self,
        return_info: &MeshReturnInfo,
        block: &mut Block,
        loop_counter_vertices: u32,
        loop_counter_primitives: u32,
        local_invocation_index_id: Word,
    ) -> Result<(), Error> {
        let u32_id = self.get_u32_type_id();

        // Load the actual vertex and primitive counts
        let mut load_u32_by_member_index =
            |members: &[MeshReturnMember], bi: crate::BuiltIn, max: u32| {
                let member_index = members
                    .iter()
                    .position(|a| a.binding == crate::Binding::BuiltIn(bi))
                    .unwrap() as u32;
                let ptr_id = self.id_gen.next();
                block.body.push(Instruction::access_chain(
                    self.get_pointer_type_id(u32_id, spirv::StorageClass::Workgroup),
                    ptr_id,
                    return_info.out_variable_id,
                    &[self.get_constant_scalar(crate::Literal::U32(member_index))],
                ));
                let before_min_id = self.id_gen.next();
                block
                    .body
                    .push(Instruction::load(u32_id, before_min_id, ptr_id, None));

                // Clamp the values
                let id = self.id_gen.next();
                block.body.push(Instruction::ext_inst_gl_op(
                    self.gl450_ext_inst_id,
                    spirv::GlslStd450Op::UMin,
                    u32_id,
                    id,
                    &[before_min_id, max],
                ));
                id
            };
        let vert_count_id = load_u32_by_member_index(
            &return_info.out_members,
            crate::BuiltIn::VertexCount,
            return_info.vertex_info.max_length_constant,
        );
        let prim_count_id = load_u32_by_member_index(
            &return_info.out_members,
            crate::BuiltIn::PrimitiveCount,
            return_info.primitive_info.max_length_constant,
        );

        // Get pointers to the arrays of data to extract
        let mut get_array_ptr = |bi: crate::BuiltIn, array_type_id: u32| {
            let id = self.id_gen.next();
            block.body.push(Instruction::access_chain(
                self.get_pointer_type_id(array_type_id, spirv::StorageClass::Workgroup),
                id,
                return_info.out_variable_id,
                &[self.get_constant_scalar(crate::Literal::U32(
                    return_info
                        .out_members
                        .iter()
                        .position(|a| a.binding == crate::Binding::BuiltIn(bi))
                        .unwrap() as u32,
                ))],
            ));
            id
        };
        let vert_array_ptr = get_array_ptr(
            crate::BuiltIn::Vertices,
            return_info.vertex_info.array_type_id,
        );
        let prim_array_ptr = get_array_ptr(
            crate::BuiltIn::Primitives,
            return_info.primitive_info.array_type_id,
        );

        // This must be called exactly once before any other mesh outputs are written
        {
            let mut ins = Instruction::new(spirv::Op::SetMeshOutputsEXT);
            ins.add_operand(vert_count_id);
            ins.add_operand(prim_count_id);
            block.body.push(ins);
        }

        // This is iterating over every returned vertex and splitting
        // it out into the multiple per-output arrays.
        let vertex_loop_header = self.id_gen.next();
        let prim_loop_header = self.id_gen.next();
        let in_between_loops = self.id_gen.next();
        let func_end = self.id_gen.next();

        block.body.push(Instruction::store(
            loop_counter_vertices,
            local_invocation_index_id,
            None,
        ));
        block.body.push(Instruction::branch(vertex_loop_header));

        let vertex_copy_body = self.write_mesh_copy_body(
            false,
            return_info,
            loop_counter_vertices,
            vert_array_ptr,
            prim_array_ptr,
        );
        // Write vertex copy loop
        self.write_mesh_copy_loop(
            &mut block.body,
            vertex_copy_body,
            vertex_loop_header,
            in_between_loops,
            vert_count_id,
            loop_counter_vertices,
            return_info,
        );

        // In between loops, reset the initial index
        {
            block.body.push(Instruction::label(in_between_loops));

            block.body.push(Instruction::store(
                loop_counter_primitives,
                local_invocation_index_id,
                None,
            ));

            block.body.push(Instruction::branch(prim_loop_header));
        }
        let primitive_copy_body = self.write_mesh_copy_body(
            true,
            return_info,
            loop_counter_primitives,
            vert_array_ptr,
            prim_array_ptr,
        );
        // Write primitive copy loop
        self.write_mesh_copy_loop(
            &mut block.body,
            primitive_copy_body,
            prim_loop_header,
            func_end,
            prim_count_id,
            loop_counter_primitives,
            return_info,
        );

        block.body.push(Instruction::label(func_end));
        Ok(())
    }

    pub(super) fn write_mesh_shader_wrapper(
        &mut self,
        return_info: &MeshReturnInfo,
        inner_id: u32,
    ) -> Result<u32, Error> {
        let out_id = self.id_gen.next();
        let mut function = super::Function::default();
        let lookup_function_type = super::LookupFunctionType {
            parameter_type_ids: alloc::vec![],
            return_type_id: self.void_type,
        };
        let function_type = self.get_function_type(lookup_function_type);
        function.signature = Some(Instruction::function(
            self.void_type,
            out_id,
            spirv::FunctionControl::empty(),
            function_type,
        ));
        let u32_id = self.get_u32_type_id();
        {
            let mut block = Block::new(self.id_gen.next());
            // A general function variable that we guarantee to allow in the final return. It must be
            // declared at the top of the function. Currently it is used in the memcpy part to keep
            // track of the current index to copy.
            let loop_counter_vertices = self.id_gen.next();
            let loop_counter_primitives = self.id_gen.next();
            block.body.insert(
                0,
                Instruction::variable(
                    self.get_pointer_type_id(u32_id, spirv::StorageClass::Function),
                    loop_counter_vertices,
                    spirv::StorageClass::Function,
                    None,
                ),
            );
            block.body.insert(
                1,
                Instruction::variable(
                    self.get_pointer_type_id(u32_id, spirv::StorageClass::Function),
                    loop_counter_primitives,
                    spirv::StorageClass::Function,
                    None,
                ),
            );
            let local_invocation_index_id = self.id_gen.next();
            block.body.push(Instruction::load(
                u32_id,
                local_invocation_index_id,
                return_info.local_invocation_index_var_id,
                None,
            ));
            block.body.push(Instruction::function_call(
                self.void_type,
                self.id_gen.next(),
                inner_id,
                &[],
            ));
            self.write_control_barrier(crate::Barrier::WORK_GROUP, &mut block.body);
            self.write_mesh_shader_return(
                return_info,
                &mut block,
                loop_counter_vertices,
                loop_counter_primitives,
                local_invocation_index_id,
            )?;
            function.consume(block, Instruction::return_void());
        }
        function.to_words(&mut self.logical_layout.function_definitions);
        Ok(out_id)
    }

    pub(super) fn write_task_shader_wrapper(
        &mut self,
        task_payload: Word,
        inner_id: u32,
    ) -> Result<u32, Error> {
        let out_id = self.id_gen.next();
        let mut function = super::Function::default();
        let lookup_function_type = super::LookupFunctionType {
            parameter_type_ids: alloc::vec![],
            return_type_id: self.void_type,
        };
        let function_type = self.get_function_type(lookup_function_type);
        function.signature = Some(Instruction::function(
            self.void_type,
            out_id,
            spirv::FunctionControl::empty(),
            function_type,
        ));

        {
            let mut block = Block::new(self.id_gen.next());
            let result = self.id_gen.next();
            block.body.push(Instruction::function_call(
                self.get_vec3u_type_id(),
                result,
                inner_id,
                &[],
            ));
            self.write_control_barrier(crate::Barrier::WORK_GROUP, &mut block.body);
            let final_value = if let Some(task_limits) = self.task_dispatch_limits {
                let zero_u32 = self.get_constant_scalar(crate::Literal::U32(0));
                let max_per_dim = self.get_constant_scalar(crate::Literal::U32(
                    task_limits.max_mesh_workgroups_per_dim,
                ));
                let max_total = self.get_constant_scalar(crate::Literal::U32(
                    task_limits.max_mesh_workgroups_total,
                ));
                let combined_struct_type = self.get_tuple_of_u32s_ty_id();
                let values = [self.id_gen.next(), self.id_gen.next(), self.id_gen.next()];
                for (i, value) in values.into_iter().enumerate() {
                    block.body.push(Instruction::composite_extract(
                        self.get_u32_type_id(),
                        value,
                        result,
                        &[i as u32],
                    ));
                }
                let prod_1 = self.id_gen.next();
                let overflows = [self.id_gen.next(), self.id_gen.next()];
                {
                    let struct_out = self.id_gen.next();
                    block.body.push(Instruction::binary(
                        spirv::Op::UMulExtended,
                        combined_struct_type,
                        struct_out,
                        values[0],
                        values[1],
                    ));
                    block.body.push(Instruction::composite_extract(
                        self.get_u32_type_id(),
                        prod_1,
                        struct_out,
                        &[0],
                    ));
                    block.body.push(Instruction::composite_extract(
                        self.get_u32_type_id(),
                        overflows[0],
                        struct_out,
                        &[1],
                    ));
                }
                let prod_final = self.id_gen.next();
                {
                    let struct_out = self.id_gen.next();
                    block.body.push(Instruction::binary(
                        spirv::Op::UMulExtended,
                        combined_struct_type,
                        struct_out,
                        prod_1,
                        values[2],
                    ));
                    block.body.push(Instruction::composite_extract(
                        self.get_u32_type_id(),
                        prod_final,
                        struct_out,
                        &[0],
                    ));
                    block.body.push(Instruction::composite_extract(
                        self.get_u32_type_id(),
                        overflows[1],
                        struct_out,
                        &[1],
                    ));
                }
                let total_too_large = self.id_gen.next();
                block.body.push(Instruction::binary(
                    spirv::Op::UGreaterThan,
                    self.get_bool_type_id(),
                    total_too_large,
                    prod_final,
                    max_total,
                ));

                let too_large = [self.id_gen.next(), self.id_gen.next(), self.id_gen.next()];
                for (i, value) in values.into_iter().enumerate() {
                    block.body.push(Instruction::binary(
                        spirv::Op::UGreaterThan,
                        self.get_bool_type_id(),
                        too_large[i],
                        value,
                        max_per_dim,
                    ));
                }
                let overflow_happens = [self.id_gen.next(), self.id_gen.next()];
                for (i, value) in overflows.into_iter().enumerate() {
                    block.body.push(Instruction::binary(
                        spirv::Op::INotEqual,
                        self.get_bool_type_id(),
                        overflow_happens[i],
                        value,
                        zero_u32,
                    ));
                }
                let mut current_violates_limits = total_too_large;
                for is_too_large in too_large {
                    let new = self.id_gen.next();
                    block.body.push(Instruction::binary(
                        spirv::Op::LogicalOr,
                        self.get_bool_type_id(),
                        new,
                        current_violates_limits,
                        is_too_large,
                    ));
                    current_violates_limits = new;
                }
                for overflow_happens in overflow_happens {
                    let new = self.id_gen.next();
                    block.body.push(Instruction::binary(
                        spirv::Op::LogicalOr,
                        self.get_bool_type_id(),
                        new,
                        current_violates_limits,
                        overflow_happens,
                    ));
                    current_violates_limits = new;
                }
                let zero_vec3 = self.id_gen.next();
                block.body.push(Instruction::composite_construct(
                    self.get_vec3u_type_id(),
                    zero_vec3,
                    &[zero_u32, zero_u32, zero_u32],
                ));
                let final_result = self.id_gen.next();
                block.body.push(Instruction::select(
                    self.get_vec3u_type_id(),
                    final_result,
                    current_violates_limits,
                    zero_vec3,
                    result,
                ));
                final_result
            } else {
                result
            };
            let ins =
                self.write_entry_point_task_return(final_value, &mut block.body, task_payload)?;
            function.consume(block, ins);
        }
        function.to_words(&mut self.logical_layout.function_definitions);
        Ok(out_id)
    }
}
