use alloc::vec::Vec;

use crate::*;

/// Handle to a pipeline cache, which is used to accelerate
/// creating [`RenderPipeline`]s and [`ComputePipeline`]s
/// in subsequent executions
///
/// This reuse is only applicable for the same or similar devices.
/// See [`util::pipeline_cache_key`] for some details and a suggested workflow.
///
/// Created using [`Device::create_pipeline_cache`].
///
/// # Background
///
/// In most GPU drivers, shader code must be converted into a machine code
/// which can be executed on the GPU.
/// Generating this machine code can require a lot of computation.
/// Pipeline caches allow this computation to be reused between executions
/// of the program.
/// This can be very useful for reducing program startup time.
///
/// Note that most desktop GPU drivers will manage their own caches,
/// meaning that little advantage can be gained from this on those platforms.
/// However, on some platforms, especially Android, drivers leave this to the
/// application to implement.
///
/// Unfortunately, drivers do not expose whether they manage their own caches.
/// Some reasonable policies for applications to use are:
/// - Manage their own pipeline cache on all platforms
/// - Only manage pipeline caches on Android
///
/// # Usage
///
/// This is used as [`RenderPipelineDescriptor::cache`] or [`ComputePipelineDescriptor::cache`].
/// It is valid to use this resource when creating multiple pipelines, in
/// which case it will likely cache each of those pipelines.
/// It is also valid to create a new cache for each pipeline.
///
/// This resource is most useful when the data produced from it (using
/// [`PipelineCache::get_data`]) is persisted.
/// Care should be taken that pipeline caches are only used for the same device,
/// as pipeline caches from compatible devices are unlikely to provide any advantage.
/// `util::pipeline_cache_key` can be used as a file/directory name to help ensure that.
///
/// It is recommended to store pipeline caches atomically. If persisting to disk,
/// this can usually be achieved by creating a temporary file, then moving/[renaming]
/// the temporary file over the existing cache
///
/// # Storage Usage
///
/// There is not currently an API available to reduce the size of a cache.
/// This is due to limitations in the underlying graphics APIs used.
/// This is especially impactful if your application is being updated, so
/// previous caches are no longer being used.
///
/// One option to work around this is to regenerate the cache.
/// That is, creating the pipelines which your program runs using
/// with the stored cached data, then recreating the *same* pipelines
/// using a new cache, which your application then store.
///
/// # Implementations
///
/// This resource currently only works on the following backends:
///  - Vulkan
///
/// This type is unique to the Rust API of `wgpu`.
///
/// [renaming]: std::fs::rename
#[derive(Debug, Clone)]
pub struct PipelineCache {
    pub(crate) inner: crate::dispatch::DispatchPipelineCache,
}

#[cfg(send_sync)]
static_assertions::assert_impl_all!(PipelineCache: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(PipelineCache => .inner);

impl PipelineCache {
    /// Get the data associated with this pipeline cache.
    /// The data format is an implementation detail of `wgpu`.
    /// The only defined operation on this data setting it as the `data` field
    /// on [`PipelineCacheDescriptor`], then to [`Device::create_pipeline_cache`].
    ///
    /// This function is unique to the Rust API of `wgpu`.
    pub fn get_data(&self) -> Option<Vec<u8>> {
        self.inner.get_data()
    }

    #[cfg(custom)]
    /// Returns custom implementation of PipelineCache (if custom backend and is internally T)
    pub fn as_custom<T: custom::PipelineCacheInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}
