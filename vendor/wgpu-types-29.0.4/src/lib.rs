//! This library describes the API surface of WebGPU that is agnostic of the backend.
//! This API is used for targeting both Web and Native.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    // We don't use syntax sugar where it's not necessary.
    clippy::match_like_matches_macro,
)]
#![warn(
    clippy::ptr_as_ptr,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    unused_qualifications
)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

use core::{fmt, hash::Hash, time::Duration};

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

mod adapter;
pub mod assertions;
mod backend;
mod binding;
mod buffer;
mod cast_utils;
mod counters;
mod device;
mod env;
pub mod error;
mod features;
pub mod instance;
mod limits;
pub mod math;
mod origin_extent;
mod ray_tracing;
mod render;
#[doc(hidden)] // without this we get spurious missing_docs warnings
mod send_sync;
mod shader;
mod surface;
mod texture;
mod tokens;
mod transfers;
mod vertex;
mod write_only;

pub use adapter::*;
pub use backend::*;
pub use binding::*;
pub use buffer::*;
pub use counters::*;
pub use device::*;
pub use features::*;
pub use instance::*;
pub use limits::*;
pub use origin_extent::*;
pub use ray_tracing::*;
pub use render::*;
#[doc(hidden)]
pub use send_sync::*;
pub use shader::*;
pub use surface::*;
pub use texture::*;
pub use tokens::*;
pub use transfers::*;
pub use vertex::*;
pub use write_only::*;

/// Create a Markdown link definition referring to the `wgpu` crate.
///
/// This macro should be used inside a `#[doc = ...]` attribute.
/// The two arguments should be string literals or macros that expand to string literals.
/// If the module in which the item using this macro is located is not the crate root,
/// use the `../` syntax.
///
/// We cannot simply use rustdoc links to `wgpu` because it is one of our dependents.
/// This link adapts to work in locally generated documentation (`cargo doc`) by default,
/// and work with `docs.rs` URL structure when building for `docs.rs`.
///
/// Note: This macro cannot be used outside this crate, because `cfg(docsrs)` will not apply.
#[cfg(not(docsrs))]
macro_rules! link_to_wgpu_docs {
    ([$reference:expr]: $url_path:expr) => {
        concat!("[", $reference, "]: ../wgpu/", $url_path)
    };

    (../ [$reference:expr]: $url_path:expr) => {
        concat!("[", $reference, "]: ../../wgpu/", $url_path)
    };
}
#[cfg(docsrs)]
macro_rules! link_to_wgpu_docs {
    ($(../)? [$reference:expr]: $url_path:expr) => {
        concat!(
            "[",
            $reference,
            // URL path will have a base URL of https://docs.rs/
            "]: /wgpu/",
            // The version of wgpu-types is not necessarily the same as the version of wgpu
            // if a patch release of either has been published, so we cannot use the full version
            // number. docs.rs will interpret this single number as a Cargo-style version
            // requirement and redirect to the latest compatible version.
            //
            // This technique would break if `wgpu` and `wgpu-types` ever switch to having distinct
            // major version numbering. An alternative would be to hardcode the corresponding `wgpu`
            // version, but that would give us another thing to forget to update.
            env!("CARGO_PKG_VERSION_MAJOR"),
            "/wgpu/",
            $url_path
        )
    };
}

/// Create a Markdown link definition referring to an item in the `wgpu` crate.
///
/// This macro should be used inside a `#[doc = ...]` attribute.
/// See [`link_to_wgpu_docs`] for more details.
macro_rules! link_to_wgpu_item {
    ($kind:ident $name:ident) => {
        $crate::link_to_wgpu_docs!(
            [concat!("`", stringify!($name), "`")]: concat!(stringify!($kind), ".", stringify!($name), ".html")
        )
    };
}

pub(crate) use {link_to_wgpu_docs, link_to_wgpu_item};

/// Integral type used for [`Buffer`] offsets and sizes.
///
#[doc = link_to_wgpu_item!(struct Buffer)]
pub type BufferAddress = u64;

/// Integral type used for [`BufferSlice`] sizes.
///
/// Note that while this type is non-zero, a [`Buffer`] *per se* can have a size of zero,
/// but no slice or mapping can be created from it.
///
#[doc = link_to_wgpu_item!(struct Buffer)]
#[doc = link_to_wgpu_item!(struct BufferSlice)]
pub type BufferSize = core::num::NonZeroU64;

/// Integral type used for binding locations in shaders.
///
/// Used in [`VertexAttribute`]s and errors.
///
#[doc = link_to_wgpu_item!(struct VertexAttribute)]
pub type ShaderLocation = u32;

/// Integral type used for
/// [dynamic bind group offsets](../wgpu/struct.RenderPass.html#method.set_bind_group).
pub type DynamicOffset = u32;

/// Buffer-texture copies must have [`bytes_per_row`] aligned to this number.
///
/// This doesn't apply to [`Queue::write_texture`][Qwt], only to [`copy_buffer_to_texture()`]
/// and [`copy_texture_to_buffer()`].
///
/// [`bytes_per_row`]: TexelCopyBufferLayout::bytes_per_row
#[doc = link_to_wgpu_docs!(["`copy_buffer_to_texture()`"]: "struct.Queue.html#method.copy_buffer_to_texture")]
#[doc = link_to_wgpu_docs!(["`copy_texture_to_buffer()`"]: "struct.Queue.html#method.copy_texture_to_buffer")]
#[doc = link_to_wgpu_docs!(["Qwt"]: "struct.Queue.html#method.write_texture")]
pub const COPY_BYTES_PER_ROW_ALIGNMENT: u32 = 256;

/// An [offset into the query resolve buffer] has to be aligned to this.
///
#[doc = link_to_wgpu_docs!(["offset into the query resolve buffer"]: "struct.CommandEncoder.html#method.resolve_query_set")]
pub const QUERY_RESOLVE_BUFFER_ALIGNMENT: BufferAddress = 256;

/// Buffer to buffer copy as well as buffer clear offsets and sizes must be aligned to this number.
pub const COPY_BUFFER_ALIGNMENT: BufferAddress = 4;

/// Minimum alignment of buffer mappings.
///
/// The range passed to [`map_async()`] or [`get_mapped_range()`] must be at least this aligned.
///
#[doc = link_to_wgpu_docs!(["`map_async()`"]: "struct.Buffer.html#method.map_async")]
#[doc = link_to_wgpu_docs!(["`get_mapped_range()`"]: "struct.Buffer.html#method.get_mapped_range")]
pub const MAP_ALIGNMENT: BufferAddress = 8;

/// [Vertex buffer offsets] and [strides] have to be a multiple of this number.
///
#[doc = link_to_wgpu_docs!(["Vertex buffer offsets"]: "util/trait.RenderEncoder.html#tymethod.set_vertex_buffer")]
#[doc = link_to_wgpu_docs!(["strides"]: "struct.VertexBufferLayout.html#structfield.array_stride")]
pub const VERTEX_ALIGNMENT: BufferAddress = 4;

/// [Vertex buffer strides] have to be a multiple of this number.
///
#[doc = link_to_wgpu_docs!(["Vertex buffer strides"]: "struct.VertexBufferLayout.html#structfield.array_stride")]
#[deprecated(note = "Use `VERTEX_ALIGNMENT` instead", since = "27.0.0")]
pub const VERTEX_STRIDE_ALIGNMENT: BufferAddress = 4;

/// Ranges of [writes to immediate data] must be at least this aligned.
///
#[doc = link_to_wgpu_docs!(["writes to immediate data"]: "struct.RenderPass.html#method.set_immediates")]
pub const IMMEDIATE_DATA_ALIGNMENT: u32 = 4;

/// Storage buffer binding sizes must be multiples of this value.
#[doc(hidden)]
pub const STORAGE_BINDING_SIZE_ALIGNMENT: u32 = 4;

/// Maximum number of query result slots that can be requested in a [`QuerySetDescriptor`].
pub const QUERY_SET_MAX_QUERIES: u32 = 4096;

/// Size in bytes of a single piece of [query] data.
///
#[doc = link_to_wgpu_docs!(["query"]: "struct.QuerySet.html")]
pub const QUERY_SIZE: u32 = 8;

/// The minimum allowed value for [`AdapterInfo::subgroup_min_size`].
///
/// See <https://gpuweb.github.io/gpuweb/#gpuadapterinfo>
/// where you can always use these values on all devices
pub const MINIMUM_SUBGROUP_MIN_SIZE: u32 = 4;
/// The maximum allowed value for [`AdapterInfo::subgroup_max_size`].
///
/// See <https://gpuweb.github.io/gpuweb/#gpuadapterinfo>
/// where you can always use these values on all devices.
pub const MAXIMUM_SUBGROUP_MAX_SIZE: u32 = 128;

/// Passed to `Device::poll` to control how and if it should block.
#[derive(Clone, Debug)]
pub enum PollType<T> {
    /// On wgpu-core based backends, block until the given submission has
    /// completed execution, and any callbacks have been invoked.
    ///
    /// On WebGPU, this has no effect. Callbacks are invoked from the
    /// window event loop.
    Wait {
        /// Submission index to wait for.
        ///
        /// If not specified, will wait for the most recent submission at the time of the poll.
        /// By the time the method returns, more submissions may have taken place.
        submission_index: Option<T>,

        /// Max time to wait for the submission to complete.
        ///
        /// If not specified, will wait indefinitely (or until an error is detected).
        /// If waiting for the GPU device takes this long or longer, the poll will return [`PollError::Timeout`].
        timeout: Option<Duration>,
    },

    /// Check the device for a single time without blocking.
    Poll,
}

impl<T> PollType<T> {
    /// Wait indefinitely until for the most recent submission to complete.
    ///
    /// This is a convenience function that creates a [`Self::Wait`] variant with
    /// no timeout and no submission index.
    #[must_use]
    pub const fn wait_indefinitely() -> Self {
        Self::Wait {
            submission_index: None,
            timeout: None,
        }
    }

    /// This `PollType` represents a wait of some kind.
    #[must_use]
    pub fn is_wait(&self) -> bool {
        match *self {
            Self::Wait { .. } => true,
            Self::Poll => false,
        }
    }

    /// Map on the wait index type.
    #[must_use]
    pub fn map_index<U, F>(self, func: F) -> PollType<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Wait {
                submission_index,
                timeout,
            } => PollType::Wait {
                submission_index: submission_index.map(func),
                timeout,
            },
            Self::Poll => PollType::Poll,
        }
    }
}

/// Error states after a device poll.
#[derive(Debug)]
pub enum PollError {
    /// The requested Wait timed out before the submission was completed.
    Timeout,
    /// The requested Wait was given a wrong submission index.
    WrongSubmissionIndex(u64, u64),
}

// This impl could be derived by `thiserror`, but by not doing so, we can reduce the number of
// dependencies this early in the dependency graph, which may improve build parallelism.
impl fmt::Display for PollError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PollError::Timeout => {
                f.write_str("The requested Wait timed out before the submission was completed.")
            }
            PollError::WrongSubmissionIndex(requested, successful) => write!(
                f,
                "Tried to wait using a submission index ({requested}) \
                that has not been returned by a successful submission \
                (last successful submission: {successful}"
            ),
        }
    }
}

impl core::error::Error for PollError {}

/// Status of device poll operation.
#[derive(Debug, PartialEq, Eq)]
pub enum PollStatus {
    /// There are no active submissions in flight as of the beginning of the poll call.
    /// Other submissions may have been queued on other threads during the call.
    ///
    /// This implies that the given Wait was satisfied before the timeout.
    QueueEmpty,

    /// The requested Wait was satisfied before the timeout.
    WaitSucceeded,

    /// This was a poll.
    Poll,
}

impl PollStatus {
    /// Returns true if the result is [`Self::QueueEmpty`].
    #[must_use]
    pub fn is_queue_empty(&self) -> bool {
        matches!(self, Self::QueueEmpty)
    }

    /// Returns true if the result is either [`Self::WaitSucceeded`] or [`Self::QueueEmpty`].
    #[must_use]
    pub fn wait_finished(&self) -> bool {
        matches!(self, Self::WaitSucceeded | Self::QueueEmpty)
    }
}

/// Describes a [`CommandEncoder`](../wgpu/struct.CommandEncoder.html).
///
/// Corresponds to [WebGPU `GPUCommandEncoderDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpucommandencoderdescriptor).
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommandEncoderDescriptor<L> {
    /// Debug label for the command encoder. This will show up in graphics debuggers for easy identification.
    pub label: L,
}

impl<L> CommandEncoderDescriptor<L> {
    /// Takes a closure and maps the label of the command encoder descriptor into another.
    #[must_use]
    pub fn map_label<K>(&self, fun: impl FnOnce(&L) -> K) -> CommandEncoderDescriptor<K> {
        CommandEncoderDescriptor {
            label: fun(&self.label),
        }
    }
}

impl<T> Default for CommandEncoderDescriptor<Option<T>> {
    fn default() -> Self {
        Self { label: None }
    }
}

/// RGBA double precision color.
///
/// This is not to be used as a generic color type, only for specific wgpu interfaces.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Color {
    /// Red component of the color
    pub r: f64,
    /// Green component of the color
    pub g: f64,
    /// Blue component of the color
    pub b: f64,
    /// Alpha component of the color
    pub a: f64,
}

#[allow(missing_docs)]
impl Color {
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
}

/// Describes a [`CommandBuffer`](../wgpu/struct.CommandBuffer.html).
///
/// Corresponds to [WebGPU `GPUCommandBufferDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpucommandbufferdescriptor).
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommandBufferDescriptor<L> {
    /// Debug label of this command buffer.
    pub label: L,
}

impl<L> CommandBufferDescriptor<L> {
    /// Takes a closure and maps the label of the command buffer descriptor into another.
    #[must_use]
    pub fn map_label<K>(&self, fun: impl FnOnce(&L) -> K) -> CommandBufferDescriptor<K> {
        CommandBufferDescriptor {
            label: fun(&self.label),
        }
    }
}

/// Describes how to create a `QuerySet`.
///
/// Corresponds to [WebGPU `GPUQuerySetDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuquerysetdescriptor).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QuerySetDescriptor<L> {
    /// Debug label for the query set.
    pub label: L,
    /// Kind of query that this query set should contain.
    pub ty: QueryType,
    /// Total number of query result slots the set contains. Must not be zero.
    /// Must not be greater than [`QUERY_SET_MAX_QUERIES`].
    pub count: u32,
}

impl<L> QuerySetDescriptor<L> {
    /// Takes a closure and maps the label of the query set descriptor into another.
    #[must_use]
    pub fn map_label<'a, K>(&'a self, fun: impl FnOnce(&'a L) -> K) -> QuerySetDescriptor<K> {
        QuerySetDescriptor {
            label: fun(&self.label),
            ty: self.ty,
            count: self.count,
        }
    }
}

/// Type of queries contained in a [`QuerySet`].
///
/// Each query set may contain any number of queries, but they must all be of the same type.
///
/// Corresponds to [WebGPU `GPUQueryType`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpuquerytype).
///
#[doc = link_to_wgpu_item!(struct QuerySet)]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum QueryType {
    /// An occlusion query reports whether any of the fragments drawn within the scope of the query
    /// passed all per-fragment tests (i.e. were not occluded).
    ///
    /// Occlusion queries are performed by setting [`RenderPassDescriptor::occlusion_query_set`],
    /// then calling [`RenderPass::begin_occlusion_query()`] and
    /// [`RenderPass::end_occlusion_query()`].
    /// The query writes to a single result slot in the query set, whose value will be either 0 or 1
    /// as a boolean.
    ///
    #[doc = link_to_wgpu_docs!(["`RenderPassDescriptor::occlusion_query_set`"]: "struct.RenderPassDescriptor.html#structfield.occlusion_query_set")]
    #[doc = link_to_wgpu_docs!(["`RenderPass::begin_occlusion_query()`"]: "struct.RenderPass.html#structfield.begin_occlusion_query")]
    #[doc = link_to_wgpu_docs!(["`RenderPass::end_occlusion_query()`"]: "struct.RenderPass.html#structfield.end_occlusion_query")]
    Occlusion,

    /// A timestamp query records a GPU-timestamp value
    /// at which a certain command started or finished executing.
    ///
    /// Timestamp queries are performed by any one of:
    /// * Setting [`ComputePassDescriptor::timestamp_writes`]
    /// * Setting [`RenderPassDescriptor::timestamp_writes`]
    /// * Calling [`CommandEncoder::write_timestamp()`]
    /// * Calling [`RenderPass::write_timestamp()`]
    /// * Calling [`ComputePass::write_timestamp()`]
    ///
    /// Each timestamp query writes to a single result slot in the query set.
    /// The timestamp value must be multiplied by [`Queue::get_timestamp_period()`][Qgtp] to get
    /// the time in nanoseconds.
    /// Absolute values have no meaning, but timestamps can be subtracted to get the time it takes
    /// for a string of operations to complete.
    /// Timestamps may overflow and wrap to 0, resulting in occasional spurious negative deltas.
    ///
    /// Additionally, passes may be executed in parallel or out of the order they were submitted;
    /// this does not affect their results but is observable via these timestamps.
    ///
    /// [`Features::TIMESTAMP_QUERY`] must be enabled to use this query type.
    ///
    #[doc = link_to_wgpu_docs!(["`CommandEncoder::write_timestamp()`"]: "struct.CommandEncoder.html#method.write_timestamp")]
    #[doc = link_to_wgpu_docs!(["`ComputePass::write_timestamp()`"]: "struct.ComputePass.html#method.write_timestamp")]
    #[doc = link_to_wgpu_docs!(["`RenderPass::write_timestamp()`"]: "struct.RenderPass.html#method.write_timestamp")]
    #[doc = link_to_wgpu_docs!(["`ComputePassDescriptor::timestamp_writes`"]: "struct.ComputePassDescriptor.html#structfield.timestamp_writes")]
    #[doc = link_to_wgpu_docs!(["`RenderPassDescriptor::timestamp_writes`"]: "struct.RenderPassDescriptor.html#structfield.timestamp_writes")]
    #[doc = link_to_wgpu_docs!(["Qgtp"]: "struct.Queue.html#method.get_timestamp_period")]
    Timestamp,

    /// A pipeline statistics query records information about the execution of pipelines;
    /// see [`PipelineStatisticsTypes`]'s documentation for details.
    ///
    /// Pipeline statistics queries are performed by:
    ///
    /// * [`ComputePass::begin_pipeline_statistics_query()`]
    /// * [`RenderPass::begin_pipeline_statistics_query()`]
    ///
    /// A single query may occupy up to 5 result slots in the query set, based on the flags given
    /// here.
    ///
    /// [`Features::PIPELINE_STATISTICS_QUERY`] must be enabled to use this query type.
    ///
    #[doc = link_to_wgpu_docs!(["`ComputePass::begin_pipeline_statistics_query()`"]: "struct.ComputePass.html#method.begin_pipeline_statistics_query")]
    #[doc = link_to_wgpu_docs!(["`RenderPass::begin_pipeline_statistics_query()`"]: "struct.RenderPass.html#method.begin_pipeline_statistics_query")]
    PipelineStatistics(PipelineStatisticsTypes),
}

bitflags::bitflags! {
    /// Flags for which pipeline data should be recorded in a query.
    ///
    /// Used in [`QueryType`].
    ///
    /// The amount of values written when resolved depends
    /// on the amount of flags set. For example, if 3 flags are set, 3
    /// 64-bit values will be written per query.
    ///
    /// The order they are written is the order they are declared
    /// in these bitflags. For example, if you enabled `CLIPPER_PRIMITIVES_OUT`
    /// and `COMPUTE_SHADER_INVOCATIONS`, it would write 16 bytes,
    /// the first 8 bytes being the primitive out value, the last 8
    /// bytes being the compute shader invocation count.
    #[repr(transparent)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineStatisticsTypes : u8 {
        /// Amount of times the vertex shader is ran. Accounts for
        /// the vertex cache when doing indexed rendering.
        const VERTEX_SHADER_INVOCATIONS = 1 << 0;
        /// Amount of times the clipper is invoked. This
        /// is also the amount of triangles output by the vertex shader.
        const CLIPPER_INVOCATIONS = 1 << 1;
        /// Amount of primitives that are not culled by the clipper.
        /// This is the amount of triangles that are actually on screen
        /// and will be rasterized and rendered.
        const CLIPPER_PRIMITIVES_OUT = 1 << 2;
        /// Amount of times the fragment shader is ran. Accounts for
        /// fragment shaders running in 2x2 blocks in order to get
        /// derivatives.
        const FRAGMENT_SHADER_INVOCATIONS = 1 << 3;
        /// Amount of times a compute shader is invoked. This will
        /// be equivalent to the dispatch count times the workgroup size.
        const COMPUTE_SHADER_INVOCATIONS = 1 << 4;
    }
}

/// Corresponds to a [`GPUDeviceLostReason`].
///
/// [`GPUDeviceLostReason`]: https://www.w3.org/TR/webgpu/#enumdef-gpudevicelostreason
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeviceLostReason {
    /// The device was lost for an unspecific reason, including driver errors.
    Unknown = 0,
    /// The device's `destroy` method was called.
    Destroyed = 1,
}
