use alloc::vec::Vec;
use core::future::Future;
#[cfg(wgpu_core)]
use core::ops::Deref;

use crate::*;

/// Handle to a physical graphics and/or compute device.
///
/// Adapters can be created using [`Instance::request_adapter`]
/// or other [`Instance`] methods.
///
/// Adapters can be used to open a connection to the corresponding [`Device`]
/// on the host system by using [`Adapter::request_device`].
///
/// Does not have to be kept alive.
///
/// Corresponds to [WebGPU `GPUAdapter`](https://gpuweb.github.io/gpuweb/#gpu-adapter).
#[derive(Debug, Clone)]
pub struct Adapter {
    pub(crate) inner: dispatch::DispatchAdapter,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(Adapter: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(Adapter => .inner);

pub use wgt::RequestAdapterOptions as RequestAdapterOptionsBase;
/// Additional information required when requesting an adapter.
///
/// For use with [`Instance::request_adapter`].
///
/// Corresponds to [WebGPU `GPURequestAdapterOptions`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpurequestadapteroptions).
pub type RequestAdapterOptions<'a, 'b> = RequestAdapterOptionsBase<&'a Surface<'b>>;
#[cfg(send_sync)]
static_assertions::assert_impl_all!(RequestAdapterOptions<'_, '_>: Send, Sync);

impl Adapter {
    /// Requests a connection to a physical device, creating a logical device.
    ///
    /// Returns the [`Device`] together with a [`Queue`] that executes command buffers.
    ///
    /// [Per the WebGPU specification], an [`Adapter`] may only be used once to create a device.
    /// If another device is wanted, call [`Instance::request_adapter()`] again to get a fresh
    /// [`Adapter`].
    /// However, `wgpu` does not currently enforce this restriction.
    ///
    /// # Panics
    ///
    /// - `request_device()` was already called on this `Adapter`.
    /// - Features specified by `desc` are not supported by this adapter.
    /// - Unsafe features were requested but not enabled when requesting the adapter.
    /// - Limits requested exceed the values provided by the adapter.
    /// - Adapter does not support all features wgpu requires to safely operate.
    ///
    /// [Per the WebGPU specification]: https://www.w3.org/TR/webgpu/#dom-gpuadapter-requestdevice
    pub fn request_device(
        &self,
        desc: &DeviceDescriptor<'_>,
    ) -> impl Future<Output = Result<(Device, Queue), RequestDeviceError>> + WasmNotSend {
        let device = self.inner.request_device(desc);
        async move {
            device
                .await
                .map(|(device, queue)| (Device { inner: device }, Queue { inner: queue }))
        }
    }

    /// Create a wgpu [`Device`] and [`Queue`] from a wgpu-hal [`hal::OpenDevice`].
    ///
    /// # Safety
    ///
    /// - `hal_device` must be created from this adapter internal handle.
    /// - `desc.features` must be a subset of `hal_device`'s supported features.
    #[cfg(wgpu_core)]
    pub unsafe fn create_device_from_hal<A: hal::Api>(
        &self,
        hal_device: hal::OpenDevice<A>,
        desc: &DeviceDescriptor<'_>,
    ) -> Result<(Device, Queue), RequestDeviceError> {
        let core_adapter = self.inner.as_core();
        let (device, queue) = unsafe {
            core_adapter
                .context
                .create_device_from_hal(core_adapter, hal_device, desc)
        }?;

        Ok((
            Device {
                inner: device.into(),
            },
            Queue {
                inner: queue.into(),
            },
        ))
    }

    /// Get the [`wgpu_hal`] adapter from this `Adapter`.
    ///
    /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
    /// and pass that struct to the to the `A` type parameter.
    ///
    /// Returns a guard that dereferences to the type of the hal backend
    /// which implements [`A::Adapter`].
    ///
    /// # Types
    ///
    /// The returned type depends on the backend:
    ///
    #[doc = crate::macros::hal_type_vulkan!("Adapter")]
    #[doc = crate::macros::hal_type_metal!("Adapter")]
    #[doc = crate::macros::hal_type_dx12!("Adapter")]
    #[doc = crate::macros::hal_type_gles!("Adapter")]
    ///
    /// # Errors
    ///
    /// This method will return None if:
    /// - The adapter is not from the backend specified by `A`.
    /// - The adapter is from the `webgpu` or `custom` backend.
    ///
    /// # Safety
    ///
    /// - The returned resource must not be destroyed unless the guard
    ///   is the last reference to it and it is not in use by the GPU.
    ///   The guard and handle may be dropped at any time however.
    /// - All the safety requirements of wgpu-hal must be upheld.
    ///
    /// [`A::Adapter`]: hal::Api::Adapter
    #[cfg(wgpu_core)]
    pub unsafe fn as_hal<A: hal::Api>(
        &self,
    ) -> Option<impl Deref<Target = A::Adapter> + WasmNotSendSync> {
        let adapter = self.inner.as_core_opt()?;

        unsafe { adapter.context.adapter_as_hal::<A>(adapter) }
    }

    #[cfg(custom)]
    /// Returns custom implementation of adapter (if custom backend and is internally T)
    pub fn as_custom<T: custom::AdapterInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }

    #[cfg(custom)]
    /// Creates Adapter from custom implementation
    pub fn from_custom<T: custom::AdapterInterface>(adapter: T) -> Self {
        Self {
            inner: dispatch::DispatchAdapter::custom(adapter),
        }
    }

    /// Returns whether this adapter may present to the passed surface.
    pub fn is_surface_supported(&self, surface: &Surface<'_>) -> bool {
        self.inner.is_surface_supported(&surface.inner)
    }

    /// The features which can be used to create devices on this adapter.
    pub fn features(&self) -> Features {
        self.inner.features()
    }

    /// The best limits which can be used to create devices on this adapter.
    pub fn limits(&self) -> Limits {
        self.inner.limits()
    }

    /// Get info about the adapter itself.
    pub fn get_info(&self) -> AdapterInfo {
        self.inner.get_info()
    }

    /// Get info about the adapter itself.
    pub fn get_downlevel_capabilities(&self) -> DownlevelCapabilities {
        self.inner.downlevel_capabilities()
    }

    /// Returns the features supported for a given texture format by this adapter.
    ///
    /// Note that the WebGPU spec further restricts the available usages/features.
    /// To disable these restrictions on a device, request the [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] feature.
    pub fn get_texture_format_features(&self, format: TextureFormat) -> TextureFormatFeatures {
        self.inner.get_texture_format_features(format)
    }

    /// Generates a timestamp using the clock used by the presentation engine.
    ///
    /// When comparing completely opaque timestamp systems, we need a way of generating timestamps that signal
    /// the exact same time. You can do this by calling your own timestamp function immediately after a call to
    /// this function. This should result in timestamps that are 0.5 to 5 microseconds apart. There are locks
    /// that must be taken during the call, so don't call your function before.
    ///
    /// ```no_run
    /// # let adapter: wgpu::Adapter = panic!();
    /// # let some_code = || wgpu::PresentationTimestamp::INVALID_TIMESTAMP;
    /// use std::time::{Duration, Instant};
    /// let presentation = adapter.get_presentation_timestamp();
    /// let instant = Instant::now();
    ///
    /// // We can now turn a new presentation timestamp into an Instant.
    /// let some_pres_timestamp = some_code();
    /// let duration = Duration::from_nanos((some_pres_timestamp.0 - presentation.0) as u64);
    /// let new_instant: Instant = instant + duration;
    /// ```
    //
    /// [Instant]: std::time::Instant
    pub fn get_presentation_timestamp(&self) -> PresentationTimestamp {
        self.inner.get_presentation_timestamp()
    }

    /// Returns the supported cooperative matrix configurations for this adapter.
    ///
    /// Cooperative matrices enable hardware-accelerated matrix multiply-accumulate
    /// operations where threads in a subgroup collectively process matrix tiles.
    ///
    /// Returns an empty vector if cooperative matrices are not supported.
    ///
    /// Requires [`Features::EXPERIMENTAL_COOPERATIVE_MATRIX`] to be meaningful.
    pub fn cooperative_matrix_properties(&self) -> Vec<CooperativeMatrixProperties> {
        self.inner.cooperative_matrix_properties()
    }
}
