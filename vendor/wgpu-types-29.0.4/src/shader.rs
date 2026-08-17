use alloc::borrow::Cow;

/// Describes how shader bound checks should be performed.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShaderRuntimeChecks {
    /// Enforce bounds checks in shaders, even if the underlying driver doesn't
    /// support doing so natively.
    ///
    /// When this is `true`, `wgpu` promises that shaders can only read or
    /// write the accessible region of a bindgroup's buffer bindings. If
    /// the underlying graphics platform cannot implement these bounds checks
    /// itself, `wgpu` will inject bounds checks before presenting the
    /// shader to the platform.
    ///
    /// When this is `false`, `wgpu` only enforces such bounds checks if the
    /// underlying platform provides a way to do so itself. `wgpu` does not
    /// itself add any bounds checks to generated shader code.
    ///
    /// Note that `wgpu` users may try to initialize only those portions of
    /// buffers that they anticipate might be read from. Passing `false` here
    /// may allow shaders to see wider regions of the buffers than expected,
    /// making such deferred initialization visible to the application.
    pub bounds_checks: bool,
    ///
    /// If false, the caller MUST ensure that all passed shaders do not contain any infinite loops.
    ///
    /// If it does, backend compilers MAY treat such a loop as unreachable code and draw
    /// conclusions about other safety-critical code paths. This option SHOULD NOT be disabled
    /// when running untrusted code.
    pub force_loop_bounding: bool,
    /// If false, the caller **MUST** ensure that in all passed shaders every function operating
    /// on a ray query must obey these rules (functions using wgsl naming)
    /// - `rayQueryInitialize` must have called before `rayQueryProceed`
    /// - `rayQueryProceed` must have been called, returned true and have hit an AABB before
    ///   `rayQueryGenerateIntersection` is called
    /// - `rayQueryProceed` must have been called, returned true and have hit a triangle before
    ///   `rayQueryConfirmIntersection` is called
    /// - `rayQueryProceed` must have been called and have returned true before `rayQueryTerminate`,
    ///   `getCandidateHitVertexPositions` or `rayQueryGetCandidateIntersection` is called
    /// - `rayQueryProceed` must have been called and have returned false before `rayQueryGetCommittedIntersection`
    ///   or `getCommittedHitVertexPositions` are called
    ///
    /// It is the aim that these cases will not cause UB if this is set to true, but currently this will still happen on DX12 and Metal.
    pub ray_query_initialization_tracking: bool,

    /// If false, task shaders will not validate that the mesh shader grid they dispatch is within legal limits.
    pub task_shader_dispatch_tracking: bool,

    /// If false, mesh shaders won't clamp the output primitives' vertex indices, which can lead to
    /// undefined behavior and arbitrary memory access.
    pub mesh_shader_primitive_indices_clamp: bool,
}

impl ShaderRuntimeChecks {
    /// Creates a new configuration where the shader is fully checked.
    #[must_use]
    pub const fn checked() -> Self {
        unsafe { Self::all(true) }
    }

    /// Creates a new configuration where none of the checks are performed.
    ///
    /// # Safety
    ///
    /// See the documentation for the `set_*` methods for the safety requirements
    /// of each sub-configuration.
    #[must_use]
    pub const fn unchecked() -> Self {
        unsafe { Self::all(false) }
    }

    /// Creates a new configuration where all checks are enabled or disabled. To safely
    /// create a configuration with all checks enabled, use [`ShaderRuntimeChecks::checked`].
    ///
    /// # Safety
    ///
    /// See the documentation for the `set_*` methods for the safety requirements
    /// of each sub-configuration.
    #[must_use]
    pub const unsafe fn all(all_checks: bool) -> Self {
        Self {
            bounds_checks: all_checks,
            force_loop_bounding: all_checks,
            ray_query_initialization_tracking: all_checks,
            task_shader_dispatch_tracking: all_checks,
            mesh_shader_primitive_indices_clamp: all_checks,
        }
    }
}

impl Default for ShaderRuntimeChecks {
    fn default() -> Self {
        Self::checked()
    }
}

/// Descriptor for a shader module given by any of several sources.
/// These shaders are passed through directly to the underlying api.
/// At least one shader type that may be used by the backend must be `Some` or a panic is raised.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateShaderModuleDescriptorPassthrough<'a, L> {
    /// Debug label of the shader module. This will show up in graphics debuggers for easy identification.
    pub label: L,
    /// Number of workgroups in each dimension x, y and z. Unused for Spir-V.
    pub num_workgroups: (u32, u32, u32),

    /// Binary SPIR-V data, in 4-byte words.
    pub spirv: Option<Cow<'a, [u32]>>,
    /// Shader DXIL source.
    pub dxil: Option<Cow<'a, [u8]>>,
    /// Shader HLSL source.
    pub hlsl: Option<Cow<'a, str>>,
    /// Shader MetalLib source.
    pub metallib: Option<Cow<'a, [u8]>>,
    /// Shader MSL source.
    pub msl: Option<Cow<'a, str>>,
    /// Shader GLSL source (currently unused).
    pub glsl: Option<Cow<'a, str>>,
    /// Shader WGSL source.
    pub wgsl: Option<Cow<'a, str>>,
}

// This is so people don't have to fill in fields they don't use, like num_workgroups,
// entry_point, or other shader languages they didn't compile for
impl<'a, L: Default> Default for CreateShaderModuleDescriptorPassthrough<'a, L> {
    fn default() -> Self {
        Self {
            label: Default::default(),
            num_workgroups: (0, 0, 0),
            spirv: None,
            dxil: None,
            metallib: None,
            msl: None,
            hlsl: None,
            glsl: None,
            wgsl: None,
        }
    }
}

impl<'a, L> CreateShaderModuleDescriptorPassthrough<'a, L> {
    /// Takes a closure and maps the label of the shader module descriptor into another.
    pub fn map_label<K>(
        &self,
        fun: impl FnOnce(&L) -> K,
    ) -> CreateShaderModuleDescriptorPassthrough<'a, K> {
        CreateShaderModuleDescriptorPassthrough {
            label: fun(&self.label),
            num_workgroups: self.num_workgroups,
            spirv: self.spirv.clone(),
            metallib: self.metallib.clone(),
            dxil: self.dxil.clone(),
            msl: self.msl.clone(),
            hlsl: self.hlsl.clone(),
            glsl: self.glsl.clone(),
            wgsl: self.wgsl.clone(),
        }
    }

    #[cfg(feature = "trace")]
    /// Returns the source data for tracing purpose.
    pub fn trace_data(&self) -> &[u8] {
        if let Some(spirv) = &self.spirv {
            bytemuck::cast_slice(spirv)
        } else if let Some(metallib) = &self.metallib {
            metallib
        } else if let Some(msl) = &self.msl {
            msl.as_bytes()
        } else if let Some(dxil) = &self.dxil {
            dxil
        } else if let Some(hlsl) = &self.hlsl {
            hlsl.as_bytes()
        } else if let Some(glsl) = &self.glsl {
            glsl.as_bytes()
        } else if let Some(wgsl) = &self.wgsl {
            wgsl.as_bytes()
        } else {
            panic!("No binary data provided to `ShaderModuleDescriptorGeneric`")
        }
    }

    #[cfg(feature = "trace")]
    /// Returns the binary file extension for tracing purpose.
    pub fn trace_binary_ext(&self) -> &'static str {
        if self.spirv.is_some() {
            "spv"
        } else if self.metallib.is_some() {
            "metallib"
        } else if self.msl.is_some() {
            "metal"
        } else if self.dxil.is_some() {
            "dxil"
        } else if self.hlsl.is_some() {
            "hlsl"
        } else if self.glsl.is_some() {
            "glsl"
        } else if self.wgsl.is_some() {
            "wgsl"
        } else {
            panic!("No binary data provided to `ShaderModuleDescriptorGeneric`")
        }
    }
}
