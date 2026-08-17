use alloc::{boxed::Box, string::String, sync::Arc, vec};
#[cfg(wgpu_core)]
use core::ops::Deref;
use core::{error, fmt, future::Future, marker::PhantomData};

use crate::api::blas::{Blas, BlasGeometrySizeDescriptors, CreateBlasDescriptor};
use crate::api::tlas::{CreateTlasDescriptor, Tlas};
use crate::util::Mutex;
use crate::*;

/// Open connection to a graphics and/or compute device.
///
/// Responsible for the creation of most rendering and compute resources.
/// These are then used in commands, which are submitted to a [`Queue`].
///
/// A device may be requested from an adapter with [`Adapter::request_device`].
///
/// Corresponds to [WebGPU `GPUDevice`](https://gpuweb.github.io/gpuweb/#gpu-device).
#[derive(Debug, Clone)]
pub struct Device {
    pub(crate) inner: dispatch::DispatchDevice,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(Device: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(Device => .inner);

/// Describes a [`Device`].
///
/// For use with [`Adapter::request_device`].
///
/// Corresponds to [WebGPU `GPUDeviceDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpudevicedescriptor).
pub type DeviceDescriptor<'a> = wgt::DeviceDescriptor<Label<'a>>;
static_assertions::assert_impl_all!(DeviceDescriptor<'_>: Send, Sync);

impl Device {
    #[cfg(custom)]
    /// Returns custom implementation of Device (if custom backend and is internally T)
    pub fn as_custom<T: custom::DeviceInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }

    #[cfg(custom)]
    /// Creates Device from custom implementation
    pub fn from_custom<T: custom::DeviceInterface>(device: T) -> Self {
        Self {
            inner: dispatch::DispatchDevice::custom(device),
        }
    }

    /// Constructs a stub device for testing using [`Backend::Noop`].
    ///
    /// This is a convenience function which avoids the configuration, `async`, and fallibility
    /// aspects of constructing a device through `Instance`.
    #[cfg(feature = "noop")]
    pub fn noop(desc: &DeviceDescriptor<'_>) -> (Device, Queue) {
        use core::future::Future as _;
        use core::pin::pin;
        use core::task;
        let ctx = &mut task::Context::from_waker(task::Waker::noop());

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::NOOP,
            backend_options: BackendOptions {
                noop: NoopBackendOptions { enable: true },
                ..Default::default()
            },
            ..InstanceDescriptor::new_without_display_handle()
        });

        // Both of these futures are trivial and should complete instantaneously,
        // so we do not need an executor and can just poll them once.
        let task::Poll::Ready(Ok(adapter)) =
            pin!(instance.request_adapter(&RequestAdapterOptions::default())).poll(ctx)
        else {
            unreachable!()
        };
        let task::Poll::Ready(Ok(device_and_queue)) = pin!(adapter.request_device(desc)).poll(ctx)
        else {
            unreachable!()
        };
        device_and_queue
    }

    /// Check for resource cleanups and mapping callbacks. Will block if [`PollType::Wait`] is passed.
    ///
    /// Return `true` if the queue is empty, or `false` if there are more queue
    /// submissions still in flight. (Note that, unless access to the [`Queue`] is
    /// coordinated somehow, this information could be out of date by the time
    /// the caller receives it. `Queue`s can be shared between threads, so
    /// other threads could submit new work at any time.)
    ///
    /// When running on WebGPU, this is a no-op. `Device`s are automatically polled.
    pub fn poll(&self, poll_type: PollType) -> Result<crate::PollStatus, crate::PollError> {
        self.inner.poll(poll_type.map_index(|s| s.index))
    }

    /// The [features][Features] which can be used on this device.
    ///
    /// This will be equal to the [`required_features`][DeviceDescriptor::required_features]
    /// specified when creating the device.
    /// No additional features can be used, even if the underlying adapter can support them.
    #[must_use]
    pub fn features(&self) -> Features {
        self.inner.features()
    }

    /// The limits which can be used on this device.
    ///
    /// This will be equal to the [`required_limits`][DeviceDescriptor::required_limits]
    /// specified when creating the device.
    /// No better limits can be used, even if the underlying adapter can support them.
    #[must_use]
    pub fn limits(&self) -> Limits {
        self.inner.limits()
    }

    /// Get info about the adapter that this device was created from.
    pub fn adapter_info(&self) -> AdapterInfo {
        self.inner.adapter_info()
    }

    /// Creates a shader module.
    ///
    /// <div class="warning">
    // NOTE: Keep this in sync with `naga::front::wgsl::parse_str`!
    // NOTE: Keep this in sync with `wgpu_core::Global::device_create_shader_module`!
    ///
    /// This function may consume a lot of stack space. Compiler-enforced limits for parsing
    /// recursion exist; if shader compilation runs into them, it will return an error gracefully.
    /// However, on some build profiles and platforms, the default stack size for a thread may be
    /// exceeded before this limit is reached during parsing. Callers should ensure that there is
    /// enough stack space for this, particularly if calls to this method are exposed to user
    /// input.
    ///
    /// </div>
    #[must_use]
    pub fn create_shader_module(&self, desc: ShaderModuleDescriptor<'_>) -> ShaderModule {
        let module = self
            .inner
            .create_shader_module(desc, wgt::ShaderRuntimeChecks::checked());
        ShaderModule { inner: module }
    }

    /// Deprecated: Use [`create_shader_module_trusted`][csmt] instead.
    ///
    /// # Safety
    ///
    /// See [`create_shader_module_trusted`][csmt].
    ///
    /// [csmt]: Self::create_shader_module_trusted
    #[deprecated(
        since = "24.0.0",
        note = "Use `Device::create_shader_module_trusted(desc, wgpu::ShaderRuntimeChecks::unchecked())` instead."
    )]
    #[must_use]
    pub unsafe fn create_shader_module_unchecked(
        &self,
        desc: ShaderModuleDescriptor<'_>,
    ) -> ShaderModule {
        unsafe { self.create_shader_module_trusted(desc, crate::ShaderRuntimeChecks::unchecked()) }
    }

    /// Creates a shader module with flags to dictate runtime checks.
    ///
    /// When running on WebGPU, this will merely call [`create_shader_module`][csm].
    ///
    /// # Safety
    ///
    /// In contrast with [`create_shader_module`][csm] this function
    /// creates a shader module with user-customizable runtime checks which allows shaders to
    /// perform operations which can lead to undefined behavior like indexing out of bounds,
    /// thus it's the caller responsibility to pass a shader which doesn't perform any of this
    /// operations.
    ///
    /// See the documentation for [`ShaderRuntimeChecks`] for more information about specific checks.
    ///
    /// [csm]: Self::create_shader_module
    #[must_use]
    pub unsafe fn create_shader_module_trusted(
        &self,
        desc: ShaderModuleDescriptor<'_>,
        runtime_checks: crate::ShaderRuntimeChecks,
    ) -> ShaderModule {
        let module = self.inner.create_shader_module(desc, runtime_checks);
        ShaderModule { inner: module }
    }

    /// Creates a shader module which will bypass wgpu's shader tooling and validation and be used directly by the backend.
    ///
    /// # Safety
    ///
    /// This function passes data to the backend as-is and can potentially result in a
    /// driver crash or bogus behaviour. No attempt is made to ensure that data is valid.
    #[must_use]
    pub unsafe fn create_shader_module_passthrough(
        &self,
        desc: ShaderModuleDescriptorPassthrough<'_>,
    ) -> ShaderModule {
        let module = unsafe { self.inner.create_shader_module_passthrough(&desc) };
        ShaderModule { inner: module }
    }

    /// Creates an empty [`CommandEncoder`].
    #[must_use]
    pub fn create_command_encoder(&self, desc: &CommandEncoderDescriptor<'_>) -> CommandEncoder {
        let encoder = self.inner.create_command_encoder(desc);
        // Each encoder starts with its own deferred-action store that travels
        // with the CommandBuffer produced by finish().
        CommandEncoder {
            inner: encoder,
            actions: Default::default(),
        }
    }

    /// Creates an empty [`RenderBundleEncoder`].
    #[must_use]
    pub fn create_render_bundle_encoder<'a>(
        &self,
        desc: &RenderBundleEncoderDescriptor<'_>,
    ) -> RenderBundleEncoder<'a> {
        let encoder = self.inner.create_render_bundle_encoder(desc);
        RenderBundleEncoder {
            inner: encoder,
            _p: PhantomData,
        }
    }

    /// Creates a new [`BindGroup`].
    #[must_use]
    pub fn create_bind_group(&self, desc: &BindGroupDescriptor<'_>) -> BindGroup {
        let group = self.inner.create_bind_group(desc);
        BindGroup { inner: group }
    }

    /// Creates a [`BindGroupLayout`].
    #[must_use]
    pub fn create_bind_group_layout(
        &self,
        desc: &BindGroupLayoutDescriptor<'_>,
    ) -> BindGroupLayout {
        let layout = self.inner.create_bind_group_layout(desc);
        BindGroupLayout { inner: layout }
    }

    /// Creates a [`PipelineLayout`].
    #[must_use]
    pub fn create_pipeline_layout(&self, desc: &PipelineLayoutDescriptor<'_>) -> PipelineLayout {
        let layout = self.inner.create_pipeline_layout(desc);
        PipelineLayout { inner: layout }
    }

    /// Creates a [`RenderPipeline`].
    #[must_use]
    pub fn create_render_pipeline(&self, desc: &RenderPipelineDescriptor<'_>) -> RenderPipeline {
        let pipeline = self.inner.create_render_pipeline(desc);
        RenderPipeline { inner: pipeline }
    }

    /// Creates a mesh shader based [`RenderPipeline`].
    #[must_use]
    pub fn create_mesh_pipeline(&self, desc: &MeshPipelineDescriptor<'_>) -> RenderPipeline {
        let pipeline = self.inner.create_mesh_pipeline(desc);
        RenderPipeline { inner: pipeline }
    }

    /// Creates a [`ComputePipeline`].
    #[must_use]
    pub fn create_compute_pipeline(&self, desc: &ComputePipelineDescriptor<'_>) -> ComputePipeline {
        let pipeline = self.inner.create_compute_pipeline(desc);
        ComputePipeline { inner: pipeline }
    }

    /// Creates a [`Buffer`].
    #[must_use]
    pub fn create_buffer(&self, desc: &BufferDescriptor<'_>) -> Buffer {
        let map_context = MapContext::new(desc.mapped_at_creation.then_some(0..desc.size));

        let buffer = self.inner.create_buffer(desc);

        Buffer {
            inner: buffer,
            map_context: Arc::new(Mutex::new(map_context)),
            size: desc.size,
            usage: desc.usage,
        }
    }

    /// Creates a new [`Texture`].
    ///
    /// `desc` specifies the general format of the texture.
    #[must_use]
    pub fn create_texture(&self, desc: &TextureDescriptor<'_>) -> Texture {
        let texture = self.inner.create_texture(desc);

        Texture {
            inner: texture,
            descriptor: TextureDescriptor {
                label: None,
                view_formats: &[],
                ..desc.clone()
            },
        }
    }

    /// Creates a [`Texture`] from a wgpu-hal Texture.
    ///
    /// # Types
    ///
    /// The type of `A::Texture` depends on the backend:
    ///
    #[doc = crate::macros::hal_type_vulkan!("Texture")]
    #[doc = crate::macros::hal_type_metal!("Texture")]
    #[doc = crate::macros::hal_type_dx12!("Texture")]
    #[doc = crate::macros::hal_type_gles!("Texture")]
    ///
    /// # Safety
    ///
    /// - `hal_texture` must be created from this device internal handle
    /// - `hal_texture` must be created respecting `desc`
    /// - `hal_texture` must be initialized
    #[cfg(wgpu_core)]
    #[must_use]
    pub unsafe fn create_texture_from_hal<A: hal::Api>(
        &self,
        hal_texture: A::Texture,
        desc: &TextureDescriptor<'_>,
    ) -> Texture {
        let texture = unsafe {
            let core_device = self.inner.as_core();
            core_device
                .context
                .create_texture_from_hal::<A>(hal_texture, core_device, desc)
        };
        Texture {
            inner: texture.into(),
            descriptor: TextureDescriptor {
                label: None,
                view_formats: &[],
                ..desc.clone()
            },
        }
    }

    /// Creates a new [`ExternalTexture`].
    #[must_use]
    pub fn create_external_texture(
        &self,
        desc: &ExternalTextureDescriptor<'_>,
        planes: &[&TextureView],
    ) -> ExternalTexture {
        let external_texture = self.inner.create_external_texture(desc, planes);

        ExternalTexture {
            inner: external_texture,
        }
    }

    /// Creates a [`Buffer`] from a wgpu-hal Buffer.
    ///
    /// # Types
    ///
    /// The type of `A::Buffer` depends on the backend:
    ///
    #[doc = crate::macros::hal_type_vulkan!("Buffer")]
    #[doc = crate::macros::hal_type_metal!("Buffer")]
    #[doc = crate::macros::hal_type_dx12!("Buffer")]
    #[doc = crate::macros::hal_type_gles!("Buffer")]
    ///
    /// # Safety
    ///
    /// - `hal_buffer` must be created from this device internal handle
    /// - `hal_buffer` must be created respecting `desc`
    /// - `hal_buffer` must be initialized
    /// - `hal_buffer` must not have zero size
    #[cfg(wgpu_core)]
    #[must_use]
    pub unsafe fn create_buffer_from_hal<A: hal::Api>(
        &self,
        hal_buffer: A::Buffer,
        desc: &BufferDescriptor<'_>,
    ) -> Buffer {
        let map_context = MapContext::new(desc.mapped_at_creation.then_some(0..desc.size));

        let buffer = unsafe {
            let core_device = self.inner.as_core();
            core_device
                .context
                .create_buffer_from_hal::<A>(hal_buffer, core_device, desc)
        };

        Buffer {
            inner: buffer.into(),
            map_context: Arc::new(Mutex::new(map_context)),
            size: desc.size,
            usage: desc.usage,
        }
    }

    /// Creates a new [`Sampler`].
    ///
    /// `desc` specifies the behavior of the sampler.
    #[must_use]
    pub fn create_sampler(&self, desc: &SamplerDescriptor<'_>) -> Sampler {
        let sampler = self.inner.create_sampler(desc);
        Sampler { inner: sampler }
    }

    /// Creates a new [`QuerySet`].
    #[must_use]
    pub fn create_query_set(&self, desc: &QuerySetDescriptor<'_>) -> QuerySet {
        let query_set = self.inner.create_query_set(desc);
        QuerySet { inner: query_set }
    }

    /// Set a callback which will be called for all errors that are not handled in error scopes.
    pub fn on_uncaptured_error(&self, handler: Arc<dyn UncapturedErrorHandler>) {
        self.inner.on_uncaptured_error(handler)
    }

    /// Push an error scope on this device's thread-local error scope
    /// stack. All operations on this device, or on resources created
    /// from this device, will have their errors captured by this scope
    /// until the scope is popped.
    ///
    /// Scopes must be popped in reverse order to their creation. If
    /// a guard is dropped without being `pop()`ped, the scope will be
    /// popped, and the captured errors will be dropped.
    ///
    /// Multiple error scopes may be active at one time, forming a stack.
    /// Each error will be reported to the inner-most scope that matches
    /// its filter.
    ///
    /// With the `std` feature enabled, this stack is **thread-local**.
    /// Without, this is **global** to all threads.
    ///
    /// ```rust
    /// # async move {
    /// # let device: wgpu::Device = unreachable!();
    /// let error_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    ///
    /// // ...
    /// // do work that may produce validation errors
    /// // ...
    ///
    /// // pop the error scope and get a future for the result
    /// let error_future = error_scope.pop();
    ///
    /// // await the future to get the error, if any
    /// let error = error_future.await;
    /// # };
    /// ```
    pub fn push_error_scope(&self, filter: ErrorFilter) -> ErrorScopeGuard {
        let index = self.inner.push_error_scope(filter);
        ErrorScopeGuard {
            device: self.inner.clone(),
            index,
            popped: false,
            _phantom: PhantomData,
        }
    }

    /// Starts a capture in the attached graphics debugger.
    ///
    /// This behaves differently depending on which graphics debugger is attached:
    ///
    /// - Renderdoc: Calls [`StartFrameCapture(device, NULL)`][rd].
    /// - Xcode: Creates a capture with [`MTLCaptureManager`][xcode].
    /// - None: No action is taken.
    ///
    /// # Safety
    ///
    /// - There should not be any other captures currently active.
    /// - All other safety rules are defined by the graphics debugger, see the
    ///   documentation for the specific debugger.
    /// - In general, graphics debuggers can easily cause crashes, so this isn't
    ///   ever guaranteed to be sound.
    ///
    /// # Tips
    ///
    /// - Debuggers need to capture both the recording of the commands and the
    ///   submission of the commands to the GPU. Try to wrap all of your
    ///   gpu work in a capture.
    /// - If you encounter issues, try waiting for the GPU to finish all work
    ///   before stopping the capture.
    ///
    /// [rd]: https://renderdoc.org/docs/in_application_api.html#_CPPv417StartFrameCapture23RENDERDOC_DevicePointer22RENDERDOC_WindowHandle
    /// [xcode]: https://developer.apple.com/documentation/metal/mtlcapturemanager
    #[doc(alias = "start_renderdoc_capture")]
    #[doc(alias = "start_xcode_capture")]
    pub unsafe fn start_graphics_debugger_capture(&self) {
        unsafe { self.inner.start_graphics_debugger_capture() }
    }

    /// Stops the current capture in the attached graphics debugger.
    ///
    /// This behaves differently depending on which graphics debugger is attached:
    ///
    /// - Renderdoc: Calls [`EndFrameCapture(device, NULL)`][rd].
    /// - Xcode: Stops the capture with [`MTLCaptureManager`][xcode].
    /// - None: No action is taken.
    ///
    /// # Safety
    ///
    /// - There should be a capture currently active.
    /// - All other safety rules are defined by the graphics debugger, see the
    ///   documentation for the specific debugger.
    /// - In general, graphics debuggers can easily cause crashes, so this isn't
    ///   ever guaranteed to be sound.
    ///
    /// # Tips
    ///
    /// - If you encounter issues, try to submit all work to the GPU, and waiting
    ///   for that work to finish before stopping the capture.
    ///
    /// [rd]: https://renderdoc.org/docs/in_application_api.html#_CPPv415EndFrameCapture23RENDERDOC_DevicePointer22RENDERDOC_WindowHandle
    /// [xcode]: https://developer.apple.com/documentation/metal/mtlcapturemanager
    #[doc(alias = "stop_renderdoc_capture")]
    #[doc(alias = "stop_xcode_capture")]
    pub unsafe fn stop_graphics_debugger_capture(&self) {
        unsafe { self.inner.stop_graphics_debugger_capture() }
    }

    /// Query internal counters from the native backend for debugging purposes.
    ///
    /// Some backends may not set all counters, or may not set any counter at all.
    /// The `counters` cargo feature must be enabled for any counter to be set.
    ///
    /// If a counter is not set, its contains its default value (zero).
    #[must_use]
    pub fn get_internal_counters(&self) -> wgt::InternalCounters {
        self.inner.get_internal_counters()
    }

    /// Generate an GPU memory allocation report if the underlying backend supports it.
    ///
    /// Backends that do not support producing these reports return `None`. A backend may
    /// Support it and still return `None` if it is not using performing sub-allocation,
    /// for example as a workaround for driver issues.
    #[must_use]
    pub fn generate_allocator_report(&self) -> Option<wgt::AllocatorReport> {
        self.inner.generate_allocator_report()
    }

    /// Get the [`wgpu_hal`] device from this `Device`.
    ///
    /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
    /// and pass that struct to the to the `A` type parameter.
    ///
    /// Returns a guard that dereferences to the type of the hal backend
    /// which implements [`A::Device`].
    ///
    /// # Types
    ///
    /// The returned type depends on the backend:
    ///
    #[doc = crate::macros::hal_type_vulkan!("Device")]
    #[doc = crate::macros::hal_type_metal!("Device")]
    #[doc = crate::macros::hal_type_dx12!("Device")]
    #[doc = crate::macros::hal_type_gles!("Device")]
    ///
    /// # Errors
    ///
    /// This method will return None if:
    /// - The device is not from the backend specified by `A`.
    /// - The device is from the `webgpu` or `custom` backend.
    ///
    /// # Safety
    ///
    /// - The returned resource must not be destroyed unless the guard
    ///   is the last reference to it and it is not in use by the GPU.
    ///   The guard and handle may be dropped at any time however.
    /// - All the safety requirements of wgpu-hal must be upheld.
    ///
    /// [`A::Device`]: hal::Api::Device
    #[cfg(wgpu_core)]
    pub unsafe fn as_hal<A: hal::Api>(
        &self,
    ) -> Option<impl Deref<Target = A::Device> + WasmNotSendSync> {
        let device = self.inner.as_core_opt()?;
        unsafe { device.context.device_as_hal::<A>(device) }
    }

    /// Destroy this device.
    pub fn destroy(&self) {
        self.inner.destroy()
    }

    /// Set a DeviceLostCallback on this device.
    pub fn set_device_lost_callback(
        &self,
        callback: impl Fn(DeviceLostReason, String) + Send + 'static,
    ) {
        self.inner.set_device_lost_callback(Box::new(callback))
    }

    /// Create a [`PipelineCache`] with initial data
    ///
    /// This can be passed to [`Device::create_compute_pipeline`]
    /// and [`Device::create_render_pipeline`] to either accelerate these
    /// or add the cache results from those.
    ///
    /// # Safety
    ///
    /// If the `data` field of `desc` is set, it must have previously been returned from a call
    /// to [`PipelineCache::get_data`][^saving]. This `data` will only be used if it came
    /// from an adapter with the same [`util::pipeline_cache_key`].
    /// This *is* compatible across wgpu versions, as any data format change will
    /// be accounted for.
    ///
    /// It is *not* supported to bring caches from previous direct uses of backend APIs
    /// into this method.
    ///
    /// # Errors
    ///
    /// Returns an error value if:
    ///  * the [`PIPELINE_CACHE`](wgt::Features::PIPELINE_CACHE) feature is not enabled
    ///  * this device is invalid; or
    ///  * the device is out of memory
    ///
    /// This method also returns an error value if:
    ///  * The `fallback` field on `desc` is false; and
    ///  * the `data` provided would not be used[^data_not_used]
    ///
    /// If an error value is used in subsequent calls, default caching will be used.
    ///
    /// [^saving]: We do recognise that saving this data to disk means this condition
    /// is impossible to fully prove. Consider the risks for your own application in this case.
    ///
    /// [^data_not_used]: This data may be not used if: the data was produced by a prior
    /// version of wgpu; or was created for an incompatible adapter, or there was a GPU driver
    /// update. In some cases, the data might not be used and a real value is returned,
    /// this is left to the discretion of GPU drivers.
    #[must_use]
    pub unsafe fn create_pipeline_cache(
        &self,
        desc: &PipelineCacheDescriptor<'_>,
    ) -> PipelineCache {
        let cache = unsafe { self.inner.create_pipeline_cache(desc) };
        PipelineCache { inner: cache }
    }
}

/// [`Features::EXPERIMENTAL_RAY_QUERY`] must be enabled on the device in order to call these functions.
impl Device {
    /// Create a bottom level acceleration structure, used inside a top level acceleration structure for ray tracing.
    /// - `desc`: The descriptor of the acceleration structure.
    /// - `sizes`: Size descriptor limiting what can be built into the acceleration structure.
    ///
    /// # Validation
    /// If any of the following is not satisfied a validation error is generated
    ///
    /// The device ***must*** have [`Features::EXPERIMENTAL_RAY_QUERY`] enabled.
    /// if `sizes` is [`BlasGeometrySizeDescriptors::Triangles`] then the following must be satisfied
    /// - For every geometry descriptor (for the purposes this is called `geo_desc`) of `sizes.descriptors` the following must be satisfied:
    ///     - `geo_desc.vertex_format` must be within allowed formats (allowed formats for a given feature set
    ///       may be queried with [`Features::allowed_vertex_formats_for_blas`]).
    ///     - Both or neither of `geo_desc.index_format` and `geo_desc.index_count` must be provided.
    ///
    /// [`Features::EXPERIMENTAL_RAY_QUERY`]: wgt::Features::EXPERIMENTAL_RAY_QUERY
    /// [`Features::allowed_vertex_formats_for_blas`]: wgt::Features::allowed_vertex_formats_for_blas
    #[must_use]
    pub fn create_blas(
        &self,
        desc: &CreateBlasDescriptor<'_>,
        sizes: BlasGeometrySizeDescriptors,
    ) -> Blas {
        let (handle, blas) = self.inner.create_blas(desc, sizes);

        Blas {
            inner: blas,
            handle,
        }
    }

    /// Create a top level acceleration structure, used for ray tracing.
    /// - `desc`: The descriptor of the acceleration structure.
    ///
    /// # Validation
    /// If any of the following is not satisfied a validation error is generated
    ///
    /// The device ***must*** have [`Features::EXPERIMENTAL_RAY_QUERY`] enabled.
    ///
    /// [`Features::EXPERIMENTAL_RAY_QUERY`]: wgt::Features::EXPERIMENTAL_RAY_QUERY
    #[must_use]
    pub fn create_tlas(&self, desc: &CreateTlasDescriptor<'_>) -> Tlas {
        let tlas = self.inner.create_tlas(desc);

        Tlas {
            inner: tlas,
            instances: vec![None; desc.max_instances as usize],
            lowest_unmodified: 0,
        }
    }
}

/// Requesting a device from an [`Adapter`] failed.
#[derive(Clone, Debug)]
pub struct RequestDeviceError {
    pub(crate) inner: RequestDeviceErrorKind,
}
#[derive(Clone, Debug)]
pub(crate) enum RequestDeviceErrorKind {
    /// Error from [`wgpu_core`].
    // must match dependency cfg
    #[cfg(wgpu_core)]
    Core(wgc::instance::RequestDeviceError),

    /// Error from web API that was called by `wgpu` to request a device.
    ///
    /// (This is currently never used by the webgl backend, but it could be.)
    #[cfg(webgpu)]
    WebGpu(String),
}

static_assertions::assert_impl_all!(RequestDeviceError: Send, Sync);

impl fmt::Display for RequestDeviceError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            #[cfg(wgpu_core)]
            RequestDeviceErrorKind::Core(error) => error.fmt(_f),
            #[cfg(webgpu)]
            RequestDeviceErrorKind::WebGpu(error) => {
                write!(_f, "{error}")
            }
            #[cfg(not(any(webgpu, wgpu_core)))]
            _ => unimplemented!("unknown `RequestDeviceErrorKind`"),
        }
    }
}

impl error::Error for RequestDeviceError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.inner {
            #[cfg(wgpu_core)]
            RequestDeviceErrorKind::Core(error) => error.source(),
            #[cfg(webgpu)]
            RequestDeviceErrorKind::WebGpu(_) => None,
            #[cfg(not(any(webgpu, wgpu_core)))]
            _ => unimplemented!("unknown `RequestDeviceErrorKind`"),
        }
    }
}

#[cfg(wgpu_core)]
impl From<wgc::instance::RequestDeviceError> for RequestDeviceError {
    fn from(error: wgc::instance::RequestDeviceError) -> Self {
        Self {
            inner: RequestDeviceErrorKind::Core(error),
        }
    }
}

/// The callback of [`Device::on_uncaptured_error()`].
///
/// It must be a function with this signature.
pub trait UncapturedErrorHandler: Fn(Error) + Send + Sync + 'static {}
impl<T> UncapturedErrorHandler for T where T: Fn(Error) + Send + Sync + 'static {}

/// Kinds of [`Error`]s a [`Device::push_error_scope()`] may be configured to catch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum ErrorFilter {
    /// Catch only out-of-memory errors.
    OutOfMemory,
    /// Catch only validation errors.
    Validation,
    /// Catch only internal errors.
    Internal,
}
static_assertions::assert_impl_all!(ErrorFilter: Send, Sync);

/// Lower level source of the error.
///
/// `Send + Sync` varies depending on configuration.
#[cfg(send_sync)]
#[cfg_attr(docsrs, doc(cfg(all())))]
pub type ErrorSource = Box<dyn error::Error + Send + Sync + 'static>;
/// Lower level source of the error.
///
/// `Send + Sync` varies depending on configuration.
#[cfg(not(send_sync))]
#[cfg_attr(docsrs, doc(cfg(all())))]
pub type ErrorSource = Box<dyn error::Error + 'static>;

/// Errors resulting from usage of GPU APIs.
///
/// By default, errors translate into panics. Depending on the backend and circumstances,
/// errors may occur synchronously or asynchronously. When errors need to be handled, use
/// [`Device::push_error_scope()`] or [`Device::on_uncaptured_error()`].
#[derive(Debug)]
pub enum Error {
    /// Out of memory.
    OutOfMemory {
        /// Lower level source of the error.
        source: ErrorSource,
    },
    /// Validation error, signifying a bug in code or data provided to `wgpu`.
    Validation {
        /// Lower level source of the error.
        source: ErrorSource,
        /// Description of the validation error.
        description: String,
    },
    /// Internal error. Used for signalling any failures not explicitly expected by WebGPU.
    ///
    /// These could be due to internal implementation or system limits being reached.
    Internal {
        /// Lower level source of the error.
        source: ErrorSource,
        /// Description of the internal GPU error.
        description: String,
    },
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(Error: Send, Sync);

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::OutOfMemory { source } => Some(source.as_ref()),
            Error::Validation { source, .. } => Some(source.as_ref()),
            Error::Internal { source, .. } => Some(source.as_ref()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfMemory { .. } => f.write_str("Out of Memory"),
            Error::Validation { description, .. } => f.write_str(description),
            Error::Internal { description, .. } => f.write_str(description),
        }
    }
}

/// Guard for an error scope pushed with [`Device::push_error_scope()`].
///
/// Call [`pop()`] to pop the scope and get a future for the result. If
/// the guard is dropped without being popped explicitly, the scope will still be popped,
/// and the captured errors will be dropped.
///
/// This guard is neither `Send` nor `Sync`, as error scopes are handled
/// on a per-thread basis when the `std` feature is enabled.
///
/// [`pop()`]: ErrorScopeGuard::pop
#[must_use = "Error scopes must be explicitly popped to retrieve errors they catch"]
pub struct ErrorScopeGuard {
    device: dispatch::DispatchDevice,
    index: u32,
    popped: bool,
    // Ensure the guard is !Send and !Sync
    _phantom: PhantomData<*mut ()>,
}

static_assertions::assert_not_impl_any!(ErrorScopeGuard: Send, Sync);

impl ErrorScopeGuard {
    /// Pops the error scope.
    ///
    /// Returns a future which resolves to the error captured by this scope, if any.
    /// The pop takes effect immediately; the future does not need to be awaited before doing work that is outside of this error scope.
    pub fn pop(mut self) -> impl Future<Output = Option<Error>> + WasmNotSend {
        self.popped = true;
        self.device.pop_error_scope(self.index)
    }
}

impl Drop for ErrorScopeGuard {
    fn drop(&mut self) {
        if !self.popped {
            drop(self.device.pop_error_scope(self.index));
        }
    }
}
