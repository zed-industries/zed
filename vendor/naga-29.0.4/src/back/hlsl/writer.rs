use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{fmt, mem};

use super::{
    help,
    help::{
        WrappedArrayLength, WrappedConstructor, WrappedImageQuery, WrappedStructMatrixAccess,
        WrappedZeroValue,
    },
    storage::StoreValue,
    BackendResult, Error, FragmentEntryPoint, Options, PipelineOptions, ShaderModel,
};
use crate::{
    back::{self, get_entry_points, Baked},
    common,
    proc::{self, index, ExternalTextureNameKey, NameKey},
    valid, Handle, Module, RayQueryFunction, Scalar, ScalarKind, ShaderStage, TypeInner,
};

const LOCATION_SEMANTIC: &str = "LOC";
const SPECIAL_CBUF_TYPE: &str = "NagaConstants";
const SPECIAL_CBUF_VAR: &str = "_NagaConstants";
const SPECIAL_FIRST_VERTEX: &str = "first_vertex";
const SPECIAL_FIRST_INSTANCE: &str = "first_instance";
const SPECIAL_OTHER: &str = "other";

pub(crate) const MODF_FUNCTION: &str = "naga_modf";
pub(crate) const FREXP_FUNCTION: &str = "naga_frexp";
pub(crate) const EXTRACT_BITS_FUNCTION: &str = "naga_extractBits";
pub(crate) const INSERT_BITS_FUNCTION: &str = "naga_insertBits";
pub(crate) const SAMPLER_HEAP_VAR: &str = "nagaSamplerHeap";
pub(crate) const COMPARISON_SAMPLER_HEAP_VAR: &str = "nagaComparisonSamplerHeap";
pub(crate) const SAMPLE_EXTERNAL_TEXTURE_FUNCTION: &str = "nagaSampleExternalTexture";
pub(crate) const ABS_FUNCTION: &str = "naga_abs";
pub(crate) const DIV_FUNCTION: &str = "naga_div";
pub(crate) const MOD_FUNCTION: &str = "naga_mod";
pub(crate) const NEG_FUNCTION: &str = "naga_neg";
pub(crate) const F2I32_FUNCTION: &str = "naga_f2i32";
pub(crate) const F2U32_FUNCTION: &str = "naga_f2u32";
pub(crate) const F2I64_FUNCTION: &str = "naga_f2i64";
pub(crate) const F2U64_FUNCTION: &str = "naga_f2u64";
pub(crate) const IMAGE_SAMPLE_BASE_CLAMP_TO_EDGE_FUNCTION: &str =
    "nagaTextureSampleBaseClampToEdge";
pub(crate) const IMAGE_LOAD_EXTERNAL_FUNCTION: &str = "nagaTextureLoadExternal";
pub(crate) const RAY_QUERY_TRACKER_VARIABLE_PREFIX: &str = "naga_query_init_tracker_for_";
/// Prefix for variables in a naga statement
pub(crate) const INTERNAL_PREFIX: &str = "naga_";

enum Index {
    Expression(Handle<crate::Expression>),
    Static(u32),
}

struct EpStructMember {
    name: String,
    ty: Handle<crate::Type>,
    // technically, this should always be `Some`
    // (we `debug_assert!` this in `write_interface_struct`)
    binding: Option<crate::Binding>,
    index: u32,
}

/// Structure contains information required for generating
/// wrapped structure of all entry points arguments
struct EntryPointBinding {
    /// Name of the fake EP argument that contains the struct
    /// with all the flattened input data.
    arg_name: String,
    /// Generated structure name
    ty_name: String,
    /// Members of generated structure
    members: Vec<EpStructMember>,
    local_invocation_index_name: Option<String>,
}

pub(super) struct EntryPointInterface {
    /// If `Some`, the input of an entry point is gathered in a special
    /// struct with members sorted by binding.
    /// The `EntryPointBinding::members` array is sorted by index,
    /// so that we can walk it in `write_ep_arguments_initialization`.
    input: Option<EntryPointBinding>,
    /// If `Some`, the output of an entry point is flattened.
    /// The `EntryPointBinding::members` array is sorted by binding,
    /// So that we can walk it in `Statement::Return` handler.
    output: Option<EntryPointBinding>,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
enum InterfaceKey {
    Location(u32),
    BuiltIn(crate::BuiltIn),
    Other,
}

impl InterfaceKey {
    const fn new(binding: Option<&crate::Binding>) -> Self {
        match binding {
            Some(&crate::Binding::Location { location, .. }) => Self::Location(location),
            Some(&crate::Binding::BuiltIn(built_in)) => Self::BuiltIn(built_in),
            None => Self::Other,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Io {
    Input,
    Output,
}

const fn is_subgroup_builtin_binding(binding: &Option<crate::Binding>) -> bool {
    let &Some(crate::Binding::BuiltIn(builtin)) = binding else {
        return false;
    };
    matches!(
        builtin,
        crate::BuiltIn::SubgroupSize
            | crate::BuiltIn::SubgroupInvocationId
            | crate::BuiltIn::NumSubgroups
            | crate::BuiltIn::SubgroupId
    )
}

/// Information for how to generate a `binding_array<sampler>` access.
struct BindingArraySamplerInfo {
    /// Variable name of the sampler heap
    sampler_heap_name: &'static str,
    /// Variable name of the sampler index buffer
    sampler_index_buffer_name: String,
    /// Variable name of the base index _into_ the sampler index buffer
    binding_array_base_index_name: String,
}

impl<'a, W: fmt::Write> super::Writer<'a, W> {
    pub fn new(out: W, options: &'a Options, pipeline_options: &'a PipelineOptions) -> Self {
        Self {
            out,
            names: crate::FastHashMap::default(),
            namer: proc::Namer::default(),
            options,
            pipeline_options,
            entry_point_io: crate::FastHashMap::default(),
            named_expressions: crate::NamedExpressions::default(),
            wrapped: super::Wrapped::default(),
            written_committed_intersection: false,
            written_candidate_intersection: false,
            continue_ctx: back::continue_forward::ContinueCtx::default(),
            temp_access_chain: Vec::new(),
            need_bake_expressions: Default::default(),
        }
    }

    fn reset(&mut self, module: &Module) {
        self.names.clear();
        self.namer.reset(
            module,
            &super::keywords::RESERVED_SET,
            proc::KeywordSet::empty(),
            &super::keywords::RESERVED_CASE_INSENSITIVE_SET,
            super::keywords::RESERVED_PREFIXES,
            &mut self.names,
        );
        self.entry_point_io.clear();
        self.named_expressions.clear();
        self.wrapped.clear();
        self.written_committed_intersection = false;
        self.written_candidate_intersection = false;
        self.continue_ctx.clear();
        self.need_bake_expressions.clear();
    }

    /// Generates statements to be inserted immediately before and at the very
    /// start of the body of each loop, to defeat infinite loop reasoning.
    /// The 0th item of the returned tuple should be inserted immediately prior
    /// to the loop and the 1st item should be inserted at the very start of
    /// the loop body.
    ///
    /// See [`back::msl::Writer::gen_force_bounded_loop_statements`] for details.
    fn gen_force_bounded_loop_statements(
        &mut self,
        level: back::Level,
    ) -> Option<(String, String)> {
        if !self.options.force_loop_bounding {
            return None;
        }

        let loop_bound_name = self.namer.call("loop_bound");
        let max = u32::MAX;
        // Count down from u32::MAX rather than up from 0 to avoid hang on
        // certain Intel drivers. See <https://github.com/gfx-rs/wgpu/issues/7319>.
        let decl = format!("{level}uint2 {loop_bound_name} = uint2({max}u, {max}u);");
        let level = level.next();
        let break_and_inc = format!(
            "{level}if (all({loop_bound_name} == uint2(0u, 0u))) {{ break; }}
{level}{loop_bound_name} -= uint2({loop_bound_name}.y == 0u, 1u);"
        );

        Some((decl, break_and_inc))
    }

    /// Helper method used to find which expressions of a given function require baking
    ///
    /// # Notes
    /// Clears `need_bake_expressions` set before adding to it
    fn update_expressions_to_bake(
        &mut self,
        module: &Module,
        func: &crate::Function,
        info: &valid::FunctionInfo,
    ) {
        use crate::Expression;
        self.need_bake_expressions.clear();
        for (exp_handle, expr) in func.expressions.iter() {
            let expr_info = &info[exp_handle];
            let min_ref_count = func.expressions[exp_handle].bake_ref_count();
            if min_ref_count <= expr_info.ref_count {
                self.need_bake_expressions.insert(exp_handle);
            }

            if let Expression::Math { fun, arg, arg1, .. } = *expr {
                match fun {
                    crate::MathFunction::Asinh
                    | crate::MathFunction::Acosh
                    | crate::MathFunction::Atanh
                    | crate::MathFunction::Unpack2x16float
                    | crate::MathFunction::Unpack2x16snorm
                    | crate::MathFunction::Unpack2x16unorm
                    | crate::MathFunction::Unpack4x8snorm
                    | crate::MathFunction::Unpack4x8unorm
                    | crate::MathFunction::Unpack4xI8
                    | crate::MathFunction::Unpack4xU8
                    | crate::MathFunction::Pack2x16float
                    | crate::MathFunction::Pack2x16snorm
                    | crate::MathFunction::Pack2x16unorm
                    | crate::MathFunction::Pack4x8snorm
                    | crate::MathFunction::Pack4x8unorm
                    | crate::MathFunction::Pack4xI8
                    | crate::MathFunction::Pack4xU8
                    | crate::MathFunction::Pack4xI8Clamp
                    | crate::MathFunction::Pack4xU8Clamp => {
                        self.need_bake_expressions.insert(arg);
                    }
                    crate::MathFunction::CountLeadingZeros => {
                        let inner = info[exp_handle].ty.inner_with(&module.types);
                        if let Some(ScalarKind::Sint) = inner.scalar_kind() {
                            self.need_bake_expressions.insert(arg);
                        }
                    }
                    crate::MathFunction::Dot4U8Packed | crate::MathFunction::Dot4I8Packed => {
                        self.need_bake_expressions.insert(arg);
                        self.need_bake_expressions.insert(arg1.unwrap());
                    }
                    _ => {}
                }
            }

            if let Expression::Derivative { axis, ctrl, expr } = *expr {
                use crate::{DerivativeAxis as Axis, DerivativeControl as Ctrl};
                if axis == Axis::Width && (ctrl == Ctrl::Coarse || ctrl == Ctrl::Fine) {
                    self.need_bake_expressions.insert(expr);
                }
            }

            if let Expression::GlobalVariable(_) = *expr {
                let inner = info[exp_handle].ty.inner_with(&module.types);

                if let TypeInner::Sampler { .. } = *inner {
                    self.need_bake_expressions.insert(exp_handle);
                }
            }
        }
        for statement in func.body.iter() {
            match *statement {
                crate::Statement::SubgroupCollectiveOperation {
                    op: _,
                    collective_op: crate::CollectiveOperation::InclusiveScan,
                    argument,
                    result: _,
                } => {
                    self.need_bake_expressions.insert(argument);
                }
                crate::Statement::Atomic {
                    fun: crate::AtomicFunction::Exchange { compare: Some(cmp) },
                    ..
                } => {
                    self.need_bake_expressions.insert(cmp);
                }
                _ => {}
            }
        }
    }

    pub fn write(
        &mut self,
        module: &Module,
        module_info: &valid::ModuleInfo,
        fragment_entry_point: Option<&FragmentEntryPoint<'_>>,
    ) -> Result<super::ReflectionInfo, Error> {
        self.reset(module);

        // Write special constants, if needed
        if let Some(ref bt) = self.options.special_constants_binding {
            writeln!(self.out, "struct {SPECIAL_CBUF_TYPE} {{")?;
            writeln!(self.out, "{}int {};", back::INDENT, SPECIAL_FIRST_VERTEX)?;
            writeln!(self.out, "{}int {};", back::INDENT, SPECIAL_FIRST_INSTANCE)?;
            writeln!(self.out, "{}uint {};", back::INDENT, SPECIAL_OTHER)?;
            writeln!(self.out, "}};")?;
            write!(
                self.out,
                "ConstantBuffer<{}> {}: register(b{}",
                SPECIAL_CBUF_TYPE, SPECIAL_CBUF_VAR, bt.register
            )?;
            if bt.space != 0 {
                write!(self.out, ", space{}", bt.space)?;
            }
            writeln!(self.out, ");")?;

            // Extra newline for readability
            writeln!(self.out)?;
        }

        for (group, bt) in self.options.dynamic_storage_buffer_offsets_targets.iter() {
            writeln!(self.out, "struct __dynamic_buffer_offsetsTy{group} {{")?;
            for i in 0..bt.size {
                writeln!(self.out, "{}uint _{};", back::INDENT, i)?;
            }
            writeln!(self.out, "}};")?;
            writeln!(
                self.out,
                "ConstantBuffer<__dynamic_buffer_offsetsTy{}> __dynamic_buffer_offsets{}: register(b{}, space{});",
                group, group, bt.register, bt.space
            )?;

            // Extra newline for readability
            writeln!(self.out)?;
        }

        // Save all entry point output types
        let ep_results = module
            .entry_points
            .iter()
            .map(|ep| (ep.stage, ep.function.result.clone()))
            .collect::<Vec<(ShaderStage, Option<crate::FunctionResult>)>>();

        self.write_all_mat_cx2_typedefs_and_functions(module)?;

        // Write all structs
        for (handle, ty) in module.types.iter() {
            if let TypeInner::Struct { ref members, span } = ty.inner {
                if module.types[members.last().unwrap().ty]
                    .inner
                    .is_dynamically_sized(&module.types)
                {
                    // unsized arrays can only be in storage buffers,
                    // for which we use `ByteAddressBuffer` anyway.
                    continue;
                }

                let ep_result = ep_results.iter().find(|e| {
                    if let Some(ref result) = e.1 {
                        result.ty == handle
                    } else {
                        false
                    }
                });

                self.write_struct(
                    module,
                    handle,
                    members,
                    span,
                    ep_result.map(|r| (r.0, Io::Output)),
                )?;
                writeln!(self.out)?;
            }
        }

        self.write_special_functions(module)?;

        self.write_wrapped_expression_functions(module, &module.global_expressions, None)?;
        self.write_wrapped_zero_value_functions(module, &module.global_expressions)?;

        // Write all named constants
        let mut constants = module
            .constants
            .iter()
            .filter(|&(_, c)| c.name.is_some())
            .peekable();
        while let Some((handle, _)) = constants.next() {
            self.write_global_constant(module, handle)?;
            // Add extra newline for readability on last iteration
            if constants.peek().is_none() {
                writeln!(self.out)?;
            }
        }

        // Write all globals
        for (global, _) in module.global_variables.iter() {
            self.write_global(module, global)?;
        }

        if !module.global_variables.is_empty() {
            // Add extra newline for readability
            writeln!(self.out)?;
        }

        let ep_range = get_entry_points(module, self.pipeline_options.entry_point.as_ref())
            .map_err(|(stage, name)| Error::EntryPointNotFound(stage, name))?;

        // Write all entry points wrapped structs
        for index in ep_range.clone() {
            let ep = &module.entry_points[index];
            let ep_name = self.names[&NameKey::EntryPoint(index as u16)].clone();
            let ep_io = self.write_ep_interface(
                module,
                &ep.function,
                ep.stage,
                &ep_name,
                fragment_entry_point,
            )?;
            self.entry_point_io.insert(index, ep_io);
        }

        // Write all regular functions
        for (handle, function) in module.functions.iter() {
            let info = &module_info[handle];

            // Check if all of the globals are accessible
            if !self.options.fake_missing_bindings {
                if let Some((var_handle, _)) =
                    module
                        .global_variables
                        .iter()
                        .find(|&(var_handle, var)| match var.binding {
                            Some(ref binding) if !info[var_handle].is_empty() => {
                                self.options.resolve_resource_binding(binding).is_err()
                                    && self
                                        .options
                                        .resolve_external_texture_resource_binding(binding)
                                        .is_err()
                            }
                            _ => false,
                        })
                {
                    log::debug!(
                        "Skipping function {:?} (name {:?}) because global {:?} is inaccessible",
                        handle,
                        function.name,
                        var_handle
                    );
                    continue;
                }
            }

            let ctx = back::FunctionCtx {
                ty: back::FunctionType::Function(handle),
                info,
                expressions: &function.expressions,
                named_expressions: &function.named_expressions,
            };
            let name = self.names[&NameKey::Function(handle)].clone();

            self.write_wrapped_functions(module, &ctx)?;

            self.write_function(module, name.as_str(), function, &ctx, info)?;

            writeln!(self.out)?;
        }

        let mut translated_ep_names = Vec::with_capacity(ep_range.len());

        // Write all entry points
        for index in ep_range {
            let ep = &module.entry_points[index];
            let info = module_info.get_entry_point(index);

            if !self.options.fake_missing_bindings {
                let mut ep_error = None;
                for (var_handle, var) in module.global_variables.iter() {
                    match var.binding {
                        Some(ref binding) if !info[var_handle].is_empty() => {
                            if let Err(err) = self.options.resolve_resource_binding(binding) {
                                if self
                                    .options
                                    .resolve_external_texture_resource_binding(binding)
                                    .is_err()
                                {
                                    ep_error = Some(err);
                                    break;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(err) = ep_error {
                    translated_ep_names.push(Err(err));
                    continue;
                }
            }

            let ctx = back::FunctionCtx {
                ty: back::FunctionType::EntryPoint(index as u16),
                info,
                expressions: &ep.function.expressions,
                named_expressions: &ep.function.named_expressions,
            };

            self.write_wrapped_functions(module, &ctx)?;

            if ep.stage.compute_like() {
                // HLSL is calling workgroup size "num threads"
                let num_threads = ep.workgroup_size;
                writeln!(
                    self.out,
                    "[numthreads({}, {}, {})]",
                    num_threads[0], num_threads[1], num_threads[2]
                )?;
            }

            let name = self.names[&NameKey::EntryPoint(index as u16)].clone();
            self.write_function(module, &name, &ep.function, &ctx, info)?;

            if index < module.entry_points.len() - 1 {
                writeln!(self.out)?;
            }

            translated_ep_names.push(Ok(name));
        }

        Ok(super::ReflectionInfo {
            entry_point_names: translated_ep_names,
        })
    }

    fn write_modifier(&mut self, binding: &crate::Binding) -> BackendResult {
        match *binding {
            crate::Binding::BuiltIn(crate::BuiltIn::Position { invariant: true }) => {
                write!(self.out, "precise ")?;
            }
            crate::Binding::BuiltIn(crate::BuiltIn::Barycentric { perspective: false }) => {
                write!(self.out, "noperspective ")?;
            }
            crate::Binding::Location {
                interpolation,
                sampling,
                ..
            } => {
                if let Some(interpolation) = interpolation {
                    if let Some(string) = interpolation.to_hlsl_str() {
                        write!(self.out, "{string} ")?
                    }
                }

                if let Some(sampling) = sampling {
                    if let Some(string) = sampling.to_hlsl_str() {
                        write!(self.out, "{string} ")?
                    }
                }
            }
            crate::Binding::BuiltIn(_) => {}
        }

        Ok(())
    }

    //TODO: we could force fragment outputs to always go through `entry_point_io.output` path
    // if they are struct, so that the `stage` argument here could be omitted.
    fn write_semantic(
        &mut self,
        binding: &Option<crate::Binding>,
        stage: Option<(ShaderStage, Io)>,
    ) -> BackendResult {
        match *binding {
            Some(crate::Binding::BuiltIn(builtin)) if !is_subgroup_builtin_binding(binding) => {
                if builtin == crate::BuiltIn::ViewIndex
                    && self.options.shader_model < ShaderModel::V6_1
                {
                    return Err(Error::ShaderModelTooLow(
                        "used @builtin(view_index) or SV_ViewID".to_string(),
                        ShaderModel::V6_1,
                    ));
                }
                let builtin_str = builtin.to_hlsl_str()?;
                write!(self.out, " : {builtin_str}")?;
            }
            Some(crate::Binding::Location {
                blend_src: Some(1), ..
            }) => {
                write!(self.out, " : SV_Target1")?;
            }
            Some(crate::Binding::Location { location, .. }) => {
                if stage == Some((ShaderStage::Fragment, Io::Output)) {
                    write!(self.out, " : SV_Target{location}")?;
                } else {
                    write!(self.out, " : {LOCATION_SEMANTIC}{location}")?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn write_interface_struct(
        &mut self,
        module: &Module,
        shader_stage: (ShaderStage, Io),
        struct_name: String,
        mut members: Vec<EpStructMember>,
    ) -> Result<EntryPointBinding, Error> {
        // Sort the members so that first come the user-defined varyings
        // in ascending locations, and then built-ins. This allows VS and FS
        // interfaces to match with regards to order.
        members.sort_by_key(|m| InterfaceKey::new(m.binding.as_ref()));

        write!(self.out, "struct {struct_name}")?;
        writeln!(self.out, " {{")?;
        let mut local_invocation_index_name = None;
        let mut subgroup_id_used = false;
        for m in members.iter() {
            // Sanity check that each IO member is a built-in or is assigned a
            // location. Also see note about nesting in `write_ep_input_struct`.
            debug_assert!(m.binding.is_some());

            match m.binding {
                Some(crate::Binding::BuiltIn(crate::BuiltIn::SubgroupId)) => {
                    subgroup_id_used = true;
                }
                Some(crate::Binding::BuiltIn(crate::BuiltIn::LocalInvocationIndex)) => {
                    local_invocation_index_name = Some(m.name.clone());
                }
                _ => (),
            }

            if is_subgroup_builtin_binding(&m.binding) {
                continue;
            }
            write!(self.out, "{}", back::INDENT)?;
            if let Some(ref binding) = m.binding {
                self.write_modifier(binding)?;
            }
            self.write_type(module, m.ty)?;
            write!(self.out, " {}", &m.name)?;
            self.write_semantic(&m.binding, Some(shader_stage))?;
            writeln!(self.out, ";")?;
        }
        if subgroup_id_used && local_invocation_index_name.is_none() {
            let name = self.namer.call("local_invocation_index");
            writeln!(self.out, "{}uint {name} : SV_GroupIndex;", back::INDENT)?;
            local_invocation_index_name = Some(name);
        }
        writeln!(self.out, "}};")?;
        writeln!(self.out)?;

        // See ordering notes on EntryPointInterface fields
        match shader_stage.1 {
            Io::Input => {
                // bring back the original order
                members.sort_by_key(|m| m.index);
            }
            Io::Output => {
                // keep it sorted by binding
            }
        }

        Ok(EntryPointBinding {
            arg_name: self.namer.call(struct_name.to_lowercase().as_str()),
            ty_name: struct_name,
            members,
            local_invocation_index_name,
        })
    }

    /// Flatten all entry point arguments into a single struct.
    /// This is needed since we need to re-order them: first placing user locations,
    /// then built-ins.
    fn write_ep_input_struct(
        &mut self,
        module: &Module,
        func: &crate::Function,
        stage: ShaderStage,
        entry_point_name: &str,
    ) -> Result<EntryPointBinding, Error> {
        let struct_name = format!("{stage:?}Input_{entry_point_name}");

        let mut fake_members = Vec::new();
        for arg in func.arguments.iter() {
            // NOTE: We don't need to handle nesting structs. All members must
            // be either built-ins or assigned a location. I.E. `binding` is
            // `Some`. This is checked in `VaryingContext::validate`. See:
            // https://gpuweb.github.io/gpuweb/wgsl/#input-output-locations
            match module.types[arg.ty].inner {
                TypeInner::Struct { ref members, .. } => {
                    for member in members.iter() {
                        let name = self.namer.call_or(&member.name, "member");
                        let index = fake_members.len() as u32;
                        fake_members.push(EpStructMember {
                            name,
                            ty: member.ty,
                            binding: member.binding.clone(),
                            index,
                        });
                    }
                }
                _ => {
                    let member_name = self.namer.call_or(&arg.name, "member");
                    let index = fake_members.len() as u32;
                    fake_members.push(EpStructMember {
                        name: member_name,
                        ty: arg.ty,
                        binding: arg.binding.clone(),
                        index,
                    });
                }
            }
        }

        self.write_interface_struct(module, (stage, Io::Input), struct_name, fake_members)
    }

    /// Flatten all entry point results into a single struct.
    /// This is needed since we need to re-order them: first placing user locations,
    /// then built-ins.
    fn write_ep_output_struct(
        &mut self,
        module: &Module,
        result: &crate::FunctionResult,
        stage: ShaderStage,
        entry_point_name: &str,
        frag_ep: Option<&FragmentEntryPoint<'_>>,
    ) -> Result<EntryPointBinding, Error> {
        let struct_name = format!("{stage:?}Output_{entry_point_name}");

        let empty = [];
        let members = match module.types[result.ty].inner {
            TypeInner::Struct { ref members, .. } => members,
            ref other => {
                log::error!("Unexpected {other:?} output type without a binding");
                &empty[..]
            }
        };

        // Gather list of fragment input locations. We use this below to remove user-defined
        // varyings from VS outputs that aren't in the FS inputs. This makes the VS interface match
        // as long as the FS inputs are a subset of the VS outputs. This is only applied if the
        // writer is supplied with information about the fragment entry point.
        let fs_input_locs = if let (Some(frag_ep), ShaderStage::Vertex) = (frag_ep, stage) {
            let mut fs_input_locs = Vec::new();
            for arg in frag_ep.func.arguments.iter() {
                let mut push_if_location = |binding: &Option<crate::Binding>| match *binding {
                    Some(crate::Binding::Location { location, .. }) => fs_input_locs.push(location),
                    Some(crate::Binding::BuiltIn(_)) | None => {}
                };

                // NOTE: We don't need to handle struct nesting. See note in
                // `write_ep_input_struct`.
                match frag_ep.module.types[arg.ty].inner {
                    TypeInner::Struct { ref members, .. } => {
                        for member in members.iter() {
                            push_if_location(&member.binding);
                        }
                    }
                    _ => push_if_location(&arg.binding),
                }
            }
            fs_input_locs.sort();
            Some(fs_input_locs)
        } else {
            None
        };

        let mut fake_members = Vec::new();
        for (index, member) in members.iter().enumerate() {
            if let Some(ref fs_input_locs) = fs_input_locs {
                match member.binding {
                    Some(crate::Binding::Location { location, .. }) => {
                        if fs_input_locs.binary_search(&location).is_err() {
                            continue;
                        }
                    }
                    Some(crate::Binding::BuiltIn(_)) | None => {}
                }
            }

            let member_name = self.namer.call_or(&member.name, "member");
            fake_members.push(EpStructMember {
                name: member_name,
                ty: member.ty,
                binding: member.binding.clone(),
                index: index as u32,
            });
        }

        self.write_interface_struct(module, (stage, Io::Output), struct_name, fake_members)
    }

    /// Writes special interface structures for an entry point. The special structures have
    /// all the fields flattened into them and sorted by binding. They are needed to emulate
    /// subgroup built-ins and to make the interfaces between VS outputs and FS inputs match.
    fn write_ep_interface(
        &mut self,
        module: &Module,
        func: &crate::Function,
        stage: ShaderStage,
        ep_name: &str,
        frag_ep: Option<&FragmentEntryPoint<'_>>,
    ) -> Result<EntryPointInterface, Error> {
        Ok(EntryPointInterface {
            input: if !func.arguments.is_empty()
                && (stage == ShaderStage::Fragment
                    || func
                        .arguments
                        .iter()
                        .any(|arg| is_subgroup_builtin_binding(&arg.binding)))
            {
                Some(self.write_ep_input_struct(module, func, stage, ep_name)?)
            } else {
                None
            },
            output: match func.result {
                Some(ref fr) if fr.binding.is_none() && stage == ShaderStage::Vertex => {
                    Some(self.write_ep_output_struct(module, fr, stage, ep_name, frag_ep)?)
                }
                _ => None,
            },
        })
    }

    fn write_ep_argument_initialization(
        &mut self,
        ep: &crate::EntryPoint,
        ep_input: &EntryPointBinding,
        fake_member: &EpStructMember,
    ) -> BackendResult {
        match fake_member.binding {
            Some(crate::Binding::BuiltIn(crate::BuiltIn::SubgroupSize)) => {
                write!(self.out, "WaveGetLaneCount()")?
            }
            Some(crate::Binding::BuiltIn(crate::BuiltIn::SubgroupInvocationId)) => {
                write!(self.out, "WaveGetLaneIndex()")?
            }
            Some(crate::Binding::BuiltIn(crate::BuiltIn::NumSubgroups)) => write!(
                self.out,
                "({}u + WaveGetLaneCount() - 1u) / WaveGetLaneCount()",
                ep.workgroup_size[0] * ep.workgroup_size[1] * ep.workgroup_size[2]
            )?,
            Some(crate::Binding::BuiltIn(crate::BuiltIn::SubgroupId)) => {
                write!(
                    self.out,
                    "{}.{} / WaveGetLaneCount()",
                    ep_input.arg_name,
                    // When writing SubgroupId, we always guarantee that local_invocation_index_name is written
                    ep_input.local_invocation_index_name.as_ref().unwrap()
                )?;
            }
            _ => {
                write!(self.out, "{}.{}", ep_input.arg_name, fake_member.name)?;
            }
        }
        Ok(())
    }

    /// Write an entry point preface that initializes the arguments as specified in IR.
    fn write_ep_arguments_initialization(
        &mut self,
        module: &Module,
        func: &crate::Function,
        ep_index: u16,
    ) -> BackendResult {
        let ep = &module.entry_points[ep_index as usize];
        let ep_input = match self
            .entry_point_io
            .get_mut(&(ep_index as usize))
            .unwrap()
            .input
            .take()
        {
            Some(ep_input) => ep_input,
            None => return Ok(()),
        };
        let mut fake_iter = ep_input.members.iter();
        for (arg_index, arg) in func.arguments.iter().enumerate() {
            write!(self.out, "{}", back::INDENT)?;
            self.write_type(module, arg.ty)?;
            let arg_name = &self.names[&NameKey::EntryPointArgument(ep_index, arg_index as u32)];
            write!(self.out, " {arg_name}")?;
            match module.types[arg.ty].inner {
                TypeInner::Array { base, size, .. } => {
                    self.write_array_size(module, base, size)?;
                    write!(self.out, " = ")?;
                    self.write_ep_argument_initialization(
                        ep,
                        &ep_input,
                        fake_iter.next().unwrap(),
                    )?;
                    writeln!(self.out, ";")?;
                }
                TypeInner::Struct { ref members, .. } => {
                    write!(self.out, " = {{ ")?;
                    for index in 0..members.len() {
                        if index != 0 {
                            write!(self.out, ", ")?;
                        }
                        self.write_ep_argument_initialization(
                            ep,
                            &ep_input,
                            fake_iter.next().unwrap(),
                        )?;
                    }
                    writeln!(self.out, " }};")?;
                }
                _ => {
                    write!(self.out, " = ")?;
                    self.write_ep_argument_initialization(
                        ep,
                        &ep_input,
                        fake_iter.next().unwrap(),
                    )?;
                    writeln!(self.out, ";")?;
                }
            }
        }
        assert!(fake_iter.next().is_none());
        Ok(())
    }

    /// Helper method used to write global variables
    /// # Notes
    /// Always adds a newline
    fn write_global(
        &mut self,
        module: &Module,
        handle: Handle<crate::GlobalVariable>,
    ) -> BackendResult {
        let global = &module.global_variables[handle];
        let inner = &module.types[global.ty].inner;

        let handle_ty = match *inner {
            TypeInner::BindingArray { ref base, .. } => &module.types[*base].inner,
            _ => inner,
        };

        // External textures are handled entirely differently, so defer entirely to that method.
        // We do so prior to calling resolve_resource_binding() below, as we even need to resolve
        // their bindings separately.
        let is_external_texture = matches!(
            *handle_ty,
            TypeInner::Image {
                class: crate::ImageClass::External,
                ..
            }
        );
        if is_external_texture {
            return self.write_global_external_texture(module, handle, global);
        }

        if let Some(ref binding) = global.binding {
            if let Err(err) = self.options.resolve_resource_binding(binding) {
                log::debug!(
                    "Skipping global {:?} (name {:?}) for being inaccessible: {}",
                    handle,
                    global.name,
                    err,
                );
                return Ok(());
            }
        }

        // Samplers are handled entirely differently, so defer entirely to that method.
        let is_sampler = matches!(*handle_ty, TypeInner::Sampler { .. });

        if is_sampler {
            return self.write_global_sampler(module, handle, global);
        }

        // https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-variable-register
        let register_ty = match global.space {
            crate::AddressSpace::Function => unreachable!("Function address space"),
            crate::AddressSpace::Private => {
                write!(self.out, "static ")?;
                self.write_type(module, global.ty)?;
                ""
            }
            crate::AddressSpace::WorkGroup => {
                write!(self.out, "groupshared ")?;
                self.write_type(module, global.ty)?;
                ""
            }
            crate::AddressSpace::TaskPayload => unimplemented!(),
            crate::AddressSpace::Uniform => {
                // constant buffer declarations are expected to be inlined, e.g.
                // `cbuffer foo: register(b0) { field1: type1; }`
                write!(self.out, "cbuffer")?;
                "b"
            }
            crate::AddressSpace::Storage { access } => {
                if global
                    .memory_decorations
                    .contains(crate::MemoryDecorations::COHERENT)
                {
                    write!(self.out, "globallycoherent ")?;
                }
                let (prefix, register) = if access.contains(crate::StorageAccess::STORE) {
                    ("RW", "u")
                } else {
                    ("", "t")
                };
                write!(self.out, "{prefix}ByteAddressBuffer")?;
                register
            }
            crate::AddressSpace::Handle => {
                let register = match *handle_ty {
                    // all storage textures are UAV, unconditionally
                    TypeInner::Image {
                        class: crate::ImageClass::Storage { .. },
                        ..
                    } => "u",
                    _ => "t",
                };
                self.write_type(module, global.ty)?;
                register
            }
            crate::AddressSpace::Immediate => {
                // The type of the immediates will be wrapped in `ConstantBuffer`
                write!(self.out, "ConstantBuffer<")?;
                "b"
            }
            crate::AddressSpace::RayPayload | crate::AddressSpace::IncomingRayPayload => {
                unimplemented!()
            }
        };

        // If the global is a immediate data write the type now because it will be a
        // generic argument to `ConstantBuffer`
        if global.space == crate::AddressSpace::Immediate {
            self.write_global_type(module, global.ty)?;

            // need to write the array size if the type was emitted with `write_type`
            if let TypeInner::Array { base, size, .. } = module.types[global.ty].inner {
                self.write_array_size(module, base, size)?;
            }

            // Close the angled brackets for the generic argument
            write!(self.out, ">")?;
        }

        let name = &self.names[&NameKey::GlobalVariable(handle)];
        write!(self.out, " {name}")?;

        // Immediates need to be assigned a binding explicitly by the consumer
        // since naga has no way to know the binding from the shader alone
        if global.space == crate::AddressSpace::Immediate {
            match module.types[global.ty].inner {
                TypeInner::Struct { .. } => {}
                _ => {
                    return Err(Error::Unimplemented(format!(
                        "push-constant '{name}' has non-struct type; tracked by: https://github.com/gfx-rs/wgpu/issues/5683"
                    )));
                }
            }

            let target = self
                .options
                .immediates_target
                .as_ref()
                .expect("No bind target was defined for the immediates block");
            write!(self.out, ": register(b{}", target.register)?;
            if target.space != 0 {
                write!(self.out, ", space{}", target.space)?;
            }
            write!(self.out, ")")?;
        }

        if let Some(ref binding) = global.binding {
            // this was already resolved earlier when we started evaluating an entry point.
            let bt = self.options.resolve_resource_binding(binding).unwrap();

            // need to write the binding array size if the type was emitted with `write_type`
            if let TypeInner::BindingArray { base, size, .. } = module.types[global.ty].inner {
                if let Some(overridden_size) = bt.binding_array_size {
                    write!(self.out, "[{overridden_size}]")?;
                } else {
                    self.write_array_size(module, base, size)?;
                }
            }

            write!(self.out, " : register({}{}", register_ty, bt.register)?;
            if bt.space != 0 {
                write!(self.out, ", space{}", bt.space)?;
            }
            write!(self.out, ")")?;
        } else {
            // need to write the array size if the type was emitted with `write_type`
            if let TypeInner::Array { base, size, .. } = module.types[global.ty].inner {
                self.write_array_size(module, base, size)?;
            }
            if global.space == crate::AddressSpace::Private {
                write!(self.out, " = ")?;
                if let Some(init) = global.init {
                    self.write_const_expression(module, init, &module.global_expressions)?;
                } else {
                    self.write_default_init(module, global.ty)?;
                }
            }
        }

        if global.space == crate::AddressSpace::Uniform {
            write!(self.out, " {{ ")?;

            self.write_global_type(module, global.ty)?;

            write!(
                self.out,
                " {}",
                &self.names[&NameKey::GlobalVariable(handle)]
            )?;

            // need to write the array size if the type was emitted with `write_type`
            if let TypeInner::Array { base, size, .. } = module.types[global.ty].inner {
                self.write_array_size(module, base, size)?;
            }

            writeln!(self.out, "; }}")?;
        } else {
            writeln!(self.out, ";")?;
        }

        Ok(())
    }

    fn write_global_sampler(
        &mut self,
        module: &Module,
        handle: Handle<crate::GlobalVariable>,
        global: &crate::GlobalVariable,
    ) -> BackendResult {
        let binding = *global.binding.as_ref().unwrap();

        let key = super::SamplerIndexBufferKey {
            group: binding.group,
        };
        self.write_wrapped_sampler_buffer(key)?;

        // This was already validated, so we can confidently unwrap it.
        let bt = self.options.resolve_resource_binding(&binding).unwrap();

        match module.types[global.ty].inner {
            TypeInner::Sampler { comparison } => {
                // If we are generating a static access, we create a variable for the sampler.
                //
                // This prevents the DXIL from containing multiple lookups for the sampler, which
                // the backend compiler will then have to eliminate. AMD does seem to be able to
                // eliminate these, but better safe than sorry.

                write!(self.out, "static const ")?;
                self.write_type(module, global.ty)?;

                let heap_var = if comparison {
                    COMPARISON_SAMPLER_HEAP_VAR
                } else {
                    SAMPLER_HEAP_VAR
                };

                let index_buffer_name = &self.wrapped.sampler_index_buffers[&key];
                let name = &self.names[&NameKey::GlobalVariable(handle)];
                writeln!(
                    self.out,
                    " {name} = {heap_var}[{index_buffer_name}[{register}]];",
                    register = bt.register
                )?;
            }
            TypeInner::BindingArray { .. } => {
                // If we are generating a binding array, we cannot directly access the sampler as the index
                // into the sampler index buffer is unknown at compile time. Instead we generate a constant
                // that represents the "base" index into the sampler index buffer. This constant is added
                // to the user provided index to get the final index into the sampler index buffer.

                let name = &self.names[&NameKey::GlobalVariable(handle)];
                writeln!(
                    self.out,
                    "static const uint {name} = {register};",
                    register = bt.register
                )?;
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    /// Write the declarations for an external texture global variable.
    /// These are emitted as multiple global variables: Three `Texture2D`s
    /// (one for each plane) and a parameters cbuffer.
    fn write_global_external_texture(
        &mut self,
        module: &Module,
        handle: Handle<crate::GlobalVariable>,
        global: &crate::GlobalVariable,
    ) -> BackendResult {
        let res_binding = global
            .binding
            .as_ref()
            .expect("External texture global variables must have a resource binding");
        let ext_tex_bindings = match self
            .options
            .resolve_external_texture_resource_binding(res_binding)
        {
            Ok(bindings) => bindings,
            Err(err) => {
                log::debug!(
                    "Skipping global {:?} (name {:?}) for being inaccessible: {}",
                    handle,
                    global.name,
                    err,
                );
                return Ok(());
            }
        };

        let mut write_plane = |bt: &super::BindTarget, name| -> BackendResult {
            write!(
                self.out,
                "Texture2D<float4> {}: register(t{}",
                name, bt.register
            )?;
            if bt.space != 0 {
                write!(self.out, ", space{}", bt.space)?;
            }
            writeln!(self.out, ");")?;
            Ok(())
        };
        for (i, bt) in ext_tex_bindings.planes.iter().enumerate() {
            let plane_name = &self.names
                [&NameKey::ExternalTextureGlobalVariable(handle, ExternalTextureNameKey::Plane(i))];
            write_plane(bt, plane_name)?;
        }

        let params_name = &self.names
            [&NameKey::ExternalTextureGlobalVariable(handle, ExternalTextureNameKey::Params)];
        let params_ty_name =
            &self.names[&NameKey::Type(module.special_types.external_texture_params.unwrap())];
        write!(
            self.out,
            "cbuffer {}: register(b{}",
            params_name, ext_tex_bindings.params.register
        )?;
        if ext_tex_bindings.params.space != 0 {
            write!(self.out, ", space{}", ext_tex_bindings.params.space)?;
        }
        writeln!(self.out, ") {{ {params_ty_name} {params_name}; }};")?;

        Ok(())
    }

    /// Helper method used to write global constants
    ///
    /// # Notes
    /// Ends in a newline
    fn write_global_constant(
        &mut self,
        module: &Module,
        handle: Handle<crate::Constant>,
    ) -> BackendResult {
        write!(self.out, "static const ")?;
        let constant = &module.constants[handle];
        self.write_type(module, constant.ty)?;
        let name = &self.names[&NameKey::Constant(handle)];
        write!(self.out, " {name}")?;
        // Write size for array type
        if let TypeInner::Array { base, size, .. } = module.types[constant.ty].inner {
            self.write_array_size(module, base, size)?;
        }
        write!(self.out, " = ")?;
        self.write_const_expression(module, constant.init, &module.global_expressions)?;
        writeln!(self.out, ";")?;
        Ok(())
    }

    pub(super) fn write_array_size(
        &mut self,
        module: &Module,
        base: Handle<crate::Type>,
        size: crate::ArraySize,
    ) -> BackendResult {
        write!(self.out, "[")?;

        match size.resolve(module.to_ctx())? {
            proc::IndexableLength::Known(size) => {
                write!(self.out, "{size}")?;
            }
            proc::IndexableLength::Dynamic => unreachable!(),
        }

        write!(self.out, "]")?;

        if let TypeInner::Array {
            base: next_base,
            size: next_size,
            ..
        } = module.types[base].inner
        {
            self.write_array_size(module, next_base, next_size)?;
        }

        Ok(())
    }

    /// Helper method used to write structs
    ///
    /// # Notes
    /// Ends in a newline
    fn write_struct(
        &mut self,
        module: &Module,
        handle: Handle<crate::Type>,
        members: &[crate::StructMember],
        span: u32,
        shader_stage: Option<(ShaderStage, Io)>,
    ) -> BackendResult {
        // Write struct name
        let struct_name = &self.names[&NameKey::Type(handle)];
        writeln!(self.out, "struct {struct_name} {{")?;

        let mut last_offset = 0;
        for (index, member) in members.iter().enumerate() {
            if member.binding.is_none() && member.offset > last_offset {
                // using int as padding should work as long as the backend
                // doesn't support a type that's less than 4 bytes in size
                // (Error::UnsupportedScalar catches this)
                let padding = (member.offset - last_offset) / 4;
                for i in 0..padding {
                    writeln!(self.out, "{}int _pad{}_{};", back::INDENT, index, i)?;
                }
            }
            let ty_inner = &module.types[member.ty].inner;
            last_offset = member.offset + ty_inner.size_hlsl(module.to_ctx())?;

            // The indentation is only for readability
            write!(self.out, "{}", back::INDENT)?;

            match module.types[member.ty].inner {
                TypeInner::Array { base, size, .. } => {
                    // HLSL arrays are written as `type name[size]`

                    self.write_global_type(module, member.ty)?;

                    // Write `name`
                    write!(
                        self.out,
                        " {}",
                        &self.names[&NameKey::StructMember(handle, index as u32)]
                    )?;
                    // Write [size]
                    self.write_array_size(module, base, size)?;
                }
                // We treat matrices of the form `matCx2` as a sequence of C `vec2`s.
                // See the module-level block comment in mod.rs for details.
                TypeInner::Matrix {
                    rows,
                    columns,
                    scalar,
                } if member.binding.is_none() && rows == crate::VectorSize::Bi => {
                    let vec_ty = TypeInner::Vector { size: rows, scalar };
                    let field_name_key = NameKey::StructMember(handle, index as u32);

                    for i in 0..columns as u8 {
                        if i != 0 {
                            write!(self.out, "; ")?;
                        }
                        self.write_value_type(module, &vec_ty)?;
                        write!(self.out, " {}_{}", &self.names[&field_name_key], i)?;
                    }
                }
                _ => {
                    // Write modifier before type
                    if let Some(ref binding) = member.binding {
                        self.write_modifier(binding)?;
                    }

                    // Even though Naga IR matrices are column-major, we must describe
                    // matrices passed from the CPU as being in row-major order.
                    // See the module-level block comment in mod.rs for details.
                    if let TypeInner::Matrix { .. } = module.types[member.ty].inner {
                        write!(self.out, "row_major ")?;
                    }

                    // Write the member type and name
                    self.write_type(module, member.ty)?;
                    write!(
                        self.out,
                        " {}",
                        &self.names[&NameKey::StructMember(handle, index as u32)]
                    )?;
                }
            }

            self.write_semantic(&member.binding, shader_stage)?;
            writeln!(self.out, ";")?;
        }

        // add padding at the end since sizes of types don't get rounded up to their alignment in HLSL
        if members.last().unwrap().binding.is_none() && span > last_offset {
            let padding = (span - last_offset) / 4;
            for i in 0..padding {
                writeln!(self.out, "{}int _end_pad_{};", back::INDENT, i)?;
            }
        }

        writeln!(self.out, "}};")?;
        Ok(())
    }

    /// Helper method used to write global/structs non image/sampler types
    ///
    /// # Notes
    /// Adds no trailing or leading whitespace
    pub(super) fn write_global_type(
        &mut self,
        module: &Module,
        ty: Handle<crate::Type>,
    ) -> BackendResult {
        let matrix_data = get_inner_matrix_data(module, ty);

        // We treat matrices of the form `matCx2` as a sequence of C `vec2`s.
        // See the module-level block comment in mod.rs for details.
        if let Some(MatrixType {
            columns,
            rows: crate::VectorSize::Bi,
            width: 4,
        }) = matrix_data
        {
            write!(self.out, "__mat{}x2", columns as u8)?;
        } else {
            // Even though Naga IR matrices are column-major, we must describe
            // matrices passed from the CPU as being in row-major order.
            // See the module-level block comment in mod.rs for details.
            if matrix_data.is_some() {
                write!(self.out, "row_major ")?;
            }

            self.write_type(module, ty)?;
        }

        Ok(())
    }

    /// Helper method used to write non image/sampler types
    ///
    /// # Notes
    /// Adds no trailing or leading whitespace
    pub(super) fn write_type(&mut self, module: &Module, ty: Handle<crate::Type>) -> BackendResult {
        let inner = &module.types[ty].inner;
        match *inner {
            TypeInner::Struct { .. } => write!(self.out, "{}", self.names[&NameKey::Type(ty)])?,
            // hlsl array has the size separated from the base type
            TypeInner::Array { base, .. } | TypeInner::BindingArray { base, .. } => {
                self.write_type(module, base)?
            }
            ref other => self.write_value_type(module, other)?,
        }

        Ok(())
    }

    /// Helper method used to write value types
    ///
    /// # Notes
    /// Adds no trailing or leading whitespace
    pub(super) fn write_value_type(&mut self, module: &Module, inner: &TypeInner) -> BackendResult {
        match *inner {
            TypeInner::Scalar(scalar) | TypeInner::Atomic(scalar) => {
                write!(self.out, "{}", scalar.to_hlsl_str()?)?;
            }
            TypeInner::Vector { size, scalar } => {
                write!(
                    self.out,
                    "{}{}",
                    scalar.to_hlsl_str()?,
                    common::vector_size_str(size)
                )?;
            }
            TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } => {
                // The IR supports only float matrix
                // https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-matrix

                // Because of the implicit transpose all matrices have in HLSL, we need to transpose the size as well.
                write!(
                    self.out,
                    "{}{}x{}",
                    scalar.to_hlsl_str()?,
                    common::vector_size_str(columns),
                    common::vector_size_str(rows),
                )?;
            }
            TypeInner::Image {
                dim,
                arrayed,
                class,
            } => {
                self.write_image_type(dim, arrayed, class)?;
            }
            TypeInner::Sampler { comparison } => {
                let sampler = if comparison {
                    "SamplerComparisonState"
                } else {
                    "SamplerState"
                };
                write!(self.out, "{sampler}")?;
            }
            // HLSL arrays are written as `type name[size]`
            // Current code is written arrays only as `[size]`
            // Base `type` and `name` should be written outside
            TypeInner::Array { base, size, .. } | TypeInner::BindingArray { base, size } => {
                self.write_array_size(module, base, size)?;
            }
            TypeInner::AccelerationStructure { .. } => {
                write!(self.out, "RaytracingAccelerationStructure")?;
            }
            TypeInner::RayQuery { .. } => {
                // these are constant flags, there are dynamic flags also but constant flags are not supported by naga
                write!(self.out, "RayQuery<RAY_FLAG_NONE>")?;
            }
            _ => return Err(Error::Unimplemented(format!("write_value_type {inner:?}"))),
        }

        Ok(())
    }

    /// Helper method used to write functions
    /// # Notes
    /// Ends in a newline
    fn write_function(
        &mut self,
        module: &Module,
        name: &str,
        func: &crate::Function,
        func_ctx: &back::FunctionCtx<'_>,
        info: &valid::FunctionInfo,
    ) -> BackendResult {
        // Function Declaration Syntax - https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-function-syntax

        self.update_expressions_to_bake(module, func, info);

        if let Some(ref result) = func.result {
            // Write typedef if return type is an array
            let array_return_type = match module.types[result.ty].inner {
                TypeInner::Array { base, size, .. } => {
                    let array_return_type = self.namer.call(&format!("ret_{name}"));
                    write!(self.out, "typedef ")?;
                    self.write_type(module, result.ty)?;
                    write!(self.out, " {array_return_type}")?;
                    self.write_array_size(module, base, size)?;
                    writeln!(self.out, ";")?;
                    Some(array_return_type)
                }
                _ => None,
            };

            // Write modifier
            if let Some(
                ref binding @ crate::Binding::BuiltIn(crate::BuiltIn::Position { invariant: true }),
            ) = result.binding
            {
                self.write_modifier(binding)?;
            }

            // Write return type
            match func_ctx.ty {
                back::FunctionType::Function(_) => {
                    if let Some(array_return_type) = array_return_type {
                        write!(self.out, "{array_return_type}")?;
                    } else {
                        self.write_type(module, result.ty)?;
                    }
                }
                back::FunctionType::EntryPoint(index) => {
                    if let Some(ref ep_output) =
                        self.entry_point_io.get(&(index as usize)).unwrap().output
                    {
                        write!(self.out, "{}", ep_output.ty_name)?;
                    } else {
                        self.write_type(module, result.ty)?;
                    }
                }
            }
        } else {
            write!(self.out, "void")?;
        }

        // Write function name
        write!(self.out, " {name}(")?;

        let need_workgroup_variables_initialization =
            self.need_workgroup_variables_initialization(func_ctx, module);

        let needs_local_invocation_id_name = need_workgroup_variables_initialization;
        let mut local_invocation_id_name = None;
        // Write function arguments for non entry point functions
        match func_ctx.ty {
            back::FunctionType::Function(handle) => {
                for (index, arg) in func.arguments.iter().enumerate() {
                    if index != 0 {
                        write!(self.out, ", ")?;
                    }

                    self.write_function_argument(module, handle, arg, index)?;
                }
            }
            back::FunctionType::EntryPoint(ep_index) => {
                if let Some(ref ep_input) =
                    self.entry_point_io.get(&(ep_index as usize)).unwrap().input
                {
                    write!(self.out, "{} {}", ep_input.ty_name, ep_input.arg_name)?;
                } else {
                    let stage = module.entry_points[ep_index as usize].stage;
                    for (index, arg) in func.arguments.iter().enumerate() {
                        if index != 0 {
                            write!(self.out, ", ")?;
                        }
                        self.write_type(module, arg.ty)?;

                        let argument_name =
                            &self.names[&NameKey::EntryPointArgument(ep_index, index as u32)];

                        if arg.binding
                            == Some(crate::Binding::BuiltIn(crate::BuiltIn::LocalInvocationId))
                        {
                            local_invocation_id_name = Some(argument_name.clone());
                        }

                        write!(self.out, " {argument_name}")?;
                        if let TypeInner::Array { base, size, .. } = module.types[arg.ty].inner {
                            self.write_array_size(module, base, size)?;
                        }

                        self.write_semantic(&arg.binding, Some((stage, Io::Input)))?;
                    }
                }
                if needs_local_invocation_id_name && local_invocation_id_name.is_none() {
                    if self
                        .entry_point_io
                        .get(&(ep_index as usize))
                        .unwrap()
                        .input
                        .is_some()
                        || !func.arguments.is_empty()
                    {
                        write!(self.out, ", ")?;
                    }
                    let var_name = self.namer.call("local_invocation_id");
                    write!(self.out, "uint3 {var_name} : SV_GroupThreadID")?;
                    local_invocation_id_name = Some(var_name);
                }
            }
        }
        // Ends of arguments
        write!(self.out, ")")?;

        // Write semantic if it present
        if let back::FunctionType::EntryPoint(index) = func_ctx.ty {
            let stage = module.entry_points[index as usize].stage;
            if let Some(crate::FunctionResult { ref binding, .. }) = func.result {
                self.write_semantic(binding, Some((stage, Io::Output)))?;
            }
        }

        // Function body start
        writeln!(self.out)?;
        writeln!(self.out, "{{")?;

        if need_workgroup_variables_initialization {
            self.write_workgroup_variables_initialization(
                func_ctx,
                module,
                // need_workgroup_variables_initialization forces this to be written
                // if the user doesn't specify it (so this must be Some())
                local_invocation_id_name.unwrap(),
            )?;
        }

        if let back::FunctionType::EntryPoint(index) = func_ctx.ty {
            self.write_ep_arguments_initialization(module, func, index)?;
        }

        // Write function local variables
        for (handle, local) in func.local_variables.iter() {
            // Write indentation (only for readability)
            write!(self.out, "{}", back::INDENT)?;

            // Write the local name
            // The leading space is important
            self.write_type(module, local.ty)?;
            write!(self.out, " {}", self.names[&func_ctx.name_key(handle)])?;
            // Write size for array type
            if let TypeInner::Array { base, size, .. } = module.types[local.ty].inner {
                self.write_array_size(module, base, size)?;
            }

            let is_ray_query = match module.types[local.ty].inner {
                // from https://microsoft.github.io/DirectX-Specs/d3d/Raytracing.html#tracerayinline-example-1 it seems that ray queries shouldn't be zeroed
                TypeInner::RayQuery { .. } => true,
                _ => {
                    write!(self.out, " = ")?;
                    // Write the local initializer if needed
                    if let Some(init) = local.init {
                        self.write_expr(module, init, func_ctx)?;
                    } else {
                        // Zero initialize local variables
                        self.write_default_init(module, local.ty)?;
                    }
                    false
                }
            };
            // Finish the local with `;` and add a newline (only for readability)
            writeln!(self.out, ";")?;
            // If it's a ray query, we also want a tracker variable
            if is_ray_query {
                write!(self.out, "{}", back::INDENT)?;
                self.write_value_type(module, &TypeInner::Scalar(Scalar::U32))?;
                writeln!(
                    self.out,
                    " {RAY_QUERY_TRACKER_VARIABLE_PREFIX}{} = 0;",
                    self.names[&func_ctx.name_key(handle)]
                )?;
            }
        }

        if !func.local_variables.is_empty() {
            writeln!(self.out)?;
        }

        // Write the function body (statement list)
        for sta in func.body.iter() {
            // The indentation should always be 1 when writing the function body
            self.write_stmt(module, sta, func_ctx, back::Level(1))?;
        }

        writeln!(self.out, "}}")?;

        self.named_expressions.clear();

        Ok(())
    }

    fn write_function_argument(
        &mut self,
        module: &Module,
        handle: Handle<crate::Function>,
        arg: &crate::FunctionArgument,
        index: usize,
    ) -> BackendResult {
        // External texture arguments must be expanded into separate
        // arguments for each plane and the params buffer.
        if let TypeInner::Image {
            class: crate::ImageClass::External,
            ..
        } = module.types[arg.ty].inner
        {
            return self.write_function_external_texture_argument(module, handle, index);
        }

        // Write argument type
        let arg_ty = match module.types[arg.ty].inner {
            // pointers in function arguments are expected and resolve to `inout`
            TypeInner::Pointer { base, .. } => {
                //TODO: can we narrow this down to just `in` when possible?
                write!(self.out, "inout ")?;
                base
            }
            _ => arg.ty,
        };
        self.write_type(module, arg_ty)?;

        let argument_name = &self.names[&NameKey::FunctionArgument(handle, index as u32)];

        // Write argument name. Space is important.
        write!(self.out, " {argument_name}")?;
        if let TypeInner::Array { base, size, .. } = module.types[arg_ty].inner {
            self.write_array_size(module, base, size)?;
        }

        Ok(())
    }

    fn write_function_external_texture_argument(
        &mut self,
        module: &Module,
        handle: Handle<crate::Function>,
        index: usize,
    ) -> BackendResult {
        let plane_names = [0, 1, 2].map(|i| {
            &self.names[&NameKey::ExternalTextureFunctionArgument(
                handle,
                index as u32,
                ExternalTextureNameKey::Plane(i),
            )]
        });
        let params_name = &self.names[&NameKey::ExternalTextureFunctionArgument(
            handle,
            index as u32,
            ExternalTextureNameKey::Params,
        )];
        let params_ty_name =
            &self.names[&NameKey::Type(module.special_types.external_texture_params.unwrap())];
        write!(
            self.out,
            "Texture2D<float4> {}, Texture2D<float4> {}, Texture2D<float4> {}, {params_ty_name} {params_name}",
            plane_names[0], plane_names[1], plane_names[2],
        )?;
        Ok(())
    }

    fn need_workgroup_variables_initialization(
        &mut self,
        func_ctx: &back::FunctionCtx,
        module: &Module,
    ) -> bool {
        self.options.zero_initialize_workgroup_memory
            && func_ctx.ty.is_compute_like_entry_point(module)
            && module.global_variables.iter().any(|(handle, var)| {
                !func_ctx.info[handle].is_empty() && var.space == crate::AddressSpace::WorkGroup
            })
    }

    fn write_workgroup_variables_initialization(
        &mut self,
        func_ctx: &back::FunctionCtx,
        module: &Module,
        local_invocation_id_name: String,
    ) -> BackendResult {
        let level = back::Level(1);

        writeln!(
            self.out,
            "{level}if (all({local_invocation_id_name} == uint3(0u, 0u, 0u))) {{"
        )?;

        let vars = module.global_variables.iter().filter(|&(handle, var)| {
            !func_ctx.info[handle].is_empty() && var.space == crate::AddressSpace::WorkGroup
        });

        for (handle, var) in vars {
            let name = &self.names[&NameKey::GlobalVariable(handle)];
            write!(self.out, "{}{} = ", level.next(), name)?;
            self.write_default_init(module, var.ty)?;
            writeln!(self.out, ";")?;
        }

        writeln!(self.out, "{level}}}")?;
        self.write_control_barrier(crate::Barrier::WORK_GROUP, level)
    }

    /// Helper method used to write switches
    fn write_switch(
        &mut self,
        module: &Module,
        func_ctx: &back::FunctionCtx<'_>,
        level: back::Level,
        selector: Handle<crate::Expression>,
        cases: &[crate::SwitchCase],
    ) -> BackendResult {
        // Write all cases
        let indent_level_1 = level.next();
        let indent_level_2 = indent_level_1.next();

        // See docs of `back::continue_forward` module.
        if let Some(variable) = self.continue_ctx.enter_switch(&mut self.namer) {
            writeln!(self.out, "{level}bool {variable} = false;",)?;
        };

        // Check if there is only one body, by seeing if all except the last case are fall through
        // with empty bodies. FXC doesn't handle these switches correctly, so
        // we generate a `do {} while(false);` loop instead. There must be a default case, so there
        // is no need to check if one of the cases would have matched.
        let one_body = cases
            .iter()
            .rev()
            .skip(1)
            .all(|case| case.fall_through && case.body.is_empty());
        if one_body {
            // Start the do-while
            writeln!(self.out, "{level}do {{")?;
            // Note: Expressions have no side-effects so we don't need to emit selector expression.

            // Body
            if let Some(case) = cases.last() {
                for sta in case.body.iter() {
                    self.write_stmt(module, sta, func_ctx, indent_level_1)?;
                }
            }
            // End do-while
            writeln!(self.out, "{level}}} while(false);")?;
        } else {
            // Start the switch
            write!(self.out, "{level}")?;
            write!(self.out, "switch(")?;
            self.write_expr(module, selector, func_ctx)?;
            writeln!(self.out, ") {{")?;

            for (i, case) in cases.iter().enumerate() {
                match case.value {
                    crate::SwitchValue::I32(value) => {
                        write!(self.out, "{indent_level_1}case {value}:")?
                    }
                    crate::SwitchValue::U32(value) => {
                        write!(self.out, "{indent_level_1}case {value}u:")?
                    }
                    crate::SwitchValue::Default => write!(self.out, "{indent_level_1}default:")?,
                }

                // The new block is not only stylistic, it plays a role here:
                // We might end up having to write the same case body
                // multiple times due to FXC not supporting fallthrough.
                // Therefore, some `Expression`s written by `Statement::Emit`
                // will end up having the same name (`_expr<handle_index>`).
                // So we need to put each case in its own scope.
                let write_block_braces = !(case.fall_through && case.body.is_empty());
                if write_block_braces {
                    writeln!(self.out, " {{")?;
                } else {
                    writeln!(self.out)?;
                }

                // Although FXC does support a series of case clauses before
                // a block[^yes], it does not support fallthrough from a
                // non-empty case block to the next[^no]. If this case has a
                // non-empty body with a fallthrough, emulate that by
                // duplicating the bodies of all the cases it would fall
                // into as extensions of this case's own body. This makes
                // the HLSL output potentially quadratic in the size of the
                // Naga IR.
                //
                // [^yes]: ```hlsl
                // case 1:
                // case 2: do_stuff()
                // ```
                // [^no]: ```hlsl
                // case 1: do_this();
                // case 2: do_that();
                // ```
                if case.fall_through && !case.body.is_empty() {
                    let curr_len = i + 1;
                    let end_case_idx = curr_len
                        + cases
                            .iter()
                            .skip(curr_len)
                            .position(|case| !case.fall_through)
                            .unwrap();
                    let indent_level_3 = indent_level_2.next();
                    for case in &cases[i..=end_case_idx] {
                        writeln!(self.out, "{indent_level_2}{{")?;
                        let prev_len = self.named_expressions.len();
                        for sta in case.body.iter() {
                            self.write_stmt(module, sta, func_ctx, indent_level_3)?;
                        }
                        // Clear all named expressions that were previously inserted by the statements in the block
                        self.named_expressions.truncate(prev_len);
                        writeln!(self.out, "{indent_level_2}}}")?;
                    }

                    let last_case = &cases[end_case_idx];
                    if last_case.body.last().is_none_or(|s| !s.is_terminator()) {
                        writeln!(self.out, "{indent_level_2}break;")?;
                    }
                } else {
                    for sta in case.body.iter() {
                        self.write_stmt(module, sta, func_ctx, indent_level_2)?;
                    }
                    if !case.fall_through && case.body.last().is_none_or(|s| !s.is_terminator()) {
                        writeln!(self.out, "{indent_level_2}break;")?;
                    }
                }

                if write_block_braces {
                    writeln!(self.out, "{indent_level_1}}}")?;
                }
            }

            writeln!(self.out, "{level}}}")?;
        }

        // Handle any forwarded continue statements.
        use back::continue_forward::ExitControlFlow;
        let op = match self.continue_ctx.exit_switch() {
            ExitControlFlow::None => None,
            ExitControlFlow::Continue { variable } => Some(("continue", variable)),
            ExitControlFlow::Break { variable } => Some(("break", variable)),
        };
        if let Some((control_flow, variable)) = op {
            writeln!(self.out, "{level}if ({variable}) {{")?;
            writeln!(self.out, "{indent_level_1}{control_flow};")?;
            writeln!(self.out, "{level}}}")?;
        }

        Ok(())
    }

    fn write_index(
        &mut self,
        module: &Module,
        index: Index,
        func_ctx: &back::FunctionCtx<'_>,
    ) -> BackendResult {
        match index {
            Index::Static(index) => {
                write!(self.out, "{index}")?;
            }
            Index::Expression(index) => {
                self.write_expr(module, index, func_ctx)?;
            }
        }
        Ok(())
    }

    /// Helper method used to write statements
    ///
    /// # Notes
    /// Always adds a newline
    fn write_stmt(
        &mut self,
        module: &Module,
        stmt: &crate::Statement,
        func_ctx: &back::FunctionCtx<'_>,
        level: back::Level,
    ) -> BackendResult {
        use crate::Statement;

        match *stmt {
            Statement::Emit(ref range) => {
                for handle in range.clone() {
                    let ptr_class = func_ctx.resolve_type(handle, &module.types).pointer_space();
                    let expr_name = if ptr_class.is_some() {
                        // HLSL can't save a pointer-valued expression in a variable,
                        // but we shouldn't ever need to: they should never be named expressions,
                        // and none of the expression types flagged by bake_ref_count can be pointer-valued.
                        None
                    } else if let Some(name) = func_ctx.named_expressions.get(&handle) {
                        // Front end provides names for all variables at the start of writing.
                        // But we write them to step by step. We need to recache them
                        // Otherwise, we could accidentally write variable name instead of full expression.
                        // Also, we use sanitized names! It defense backend from generating variable with name from reserved keywords.
                        Some(self.namer.call(name))
                    } else if self.need_bake_expressions.contains(&handle) {
                        Some(Baked(handle).to_string())
                    } else {
                        None
                    };

                    if let Some(name) = expr_name {
                        write!(self.out, "{level}")?;
                        self.write_named_expr(module, handle, name, handle, func_ctx)?;
                    }
                }
            }
            // TODO: copy-paste from glsl-out
            Statement::Block(ref block) => {
                write!(self.out, "{level}")?;
                writeln!(self.out, "{{")?;
                for sta in block.iter() {
                    // Increase the indentation to help with readability
                    self.write_stmt(module, sta, func_ctx, level.next())?
                }
                writeln!(self.out, "{level}}}")?
            }
            // TODO: copy-paste from glsl-out
            Statement::If {
                condition,
                ref accept,
                ref reject,
            } => {
                write!(self.out, "{level}")?;
                write!(self.out, "if (")?;
                self.write_expr(module, condition, func_ctx)?;
                writeln!(self.out, ") {{")?;

                let l2 = level.next();
                for sta in accept {
                    // Increase indentation to help with readability
                    self.write_stmt(module, sta, func_ctx, l2)?;
                }

                // If there are no statements in the reject block we skip writing it
                // This is only for readability
                if !reject.is_empty() {
                    writeln!(self.out, "{level}}} else {{")?;

                    for sta in reject {
                        // Increase indentation to help with readability
                        self.write_stmt(module, sta, func_ctx, l2)?;
                    }
                }

                writeln!(self.out, "{level}}}")?
            }
            // TODO: copy-paste from glsl-out
            Statement::Kill => writeln!(self.out, "{level}discard;")?,
            Statement::Return { value: None } => {
                writeln!(self.out, "{level}return;")?;
            }
            Statement::Return { value: Some(expr) } => {
                let base_ty_res = &func_ctx.info[expr].ty;
                let mut resolved = base_ty_res.inner_with(&module.types);
                if let TypeInner::Pointer { base, space: _ } = *resolved {
                    resolved = &module.types[base].inner;
                }

                if let TypeInner::Struct { .. } = *resolved {
                    // We can safely unwrap here, since we now we working with struct
                    let ty = base_ty_res.handle().unwrap();
                    let struct_name = &self.names[&NameKey::Type(ty)];
                    let variable_name = self.namer.call(&struct_name.to_lowercase());
                    write!(self.out, "{level}const {struct_name} {variable_name} = ",)?;
                    self.write_expr(module, expr, func_ctx)?;
                    writeln!(self.out, ";")?;

                    // for entry point returns, we may need to reshuffle the outputs into a different struct
                    let ep_output = match func_ctx.ty {
                        back::FunctionType::Function(_) => None,
                        back::FunctionType::EntryPoint(index) => self
                            .entry_point_io
                            .get(&(index as usize))
                            .unwrap()
                            .output
                            .as_ref(),
                    };
                    let final_name = match ep_output {
                        Some(ep_output) => {
                            let final_name = self.namer.call(&variable_name);
                            write!(
                                self.out,
                                "{}const {} {} = {{ ",
                                level, ep_output.ty_name, final_name,
                            )?;
                            for (index, m) in ep_output.members.iter().enumerate() {
                                if index != 0 {
                                    write!(self.out, ", ")?;
                                }
                                let member_name = &self.names[&NameKey::StructMember(ty, m.index)];
                                write!(self.out, "{variable_name}.{member_name}")?;
                            }
                            writeln!(self.out, " }};")?;
                            final_name
                        }
                        None => variable_name,
                    };
                    writeln!(self.out, "{level}return {final_name};")?;
                } else {
                    write!(self.out, "{level}return ")?;
                    self.write_expr(module, expr, func_ctx)?;
                    writeln!(self.out, ";")?
                }
            }
            Statement::Store { pointer, value } => {
                let ty_inner = func_ctx.resolve_type(pointer, &module.types);
                if let Some(crate::AddressSpace::Storage { .. }) = ty_inner.pointer_space() {
                    let var_handle = self.fill_access_chain(module, pointer, func_ctx)?;
                    self.write_storage_store(
                        module,
                        var_handle,
                        StoreValue::Expression(value),
                        func_ctx,
                        level,
                        None,
                    )?;
                } else {
                    // We treat matrices of the form `matCx2` as a sequence of C `vec2`s.
                    // See the module-level block comment in mod.rs for details.
                    //
                    // We handle matrix Stores here directly (including sub accesses for Vectors and Scalars).
                    // Loads are handled by `Expression::AccessIndex` (since sub accesses work fine for Loads).
                    enum MatrixAccess {
                        Direct {
                            base: Handle<crate::Expression>,
                            index: u32,
                        },
                        Struct {
                            columns: crate::VectorSize,
                            base: Handle<crate::Expression>,
                        },
                    }

                    let get_members = |expr: Handle<crate::Expression>| {
                        let resolved = func_ctx.resolve_type(expr, &module.types);
                        match *resolved {
                            TypeInner::Pointer { base, .. } => match module.types[base].inner {
                                TypeInner::Struct { ref members, .. } => Some(members),
                                _ => None,
                            },
                            _ => None,
                        }
                    };

                    write!(self.out, "{level}")?;

                    let matrix_access_on_lhs =
                        find_matrix_in_access_chain(module, pointer, func_ctx).and_then(
                            |(matrix_expr, vector, scalar)| match (
                                func_ctx.resolve_type(matrix_expr, &module.types),
                                &func_ctx.expressions[matrix_expr],
                            ) {
                                (
                                    &TypeInner::Pointer { base: ty, .. },
                                    &crate::Expression::AccessIndex { base, index },
                                ) if matches!(
                                    module.types[ty].inner,
                                    TypeInner::Matrix {
                                        rows: crate::VectorSize::Bi,
                                        ..
                                    }
                                ) && get_members(base)
                                    .map(|members| members[index as usize].binding.is_none())
                                    == Some(true) =>
                                {
                                    Some((MatrixAccess::Direct { base, index }, vector, scalar))
                                }
                                _ => {
                                    if let Some(MatrixType {
                                        columns,
                                        rows: crate::VectorSize::Bi,
                                        width: 4,
                                    }) = get_inner_matrix_of_struct_array_member(
                                        module,
                                        matrix_expr,
                                        func_ctx,
                                        true,
                                    ) {
                                        Some((
                                            MatrixAccess::Struct {
                                                columns,
                                                base: matrix_expr,
                                            },
                                            vector,
                                            scalar,
                                        ))
                                    } else {
                                        None
                                    }
                                }
                            },
                        );

                    match matrix_access_on_lhs {
                        Some((MatrixAccess::Direct { index, base }, vector, scalar)) => {
                            let base_ty_res = &func_ctx.info[base].ty;
                            let resolved = base_ty_res.inner_with(&module.types);
                            let ty = match *resolved {
                                TypeInner::Pointer { base, .. } => base,
                                _ => base_ty_res.handle().unwrap(),
                            };

                            if let Some(Index::Static(vec_index)) = vector {
                                self.write_expr(module, base, func_ctx)?;
                                write!(
                                    self.out,
                                    ".{}_{}",
                                    &self.names[&NameKey::StructMember(ty, index)],
                                    vec_index
                                )?;

                                if let Some(scalar_index) = scalar {
                                    write!(self.out, "[")?;
                                    self.write_index(module, scalar_index, func_ctx)?;
                                    write!(self.out, "]")?;
                                }

                                write!(self.out, " = ")?;
                                self.write_expr(module, value, func_ctx)?;
                                writeln!(self.out, ";")?;
                            } else {
                                let access = WrappedStructMatrixAccess { ty, index };
                                match (&vector, &scalar) {
                                    (&Some(_), &Some(_)) => {
                                        self.write_wrapped_struct_matrix_set_scalar_function_name(
                                            access,
                                        )?;
                                    }
                                    (&Some(_), &None) => {
                                        self.write_wrapped_struct_matrix_set_vec_function_name(
                                            access,
                                        )?;
                                    }
                                    (&None, _) => {
                                        self.write_wrapped_struct_matrix_set_function_name(access)?;
                                    }
                                }

                                write!(self.out, "(")?;
                                self.write_expr(module, base, func_ctx)?;
                                write!(self.out, ", ")?;
                                self.write_expr(module, value, func_ctx)?;

                                if let Some(Index::Expression(vec_index)) = vector {
                                    write!(self.out, ", ")?;
                                    self.write_expr(module, vec_index, func_ctx)?;

                                    if let Some(scalar_index) = scalar {
                                        write!(self.out, ", ")?;
                                        self.write_index(module, scalar_index, func_ctx)?;
                                    }
                                }
                                writeln!(self.out, ");")?;
                            }
                        }
                        Some((
                            MatrixAccess::Struct { columns, base },
                            Some(Index::Expression(vec_index)),
                            scalar,
                        )) => {
                            // We handle `Store`s to __matCx2 column vectors and scalar elements via
                            // the previously injected functions __set_col_of_matCx2 / __set_el_of_matCx2.

                            if scalar.is_some() {
                                write!(self.out, "__set_el_of_mat{}x2", columns as u8)?;
                            } else {
                                write!(self.out, "__set_col_of_mat{}x2", columns as u8)?;
                            }
                            write!(self.out, "(")?;
                            self.write_expr(module, base, func_ctx)?;
                            write!(self.out, ", ")?;
                            self.write_expr(module, vec_index, func_ctx)?;

                            if let Some(scalar_index) = scalar {
                                write!(self.out, ", ")?;
                                self.write_index(module, scalar_index, func_ctx)?;
                            }

                            write!(self.out, ", ")?;
                            self.write_expr(module, value, func_ctx)?;

                            writeln!(self.out, ");")?;
                        }
                        Some((MatrixAccess::Struct { .. }, Some(Index::Static(_)), _))
                        | Some((MatrixAccess::Struct { .. }, None, _))
                        | None => {
                            self.write_expr(module, pointer, func_ctx)?;
                            write!(self.out, " = ")?;

                            // We cast the RHS of this store in cases where the LHS
                            // is a struct member with type:
                            //  - matCx2 or
                            //  - a (possibly nested) array of matCx2's
                            if let Some(MatrixType {
                                columns,
                                rows: crate::VectorSize::Bi,
                                width: 4,
                            }) = get_inner_matrix_of_struct_array_member(
                                module, pointer, func_ctx, false,
                            ) {
                                let mut resolved = func_ctx.resolve_type(pointer, &module.types);
                                if let TypeInner::Pointer { base, .. } = *resolved {
                                    resolved = &module.types[base].inner;
                                }

                                write!(self.out, "(__mat{}x2", columns as u8)?;
                                if let TypeInner::Array { base, size, .. } = *resolved {
                                    self.write_array_size(module, base, size)?;
                                }
                                write!(self.out, ")")?;
                            }

                            self.write_expr(module, value, func_ctx)?;
                            writeln!(self.out, ";")?
                        }
                    }
                }
            }
            Statement::Loop {
                ref body,
                ref continuing,
                break_if,
            } => {
                let force_loop_bound_statements = self.gen_force_bounded_loop_statements(level);
                let gate_name = (!continuing.is_empty() || break_if.is_some())
                    .then(|| self.namer.call("loop_init"));

                if let Some((ref decl, _)) = force_loop_bound_statements {
                    writeln!(self.out, "{decl}")?;
                }
                if let Some(ref gate_name) = gate_name {
                    writeln!(self.out, "{level}bool {gate_name} = true;")?;
                }

                self.continue_ctx.enter_loop();
                writeln!(self.out, "{level}while(true) {{")?;
                if let Some((_, ref break_and_inc)) = force_loop_bound_statements {
                    writeln!(self.out, "{break_and_inc}")?;
                }
                let l2 = level.next();
                if let Some(gate_name) = gate_name {
                    writeln!(self.out, "{l2}if (!{gate_name}) {{")?;
                    let l3 = l2.next();
                    for sta in continuing.iter() {
                        self.write_stmt(module, sta, func_ctx, l3)?;
                    }
                    if let Some(condition) = break_if {
                        write!(self.out, "{l3}if (")?;
                        self.write_expr(module, condition, func_ctx)?;
                        writeln!(self.out, ") {{")?;
                        writeln!(self.out, "{}break;", l3.next())?;
                        writeln!(self.out, "{l3}}}")?;
                    }
                    writeln!(self.out, "{l2}}}")?;
                    writeln!(self.out, "{l2}{gate_name} = false;")?;
                }

                for sta in body.iter() {
                    self.write_stmt(module, sta, func_ctx, l2)?;
                }

                writeln!(self.out, "{level}}}")?;
                self.continue_ctx.exit_loop();
            }
            Statement::Break => writeln!(self.out, "{level}break;")?,
            Statement::Continue => {
                if let Some(variable) = self.continue_ctx.continue_encountered() {
                    writeln!(self.out, "{level}{variable} = true;")?;
                    writeln!(self.out, "{level}break;")?
                } else {
                    writeln!(self.out, "{level}continue;")?
                }
            }
            Statement::ControlBarrier(barrier) => {
                self.write_control_barrier(barrier, level)?;
            }
            Statement::MemoryBarrier(barrier) => {
                self.write_memory_barrier(barrier, level)?;
            }
            Statement::ImageStore {
                image,
                coordinate,
                array_index,
                value,
            } => {
                write!(self.out, "{level}")?;
                self.write_expr(module, image, func_ctx)?;

                write!(self.out, "[")?;
                if let Some(index) = array_index {
                    // Array index accepted only for texture_storage_2d_array, so we can safety use int3(coordinate, array_index) here
                    write!(self.out, "int3(")?;
                    self.write_expr(module, coordinate, func_ctx)?;
                    write!(self.out, ", ")?;
                    self.write_expr(module, index, func_ctx)?;
                    write!(self.out, ")")?;
                } else {
                    self.write_expr(module, coordinate, func_ctx)?;
                }
                write!(self.out, "]")?;

                write!(self.out, " = ")?;
                self.write_expr(module, value, func_ctx)?;
                writeln!(self.out, ";")?;
            }
            Statement::Call {
                function,
                ref arguments,
                result,
            } => {
                write!(self.out, "{level}")?;
                if let Some(expr) = result {
                    write!(self.out, "const ")?;
                    let name = Baked(expr).to_string();
                    let expr_ty = &func_ctx.info[expr].ty;
                    let ty_inner = match *expr_ty {
                        proc::TypeResolution::Handle(handle) => {
                            self.write_type(module, handle)?;
                            &module.types[handle].inner
                        }
                        proc::TypeResolution::Value(ref value) => {
                            self.write_value_type(module, value)?;
                            value
                        }
                    };
                    write!(self.out, " {name}")?;
                    if let TypeInner::Array { base, size, .. } = *ty_inner {
                        self.write_array_size(module, base, size)?;
                    }
                    write!(self.out, " = ")?;
                    self.named_expressions.insert(expr, name);
                }
                let func_name = &self.names[&NameKey::Function(function)];
                write!(self.out, "{func_name}(")?;
                for (index, argument) in arguments.iter().enumerate() {
                    if index != 0 {
                        write!(self.out, ", ")?;
                    }
                    self.write_expr(module, *argument, func_ctx)?;
                }
                writeln!(self.out, ");")?
            }
            Statement::Atomic {
                pointer,
                ref fun,
                value,
                result,
            } => {
                write!(self.out, "{level}")?;
                let res_var_info = if let Some(res_handle) = result {
                    let name = Baked(res_handle).to_string();
                    match func_ctx.info[res_handle].ty {
                        proc::TypeResolution::Handle(handle) => self.write_type(module, handle)?,
                        proc::TypeResolution::Value(ref value) => {
                            self.write_value_type(module, value)?
                        }
                    };
                    write!(self.out, " {name}; ")?;
                    self.named_expressions.insert(res_handle, name.clone());
                    Some((res_handle, name))
                } else {
                    None
                };
                let pointer_space = func_ctx
                    .resolve_type(pointer, &module.types)
                    .pointer_space()
                    .unwrap();
                let fun_str = fun.to_hlsl_suffix();
                let compare_expr = match *fun {
                    crate::AtomicFunction::Exchange { compare: Some(cmp) } => Some(cmp),
                    _ => None,
                };
                match pointer_space {
                    crate::AddressSpace::WorkGroup => {
                        write!(self.out, "Interlocked{fun_str}(")?;
                        self.write_expr(module, pointer, func_ctx)?;
                        self.emit_hlsl_atomic_tail(
                            module,
                            func_ctx,
                            fun,
                            compare_expr,
                            value,
                            &res_var_info,
                        )?;
                    }
                    crate::AddressSpace::Storage { .. } => {
                        let var_handle = self.fill_access_chain(module, pointer, func_ctx)?;
                        let var_name = &self.names[&NameKey::GlobalVariable(var_handle)];
                        let width = match func_ctx.resolve_type(value, &module.types) {
                            &TypeInner::Scalar(Scalar { width: 8, .. }) => "64",
                            _ => "",
                        };
                        write!(self.out, "{var_name}.Interlocked{fun_str}{width}(")?;
                        let chain = mem::take(&mut self.temp_access_chain);
                        self.write_storage_address(module, &chain, func_ctx)?;
                        self.temp_access_chain = chain;
                        self.emit_hlsl_atomic_tail(
                            module,
                            func_ctx,
                            fun,
                            compare_expr,
                            value,
                            &res_var_info,
                        )?;
                    }
                    ref other => {
                        return Err(Error::Custom(format!(
                            "invalid address space {other:?} for atomic statement"
                        )))
                    }
                }
                if let Some(cmp) = compare_expr {
                    if let Some(&(_res_handle, ref res_name)) = res_var_info.as_ref() {
                        write!(
                            self.out,
                            "{level}{res_name}.exchanged = ({res_name}.old_value == "
                        )?;
                        self.write_expr(module, cmp, func_ctx)?;
                        writeln!(self.out, ");")?;
                    }
                }
            }
            Statement::ImageAtomic {
                image,
                coordinate,
                array_index,
                fun,
                value,
            } => {
                write!(self.out, "{level}")?;

                let fun_str = fun.to_hlsl_suffix();
                write!(self.out, "Interlocked{fun_str}(")?;
                self.write_expr(module, image, func_ctx)?;
                write!(self.out, "[")?;
                self.write_texture_coordinates(
                    "int",
                    coordinate,
                    array_index,
                    None,
                    module,
                    func_ctx,
                )?;
                write!(self.out, "],")?;

                self.write_expr(module, value, func_ctx)?;
                writeln!(self.out, ");")?;
            }
            Statement::WorkGroupUniformLoad { pointer, result } => {
                self.write_control_barrier(crate::Barrier::WORK_GROUP, level)?;
                write!(self.out, "{level}")?;
                let name = Baked(result).to_string();
                self.write_named_expr(module, pointer, name, result, func_ctx)?;

                self.write_control_barrier(crate::Barrier::WORK_GROUP, level)?;
            }
            Statement::Switch {
                selector,
                ref cases,
            } => {
                self.write_switch(module, func_ctx, level, selector, cases)?;
            }
            Statement::RayQuery { query, ref fun } => {
                // There are three possibilities for a ptr to be:
                // 1. A variable
                // 2. A function argument
                // 3. part of a struct
                //
                // 2 and 3 are not possible, a ray query (in naga IR)
                // is not allowed to be passed into a function, and
                // all languages disallow it in a struct (you get fun results if
                // you try it :) ).
                //
                // Therefore, the ray query expression must be a variable.
                let crate::Expression::LocalVariable(query_var) = func_ctx.expressions[query]
                else {
                    unreachable!()
                };

                let tracker_expr_name = format!(
                    "{RAY_QUERY_TRACKER_VARIABLE_PREFIX}{}",
                    self.names[&func_ctx.name_key(query_var)]
                );

                match *fun {
                    RayQueryFunction::Initialize {
                        acceleration_structure,
                        descriptor,
                    } => {
                        self.write_initialize_function(
                            module,
                            level,
                            query,
                            acceleration_structure,
                            descriptor,
                            &tracker_expr_name,
                            func_ctx,
                        )?;
                    }
                    RayQueryFunction::Proceed { result } => {
                        self.write_proceed(
                            module,
                            level,
                            query,
                            result,
                            &tracker_expr_name,
                            func_ctx,
                        )?;
                    }
                    RayQueryFunction::GenerateIntersection { hit_t } => {
                        self.write_generate_intersection(
                            module,
                            level,
                            query,
                            hit_t,
                            &tracker_expr_name,
                            func_ctx,
                        )?;
                    }
                    RayQueryFunction::ConfirmIntersection => {
                        self.write_confirm_intersection(
                            module,
                            level,
                            query,
                            &tracker_expr_name,
                            func_ctx,
                        )?;
                    }
                    RayQueryFunction::Terminate => {
                        self.write_terminate(module, level, query, &tracker_expr_name, func_ctx)?;
                    }
                }
            }
            Statement::SubgroupBallot { result, predicate } => {
                write!(self.out, "{level}")?;
                let name = Baked(result).to_string();
                write!(self.out, "const uint4 {name} = ")?;
                self.named_expressions.insert(result, name);

                write!(self.out, "WaveActiveBallot(")?;
                match predicate {
                    Some(predicate) => self.write_expr(module, predicate, func_ctx)?,
                    None => write!(self.out, "true")?,
                }
                writeln!(self.out, ");")?;
            }
            Statement::SubgroupCollectiveOperation {
                op,
                collective_op,
                argument,
                result,
            } => {
                write!(self.out, "{level}")?;
                write!(self.out, "const ")?;
                let name = Baked(result).to_string();
                match func_ctx.info[result].ty {
                    proc::TypeResolution::Handle(handle) => self.write_type(module, handle)?,
                    proc::TypeResolution::Value(ref value) => {
                        self.write_value_type(module, value)?
                    }
                };
                write!(self.out, " {name} = ")?;
                self.named_expressions.insert(result, name);

                match (collective_op, op) {
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::All) => {
                        write!(self.out, "WaveActiveAllTrue(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Any) => {
                        write!(self.out, "WaveActiveAnyTrue(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Add) => {
                        write!(self.out, "WaveActiveSum(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Mul) => {
                        write!(self.out, "WaveActiveProduct(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Max) => {
                        write!(self.out, "WaveActiveMax(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Min) => {
                        write!(self.out, "WaveActiveMin(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::And) => {
                        write!(self.out, "WaveActiveBitAnd(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Or) => {
                        write!(self.out, "WaveActiveBitOr(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Xor) => {
                        write!(self.out, "WaveActiveBitXor(")?
                    }
                    (crate::CollectiveOperation::ExclusiveScan, crate::SubgroupOperation::Add) => {
                        write!(self.out, "WavePrefixSum(")?
                    }
                    (crate::CollectiveOperation::ExclusiveScan, crate::SubgroupOperation::Mul) => {
                        write!(self.out, "WavePrefixProduct(")?
                    }
                    (crate::CollectiveOperation::InclusiveScan, crate::SubgroupOperation::Add) => {
                        self.write_expr(module, argument, func_ctx)?;
                        write!(self.out, " + WavePrefixSum(")?;
                    }
                    (crate::CollectiveOperation::InclusiveScan, crate::SubgroupOperation::Mul) => {
                        self.write_expr(module, argument, func_ctx)?;
                        write!(self.out, " * WavePrefixProduct(")?;
                    }
                    _ => unimplemented!(),
                }
                self.write_expr(module, argument, func_ctx)?;
                writeln!(self.out, ");")?;
            }
            Statement::SubgroupGather {
                mode,
                argument,
                result,
            } => {
                write!(self.out, "{level}")?;
                write!(self.out, "const ")?;
                let name = Baked(result).to_string();
                match func_ctx.info[result].ty {
                    proc::TypeResolution::Handle(handle) => self.write_type(module, handle)?,
                    proc::TypeResolution::Value(ref value) => {
                        self.write_value_type(module, value)?
                    }
                };
                write!(self.out, " {name} = ")?;
                self.named_expressions.insert(result, name);
                match mode {
                    crate::GatherMode::BroadcastFirst => {
                        write!(self.out, "WaveReadLaneFirst(")?;
                        self.write_expr(module, argument, func_ctx)?;
                    }
                    crate::GatherMode::QuadBroadcast(index) => {
                        write!(self.out, "QuadReadLaneAt(")?;
                        self.write_expr(module, argument, func_ctx)?;
                        write!(self.out, ", ")?;
                        self.write_expr(module, index, func_ctx)?;
                    }
                    crate::GatherMode::QuadSwap(direction) => {
                        match direction {
                            crate::Direction::X => {
                                write!(self.out, "QuadReadAcrossX(")?;
                            }
                            crate::Direction::Y => {
                                write!(self.out, "QuadReadAcrossY(")?;
                            }
                            crate::Direction::Diagonal => {
                                write!(self.out, "QuadReadAcrossDiagonal(")?;
                            }
                        }
                        self.write_expr(module, argument, func_ctx)?;
                    }
                    _ => {
                        write!(self.out, "WaveReadLaneAt(")?;
                        self.write_expr(module, argument, func_ctx)?;
                        write!(self.out, ", ")?;
                        match mode {
                            crate::GatherMode::BroadcastFirst => unreachable!(),
                            crate::GatherMode::Broadcast(index)
                            | crate::GatherMode::Shuffle(index) => {
                                self.write_expr(module, index, func_ctx)?;
                            }
                            crate::GatherMode::ShuffleDown(index) => {
                                write!(self.out, "WaveGetLaneIndex() + ")?;
                                self.write_expr(module, index, func_ctx)?;
                            }
                            crate::GatherMode::ShuffleUp(index) => {
                                write!(self.out, "WaveGetLaneIndex() - ")?;
                                self.write_expr(module, index, func_ctx)?;
                            }
                            crate::GatherMode::ShuffleXor(index) => {
                                write!(self.out, "WaveGetLaneIndex() ^ ")?;
                                self.write_expr(module, index, func_ctx)?;
                            }
                            crate::GatherMode::QuadBroadcast(_) => unreachable!(),
                            crate::GatherMode::QuadSwap(_) => unreachable!(),
                        }
                    }
                }
                writeln!(self.out, ");")?;
            }
            Statement::CooperativeStore { .. } => unimplemented!(),
            Statement::RayPipelineFunction(_) => unreachable!(),
        }

        Ok(())
    }

    fn write_const_expression(
        &mut self,
        module: &Module,
        expr: Handle<crate::Expression>,
        arena: &crate::Arena<crate::Expression>,
    ) -> BackendResult {
        self.write_possibly_const_expression(module, expr, arena, |writer, expr| {
            writer.write_const_expression(module, expr, arena)
        })
    }

    pub(super) fn write_literal(&mut self, literal: crate::Literal) -> BackendResult {
        match literal {
            crate::Literal::F64(value) => write!(self.out, "{value:?}L")?,
            crate::Literal::F32(value) => write!(self.out, "{value:?}")?,
            crate::Literal::F16(value) => write!(self.out, "{value:?}h")?,
            crate::Literal::U32(value) => write!(self.out, "{value}u")?,
            // `-2147483648` is parsed by some compilers as unary negation of
            // positive 2147483648, which is too large for an int, causing
            // issues for some compilers. Neither DXC nor FXC appear to have
            // this problem, but this is not specified and could change. We
            // therefore use `-2147483647 - 1` as a precaution.
            crate::Literal::I32(value) if value == i32::MIN => {
                write!(self.out, "int({} - 1)", value + 1)?
            }
            // HLSL has no suffix for explicit i32 literals, but not using any suffix
            // makes the type ambiguous which prevents overload resolution from
            // working. So we explicitly use the int() constructor syntax.
            crate::Literal::I32(value) => write!(self.out, "int({value})")?,
            crate::Literal::U64(value) => write!(self.out, "{value}uL")?,
            // I64 version of the minimum I32 value issue described above.
            crate::Literal::I64(value) if value == i64::MIN => {
                write!(self.out, "({}L - 1L)", value + 1)?;
            }
            crate::Literal::I64(value) => write!(self.out, "{value}L")?,
            crate::Literal::Bool(value) => write!(self.out, "{value}")?,
            crate::Literal::AbstractInt(_) | crate::Literal::AbstractFloat(_) => {
                return Err(Error::Custom(
                    "Abstract types should not appear in IR presented to backends".into(),
                ));
            }
        }
        Ok(())
    }

    fn write_possibly_const_expression<E>(
        &mut self,
        module: &Module,
        expr: Handle<crate::Expression>,
        expressions: &crate::Arena<crate::Expression>,
        write_expression: E,
    ) -> BackendResult
    where
        E: Fn(&mut Self, Handle<crate::Expression>) -> BackendResult,
    {
        use crate::Expression;

        match expressions[expr] {
            Expression::Literal(literal) => {
                self.write_literal(literal)?;
            }
            Expression::Constant(handle) => {
                let constant = &module.constants[handle];
                if constant.name.is_some() {
                    write!(self.out, "{}", self.names[&NameKey::Constant(handle)])?;
                } else {
                    self.write_const_expression(module, constant.init, &module.global_expressions)?;
                }
            }
            Expression::ZeroValue(ty) => {
                self.write_wrapped_zero_value_function_name(module, WrappedZeroValue { ty })?;
                write!(self.out, "()")?;
            }
            Expression::Compose { ty, ref components } => {
                match module.types[ty].inner {
                    TypeInner::Struct { .. } | TypeInner::Array { .. } => {
                        self.write_wrapped_constructor_function_name(
                            module,
                            WrappedConstructor { ty },
                        )?;
                    }
                    _ => {
                        self.write_type(module, ty)?;
                    }
                };
                write!(self.out, "(")?;
                for (index, component) in components.iter().enumerate() {
                    if index != 0 {
                        write!(self.out, ", ")?;
                    }
                    write_expression(self, *component)?;
                }
                write!(self.out, ")")?;
            }
            Expression::Splat { size, value } => {
                // hlsl is not supported one value constructor
                // if we write, for example, int4(0), dxc returns error:
                // error: too few elements in vector initialization (expected 4 elements, have 1)
                let number_of_components = match size {
                    crate::VectorSize::Bi => "xx",
                    crate::VectorSize::Tri => "xxx",
                    crate::VectorSize::Quad => "xxxx",
                };
                write!(self.out, "(")?;
                write_expression(self, value)?;
                write!(self.out, ").{number_of_components}")?
            }
            _ => {
                return Err(Error::Override);
            }
        }

        Ok(())
    }

    /// Helper method to write expressions
    ///
    /// # Notes
    /// Doesn't add any newlines or leading/trailing spaces
    pub(super) fn write_expr(
        &mut self,
        module: &Module,
        expr: Handle<crate::Expression>,
        func_ctx: &back::FunctionCtx<'_>,
    ) -> BackendResult {
        use crate::Expression;

        // Handle the special semantics of vertex_index/instance_index
        let ff_input = if self.options.special_constants_binding.is_some() {
            func_ctx.is_fixed_function_input(expr, module)
        } else {
            None
        };
        let closing_bracket = match ff_input {
            Some(crate::BuiltIn::VertexIndex) => {
                write!(self.out, "({SPECIAL_CBUF_VAR}.{SPECIAL_FIRST_VERTEX} + ")?;
                ")"
            }
            Some(crate::BuiltIn::InstanceIndex) => {
                write!(self.out, "({SPECIAL_CBUF_VAR}.{SPECIAL_FIRST_INSTANCE} + ",)?;
                ")"
            }
            Some(crate::BuiltIn::NumWorkGroups) => {
                // Note: despite their names (`FIRST_VERTEX` and `FIRST_INSTANCE`),
                // in compute shaders the special constants contain the number
                // of workgroups, which we are using here.
                write!(
                    self.out,
                    "uint3({SPECIAL_CBUF_VAR}.{SPECIAL_FIRST_VERTEX}, {SPECIAL_CBUF_VAR}.{SPECIAL_FIRST_INSTANCE}, {SPECIAL_CBUF_VAR}.{SPECIAL_OTHER})",
                )?;
                return Ok(());
            }
            _ => "",
        };

        if let Some(name) = self.named_expressions.get(&expr) {
            write!(self.out, "{name}{closing_bracket}")?;
            return Ok(());
        }

        let expression = &func_ctx.expressions[expr];

        match *expression {
            Expression::Literal(_)
            | Expression::Constant(_)
            | Expression::ZeroValue(_)
            | Expression::Compose { .. }
            | Expression::Splat { .. } => {
                self.write_possibly_const_expression(
                    module,
                    expr,
                    func_ctx.expressions,
                    |writer, expr| writer.write_expr(module, expr, func_ctx),
                )?;
            }
            Expression::Override(_) => return Err(Error::Override),
            // Avoid undefined behaviour for addition, subtraction, and
            // multiplication of signed integers by casting operands to
            // unsigned, performing the operation, then casting the result back
            // to signed.
            // TODO(#7109): This relies on the asint()/asuint() functions which only work
            // for 32-bit types, so we must find another solution for different bit widths.
            Expression::Binary {
                op:
                    op @ crate::BinaryOperator::Add
                    | op @ crate::BinaryOperator::Subtract
                    | op @ crate::BinaryOperator::Multiply,
                left,
                right,
            } if matches!(
                func_ctx.resolve_type(expr, &module.types).scalar(),
                Some(Scalar::I32)
            ) =>
            {
                write!(self.out, "asint(asuint(",)?;
                self.write_expr(module, left, func_ctx)?;
                write!(self.out, ") {} asuint(", back::binary_operation_str(op))?;
                self.write_expr(module, right, func_ctx)?;
                write!(self.out, "))")?;
            }
            // All of the multiplication can be expressed as `mul`,
            // except vector * vector, which needs to use the "*" operator.
            Expression::Binary {
                op: crate::BinaryOperator::Multiply,
                left,
                right,
            } if func_ctx.resolve_type(left, &module.types).is_matrix()
                || func_ctx.resolve_type(right, &module.types).is_matrix() =>
            {
                // We intentionally flip the order of multiplication as our matrices are implicitly transposed.
                write!(self.out, "mul(")?;
                self.write_expr(module, right, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, left, func_ctx)?;
                write!(self.out, ")")?;
            }

            // WGSL says that floating-point division by zero should return
            // infinity. Microsoft's Direct3D 11 functional specification
            // (https://microsoft.github.io/DirectX-Specs/d3d/archive/D3D11_3_FunctionalSpec.htm)
            // says:
            //
            //     Divide by 0 produces +/- INF, except 0/0 which results in NaN.
            //
            // which is what we want. The DXIL specification for the FDiv
            // instruction corroborates this:
            //
            // https://github.com/microsoft/DirectXShaderCompiler/blob/main/docs/DXIL.rst#fdiv
            Expression::Binary {
                op: crate::BinaryOperator::Divide,
                left,
                right,
            } if matches!(
                func_ctx.resolve_type(expr, &module.types).scalar_kind(),
                Some(ScalarKind::Sint | ScalarKind::Uint)
            ) =>
            {
                write!(self.out, "{DIV_FUNCTION}(")?;
                self.write_expr(module, left, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, right, func_ctx)?;
                write!(self.out, ")")?;
            }

            Expression::Binary {
                op: crate::BinaryOperator::Modulo,
                left,
                right,
            } if matches!(
                func_ctx.resolve_type(expr, &module.types).scalar_kind(),
                Some(ScalarKind::Sint | ScalarKind::Uint | ScalarKind::Float)
            ) =>
            {
                write!(self.out, "{MOD_FUNCTION}(")?;
                self.write_expr(module, left, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, right, func_ctx)?;
                write!(self.out, ")")?;
            }

            Expression::Binary { op, left, right } => {
                write!(self.out, "(")?;
                self.write_expr(module, left, func_ctx)?;
                write!(self.out, " {} ", back::binary_operation_str(op))?;
                self.write_expr(module, right, func_ctx)?;
                write!(self.out, ")")?;
            }
            Expression::Access { base, index } => {
                if let Some(crate::AddressSpace::Storage { .. }) =
                    func_ctx.resolve_type(expr, &module.types).pointer_space()
                {
                    // do nothing, the chain is written on `Load`/`Store`
                } else {
                    // We use the function __get_col_of_matCx2 here in cases
                    // where `base`s type resolves to a matCx2 and is part of a
                    // struct member with type of (possibly nested) array of matCx2's.
                    //
                    // Note that this only works for `Load`s and we handle
                    // `Store`s differently in `Statement::Store`.
                    if let Some(MatrixType {
                        columns,
                        rows: crate::VectorSize::Bi,
                        width: 4,
                    }) = get_inner_matrix_of_struct_array_member(module, base, func_ctx, true)
                        .or_else(|| get_global_uniform_matrix(module, base, func_ctx))
                    {
                        write!(self.out, "__get_col_of_mat{}x2(", columns as u8)?;
                        self.write_expr(module, base, func_ctx)?;
                        write!(self.out, ", ")?;
                        self.write_expr(module, index, func_ctx)?;
                        write!(self.out, ")")?;
                        return Ok(());
                    }

                    let resolved = func_ctx.resolve_type(base, &module.types);

                    let (indexing_binding_array, non_uniform_qualifier) = match *resolved {
                        TypeInner::BindingArray { .. } => {
                            let uniformity = &func_ctx.info[index].uniformity;

                            (true, uniformity.non_uniform_result.is_some())
                        }
                        _ => (false, false),
                    };

                    self.write_expr(module, base, func_ctx)?;

                    let array_sampler_info = self.sampler_binding_array_info_from_expression(
                        module, func_ctx, base, resolved,
                    );

                    if let Some(ref info) = array_sampler_info {
                        write!(self.out, "{}[", info.sampler_heap_name)?;
                    } else {
                        write!(self.out, "[")?;
                    }

                    let needs_bound_check = self.options.restrict_indexing
                        && !indexing_binding_array
                        && match resolved.pointer_space() {
                            Some(
                                crate::AddressSpace::Function
                                | crate::AddressSpace::Private
                                | crate::AddressSpace::WorkGroup
                                | crate::AddressSpace::Immediate
                                | crate::AddressSpace::TaskPayload
                                | crate::AddressSpace::RayPayload
                                | crate::AddressSpace::IncomingRayPayload,
                            )
                            | None => true,
                            Some(crate::AddressSpace::Uniform) => {
                                // check if BindTarget.restrict_indexing is set, this is used for dynamic buffers
                                let var_handle = self.fill_access_chain(module, base, func_ctx)?;
                                let bind_target = self
                                    .options
                                    .resolve_resource_binding(
                                        module.global_variables[var_handle]
                                            .binding
                                            .as_ref()
                                            .unwrap(),
                                    )
                                    .unwrap();
                                bind_target.restrict_indexing
                            }
                            Some(
                                crate::AddressSpace::Handle | crate::AddressSpace::Storage { .. },
                            ) => unreachable!(),
                        };
                    // Decide whether this index needs to be clamped to fall within range.
                    let restriction_needed = if needs_bound_check {
                        index::access_needs_check(
                            base,
                            index::GuardedIndex::Expression(index),
                            module,
                            func_ctx.expressions,
                            func_ctx.info,
                        )
                    } else {
                        None
                    };
                    if let Some(limit) = restriction_needed {
                        write!(self.out, "min(uint(")?;
                        self.write_expr(module, index, func_ctx)?;
                        write!(self.out, "), ")?;
                        match limit {
                            index::IndexableLength::Known(limit) => {
                                write!(self.out, "{}u", limit - 1)?;
                            }
                            index::IndexableLength::Dynamic => unreachable!(),
                        }
                        write!(self.out, ")")?;
                    } else {
                        if non_uniform_qualifier {
                            write!(self.out, "NonUniformResourceIndex(")?;
                        }
                        if let Some(ref info) = array_sampler_info {
                            write!(
                                self.out,
                                "{}[{} + ",
                                info.sampler_index_buffer_name, info.binding_array_base_index_name,
                            )?;
                        }
                        self.write_expr(module, index, func_ctx)?;
                        if array_sampler_info.is_some() {
                            write!(self.out, "]")?;
                        }
                        if non_uniform_qualifier {
                            write!(self.out, ")")?;
                        }
                    }

                    write!(self.out, "]")?;
                }
            }
            Expression::AccessIndex { base, index } => {
                if let Some(crate::AddressSpace::Storage { .. }) =
                    func_ctx.resolve_type(expr, &module.types).pointer_space()
                {
                    // do nothing, the chain is written on `Load`/`Store`
                } else {
                    // See if we need to write the matrix column access in a
                    // special way since the type of `base` is our special
                    // __matCx2 struct.
                    if let Some(MatrixType {
                        rows: crate::VectorSize::Bi,
                        width: 4,
                        ..
                    }) = get_inner_matrix_of_struct_array_member(module, base, func_ctx, true)
                        .or_else(|| get_global_uniform_matrix(module, base, func_ctx))
                    {
                        self.write_expr(module, base, func_ctx)?;
                        write!(self.out, "._{index}")?;
                        return Ok(());
                    }

                    let base_ty_res = &func_ctx.info[base].ty;
                    let mut resolved = base_ty_res.inner_with(&module.types);
                    let base_ty_handle = match *resolved {
                        TypeInner::Pointer { base, .. } => {
                            resolved = &module.types[base].inner;
                            Some(base)
                        }
                        _ => base_ty_res.handle(),
                    };

                    // We treat matrices of the form `matCx2` as a sequence of C `vec2`s.
                    // See the module-level block comment in mod.rs for details.
                    //
                    // We handle matrix reconstruction here for Loads.
                    // Stores are handled directly by `Statement::Store`.
                    if let TypeInner::Struct { ref members, .. } = *resolved {
                        let member = &members[index as usize];

                        match module.types[member.ty].inner {
                            TypeInner::Matrix {
                                rows: crate::VectorSize::Bi,
                                ..
                            } if member.binding.is_none() => {
                                let ty = base_ty_handle.unwrap();
                                self.write_wrapped_struct_matrix_get_function_name(
                                    WrappedStructMatrixAccess { ty, index },
                                )?;
                                write!(self.out, "(")?;
                                self.write_expr(module, base, func_ctx)?;
                                write!(self.out, ")")?;
                                return Ok(());
                            }
                            _ => {}
                        }
                    }

                    let array_sampler_info = self.sampler_binding_array_info_from_expression(
                        module, func_ctx, base, resolved,
                    );

                    if let Some(ref info) = array_sampler_info {
                        write!(
                            self.out,
                            "{}[{}",
                            info.sampler_heap_name, info.sampler_index_buffer_name
                        )?;
                    }

                    self.write_expr(module, base, func_ctx)?;

                    match *resolved {
                        // We specifically lift the ValuePointer to this case. While `[0]` is valid
                        // HLSL for any vector behind a value pointer, FXC completely miscompiles
                        // it and generates completely nonsensical DXBC.
                        //
                        // See https://github.com/gfx-rs/naga/issues/2095 for more details.
                        TypeInner::Vector { .. } | TypeInner::ValuePointer { .. } => {
                            // Write vector access as a swizzle
                            write!(self.out, ".{}", back::COMPONENTS[index as usize])?
                        }
                        TypeInner::Matrix { .. }
                        | TypeInner::Array { .. }
                        | TypeInner::BindingArray { .. } => {
                            if let Some(ref info) = array_sampler_info {
                                write!(
                                    self.out,
                                    "[{} + {index}]",
                                    info.binding_array_base_index_name
                                )?;
                            } else {
                                write!(self.out, "[{index}]")?;
                            }
                        }
                        TypeInner::Struct { .. } => {
                            // This will never panic in case the type is a `Struct`, this is not true
                            // for other types so we can only check while inside this match arm
                            let ty = base_ty_handle.unwrap();

                            write!(
                                self.out,
                                ".{}",
                                &self.names[&NameKey::StructMember(ty, index)]
                            )?
                        }
                        ref other => return Err(Error::Custom(format!("Cannot index {other:?}"))),
                    }

                    if array_sampler_info.is_some() {
                        write!(self.out, "]")?;
                    }
                }
            }
            Expression::FunctionArgument(pos) => {
                let ty = func_ctx.resolve_type(expr, &module.types);

                // We know that any external texture function argument has been expanded into
                // separate consecutive arguments for each plane and the parameters buffer. And we
                // also know that external textures can only ever be used as an argument to another
                // function. Therefore we can simply emit each of the expanded arguments in a
                // consecutive comma-separated list.
                if let TypeInner::Image {
                    class: crate::ImageClass::External,
                    ..
                } = *ty
                {
                    let plane_names = [0, 1, 2].map(|i| {
                        &self.names[&func_ctx
                            .external_texture_argument_key(pos, ExternalTextureNameKey::Plane(i))]
                    });
                    let params_name = &self.names[&func_ctx
                        .external_texture_argument_key(pos, ExternalTextureNameKey::Params)];
                    write!(
                        self.out,
                        "{}, {}, {}, {}",
                        plane_names[0], plane_names[1], plane_names[2], params_name
                    )?;
                } else {
                    let key = func_ctx.argument_key(pos);
                    let name = &self.names[&key];
                    write!(self.out, "{name}")?;
                }
            }
            Expression::ImageSample {
                coordinate,
                image,
                sampler,
                clamp_to_edge: true,
                gather: None,
                array_index: None,
                offset: None,
                level: crate::SampleLevel::Zero,
                depth_ref: None,
            } => {
                write!(self.out, "{IMAGE_SAMPLE_BASE_CLAMP_TO_EDGE_FUNCTION}(")?;
                self.write_expr(module, image, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, sampler, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, coordinate, func_ctx)?;
                write!(self.out, ")")?;
            }
            Expression::ImageSample {
                image,
                sampler,
                gather,
                coordinate,
                array_index,
                offset,
                level,
                depth_ref,
                clamp_to_edge,
            } => {
                if clamp_to_edge {
                    return Err(Error::Custom(
                        "ImageSample::clamp_to_edge should have been validated out".to_string(),
                    ));
                }

                use crate::SampleLevel as Sl;
                const COMPONENTS: [&str; 4] = ["", "Green", "Blue", "Alpha"];

                let (base_str, component_str) = match gather {
                    Some(component) => ("Gather", COMPONENTS[component as usize]),
                    None => ("Sample", ""),
                };
                let cmp_str = match depth_ref {
                    Some(_) => "Cmp",
                    None => "",
                };
                let level_str = match level {
                    Sl::Zero if gather.is_none() => "LevelZero",
                    Sl::Auto | Sl::Zero => "",
                    Sl::Exact(_) => "Level",
                    Sl::Bias(_) => "Bias",
                    Sl::Gradient { .. } => "Grad",
                };

                self.write_expr(module, image, func_ctx)?;
                write!(self.out, ".{base_str}{cmp_str}{component_str}{level_str}(")?;
                self.write_expr(module, sampler, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_texture_coordinates(
                    "float",
                    coordinate,
                    array_index,
                    None,
                    module,
                    func_ctx,
                )?;

                if let Some(depth_ref) = depth_ref {
                    write!(self.out, ", ")?;
                    self.write_expr(module, depth_ref, func_ctx)?;
                }

                match level {
                    Sl::Auto | Sl::Zero => {}
                    Sl::Exact(expr) => {
                        write!(self.out, ", ")?;
                        self.write_expr(module, expr, func_ctx)?;
                    }
                    Sl::Bias(expr) => {
                        write!(self.out, ", ")?;
                        self.write_expr(module, expr, func_ctx)?;
                    }
                    Sl::Gradient { x, y } => {
                        write!(self.out, ", ")?;
                        self.write_expr(module, x, func_ctx)?;
                        write!(self.out, ", ")?;
                        self.write_expr(module, y, func_ctx)?;
                    }
                }

                if let Some(offset) = offset {
                    write!(self.out, ", ")?;
                    write!(self.out, "int2(")?; // work around https://github.com/microsoft/DirectXShaderCompiler/issues/5082#issuecomment-1540147807
                    self.write_const_expression(module, offset, func_ctx.expressions)?;
                    write!(self.out, ")")?;
                }

                write!(self.out, ")")?;
            }
            Expression::ImageQuery { image, query } => {
                // use wrapped image query function
                if let TypeInner::Image {
                    dim,
                    arrayed,
                    class,
                } = *func_ctx.resolve_type(image, &module.types)
                {
                    let wrapped_image_query = WrappedImageQuery {
                        dim,
                        arrayed,
                        class,
                        query: query.into(),
                    };

                    self.write_wrapped_image_query_function_name(wrapped_image_query)?;
                    write!(self.out, "(")?;
                    // Image always first param
                    self.write_expr(module, image, func_ctx)?;
                    if let crate::ImageQuery::Size { level: Some(level) } = query {
                        write!(self.out, ", ")?;
                        self.write_expr(module, level, func_ctx)?;
                    }
                    write!(self.out, ")")?;
                }
            }
            Expression::ImageLoad {
                image,
                coordinate,
                array_index,
                sample,
                level,
            } => self.write_image_load(
                &module,
                expr,
                func_ctx,
                image,
                coordinate,
                array_index,
                sample,
                level,
            )?,
            Expression::GlobalVariable(handle) => {
                let global_variable = &module.global_variables[handle];
                let ty = &module.types[global_variable.ty].inner;

                // In the case of binding arrays of samplers, we need to not write anything
                // as the we are in the wrong position to fully write the expression.
                //
                // The entire writing is done by AccessIndex.
                let is_binding_array_of_samplers = match *ty {
                    TypeInner::BindingArray { base, .. } => {
                        let base_ty = &module.types[base].inner;
                        matches!(*base_ty, TypeInner::Sampler { .. })
                    }
                    _ => false,
                };

                let is_storage_space =
                    matches!(global_variable.space, crate::AddressSpace::Storage { .. });

                // Our external texture global variable has been expanded into multiple
                // global variables, one for each plane and the parameters buffer.
                // External textures can only ever be used as arguments to a function
                // call, and we know that an external texture argument to any function
                // will have been expanded to separate consecutive arguments for each
                // plane and the parameters buffer. Therefore we can simply emit each of
                // the expanded global variables in a consecutive comma-separated list.
                if let TypeInner::Image {
                    class: crate::ImageClass::External,
                    ..
                } = *ty
                {
                    let plane_names = [0, 1, 2].map(|i| {
                        &self.names[&NameKey::ExternalTextureGlobalVariable(
                            handle,
                            ExternalTextureNameKey::Plane(i),
                        )]
                    });
                    let params_name = &self.names[&NameKey::ExternalTextureGlobalVariable(
                        handle,
                        ExternalTextureNameKey::Params,
                    )];
                    write!(
                        self.out,
                        "{}, {}, {}, {}",
                        plane_names[0], plane_names[1], plane_names[2], params_name
                    )?;
                } else if !is_binding_array_of_samplers && !is_storage_space {
                    let name = &self.names[&NameKey::GlobalVariable(handle)];
                    write!(self.out, "{name}")?;
                }
            }
            Expression::LocalVariable(handle) => {
                write!(self.out, "{}", self.names[&func_ctx.name_key(handle)])?
            }
            Expression::Load { pointer } => {
                match func_ctx
                    .resolve_type(pointer, &module.types)
                    .pointer_space()
                {
                    Some(crate::AddressSpace::Storage { .. }) => {
                        let var_handle = self.fill_access_chain(module, pointer, func_ctx)?;
                        let result_ty = func_ctx.info[expr].ty.clone();
                        self.write_storage_load(module, var_handle, result_ty, func_ctx)?;
                    }
                    _ => {
                        let mut close_paren = false;

                        // We cast the value loaded to a native HLSL floatCx2
                        // in cases where it is of type:
                        //  - __matCx2 or
                        //  - a (possibly nested) array of __matCx2's
                        if let Some(MatrixType {
                            rows: crate::VectorSize::Bi,
                            width: 4,
                            ..
                        }) = get_inner_matrix_of_struct_array_member(
                            module, pointer, func_ctx, false,
                        )
                        .or_else(|| get_inner_matrix_of_global_uniform(module, pointer, func_ctx))
                        {
                            let mut resolved = func_ctx.resolve_type(pointer, &module.types);
                            let ptr_tr = resolved.pointer_base_type();
                            if let Some(ptr_ty) =
                                ptr_tr.as_ref().map(|tr| tr.inner_with(&module.types))
                            {
                                resolved = ptr_ty;
                            }

                            write!(self.out, "((")?;
                            if let TypeInner::Array { base, size, .. } = *resolved {
                                self.write_type(module, base)?;
                                self.write_array_size(module, base, size)?;
                            } else {
                                self.write_value_type(module, resolved)?;
                            }
                            write!(self.out, ")")?;
                            close_paren = true;
                        }

                        self.write_expr(module, pointer, func_ctx)?;

                        if close_paren {
                            write!(self.out, ")")?;
                        }
                    }
                }
            }
            Expression::Unary { op, expr } => {
                // https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-operators#unary-operators
                let op_str = match op {
                    crate::UnaryOperator::Negate => {
                        match func_ctx.resolve_type(expr, &module.types).scalar() {
                            Some(Scalar::I32) => NEG_FUNCTION,
                            _ => "-",
                        }
                    }
                    crate::UnaryOperator::LogicalNot => "!",
                    crate::UnaryOperator::BitwiseNot => "~",
                };
                write!(self.out, "{op_str}(")?;
                self.write_expr(module, expr, func_ctx)?;
                write!(self.out, ")")?;
            }
            Expression::As {
                expr,
                kind,
                convert,
            } => {
                let inner = func_ctx.resolve_type(expr, &module.types);
                if inner.scalar_kind() == Some(ScalarKind::Float)
                    && (kind == ScalarKind::Sint || kind == ScalarKind::Uint)
                    && convert.is_some()
                {
                    // Use helper functions for float to int casts in order to
                    // avoid undefined behaviour when value is out of range for
                    // the target type.
                    let fun_name = match (kind, convert) {
                        (ScalarKind::Sint, Some(4)) => F2I32_FUNCTION,
                        (ScalarKind::Uint, Some(4)) => F2U32_FUNCTION,
                        (ScalarKind::Sint, Some(8)) => F2I64_FUNCTION,
                        (ScalarKind::Uint, Some(8)) => F2U64_FUNCTION,
                        _ => unreachable!(),
                    };
                    write!(self.out, "{fun_name}(")?;
                    self.write_expr(module, expr, func_ctx)?;
                    write!(self.out, ")")?;
                } else {
                    let close_paren = match convert {
                        Some(dst_width) => {
                            let scalar = Scalar {
                                kind,
                                width: dst_width,
                            };
                            match *inner {
                                TypeInner::Vector { size, .. } => {
                                    write!(
                                        self.out,
                                        "{}{}(",
                                        scalar.to_hlsl_str()?,
                                        common::vector_size_str(size)
                                    )?;
                                }
                                TypeInner::Scalar(_) => {
                                    write!(self.out, "{}(", scalar.to_hlsl_str()?,)?;
                                }
                                TypeInner::Matrix { columns, rows, .. } => {
                                    write!(
                                        self.out,
                                        "{}{}x{}(",
                                        scalar.to_hlsl_str()?,
                                        common::vector_size_str(columns),
                                        common::vector_size_str(rows)
                                    )?;
                                }
                                _ => {
                                    return Err(Error::Unimplemented(format!(
                                        "write_expr expression::as {inner:?}"
                                    )));
                                }
                            };
                            true
                        }
                        None => {
                            if inner.scalar_width() == Some(8) {
                                false
                            } else {
                                write!(self.out, "{}(", kind.to_hlsl_cast(),)?;
                                true
                            }
                        }
                    };
                    self.write_expr(module, expr, func_ctx)?;
                    if close_paren {
                        write!(self.out, ")")?;
                    }
                }
            }
            Expression::Math {
                fun,
                arg,
                arg1,
                arg2,
                arg3,
            } => {
                use crate::MathFunction as Mf;

                enum Function {
                    Asincosh { is_sin: bool },
                    Atanh,
                    Pack2x16float,
                    Pack2x16snorm,
                    Pack2x16unorm,
                    Pack4x8snorm,
                    Pack4x8unorm,
                    Pack4xI8,
                    Pack4xU8,
                    Pack4xI8Clamp,
                    Pack4xU8Clamp,
                    Unpack2x16float,
                    Unpack2x16snorm,
                    Unpack2x16unorm,
                    Unpack4x8snorm,
                    Unpack4x8unorm,
                    Unpack4xI8,
                    Unpack4xU8,
                    Dot4I8Packed,
                    Dot4U8Packed,
                    QuantizeToF16,
                    Regular(&'static str),
                    MissingIntOverload(&'static str),
                    MissingIntReturnType(&'static str),
                    CountTrailingZeros,
                    CountLeadingZeros,
                }

                let fun = match fun {
                    // comparison
                    Mf::Abs => match func_ctx.resolve_type(arg, &module.types).scalar() {
                        Some(Scalar::I32) => Function::Regular(ABS_FUNCTION),
                        _ => Function::Regular("abs"),
                    },
                    Mf::Min => Function::Regular("min"),
                    Mf::Max => Function::Regular("max"),
                    Mf::Clamp => Function::Regular("clamp"),
                    Mf::Saturate => Function::Regular("saturate"),
                    // trigonometry
                    Mf::Cos => Function::Regular("cos"),
                    Mf::Cosh => Function::Regular("cosh"),
                    Mf::Sin => Function::Regular("sin"),
                    Mf::Sinh => Function::Regular("sinh"),
                    Mf::Tan => Function::Regular("tan"),
                    Mf::Tanh => Function::Regular("tanh"),
                    Mf::Acos => Function::Regular("acos"),
                    Mf::Asin => Function::Regular("asin"),
                    Mf::Atan => Function::Regular("atan"),
                    Mf::Atan2 => Function::Regular("atan2"),
                    Mf::Asinh => Function::Asincosh { is_sin: true },
                    Mf::Acosh => Function::Asincosh { is_sin: false },
                    Mf::Atanh => Function::Atanh,
                    Mf::Radians => Function::Regular("radians"),
                    Mf::Degrees => Function::Regular("degrees"),
                    // decomposition
                    Mf::Ceil => Function::Regular("ceil"),
                    Mf::Floor => Function::Regular("floor"),
                    Mf::Round => Function::Regular("round"),
                    Mf::Fract => Function::Regular("frac"),
                    Mf::Trunc => Function::Regular("trunc"),
                    Mf::Modf => Function::Regular(MODF_FUNCTION),
                    Mf::Frexp => Function::Regular(FREXP_FUNCTION),
                    Mf::Ldexp => Function::Regular("ldexp"),
                    // exponent
                    Mf::Exp => Function::Regular("exp"),
                    Mf::Exp2 => Function::Regular("exp2"),
                    Mf::Log => Function::Regular("log"),
                    Mf::Log2 => Function::Regular("log2"),
                    Mf::Pow => Function::Regular("pow"),
                    // geometry
                    Mf::Dot => Function::Regular("dot"),
                    Mf::Dot4I8Packed => Function::Dot4I8Packed,
                    Mf::Dot4U8Packed => Function::Dot4U8Packed,
                    //Mf::Outer => ,
                    Mf::Cross => Function::Regular("cross"),
                    Mf::Distance => Function::Regular("distance"),
                    Mf::Length => Function::Regular("length"),
                    Mf::Normalize => Function::Regular("normalize"),
                    Mf::FaceForward => Function::Regular("faceforward"),
                    Mf::Reflect => Function::Regular("reflect"),
                    Mf::Refract => Function::Regular("refract"),
                    // computational
                    Mf::Sign => Function::Regular("sign"),
                    Mf::Fma => Function::Regular("mad"),
                    Mf::Mix => Function::Regular("lerp"),
                    Mf::Step => Function::Regular("step"),
                    Mf::SmoothStep => Function::Regular("smoothstep"),
                    Mf::Sqrt => Function::Regular("sqrt"),
                    Mf::InverseSqrt => Function::Regular("rsqrt"),
                    //Mf::Inverse =>,
                    Mf::Transpose => Function::Regular("transpose"),
                    Mf::Determinant => Function::Regular("determinant"),
                    Mf::QuantizeToF16 => Function::QuantizeToF16,
                    // bits
                    Mf::CountTrailingZeros => Function::CountTrailingZeros,
                    Mf::CountLeadingZeros => Function::CountLeadingZeros,
                    Mf::CountOneBits => Function::MissingIntOverload("countbits"),
                    Mf::ReverseBits => Function::MissingIntOverload("reversebits"),
                    Mf::FirstTrailingBit => Function::MissingIntReturnType("firstbitlow"),
                    Mf::FirstLeadingBit => Function::MissingIntReturnType("firstbithigh"),
                    Mf::ExtractBits => Function::Regular(EXTRACT_BITS_FUNCTION),
                    Mf::InsertBits => Function::Regular(INSERT_BITS_FUNCTION),
                    // Data Packing
                    Mf::Pack2x16float => Function::Pack2x16float,
                    Mf::Pack2x16snorm => Function::Pack2x16snorm,
                    Mf::Pack2x16unorm => Function::Pack2x16unorm,
                    Mf::Pack4x8snorm => Function::Pack4x8snorm,
                    Mf::Pack4x8unorm => Function::Pack4x8unorm,
                    Mf::Pack4xI8 => Function::Pack4xI8,
                    Mf::Pack4xU8 => Function::Pack4xU8,
                    Mf::Pack4xI8Clamp => Function::Pack4xI8Clamp,
                    Mf::Pack4xU8Clamp => Function::Pack4xU8Clamp,
                    // Data Unpacking
                    Mf::Unpack2x16float => Function::Unpack2x16float,
                    Mf::Unpack2x16snorm => Function::Unpack2x16snorm,
                    Mf::Unpack2x16unorm => Function::Unpack2x16unorm,
                    Mf::Unpack4x8snorm => Function::Unpack4x8snorm,
                    Mf::Unpack4x8unorm => Function::Unpack4x8unorm,
                    Mf::Unpack4xI8 => Function::Unpack4xI8,
                    Mf::Unpack4xU8 => Function::Unpack4xU8,
                    _ => return Err(Error::Unimplemented(format!("write_expr_math {fun:?}"))),
                };

                match fun {
                    Function::Asincosh { is_sin } => {
                        write!(self.out, "log(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " + sqrt(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " * ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        match is_sin {
                            true => write!(self.out, " + 1.0))")?,
                            false => write!(self.out, " - 1.0))")?,
                        }
                    }
                    Function::Atanh => {
                        write!(self.out, "0.5 * log((1.0 + ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, ") / (1.0 - ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, "))")?;
                    }
                    Function::Pack2x16float => {
                        write!(self.out, "(f32tof16(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, "[0]) | f32tof16(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, "[1]) << 16)")?;
                    }
                    Function::Pack2x16snorm => {
                        let scale = 32767;

                        write!(self.out, "uint((int(round(clamp(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(
                            self.out,
                            "[0], -1.0, 1.0) * {scale}.0)) & 0xFFFF) | ((int(round(clamp("
                        )?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, "[1], -1.0, 1.0) * {scale}.0)) & 0xFFFF) << 16))",)?;
                    }
                    Function::Pack2x16unorm => {
                        let scale = 65535;

                        write!(self.out, "(uint(round(clamp(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, "[0], 0.0, 1.0) * {scale}.0)) | uint(round(clamp(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, "[1], 0.0, 1.0) * {scale}.0)) << 16)")?;
                    }
                    Function::Pack4x8snorm => {
                        let scale = 127;

                        write!(self.out, "uint((int(round(clamp(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(
                            self.out,
                            "[0], -1.0, 1.0) * {scale}.0)) & 0xFF) | ((int(round(clamp("
                        )?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(
                            self.out,
                            "[1], -1.0, 1.0) * {scale}.0)) & 0xFF) << 8) | ((int(round(clamp("
                        )?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(
                            self.out,
                            "[2], -1.0, 1.0) * {scale}.0)) & 0xFF) << 16) | ((int(round(clamp("
                        )?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, "[3], -1.0, 1.0) * {scale}.0)) & 0xFF) << 24))",)?;
                    }
                    Function::Pack4x8unorm => {
                        let scale = 255;

                        write!(self.out, "(uint(round(clamp(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, "[0], 0.0, 1.0) * {scale}.0)) | uint(round(clamp(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(
                            self.out,
                            "[1], 0.0, 1.0) * {scale}.0)) << 8 | uint(round(clamp("
                        )?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(
                            self.out,
                            "[2], 0.0, 1.0) * {scale}.0)) << 16 | uint(round(clamp("
                        )?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, "[3], 0.0, 1.0) * {scale}.0)) << 24)")?;
                    }
                    fun @ (Function::Pack4xI8
                    | Function::Pack4xU8
                    | Function::Pack4xI8Clamp
                    | Function::Pack4xU8Clamp) => {
                        let was_signed =
                            matches!(fun, Function::Pack4xI8 | Function::Pack4xI8Clamp);
                        let clamp_bounds = match fun {
                            Function::Pack4xI8Clamp => Some(("-128", "127")),
                            Function::Pack4xU8Clamp => Some(("0", "255")),
                            _ => None,
                        };
                        if was_signed {
                            write!(self.out, "uint(")?;
                        }
                        let write_arg = |this: &mut Self| -> BackendResult {
                            if let Some((min, max)) = clamp_bounds {
                                write!(this.out, "clamp(")?;
                                this.write_expr(module, arg, func_ctx)?;
                                write!(this.out, ", {min}, {max})")?;
                            } else {
                                this.write_expr(module, arg, func_ctx)?;
                            }
                            Ok(())
                        };
                        write!(self.out, "(")?;
                        write_arg(self)?;
                        write!(self.out, "[0] & 0xFF) | ((")?;
                        write_arg(self)?;
                        write!(self.out, "[1] & 0xFF) << 8) | ((")?;
                        write_arg(self)?;
                        write!(self.out, "[2] & 0xFF) << 16) | ((")?;
                        write_arg(self)?;
                        write!(self.out, "[3] & 0xFF) << 24)")?;
                        if was_signed {
                            write!(self.out, ")")?;
                        }
                    }

                    Function::Unpack2x16float => {
                        write!(self.out, "float2(f16tof32(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, "), f16tof32((")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, ") >> 16))")?;
                    }
                    Function::Unpack2x16snorm => {
                        let scale = 32767;

                        write!(self.out, "(float2(int2(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " << 16, ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, ") >> 16) / {scale}.0)")?;
                    }
                    Function::Unpack2x16unorm => {
                        let scale = 65535;

                        write!(self.out, "(float2(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " & 0xFFFF, ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " >> 16) / {scale}.0)")?;
                    }
                    Function::Unpack4x8snorm => {
                        let scale = 127;

                        write!(self.out, "(float4(int4(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " << 24, ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " << 16, ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " << 8, ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, ") >> 24) / {scale}.0)")?;
                    }
                    Function::Unpack4x8unorm => {
                        let scale = 255;

                        write!(self.out, "(float4(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " & 0xFF, ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " >> 8 & 0xFF, ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " >> 16 & 0xFF, ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " >> 24) / {scale}.0)")?;
                    }
                    fun @ (Function::Unpack4xI8 | Function::Unpack4xU8) => {
                        write!(self.out, "(")?;
                        if matches!(fun, Function::Unpack4xU8) {
                            write!(self.out, "u")?;
                        }
                        write!(self.out, "int4(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, ", ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " >> 8, ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " >> 16, ")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, " >> 24) << 24 >> 24)")?;
                    }
                    fun @ (Function::Dot4I8Packed | Function::Dot4U8Packed) => {
                        let arg1 = arg1.unwrap();

                        if self.options.shader_model >= ShaderModel::V6_4 {
                            // Intrinsics `dot4add_{i, u}8packed` are available in SM 6.4 and later.
                            let function_name = match fun {
                                Function::Dot4I8Packed => "dot4add_i8packed",
                                Function::Dot4U8Packed => "dot4add_u8packed",
                                _ => unreachable!(),
                            };
                            write!(self.out, "{function_name}(")?;
                            self.write_expr(module, arg, func_ctx)?;
                            write!(self.out, ", ")?;
                            self.write_expr(module, arg1, func_ctx)?;
                            write!(self.out, ", 0)")?;
                        } else {
                            // Fall back to a polyfill as `dot4add_u8packed` is not available.
                            write!(self.out, "dot(")?;

                            if matches!(fun, Function::Dot4U8Packed) {
                                write!(self.out, "u")?;
                            }
                            write!(self.out, "int4(")?;
                            self.write_expr(module, arg, func_ctx)?;
                            write!(self.out, ", ")?;
                            self.write_expr(module, arg, func_ctx)?;
                            write!(self.out, " >> 8, ")?;
                            self.write_expr(module, arg, func_ctx)?;
                            write!(self.out, " >> 16, ")?;
                            self.write_expr(module, arg, func_ctx)?;
                            write!(self.out, " >> 24) << 24 >> 24, ")?;

                            if matches!(fun, Function::Dot4U8Packed) {
                                write!(self.out, "u")?;
                            }
                            write!(self.out, "int4(")?;
                            self.write_expr(module, arg1, func_ctx)?;
                            write!(self.out, ", ")?;
                            self.write_expr(module, arg1, func_ctx)?;
                            write!(self.out, " >> 8, ")?;
                            self.write_expr(module, arg1, func_ctx)?;
                            write!(self.out, " >> 16, ")?;
                            self.write_expr(module, arg1, func_ctx)?;
                            write!(self.out, " >> 24) << 24 >> 24)")?;
                        }
                    }
                    Function::QuantizeToF16 => {
                        write!(self.out, "f16tof32(f32tof16(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, "))")?;
                    }
                    Function::Regular(fun_name) => {
                        write!(self.out, "{fun_name}(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        if let Some(arg) = arg1 {
                            write!(self.out, ", ")?;
                            self.write_expr(module, arg, func_ctx)?;
                        }
                        if let Some(arg) = arg2 {
                            write!(self.out, ", ")?;
                            self.write_expr(module, arg, func_ctx)?;
                        }
                        if let Some(arg) = arg3 {
                            write!(self.out, ", ")?;
                            self.write_expr(module, arg, func_ctx)?;
                        }
                        write!(self.out, ")")?
                    }
                    // These overloads are only missing on FXC, so this is only needed for 32bit types,
                    // as non-32bit types are DXC only.
                    Function::MissingIntOverload(fun_name) => {
                        let scalar_kind = func_ctx.resolve_type(arg, &module.types).scalar();
                        if let Some(Scalar::I32) = scalar_kind {
                            write!(self.out, "asint({fun_name}(asuint(")?;
                            self.write_expr(module, arg, func_ctx)?;
                            write!(self.out, ")))")?;
                        } else {
                            write!(self.out, "{fun_name}(")?;
                            self.write_expr(module, arg, func_ctx)?;
                            write!(self.out, ")")?;
                        }
                    }
                    // These overloads are only missing on FXC, so this is only needed for 32bit types,
                    // as non-32bit types are DXC only.
                    Function::MissingIntReturnType(fun_name) => {
                        let scalar_kind = func_ctx.resolve_type(arg, &module.types).scalar();
                        if let Some(Scalar::I32) = scalar_kind {
                            write!(self.out, "asint({fun_name}(")?;
                            self.write_expr(module, arg, func_ctx)?;
                            write!(self.out, "))")?;
                        } else {
                            write!(self.out, "{fun_name}(")?;
                            self.write_expr(module, arg, func_ctx)?;
                            write!(self.out, ")")?;
                        }
                    }
                    Function::CountTrailingZeros => {
                        match *func_ctx.resolve_type(arg, &module.types) {
                            TypeInner::Vector { size, scalar } => {
                                let s = match size {
                                    crate::VectorSize::Bi => ".xx",
                                    crate::VectorSize::Tri => ".xxx",
                                    crate::VectorSize::Quad => ".xxxx",
                                };

                                let scalar_width_bits = scalar.width * 8;

                                if scalar.kind == ScalarKind::Uint || scalar.width != 4 {
                                    write!(
                                        self.out,
                                        "min(({scalar_width_bits}u){s}, firstbitlow("
                                    )?;
                                    self.write_expr(module, arg, func_ctx)?;
                                    write!(self.out, "))")?;
                                } else {
                                    // This is only needed for the FXC path, on 32bit signed integers.
                                    write!(
                                        self.out,
                                        "asint(min(({scalar_width_bits}u){s}, firstbitlow("
                                    )?;
                                    self.write_expr(module, arg, func_ctx)?;
                                    write!(self.out, ")))")?;
                                }
                            }
                            TypeInner::Scalar(scalar) => {
                                let scalar_width_bits = scalar.width * 8;

                                if scalar.kind == ScalarKind::Uint || scalar.width != 4 {
                                    write!(self.out, "min({scalar_width_bits}u, firstbitlow(")?;
                                    self.write_expr(module, arg, func_ctx)?;
                                    write!(self.out, "))")?;
                                } else {
                                    // This is only needed for the FXC path, on 32bit signed integers.
                                    write!(
                                        self.out,
                                        "asint(min({scalar_width_bits}u, firstbitlow("
                                    )?;
                                    self.write_expr(module, arg, func_ctx)?;
                                    write!(self.out, ")))")?;
                                }
                            }
                            _ => unreachable!(),
                        }

                        return Ok(());
                    }
                    Function::CountLeadingZeros => {
                        match *func_ctx.resolve_type(arg, &module.types) {
                            TypeInner::Vector { size, scalar } => {
                                let s = match size {
                                    crate::VectorSize::Bi => ".xx",
                                    crate::VectorSize::Tri => ".xxx",
                                    crate::VectorSize::Quad => ".xxxx",
                                };

                                // scalar width - 1
                                let constant = scalar.width * 8 - 1;

                                if scalar.kind == ScalarKind::Uint {
                                    write!(self.out, "(({constant}u){s} - firstbithigh(")?;
                                    self.write_expr(module, arg, func_ctx)?;
                                    write!(self.out, "))")?;
                                } else {
                                    let conversion_func = match scalar.width {
                                        4 => "asint",
                                        _ => "",
                                    };
                                    write!(self.out, "(")?;
                                    self.write_expr(module, arg, func_ctx)?;
                                    write!(
                                        self.out,
                                        " < (0){s} ? (0){s} : ({constant}){s} - {conversion_func}(firstbithigh("
                                    )?;
                                    self.write_expr(module, arg, func_ctx)?;
                                    write!(self.out, ")))")?;
                                }
                            }
                            TypeInner::Scalar(scalar) => {
                                // scalar width - 1
                                let constant = scalar.width * 8 - 1;

                                if let ScalarKind::Uint = scalar.kind {
                                    write!(self.out, "({constant}u - firstbithigh(")?;
                                    self.write_expr(module, arg, func_ctx)?;
                                    write!(self.out, "))")?;
                                } else {
                                    let conversion_func = match scalar.width {
                                        4 => "asint",
                                        _ => "",
                                    };
                                    write!(self.out, "(")?;
                                    self.write_expr(module, arg, func_ctx)?;
                                    write!(
                                        self.out,
                                        " < 0 ? 0 : {constant} - {conversion_func}(firstbithigh("
                                    )?;
                                    self.write_expr(module, arg, func_ctx)?;
                                    write!(self.out, ")))")?;
                                }
                            }
                            _ => unreachable!(),
                        }

                        return Ok(());
                    }
                }
            }
            Expression::Swizzle {
                size,
                vector,
                pattern,
            } => {
                self.write_expr(module, vector, func_ctx)?;
                write!(self.out, ".")?;
                for &sc in pattern[..size as usize].iter() {
                    self.out.write_char(back::COMPONENTS[sc as usize])?;
                }
            }
            Expression::ArrayLength(expr) => {
                let var_handle = match func_ctx.expressions[expr] {
                    Expression::AccessIndex { base, index: _ } => {
                        match func_ctx.expressions[base] {
                            Expression::GlobalVariable(handle) => handle,
                            _ => unreachable!(),
                        }
                    }
                    Expression::GlobalVariable(handle) => handle,
                    _ => unreachable!(),
                };

                let var = &module.global_variables[var_handle];
                let (offset, stride) = match module.types[var.ty].inner {
                    TypeInner::Array { stride, .. } => (0, stride),
                    TypeInner::Struct { ref members, .. } => {
                        let last = members.last().unwrap();
                        let stride = match module.types[last.ty].inner {
                            TypeInner::Array { stride, .. } => stride,
                            _ => unreachable!(),
                        };
                        (last.offset, stride)
                    }
                    _ => unreachable!(),
                };

                let storage_access = match var.space {
                    crate::AddressSpace::Storage { access } => access,
                    _ => crate::StorageAccess::default(),
                };
                let wrapped_array_length = WrappedArrayLength {
                    writable: storage_access.contains(crate::StorageAccess::STORE),
                };

                write!(self.out, "((")?;
                self.write_wrapped_array_length_function_name(wrapped_array_length)?;
                let var_name = &self.names[&NameKey::GlobalVariable(var_handle)];
                write!(self.out, "({var_name}) - {offset}) / {stride})")?
            }
            Expression::Derivative { axis, ctrl, expr } => {
                use crate::{DerivativeAxis as Axis, DerivativeControl as Ctrl};
                if axis == Axis::Width && (ctrl == Ctrl::Coarse || ctrl == Ctrl::Fine) {
                    let tail = match ctrl {
                        Ctrl::Coarse => "coarse",
                        Ctrl::Fine => "fine",
                        Ctrl::None => unreachable!(),
                    };
                    write!(self.out, "abs(ddx_{tail}(")?;
                    self.write_expr(module, expr, func_ctx)?;
                    write!(self.out, ")) + abs(ddy_{tail}(")?;
                    self.write_expr(module, expr, func_ctx)?;
                    write!(self.out, "))")?
                } else {
                    let fun_str = match (axis, ctrl) {
                        (Axis::X, Ctrl::Coarse) => "ddx_coarse",
                        (Axis::X, Ctrl::Fine) => "ddx_fine",
                        (Axis::X, Ctrl::None) => "ddx",
                        (Axis::Y, Ctrl::Coarse) => "ddy_coarse",
                        (Axis::Y, Ctrl::Fine) => "ddy_fine",
                        (Axis::Y, Ctrl::None) => "ddy",
                        (Axis::Width, Ctrl::Coarse | Ctrl::Fine) => unreachable!(),
                        (Axis::Width, Ctrl::None) => "fwidth",
                    };
                    write!(self.out, "{fun_str}(")?;
                    self.write_expr(module, expr, func_ctx)?;
                    write!(self.out, ")")?
                }
            }
            Expression::Relational { fun, argument } => {
                use crate::RelationalFunction as Rf;

                let fun_str = match fun {
                    Rf::All => "all",
                    Rf::Any => "any",
                    Rf::IsNan => "isnan",
                    Rf::IsInf => "isinf",
                };
                write!(self.out, "{fun_str}(")?;
                self.write_expr(module, argument, func_ctx)?;
                write!(self.out, ")")?
            }
            Expression::Select {
                condition,
                accept,
                reject,
            } => {
                write!(self.out, "(")?;
                self.write_expr(module, condition, func_ctx)?;
                write!(self.out, " ? ")?;
                self.write_expr(module, accept, func_ctx)?;
                write!(self.out, " : ")?;
                self.write_expr(module, reject, func_ctx)?;
                write!(self.out, ")")?
            }
            Expression::RayQueryGetIntersection { query, committed } => {
                // For reasoning, see write_stmt
                let Expression::LocalVariable(query_var) = func_ctx.expressions[query] else {
                    unreachable!()
                };

                let tracker_expr_name = format!(
                    "{RAY_QUERY_TRACKER_VARIABLE_PREFIX}{}",
                    self.names[&func_ctx.name_key(query_var)]
                );

                if committed {
                    write!(self.out, "GetCommittedIntersection(")?;
                    self.write_expr(module, query, func_ctx)?;
                    write!(self.out, ", {tracker_expr_name})")?;
                } else {
                    write!(self.out, "GetCandidateIntersection(")?;
                    self.write_expr(module, query, func_ctx)?;
                    write!(self.out, ", {tracker_expr_name})")?;
                }
            }
            // Not supported yet
            Expression::RayQueryVertexPositions { .. }
            | Expression::CooperativeLoad { .. }
            | Expression::CooperativeMultiplyAdd { .. } => {
                unreachable!()
            }
            // Nothing to do here, since call expression already cached
            Expression::CallResult(_)
            | Expression::AtomicResult { .. }
            | Expression::WorkGroupUniformLoadResult { .. }
            | Expression::RayQueryProceedResult
            | Expression::SubgroupBallotResult
            | Expression::SubgroupOperationResult { .. } => {}
        }

        if !closing_bracket.is_empty() {
            write!(self.out, "{closing_bracket}")?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn write_image_load(
        &mut self,
        module: &&Module,
        expr: Handle<crate::Expression>,
        func_ctx: &back::FunctionCtx,
        image: Handle<crate::Expression>,
        coordinate: Handle<crate::Expression>,
        array_index: Option<Handle<crate::Expression>>,
        sample: Option<Handle<crate::Expression>>,
        level: Option<Handle<crate::Expression>>,
    ) -> Result<(), Error> {
        let mut wrapping_type = None;
        match *func_ctx.resolve_type(image, &module.types) {
            TypeInner::Image {
                class: crate::ImageClass::External,
                ..
            } => {
                write!(self.out, "{IMAGE_LOAD_EXTERNAL_FUNCTION}(")?;
                self.write_expr(module, image, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, coordinate, func_ctx)?;
                write!(self.out, ")")?;
                return Ok(());
            }
            TypeInner::Image {
                class: crate::ImageClass::Storage { format, .. },
                ..
            } => {
                if format.single_component() {
                    wrapping_type = Some(Scalar::from(format));
                }
            }
            _ => {}
        }
        if let Some(scalar) = wrapping_type {
            write!(
                self.out,
                "{}{}(",
                help::IMAGE_STORAGE_LOAD_SCALAR_WRAPPER,
                scalar.to_hlsl_str()?
            )?;
        }
        // https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-to-load
        self.write_expr(module, image, func_ctx)?;
        write!(self.out, ".Load(")?;

        self.write_texture_coordinates("int", coordinate, array_index, level, module, func_ctx)?;

        if let Some(sample) = sample {
            write!(self.out, ", ")?;
            self.write_expr(module, sample, func_ctx)?;
        }

        // close bracket for Load function
        write!(self.out, ")")?;

        if wrapping_type.is_some() {
            write!(self.out, ")")?;
        }

        // return x component if return type is scalar
        if let TypeInner::Scalar(_) = *func_ctx.resolve_type(expr, &module.types) {
            write!(self.out, ".x")?;
        }
        Ok(())
    }

    /// Find the [`BindingArraySamplerInfo`] from an expression so that such an access
    /// can be generated later.
    fn sampler_binding_array_info_from_expression(
        &mut self,
        module: &Module,
        func_ctx: &back::FunctionCtx<'_>,
        base: Handle<crate::Expression>,
        resolved: &TypeInner,
    ) -> Option<BindingArraySamplerInfo> {
        if let TypeInner::BindingArray {
            base: base_ty_handle,
            ..
        } = *resolved
        {
            let base_ty = &module.types[base_ty_handle].inner;
            if let TypeInner::Sampler { comparison, .. } = *base_ty {
                let base = &func_ctx.expressions[base];

                if let crate::Expression::GlobalVariable(handle) = *base {
                    let variable = &module.global_variables[handle];

                    let sampler_heap_name = match comparison {
                        true => COMPARISON_SAMPLER_HEAP_VAR,
                        false => SAMPLER_HEAP_VAR,
                    };

                    return Some(BindingArraySamplerInfo {
                        sampler_heap_name,
                        sampler_index_buffer_name: self
                            .wrapped
                            .sampler_index_buffers
                            .get(&super::SamplerIndexBufferKey {
                                group: variable.binding.unwrap().group,
                            })
                            .unwrap()
                            .clone(),
                        binding_array_base_index_name: self.names[&NameKey::GlobalVariable(handle)]
                            .clone(),
                    });
                }
            }
        }

        None
    }

    fn write_named_expr(
        &mut self,
        module: &Module,
        handle: Handle<crate::Expression>,
        name: String,
        // The expression which is being named.
        // Generally, this is the same as handle, except in WorkGroupUniformLoad
        named: Handle<crate::Expression>,
        ctx: &back::FunctionCtx,
    ) -> BackendResult {
        match ctx.info[named].ty {
            proc::TypeResolution::Handle(ty_handle) => match module.types[ty_handle].inner {
                TypeInner::Struct { .. } => {
                    let ty_name = &self.names[&NameKey::Type(ty_handle)];
                    write!(self.out, "{ty_name}")?;
                }
                _ => {
                    self.write_type(module, ty_handle)?;
                }
            },
            proc::TypeResolution::Value(ref inner) => {
                self.write_value_type(module, inner)?;
            }
        }

        let resolved = ctx.resolve_type(named, &module.types);

        write!(self.out, " {name}")?;
        // If rhs is a array type, we should write array size
        if let TypeInner::Array { base, size, .. } = *resolved {
            self.write_array_size(module, base, size)?;
        }
        write!(self.out, " = ")?;
        self.write_expr(module, handle, ctx)?;
        writeln!(self.out, ";")?;
        self.named_expressions.insert(named, name);

        Ok(())
    }

    /// Helper function that write default zero initialization
    pub(super) fn write_default_init(
        &mut self,
        module: &Module,
        ty: Handle<crate::Type>,
    ) -> BackendResult {
        write!(self.out, "(")?;
        self.write_type(module, ty)?;
        if let TypeInner::Array { base, size, .. } = module.types[ty].inner {
            self.write_array_size(module, base, size)?;
        }
        write!(self.out, ")0")?;
        Ok(())
    }

    fn write_control_barrier(
        &mut self,
        barrier: crate::Barrier,
        level: back::Level,
    ) -> BackendResult {
        if barrier.contains(crate::Barrier::STORAGE) {
            writeln!(self.out, "{level}DeviceMemoryBarrierWithGroupSync();")?;
        }
        if barrier.contains(crate::Barrier::WORK_GROUP) {
            writeln!(self.out, "{level}GroupMemoryBarrierWithGroupSync();")?;
        }
        if barrier.contains(crate::Barrier::SUB_GROUP) {
            // Does not exist in DirectX
        }
        if barrier.contains(crate::Barrier::TEXTURE) {
            writeln!(self.out, "{level}DeviceMemoryBarrierWithGroupSync();")?;
        }
        Ok(())
    }

    fn write_memory_barrier(
        &mut self,
        barrier: crate::Barrier,
        level: back::Level,
    ) -> BackendResult {
        if barrier.contains(crate::Barrier::STORAGE) {
            writeln!(self.out, "{level}DeviceMemoryBarrier();")?;
        }
        if barrier.contains(crate::Barrier::WORK_GROUP) {
            writeln!(self.out, "{level}GroupMemoryBarrier();")?;
        }
        if barrier.contains(crate::Barrier::SUB_GROUP) {
            // Does not exist in DirectX
        }
        if barrier.contains(crate::Barrier::TEXTURE) {
            writeln!(self.out, "{level}DeviceMemoryBarrier();")?;
        }
        Ok(())
    }

    /// Helper to emit the shared tail of an HLSL atomic call (arguments, value, result)
    fn emit_hlsl_atomic_tail(
        &mut self,
        module: &Module,
        func_ctx: &back::FunctionCtx<'_>,
        fun: &crate::AtomicFunction,
        compare_expr: Option<Handle<crate::Expression>>,
        value: Handle<crate::Expression>,
        res_var_info: &Option<(Handle<crate::Expression>, String)>,
    ) -> BackendResult {
        if let Some(cmp) = compare_expr {
            write!(self.out, ", ")?;
            self.write_expr(module, cmp, func_ctx)?;
        }
        write!(self.out, ", ")?;
        if let crate::AtomicFunction::Subtract = *fun {
            // we just wrote `InterlockedAdd`, so negate the argument
            write!(self.out, "-")?;
        }
        self.write_expr(module, value, func_ctx)?;
        if let Some(&(_res_handle, ref res_name)) = res_var_info.as_ref() {
            write!(self.out, ", ")?;
            if compare_expr.is_some() {
                write!(self.out, "{res_name}.old_value")?;
            } else {
                write!(self.out, "{res_name}")?;
            }
        }
        writeln!(self.out, ");")?;
        Ok(())
    }
}

pub(super) struct MatrixType {
    pub(super) columns: crate::VectorSize,
    pub(super) rows: crate::VectorSize,
    pub(super) width: crate::Bytes,
}

pub(super) fn get_inner_matrix_data(
    module: &Module,
    handle: Handle<crate::Type>,
) -> Option<MatrixType> {
    match module.types[handle].inner {
        TypeInner::Matrix {
            columns,
            rows,
            scalar,
        } => Some(MatrixType {
            columns,
            rows,
            width: scalar.width,
        }),
        TypeInner::Array { base, .. } => get_inner_matrix_data(module, base),
        _ => None,
    }
}

/// If `base` is an access chain of the form `mat`, `mat[col]`, or `mat[col][row]`,
/// returns a tuple of the matrix, the column (vector) index (if present), and
/// the row (scalar) index (if present).
fn find_matrix_in_access_chain(
    module: &Module,
    base: Handle<crate::Expression>,
    func_ctx: &back::FunctionCtx<'_>,
) -> Option<(Handle<crate::Expression>, Option<Index>, Option<Index>)> {
    let mut current_base = base;
    let mut vector = None;
    let mut scalar = None;
    loop {
        let resolved_tr = func_ctx
            .resolve_type(current_base, &module.types)
            .pointer_base_type();
        let resolved = resolved_tr.as_ref()?.inner_with(&module.types);

        match *resolved {
            TypeInner::Matrix { .. } => return Some((current_base, vector, scalar)),
            TypeInner::Scalar(_) | TypeInner::Vector { .. } => {}
            _ => return None,
        }

        let index;
        (current_base, index) = match func_ctx.expressions[current_base] {
            crate::Expression::Access { base, index } => (base, Index::Expression(index)),
            crate::Expression::AccessIndex { base, index } => (base, Index::Static(index)),
            _ => return None,
        };

        match *resolved {
            TypeInner::Scalar(_) => scalar = Some(index),
            TypeInner::Vector { .. } => vector = Some(index),
            _ => unreachable!(),
        }
    }
}

/// Returns the matrix data if the access chain starting at `base`:
/// - starts with an expression with resolved type of [`TypeInner::Matrix`] if `direct = true`
/// - contains one or more expressions with resolved type of [`TypeInner::Array`] of [`TypeInner::Matrix`]
/// - ends at an expression with resolved type of [`TypeInner::Struct`]
pub(super) fn get_inner_matrix_of_struct_array_member(
    module: &Module,
    base: Handle<crate::Expression>,
    func_ctx: &back::FunctionCtx<'_>,
    direct: bool,
) -> Option<MatrixType> {
    let mut mat_data = None;
    let mut array_base = None;

    let mut current_base = base;
    loop {
        let mut resolved = func_ctx.resolve_type(current_base, &module.types);
        if let TypeInner::Pointer { base, .. } = *resolved {
            resolved = &module.types[base].inner;
        };

        match *resolved {
            TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } => {
                mat_data = Some(MatrixType {
                    columns,
                    rows,
                    width: scalar.width,
                })
            }
            TypeInner::Array { base, .. } => {
                array_base = Some(base);
            }
            TypeInner::Struct { .. } => {
                if let Some(array_base) = array_base {
                    if direct {
                        return mat_data;
                    } else {
                        return get_inner_matrix_data(module, array_base);
                    }
                }

                break;
            }
            _ => break,
        }

        current_base = match func_ctx.expressions[current_base] {
            crate::Expression::Access { base, .. } => base,
            crate::Expression::AccessIndex { base, .. } => base,
            _ => break,
        };
    }
    None
}

/// Simpler version of get_inner_matrix_of_global_uniform that only looks at the
/// immediate expression, rather than traversing an access chain.
fn get_global_uniform_matrix(
    module: &Module,
    base: Handle<crate::Expression>,
    func_ctx: &back::FunctionCtx<'_>,
) -> Option<MatrixType> {
    let base_tr = func_ctx
        .resolve_type(base, &module.types)
        .pointer_base_type();
    let base_ty = base_tr.as_ref().map(|tr| tr.inner_with(&module.types));
    match (&func_ctx.expressions[base], base_ty) {
        (
            &crate::Expression::GlobalVariable(handle),
            Some(&TypeInner::Matrix {
                columns,
                rows,
                scalar,
            }),
        ) if module.global_variables[handle].space == crate::AddressSpace::Uniform => {
            Some(MatrixType {
                columns,
                rows,
                width: scalar.width,
            })
        }
        _ => None,
    }
}

/// Returns the matrix data if the access chain starting at `base`:
/// - starts with an expression with resolved type of [`TypeInner::Matrix`]
/// - contains zero or more expressions with resolved type of [`TypeInner::Array`] of [`TypeInner::Matrix`]
/// - ends with an [`Expression::GlobalVariable`](crate::Expression::GlobalVariable) in [`AddressSpace::Uniform`](crate::AddressSpace::Uniform)
fn get_inner_matrix_of_global_uniform(
    module: &Module,
    base: Handle<crate::Expression>,
    func_ctx: &back::FunctionCtx<'_>,
) -> Option<MatrixType> {
    let mut mat_data = None;
    let mut array_base = None;

    let mut current_base = base;
    loop {
        let mut resolved = func_ctx.resolve_type(current_base, &module.types);
        if let TypeInner::Pointer { base, .. } = *resolved {
            resolved = &module.types[base].inner;
        };

        match *resolved {
            TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } => {
                mat_data = Some(MatrixType {
                    columns,
                    rows,
                    width: scalar.width,
                })
            }
            TypeInner::Array { base, .. } => {
                array_base = Some(base);
            }
            _ => break,
        }

        current_base = match func_ctx.expressions[current_base] {
            crate::Expression::Access { base, .. } => base,
            crate::Expression::AccessIndex { base, .. } => base,
            crate::Expression::GlobalVariable(handle)
                if module.global_variables[handle].space == crate::AddressSpace::Uniform =>
            {
                return mat_data.or_else(|| {
                    array_base.and_then(|array_base| get_inner_matrix_data(module, array_base))
                })
            }
            _ => break,
        };
    }
    None
}
