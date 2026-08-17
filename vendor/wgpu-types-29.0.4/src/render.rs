//! Types for configuring render passes and render pipelines (except for vertex attributes).

use bytemuck::{Pod, Zeroable};

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

use crate::{link_to_wgpu_docs, LoadOpDontCare};

#[cfg(doc)]
use crate::{Features, TextureFormat};

/// Alpha blend factor.
///
/// Corresponds to [WebGPU `GPUBlendFactor`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpublendfactor). Values using `Src1`
/// require [`Features::DUAL_SOURCE_BLENDING`] and can only be used with the first
/// render target.
///
/// For further details on how the blend factors are applied, see the analogous
/// functionality in OpenGL: <https://www.khronos.org/opengl/wiki/Blending#Blending_Parameters>.
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum BlendFactor {
    /// 0.0
    Zero = 0,
    /// 1.0
    One = 1,
    /// S.component
    Src = 2,
    /// 1.0 - S.component
    OneMinusSrc = 3,
    /// S.alpha
    SrcAlpha = 4,
    /// 1.0 - S.alpha
    OneMinusSrcAlpha = 5,
    /// D.component
    Dst = 6,
    /// 1.0 - D.component
    OneMinusDst = 7,
    /// D.alpha
    DstAlpha = 8,
    /// 1.0 - D.alpha
    OneMinusDstAlpha = 9,
    /// min(S.alpha, 1.0 - D.alpha)
    SrcAlphaSaturated = 10,
    /// Constant
    Constant = 11,
    /// 1.0 - Constant
    OneMinusConstant = 12,
    /// S1.component
    Src1 = 13,
    /// 1.0 - S1.component
    OneMinusSrc1 = 14,
    /// S1.alpha
    Src1Alpha = 15,
    /// 1.0 - S1.alpha
    OneMinusSrc1Alpha = 16,
}

impl BlendFactor {
    /// Returns `true` if the blend factor references the second blend source.
    ///
    /// Note that the usage of those blend factors require [`Features::DUAL_SOURCE_BLENDING`].
    #[must_use]
    pub fn ref_second_blend_source(&self) -> bool {
        match self {
            BlendFactor::Src1
            | BlendFactor::OneMinusSrc1
            | BlendFactor::Src1Alpha
            | BlendFactor::OneMinusSrc1Alpha => true,
            _ => false,
        }
    }
}

/// Alpha blend operation.
///
/// Corresponds to [WebGPU `GPUBlendOperation`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpublendoperation).
///
/// For further details on how the blend operations are applied, see
/// the analogous functionality in OpenGL: <https://www.khronos.org/opengl/wiki/Blending#Blend_Equations>.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum BlendOperation {
    /// Src + Dst
    #[default]
    Add = 0,
    /// Src - Dst
    Subtract = 1,
    /// Dst - Src
    ReverseSubtract = 2,
    /// min(Src, Dst)
    Min = 3,
    /// max(Src, Dst)
    Max = 4,
}

/// Describes a blend component of a [`BlendState`].
///
/// Corresponds to [WebGPU `GPUBlendComponent`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpublendcomponent).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct BlendComponent {
    /// Multiplier for the source, which is produced by the fragment shader.
    pub src_factor: BlendFactor,
    /// Multiplier for the destination, which is stored in the target.
    pub dst_factor: BlendFactor,
    /// The binary operation applied to the source and destination,
    /// multiplied by their respective factors.
    pub operation: BlendOperation,
}

impl BlendComponent {
    /// Default blending state that replaces destination with the source.
    pub const REPLACE: Self = Self {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::Zero,
        operation: BlendOperation::Add,
    };

    /// Blend state of `(1 * src) + ((1 - src_alpha) * dst)`.
    pub const OVER: Self = Self {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::OneMinusSrcAlpha,
        operation: BlendOperation::Add,
    };

    /// Returns true if the state relies on the constant color, which is
    /// set independently on a render command encoder.
    #[must_use]
    pub fn uses_constant(&self) -> bool {
        match (self.src_factor, self.dst_factor) {
            (BlendFactor::Constant, _)
            | (BlendFactor::OneMinusConstant, _)
            | (_, BlendFactor::Constant)
            | (_, BlendFactor::OneMinusConstant) => true,
            (_, _) => false,
        }
    }
}

impl Default for BlendComponent {
    fn default() -> Self {
        Self::REPLACE
    }
}

/// Describe the blend state of a render pipeline,
/// within [`ColorTargetState`].
///
/// Corresponds to [WebGPU `GPUBlendState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpublendstate).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct BlendState {
    /// Color equation.
    pub color: BlendComponent,
    /// Alpha equation.
    pub alpha: BlendComponent,
}

impl BlendState {
    /// Blend mode that does no color blending, just overwrites the output with the contents of the shader.
    pub const REPLACE: Self = Self {
        color: BlendComponent::REPLACE,
        alpha: BlendComponent::REPLACE,
    };

    /// Blend mode that does standard alpha blending with non-premultiplied alpha.
    pub const ALPHA_BLENDING: Self = Self {
        color: BlendComponent {
            src_factor: BlendFactor::SrcAlpha,
            dst_factor: BlendFactor::OneMinusSrcAlpha,
            operation: BlendOperation::Add,
        },
        alpha: BlendComponent::OVER,
    };

    /// Blend mode that does standard alpha blending with premultiplied alpha.
    pub const PREMULTIPLIED_ALPHA_BLENDING: Self = Self {
        color: BlendComponent::OVER,
        alpha: BlendComponent::OVER,
    };
}

/// Describes the color state of a render pipeline.
///
/// Corresponds to [WebGPU `GPUColorTargetState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpucolortargetstate).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ColorTargetState {
    /// The [`TextureFormat`] of the image that this pipeline will render to. Must match the format
    /// of the corresponding color attachment in [`CommandEncoder::begin_render_pass`][CEbrp]
    ///
    #[doc = link_to_wgpu_docs!(["CEbrp"]: "struct.CommandEncoder.html#method.begin_render_pass")]
    pub format: crate::TextureFormat,
    /// The blending that is used for this pipeline.
    #[cfg_attr(feature = "serde", serde(default))]
    pub blend: Option<BlendState>,
    /// Mask which enables/disables writes to different color/alpha channel.
    #[cfg_attr(feature = "serde", serde(default))]
    pub write_mask: ColorWrites,
}

impl From<crate::TextureFormat> for ColorTargetState {
    fn from(format: crate::TextureFormat) -> Self {
        Self {
            format,
            blend: None,
            write_mask: ColorWrites::ALL,
        }
    }
}

/// Color write mask. Disabled color channels will not be written to.
///
/// Corresponds to [WebGPU `GPUColorWriteFlags`](
/// https://gpuweb.github.io/gpuweb/#typedefdef-gpucolorwriteflags).
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ColorWrites(u32);

bitflags::bitflags! {
    impl ColorWrites: u32 {
        /// Enable red channel writes
        const RED = 1 << 0;
        /// Enable green channel writes
        const GREEN = 1 << 1;
        /// Enable blue channel writes
        const BLUE = 1 << 2;
        /// Enable alpha channel writes
        const ALPHA = 1 << 3;
        /// Enable red, green, and blue channel writes
        const COLOR = Self::RED.bits() | Self::GREEN.bits() | Self::BLUE.bits();
        /// Enable writes to all channels.
        const ALL = Self::RED.bits() | Self::GREEN.bits() | Self::BLUE.bits() | Self::ALPHA.bits();
    }
}

impl Default for ColorWrites {
    fn default() -> Self {
        Self::ALL
    }
}

/// Primitive type the input mesh is composed of.
///
/// Corresponds to [WebGPU `GPUPrimitiveTopology`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpuprimitivetopology).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum PrimitiveTopology {
    /// Vertex data is a list of points. Each vertex is a new point.
    PointList = 0,
    /// Vertex data is a list of lines. Each pair of vertices composes a new line.
    ///
    /// Vertices `0 1 2 3` create two lines `0 1` and `2 3`
    LineList = 1,
    /// Vertex data is a strip of lines. Each set of two adjacent vertices form a line.
    ///
    /// Vertices `0 1 2 3` create three lines `0 1`, `1 2`, and `2 3`.
    LineStrip = 2,
    /// Vertex data is a list of triangles. Each set of 3 vertices composes a new triangle.
    ///
    /// Vertices `0 1 2 3 4 5` create two triangles `0 1 2` and `3 4 5`
    #[default]
    TriangleList = 3,
    /// Vertex data is a triangle strip. Each set of three adjacent vertices form a triangle.
    ///
    /// Vertices `0 1 2 3 4 5` create four triangles `0 1 2`, `2 1 3`, `2 3 4`, and `4 3 5`
    TriangleStrip = 4,
}

impl PrimitiveTopology {
    /// Returns true for strip topologies.
    #[must_use]
    pub fn is_strip(&self) -> bool {
        match *self {
            Self::PointList | Self::LineList | Self::TriangleList => false,
            Self::LineStrip | Self::TriangleStrip => true,
        }
    }

    /// Returns true for triangle topologies.
    #[must_use]
    pub fn is_triangles(&self) -> bool {
        match *self {
            Self::TriangleList | Self::TriangleStrip => true,
            Self::PointList | Self::LineList | Self::LineStrip => false,
        }
    }
}

/// Vertex winding order which classifies the "front" face of a triangle.
///
/// Corresponds to [WebGPU `GPUFrontFace`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpufrontface).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum FrontFace {
    /// Triangles with vertices in counter clockwise order are considered the front face.
    ///
    /// This is the default with right handed coordinate spaces.
    #[default]
    Ccw = 0,
    /// Triangles with vertices in clockwise order are considered the front face.
    ///
    /// This is the default with left handed coordinate spaces.
    Cw = 1,
}

/// Face of a vertex.
///
/// Corresponds to [WebGPU `GPUCullMode`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpucullmode),
/// except that the `"none"` value is represented using `Option<Face>` instead.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum Face {
    /// Front face
    Front = 0,
    /// Back face
    Back = 1,
}

/// Type of drawing mode for polygons
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum PolygonMode {
    /// Polygons are filled
    #[default]
    Fill = 0,
    /// Polygons are drawn as line segments
    Line = 1,
    /// Polygons are drawn as points
    Point = 2,
}

/// Describes the state of primitive assembly and rasterization in a render pipeline.
///
/// Corresponds to [WebGPU `GPUPrimitiveState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuprimitivestate).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PrimitiveState {
    /// The primitive topology used to interpret vertices.
    pub topology: PrimitiveTopology,
    /// When drawing strip topologies with indices, this is the required format for the index buffer.
    /// This has no effect on non-indexed or non-strip draws.
    ///
    /// This is required for indexed drawing with strip topology and must match index buffer format, as primitive restart is always enabled
    /// in all backends and individual strips will be separated
    /// with the index value `0xFFFF` when using `Uint16`, or `0xFFFFFFFF` when using `Uint32`.
    #[cfg_attr(feature = "serde", serde(default))]
    pub strip_index_format: Option<IndexFormat>,
    /// The face to consider the front for the purpose of culling and stencil operations.
    #[cfg_attr(feature = "serde", serde(default))]
    pub front_face: FrontFace,
    /// The face culling mode.
    #[cfg_attr(feature = "serde", serde(default))]
    pub cull_mode: Option<Face>,
    /// If set to true, the polygon depth is not clipped to 0-1 before rasterization.
    ///
    /// Enabling this requires [`Features::DEPTH_CLIP_CONTROL`] to be enabled.
    #[cfg_attr(feature = "serde", serde(default))]
    pub unclipped_depth: bool,
    /// Controls the way each polygon is rasterized. Can be either `Fill` (default), `Line` or `Point`
    ///
    /// Setting this to `Line` requires [`Features::POLYGON_MODE_LINE`] to be enabled.
    ///
    /// Setting this to `Point` requires [`Features::POLYGON_MODE_POINT`] to be enabled.
    #[cfg_attr(feature = "serde", serde(default))]
    pub polygon_mode: PolygonMode,
    /// If set to true, the primitives are rendered with conservative overestimation. I.e. any rastered pixel touched by it is filled.
    /// Only valid for `[PolygonMode::Fill`]!
    ///
    /// Enabling this requires [`Features::CONSERVATIVE_RASTERIZATION`] to be enabled.
    pub conservative: bool,
}

/// Describes the multi-sampling state of a render pipeline.
///
/// Corresponds to [WebGPU `GPUMultisampleState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpumultisamplestate).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct MultisampleState {
    /// The number of samples calculated per pixel (for MSAA). For non-multisampled textures,
    /// this should be `1`
    pub count: u32,
    /// Bitmask that restricts the samples of a pixel modified by this pipeline. All samples
    /// can be enabled using the value `!0`
    pub mask: u64,
    /// When enabled, produces another sample mask per pixel based on the alpha output value, that
    /// is ANDed with the sample mask and the primitive coverage to restrict the set of samples
    /// affected by a primitive.
    ///
    /// The implicit mask produced for alpha of zero is guaranteed to be zero, and for alpha of one
    /// is guaranteed to be all 1-s.
    pub alpha_to_coverage_enabled: bool,
}

impl Default for MultisampleState {
    fn default() -> Self {
        MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        }
    }
}

/// Format of indices used with pipeline.
///
/// Corresponds to [WebGPU `GPUIndexFormat`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpuindexformat).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum IndexFormat {
    /// Indices are 16 bit unsigned integers.
    Uint16 = 0,
    /// Indices are 32 bit unsigned integers.
    #[default]
    Uint32 = 1,
}

impl IndexFormat {
    /// Returns the size in bytes of the index format
    pub fn byte_size(&self) -> usize {
        match self {
            IndexFormat::Uint16 => 2,
            IndexFormat::Uint32 => 4,
        }
    }
}

/// Operation to perform on the stencil value.
///
/// Corresponds to [WebGPU `GPUStencilOperation`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpustenciloperation).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum StencilOperation {
    /// Keep stencil value unchanged.
    #[default]
    Keep = 0,
    /// Set stencil value to zero.
    Zero = 1,
    /// Replace stencil value with value provided in most recent call to
    /// [`RenderPass::set_stencil_reference`][RPssr].
    ///
    #[doc = link_to_wgpu_docs!(["RPssr"]: "struct.RenderPass.html#method.set_stencil_reference")]
    Replace = 2,
    /// Bitwise inverts stencil value.
    Invert = 3,
    /// Increments stencil value by one, clamping on overflow.
    IncrementClamp = 4,
    /// Decrements stencil value by one, clamping on underflow.
    DecrementClamp = 5,
    /// Increments stencil value by one, wrapping on overflow.
    IncrementWrap = 6,
    /// Decrements stencil value by one, wrapping on underflow.
    DecrementWrap = 7,
}

/// Describes stencil state in a render pipeline.
///
/// If you are not using stencil state, set this to [`StencilFaceState::IGNORE`].
///
/// Corresponds to [WebGPU `GPUStencilFaceState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpustencilfacestate).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct StencilFaceState {
    /// Comparison function that determines if the fail_op or pass_op is used on the stencil buffer.
    pub compare: CompareFunction,
    /// Operation that is performed when stencil test fails.
    pub fail_op: StencilOperation,
    /// Operation that is performed when depth test fails but stencil test succeeds.
    pub depth_fail_op: StencilOperation,
    /// Operation that is performed when stencil test success.
    pub pass_op: StencilOperation,
}

impl StencilFaceState {
    /// Ignore the stencil state for the face.
    pub const IGNORE: Self = StencilFaceState {
        compare: CompareFunction::Always,
        fail_op: StencilOperation::Keep,
        depth_fail_op: StencilOperation::Keep,
        pass_op: StencilOperation::Keep,
    };

    /// Returns true if the face state uses the reference value for testing or operation.
    #[must_use]
    pub fn needs_ref_value(&self) -> bool {
        self.compare.needs_ref_value()
            || self.fail_op == StencilOperation::Replace
            || self.depth_fail_op == StencilOperation::Replace
            || self.pass_op == StencilOperation::Replace
    }

    /// Returns true if the face state doesn't mutate the target values.
    #[must_use]
    pub fn is_read_only(&self) -> bool {
        self.pass_op == StencilOperation::Keep
            && self.depth_fail_op == StencilOperation::Keep
            && self.fail_op == StencilOperation::Keep
    }
}

impl Default for StencilFaceState {
    fn default() -> Self {
        Self::IGNORE
    }
}

/// Comparison function used for depth and stencil operations.
///
/// Corresponds to [WebGPU `GPUCompareFunction`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpucomparefunction).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum CompareFunction {
    /// Function never passes
    Never = 1,
    /// Function passes if new value less than existing value
    Less = 2,
    /// Function passes if new value is equal to existing value. When using
    /// this compare function, make sure to mark your Vertex Shader's `@builtin(position)`
    /// output as `@invariant` to prevent artifacting.
    Equal = 3,
    /// Function passes if new value is less than or equal to existing value
    LessEqual = 4,
    /// Function passes if new value is greater than existing value
    Greater = 5,
    /// Function passes if new value is not equal to existing value. When using
    /// this compare function, make sure to mark your Vertex Shader's `@builtin(position)`
    /// output as `@invariant` to prevent artifacting.
    NotEqual = 6,
    /// Function passes if new value is greater than or equal to existing value
    GreaterEqual = 7,
    /// Function always passes
    #[default]
    Always = 8,
}

impl CompareFunction {
    /// Returns true if the comparison depends on the reference value.
    #[must_use]
    pub fn needs_ref_value(self) -> bool {
        match self {
            Self::Never | Self::Always => false,
            _ => true,
        }
    }
}

/// State of the stencil operation (fixed-pipeline stage).
///
/// For use in [`DepthStencilState`].
///
/// Corresponds to a portion of [WebGPU `GPUDepthStencilState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpudepthstencilstate).
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StencilState {
    /// Front face mode.
    pub front: StencilFaceState,
    /// Back face mode.
    pub back: StencilFaceState,
    /// Stencil values are AND'd with this mask when reading and writing from the stencil buffer. Only low 8 bits are used.
    pub read_mask: u32,
    /// Stencil values are AND'd with this mask when writing to the stencil buffer. Only low 8 bits are used.
    pub write_mask: u32,
}

impl StencilState {
    /// Returns true if the stencil test is enabled.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        (self.front != StencilFaceState::IGNORE || self.back != StencilFaceState::IGNORE)
            && (self.read_mask != 0 || self.write_mask != 0)
    }
    /// Returns true if the state doesn't mutate the target values.
    #[must_use]
    pub fn is_read_only(&self, cull_mode: Option<Face>) -> bool {
        // The rules are defined in step 7 of the "Device timeline initialization steps"
        // subsection of the "Render Pipeline Creation" section of WebGPU
        // (link to the section: https://gpuweb.github.io/gpuweb/#render-pipeline-creation)

        if self.write_mask == 0 {
            return true;
        }

        let front_ro = cull_mode == Some(Face::Front) || self.front.is_read_only();
        let back_ro = cull_mode == Some(Face::Back) || self.back.is_read_only();

        front_ro && back_ro
    }
    /// Returns true if the stencil state uses the reference value for testing.
    #[must_use]
    pub fn needs_ref_value(&self) -> bool {
        self.front.needs_ref_value() || self.back.needs_ref_value()
    }
}

/// Describes the biasing setting for the depth target.
///
/// For use in [`DepthStencilState`].
///
/// Corresponds to a portion of [WebGPU `GPUDepthStencilState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpudepthstencilstate).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DepthBiasState {
    /// Constant depth biasing factor, in basic units of the depth format.
    pub constant: i32,
    /// Slope depth biasing factor.
    pub slope_scale: f32,
    /// Depth bias clamp value (absolute).
    pub clamp: f32,
}

impl DepthBiasState {
    /// Returns true if the depth biasing is enabled.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.constant != 0 || self.slope_scale != 0.0
    }
}

impl core::hash::Hash for DepthBiasState {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.constant.hash(state);
        self.slope_scale.to_bits().hash(state);
        self.clamp.to_bits().hash(state);
    }
}

impl PartialEq for DepthBiasState {
    fn eq(&self, other: &Self) -> bool {
        (self.constant == other.constant)
            && (self.slope_scale.to_bits() == other.slope_scale.to_bits())
            && (self.clamp.to_bits() == other.clamp.to_bits())
    }
}

impl Eq for DepthBiasState {}

/// Operation to perform to the output attachment at the start of a render pass.
///
/// Corresponds to [WebGPU `GPULoadOp`](https://gpuweb.github.io/gpuweb/#enumdef-gpuloadop),
/// plus the corresponding clearValue.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum LoadOp<V> {
    /// Loads the specified value for this attachment into the render pass.
    ///
    /// On some GPU hardware (primarily mobile), "clear" is significantly cheaper
    /// because it avoids loading data from main memory into tile-local memory.
    ///
    /// On other GPU hardware, there isn’t a significant difference.
    ///
    /// As a result, it is recommended to use "clear" rather than "load" in cases
    /// where the initial value doesn’t matter
    /// (e.g. the render target will be cleared using a skybox).
    Clear(V) = 0,
    /// Loads the existing value for this attachment into the render pass.
    Load = 1,
    /// The render target has undefined contents at the start of the render pass.
    /// This may lead to undefined behavior if you read from the any of the
    /// render target pixels without first writing to them.
    ///
    /// Blending also becomes undefined behavior if the source
    /// pixels are undefined.
    ///
    /// This is the fastest option on all GPUs if you always overwrite all pixels
    /// in the render target after this load operation.
    ///
    /// Backends that don't support `DontCare` internally, will pick a different (unspecified)
    /// load op instead.
    ///
    /// # Safety
    ///
    /// - All pixels in the render target must be written to before
    ///   any read or a [`StoreOp::Store`] occurs.
    DontCare(#[cfg_attr(feature = "serde", serde(skip))] LoadOpDontCare) = 2,
}

impl<V> LoadOp<V> {
    /// Returns true if variants are same (ignoring clear value)
    pub fn eq_variant<T>(&self, other: LoadOp<T>) -> bool {
        matches!(
            (self, other),
            (LoadOp::Clear(_), LoadOp::Clear(_))
                | (LoadOp::Load, LoadOp::Load)
                | (LoadOp::DontCare(_), LoadOp::DontCare(_))
        )
    }
}

impl<V: Default> Default for LoadOp<V> {
    fn default() -> Self {
        Self::Clear(Default::default())
    }
}

/// Operation to perform to the output attachment at the end of a render pass.
///
/// Corresponds to [WebGPU `GPUStoreOp`](https://gpuweb.github.io/gpuweb/#enumdef-gpustoreop).
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum StoreOp {
    /// Stores the resulting value of the render pass for this attachment.
    #[default]
    Store = 0,
    /// Discards the resulting value of the render pass for this attachment.
    ///
    /// The attachment will be treated as uninitialized afterwards.
    /// (If only either Depth or Stencil texture-aspects is set to `Discard`,
    /// the respective other texture-aspect will be preserved.)
    ///
    /// This can be significantly faster on tile-based render hardware.
    ///
    /// Prefer this if the attachment is not read by subsequent passes.
    Discard = 1,
}

/// Pair of load and store operations for an attachment aspect.
///
/// This type is unique to the Rust API of `wgpu`. In the WebGPU specification,
/// separate `loadOp` and `storeOp` fields are used instead.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Operations<V> {
    /// How data should be read through this attachment.
    pub load: LoadOp<V>,
    /// Whether data will be written to through this attachment.
    ///
    /// Note that resolve textures (if specified) are always written to,
    /// regardless of this setting.
    pub store: StoreOp,
}

impl<V: Default> Default for Operations<V> {
    #[inline]
    fn default() -> Self {
        Self {
            load: LoadOp::<V>::default(),
            store: StoreOp::default(),
        }
    }
}

/// Describes the depth/stencil state in a render pipeline.
///
/// Corresponds to [WebGPU `GPUDepthStencilState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpudepthstencilstate).
#[repr(C)]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DepthStencilState {
    /// Format of the depth/stencil buffer, must be special depth format. Must match the format
    /// of the depth/stencil attachment in [`CommandEncoder::begin_render_pass`][CEbrp].
    ///
    #[doc = link_to_wgpu_docs!(["CEbrp"]: "struct.CommandEncoder.html#method.begin_render_pass")]
    pub format: crate::TextureFormat,
    /// Whether to write updated depth values to the depth attachment.
    ///
    /// If `format` is a depth or depth/stencil format, then this must be `Some`.
    /// Otherwise, specifying `None` is preferred, but `Some(false)` is also
    /// accepted.
    pub depth_write_enabled: Option<bool>,
    /// Comparison function used to compare depth values in the depth test.
    ///
    /// If `depth_write_enabled` is `Some(true)` or if `depth_fail_op` for either
    /// stencil face is not `Keep`, then this must be `Some`. Otherwise, specifying
    /// `None` is preferred, but `Some(CompareFunction::Always)` is also accepted.
    pub depth_compare: Option<CompareFunction>,
    /// Stencil state.
    #[cfg_attr(feature = "serde", serde(default))]
    pub stencil: StencilState,
    /// Depth bias state.
    #[cfg_attr(feature = "serde", serde(default))]
    pub bias: DepthBiasState,
}

impl DepthStencilState {
    /// Construct `DepthStencilState` for a stencil operation with no depth operation.
    ///
    /// Panics if `format` does not have a stencil aspect.
    pub fn stencil(format: crate::TextureFormat, stencil: StencilState) -> DepthStencilState {
        assert!(
            format.has_stencil_aspect(),
            "{format:?} is not a stencil format"
        );
        DepthStencilState {
            format,
            depth_write_enabled: None,
            depth_compare: None,
            stencil,
            bias: DepthBiasState::default(),
        }
    }

    /// Returns true if the depth testing is enabled.
    #[must_use]
    pub fn is_depth_enabled(&self) -> bool {
        self.depth_compare.unwrap_or_default() != CompareFunction::Always
            || self.depth_write_enabled.unwrap_or_default()
    }

    /// Returns true if the state doesn't mutate the depth buffer.
    #[must_use]
    pub fn is_depth_read_only(&self) -> bool {
        !self.depth_write_enabled.unwrap_or_default()
    }

    /// Returns true if the state doesn't mutate the stencil.
    #[must_use]
    pub fn is_stencil_read_only(&self, cull_mode: Option<Face>) -> bool {
        self.stencil.is_read_only(cull_mode)
    }

    /// Returns true if the state doesn't mutate either depth or stencil of the target.
    #[must_use]
    pub fn is_read_only(&self, cull_mode: Option<Face>) -> bool {
        self.is_depth_read_only() && self.is_stencil_read_only(cull_mode)
    }
}

/// Describes the depth/stencil attachment for render bundles.
///
/// Corresponds to a portion of [WebGPU `GPURenderBundleEncoderDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpurenderbundleencoderdescriptor).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderBundleDepthStencil {
    /// Format of the attachment.
    pub format: crate::TextureFormat,
    /// If the depth aspect of the depth stencil attachment is going to be written to.
    ///
    /// This must match the [`RenderPassDepthStencilAttachment::depth_ops`] of the renderpass this render bundle is executed in.
    /// If `depth_ops` is `Some(..)` this must be false. If it is `None` this must be true.
    ///
    #[doc = link_to_wgpu_docs!(["`RenderPassDepthStencilAttachment::depth_ops`"]: "struct.RenderPassDepthStencilAttachment.html#structfield.depth_ops")]
    pub depth_read_only: bool,

    /// If the stencil aspect of the depth stencil attachment is going to be written to.
    ///
    /// This must match the [`RenderPassDepthStencilAttachment::stencil_ops`] of the renderpass this render bundle is executed in.
    /// If `depth_ops` is `Some(..)` this must be false. If it is `None` this must be true.
    ///
    #[doc = link_to_wgpu_docs!(["`RenderPassDepthStencilAttachment::stencil_ops`"]: "struct.RenderPassDepthStencilAttachment.html#structfield.stencil_ops")]
    pub stencil_read_only: bool,
}

/// Describes a [`RenderBundle`](../wgpu/struct.RenderBundle.html).
///
/// Corresponds to [WebGPU `GPURenderBundleDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpurenderbundledescriptor).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderBundleDescriptor<L> {
    /// Debug label of the render bundle encoder. This will show up in graphics debuggers for easy identification.
    pub label: L,
}

impl<L> RenderBundleDescriptor<L> {
    /// Takes a closure and maps the label of the render bundle descriptor into another.
    #[must_use]
    pub fn map_label<K>(&self, fun: impl FnOnce(&L) -> K) -> RenderBundleDescriptor<K> {
        RenderBundleDescriptor {
            label: fun(&self.label),
        }
    }
}

impl<T> Default for RenderBundleDescriptor<Option<T>> {
    fn default() -> Self {
        Self { label: None }
    }
}

/// Argument buffer layout for `draw_indirect` commands.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct DrawIndirectArgs {
    /// The number of vertices to draw.
    pub vertex_count: u32,
    /// The number of instances to draw.
    pub instance_count: u32,
    /// The Index of the first vertex to draw.
    pub first_vertex: u32,
    /// The instance ID of the first instance to draw.
    ///
    /// Has to be 0, unless [`Features::INDIRECT_FIRST_INSTANCE`](crate::Features::INDIRECT_FIRST_INSTANCE) is enabled.
    pub first_instance: u32,
}

impl DrawIndirectArgs {
    /// Returns the bytes representation of the struct, ready to be written in a buffer.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

/// Argument buffer layout for `draw_indexed_indirect` commands.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct DrawIndexedIndirectArgs {
    /// The number of indices to draw.
    pub index_count: u32,
    /// The number of instances to draw.
    pub instance_count: u32,
    /// The first index within the index buffer.
    pub first_index: u32,
    /// The value added to the vertex index before indexing into the vertex buffer.
    pub base_vertex: i32,
    /// The instance ID of the first instance to draw.
    ///
    /// Has to be 0, unless [`Features::INDIRECT_FIRST_INSTANCE`](crate::Features::INDIRECT_FIRST_INSTANCE) is enabled.
    pub first_instance: u32,
}

impl DrawIndexedIndirectArgs {
    /// Returns the bytes representation of the struct, ready to be written in a buffer.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

/// Argument buffer layout for `dispatch_indirect` commands.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct DispatchIndirectArgs {
    /// The number of work groups in X dimension.
    pub x: u32,
    /// The number of work groups in Y dimension.
    pub y: u32,
    /// The number of work groups in Z dimension.
    pub z: u32,
}

impl DispatchIndirectArgs {
    /// Returns the bytes representation of the struct, ready to be written into a buffer.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}
