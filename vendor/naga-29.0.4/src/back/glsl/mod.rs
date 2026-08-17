/*!
Backend for [GLSL][glsl] (OpenGL Shading Language).

The main structure is [`Writer`], it maintains internal state that is used
to output a [`Module`](crate::Module) into glsl

# Supported versions
### Core
- 330
- 400
- 410
- 420
- 430
- 450

### ES
- 300
- 310

[glsl]: https://www.khronos.org/registry/OpenGL/index_gl.php
*/

// GLSL is mostly a superset of C but it also removes some parts of it this is a list of relevant
// aspects for this backend.
//
// The most notable change is the introduction of the version preprocessor directive that must
// always be the first line of a glsl file and is written as
// `#version number profile`
// `number` is the version itself (i.e. 300) and `profile` is the
// shader profile we only support "core" and "es", the former is used in desktop applications and
// the later is used in embedded contexts, mobile devices and browsers. Each one as it's own
// versions (at the time of writing this the latest version for "core" is 460 and for "es" is 320)
//
// Other important preprocessor addition is the extension directive which is written as
// `#extension name: behaviour`
// Extensions provide increased features in a plugin fashion but they aren't required to be
// supported hence why they are called extensions, that's why `behaviour` is used it specifies
// whether the extension is strictly required or if it should only be enabled if needed. In our case
// when we use extensions we set behaviour to `require` always.
//
// The only thing that glsl removes that makes a difference are pointers.
//
// Additions that are relevant for the backend are the discard keyword, the introduction of
// vector, matrices, samplers, image types and functions that provide common shader operations

pub use features::Features;
pub use writer::Writer;

use alloc::{
    borrow::ToOwned,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{
    cmp::Ordering,
    fmt::{self, Error as FmtError, Write},
    mem,
};

use hashbrown::hash_map;
use thiserror::Error;

use crate::{
    back::{self, Baked},
    common,
    proc::{self, NameKey},
    valid, Handle, ShaderStage, TypeInner,
};
use conv::*;
use features::FeaturesManager;

/// Contains simple 1:1 conversion functions.
mod conv;
/// Contains the features related code and the features querying method
mod features;
/// Contains a constant with a slice of all the reserved keywords RESERVED_KEYWORDS
mod keywords;
/// Contains the [`Writer`] type.
mod writer;

/// List of supported `core` GLSL versions.
pub const SUPPORTED_CORE_VERSIONS: &[u16] = &[140, 150, 330, 400, 410, 420, 430, 440, 450, 460];
/// List of supported `es` GLSL versions.
pub const SUPPORTED_ES_VERSIONS: &[u16] = &[300, 310, 320];

/// The suffix of the variable that will hold the calculated clamped level
/// of detail for bounds checking in `ImageLoad`
const CLAMPED_LOD_SUFFIX: &str = "_clamped_lod";

pub(crate) const MODF_FUNCTION: &str = "naga_modf";
pub(crate) const FREXP_FUNCTION: &str = "naga_frexp";

// Must match code in glsl_built_in
pub const FIRST_INSTANCE_BINDING: &str = "naga_vs_first_instance";

#[cfg(feature = "deserialize")]
#[derive(serde::Deserialize)]
struct BindingMapSerialization {
    resource_binding: crate::ResourceBinding,
    bind_target: u8,
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

/// Mapping between resources and bindings.
pub type BindingMap = alloc::collections::BTreeMap<crate::ResourceBinding, u8>;

impl crate::AtomicFunction {
    const fn to_glsl(self) -> &'static str {
        match self {
            Self::Add | Self::Subtract => "Add",
            Self::And => "And",
            Self::InclusiveOr => "Or",
            Self::ExclusiveOr => "Xor",
            Self::Min => "Min",
            Self::Max => "Max",
            Self::Exchange { compare: None } => "Exchange",
            Self::Exchange { compare: Some(_) } => "", //TODO
        }
    }
}

impl crate::AddressSpace {
    /// Whether a variable with this address space can be initialized
    const fn initializable(&self) -> bool {
        match *self {
            crate::AddressSpace::Function | crate::AddressSpace::Private => true,
            crate::AddressSpace::WorkGroup
            | crate::AddressSpace::Uniform
            | crate::AddressSpace::Storage { .. }
            | crate::AddressSpace::Handle
            | crate::AddressSpace::Immediate
            | crate::AddressSpace::TaskPayload => false,

            crate::AddressSpace::RayPayload | crate::AddressSpace::IncomingRayPayload => {
                unreachable!()
            }
        }
    }
}

/// A GLSL version.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum Version {
    /// `core` GLSL.
    Desktop(u16),
    /// `es` GLSL.
    Embedded { version: u16, is_webgl: bool },
}

impl Version {
    /// Create a new gles version
    pub const fn new_gles(version: u16) -> Self {
        Self::Embedded {
            version,
            is_webgl: false,
        }
    }

    /// Returns true if self is `Version::Embedded` (i.e. is a es version)
    const fn is_es(&self) -> bool {
        match *self {
            Version::Desktop(_) => false,
            Version::Embedded { .. } => true,
        }
    }

    /// Returns true if targeting WebGL
    const fn is_webgl(&self) -> bool {
        match *self {
            Version::Desktop(_) => false,
            Version::Embedded { is_webgl, .. } => is_webgl,
        }
    }

    /// Checks the list of currently supported versions and returns true if it contains the
    /// specified version
    ///
    /// # Notes
    /// As an invalid version number will never be added to the supported version list
    /// so this also checks for version validity
    fn is_supported(&self) -> bool {
        match *self {
            Version::Desktop(v) => SUPPORTED_CORE_VERSIONS.contains(&v),
            Version::Embedded { version: v, .. } => SUPPORTED_ES_VERSIONS.contains(&v),
        }
    }

    fn supports_io_locations(&self) -> bool {
        *self >= Version::Desktop(330) || *self >= Version::new_gles(300)
    }

    /// Checks if the version supports all of the explicit layouts:
    /// - `location=` qualifiers for bindings
    /// - `binding=` qualifiers for resources
    ///
    /// Note: `location=` for vertex inputs and fragment outputs is supported
    /// unconditionally for GLES 300.
    fn supports_explicit_locations(&self) -> bool {
        *self >= Version::Desktop(420) || *self >= Version::new_gles(310)
    }

    fn supports_early_depth_test(&self) -> bool {
        *self >= Version::Desktop(130) || *self >= Version::new_gles(310)
    }

    fn supports_std140_layout(&self) -> bool {
        *self >= Version::Desktop(140) || *self >= Version::new_gles(300)
    }

    fn supports_std430_layout(&self) -> bool {
        // std430 is available from 400 via GL_ARB_shader_storage_buffer_object.
        *self >= Version::Desktop(400) || *self >= Version::new_gles(310)
    }

    fn supports_fma_function(&self) -> bool {
        *self >= Version::Desktop(400) || *self >= Version::new_gles(320)
    }

    fn supports_integer_functions(&self) -> bool {
        *self >= Version::Desktop(400) || *self >= Version::new_gles(310)
    }

    fn supports_frexp_function(&self) -> bool {
        *self >= Version::Desktop(400) || *self >= Version::new_gles(310)
    }

    fn supports_derivative_control(&self) -> bool {
        *self >= Version::Desktop(450)
    }

    // For supports_pack_unpack_4x8, supports_pack_unpack_snorm_2x16, supports_pack_unpack_unorm_2x16
    // see:
    // https://registry.khronos.org/OpenGL-Refpages/gl4/html/unpackUnorm.xhtml
    // https://registry.khronos.org/OpenGL-Refpages/es3/html/unpackUnorm.xhtml
    // https://registry.khronos.org/OpenGL-Refpages/gl4/html/packUnorm.xhtml
    // https://registry.khronos.org/OpenGL-Refpages/es3/html/packUnorm.xhtml
    fn supports_pack_unpack_4x8(&self) -> bool {
        *self >= Version::Desktop(400) || *self >= Version::new_gles(310)
    }
    fn supports_pack_unpack_snorm_2x16(&self) -> bool {
        *self >= Version::Desktop(420) || *self >= Version::new_gles(300)
    }
    fn supports_pack_unpack_unorm_2x16(&self) -> bool {
        *self >= Version::Desktop(400) || *self >= Version::new_gles(300)
    }

    // https://registry.khronos.org/OpenGL-Refpages/gl4/html/unpackHalf2x16.xhtml
    // https://registry.khronos.org/OpenGL-Refpages/gl4/html/packHalf2x16.xhtml
    // https://registry.khronos.org/OpenGL-Refpages/es3/html/unpackHalf2x16.xhtml
    // https://registry.khronos.org/OpenGL-Refpages/es3/html/packHalf2x16.xhtml
    fn supports_pack_unpack_half_2x16(&self) -> bool {
        *self >= Version::Desktop(420) || *self >= Version::new_gles(300)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (Version::Desktop(x), Version::Desktop(y)) => Some(x.cmp(&y)),
            (Version::Embedded { version: x, .. }, Version::Embedded { version: y, .. }) => {
                Some(x.cmp(&y))
            }
            _ => None,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Version::Desktop(v) => write!(f, "{v} core"),
            Version::Embedded { version: v, .. } => write!(f, "{v} es"),
        }
    }
}

bitflags::bitflags! {
    /// Configuration flags for the [`Writer`].
    #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
    #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct WriterFlags: u32 {
        /// Flip output Y and extend Z from (0, 1) to (-1, 1).
        const ADJUST_COORDINATE_SPACE = 0x1;
        /// Supports GL_EXT_texture_shadow_lod on the host, which provides
        /// additional functions on shadows and arrays of shadows.
        const TEXTURE_SHADOW_LOD = 0x2;
        /// Supports ARB_shader_draw_parameters on the host, which provides
        /// support for `gl_BaseInstanceARB`, `gl_BaseVertexARB`, `gl_DrawIDARB`, and `gl_DrawID`.
        const DRAW_PARAMETERS = 0x4;
        /// Include unused global variables, constants and functions. By default the output will exclude
        /// global variables that are not used in the specified entrypoint (including indirect use),
        /// all constant declarations, and functions that use excluded global variables.
        const INCLUDE_UNUSED_ITEMS = 0x10;
        /// Emit `PointSize` output builtin to vertex shaders, which is
        /// required for drawing with `PointList` topology.
        ///
        /// https://registry.khronos.org/OpenGL/specs/es/3.2/GLSL_ES_Specification_3.20.html#built-in-language-variables
        /// The variable gl_PointSize is intended for a shader to write the size of the point to be rasterized. It is measured in pixels.
        /// If gl_PointSize is not written to, its value is undefined in subsequent pipe stages.
        const FORCE_POINT_SIZE = 0x20;
    }
}

/// Configuration used in the [`Writer`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "deserialize", serde(default))]
pub struct Options {
    /// The GLSL version to be used.
    pub version: Version,
    /// Configuration flags for the [`Writer`].
    pub writer_flags: WriterFlags,
    /// Map of resources association to binding locations.
    #[cfg_attr(
        feature = "deserialize",
        serde(deserialize_with = "deserialize_binding_map")
    )]
    pub binding_map: BindingMap,
    /// Should workgroup variables be zero initialized (by polyfilling)?
    pub zero_initialize_workgroup_memory: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            version: Version::new_gles(310),
            writer_flags: WriterFlags::ADJUST_COORDINATE_SPACE,
            binding_map: BindingMap::default(),
            zero_initialize_workgroup_memory: true,
        }
    }
}

/// A subset of options meant to be changed per pipeline.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PipelineOptions {
    /// The stage of the entry point.
    pub shader_stage: ShaderStage,
    /// The name of the entry point.
    ///
    /// If no entry point that matches is found while creating a [`Writer`], an
    /// error will be thrown.
    pub entry_point: String,
    /// How many views to render to, if doing multiview rendering.
    pub multiview: Option<core::num::NonZeroU32>,
}

#[derive(Debug)]
pub struct VaryingLocation {
    /// The location of the global.
    /// This corresponds to `layout(location = ..)` in GLSL.
    pub location: u32,
    /// The index which can be used for dual source blending.
    /// This corresponds to `layout(index = ..)` in GLSL.
    pub index: u32,
}

/// Reflection info for texture mappings and uniforms.
#[derive(Debug)]
pub struct ReflectionInfo {
    /// Mapping between texture names and variables/samplers.
    pub texture_mapping: crate::FastHashMap<String, TextureMapping>,
    /// Mapping between uniform variables and names.
    pub uniforms: crate::FastHashMap<Handle<crate::GlobalVariable>, String>,
    /// Mapping between names and attribute locations.
    pub varying: crate::FastHashMap<String, VaryingLocation>,
    /// List of immediate data items in the shader.
    pub immediates_items: Vec<ImmediateItem>,
    /// Number of user-defined clip planes. Only applicable to vertex shaders.
    pub clip_distance_count: u32,
}

/// Mapping between a texture and its sampler, if it exists.
///
/// GLSL pre-Vulkan has no concept of separate textures and samplers. Instead, everything is a
/// `gsamplerN` where `g` is the scalar type and `N` is the dimension. But naga uses separate textures
/// and samplers in the IR, so the backend produces a [`FastHashMap`](crate::FastHashMap) with the texture name
/// as a key and a [`TextureMapping`] as a value. This way, the user knows where to bind.
///
/// [`Storage`](crate::ImageClass::Storage) images produce `gimageN` and don't have an associated sampler,
/// so the [`sampler`](Self::sampler) field will be [`None`].
#[derive(Debug, Clone)]
pub struct TextureMapping {
    /// Handle to the image global variable.
    pub texture: Handle<crate::GlobalVariable>,
    /// Handle to the associated sampler global variable, if it exists.
    pub sampler: Option<Handle<crate::GlobalVariable>>,
}

/// All information to bind a single uniform value to the shader.
///
/// Immediates are emulated using traditional uniforms in OpenGL.
///
/// These are composed of a set of primitives (scalar, vector, matrix) that
/// are given names. Because they are not backed by the concept of a buffer,
/// we must do the work of calculating the offset of each primitive in the
/// immediate data block.
#[derive(Debug, Clone)]
pub struct ImmediateItem {
    /// GL uniform name for the item. This name is the same as if you were
    /// to access it directly from a GLSL shader.
    ///
    /// The with the following example, the following names will be generated,
    /// one name per GLSL uniform.
    ///
    /// ```glsl
    /// struct InnerStruct {
    ///     value: f32,
    /// }
    ///
    /// struct ImmediateData {
    ///     InnerStruct inner;
    ///     vec4 array[2];
    /// }
    ///
    /// uniform ImmediateData _immediates_binding_cs;
    /// ```
    ///
    /// ```text
    /// - _immediates_binding_cs.inner.value
    /// - _immediates_binding_cs.array[0]
    /// - _immediates_binding_cs.array[1]
    /// ```
    ///
    pub access_path: String,
    /// Type of the uniform. This will only ever be a scalar, vector, or matrix.
    pub ty: Handle<crate::Type>,
    /// The offset in the immediate data memory block this uniform maps to.
    ///
    /// The size of the uniform can be derived from the type.
    pub offset: u32,
}

/// Helper structure that generates a number
#[derive(Default)]
struct IdGenerator(u32);

impl IdGenerator {
    /// Generates a number that's guaranteed to be unique for this `IdGenerator`
    const fn generate(&mut self) -> u32 {
        // It's just an increasing number but it does the job
        let ret = self.0;
        self.0 += 1;
        ret
    }
}

/// Assorted options needed for generating varyings.
#[derive(Clone, Copy)]
struct VaryingOptions {
    output: bool,
    targeting_webgl: bool,
    draw_parameters: bool,
}

impl VaryingOptions {
    const fn from_writer_options(options: &Options, output: bool) -> Self {
        Self {
            output,
            targeting_webgl: options.version.is_webgl(),
            draw_parameters: options.writer_flags.contains(WriterFlags::DRAW_PARAMETERS),
        }
    }
}

/// Helper wrapper used to get a name for a varying
///
/// Varying have different naming schemes depending on their binding:
/// - Varyings with builtin bindings get their name from [`glsl_built_in`].
/// - Varyings with location bindings are named `_S_location_X` where `S` is a
///   prefix identifying which pipeline stage the varying connects, and `X` is
///   the location.
struct VaryingName<'a> {
    binding: &'a crate::Binding,
    stage: ShaderStage,
    options: VaryingOptions,
}
impl fmt::Display for VaryingName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.binding {
            crate::Binding::Location {
                blend_src: Some(1), ..
            } => {
                write!(f, "_fs2p_location1",)
            }
            crate::Binding::Location { location, .. } => {
                let prefix = match (self.stage, self.options.output) {
                    (ShaderStage::Compute, _) => unreachable!(),
                    // pipeline to vertex
                    (ShaderStage::Vertex, false) => "p2vs",
                    // vertex to fragment
                    (ShaderStage::Vertex, true) | (ShaderStage::Fragment, false) => "vs2fs",
                    // fragment to pipeline
                    (ShaderStage::Fragment, true) => "fs2p",
                    (
                        ShaderStage::Task
                        | ShaderStage::Mesh
                        | ShaderStage::RayGeneration
                        | ShaderStage::AnyHit
                        | ShaderStage::ClosestHit
                        | ShaderStage::Miss,
                        _,
                    ) => unreachable!(),
                };
                write!(f, "_{prefix}_location{location}",)
            }
            crate::Binding::BuiltIn(built_in) => {
                write!(f, "{}", glsl_built_in(built_in, self.options))
            }
        }
    }
}

impl ShaderStage {
    const fn to_str(self) -> &'static str {
        match self {
            ShaderStage::Compute => "cs",
            ShaderStage::Fragment => "fs",
            ShaderStage::Vertex => "vs",
            ShaderStage::Task
            | ShaderStage::Mesh
            | ShaderStage::RayGeneration
            | ShaderStage::AnyHit
            | ShaderStage::ClosestHit
            | ShaderStage::Miss => unreachable!(),
        }
    }
}

/// Shorthand result used internally by the backend
type BackendResult<T = ()> = Result<T, Error>;

/// A GLSL compilation error.
#[derive(Debug, Error)]
pub enum Error {
    /// A error occurred while writing to the output.
    #[error("Format error")]
    FmtError(#[from] FmtError),
    /// The specified [`Version`] doesn't have all required [`Features`].
    ///
    /// Contains the missing [`Features`].
    #[error("The selected version doesn't support {0:?}")]
    MissingFeatures(Features),
    /// [`AddressSpace::Immediate`](crate::AddressSpace::Immediate) was used more than
    /// once in the entry point, which isn't supported.
    #[error("Multiple immediates aren't supported")]
    MultipleImmediateData,
    /// The specified [`Version`] isn't supported.
    #[error("The specified version isn't supported")]
    VersionNotSupported,
    /// The entry point couldn't be found.
    #[error("The requested entry point couldn't be found")]
    EntryPointNotFound,
    /// A call was made to an unsupported external.
    #[error("A call was made to an unsupported external: {0}")]
    UnsupportedExternal(String),
    /// A scalar with an unsupported width was requested.
    #[error("A scalar with an unsupported width was requested: {0:?}")]
    UnsupportedScalar(crate::Scalar),
    /// A image was used with multiple samplers, which isn't supported.
    #[error("A image was used with multiple samplers")]
    ImageMultipleSamplers,
    #[error("{0}")]
    Custom(String),
    #[error("overrides should not be present at this stage")]
    Override,
    /// [`crate::Sampling::First`] is unsupported.
    #[error("`{:?}` sampling is unsupported", crate::Sampling::First)]
    FirstSamplingNotSupported,
    #[error(transparent)]
    ResolveArraySizeError(#[from] proc::ResolveArraySizeError),
}

/// Binary operation with a different logic on the GLSL side.
enum BinaryOperation {
    /// Vector comparison should use the function like `greaterThan()`, etc.
    VectorCompare,
    /// Vector component wise operation; used to polyfill unsupported ops like `|` and `&` for `bvecN`'s
    VectorComponentWise,
    /// GLSL `%` is SPIR-V `OpUMod/OpSMod` and `mod()` is `OpFMod`, but [`BinaryOperator::Modulo`](crate::BinaryOperator::Modulo) is `OpFRem`.
    Modulo,
    /// Any plain operation. No additional logic required.
    Other,
}

fn is_value_init_supported(module: &crate::Module, ty: Handle<crate::Type>) -> bool {
    match module.types[ty].inner {
        TypeInner::Scalar { .. } | TypeInner::Vector { .. } | TypeInner::Matrix { .. } => true,
        TypeInner::Array { base, size, .. } => {
            size != crate::ArraySize::Dynamic && is_value_init_supported(module, base)
        }
        TypeInner::Struct { ref members, .. } => members
            .iter()
            .all(|member| is_value_init_supported(module, member.ty)),
        _ => false,
    }
}

pub fn supported_capabilities() -> valid::Capabilities {
    use valid::Capabilities as Caps;

    // Lots of these aren't supported on GLES in general, but naga is able to write them without panicking.

    Caps::IMMEDIATES
        | Caps::FLOAT64
        | Caps::PRIMITIVE_INDEX
        | Caps::CLIP_DISTANCE
        | Caps::MULTIVIEW
        | Caps::EARLY_DEPTH_TEST
        | Caps::MULTISAMPLED_SHADING
        | Caps::DUAL_SOURCE_BLENDING
        | Caps::CUBE_ARRAY_TEXTURES
        | Caps::SHADER_INT64
        | Caps::SHADER_INT64_ATOMIC_ALL_OPS
        | Caps::TEXTURE_ATOMIC
        | Caps::TEXTURE_INT64_ATOMIC
        | Caps::SUBGROUP
        | Caps::SUBGROUP_BARRIER
        | Caps::SHADER_FLOAT16
        | Caps::SHADER_FLOAT16_IN_FLOAT32
        | Caps::SHADER_BARYCENTRICS
        | Caps::DRAW_INDEX
        | Caps::MEMORY_DECORATION_COHERENT
        | Caps::MEMORY_DECORATION_VOLATILE
}
