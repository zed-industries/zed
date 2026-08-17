use alloc::{string::String, vec::Vec};
use core::{future::Future, marker::PhantomData};

use crate::*;

/// Handle to a compiled shader module.
///
/// A `ShaderModule` represents a compiled shader module on the GPU. It can be created by passing
/// source code to [`Device::create_shader_module`]. MSL shader or SPIR-V binary can also be passed
/// directly using [`Device::create_shader_module_passthrough`]. Shader modules are used to define
/// programmable stages of a pipeline.
///
/// Corresponds to [WebGPU `GPUShaderModule`](https://gpuweb.github.io/gpuweb/#shader-module).
#[derive(Debug, Clone)]
pub struct ShaderModule {
    pub(crate) inner: dispatch::DispatchShaderModule,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(ShaderModule: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(ShaderModule => .inner);

impl ShaderModule {
    /// Get the compilation info for the shader module.
    pub fn get_compilation_info(&self) -> impl Future<Output = CompilationInfo> + WasmNotSend {
        self.inner.get_compilation_info()
    }

    #[cfg(custom)]
    /// Returns custom implementation of ShaderModule (if custom backend and is internally T)
    pub fn as_custom<T: custom::ShaderModuleInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// Compilation information for a shader module.
///
/// Corresponds to [WebGPU `GPUCompilationInfo`](https://gpuweb.github.io/gpuweb/#gpucompilationinfo).
/// The source locations use bytes, and index a UTF-8 encoded string.
#[derive(Debug, Clone)]
pub struct CompilationInfo {
    /// The messages from the shader compilation process.
    pub messages: Vec<CompilationMessage>,
}

/// A single message from the shader compilation process.
///
/// Roughly corresponds to [`GPUCompilationMessage`](https://www.w3.org/TR/webgpu/#gpucompilationmessage),
/// except that the location uses UTF-8 for all positions.
#[derive(Debug, Clone)]
pub struct CompilationMessage {
    /// The text of the message.
    pub message: String,
    /// The type of the message.
    pub message_type: CompilationMessageType,
    /// Where in the source code the message points at.
    pub location: Option<SourceLocation>,
}

/// The type of a compilation message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationMessageType {
    /// An error message.
    Error,
    /// A warning message.
    Warning,
    /// An informational message.
    Info,
}

/// A human-readable representation for a span, tailored for text source.
///
/// Roughly corresponds to the positional members of [`GPUCompilationMessage`][gcm] from
/// the WebGPU specification, except
/// - `offset` and `length` are in bytes (UTF-8 code units), instead of UTF-16 code units.
/// - `line_position` is in bytes (UTF-8 code units), and is usually not directly intended for humans.
///
/// [gcm]: https://www.w3.org/TR/webgpu/#gpucompilationmessage
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SourceLocation {
    /// 1-based line number.
    pub line_number: u32,
    /// 1-based column in code units (in bytes) of the start of the span.
    /// Remember to convert accordingly when displaying to the user.
    pub line_position: u32,
    /// 0-based Offset in code units (in bytes) of the start of the span.
    pub offset: u32,
    /// Length in code units (in bytes) of the span.
    pub length: u32,
}

#[cfg(all(feature = "wgsl", wgpu_core))]
impl From<crate::naga::error::ShaderError<crate::naga::front::wgsl::ParseError>>
    for CompilationInfo
{
    fn from(value: crate::naga::error::ShaderError<crate::naga::front::wgsl::ParseError>) -> Self {
        use alloc::{string::ToString, vec};
        CompilationInfo {
            messages: vec![CompilationMessage {
                message: value.to_string(),
                message_type: CompilationMessageType::Error,
                location: value.inner.location(&value.source).map(Into::into),
            }],
        }
    }
}
#[cfg(feature = "glsl")]
impl From<naga::error::ShaderError<naga::front::glsl::ParseErrors>> for CompilationInfo {
    fn from(value: naga::error::ShaderError<naga::front::glsl::ParseErrors>) -> Self {
        use alloc::string::ToString;
        let messages = value
            .inner
            .errors
            .into_iter()
            .map(|err| CompilationMessage {
                message: err.to_string(),
                message_type: CompilationMessageType::Error,
                location: err.location(&value.source).map(Into::into),
            })
            .collect();
        CompilationInfo { messages }
    }
}

#[cfg(feature = "spirv")]
impl From<naga::error::ShaderError<naga::front::spv::Error>> for CompilationInfo {
    fn from(value: naga::error::ShaderError<naga::front::spv::Error>) -> Self {
        use alloc::{string::ToString, vec};
        CompilationInfo {
            messages: vec![CompilationMessage {
                message: value.to_string(),
                message_type: CompilationMessageType::Error,
                location: None,
            }],
        }
    }
}

#[cfg(any(wgpu_core, naga))]
impl
    From<
        crate::naga::error::ShaderError<crate::naga::WithSpan<crate::naga::valid::ValidationError>>,
    > for CompilationInfo
{
    fn from(
        value: crate::naga::error::ShaderError<
            crate::naga::WithSpan<crate::naga::valid::ValidationError>,
        >,
    ) -> Self {
        use alloc::{string::ToString, vec};
        CompilationInfo {
            messages: vec![CompilationMessage {
                message: value.to_string(),
                message_type: CompilationMessageType::Error,
                location: value.inner.location(&value.source).map(Into::into),
            }],
        }
    }
}

#[cfg(any(wgpu_core, naga))]
impl From<crate::naga::SourceLocation> for SourceLocation {
    fn from(value: crate::naga::SourceLocation) -> Self {
        SourceLocation {
            length: value.length,
            offset: value.offset,
            line_number: value.line_number,
            line_position: value.line_position,
        }
    }
}

/// Source of a shader module.
///
/// The source will be parsed and validated.
///
/// Any necessary shader translation (e.g. from WGSL to SPIR-V or vice versa)
/// will be done internally by wgpu.
///
/// This type is unique to the Rust API of `wgpu`. In the WebGPU specification,
/// only WGSL source code strings are accepted.
#[cfg_attr(feature = "naga-ir", expect(clippy::large_enum_variant))]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ShaderSource<'a> {
    /// SPIR-V module represented as a slice of words.
    ///
    /// See also: [`util::make_spirv`], [`include_spirv`]
    #[cfg(feature = "spirv")]
    SpirV(alloc::borrow::Cow<'a, [u32]>),
    /// GLSL module as a string slice.
    ///
    /// Note: GLSL is not yet fully supported and must be a specific ShaderStage.
    #[cfg(feature = "glsl")]
    Glsl {
        /// The source code of the shader.
        shader: alloc::borrow::Cow<'a, str>,
        /// The shader stage that the shader targets. For example, `naga::ShaderStage::Vertex`
        stage: naga::ShaderStage,
        /// Key-value pairs to represent defines sent to the glsl preprocessor.
        ///
        /// If the same name is defined multiple times, the last value is used.
        defines: &'a [(&'a str, &'a str)],
    },
    /// WGSL module as a string slice.
    #[cfg(feature = "wgsl")]
    Wgsl(alloc::borrow::Cow<'a, str>),
    /// Naga module.
    #[cfg(feature = "naga-ir")]
    Naga(alloc::borrow::Cow<'static, naga::Module>),
    /// Dummy variant because `Naga` doesn't have a lifetime and without enough active features it
    /// could be the last one active.
    #[doc(hidden)]
    Dummy(PhantomData<&'a ()>),
}
static_assertions::assert_impl_all!(ShaderSource<'_>: Send, Sync);

/// Descriptor for use with [`Device::create_shader_module`].
///
/// Corresponds to [WebGPU `GPUShaderModuleDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpushadermoduledescriptor).
#[derive(Clone, Debug)]
pub struct ShaderModuleDescriptor<'a> {
    /// Debug label of the shader module. This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// Source code for the shader.
    pub source: ShaderSource<'a>,
}
static_assertions::assert_impl_all!(ShaderModuleDescriptor<'_>: Send, Sync);

/// Descriptor for a shader module given by any of several sources.
/// At least one of the shader types that may be used by the backend must be `Some`
///
/// This type is unique to the Rust API of `wgpu`. In the WebGPU specification,
/// only WGSL source code strings are accepted.
pub type ShaderModuleDescriptorPassthrough<'a> =
    wgt::CreateShaderModuleDescriptorPassthrough<'a, Label<'a>>;
