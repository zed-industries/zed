//! Bind groups and the bindings in them.

use core::num::NonZeroU32;

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

use crate::{link_to_wgpu_docs, link_to_wgpu_item, BufferSize};

#[cfg(doc)]
use crate::Features;

/// Type of a binding in a [bind group layout][`BindGroupLayoutEntry`].
///
/// For each binding in a layout, a [`BindGroup`] must provide a [`BindingResource`] of the
/// corresponding type.
///
/// Corresponds to WebGPU's mutually exclusive fields within [`GPUBindGroupLayoutEntry`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpubindgrouplayoutentry).
///
#[doc = link_to_wgpu_item!(enum BindingResource)]
#[doc = link_to_wgpu_item!(struct BindGroup)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BindingType {
    /// A buffer binding.
    ///
    /// Corresponds to [WebGPU `GPUBufferBindingLayout`](
    /// https://gpuweb.github.io/gpuweb/#dictdef-gpubufferbindinglayout).
    Buffer {
        /// Sub-type of the buffer binding.
        ty: BufferBindingType,

        /// Indicates that the binding has a dynamic offset.
        ///
        /// One offset must be passed to [`RenderPass::set_bind_group`][RPsbg]
        /// for each dynamic binding in increasing order of binding number.
        ///
        #[doc = link_to_wgpu_docs!(["RPsbg"]: "struct.RenderPass.html#method.set_bind_group")]
        #[cfg_attr(feature = "serde", serde(default))]
        has_dynamic_offset: bool,

        /// The minimum size for a [`BufferBinding`] matching this entry, in bytes.
        ///
        /// If this is `Some(size)`:
        ///
        /// - When calling [`create_bind_group`], the resource at this bind point
        ///   must be a [`BindingResource::Buffer`] whose effective size is at
        ///   least `size`.
        ///
        /// - When calling [`create_render_pipeline`] or [`create_compute_pipeline`],
        ///   `size` must be at least the [minimum buffer binding size] for the
        ///   shader module global at this bind point: large enough to hold the
        ///   global's value, along with one element of a trailing runtime-sized
        ///   array, if present.
        ///
        /// If this is `None`:
        ///
        /// - Each draw or dispatch command checks that the buffer range at this
        ///   bind point satisfies the [minimum buffer binding size].
        ///
        #[doc = link_to_wgpu_item!(struct BufferBinding)]
        #[doc = link_to_wgpu_docs!(["`create_bind_group`"]: "struct.Device.html#method.create_bind_group")]
        #[doc = link_to_wgpu_docs!(["`BindingResource::Buffer`"]: "enum.BindingResource.html#variant.Buffer")]
        /// [minimum buffer binding size]: https://www.w3.org/TR/webgpu/#minimum-buffer-binding-size
        #[doc = link_to_wgpu_docs!(["`create_render_pipeline`"]: "struct.Device.html#method.create_render_pipeline")]
        #[doc = link_to_wgpu_docs!(["`create_compute_pipeline`"]: "struct.Device.html#method.create_compute_pipeline")]
        #[cfg_attr(feature = "serde", serde(default))]
        min_binding_size: Option<BufferSize>,
    },
    /// A sampler that can be used to sample a texture.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var s: sampler;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform sampler s;
    /// ```
    ///
    /// Corresponds to [WebGPU `GPUSamplerBindingLayout`](
    /// https://gpuweb.github.io/gpuweb/#dictdef-gpusamplerbindinglayout).
    Sampler(SamplerBindingType),
    /// A texture binding.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var t: texture_2d<f32>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform texture2D t;
    /// ```
    ///
    /// Corresponds to [WebGPU `GPUTextureBindingLayout`](
    /// https://gpuweb.github.io/gpuweb/#dictdef-gputexturebindinglayout).
    Texture {
        /// Sample type of the texture binding.
        sample_type: crate::TextureSampleType,
        /// Dimension of the texture view that is going to be sampled.
        view_dimension: crate::TextureViewDimension,
        /// True if the texture has a sample count greater than 1. If this is true,
        /// the texture must be declared as `texture_multisampled_2d` or
        /// `texture_depth_multisampled_2d` in the shader, and read using `textureLoad`.
        multisampled: bool,
    },
    /// A storage texture.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<r32float, write>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(set=0, binding=0, r32f) writeonly uniform image2D myStorageImage;
    /// ```
    /// Note that the texture format must be specified in the shader, along with the
    /// access mode. For WGSL, the format must be one of the enumerants in the list
    /// of [storage texel formats](https://gpuweb.github.io/gpuweb/wgsl/#storage-texel-formats).
    ///
    /// Corresponds to [WebGPU `GPUStorageTextureBindingLayout`](
    /// https://gpuweb.github.io/gpuweb/#dictdef-gpustoragetexturebindinglayout).
    StorageTexture {
        /// Allowed access to this texture.
        access: crate::StorageTextureAccess,
        /// Format of the texture.
        format: crate::TextureFormat,
        /// Dimension of the texture view that is going to be sampled.
        view_dimension: crate::TextureViewDimension,
    },

    /// A ray-tracing acceleration structure binding.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var as: acceleration_structure;
    /// ```
    ///
    /// or with vertex return enabled
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var as: acceleration_structure<vertex_return>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform accelerationStructureEXT as;
    /// ```
    AccelerationStructure {
        /// Whether this acceleration structure can be used to
        /// create a ray query that has flag vertex return in the shader
        ///
        /// If enabled requires [`Features::EXPERIMENTAL_RAY_HIT_VERTEX_RETURN`]
        vertex_return: bool,
    },

    /// An external texture binding.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var t: texture_external;
    /// ```
    ///
    /// Corresponds to [WebGPU `GPUExternalTextureBindingLayout`](
    /// https://gpuweb.github.io/gpuweb/#dictdef-gpuexternaltexturebindinglayout).
    ///
    /// Requires [`Features::EXTERNAL_TEXTURE`]
    ExternalTexture,
}

impl BindingType {
    /// Returns true for buffer bindings with dynamic offset enabled.
    #[must_use]
    pub fn has_dynamic_offset(&self) -> bool {
        match *self {
            Self::Buffer {
                has_dynamic_offset, ..
            } => has_dynamic_offset,
            _ => false,
        }
    }
}

bitflags::bitflags! {
    /// Describes the shader stages that a binding will be visible from.
    ///
    /// These can be combined so something that is visible from both vertex and fragment shaders can be defined as:
    ///
    /// `ShaderStages::VERTEX | ShaderStages::FRAGMENT`
    ///
    /// Corresponds to [WebGPU `GPUShaderStageFlags`](
    /// https://gpuweb.github.io/gpuweb/#typedefdef-gpushaderstageflags).
    #[repr(transparent)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ShaderStages: u32 {
        /// Binding is not visible from any shader stage.
        const NONE = 0;
        /// Binding is visible from the vertex shader of a render pipeline.
        const VERTEX = 1 << 0;
        /// Binding is visible from the fragment shader of a render pipeline.
        const FRAGMENT = 1 << 1;
        /// Binding is visible from the compute shader of a compute pipeline.
        const COMPUTE = 1 << 2;
        /// Binding is visible from the vertex and fragment shaders of a render pipeline.
        const VERTEX_FRAGMENT = Self::VERTEX.bits() | Self::FRAGMENT.bits();
        /// Binding is visible from the task shader of a mesh pipeline.
        const TASK = 1 << 3;
        /// Binding is visible from the mesh shader of a mesh pipeline.
        const MESH = 1 << 4;
        /// Binding is visible from the ray generation shader of a ray tracing pipeline.
        const RAY_GENERATION = 1 << 5;
        /// Binding is visible from the ray any hit shader of a ray tracing pipeline.
        const ANY_HIT = 1 << 6;
        /// Binding is visible from the ray closest hit shader of a ray tracing pipeline.
        const CLOSEST_HIT = 1 << 7;
        /// Binding is visible from the ray miss shader of a ray tracing pipeline.
        const MISS = 1 << 8;
    }
}

/// Specific type of a buffer binding.
///
/// Corresponds to [WebGPU `GPUBufferBindingType`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpubufferbindingtype).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BufferBindingType {
    /// A buffer for uniform values.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// struct Globals {
    ///     a_uniform: vec2<f32>,
    ///     another_uniform: vec2<f32>,
    /// }
    /// @group(0) @binding(0)
    /// var<uniform> globals: Globals;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(std140, binding = 0)
    /// uniform Globals {
    ///     vec2 aUniform;
    ///     vec2 anotherUniform;
    /// };
    /// ```
    #[default]
    Uniform,
    /// A storage buffer.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var<storage, read_write> my_element: array<vec4<f32>>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout (set=0, binding=0) buffer myStorageBuffer {
    ///     vec4 myElement[];
    /// };
    /// ```
    Storage {
        /// If `true`, the buffer can only be read in the shader,
        /// and it:
        /// - may or may not be annotated with `read` (WGSL).
        /// - must be annotated with `readonly` (GLSL).
        ///
        /// Example WGSL syntax:
        /// ```rust,ignore
        /// @group(0) @binding(0)
        /// var<storage, read> my_element: array<vec4<f32>>;
        /// ```
        ///
        /// Example GLSL syntax:
        /// ```cpp,ignore
        /// layout (set=0, binding=0) readonly buffer myStorageBuffer {
        ///     vec4 myElement[];
        /// };
        /// ```
        read_only: bool,
    },
}

/// Specific type of a sampler binding.
///
/// For use in [`BindingType::Sampler`].
///
/// Corresponds to [WebGPU `GPUSamplerBindingType`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpusamplerbindingtype).
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum SamplerBindingType {
    /// The sampling result is produced based on more than a single color sample from a texture,
    /// e.g. when bilinear interpolation is enabled.
    Filtering,
    /// The sampling result is produced based on a single color sample from a texture.
    NonFiltering,
    /// Use as a comparison sampler instead of a normal sampler.
    /// For more info take a look at the analogous functionality in OpenGL: <https://www.khronos.org/opengl/wiki/Sampler_Object#Comparison_mode>.
    Comparison,
}

/// Describes a single binding inside a bind group.
///
/// Corresponds to [WebGPU `GPUBindGroupLayoutEntry`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpubindgrouplayoutentry).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BindGroupLayoutEntry {
    /// Binding index. Must match shader index and be unique inside a `BindGroupLayout`. A binding
    /// of index 1, would be described as `@group(0) @binding(1)` in shaders.
    pub binding: u32,
    /// Which shader stages can see this binding.
    pub visibility: ShaderStages,
    /// The type of the binding
    pub ty: BindingType,
    /// If the binding is an array of multiple resources. Corresponds to `binding_array<T>` in the shader.
    ///
    /// When this is `Some` the following validation applies:
    /// - Count must be of value 1 or greater, this corresponds to the length of the array of resources that will be bound.
    /// - When `ty == BindingType::Texture`, [`Features::TEXTURE_BINDING_ARRAY`] must be supported.
    /// - When `ty == BindingType::Sampler`, [`Features::TEXTURE_BINDING_ARRAY`] must be supported.
    /// - When `ty == BindingType::Buffer`, [`Features::BUFFER_BINDING_ARRAY`] must be supported.
    /// - When `ty == BindingType::Buffer` and `ty.ty == BufferBindingType::Storage`, [`Features::STORAGE_RESOURCE_BINDING_ARRAY`] must be supported.
    /// - When `ty == BindingType::StorageTexture`, [`Features::STORAGE_RESOURCE_BINDING_ARRAY`] must be supported.
    /// - When any binding in the group is an array, no `BindingType::Buffer` in the group may have `has_dynamic_offset == true`
    /// - When any binding in the group is an array, no `BindingType::Buffer` in the group may have `ty.ty == BufferBindingType::Uniform`.
    /// - If [`Features::PARTIALLY_BOUND_BINDING_ARRAY`] is enabled, the specified count becomes the upper bound instead.
    ///
    #[cfg_attr(feature = "serde", serde(default))]
    pub count: Option<NonZeroU32>,
}
