//! Types for defining vertex attributes and their buffers.

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

use crate::{link_to_wgpu_docs, link_to_wgpu_item};

#[cfg(doc)]
use crate::Features;

/// Whether a vertex buffer is indexed by vertex or by instance.
///
/// Consider a call to [`RenderPass::draw`] like this:
///
/// ```ignore
/// render_pass.draw(vertices, instances)
/// ```
///
/// where `vertices` is a `Range<u32>` of vertex indices, and
/// `instances` is a `Range<u32>` of instance indices.
///
/// For this call, `wgpu` invokes the vertex shader entry point once
/// for every possible `(v, i)` pair, where `v` is drawn from
/// `vertices` and `i` is drawn from `instances`. These invocations
/// may happen in any order, and will usually run in parallel.
///
/// Each vertex buffer has a step mode, established by the
/// [`step_mode`] field of its [`VertexBufferLayout`], given when the
/// pipeline was created. Buffers whose step mode is [`Vertex`] use
/// `v` as the index into their contents, whereas buffers whose step
/// mode is [`Instance`] use `i`. The indicated buffer element then
/// contributes zero or more attribute values for the `(v, i)` vertex
/// shader invocation to use, based on the [`VertexBufferLayout`]'s
/// [`attributes`] list.
///
/// You can visualize the results from all these vertex shader
/// invocations as a matrix with a row for each `i` from `instances`,
/// and with a column for each `v` from `vertices`. In one sense, `v`
/// and `i` are symmetrical: both are used to index vertex buffers and
/// provide attribute values.  But the key difference between `v` and
/// `i` is that line and triangle primitives are built from the values
/// of each row, along which `i` is constant and `v` varies, not the
/// columns.
///
/// An indexed draw call works similarly:
///
/// ```ignore
/// render_pass.draw_indexed(indices, base_vertex, instances)
/// ```
///
/// The only difference is that `v` values are drawn from the contents
/// of the index buffer&mdash;specifically, the subrange of the index
/// buffer given by `indices`&mdash;instead of simply being sequential
/// integers, as they are in a `draw` call.
///
/// A non-instanced call, where `instances` is `0..1`, is simply a
/// matrix with only one row.
///
/// Corresponds to [WebGPU `GPUVertexStepMode`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpuvertexstepmode).
///
#[doc = link_to_wgpu_docs!(["`RenderPass::draw`"]: "struct.RenderPass.html#method.draw")]
#[doc = link_to_wgpu_item!(struct VertexBufferLayout)]
#[doc = link_to_wgpu_docs!(["`step_mode`"]: "struct.VertexBufferLayout.html#structfield.step_mode")]
#[doc = link_to_wgpu_docs!(["`attributes`"]: "struct.VertexBufferLayout.html#structfield.attributes")]
/// [`Vertex`]: VertexStepMode::Vertex
/// [`Instance`]: VertexStepMode::Instance
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum VertexStepMode {
    /// Vertex data is advanced every vertex.
    #[default]
    Vertex = 0,
    /// Vertex data is advanced every instance.
    Instance = 1,
}

/// Vertex inputs (attributes) to shaders.
///
/// These are used to specify the individual attributes within a [`VertexBufferLayout`].
/// See its documentation for an example.
///
/// The [`vertex_attr_array!`] macro can help create these with appropriate offsets.
///
/// Corresponds to [WebGPU `GPUVertexAttribute`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuvertexattribute).
///
#[doc = link_to_wgpu_docs!(["`vertex_attr_array!`"]: "macro.vertex_attr_array.html")]
#[doc = link_to_wgpu_item!(struct VertexBufferLayout)]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct VertexAttribute {
    /// Format of the input
    pub format: VertexFormat,
    /// Byte offset of the start of the input
    pub offset: crate::BufferAddress,
    /// Location for this input. Must match the location in the shader.
    pub shader_location: crate::ShaderLocation,
}

/// Vertex Format for a [`VertexAttribute`] (input).
///
/// Corresponds to [WebGPU `GPUVertexFormat`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpuvertexformat).
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
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
    /// One signed byte (i8). [&minus;127, 127] converted to float [&minus;1, 1] `f32` in shaders.
    Snorm8 = 9,
    /// Two signed bytes (i8). [&minus;127, 127] converted to float [&minus;1, 1] `vec2<f32>` in shaders.
    Snorm8x2 = 10,
    /// Four signed bytes (i8). [&minus;127, 127] converted to float [&minus;1, 1] `vec4<f32>` in shaders.
    Snorm8x4 = 11,
    /// One unsigned short (u16). `u32` in shaders.
    Uint16 = 12,
    /// Two unsigned shorts (u16). `vec2<u32>` in shaders.
    Uint16x2 = 13,
    /// Four unsigned shorts (u16). `vec4<u32>` in shaders.
    Uint16x4 = 14,
    /// One signed short (i16). `i32` in shaders.
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
    /// One signed short (i16). [&minus;32767, 32767] converted to float [&minus;1, 1] `f32` in shaders.
    Snorm16 = 21,
    /// Two signed shorts (i16). [&minus;32767, 32767] converted to float [&minus;1, 1] `vec2<f32>` in shaders.
    Snorm16x2 = 22,
    /// Four signed shorts (i16). [&minus;32767, 32767] converted to float [&minus;1, 1] `vec4<f32>` in shaders.
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
    /// One double-precision float (f64). `f32` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64 = 39,
    /// Two double-precision floats (f64). `vec2<f32>` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64x2 = 40,
    /// Three double-precision floats (f64). `vec3<f32>` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64x3 = 41,
    /// Four double-precision floats (f64). `vec4<f32>` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64x4 = 42,
    /// Three unsigned 10-bit integers and one 2-bit integer, packed into a 32-bit integer (u32). [0, 1023] and [0, 3] converted to float [0, 1] `vec4<f32>` in shaders.
    #[cfg_attr(feature = "serde", serde(rename = "unorm10-10-10-2"))]
    Unorm10_10_10_2 = 43,
    /// Four unsigned 8-bit integers (u8) in BGRA. [0, 255] converted to float [0, 1] `vec4<f32>` RGBA in shaders.
    #[cfg_attr(feature = "serde", serde(rename = "unorm8x4-bgra"))]
    Unorm8x4Bgra = 44,
}

impl VertexFormat {
    /// Returns the byte size of the format.
    #[must_use]
    pub const fn size(&self) -> u64 {
        match self {
            Self::Uint8 | Self::Sint8 | Self::Unorm8 | Self::Snorm8 => 1,
            Self::Uint8x2
            | Self::Sint8x2
            | Self::Unorm8x2
            | Self::Snorm8x2
            | Self::Uint16
            | Self::Sint16
            | Self::Unorm16
            | Self::Snorm16
            | Self::Float16 => 2,
            Self::Uint8x4
            | Self::Sint8x4
            | Self::Unorm8x4
            | Self::Snorm8x4
            | Self::Uint16x2
            | Self::Sint16x2
            | Self::Unorm16x2
            | Self::Snorm16x2
            | Self::Float16x2
            | Self::Float32
            | Self::Uint32
            | Self::Sint32
            | Self::Unorm10_10_10_2
            | Self::Unorm8x4Bgra => 4,
            Self::Uint16x4
            | Self::Sint16x4
            | Self::Unorm16x4
            | Self::Snorm16x4
            | Self::Float16x4
            | Self::Float32x2
            | Self::Uint32x2
            | Self::Sint32x2
            | Self::Float64 => 8,
            Self::Float32x3 | Self::Uint32x3 | Self::Sint32x3 => 12,
            Self::Float32x4 | Self::Uint32x4 | Self::Sint32x4 | Self::Float64x2 => 16,
            Self::Float64x3 => 24,
            Self::Float64x4 => 32,
        }
    }

    /// Returns the size read by an acceleration structure build of the vertex format. This is
    /// slightly different from [`Self::size`] because the alpha component of 4-component formats
    /// are not read in an acceleration structure build, allowing for a smaller stride.
    #[must_use]
    pub const fn min_acceleration_structure_vertex_stride(&self) -> u64 {
        match self {
            Self::Float16x2 | Self::Snorm16x2 => 4,
            Self::Float32x3 => 12,
            Self::Float32x2 => 8,
            // This is the minimum value from DirectX
            // > A16 component is ignored, other data can be packed there, such as setting vertex stride to 6 bytes
            //
            // https://microsoft.github.io/DirectX-Specs/d3d/Raytracing.html#d3d12_raytracing_geometry_triangles_desc
            //
            // Vulkan does not express a minimum stride.
            Self::Float16x4 | Self::Snorm16x4 => 6,
            _ => unreachable!(),
        }
    }

    /// Returns the alignment required for `wgpu::BlasTriangleGeometry::vertex_stride`
    #[must_use]
    pub const fn acceleration_structure_stride_alignment(&self) -> u64 {
        match self {
            Self::Float16x4 | Self::Float16x2 | Self::Snorm16x4 | Self::Snorm16x2 => 2,
            Self::Float32x2 | Self::Float32x3 => 4,
            _ => unreachable!(),
        }
    }
}
