use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::fmt::Write;

use super::Error;
use super::ToWgslIfImplemented as _;
use crate::{back::wgsl::polyfill::InversePolyfill, common::wgsl::TypeContext};
use crate::{
    back::{self, Baked},
    common::{
        self,
        wgsl::{address_space_str, ToWgsl, TryToWgsl},
    },
    proc::{self, NameKey},
    valid, Handle, Module, ShaderStage, TypeInner,
};

/// Shorthand result used internally by the backend
type BackendResult = Result<(), Error>;

/// WGSL [attribute](https://gpuweb.github.io/gpuweb/wgsl/#attributes)
enum Attribute {
    Binding(u32),
    BuiltIn(crate::BuiltIn),
    Group(u32),
    Invariant,
    Interpolate(Option<crate::Interpolation>, Option<crate::Sampling>),
    Location(u32),
    BlendSrc(u32),
    Stage(ShaderStage),
    WorkGroupSize([u32; 3]),
    MeshStage(String),
    TaskPayload(String),
    PerPrimitive,
    IncomingRayPayload(String),
}

/// The WGSL form that `write_expr_with_indirection` should use to render a Naga
/// expression.
///
/// Sometimes a Naga `Expression` alone doesn't provide enough information to
/// choose the right rendering for it in WGSL. For example, one natural WGSL
/// rendering of a Naga `LocalVariable(x)` expression might be `&x`, since
/// `LocalVariable` produces a pointer to the local variable's storage. But when
/// rendering a `Store` statement, the `pointer` operand must be the left hand
/// side of a WGSL assignment, so the proper rendering is `x`.
///
/// The caller of `write_expr_with_indirection` must provide an `Expected` value
/// to indicate how ambiguous expressions should be rendered.
#[derive(Clone, Copy, Debug)]
enum Indirection {
    /// Render pointer-construction expressions as WGSL `ptr`-typed expressions.
    ///
    /// This is the right choice for most cases. Whenever a Naga pointer
    /// expression is not the `pointer` operand of a `Load` or `Store`, it
    /// must be a WGSL pointer expression.
    Ordinary,

    /// Render pointer-construction expressions as WGSL reference-typed
    /// expressions.
    ///
    /// For example, this is the right choice for the `pointer` operand when
    /// rendering a `Store` statement as a WGSL assignment.
    Reference,
}

bitflags::bitflags! {
    #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
    #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct WriterFlags: u32 {
        /// Always annotate the type information instead of inferring.
        const EXPLICIT_TYPES = 0x1;
    }
}

pub struct Writer<W> {
    out: W,
    flags: WriterFlags,
    names: crate::FastHashMap<NameKey, String>,
    namer: proc::Namer,
    named_expressions: crate::NamedExpressions,
    required_polyfills: crate::FastIndexSet<InversePolyfill>,
}

impl<W: Write> Writer<W> {
    pub fn new(out: W, flags: WriterFlags) -> Self {
        Writer {
            out,
            flags,
            names: crate::FastHashMap::default(),
            namer: proc::Namer::default(),
            named_expressions: crate::NamedExpressions::default(),
            required_polyfills: crate::FastIndexSet::default(),
        }
    }

    fn reset(&mut self, module: &Module) {
        self.names.clear();
        self.namer.reset(
            module,
            &crate::keywords::wgsl::RESERVED_SET,
            &crate::keywords::wgsl::BUILTIN_IDENTIFIER_SET,
            // an identifier must not start with two underscore
            proc::CaseInsensitiveKeywordSet::empty(),
            &["__", "_naga"],
            &mut self.names,
        );
        self.named_expressions.clear();
        self.required_polyfills.clear();
    }

    /// Determine if `ty` is the Naga IR presentation of a WGSL builtin type.
    ///
    /// Return true if `ty` refers to the Naga IR form of a WGSL builtin type
    /// like `__atomic_compare_exchange_result`.
    ///
    /// Even though the module may use the type, the WGSL backend should avoid
    /// emitting a definition for it, since it is [predeclared] in WGSL.
    ///
    /// This also covers types like [`NagaExternalTextureParams`], which other
    /// backends use to lower WGSL constructs like external textures to their
    /// implementations. WGSL can express these directly, so the types need not
    /// be emitted.
    ///
    /// [predeclared]: https://www.w3.org/TR/WGSL/#predeclared
    /// [`NagaExternalTextureParams`]: crate::ir::SpecialTypes::external_texture_params
    fn is_builtin_wgsl_struct(&self, module: &Module, ty: Handle<crate::Type>) -> bool {
        module
            .special_types
            .predeclared_types
            .values()
            .any(|t| *t == ty)
            || Some(ty) == module.special_types.external_texture_params
            || Some(ty) == module.special_types.external_texture_transfer_function
    }

    pub fn write(&mut self, module: &Module, info: &valid::ModuleInfo) -> BackendResult {
        self.reset(module);

        // Write all `enable` declarations
        self.write_enable_declarations(module)?;

        // Write all structs
        for (handle, ty) in module.types.iter() {
            if let TypeInner::Struct { ref members, .. } = ty.inner {
                {
                    if !self.is_builtin_wgsl_struct(module, handle) {
                        self.write_struct(module, handle, members)?;
                        writeln!(self.out)?;
                    }
                }
            }
        }

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

        // Write all overrides
        let mut overrides = module.overrides.iter().peekable();
        while let Some((handle, _)) = overrides.next() {
            self.write_override(module, handle)?;
            // Add extra newline for readability on last iteration
            if overrides.peek().is_none() {
                writeln!(self.out)?;
            }
        }

        // Write all globals
        for (ty, global) in module.global_variables.iter() {
            self.write_global(module, global, ty)?;
        }

        if !module.global_variables.is_empty() {
            // Add extra newline for readability
            writeln!(self.out)?;
        }

        // Write all regular functions
        for (handle, function) in module.functions.iter() {
            let fun_info = &info[handle];

            let func_ctx = back::FunctionCtx {
                ty: back::FunctionType::Function(handle),
                info: fun_info,
                expressions: &function.expressions,
                named_expressions: &function.named_expressions,
            };

            // Write the function
            self.write_function(module, function, &func_ctx)?;

            writeln!(self.out)?;
        }

        // Write all entry points
        for (index, ep) in module.entry_points.iter().enumerate() {
            let attributes = match ep.stage {
                ShaderStage::Vertex | ShaderStage::Fragment => vec![Attribute::Stage(ep.stage)],
                ShaderStage::Compute => vec![
                    Attribute::Stage(ShaderStage::Compute),
                    Attribute::WorkGroupSize(ep.workgroup_size),
                ],
                ShaderStage::Mesh => {
                    let mesh_output_name = module.global_variables
                        [ep.mesh_info.as_ref().unwrap().output_variable]
                        .name
                        .clone()
                        .unwrap();
                    let mut mesh_attrs = vec![
                        Attribute::MeshStage(mesh_output_name),
                        Attribute::WorkGroupSize(ep.workgroup_size),
                    ];
                    if let Some(task_payload) = ep.task_payload {
                        let payload_name =
                            module.global_variables[task_payload].name.clone().unwrap();
                        mesh_attrs.push(Attribute::TaskPayload(payload_name));
                    }
                    mesh_attrs
                }
                ShaderStage::Task => {
                    let payload_name = module.global_variables[ep.task_payload.unwrap()]
                        .name
                        .clone()
                        .unwrap();
                    vec![
                        Attribute::Stage(ShaderStage::Task),
                        Attribute::TaskPayload(payload_name),
                        Attribute::WorkGroupSize(ep.workgroup_size),
                    ]
                }
                ShaderStage::RayGeneration => vec![Attribute::Stage(ShaderStage::RayGeneration)],
                ShaderStage::AnyHit | ShaderStage::ClosestHit | ShaderStage::Miss => {
                    let payload_name = module.global_variables[ep.incoming_ray_payload.unwrap()]
                        .name
                        .clone()
                        .unwrap();
                    vec![
                        Attribute::Stage(ep.stage),
                        Attribute::IncomingRayPayload(payload_name),
                    ]
                }
            };
            self.write_attributes(&attributes)?;
            // Add a newline after attribute
            writeln!(self.out)?;

            let func_ctx = back::FunctionCtx {
                ty: back::FunctionType::EntryPoint(index as u16),
                info: info.get_entry_point(index),
                expressions: &ep.function.expressions,
                named_expressions: &ep.function.named_expressions,
            };
            self.write_function(module, &ep.function, &func_ctx)?;

            if index < module.entry_points.len() - 1 {
                writeln!(self.out)?;
            }
        }

        // Write any polyfills that were required.
        for polyfill in &self.required_polyfills {
            writeln!(self.out)?;
            write!(self.out, "{}", polyfill.source)?;
            writeln!(self.out)?;
        }

        Ok(())
    }

    /// Helper method which writes all the `enable` declarations
    /// needed for a module.
    fn write_enable_declarations(&mut self, module: &Module) -> BackendResult {
        #[derive(Default)]
        struct RequiredEnabled {
            f16: bool,
            dual_source_blending: bool,
            clip_distances: bool,
            mesh_shaders: bool,
            primitive_index: bool,
            cooperative_matrix: bool,
            draw_index: bool,
            ray_tracing_pipeline: bool,
        }
        let mut needed = RequiredEnabled {
            mesh_shaders: module.uses_mesh_shaders(),
            ..Default::default()
        };

        let check_binding = |binding: &crate::Binding, needed: &mut RequiredEnabled| match *binding
        {
            crate::Binding::Location {
                blend_src: Some(_), ..
            } => {
                needed.dual_source_blending = true;
            }
            crate::Binding::BuiltIn(crate::BuiltIn::ClipDistance) => {
                needed.clip_distances = true;
            }
            crate::Binding::BuiltIn(crate::BuiltIn::PrimitiveIndex) => {
                needed.primitive_index = true;
            }
            crate::Binding::Location {
                per_primitive: true,
                ..
            } => {
                needed.mesh_shaders = true;
            }
            crate::Binding::BuiltIn(crate::BuiltIn::DrawIndex) => needed.draw_index = true,
            crate::Binding::BuiltIn(
                crate::BuiltIn::RayInvocationId
                | crate::BuiltIn::NumRayInvocations
                | crate::BuiltIn::InstanceCustomData
                | crate::BuiltIn::GeometryIndex
                | crate::BuiltIn::WorldRayOrigin
                | crate::BuiltIn::WorldRayDirection
                | crate::BuiltIn::ObjectRayOrigin
                | crate::BuiltIn::ObjectRayDirection
                | crate::BuiltIn::RayTmin
                | crate::BuiltIn::RayTCurrentMax
                | crate::BuiltIn::ObjectToWorld
                | crate::BuiltIn::WorldToObject,
            ) => {
                needed.ray_tracing_pipeline = true;
            }
            _ => {}
        };

        // Determine which `enable` declarations are needed
        for (_, ty) in module.types.iter() {
            match ty.inner {
                TypeInner::Scalar(scalar)
                | TypeInner::Vector { scalar, .. }
                | TypeInner::Matrix { scalar, .. } => {
                    needed.f16 |= scalar == crate::Scalar::F16;
                }
                TypeInner::Struct { ref members, .. } => {
                    for binding in members.iter().filter_map(|m| m.binding.as_ref()) {
                        check_binding(binding, &mut needed);
                    }
                }
                TypeInner::CooperativeMatrix { .. } => {
                    needed.cooperative_matrix = true;
                }
                TypeInner::AccelerationStructure { .. } => {
                    needed.ray_tracing_pipeline = true;
                }
                _ => {}
            }
        }

        for ep in &module.entry_points {
            if let Some(res) = ep.function.result.as_ref().and_then(|a| a.binding.as_ref()) {
                check_binding(res, &mut needed);
            }
            for arg in ep
                .function
                .arguments
                .iter()
                .filter_map(|a| a.binding.as_ref())
            {
                check_binding(arg, &mut needed);
            }
        }

        if module.global_variables.iter().any(|gv| {
            gv.1.space == crate::AddressSpace::IncomingRayPayload
                || gv.1.space == crate::AddressSpace::RayPayload
        }) {
            needed.ray_tracing_pipeline = true;
        }

        if module.entry_points.iter().any(|ep| {
            matches!(
                ep.stage,
                ShaderStage::RayGeneration
                    | ShaderStage::AnyHit
                    | ShaderStage::ClosestHit
                    | ShaderStage::Miss
            )
        }) {
            needed.ray_tracing_pipeline = true;
        }

        if module.global_variables.iter().any(|gv| {
            gv.1.space == crate::AddressSpace::IncomingRayPayload
                || gv.1.space == crate::AddressSpace::RayPayload
        }) {
            needed.ray_tracing_pipeline = true;
        }

        if module.entry_points.iter().any(|ep| {
            matches!(
                ep.stage,
                ShaderStage::RayGeneration
                    | ShaderStage::AnyHit
                    | ShaderStage::ClosestHit
                    | ShaderStage::Miss
            )
        }) {
            needed.ray_tracing_pipeline = true;
        }

        // Write required declarations
        let mut any_written = false;
        if needed.f16 {
            writeln!(self.out, "enable f16;")?;
            any_written = true;
        }
        if needed.dual_source_blending {
            writeln!(self.out, "enable dual_source_blending;")?;
            any_written = true;
        }
        if needed.clip_distances {
            writeln!(self.out, "enable clip_distances;")?;
            any_written = true;
        }
        if module.uses_mesh_shaders() {
            writeln!(self.out, "enable wgpu_mesh_shader;")?;
            any_written = true;
        }
        if needed.draw_index {
            writeln!(self.out, "enable draw_index;")?;
            any_written = true;
        }
        if needed.primitive_index {
            writeln!(self.out, "enable primitive_index;")?;
            any_written = true;
        }
        if needed.cooperative_matrix {
            writeln!(self.out, "enable wgpu_cooperative_matrix;")?;
            any_written = true;
        }
        if needed.ray_tracing_pipeline {
            writeln!(self.out, "enable wgpu_ray_tracing_pipeline;")?;
            any_written = true;
        }
        if any_written {
            // Empty line for readability
            writeln!(self.out)?;
        }

        Ok(())
    }

    /// Helper method used to write
    /// [functions](https://gpuweb.github.io/gpuweb/wgsl/#functions)
    ///
    /// # Notes
    /// Ends in a newline
    fn write_function(
        &mut self,
        module: &Module,
        func: &crate::Function,
        func_ctx: &back::FunctionCtx<'_>,
    ) -> BackendResult {
        let func_name = match func_ctx.ty {
            back::FunctionType::EntryPoint(index) => &self.names[&NameKey::EntryPoint(index)],
            back::FunctionType::Function(handle) => &self.names[&NameKey::Function(handle)],
        };

        // Write function name
        write!(self.out, "fn {func_name}(")?;

        // Write function arguments
        for (index, arg) in func.arguments.iter().enumerate() {
            // Write argument attribute if a binding is present
            if let Some(ref binding) = arg.binding {
                self.write_attributes(&map_binding_to_attribute(binding))?;
            }
            // Write argument name
            let argument_name = &self.names[&func_ctx.argument_key(index as u32)];

            write!(self.out, "{argument_name}: ")?;
            // Write argument type
            self.write_type(module, arg.ty)?;
            if index < func.arguments.len() - 1 {
                // Add a separator between args
                write!(self.out, ", ")?;
            }
        }

        write!(self.out, ")")?;

        // Write function return type
        if let Some(ref result) = func.result {
            write!(self.out, " -> ")?;
            if let Some(ref binding) = result.binding {
                self.write_attributes(&map_binding_to_attribute(binding))?;
            }
            self.write_type(module, result.ty)?;
        }

        write!(self.out, " {{")?;
        writeln!(self.out)?;

        // Write function local variables
        for (handle, local) in func.local_variables.iter() {
            // Write indentation (only for readability)
            write!(self.out, "{}", back::INDENT)?;

            // Write the local name
            // The leading space is important
            write!(self.out, "var {}: ", self.names[&func_ctx.name_key(handle)])?;

            // Write the local type
            self.write_type(module, local.ty)?;

            // Write the local initializer if needed
            if let Some(init) = local.init {
                // Put the equal signal only if there's a initializer
                // The leading and trailing spaces aren't needed but help with readability
                write!(self.out, " = ")?;

                // Write the constant
                // `write_constant` adds no trailing or leading space/newline
                self.write_expr(module, init, func_ctx)?;
            }

            // Finish the local with `;` and add a newline (only for readability)
            writeln!(self.out, ";")?
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

    /// Helper method to write a attribute
    fn write_attributes(&mut self, attributes: &[Attribute]) -> BackendResult {
        for attribute in attributes {
            match *attribute {
                Attribute::Location(id) => write!(self.out, "@location({id}) ")?,
                Attribute::BlendSrc(blend_src) => write!(self.out, "@blend_src({blend_src}) ")?,
                Attribute::BuiltIn(builtin_attrib) => {
                    let builtin = builtin_attrib.to_wgsl_if_implemented()?;
                    write!(self.out, "@builtin({builtin}) ")?;
                }
                Attribute::Stage(shader_stage) => {
                    let stage_str = match shader_stage {
                        ShaderStage::Vertex => "vertex",
                        ShaderStage::Fragment => "fragment",
                        ShaderStage::Compute => "compute",
                        ShaderStage::Task => "task",
                        //Handled by another variant in the Attribute enum, so this code should never be hit.
                        ShaderStage::Mesh => unreachable!(),
                        ShaderStage::RayGeneration => "ray_generation",
                        ShaderStage::AnyHit => "any_hit",
                        ShaderStage::ClosestHit => "closest_hit",
                        ShaderStage::Miss => "miss",
                    };

                    write!(self.out, "@{stage_str} ")?;
                }
                Attribute::WorkGroupSize(size) => {
                    write!(
                        self.out,
                        "@workgroup_size({}, {}, {}) ",
                        size[0], size[1], size[2]
                    )?;
                }
                Attribute::Binding(id) => write!(self.out, "@binding({id}) ")?,
                Attribute::Group(id) => write!(self.out, "@group({id}) ")?,
                Attribute::Invariant => write!(self.out, "@invariant ")?,
                Attribute::Interpolate(interpolation, sampling) => {
                    if sampling.is_some() && sampling != Some(crate::Sampling::Center) {
                        let interpolation = interpolation
                            .unwrap_or(crate::Interpolation::Perspective)
                            .to_wgsl();
                        let sampling = sampling.unwrap_or(crate::Sampling::Center).to_wgsl();
                        write!(self.out, "@interpolate({interpolation}, {sampling}) ")?;
                    } else if interpolation.is_some()
                        && interpolation != Some(crate::Interpolation::Perspective)
                    {
                        let interpolation = interpolation
                            .unwrap_or(crate::Interpolation::Perspective)
                            .to_wgsl();
                        write!(self.out, "@interpolate({interpolation}) ")?;
                    }
                }
                Attribute::MeshStage(ref name) => {
                    write!(self.out, "@mesh({name}) ")?;
                }
                Attribute::TaskPayload(ref payload_name) => {
                    write!(self.out, "@payload({payload_name}) ")?;
                }
                Attribute::PerPrimitive => write!(self.out, "@per_primitive ")?,
                Attribute::IncomingRayPayload(ref payload_name) => {
                    write!(self.out, "@incoming_payload({payload_name}) ")?;
                }
            };
        }
        Ok(())
    }

    /// Helper method used to write structs
    /// Write the full declaration of a struct type.
    ///
    /// Write out a definition of the struct type referred to by
    /// `handle` in `module`. The output will be an instance of the
    /// `struct_decl` production in the WGSL grammar.
    ///
    /// Use `members` as the list of `handle`'s members. (This
    /// function is usually called after matching a `TypeInner`, so
    /// the callers already have the members at hand.)
    fn write_struct(
        &mut self,
        module: &Module,
        handle: Handle<crate::Type>,
        members: &[crate::StructMember],
    ) -> BackendResult {
        write!(self.out, "struct {}", self.names[&NameKey::Type(handle)])?;
        write!(self.out, " {{")?;
        writeln!(self.out)?;
        for (index, member) in members.iter().enumerate() {
            // The indentation is only for readability
            write!(self.out, "{}", back::INDENT)?;
            if let Some(ref binding) = member.binding {
                self.write_attributes(&map_binding_to_attribute(binding))?;
            }
            // Write struct member name and type
            let member_name = &self.names[&NameKey::StructMember(handle, index as u32)];
            write!(self.out, "{member_name}: ")?;
            self.write_type(module, member.ty)?;
            write!(self.out, ",")?;
            writeln!(self.out)?;
        }

        writeln!(self.out, "}}")?;

        Ok(())
    }

    fn write_type(&mut self, module: &Module, ty: Handle<crate::Type>) -> BackendResult {
        // This actually can't be factored out into a nice constructor method,
        // because the borrow checker needs to be able to see that the borrows
        // of `self.names` and `self.out` are disjoint.
        let type_context = WriterTypeContext {
            module,
            names: &self.names,
        };
        type_context.write_type(ty, &mut self.out)?;

        Ok(())
    }

    fn write_type_resolution(
        &mut self,
        module: &Module,
        resolution: &proc::TypeResolution,
    ) -> BackendResult {
        // This actually can't be factored out into a nice constructor method,
        // because the borrow checker needs to be able to see that the borrows
        // of `self.names` and `self.out` are disjoint.
        let type_context = WriterTypeContext {
            module,
            names: &self.names,
        };
        type_context.write_type_resolution(resolution, &mut self.out)?;

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
        use crate::{Expression, Statement};

        match *stmt {
            Statement::Emit(ref range) => {
                for handle in range.clone() {
                    let info = &func_ctx.info[handle];
                    let expr_name = if let Some(name) = func_ctx.named_expressions.get(&handle) {
                        // Front end provides names for all variables at the start of writing.
                        // But we write them to step by step. We need to recache them
                        // Otherwise, we could accidentally write variable name instead of full expression.
                        // Also, we use sanitized names! It defense backend from generating variable with name from reserved keywords.
                        Some(self.namer.call(name))
                    } else {
                        let expr = &func_ctx.expressions[handle];
                        let min_ref_count = expr.bake_ref_count();
                        // Forcefully creating baking expressions in some cases to help with readability
                        let required_baking_expr = match *expr {
                            Expression::ImageLoad { .. }
                            | Expression::ImageQuery { .. }
                            | Expression::ImageSample { .. } => true,
                            _ => false,
                        };
                        if min_ref_count <= info.ref_count || required_baking_expr {
                            Some(Baked(handle).to_string())
                        } else {
                            None
                        }
                    };

                    if let Some(name) = expr_name {
                        write!(self.out, "{level}")?;
                        self.start_named_expr(module, handle, func_ctx, &name)?;
                        self.write_expr(module, handle, func_ctx)?;
                        self.named_expressions.insert(handle, name);
                        writeln!(self.out, ";")?;
                    }
                }
            }
            // TODO: copy-paste from glsl-out
            Statement::If {
                condition,
                ref accept,
                ref reject,
            } => {
                write!(self.out, "{level}")?;
                write!(self.out, "if ")?;
                self.write_expr(module, condition, func_ctx)?;
                writeln!(self.out, " {{")?;

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
            Statement::Return { value } => {
                write!(self.out, "{level}")?;
                write!(self.out, "return")?;
                if let Some(return_value) = value {
                    // The leading space is important
                    write!(self.out, " ")?;
                    self.write_expr(module, return_value, func_ctx)?;
                }
                writeln!(self.out, ";")?;
            }
            // TODO: copy-paste from glsl-out
            Statement::Kill => {
                write!(self.out, "{level}")?;
                writeln!(self.out, "discard;")?
            }
            Statement::Store { pointer, value } => {
                write!(self.out, "{level}")?;

                let is_atomic_pointer = func_ctx
                    .resolve_type(pointer, &module.types)
                    .is_atomic_pointer(&module.types);

                if is_atomic_pointer {
                    write!(self.out, "atomicStore(")?;
                    self.write_expr(module, pointer, func_ctx)?;
                    write!(self.out, ", ")?;
                    self.write_expr(module, value, func_ctx)?;
                    write!(self.out, ")")?;
                } else {
                    self.write_expr_with_indirection(
                        module,
                        pointer,
                        func_ctx,
                        Indirection::Reference,
                    )?;
                    write!(self.out, " = ")?;
                    self.write_expr(module, value, func_ctx)?;
                }
                writeln!(self.out, ";")?
            }
            Statement::Call {
                function,
                ref arguments,
                result,
            } => {
                write!(self.out, "{level}")?;
                if let Some(expr) = result {
                    let name = Baked(expr).to_string();
                    self.start_named_expr(module, expr, func_ctx, &name)?;
                    self.named_expressions.insert(expr, name);
                }
                let func_name = &self.names[&NameKey::Function(function)];
                write!(self.out, "{func_name}(")?;
                for (index, &argument) in arguments.iter().enumerate() {
                    if index != 0 {
                        write!(self.out, ", ")?;
                    }
                    self.write_expr(module, argument, func_ctx)?;
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
                if let Some(result) = result {
                    let res_name = Baked(result).to_string();
                    self.start_named_expr(module, result, func_ctx, &res_name)?;
                    self.named_expressions.insert(result, res_name);
                }

                let fun_str = fun.to_wgsl();
                write!(self.out, "atomic{fun_str}(")?;
                self.write_expr(module, pointer, func_ctx)?;
                if let crate::AtomicFunction::Exchange { compare: Some(cmp) } = *fun {
                    write!(self.out, ", ")?;
                    self.write_expr(module, cmp, func_ctx)?;
                }
                write!(self.out, ", ")?;
                self.write_expr(module, value, func_ctx)?;
                writeln!(self.out, ");")?
            }
            Statement::ImageAtomic {
                image,
                coordinate,
                array_index,
                ref fun,
                value,
            } => {
                write!(self.out, "{level}")?;
                let fun_str = fun.to_wgsl();
                write!(self.out, "textureAtomic{fun_str}(")?;
                self.write_expr(module, image, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, coordinate, func_ctx)?;
                if let Some(array_index_expr) = array_index {
                    write!(self.out, ", ")?;
                    self.write_expr(module, array_index_expr, func_ctx)?;
                }
                write!(self.out, ", ")?;
                self.write_expr(module, value, func_ctx)?;
                writeln!(self.out, ");")?;
            }
            Statement::WorkGroupUniformLoad { pointer, result } => {
                write!(self.out, "{level}")?;
                // TODO: Obey named expressions here.
                let res_name = Baked(result).to_string();
                self.start_named_expr(module, result, func_ctx, &res_name)?;
                self.named_expressions.insert(result, res_name);
                write!(self.out, "workgroupUniformLoad(")?;
                self.write_expr(module, pointer, func_ctx)?;
                writeln!(self.out, ");")?;
            }
            Statement::ImageStore {
                image,
                coordinate,
                array_index,
                value,
            } => {
                write!(self.out, "{level}")?;
                write!(self.out, "textureStore(")?;
                self.write_expr(module, image, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, coordinate, func_ctx)?;
                if let Some(array_index_expr) = array_index {
                    write!(self.out, ", ")?;
                    self.write_expr(module, array_index_expr, func_ctx)?;
                }
                write!(self.out, ", ")?;
                self.write_expr(module, value, func_ctx)?;
                writeln!(self.out, ");")?;
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
            Statement::Switch {
                selector,
                ref cases,
            } => {
                // Start the switch
                write!(self.out, "{level}")?;
                write!(self.out, "switch ")?;
                self.write_expr(module, selector, func_ctx)?;
                writeln!(self.out, " {{")?;

                let l2 = level.next();
                let mut new_case = true;
                for case in cases {
                    if case.fall_through && !case.body.is_empty() {
                        // TODO: we could do the same workaround as we did for the HLSL backend
                        return Err(Error::Unimplemented(
                            "fall-through switch case block".into(),
                        ));
                    }

                    match case.value {
                        crate::SwitchValue::I32(value) => {
                            if new_case {
                                write!(self.out, "{l2}case ")?;
                            }
                            write!(self.out, "{value}")?;
                        }
                        crate::SwitchValue::U32(value) => {
                            if new_case {
                                write!(self.out, "{l2}case ")?;
                            }
                            write!(self.out, "{value}u")?;
                        }
                        crate::SwitchValue::Default => {
                            if new_case {
                                if case.fall_through {
                                    write!(self.out, "{l2}case ")?;
                                } else {
                                    write!(self.out, "{l2}")?;
                                }
                            }
                            write!(self.out, "default")?;
                        }
                    }

                    new_case = !case.fall_through;

                    if case.fall_through {
                        write!(self.out, ", ")?;
                    } else {
                        writeln!(self.out, ": {{")?;
                    }

                    for sta in case.body.iter() {
                        self.write_stmt(module, sta, func_ctx, l2.next())?;
                    }

                    if !case.fall_through {
                        writeln!(self.out, "{l2}}}")?;
                    }
                }

                writeln!(self.out, "{level}}}")?
            }
            Statement::Loop {
                ref body,
                ref continuing,
                break_if,
            } => {
                write!(self.out, "{level}")?;
                writeln!(self.out, "loop {{")?;

                let l2 = level.next();
                for sta in body.iter() {
                    self.write_stmt(module, sta, func_ctx, l2)?;
                }

                // The continuing is optional so we don't need to write it if
                // it is empty, but the `break if` counts as a continuing statement
                // so even if `continuing` is empty we must generate it if a
                // `break if` exists
                if !continuing.is_empty() || break_if.is_some() {
                    writeln!(self.out, "{l2}continuing {{")?;
                    for sta in continuing.iter() {
                        self.write_stmt(module, sta, func_ctx, l2.next())?;
                    }

                    // The `break if` is always the last
                    // statement of the `continuing` block
                    if let Some(condition) = break_if {
                        // The trailing space is important
                        write!(self.out, "{}break if ", l2.next())?;
                        self.write_expr(module, condition, func_ctx)?;
                        // Close the `break if` statement
                        writeln!(self.out, ";")?;
                    }

                    writeln!(self.out, "{l2}}}")?;
                }

                writeln!(self.out, "{level}}}")?
            }
            Statement::Break => {
                writeln!(self.out, "{level}break;")?;
            }
            Statement::Continue => {
                writeln!(self.out, "{level}continue;")?;
            }
            Statement::ControlBarrier(barrier) | Statement::MemoryBarrier(barrier) => {
                if barrier.contains(crate::Barrier::STORAGE) {
                    writeln!(self.out, "{level}storageBarrier();")?;
                }

                if barrier.contains(crate::Barrier::WORK_GROUP) {
                    writeln!(self.out, "{level}workgroupBarrier();")?;
                }

                if barrier.contains(crate::Barrier::SUB_GROUP) {
                    writeln!(self.out, "{level}subgroupBarrier();")?;
                }

                if barrier.contains(crate::Barrier::TEXTURE) {
                    writeln!(self.out, "{level}textureBarrier();")?;
                }
            }
            Statement::RayQuery { .. } => unreachable!(),
            Statement::SubgroupBallot { result, predicate } => {
                write!(self.out, "{level}")?;
                let res_name = Baked(result).to_string();
                self.start_named_expr(module, result, func_ctx, &res_name)?;
                self.named_expressions.insert(result, res_name);

                write!(self.out, "subgroupBallot(")?;
                if let Some(predicate) = predicate {
                    self.write_expr(module, predicate, func_ctx)?;
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
                let res_name = Baked(result).to_string();
                self.start_named_expr(module, result, func_ctx, &res_name)?;
                self.named_expressions.insert(result, res_name);

                match (collective_op, op) {
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::All) => {
                        write!(self.out, "subgroupAll(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Any) => {
                        write!(self.out, "subgroupAny(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Add) => {
                        write!(self.out, "subgroupAdd(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Mul) => {
                        write!(self.out, "subgroupMul(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Max) => {
                        write!(self.out, "subgroupMax(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Min) => {
                        write!(self.out, "subgroupMin(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::And) => {
                        write!(self.out, "subgroupAnd(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Or) => {
                        write!(self.out, "subgroupOr(")?
                    }
                    (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Xor) => {
                        write!(self.out, "subgroupXor(")?
                    }
                    (crate::CollectiveOperation::ExclusiveScan, crate::SubgroupOperation::Add) => {
                        write!(self.out, "subgroupExclusiveAdd(")?
                    }
                    (crate::CollectiveOperation::ExclusiveScan, crate::SubgroupOperation::Mul) => {
                        write!(self.out, "subgroupExclusiveMul(")?
                    }
                    (crate::CollectiveOperation::InclusiveScan, crate::SubgroupOperation::Add) => {
                        write!(self.out, "subgroupInclusiveAdd(")?
                    }
                    (crate::CollectiveOperation::InclusiveScan, crate::SubgroupOperation::Mul) => {
                        write!(self.out, "subgroupInclusiveMul(")?
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
                let res_name = Baked(result).to_string();
                self.start_named_expr(module, result, func_ctx, &res_name)?;
                self.named_expressions.insert(result, res_name);

                match mode {
                    crate::GatherMode::BroadcastFirst => {
                        write!(self.out, "subgroupBroadcastFirst(")?;
                    }
                    crate::GatherMode::Broadcast(_) => {
                        write!(self.out, "subgroupBroadcast(")?;
                    }
                    crate::GatherMode::Shuffle(_) => {
                        write!(self.out, "subgroupShuffle(")?;
                    }
                    crate::GatherMode::ShuffleDown(_) => {
                        write!(self.out, "subgroupShuffleDown(")?;
                    }
                    crate::GatherMode::ShuffleUp(_) => {
                        write!(self.out, "subgroupShuffleUp(")?;
                    }
                    crate::GatherMode::ShuffleXor(_) => {
                        write!(self.out, "subgroupShuffleXor(")?;
                    }
                    crate::GatherMode::QuadBroadcast(_) => {
                        write!(self.out, "quadBroadcast(")?;
                    }
                    crate::GatherMode::QuadSwap(direction) => match direction {
                        crate::Direction::X => {
                            write!(self.out, "quadSwapX(")?;
                        }
                        crate::Direction::Y => {
                            write!(self.out, "quadSwapY(")?;
                        }
                        crate::Direction::Diagonal => {
                            write!(self.out, "quadSwapDiagonal(")?;
                        }
                    },
                }
                self.write_expr(module, argument, func_ctx)?;
                match mode {
                    crate::GatherMode::BroadcastFirst => {}
                    crate::GatherMode::Broadcast(index)
                    | crate::GatherMode::Shuffle(index)
                    | crate::GatherMode::ShuffleDown(index)
                    | crate::GatherMode::ShuffleUp(index)
                    | crate::GatherMode::ShuffleXor(index)
                    | crate::GatherMode::QuadBroadcast(index) => {
                        write!(self.out, ", ")?;
                        self.write_expr(module, index, func_ctx)?;
                    }
                    crate::GatherMode::QuadSwap(_) => {}
                }
                writeln!(self.out, ");")?;
            }
            Statement::CooperativeStore { target, ref data } => {
                let suffix = if data.row_major { "T" } else { "" };
                write!(self.out, "{level}coopStore{suffix}(")?;
                self.write_expr(module, target, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, data.pointer, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, data.stride, func_ctx)?;
                writeln!(self.out, ");")?
            }
            Statement::RayPipelineFunction(fun) => match fun {
                crate::RayPipelineFunction::TraceRay {
                    acceleration_structure,
                    descriptor,
                    payload,
                } => {
                    write!(self.out, "{level}traceRay(")?;
                    self.write_expr(module, acceleration_structure, func_ctx)?;
                    write!(self.out, ", ")?;
                    self.write_expr(module, descriptor, func_ctx)?;
                    write!(self.out, ", ")?;
                    self.write_expr(module, payload, func_ctx)?;
                    writeln!(self.out, ");")?
                }
            },
        }

        Ok(())
    }

    /// Return the sort of indirection that `expr`'s plain form evaluates to.
    ///
    /// An expression's 'plain form' is the most general rendition of that
    /// expression into WGSL, lacking `&` or `*` operators:
    ///
    /// - The plain form of `LocalVariable(x)` is simply `x`, which is a reference
    ///   to the local variable's storage.
    ///
    /// - The plain form of `GlobalVariable(g)` is simply `g`, which is usually a
    ///   reference to the global variable's storage. However, globals in the
    ///   `Handle` address space are immutable, and `GlobalVariable` expressions for
    ///   those produce the value directly, not a pointer to it. Such
    ///   `GlobalVariable` expressions are `Ordinary`.
    ///
    /// - `Access` and `AccessIndex` are `Reference` when their `base` operand is a
    ///   pointer. If they are applied directly to a composite value, they are
    ///   `Ordinary`.
    ///
    /// Note that `FunctionArgument` expressions are never `Reference`, even when
    /// the argument's type is `Pointer`. `FunctionArgument` always evaluates to the
    /// argument's value directly, so any pointer it produces is merely the value
    /// passed by the caller.
    fn plain_form_indirection(
        &self,
        expr: Handle<crate::Expression>,
        module: &Module,
        func_ctx: &back::FunctionCtx<'_>,
    ) -> Indirection {
        use crate::Expression as Ex;

        // Named expressions are `let` expressions, which apply the Load Rule,
        // so if their type is a Naga pointer, then that must be a WGSL pointer
        // as well.
        if self.named_expressions.contains_key(&expr) {
            return Indirection::Ordinary;
        }

        match func_ctx.expressions[expr] {
            Ex::LocalVariable(_) => Indirection::Reference,
            Ex::GlobalVariable(handle) => {
                let global = &module.global_variables[handle];
                match global.space {
                    crate::AddressSpace::Handle => Indirection::Ordinary,
                    _ => Indirection::Reference,
                }
            }
            Ex::Access { base, .. } | Ex::AccessIndex { base, .. } => {
                let base_ty = func_ctx.resolve_type(base, &module.types);
                match *base_ty {
                    TypeInner::Pointer { .. } | TypeInner::ValuePointer { .. } => {
                        Indirection::Reference
                    }
                    _ => Indirection::Ordinary,
                }
            }
            _ => Indirection::Ordinary,
        }
    }

    fn start_named_expr(
        &mut self,
        module: &Module,
        handle: Handle<crate::Expression>,
        func_ctx: &back::FunctionCtx,
        name: &str,
    ) -> BackendResult {
        // Write variable name
        write!(self.out, "let {name}")?;
        if self.flags.contains(WriterFlags::EXPLICIT_TYPES) {
            write!(self.out, ": ")?;
            // Write variable type
            self.write_type_resolution(module, &func_ctx.info[handle].ty)?;
        }

        write!(self.out, " = ")?;
        Ok(())
    }

    /// Write the ordinary WGSL form of `expr`.
    ///
    /// See `write_expr_with_indirection` for details.
    fn write_expr(
        &mut self,
        module: &Module,
        expr: Handle<crate::Expression>,
        func_ctx: &back::FunctionCtx<'_>,
    ) -> BackendResult {
        self.write_expr_with_indirection(module, expr, func_ctx, Indirection::Ordinary)
    }

    /// Write `expr` as a WGSL expression with the requested indirection.
    ///
    /// In terms of the WGSL grammar, the resulting expression is a
    /// `singular_expression`. It may be parenthesized. This makes it suitable
    /// for use as the operand of a unary or binary operator without worrying
    /// about precedence.
    ///
    /// This does not produce newlines or indentation.
    ///
    /// The `requested` argument indicates (roughly) whether Naga
    /// `Pointer`-valued expressions represent WGSL references or pointers. See
    /// `Indirection` for details.
    fn write_expr_with_indirection(
        &mut self,
        module: &Module,
        expr: Handle<crate::Expression>,
        func_ctx: &back::FunctionCtx<'_>,
        requested: Indirection,
    ) -> BackendResult {
        // If the plain form of the expression is not what we need, emit the
        // operator necessary to correct that.
        let plain = self.plain_form_indirection(expr, module, func_ctx);
        log::trace!(
            "expression {:?}={:?} is {:?}, expected {:?}",
            expr,
            func_ctx.expressions[expr],
            plain,
            requested,
        );
        match (requested, plain) {
            (Indirection::Ordinary, Indirection::Reference) => {
                write!(self.out, "(&")?;
                self.write_expr_plain_form(module, expr, func_ctx, plain)?;
                write!(self.out, ")")?;
            }
            (Indirection::Reference, Indirection::Ordinary) => {
                write!(self.out, "(*")?;
                self.write_expr_plain_form(module, expr, func_ctx, plain)?;
                write!(self.out, ")")?;
            }
            (_, _) => self.write_expr_plain_form(module, expr, func_ctx, plain)?,
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
            Expression::Literal(literal) => match literal {
                crate::Literal::F16(value) => write!(self.out, "{value}h")?,
                crate::Literal::F32(value) => write!(self.out, "{value}f")?,
                crate::Literal::U32(value) => write!(self.out, "{value}u")?,
                crate::Literal::I32(value) => {
                    // `-2147483648i` is not valid WGSL. The most negative `i32`
                    // value can only be expressed in WGSL using AbstractInt and
                    // a unary negation operator.
                    if value == i32::MIN {
                        write!(self.out, "i32({value})")?;
                    } else {
                        write!(self.out, "{value}i")?;
                    }
                }
                crate::Literal::Bool(value) => write!(self.out, "{value}")?,
                crate::Literal::F64(value) => write!(self.out, "{value:?}lf")?,
                crate::Literal::I64(value) => {
                    // `-9223372036854775808li` is not valid WGSL. Nor can we simply use the
                    // AbstractInt trick above, as AbstractInt also cannot represent
                    // `9223372036854775808`. Instead construct the second most negative
                    // AbstractInt, subtract one from it, then cast to i64.
                    if value == i64::MIN {
                        write!(self.out, "i64({} - 1)", value + 1)?;
                    } else {
                        write!(self.out, "{value}li")?;
                    }
                }
                crate::Literal::U64(value) => write!(self.out, "{value:?}lu")?,
                crate::Literal::AbstractInt(_) | crate::Literal::AbstractFloat(_) => {
                    return Err(Error::Custom(
                        "Abstract types should not appear in IR presented to backends".into(),
                    ));
                }
            },
            Expression::Constant(handle) => {
                let constant = &module.constants[handle];
                if constant.name.is_some() {
                    write!(self.out, "{}", self.names[&NameKey::Constant(handle)])?;
                } else {
                    self.write_const_expression(module, constant.init, &module.global_expressions)?;
                }
            }
            Expression::ZeroValue(ty) => {
                self.write_type(module, ty)?;
                write!(self.out, "()")?;
            }
            Expression::Compose { ty, ref components } => {
                self.write_type(module, ty)?;
                write!(self.out, "(")?;
                for (index, component) in components.iter().enumerate() {
                    if index != 0 {
                        write!(self.out, ", ")?;
                    }
                    write_expression(self, *component)?;
                }
                write!(self.out, ")")?
            }
            Expression::Splat { size, value } => {
                let size = common::vector_size_str(size);
                write!(self.out, "vec{size}(")?;
                write_expression(self, value)?;
                write!(self.out, ")")?;
            }
            Expression::Override(handle) => {
                write!(self.out, "{}", self.names[&NameKey::Override(handle)])?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    /// Write the 'plain form' of `expr`.
    ///
    /// An expression's 'plain form' is the most general rendition of that
    /// expression into WGSL, lacking `&` or `*` operators. The plain forms of
    /// `LocalVariable(x)` and `GlobalVariable(g)` are simply `x` and `g`. Such
    /// Naga expressions represent both WGSL pointers and references; it's the
    /// caller's responsibility to distinguish those cases appropriately.
    fn write_expr_plain_form(
        &mut self,
        module: &Module,
        expr: Handle<crate::Expression>,
        func_ctx: &back::FunctionCtx<'_>,
        indirection: Indirection,
    ) -> BackendResult {
        use crate::Expression;

        if let Some(name) = self.named_expressions.get(&expr) {
            write!(self.out, "{name}")?;
            return Ok(());
        }

        let expression = &func_ctx.expressions[expr];

        // Write the plain WGSL form of a Naga expression.
        //
        // The plain form of `LocalVariable` and `GlobalVariable` expressions is
        // simply the variable name; `*` and `&` operators are never emitted.
        //
        // The plain form of `Access` and `AccessIndex` expressions are WGSL
        // `postfix_expression` forms for member/component access and
        // subscripting.
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
            Expression::Override(handle) => {
                write!(self.out, "{}", self.names[&NameKey::Override(handle)])?;
            }
            Expression::FunctionArgument(pos) => {
                let name_key = func_ctx.argument_key(pos);
                let name = &self.names[&name_key];
                write!(self.out, "{name}")?;
            }
            Expression::Binary { op, left, right } => {
                write!(self.out, "(")?;
                self.write_expr(module, left, func_ctx)?;
                write!(self.out, " {} ", back::binary_operation_str(op))?;
                self.write_expr(module, right, func_ctx)?;
                write!(self.out, ")")?;
            }
            Expression::Access { base, index } => {
                self.write_expr_with_indirection(module, base, func_ctx, indirection)?;
                write!(self.out, "[")?;
                self.write_expr(module, index, func_ctx)?;
                write!(self.out, "]")?
            }
            Expression::AccessIndex { base, index } => {
                let base_ty_res = &func_ctx.info[base].ty;
                let mut resolved = base_ty_res.inner_with(&module.types);

                self.write_expr_with_indirection(module, base, func_ctx, indirection)?;

                let base_ty_handle = match *resolved {
                    TypeInner::Pointer { base, space: _ } => {
                        resolved = &module.types[base].inner;
                        Some(base)
                    }
                    _ => base_ty_res.handle(),
                };

                match *resolved {
                    TypeInner::Vector { .. } => {
                        // Write vector access as a swizzle
                        write!(self.out, ".{}", back::COMPONENTS[index as usize])?
                    }
                    TypeInner::Matrix { .. }
                    | TypeInner::Array { .. }
                    | TypeInner::BindingArray { .. }
                    | TypeInner::ValuePointer { .. } => write!(self.out, "[{index}]")?,
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
            }
            Expression::ImageSample {
                image,
                sampler,
                gather: None,
                coordinate,
                array_index,
                offset,
                level,
                depth_ref,
                clamp_to_edge,
            } => {
                use crate::SampleLevel as Sl;

                let suffix_cmp = match depth_ref {
                    Some(_) => "Compare",
                    None => "",
                };
                let suffix_level = match level {
                    Sl::Auto => "",
                    Sl::Zero if clamp_to_edge => "BaseClampToEdge",
                    Sl::Zero | Sl::Exact(_) => "Level",
                    Sl::Bias(_) => "Bias",
                    Sl::Gradient { .. } => "Grad",
                };

                write!(self.out, "textureSample{suffix_cmp}{suffix_level}(")?;
                self.write_expr(module, image, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, sampler, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, coordinate, func_ctx)?;

                if let Some(array_index) = array_index {
                    write!(self.out, ", ")?;
                    self.write_expr(module, array_index, func_ctx)?;
                }

                if let Some(depth_ref) = depth_ref {
                    write!(self.out, ", ")?;
                    self.write_expr(module, depth_ref, func_ctx)?;
                }

                match level {
                    Sl::Auto => {}
                    Sl::Zero => {
                        // Level 0 is implied for depth comparison and BaseClampToEdge
                        if depth_ref.is_none() && !clamp_to_edge {
                            write!(self.out, ", 0.0")?;
                        }
                    }
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
                    self.write_const_expression(module, offset, func_ctx.expressions)?;
                }

                write!(self.out, ")")?;
            }

            Expression::ImageSample {
                image,
                sampler,
                gather: Some(component),
                coordinate,
                array_index,
                offset,
                level: _,
                depth_ref,
                clamp_to_edge: _,
            } => {
                let suffix_cmp = match depth_ref {
                    Some(_) => "Compare",
                    None => "",
                };

                write!(self.out, "textureGather{suffix_cmp}(")?;
                match *func_ctx.resolve_type(image, &module.types) {
                    TypeInner::Image {
                        class: crate::ImageClass::Depth { multi: _ },
                        ..
                    } => {}
                    _ => {
                        write!(self.out, "{}, ", component as u8)?;
                    }
                }
                self.write_expr(module, image, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, sampler, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, coordinate, func_ctx)?;

                if let Some(array_index) = array_index {
                    write!(self.out, ", ")?;
                    self.write_expr(module, array_index, func_ctx)?;
                }

                if let Some(depth_ref) = depth_ref {
                    write!(self.out, ", ")?;
                    self.write_expr(module, depth_ref, func_ctx)?;
                }

                if let Some(offset) = offset {
                    write!(self.out, ", ")?;
                    self.write_const_expression(module, offset, func_ctx.expressions)?;
                }

                write!(self.out, ")")?;
            }
            Expression::ImageQuery { image, query } => {
                use crate::ImageQuery as Iq;

                let texture_function = match query {
                    Iq::Size { .. } => "textureDimensions",
                    Iq::NumLevels => "textureNumLevels",
                    Iq::NumLayers => "textureNumLayers",
                    Iq::NumSamples => "textureNumSamples",
                };

                write!(self.out, "{texture_function}(")?;
                self.write_expr(module, image, func_ctx)?;
                if let Iq::Size { level: Some(level) } = query {
                    write!(self.out, ", ")?;
                    self.write_expr(module, level, func_ctx)?;
                };
                write!(self.out, ")")?;
            }

            Expression::ImageLoad {
                image,
                coordinate,
                array_index,
                sample,
                level,
            } => {
                write!(self.out, "textureLoad(")?;
                self.write_expr(module, image, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, coordinate, func_ctx)?;
                if let Some(array_index) = array_index {
                    write!(self.out, ", ")?;
                    self.write_expr(module, array_index, func_ctx)?;
                }
                if let Some(index) = sample.or(level) {
                    write!(self.out, ", ")?;
                    self.write_expr(module, index, func_ctx)?;
                }
                write!(self.out, ")")?;
            }
            Expression::GlobalVariable(handle) => {
                let name = &self.names[&NameKey::GlobalVariable(handle)];
                write!(self.out, "{name}")?;
            }

            Expression::As {
                expr,
                kind,
                convert,
            } => {
                let inner = func_ctx.resolve_type(expr, &module.types);
                match *inner {
                    TypeInner::Matrix {
                        columns,
                        rows,
                        scalar,
                    } => {
                        let scalar = crate::Scalar {
                            kind,
                            width: convert.unwrap_or(scalar.width),
                        };
                        let scalar_kind_str = scalar.to_wgsl_if_implemented()?;
                        write!(
                            self.out,
                            "mat{}x{}<{}>",
                            common::vector_size_str(columns),
                            common::vector_size_str(rows),
                            scalar_kind_str
                        )?;
                    }
                    TypeInner::Vector {
                        size,
                        scalar: crate::Scalar { width, .. },
                    } => {
                        let scalar = crate::Scalar {
                            kind,
                            width: convert.unwrap_or(width),
                        };
                        let vector_size_str = common::vector_size_str(size);
                        let scalar_kind_str = scalar.to_wgsl_if_implemented()?;
                        if convert.is_some() {
                            write!(self.out, "vec{vector_size_str}<{scalar_kind_str}>")?;
                        } else {
                            write!(self.out, "bitcast<vec{vector_size_str}<{scalar_kind_str}>>")?;
                        }
                    }
                    TypeInner::Scalar(crate::Scalar { width, .. }) => {
                        let scalar = crate::Scalar {
                            kind,
                            width: convert.unwrap_or(width),
                        };
                        let scalar_kind_str = scalar.to_wgsl_if_implemented()?;
                        if convert.is_some() {
                            write!(self.out, "{scalar_kind_str}")?
                        } else {
                            write!(self.out, "bitcast<{scalar_kind_str}>")?
                        }
                    }
                    _ => {
                        return Err(Error::Unimplemented(format!(
                            "write_expr expression::as {inner:?}"
                        )));
                    }
                };
                write!(self.out, "(")?;
                self.write_expr(module, expr, func_ctx)?;
                write!(self.out, ")")?;
            }
            Expression::Load { pointer } => {
                let is_atomic_pointer = func_ctx
                    .resolve_type(pointer, &module.types)
                    .is_atomic_pointer(&module.types);

                if is_atomic_pointer {
                    write!(self.out, "atomicLoad(")?;
                    self.write_expr(module, pointer, func_ctx)?;
                    write!(self.out, ")")?;
                } else {
                    self.write_expr_with_indirection(
                        module,
                        pointer,
                        func_ctx,
                        Indirection::Reference,
                    )?;
                }
            }
            Expression::LocalVariable(handle) => {
                write!(self.out, "{}", self.names[&func_ctx.name_key(handle)])?
            }
            Expression::ArrayLength(expr) => {
                write!(self.out, "arrayLength(")?;
                self.write_expr(module, expr, func_ctx)?;
                write!(self.out, ")")?;
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
                    Regular(&'static str),
                    InversePolyfill(InversePolyfill),
                }

                let function = match fun.try_to_wgsl() {
                    Some(name) => Function::Regular(name),
                    None => match fun {
                        Mf::Inverse => {
                            let ty = func_ctx.resolve_type(arg, &module.types);
                            let Some(overload) = InversePolyfill::find_overload(ty) else {
                                return Err(Error::unsupported("math function", fun));
                            };

                            Function::InversePolyfill(overload)
                        }
                        _ => return Err(Error::unsupported("math function", fun)),
                    },
                };

                match function {
                    Function::Regular(fun_name) => {
                        write!(self.out, "{fun_name}(")?;
                        self.write_expr(module, arg, func_ctx)?;
                        for arg in IntoIterator::into_iter([arg1, arg2, arg3]).flatten() {
                            write!(self.out, ", ")?;
                            self.write_expr(module, arg, func_ctx)?;
                        }
                        write!(self.out, ")")?
                    }
                    Function::InversePolyfill(inverse) => {
                        write!(self.out, "{}(", inverse.fun_name)?;
                        self.write_expr(module, arg, func_ctx)?;
                        write!(self.out, ")")?;
                        self.required_polyfills.insert(inverse);
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
            Expression::Unary { op, expr } => {
                let unary = match op {
                    crate::UnaryOperator::Negate => "-",
                    crate::UnaryOperator::LogicalNot => "!",
                    crate::UnaryOperator::BitwiseNot => "~",
                };

                write!(self.out, "{unary}(")?;
                self.write_expr(module, expr, func_ctx)?;

                write!(self.out, ")")?
            }

            Expression::Select {
                condition,
                accept,
                reject,
            } => {
                write!(self.out, "select(")?;
                self.write_expr(module, reject, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, accept, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, condition, func_ctx)?;
                write!(self.out, ")")?
            }
            Expression::Derivative { axis, ctrl, expr } => {
                use crate::{DerivativeAxis as Axis, DerivativeControl as Ctrl};
                let op = match (axis, ctrl) {
                    (Axis::X, Ctrl::Coarse) => "dpdxCoarse",
                    (Axis::X, Ctrl::Fine) => "dpdxFine",
                    (Axis::X, Ctrl::None) => "dpdx",
                    (Axis::Y, Ctrl::Coarse) => "dpdyCoarse",
                    (Axis::Y, Ctrl::Fine) => "dpdyFine",
                    (Axis::Y, Ctrl::None) => "dpdy",
                    (Axis::Width, Ctrl::Coarse) => "fwidthCoarse",
                    (Axis::Width, Ctrl::Fine) => "fwidthFine",
                    (Axis::Width, Ctrl::None) => "fwidth",
                };
                write!(self.out, "{op}(")?;
                self.write_expr(module, expr, func_ctx)?;
                write!(self.out, ")")?
            }
            Expression::Relational { fun, argument } => {
                use crate::RelationalFunction as Rf;

                let fun_name = match fun {
                    Rf::All => "all",
                    Rf::Any => "any",
                    _ => return Err(Error::UnsupportedRelationalFunction(fun)),
                };
                write!(self.out, "{fun_name}(")?;

                self.write_expr(module, argument, func_ctx)?;

                write!(self.out, ")")?
            }
            // Not supported yet
            Expression::RayQueryGetIntersection { .. }
            | Expression::RayQueryVertexPositions { .. } => unreachable!(),
            // Nothing to do here, since call expression already cached
            Expression::CallResult(_)
            | Expression::AtomicResult { .. }
            | Expression::RayQueryProceedResult
            | Expression::SubgroupBallotResult
            | Expression::SubgroupOperationResult { .. }
            | Expression::WorkGroupUniformLoadResult { .. } => {}
            Expression::CooperativeLoad {
                columns,
                rows,
                role,
                ref data,
            } => {
                let suffix = if data.row_major { "T" } else { "" };
                let scalar = func_ctx.info[data.pointer]
                    .ty
                    .inner_with(&module.types)
                    .pointer_base_type()
                    .unwrap()
                    .inner_with(&module.types)
                    .scalar()
                    .unwrap();
                write!(
                    self.out,
                    "coopLoad{suffix}<coop_mat{}x{}<{},{:?}>>(",
                    columns as u32,
                    rows as u32,
                    scalar.try_to_wgsl().unwrap(),
                    role,
                )?;
                self.write_expr(module, data.pointer, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, data.stride, func_ctx)?;
                write!(self.out, ")")?;
            }
            Expression::CooperativeMultiplyAdd { a, b, c } => {
                write!(self.out, "coopMultiplyAdd(")?;
                self.write_expr(module, a, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, b, func_ctx)?;
                write!(self.out, ", ")?;
                self.write_expr(module, c, func_ctx)?;
                write!(self.out, ")")?;
            }
        }

        Ok(())
    }

    /// Helper method used to write global variables
    /// # Notes
    /// Always adds a newline
    fn write_global(
        &mut self,
        module: &Module,
        global: &crate::GlobalVariable,
        handle: Handle<crate::GlobalVariable>,
    ) -> BackendResult {
        // Write group and binding attributes if present
        if let Some(ref binding) = global.binding {
            self.write_attributes(&[
                Attribute::Group(binding.group),
                Attribute::Binding(binding.binding),
            ])?;
            writeln!(self.out)?;
        }

        if global
            .memory_decorations
            .contains(crate::MemoryDecorations::COHERENT)
        {
            write!(self.out, "@coherent ")?;
        }
        if global
            .memory_decorations
            .contains(crate::MemoryDecorations::VOLATILE)
        {
            write!(self.out, "@volatile ")?;
        }

        // First write global name and address space if supported
        write!(self.out, "var")?;
        let (address, maybe_access) = address_space_str(global.space);
        if let Some(space) = address {
            write!(self.out, "<{space}")?;
            if let Some(access) = maybe_access {
                write!(self.out, ", {access}")?;
            }
            write!(self.out, ">")?;
        }
        write!(
            self.out,
            " {}: ",
            &self.names[&NameKey::GlobalVariable(handle)]
        )?;

        // Write global type
        self.write_type(module, global.ty)?;

        // Write initializer
        if let Some(init) = global.init {
            write!(self.out, " = ")?;
            self.write_const_expression(module, init, &module.global_expressions)?;
        }

        // End with semicolon
        writeln!(self.out, ";")?;

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
        let name = &self.names[&NameKey::Constant(handle)];
        // First write only constant name
        write!(self.out, "const {name}: ")?;
        self.write_type(module, module.constants[handle].ty)?;
        write!(self.out, " = ")?;
        let init = module.constants[handle].init;
        self.write_const_expression(module, init, &module.global_expressions)?;
        writeln!(self.out, ";")?;

        Ok(())
    }

    /// Helper method used to write overrides
    ///
    /// # Notes
    /// Ends in a newline
    fn write_override(
        &mut self,
        module: &Module,
        handle: Handle<crate::Override>,
    ) -> BackendResult {
        let override_ = &module.overrides[handle];
        let name = &self.names[&NameKey::Override(handle)];

        // Write @id attribute if present
        if let Some(id) = override_.id {
            write!(self.out, "@id({id}) ")?;
        }

        // Write override declaration
        write!(self.out, "override {name}: ")?;
        self.write_type(module, override_.ty)?;

        // Write initializer if present
        if let Some(init) = override_.init {
            write!(self.out, " = ")?;
            self.write_const_expression(module, init, &module.global_expressions)?;
        }

        writeln!(self.out, ";")?;

        Ok(())
    }

    // See https://github.com/rust-lang/rust-clippy/issues/4979.
    pub fn finish(self) -> W {
        self.out
    }
}

struct WriterTypeContext<'m> {
    module: &'m Module,
    names: &'m crate::FastHashMap<NameKey, String>,
}

impl TypeContext for WriterTypeContext<'_> {
    fn lookup_type(&self, handle: Handle<crate::Type>) -> &crate::Type {
        &self.module.types[handle]
    }

    fn type_name(&self, handle: Handle<crate::Type>) -> &str {
        self.names[&NameKey::Type(handle)].as_str()
    }

    fn write_unnamed_struct<W: Write>(&self, _: &TypeInner, _: &mut W) -> core::fmt::Result {
        unreachable!("the WGSL back end should always provide type handles");
    }

    fn write_override<W: Write>(
        &self,
        handle: Handle<crate::Override>,
        out: &mut W,
    ) -> core::fmt::Result {
        write!(out, "{}", self.names[&NameKey::Override(handle)])
    }

    fn write_non_wgsl_inner<W: Write>(&self, _: &TypeInner, _: &mut W) -> core::fmt::Result {
        unreachable!("backends should only be passed validated modules");
    }

    fn write_non_wgsl_scalar<W: Write>(&self, _: crate::Scalar, _: &mut W) -> core::fmt::Result {
        unreachable!("backends should only be passed validated modules");
    }
}

fn map_binding_to_attribute(binding: &crate::Binding) -> Vec<Attribute> {
    match *binding {
        crate::Binding::BuiltIn(built_in) => {
            if let crate::BuiltIn::Position { invariant: true } = built_in {
                vec![Attribute::BuiltIn(built_in), Attribute::Invariant]
            } else {
                vec![Attribute::BuiltIn(built_in)]
            }
        }
        crate::Binding::Location {
            location,
            interpolation,
            sampling,
            blend_src: None,
            per_primitive,
        } => {
            let mut attrs = vec![
                Attribute::Location(location),
                Attribute::Interpolate(interpolation, sampling),
            ];
            if per_primitive {
                attrs.push(Attribute::PerPrimitive);
            }
            attrs
        }
        crate::Binding::Location {
            location,
            interpolation,
            sampling,
            blend_src: Some(blend_src),
            per_primitive,
        } => {
            let mut attrs = vec![
                Attribute::Location(location),
                Attribute::BlendSrc(blend_src),
                Attribute::Interpolate(interpolation, sampling),
            ];
            if per_primitive {
                attrs.push(Attribute::PerPrimitive);
            }
            attrs
        }
    }
}
