//! Types for dealing with Instances.

use crate::{link_to_wgpu_docs, Backends};

#[cfg(doc)]
use crate::{Backend, DownlevelFlags};

/// A [`raw_window_handle::HasDisplayHandle`] that can be shared across threads and has no borrows.
///
/// This blanket trait is automatically implemented for all objects that qualify.
pub trait WgpuHasDisplayHandle:
    raw_window_handle::HasDisplayHandle + core::fmt::Debug + Send + Sync + 'static
{
}
impl<T: raw_window_handle::HasDisplayHandle + core::fmt::Debug + Send + Sync + 'static>
    WgpuHasDisplayHandle for T
{
}

/// Options for creating an instance.
///
/// If you want to allow control of instance settings via environment variables, call any of the
/// `*from_env()` functions or [`InstanceDescriptor::with_env()`]. Each type within this descriptor
/// has its own equivalent methods, so you can select which options you want to expose to influence
/// from the environment.
#[derive(Debug)]
pub struct InstanceDescriptor {
    /// Which [`Backends`] to enable.
    ///
    /// [`Backends::BROWSER_WEBGPU`] has an additional effect:
    /// If it is set and a [`navigator.gpu`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/gpu)
    /// object is present, this instance will *only* be able to create WebGPU adapters.
    ///
    /// ⚠️ On some browsers this check is insufficient to determine whether WebGPU is supported,
    /// as the browser may define the `navigator.gpu` object, but be unable to create any WebGPU adapters.
    /// For targeting _both_ WebGPU & WebGL, it is recommended to use [`crate::util::new_instance_with_webgpu_detection`](../wgpu/util/fn.new_instance_with_webgpu_detection.html).
    ///
    /// If you instead want to force use of WebGL, either disable the `webgpu` compile-time feature
    /// or don't include the [`Backends::BROWSER_WEBGPU`] flag in this field.
    /// If it is set and WebGPU support is *not* detected, the instance will use `wgpu-core`
    /// to create adapters, meaning that if the `webgl` feature is enabled, it is able to create
    /// a WebGL adapter.
    pub backends: Backends,
    /// Flags to tune the behavior of the instance.
    pub flags: InstanceFlags,
    /// Memory budget thresholds used by some backends.
    pub memory_budget_thresholds: MemoryBudgetThresholds,
    /// Options the control the behavior of specific backends.
    pub backend_options: crate::BackendOptions,
    /// System platform or compositor connection to connect this `Instance` to.
    ///
    /// If not [`None`], it is invalid to pass a different [`raw_window_handle::HasDisplayHandle`] to `create_surface()`.
    ///
    /// - On GLES, this is required when intending to present on the platform, especially for Wayland.
    /// - On Vulkan, Metal and Dx12, this is currently unused.
    ///
    /// When used with `winit`, callers are expected to pass its [`OwnedDisplayHandle`] (created from
    /// the `EventLoop`) here.
    ///
    /// [`OwnedDisplayHandle`]: https://docs.rs/winit/latest/winit/event_loop/struct.OwnedDisplayHandle.html
    pub display: Option<alloc::boxed::Box<dyn WgpuHasDisplayHandle>>,
}

impl InstanceDescriptor {
    /// The default instance options, without display handle.
    #[must_use]
    pub fn new_without_display_handle() -> Self {
        Self {
            backends: Default::default(),
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        }
    }

    /// The default instance options, with display handle.
    #[must_use]
    pub fn new_with_display_handle(display: alloc::boxed::Box<dyn WgpuHasDisplayHandle>) -> Self {
        Self::new_without_display_handle().with_display_handle(display)
    }

    /// Choose instance options entirely from environment variables.
    ///
    /// This is equivalent to calling `from_env` on every field.
    #[must_use]
    pub fn new_without_display_handle_from_env() -> Self {
        Self::new_without_display_handle().with_env()
    }

    /// Choose instance options entirely from environment variables, while including a display handle.
    ///
    /// This is equivalent to calling `from_env` on every field.
    #[must_use]
    pub fn new_with_display_handle_from_env(
        display: alloc::boxed::Box<dyn WgpuHasDisplayHandle>,
    ) -> Self {
        // Self::new_without_display_handle_from_env().with_display_handle(display)
        Self::new_with_display_handle(display).with_env()
    }

    /// Takes the given options, modifies them based on the environment variables, and returns the result.
    ///
    /// This is equivalent to calling `with_env` on every field.
    #[must_use]
    pub fn with_env(self) -> Self {
        let backends = self.backends.with_env();
        let flags = self.flags.with_env();
        let backend_options = self.backend_options.with_env();
        Self {
            backends,
            flags,
            memory_budget_thresholds: MemoryBudgetThresholds::default(),
            backend_options,
            display: self.display,
        }
    }

    /// Appends the given `display` object to the descriptor.
    #[must_use]
    pub fn with_display_handle(self, display: alloc::boxed::Box<dyn WgpuHasDisplayHandle>) -> Self {
        Self {
            display: Some(display),
            ..self
        }
    }
}

bitflags::bitflags! {
    /// Instance debugging flags.
    ///
    /// These are not part of the WebGPU standard.
    ///
    /// Defaults to enabling debugging-related flags if the build configuration has `debug_assertions`.
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct InstanceFlags: u32 {
        /// Generate debug information in shaders and objects.
        ///
        /// When `Self::from_env()` is used takes value from `WGPU_DEBUG` environment variable.
        const DEBUG = 1 << 0;
        /// Enable validation in the backend API, if possible:
        ///
        /// - On the Direct3D `dx12` backend, this calls [`ID3D12Debug::EnableDebugLayer`][dx12].
        ///
        /// - On the Vulkan backend, this enables the [Vulkan Validation Layer][vvl].
        ///
        /// - On the `gles` backend driving Windows OpenGL, this enables [debug
        ///   output][gl:do], effectively calling `glEnable(GL_DEBUG_OUTPUT)`.
        ///
        /// - On non-Windows `gles` backends, this calls
        ///   [`eglDebugMessageControlKHR`][gl:dm] to enable all debugging messages.
        ///   If the GLES implementation is ANGLE running on Vulkan, this also
        ///   enables the Vulkan validation layers by setting
        ///   [`EGL_PLATFORM_ANGLE_DEBUG_LAYERS_ENABLED`][gl:av].
        ///
        /// When `Self::from_env()` is used, this bit is set if the `WGPU_VALIDATION`
        /// environment variable has any value but "0".
        ///
        /// [dx12]: https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug-enabledebuglayer
        /// [vvl]: https://github.com/KhronosGroup/Vulkan-ValidationLayers
        /// [gl:dm]: https://registry.khronos.org/EGL/extensions/KHR/EGL_KHR_debug.txt
        /// [gl:do]: https://www.khronos.org/opengl/wiki/Debug_Output
        /// [gl:av]: https://chromium.googlesource.com/angle/angle/+/HEAD/extensions/EGL_ANGLE_platform_angle.txt
        const VALIDATION = 1 << 1;
        /// Don't pass labels to wgpu-hal.
        ///
        /// When `Self::from_env()` is used takes value from `WGPU_DISCARD_HAL_LABELS` environment variable.
        const DISCARD_HAL_LABELS = 1 << 2;
        /// Whether wgpu should expose adapters that run on top of non-compliant adapters.
        ///
        /// Turning this on might mean that some of the functionality provided by the wgpu
        /// adapter/device is not working or is broken. It could be that all the functionality
        /// wgpu currently exposes works but we can't tell for sure since we have no additional
        /// transparency into what is working and what is not on the underlying adapter.
        ///
        /// This mainly applies to a Vulkan driver's compliance version. If the major compliance version
        /// is `0`, then the driver is ignored. This flag allows that driver to be enabled for testing.
        ///
        /// When `Self::from_env()` is used takes value from `WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER` environment variable.
        const ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER = 1 << 3;
        /// Enable GPU-based validation. Implies [`Self::VALIDATION`]. Currently, this only changes
        /// behavior on the DX12 and Vulkan backends.
        ///
        /// Supported platforms:
        ///
        /// - D3D12; called ["GPU-based validation", or
        ///   "GBV"](https://web.archive.org/web/20230206120404/https://learn.microsoft.com/en-us/windows/win32/direct3d12/using-d3d12-debug-layer-gpu-based-validation)
        /// - Vulkan, via the `VK_LAYER_KHRONOS_validation` layer; called ["GPU-Assisted
        ///   Validation"](https://github.com/KhronosGroup/Vulkan-ValidationLayers/blob/e45aeb85079e0835694cb8f03e6681fd18ae72c9/docs/gpu_validation.md#gpu-assisted-validation)
        ///
        /// When `Self::from_env()` is used takes value from `WGPU_GPU_BASED_VALIDATION` environment variable.
        const GPU_BASED_VALIDATION = 1 << 4;

        /// Validate indirect buffer content prior to issuing indirect draws/dispatches.
        ///
        /// This validation will transform indirect calls into no-ops if they are not valid:
        ///
        /// - When calling `dispatch_workgroups_indirect`, all 3 indirect arguments encoded in the buffer
        /// must be less than the `max_compute_workgroups_per_dimension` device limit.
        /// - When calling `draw_indirect`/`draw_indexed_indirect`/`multi_draw_indirect`/`multi_draw_indexed_indirect`:
        ///   - If `Features::INDIRECT_FIRST_INSTANCE` is not enabled on the device, the `first_instance` indirect argument must be 0.
        ///   - The `first_instance` & `instance_count` indirect arguments must form a range that fits within all bound vertex buffers with `step_mode` set to `Instance`.
        /// - When calling `draw_indirect`/`multi_draw_indirect`:
        ///   - The `first_vertex` & `vertex_count` indirect arguments must form a range that fits within all bound vertex buffers with `step_mode` set to `Vertex`.
        /// - When calling `draw_indexed_indirect`/`multi_draw_indexed_indirect`:
        ///   - The `first_index` & `index_count` indirect arguments must form a range that fits within the bound index buffer.
        ///
        /// __Behavior is undefined if this validation is disabled and the rules above are not satisfied.__
        ///
        /// Disabling this will also cause the following built-ins to not report the right values on the D3D12 backend:
        ///
        /// - the 3 components of `@builtin(num_workgroups)` will be 0
        /// - the value of `@builtin(vertex_index)` will not take into account the value of the `first_vertex`/`base_vertex` argument present in the indirect buffer
        /// - the value of `@builtin(instance_index)` will not take into account the value of the `first_instance` argument present in the indirect buffer
        ///
        /// When `Self::from_env()` is used takes value from `WGPU_VALIDATION_INDIRECT_CALL` environment variable.
        const VALIDATION_INDIRECT_CALL = 1 << 5;

        /// Enable automatic timestamp normalization. This means that in [`CommandEncoder::resolve_query_set`][rqs],
        /// the timestamps will automatically be normalized to be in nanoseconds instead of the raw timestamp values.
        ///
        /// This is disabled by default because it introduces a compute shader into the resolution of query sets.
        ///
        /// This can be useful for users that need to read timestamps on the gpu, as the normalization
        /// can be a hassle to do manually. When this is enabled, the timestamp period returned by the queue
        /// will always be `1.0`.
        ///
        #[doc = link_to_wgpu_docs!(["rqs"]: "struct.CommandEncoder.html#method.resolve_query_set")]
        const AUTOMATIC_TIMESTAMP_NORMALIZATION = 1 << 6;
    }
}

impl Default for InstanceFlags {
    fn default() -> Self {
        Self::from_build_config()
    }
}

impl InstanceFlags {
    /// Enable recommended debugging and validation flags.
    #[must_use]
    pub fn debugging() -> Self {
        InstanceFlags::DEBUG | InstanceFlags::VALIDATION | InstanceFlags::VALIDATION_INDIRECT_CALL
    }

    /// Enable advanced debugging and validation flags (potentially very slow).
    #[must_use]
    pub fn advanced_debugging() -> Self {
        Self::debugging() | InstanceFlags::GPU_BASED_VALIDATION
    }

    /// Infer decent defaults from the build type.
    ///
    /// If `cfg!(debug_assertions)` is true, then this returns [`Self::debugging()`].
    /// Otherwise, it returns [`Self::empty()`].
    #[must_use]
    pub fn from_build_config() -> Self {
        if cfg!(debug_assertions) {
            return InstanceFlags::debugging();
        }

        InstanceFlags::VALIDATION_INDIRECT_CALL
    }

    /// Derive defaults from environment variables. See [`Self::with_env()`] for more information.
    #[must_use]
    pub fn from_env_or_default() -> Self {
        Self::default().with_env()
    }

    /// Takes the given flags, modifies them based on the environment variables, and returns the result.
    ///
    /// - If an environment variable is set to anything but "0", the corresponding flag is set.
    /// - If the value is "0", the flag is unset.
    /// - If the environment variable is not present, then the flag retains its initial value.
    ///
    /// For example `let flags = InstanceFlags::debugging().with_env();` with `WGPU_VALIDATION=0`
    /// does not contain [`InstanceFlags::VALIDATION`].
    ///
    /// The environment variables are named after the flags prefixed with "WGPU_". For example:
    /// - `WGPU_DEBUG`
    /// - `WGPU_VALIDATION`
    /// - `WGPU_DISCARD_HAL_LABELS`
    /// - `WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER`
    /// - `WGPU_GPU_BASED_VALIDATION`
    /// - `WGPU_VALIDATION_INDIRECT_CALL`
    #[must_use]
    pub fn with_env(mut self) -> Self {
        fn env(key: &str) -> Option<bool> {
            crate::env::var(key).map(|s| match s.as_str() {
                "0" => false,
                _ => true,
            })
        }

        if let Some(bit) = env("WGPU_VALIDATION") {
            self.set(Self::VALIDATION, bit);
        }

        if let Some(bit) = env("WGPU_DEBUG") {
            self.set(Self::DEBUG, bit);
        }
        if let Some(bit) = env("WGPU_DISCARD_HAL_LABELS") {
            self.set(Self::DISCARD_HAL_LABELS, bit);
        }
        if let Some(bit) = env("WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER") {
            self.set(Self::ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER, bit);
        }
        if let Some(bit) = env("WGPU_GPU_BASED_VALIDATION") {
            self.set(Self::GPU_BASED_VALIDATION, bit);
        }
        if let Some(bit) = env("WGPU_VALIDATION_INDIRECT_CALL") {
            self.set(Self::VALIDATION_INDIRECT_CALL, bit);
        }

        self
    }
}

/// Memory budget thresholds used by backends to try to avoid high memory pressure situations.
///
/// Currently only the D3D12 and (optionally) Vulkan backends support these options.
#[derive(Default, Clone, Debug, Copy)]
pub struct MemoryBudgetThresholds {
    /// Threshold at which texture, buffer, query set and acceleration structure creation will start to return OOM errors.
    /// This is a percent of the memory budget reported by native APIs.
    ///
    /// If not specified, resource creation might still return OOM errors.
    pub for_resource_creation: Option<u8>,

    /// Threshold at which devices will become lost due to memory pressure.
    /// This is a percent of the memory budget reported by native APIs.
    ///
    /// If not specified, devices might still become lost due to memory pressure.
    pub for_device_loss: Option<u8>,
}
