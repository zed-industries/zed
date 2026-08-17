//! Utility structures and functions that are built on top of the main `wgpu` API.
//!
//! Nothing in this module is a part of the WebGPU API specification;
//! they are unique to the `wgpu` library.

// TODO: For [`belt::StagingBelt`] to be available in `no_std` its usage of [`std::sync::mpsc`]
// must be replaced with an appropriate alternative.
#[cfg(std)]
mod belt;
mod device;
mod encoder;
mod init;
mod mutex;
mod panicking;
mod spirv;
mod texture_blitter;

use alloc::{format, string::String};

#[cfg(std)]
pub use belt::StagingBelt;
pub use device::{BufferInitDescriptor, DeviceExt};
pub use encoder::RenderEncoder;
pub use init::*;
pub use spirv::*;
#[cfg(feature = "wgsl")]
pub use texture_blitter::{TextureBlitter, TextureBlitterBuilder};
pub use wgt::{
    math::*, DispatchIndirectArgs, DrawIndexedIndirectArgs, DrawIndirectArgs, TextureDataOrder,
};

pub(crate) use mutex::Mutex;
pub(crate) use panicking::is_panicking;

use crate::dispatch;

/// CPU accessible buffer used to download data back from the GPU.
pub struct DownloadBuffer {
    _gpu_buffer: super::Buffer,
    mapped_range: dispatch::DispatchBufferMappedRange,
}

impl DownloadBuffer {
    /// Asynchronously read the contents of a buffer.
    pub fn read_buffer(
        device: &super::Device,
        queue: &super::Queue,
        buffer: &super::BufferSlice<'_>,
        callback: impl FnOnce(Result<Self, super::BufferAsyncError>) + Send + 'static,
    ) {
        let size = buffer.size.into();

        let download = device.create_buffer(&super::BufferDescriptor {
            size,
            usage: super::BufferUsages::COPY_DST | super::BufferUsages::MAP_READ,
            mapped_at_creation: false,
            label: None,
        });

        let mut encoder =
            device.create_command_encoder(&super::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(buffer.buffer, buffer.offset, &download, 0, size);
        let command_buffer: super::CommandBuffer = encoder.finish();
        queue.submit(Some(command_buffer));

        download
            .clone()
            .slice(..)
            .map_async(super::MapMode::Read, move |result| {
                if let Err(e) = result {
                    callback(Err(e));
                    return;
                }

                let mapped_range = download.inner.get_mapped_range(0..size);
                callback(Ok(Self {
                    _gpu_buffer: download,
                    mapped_range,
                }));
            });
    }
}

impl core::ops::Deref for DownloadBuffer {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        // SAFETY: `self.mapped_range` is always a read mapping
        unsafe { self.mapped_range.read_slice() }
    }
}

/// A recommended key for storing [`PipelineCache`]s for the adapter
/// associated with the given [`AdapterInfo`](wgt::AdapterInfo)
/// This key will define a class of adapters for which the same cache
/// might be valid.
///
/// If this returns `None`, the adapter doesn't support [`PipelineCache`].
/// This may be because the API doesn't support application managed caches
/// (such as browser WebGPU), or that `wgpu` hasn't implemented it for
/// that API yet.
///
/// This key could be used as a filename, as seen in the example below.
///
/// # Examples
///
/// ```no_run
/// # use std::path::PathBuf;
/// use wgpu::PipelineCacheDescriptor;
/// # let adapter_info = todo!();
/// # let device: wgpu::Device = todo!();
/// let cache_dir: PathBuf = unimplemented!("Some reasonable platform-specific cache directory for your app.");
/// let filename = wgpu::util::pipeline_cache_key(&adapter_info);
/// let (pipeline_cache, cache_file) = if let Some(filename) = filename {
///     let cache_path = cache_dir.join(&filename);
///     // If we failed to read the cache, for whatever reason, treat the data as lost.
///     // In a real app, we'd probably avoid caching entirely unless the error was "file not found".
///     let cache_data = std::fs::read(&cache_path).ok();
///     let pipeline_cache = unsafe {
///         device.create_pipeline_cache(&PipelineCacheDescriptor {
///             data: cache_data.as_deref(),
///             label: None,
///             fallback: true
///         })
///     };
///     (Some(pipeline_cache), Some(cache_path))
/// } else {
///     (None, None)
/// };
///
/// // Run pipeline initialisation, making sure to set the `cache`
/// // fields of your `*PipelineDescriptor` to `pipeline_cache`
///
/// // And then save the resulting cache (probably off the main thread).
/// if let (Some(pipeline_cache), Some(cache_file)) = (pipeline_cache, cache_file) {
///     let data = pipeline_cache.get_data();
///     if let Some(data) = data {
///         let temp_file = cache_file.with_extension("temp");
///         std::fs::write(&temp_file, &data)?;
///         std::fs::rename(&temp_file, &cache_file)?;
///     }
/// }
/// # Ok::<_, std::io::Error>(())
/// ```
///
/// [`PipelineCache`]: super::PipelineCache
pub fn pipeline_cache_key(adapter_info: &wgt::AdapterInfo) -> Option<String> {
    match adapter_info.backend {
        wgt::Backend::Vulkan => Some(format!(
            // The vendor/device should uniquely define a driver
            // We/the driver will also later validate that the vendor/device and driver
            // version match, which may lead to clearing an outdated
            // cache for the same device.
            "wgpu_pipeline_cache_vulkan_{}_{}",
            adapter_info.vendor, adapter_info.device
        )),
        _ => None,
    }
}

/// Adds extra conversion functions to `TextureFormat`.
pub trait TextureFormatExt {
    /// Finds the [`TextureFormat`](wgt::TextureFormat) corresponding to the given
    /// [`StorageFormat`](wgc::naga::StorageFormat).
    ///
    /// # Examples
    /// ```
    /// use wgpu::util::TextureFormatExt;
    /// assert_eq!(wgpu::TextureFormat::from_storage_format(wgpu::naga::StorageFormat::Bgra8Unorm), wgpu::TextureFormat::Bgra8Unorm);
    /// ```
    #[cfg(wgpu_core)]
    fn from_storage_format(storage_format: crate::naga::StorageFormat) -> Self;

    /// Finds the [`StorageFormat`](wgc::naga::StorageFormat) corresponding to the given [`TextureFormat`](wgt::TextureFormat).
    /// Returns `None` if there is no matching storage format,
    /// which typically indicates this format is not supported
    /// for storage textures.
    ///
    /// # Examples
    /// ```
    /// use wgpu::util::TextureFormatExt;
    /// assert_eq!(wgpu::TextureFormat::Bgra8Unorm.to_storage_format(), Some(wgpu::naga::StorageFormat::Bgra8Unorm));
    /// ```
    #[cfg(wgpu_core)]
    fn to_storage_format(&self) -> Option<crate::naga::StorageFormat>;
}

impl TextureFormatExt for wgt::TextureFormat {
    #[cfg(wgpu_core)]
    fn from_storage_format(storage_format: crate::naga::StorageFormat) -> Self {
        wgc::map_storage_format_from_naga(storage_format)
    }

    #[cfg(wgpu_core)]
    fn to_storage_format(&self) -> Option<crate::naga::StorageFormat> {
        wgc::map_storage_format_to_naga(*self)
    }
}
