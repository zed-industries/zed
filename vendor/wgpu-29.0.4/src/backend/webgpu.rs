#![allow(clippy::type_complexity)]

mod defined_non_null_js_value;
mod ext_bindings;
#[allow(clippy::allow_attributes)]
mod webgpu_sys;

use alloc::{
    boxed::Box,
    format,
    rc::Rc,
    string::{String, ToString as _},
    sync::Arc,
    vec,
    vec::Vec,
};
use core::{
    cell::{Cell, OnceCell, RefCell},
    fmt,
    future::Future,
    ops::Range,
    pin::Pin,
    task::{self, Poll},
};
use wgt::Backends;

use js_sys::Promise;
use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    dispatch::{self, BlasCompactCallback},
    Blas, SurfaceTargetUnsafe, Tlas, WriteOnly,
};

use defined_non_null_js_value::DefinedNonNullJsValue;

// We need to mark various types as Send and Sync to satisfy the Rust type system.
//
// SAFETY: All webgpu handle types in wasm32 are internally a `JsValue`, and `JsValue` is neither
// Send nor Sync.  Currently, wasm32 has no threading support by default, so implementing `Send` or
// `Sync` for a type is harmless. However, nightly Rust supports compiling wasm with experimental
// threading support via `--target-features`. If `wgpu` is being compiled with those features, we do
// not implement `Send` and `Sync` on the webgpu handle types.
macro_rules! impl_send_sync {
    ($name:ty) => {
        #[cfg(send_sync)]
        unsafe impl Send for $name {}
        #[cfg(send_sync)]
        unsafe impl Sync for $name {}
    };
}

#[derive(Clone)]
pub struct ContextWebGpu {
    /// `None` if browser does not advertise support for WebGPU.
    gpu: Option<DefinedNonNullJsValue<webgpu_sys::Gpu>>,
    /// Unique identifier for this context.
    ident: crate::cmp::Identifier,
    /// Backends requested in the [`crate::InstanceDescriptor`].
    /// Remembered for error reporting even though this itself is strictly
    /// [`Backends::BROWSER_WEBGPU`].
    requested_backends: Backends,
}

impl fmt::Debug for ContextWebGpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContextWebGpu")
            .field("type", &"Web")
            .finish()
    }
}

impl crate::Error {
    fn from_js(js_error: js_sys::Object) -> Self {
        let source = Box::<dyn core::error::Error + Send + Sync>::from("<WebGPU Error>");
        if let Some(js_error) = js_error.dyn_ref::<webgpu_sys::GpuValidationError>() {
            crate::Error::Validation {
                source,
                description: js_error.message(),
            }
        } else if js_error.has_type::<webgpu_sys::GpuOutOfMemoryError>() {
            crate::Error::OutOfMemory { source }
        } else {
            panic!("Unexpected error");
        }
    }
}

#[derive(Debug, Clone)]
pub struct WebShaderModule {
    module: webgpu_sys::GpuShaderModule,
    compilation_info: WebShaderCompilationInfo,
    /// Unique identifier for this shader module.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
enum WebShaderCompilationInfo {
    /// WGSL shaders get their compilation info from a native WebGPU function.
    /// We need the source to be able to do UTF16 to UTF8 location remapping.
    Wgsl { source: String },
    /// Transformed shaders get their compilation info from the transformer.
    /// Further compilation errors are reported without a span.
    Transformed {
        compilation_info: crate::CompilationInfo,
    },
}

fn map_utf16_to_utf8_offset(utf16_offset: u32, text: &str) -> u32 {
    let mut utf16_i = 0;
    for (utf8_index, c) in text.char_indices() {
        if utf16_i >= utf16_offset {
            return utf8_index as u32;
        }
        utf16_i += c.len_utf16() as u32;
    }
    if utf16_i >= utf16_offset {
        text.len() as u32
    } else {
        log::error!("UTF16 offset {utf16_offset} is out of bounds for string {text}");
        u32::MAX
    }
}

impl crate::CompilationMessage {
    fn from_js(
        js_message: webgpu_sys::GpuCompilationMessage,
        compilation_info: &WebShaderCompilationInfo,
    ) -> Self {
        let message_type = match js_message.type_() {
            webgpu_sys::GpuCompilationMessageType::Error => crate::CompilationMessageType::Error,
            webgpu_sys::GpuCompilationMessageType::Warning => {
                crate::CompilationMessageType::Warning
            }
            webgpu_sys::GpuCompilationMessageType::Info => crate::CompilationMessageType::Info,
            _ => crate::CompilationMessageType::Error,
        };
        let utf16_offset = js_message.offset() as u32;
        let utf16_length = js_message.length() as u32;
        let span = match compilation_info {
            WebShaderCompilationInfo::Wgsl { .. } if utf16_offset == 0 && utf16_length == 0 => None,
            WebShaderCompilationInfo::Wgsl { source } => {
                let offset = map_utf16_to_utf8_offset(utf16_offset, source);
                let length = map_utf16_to_utf8_offset(utf16_length, &source[offset as usize..]);
                let line_number = js_message.line_num() as u32; // That's legal, because we're counting lines the same way

                let prefix = &source[..offset as usize];
                let line_start = prefix.rfind('\n').map(|pos| pos + 1).unwrap_or(0) as u32;
                let line_position = offset - line_start + 1; // Counting UTF-8 byte indices

                Some(crate::SourceLocation {
                    offset,
                    length,
                    line_number,
                    line_position,
                })
            }
            WebShaderCompilationInfo::Transformed { .. } => None,
        };

        crate::CompilationMessage {
            message: js_message.message(),
            message_type,
            location: span,
        }
    }
}

// We need to assert that any future we return is Send to match the native API.
//
// This is safe on wasm32 *for now*, but similarly to the unsafe Send impls for the handle type
// wrappers, the full story for threading on wasm32 is still unfolding.

pub(crate) struct MakeSendFuture<F, M> {
    future: F,
    map: M,
}

impl<F: Future, M: Fn(F::Output) -> T, T> Future for MakeSendFuture<F, M> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        // This is safe because we have no Drop implementation to violate the Pin requirements and
        // do not provide any means of moving the inner future.
        unsafe {
            let this = self.get_unchecked_mut();
            match Pin::new_unchecked(&mut this.future).poll(cx) {
                task::Poll::Ready(value) => task::Poll::Ready((this.map)(value)),
                task::Poll::Pending => task::Poll::Pending,
            }
        }
    }
}

impl<F, M> MakeSendFuture<F, M> {
    fn new(future: F, map: M) -> Self {
        Self { future, map }
    }
}

#[cfg(send_sync)]
unsafe impl<F, M> Send for MakeSendFuture<F, M> {}

fn map_texture_format(texture_format: wgt::TextureFormat) -> webgpu_sys::GpuTextureFormat {
    use webgpu_sys::GpuTextureFormat as tf;
    use wgt::TextureFormat;
    match texture_format {
        // 8-bit formats
        TextureFormat::R8Unorm => tf::R8unorm,
        TextureFormat::R8Snorm => tf::R8snorm,
        TextureFormat::R8Uint => tf::R8uint,
        TextureFormat::R8Sint => tf::R8sint,
        // 16-bit formats
        TextureFormat::R16Uint => tf::R16uint,
        TextureFormat::R16Sint => tf::R16sint,
        TextureFormat::R16Float => tf::R16float,
        TextureFormat::Rg8Unorm => tf::Rg8unorm,
        TextureFormat::Rg8Snorm => tf::Rg8snorm,
        TextureFormat::Rg8Uint => tf::Rg8uint,
        TextureFormat::Rg8Sint => tf::Rg8sint,
        // 32-bit formats
        TextureFormat::R32Uint => tf::R32uint,
        TextureFormat::R32Sint => tf::R32sint,
        TextureFormat::R32Float => tf::R32float,
        TextureFormat::Rg16Uint => tf::Rg16uint,
        TextureFormat::Rg16Sint => tf::Rg16sint,
        TextureFormat::Rg16Float => tf::Rg16float,
        TextureFormat::Rgba8Unorm => tf::Rgba8unorm,
        TextureFormat::Rgba8UnormSrgb => tf::Rgba8unormSrgb,
        TextureFormat::Rgba8Snorm => tf::Rgba8snorm,
        TextureFormat::Rgba8Uint => tf::Rgba8uint,
        TextureFormat::Rgba8Sint => tf::Rgba8sint,
        TextureFormat::Bgra8Unorm => tf::Bgra8unorm,
        TextureFormat::Bgra8UnormSrgb => tf::Bgra8unormSrgb,
        // Packed 32-bit formats
        TextureFormat::Rgb9e5Ufloat => tf::Rgb9e5ufloat,
        TextureFormat::Rgb10a2Uint => tf::Rgb10a2uint,
        TextureFormat::Rgb10a2Unorm => tf::Rgb10a2unorm,
        TextureFormat::Rg11b10Ufloat => tf::Rg11b10ufloat,
        // 64-bit formats
        TextureFormat::Rg32Uint => tf::Rg32uint,
        TextureFormat::Rg32Sint => tf::Rg32sint,
        TextureFormat::Rg32Float => tf::Rg32float,
        TextureFormat::Rgba16Uint => tf::Rgba16uint,
        TextureFormat::Rgba16Sint => tf::Rgba16sint,
        TextureFormat::Rgba16Float => tf::Rgba16float,
        // 128-bit formats
        TextureFormat::Rgba32Uint => tf::Rgba32uint,
        TextureFormat::Rgba32Sint => tf::Rgba32sint,
        TextureFormat::Rgba32Float => tf::Rgba32float,
        // Depth/stencil formats
        TextureFormat::Stencil8 => tf::Stencil8,
        TextureFormat::Depth16Unorm => tf::Depth16unorm,
        TextureFormat::Depth24Plus => tf::Depth24plus,
        TextureFormat::Depth24PlusStencil8 => tf::Depth24plusStencil8,
        TextureFormat::Depth32Float => tf::Depth32float,
        // "depth32float-stencil8" feature
        TextureFormat::Depth32FloatStencil8 => tf::Depth32floatStencil8,

        TextureFormat::Bc1RgbaUnorm => tf::Bc1RgbaUnorm,
        TextureFormat::Bc1RgbaUnormSrgb => tf::Bc1RgbaUnormSrgb,
        TextureFormat::Bc2RgbaUnorm => tf::Bc2RgbaUnorm,
        TextureFormat::Bc2RgbaUnormSrgb => tf::Bc2RgbaUnormSrgb,
        TextureFormat::Bc3RgbaUnorm => tf::Bc3RgbaUnorm,
        TextureFormat::Bc3RgbaUnormSrgb => tf::Bc3RgbaUnormSrgb,
        TextureFormat::Bc4RUnorm => tf::Bc4RUnorm,
        TextureFormat::Bc4RSnorm => tf::Bc4RSnorm,
        TextureFormat::Bc5RgUnorm => tf::Bc5RgUnorm,
        TextureFormat::Bc5RgSnorm => tf::Bc5RgSnorm,
        TextureFormat::Bc6hRgbUfloat => tf::Bc6hRgbUfloat,
        TextureFormat::Bc6hRgbFloat => tf::Bc6hRgbFloat,
        TextureFormat::Bc7RgbaUnorm => tf::Bc7RgbaUnorm,
        TextureFormat::Bc7RgbaUnormSrgb => tf::Bc7RgbaUnormSrgb,
        TextureFormat::Etc2Rgb8Unorm => tf::Etc2Rgb8unorm,
        TextureFormat::Etc2Rgb8UnormSrgb => tf::Etc2Rgb8unormSrgb,
        TextureFormat::Etc2Rgb8A1Unorm => tf::Etc2Rgb8a1unorm,
        TextureFormat::Etc2Rgb8A1UnormSrgb => tf::Etc2Rgb8a1unormSrgb,
        TextureFormat::Etc2Rgba8Unorm => tf::Etc2Rgba8unorm,
        TextureFormat::Etc2Rgba8UnormSrgb => tf::Etc2Rgba8unormSrgb,
        TextureFormat::EacR11Unorm => tf::EacR11unorm,
        TextureFormat::EacR11Snorm => tf::EacR11snorm,
        TextureFormat::EacRg11Unorm => tf::EacRg11unorm,
        TextureFormat::EacRg11Snorm => tf::EacRg11snorm,
        TextureFormat::Astc { block, channel } => match channel {
            wgt::AstcChannel::Unorm => match block {
                wgt::AstcBlock::B4x4 => tf::Astc4x4Unorm,
                wgt::AstcBlock::B5x4 => tf::Astc5x4Unorm,
                wgt::AstcBlock::B5x5 => tf::Astc5x5Unorm,
                wgt::AstcBlock::B6x5 => tf::Astc6x5Unorm,
                wgt::AstcBlock::B6x6 => tf::Astc6x6Unorm,
                wgt::AstcBlock::B8x5 => tf::Astc8x5Unorm,
                wgt::AstcBlock::B8x6 => tf::Astc8x6Unorm,
                wgt::AstcBlock::B8x8 => tf::Astc8x8Unorm,
                wgt::AstcBlock::B10x5 => tf::Astc10x5Unorm,
                wgt::AstcBlock::B10x6 => tf::Astc10x6Unorm,
                wgt::AstcBlock::B10x8 => tf::Astc10x8Unorm,
                wgt::AstcBlock::B10x10 => tf::Astc10x10Unorm,
                wgt::AstcBlock::B12x10 => tf::Astc12x10Unorm,
                wgt::AstcBlock::B12x12 => tf::Astc12x12Unorm,
            },
            wgt::AstcChannel::UnormSrgb => match block {
                wgt::AstcBlock::B4x4 => tf::Astc4x4UnormSrgb,
                wgt::AstcBlock::B5x4 => tf::Astc5x4UnormSrgb,
                wgt::AstcBlock::B5x5 => tf::Astc5x5UnormSrgb,
                wgt::AstcBlock::B6x5 => tf::Astc6x5UnormSrgb,
                wgt::AstcBlock::B6x6 => tf::Astc6x6UnormSrgb,
                wgt::AstcBlock::B8x5 => tf::Astc8x5UnormSrgb,
                wgt::AstcBlock::B8x6 => tf::Astc8x6UnormSrgb,
                wgt::AstcBlock::B8x8 => tf::Astc8x8UnormSrgb,
                wgt::AstcBlock::B10x5 => tf::Astc10x5UnormSrgb,
                wgt::AstcBlock::B10x6 => tf::Astc10x6UnormSrgb,
                wgt::AstcBlock::B10x8 => tf::Astc10x8UnormSrgb,
                wgt::AstcBlock::B10x10 => tf::Astc10x10UnormSrgb,
                wgt::AstcBlock::B12x10 => tf::Astc12x10UnormSrgb,
                wgt::AstcBlock::B12x12 => tf::Astc12x12UnormSrgb,
            },
            wgt::AstcChannel::Hdr => {
                unimplemented!("Format {texture_format:?} has no WebGPU equivalent")
            }
        },
        _ => unimplemented!("Format {texture_format:?} has no WebGPU equivalent"),
    }
}

fn map_texture_component_type(
    sample_type: wgt::TextureSampleType,
) -> webgpu_sys::GpuTextureSampleType {
    use webgpu_sys::GpuTextureSampleType as ts;
    use wgt::TextureSampleType;
    match sample_type {
        TextureSampleType::Float { filterable: true } => ts::Float,
        TextureSampleType::Float { filterable: false } => ts::UnfilterableFloat,
        TextureSampleType::Sint => ts::Sint,
        TextureSampleType::Uint => ts::Uint,
        TextureSampleType::Depth => ts::Depth,
    }
}

fn map_cull_mode(cull_mode: Option<wgt::Face>) -> webgpu_sys::GpuCullMode {
    use webgpu_sys::GpuCullMode as cm;
    use wgt::Face;
    match cull_mode {
        None => cm::None,
        Some(Face::Front) => cm::Front,
        Some(Face::Back) => cm::Back,
    }
}

fn map_front_face(front_face: wgt::FrontFace) -> webgpu_sys::GpuFrontFace {
    use webgpu_sys::GpuFrontFace as ff;
    use wgt::FrontFace;
    match front_face {
        FrontFace::Ccw => ff::Ccw,
        FrontFace::Cw => ff::Cw,
    }
}

fn map_primitive_state(primitive: &wgt::PrimitiveState) -> webgpu_sys::GpuPrimitiveState {
    use webgpu_sys::GpuPrimitiveTopology as pt;
    use wgt::PrimitiveTopology;

    let mapped = webgpu_sys::GpuPrimitiveState::new();
    mapped.set_cull_mode(map_cull_mode(primitive.cull_mode));
    mapped.set_front_face(map_front_face(primitive.front_face));

    if let Some(format) = primitive.strip_index_format {
        mapped.set_strip_index_format(map_index_format(format));
    }

    mapped.set_topology(match primitive.topology {
        PrimitiveTopology::PointList => pt::PointList,
        PrimitiveTopology::LineList => pt::LineList,
        PrimitiveTopology::LineStrip => pt::LineStrip,
        PrimitiveTopology::TriangleList => pt::TriangleList,
        PrimitiveTopology::TriangleStrip => pt::TriangleStrip,
    });

    mapped.set_unclipped_depth(primitive.unclipped_depth);

    match primitive.polygon_mode {
        wgt::PolygonMode::Fill => {}
        wgt::PolygonMode::Line => panic!(
            "{:?} is not enabled for this backend",
            wgt::Features::POLYGON_MODE_LINE
        ),
        wgt::PolygonMode::Point => panic!(
            "{:?} is not enabled for this backend",
            wgt::Features::POLYGON_MODE_POINT
        ),
    }

    mapped
}

fn map_compare_function(compare_fn: wgt::CompareFunction) -> webgpu_sys::GpuCompareFunction {
    use webgpu_sys::GpuCompareFunction as cf;
    use wgt::CompareFunction;
    match compare_fn {
        CompareFunction::Never => cf::Never,
        CompareFunction::Less => cf::Less,
        CompareFunction::Equal => cf::Equal,
        CompareFunction::LessEqual => cf::LessEqual,
        CompareFunction::Greater => cf::Greater,
        CompareFunction::NotEqual => cf::NotEqual,
        CompareFunction::GreaterEqual => cf::GreaterEqual,
        CompareFunction::Always => cf::Always,
    }
}

fn map_stencil_operation(op: wgt::StencilOperation) -> webgpu_sys::GpuStencilOperation {
    use webgpu_sys::GpuStencilOperation as so;
    use wgt::StencilOperation;
    match op {
        StencilOperation::Keep => so::Keep,
        StencilOperation::Zero => so::Zero,
        StencilOperation::Replace => so::Replace,
        StencilOperation::Invert => so::Invert,
        StencilOperation::IncrementClamp => so::IncrementClamp,
        StencilOperation::DecrementClamp => so::DecrementClamp,
        StencilOperation::IncrementWrap => so::IncrementWrap,
        StencilOperation::DecrementWrap => so::DecrementWrap,
    }
}

fn map_stencil_state_face(desc: &wgt::StencilFaceState) -> webgpu_sys::GpuStencilFaceState {
    let mapped = webgpu_sys::GpuStencilFaceState::new();
    mapped.set_compare(map_compare_function(desc.compare));
    mapped.set_depth_fail_op(map_stencil_operation(desc.depth_fail_op));
    mapped.set_fail_op(map_stencil_operation(desc.fail_op));
    mapped.set_pass_op(map_stencil_operation(desc.pass_op));
    mapped
}

fn map_depth_stencil_state(desc: &wgt::DepthStencilState) -> webgpu_sys::GpuDepthStencilState {
    let mapped = webgpu_sys::GpuDepthStencilState::new(map_texture_format(desc.format));
    if let Some(compare) = desc.depth_compare {
        mapped.set_depth_compare(map_compare_function(compare));
    }
    if let Some(write_enabled) = desc.depth_write_enabled {
        mapped.set_depth_write_enabled(write_enabled);
    }
    mapped.set_depth_bias(desc.bias.constant);
    mapped.set_depth_bias_clamp(desc.bias.clamp);
    mapped.set_depth_bias_slope_scale(desc.bias.slope_scale);
    mapped.set_stencil_back(&map_stencil_state_face(&desc.stencil.back));
    mapped.set_stencil_front(&map_stencil_state_face(&desc.stencil.front));
    mapped.set_stencil_read_mask(desc.stencil.read_mask);
    mapped.set_stencil_write_mask(desc.stencil.write_mask);
    mapped
}

fn map_blend_component(desc: &wgt::BlendComponent) -> webgpu_sys::GpuBlendComponent {
    let mapped = webgpu_sys::GpuBlendComponent::new();
    mapped.set_dst_factor(map_blend_factor(desc.dst_factor));
    mapped.set_operation(map_blend_operation(desc.operation));
    mapped.set_src_factor(map_blend_factor(desc.src_factor));
    mapped
}

fn map_blend_factor(factor: wgt::BlendFactor) -> webgpu_sys::GpuBlendFactor {
    use webgpu_sys::GpuBlendFactor as bf;
    use wgt::BlendFactor;
    match factor {
        BlendFactor::Zero => bf::Zero,
        BlendFactor::One => bf::One,
        BlendFactor::Src => bf::Src,
        BlendFactor::OneMinusSrc => bf::OneMinusSrc,
        BlendFactor::SrcAlpha => bf::SrcAlpha,
        BlendFactor::OneMinusSrcAlpha => bf::OneMinusSrcAlpha,
        BlendFactor::Dst => bf::Dst,
        BlendFactor::OneMinusDst => bf::OneMinusDst,
        BlendFactor::DstAlpha => bf::DstAlpha,
        BlendFactor::OneMinusDstAlpha => bf::OneMinusDstAlpha,
        BlendFactor::SrcAlphaSaturated => bf::SrcAlphaSaturated,
        BlendFactor::Constant => bf::Constant,
        BlendFactor::OneMinusConstant => bf::OneMinusConstant,
        BlendFactor::Src1 => bf::Src1,
        BlendFactor::OneMinusSrc1 => bf::OneMinusSrc1,
        BlendFactor::Src1Alpha => bf::Src1Alpha,
        BlendFactor::OneMinusSrc1Alpha => bf::OneMinusSrc1Alpha,
    }
}

fn map_blend_operation(op: wgt::BlendOperation) -> webgpu_sys::GpuBlendOperation {
    use webgpu_sys::GpuBlendOperation as bo;
    use wgt::BlendOperation;
    match op {
        BlendOperation::Add => bo::Add,
        BlendOperation::Subtract => bo::Subtract,
        BlendOperation::ReverseSubtract => bo::ReverseSubtract,
        BlendOperation::Min => bo::Min,
        BlendOperation::Max => bo::Max,
    }
}

fn map_index_format(format: wgt::IndexFormat) -> webgpu_sys::GpuIndexFormat {
    use webgpu_sys::GpuIndexFormat as f;
    use wgt::IndexFormat;
    match format {
        IndexFormat::Uint16 => f::Uint16,
        IndexFormat::Uint32 => f::Uint32,
    }
}

fn map_vertex_format(format: wgt::VertexFormat) -> webgpu_sys::GpuVertexFormat {
    use webgpu_sys::GpuVertexFormat as vf;
    use wgt::VertexFormat;
    match format {
        VertexFormat::Uint8 => vf::Uint8,
        VertexFormat::Uint8x2 => vf::Uint8x2,
        VertexFormat::Uint8x4 => vf::Uint8x4,
        VertexFormat::Sint8 => vf::Sint8,
        VertexFormat::Sint8x2 => vf::Sint8x2,
        VertexFormat::Sint8x4 => vf::Sint8x4,
        VertexFormat::Unorm8 => vf::Unorm8,
        VertexFormat::Unorm8x2 => vf::Unorm8x2,
        VertexFormat::Unorm8x4 => vf::Unorm8x4,
        VertexFormat::Snorm8 => vf::Snorm8,
        VertexFormat::Snorm8x2 => vf::Snorm8x2,
        VertexFormat::Snorm8x4 => vf::Snorm8x4,
        VertexFormat::Uint16 => vf::Uint16,
        VertexFormat::Uint16x2 => vf::Uint16x2,
        VertexFormat::Uint16x4 => vf::Uint16x4,
        VertexFormat::Sint16 => vf::Sint16,
        VertexFormat::Sint16x2 => vf::Sint16x2,
        VertexFormat::Sint16x4 => vf::Sint16x4,
        VertexFormat::Unorm16 => vf::Unorm16,
        VertexFormat::Unorm16x2 => vf::Unorm16x2,
        VertexFormat::Unorm16x4 => vf::Unorm16x4,
        VertexFormat::Snorm16 => vf::Snorm16,
        VertexFormat::Snorm16x2 => vf::Snorm16x2,
        VertexFormat::Snorm16x4 => vf::Snorm16x4,
        VertexFormat::Float16 => vf::Float16,
        VertexFormat::Float16x2 => vf::Float16x2,
        VertexFormat::Float16x4 => vf::Float16x4,
        VertexFormat::Float32 => vf::Float32,
        VertexFormat::Float32x2 => vf::Float32x2,
        VertexFormat::Float32x3 => vf::Float32x3,
        VertexFormat::Float32x4 => vf::Float32x4,
        VertexFormat::Uint32 => vf::Uint32,
        VertexFormat::Uint32x2 => vf::Uint32x2,
        VertexFormat::Uint32x3 => vf::Uint32x3,
        VertexFormat::Uint32x4 => vf::Uint32x4,
        VertexFormat::Sint32 => vf::Sint32,
        VertexFormat::Sint32x2 => vf::Sint32x2,
        VertexFormat::Sint32x3 => vf::Sint32x3,
        VertexFormat::Sint32x4 => vf::Sint32x4,
        VertexFormat::Unorm10_10_10_2 => vf::Unorm1010102,
        VertexFormat::Unorm8x4Bgra => vf::Unorm8x4Bgra,
        VertexFormat::Float64
        | VertexFormat::Float64x2
        | VertexFormat::Float64x3
        | VertexFormat::Float64x4 => {
            panic!("VERTEX_ATTRIBUTE_64BIT feature must be enabled to use Double formats")
        }
    }
}

fn map_vertex_step_mode(mode: wgt::VertexStepMode) -> webgpu_sys::GpuVertexStepMode {
    use webgpu_sys::GpuVertexStepMode as sm;
    use wgt::VertexStepMode;
    match mode {
        VertexStepMode::Vertex => sm::Vertex,
        VertexStepMode::Instance => sm::Instance,
    }
}

fn map_extent_3d(extent: wgt::Extent3d) -> webgpu_sys::GpuExtent3dDict {
    let mapped = webgpu_sys::GpuExtent3dDict::new(extent.width);
    mapped.set_height(extent.height);
    mapped.set_depth_or_array_layers(extent.depth_or_array_layers);
    mapped
}

fn map_origin_2d(extent: wgt::Origin2d) -> webgpu_sys::GpuOrigin2dDict {
    let mapped = webgpu_sys::GpuOrigin2dDict::new();
    mapped.set_x(extent.x);
    mapped.set_y(extent.y);
    mapped
}

fn map_origin_3d(origin: wgt::Origin3d) -> webgpu_sys::GpuOrigin3dDict {
    let mapped = webgpu_sys::GpuOrigin3dDict::new();
    mapped.set_x(origin.x);
    mapped.set_y(origin.y);
    mapped.set_z(origin.z);
    mapped
}

fn map_texture_dimension(
    texture_dimension: wgt::TextureDimension,
) -> webgpu_sys::GpuTextureDimension {
    match texture_dimension {
        wgt::TextureDimension::D1 => webgpu_sys::GpuTextureDimension::N1d,
        wgt::TextureDimension::D2 => webgpu_sys::GpuTextureDimension::N2d,
        wgt::TextureDimension::D3 => webgpu_sys::GpuTextureDimension::N3d,
    }
}

fn map_texture_view_dimension(
    texture_view_dimension: wgt::TextureViewDimension,
) -> webgpu_sys::GpuTextureViewDimension {
    use webgpu_sys::GpuTextureViewDimension as tvd;
    match texture_view_dimension {
        wgt::TextureViewDimension::D1 => tvd::N1d,
        wgt::TextureViewDimension::D2 => tvd::N2d,
        wgt::TextureViewDimension::D2Array => tvd::N2dArray,
        wgt::TextureViewDimension::Cube => tvd::Cube,
        wgt::TextureViewDimension::CubeArray => tvd::CubeArray,
        wgt::TextureViewDimension::D3 => tvd::N3d,
    }
}

fn map_buffer_copy_view(
    view: crate::TexelCopyBufferInfo<'_>,
) -> webgpu_sys::GpuTexelCopyBufferInfo {
    let buffer = view.buffer.inner.as_webgpu();
    let mapped = webgpu_sys::GpuTexelCopyBufferInfo::new(&buffer.inner);
    if let Some(bytes_per_row) = view.layout.bytes_per_row {
        mapped.set_bytes_per_row(bytes_per_row);
    }
    if let Some(rows_per_image) = view.layout.rows_per_image {
        mapped.set_rows_per_image(rows_per_image);
    }
    mapped.set_offset(view.layout.offset as f64);
    mapped
}

fn map_texture_copy_view(
    view: crate::TexelCopyTextureInfo<'_>,
) -> webgpu_sys::GpuTexelCopyTextureInfo {
    let texture = view.texture.inner.as_webgpu();
    let mapped = webgpu_sys::GpuTexelCopyTextureInfo::new(&texture.inner);
    mapped.set_mip_level(view.mip_level);
    mapped.set_origin(&map_origin_3d(view.origin));
    mapped.set_aspect(map_texture_aspect(view.aspect));
    mapped
}

fn map_tagged_texture_copy_view(
    view: crate::CopyExternalImageDestInfo<&crate::api::Texture>,
) -> webgpu_sys::GpuCopyExternalImageDestInfo {
    let texture = view.texture.inner.as_webgpu();
    let mapped = webgpu_sys::GpuCopyExternalImageDestInfo::new(&texture.inner);
    mapped.set_mip_level(view.mip_level);
    mapped.set_origin(&map_origin_3d(view.origin));
    mapped.set_aspect(map_texture_aspect(view.aspect));
    // mapped.set_color_space(map_color_space(view.color_space));
    mapped.set_premultiplied_alpha(view.premultiplied_alpha);
    mapped
}

fn map_external_texture_copy_view(
    view: &crate::CopyExternalImageSourceInfo,
) -> webgpu_sys::GpuCopyExternalImageSourceInfo {
    let mapped = webgpu_sys::GpuCopyExternalImageSourceInfo::new(&view.source);
    mapped.set_origin(&map_origin_2d(view.origin));
    mapped.set_flip_y(view.flip_y);
    mapped
}

fn map_texture_aspect(aspect: wgt::TextureAspect) -> webgpu_sys::GpuTextureAspect {
    match aspect {
        wgt::TextureAspect::All => webgpu_sys::GpuTextureAspect::All,
        wgt::TextureAspect::StencilOnly => webgpu_sys::GpuTextureAspect::StencilOnly,
        wgt::TextureAspect::DepthOnly => webgpu_sys::GpuTextureAspect::DepthOnly,
        wgt::TextureAspect::Plane0 | wgt::TextureAspect::Plane1 | wgt::TextureAspect::Plane2 => {
            panic!("multi-plane textures are not supported")
        }
    }
}

fn map_filter_mode(mode: wgt::FilterMode) -> webgpu_sys::GpuFilterMode {
    match mode {
        wgt::FilterMode::Nearest => webgpu_sys::GpuFilterMode::Nearest,
        wgt::FilterMode::Linear => webgpu_sys::GpuFilterMode::Linear,
    }
}

fn map_mipmap_filter_mode(mode: wgt::MipmapFilterMode) -> webgpu_sys::GpuMipmapFilterMode {
    match mode {
        wgt::MipmapFilterMode::Nearest => webgpu_sys::GpuMipmapFilterMode::Nearest,
        wgt::MipmapFilterMode::Linear => webgpu_sys::GpuMipmapFilterMode::Linear,
    }
}

fn map_address_mode(mode: wgt::AddressMode) -> webgpu_sys::GpuAddressMode {
    match mode {
        wgt::AddressMode::ClampToEdge => webgpu_sys::GpuAddressMode::ClampToEdge,
        wgt::AddressMode::Repeat => webgpu_sys::GpuAddressMode::Repeat,
        wgt::AddressMode::MirrorRepeat => webgpu_sys::GpuAddressMode::MirrorRepeat,
        wgt::AddressMode::ClampToBorder => panic!("Clamp to border is not supported"),
    }
}

fn map_color(color: wgt::Color) -> webgpu_sys::GpuColorDict {
    webgpu_sys::GpuColorDict::new(color.a, color.b, color.g, color.r)
}

fn map_store_op(store: crate::StoreOp) -> webgpu_sys::GpuStoreOp {
    match store {
        crate::StoreOp::Store => webgpu_sys::GpuStoreOp::Store,
        crate::StoreOp::Discard => webgpu_sys::GpuStoreOp::Discard,
    }
}

fn map_map_mode(mode: crate::MapMode) -> u32 {
    match mode {
        crate::MapMode::Read => webgpu_sys::gpu_map_mode::READ,
        crate::MapMode::Write => webgpu_sys::gpu_map_mode::WRITE,
    }
}

const FEATURES_MAPPING: [(wgt::Features, webgpu_sys::GpuFeatureName); 16] = [
    (
        wgt::Features::DEPTH_CLIP_CONTROL,
        webgpu_sys::GpuFeatureName::DepthClipControl,
    ),
    (
        wgt::Features::DEPTH32FLOAT_STENCIL8,
        webgpu_sys::GpuFeatureName::Depth32floatStencil8,
    ),
    (
        wgt::Features::TEXTURE_COMPRESSION_BC,
        webgpu_sys::GpuFeatureName::TextureCompressionBc,
    ),
    (
        wgt::Features::TEXTURE_COMPRESSION_BC_SLICED_3D,
        webgpu_sys::GpuFeatureName::TextureCompressionBcSliced3d,
    ),
    (
        wgt::Features::TEXTURE_COMPRESSION_ETC2,
        webgpu_sys::GpuFeatureName::TextureCompressionEtc2,
    ),
    (
        wgt::Features::TEXTURE_COMPRESSION_ASTC,
        webgpu_sys::GpuFeatureName::TextureCompressionAstc,
    ),
    (
        wgt::Features::TEXTURE_COMPRESSION_ASTC_SLICED_3D,
        webgpu_sys::GpuFeatureName::TextureCompressionAstcSliced3d,
    ),
    (
        wgt::Features::TIMESTAMP_QUERY,
        webgpu_sys::GpuFeatureName::TimestampQuery,
    ),
    (
        wgt::Features::INDIRECT_FIRST_INSTANCE,
        webgpu_sys::GpuFeatureName::IndirectFirstInstance,
    ),
    (
        wgt::Features::SHADER_F16,
        webgpu_sys::GpuFeatureName::ShaderF16,
    ),
    (
        wgt::Features::RG11B10UFLOAT_RENDERABLE,
        webgpu_sys::GpuFeatureName::Rg11b10ufloatRenderable,
    ),
    (
        wgt::Features::BGRA8UNORM_STORAGE,
        webgpu_sys::GpuFeatureName::Bgra8unormStorage,
    ),
    (
        wgt::Features::FLOAT32_FILTERABLE,
        webgpu_sys::GpuFeatureName::Float32Filterable,
    ),
    (
        wgt::Features::FLOAT32_BLENDABLE,
        webgpu_sys::GpuFeatureName::Float32Blendable,
    ),
    (
        wgt::Features::DUAL_SOURCE_BLENDING,
        webgpu_sys::GpuFeatureName::DualSourceBlending,
    ),
    (
        wgt::Features::CLIP_DISTANCES,
        webgpu_sys::GpuFeatureName::ClipDistances,
    ),
];

fn map_wgt_features(supported_features: webgpu_sys::GpuSupportedFeatures) -> wgt::Features {
    let mut features = wgt::Features::empty();
    for (wgpu_feat, web_feat) in FEATURES_MAPPING {
        match wasm_bindgen::JsValue::from(web_feat).as_string() {
            Some(value) if supported_features.has(&value) => features |= wgpu_feat,
            _ => {}
        }
    }
    features
}

fn map_wgt_limits(limits: webgpu_sys::GpuSupportedLimits) -> wgt::Limits {
    wgt::Limits {
        max_texture_dimension_1d: limits.max_texture_dimension_1d(),
        max_texture_dimension_2d: limits.max_texture_dimension_2d(),
        max_texture_dimension_3d: limits.max_texture_dimension_3d(),
        max_texture_array_layers: limits.max_texture_array_layers(),
        max_bind_groups: limits.max_bind_groups(),
        max_bindings_per_bind_group: limits.max_bindings_per_bind_group(),
        max_dynamic_uniform_buffers_per_pipeline_layout: limits
            .max_dynamic_uniform_buffers_per_pipeline_layout(),
        max_dynamic_storage_buffers_per_pipeline_layout: limits
            .max_dynamic_storage_buffers_per_pipeline_layout(),
        max_sampled_textures_per_shader_stage: limits.max_sampled_textures_per_shader_stage(),
        max_samplers_per_shader_stage: limits.max_samplers_per_shader_stage(),
        max_storage_buffers_per_shader_stage: limits.max_storage_buffers_per_shader_stage(),
        max_storage_textures_per_shader_stage: limits.max_storage_textures_per_shader_stage(),
        max_uniform_buffers_per_shader_stage: limits.max_uniform_buffers_per_shader_stage(),
        max_binding_array_elements_per_shader_stage: 0,
        max_binding_array_sampler_elements_per_shader_stage: 0,
        max_binding_array_acceleration_structure_elements_per_shader_stage: 0,
        max_uniform_buffer_binding_size: limits.max_uniform_buffer_binding_size() as u64,
        max_storage_buffer_binding_size: limits.max_storage_buffer_binding_size() as u64,
        max_vertex_buffers: limits.max_vertex_buffers(),
        max_buffer_size: limits.max_buffer_size() as u64,
        max_vertex_attributes: limits.max_vertex_attributes(),
        max_vertex_buffer_array_stride: limits.max_vertex_buffer_array_stride(),
        max_inter_stage_shader_variables: limits.max_inter_stage_shader_variables(),
        min_uniform_buffer_offset_alignment: limits.min_uniform_buffer_offset_alignment(),
        min_storage_buffer_offset_alignment: limits.min_storage_buffer_offset_alignment(),
        max_color_attachments: limits.max_color_attachments(),
        max_color_attachment_bytes_per_sample: limits.max_color_attachment_bytes_per_sample(),
        max_compute_workgroup_storage_size: limits.max_compute_workgroup_storage_size(),
        max_compute_invocations_per_workgroup: limits.max_compute_invocations_per_workgroup(),
        max_compute_workgroup_size_x: limits.max_compute_workgroup_size_x(),
        max_compute_workgroup_size_y: limits.max_compute_workgroup_size_y(),
        max_compute_workgroup_size_z: limits.max_compute_workgroup_size_z(),
        max_compute_workgroups_per_dimension: limits.max_compute_workgroups_per_dimension(),
        max_immediate_size: wgt::Limits::default().max_immediate_size,
        max_non_sampler_bindings: wgt::Limits::default().max_non_sampler_bindings,

        max_task_mesh_workgroup_total_count: wgt::Limits::default()
            .max_task_mesh_workgroup_total_count,
        max_task_mesh_workgroups_per_dimension: wgt::Limits::default()
            .max_task_mesh_workgroups_per_dimension,
        max_task_invocations_per_workgroup: wgt::Limits::default()
            .max_task_invocations_per_workgroup,
        max_task_invocations_per_dimension: wgt::Limits::default()
            .max_task_invocations_per_dimension,
        max_mesh_invocations_per_workgroup: wgt::Limits::default()
            .max_mesh_invocations_per_workgroup,
        max_mesh_invocations_per_dimension: wgt::Limits::default()
            .max_mesh_invocations_per_dimension,
        max_task_payload_size: wgt::Limits::default().max_task_payload_size,
        max_mesh_output_vertices: wgt::Limits::default().max_mesh_output_vertices,
        max_mesh_output_primitives: wgt::Limits::default().max_mesh_output_primitives,
        max_mesh_output_layers: wgt::Limits::default().max_mesh_output_layers,
        max_mesh_multiview_view_count: wgt::Limits::default().max_mesh_multiview_view_count,

        max_blas_primitive_count: wgt::Limits::default().max_blas_primitive_count,
        max_blas_geometry_count: wgt::Limits::default().max_blas_geometry_count,
        max_tlas_instance_count: wgt::Limits::default().max_tlas_instance_count,
        max_acceleration_structures_per_shader_stage: wgt::Limits::default()
            .max_acceleration_structures_per_shader_stage,

        max_multiview_view_count: wgt::Limits::default().max_multiview_view_count,
    }
}

fn map_adapter_info(adapter_info: &webgpu_sys::GpuAdapterInfo) -> wgt::AdapterInfo {
    // TODO(https://github.com/gfx-rs/wgpu/issues/8819): populate more fields if/when possible
    wgt::AdapterInfo {
        name: adapter_info.description().to_string(),
        vendor: 0,
        device: 0,
        device_type: wgt::DeviceType::Other,
        device_pci_bus_id: String::new(),
        driver: String::new(),
        driver_info: String::new(),
        backend: wgt::Backend::BrowserWebGpu,
        subgroup_min_size: wgt::MINIMUM_SUBGROUP_MIN_SIZE,
        subgroup_max_size: wgt::MAXIMUM_SUBGROUP_MAX_SIZE,
        transient_saves_memory: false,
    }
}

fn map_js_sys_limits(limits: &wgt::Limits) -> js_sys::Object {
    let object = js_sys::Object::new();

    macro_rules! set_properties {
        (($from:expr) => ($on:expr) : $(($js_ident:ident, $rs_ident:ident)),* $(,)?) => {
            $(
                ::js_sys::Reflect::set(
                    &$on,
                    &::wasm_bindgen::JsValue::from(stringify!($js_ident)),
                    // Numbers may be u64, however using `from` on a u64 yields
                    // errors on the wasm side, since it uses an unsupported api.
                    // Wasm sends us things that need to fit into u64s by sending
                    // us f64s instead. So we just send them f64s back.
                    &::wasm_bindgen::JsValue::from($from.$rs_ident as f64)
                )
                    .expect("Setting Object properties should never fail.");
            )*
        }
    }

    // https://gpuweb.github.io/gpuweb/#gpusupportedlimits
    set_properties![
        (limits) => (object):
        (maxTextureDimension1D, max_texture_dimension_1d),
        (maxTextureDimension2D, max_texture_dimension_2d),
        (maxTextureDimension3D, max_texture_dimension_3d),
        (maxTextureArrayLayers, max_texture_array_layers),
        (maxBindGroups, max_bind_groups),
        // TODO: (maxBindGroupsPlusVertexBuffers, max_bind_groups_plus_vertex_buffers),
        (maxBindingsPerBindGroup, max_bindings_per_bind_group),
        (maxDynamicUniformBuffersPerPipelineLayout, max_dynamic_uniform_buffers_per_pipeline_layout),
        (maxDynamicStorageBuffersPerPipelineLayout, max_dynamic_storage_buffers_per_pipeline_layout),
        (maxSampledTexturesPerShaderStage, max_sampled_textures_per_shader_stage),
        (maxSamplersPerShaderStage, max_samplers_per_shader_stage),
        (maxStorageBuffersPerShaderStage, max_storage_buffers_per_shader_stage),
        (maxStorageTexturesPerShaderStage, max_storage_textures_per_shader_stage),
        (maxUniformBuffersPerShaderStage, max_uniform_buffers_per_shader_stage),
        (maxUniformBufferBindingSize, max_uniform_buffer_binding_size),
        (maxStorageBufferBindingSize, max_storage_buffer_binding_size),
        (minUniformBufferOffsetAlignment, min_uniform_buffer_offset_alignment),
        (minStorageBufferOffsetAlignment, min_storage_buffer_offset_alignment),
        (maxVertexBuffers, max_vertex_buffers),
        (maxBufferSize, max_buffer_size),
        (maxVertexAttributes, max_vertex_attributes),
        (maxVertexBufferArrayStride, max_vertex_buffer_array_stride),
        (maxInterStageShaderVariables, max_inter_stage_shader_variables),
        (maxColorAttachments, max_color_attachments),
        (maxColorAttachmentBytesPerSample, max_color_attachment_bytes_per_sample),
        (maxComputeWorkgroupStorageSize, max_compute_workgroup_storage_size),
        (maxComputeInvocationsPerWorkgroup, max_compute_invocations_per_workgroup),
        (maxComputeWorkgroupSizeX, max_compute_workgroup_size_x),
        (maxComputeWorkgroupSizeY, max_compute_workgroup_size_y),
        (maxComputeWorkgroupSizeZ, max_compute_workgroup_size_z),
        (maxComputeWorkgroupsPerDimension, max_compute_workgroups_per_dimension),
    ];

    object
}

type JsFutureResult = Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>;

fn future_request_adapter(
    result: JsFutureResult,
    requested_backends: Backends,
) -> Result<dispatch::DispatchAdapter, wgt::RequestAdapterError> {
    let web_adapter: Option<webgpu_sys::GpuAdapter> =
        result.and_then(wasm_bindgen::JsCast::dyn_into).ok();
    web_adapter
        .map(|adapter| {
            WebAdapter {
                inner: adapter,
                ident: crate::cmp::Identifier::create(),
            }
            .into()
        })
        .ok_or_else(|| request_adapter_null_error(requested_backends))
}

// Translate WebGPU’s null return into our error.
fn request_adapter_null_error(requested_backends: Backends) -> wgt::RequestAdapterError {
    wgt::RequestAdapterError::NotFound {
        active_backends: Backends::BROWSER_WEBGPU,
        requested_backends,
        // TODO: supported_backends should also include wgpu-core-based backends,
        // if they were compiled in.
        supported_backends: Backends::BROWSER_WEBGPU,
        no_fallback_backends: Backends::empty(),
        no_adapter_backends: Backends::BROWSER_WEBGPU,
        incompatible_surface_backends: Backends::empty(),
    }
}

fn future_request_device(
    result: JsFutureResult,
) -> Result<(dispatch::DispatchDevice, dispatch::DispatchQueue), crate::RequestDeviceError> {
    result
        .map(|js_value| {
            let device = webgpu_sys::GpuDevice::from(js_value);
            let queue = device.queue();

            (
                WebDevice {
                    inner: device,
                    ident: crate::cmp::Identifier::create(),
                    error_scope_count: Rc::new(Cell::new(0)),
                }
                .into(),
                WebQueue {
                    inner: queue,
                    ident: crate::cmp::Identifier::create(),
                }
                .into(),
            )
        })
        .map_err(|error_value| crate::RequestDeviceError {
            // wasm-bindgen provides a reasonable error stringification via `Debug` impl
            inner: crate::RequestDeviceErrorKind::WebGpu(format!("{error_value:?}")),
        })
}

fn future_pop_error_scope(result: JsFutureResult) -> Option<crate::Error> {
    match result {
        Ok(js_value) if js_value.is_object() => {
            let js_error = wasm_bindgen::JsCast::dyn_into(js_value).unwrap();
            Some(crate::Error::from_js(js_error))
        }
        _ => None,
    }
}

fn future_compilation_info(
    result: JsFutureResult,
    base_compilation_info: &WebShaderCompilationInfo,
) -> crate::CompilationInfo {
    let base_messages = match base_compilation_info {
        WebShaderCompilationInfo::Transformed { compilation_info } => {
            compilation_info.messages.iter().cloned()
        }
        _ => [].iter().cloned(),
    };

    let messages = match result {
        Ok(js_value) => {
            let info = webgpu_sys::GpuCompilationInfo::from(js_value);
            base_messages
                .chain(info.messages().into_iter().map(|message| {
                    crate::CompilationMessage::from_js(
                        webgpu_sys::GpuCompilationMessage::from(message),
                        base_compilation_info,
                    )
                }))
                .collect()
        }
        Err(_v) => base_messages
            .chain(core::iter::once(crate::CompilationMessage {
                message: "Getting compilation info failed".to_string(),
                message_type: crate::CompilationMessageType::Error,
                location: None,
            }))
            .collect(),
    };

    crate::CompilationInfo { messages }
}

/// Calls `callback(success_value)` when the promise completes successfully, calls `callback(failure_value)`
/// when the promise completes unsuccessfully.
fn register_then_closures<F, T>(promise: &Promise, callback: F, success_value: T, failure_value: T)
where
    F: FnOnce(T) + 'static,
    T: 'static,
{
    // Both the 'success' and 'rejected' closures need access to callback, but only one
    // of them will ever run. We have them both hold a reference to a `Rc<RefCell<Option<impl FnOnce...>>>`,
    // and then take ownership of callback when invoked.
    //
    // We also only need Rc's because these will only ever be called on our thread.
    //
    // We also store the actual closure types inside this Rc, as the closures need to be kept alive
    // until they are actually called by the callback. It is valid to drop a closure inside of a callback.
    // This allows us to keep the closures alive without leaking them.
    let rc_callback: Rc<RefCell<Option<(_, _, F)>>> = Rc::new(RefCell::new(None));

    let rc_callback_clone1 = rc_callback.clone();
    let rc_callback_clone2 = rc_callback.clone();
    let closure_success = wasm_bindgen::closure::Closure::once(move |_| {
        let (success_closure, rejection_closure, callback) =
            rc_callback_clone1.borrow_mut().take().unwrap();
        callback(success_value);
        // drop the closures, including ourselves, which will free any captured memory.
        drop((success_closure, rejection_closure));
    });
    let closure_rejected = wasm_bindgen::closure::Closure::once(move |_| {
        let (success_closure, rejection_closure, callback) =
            rc_callback_clone2.borrow_mut().take().unwrap();
        callback(failure_value);
        // drop the closures, including ourselves, which will free any captured memory.
        drop((success_closure, rejection_closure));
    });

    // Calling then before setting the value in the Rc seems like a race, but it isn't
    // because the promise callback will run on this thread, so there is no race.
    let _ = promise.then2(&closure_success, &closure_rejected);

    *rc_callback.borrow_mut() = Some((closure_success, closure_rejected, callback));
}

impl ContextWebGpu {
    /// Common portion of the internal branches of the public `instance_create_surface` function.
    ///
    /// Note: Analogous code also exists in the WebGL2 backend at
    /// `wgpu_hal::gles::web::Instance`.
    fn create_surface_from_context(
        &self,
        canvas: Canvas,
        context_result: Result<Option<js_sys::Object>, wasm_bindgen::JsValue>,
    ) -> Result<dispatch::DispatchSurface, crate::CreateSurfaceError> {
        let context: js_sys::Object = match context_result {
            Ok(Some(context)) => context,
            Ok(None) => {
                // <https://html.spec.whatwg.org/multipage/canvas.html#dom-canvas-getcontext-dev>
                // A getContext() call “returns null if contextId is not supported, or if the
                // canvas has already been initialized with another context type”. Additionally,
                // “not supported” could include “insufficient GPU resources” or “the GPU process
                // previously crashed”. So, we must return it as an `Err` since it could occur
                // for circumstances outside the application author's control.
                return Err(crate::CreateSurfaceError {
                    inner: crate::CreateSurfaceErrorKind::Web(
                        String::from(
                            "canvas.getContext() returned null; webgpu not available or canvas already in use"
                        )
                    )
                });
            }
            Err(js_error) => {
                // <https://html.spec.whatwg.org/multipage/canvas.html#dom-canvas-getcontext>
                // A thrown exception indicates misuse of the canvas state.
                return Err(crate::CreateSurfaceError {
                    inner: crate::CreateSurfaceErrorKind::Web(format!(
                        "canvas.getContext() threw exception {js_error:?}",
                    )),
                });
            }
        };

        // Not returning this error because it is a type error that shouldn't happen unless
        // the browser, JS builtin objects, or wasm bindings are misbehaving somehow.
        let context: webgpu_sys::GpuCanvasContext = context
            .dyn_into()
            .expect("canvas context is not a GPUCanvasContext");

        Ok(WebSurface {
            gpu: self.gpu.clone(),
            context,
            canvas,
            ident: crate::cmp::Identifier::create(),
        }
        .into())
    }
}

// Represents the global object in the JavaScript context.
// It can be cast to from `webgpu_sys::global` and exposes two getters `window` and `worker` of which only one is defined depending on the caller's context.
// When called from the UI thread only `window` is defined whereas `worker` is only defined within a web worker context.
// See: https://github.com/rustwasm/gloo/blob/2c9e776701ecb90c53e62dec1abd19c2b70e47c7/crates/timers/src/callback.rs#L8-L40
#[wasm_bindgen]
extern "C" {
    type Global;

    #[wasm_bindgen(method, getter, js_name = Window)]
    fn window(this: &Global) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = WorkerGlobalScope)]
    fn worker(this: &Global) -> JsValue;
}

#[derive(Debug, Clone)]
pub enum Canvas {
    Canvas(web_sys::HtmlCanvasElement),
    Offscreen(web_sys::OffscreenCanvas),
}

#[derive(Debug, Clone, Copy)]
pub struct BrowserGpuPropertyInaccessible;

/// Returns the browser's gpu object or `Err(BrowserGpuPropertyInaccessible)` if
/// the current context is neither the main thread nor a dedicated worker.
///
/// If WebGPU is not supported, the Gpu property may (!) be `undefined`,
/// and so this function will return `Ok(None)`.
/// Note that this check is insufficient to determine whether WebGPU is
/// supported, as the browser may define the Gpu property, but be unable to
/// create any WebGPU adapters.
/// To detect whether WebGPU is supported, use the [`crate::utils::is_browser_webgpu_supported`] function.
///
/// See:
/// * <https://developer.mozilla.org/en-US/docs/Web/API/Navigator/gpu>
/// * <https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/gpu>
pub fn get_browser_gpu_property(
) -> Result<Option<DefinedNonNullJsValue<webgpu_sys::Gpu>>, BrowserGpuPropertyInaccessible> {
    let global: Global = js_sys::global().unchecked_into();

    let maybe_undefined_gpu: webgpu_sys::Gpu = if !global.window().is_undefined() {
        let navigator = global.unchecked_into::<web_sys::Window>().navigator();
        ext_bindings::NavigatorGpu::gpu(&navigator)
    } else if !global.worker().is_undefined() {
        let navigator = global
            .unchecked_into::<web_sys::WorkerGlobalScope>()
            .navigator();
        ext_bindings::NavigatorGpu::gpu(&navigator)
    } else {
        return Err(BrowserGpuPropertyInaccessible);
    };
    Ok(DefinedNonNullJsValue::new(maybe_undefined_gpu))
}

#[derive(Debug, Clone)]
pub struct WebAdapter {
    pub(crate) inner: webgpu_sys::GpuAdapter,
    /// Unique identifier for this Adapter.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebDevice {
    pub(crate) inner: webgpu_sys::GpuDevice,
    /// Unique identifier for this Device.
    ident: crate::cmp::Identifier,
    /// Current number of error scopes that have been pushed on the device.
    error_scope_count: Rc<Cell<u32>>,
}

#[derive(Debug, Clone)]
pub struct WebQueue {
    pub(crate) inner: webgpu_sys::GpuQueue,
    /// Unique identifier for this Queue.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebBindGroupLayout {
    pub(crate) inner: webgpu_sys::GpuBindGroupLayout,
    /// Unique identifier for this BindGroupLayout.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebBindGroup {
    pub(crate) inner: webgpu_sys::GpuBindGroup,
    /// Unique identifier for this BindGroup.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebTextureView {
    pub(crate) inner: webgpu_sys::GpuTextureView,
    /// Unique identifier for this TextureView.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebSampler {
    pub(crate) inner: webgpu_sys::GpuSampler,
    /// Unique identifier for this Sampler.
    ident: crate::cmp::Identifier,
}

/// Remembers which portion of a buffer has been mapped, along with a reference
/// to the mapped portion.
#[derive(Debug, Clone)]
struct WebBufferMapState {
    /// The mapped memory of the buffer.
    pub mapped_buffer: Option<js_sys::ArrayBuffer>,
    /// The total range which has been mapped in the buffer overall.
    pub range: Range<wgt::BufferAddress>,
}

/// Stores the state of a GPU buffer and a reference to its mapped `ArrayBuffer` (if any).
/// The WebGPU specification forbids calling `getMappedRange` on a `webgpu_sys::GpuBuffer` more than
/// once, so this struct stores the initial mapped range and re-uses it, allowing for multiple `get_mapped_range`
/// calls on the Rust-side.
#[derive(Debug, Clone)]
pub struct WebBuffer {
    /// The associated GPU buffer.
    inner: webgpu_sys::GpuBuffer,
    /// The mapped array buffer and mapped range.
    mapping: Rc<RefCell<WebBufferMapState>>,
    /// Unique identifier for this Buffer.
    ident: crate::cmp::Identifier,
}

impl WebBuffer {
    /// Creates a new web buffer for the given Javascript object and description.
    fn new(inner: webgpu_sys::GpuBuffer, desc: &crate::BufferDescriptor<'_>) -> Self {
        Self {
            inner,
            mapping: Rc::new(RefCell::new(WebBufferMapState {
                mapped_buffer: None,
                range: 0..desc.size,
            })),
            ident: crate::cmp::Identifier::create(),
        }
    }

    /// Obtains a reference to the re-usable buffer mapping as a Javascript array view.
    fn get_mapped_range(&self, sub_range: Range<wgt::BufferAddress>) -> js_sys::Uint8Array {
        let mut mapping = self.mapping.borrow_mut();
        let range = mapping.range.clone();
        let array_buffer = mapping.mapped_buffer.get_or_insert_with(|| {
            self.inner
                .get_mapped_range_with_f64_and_f64(
                    range.start as f64,
                    (range.end - range.start) as f64,
                )
                .unwrap()
        });
        js_sys::Uint8Array::new_with_byte_offset_and_length(
            array_buffer,
            (sub_range.start - range.start) as u32,
            (sub_range.end - sub_range.start) as u32,
        )
    }

    /// Sets the range of the buffer which is presently mapped.
    fn set_mapped_range(&self, range: Range<wgt::BufferAddress>) {
        self.mapping.borrow_mut().range = range;
    }
}

#[derive(Debug, Clone)]
pub struct WebTexture {
    pub(crate) inner: webgpu_sys::GpuTexture,
    /// Unique identifier for this Texture.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebExternalTexture {
    /// Unique identifier for this ExternalTexture.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebBlas {
    /// Unique identifier for this Blas.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebTlas {
    /// Unique identifier for this Blas.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebQuerySet {
    pub(crate) inner: webgpu_sys::GpuQuerySet,
    /// Unique identifier for this QuerySet.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebPipelineLayout {
    pub(crate) inner: webgpu_sys::GpuPipelineLayout,
    /// Unique identifier for this PipelineLayout.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebRenderPipeline {
    pub(crate) inner: webgpu_sys::GpuRenderPipeline,
    /// Unique identifier for this RenderPipeline.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebComputePipeline {
    pub(crate) inner: webgpu_sys::GpuComputePipeline,
    /// Unique identifier for this ComputePipeline.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebPipelineCache {
    /// Unique identifier for this PipelineCache.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebCommandEncoder {
    pub(crate) inner: webgpu_sys::GpuCommandEncoder,
    /// Unique identifier for this CommandEncoder.
    ident: crate::cmp::Identifier,
}

#[derive(Debug)]
pub struct WebComputePassEncoder {
    pub(crate) inner: webgpu_sys::GpuComputePassEncoder,
    /// Unique identifier for this ComputePassEncoder.
    ident: crate::cmp::Identifier,
}

#[derive(Debug)]
pub struct WebRenderPassEncoder {
    pub(crate) inner: webgpu_sys::GpuRenderPassEncoder,
    /// Unique identifier for this RenderPassEncoder.
    ident: crate::cmp::Identifier,
}

#[derive(Debug)]
pub struct WebCommandBuffer {
    pub(crate) inner: webgpu_sys::GpuCommandBuffer,
    /// Unique identifier for this CommandBuffer.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebRenderBundleEncoder {
    pub(crate) inner: webgpu_sys::GpuRenderBundleEncoder,
    /// Unique identifier for this RenderBundleEncoder.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebRenderBundle {
    pub(crate) inner: webgpu_sys::GpuRenderBundle,
    /// Unique identifier for this RenderBundle.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebSurface {
    gpu: Option<DefinedNonNullJsValue<webgpu_sys::Gpu>>,
    canvas: Canvas,
    context: webgpu_sys::GpuCanvasContext,
    /// Unique identifier for this Surface.
    ident: crate::cmp::Identifier,
}

#[derive(Debug, Clone)]
pub struct WebSurfaceOutputDetail {
    /// Unique identifier for this SurfaceOutputDetail.
    ident: crate::cmp::Identifier,
}

#[derive(Debug)]
pub struct WebQueueWriteBuffer {
    inner: Box<[u8]>,
    /// Unique identifier for this QueueWriteBuffer.
    ident: crate::cmp::Identifier,
}

#[derive(Debug)]
pub struct WebBufferMappedRange {
    actual_mapping: js_sys::Uint8Array,
    /// Copy of actual_mapping that lives in the Rust/Wasm heap instead of JS. This
    /// is done only when accessed for the first time to avoid unnecessary allocations.
    temporary_mapping: OnceCell<Vec<u8>>,
    /// Whether `temporary_mapping` has possibly been written to and needs to be written back to JS.
    temporary_mapping_modified: bool,
    /// Unique identifier for this BufferMappedRange.
    ident: crate::cmp::Identifier,
}

impl_send_sync!(ContextWebGpu);
impl_send_sync!(WebAdapter);
impl_send_sync!(WebDevice);
impl_send_sync!(WebQueue);
impl_send_sync!(WebShaderModule);
impl_send_sync!(WebBindGroupLayout);
impl_send_sync!(WebBindGroup);
impl_send_sync!(WebTextureView);
impl_send_sync!(WebSampler);
impl_send_sync!(WebBuffer);
impl_send_sync!(WebTexture);
impl_send_sync!(WebExternalTexture);
impl_send_sync!(WebBlas);
impl_send_sync!(WebTlas);
impl_send_sync!(WebQuerySet);
impl_send_sync!(WebPipelineLayout);
impl_send_sync!(WebRenderPipeline);
impl_send_sync!(WebComputePipeline);
impl_send_sync!(WebPipelineCache);
impl_send_sync!(WebCommandEncoder);
impl_send_sync!(WebComputePassEncoder);
impl_send_sync!(WebRenderPassEncoder);
impl_send_sync!(WebCommandBuffer);
impl_send_sync!(WebRenderBundleEncoder);
impl_send_sync!(WebRenderBundle);
impl_send_sync!(WebSurface);
impl_send_sync!(WebSurfaceOutputDetail);
impl_send_sync!(WebQueueWriteBuffer);
impl_send_sync!(WebBufferMappedRange);

crate::cmp::impl_eq_ord_hash_proxy!(ContextWebGpu => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebAdapter => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebDevice => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebQueue => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebShaderModule => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebBindGroupLayout => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebBindGroup => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebTextureView => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebSampler => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebBuffer => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebTexture => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebExternalTexture => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebBlas => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebTlas => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebQuerySet => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebPipelineLayout => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebRenderPipeline => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebComputePipeline => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebPipelineCache => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebCommandEncoder => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebComputePassEncoder => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebRenderPassEncoder => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebCommandBuffer => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebRenderBundleEncoder => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebRenderBundle => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebSurface => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebSurfaceOutputDetail => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebQueueWriteBuffer => .ident);
crate::cmp::impl_eq_ord_hash_proxy!(WebBufferMappedRange => .ident);

impl dispatch::InstanceInterface for ContextWebGpu {
    fn new(desc: crate::InstanceDescriptor) -> Self
    where
        Self: Sized,
    {
        let Ok(gpu) = get_browser_gpu_property() else {
            panic!(
                "Accessing the GPU is only supported on the main thread or from a dedicated worker"
            );
        };

        ContextWebGpu {
            gpu,
            requested_backends: desc.backends,
            ident: crate::cmp::Identifier::create(),
        }
    }

    unsafe fn create_surface(
        &self,
        target: crate::SurfaceTargetUnsafe,
    ) -> Result<dispatch::DispatchSurface, crate::CreateSurfaceError> {
        match target {
            SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: _,
                raw_window_handle,
            } => {
                let canvas_element: web_sys::HtmlCanvasElement = match raw_window_handle {
                    raw_window_handle::RawWindowHandle::Web(handle) => {
                        let canvas_node: wasm_bindgen::JsValue = web_sys::window()
                            .and_then(|win| win.document())
                            .and_then(|doc| {
                                doc.query_selector_all(&format!(
                                    "[data-raw-handle=\"{}\"]",
                                    handle.id
                                ))
                                .ok()
                            })
                            .and_then(|nodes| nodes.get(0))
                            .expect("expected to find single canvas")
                            .into();
                        canvas_node.into()
                    }
                    raw_window_handle::RawWindowHandle::WebCanvas(handle) => {
                        let value: &JsValue = unsafe { handle.obj.cast().as_ref() };
                        value.clone().unchecked_into()
                    }
                    raw_window_handle::RawWindowHandle::WebOffscreenCanvas(handle) => {
                        let value: &JsValue = unsafe { handle.obj.cast().as_ref() };
                        let canvas: web_sys::OffscreenCanvas = value.clone().unchecked_into();
                        let context_result = canvas.get_context("webgpu");

                        return self.create_surface_from_context(
                            Canvas::Offscreen(canvas),
                            context_result,
                        );
                    }
                    _ => panic!("expected valid handle for canvas"),
                };

                let context_result = canvas_element.get_context("webgpu");
                self.create_surface_from_context(Canvas::Canvas(canvas_element), context_result)
            }
        }
    }

    fn request_adapter(
        &self,
        options: &crate::RequestAdapterOptions<'_, '_>,
    ) -> Pin<Box<dyn dispatch::RequestAdapterFuture>> {
        let requested_backends = self.requested_backends;

        //TODO: support this check, return `None` if the flag is not set.
        // It's not trivial, since we need the Future logic to have this check,
        // and currently the Future here has no room for extra parameter `backends`.
        if !(requested_backends.contains(wgt::Backends::BROWSER_WEBGPU)) {
            return Box::pin(core::future::ready(Err(
                wgt::RequestAdapterError::NotFound {
                    active_backends: Backends::BROWSER_WEBGPU,
                    requested_backends,
                    // TODO: supported_backends should also include wgpu-core-based backends,
                    // if they were compiled in.
                    supported_backends: Backends::BROWSER_WEBGPU,
                    no_fallback_backends: Backends::default(),
                    no_adapter_backends: Backends::default(),
                    incompatible_surface_backends: Backends::default(),
                },
            )));
        }
        let mapped_options = webgpu_sys::GpuRequestAdapterOptions::new();
        let mapped_power_preference = match options.power_preference {
            wgt::PowerPreference::None => None,
            wgt::PowerPreference::LowPower => Some(webgpu_sys::GpuPowerPreference::LowPower),
            wgt::PowerPreference::HighPerformance => {
                Some(webgpu_sys::GpuPowerPreference::HighPerformance)
            }
        };
        if let Some(mapped_pref) = mapped_power_preference {
            mapped_options.set_power_preference(mapped_pref);
        }

        if let Some(gpu) = &self.gpu {
            let adapter_promise = gpu.request_adapter_with_options(&mapped_options);
            Box::pin(MakeSendFuture::new(
                wasm_bindgen_futures::JsFuture::from(adapter_promise),
                move |result| future_request_adapter(result, requested_backends),
            ))
        } else {
            // Gpu is undefined; WebGPU is not supported in this browser.
            // Treat this exactly like requestAdapter() returned null.
            Box::pin(core::future::ready(Err(request_adapter_null_error(
                requested_backends,
            ))))
        }
    }
    fn enumerate_adapters(
        &self,
        _backends: crate::Backends,
    ) -> Pin<Box<dyn dispatch::EnumerateAdapterFuture>> {
        let future = self.request_adapter(&crate::RequestAdapterOptions::default());
        let enumerate_future = async move {
            let adapter = future.await;
            match adapter {
                Ok(a) => vec![a],
                Err(_) => vec![],
            }
        };
        Box::pin(enumerate_future)
    }

    fn poll_all_devices(&self, _force_wait: bool) -> bool {
        // Devices are automatically polled.
        true
    }

    #[cfg(feature = "wgsl")]
    fn wgsl_language_features(&self) -> crate::WgslLanguageFeatures {
        let mut wgsl_language_features = crate::WgslLanguageFeatures::empty();
        if let Some(gpu) = &self.gpu {
            gpu.wgsl_language_features()
                .keys()
                .into_iter()
                .map(|wlf| wlf.expect("`WgslLanguageFeatures` elements should be valid"))
                .map(|wlf| {
                    wlf.as_string()
                        .expect("`WgslLanguageFeatures` should be string set")
                })
                .filter_map(|wlf| match wlf.as_str() {
                    "readonly_and_readwrite_storage_textures" => {
                        Some(crate::WgslLanguageFeatures::ReadOnlyAndReadWriteStorageTextures)
                    }
                    "packed_4x8_integer_dot_product" => {
                        Some(crate::WgslLanguageFeatures::Packed4x8IntegerDotProduct)
                    }
                    "unrestricted_pointer_parameters" => {
                        Some(crate::WgslLanguageFeatures::UnrestrictedPointerParameters)
                    }
                    "pointer_composite_access" => {
                        Some(crate::WgslLanguageFeatures::PointerCompositeAccess)
                    }
                    _ => None,
                })
                .for_each(|wlf| {
                    wgsl_language_features |= wlf;
                })
        }
        wgsl_language_features
    }
}

impl Drop for ContextWebGpu {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::AdapterInterface for WebAdapter {
    fn request_device(
        &self,
        desc: &crate::DeviceDescriptor<'_>,
    ) -> Pin<Box<dyn dispatch::RequestDeviceFuture>> {
        if !matches!(desc.trace, wgt::Trace::Off) {
            log::warn!("The `trace` parameter is not supported on the WebGPU backend.");
        }

        let mapped_desc = webgpu_sys::GpuDeviceDescriptor::new();

        let required_limits = map_js_sys_limits(&desc.required_limits);
        mapped_desc.set_required_limits(&required_limits);

        let required_features = FEATURES_MAPPING
            .iter()
            .copied()
            .flat_map(|(flag, value)| {
                if desc.required_features.contains(flag) {
                    Some(JsValue::from(value))
                } else {
                    None
                }
            })
            .collect::<js_sys::Array>();
        mapped_desc.set_required_features(&required_features);

        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }

        let device_promise = self.inner.request_device_with_descriptor(&mapped_desc);

        Box::pin(MakeSendFuture::new(
            wasm_bindgen_futures::JsFuture::from(device_promise),
            future_request_device,
        ))
    }

    fn is_surface_supported(&self, _surface: &dispatch::DispatchSurface) -> bool {
        // All surfaces are inherently supported.
        true
    }

    fn features(&self) -> crate::Features {
        map_wgt_features(self.inner.features())
    }

    fn limits(&self) -> crate::Limits {
        map_wgt_limits(self.inner.limits())
    }

    fn downlevel_capabilities(&self) -> crate::DownlevelCapabilities {
        // WebGPU is assumed to be fully compliant
        crate::DownlevelCapabilities::default()
    }

    fn get_info(&self) -> crate::AdapterInfo {
        map_adapter_info(&self.inner.info())
    }

    fn get_texture_format_features(
        &self,
        format: crate::TextureFormat,
    ) -> crate::TextureFormatFeatures {
        format.guaranteed_format_features(dispatch::AdapterInterface::features(self))
    }

    fn get_presentation_timestamp(&self) -> crate::PresentationTimestamp {
        crate::PresentationTimestamp::INVALID_TIMESTAMP
    }

    fn cooperative_matrix_properties(&self) -> Vec<wgt::CooperativeMatrixProperties> {
        Vec::new()
    }
}
impl Drop for WebAdapter {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::DeviceInterface for WebDevice {
    fn features(&self) -> crate::Features {
        map_wgt_features(self.inner.features())
    }

    fn limits(&self) -> crate::Limits {
        map_wgt_limits(self.inner.limits())
    }

    fn adapter_info(&self) -> crate::AdapterInfo {
        map_adapter_info(&self.inner.adapter_info())
    }

    fn create_shader_module(
        &self,
        desc: crate::ShaderModuleDescriptor<'_>,
        _shader_runtime_checks: crate::ShaderRuntimeChecks,
    ) -> dispatch::DispatchShaderModule {
        let shader_module_result = match desc.source {
            #[cfg(feature = "spirv")]
            crate::ShaderSource::SpirV(ref spv) => {
                use naga::front;

                let options = naga::front::spv::Options {
                    adjust_coordinate_space: false,
                    strict_capabilities: true,
                    block_ctx_dump_prefix: None,
                };
                let spv_parser = front::spv::Frontend::new(spv.iter().cloned(), &options);
                spv_parser
                    .parse()
                    .map_err(|inner| {
                        crate::CompilationInfo::from(naga::error::ShaderError {
                            source: String::new(),
                            label: desc.label.map(|s| s.to_string()),
                            inner: Box::new(inner),
                        })
                    })
                    .and_then(|spv_module| {
                        validate_transformed_shader_module(&spv_module, "", &desc).map(|v| {
                            (
                                v,
                                WebShaderCompilationInfo::Transformed {
                                    compilation_info: crate::CompilationInfo { messages: vec![] },
                                },
                            )
                        })
                    })
            }
            #[cfg(feature = "glsl")]
            crate::ShaderSource::Glsl {
                ref shader,
                stage,
                defines,
            } => {
                use naga::front;

                // Parse the given shader code and store its representation.
                let options = front::glsl::Options {
                    stage,
                    defines: defines
                        .iter()
                        .map(|&(key, value)| (String::from(key), String::from(value)))
                        .collect(),
                };
                let mut parser = front::glsl::Frontend::default();
                parser
                    .parse(&options, shader)
                    .map_err(|inner| {
                        crate::CompilationInfo::from(naga::error::ShaderError {
                            source: shader.to_string(),
                            label: desc.label.map(|s| s.to_string()),
                            inner: Box::new(inner),
                        })
                    })
                    .and_then(|glsl_module| {
                        validate_transformed_shader_module(&glsl_module, shader, &desc).map(|v| {
                            (
                                v,
                                WebShaderCompilationInfo::Transformed {
                                    compilation_info: crate::CompilationInfo { messages: vec![] },
                                },
                            )
                        })
                    })
            }
            #[cfg(feature = "wgsl")]
            crate::ShaderSource::Wgsl(ref code) => {
                let shader_module = webgpu_sys::GpuShaderModuleDescriptor::new(code);
                Ok((
                    shader_module,
                    WebShaderCompilationInfo::Wgsl {
                        source: code.to_string(),
                    },
                ))
            }
            #[cfg(feature = "naga-ir")]
            crate::ShaderSource::Naga(ref module) => {
                validate_transformed_shader_module(module, "", &desc).map(|v| {
                    (
                        v,
                        WebShaderCompilationInfo::Transformed {
                            compilation_info: crate::CompilationInfo { messages: vec![] },
                        },
                    )
                })
            }
            crate::ShaderSource::Dummy(_) => {
                panic!("found `ShaderSource::Dummy`")
            }
        };

        #[cfg(naga)]
        fn validate_transformed_shader_module(
            module: &naga::Module,
            source: &str,
            desc: &crate::ShaderModuleDescriptor<'_>,
        ) -> Result<webgpu_sys::GpuShaderModuleDescriptor, crate::CompilationInfo> {
            use naga::{back, valid};
            let mut validator =
                valid::Validator::new(valid::ValidationFlags::all(), valid::Capabilities::all());
            let module_info = validator.validate(module).map_err(|err| {
                crate::CompilationInfo::from(naga::error::ShaderError {
                    source: source.to_string(),
                    label: desc.label.map(|s| s.to_string()),
                    inner: Box::new(err),
                })
            })?;

            let writer_flags = naga::back::wgsl::WriterFlags::empty();
            let wgsl_text = back::wgsl::write_string(module, &module_info, writer_flags).unwrap();
            Ok(webgpu_sys::GpuShaderModuleDescriptor::new(
                wgsl_text.as_str(),
            ))
        }
        let (descriptor, compilation_info) = match shader_module_result {
            Ok(v) => v,
            Err(compilation_info) => (
                webgpu_sys::GpuShaderModuleDescriptor::new(""),
                WebShaderCompilationInfo::Transformed { compilation_info },
            ),
        };
        if let Some(label) = desc.label {
            descriptor.set_label(label);
        }
        WebShaderModule {
            module: self.inner.create_shader_module(&descriptor),
            compilation_info,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    unsafe fn create_shader_module_passthrough(
        &self,
        desc: &crate::ShaderModuleDescriptorPassthrough<'_>,
    ) -> dispatch::DispatchShaderModule {
        let shader_module_result = if let Some(ref code) = desc.wgsl {
            let shader_module = webgpu_sys::GpuShaderModuleDescriptor::new(code);
            Ok((
                shader_module,
                WebShaderCompilationInfo::Wgsl {
                    source: code.to_string(),
                },
            ))
        } else {
            Err(crate::CompilationInfo {
                messages: vec![crate::CompilationMessage {
                    message:
                        "Passthrough shader not compiled for WGSL on WebGPU backend (wgpu error)"
                            .to_string(),
                    location: None,
                    message_type: crate::CompilationMessageType::Error,
                }],
            })
        };
        let (descriptor, compilation_info) = match shader_module_result {
            Ok(v) => v,
            Err(compilation_info) => (
                webgpu_sys::GpuShaderModuleDescriptor::new(""),
                WebShaderCompilationInfo::Transformed { compilation_info },
            ),
        };
        if let Some(label) = desc.label {
            descriptor.set_label(label);
        }
        WebShaderModule {
            module: self.inner.create_shader_module(&descriptor),
            compilation_info,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn create_bind_group_layout(
        &self,
        desc: &crate::BindGroupLayoutDescriptor<'_>,
    ) -> dispatch::DispatchBindGroupLayout {
        let mapped_bindings = desc
            .entries
            .iter()
            .map(|bind| {
                let mapped_entry =
                    webgpu_sys::GpuBindGroupLayoutEntry::new(bind.binding, bind.visibility.bits());

                match bind.ty {
                    wgt::BindingType::Buffer {
                        ty,
                        has_dynamic_offset,
                        min_binding_size,
                    } => {
                        let buffer = webgpu_sys::GpuBufferBindingLayout::new();
                        buffer.set_has_dynamic_offset(has_dynamic_offset);
                        if let Some(size) = min_binding_size {
                            buffer.set_min_binding_size(size.get() as f64);
                        }
                        buffer.set_type(match ty {
                            wgt::BufferBindingType::Uniform => {
                                webgpu_sys::GpuBufferBindingType::Uniform
                            }
                            wgt::BufferBindingType::Storage { read_only: false } => {
                                webgpu_sys::GpuBufferBindingType::Storage
                            }
                            wgt::BufferBindingType::Storage { read_only: true } => {
                                webgpu_sys::GpuBufferBindingType::ReadOnlyStorage
                            }
                        });
                        mapped_entry.set_buffer(&buffer);
                    }
                    wgt::BindingType::Sampler(ty) => {
                        let sampler = webgpu_sys::GpuSamplerBindingLayout::new();
                        sampler.set_type(match ty {
                            wgt::SamplerBindingType::NonFiltering => {
                                webgpu_sys::GpuSamplerBindingType::NonFiltering
                            }
                            wgt::SamplerBindingType::Filtering => {
                                webgpu_sys::GpuSamplerBindingType::Filtering
                            }
                            wgt::SamplerBindingType::Comparison => {
                                webgpu_sys::GpuSamplerBindingType::Comparison
                            }
                        });
                        mapped_entry.set_sampler(&sampler);
                    }
                    wgt::BindingType::Texture {
                        multisampled,
                        sample_type,
                        view_dimension,
                    } => {
                        let texture = webgpu_sys::GpuTextureBindingLayout::new();
                        texture.set_multisampled(multisampled);
                        texture.set_sample_type(map_texture_component_type(sample_type));
                        texture.set_view_dimension(map_texture_view_dimension(view_dimension));
                        mapped_entry.set_texture(&texture);
                    }
                    wgt::BindingType::StorageTexture {
                        access,
                        format,
                        view_dimension,
                    } => {
                        let mapped_access = match access {
                            wgt::StorageTextureAccess::WriteOnly => {
                                webgpu_sys::GpuStorageTextureAccess::WriteOnly
                            }
                            wgt::StorageTextureAccess::ReadOnly => {
                                webgpu_sys::GpuStorageTextureAccess::ReadOnly
                            }
                            wgt::StorageTextureAccess::ReadWrite => {
                                webgpu_sys::GpuStorageTextureAccess::ReadWrite
                            }
                            wgt::StorageTextureAccess::Atomic => {
                                // Validated out by `BindGroupLayoutEntryError::StorageTextureAtomic`
                                unreachable!()
                            }
                        };
                        let storage_texture = webgpu_sys::GpuStorageTextureBindingLayout::new(
                            map_texture_format(format),
                        );
                        storage_texture.set_access(mapped_access);
                        storage_texture
                            .set_view_dimension(map_texture_view_dimension(view_dimension));
                        mapped_entry.set_storage_texture(&storage_texture);
                    }
                    wgt::BindingType::AccelerationStructure { .. } => todo!(),
                    wgt::BindingType::ExternalTexture => {
                        mapped_entry.set_external_texture(
                            &webgpu_sys::GpuExternalTextureBindingLayout::new(),
                        );
                    }
                }

                mapped_entry
            })
            .collect::<js_sys::Array>();

        let mapped_desc = webgpu_sys::GpuBindGroupLayoutDescriptor::new(&mapped_bindings);
        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }
        let bind_group_layout = self.inner.create_bind_group_layout(&mapped_desc).unwrap();

        WebBindGroupLayout {
            inner: bind_group_layout,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn create_bind_group(
        &self,
        desc: &crate::BindGroupDescriptor<'_>,
    ) -> dispatch::DispatchBindGroup {
        let mapped_entries = desc
            .entries
            .iter()
            .map(|binding| {
                let mapped_resource = match binding.resource {
                    crate::BindingResource::Buffer(crate::BufferBinding {
                        buffer,
                        offset,
                        size,
                    }) => {
                        let buffer = buffer.inner.as_webgpu();
                        let mapped_buffer_binding =
                            webgpu_sys::GpuBufferBinding::new(&buffer.inner);
                        mapped_buffer_binding.set_offset(offset as f64);
                        if let Some(s) = size {
                            mapped_buffer_binding.set_size(s.get() as f64);
                        }
                        JsValue::from(mapped_buffer_binding)
                    }
                    crate::BindingResource::BufferArray(..) => {
                        panic!("Web backend does not support arrays of buffers")
                    }
                    crate::BindingResource::Sampler(sampler) => {
                        let sampler = &sampler.inner.as_webgpu().inner;
                        JsValue::from(sampler)
                    }
                    crate::BindingResource::SamplerArray(..) => {
                        panic!("Web backend does not support arrays of samplers")
                    }
                    crate::BindingResource::TextureView(texture_view) => {
                        let texture_view = &texture_view.inner.as_webgpu().inner;
                        JsValue::from(texture_view)
                    }
                    crate::BindingResource::TextureViewArray(..) => {
                        panic!("Web backend does not support BINDING_INDEXING extension")
                    }
                    crate::BindingResource::AccelerationStructure(_) => {
                        unimplemented!("Raytracing not implemented for web")
                    }
                    crate::BindingResource::AccelerationStructureArray(_) => {
                        unimplemented!("Raytracing not implemented for web")
                    }
                    crate::BindingResource::ExternalTexture(_) => {
                        unimplemented!("ExternalTexture not implemented for web")
                    }
                };

                webgpu_sys::GpuBindGroupEntry::new(binding.binding, &mapped_resource)
            })
            .collect::<js_sys::Array>();

        let bgl = &desc.layout.inner.as_webgpu().inner;
        let mapped_desc = webgpu_sys::GpuBindGroupDescriptor::new(&mapped_entries, bgl);
        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }
        let bind_group = self.inner.create_bind_group(&mapped_desc);

        WebBindGroup {
            inner: bind_group,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn create_pipeline_layout(
        &self,
        desc: &crate::PipelineLayoutDescriptor<'_>,
    ) -> dispatch::DispatchPipelineLayout {
        let null = wasm_bindgen::JsValue::NULL;
        let temp_layouts = desc
            .bind_group_layouts
            .iter()
            .map(|bgl| match bgl {
                Some(bgl) => bgl.inner.as_webgpu().inner.as_ref(),
                None => &null,
            })
            .collect::<js_sys::Array>();
        let mapped_desc = webgpu_sys::GpuPipelineLayoutDescriptor::new(&temp_layouts);
        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }

        let pipeline_layout = self.inner.create_pipeline_layout(&mapped_desc);

        WebPipelineLayout {
            inner: pipeline_layout,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn create_render_pipeline(
        &self,
        desc: &crate::RenderPipelineDescriptor<'_>,
    ) -> dispatch::DispatchRenderPipeline {
        let module = desc.vertex.module.inner.as_webgpu();
        let mapped_vertex_state = webgpu_sys::GpuVertexState::new(&module.module);
        insert_constants_map(
            &mapped_vertex_state,
            desc.vertex.compilation_options.constants,
        );
        if let Some(ep) = desc.vertex.entry_point {
            mapped_vertex_state.set_entry_point(ep);
        }

        let buffers = desc
            .vertex
            .buffers
            .iter()
            .map(|vbuf| {
                let mapped_attributes = vbuf
                    .attributes
                    .iter()
                    .map(|attr| {
                        webgpu_sys::GpuVertexAttribute::new(
                            map_vertex_format(attr.format),
                            attr.offset as f64,
                            attr.shader_location,
                        )
                    })
                    .collect::<js_sys::Array>();

                let mapped_vbuf = webgpu_sys::GpuVertexBufferLayout::new(
                    vbuf.array_stride as f64,
                    &mapped_attributes,
                );
                mapped_vbuf.set_step_mode(map_vertex_step_mode(vbuf.step_mode));
                mapped_vbuf
            })
            .collect::<js_sys::Array>();

        mapped_vertex_state.set_buffers(&buffers);

        let auto_layout = wasm_bindgen::JsValue::from(webgpu_sys::GpuAutoLayoutMode::Auto);
        let mapped_desc = webgpu_sys::GpuRenderPipelineDescriptor::new(
            &match desc.layout {
                Some(layout) => {
                    let layout = &layout.inner.as_webgpu().inner;
                    JsValue::from(layout)
                }
                None => auto_layout,
            },
            &mapped_vertex_state,
        );

        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }

        if let Some(ref depth_stencil) = desc.depth_stencil {
            mapped_desc.set_depth_stencil(&map_depth_stencil_state(depth_stencil));
        }

        if let Some(ref frag) = desc.fragment {
            let targets = frag
                .targets
                .iter()
                .map(|target| match target {
                    Some(target) => {
                        let mapped_format = map_texture_format(target.format);
                        let mapped_color_state =
                            webgpu_sys::GpuColorTargetState::new(mapped_format);
                        if let Some(ref bs) = target.blend {
                            let alpha = map_blend_component(&bs.alpha);
                            let color = map_blend_component(&bs.color);
                            let mapped_blend_state = webgpu_sys::GpuBlendState::new(&alpha, &color);
                            mapped_color_state.set_blend(&mapped_blend_state);
                        }
                        mapped_color_state.set_write_mask(target.write_mask.bits());
                        wasm_bindgen::JsValue::from(mapped_color_state)
                    }
                    None => wasm_bindgen::JsValue::null(),
                })
                .collect::<js_sys::Array>();
            let module = frag.module.inner.as_webgpu();
            let mapped_fragment_desc = webgpu_sys::GpuFragmentState::new(&module.module, &targets);
            insert_constants_map(&mapped_fragment_desc, frag.compilation_options.constants);
            if let Some(ep) = frag.entry_point {
                mapped_fragment_desc.set_entry_point(ep);
            }
            mapped_desc.set_fragment(&mapped_fragment_desc);
        }

        let mapped_multisample = webgpu_sys::GpuMultisampleState::new();
        mapped_multisample.set_count(desc.multisample.count);
        mapped_multisample.set_mask(desc.multisample.mask as u32);
        mapped_multisample
            .set_alpha_to_coverage_enabled(desc.multisample.alpha_to_coverage_enabled);
        mapped_desc.set_multisample(&mapped_multisample);

        let mapped_primitive = map_primitive_state(&desc.primitive);
        mapped_desc.set_primitive(&mapped_primitive);

        let render_pipeline = self.inner.create_render_pipeline(&mapped_desc).unwrap();

        WebRenderPipeline {
            inner: render_pipeline,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn create_mesh_pipeline(
        &self,
        _desc: &crate::MeshPipelineDescriptor<'_>,
    ) -> dispatch::DispatchRenderPipeline {
        panic!("MESH_SHADER feature must be enabled to call create_mesh_pipeline")
    }

    fn create_compute_pipeline(
        &self,
        desc: &crate::ComputePipelineDescriptor<'_>,
    ) -> dispatch::DispatchComputePipeline {
        let shader_module = desc.module.inner.as_webgpu();
        let mapped_compute_stage = webgpu_sys::GpuProgrammableStage::new(&shader_module.module);
        insert_constants_map(&mapped_compute_stage, desc.compilation_options.constants);
        if let Some(ep) = desc.entry_point {
            mapped_compute_stage.set_entry_point(ep);
        }
        let auto_layout = wasm_bindgen::JsValue::from(webgpu_sys::GpuAutoLayoutMode::Auto);
        let mapped_desc = webgpu_sys::GpuComputePipelineDescriptor::new(
            &match desc.layout {
                Some(layout) => {
                    let layout = &layout.inner.as_webgpu().inner;
                    JsValue::from(layout)
                }
                None => auto_layout,
            },
            &mapped_compute_stage,
        );
        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }

        let compute_pipeline = self.inner.create_compute_pipeline(&mapped_desc);

        WebComputePipeline {
            inner: compute_pipeline,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    unsafe fn create_pipeline_cache(
        &self,
        _desc: &crate::PipelineCacheDescriptor<'_>,
    ) -> dispatch::DispatchPipelineCache {
        WebPipelineCache {
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn create_buffer(&self, desc: &crate::BufferDescriptor<'_>) -> dispatch::DispatchBuffer {
        let mapped_desc = webgpu_sys::GpuBufferDescriptor::new(desc.size as f64, desc.usage.bits());
        mapped_desc.set_mapped_at_creation(desc.mapped_at_creation);
        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }
        WebBuffer::new(self.inner.create_buffer(&mapped_desc).unwrap(), desc).into()
    }

    fn create_texture(&self, desc: &crate::TextureDescriptor<'_>) -> dispatch::DispatchTexture {
        let mapped_desc = webgpu_sys::GpuTextureDescriptor::new(
            map_texture_format(desc.format),
            &map_extent_3d(desc.size),
            (desc.usage - crate::TextureUsages::TRANSIENT).bits(),
        );
        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }
        mapped_desc.set_dimension(map_texture_dimension(desc.dimension));
        mapped_desc.set_mip_level_count(desc.mip_level_count);
        mapped_desc.set_sample_count(desc.sample_count);
        let mapped_view_formats = desc
            .view_formats
            .iter()
            .map(|format| JsValue::from(map_texture_format(*format)))
            .collect::<js_sys::Array>();
        mapped_desc.set_view_formats(&mapped_view_formats);

        let texture = self.inner.create_texture(&mapped_desc).unwrap();
        WebTexture {
            inner: texture,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn create_external_texture(
        &self,
        _desc: &crate::ExternalTextureDescriptor<'_>,
        _planes: &[&crate::TextureView],
    ) -> dispatch::DispatchExternalTexture {
        unimplemented!("ExternalTexture not implemented for web");
    }

    fn create_blas(
        &self,
        _desc: &crate::CreateBlasDescriptor<'_>,
        _sizes: crate::BlasGeometrySizeDescriptors,
    ) -> (Option<u64>, dispatch::DispatchBlas) {
        unimplemented!("Raytracing not implemented for web");
    }

    fn create_tlas(&self, _desc: &crate::CreateTlasDescriptor<'_>) -> dispatch::DispatchTlas {
        unimplemented!("Raytracing not implemented for web");
    }

    fn create_sampler(&self, desc: &crate::SamplerDescriptor<'_>) -> dispatch::DispatchSampler {
        let mapped_desc = webgpu_sys::GpuSamplerDescriptor::new();
        mapped_desc.set_address_mode_u(map_address_mode(desc.address_mode_u));
        mapped_desc.set_address_mode_v(map_address_mode(desc.address_mode_v));
        mapped_desc.set_address_mode_w(map_address_mode(desc.address_mode_w));
        if let Some(compare) = desc.compare {
            mapped_desc.set_compare(map_compare_function(compare));
        }
        mapped_desc.set_lod_max_clamp(desc.lod_max_clamp);
        mapped_desc.set_lod_min_clamp(desc.lod_min_clamp);
        mapped_desc.set_mag_filter(map_filter_mode(desc.mag_filter));
        mapped_desc.set_min_filter(map_filter_mode(desc.min_filter));
        mapped_desc.set_mipmap_filter(map_mipmap_filter_mode(desc.mipmap_filter));
        mapped_desc.set_max_anisotropy(desc.anisotropy_clamp);
        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }

        let sampler = self.inner.create_sampler_with_descriptor(&mapped_desc);

        WebSampler {
            inner: sampler,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn create_query_set(&self, desc: &crate::QuerySetDescriptor<'_>) -> dispatch::DispatchQuerySet {
        let ty = match desc.ty {
            wgt::QueryType::Occlusion => webgpu_sys::GpuQueryType::Occlusion,
            wgt::QueryType::Timestamp => webgpu_sys::GpuQueryType::Timestamp,
            wgt::QueryType::PipelineStatistics(_) => unreachable!(),
        };
        let mapped_desc = webgpu_sys::GpuQuerySetDescriptor::new(desc.count, ty);
        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }

        let query_set = self.inner.create_query_set(&mapped_desc).unwrap();

        WebQuerySet {
            inner: query_set,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn create_command_encoder(
        &self,
        desc: &crate::CommandEncoderDescriptor<'_>,
    ) -> dispatch::DispatchCommandEncoder {
        let mapped_desc = webgpu_sys::GpuCommandEncoderDescriptor::new();
        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }

        let command_encoder = self
            .inner
            .create_command_encoder_with_descriptor(&mapped_desc);

        WebCommandEncoder {
            inner: command_encoder,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn create_render_bundle_encoder(
        &self,
        desc: &crate::RenderBundleEncoderDescriptor<'_>,
    ) -> dispatch::DispatchRenderBundleEncoder {
        let mapped_color_formats = desc
            .color_formats
            .iter()
            .map(|cf| match cf {
                Some(cf) => wasm_bindgen::JsValue::from(map_texture_format(*cf)),
                None => wasm_bindgen::JsValue::null(),
            })
            .collect::<js_sys::Array>();
        let mapped_desc = webgpu_sys::GpuRenderBundleEncoderDescriptor::new(&mapped_color_formats);
        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }
        if let Some(ds) = desc.depth_stencil {
            mapped_desc.set_depth_stencil_format(map_texture_format(ds.format));
            mapped_desc.set_depth_read_only(ds.depth_read_only);
            mapped_desc.set_stencil_read_only(ds.stencil_read_only);
        }
        mapped_desc.set_sample_count(desc.sample_count);

        let render_bundle_encoder = self
            .inner
            .create_render_bundle_encoder(&mapped_desc)
            .unwrap();

        WebRenderBundleEncoder {
            inner: render_bundle_encoder,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn set_device_lost_callback(&self, device_lost_callback: dispatch::BoxDeviceLostCallback) {
        let closure = Closure::once(move |info: JsValue| {
            let info = info.dyn_into::<webgpu_sys::GpuDeviceLostInfo>().unwrap();
            device_lost_callback(
                match info.reason() {
                    webgpu_sys::GpuDeviceLostReason::Destroyed => {
                        crate::DeviceLostReason::Destroyed
                    }
                    webgpu_sys::GpuDeviceLostReason::Unknown => crate::DeviceLostReason::Unknown,
                    _ => crate::DeviceLostReason::Unknown,
                },
                info.message(),
            );
        });
        let _ = self.inner.lost().then(&closure);
        // Release memory management of this closure from Rust to the JS GC.
        // TODO: This will leak if weak references is not supported.
        closure.forget();
    }

    fn on_uncaptured_error(&self, handler: Arc<dyn crate::UncapturedErrorHandler>) {
        let f = Closure::wrap(Box::new(move |event: webgpu_sys::GpuUncapturedErrorEvent| {
            let error = crate::Error::from_js(event.error().value_of());
            handler(error);
        }) as Box<dyn FnMut(_)>);
        self.inner
            .set_onuncapturederror(Some(f.as_ref().unchecked_ref()));
        // Release memory management of this closure from Rust to the JS GC.
        // TODO: This will leak if weak references is not supported.
        f.forget();
    }

    fn push_error_scope(&self, filter: crate::ErrorFilter) -> u32 {
        let index = self.error_scope_count.get();
        self.error_scope_count.set(
            index
                .checked_add(1)
                .expect("Greater than 2^32 nested error scopes"),
        );
        self.inner.push_error_scope(match filter {
            crate::ErrorFilter::OutOfMemory => webgpu_sys::GpuErrorFilter::OutOfMemory,
            crate::ErrorFilter::Validation => webgpu_sys::GpuErrorFilter::Validation,
            crate::ErrorFilter::Internal => webgpu_sys::GpuErrorFilter::Internal,
        });
        index
    }

    fn pop_error_scope(&self, index: u32) -> Pin<Box<dyn dispatch::PopErrorScopeFuture>> {
        let current_scope_count = self.error_scope_count.get();
        let is_panicking = crate::util::is_panicking();
        if current_scope_count == 0 && !is_panicking {
            panic!("Mismatched pop_error_scope call: no error scope for this thread. Error scopes are thread-local.");
        }
        if index + 1 != current_scope_count && !is_panicking {
            panic!(
                "Mismatched pop_error_scope call: error scopes must be popped in reverse order."
            );
        }
        // Decrement the error scope count. We've asserted that the current
        // size is `index + 1` above.
        self.error_scope_count.set(index);

        let error_promise = self.inner.pop_error_scope();
        Box::pin(MakeSendFuture::new(
            wasm_bindgen_futures::JsFuture::from(error_promise),
            future_pop_error_scope,
        ))
    }

    unsafe fn start_graphics_debugger_capture(&self) {
        // No capturing api in webgpu
    }

    unsafe fn stop_graphics_debugger_capture(&self) {
        // No capturing api in webgpu
    }

    fn poll(&self, _poll_type: wgt::PollType<u64>) -> Result<crate::PollStatus, crate::PollError> {
        // Device is polled automatically
        Ok(crate::PollStatus::QueueEmpty)
    }

    fn get_internal_counters(&self) -> crate::InternalCounters {
        crate::InternalCounters::default()
    }

    fn generate_allocator_report(&self) -> Option<wgt::AllocatorReport> {
        None
    }

    fn destroy(&self) {
        self.inner.destroy();
    }
}

impl Drop for WebDevice {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::QueueInterface for WebQueue {
    fn write_buffer(
        &self,
        buffer: &dispatch::DispatchBuffer,
        offset: crate::BufferAddress,
        data: &[u8],
    ) {
        let buffer = buffer.as_webgpu();
        self.inner
            .write_buffer_with_f64_and_u8_slice_and_f64_and_f64(
                &buffer.inner,
                offset as f64,
                data,
                0f64,
                data.len() as f64,
            )
            .unwrap();
    }

    fn create_staging_buffer(
        &self,
        size: crate::BufferSize,
    ) -> Option<dispatch::DispatchQueueWriteBuffer> {
        Some(
            WebQueueWriteBuffer {
                inner: vec![0; size.get() as usize].into_boxed_slice(),
                ident: crate::cmp::Identifier::create(),
            }
            .into(),
        )
    }

    fn validate_write_buffer(
        &self,
        buffer: &dispatch::DispatchBuffer,
        offset: wgt::BufferAddress,
        size: wgt::BufferSize,
    ) -> Option<()> {
        let buffer = buffer.as_webgpu();

        let usage = wgt::BufferUsages::from_bits_truncate(buffer.inner.usage());
        // TODO: actually send this down the error scope
        if !usage.contains(wgt::BufferUsages::COPY_DST) {
            log::error!("Destination buffer is missing the `COPY_DST` usage flag");
            return None;
        }
        let write_size = u64::from(size);
        if !write_size.is_multiple_of(wgt::COPY_BUFFER_ALIGNMENT) {
            log::error!("Copy size {size} does not respect `COPY_BUFFER_ALIGNMENT`");
            return None;
        }
        if !offset.is_multiple_of(wgt::COPY_BUFFER_ALIGNMENT) {
            log::error!(
                "Buffer offset {offset} is not aligned to block size or `COPY_BUFFER_ALIGNMENT`"
            );
            return None;
        }
        if write_size + offset > buffer.inner.size() as u64 {
            log::error!("copy of {}..{} would end up overrunning the bounds of the destination buffer of size {}", offset, offset + write_size, buffer.inner.size());
            return None;
        }
        Some(())
    }

    fn write_staging_buffer(
        &self,
        buffer: &dispatch::DispatchBuffer,
        offset: crate::BufferAddress,
        staging_buffer: &dispatch::DispatchQueueWriteBuffer,
    ) {
        let staging_buffer = staging_buffer.as_webgpu();

        dispatch::QueueInterface::write_buffer(self, buffer, offset, &staging_buffer.inner)
    }

    fn write_texture(
        &self,
        texture: crate::TexelCopyTextureInfo<'_>,
        data: &[u8],
        data_layout: crate::TexelCopyBufferLayout,
        size: crate::Extent3d,
    ) {
        let mapped_data_layout = webgpu_sys::GpuTexelCopyBufferLayout::new();
        if let Some(bytes_per_row) = data_layout.bytes_per_row {
            mapped_data_layout.set_bytes_per_row(bytes_per_row);
        }
        if let Some(rows_per_image) = data_layout.rows_per_image {
            mapped_data_layout.set_rows_per_image(rows_per_image);
        }
        mapped_data_layout.set_offset(data_layout.offset as f64);

        self.inner
            .write_texture_with_u8_slice_and_gpu_extent_3d_dict(
                &map_texture_copy_view(texture),
                data,
                &mapped_data_layout,
                &map_extent_3d(size),
            )
            .unwrap();
    }

    fn copy_external_image_to_texture(
        &self,
        source: &crate::CopyExternalImageSourceInfo,
        dest: crate::CopyExternalImageDestInfo<&crate::api::Texture>,
        size: crate::Extent3d,
    ) {
        self.inner
            .copy_external_image_to_texture_with_gpu_extent_3d_dict(
                &map_external_texture_copy_view(source),
                &map_tagged_texture_copy_view(dest),
                &map_extent_3d(size),
            )
            .unwrap();
    }

    fn submit(
        &self,
        command_buffers: &mut dyn Iterator<Item = dispatch::DispatchCommandBuffer>,
    ) -> u64 {
        let temp_command_buffers = command_buffers.collect::<Vec<_>>();

        let array = temp_command_buffers
            .iter()
            .map(|buffer| &buffer.as_webgpu().inner)
            .collect::<js_sys::Array>();

        self.inner.submit(&array);

        0
    }

    fn get_timestamp_period(&self) -> f32 {
        // Timestamp values are always in nanoseconds, see https://gpuweb.github.io/gpuweb/#timestamp
        1.0
    }

    fn on_submitted_work_done(&self, callback: dispatch::BoxSubmittedWorkDoneCallback) {
        let promise = self.inner.on_submitted_work_done();
        wasm_bindgen_futures::spawn_local(async move {
            match wasm_bindgen_futures::JsFuture::from(promise).await {
                Ok(_) => callback(),
                Err(error) => {
                    log::error!("on_submitted_work_done promise failed: {error:?}");
                    callback();
                }
            }
        });
    }

    fn compact_blas(
        &self,
        _blas: &dispatch::DispatchBlas,
    ) -> (Option<u64>, dispatch::DispatchBlas) {
        unimplemented!("Raytracing not implemented for web")
    }
}
impl Drop for WebQueue {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::ShaderModuleInterface for WebShaderModule {
    fn get_compilation_info(&self) -> Pin<Box<dyn dispatch::ShaderCompilationInfoFuture>> {
        let compilation_info_promise = self.module.get_compilation_info();
        let map_future = Box::new({
            let compilation_info = self.compilation_info.clone();
            move |result| future_compilation_info(result, &compilation_info)
        });
        Box::pin(MakeSendFuture::new(
            wasm_bindgen_futures::JsFuture::from(compilation_info_promise),
            map_future,
        ))
    }
}
impl Drop for WebShaderModule {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::BindGroupLayoutInterface for WebBindGroupLayout {}
impl Drop for WebBindGroupLayout {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::BindGroupInterface for WebBindGroup {}
impl Drop for WebBindGroup {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::TextureViewInterface for WebTextureView {}
impl Drop for WebTextureView {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::SamplerInterface for WebSampler {}
impl Drop for WebSampler {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::BufferInterface for WebBuffer {
    fn map_async(
        &self,
        mode: crate::MapMode,
        range: Range<crate::BufferAddress>,
        callback: dispatch::BufferMapCallback,
    ) {
        let map_promise = self.inner.map_async_with_f64_and_f64(
            map_map_mode(mode),
            range.start as f64,
            (range.end - range.start) as f64,
        );

        self.set_mapped_range(range);

        register_then_closures(&map_promise, callback, Ok(()), Err(crate::BufferAsyncError));
    }

    fn get_mapped_range(
        &self,
        sub_range: Range<crate::BufferAddress>,
    ) -> dispatch::DispatchBufferMappedRange {
        let actual_mapping = self.get_mapped_range(sub_range);
        WebBufferMappedRange {
            actual_mapping,
            temporary_mapping: OnceCell::new(),
            temporary_mapping_modified: false,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn unmap(&self) {
        self.inner.unmap();
        self.mapping.borrow_mut().mapped_buffer = None;
    }

    fn destroy(&self) {
        self.inner.destroy();
    }
}
impl Drop for WebBuffer {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::TextureInterface for WebTexture {
    fn create_view(
        &self,
        desc: &crate::TextureViewDescriptor<'_>,
    ) -> dispatch::DispatchTextureView {
        let mapped = webgpu_sys::GpuTextureViewDescriptor::new();
        if let Some(dim) = desc.dimension {
            mapped.set_dimension(map_texture_view_dimension(dim));
        }
        if let Some(format) = desc.format {
            mapped.set_format(map_texture_format(format));
        }
        mapped.set_aspect(map_texture_aspect(desc.aspect));
        mapped.set_base_array_layer(desc.base_array_layer);
        if let Some(count) = desc.array_layer_count {
            mapped.set_array_layer_count(count);
        }
        mapped.set_base_mip_level(desc.base_mip_level);
        if let Some(count) = desc.mip_level_count {
            mapped.set_mip_level_count(count);
        }
        if let Some(label) = desc.label {
            mapped.set_label(label);
        }
        mapped.set_usage(desc.usage.unwrap_or(wgt::TextureUsages::empty()).bits());

        let view = self.inner.create_view_with_descriptor(&mapped).unwrap();

        WebTextureView {
            inner: view,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn destroy(&self) {
        self.inner.destroy();
    }
}
impl Drop for WebTexture {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::ExternalTextureInterface for WebExternalTexture {
    fn destroy(&self) {
        unimplemented!("ExternalTexture not implemented for web");
    }
}
impl Drop for WebExternalTexture {
    fn drop(&mut self) {
        unimplemented!("ExternalTexture not implemented for web");
    }
}

impl dispatch::BlasInterface for WebBlas {
    fn prepare_compact_async(&self, _callback: BlasCompactCallback) {
        unimplemented!("Raytracing not implemented for web")
    }
    fn ready_for_compaction(&self) -> bool {
        unimplemented!("Raytracing not implemented for web")
    }
}
impl Drop for WebBlas {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::TlasInterface for WebTlas {}
impl Drop for WebTlas {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::QuerySetInterface for WebQuerySet {}
impl Drop for WebQuerySet {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::PipelineLayoutInterface for WebPipelineLayout {}
impl Drop for WebPipelineLayout {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::RenderPipelineInterface for WebRenderPipeline {
    fn get_bind_group_layout(&self, index: u32) -> dispatch::DispatchBindGroupLayout {
        let bind_group_layout = self.inner.get_bind_group_layout(index);

        WebBindGroupLayout {
            inner: bind_group_layout,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }
}
impl Drop for WebRenderPipeline {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::ComputePipelineInterface for WebComputePipeline {
    fn get_bind_group_layout(&self, index: u32) -> dispatch::DispatchBindGroupLayout {
        let bind_group_layout = self.inner.get_bind_group_layout(index);

        WebBindGroupLayout {
            inner: bind_group_layout,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }
}
impl Drop for WebComputePipeline {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::CommandEncoderInterface for WebCommandEncoder {
    fn copy_buffer_to_buffer(
        &self,
        source: &dispatch::DispatchBuffer,
        source_offset: crate::BufferAddress,
        destination: &dispatch::DispatchBuffer,
        destination_offset: crate::BufferAddress,
        copy_size: Option<crate::BufferAddress>,
    ) {
        let source = source.as_webgpu();
        let destination = destination.as_webgpu();

        if let Some(size) = copy_size {
            self.inner
                .copy_buffer_to_buffer_with_f64_and_f64_and_f64(
                    &source.inner,
                    source_offset as f64,
                    &destination.inner,
                    destination_offset as f64,
                    size as f64,
                )
                .unwrap();
        } else {
            self.inner
                .copy_buffer_to_buffer_with_f64_and_f64(
                    &source.inner,
                    source_offset as f64,
                    &destination.inner,
                    destination_offset as f64,
                )
                .unwrap();
        }
    }

    fn copy_buffer_to_texture(
        &self,
        source: crate::TexelCopyBufferInfo<'_>,
        destination: crate::TexelCopyTextureInfo<'_>,
        copy_size: crate::Extent3d,
    ) {
        self.inner
            .copy_buffer_to_texture_with_gpu_extent_3d_dict(
                &map_buffer_copy_view(source),
                &map_texture_copy_view(destination),
                &map_extent_3d(copy_size),
            )
            .unwrap();
    }

    fn copy_texture_to_buffer(
        &self,
        source: crate::TexelCopyTextureInfo<'_>,
        destination: crate::TexelCopyBufferInfo<'_>,
        copy_size: crate::Extent3d,
    ) {
        self.inner
            .copy_texture_to_buffer_with_gpu_extent_3d_dict(
                &map_texture_copy_view(source),
                &map_buffer_copy_view(destination),
                &map_extent_3d(copy_size),
            )
            .unwrap();
    }

    fn copy_texture_to_texture(
        &self,
        source: crate::TexelCopyTextureInfo<'_>,
        destination: crate::TexelCopyTextureInfo<'_>,
        copy_size: crate::Extent3d,
    ) {
        self.inner
            .copy_texture_to_texture_with_gpu_extent_3d_dict(
                &map_texture_copy_view(source),
                &map_texture_copy_view(destination),
                &map_extent_3d(copy_size),
            )
            .unwrap();
    }

    fn begin_compute_pass(
        &self,
        desc: &crate::ComputePassDescriptor<'_>,
    ) -> dispatch::DispatchComputePass {
        let mapped_desc = webgpu_sys::GpuComputePassDescriptor::new();
        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }

        if let Some(ref timestamp_writes) = desc.timestamp_writes {
            let query_set = timestamp_writes.query_set.inner.as_webgpu();
            let writes = webgpu_sys::GpuComputePassTimestampWrites::new(&query_set.inner);
            if let Some(index) = timestamp_writes.beginning_of_pass_write_index {
                writes.set_beginning_of_pass_write_index(index);
            }
            if let Some(index) = timestamp_writes.end_of_pass_write_index {
                writes.set_end_of_pass_write_index(index);
            }
            mapped_desc.set_timestamp_writes(&writes);
        }

        let compute_pass = self.inner.begin_compute_pass_with_descriptor(&mapped_desc);

        WebComputePassEncoder {
            inner: compute_pass,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn begin_render_pass(
        &self,
        desc: &crate::RenderPassDescriptor<'_>,
    ) -> dispatch::DispatchRenderPass {
        let mapped_color_attachments = desc
            .color_attachments
            .iter()
            .map(|attachment| match attachment {
                Some(ca) => {
                    let mut clear_value: Option<wasm_bindgen::JsValue> = None;
                    let load_value = match ca.ops.load {
                        crate::LoadOp::Clear(color) => {
                            clear_value = Some(wasm_bindgen::JsValue::from(map_color(color)));
                            webgpu_sys::GpuLoadOp::Clear
                        }
                        crate::LoadOp::DontCare(_token) => {
                            // WebGPU can't safely have a ClearOp::DontCare, so we clear to black
                            // which is ideal for most GPUs.
                            clear_value =
                                Some(wasm_bindgen::JsValue::from(map_color(crate::Color::BLACK)));
                            webgpu_sys::GpuLoadOp::Clear
                        }
                        crate::LoadOp::Load => webgpu_sys::GpuLoadOp::Load,
                    };

                    let view = &ca.view.inner.as_webgpu().inner;

                    let mapped_color_attachment = webgpu_sys::GpuRenderPassColorAttachment::new(
                        load_value,
                        map_store_op(ca.ops.store),
                        view,
                    );
                    if let Some(cv) = clear_value {
                        mapped_color_attachment.set_clear_value(&cv);
                    }
                    if let Some(rt) = ca.resolve_target {
                        let resolve_target_view = &rt.inner.as_webgpu().inner;
                        mapped_color_attachment.set_resolve_target(resolve_target_view);
                    }
                    mapped_color_attachment.set_store_op(map_store_op(ca.ops.store));

                    wasm_bindgen::JsValue::from(mapped_color_attachment)
                }
                None => wasm_bindgen::JsValue::null(),
            })
            .collect::<js_sys::Array>();

        let mapped_desc = webgpu_sys::GpuRenderPassDescriptor::new(&mapped_color_attachments);

        if let Some(label) = desc.label {
            mapped_desc.set_label(label);
        }

        if let Some(dsa) = &desc.depth_stencil_attachment {
            let depth_stencil_attachment = &dsa.view.inner.as_webgpu().inner;
            let mapped_depth_stencil_attachment =
                webgpu_sys::GpuRenderPassDepthStencilAttachment::new(depth_stencil_attachment);
            if let Some(ref ops) = dsa.depth_ops {
                let load_op = match ops.load {
                    crate::LoadOp::Clear(v) => {
                        mapped_depth_stencil_attachment.set_depth_clear_value(v);
                        webgpu_sys::GpuLoadOp::Clear
                    }
                    crate::LoadOp::DontCare(_token) => {
                        // WebGPU can't safely have a ClearOp::DontCare, so we clear to 1.0
                        mapped_depth_stencil_attachment.set_depth_clear_value(1.0);
                        webgpu_sys::GpuLoadOp::Clear
                    }
                    crate::LoadOp::Load => webgpu_sys::GpuLoadOp::Load,
                };
                mapped_depth_stencil_attachment.set_depth_load_op(load_op);
                mapped_depth_stencil_attachment.set_depth_store_op(map_store_op(ops.store));
            }
            mapped_depth_stencil_attachment.set_depth_read_only(dsa.depth_ops.is_none());
            if let Some(ref ops) = dsa.stencil_ops {
                let load_op = match ops.load {
                    crate::LoadOp::Clear(v) => {
                        mapped_depth_stencil_attachment.set_stencil_clear_value(v);
                        webgpu_sys::GpuLoadOp::Clear
                    }
                    crate::LoadOp::DontCare(_token) => {
                        // WebGPU can't safely have a ClearOp::DontCare, so we clear to 0
                        mapped_depth_stencil_attachment.set_stencil_clear_value(0);
                        webgpu_sys::GpuLoadOp::Clear
                    }
                    crate::LoadOp::Load => webgpu_sys::GpuLoadOp::Load,
                };
                mapped_depth_stencil_attachment.set_stencil_load_op(load_op);
                mapped_depth_stencil_attachment.set_stencil_store_op(map_store_op(ops.store));
            }
            mapped_depth_stencil_attachment.set_stencil_read_only(dsa.stencil_ops.is_none());
            mapped_desc.set_depth_stencil_attachment(&mapped_depth_stencil_attachment);
        }

        if let Some(ref timestamp_writes) = desc.timestamp_writes {
            let query_set = &timestamp_writes.query_set.inner.as_webgpu().inner;
            let writes = webgpu_sys::GpuRenderPassTimestampWrites::new(query_set);
            if let Some(index) = timestamp_writes.beginning_of_pass_write_index {
                writes.set_beginning_of_pass_write_index(index);
            }
            if let Some(index) = timestamp_writes.end_of_pass_write_index {
                writes.set_end_of_pass_write_index(index);
            }
            mapped_desc.set_timestamp_writes(&writes);
        }

        let render_pass = self.inner.begin_render_pass(&mapped_desc).unwrap();

        WebRenderPassEncoder {
            inner: render_pass,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn finish(&mut self) -> dispatch::DispatchCommandBuffer {
        let label = self.inner.label();
        let buffer = if label.is_empty() {
            self.inner.finish()
        } else {
            let mapped_desc = webgpu_sys::GpuCommandBufferDescriptor::new();
            mapped_desc.set_label(&label);

            self.inner.finish_with_descriptor(&mapped_desc)
        };

        WebCommandBuffer {
            inner: buffer,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }

    fn clear_texture(
        &self,
        _texture: &dispatch::DispatchTexture,
        _subresource_range: &crate::ImageSubresourceRange,
    ) {
        unimplemented!("clear_texture is not yet implemented");
    }

    fn clear_buffer(
        &self,
        buffer: &dispatch::DispatchBuffer,
        offset: crate::BufferAddress,
        size: Option<crate::BufferAddress>,
    ) {
        let buffer = buffer.as_webgpu();

        match size {
            Some(size) => {
                self.inner
                    .clear_buffer_with_f64_and_f64(&buffer.inner, offset as f64, size as f64)
            }
            None => self
                .inner
                .clear_buffer_with_f64(&buffer.inner, offset as f64),
        }
    }

    fn insert_debug_marker(&self, label: &str) {
        self.inner.insert_debug_marker(label)
    }

    fn push_debug_group(&self, group_label: &str) {
        self.inner.push_debug_group(group_label)
    }

    fn pop_debug_group(&self) {
        self.inner.pop_debug_group()
    }

    fn write_timestamp(&self, _query_set: &dispatch::DispatchQuerySet, _query_index: u32) {
        // Not available on WebGPU.
        // This was part of the spec originally but got removed, see https://github.com/gpuweb/gpuweb/pull/4370
        panic!("TIMESTAMP_QUERY_INSIDE_ENCODERS feature must be enabled to call write_timestamp on a command encoder.")
    }

    fn resolve_query_set(
        &self,
        query_set: &dispatch::DispatchQuerySet,
        first_query: u32,
        query_count: u32,
        destination: &dispatch::DispatchBuffer,
        destination_offset: crate::BufferAddress,
    ) {
        let query_set = &query_set.as_webgpu().inner;
        let destination = &destination.as_webgpu().inner;

        self.inner.resolve_query_set_with_u32(
            query_set,
            first_query,
            query_count,
            destination,
            destination_offset as u32,
        );
    }

    fn mark_acceleration_structures_built<'a>(
        &self,
        _blas: &mut dyn Iterator<Item = &'a Blas>,
        _tlas: &mut dyn Iterator<Item = &'a Tlas>,
    ) {
        unimplemented!("Raytracing not implemented for web");
    }

    fn build_acceleration_structures<'a>(
        &self,
        _blas: &mut dyn Iterator<Item = &'a crate::BlasBuildEntry<'a>>,
        _tlas: &mut dyn Iterator<Item = &'a crate::Tlas>,
    ) {
        unimplemented!("Raytracing not implemented for web");
    }

    fn transition_resources<'a>(
        &mut self,
        _buffer_transitions: &mut dyn Iterator<
            Item = wgt::BufferTransition<&'a dispatch::DispatchBuffer>,
        >,
        _texture_transitions: &mut dyn Iterator<
            Item = wgt::TextureTransition<&'a dispatch::DispatchTexture>,
        >,
    ) {
        // no-op
    }
}
impl Drop for WebCommandEncoder {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::PipelineCacheInterface for WebPipelineCache {
    fn get_data(&self) -> Option<Vec<u8>> {
        todo!()
    }
}
impl Drop for WebPipelineCache {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::ComputePassInterface for WebComputePassEncoder {
    fn set_pipeline(&mut self, pipeline: &dispatch::DispatchComputePipeline) {
        let pipeline = &pipeline.as_webgpu().inner;
        self.inner.set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&dispatch::DispatchBindGroup>,
        offsets: &[crate::DynamicOffset],
    ) {
        let bind_group = bind_group.map(|bind_group| &bind_group.as_webgpu().inner);

        if offsets.is_empty() {
            self.inner.set_bind_group(index, bind_group);
        } else {
            self.inner
                .set_bind_group_with_u32_slice_and_f64_and_dynamic_offsets_data_length(
                    index,
                    bind_group,
                    offsets,
                    0f64,
                    offsets.len() as u32,
                )
                .unwrap();
        }
    }

    fn set_immediates(&mut self, _offset: u32, _data: &[u8]) {
        panic!("IMMEDIATES feature must be enabled to call set_immediates")
    }

    fn insert_debug_marker(&mut self, label: &str) {
        self.inner.insert_debug_marker(label);
    }

    fn push_debug_group(&mut self, group_label: &str) {
        self.inner.push_debug_group(group_label);
    }

    fn pop_debug_group(&mut self) {
        self.inner.pop_debug_group();
    }

    fn write_timestamp(&mut self, _query_set: &dispatch::DispatchQuerySet, _query_index: u32) {
        panic!("TIMESTAMP_QUERY_INSIDE_PASSES feature must be enabled to call write_timestamp in a compute pass.")
    }

    fn begin_pipeline_statistics_query(
        &mut self,
        _query_set: &dispatch::DispatchQuerySet,
        _query_index: u32,
    ) {
        // Not available in gecko yet
    }

    fn end_pipeline_statistics_query(&mut self) {
        // Not available in gecko yet
    }

    fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32) {
        self.inner
            .dispatch_workgroups_with_workgroup_count_y_and_workgroup_count_z(x, y, z);
    }

    fn dispatch_workgroups_indirect(
        &mut self,
        indirect_buffer: &dispatch::DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    ) {
        let indirect_buffer = indirect_buffer.as_webgpu();

        self.inner
            .dispatch_workgroups_indirect_with_f64(&indirect_buffer.inner, indirect_offset as f64);
    }
}
impl Drop for WebComputePassEncoder {
    fn drop(&mut self) {
        self.inner.end();
    }
}

impl dispatch::RenderPassInterface for WebRenderPassEncoder {
    fn set_pipeline(&mut self, pipeline: &dispatch::DispatchRenderPipeline) {
        let pipeline = &pipeline.as_webgpu().inner;

        self.inner.set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&dispatch::DispatchBindGroup>,
        offsets: &[crate::DynamicOffset],
    ) {
        let bind_group = bind_group.map(|bind_group| &bind_group.as_webgpu().inner);

        if offsets.is_empty() {
            self.inner.set_bind_group(index, bind_group);
        } else {
            self.inner
                .set_bind_group_with_u32_slice_and_f64_and_dynamic_offsets_data_length(
                    index,
                    bind_group,
                    offsets,
                    0f64,
                    offsets.len() as u32,
                )
                .unwrap();
        }
    }

    fn set_index_buffer(
        &mut self,
        buffer: &dispatch::DispatchBuffer,
        index_format: crate::IndexFormat,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    ) {
        let buffer = buffer.as_webgpu();
        let index_format = map_index_format(index_format);

        if let Some(size) = size {
            self.inner.set_index_buffer_with_f64_and_f64(
                &buffer.inner,
                index_format,
                offset as f64,
                size.get() as f64,
            );
        } else {
            self.inner
                .set_index_buffer_with_f64(&buffer.inner, index_format, offset as f64);
        }
    }

    fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer: &dispatch::DispatchBuffer,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    ) {
        let buffer = buffer.as_webgpu();

        if let Some(size) = size {
            self.inner.set_vertex_buffer_with_f64_and_f64(
                slot,
                Some(&buffer.inner),
                offset as f64,
                size.get() as f64,
            );
        } else {
            self.inner
                .set_vertex_buffer_with_f64(slot, Some(&buffer.inner), offset as f64);
        }
    }

    fn set_immediates(&mut self, _offset: u32, _data: &[u8]) {
        panic!("IMMEDIATES feature must be enabled to call set_immediates")
    }

    fn set_blend_constant(&mut self, color: crate::Color) {
        self.inner
            .set_blend_constant_with_gpu_color_dict(&map_color(color))
            .unwrap();
    }

    fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.inner.set_scissor_rect(x, y, width, height);
    }

    fn set_viewport(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) {
        self.inner
            .set_viewport(x, y, width, height, min_depth, max_depth);
    }

    fn set_stencil_reference(&mut self, reference: u32) {
        self.inner.set_stencil_reference(reference);
    }

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.inner
            .draw_with_instance_count_and_first_vertex_and_first_instance(
                vertices.end - vertices.start,
                instances.end - instances.start,
                vertices.start,
                instances.start,
            );
    }

    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.inner
            .draw_indexed_with_instance_count_and_first_index_and_base_vertex_and_first_instance(
                indices.end - indices.start,
                instances.end - instances.start,
                indices.start,
                base_vertex,
                instances.start,
            )
    }

    fn draw_mesh_tasks(&mut self, _group_count_x: u32, _group_count_y: u32, _group_count_z: u32) {
        panic!("MESH_SHADER feature must be enabled to call draw_mesh_tasks")
    }

    fn draw_indirect(
        &mut self,
        indirect_buffer: &dispatch::DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    ) {
        let buffer = indirect_buffer.as_webgpu();
        self.inner
            .draw_indirect_with_f64(&buffer.inner, indirect_offset as f64);
    }

    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &dispatch::DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    ) {
        let buffer = indirect_buffer.as_webgpu();
        self.inner
            .draw_indexed_indirect_with_f64(&buffer.inner, indirect_offset as f64);
    }

    fn draw_mesh_tasks_indirect(
        &mut self,
        _indirect_buffer: &dispatch::DispatchBuffer,
        _indirect_offset: crate::BufferAddress,
    ) {
        panic!("MESH_SHADER feature must be enabled to call draw_mesh_tasks_indirect")
    }

    fn multi_draw_indirect(
        &mut self,
        indirect_buffer: &dispatch::DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count: u32,
    ) {
        let buffer = indirect_buffer.as_webgpu();

        for i in 0..count {
            let offset = indirect_offset + i as crate::BufferAddress * 16;
            self.inner
                .draw_indirect_with_f64(&buffer.inner, offset as f64);
        }
    }

    fn multi_draw_indexed_indirect(
        &mut self,
        indirect_buffer: &dispatch::DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count: u32,
    ) {
        let buffer = indirect_buffer.as_webgpu();

        for i in 0..count {
            let offset = indirect_offset + i as crate::BufferAddress * 20;
            self.inner
                .draw_indexed_indirect_with_f64(&buffer.inner, offset as f64);
        }
    }

    fn multi_draw_mesh_tasks_indirect(
        &mut self,
        _indirect_buffer: &dispatch::DispatchBuffer,
        _indirect_offset: crate::BufferAddress,
        _count: u32,
    ) {
        panic!("MESH_SHADER feature must be enabled to call multi_draw_mesh_tasks_indirect")
    }

    fn multi_draw_indirect_count(
        &mut self,
        _indirect_buffer: &dispatch::DispatchBuffer,
        _indirect_offset: crate::BufferAddress,
        _count_buffer: &dispatch::DispatchBuffer,
        _count_buffer_offset: crate::BufferAddress,
        _max_count: u32,
    ) {
        panic!(
            "MULTI_DRAW_INDIRECT_COUNT feature must be enabled to call multi_draw_indirect_count"
        )
    }

    fn multi_draw_indexed_indirect_count(
        &mut self,
        _indirect_buffer: &dispatch::DispatchBuffer,
        _indirect_offset: crate::BufferAddress,
        _count_buffer: &dispatch::DispatchBuffer,
        _count_buffer_offset: crate::BufferAddress,
        _max_count: u32,
    ) {
        panic!("MULTI_DRAW_INDIRECT_COUNT feature must be enabled to call multi_draw_indexed_indirect_count")
    }

    fn multi_draw_mesh_tasks_indirect_count(
        &mut self,
        _indirect_buffer: &dispatch::DispatchBuffer,
        _indirect_offset: crate::BufferAddress,
        _count_buffer: &dispatch::DispatchBuffer,
        _count_buffer_offset: crate::BufferAddress,
        _max_count: u32,
    ) {
        panic!("MESH_SHADER feature must be enabled to call multi_draw_mesh_tasks_indirect_count")
    }

    fn insert_debug_marker(&mut self, label: &str) {
        self.inner.insert_debug_marker(label);
    }

    fn push_debug_group(&mut self, group_label: &str) {
        self.inner.push_debug_group(group_label);
    }

    fn pop_debug_group(&mut self) {
        self.inner.pop_debug_group();
    }

    fn write_timestamp(&mut self, _query_set: &dispatch::DispatchQuerySet, _query_index: u32) {
        panic!("TIMESTAMP_QUERY_INSIDE_PASSES feature must be enabled to call write_timestamp in a render pass.")
    }

    fn begin_occlusion_query(&mut self, query_index: u32) {
        self.inner.begin_occlusion_query(query_index);
    }

    fn end_occlusion_query(&mut self) {
        self.inner.end_occlusion_query();
    }

    fn begin_pipeline_statistics_query(
        &mut self,
        _query_set: &dispatch::DispatchQuerySet,
        _query_index: u32,
    ) {
        // Removed from WebGPU in https://github.com/gpuweb/gpuweb/pull/2296
    }

    fn end_pipeline_statistics_query(&mut self) {
        // Removed from WebGPU https://github.com/gpuweb/gpuweb/pull/2296
    }

    fn execute_bundles(
        &mut self,
        render_bundles: &mut dyn Iterator<Item = &dispatch::DispatchRenderBundle>,
    ) {
        let mapped = render_bundles
            .map(|bundle| &bundle.as_webgpu().inner)
            .collect::<js_sys::Array>();
        self.inner.execute_bundles(&mapped);
    }
}
impl Drop for WebRenderPassEncoder {
    fn drop(&mut self) {
        self.inner.end();
    }
}

impl dispatch::CommandBufferInterface for WebCommandBuffer {}
impl Drop for WebCommandBuffer {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::RenderBundleEncoderInterface for WebRenderBundleEncoder {
    fn set_pipeline(&mut self, pipeline: &dispatch::DispatchRenderPipeline) {
        let pipeline = &pipeline.as_webgpu().inner;
        self.inner.set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&dispatch::DispatchBindGroup>,
        offsets: &[crate::DynamicOffset],
    ) {
        let bind_group = bind_group.map(|bind_group| &bind_group.as_webgpu().inner);

        if offsets.is_empty() {
            self.inner.set_bind_group(index, bind_group);
        } else {
            self.inner
                .set_bind_group_with_u32_slice_and_f64_and_dynamic_offsets_data_length(
                    index,
                    bind_group,
                    offsets,
                    0f64,
                    offsets.len() as u32,
                )
                .unwrap();
        }
    }

    fn set_index_buffer(
        &mut self,
        buffer: &dispatch::DispatchBuffer,
        index_format: crate::IndexFormat,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    ) {
        let buffer = buffer.as_webgpu();
        let index_format = map_index_format(index_format);

        if let Some(size) = size {
            self.inner.set_index_buffer_with_f64_and_f64(
                &buffer.inner,
                index_format,
                offset as f64,
                size.get() as f64,
            );
        } else {
            self.inner
                .set_index_buffer_with_f64(&buffer.inner, index_format, offset as f64);
        }
    }

    fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer: &dispatch::DispatchBuffer,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    ) {
        let buffer = buffer.as_webgpu();

        if let Some(size) = size {
            self.inner.set_vertex_buffer_with_f64_and_f64(
                slot,
                Some(&buffer.inner),
                offset as f64,
                size.get() as f64,
            );
        } else {
            self.inner
                .set_vertex_buffer_with_f64(slot, Some(&buffer.inner), offset as f64);
        }
    }

    fn set_immediates(&mut self, _offset: u32, _data: &[u8]) {
        panic!("IMMEDIATES feature must be enabled to call set_immediates")
    }

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.inner
            .draw_with_instance_count_and_first_vertex_and_first_instance(
                vertices.end - vertices.start,
                instances.end - instances.start,
                vertices.start,
                instances.start,
            );
    }

    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.inner
            .draw_indexed_with_instance_count_and_first_index_and_base_vertex_and_first_instance(
                indices.end - indices.start,
                instances.end - instances.start,
                indices.start,
                base_vertex,
                instances.start,
            )
    }

    fn draw_indirect(
        &mut self,
        indirect_buffer: &dispatch::DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    ) {
        let buffer = indirect_buffer.as_webgpu();
        self.inner
            .draw_indirect_with_f64(&buffer.inner, indirect_offset as f64);
    }

    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &dispatch::DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    ) {
        let buffer = indirect_buffer.as_webgpu();
        self.inner
            .draw_indexed_indirect_with_f64(&buffer.inner, indirect_offset as f64);
    }

    fn finish(self, desc: &crate::RenderBundleDescriptor<'_>) -> dispatch::DispatchRenderBundle
    where
        Self: Sized,
    {
        let bundle = match desc.label {
            Some(label) => {
                let mapped_desc = webgpu_sys::GpuRenderBundleDescriptor::new();
                mapped_desc.set_label(label);
                self.inner.finish_with_descriptor(&mapped_desc)
            }
            None => self.inner.finish(),
        };

        WebRenderBundle {
            inner: bundle,
            ident: crate::cmp::Identifier::create(),
        }
        .into()
    }
}
impl Drop for WebRenderBundleEncoder {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::RenderBundleInterface for WebRenderBundle {}
impl Drop for WebRenderBundle {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::SurfaceInterface for WebSurface {
    fn get_capabilities(&self, _adapter: &dispatch::DispatchAdapter) -> wgt::SurfaceCapabilities {
        let mut formats = vec![
            wgt::TextureFormat::Rgba8Unorm,
            wgt::TextureFormat::Bgra8Unorm,
            wgt::TextureFormat::Rgba16Float,
        ];
        let mut mapped_formats = formats.iter().map(|format| map_texture_format(*format));
        // Preferred canvas format will only be either "rgba8unorm" or "bgra8unorm".
        // https://www.w3.org/TR/webgpu/#dom-gpu-getpreferredcanvasformat
        let preferred_format = self
            .gpu
            .as_ref()
            .expect("Caller could not have created an adapter if gpu is undefined.")
            .get_preferred_canvas_format();
        if let Some(index) = mapped_formats.position(|format| format == preferred_format) {
            formats.swap(0, index);
        }

        wgt::SurfaceCapabilities {
            // https://gpuweb.github.io/gpuweb/#supported-context-formats
            formats,
            // Doesn't really have meaning on the web.
            present_modes: vec![wgt::PresentMode::Fifo],
            alpha_modes: vec![wgt::CompositeAlphaMode::Opaque],
            // Statically set to RENDER_ATTACHMENT for now. See https://gpuweb.github.io/gpuweb/#dom-gpucanvasconfiguration-usage
            usages: wgt::TextureUsages::RENDER_ATTACHMENT,
        }
    }

    fn configure(&self, device: &dispatch::DispatchDevice, config: &crate::SurfaceConfiguration) {
        let device = device.as_webgpu();

        match self.canvas {
            Canvas::Canvas(ref canvas) => {
                canvas.set_width(config.width);
                canvas.set_height(config.height);
            }
            Canvas::Offscreen(ref canvas) => {
                canvas.set_width(config.width);
                canvas.set_height(config.height);
            }
        }

        if let wgt::PresentMode::Mailbox | wgt::PresentMode::Immediate = config.present_mode {
            panic!("Only FIFO/Auto* is supported on web");
        }
        if let wgt::CompositeAlphaMode::PostMultiplied | wgt::CompositeAlphaMode::Inherit =
            config.alpha_mode
        {
            panic!("Only Opaque/Auto or PreMultiplied alpha mode are supported on web");
        }
        let alpha_mode = match config.alpha_mode {
            wgt::CompositeAlphaMode::PreMultiplied => webgpu_sys::GpuCanvasAlphaMode::Premultiplied,
            _ => webgpu_sys::GpuCanvasAlphaMode::Opaque,
        };
        let mapped = webgpu_sys::GpuCanvasConfiguration::new(
            &device.inner,
            map_texture_format(config.format),
        );
        mapped.set_usage(config.usage.bits());
        mapped.set_alpha_mode(alpha_mode);
        let mapped_view_formats = config
            .view_formats
            .iter()
            .map(|format| JsValue::from(map_texture_format(*format)))
            .collect::<js_sys::Array>();
        mapped.set_view_formats(&mapped_view_formats);
        self.context.configure(&mapped).unwrap();
    }

    fn get_current_texture(
        &self,
    ) -> (
        Option<dispatch::DispatchTexture>,
        crate::SurfaceStatus,
        dispatch::DispatchSurfaceOutputDetail,
    ) {
        let surface_texture = self.context.get_current_texture().unwrap();

        let web_surface_texture = WebTexture {
            inner: surface_texture,
            ident: crate::cmp::Identifier::create(),
        };

        (
            Some(web_surface_texture.into()),
            crate::SurfaceStatus::Good,
            WebSurfaceOutputDetail {
                ident: crate::cmp::Identifier::create(),
            }
            .into(),
        )
    }
}
impl Drop for WebSurface {
    fn drop(&mut self) {
        // no-op
    }
}

impl dispatch::SurfaceOutputDetailInterface for WebSurfaceOutputDetail {
    fn present(&self) {
        // Swapchain is presented automatically on the web.
    }

    fn texture_discard(&self) {
        // Can't really discard the texture on the web.
    }
}
impl Drop for WebSurfaceOutputDetail {
    fn drop(&mut self) {
        // no-op
    }
}

impl WebBufferMappedRange {
    fn get_temporary_mapping(&self) -> &[u8] {
        self.temporary_mapping
            .get_or_init(|| self.actual_mapping.to_vec())
    }
}
impl dispatch::BufferMappedRangeInterface for WebBufferMappedRange {
    fn len(&self) -> usize {
        self.get_temporary_mapping().len()
    }

    #[inline]
    unsafe fn read_slice(&self) -> &[u8] {
        self.get_temporary_mapping()
    }

    #[inline]
    unsafe fn write_slice(&mut self) -> WriteOnly<'_, [u8]> {
        self.temporary_mapping_modified = true;
        self.get_temporary_mapping();
        let t: &mut Vec<u8> = self.temporary_mapping.get_mut().unwrap();
        WriteOnly::from_mut(t)
    }

    #[inline]
    fn as_uint8array(&self) -> &js_sys::Uint8Array {
        &self.actual_mapping
    }
}
impl Drop for WebBufferMappedRange {
    fn drop(&mut self) {
        if !self.temporary_mapping_modified {
            // For efficiency, skip the copy if it is not needed.
            // This is also how we skip copying back on *read-only* mappings.
            return;
        }

        // Copy from the temporary mapping back into the array buffer that was
        // originally provided by the browser
        let temporary_mapping_slice = self.temporary_mapping.get().unwrap().as_slice();
        unsafe {
            // Note: no allocations can happen between `view` and `set`, or this
            // will break
            self.actual_mapping
                .set(&js_sys::Uint8Array::view(temporary_mapping_slice), 0);
        }
    }
}

impl dispatch::QueueWriteBufferInterface for WebQueueWriteBuffer {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    unsafe fn write_slice(&mut self) -> WriteOnly<'_, [u8]> {
        WriteOnly::from_mut(&mut *self.inner)
    }
}
impl Drop for WebQueueWriteBuffer {
    fn drop(&mut self) {
        // The api struct calls write_staging_buffer

        // no-op
    }
}

/// Adds the constants map to the given pipeline descriptor if the map is nonempty.
/// Panics if the map cannot be set.
///
/// This function is necessary because the constants array is not currently
/// exposed by `wasm-bindgen`. See the following issues for details:
/// - [gfx-rs/wgpu#5688](https://github.com/gfx-rs/wgpu/pull/5688)
/// - [rustwasm/wasm-bindgen#3587](https://github.com/rustwasm/wasm-bindgen/issues/3587)
fn insert_constants_map(target: &JsValue, map: &[(&str, f64)]) {
    if !map.is_empty() {
        js_sys::Reflect::set(
            target,
            &JsValue::from_str("constants"),
            &hashmap_to_jsvalue(map),
        )
        .expect("Setting the values in a Javascript pipeline descriptor should never fail");
    }
}

/// Converts a hashmap to a Javascript object.
fn hashmap_to_jsvalue(map: &[(&str, f64)]) -> JsValue {
    let obj = js_sys::Object::new();

    for &(key, v) in map.iter() {
        js_sys::Reflect::set(&obj, &JsValue::from_str(key), &JsValue::from_f64(v))
            .expect("Setting the values in a Javascript map should never fail");
    }

    JsValue::from(obj)
}
