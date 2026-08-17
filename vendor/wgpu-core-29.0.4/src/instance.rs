use alloc::{
    borrow::{Cow, ToOwned as _},
    boxed::Box,
    string::String,
    sync::Arc,
    vec,
    vec::Vec,
};

use hashbrown::HashMap;
use thiserror::Error;
use wgt::error::{ErrorType, WebGpuError};

use crate::{
    api_log, api_log_debug,
    device::{queue::Queue, resource::Device, DeviceDescriptor, DeviceError},
    global::Global,
    id::{markers, AdapterId, DeviceId, QueueId, SurfaceId},
    lock::{rank, Mutex},
    present::Presentation,
    resource::ResourceType,
    resource_log,
    timestamp_normalization::TimestampNormalizerInitError,
    DOWNLEVEL_WARNING_MESSAGE,
};

use wgt::{Backend, Backends, PowerPreference};

pub type RequestAdapterOptions = wgt::RequestAdapterOptions<SurfaceId>;

#[derive(Clone, Debug, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[error("Limit '{name}' value {requested} is better than allowed {allowed}")]
pub struct FailedLimit {
    name: Cow<'static, str>,
    requested: u64,
    allowed: u64,
}

impl WebGpuError for FailedLimit {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

fn check_limits(requested: &wgt::Limits, allowed: &wgt::Limits) -> Vec<FailedLimit> {
    let mut failed = Vec::new();

    requested.check_limits_with_fail_fn(allowed, false, |name, requested, allowed| {
        failed.push(FailedLimit {
            name: Cow::Borrowed(name),
            requested,
            allowed,
        })
    });

    failed
}

#[test]
fn downlevel_default_limits_less_than_default_limits() {
    let res = check_limits(&wgt::Limits::downlevel_defaults(), &wgt::Limits::default());
    assert!(
        res.is_empty(),
        "Downlevel limits are greater than default limits",
    )
}

#[derive(Default)]
pub struct Instance {
    _name: String,

    /// List of instances per `wgpu-hal` backend.
    ///
    /// The ordering in this list implies prioritization and needs to be preserved.
    instance_per_backend: Vec<(Backend, Box<dyn hal::DynInstance>)>,

    /// The backends that were requested by the user.
    requested_backends: Backends,

    /// The backends that we could have attempted to obtain from `wgpu-hal` —
    /// those for which support is compiled in, currently.
    ///
    /// The union of this and `requested_backends` is the set of backends that would be used,
    /// independent of whether accessing the drivers/hardware for them succeeds.
    /// To obtain the set of backends actually in use by this instance, check
    /// `instance_per_backend` instead.
    supported_backends: Backends,

    pub flags: wgt::InstanceFlags,

    /// Non-lifetimed [`raw_window_handle::DisplayHandle`], for keepalive and validation purposes in
    /// [`Self::create_surface()`].
    ///
    /// When used with `winit`, callers are expected to pass its `OwnedDisplayHandle` (created from
    /// the `EventLoop`) here.
    display: Option<Box<dyn wgt::WgpuHasDisplayHandle>>,
}

impl Instance {
    pub fn new(
        name: &str,
        mut instance_desc: wgt::InstanceDescriptor,
        telemetry: Option<hal::Telemetry>,
    ) -> Self {
        let mut this = Self {
            _name: name.to_owned(),
            instance_per_backend: Vec::new(),
            requested_backends: instance_desc.backends,
            supported_backends: Backends::empty(),
            flags: instance_desc.flags,
            // HACK: We must take ownership of the field here, without being able to pass it into
            // try_add_hal(). Remove it from the mutable descriptor instead, while try_add_hal()
            // borrows the handle from `this.display` instead.
            display: instance_desc.display.take(),
        };

        #[cfg(all(vulkan, not(target_os = "netbsd")))]
        this.try_add_hal(hal::api::Vulkan, &instance_desc, telemetry);
        #[cfg(metal)]
        this.try_add_hal(hal::api::Metal, &instance_desc, telemetry);
        #[cfg(dx12)]
        this.try_add_hal(hal::api::Dx12, &instance_desc, telemetry);
        #[cfg(gles)]
        this.try_add_hal(hal::api::Gles, &instance_desc, telemetry);
        #[cfg(feature = "noop")]
        this.try_add_hal(hal::api::Noop, &instance_desc, telemetry);

        this
    }

    /// Helper for `Instance::new()`; attempts to add a single `wgpu-hal` backend to this instance.
    fn try_add_hal<A: hal::Api>(
        &mut self,
        _: A,
        instance_desc: &wgt::InstanceDescriptor,
        telemetry: Option<hal::Telemetry>,
    ) {
        // Whether or not the backend was requested, and whether or not it succeeds,
        // note that we *could* try it.
        self.supported_backends |= A::VARIANT.into();

        if !instance_desc.backends.contains(A::VARIANT.into()) {
            log::trace!("Instance::new: backend {:?} not requested", A::VARIANT);
            return;
        }

        // If this was Some, it was moved into self
        assert!(instance_desc.display.is_none());

        let hal_desc = hal::InstanceDescriptor {
            name: "wgpu",
            flags: self.flags,
            memory_budget_thresholds: instance_desc.memory_budget_thresholds,
            backend_options: instance_desc.backend_options.clone(),
            telemetry,
            // Pass a borrow, the core instance here keeps the owned handle alive already
            // WARNING: Using self here, not instance_desc!
            display: self.display.as_ref().map(|hdh| {
                hdh.display_handle()
                    .expect("Implementation did not provide a DisplayHandle")
            }),
        };

        use hal::Instance as _;
        // SAFETY: ???
        match unsafe { A::Instance::init(&hal_desc) } {
            Ok(instance) => {
                log::debug!("Instance::new: created {:?} backend", A::VARIANT);
                self.instance_per_backend
                    .push((A::VARIANT, Box::new(instance)));
            }
            Err(err) => {
                log::debug!(
                    "Instance::new: failed to create {:?} backend: {:?}",
                    A::VARIANT,
                    err
                );
            }
        }
    }

    pub(crate) fn from_hal_instance<A: hal::Api>(
        name: String,
        hal_instance: <A as hal::Api>::Instance,
    ) -> Self {
        Self {
            _name: name,
            instance_per_backend: vec![(A::VARIANT, Box::new(hal_instance))],
            requested_backends: A::VARIANT.into(),
            supported_backends: A::VARIANT.into(),
            flags: wgt::InstanceFlags::default(),
            display: None, // TODO: Extract display from HAL instance if available?
        }
    }

    pub fn raw(&self, backend: Backend) -> Option<&dyn hal::DynInstance> {
        self.instance_per_backend
            .iter()
            .find_map(|(instance_backend, instance)| {
                (*instance_backend == backend).then(|| instance.as_ref())
            })
    }

    /// # Safety
    ///
    /// - The raw instance handle returned must not be manually destroyed.
    pub unsafe fn as_hal<A: hal::Api>(&self) -> Option<&A::Instance> {
        self.raw(A::VARIANT).map(|instance| {
            instance
                .as_any()
                .downcast_ref()
                // This should be impossible. It would mean that backend instance and enum type are mismatching.
                .expect("Stored instance is not of the correct type")
        })
    }

    /// Creates a new surface targeting the given display/window handles.
    ///
    /// Internally attempts to create hal surfaces for all enabled backends.
    ///
    /// Fails only if creation for surfaces for all enabled backends fails in which case
    /// the error for each enabled backend is listed.
    /// Vice versa, if creation for any backend succeeds, success is returned.
    /// Surface creation errors are logged to the debug log in any case.
    ///
    /// # Safety
    ///
    /// - `display_handle` must be a valid object to create a surface upon,
    ///   falls back to the instance display handle otherwise.
    /// - `window_handle` must remain valid as long as the returned
    ///   [`SurfaceId`] is being used.
    pub unsafe fn create_surface(
        &self,
        display_handle: Option<raw_window_handle::RawDisplayHandle>,
        window_handle: raw_window_handle::RawWindowHandle,
    ) -> Result<Surface, CreateSurfaceError> {
        profiling::scope!("Instance::create_surface");

        let instance_display_handle = self.display.as_ref().map(|d| {
            d.display_handle()
                .expect("Implementation did not provide a DisplayHandle")
                .as_raw()
        });
        let display_handle = match (instance_display_handle, display_handle) {
            (Some(a), Some(b)) => {
                if a != b {
                    return Err(CreateSurfaceError::MismatchingDisplayHandle);
                }
                a
            }
            (Some(hnd), None) => hnd,
            (None, Some(hnd)) => hnd,
            (None, None) => return Err(CreateSurfaceError::MissingDisplayHandle),
        };

        let mut errors = HashMap::default();
        let mut surface_per_backend = HashMap::default();

        for (backend, instance) in &self.instance_per_backend {
            match unsafe {
                instance
                    .as_ref()
                    .create_surface(display_handle, window_handle)
            } {
                Ok(raw) => {
                    surface_per_backend.insert(*backend, raw);
                }
                Err(err) => {
                    log::debug!(
                        "Instance::create_surface: failed to create surface for {backend:?}: {err:?}"
                    );
                    errors.insert(*backend, err);
                }
            }
        }

        if surface_per_backend.is_empty() {
            Err(CreateSurfaceError::FailedToCreateSurfaceForAnyBackend(
                errors,
            ))
        } else {
            let surface = Surface {
                presentation: Mutex::new(rank::SURFACE_PRESENTATION, None),
                surface_per_backend,
            };

            Ok(surface)
        }
    }

    /// Creates a new surface from the given drm configuration.
    ///
    /// # Safety
    ///
    /// - All parameters must point to valid DRM values.
    ///
    /// # Platform Support
    ///
    /// This function is only available on non-apple Unix-like platforms (Linux, FreeBSD) and
    /// currently only works with the Vulkan backend.
    #[cfg(all(
        unix,
        not(target_vendor = "apple"),
        not(target_family = "wasm"),
        not(target_os = "netbsd")
    ))]
    #[cfg_attr(not(vulkan), expect(unused_variables))]
    pub unsafe fn create_surface_from_drm(
        &self,
        fd: i32,
        plane: u32,
        connector_id: u32,
        width: u32,
        height: u32,
        refresh_rate: u32,
    ) -> Result<Surface, CreateSurfaceError> {
        profiling::scope!("Instance::create_surface_from_drm");

        let mut errors = HashMap::default();
        let mut surface_per_backend: HashMap<Backend, Box<dyn hal::DynSurface>> =
            HashMap::default();

        #[cfg(all(vulkan, not(target_os = "netbsd")))]
        {
            let instance = unsafe { self.as_hal::<hal::api::Vulkan>() }
                .ok_or(CreateSurfaceError::BackendNotEnabled(Backend::Vulkan))?;

            // Safety must be upheld by the caller
            match unsafe {
                instance.create_surface_from_drm(
                    fd,
                    plane,
                    connector_id,
                    width,
                    height,
                    refresh_rate,
                )
            } {
                Ok(surface) => {
                    surface_per_backend.insert(Backend::Vulkan, Box::new(surface));
                }
                Err(err) => {
                    errors.insert(Backend::Vulkan, err);
                }
            }
        }

        if surface_per_backend.is_empty() {
            Err(CreateSurfaceError::FailedToCreateSurfaceForAnyBackend(
                errors,
            ))
        } else {
            let surface = Surface {
                presentation: Mutex::new(rank::SURFACE_PRESENTATION, None),
                surface_per_backend,
            };

            Ok(surface)
        }
    }

    /// # Safety
    ///
    /// `layer` must be a valid pointer.
    #[cfg(metal)]
    pub unsafe fn create_surface_metal(
        &self,
        layer: *mut core::ffi::c_void,
    ) -> Result<Surface, CreateSurfaceError> {
        profiling::scope!("Instance::create_surface_metal");

        let instance = unsafe { self.as_hal::<hal::api::Metal>() }
            .ok_or(CreateSurfaceError::BackendNotEnabled(Backend::Metal))?;

        let layer = layer.cast();
        // SAFETY: We do this cast and deref. (rather than using `metal` to get the
        // object we want) to avoid direct coupling on the `metal` crate.
        //
        // To wit, this pointer…
        //
        // - …is properly aligned.
        // - …is dereferenceable to a `MetalLayerRef` as an invariant of the `metal`
        //   field.
        // - …points to an _initialized_ `MetalLayerRef`.
        // - …is only ever aliased via an immutable reference that lives within this
        //   lexical scope.
        let layer = unsafe { &*layer };
        let raw_surface: Box<dyn hal::DynSurface> =
            Box::new(instance.create_surface_from_layer(layer));

        let surface = Surface {
            presentation: Mutex::new(rank::SURFACE_PRESENTATION, None),
            surface_per_backend: core::iter::once((Backend::Metal, raw_surface)).collect(),
        };

        Ok(surface)
    }

    #[cfg(dx12)]
    fn create_surface_dx12(
        &self,
        create_surface_func: impl FnOnce(&hal::dx12::Instance) -> hal::dx12::Surface,
    ) -> Result<Surface, CreateSurfaceError> {
        let instance = unsafe { self.as_hal::<hal::api::Dx12>() }
            .ok_or(CreateSurfaceError::BackendNotEnabled(Backend::Dx12))?;
        let surface: Box<dyn hal::DynSurface> = Box::new(create_surface_func(instance));

        let surface = Surface {
            presentation: Mutex::new(rank::SURFACE_PRESENTATION, None),
            surface_per_backend: core::iter::once((Backend::Dx12, surface)).collect(),
        };

        Ok(surface)
    }

    #[cfg(dx12)]
    /// # Safety
    ///
    /// The visual must be valid and able to be used to make a swapchain with.
    pub unsafe fn create_surface_from_visual(
        &self,
        visual: *mut core::ffi::c_void,
    ) -> Result<Surface, CreateSurfaceError> {
        profiling::scope!("Instance::instance_create_surface_from_visual");
        self.create_surface_dx12(|inst| unsafe { inst.create_surface_from_visual(visual) })
    }

    #[cfg(dx12)]
    /// # Safety
    ///
    /// The surface_handle must be valid and able to be used to make a swapchain with.
    pub unsafe fn create_surface_from_surface_handle(
        &self,
        surface_handle: *mut core::ffi::c_void,
    ) -> Result<Surface, CreateSurfaceError> {
        profiling::scope!("Instance::instance_create_surface_from_surface_handle");
        self.create_surface_dx12(|inst| unsafe {
            inst.create_surface_from_surface_handle(surface_handle)
        })
    }

    #[cfg(dx12)]
    /// # Safety
    ///
    /// The swap_chain_panel must be valid and able to be used to make a swapchain with.
    pub unsafe fn create_surface_from_swap_chain_panel(
        &self,
        swap_chain_panel: *mut core::ffi::c_void,
    ) -> Result<Surface, CreateSurfaceError> {
        profiling::scope!("Instance::instance_create_surface_from_swap_chain_panel");
        self.create_surface_dx12(|inst| unsafe {
            inst.create_surface_from_swap_chain_panel(swap_chain_panel)
        })
    }

    pub fn enumerate_adapters(&self, backends: Backends) -> Vec<Adapter> {
        profiling::scope!("Instance::enumerate_adapters");
        api_log!("Instance::enumerate_adapters");

        let mut adapters = Vec::new();
        for (_backend, instance) in self
            .instance_per_backend
            .iter()
            .filter(|(backend, _)| backends.contains(Backends::from(*backend)))
        {
            // NOTE: We might be using `profiling` without any features. The empty backend of this
            // macro emits no code, so unused code linting changes depending on the backend.
            profiling::scope!("enumerating", &*alloc::format!("{_backend:?}"));

            let hal_adapters = unsafe { instance.enumerate_adapters(None) };
            for raw in hal_adapters {
                let adapter = Adapter::new(raw);
                api_log_debug!("Adapter {:?}", adapter.raw.info);
                adapters.push(adapter);
            }
        }
        adapters
    }

    pub fn request_adapter(
        &self,
        desc: &wgt::RequestAdapterOptions<&Surface>,
        backends: Backends,
    ) -> Result<Adapter, wgt::RequestAdapterError> {
        profiling::scope!("Instance::request_adapter");
        api_log!("Instance::request_adapter");

        let mut adapters = Vec::new();
        let mut incompatible_surface_backends = Backends::empty();
        let mut no_fallback_backends = Backends::empty();
        let mut no_adapter_backends = Backends::empty();

        for &(backend, ref instance) in self
            .instance_per_backend
            .iter()
            .filter(|&&(backend, _)| backends.contains(Backends::from(backend)))
        {
            let compatible_hal_surface = desc
                .compatible_surface
                .and_then(|surface| surface.raw(backend));

            let mut backend_adapters =
                unsafe { instance.enumerate_adapters(compatible_hal_surface) };
            if backend_adapters.is_empty() {
                log::debug!("enabled backend `{backend:?}` has no adapters");
                no_adapter_backends |= Backends::from(backend);
                // by continuing, we avoid setting the further error bits below
                continue;
            }

            if desc.force_fallback_adapter {
                log::debug!("Filtering `{backend:?}` for `force_fallback_adapter`");
                backend_adapters.retain(|exposed| {
                    let keep = exposed.info.device_type == wgt::DeviceType::Cpu;
                    if !keep {
                        log::debug!("* Eliminating adapter `{}`", exposed.info.name);
                    }
                    keep
                });
                if backend_adapters.is_empty() {
                    log::debug!("* Backend `{backend:?}` has no fallback adapters");
                    no_fallback_backends |= Backends::from(backend);
                    continue;
                }
            }

            if let Some(surface) = desc.compatible_surface {
                backend_adapters.retain(|exposed| {
                    let capabilities = surface.get_capabilities_with_raw(exposed);
                    if let Err(err) = capabilities {
                        log::debug!(
                            "Adapter {:?} not compatible with surface: {}",
                            exposed.info,
                            err
                        );
                        incompatible_surface_backends |= Backends::from(backend);
                        false
                    } else {
                        true
                    }
                });
                if backend_adapters.is_empty() {
                    incompatible_surface_backends |= Backends::from(backend);
                    continue;
                }
            }
            adapters.extend(backend_adapters);
        }

        match desc.power_preference {
            PowerPreference::LowPower => {
                sort(&mut adapters, true);
            }
            PowerPreference::HighPerformance => {
                sort(&mut adapters, false);
            }
            PowerPreference::None => {}
        };

        fn sort(adapters: &mut [hal::DynExposedAdapter], prefer_integrated_gpu: bool) {
            adapters
                .sort_by_key(|adapter| get_order(adapter.info.device_type, prefer_integrated_gpu));
        }

        fn get_order(device_type: wgt::DeviceType, prefer_integrated_gpu: bool) -> u8 {
            // Since devices of type "Other" might really be "Unknown" and come
            // from APIs like OpenGL that don't specify device type, Prefer more
            // Specific types over Other.
            //
            // This means that backends which do provide accurate device types
            // will be preferred if their device type indicates an actual
            // hardware GPU (integrated or discrete).
            match device_type {
                wgt::DeviceType::DiscreteGpu if prefer_integrated_gpu => 2,
                wgt::DeviceType::IntegratedGpu if prefer_integrated_gpu => 1,
                wgt::DeviceType::DiscreteGpu => 1,
                wgt::DeviceType::IntegratedGpu => 2,
                wgt::DeviceType::Other => 3,
                wgt::DeviceType::VirtualGpu => 4,
                wgt::DeviceType::Cpu => 5,
            }
        }

        // `request_adapter` can be a bit of a black box.
        // Shine some light on its decision in debug log.
        if adapters.is_empty() {
            log::debug!("Request adapter didn't find compatible adapters.");
        } else {
            log::debug!(
                "Found {} compatible adapters. Sorted by preference:",
                adapters.len()
            );
            for adapter in &adapters {
                log::debug!("* {:?}", adapter.info);
            }
        }

        if let Some(adapter) = adapters.into_iter().next() {
            api_log_debug!("Request adapter result {:?}", adapter.info);
            let adapter = Adapter::new(adapter);
            Ok(adapter)
        } else {
            Err(wgt::RequestAdapterError::NotFound {
                supported_backends: self.supported_backends,
                requested_backends: self.requested_backends,
                active_backends: self.active_backends(),
                no_fallback_backends,
                no_adapter_backends,
                incompatible_surface_backends,
            })
        }
    }

    fn active_backends(&self) -> Backends {
        self.instance_per_backend
            .iter()
            .map(|&(backend, _)| Backends::from(backend))
            .collect()
    }
}

pub struct Surface {
    pub(crate) presentation: Mutex<Option<Presentation>>,
    pub surface_per_backend: HashMap<Backend, Box<dyn hal::DynSurface>>,
}

impl ResourceType for Surface {
    const TYPE: &'static str = "Surface";
}
impl crate::storage::StorageItem for Surface {
    type Marker = markers::Surface;
}

impl Surface {
    pub fn get_capabilities(
        &self,
        adapter: &Adapter,
    ) -> Result<hal::SurfaceCapabilities, GetSurfaceSupportError> {
        self.get_capabilities_with_raw(&adapter.raw)
    }

    pub fn get_capabilities_with_raw(
        &self,
        adapter: &hal::DynExposedAdapter,
    ) -> Result<hal::SurfaceCapabilities, GetSurfaceSupportError> {
        let backend = adapter.backend();
        let suf = self
            .raw(backend)
            .ok_or(GetSurfaceSupportError::NotSupportedByBackend(backend))?;
        profiling::scope!("surface_capabilities");
        let caps = unsafe { adapter.adapter.surface_capabilities(suf) }
            .ok_or(GetSurfaceSupportError::FailedToRetrieveSurfaceCapabilitiesForAdapter)?;
        Ok(caps)
    }

    pub fn raw(&self, backend: Backend) -> Option<&dyn hal::DynSurface> {
        self.surface_per_backend
            .get(&backend)
            .map(|surface| surface.as_ref())
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        if let Some(present) = self.presentation.lock().take() {
            for (&backend, surface) in &self.surface_per_backend {
                if backend == present.device.backend() {
                    unsafe { surface.unconfigure(present.device.raw()) };
                }
            }
        }
    }
}

pub struct Adapter {
    pub(crate) raw: hal::DynExposedAdapter,
}

impl Adapter {
    pub fn new(mut raw: hal::DynExposedAdapter) -> Self {
        // WebGPU requires this offset alignment as lower bound on all adapters.
        const MIN_BUFFER_OFFSET_ALIGNMENT_LOWER_BOUND: u32 = 32;

        let limits = &mut raw.capabilities.limits;

        limits.min_uniform_buffer_offset_alignment = limits
            .min_uniform_buffer_offset_alignment
            .max(MIN_BUFFER_OFFSET_ALIGNMENT_LOWER_BOUND);
        limits.min_storage_buffer_offset_alignment = limits
            .min_storage_buffer_offset_alignment
            .max(MIN_BUFFER_OFFSET_ALIGNMENT_LOWER_BOUND);

        Self { raw }
    }

    /// Returns the backend this adapter is using.
    pub fn backend(&self) -> Backend {
        self.raw.backend()
    }

    pub fn is_surface_supported(&self, surface: &Surface) -> bool {
        // If get_capabilities returns Err, then the API does not advertise support for the surface.
        //
        // This could occur if the user is running their app on Wayland but Vulkan does not support
        // VK_KHR_wayland_surface.
        surface.get_capabilities(self).is_ok()
    }

    pub fn get_info(&self) -> wgt::AdapterInfo {
        self.raw.info.clone()
    }

    pub fn features(&self) -> wgt::Features {
        self.raw.features
    }

    pub fn limits(&self) -> wgt::Limits {
        self.raw.capabilities.limits.clone()
    }

    pub fn downlevel_capabilities(&self) -> wgt::DownlevelCapabilities {
        self.raw.capabilities.downlevel.clone()
    }

    pub fn get_presentation_timestamp(&self) -> wgt::PresentationTimestamp {
        unsafe { self.raw.adapter.get_presentation_timestamp() }
    }

    pub fn cooperative_matrix_properties(&self) -> Vec<wgt::CooperativeMatrixProperties> {
        self.raw.capabilities.cooperative_matrix_properties.clone()
    }

    pub fn get_texture_format_features(
        &self,
        format: wgt::TextureFormat,
    ) -> wgt::TextureFormatFeatures {
        use hal::TextureFormatCapabilities as Tfc;

        let caps = unsafe { self.raw.adapter.texture_format_capabilities(format) };
        let mut allowed_usages = wgt::TextureUsages::empty();

        allowed_usages.set(wgt::TextureUsages::COPY_SRC, caps.contains(Tfc::COPY_SRC));
        allowed_usages.set(wgt::TextureUsages::COPY_DST, caps.contains(Tfc::COPY_DST));
        allowed_usages.set(
            wgt::TextureUsages::TEXTURE_BINDING,
            caps.contains(Tfc::SAMPLED),
        );
        allowed_usages.set(
            wgt::TextureUsages::STORAGE_BINDING,
            caps.intersects(
                Tfc::STORAGE_WRITE_ONLY
                    | Tfc::STORAGE_READ_ONLY
                    | Tfc::STORAGE_READ_WRITE
                    | Tfc::STORAGE_ATOMIC,
            ),
        );
        allowed_usages.set(
            wgt::TextureUsages::RENDER_ATTACHMENT | wgt::TextureUsages::TRANSIENT,
            caps.intersects(Tfc::COLOR_ATTACHMENT | Tfc::DEPTH_STENCIL_ATTACHMENT),
        );
        allowed_usages.set(
            wgt::TextureUsages::STORAGE_ATOMIC,
            caps.contains(Tfc::STORAGE_ATOMIC),
        );

        let mut flags = wgt::TextureFormatFeatureFlags::empty();
        flags.set(
            wgt::TextureFormatFeatureFlags::STORAGE_READ_ONLY,
            caps.contains(Tfc::STORAGE_READ_ONLY),
        );
        flags.set(
            wgt::TextureFormatFeatureFlags::STORAGE_WRITE_ONLY,
            caps.contains(Tfc::STORAGE_WRITE_ONLY),
        );
        flags.set(
            wgt::TextureFormatFeatureFlags::STORAGE_READ_WRITE,
            caps.contains(Tfc::STORAGE_READ_WRITE),
        );

        flags.set(
            wgt::TextureFormatFeatureFlags::STORAGE_ATOMIC,
            caps.contains(Tfc::STORAGE_ATOMIC),
        );

        flags.set(
            wgt::TextureFormatFeatureFlags::FILTERABLE,
            caps.contains(Tfc::SAMPLED_LINEAR),
        );

        flags.set(
            wgt::TextureFormatFeatureFlags::BLENDABLE,
            caps.contains(Tfc::COLOR_ATTACHMENT_BLEND),
        );

        flags.set(
            wgt::TextureFormatFeatureFlags::MULTISAMPLE_X2,
            caps.contains(Tfc::MULTISAMPLE_X2),
        );
        flags.set(
            wgt::TextureFormatFeatureFlags::MULTISAMPLE_X4,
            caps.contains(Tfc::MULTISAMPLE_X4),
        );
        flags.set(
            wgt::TextureFormatFeatureFlags::MULTISAMPLE_X8,
            caps.contains(Tfc::MULTISAMPLE_X8),
        );
        flags.set(
            wgt::TextureFormatFeatureFlags::MULTISAMPLE_X16,
            caps.contains(Tfc::MULTISAMPLE_X16),
        );

        flags.set(
            wgt::TextureFormatFeatureFlags::MULTISAMPLE_RESOLVE,
            caps.contains(Tfc::MULTISAMPLE_RESOLVE),
        );

        wgt::TextureFormatFeatures {
            allowed_usages,
            flags,
        }
    }

    fn create_device_and_queue_from_hal(
        self: &Arc<Self>,
        hal_device: hal::DynOpenDevice,
        desc: &DeviceDescriptor,
        instance_flags: wgt::InstanceFlags,
    ) -> Result<(Arc<Device>, Arc<Queue>), RequestDeviceError> {
        api_log!("Adapter::create_device");

        let device = Device::new(hal_device.device, self, desc, instance_flags)?;
        let device = Arc::new(device);

        let queue = Queue::new(device.clone(), hal_device.queue, instance_flags)?;
        let queue = Arc::new(queue);

        device.set_queue(&queue);
        device.late_init_resources_with_queue()?;

        Ok((device, queue))
    }

    pub fn create_device_and_queue(
        self: &Arc<Self>,
        desc: &DeviceDescriptor,
        instance_flags: wgt::InstanceFlags,
    ) -> Result<(Arc<Device>, Arc<Queue>), RequestDeviceError> {
        // Verify all features were exposed by the adapter
        if !self.raw.features.contains(desc.required_features) {
            return Err(RequestDeviceError::UnsupportedFeature(
                desc.required_features - self.raw.features,
            ));
        }

        // Check if experimental features are permitted to be enabled.
        if desc
            .required_features
            .intersects(wgt::Features::all_experimental_mask())
            && !desc.experimental_features.is_enabled()
        {
            return Err(RequestDeviceError::ExperimentalFeaturesNotEnabled(
                desc.required_features
                    .intersection(wgt::Features::all_experimental_mask()),
            ));
        }

        let caps = &self.raw.capabilities;
        if Backends::PRIMARY.contains(Backends::from(self.backend()))
            && !caps.downlevel.is_webgpu_compliant()
        {
            let missing_flags = wgt::DownlevelFlags::compliant() - caps.downlevel.flags;
            log::warn!("Missing downlevel flags: {missing_flags:?}\n{DOWNLEVEL_WARNING_MESSAGE}");
            log::warn!("{:#?}", caps.downlevel);
        }

        // Verify feature preconditions
        if desc
            .required_features
            .contains(wgt::Features::MAPPABLE_PRIMARY_BUFFERS)
            && self.raw.info.device_type == wgt::DeviceType::DiscreteGpu
        {
            log::warn!(
                "Feature MAPPABLE_PRIMARY_BUFFERS enabled on a discrete gpu. \
                        This is a massive performance footgun and likely not what you wanted"
            );
        }

        if let Some(failed) = check_limits(&desc.required_limits, &caps.limits).pop() {
            return Err(RequestDeviceError::LimitsExceeded(failed));
        }

        let open = unsafe {
            self.raw.adapter.open(
                desc.required_features,
                &desc.required_limits,
                &desc.memory_hints,
            )
        }
        .map_err(DeviceError::from_hal)?;

        self.create_device_and_queue_from_hal(open, desc, instance_flags)
    }
}

crate::impl_resource_type!(Adapter);
crate::impl_storage_item!(Adapter);

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum GetSurfaceSupportError {
    #[error("Surface is not supported for the specified backend {0}")]
    NotSupportedByBackend(Backend),
    #[error("Failed to retrieve surface capabilities for the specified adapter.")]
    FailedToRetrieveSurfaceCapabilitiesForAdapter,
}

#[derive(Clone, Debug, Error)]
/// Error when requesting a device from the adapter
#[non_exhaustive]
pub enum RequestDeviceError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    LimitsExceeded(#[from] FailedLimit),
    #[error("Failed to initialize Timestamp Normalizer")]
    TimestampNormalizerInitFailed(#[from] TimestampNormalizerInitError),
    #[error("Unsupported features were requested: {0}")]
    UnsupportedFeature(wgt::Features),
    #[error(
        "Some experimental features, {0}, were requested, but experimental features are not enabled"
    )]
    ExperimentalFeaturesNotEnabled(wgt::Features),
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateSurfaceError {
    #[error("The backend {0} was not enabled on the instance.")]
    BackendNotEnabled(Backend),
    #[error("Failed to create surface for any enabled backend: {0:?}")]
    FailedToCreateSurfaceForAnyBackend(HashMap<Backend, hal::InstanceError>),
    #[error("The display handle used to create this Instance does not match the one used to create a surface on it")]
    MismatchingDisplayHandle,
    #[error(
        "No `DisplayHandle` is available to create this surface with.  When creating a surface with `create_surface()` \
        you must specify a display handle in `InstanceDescriptor::display`.  \
        Rarely, if you need to create surfaces from different `DisplayHandle`s (ex. different Wayland or X11 connections), \
        you must use `create_surface_unsafe()`."
    )]
    MissingDisplayHandle,
}

impl Global {
    /// Creates a new surface targeting the given display/window handles.
    ///
    /// Internally attempts to create hal surfaces for all enabled backends.
    ///
    /// Fails only if creation for surfaces for all enabled backends fails in which case
    /// the error for each enabled backend is listed.
    /// Vice versa, if creation for any backend succeeds, success is returned.
    /// Surface creation errors are logged to the debug log in any case.
    ///
    /// id_in:
    /// - If `Some`, the id to assign to the surface. A new one will be generated otherwise.
    ///
    /// # Safety
    ///
    /// - `display_handle` must be a valid object to create a surface upon,
    ///   falls back to the instance display handle otherwise.
    /// - `window_handle` must remain valid as long as the returned
    ///   [`SurfaceId`] is being used.
    pub unsafe fn instance_create_surface(
        &self,
        display_handle: Option<raw_window_handle::RawDisplayHandle>,
        window_handle: raw_window_handle::RawWindowHandle,
        id_in: Option<SurfaceId>,
    ) -> Result<SurfaceId, CreateSurfaceError> {
        let surface = unsafe { self.instance.create_surface(display_handle, window_handle) }?;
        let id = self.surfaces.prepare(id_in).assign(Arc::new(surface));
        Ok(id)
    }

    /// Creates a new surface from the given drm configuration.
    ///
    /// # Safety
    ///
    /// - All parameters must point to valid DRM values.
    ///
    /// # Platform Support
    ///
    /// This function is only available on non-apple Unix-like platforms (Linux, FreeBSD) and
    /// currently only works with the Vulkan backend.
    #[cfg(all(
        unix,
        not(target_vendor = "apple"),
        not(target_family = "wasm"),
        not(target_os = "netbsd")
    ))]
    pub unsafe fn instance_create_surface_from_drm(
        &self,
        fd: i32,
        plane: u32,
        connector_id: u32,
        width: u32,
        height: u32,
        refresh_rate: u32,
        id_in: Option<SurfaceId>,
    ) -> Result<SurfaceId, CreateSurfaceError> {
        let surface = unsafe {
            self.instance.create_surface_from_drm(
                fd,
                plane,
                connector_id,
                width,
                height,
                refresh_rate,
            )
        }?;
        let id = self.surfaces.prepare(id_in).assign(Arc::new(surface));

        Ok(id)
    }

    /// # Safety
    ///
    /// `layer` must be a valid pointer.
    #[cfg(metal)]
    pub unsafe fn instance_create_surface_metal(
        &self,
        layer: *mut core::ffi::c_void,
        id_in: Option<SurfaceId>,
    ) -> Result<SurfaceId, CreateSurfaceError> {
        let surface = unsafe { self.instance.create_surface_metal(layer) }?;
        let id = self.surfaces.prepare(id_in).assign(Arc::new(surface));
        Ok(id)
    }

    #[cfg(dx12)]
    /// # Safety
    ///
    /// The visual must be valid and able to be used to make a swapchain with.
    pub unsafe fn instance_create_surface_from_visual(
        &self,
        visual: *mut core::ffi::c_void,
        id_in: Option<SurfaceId>,
    ) -> Result<SurfaceId, CreateSurfaceError> {
        let surface = unsafe { self.instance.create_surface_from_visual(visual) }?;
        let id = self.surfaces.prepare(id_in).assign(Arc::new(surface));
        Ok(id)
    }

    #[cfg(dx12)]
    /// # Safety
    ///
    /// The surface_handle must be valid and able to be used to make a swapchain with.
    pub unsafe fn instance_create_surface_from_surface_handle(
        &self,
        surface_handle: *mut core::ffi::c_void,
        id_in: Option<SurfaceId>,
    ) -> Result<SurfaceId, CreateSurfaceError> {
        let surface = unsafe {
            self.instance
                .create_surface_from_surface_handle(surface_handle)
        }?;
        let id = self.surfaces.prepare(id_in).assign(Arc::new(surface));
        Ok(id)
    }

    #[cfg(dx12)]
    /// # Safety
    ///
    /// The swap_chain_panel must be valid and able to be used to make a swapchain with.
    pub unsafe fn instance_create_surface_from_swap_chain_panel(
        &self,
        swap_chain_panel: *mut core::ffi::c_void,
        id_in: Option<SurfaceId>,
    ) -> Result<SurfaceId, CreateSurfaceError> {
        let surface = unsafe {
            self.instance
                .create_surface_from_swap_chain_panel(swap_chain_panel)
        }?;
        let id = self.surfaces.prepare(id_in).assign(Arc::new(surface));
        Ok(id)
    }

    pub fn surface_drop(&self, id: SurfaceId) {
        profiling::scope!("Surface::drop");

        api_log!("Surface::drop {id:?}");

        self.surfaces.remove(id);
    }

    pub fn enumerate_adapters(&self, backends: Backends) -> Vec<AdapterId> {
        let adapters = self.instance.enumerate_adapters(backends);
        adapters
            .into_iter()
            .map(|adapter| self.hub.adapters.prepare(None).assign(Arc::new(adapter)))
            .collect()
    }

    pub fn request_adapter(
        &self,
        desc: &RequestAdapterOptions,
        backends: Backends,
        id_in: Option<AdapterId>,
    ) -> Result<AdapterId, wgt::RequestAdapterError> {
        let compatible_surface = desc.compatible_surface.map(|id| self.surfaces.get(id));
        let desc = wgt::RequestAdapterOptions {
            power_preference: desc.power_preference,
            force_fallback_adapter: desc.force_fallback_adapter,
            compatible_surface: compatible_surface.as_deref(),
        };
        let adapter = self.instance.request_adapter(&desc, backends)?;
        let id = self.hub.adapters.prepare(id_in).assign(Arc::new(adapter));
        Ok(id)
    }

    /// # Safety
    ///
    /// `hal_adapter` must be created from this global internal instance handle.
    pub unsafe fn create_adapter_from_hal(
        &self,
        hal_adapter: hal::DynExposedAdapter,
        input: Option<AdapterId>,
    ) -> AdapterId {
        profiling::scope!("Instance::create_adapter_from_hal");

        let fid = self.hub.adapters.prepare(input);
        let id = fid.assign(Arc::new(Adapter::new(hal_adapter)));

        resource_log!("Created Adapter {:?}", id);
        id
    }

    pub fn adapter_get_info(&self, adapter_id: AdapterId) -> wgt::AdapterInfo {
        let adapter = self.hub.adapters.get(adapter_id);
        adapter.get_info()
    }

    pub fn adapter_get_texture_format_features(
        &self,
        adapter_id: AdapterId,
        format: wgt::TextureFormat,
    ) -> wgt::TextureFormatFeatures {
        let adapter = self.hub.adapters.get(adapter_id);
        adapter.get_texture_format_features(format)
    }

    pub fn adapter_features(&self, adapter_id: AdapterId) -> wgt::Features {
        let adapter = self.hub.adapters.get(adapter_id);
        adapter.features()
    }

    pub fn adapter_limits(&self, adapter_id: AdapterId) -> wgt::Limits {
        let adapter = self.hub.adapters.get(adapter_id);
        adapter.limits()
    }

    pub fn adapter_downlevel_capabilities(
        &self,
        adapter_id: AdapterId,
    ) -> wgt::DownlevelCapabilities {
        let adapter = self.hub.adapters.get(adapter_id);
        adapter.downlevel_capabilities()
    }

    pub fn adapter_get_presentation_timestamp(
        &self,
        adapter_id: AdapterId,
    ) -> wgt::PresentationTimestamp {
        let adapter = self.hub.adapters.get(adapter_id);
        adapter.get_presentation_timestamp()
    }

    pub fn adapter_cooperative_matrix_properties(
        &self,
        adapter_id: AdapterId,
    ) -> Vec<wgt::CooperativeMatrixProperties> {
        let adapter = self.hub.adapters.get(adapter_id);
        adapter.cooperative_matrix_properties()
    }

    pub fn adapter_drop(&self, adapter_id: AdapterId) {
        profiling::scope!("Adapter::drop");
        api_log!("Adapter::drop {adapter_id:?}");

        self.hub.adapters.remove(adapter_id);
    }
}

impl Global {
    pub fn adapter_request_device(
        &self,
        adapter_id: AdapterId,
        desc: &DeviceDescriptor,
        device_id_in: Option<DeviceId>,
        queue_id_in: Option<QueueId>,
    ) -> Result<(DeviceId, QueueId), RequestDeviceError> {
        profiling::scope!("Adapter::request_device");
        api_log!("Adapter::request_device");

        let device_fid = self.hub.devices.prepare(device_id_in);
        let queue_fid = self.hub.queues.prepare(queue_id_in);

        let adapter = self.hub.adapters.get(adapter_id);
        let (device, queue) = adapter.create_device_and_queue(desc, self.instance.flags)?;

        let device_id = device_fid.assign(device);
        resource_log!("Created Device {:?}", device_id);

        let queue_id = queue_fid.assign(queue);
        resource_log!("Created Queue {:?}", queue_id);

        Ok((device_id, queue_id))
    }

    /// # Safety
    ///
    /// - `hal_device` must be created from `adapter_id` or its internal handle.
    /// - `desc` must be a subset of `hal_device` features and limits.
    pub unsafe fn create_device_from_hal(
        &self,
        adapter_id: AdapterId,
        hal_device: hal::DynOpenDevice,
        desc: &DeviceDescriptor,
        device_id_in: Option<DeviceId>,
        queue_id_in: Option<QueueId>,
    ) -> Result<(DeviceId, QueueId), RequestDeviceError> {
        profiling::scope!("Global::create_device_from_hal");

        let devices_fid = self.hub.devices.prepare(device_id_in);
        let queues_fid = self.hub.queues.prepare(queue_id_in);

        let adapter = self.hub.adapters.get(adapter_id);
        let (device, queue) =
            adapter.create_device_and_queue_from_hal(hal_device, desc, self.instance.flags)?;

        let device_id = devices_fid.assign(device);
        resource_log!("Created Device {:?}", device_id);

        let queue_id = queues_fid.assign(queue);
        resource_log!("Created Queue {:?}", queue_id);

        Ok((device_id, queue_id))
    }
}
