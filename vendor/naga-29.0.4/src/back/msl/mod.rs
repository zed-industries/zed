/*!
Backend for [MSL][msl] (Metal Shading Language).

This backend does not support the [`SHADER_INT64_ATOMIC_ALL_OPS`][all-atom]
capability.

## Binding model

Metal's bindings are flat per resource. Since there isn't an obvious mapping
from SPIR-V's descriptor sets, we require a separate mapping provided in the options.
This mapping may have one or more resource end points for each descriptor set + index
pair.

## Entry points

Even though MSL and our IR appear to be similar in that the entry points in both can
accept arguments and return values, the restrictions are different.
MSL allows the varyings to be either in separate arguments, or inside a single
`[[stage_in]]` struct. We gather input varyings and form this artificial structure.
We also add all the (non-Private) globals into the arguments.

At the beginning of the entry point, we assign the local constants and re-compose
the arguments as they are declared on IR side, so that the rest of the logic can
pretend that MSL doesn't have all the restrictions it has.

For the result type, if it's a structure, we re-compose it with a temporary value
holding the result.

[msl]: https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf
[all-atom]: crate::valid::Capabilities::SHADER_INT64_ATOMIC_ALL_OPS

## Pointer-typed bounds-checked expressions and OOB locals

MSL (unlike HLSL and GLSL) has native support for pointer-typed function
arguments. When the [`BoundsCheckPolicy`] is `ReadZeroSkipWrite` and an
out-of-bounds index expression is used for such an argument, our strategy is to
pass a pointer to a dummy variable. These dummy variables are called "OOB
locals". We emit at most one OOB local per function for each type, since all
expressions producing a result of that type can share the same OOB local. (Note
that the OOB local mechanism is not actually implementing "skip write", nor even
"read zero" in some cases of read-after-write, but doing so would require
additional effort and the difference is unlikely to matter.)

[`BoundsCheckPolicy`]: crate::proc::BoundsCheckPolicy

## External textures

Support for [`crate::ImageClass::External`] textures is implemented by lowering
each external texture global variable to 3 `texture2d<float, sample>`s, and a
constant buffer of type `NagaExternalTextureParams`. This provides up to 3
planes of texture data (for example single planar RGBA, or separate Y, Cb, and
Cr planes), and the parameters buffer containing information describing how to
handle these correctly. The bind target to use for each of these globals is
specified via the [`BindTarget::external_texture`] field of the relevant
entries in [`EntryPointResources::resources`].

External textures are supported by WGSL's `textureDimensions()`,
`textureLoad()`, and `textureSampleBaseClampToEdge()` built-in functions. These
are implemented using helper functions. See the following functions for how
these are generated:
 * `Writer::write_wrapped_image_query`
 * `Writer::write_wrapped_image_load`
 * `Writer::write_wrapped_image_sample`

The lowered global variables for each external texture global are passed to the
entry point as separate arguments (see "Entry points" above). However, they are
then wrapped in a struct to allow them to be conveniently passed to user
defined and helper functions. See `writer::EXTERNAL_TEXTURE_WRAPPER_STRUCT`.
*/

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::{Error as FmtError, Write};

use crate::{arena::Handle, ir, proc::index, valid::ModuleInfo};

mod keywords;
pub mod sampler;
mod writer;

pub use writer::Writer;

pub type Slot = u8;
pub type InlineSamplerIndex = u8;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum BindSamplerTarget {
    Resource(Slot),
    Inline(InlineSamplerIndex),
}

/// Binding information for a Naga [`External`] image global variable.
///
/// See the module documentation's section on external textures for details.
///
/// [`External`]: crate::ir::ImageClass::External
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BindExternalTextureTarget {
    pub planes: [Slot; 3],
    pub params: Slot,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(default))]
pub struct BindTarget {
    pub buffer: Option<Slot>,
    pub texture: Option<Slot>,
    pub sampler: Option<BindSamplerTarget>,
    pub external_texture: Option<BindExternalTextureTarget>,
    pub mutable: bool,
}

#[cfg(feature = "deserialize")]
#[derive(serde::Deserialize)]
struct BindingMapSerialization {
    resource_binding: crate::ResourceBinding,
    bind_target: BindTarget,
}

#[cfg(feature = "deserialize")]
fn deserialize_binding_map<'de, D>(deserializer: D) -> Result<BindingMap, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let vec = Vec::<BindingMapSerialization>::deserialize(deserializer)?;
    let mut map = BindingMap::default();
    for item in vec {
        map.insert(item.resource_binding, item.bind_target);
    }
    Ok(map)
}

// Using `BTreeMap` instead of `HashMap` so that we can hash itself.
pub type BindingMap = alloc::collections::BTreeMap<crate::ResourceBinding, BindTarget>;

#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(default))]
pub struct EntryPointResources {
    #[cfg_attr(
        feature = "deserialize",
        serde(deserialize_with = "deserialize_binding_map")
    )]
    pub resources: BindingMap,

    pub immediates_buffer: Option<Slot>,

    /// The slot of a buffer that contains an array of `u32`,
    /// one for the size of each bound buffer that contains a runtime array,
    /// in order of [`crate::GlobalVariable`] declarations.
    pub sizes_buffer: Option<Slot>,
}

pub type EntryPointResourceMap = alloc::collections::BTreeMap<String, EntryPointResources>;

enum ResolvedBinding {
    BuiltIn(crate::BuiltIn),
    Attribute(u32),
    Color {
        location: u32,
        blend_src: Option<u32>,
    },
    User {
        prefix: &'static str,
        index: u32,
        interpolation: Option<ResolvedInterpolation>,
    },
    Resource(BindTarget),
}

#[derive(Copy, Clone)]
enum ResolvedInterpolation {
    CenterPerspective,
    CenterNoPerspective,
    CentroidPerspective,
    CentroidNoPerspective,
    SamplePerspective,
    SampleNoPerspective,
    Flat,
}

// Note: some of these should be removed in favor of proper IR validation.

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Format(#[from] FmtError),
    #[error("bind target {0:?} is empty")]
    UnimplementedBindTarget(BindTarget),
    #[error("composing of {0:?} is not implemented yet")]
    UnsupportedCompose(Handle<crate::Type>),
    #[error("operation {0:?} is not implemented yet")]
    UnsupportedBinaryOp(crate::BinaryOperator),
    #[error("standard function '{0}' is not implemented yet")]
    UnsupportedCall(String),
    #[error("feature '{0}' is not implemented yet")]
    FeatureNotImplemented(String),
    #[error("internal naga error: module should not have validated: {0}")]
    GenericValidation(String),
    #[error("BuiltIn {0:?} is not supported")]
    UnsupportedBuiltIn(crate::BuiltIn),
    #[error("capability {0:?} is not supported")]
    CapabilityNotSupported(crate::valid::Capabilities),
    #[error("attribute '{0}' is not supported for target MSL version")]
    UnsupportedAttribute(String),
    #[error("function '{0}' is not supported for target MSL version")]
    UnsupportedFunction(String),
    #[error("can not use writeable storage buffers in fragment stage prior to MSL 1.2")]
    UnsupportedWriteableStorageBuffer,
    #[error("can not use writeable storage textures in {0:?} stage prior to MSL 1.2")]
    UnsupportedWriteableStorageTexture(ir::ShaderStage),
    #[error("can not use read-write storage textures prior to MSL 1.2")]
    UnsupportedRWStorageTexture,
    #[error("array of '{0}' is not supported for target MSL version")]
    UnsupportedArrayOf(String),
    #[error("array of type '{0:?}' is not supported")]
    UnsupportedArrayOfType(Handle<crate::Type>),
    #[error("ray tracing is not supported prior to MSL 2.4")]
    UnsupportedRayTracing,
    #[error("cooperative matrix is not supported prior to MSL 2.3")]
    UnsupportedCooperativeMatrix,
    #[error("overrides should not be present at this stage")]
    Override,
    #[error("bitcasting to {0:?} is not supported")]
    UnsupportedBitCast(crate::TypeInner),
    #[error(transparent)]
    ResolveArraySizeError(#[from] crate::proc::ResolveArraySizeError),
    #[error("entry point with stage {0:?} and name '{1}' not found")]
    EntryPointNotFound(ir::ShaderStage, String),
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum EntryPointError {
    #[error("global '{0}' doesn't have a binding")]
    MissingBinding(String),
    #[error("mapping of {0:?} is missing")]
    MissingBindTarget(crate::ResourceBinding),
    #[error("mapping for immediates is missing")]
    MissingImmediateData,
    #[error("mapping for sizes buffer is missing")]
    MissingSizesBuffer,
}

/// Points in the MSL code where we might emit a pipeline input or output.
///
/// Note that, even though vertex shaders' outputs are always fragment
/// shaders' inputs, we still need to distinguish `VertexOutput` and
/// `FragmentInput`, since there are certain differences in the way
/// [`ResolvedBinding`s] are represented on either side.
///
/// [`ResolvedBinding`s]: ResolvedBinding
#[derive(Clone, Copy, Debug)]
enum LocationMode {
    /// Input to the vertex shader.
    VertexInput,

    /// Output from the vertex shader.
    VertexOutput,

    /// Input to the fragment shader.
    FragmentInput,

    /// Output from the fragment shader.
    FragmentOutput,

    /// Compute shader input or output.
    Uniform,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "deserialize", serde(default))]
pub struct Options {
    /// (Major, Minor) target version of the Metal Shading Language.
    pub lang_version: (u8, u8),
    /// Map of entry-point resources, indexed by entry point function name, to slots.
    pub per_entry_point_map: EntryPointResourceMap,
    /// Samplers to be inlined into the code.
    pub inline_samplers: Vec<sampler::InlineSampler>,
    /// Make it possible to link different stages via SPIRV-Cross.
    pub spirv_cross_compatibility: bool,
    /// Don't panic on missing bindings, instead generate invalid MSL.
    pub fake_missing_bindings: bool,
    /// Bounds checking policies.
    pub bounds_check_policies: index::BoundsCheckPolicies,
    /// Should workgroup variables be zero initialized (by polyfilling)?
    pub zero_initialize_workgroup_memory: bool,
    /// If set, loops will have code injected into them, forcing the compiler
    /// to think the number of iterations is bounded.
    pub force_loop_bounding: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            lang_version: (1, 0),
            per_entry_point_map: EntryPointResourceMap::default(),
            inline_samplers: Vec::new(),
            spirv_cross_compatibility: false,
            fake_missing_bindings: true,
            bounds_check_policies: index::BoundsCheckPolicies::default(),
            zero_initialize_workgroup_memory: true,
            force_loop_bounding: true,
        }
    }
}

/// Corresponds to [WebGPU `GPUVertexFormat`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpuvertexformat).
#[repr(u32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum VertexFormat {
    /// One unsigned byte (u8). `u32` in shaders.
    Uint8 = 0,
    /// Two unsigned bytes (u8). `vec2<u32>` in shaders.
    Uint8x2 = 1,
    /// Four unsigned bytes (u8). `vec4<u32>` in shaders.
    Uint8x4 = 2,
    /// One signed byte (i8). `i32` in shaders.
    Sint8 = 3,
    /// Two signed bytes (i8). `vec2<i32>` in shaders.
    Sint8x2 = 4,
    /// Four signed bytes (i8). `vec4<i32>` in shaders.
    Sint8x4 = 5,
    /// One unsigned byte (u8). [0, 255] converted to float [0, 1] `f32` in shaders.
    Unorm8 = 6,
    /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2<f32>` in shaders.
    Unorm8x2 = 7,
    /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4<f32>` in shaders.
    Unorm8x4 = 8,
    /// One signed byte (i8). [-127, 127] converted to float [-1, 1] `f32` in shaders.
    Snorm8 = 9,
    /// Two signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec2<f32>` in shaders.
    Snorm8x2 = 10,
    /// Four signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec4<f32>` in shaders.
    Snorm8x4 = 11,
    /// One unsigned short (u16). `u32` in shaders.
    Uint16 = 12,
    /// Two unsigned shorts (u16). `vec2<u32>` in shaders.
    Uint16x2 = 13,
    /// Four unsigned shorts (u16). `vec4<u32>` in shaders.
    Uint16x4 = 14,
    /// One signed short (u16). `i32` in shaders.
    Sint16 = 15,
    /// Two signed shorts (i16). `vec2<i32>` in shaders.
    Sint16x2 = 16,
    /// Four signed shorts (i16). `vec4<i32>` in shaders.
    Sint16x4 = 17,
    /// One unsigned short (u16). [0, 65535] converted to float [0, 1] `f32` in shaders.
    Unorm16 = 18,
    /// Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2<f32>` in shaders.
    Unorm16x2 = 19,
    /// Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4<f32>` in shaders.
    Unorm16x4 = 20,
    /// One signed short (i16). [-32767, 32767] converted to float [-1, 1] `f32` in shaders.
    Snorm16 = 21,
    /// Two signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec2<f32>` in shaders.
    Snorm16x2 = 22,
    /// Four signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec4<f32>` in shaders.
    Snorm16x4 = 23,
    /// One half-precision float (no Rust equiv). `f32` in shaders.
    Float16 = 24,
    /// Two half-precision floats (no Rust equiv). `vec2<f32>` in shaders.
    Float16x2 = 25,
    /// Four half-precision floats (no Rust equiv). `vec4<f32>` in shaders.
    Float16x4 = 26,
    /// One single-precision float (f32). `f32` in shaders.
    Float32 = 27,
    /// Two single-precision floats (f32). `vec2<f32>` in shaders.
    Float32x2 = 28,
    /// Three single-precision floats (f32). `vec3<f32>` in shaders.
    Float32x3 = 29,
    /// Four single-precision floats (f32). `vec4<f32>` in shaders.
    Float32x4 = 30,
    /// One unsigned int (u32). `u32` in shaders.
    Uint32 = 31,
    /// Two unsigned ints (u32). `vec2<u32>` in shaders.
    Uint32x2 = 32,
    /// Three unsigned ints (u32). `vec3<u32>` in shaders.
    Uint32x3 = 33,
    /// Four unsigned ints (u32). `vec4<u32>` in shaders.
    Uint32x4 = 34,
    /// One signed int (i32). `i32` in shaders.
    Sint32 = 35,
    /// Two signed ints (i32). `vec2<i32>` in shaders.
    Sint32x2 = 36,
    /// Three signed ints (i32). `vec3<i32>` in shaders.
    Sint32x3 = 37,
    /// Four signed ints (i32). `vec4<i32>` in shaders.
    Sint32x4 = 38,
    /// Three unsigned 10-bit integers and one 2-bit integer, packed into a 32-bit integer (u32). [0, 1024] converted to float [0, 1] `vec4<f32>` in shaders.
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(rename = "unorm10-10-10-2")
    )]
    Unorm10_10_10_2 = 43,
    /// Four unsigned 8-bit integers, packed into a 32-bit integer (u32). [0, 255] converted to float [0, 1] `vec4<f32>` in shaders.
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(rename = "unorm8x4-bgra")
    )]
    Unorm8x4Bgra = 44,
}

/// Defines how to advance the data in vertex buffers.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum VertexBufferStepMode {
    Constant,
    #[default]
    ByVertex,
    ByInstance,
}

/// A mapping of vertex buffers and their attributes to shader
/// locations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AttributeMapping {
    /// Shader location associated with this attribute
    pub shader_location: u32,
    /// Offset in bytes from start of vertex buffer structure
    pub offset: u32,
    /// Format code to help us unpack the attribute into the type
    /// used by the shader. Codes correspond to a 0-based index of
    /// <https://gpuweb.github.io/gpuweb/#enumdef-gpuvertexformat>.
    /// The conversion process is described by
    /// <https://gpuweb.github.io/gpuweb/#vertex-processing>.
    pub format: VertexFormat,
}

/// A description of a vertex buffer with all the information we
/// need to address the attributes within it.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct VertexBufferMapping {
    /// Shader location associated with this buffer
    pub id: u32,
    /// Size of the structure in bytes
    pub stride: u32,
    /// Vertex buffer step mode
    pub step_mode: VertexBufferStepMode,
    /// Vec of the attributes within the structure
    pub attributes: Vec<AttributeMapping>,
}

/// A subset of options that are meant to be changed per pipeline.
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "deserialize", serde(default))]
pub struct PipelineOptions {
    /// The entry point to write.
    ///
    /// Entry points are identified by a shader stage specification,
    /// and a name.
    ///
    /// If `None`, all entry points will be written. If `Some` and the entry
    /// point is not found, an error will be thrown while writing.
    pub entry_point: Option<(ir::ShaderStage, String)>,

    /// Allow `BuiltIn::PointSize` and inject it if doesn't exist.
    ///
    /// Metal doesn't like this for non-point primitive topologies and requires it for
    /// point primitive topologies.
    ///
    /// Enable this for vertex shaders with point primitive topologies.
    pub allow_and_force_point_size: bool,

    /// If set, when generating the Metal vertex shader, transform it
    /// to receive the vertex buffers, lengths, and vertex id as args,
    /// and bounds-check the vertex id and use the index into the
    /// vertex buffers to access attributes, rather than using Metal's
    /// [[stage-in]] assembled attribute data. This is true by default,
    /// but remains configurable for use by tests via deserialization
    /// of this struct. There is no user-facing way to set this value.
    pub vertex_pulling_transform: bool,

    /// vertex_buffer_mappings are used during shader translation to
    /// support vertex pulling.
    pub vertex_buffer_mappings: Vec<VertexBufferMapping>,
}

impl Options {
    fn resolve_local_binding(
        &self,
        binding: &crate::Binding,
        mode: LocationMode,
    ) -> Result<ResolvedBinding, Error> {
        match *binding {
            crate::Binding::BuiltIn(mut built_in) => {
                match built_in {
                    crate::BuiltIn::Position { ref mut invariant } => {
                        if *invariant && self.lang_version < (2, 1) {
                            return Err(Error::UnsupportedAttribute("invariant".to_string()));
                        }

                        // The 'invariant' attribute may only appear on vertex
                        // shader outputs, not fragment shader inputs.
                        if !matches!(mode, LocationMode::VertexOutput) {
                            *invariant = false;
                        }
                    }
                    crate::BuiltIn::BaseInstance if self.lang_version < (1, 2) => {
                        return Err(Error::UnsupportedAttribute("base_instance".to_string()));
                    }
                    crate::BuiltIn::InstanceIndex if self.lang_version < (1, 2) => {
                        return Err(Error::UnsupportedAttribute("instance_id".to_string()));
                    }
                    // macOS: Since Metal 2.2
                    // iOS: Since Metal 2.3 (check depends on https://github.com/gfx-rs/wgpu/issues/4414)
                    crate::BuiltIn::PrimitiveIndex if self.lang_version < (2, 3) => {
                        return Err(Error::UnsupportedAttribute("primitive_id".to_string()));
                    }
                    // macOS: since Metal 2.3
                    // iOS: Since Metal 2.2
                    // https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf#page=114
                    crate::BuiltIn::ViewIndex if self.lang_version < (2, 2) => {
                        return Err(Error::UnsupportedAttribute("amplification_id".to_string()));
                    }
                    // macOS: Since Metal 2.2
                    // iOS: Since Metal 2.3 (check depends on https://github.com/gfx-rs/wgpu/issues/4414)
                    crate::BuiltIn::Barycentric { .. } if self.lang_version < (2, 3) => {
                        return Err(Error::UnsupportedAttribute("barycentric_coord".to_string()));
                    }
                    _ => {}
                }

                Ok(ResolvedBinding::BuiltIn(built_in))
            }
            crate::Binding::Location {
                location,
                interpolation,
                sampling,
                blend_src,
                per_primitive: _,
            } => match mode {
                LocationMode::VertexInput => Ok(ResolvedBinding::Attribute(location)),
                LocationMode::FragmentOutput => {
                    if blend_src.is_some() && self.lang_version < (1, 2) {
                        return Err(Error::UnsupportedAttribute("blend_src".to_string()));
                    }
                    Ok(ResolvedBinding::Color {
                        location,
                        blend_src,
                    })
                }
                LocationMode::VertexOutput | LocationMode::FragmentInput => {
                    Ok(ResolvedBinding::User {
                        prefix: if self.spirv_cross_compatibility {
                            "locn"
                        } else {
                            "loc"
                        },
                        index: location,
                        interpolation: {
                            // unwrap: The verifier ensures that vertex shader outputs and fragment
                            // shader inputs always have fully specified interpolation, and that
                            // sampling is `None` only for Flat interpolation.
                            let interpolation = interpolation.unwrap();
                            let sampling = sampling.unwrap_or(crate::Sampling::Center);
                            Some(ResolvedInterpolation::from_binding(interpolation, sampling))
                        },
                    })
                }
                LocationMode::Uniform => Err(Error::GenericValidation(format!(
                    "Unexpected Binding::Location({location}) for the Uniform mode"
                ))),
            },
        }
    }

    fn get_entry_point_resources(&self, ep: &crate::EntryPoint) -> Option<&EntryPointResources> {
        self.per_entry_point_map.get(&ep.name)
    }

    fn get_resource_binding_target(
        &self,
        ep: &crate::EntryPoint,
        res_binding: &crate::ResourceBinding,
    ) -> Option<&BindTarget> {
        self.get_entry_point_resources(ep)
            .and_then(|res| res.resources.get(res_binding))
    }

    fn resolve_resource_binding(
        &self,
        ep: &crate::EntryPoint,
        res_binding: &crate::ResourceBinding,
    ) -> Result<ResolvedBinding, EntryPointError> {
        let target = self.get_resource_binding_target(ep, res_binding);
        match target {
            Some(target) => Ok(ResolvedBinding::Resource(target.clone())),
            None if self.fake_missing_bindings => Ok(ResolvedBinding::User {
                prefix: "fake",
                index: 0,
                interpolation: None,
            }),
            None => Err(EntryPointError::MissingBindTarget(*res_binding)),
        }
    }

    fn resolve_immediates(
        &self,
        ep: &crate::EntryPoint,
    ) -> Result<ResolvedBinding, EntryPointError> {
        let slot = self
            .get_entry_point_resources(ep)
            .and_then(|res| res.immediates_buffer);
        match slot {
            Some(slot) => Ok(ResolvedBinding::Resource(BindTarget {
                buffer: Some(slot),
                ..Default::default()
            })),
            None if self.fake_missing_bindings => Ok(ResolvedBinding::User {
                prefix: "fake",
                index: 0,
                interpolation: None,
            }),
            None => Err(EntryPointError::MissingImmediateData),
        }
    }

    fn resolve_sizes_buffer(
        &self,
        ep: &crate::EntryPoint,
    ) -> Result<ResolvedBinding, EntryPointError> {
        let slot = self
            .get_entry_point_resources(ep)
            .and_then(|res| res.sizes_buffer);
        match slot {
            Some(slot) => Ok(ResolvedBinding::Resource(BindTarget {
                buffer: Some(slot),
                ..Default::default()
            })),
            None if self.fake_missing_bindings => Ok(ResolvedBinding::User {
                prefix: "fake",
                index: 0,
                interpolation: None,
            }),
            None => Err(EntryPointError::MissingSizesBuffer),
        }
    }
}

impl ResolvedBinding {
    fn as_inline_sampler<'a>(&self, options: &'a Options) -> Option<&'a sampler::InlineSampler> {
        match *self {
            Self::Resource(BindTarget {
                sampler: Some(BindSamplerTarget::Inline(index)),
                ..
            }) => Some(&options.inline_samplers[index as usize]),
            _ => None,
        }
    }

    fn try_fmt<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        write!(out, " [[")?;
        match *self {
            Self::BuiltIn(built_in) => {
                use crate::BuiltIn as Bi;
                let name = match built_in {
                    Bi::Position { invariant: false } => "position",
                    Bi::Position { invariant: true } => "position, invariant",
                    Bi::ViewIndex => "amplification_id",
                    // vertex
                    Bi::BaseInstance => "base_instance",
                    Bi::BaseVertex => "base_vertex",
                    Bi::ClipDistance => "clip_distance",
                    Bi::InstanceIndex => "instance_id",
                    Bi::PointSize => "point_size",
                    Bi::VertexIndex => "vertex_id",
                    // fragment
                    Bi::FragDepth => "depth(any)",
                    Bi::PointCoord => "point_coord",
                    Bi::FrontFacing => "front_facing",
                    Bi::PrimitiveIndex => "primitive_id",
                    Bi::Barycentric { perspective: true } => "barycentric_coord",
                    Bi::Barycentric { perspective: false } => {
                        "barycentric_coord, center_no_perspective"
                    }
                    Bi::SampleIndex => "sample_id",
                    Bi::SampleMask => "sample_mask",
                    // compute
                    Bi::GlobalInvocationId => "thread_position_in_grid",
                    Bi::LocalInvocationId => "thread_position_in_threadgroup",
                    Bi::LocalInvocationIndex => "thread_index_in_threadgroup",
                    Bi::WorkGroupId => "threadgroup_position_in_grid",
                    Bi::WorkGroupSize => "dispatch_threads_per_threadgroup",
                    Bi::NumWorkGroups => "threadgroups_per_grid",
                    // subgroup
                    Bi::NumSubgroups => "simdgroups_per_threadgroup",
                    Bi::SubgroupId => "simdgroup_index_in_threadgroup",
                    Bi::SubgroupSize => "threads_per_simdgroup",
                    Bi::SubgroupInvocationId => "thread_index_in_simdgroup",
                    Bi::CullDistance | Bi::DrawIndex => {
                        return Err(Error::UnsupportedBuiltIn(built_in))
                    }
                    Bi::CullPrimitive => "primitive_culled",
                    // TODO: figure out how to make this written as a function call
                    Bi::PointIndex | Bi::LineIndices | Bi::TriangleIndices => unimplemented!(),
                    Bi::MeshTaskSize
                    | Bi::VertexCount
                    | Bi::PrimitiveCount
                    | Bi::Vertices
                    | Bi::Primitives
                    | Bi::RayInvocationId
                    | Bi::NumRayInvocations
                    | Bi::InstanceCustomData
                    | Bi::GeometryIndex
                    | Bi::WorldRayOrigin
                    | Bi::WorldRayDirection
                    | Bi::ObjectRayOrigin
                    | Bi::ObjectRayDirection
                    | Bi::RayTmin
                    | Bi::RayTCurrentMax
                    | Bi::ObjectToWorld
                    | Bi::WorldToObject
                    | Bi::HitKind => unreachable!(),
                };
                write!(out, "{name}")?;
            }
            Self::Attribute(index) => write!(out, "attribute({index})")?,
            Self::Color {
                location,
                blend_src,
            } => {
                if let Some(blend_src) = blend_src {
                    write!(out, "color({location}) index({blend_src})")?
                } else {
                    write!(out, "color({location})")?
                }
            }
            Self::User {
                prefix,
                index,
                interpolation,
            } => {
                write!(out, "user({prefix}{index})")?;
                if let Some(interpolation) = interpolation {
                    write!(out, ", ")?;
                    interpolation.try_fmt(out)?;
                }
            }
            Self::Resource(ref target) => {
                if let Some(id) = target.buffer {
                    write!(out, "buffer({id})")?;
                } else if let Some(id) = target.texture {
                    write!(out, "texture({id})")?;
                } else if let Some(BindSamplerTarget::Resource(id)) = target.sampler {
                    write!(out, "sampler({id})")?;
                } else {
                    return Err(Error::UnimplementedBindTarget(target.clone()));
                }
            }
        }
        write!(out, "]]")?;
        Ok(())
    }
}

impl ResolvedInterpolation {
    const fn from_binding(interpolation: crate::Interpolation, sampling: crate::Sampling) -> Self {
        use crate::Interpolation as I;
        use crate::Sampling as S;

        match (interpolation, sampling) {
            (I::Perspective, S::Center) => Self::CenterPerspective,
            (I::Perspective, S::Centroid) => Self::CentroidPerspective,
            (I::Perspective, S::Sample) => Self::SamplePerspective,
            (I::Linear, S::Center) => Self::CenterNoPerspective,
            (I::Linear, S::Centroid) => Self::CentroidNoPerspective,
            (I::Linear, S::Sample) => Self::SampleNoPerspective,
            (I::Flat, _) => Self::Flat,
            _ => unreachable!(),
        }
    }

    fn try_fmt<W: Write>(self, out: &mut W) -> Result<(), Error> {
        let identifier = match self {
            Self::CenterPerspective => "center_perspective",
            Self::CenterNoPerspective => "center_no_perspective",
            Self::CentroidPerspective => "centroid_perspective",
            Self::CentroidNoPerspective => "centroid_no_perspective",
            Self::SamplePerspective => "sample_perspective",
            Self::SampleNoPerspective => "sample_no_perspective",
            Self::Flat => "flat",
        };
        out.write_str(identifier)?;
        Ok(())
    }
}

/// Information about a translated module that is required
/// for the use of the result.
pub struct TranslationInfo {
    /// Mapping of the entry point names. Each item in the array
    /// corresponds to an entry point index.
    ///
    ///Note: Some entry points may fail translation because of missing bindings.
    pub entry_point_names: Vec<Result<String, EntryPointError>>,
}

pub fn write_string(
    module: &crate::Module,
    info: &ModuleInfo,
    options: &Options,
    pipeline_options: &PipelineOptions,
) -> Result<(String, TranslationInfo), Error> {
    let mut w = Writer::new(String::new());
    let info = w.write(module, info, options, pipeline_options)?;
    Ok((w.finish(), info))
}

pub fn supported_capabilities() -> crate::valid::Capabilities {
    use crate::valid::Capabilities as Caps;
    Caps::IMMEDIATES
        // No FLOAT64
        | Caps::PRIMITIVE_INDEX
        | Caps::TEXTURE_AND_SAMPLER_BINDING_ARRAY
        // No BUFFER_BINDING_ARRAY
        | Caps::STORAGE_TEXTURE_BINDING_ARRAY
        | Caps::STORAGE_BUFFER_BINDING_ARRAY
        | Caps::CLIP_DISTANCE // CLIP_DISTANCE isn't supported by metal backend? But is supported by MSL writer
        // No CULL_DISTANCE
        | Caps::STORAGE_TEXTURE_16BIT_NORM_FORMATS
        | Caps::MULTIVIEW
        // No EARLY_DEPTH_TEST
        | Caps::MULTISAMPLED_SHADING
        | Caps::RAY_QUERY
        | Caps::DUAL_SOURCE_BLENDING
        | Caps::CUBE_ARRAY_TEXTURES
        | Caps::SHADER_INT64
        | Caps::SUBGROUP
        | Caps::SUBGROUP_BARRIER
        // No SUBGROUP_VERTEX_STAGE
        | Caps::SHADER_INT64_ATOMIC_MIN_MAX
        // No SHADER_INT64_ATOMIC_ALL_OPS
        | Caps::SHADER_FLOAT32_ATOMIC
        | Caps::TEXTURE_ATOMIC
        | Caps::TEXTURE_INT64_ATOMIC
        // No RAY_HIT_VERTEX_POSITION
        | Caps::SHADER_FLOAT16
        | Caps::TEXTURE_EXTERNAL
        | Caps::SHADER_FLOAT16_IN_FLOAT32
        | Caps::SHADER_BARYCENTRICS
        // No MESH_SHADER
        // No MESH_SHADER_POINT_TOPOLOGY
        | Caps::TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING
        // No BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING
        | Caps::STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING
        | Caps::STORAGE_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING
        | Caps::COOPERATIVE_MATRIX
        // No PER_VERTEX
        // No RAY_TRACING_PIPELINE
        // No DRAW_INDEX
        // No MEMORY_DECORATION_VOLATILE
        | Caps::MEMORY_DECORATION_COHERENT
}

#[test]
fn test_error_size() {
    assert_eq!(size_of::<Error>(), 40);
}
