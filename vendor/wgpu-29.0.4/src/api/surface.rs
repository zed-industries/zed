use alloc::{boxed::Box, string::String, vec, vec::Vec};
#[cfg(wgpu_core)]
use core::ops::Deref;
use core::{error, fmt};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::util::Mutex;
use crate::*;

/// Describes a [`Surface`].
///
/// For use with [`Surface::configure`].
///
/// Corresponds to [WebGPU `GPUCanvasConfiguration`](
/// https://gpuweb.github.io/gpuweb/#canvas-configuration).
pub type SurfaceConfiguration = wgt::SurfaceConfiguration<Vec<TextureFormat>>;
static_assertions::assert_impl_all!(SurfaceConfiguration: Send, Sync);

/// Handle to a presentable surface.
///
/// A `Surface` represents a platform-specific surface (e.g. a window) onto which rendered images may
/// be presented. A `Surface` may be created with the function [`Instance::create_surface`].
///
/// This type is unique to the Rust API of `wgpu`. In the WebGPU specification,
/// [`GPUCanvasContext`](https://gpuweb.github.io/gpuweb/#canvas-context)
/// serves a similar role.
pub struct Surface<'window> {
    /// Additional surface data returned by [`InstanceInterface::create_surface`][cs].
    ///
    /// [cs]: crate::dispatch::InstanceInterface::create_surface
    pub(crate) inner: dispatch::DispatchSurface,

    // Stores the latest `SurfaceConfiguration` that was set using `Surface::configure`.
    // It is required to set the attributes of the `SurfaceTexture` in the
    // `Surface::get_current_texture` method.
    // Because the `Surface::configure` method operates on an immutable reference this type has to
    // be wrapped in a mutex and since the configuration is only supplied after the surface has
    // been created is is additionally wrapped in an option.
    pub(crate) config: Mutex<Option<SurfaceConfiguration>>,

    /// Optionally, keep the source of the handle used for the surface alive.
    ///
    /// This is useful for platforms where the surface is created from a window and the surface
    /// would become invalid when the window is dropped.
    ///
    /// SAFETY: This field must be dropped *after* all other fields to ensure proper cleanup.
    pub(crate) _handle_source: Option<Box<dyn WindowHandle + 'window>>,
}

impl Surface<'_> {
    /// Returns the capabilities of the surface when used with the given adapter.
    ///
    /// Returns specified values (see [`SurfaceCapabilities`]) if surface is incompatible with the adapter.
    pub fn get_capabilities(&self, adapter: &Adapter) -> SurfaceCapabilities {
        self.inner.get_capabilities(&adapter.inner)
    }

    /// Return a default `SurfaceConfiguration` from width and height to use for the [`Surface`] with this adapter.
    ///
    /// Returns None if the surface isn't supported by this adapter
    pub fn get_default_config(
        &self,
        adapter: &Adapter,
        width: u32,
        height: u32,
    ) -> Option<SurfaceConfiguration> {
        let caps = self.get_capabilities(adapter);
        Some(SurfaceConfiguration {
            usage: wgt::TextureUsages::RENDER_ATTACHMENT,
            format: *caps.formats.first()?,
            width,
            height,
            desired_maximum_frame_latency: 2,
            present_mode: *caps.present_modes.first()?,
            alpha_mode: wgt::CompositeAlphaMode::Auto,
            view_formats: vec![],
        })
    }

    /// Initializes [`Surface`] for presentation.
    ///
    /// If the surface is already configured, this will wait for the GPU to come idle
    /// before recreating the swapchain to prevent race conditions.
    ///
    /// # Validation Errors
    /// - Submissions that happen _during_ the configure may cause the
    ///   internal wait-for-idle to fail, raising a validation error.
    ///
    /// # Panics
    ///
    /// - A old [`SurfaceTexture`] is still alive referencing an old surface.
    /// - Texture format requested is unsupported on the surface.
    /// - `config.width` or `config.height` is zero.
    pub fn configure(&self, device: &Device, config: &SurfaceConfiguration) {
        self.inner.configure(&device.inner, config);

        let mut conf = self.config.lock();
        *conf = Some(config.clone());
    }

    /// Returns the current configuration of [`Surface`], if configured.
    ///
    /// This is similar to [WebGPU `GPUcCanvasContext::getConfiguration`](https://gpuweb.github.io/gpuweb/#dom-gpucanvascontext-getconfiguration).
    pub fn get_configuration(&self) -> Option<SurfaceConfiguration> {
        self.config.lock().clone()
    }

    /// Returns the next texture to be presented by the surface for drawing.
    ///
    /// In order to present the [`SurfaceTexture`] returned by this method,
    /// first a [`Queue::submit`] needs to be done with some work rendering to this texture.
    /// Then [`SurfaceTexture::present`] needs to be called.
    ///
    /// If a [`SurfaceTexture`] referencing this surface is alive when [`Surface::configure()`]
    /// is called, the configure call will panic.
    ///
    /// See the documentation of [`CurrentSurfaceTexture`] for how each possible result
    /// should be handled.
    pub fn get_current_texture(&self) -> CurrentSurfaceTexture {
        let (texture, status, detail) = self.inner.get_current_texture();

        let suboptimal = match status {
            SurfaceStatus::Good => false,
            SurfaceStatus::Suboptimal => true,
            SurfaceStatus::Timeout => return CurrentSurfaceTexture::Timeout,
            SurfaceStatus::Occluded => return CurrentSurfaceTexture::Occluded,
            SurfaceStatus::Outdated => return CurrentSurfaceTexture::Outdated,
            SurfaceStatus::Lost => return CurrentSurfaceTexture::Lost,
            SurfaceStatus::Validation => return CurrentSurfaceTexture::Validation,
        };

        let guard = self.config.lock();
        let config = guard
            .as_ref()
            .expect("This surface has not been configured yet.");

        let descriptor = TextureDescriptor {
            label: None,
            size: Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            format: config.format,
            usage: config.usage,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            view_formats: &[],
        };

        match texture {
            Some(texture) => {
                let surface_texture = SurfaceTexture {
                    texture: Texture {
                        inner: texture,
                        descriptor,
                    },
                    presented: false,
                    detail,
                };
                if suboptimal {
                    CurrentSurfaceTexture::Suboptimal(surface_texture)
                } else {
                    CurrentSurfaceTexture::Success(surface_texture)
                }
            }
            None => CurrentSurfaceTexture::Lost,
        }
    }

    /// Get the [`wgpu_hal`] surface from this `Surface`.
    ///
    /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
    /// and pass that struct to the to the `A` type parameter.
    ///
    /// Returns a guard that dereferences to the type of the hal backend
    /// which implements [`A::Surface`].
    ///
    /// # Types
    ///
    /// The returned type depends on the backend:
    ///
    #[doc = crate::macros::hal_type_vulkan!("Surface")]
    #[doc = crate::macros::hal_type_metal!("Surface")]
    #[doc = crate::macros::hal_type_dx12!("Surface")]
    #[doc = crate::macros::hal_type_gles!("Surface")]
    ///
    /// # Errors
    ///
    /// This method will return None if:
    /// - The surface is not from the backend specified by `A`.
    /// - The surface is from the `webgpu` or `custom` backend.
    ///
    /// # Safety
    ///
    /// - The returned resource must not be destroyed unless the guard
    ///   is the last reference to it and it is not in use by the GPU.
    ///   The guard and handle may be dropped at any time however.
    /// - All the safety requirements of wgpu-hal must be upheld.
    ///
    /// [`A::Surface`]: hal::Api::Surface
    #[cfg(wgpu_core)]
    pub unsafe fn as_hal<A: hal::Api>(
        &self,
    ) -> Option<impl Deref<Target = A::Surface> + WasmNotSendSync> {
        let core_surface = self.inner.as_core_opt()?;

        unsafe { core_surface.context.surface_as_hal::<A>(core_surface) }
    }

    #[cfg(custom)]
    /// Returns custom implementation of Surface (if custom backend and is internally T)
    pub fn as_custom<T: custom::SurfaceInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

// This custom implementation is required because [`Surface::_surface`] doesn't
// require [`Debug`](fmt::Debug), which we should not require from the user.
impl fmt::Debug for Surface<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Surface")
            .field(
                "_handle_source",
                &if self._handle_source.is_some() {
                    "Some"
                } else {
                    "None"
                },
            )
            .field("inner", &self.inner)
            .field("config", &self.config)
            .finish()
    }
}

#[cfg(send_sync)]
static_assertions::assert_impl_all!(Surface<'_>: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(Surface<'_> => .inner);

/// [`Send`]/[`Sync`] blanket trait for [`HasWindowHandle`] used in [`SurfaceTarget`].
pub trait WindowHandle: HasWindowHandle + WasmNotSendSync {}

impl<T: HasWindowHandle + WasmNotSendSync> WindowHandle for T {}

/// Super trait for a pair of display and window handles as used in [`SurfaceTarget`].
pub trait DisplayAndWindowHandle: WindowHandle + HasDisplayHandle {}

impl<T> DisplayAndWindowHandle for T where T: WindowHandle + HasDisplayHandle {}

/// The window/canvas/surface/swap-chain/etc. a surface is attached to, for use with safe surface creation.
///
/// This is either a window or an actual web canvas depending on the platform and
/// enabled features.
/// Refer to the individual variants for more information.
///
/// See also [`SurfaceTargetUnsafe`] for unsafe variants.
#[non_exhaustive]
pub enum SurfaceTarget<'window> {
    /// Window and display handle producer.
    ///
    /// If the specified display and window handle are not supported by any of the backends, then the surface
    /// will not be supported by any adapters.
    ///
    /// # Errors
    ///
    /// - On WebGL2: surface creation returns an error if the browser does not support WebGL2,
    ///   or declines to provide GPU access (such as due to a resource shortage).
    ///
    /// # Panics
    ///
    /// - On macOS/Metal: will panic if not called on the main thread.
    /// - On web: will panic if the [`HasWindowHandle`] does not properly refer to a
    ///   canvas element.
    /// - On all platforms: If [`crate::InstanceDescriptor::display`] was not [`None`]
    ///   but its value is not identical to that returned by [`HasDisplayHandle::display_handle()`].
    DisplayAndWindow(Box<dyn DisplayAndWindowHandle + 'window>),

    /// Window handle producer.
    ///
    /// [`HasWindowHandle`]-only version of [`SurfaceTarget::DisplayAndWindow`].
    ///
    /// This requires that the display handle was already passed through
    /// [`crate::InstanceDescriptor::display`].
    Window(Box<dyn WindowHandle + 'window>),

    /// Surface from a `web_sys::HtmlCanvasElement`.
    ///
    /// The `canvas` argument must be a valid `<canvas>` element to
    /// create a surface upon.
    ///
    /// # Errors
    ///
    /// - On WebGL2: surface creation will return an error if the browser does not support WebGL2,
    ///   or declines to provide GPU access (such as due to a resource shortage).
    #[cfg(web)]
    Canvas(web_sys::HtmlCanvasElement),

    /// Surface from a `web_sys::OffscreenCanvas`.
    ///
    /// The `canvas` argument must be a valid `OffscreenCanvas` object
    /// to create a surface upon.
    ///
    /// # Errors
    ///
    /// - On WebGL2: surface creation will return an error if the browser does not support WebGL2,
    ///   or declines to provide GPU access (such as due to a resource shortage).
    #[cfg(web)]
    OffscreenCanvas(web_sys::OffscreenCanvas),
}

impl<'a> SurfaceTarget<'a> {
    /// Constructor for [`Self::Window`] without consuming a display handle
    pub fn from_window_without_display(window: impl WindowHandle + 'a) -> Self {
        Self::Window(Box::new(window))
    }
}

impl<'a, T> From<T> for SurfaceTarget<'a>
where
    T: DisplayAndWindowHandle + 'a,
{
    fn from(window: T) -> Self {
        Self::DisplayAndWindow(Box::new(window))
    }
}

/// The window/canvas/surface/swap-chain/etc. a surface is attached to, for use with unsafe surface creation.
///
/// This is either a window or an actual web canvas depending on the platform and
/// enabled features.
/// Refer to the individual variants for more information.
///
/// See also [`SurfaceTarget`] for safe variants.
#[non_exhaustive]
pub enum SurfaceTargetUnsafe {
    /// Raw window & display handle.
    ///
    /// If the specified display and window handle are not supported by any of the backends, then the surface
    /// will not be supported by any adapters.
    ///
    /// If the `raw_display_handle` is not [`None`] here and was not [`None`] in
    /// [`crate::InstanceDescriptor::display`], their values _must_ be identical.
    ///
    /// # Safety
    ///
    /// - `raw_window_handle` & `raw_display_handle` must be valid objects to create a surface upon.
    /// - `raw_window_handle` & `raw_display_handle` must remain valid until after the returned
    ///   [`Surface`] is  dropped.
    RawHandle {
        /// Raw display handle, underlying display must outlive the surface created from this.
        raw_display_handle: Option<raw_window_handle::RawDisplayHandle>,

        /// Raw window handle, underlying window must outlive the surface created from this.
        raw_window_handle: raw_window_handle::RawWindowHandle,
    },

    /// Surface from a DRM device.
    ///
    /// If the specified DRM configuration is not supported by any of the backends, then the surface
    /// will not be supported by any adapters.
    ///
    /// # Safety
    ///
    /// - All parameters must point to valid DRM values and remain valid for as long as the resulting [`Surface`] exists.
    /// - The file descriptor (`fd`), plane, connector, and mode configuration must be valid and compatible.
    #[cfg(all(unix, not(target_vendor = "apple"), not(target_family = "wasm")))]
    Drm {
        /// The file descriptor of the DRM device.
        fd: i32,
        /// The plane index on which to create the surface.
        plane: u32,
        /// The ID of the connector associated with the selected mode.
        connector_id: u32,
        /// The display width of the selected mode.
        width: u32,
        /// The display height of the selected mode.
        height: u32,
        /// The display refresh rate of the selected mode multiplied by 1000 (e.g., 60Hz → 60000).
        refresh_rate: u32,
    },

    /// Surface from `CoreAnimationLayer`.
    ///
    /// # Safety
    ///
    /// - layer must be a valid object to create a surface upon.
    #[cfg(metal)]
    CoreAnimationLayer(*mut core::ffi::c_void),

    /// Surface from `IDCompositionVisual`.
    ///
    /// # Safety
    ///
    /// - visual must be a valid `IDCompositionVisual` to create a surface upon.  Its refcount will be incremented internally and kept live as long as the resulting [`Surface`] is live.
    #[cfg(dx12)]
    CompositionVisual(*mut core::ffi::c_void),

    /// Surface from DX12 `DirectComposition` handle.
    ///
    /// <https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-idxgifactorymedia-createswapchainforcompositionsurfacehandle>
    ///
    /// # Safety
    ///
    /// - surface_handle must be a valid `DirectComposition` handle to create a surface upon.   Its lifetime **will not** be internally managed: this handle **should not** be freed before
    ///   the resulting [`Surface`] is destroyed.
    #[cfg(dx12)]
    SurfaceHandle(*mut core::ffi::c_void),

    /// Surface from DX12 `SwapChainPanel`.
    ///
    /// # Safety
    ///
    /// - visual must be a valid SwapChainPanel to create a surface upon.  Its refcount will be incremented internally and kept live as long as the resulting [`Surface`] is live.
    #[cfg(dx12)]
    SwapChainPanel(*mut core::ffi::c_void),
}

impl SurfaceTargetUnsafe {
    /// Creates a [`SurfaceTargetUnsafe::RawHandle`] from a display and window.
    ///
    /// The `display` is optional and may be omitted if it was also passed to
    /// [`crate::InstanceDescriptor::display`].  If passed to both it must (currently) be identical.
    ///
    /// # Safety
    ///
    /// - `display` must outlive the resulting surface target
    ///   (and subsequently the surface created for this target).
    /// - `window` must outlive the resulting surface target
    ///   (and subsequently the surface created for this target).
    pub unsafe fn from_display_and_window(
        display: &impl HasDisplayHandle,
        window: &impl HasWindowHandle,
    ) -> Result<Self, raw_window_handle::HandleError> {
        Ok(Self::RawHandle {
            raw_display_handle: Some(display.display_handle()?.as_raw()),
            raw_window_handle: window.window_handle()?.as_raw(),
        })
    }

    /// Creates a [`SurfaceTargetUnsafe::RawHandle`] from a window.
    ///
    /// # Safety
    ///
    /// - `window` must outlive the resulting surface target
    ///   (and subsequently the surface created for this target).
    pub unsafe fn from_window(
        window: &impl HasWindowHandle,
    ) -> Result<Self, raw_window_handle::HandleError> {
        Ok(Self::RawHandle {
            raw_display_handle: None,
            raw_window_handle: window.window_handle()?.as_raw(),
        })
    }
}

/// [`Instance::create_surface()`] or a related function failed.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CreateSurfaceError {
    pub(crate) inner: CreateSurfaceErrorKind,
}
#[derive(Clone, Debug)]
pub(crate) enum CreateSurfaceErrorKind {
    /// Error from [`wgpu_hal`].
    #[cfg(wgpu_core)]
    Hal(wgc::instance::CreateSurfaceError),

    /// Error from WebGPU surface creation.
    #[cfg_attr(not(webgpu), expect(dead_code))]
    Web(String),

    /// Error when trying to get a [`RawDisplayHandle`][rdh] or a
    /// [`RawWindowHandle`][rwh] from a [`SurfaceTarget`].
    ///
    /// [rdh]: raw_window_handle::RawDisplayHandle
    /// [rwh]: raw_window_handle::RawWindowHandle
    RawHandle(raw_window_handle::HandleError),
}
static_assertions::assert_impl_all!(CreateSurfaceError: Send, Sync);

impl fmt::Display for CreateSurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            #[cfg(wgpu_core)]
            CreateSurfaceErrorKind::Hal(e) => e.fmt(f),
            CreateSurfaceErrorKind::Web(e) => e.fmt(f),
            CreateSurfaceErrorKind::RawHandle(e) => e.fmt(f),
        }
    }
}

impl error::Error for CreateSurfaceError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.inner {
            #[cfg(wgpu_core)]
            CreateSurfaceErrorKind::Hal(e) => e.source(),
            CreateSurfaceErrorKind::Web(_) => None,
            #[cfg(feature = "std")]
            CreateSurfaceErrorKind::RawHandle(e) => e.source(),
            #[cfg(not(feature = "std"))]
            CreateSurfaceErrorKind::RawHandle(_) => None,
        }
    }
}

#[cfg(wgpu_core)]
impl From<wgc::instance::CreateSurfaceError> for CreateSurfaceError {
    fn from(e: wgc::instance::CreateSurfaceError) -> Self {
        Self {
            inner: CreateSurfaceErrorKind::Hal(e),
        }
    }
}
