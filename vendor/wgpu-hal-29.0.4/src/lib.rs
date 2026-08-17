//! A cross-platform unsafe graphics abstraction.
//!
//! This crate defines a set of traits abstracting over modern graphics APIs,
//! with implementations ("backends") for Vulkan, Metal, Direct3D, and GL.
//!
//! `wgpu-hal` is a spiritual successor to
//! [gfx-hal](https://github.com/gfx-rs/gfx), but with reduced scope, and
//! oriented towards WebGPU implementation goals. It has no overhead for
//! validation or tracking, and the API translation overhead is kept to the bare
//! minimum by the design of WebGPU. This API can be used for resource-demanding
//! applications and engines.
//!
//! The `wgpu-hal` crate's main design choices:
//!
//! - Our traits are meant to be *portable*: proper use
//!   should get equivalent results regardless of the backend.
//!
//! - Our traits' contracts are *unsafe*: implementations perform minimal
//!   validation, if any, and incorrect use will often cause undefined behavior.
//!   This allows us to minimize the overhead we impose over the underlying
//!   graphics system. If you need safety, the [`wgpu-core`] crate provides a
//!   safe API for driving `wgpu-hal`, implementing all necessary validation,
//!   resource state tracking, and so on. (Note that `wgpu-core` is designed for
//!   use via FFI; the [`wgpu`] crate provides more idiomatic Rust bindings for
//!   `wgpu-core`.) Or, you can do your own validation.
//!
//! - In the same vein, returned errors *only cover cases the user can't
//!   anticipate*, like running out of memory or losing the device. Any errors
//!   that the user could reasonably anticipate are their responsibility to
//!   avoid. For example, `wgpu-hal` returns no error for mapping a buffer that's
//!   not mappable: as the buffer creator, the user should already know if they
//!   can map it.
//!
//! - We use *static dispatch*. The traits are not
//!   generally object-safe. You must select a specific backend type
//!   like [`vulkan::Api`] or [`metal::Api`], and then use that
//!   according to the main traits, or call backend-specific methods.
//!
//! - We use *idiomatic Rust parameter passing*,
//!   taking objects by reference, returning them by value, and so on,
//!   unlike `wgpu-core`, which refers to objects by ID.
//!
//! - We map buffer contents *persistently*. This means that the buffer can
//!   remain mapped on the CPU while the GPU reads or writes to it. You must
//!   explicitly indicate when data might need to be transferred between CPU and
//!   GPU, if [`Device::map_buffer`] indicates that this is necessary.
//!
//! - You must record *explicit barriers* between different usages of a
//!   resource. For example, if a buffer is written to by a compute
//!   shader, and then used as and index buffer to a draw call, you
//!   must use [`CommandEncoder::transition_buffers`] between those two
//!   operations.
//!
//! - Pipeline layouts are *explicitly specified* when setting bind groups.
//!   Incompatible layouts disturb groups bound at higher indices.
//!
//! - The API *accepts collections as iterators*, to avoid forcing the user to
//!   store data in particular containers. The implementation doesn't guarantee
//!   that any of the iterators are drained, unless stated otherwise by the
//!   function documentation. For this reason, we recommend that iterators don't
//!   do any mutating work.
//!
//! Unfortunately, `wgpu-hal`'s safety requirements are not fully documented.
//! Ideally, all trait methods would have doc comments setting out the
//! requirements users must meet to ensure correct and portable behavior. If you
//! are aware of a specific requirement that a backend imposes that is not
//! ensured by the traits' documented rules, please file an issue. Or, if you are
//! a capable technical writer, please file a pull request!
//!
//! [`wgpu-core`]: https://crates.io/crates/wgpu-core
//! [`wgpu`]: https://crates.io/crates/wgpu
//! [`vulkan::Api`]: vulkan/struct.Api.html
//! [`metal::Api`]: metal/struct.Api.html
//!
//! ## Primary backends
//!
//! The `wgpu-hal` crate has full-featured backends implemented on the following
//! platform graphics APIs:
//!
//! - Vulkan, available on Linux, Android, and Windows, using the [`ash`] crate's
//!   Vulkan bindings. It's also available on macOS, if you install [MoltenVK].
//!
//! - Metal on macOS, using the [`metal`] crate's bindings.
//!
//! - Direct3D 12 on Windows, using the [`windows`] crate's bindings.
//!
//! [`ash`]: https://crates.io/crates/ash
//! [MoltenVK]: https://github.com/KhronosGroup/MoltenVK
//! [`metal`]: https://crates.io/crates/metal
//! [`windows`]: https://crates.io/crates/windows
//!
//! ## Secondary backends
//!
//! The `wgpu-hal` crate has a partial implementation based on the following
//! platform graphics API:
//!
//! - The GL backend is available anywhere OpenGL, OpenGL ES, or WebGL are
//!   available. See the [`gles`] module documentation for details.
//!
//! [`gles`]: gles/index.html
//!
//! You can see what capabilities an adapter is missing by checking the
//! [`DownlevelCapabilities`][tdc] in [`ExposedAdapter::capabilities`], available
//! from [`Instance::enumerate_adapters`].
//!
//! The API is generally designed to fit the primary backends better than the
//! secondary backends, so the latter may impose more overhead.
//!
//! [tdc]: wgt::DownlevelCapabilities
//!
//! ## Traits
//!
//! The `wgpu-hal` crate defines a handful of traits that together
//! represent a cross-platform abstraction for modern GPU APIs.
//!
//! - The [`Api`] trait represents a `wgpu-hal` backend. It has no methods of its
//!   own, only a collection of associated types.
//!
//! - [`Api::Instance`] implements the [`Instance`] trait. [`Instance::init`]
//!   creates an instance value, which you can use to enumerate the adapters
//!   available on the system. For example, [`vulkan::Api::Instance::init`][Ii]
//!   returns an instance that can enumerate the Vulkan physical devices on your
//!   system.
//!
//! - [`Api::Adapter`] implements the [`Adapter`] trait, representing a
//!   particular device from a particular backend. For example, a Vulkan instance
//!   might have a Lavapipe software adapter and a GPU-based adapter.
//!
//! - [`Api::Device`] implements the [`Device`] trait, representing an active
//!   link to a device. You get a device value by calling [`Adapter::open`], and
//!   then use it to create buffers, textures, shader modules, and so on.
//!
//! - [`Api::Queue`] implements the [`Queue`] trait, which you use to submit
//!   command buffers to a given device.
//!
//! - [`Api::CommandEncoder`] implements the [`CommandEncoder`] trait, which you
//!   use to build buffers of commands to submit to a queue. This has all the
//!   methods for drawing and running compute shaders, which is presumably what
//!   you're here for.
//!
//! - [`Api::Surface`] implements the [`Surface`] trait, which represents a
//!   swapchain for presenting images on the screen, via interaction with the
//!   system's window manager.
//!
//! The [`Api`] trait has various other associated types like [`Api::Buffer`] and
//! [`Api::Texture`] that represent resources the rest of the interface can
//! operate on, but these generally do not have their own traits.
//!
//! [Ii]: Instance::init
//!
//! ## Validation is the calling code's responsibility, not `wgpu-hal`'s
//!
//! As much as possible, `wgpu-hal` traits place the burden of validation,
//! resource tracking, and state tracking on the caller, not on the trait
//! implementations themselves. Anything which can reasonably be handled in
//! backend-independent code should be. A `wgpu_hal` backend's sole obligation is
//! to provide portable behavior, and report conditions that the calling code
//! can't reasonably anticipate, like device loss or running out of memory.
//!
//! The `wgpu` crate collection is intended for use in security-sensitive
//! applications, like web browsers, where the API is available to untrusted
//! code. This means that `wgpu-core`'s validation is not simply a service to
//! developers, to be provided opportunistically when the performance costs are
//! acceptable and the necessary data is ready at hand. Rather, `wgpu-core`'s
//! validation must be exhaustive, to ensure that even malicious content cannot
//! provoke and exploit undefined behavior in the platform's graphics API.
//!
//! Because graphics APIs' requirements are complex, the only practical way for
//! `wgpu` to provide exhaustive validation is to comprehensively track the
//! lifetime and state of all the resources in the system. Implementing this
//! separately for each backend is infeasible; effort would be better spent
//! making the cross-platform validation in `wgpu-core` legible and trustworthy.
//! Fortunately, the requirements are largely similar across the various
//! platforms, so cross-platform validation is practical.
//!
//! Some backends have specific requirements that aren't practical to foist off
//! on the `wgpu-hal` user. For example, properly managing macOS Objective-C or
//! Microsoft COM reference counts is best handled by using appropriate pointer
//! types within the backend.
//!
//! A desire for "defense in depth" may suggest performing additional validation
//! in `wgpu-hal` when the opportunity arises, but this must be done with
//! caution. Even experienced contributors infer the expectations their changes
//! must meet by considering not just requirements made explicit in types, tests,
//! assertions, and comments, but also those implicit in the surrounding code.
//! When one sees validation or state-tracking code in `wgpu-hal`, it is tempting
//! to conclude, "Oh, `wgpu-hal` checks for this, so `wgpu-core` needn't worry
//! about it - that would be redundant!" The responsibility for exhaustive
//! validation always rests with `wgpu-core`, regardless of what may or may not
//! be checked in `wgpu-hal`.
//!
//! To this end, any "defense in depth" validation that does appear in `wgpu-hal`
//! for requirements that `wgpu-core` should have enforced should report failure
//! via the `unreachable!` macro, because problems detected at this stage always
//! indicate a bug in `wgpu-core`.
//!
//! ## Debugging
//!
//! Most of the information on the wiki [Debugging wgpu Applications][wiki-debug]
//! page still applies to this API, with the exception of API tracing/replay
//! functionality, which is only available in `wgpu-core`.
//!
//! [wiki-debug]: https://github.com/gfx-rs/wgpu/wiki/Debugging-wgpu-Applications

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    // this happens on the GL backend, where it is both thread safe and non-thread safe in the same code.
    clippy::arc_with_non_send_sync,
    // We don't use syntax sugar where it's not necessary.
    clippy::match_like_matches_macro,
    // Redundant matching is more explicit.
    clippy::redundant_pattern_matching,
    // Explicit lifetimes are often easier to reason about.
    clippy::needless_lifetimes,
    // No need for defaults in the internal types.
    clippy::new_without_default,
    // Matches are good and extendable, no need to make an exception here.
    clippy::single_match,
    // Push commands are more regular than macros.
    clippy::vec_init_then_push,
    // We unsafe impl `Send` for a reason.
    clippy::non_send_fields_in_send_ty,
    // TODO!
    clippy::missing_safety_doc,
    // It gets in the way a lot and does not prevent bugs in practice.
    clippy::pattern_type_mismatch,
    // We should investigate these.
    clippy::large_enum_variant
)]
#![warn(
    clippy::alloc_instead_of_core,
    clippy::ptr_as_ptr,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_op_in_unsafe_fn,
    unused_extern_crates,
    unused_qualifications
)]

extern crate alloc;
extern crate wgpu_types as wgt;
// Each of these backends needs `std` in some fashion; usually `std::thread` functions.
#[cfg(any(dx12, gles_with_std, metal, vulkan))]
#[macro_use]
extern crate std;

/// DirectX12 API internals.
#[cfg(dx12)]
pub mod dx12;
/// GLES API internals.
#[cfg(gles)]
pub mod gles;
/// Metal API internals.
#[cfg(metal)]
pub mod metal;
/// A dummy API implementation.
// TODO(https://github.com/gfx-rs/wgpu/issues/7120): this should have a cfg
pub mod noop;
/// Vulkan API internals.
#[cfg(vulkan)]
pub mod vulkan;

pub mod auxil;
pub mod api {
    #[cfg(dx12)]
    pub use super::dx12::Api as Dx12;
    #[cfg(gles)]
    pub use super::gles::Api as Gles;
    #[cfg(metal)]
    pub use super::metal::Api as Metal;
    pub use super::noop::Api as Noop;
    #[cfg(vulkan)]
    pub use super::vulkan::Api as Vulkan;
}

mod dynamic;
#[cfg(feature = "validation_canary")]
mod validation_canary;

#[cfg(feature = "validation_canary")]
pub use validation_canary::{ValidationCanary, VALIDATION_CANARY};

pub(crate) use dynamic::impl_dyn_resource;
pub use dynamic::{
    DynAccelerationStructure, DynAcquiredSurfaceTexture, DynAdapter, DynBindGroup,
    DynBindGroupLayout, DynBuffer, DynCommandBuffer, DynCommandEncoder, DynComputePipeline,
    DynDevice, DynExposedAdapter, DynFence, DynInstance, DynOpenDevice, DynPipelineCache,
    DynPipelineLayout, DynQuerySet, DynQueue, DynRenderPipeline, DynResource, DynSampler,
    DynShaderModule, DynSurface, DynSurfaceTexture, DynTexture, DynTextureView,
};

#[allow(unused)]
use alloc::boxed::Box;
use alloc::{borrow::Cow, string::String, vec::Vec};
use core::{
    borrow::Borrow,
    error::Error,
    fmt,
    num::{NonZeroU32, NonZeroU64},
    ops::{Range, RangeInclusive},
    ptr::NonNull,
};

use bitflags::bitflags;
use raw_window_handle::DisplayHandle;
use thiserror::Error;
use wgt::WasmNotSendSync;

cfg_if::cfg_if! {
    if #[cfg(supports_ptr_atomics)] {
        use alloc::sync::Arc;
    } else if #[cfg(feature = "portable-atomic")] {
        use portable_atomic_util::Arc;
    }
}

// - Vertex + Fragment
// - Compute
// Task + Mesh + Fragment
pub const MAX_CONCURRENT_SHADER_STAGES: usize = 3;
pub const MAX_ANISOTROPY: u8 = 16;
pub const MAX_BIND_GROUPS: usize = 8;
pub const MAX_VERTEX_BUFFERS: usize = 16;
pub const MAX_COLOR_ATTACHMENTS: usize = 8;
pub const MAX_MIP_LEVELS: u32 = 16;
/// Size of a single occlusion/timestamp query, when copied into a buffer, in bytes.
/// cbindgen:ignore
pub const QUERY_SIZE: wgt::BufferAddress = 8;

pub type Label<'a> = Option<&'a str>;
pub type MemoryRange = Range<wgt::BufferAddress>;
pub type FenceValue = u64;
#[cfg(supports_64bit_atomics)]
pub type AtomicFenceValue = core::sync::atomic::AtomicU64;
#[cfg(not(supports_64bit_atomics))]
pub type AtomicFenceValue = portable_atomic::AtomicU64;

/// A callback to signal that wgpu is no longer using a resource.
#[cfg(any(gles, vulkan))]
pub type DropCallback = Box<dyn FnOnce() + Send + Sync + 'static>;

#[cfg(any(gles, vulkan))]
pub struct DropGuard {
    callback: Option<DropCallback>,
}

#[cfg(all(any(gles, vulkan), any(native, Emscripten)))]
impl DropGuard {
    fn from_option(callback: Option<DropCallback>) -> Option<Self> {
        callback.map(|callback| Self {
            callback: Some(callback),
        })
    }
}

#[cfg(any(gles, vulkan))]
impl Drop for DropGuard {
    fn drop(&mut self) {
        if let Some(cb) = self.callback.take() {
            (cb)();
        }
    }
}

#[cfg(any(gles, vulkan))]
impl fmt::Debug for DropGuard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DropGuard").finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DeviceError {
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Device is lost")]
    Lost,
    #[error("Unexpected error variant (driver implementation is at fault)")]
    Unexpected,
}

#[cfg(any(dx12, vulkan))]
impl From<gpu_allocator::AllocationError> for DeviceError {
    fn from(result: gpu_allocator::AllocationError) -> Self {
        match result {
            gpu_allocator::AllocationError::OutOfMemory => Self::OutOfMemory,
            gpu_allocator::AllocationError::FailedToMap(e) => {
                log::error!("gpu-allocator: Failed to map: {e}");
                Self::Lost
            }
            gpu_allocator::AllocationError::NoCompatibleMemoryTypeFound => {
                log::error!("gpu-allocator: No Compatible Memory Type Found");
                Self::Lost
            }
            gpu_allocator::AllocationError::InvalidAllocationCreateDesc => {
                log::error!("gpu-allocator: Invalid Allocation Creation Description");
                Self::Lost
            }
            gpu_allocator::AllocationError::InvalidAllocatorCreateDesc(e) => {
                log::error!("gpu-allocator: Invalid Allocator Creation Description: {e}");
                Self::Lost
            }

            gpu_allocator::AllocationError::Internal(e) => {
                log::error!("gpu-allocator: Internal Error: {e}");
                Self::Lost
            }
            gpu_allocator::AllocationError::BarrierLayoutNeedsDevice10
            | gpu_allocator::AllocationError::CastableFormatsRequiresEnhancedBarriers
            | gpu_allocator::AllocationError::CastableFormatsRequiresAtLeastDevice12 => {
                unreachable!()
            }
        }
    }
}

// A copy of gpu_allocator::AllocationSizes, allowing to read the configured value for
// the dx12 backend, we should instead add getters to gpu_allocator::AllocationSizes
// and remove this type.
// https://github.com/Traverse-Research/gpu-allocator/issues/295
#[cfg_attr(not(any(dx12, vulkan)), expect(dead_code))]
pub(crate) struct AllocationSizes {
    pub(crate) min_device_memblock_size: u64,
    pub(crate) max_device_memblock_size: u64,
    pub(crate) min_host_memblock_size: u64,
    pub(crate) max_host_memblock_size: u64,
}

impl AllocationSizes {
    #[allow(dead_code)] // may be unused on some platforms
    pub(crate) fn from_memory_hints(memory_hints: &wgt::MemoryHints) -> Self {
        // TODO: the allocator's configuration should take hardware capability into
        // account.
        const MB: u64 = 1024 * 1024;

        match memory_hints {
            wgt::MemoryHints::Performance => Self {
                min_device_memblock_size: 128 * MB,
                max_device_memblock_size: 256 * MB,
                min_host_memblock_size: 64 * MB,
                max_host_memblock_size: 128 * MB,
            },
            wgt::MemoryHints::MemoryUsage => Self {
                min_device_memblock_size: 8 * MB,
                max_device_memblock_size: 64 * MB,
                min_host_memblock_size: 4 * MB,
                max_host_memblock_size: 32 * MB,
            },
            wgt::MemoryHints::Manual {
                suballocated_device_memory_block_size,
            } => {
                // TODO: https://github.com/gfx-rs/wgpu/issues/8625
                // Would it be useful to expose the host size in memory hints
                // instead of always using half of the device size?
                let device_size = suballocated_device_memory_block_size;
                let host_size = device_size.start / 2..device_size.end / 2;

                // gpu_allocator clamps the sizes between 4MiB and 256MiB, but we clamp them ourselves since we use
                // the sizes when detecting high memory pressure and there is no way to query the values otherwise.
                Self {
                    min_device_memblock_size: device_size.start.clamp(4 * MB, 256 * MB),
                    max_device_memblock_size: device_size.end.clamp(4 * MB, 256 * MB),
                    min_host_memblock_size: host_size.start.clamp(4 * MB, 256 * MB),
                    max_host_memblock_size: host_size.end.clamp(4 * MB, 256 * MB),
                }
            }
        }
    }
}

#[cfg(any(dx12, vulkan))]
impl From<AllocationSizes> for gpu_allocator::AllocationSizes {
    fn from(value: AllocationSizes) -> gpu_allocator::AllocationSizes {
        gpu_allocator::AllocationSizes::new(
            value.min_device_memblock_size,
            value.min_host_memblock_size,
        )
        .with_max_device_memblock_size(value.max_device_memblock_size)
        .with_max_host_memblock_size(value.max_host_memblock_size)
    }
}

#[allow(dead_code)] // may be unused on some platforms
#[cold]
fn hal_usage_error<T: fmt::Display>(txt: T) -> ! {
    panic!("wgpu-hal invariant was violated (usage error): {txt}")
}

#[allow(dead_code)] // may be unused on some platforms
#[cold]
fn hal_internal_error<T: fmt::Display>(txt: T) -> ! {
    panic!("wgpu-hal ran into a preventable internal error: {txt}")
}

#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum ShaderError {
    #[error("Compilation failed: {0:?}")]
    Compilation(String),
    #[error(transparent)]
    Device(#[from] DeviceError),
}

#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum PipelineError {
    #[error("Linkage failed for stage {0:?}: {1}")]
    Linkage(wgt::ShaderStages, String),
    #[error("Entry point for stage {0:?} is invalid")]
    EntryPoint(naga::ShaderStage),
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("Pipeline constant error for stage {0:?}: {1}")]
    PipelineConstants(wgt::ShaderStages, String),
}

#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum PipelineCacheError {
    #[error(transparent)]
    Device(#[from] DeviceError),
}

#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum SurfaceError {
    #[error("Surface is lost")]
    Lost,
    #[error("Surface is outdated, needs to be re-created")]
    Outdated,
    #[error("Timed out waiting for a surface texture")]
    Timeout,
    #[error("The window is occluded (e.g. minimized or behind another window). Try again once the window is no longer occluded.")]
    Occluded,
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("Other reason: {0}")]
    Other(&'static str),
}

/// Error occurring while trying to create an instance, or create a surface from an instance;
/// typically relating to the state of the underlying graphics API or hardware.
#[derive(Clone, Debug, Error)]
#[error("{message}")]
pub struct InstanceError {
    /// These errors are very platform specific, so do not attempt to encode them as an enum.
    ///
    /// This message should describe the problem in sufficient detail to be useful for a
    /// user-to-developer “why won't this work on my machine” bug report, and otherwise follow
    /// <https://rust-lang.github.io/api-guidelines/interoperability.html#error-types-are-meaningful-and-well-behaved-c-good-err>.
    message: String,

    /// Underlying error value, if any is available.
    #[source]
    source: Option<Arc<dyn Error + Send + Sync + 'static>>,
}

impl InstanceError {
    #[allow(dead_code)] // may be unused on some platforms
    pub(crate) fn new(message: String) -> Self {
        Self {
            message,
            source: None,
        }
    }
    #[allow(dead_code)] // may be unused on some platforms
    pub(crate) fn with_source(message: String, source: impl Error + Send + Sync + 'static) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(supports_ptr_atomics)] {
                let source = Arc::new(source);
            } else {
                // TODO(https://github.com/rust-lang/rust/issues/18598): avoid indirection via Box once arbitrary types support unsized coercion
                let source: Box<dyn Error + Send + Sync + 'static> = Box::new(source);
                let source = Arc::from(source);
            }
        }
        Self {
            message,
            source: Some(source),
        }
    }
}

/// All the types and methods that make up a implementation on top of a backend.
///
/// Only the types that have non-dyn trait bounds have methods on them. Most methods
/// are either on [`CommandEncoder`] or [`Device`].
///
/// The api can either be used through generics (through use of this trait and associated
/// types) or dynamically through using the `Dyn*` traits.
pub trait Api: Clone + fmt::Debug + Sized + WasmNotSendSync + 'static {
    const VARIANT: wgt::Backend;

    type Instance: DynInstance + Instance<A = Self>;
    type Surface: DynSurface + Surface<A = Self>;
    type Adapter: DynAdapter + Adapter<A = Self>;
    type Device: DynDevice + Device<A = Self>;

    type Queue: DynQueue + Queue<A = Self>;
    type CommandEncoder: DynCommandEncoder + CommandEncoder<A = Self>;

    /// This API's command buffer type.
    ///
    /// The only thing you can do with `CommandBuffer`s is build them
    /// with a [`CommandEncoder`] and then pass them to
    /// [`Queue::submit`] for execution, or destroy them by passing
    /// them to [`CommandEncoder::reset_all`].
    ///
    /// [`CommandEncoder`]: Api::CommandEncoder
    type CommandBuffer: DynCommandBuffer;

    type Buffer: DynBuffer;
    type Texture: DynTexture;
    type SurfaceTexture: DynSurfaceTexture + Borrow<Self::Texture>;
    type TextureView: DynTextureView;
    type Sampler: DynSampler;
    type QuerySet: DynQuerySet;

    /// A value you can block on to wait for something to finish.
    ///
    /// A `Fence` holds a monotonically increasing [`FenceValue`]. You can call
    /// [`Device::wait`] to block until a fence reaches or passes a value you
    /// choose. [`Queue::submit`] can take a `Fence` and a [`FenceValue`] to
    /// store in it when the submitted work is complete.
    ///
    /// Attempting to set a fence to a value less than its current value has no
    /// effect.
    ///
    /// Waiting on a fence returns as soon as the fence reaches *or passes* the
    /// requested value. This implies that, in order to reliably determine when
    /// an operation has completed, operations must finish in order of
    /// increasing fence values: if a higher-valued operation were to finish
    /// before a lower-valued operation, then waiting for the fence to reach the
    /// lower value could return before the lower-valued operation has actually
    /// finished.
    type Fence: DynFence;

    type BindGroupLayout: DynBindGroupLayout;
    type BindGroup: DynBindGroup;
    type PipelineLayout: DynPipelineLayout;
    type ShaderModule: DynShaderModule;
    type RenderPipeline: DynRenderPipeline;
    type ComputePipeline: DynComputePipeline;
    type PipelineCache: DynPipelineCache;

    type AccelerationStructure: DynAccelerationStructure + 'static;
}

pub trait Instance: Sized + WasmNotSendSync {
    type A: Api;

    unsafe fn init(desc: &InstanceDescriptor<'_>) -> Result<Self, InstanceError>;
    unsafe fn create_surface(
        &self,
        display_handle: raw_window_handle::RawDisplayHandle,
        window_handle: raw_window_handle::RawWindowHandle,
    ) -> Result<<Self::A as Api>::Surface, InstanceError>;
    /// `surface_hint` is only used by the GLES backend targeting WebGL2
    unsafe fn enumerate_adapters(
        &self,
        surface_hint: Option<&<Self::A as Api>::Surface>,
    ) -> Vec<ExposedAdapter<Self::A>>;
}

pub trait Surface: WasmNotSendSync {
    type A: Api;

    /// Configure `self` to use `device`.
    ///
    /// # Safety
    ///
    /// - All GPU work using `self` must have been completed.
    /// - All [`AcquiredSurfaceTexture`]s must have been destroyed.
    /// - All [`Api::TextureView`]s derived from the [`AcquiredSurfaceTexture`]s must have been destroyed.
    /// - The surface `self` must not currently be configured to use any other [`Device`].
    unsafe fn configure(
        &self,
        device: &<Self::A as Api>::Device,
        config: &SurfaceConfiguration,
    ) -> Result<(), SurfaceError>;

    /// Unconfigure `self` on `device`.
    ///
    /// # Safety
    ///
    /// - All GPU work that uses `surface` must have been completed.
    /// - All [`AcquiredSurfaceTexture`]s must have been destroyed.
    /// - All [`Api::TextureView`]s derived from the [`AcquiredSurfaceTexture`]s must have been destroyed.
    /// - The surface `self` must have been configured on `device`.
    unsafe fn unconfigure(&self, device: &<Self::A as Api>::Device);

    /// Return the next texture to be presented by `self`, for the caller to draw on.
    ///
    /// On success, return an [`AcquiredSurfaceTexture`] representing the
    /// texture into which the caller should draw the image to be displayed on
    /// `self`.
    ///
    /// If `timeout` elapses before `self` has a texture ready to be acquired,
    /// return `Err(SurfaceError::Timeout)`. If `timeout` is `None`, wait
    /// indefinitely, with no timeout.
    ///
    /// # Using an [`AcquiredSurfaceTexture`]
    ///
    /// On success, this function returns an [`AcquiredSurfaceTexture`] whose
    /// [`texture`] field is a [`SurfaceTexture`] from which the caller can
    /// [`borrow`] a [`Texture`] to draw on. The [`AcquiredSurfaceTexture`] also
    /// carries some metadata about that [`SurfaceTexture`].
    ///
    /// All calls to [`Queue::submit`] that draw on that [`Texture`] must also
    /// include the [`SurfaceTexture`] in the `surface_textures` argument.
    ///
    /// When you are done drawing on the texture, you can display it on `self`
    /// by passing the [`SurfaceTexture`] and `self` to [`Queue::present`].
    ///
    /// If you do not wish to display the texture, you must pass the
    /// [`SurfaceTexture`] to [`self.discard_texture`], so that it can be reused
    /// by future acquisitions.
    ///
    /// # Portability
    ///
    /// Some backends can't support a timeout when acquiring a texture. On these
    /// backends, `timeout` is ignored.
    ///
    /// On macOS, this returns `Err(SurfaceError::Timeout)` when the window is
    /// not visible (minimized, fully occluded, or on another virtual desktop)
    /// to avoid blocking in `CAMetalLayer.nextDrawable()`.
    ///
    /// # Safety
    ///
    /// - The surface `self` must currently be configured on some [`Device`].
    ///
    /// - The `fence` argument must be the same [`Fence`] passed to all calls to
    ///   [`Queue::submit`] that used [`Texture`]s acquired from this surface.
    ///
    /// - You may only have one texture acquired from `self` at a time. When
    ///   `acquire_texture` returns `Ok(ast)`, you must pass the returned
    ///   [`SurfaceTexture`] `ast.texture` to either [`Queue::present`] or
    ///   [`Surface::discard_texture`] before calling `acquire_texture` again.
    ///
    /// [`texture`]: AcquiredSurfaceTexture::texture
    /// [`SurfaceTexture`]: Api::SurfaceTexture
    /// [`borrow`]: alloc::borrow::Borrow::borrow
    /// [`Texture`]: Api::Texture
    /// [`Fence`]: Api::Fence
    /// [`self.discard_texture`]: Surface::discard_texture
    unsafe fn acquire_texture(
        &self,
        timeout: Option<core::time::Duration>,
        fence: &<Self::A as Api>::Fence,
    ) -> Result<AcquiredSurfaceTexture<Self::A>, SurfaceError>;

    /// Relinquish an acquired texture without presenting it.
    ///
    /// After this call, the texture underlying [`SurfaceTexture`] may be
    /// returned by subsequent calls to [`self.acquire_texture`].
    ///
    /// # Safety
    ///
    /// - The surface `self` must currently be configured on some [`Device`].
    ///
    /// - `texture` must be a [`SurfaceTexture`] returned by a call to
    ///   [`self.acquire_texture`] that has not yet been passed to
    ///   [`Queue::present`].
    ///
    /// [`SurfaceTexture`]: Api::SurfaceTexture
    /// [`self.acquire_texture`]: Surface::acquire_texture
    unsafe fn discard_texture(&self, texture: <Self::A as Api>::SurfaceTexture);
}

pub trait Adapter: WasmNotSendSync {
    type A: Api;

    unsafe fn open(
        &self,
        features: wgt::Features,
        limits: &wgt::Limits,
        memory_hints: &wgt::MemoryHints,
    ) -> Result<OpenDevice<Self::A>, DeviceError>;

    /// Return the set of supported capabilities for a texture format.
    unsafe fn texture_format_capabilities(
        &self,
        format: wgt::TextureFormat,
    ) -> TextureFormatCapabilities;

    /// Returns the capabilities of working with a specified surface.
    ///
    /// `None` means presentation is not supported for it.
    unsafe fn surface_capabilities(
        &self,
        surface: &<Self::A as Api>::Surface,
    ) -> Option<SurfaceCapabilities>;

    /// Creates a [`PresentationTimestamp`] using the adapter's WSI.
    ///
    /// [`PresentationTimestamp`]: wgt::PresentationTimestamp
    unsafe fn get_presentation_timestamp(&self) -> wgt::PresentationTimestamp;

    /// The combination of all usages that the are guaranteed to be be ordered by the hardware.
    /// If a usage is ordered, then if the buffer state doesn't change between draw calls,
    /// there are no barriers needed for synchronization.
    fn get_ordered_buffer_usages(&self) -> wgt::BufferUses;

    /// The combination of all usages that the are guaranteed to be be ordered by the hardware.
    /// If a usage is ordered, then if the buffer state doesn't change between draw calls,
    /// there are no barriers needed for synchronization.
    fn get_ordered_texture_usages(&self) -> wgt::TextureUses;
}

/// A connection to a GPU and a pool of resources to use with it.
///
/// A `wgpu-hal` `Device` represents an open connection to a specific graphics
/// processor, controlled via the backend [`Device::A`]. A `Device` is mostly
/// used for creating resources. Each `Device` has an associated [`Queue`] used
/// for command submission.
///
/// On Vulkan a `Device` corresponds to a logical device ([`VkDevice`]). Other
/// backends don't have an exact analog: for example, [`ID3D12Device`]s and
/// [`MTLDevice`]s are owned by the backends' [`wgpu_hal::Adapter`]
/// implementations, and shared by all [`wgpu_hal::Device`]s created from that
/// `Adapter`.
///
/// A `Device`'s life cycle is generally:
///
/// 1)  Obtain a `Device` and its associated [`Queue`] by calling
///     [`Adapter::open`].
///
///     Alternatively, the backend-specific types that implement [`Adapter`] often
///     have methods for creating a `wgpu-hal` `Device` from a platform-specific
///     handle. For example, [`vulkan::Adapter::device_from_raw`] can create a
///     [`vulkan::Device`] from an [`ash::Device`].
///
/// 1)  Create resources to use on the device by calling methods like
///     [`Device::create_texture`] or [`Device::create_shader_module`].
///
/// 1)  Call [`Device::create_command_encoder`] to obtain a [`CommandEncoder`],
///     which you can use to build [`CommandBuffer`]s holding commands to be
///     executed on the GPU.
///
/// 1)  Call [`Queue::submit`] on the `Device`'s associated [`Queue`] to submit
///     [`CommandBuffer`]s for execution on the GPU. If needed, call
///     [`Device::wait`] to wait for them to finish execution.
///
/// 1)  Free resources with methods like [`Device::destroy_texture`] or
///     [`Device::destroy_shader_module`].
///
/// 1)  Drop the device.
///
/// [`vkDevice`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#VkDevice
/// [`ID3D12Device`]: https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12device
/// [`MTLDevice`]: https://developer.apple.com/documentation/metal/mtldevice
/// [`wgpu_hal::Adapter`]: Adapter
/// [`wgpu_hal::Device`]: Device
/// [`vulkan::Adapter::device_from_raw`]: vulkan/struct.Adapter.html#method.device_from_raw
/// [`vulkan::Device`]: vulkan/struct.Device.html
/// [`ash::Device`]: https://docs.rs/ash/latest/ash/struct.Device.html
/// [`CommandBuffer`]: Api::CommandBuffer
///
/// # Safety
///
/// As with other `wgpu-hal` APIs, [validation] is the caller's
/// responsibility. Here are the general requirements for all `Device`
/// methods:
///
/// - Any resource passed to a `Device` method must have been created by that
///   `Device`. For example, a [`Texture`] passed to [`Device::destroy_texture`] must
///   have been created with the `Device` passed as `self`.
///
/// - Resources may not be destroyed if they are used by any submitted command
///   buffers that have not yet finished execution.
///
/// [validation]: index.html#validation-is-the-calling-codes-responsibility-not-wgpu-hals
/// [`Texture`]: Api::Texture
pub trait Device: WasmNotSendSync {
    type A: Api;

    /// Creates a new buffer.
    ///
    /// The initial usage is `wgt::BufferUses::empty()`.
    unsafe fn create_buffer(
        &self,
        desc: &BufferDescriptor,
    ) -> Result<<Self::A as Api>::Buffer, DeviceError>;

    /// Free `buffer` and any GPU resources it owns.
    ///
    /// Note that backends are allowed to allocate GPU memory for buffers from
    /// allocation pools, and this call is permitted to simply return `buffer`'s
    /// storage to that pool, without making it available to other applications.
    ///
    /// # Safety
    ///
    /// - The given `buffer` must not currently be mapped.
    unsafe fn destroy_buffer(&self, buffer: <Self::A as Api>::Buffer);

    /// A hook for when a wgpu-core buffer is created from a raw wgpu-hal buffer.
    unsafe fn add_raw_buffer(&self, buffer: &<Self::A as Api>::Buffer);

    /// Return a pointer to CPU memory mapping the contents of `buffer`.
    ///
    /// Buffer mappings are persistent: the buffer may remain mapped on the CPU
    /// while the GPU reads or writes to it. (Note that `wgpu_core` does not use
    /// this feature: when a `wgpu_core::Buffer` is unmapped, the underlying
    /// `wgpu_hal` buffer is also unmapped.)
    ///
    /// If this function returns `Ok(mapping)`, then:
    ///
    /// - `mapping.ptr` is the CPU address of the start of the mapped memory.
    ///
    /// - If `mapping.is_coherent` is `true`, then CPU writes to the mapped
    ///   memory are immediately visible on the GPU, and vice versa.
    ///
    /// # Safety
    ///
    /// - The given `buffer` must have been created with the [`MAP_READ`] or
    ///   [`MAP_WRITE`] flags set in [`BufferDescriptor::usage`].
    ///
    /// - The given `range` must fall within the size of `buffer`.
    ///
    /// - The caller must avoid data races between the CPU and the GPU. A data
    ///   race is any pair of accesses to a particular byte, one of which is a
    ///   write, that are not ordered with respect to each other by some sort of
    ///   synchronization operation.
    ///
    /// - If this function returns `Ok(mapping)` and `mapping.is_coherent` is
    ///   `false`, then:
    ///
    ///   - Every CPU write to a mapped byte followed by a GPU read of that byte
    ///     must have at least one call to [`Device::flush_mapped_ranges`]
    ///     covering that byte that occurs between those two accesses.
    ///
    ///   - Every GPU write to a mapped byte followed by a CPU read of that byte
    ///     must have at least one call to [`Device::invalidate_mapped_ranges`]
    ///     covering that byte that occurs between those two accesses.
    ///
    ///   Note that the data race rule above requires that all such access pairs
    ///   be ordered, so it is meaningful to talk about what must occur
    ///   "between" them.
    ///
    /// - Zero-sized mappings are not allowed.
    ///
    /// - The returned [`BufferMapping::ptr`] must not be used after a call to
    ///   [`Device::unmap_buffer`].
    ///
    /// [`MAP_READ`]: wgt::BufferUses::MAP_READ
    /// [`MAP_WRITE`]: wgt::BufferUses::MAP_WRITE
    unsafe fn map_buffer(
        &self,
        buffer: &<Self::A as Api>::Buffer,
        range: MemoryRange,
    ) -> Result<BufferMapping, DeviceError>;

    /// Remove the mapping established by the last call to [`Device::map_buffer`].
    ///
    /// # Safety
    ///
    /// - The given `buffer` must be currently mapped.
    unsafe fn unmap_buffer(&self, buffer: &<Self::A as Api>::Buffer);

    /// Indicate that CPU writes to mapped buffer memory should be made visible to the GPU.
    ///
    /// # Safety
    ///
    /// - The given `buffer` must be currently mapped.
    ///
    /// - All ranges produced by `ranges` must fall within `buffer`'s size.
    unsafe fn flush_mapped_ranges<I>(&self, buffer: &<Self::A as Api>::Buffer, ranges: I)
    where
        I: Iterator<Item = MemoryRange>;

    /// Indicate that GPU writes to mapped buffer memory should be made visible to the CPU.
    ///
    /// # Safety
    ///
    /// - The given `buffer` must be currently mapped.
    ///
    /// - All ranges produced by `ranges` must fall within `buffer`'s size.
    unsafe fn invalidate_mapped_ranges<I>(&self, buffer: &<Self::A as Api>::Buffer, ranges: I)
    where
        I: Iterator<Item = MemoryRange>;

    /// Creates a new texture.
    ///
    /// The initial usage for all subresources is `wgt::TextureUses::UNINITIALIZED`.
    unsafe fn create_texture(
        &self,
        desc: &TextureDescriptor,
    ) -> Result<<Self::A as Api>::Texture, DeviceError>;
    unsafe fn destroy_texture(&self, texture: <Self::A as Api>::Texture);

    /// A hook for when a wgpu-core texture is created from a raw wgpu-hal texture.
    unsafe fn add_raw_texture(&self, texture: &<Self::A as Api>::Texture);

    unsafe fn create_texture_view(
        &self,
        texture: &<Self::A as Api>::Texture,
        desc: &TextureViewDescriptor,
    ) -> Result<<Self::A as Api>::TextureView, DeviceError>;
    unsafe fn destroy_texture_view(&self, view: <Self::A as Api>::TextureView);
    unsafe fn create_sampler(
        &self,
        desc: &SamplerDescriptor,
    ) -> Result<<Self::A as Api>::Sampler, DeviceError>;
    unsafe fn destroy_sampler(&self, sampler: <Self::A as Api>::Sampler);

    /// Create a fresh [`CommandEncoder`].
    ///
    /// The new `CommandEncoder` is in the "closed" state.
    unsafe fn create_command_encoder(
        &self,
        desc: &CommandEncoderDescriptor<<Self::A as Api>::Queue>,
    ) -> Result<<Self::A as Api>::CommandEncoder, DeviceError>;

    /// Creates a bind group layout.
    unsafe fn create_bind_group_layout(
        &self,
        desc: &BindGroupLayoutDescriptor,
    ) -> Result<<Self::A as Api>::BindGroupLayout, DeviceError>;
    unsafe fn destroy_bind_group_layout(&self, bg_layout: <Self::A as Api>::BindGroupLayout);
    unsafe fn create_pipeline_layout(
        &self,
        desc: &PipelineLayoutDescriptor<<Self::A as Api>::BindGroupLayout>,
    ) -> Result<<Self::A as Api>::PipelineLayout, DeviceError>;
    unsafe fn destroy_pipeline_layout(&self, pipeline_layout: <Self::A as Api>::PipelineLayout);

    #[allow(clippy::type_complexity)]
    unsafe fn create_bind_group(
        &self,
        desc: &BindGroupDescriptor<
            <Self::A as Api>::BindGroupLayout,
            <Self::A as Api>::Buffer,
            <Self::A as Api>::Sampler,
            <Self::A as Api>::TextureView,
            <Self::A as Api>::AccelerationStructure,
        >,
    ) -> Result<<Self::A as Api>::BindGroup, DeviceError>;
    unsafe fn destroy_bind_group(&self, group: <Self::A as Api>::BindGroup);

    unsafe fn create_shader_module(
        &self,
        desc: &ShaderModuleDescriptor,
        shader: ShaderInput,
    ) -> Result<<Self::A as Api>::ShaderModule, ShaderError>;
    unsafe fn destroy_shader_module(&self, module: <Self::A as Api>::ShaderModule);

    #[allow(clippy::type_complexity)]
    unsafe fn create_render_pipeline(
        &self,
        desc: &RenderPipelineDescriptor<
            <Self::A as Api>::PipelineLayout,
            <Self::A as Api>::ShaderModule,
            <Self::A as Api>::PipelineCache,
        >,
    ) -> Result<<Self::A as Api>::RenderPipeline, PipelineError>;
    unsafe fn destroy_render_pipeline(&self, pipeline: <Self::A as Api>::RenderPipeline);

    #[allow(clippy::type_complexity)]
    unsafe fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDescriptor<
            <Self::A as Api>::PipelineLayout,
            <Self::A as Api>::ShaderModule,
            <Self::A as Api>::PipelineCache,
        >,
    ) -> Result<<Self::A as Api>::ComputePipeline, PipelineError>;
    unsafe fn destroy_compute_pipeline(&self, pipeline: <Self::A as Api>::ComputePipeline);

    unsafe fn create_pipeline_cache(
        &self,
        desc: &PipelineCacheDescriptor<'_>,
    ) -> Result<<Self::A as Api>::PipelineCache, PipelineCacheError>;
    fn pipeline_cache_validation_key(&self) -> Option<[u8; 16]> {
        None
    }
    unsafe fn destroy_pipeline_cache(&self, cache: <Self::A as Api>::PipelineCache);

    unsafe fn create_query_set(
        &self,
        desc: &wgt::QuerySetDescriptor<Label>,
    ) -> Result<<Self::A as Api>::QuerySet, DeviceError>;
    unsafe fn destroy_query_set(&self, set: <Self::A as Api>::QuerySet);
    unsafe fn create_fence(&self) -> Result<<Self::A as Api>::Fence, DeviceError>;
    unsafe fn destroy_fence(&self, fence: <Self::A as Api>::Fence);
    unsafe fn get_fence_value(
        &self,
        fence: &<Self::A as Api>::Fence,
    ) -> Result<FenceValue, DeviceError>;

    /// Wait for `fence` to reach `value`.
    ///
    /// Operations like [`Queue::submit`] can accept a [`Fence`] and a
    /// [`FenceValue`] to store in it, so you can use this `wait` function
    /// to wait for a given queue submission to finish execution.
    ///
    /// The `value` argument must be a value that some actual operation you have
    /// already presented to the device is going to store in `fence`. You cannot
    /// wait for values yet to be submitted. (This restriction accommodates
    /// implementations like the `vulkan` backend's [`FencePool`] that must
    /// allocate a distinct synchronization object for each fence value one is
    /// able to wait for.)
    ///
    /// Calling `wait` with a lower [`FenceValue`] than `fence`'s current value
    /// returns immediately.
    ///
    /// If `timeout` is provided, the function will block indefinitely or until
    /// an error is encountered.
    ///
    /// Returns `Ok(true)` on success and `Ok(false)` on timeout.
    ///
    /// [`Fence`]: Api::Fence
    /// [`FencePool`]: vulkan/enum.Fence.html#variant.FencePool
    unsafe fn wait(
        &self,
        fence: &<Self::A as Api>::Fence,
        value: FenceValue,
        timeout: Option<core::time::Duration>,
    ) -> Result<bool, DeviceError>;

    /// Start a graphics debugger capture.
    ///
    /// # Safety
    ///
    /// See [`wgpu::Device::start_graphics_debugger_capture`][api] for more details.
    ///
    /// [api]: ../wgpu/struct.Device.html#method.start_graphics_debugger_capture
    unsafe fn start_graphics_debugger_capture(&self) -> bool;

    /// Stop a graphics debugger capture.
    ///
    /// # Safety
    ///
    /// See [`wgpu::Device::stop_graphics_debugger_capture`][api] for more details.
    ///
    /// [api]: ../wgpu/struct.Device.html#method.stop_graphics_debugger_capture
    unsafe fn stop_graphics_debugger_capture(&self);

    #[allow(unused_variables)]
    unsafe fn pipeline_cache_get_data(
        &self,
        cache: &<Self::A as Api>::PipelineCache,
    ) -> Option<Vec<u8>> {
        None
    }

    unsafe fn create_acceleration_structure(
        &self,
        desc: &AccelerationStructureDescriptor,
    ) -> Result<<Self::A as Api>::AccelerationStructure, DeviceError>;
    unsafe fn get_acceleration_structure_build_sizes(
        &self,
        desc: &GetAccelerationStructureBuildSizesDescriptor<<Self::A as Api>::Buffer>,
    ) -> AccelerationStructureBuildSizes;
    unsafe fn get_acceleration_structure_device_address(
        &self,
        acceleration_structure: &<Self::A as Api>::AccelerationStructure,
    ) -> wgt::BufferAddress;
    unsafe fn destroy_acceleration_structure(
        &self,
        acceleration_structure: <Self::A as Api>::AccelerationStructure,
    );
    fn tlas_instance_to_bytes(&self, instance: TlasInstance) -> Vec<u8>;

    fn get_internal_counters(&self) -> wgt::HalCounters;

    fn generate_allocator_report(&self) -> Option<wgt::AllocatorReport> {
        None
    }

    fn check_if_oom(&self) -> Result<(), DeviceError>;
}

pub trait Queue: WasmNotSendSync {
    type A: Api;

    /// Submit `command_buffers` for execution on GPU.
    ///
    /// Update `fence` to `value` when the operation is complete. See
    /// [`Fence`] for details.
    ///
    /// All command buffers submitted to a `wgpu_hal` queue are executed in the
    /// order they're submitted, with each buffer able to observe the effects of
    /// previous buffers' execution. Specifically:
    ///
    /// - If two calls to `submit` on a single `Queue` occur in a particular
    ///   order (that is, they happen on the same thread, or on two threads that
    ///   have synchronized to establish an ordering), then the first
    ///   submission's commands all complete execution before any of the second
    ///   submission's commands begin. All results produced by one submission
    ///   are visible to the next.
    ///
    /// - Within a submission, command buffers execute in the order in which they
    ///   appear in `command_buffers`. All results produced by one buffer are
    ///   visible to the next.
    ///
    /// If two calls to `submit` on a single `Queue` from different threads are
    /// not synchronized to occur in a particular order, they must pass distinct
    /// [`Fence`]s. As explained in the [`Fence`] documentation, waiting for
    /// operations to complete is only trustworthy when operations finish in
    /// order of increasing fence value, but submissions from different threads
    /// cannot determine how to order the fence values if the submissions
    /// themselves are unordered. If each thread uses a separate [`Fence`], this
    /// problem does not arise.
    ///
    /// # Safety
    ///
    /// - Each [`CommandBuffer`][cb] in `command_buffers` must have been created
    ///   from a [`CommandEncoder`][ce] that was constructed from the
    ///   [`Device`][d] associated with this [`Queue`].
    ///
    /// - Each [`CommandBuffer`][cb] must remain alive until the submitted
    ///   commands have finished execution. Since command buffers must not
    ///   outlive their encoders, this implies that the encoders must remain
    ///   alive as well.
    ///
    /// - All resources used by a submitted [`CommandBuffer`][cb]
    ///   ([`Texture`][t]s, [`BindGroup`][bg]s, [`RenderPipeline`][rp]s, and so
    ///   on) must remain alive until the command buffer finishes execution.
    ///
    /// - Every [`SurfaceTexture`][st] that any command in `command_buffers`
    ///   writes to must appear in the `surface_textures` argument.
    ///
    /// - No [`SurfaceTexture`][st] may appear in the `surface_textures`
    ///   argument more than once.
    ///
    /// - Each [`SurfaceTexture`][st] in `surface_textures` must be configured
    ///   for use with the [`Device`][d] associated with this [`Queue`],
    ///   typically by calling [`Surface::configure`].
    ///
    /// - All calls to this function that include a given [`SurfaceTexture`][st]
    ///   in `surface_textures` must use the same [`Fence`].
    ///
    /// - The [`Fence`] passed as `signal_fence.0` must remain alive until
    ///   all submissions that will signal it have completed.
    ///
    /// [`Fence`]: Api::Fence
    /// [cb]: Api::CommandBuffer
    /// [ce]: Api::CommandEncoder
    /// [d]: Api::Device
    /// [t]: Api::Texture
    /// [bg]: Api::BindGroup
    /// [rp]: Api::RenderPipeline
    /// [st]: Api::SurfaceTexture
    unsafe fn submit(
        &self,
        command_buffers: &[&<Self::A as Api>::CommandBuffer],
        surface_textures: &[&<Self::A as Api>::SurfaceTexture],
        signal_fence: (&mut <Self::A as Api>::Fence, FenceValue),
    ) -> Result<(), DeviceError>;
    unsafe fn present(
        &self,
        surface: &<Self::A as Api>::Surface,
        texture: <Self::A as Api>::SurfaceTexture,
    ) -> Result<(), SurfaceError>;
    unsafe fn get_timestamp_period(&self) -> f32;
}

/// Encoder and allocation pool for `CommandBuffer`s.
///
/// A `CommandEncoder` not only constructs `CommandBuffer`s but also
/// acts as the allocation pool that owns the buffers' underlying
/// storage. Thus, `CommandBuffer`s must not outlive the
/// `CommandEncoder` that created them.
///
/// The life cycle of a `CommandBuffer` is as follows:
///
/// - Call [`Device::create_command_encoder`] to create a new
///   `CommandEncoder`, in the "closed" state.
///
/// - Call `begin_encoding` on a closed `CommandEncoder` to begin
///   recording commands. This puts the `CommandEncoder` in the
///   "recording" state.
///
/// - Call methods like `copy_buffer_to_buffer`, `begin_render_pass`,
///   etc. on a "recording" `CommandEncoder` to add commands to the
///   list. (If an error occurs, you must call `discard_encoding`; see
///   below.)
///
/// - Call `end_encoding` on a recording `CommandEncoder` to close the
///   encoder and construct a fresh `CommandBuffer` consisting of the
///   list of commands recorded up to that point.
///
/// - Call `discard_encoding` on a recording `CommandEncoder` to drop
///   the commands recorded thus far and close the encoder. This is
///   the only safe thing to do on a `CommandEncoder` if an error has
///   occurred while recording commands.
///
/// - Call `reset_all` on a closed `CommandEncoder`, passing all the
///   live `CommandBuffers` built from it. All the `CommandBuffer`s
///   are destroyed, and their resources are freed.
///
/// # Safety
///
/// - The `CommandEncoder` must be in the states described above to
///   make the given calls.
///
/// - A `CommandBuffer` that has been submitted for execution on the
///   GPU must live until its execution is complete.
///
/// - A `CommandBuffer` must not outlive the `CommandEncoder` that
///   built it.
///
/// It is the user's responsibility to meet this requirements. This
/// allows `CommandEncoder` implementations to keep their state
/// tracking to a minimum.
pub trait CommandEncoder: WasmNotSendSync + fmt::Debug {
    type A: Api;

    /// Begin encoding a new command buffer.
    ///
    /// This puts this `CommandEncoder` in the "recording" state.
    ///
    /// # Safety
    ///
    /// This `CommandEncoder` must be in the "closed" state.
    unsafe fn begin_encoding(&mut self, label: Label) -> Result<(), DeviceError>;

    /// Discard the command list under construction.
    ///
    /// If an error has occurred while recording commands, this
    /// is the only safe thing to do with the encoder.
    ///
    /// This puts this `CommandEncoder` in the "closed" state.
    ///
    /// # Safety
    ///
    /// This `CommandEncoder` must be in the "recording" state.
    ///
    /// Callers must not assume that implementations of this
    /// function are idempotent, and thus should not call it
    /// multiple times in a row.
    unsafe fn discard_encoding(&mut self);

    /// Return a fresh [`CommandBuffer`] holding the recorded commands.
    ///
    /// The returned [`CommandBuffer`] holds all the commands recorded
    /// on this `CommandEncoder` since the last call to
    /// [`begin_encoding`].
    ///
    /// This puts this `CommandEncoder` in the "closed" state.
    ///
    /// # Safety
    ///
    /// This `CommandEncoder` must be in the "recording" state.
    ///
    /// The returned [`CommandBuffer`] must not outlive this
    /// `CommandEncoder`. Implementations are allowed to build
    /// `CommandBuffer`s that depend on storage owned by this
    /// `CommandEncoder`.
    ///
    /// [`CommandBuffer`]: Api::CommandBuffer
    /// [`begin_encoding`]: CommandEncoder::begin_encoding
    unsafe fn end_encoding(&mut self) -> Result<<Self::A as Api>::CommandBuffer, DeviceError>;

    /// Reclaim all resources belonging to this `CommandEncoder`.
    ///
    /// # Safety
    ///
    /// This `CommandEncoder` must be in the "closed" state.
    ///
    /// The `command_buffers` iterator must produce all the live
    /// [`CommandBuffer`]s built using this `CommandEncoder` --- that
    /// is, every extant `CommandBuffer` returned from `end_encoding`.
    ///
    /// [`CommandBuffer`]: Api::CommandBuffer
    unsafe fn reset_all<I>(&mut self, command_buffers: I)
    where
        I: Iterator<Item = <Self::A as Api>::CommandBuffer>;

    unsafe fn transition_buffers<'a, T>(&mut self, barriers: T)
    where
        T: Iterator<Item = BufferBarrier<'a, <Self::A as Api>::Buffer>>;

    unsafe fn transition_textures<'a, T>(&mut self, barriers: T)
    where
        T: Iterator<Item = TextureBarrier<'a, <Self::A as Api>::Texture>>;

    // copy operations

    unsafe fn clear_buffer(&mut self, buffer: &<Self::A as Api>::Buffer, range: MemoryRange);

    unsafe fn copy_buffer_to_buffer<T>(
        &mut self,
        src: &<Self::A as Api>::Buffer,
        dst: &<Self::A as Api>::Buffer,
        regions: T,
    ) where
        T: Iterator<Item = BufferCopy>;

    /// Copy from an external image to an internal texture.
    /// Works with a single array layer.
    /// Note: `dst` current usage has to be `wgt::TextureUses::COPY_DST`.
    /// Note: the copy extent is in physical size (rounded to the block size)
    #[cfg(webgl)]
    unsafe fn copy_external_image_to_texture<T>(
        &mut self,
        src: &wgt::CopyExternalImageSourceInfo,
        dst: &<Self::A as Api>::Texture,
        dst_premultiplication: bool,
        regions: T,
    ) where
        T: Iterator<Item = TextureCopy>;

    /// Copy from one texture to another.
    /// Works with a single array layer.
    /// Note: `dst` current usage has to be `wgt::TextureUses::COPY_DST`.
    /// Note: the copy extent is in physical size (rounded to the block size)
    unsafe fn copy_texture_to_texture<T>(
        &mut self,
        src: &<Self::A as Api>::Texture,
        src_usage: wgt::TextureUses,
        dst: &<Self::A as Api>::Texture,
        regions: T,
    ) where
        T: Iterator<Item = TextureCopy>;

    /// Copy from buffer to texture.
    /// Works with a single array layer.
    /// Note: `dst` current usage has to be `wgt::TextureUses::COPY_DST`.
    /// Note: the copy extent is in physical size (rounded to the block size)
    unsafe fn copy_buffer_to_texture<T>(
        &mut self,
        src: &<Self::A as Api>::Buffer,
        dst: &<Self::A as Api>::Texture,
        regions: T,
    ) where
        T: Iterator<Item = BufferTextureCopy>;

    /// Copy from texture to buffer.
    /// Works with a single array layer.
    /// Note: the copy extent is in physical size (rounded to the block size)
    unsafe fn copy_texture_to_buffer<T>(
        &mut self,
        src: &<Self::A as Api>::Texture,
        src_usage: wgt::TextureUses,
        dst: &<Self::A as Api>::Buffer,
        regions: T,
    ) where
        T: Iterator<Item = BufferTextureCopy>;

    unsafe fn copy_acceleration_structure_to_acceleration_structure(
        &mut self,
        src: &<Self::A as Api>::AccelerationStructure,
        dst: &<Self::A as Api>::AccelerationStructure,
        copy: wgt::AccelerationStructureCopy,
    );
    // pass common

    /// Sets the bind group at `index` to `group`.
    ///
    /// If this is not the first call to `set_bind_group` within the current
    /// render or compute pass:
    ///
    /// - If `layout` contains `n` bind group layouts, then any previously set
    ///   bind groups at indices `n` or higher are cleared.
    ///
    /// - If the first `m` bind group layouts of `layout` are equal to those of
    ///   the previously passed layout, but no more, then any previously set
    ///   bind groups at indices `m` or higher are cleared.
    ///
    /// It follows from the above that passing the same layout as before doesn't
    /// clear any bind groups.
    ///
    /// # Safety
    ///
    /// - This [`CommandEncoder`] must be within a render or compute pass.
    ///
    /// - `index` must be the valid index of some bind group layout in `layout`.
    ///   Call this the "relevant bind group layout".
    ///
    /// - The layout of `group` must be equal to the relevant bind group layout.
    ///
    /// - The length of `dynamic_offsets` must match the number of buffer
    ///   bindings [with dynamic offsets][hdo] in the relevant bind group
    ///   layout.
    ///
    /// - If those buffer bindings are ordered by increasing [`binding` number]
    ///   and paired with elements from `dynamic_offsets`, then each offset must
    ///   be a valid offset for the binding's corresponding buffer in `group`.
    ///
    /// [hdo]: wgt::BindingType::Buffer::has_dynamic_offset
    /// [`binding` number]: wgt::BindGroupLayoutEntry::binding
    unsafe fn set_bind_group(
        &mut self,
        layout: &<Self::A as Api>::PipelineLayout,
        index: u32,
        group: &<Self::A as Api>::BindGroup,
        dynamic_offsets: &[wgt::DynamicOffset],
    );

    /// Sets a range in immediate data.
    ///
    /// IMPORTANT: while the data is passed as words, the offset is in bytes!
    ///
    /// # Safety
    ///
    /// - `offset_bytes` must be a multiple of 4.
    /// - The range of immediates written must be valid for the pipeline layout at draw time.
    unsafe fn set_immediates(
        &mut self,
        layout: &<Self::A as Api>::PipelineLayout,
        offset_bytes: u32,
        data: &[u32],
    );

    unsafe fn insert_debug_marker(&mut self, label: &str);
    unsafe fn begin_debug_marker(&mut self, group_label: &str);
    unsafe fn end_debug_marker(&mut self);

    // queries

    /// # Safety:
    ///
    /// - If `set` is an occlusion query set, it must be the same one as used in the [`RenderPassDescriptor::occlusion_query_set`] parameter.
    unsafe fn begin_query(&mut self, set: &<Self::A as Api>::QuerySet, index: u32);
    /// # Safety:
    ///
    /// - If `set` is an occlusion query set, it must be the same one as used in the [`RenderPassDescriptor::occlusion_query_set`] parameter.
    unsafe fn end_query(&mut self, set: &<Self::A as Api>::QuerySet, index: u32);
    unsafe fn write_timestamp(&mut self, set: &<Self::A as Api>::QuerySet, index: u32);
    unsafe fn reset_queries(&mut self, set: &<Self::A as Api>::QuerySet, range: Range<u32>);
    unsafe fn copy_query_results(
        &mut self,
        set: &<Self::A as Api>::QuerySet,
        range: Range<u32>,
        buffer: &<Self::A as Api>::Buffer,
        offset: wgt::BufferAddress,
        stride: wgt::BufferSize,
    );

    // render passes

    /// Begin a new render pass, clearing all active bindings.
    ///
    /// This clears any bindings established by the following calls:
    ///
    /// - [`set_bind_group`](CommandEncoder::set_bind_group)
    /// - [`set_immediates`](CommandEncoder::set_immediates)
    /// - [`begin_query`](CommandEncoder::begin_query)
    /// - [`set_render_pipeline`](CommandEncoder::set_render_pipeline)
    /// - [`set_index_buffer`](CommandEncoder::set_index_buffer)
    /// - [`set_vertex_buffer`](CommandEncoder::set_vertex_buffer)
    ///
    /// # Safety
    ///
    /// - All prior calls to [`begin_render_pass`] on this [`CommandEncoder`] must have been followed
    ///   by a call to [`end_render_pass`].
    ///
    /// - All prior calls to [`begin_compute_pass`] on this [`CommandEncoder`] must have been followed
    ///   by a call to [`end_compute_pass`].
    ///
    /// [`begin_render_pass`]: CommandEncoder::begin_render_pass
    /// [`begin_compute_pass`]: CommandEncoder::begin_compute_pass
    /// [`end_render_pass`]: CommandEncoder::end_render_pass
    /// [`end_compute_pass`]: CommandEncoder::end_compute_pass
    unsafe fn begin_render_pass(
        &mut self,
        desc: &RenderPassDescriptor<<Self::A as Api>::QuerySet, <Self::A as Api>::TextureView>,
    ) -> Result<(), DeviceError>;

    /// End the current render pass.
    ///
    /// # Safety
    ///
    /// - There must have been a prior call to [`begin_render_pass`] on this [`CommandEncoder`]
    ///   that has not been followed by a call to [`end_render_pass`].
    ///
    /// [`begin_render_pass`]: CommandEncoder::begin_render_pass
    /// [`end_render_pass`]: CommandEncoder::end_render_pass
    unsafe fn end_render_pass(&mut self);

    unsafe fn set_render_pipeline(&mut self, pipeline: &<Self::A as Api>::RenderPipeline);

    unsafe fn set_index_buffer<'a>(
        &mut self,
        binding: BufferBinding<'a, <Self::A as Api>::Buffer>,
        format: wgt::IndexFormat,
    );
    unsafe fn set_vertex_buffer<'a>(
        &mut self,
        index: u32,
        binding: BufferBinding<'a, <Self::A as Api>::Buffer>,
    );
    unsafe fn set_viewport(&mut self, rect: &Rect<f32>, depth_range: Range<f32>);
    unsafe fn set_scissor_rect(&mut self, rect: &Rect<u32>);
    unsafe fn set_stencil_reference(&mut self, value: u32);
    unsafe fn set_blend_constants(&mut self, color: &[f32; 4]);

    unsafe fn draw(
        &mut self,
        first_vertex: u32,
        vertex_count: u32,
        first_instance: u32,
        instance_count: u32,
    );
    unsafe fn draw_indexed(
        &mut self,
        first_index: u32,
        index_count: u32,
        base_vertex: i32,
        first_instance: u32,
        instance_count: u32,
    );
    unsafe fn draw_indirect(
        &mut self,
        buffer: &<Self::A as Api>::Buffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    );
    unsafe fn draw_indexed_indirect(
        &mut self,
        buffer: &<Self::A as Api>::Buffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    );
    unsafe fn draw_indirect_count(
        &mut self,
        buffer: &<Self::A as Api>::Buffer,
        offset: wgt::BufferAddress,
        count_buffer: &<Self::A as Api>::Buffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    );
    unsafe fn draw_indexed_indirect_count(
        &mut self,
        buffer: &<Self::A as Api>::Buffer,
        offset: wgt::BufferAddress,
        count_buffer: &<Self::A as Api>::Buffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    );
    unsafe fn draw_mesh_tasks(
        &mut self,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    );
    unsafe fn draw_mesh_tasks_indirect(
        &mut self,
        buffer: &<Self::A as Api>::Buffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    );
    unsafe fn draw_mesh_tasks_indirect_count(
        &mut self,
        buffer: &<Self::A as Api>::Buffer,
        offset: wgt::BufferAddress,
        count_buffer: &<Self::A as Api>::Buffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    );

    // compute passes

    /// Begin a new compute pass, clearing all active bindings.
    ///
    /// This clears any bindings established by the following calls:
    ///
    /// - [`set_bind_group`](CommandEncoder::set_bind_group)
    /// - [`set_immediates`](CommandEncoder::set_immediates)
    /// - [`begin_query`](CommandEncoder::begin_query)
    /// - [`set_compute_pipeline`](CommandEncoder::set_compute_pipeline)
    ///
    /// # Safety
    ///
    /// - All prior calls to [`begin_render_pass`] on this [`CommandEncoder`] must have been followed
    ///   by a call to [`end_render_pass`].
    ///
    /// - All prior calls to [`begin_compute_pass`] on this [`CommandEncoder`] must have been followed
    ///   by a call to [`end_compute_pass`].
    ///
    /// [`begin_render_pass`]: CommandEncoder::begin_render_pass
    /// [`begin_compute_pass`]: CommandEncoder::begin_compute_pass
    /// [`end_render_pass`]: CommandEncoder::end_render_pass
    /// [`end_compute_pass`]: CommandEncoder::end_compute_pass
    unsafe fn begin_compute_pass(
        &mut self,
        desc: &ComputePassDescriptor<<Self::A as Api>::QuerySet>,
    );

    /// End the current compute pass.
    ///
    /// # Safety
    ///
    /// - There must have been a prior call to [`begin_compute_pass`] on this [`CommandEncoder`]
    ///   that has not been followed by a call to [`end_compute_pass`].
    ///
    /// [`begin_compute_pass`]: CommandEncoder::begin_compute_pass
    /// [`end_compute_pass`]: CommandEncoder::end_compute_pass
    unsafe fn end_compute_pass(&mut self);

    unsafe fn set_compute_pipeline(&mut self, pipeline: &<Self::A as Api>::ComputePipeline);

    unsafe fn dispatch(&mut self, count: [u32; 3]);
    unsafe fn dispatch_indirect(
        &mut self,
        buffer: &<Self::A as Api>::Buffer,
        offset: wgt::BufferAddress,
    );

    /// To get the required sizes for the buffer allocations use `get_acceleration_structure_build_sizes` per descriptor
    /// All buffers must be synchronized externally
    /// All buffer regions, which are written to may only be passed once per function call,
    /// with the exception of updates in the same descriptor.
    /// Consequences of this limitation:
    /// - scratch buffers need to be unique
    /// - a tlas can't be build in the same call with a blas it contains
    unsafe fn build_acceleration_structures<'a, T>(
        &mut self,
        descriptor_count: u32,
        descriptors: T,
    ) where
        Self::A: 'a,
        T: IntoIterator<
            Item = BuildAccelerationStructureDescriptor<
                'a,
                <Self::A as Api>::Buffer,
                <Self::A as Api>::AccelerationStructure,
            >,
        >;
    unsafe fn place_acceleration_structure_barrier(
        &mut self,
        barrier: AccelerationStructureBarrier,
    );
    // modeled off dx12, because this is able to be polyfilled in vulkan as opposed to the other way round
    unsafe fn read_acceleration_structure_compact_size(
        &mut self,
        acceleration_structure: &<Self::A as Api>::AccelerationStructure,
        buf: &<Self::A as Api>::Buffer,
    );
    unsafe fn set_acceleration_structure_dependencies(
        command_buffers: &[&<Self::A as Api>::CommandBuffer],
        dependencies: &[&<Self::A as Api>::AccelerationStructure],
    );
}

bitflags!(
    /// Pipeline layout creation flags.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineLayoutFlags: u32 {
        /// D3D12: Add support for `first_vertex` and `first_instance` builtins
        /// via immediates for direct execution.
        const FIRST_VERTEX_INSTANCE = 1 << 0;
        /// D3D12: Add support for `num_workgroups` builtins via immediates
        /// for direct execution.
        const NUM_WORK_GROUPS = 1 << 1;
        /// D3D12: Add support for the builtins that the other flags enable for
        /// indirect execution.
        const INDIRECT_BUILTIN_UPDATE = 1 << 2;
    }
);

bitflags!(
    /// Pipeline layout creation flags.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct BindGroupLayoutFlags: u32 {
        /// Allows for bind group binding arrays to be shorter than the array in the BGL.
        const PARTIALLY_BOUND = 1 << 0;
    }
);

bitflags!(
    /// Texture format capability flags.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct TextureFormatCapabilities: u32 {
        /// Format can be sampled.
        const SAMPLED = 1 << 0;
        /// Format can be sampled with a linear sampler.
        const SAMPLED_LINEAR = 1 << 1;
        /// Format can be sampled with a min/max reduction sampler.
        const SAMPLED_MINMAX = 1 << 2;

        /// Format can be used as storage with read-only access.
        const STORAGE_READ_ONLY = 1 << 3;
        /// Format can be used as storage with write-only access.
        const STORAGE_WRITE_ONLY = 1 << 4;
        /// Format can be used as storage with both read and write access.
        const STORAGE_READ_WRITE = 1 << 5;
        /// Format can be used as storage with atomics.
        const STORAGE_ATOMIC = 1 << 6;

        /// Format can be used as color and input attachment.
        const COLOR_ATTACHMENT = 1 << 7;
        /// Format can be used as color (with blending) and input attachment.
        const COLOR_ATTACHMENT_BLEND = 1 << 8;
        /// Format can be used as depth-stencil and input attachment.
        const DEPTH_STENCIL_ATTACHMENT = 1 << 9;

        /// Format can be multisampled by x2.
        const MULTISAMPLE_X2   = 1 << 10;
        /// Format can be multisampled by x4.
        const MULTISAMPLE_X4   = 1 << 11;
        /// Format can be multisampled by x8.
        const MULTISAMPLE_X8   = 1 << 12;
        /// Format can be multisampled by x16.
        const MULTISAMPLE_X16  = 1 << 13;

        /// Format can be used for render pass resolve targets.
        const MULTISAMPLE_RESOLVE = 1 << 14;

        /// Format can be copied from.
        const COPY_SRC = 1 << 15;
        /// Format can be copied to.
        const COPY_DST = 1 << 16;
    }
);

bitflags!(
    /// Texture format capability flags.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FormatAspects: u8 {
        const COLOR = 1 << 0;
        const DEPTH = 1 << 1;
        const STENCIL = 1 << 2;
        const PLANE_0 = 1 << 3;
        const PLANE_1 = 1 << 4;
        const PLANE_2 = 1 << 5;

        const DEPTH_STENCIL = Self::DEPTH.bits() | Self::STENCIL.bits();
    }
);

impl FormatAspects {
    pub fn new(format: wgt::TextureFormat, aspect: wgt::TextureAspect) -> Self {
        let aspect_mask = match aspect {
            wgt::TextureAspect::All => Self::all(),
            wgt::TextureAspect::DepthOnly => Self::DEPTH,
            wgt::TextureAspect::StencilOnly => Self::STENCIL,
            wgt::TextureAspect::Plane0 => Self::PLANE_0,
            wgt::TextureAspect::Plane1 => Self::PLANE_1,
            wgt::TextureAspect::Plane2 => Self::PLANE_2,
        };
        Self::from(format) & aspect_mask
    }

    /// Returns `true` if only one flag is set
    pub fn is_one(&self) -> bool {
        self.bits().is_power_of_two()
    }

    pub fn map(&self) -> wgt::TextureAspect {
        match *self {
            Self::COLOR => wgt::TextureAspect::All,
            Self::DEPTH => wgt::TextureAspect::DepthOnly,
            Self::STENCIL => wgt::TextureAspect::StencilOnly,
            Self::PLANE_0 => wgt::TextureAspect::Plane0,
            Self::PLANE_1 => wgt::TextureAspect::Plane1,
            Self::PLANE_2 => wgt::TextureAspect::Plane2,
            _ => unreachable!(),
        }
    }
}

impl From<wgt::TextureFormat> for FormatAspects {
    fn from(format: wgt::TextureFormat) -> Self {
        match format {
            wgt::TextureFormat::Stencil8 => Self::STENCIL,
            wgt::TextureFormat::Depth16Unorm
            | wgt::TextureFormat::Depth32Float
            | wgt::TextureFormat::Depth24Plus => Self::DEPTH,
            wgt::TextureFormat::Depth32FloatStencil8 | wgt::TextureFormat::Depth24PlusStencil8 => {
                Self::DEPTH_STENCIL
            }
            wgt::TextureFormat::NV12 | wgt::TextureFormat::P010 => Self::PLANE_0 | Self::PLANE_1,
            _ => Self::COLOR,
        }
    }
}

bitflags!(
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MemoryFlags: u32 {
        const TRANSIENT = 1 << 0;
        const PREFER_COHERENT = 1 << 1;
    }
);

bitflags!(
    /// Attachment load and store operations.
    ///
    /// There must be at least one flag from the LOAD group and one from the STORE group set.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AttachmentOps: u8 {
        /// Load the existing contents of the attachment.
        const LOAD = 1 << 0;
        /// Clear the attachment to a specified value.
        const LOAD_CLEAR = 1 << 1;
        /// The contents of the attachment are undefined.
        const LOAD_DONT_CARE = 1 << 2;
        /// Store the contents of the attachment.
        const STORE = 1 << 3;
        /// The contents of the attachment are undefined after the pass.
        const STORE_DISCARD = 1 << 4;
    }
);

#[derive(Debug)]
pub struct InstanceDescriptor<'a> {
    pub name: &'a str,
    pub flags: wgt::InstanceFlags,
    pub memory_budget_thresholds: wgt::MemoryBudgetThresholds,
    pub backend_options: wgt::BackendOptions,
    pub telemetry: Option<Telemetry>,
    /// This is a borrow because the surrounding `core::Instance` keeps the the owned display handle
    /// alive already.
    pub display: Option<DisplayHandle<'a>>,
}

#[derive(Clone, Debug)]
pub struct Alignments {
    /// The alignment of the start of the buffer used as a GPU copy source.
    pub buffer_copy_offset: wgt::BufferSize,

    /// The alignment of the row pitch of the texture data stored in a buffer that is
    /// used in a GPU copy operation.
    pub buffer_copy_pitch: wgt::BufferSize,

    /// The finest alignment of bound range checking for uniform buffers.
    ///
    /// When `wgpu_hal` restricts shader references to the [accessible
    /// region][ar] of a [`Uniform`] buffer, the size of the accessible region
    /// is the bind group binding's stated [size], rounded up to the next
    /// multiple of this value.
    ///
    /// We don't need an analogous field for storage buffer bindings, because
    /// all our backends promise to enforce the size at least to a four-byte
    /// alignment, and `wgpu_hal` requires bound range lengths to be a multiple
    /// of four anyway.
    ///
    /// [ar]: struct.BufferBinding.html#accessible-region
    /// [`Uniform`]: wgt::BufferBindingType::Uniform
    /// [size]: BufferBinding::size
    pub uniform_bounds_check_alignment: wgt::BufferSize,

    /// The size of the raw TLAS instance
    pub raw_tlas_instance_size: usize,

    /// What the scratch buffer for building an acceleration structure must be aligned to
    pub ray_tracing_scratch_buffer_alignment: u32,
}

#[derive(Clone, Debug)]
pub struct Capabilities {
    pub limits: wgt::Limits,
    pub alignments: Alignments,
    pub downlevel: wgt::DownlevelCapabilities,
    /// Supported cooperative matrix configurations.
    ///
    /// Empty if cooperative matrices are not supported.
    pub cooperative_matrix_properties: Vec<wgt::CooperativeMatrixProperties>,
}

/// An adapter with all the information needed to reason about its capabilities.
///
/// These are either made by [`Instance::enumerate_adapters`] or by backend specific
/// methods on the backend [`Instance`] or [`Adapter`].
#[derive(Debug)]
pub struct ExposedAdapter<A: Api> {
    pub adapter: A::Adapter,
    pub info: wgt::AdapterInfo,
    pub features: wgt::Features,
    pub capabilities: Capabilities,
}

/// Describes information about what a `Surface`'s presentation capabilities are.
/// Fetch this with [Adapter::surface_capabilities].
#[derive(Debug, Clone)]
pub struct SurfaceCapabilities {
    /// List of supported texture formats.
    ///
    /// Must be at least one.
    pub formats: Vec<wgt::TextureFormat>,

    /// Range for the number of queued frames.
    ///
    /// This adjusts either the swapchain frame count to value + 1 - or sets SetMaximumFrameLatency to the value given,
    /// or uses a wait-for-present in the acquire method to limit rendering such that it acts like it's a value + 1 swapchain frame set.
    ///
    /// - `maximum_frame_latency.start` must be at least 1.
    /// - `maximum_frame_latency.end` must be larger or equal to `maximum_frame_latency.start`.
    pub maximum_frame_latency: RangeInclusive<u32>,

    /// Current extent of the surface, if known.
    pub current_extent: Option<wgt::Extent3d>,

    /// Supported texture usage flags.
    ///
    /// Must have at least `wgt::TextureUses::COLOR_TARGET`
    pub usage: wgt::TextureUses,

    /// List of supported V-sync modes.
    ///
    /// Must be at least one.
    pub present_modes: Vec<wgt::PresentMode>,

    /// List of supported alpha composition modes.
    ///
    /// Must be at least one.
    pub composite_alpha_modes: Vec<wgt::CompositeAlphaMode>,
}

#[derive(Debug)]
pub struct AcquiredSurfaceTexture<A: Api> {
    pub texture: A::SurfaceTexture,
    /// The presentation configuration no longer matches
    /// the surface properties exactly, but can still be used to present
    /// to the surface successfully.
    pub suboptimal: bool,
}

/// An open connection to a device and a queue.
///
/// This can be created from [`Adapter::open`] or backend
/// specific methods on the backend's [`Instance`] or [`Adapter`].
#[derive(Debug)]
pub struct OpenDevice<A: Api> {
    pub device: A::Device,
    pub queue: A::Queue,
}

#[derive(Clone, Debug)]
pub struct BufferMapping {
    pub ptr: NonNull<u8>,
    pub is_coherent: bool,
}

#[derive(Clone, Debug)]
pub struct BufferDescriptor<'a> {
    pub label: Label<'a>,
    pub size: wgt::BufferAddress,
    pub usage: wgt::BufferUses,
    pub memory_flags: MemoryFlags,
}

#[derive(Clone, Debug)]
pub struct TextureDescriptor<'a> {
    pub label: Label<'a>,
    pub size: wgt::Extent3d,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: wgt::TextureDimension,
    pub format: wgt::TextureFormat,
    pub usage: wgt::TextureUses,
    pub memory_flags: MemoryFlags,
    /// Allows views of this texture to have a different format
    /// than the texture does.
    pub view_formats: Vec<wgt::TextureFormat>,
}

impl TextureDescriptor<'_> {
    pub fn copy_extent(&self) -> CopyExtent {
        CopyExtent::map_extent_to_copy_size(&self.size, self.dimension)
    }

    pub fn is_cube_compatible(&self) -> bool {
        self.dimension == wgt::TextureDimension::D2
            && self.size.depth_or_array_layers.is_multiple_of(6)
            && self.sample_count == 1
            && self.size.width == self.size.height
    }

    pub fn array_layer_count(&self) -> u32 {
        match self.dimension {
            wgt::TextureDimension::D1 | wgt::TextureDimension::D3 => 1,
            wgt::TextureDimension::D2 => self.size.depth_or_array_layers,
        }
    }
}

/// TextureView descriptor.
///
/// Valid usage:
///. - `format` has to be the same as `TextureDescriptor::format`
///. - `dimension` has to be compatible with `TextureDescriptor::dimension`
///. - `usage` has to be a subset of `TextureDescriptor::usage`
///. - `range` has to be a subset of parent texture
#[derive(Clone, Debug)]
pub struct TextureViewDescriptor<'a> {
    pub label: Label<'a>,
    pub format: wgt::TextureFormat,
    pub dimension: wgt::TextureViewDimension,
    pub usage: wgt::TextureUses,
    pub range: wgt::ImageSubresourceRange,
}

#[derive(Clone, Debug)]
pub struct SamplerDescriptor<'a> {
    pub label: Label<'a>,
    pub address_modes: [wgt::AddressMode; 3],
    pub mag_filter: wgt::FilterMode,
    pub min_filter: wgt::FilterMode,
    pub mipmap_filter: wgt::MipmapFilterMode,
    pub lod_clamp: Range<f32>,
    pub compare: Option<wgt::CompareFunction>,
    // Must in the range [1, 16].
    //
    // Anisotropic filtering must be supported if this is not 1.
    pub anisotropy_clamp: u16,
    pub border_color: Option<wgt::SamplerBorderColor>,
}

/// BindGroupLayout descriptor.
///
/// Valid usage:
/// - `entries` are sorted by ascending `wgt::BindGroupLayoutEntry::binding`
#[derive(Clone, Debug)]
pub struct BindGroupLayoutDescriptor<'a> {
    pub label: Label<'a>,
    pub flags: BindGroupLayoutFlags,
    pub entries: &'a [wgt::BindGroupLayoutEntry],
}

#[derive(Clone, Debug)]
pub struct PipelineLayoutDescriptor<'a, B: DynBindGroupLayout + ?Sized> {
    pub label: Label<'a>,
    pub flags: PipelineLayoutFlags,
    pub bind_group_layouts: &'a [Option<&'a B>],
    pub immediate_size: u32,
}

/// A region of a buffer made visible to shaders via a [`BindGroup`].
///
/// [`BindGroup`]: Api::BindGroup
///
/// ## Construction
///
/// The recommended way to construct a `BufferBinding` is using the `binding`
/// method on a wgpu-core `Buffer`, which will validate the binding size
/// against the buffer size. A `new_unchecked` constructor is also provided for
/// cases where direct construction is necessary.
///
/// ## Accessible region
///
/// `wgpu_hal` guarantees that shaders compiled with
/// [`ShaderModuleDescriptor::runtime_checks`] set to `true` cannot read or
/// write data via this binding outside the *accessible region* of a buffer:
///
/// - The accessible region starts at [`offset`].
///
/// - For [`Storage`] bindings, the size of the accessible region is [`size`],
///   which must be a multiple of 4.
///
/// - For [`Uniform`] bindings, the size of the accessible region is [`size`]
///   rounded up to the next multiple of
///   [`Alignments::uniform_bounds_check_alignment`].
///
/// Note that this guarantee is stricter than WGSL's requirements for
/// [out-of-bounds accesses][woob], as WGSL allows them to return values from
/// elsewhere in the buffer. But this guarantee is necessary anyway, to permit
/// `wgpu-core` to avoid clearing uninitialized regions of buffers that will
/// never be read by the application before they are overwritten. This
/// optimization consults bind group buffer binding regions to determine which
/// parts of which buffers shaders might observe. This optimization is only
/// sound if shader access is bounds-checked.
///
/// ## Zero-length bindings
///
/// Some back ends cannot tolerate zero-length regions; for example, see
/// [VUID-VkDescriptorBufferInfo-offset-00340][340] and
/// [VUID-VkDescriptorBufferInfo-range-00341][341], or the
/// documentation for GLES's [glBindBufferRange][bbr]. This documentation
/// previously stated that a `BufferBinding` must have `offset` strictly less
/// than the size of the buffer, but this restriction was not honored elsewhere
/// in the code, so has been removed. However, it remains the case that
/// some backends do not support zero-length bindings, so additional
/// logic is needed somewhere to handle this properly. See
/// [#3170](https://github.com/gfx-rs/wgpu/issues/3170).
///
/// [`offset`]: BufferBinding::offset
/// [`size`]: BufferBinding::size
/// [`Storage`]: wgt::BufferBindingType::Storage
/// [`Uniform`]: wgt::BufferBindingType::Uniform
/// [340]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#VUID-VkDescriptorBufferInfo-offset-00340
/// [341]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#VUID-VkDescriptorBufferInfo-range-00341
/// [bbr]: https://registry.khronos.org/OpenGL-Refpages/es3.0/html/glBindBufferRange.xhtml
/// [woob]: https://gpuweb.github.io/gpuweb/wgsl/#out-of-bounds-access-sec
#[derive(Debug)]
pub struct BufferBinding<'a, B: DynBuffer + ?Sized> {
    /// The buffer being bound.
    ///
    /// This is not fully `pub` to prevent direct construction of
    /// `BufferBinding`s, while still allowing public read access to the `offset`
    /// and `size` properties.
    pub(crate) buffer: &'a B,

    /// The offset at which the bound region starts.
    ///
    /// This must be less or equal to the size of the buffer.
    pub offset: wgt::BufferAddress,

    /// The size of the region bound, in bytes.
    ///
    /// If `None`, the region extends from `offset` to the end of the
    /// buffer. Given the restrictions on `offset`, this means that
    /// the size is always greater than zero.
    pub size: Option<wgt::BufferSize>,
}

// We must implement this manually because `B` is not necessarily `Clone`.
impl<B: DynBuffer + ?Sized> Clone for BufferBinding<'_, B> {
    fn clone(&self) -> Self {
        BufferBinding {
            buffer: self.buffer,
            offset: self.offset,
            size: self.size,
        }
    }
}

/// Temporary convenience trait to let us call `.get()` on `u64`s in code that
/// really wants to be using `NonZeroU64`.
/// TODO(<https://github.com/gfx-rs/wgpu/issues/3170>): remove this
pub trait ShouldBeNonZeroExt {
    fn get(&self) -> u64;
}

impl ShouldBeNonZeroExt for NonZeroU64 {
    fn get(&self) -> u64 {
        NonZeroU64::get(*self)
    }
}

impl ShouldBeNonZeroExt for u64 {
    fn get(&self) -> u64 {
        *self
    }
}

impl ShouldBeNonZeroExt for Option<NonZeroU64> {
    fn get(&self) -> u64 {
        match *self {
            Some(non_zero) => non_zero.get(),
            None => 0,
        }
    }
}

impl<'a, B: DynBuffer + ?Sized> BufferBinding<'a, B> {
    /// Construct a `BufferBinding` with the given contents.
    ///
    /// When possible, use the `binding` method on a wgpu-core `Buffer` instead
    /// of this method. `Buffer::binding` validates the size of the binding
    /// against the size of the buffer.
    ///
    /// It is more difficult to provide a validating constructor here, due to
    /// not having direct access to the size of a `DynBuffer`.
    ///
    /// SAFETY: The caller is responsible for ensuring that a binding of `size`
    /// bytes starting at `offset` is contained within the buffer.
    ///
    /// The `S` type parameter is a temporary convenience to allow callers to
    /// pass a zero size. When the zero-size binding issue is resolved, the
    /// argument should just match the type of the member.
    /// TODO(<https://github.com/gfx-rs/wgpu/issues/3170>): remove the parameter
    pub fn new_unchecked<S: Into<Option<NonZeroU64>>>(
        buffer: &'a B,
        offset: wgt::BufferAddress,
        size: S,
    ) -> Self {
        Self {
            buffer,
            offset,
            size: size.into(),
        }
    }
}

#[derive(Debug)]
pub struct TextureBinding<'a, T: DynTextureView + ?Sized> {
    pub view: &'a T,
    pub usage: wgt::TextureUses,
}

impl<'a, T: DynTextureView + ?Sized> Clone for TextureBinding<'a, T> {
    fn clone(&self) -> Self {
        TextureBinding {
            view: self.view,
            usage: self.usage,
        }
    }
}

#[derive(Debug)]
pub struct ExternalTextureBinding<'a, B: DynBuffer + ?Sized, T: DynTextureView + ?Sized> {
    pub planes: [TextureBinding<'a, T>; 3],
    pub params: BufferBinding<'a, B>,
}

impl<'a, B: DynBuffer + ?Sized, T: DynTextureView + ?Sized> Clone
    for ExternalTextureBinding<'a, B, T>
{
    fn clone(&self) -> Self {
        ExternalTextureBinding {
            planes: self.planes.clone(),
            params: self.params.clone(),
        }
    }
}

/// cbindgen:ignore
#[derive(Clone, Debug)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub resource_index: u32,
    pub count: u32,
}

/// BindGroup descriptor.
///
/// Valid usage:
///. - `entries` has to be sorted by ascending `BindGroupEntry::binding`
///. - `entries` has to have the same set of `BindGroupEntry::binding` as `layout`
///. - each entry has to be compatible with the `layout`
///. - each entry's `BindGroupEntry::resource_index` is within range
///    of the corresponding resource array, selected by the relevant
///    `BindGroupLayoutEntry`.
#[derive(Clone, Debug)]
pub struct BindGroupDescriptor<
    'a,
    Bgl: DynBindGroupLayout + ?Sized,
    B: DynBuffer + ?Sized,
    S: DynSampler + ?Sized,
    T: DynTextureView + ?Sized,
    A: DynAccelerationStructure + ?Sized,
> {
    pub label: Label<'a>,
    pub layout: &'a Bgl,
    pub buffers: &'a [BufferBinding<'a, B>],
    pub samplers: &'a [&'a S],
    pub textures: &'a [TextureBinding<'a, T>],
    pub entries: &'a [BindGroupEntry],
    pub acceleration_structures: &'a [&'a A],
    pub external_textures: &'a [ExternalTextureBinding<'a, B, T>],
}

#[derive(Clone, Debug)]
pub struct CommandEncoderDescriptor<'a, Q: DynQueue + ?Sized> {
    pub label: Label<'a>,
    pub queue: &'a Q,
}

/// Naga shader module.
#[derive(Default)]
pub struct NagaShader {
    /// Shader module IR.
    pub module: Cow<'static, naga::Module>,
    /// Analysis information of the module.
    pub info: naga::valid::ModuleInfo,
    /// Source codes for debug
    pub debug_source: Option<DebugSource>,
}

// Custom implementation avoids the need to generate Debug impl code
// for the whole Naga module and info.
impl fmt::Debug for NagaShader {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Naga shader")
    }
}

/// Shader input.
pub enum ShaderInput<'a> {
    Naga(NagaShader),
    MetalLib {
        file: &'a [u8],
        num_workgroups: (u32, u32, u32),
    },
    Msl {
        shader: &'a str,
        num_workgroups: (u32, u32, u32),
    },
    SpirV(&'a [u32]),
    Dxil {
        shader: &'a [u8],
        num_workgroups: (u32, u32, u32),
    },
    Hlsl {
        shader: &'a str,
        num_workgroups: (u32, u32, u32),
    },
    Glsl {
        shader: &'a str,
        num_workgroups: (u32, u32, u32),
    },
}

pub struct ShaderModuleDescriptor<'a> {
    pub label: Label<'a>,

    /// # Safety
    ///
    /// See the documentation for each flag in [`ShaderRuntimeChecks`][src].
    ///
    /// [src]: wgt::ShaderRuntimeChecks
    pub runtime_checks: wgt::ShaderRuntimeChecks,
}

#[derive(Debug, Clone)]
pub struct DebugSource {
    pub file_name: Cow<'static, str>,
    pub source_code: Cow<'static, str>,
}

/// Describes a programmable pipeline stage.
#[derive(Debug)]
pub struct ProgrammableStage<'a, M: DynShaderModule + ?Sized> {
    /// The compiled shader module for this stage.
    pub module: &'a M,
    /// The name of the entry point in the compiled shader. There must be a function with this name
    ///  in the shader.
    pub entry_point: &'a str,
    /// Pipeline constants
    pub constants: &'a naga::back::PipelineConstants,
    /// Whether workgroup scoped memory will be initialized with zero values for this stage.
    ///
    /// This is required by the WebGPU spec, but may have overhead which can be avoided
    /// for cross-platform applications
    pub zero_initialize_workgroup_memory: bool,
}

impl<M: DynShaderModule + ?Sized> Clone for ProgrammableStage<'_, M> {
    fn clone(&self) -> Self {
        Self {
            module: self.module,
            entry_point: self.entry_point,
            constants: self.constants,
            zero_initialize_workgroup_memory: self.zero_initialize_workgroup_memory,
        }
    }
}

/// Describes a compute pipeline.
#[derive(Clone, Debug)]
pub struct ComputePipelineDescriptor<
    'a,
    Pl: DynPipelineLayout + ?Sized,
    M: DynShaderModule + ?Sized,
    Pc: DynPipelineCache + ?Sized,
> {
    pub label: Label<'a>,
    /// The layout of bind groups for this pipeline.
    pub layout: &'a Pl,
    /// The compiled compute stage and its entry point.
    pub stage: ProgrammableStage<'a, M>,
    /// The cache which will be used and filled when compiling this pipeline
    pub cache: Option<&'a Pc>,
}

pub struct PipelineCacheDescriptor<'a> {
    pub label: Label<'a>,
    pub data: Option<&'a [u8]>,
}

/// Describes how the vertex buffer is interpreted.
#[derive(Clone, Debug)]
pub struct VertexBufferLayout<'a> {
    /// The stride, in bytes, between elements of this buffer.
    pub array_stride: wgt::BufferAddress,
    /// How often this vertex buffer is "stepped" forward.
    pub step_mode: wgt::VertexStepMode,
    /// The list of attributes which comprise a single vertex.
    pub attributes: &'a [wgt::VertexAttribute],
}

#[derive(Clone, Debug)]
pub enum VertexProcessor<'a, M: DynShaderModule + ?Sized> {
    Standard {
        /// The format of any vertex buffers used with this pipeline.
        vertex_buffers: &'a [VertexBufferLayout<'a>],
        /// The vertex stage for this pipeline.
        vertex_stage: ProgrammableStage<'a, M>,
    },
    Mesh {
        task_stage: Option<ProgrammableStage<'a, M>>,
        mesh_stage: ProgrammableStage<'a, M>,
    },
}

/// Describes a render (graphics) pipeline.
#[derive(Clone, Debug)]
pub struct RenderPipelineDescriptor<
    'a,
    Pl: DynPipelineLayout + ?Sized,
    M: DynShaderModule + ?Sized,
    Pc: DynPipelineCache + ?Sized,
> {
    pub label: Label<'a>,
    /// The layout of bind groups for this pipeline.
    pub layout: &'a Pl,
    /// The vertex processing state(vertex shader + buffers or task + mesh shaders)
    pub vertex_processor: VertexProcessor<'a, M>,
    /// The properties of the pipeline at the primitive assembly and rasterization level.
    pub primitive: wgt::PrimitiveState,
    /// The effect of draw calls on the depth and stencil aspects of the output target, if any.
    pub depth_stencil: Option<wgt::DepthStencilState>,
    /// The multi-sampling properties of the pipeline.
    pub multisample: wgt::MultisampleState,
    /// The fragment stage for this pipeline.
    pub fragment_stage: Option<ProgrammableStage<'a, M>>,
    /// The effect of draw calls on the color aspect of the output target.
    pub color_targets: &'a [Option<wgt::ColorTargetState>],
    /// If the pipeline will be used with a multiview render pass, this indicates how many array
    /// layers the attachments will have.
    pub multiview_mask: Option<NonZeroU32>,
    /// The cache which will be used and filled when compiling this pipeline
    pub cache: Option<&'a Pc>,
}

#[derive(Debug, Clone)]
pub struct SurfaceConfiguration {
    /// Maximum number of queued frames. Must be in
    /// `SurfaceCapabilities::maximum_frame_latency` range.
    pub maximum_frame_latency: u32,
    /// Vertical synchronization mode.
    pub present_mode: wgt::PresentMode,
    /// Alpha composition mode.
    pub composite_alpha_mode: wgt::CompositeAlphaMode,
    /// Format of the surface textures.
    pub format: wgt::TextureFormat,
    /// Requested texture extent. Must be in
    /// `SurfaceCapabilities::extents` range.
    pub extent: wgt::Extent3d,
    /// Allowed usage of surface textures,
    pub usage: wgt::TextureUses,
    /// Allows views of swapchain texture to have a different format
    /// than the texture does.
    pub view_formats: Vec<wgt::TextureFormat>,
}

#[derive(Debug, Clone)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateTransition<T> {
    pub from: T,
    pub to: T,
}

#[derive(Debug, Clone)]
pub struct BufferBarrier<'a, B: DynBuffer + ?Sized> {
    pub buffer: &'a B,
    pub usage: StateTransition<wgt::BufferUses>,
}

#[derive(Debug, Clone)]
pub struct TextureBarrier<'a, T: DynTexture + ?Sized> {
    pub texture: &'a T,
    pub range: wgt::ImageSubresourceRange,
    pub usage: StateTransition<wgt::TextureUses>,
}

#[derive(Clone, Copy, Debug)]
pub struct BufferCopy {
    pub src_offset: wgt::BufferAddress,
    pub dst_offset: wgt::BufferAddress,
    pub size: wgt::BufferSize,
}

#[derive(Clone, Debug)]
pub struct TextureCopyBase {
    pub mip_level: u32,
    pub array_layer: u32,
    /// Origin within a texture.
    /// Note: for 1D and 2D textures, Z must be 0.
    pub origin: wgt::Origin3d,
    pub aspect: FormatAspects,
}

#[derive(Clone, Copy, Debug)]
pub struct CopyExtent {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl From<wgt::Extent3d> for CopyExtent {
    fn from(value: wgt::Extent3d) -> Self {
        let wgt::Extent3d {
            width,
            height,
            depth_or_array_layers,
        } = value;
        Self {
            width,
            height,
            depth: depth_or_array_layers,
        }
    }
}

impl From<CopyExtent> for wgt::Extent3d {
    fn from(value: CopyExtent) -> Self {
        let CopyExtent {
            width,
            height,
            depth,
        } = value;
        Self {
            width,
            height,
            depth_or_array_layers: depth,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextureCopy {
    pub src_base: TextureCopyBase,
    pub dst_base: TextureCopyBase,
    pub size: CopyExtent,
}

#[derive(Clone, Debug)]
pub struct BufferTextureCopy {
    pub buffer_layout: wgt::TexelCopyBufferLayout,
    pub texture_base: TextureCopyBase,
    pub size: CopyExtent,
}

#[derive(Clone, Debug)]
pub struct Attachment<'a, T: DynTextureView + ?Sized> {
    pub view: &'a T,
    /// Contains either a single mutating usage as a target,
    /// or a valid combination of read-only usages.
    pub usage: wgt::TextureUses,
}

#[derive(Clone, Debug)]
pub struct ColorAttachment<'a, T: DynTextureView + ?Sized> {
    pub target: Attachment<'a, T>,
    pub depth_slice: Option<u32>,
    pub resolve_target: Option<Attachment<'a, T>>,
    pub ops: AttachmentOps,
    pub clear_value: wgt::Color,
}

#[derive(Clone, Debug)]
pub struct DepthStencilAttachment<'a, T: DynTextureView + ?Sized> {
    pub target: Attachment<'a, T>,
    pub depth_ops: AttachmentOps,
    pub stencil_ops: AttachmentOps,
    pub clear_value: (f32, u32),
}

#[derive(Clone, Debug)]
pub struct PassTimestampWrites<'a, Q: DynQuerySet + ?Sized> {
    pub query_set: &'a Q,
    pub beginning_of_pass_write_index: Option<u32>,
    pub end_of_pass_write_index: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct RenderPassDescriptor<'a, Q: DynQuerySet + ?Sized, T: DynTextureView + ?Sized> {
    pub label: Label<'a>,
    pub extent: wgt::Extent3d,
    pub sample_count: u32,
    pub color_attachments: &'a [Option<ColorAttachment<'a, T>>],
    pub depth_stencil_attachment: Option<DepthStencilAttachment<'a, T>>,
    pub multiview_mask: Option<NonZeroU32>,
    pub timestamp_writes: Option<PassTimestampWrites<'a, Q>>,
    pub occlusion_query_set: Option<&'a Q>,
}

#[derive(Clone, Debug)]
pub struct ComputePassDescriptor<'a, Q: DynQuerySet + ?Sized> {
    pub label: Label<'a>,
    pub timestamp_writes: Option<PassTimestampWrites<'a, Q>>,
}

#[test]
fn test_default_limits() {
    let limits = wgt::Limits::default();
    assert!(limits.max_bind_groups <= MAX_BIND_GROUPS as u32);
}

#[derive(Clone, Debug)]
pub struct AccelerationStructureDescriptor<'a> {
    pub label: Label<'a>,
    pub size: wgt::BufferAddress,
    pub format: AccelerationStructureFormat,
    pub allow_compaction: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AccelerationStructureFormat {
    TopLevel,
    BottomLevel,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AccelerationStructureBuildMode {
    Build,
    Update,
}

/// Information of the required size for a corresponding entries struct (+ flags)
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct AccelerationStructureBuildSizes {
    pub acceleration_structure_size: wgt::BufferAddress,
    pub update_scratch_size: wgt::BufferAddress,
    pub build_scratch_size: wgt::BufferAddress,
}

/// Updates use source_acceleration_structure if present, else the update will be performed in place.
/// For updates, only the data is allowed to change (not the meta data or sizes).
#[derive(Clone, Debug)]
pub struct BuildAccelerationStructureDescriptor<
    'a,
    B: DynBuffer + ?Sized,
    A: DynAccelerationStructure + ?Sized,
> {
    pub entries: &'a AccelerationStructureEntries<'a, B>,
    pub mode: AccelerationStructureBuildMode,
    pub flags: AccelerationStructureBuildFlags,
    pub source_acceleration_structure: Option<&'a A>,
    pub destination_acceleration_structure: &'a A,
    pub scratch_buffer: &'a B,
    pub scratch_buffer_offset: wgt::BufferAddress,
}

/// - All buffers, buffer addresses and offsets will be ignored.
/// - The build mode will be ignored.
/// - Reducing the amount of Instances, Triangle groups or AABB groups (or the number of Triangles/AABBs in corresponding groups),
///   may result in reduced size requirements.
/// - Any other change may result in a bigger or smaller size requirement.
#[derive(Clone, Debug)]
pub struct GetAccelerationStructureBuildSizesDescriptor<'a, B: DynBuffer + ?Sized> {
    pub entries: &'a AccelerationStructureEntries<'a, B>,
    pub flags: AccelerationStructureBuildFlags,
}

/// Entries for a single descriptor
/// * `Instances` - Multiple instances for a top level acceleration structure
/// * `Triangles` - Multiple triangle meshes for a bottom level acceleration structure
/// * `AABBs` - List of list of axis aligned bounding boxes for a bottom level acceleration structure
#[derive(Debug)]
pub enum AccelerationStructureEntries<'a, B: DynBuffer + ?Sized> {
    Instances(AccelerationStructureInstances<'a, B>),
    Triangles(Vec<AccelerationStructureTriangles<'a, B>>),
    AABBs(Vec<AccelerationStructureAABBs<'a, B>>),
}

/// * `first_vertex` - offset in the vertex buffer (as number of vertices)
/// * `indices` - optional index buffer with attributes
/// * `transform` - optional transform
#[derive(Clone, Debug)]
pub struct AccelerationStructureTriangles<'a, B: DynBuffer + ?Sized> {
    pub vertex_buffer: Option<&'a B>,
    pub vertex_format: wgt::VertexFormat,
    pub first_vertex: u32,
    pub vertex_count: u32,
    pub vertex_stride: wgt::BufferAddress,
    pub indices: Option<AccelerationStructureTriangleIndices<'a, B>>,
    pub transform: Option<AccelerationStructureTriangleTransform<'a, B>>,
    pub flags: AccelerationStructureGeometryFlags,
}

/// * `offset` - offset in bytes
#[derive(Clone, Debug)]
pub struct AccelerationStructureAABBs<'a, B: DynBuffer + ?Sized> {
    pub buffer: Option<&'a B>,
    pub offset: u32,
    pub count: u32,
    pub stride: wgt::BufferAddress,
    pub flags: AccelerationStructureGeometryFlags,
}

pub struct AccelerationStructureCopy {
    pub copy_flags: wgt::AccelerationStructureCopy,
    pub type_flags: wgt::AccelerationStructureType,
}

/// * `offset` - offset in bytes
#[derive(Clone, Debug)]
pub struct AccelerationStructureInstances<'a, B: DynBuffer + ?Sized> {
    pub buffer: Option<&'a B>,
    pub offset: u32,
    pub count: u32,
}

/// * `offset` - offset in bytes
#[derive(Clone, Debug)]
pub struct AccelerationStructureTriangleIndices<'a, B: DynBuffer + ?Sized> {
    pub format: wgt::IndexFormat,
    pub buffer: Option<&'a B>,
    pub offset: u32,
    pub count: u32,
}

/// * `offset` - offset in bytes
#[derive(Clone, Debug)]
pub struct AccelerationStructureTriangleTransform<'a, B: DynBuffer + ?Sized> {
    pub buffer: &'a B,
    pub offset: u32,
}

pub use wgt::AccelerationStructureFlags as AccelerationStructureBuildFlags;
pub use wgt::AccelerationStructureGeometryFlags;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AccelerationStructureUses: u8 {
        // For blas used as input for tlas
        const BUILD_INPUT = 1 << 0;
        // Target for acceleration structure build
        const BUILD_OUTPUT = 1 << 1;
        // Tlas used in a shader
        const SHADER_INPUT = 1 << 2;
        // Blas used to query compacted size
        const QUERY_INPUT = 1 << 3;
        // BLAS used as a src for a copy operation
        const COPY_SRC = 1 << 4;
        // BLAS used as a dst for a copy operation
        const COPY_DST = 1 << 5;
    }
}

#[derive(Debug, Clone)]
pub struct AccelerationStructureBarrier {
    pub usage: StateTransition<AccelerationStructureUses>,
}

#[derive(Debug, Copy, Clone)]
pub struct TlasInstance {
    pub transform: [f32; 12],
    pub custom_data: u32,
    pub mask: u8,
    pub blas_address: u64,
}

#[cfg(dx12)]
pub enum D3D12ExposeAdapterResult {
    CreateDeviceError(dx12::CreateDeviceError),
    UnknownFeatureLevel(i32),
    ResourceBindingTier2Requirement,
    ShaderModel6Requirement,
    Success(dx12::FeatureLevel, dx12::ShaderModel),
}

/// Pluggable telemetry, mainly to be used by Firefox.
#[derive(Debug, Clone, Copy)]
pub struct Telemetry {
    #[cfg(dx12)]
    pub d3d12_expose_adapter: fn(
        desc: &windows::Win32::Graphics::Dxgi::DXGI_ADAPTER_DESC2,
        driver_version: Result<[u16; 4], windows_core::HRESULT>,
        result: D3D12ExposeAdapterResult,
    ),
}
