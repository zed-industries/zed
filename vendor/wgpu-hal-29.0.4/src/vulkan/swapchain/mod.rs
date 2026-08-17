use alloc::boxed::Box;
use core::{any::Any, fmt::Debug, time::Duration};

use crate::vulkan::{semaphore_list::SemaphoreType, DeviceShared};

pub(super) use native::*;

mod native;

pub(super) trait Surface: Send + Sync + 'static {
    /// Deletes the surface and associated resources.
    ///
    /// The surface must not be in use when it is deleted.
    unsafe fn delete_surface(self: Box<Self>);

    /// Returns the surface capabilities for the given adapter.
    ///
    /// Returns `None` if the surface is not compatible with the adapter.
    fn surface_capabilities(&self, adapter: &super::Adapter) -> Option<crate::SurfaceCapabilities>;

    /// Creates a swapchain for the surface with the given configuration.
    ///
    /// If this is not the first swapchain created for the surface, the old swapchain
    /// must be provided. [`Swapchain::release_resources`] must be called on the old swapchain
    /// before calling this method.
    unsafe fn create_swapchain(
        &self,
        device: &super::Device,
        config: &crate::SurfaceConfiguration,
        provided_old_swapchain: Option<Box<dyn Swapchain>>,
    ) -> Result<Box<dyn Swapchain>, crate::SurfaceError>;

    /// Allows downcasting to the concrete type.
    fn as_any(&self) -> &dyn Any;
}

pub(super) trait Swapchain: Send + Sync + 'static {
    /// Releases all resources associated with the swapchain, without
    /// destroying the swapchain itself. Must be called before calling
    /// either [`Surface::create_swapchain`] or [`Swapchain::delete_swapchain`].
    ///
    /// The swapchain must not be in use when this is called.
    unsafe fn release_resources(&mut self, device: &super::Device);

    /// Deletes the swapchain.
    ///
    /// The swapchain must not be in use when it is deleted and
    /// [`Swapchain::release_resources`] must have been called first.
    unsafe fn delete_swapchain(self: Box<Self>);

    /// Acquires the next available surface texture for rendering.
    ///
    /// `timeout` specifies the maximum time to wait for an image to become available.
    /// If `None` is specified, this function will wait indefinitely.
    ///
    /// Returns `Err(SurfaceError::Timeout)` if the timeout elapsed before an image became available.
    unsafe fn acquire(
        &mut self,
        timeout: Option<Duration>,
        fence: &super::Fence,
    ) -> Result<crate::AcquiredSurfaceTexture<crate::api::Vulkan>, crate::SurfaceError>;

    /// Tries to discard the acquired texture without presenting it.
    ///
    /// In practice, this doesn't really work in the current implementations.
    unsafe fn discard_texture(
        &mut self,
        texture: super::SurfaceTexture,
    ) -> Result<(), crate::SurfaceError>;

    /// Presents the given surface texture using the queue.
    unsafe fn present(
        &mut self,
        queue: &super::Queue,
        texture: crate::vulkan::SurfaceTexture,
    ) -> Result<(), crate::SurfaceError>;

    /// Allows downcasting to the concrete type.
    fn as_any(&self) -> &dyn Any;

    /// Allows downcasting to the concrete type mutably.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Swapchain specific metadata associated with a surface texture.
pub(super) trait SurfaceTextureMetadata: Debug + Send + Sync + 'static {
    /// Returns a guard which can yield the semaphores needed for submission using this swapchain texture.
    fn get_semaphore_guard(&self) -> Box<dyn SwapchainSubmissionSemaphoreGuard + '_>;

    /// Allows downcasting to the concrete type.
    fn as_any(&self) -> &dyn Any;
}

/// Guard type for managing swapchain submission semaphores.
pub(super) trait SwapchainSubmissionSemaphoreGuard {
    /// Sets the Fence value for this submission.
    fn set_used_fence_value(&mut self, value: u64);

    /// Gets semaphores to wait on before doing GPU work for this swapchain texture.
    fn get_acquire_wait_semaphore(&mut self) -> Option<SemaphoreType>;

    /// Gets the semaphore to signal when GPU work for this swapchain texture is complete.
    fn get_submit_signal_semaphore(
        &mut self,
        device: &DeviceShared,
    ) -> Result<SemaphoreType, crate::DeviceError>;
}
