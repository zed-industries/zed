use alloc::{vec, vec::Vec};

use crate::{link_to_wgpu_docs, link_to_wgpu_item, TextureFormat, TextureUsages};

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

/// Timing and queueing with which frames are actually displayed to the user.
///
/// Use this as part of a [`SurfaceConfiguration`] to control the behavior of
/// [`SurfaceTexture::present()`].
///
/// Some modes are only supported by some backends.
/// You can use one of the `Auto*` modes, [`Fifo`](Self::Fifo),
/// or choose one of the supported modes from [`SurfaceCapabilities::present_modes`].
///
#[doc = link_to_wgpu_docs!(["presented"]: "struct.SurfaceTexture.html#method.present")]
#[doc = link_to_wgpu_docs!(["`SurfaceTexture::present()`"]: "struct.SurfaceTexture.html#method.present")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PresentMode {
    /// Chooses the first supported mode out of:
    ///
    /// 1. [`FifoRelaxed`](Self::FifoRelaxed)
    /// 2. [`Fifo`](Self::Fifo)
    ///
    /// Because of the fallback behavior, this is supported everywhere.
    AutoVsync = 0,

    /// Chooses the first supported mode out of:
    ///
    /// 1. [`Immediate`](Self::Immediate)
    /// 2. [`Mailbox`](Self::Mailbox)
    /// 3. [`Fifo`](Self::Fifo)
    ///
    /// Because of the fallback behavior, this is supported everywhere.
    AutoNoVsync = 1,

    /// Presentation frames are kept in a First-In-First-Out queue approximately 3 frames
    /// long. Every vertical blanking period, the presentation engine will pop a frame
    /// off the queue to display. If there is no frame to display, it will present the same
    /// frame again until the next vblank.
    ///
    /// When a present command is executed on the GPU, the presented image is added on the queue.
    ///
    /// Calls to [`Surface::get_current_texture()`] will block until there is a spot in the queue.
    ///
    /// * **Tearing:** No tearing will be observed.
    /// * **Supported on**: All platforms.
    /// * **Also known as**: "Vsync On"
    ///
    /// This is the [default](Self::default) value for `PresentMode`.
    /// If you don't know what mode to choose, choose this mode.
    ///
    #[doc = link_to_wgpu_docs!(["`Surface::get_current_texture()`"]: "struct.Surface.html#method.get_current_texture")]
    #[default]
    Fifo = 2,

    /// Presentation frames are kept in a First-In-First-Out queue approximately 3 frames
    /// long. Every vertical blanking period, the presentation engine will pop a frame
    /// off the queue to display. If there is no frame to display, it will present the
    /// same frame until there is a frame in the queue. The moment there is a frame in the
    /// queue, it will immediately pop the frame off the queue.
    ///
    /// When a present command is executed on the GPU, the presented image is added on the queue.
    ///
    /// Calls to [`Surface::get_current_texture()`] will block until there is a spot in the queue.
    ///
    /// * **Tearing**:
    ///   Tearing will be observed if frames last more than one vblank as the front buffer.
    /// * **Supported on**: AMD on Vulkan.
    /// * **Also known as**: "Adaptive Vsync"
    ///
    #[doc = link_to_wgpu_docs!(["`Surface::get_current_texture()`"]: "struct.Surface.html#method.get_current_texture")]
    FifoRelaxed = 3,

    /// Presentation frames are not queued at all. The moment a present command
    /// is executed on the GPU, the presented image is swapped onto the front buffer
    /// immediately.
    ///
    /// * **Tearing**: Tearing can be observed.
    /// * **Supported on**: Most platforms except older DX12 and Wayland.
    /// * **Also known as**: "Vsync Off"
    Immediate = 4,

    /// Presentation frames are kept in a single-frame queue. Every vertical blanking period,
    /// the presentation engine will pop a frame from the queue. If there is no frame to display,
    /// it will present the same frame again until the next vblank.
    ///
    /// When a present command is executed on the GPU, the frame will be put into the queue.
    /// If there was already a frame in the queue, the new frame will _replace_ the old frame
    /// on the queue.
    ///
    /// * **Tearing**: No tearing will be observed.
    /// * **Supported on**: DX12 on Windows 10, NVidia on Vulkan and Wayland on Vulkan.
    /// * **Also known as**: "Fast Vsync"
    Mailbox = 5,
}

/// Specifies how the alpha channel of the textures should be handled during
/// compositing.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum CompositeAlphaMode {
    /// Chooses either `Opaque` or `Inherit` automatically, depending on the
    /// `alpha_mode` that the current surface can support.
    #[default]
    Auto = 0,
    /// The alpha channel, if it exists, of the textures is ignored in the
    /// compositing process. Instead, the textures is treated as if it has a
    /// constant alpha of 1.0.
    Opaque = 1,
    /// The alpha channel, if it exists, of the textures is respected in the
    /// compositing process. The non-alpha channels of the textures are
    /// expected to already be multiplied by the alpha channel by the
    /// application.
    PreMultiplied = 2,
    /// The alpha channel, if it exists, of the textures is respected in the
    /// compositing process. The non-alpha channels of the textures are not
    /// expected to already be multiplied by the alpha channel by the
    /// application; instead, the compositor will multiply the non-alpha
    /// channels of the texture by the alpha channel during compositing.
    PostMultiplied = 3,
    /// The alpha channel, if it exists, of the textures is unknown for processing
    /// during compositing. Instead, the application is responsible for setting
    /// the composite alpha blending mode using native WSI command. If not set,
    /// then a platform-specific default will be used.
    Inherit = 4,
}

/// Defines the capabilities of a given surface and adapter.
#[derive(Debug)]
pub struct SurfaceCapabilities {
    /// List of supported formats to use with the given adapter. The first format in the vector is preferred.
    ///
    /// Returns an empty vector if the surface is incompatible with the adapter.
    pub formats: Vec<TextureFormat>,
    /// List of supported presentation modes to use with the given adapter.
    ///
    /// Returns an empty vector if the surface is incompatible with the adapter.
    pub present_modes: Vec<PresentMode>,
    /// List of supported alpha modes to use with the given adapter.
    ///
    /// Will return at least one element, [`CompositeAlphaMode::Opaque`] or [`CompositeAlphaMode::Inherit`].
    pub alpha_modes: Vec<CompositeAlphaMode>,
    /// Bitflag of supported texture usages for the surface to use with the given adapter.
    ///
    /// The usage [`TextureUsages::RENDER_ATTACHMENT`] is guaranteed.
    pub usages: TextureUsages,
}

impl Default for SurfaceCapabilities {
    fn default() -> Self {
        Self {
            formats: Vec::new(),
            present_modes: Vec::new(),
            alpha_modes: vec![CompositeAlphaMode::Opaque],
            usages: TextureUsages::RENDER_ATTACHMENT,
        }
    }
}

/// Configures a [`Surface`] for presentation.
///
#[doc = link_to_wgpu_item!(struct Surface)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SurfaceConfiguration<V> {
    /// The usage of the swap chain. The only usage guaranteed to be supported is [`TextureUsages::RENDER_ATTACHMENT`].
    pub usage: TextureUsages,
    /// The texture format of the swap chain. The only formats that are guaranteed are
    /// [`TextureFormat::Bgra8Unorm`] and [`TextureFormat::Bgra8UnormSrgb`].
    pub format: TextureFormat,
    /// Width of the swap chain. Must be the same size as the surface, and nonzero.
    ///
    /// If this is not the same size as the underlying surface (e.g. if it is
    /// set once, and the window is later resized), the behaviour is defined
    /// but platform-specific, and may change in the future (currently macOS
    /// scales the surface, other platforms may do something else).
    pub width: u32,
    /// Height of the swap chain. Must be the same size as the surface, and nonzero.
    ///
    /// If this is not the same size as the underlying surface (e.g. if it is
    /// set once, and the window is later resized), the behaviour is defined
    /// but platform-specific, and may change in the future (currently macOS
    /// scales the surface, other platforms may do something else).
    pub height: u32,
    /// Presentation mode of the swap chain. Fifo is the only mode guaranteed to be supported.
    /// `FifoRelaxed`, `Immediate`, and `Mailbox` will crash if unsupported, while `AutoVsync` and
    /// `AutoNoVsync` will gracefully do a designed sets of fallbacks if their primary modes are
    /// unsupported.
    pub present_mode: PresentMode,
    /// Desired maximum number of monitor refreshes between a [`Surface::get_current_texture`] call and the
    /// texture being presented to the screen. This is sometimes called "Frames in Flight".
    ///
    /// Defaults to `2` when created via [`Surface::get_default_config`] as this is a reasonable default.
    ///
    /// This is ultimately a hint to the backend implementation and will always be clamped
    /// to the supported range.
    ///
    /// Typical values are `1` to `3`, but higher values are valid, though likely to be clamped.
    /// * Choose `1` to minimize latency above all else. This only gives a single monitor refresh for all of
    ///   the CPU and GPU work to complete. ⚠️ As a result of these short swapchains, the CPU and GPU
    ///   cannot run in parallel, prioritizing latency over throughput. For applications like GUIs doing
    ///   a small amount of GPU work each frame that need low latency, this is a reasonable choice.
    /// * Choose `2` for a balance between latency and throughput. The CPU and GPU both can each use
    ///   a full monitor refresh to do their computations. This is a reasonable default for most applications.
    /// * Choose `3` or higher to maximize throughput, sacrificing latency when the the CPU and GPU
    ///   are using less than a full monitor refresh each. For applications that use CPU-side pipelining
    ///   of frames this may be a reasonable choice. ⚠️ On 60hz displays the latency can be very noticeable.
    ///
    /// This maps to the backend in the following ways:
    /// - Vulkan: Number of frames in the swapchain is `desired_maximum_frame_latency + 1`,
    ///   clamped to the supported range.
    /// - DX12: Calls [`IDXGISwapChain2::SetMaximumFrameLatency(desired_maximum_frame_latency)`][SMFL].
    /// - Metal: Sets the `maximumDrawableCount` of the underlying `CAMetalLayer` to
    ///   `desired_maximum_frame_latency + 1`, clamped to the supported range.
    /// - OpenGL: Ignored
    ///
    /// It also has various subtle interactions with various present modes and APIs.
    /// - DX12 + Mailbox: Limits framerate to `desired_maximum_frame_latency * Monitor Hz` fps.
    /// - Vulkan/Metal + Mailbox: If this is set to `2`, limits framerate to `2 * Monitor Hz` fps. `3` or higher is unlimited.
    ///
    #[doc = link_to_wgpu_docs!(["`Surface::get_current_texture`"]: "struct.Surface.html#method.get_current_texture")]
    #[doc = link_to_wgpu_docs!(["`Surface::get_default_config`"]: "struct.Surface.html#method.get_default_config")]
    /// [SMFL]: https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-idxgiswapchain2-setmaximumframelatency
    pub desired_maximum_frame_latency: u32,
    /// Specifies how the alpha channel of the textures should be handled during compositing.
    pub alpha_mode: CompositeAlphaMode,
    /// Specifies what view formats will be allowed when calling `Texture::create_view` on the texture returned by `Surface::get_current_texture`.
    ///
    /// View formats of the same format as the texture are always allowed.
    ///
    /// Note: currently, only the srgb-ness is allowed to change. (ex: `Rgba8Unorm` texture + `Rgba8UnormSrgb` view)
    pub view_formats: V,
}

impl<V: Clone> SurfaceConfiguration<V> {
    /// Map `view_formats` of the texture descriptor into another.
    pub fn map_view_formats<M>(&self, fun: impl FnOnce(V) -> M) -> SurfaceConfiguration<M> {
        SurfaceConfiguration {
            usage: self.usage,
            format: self.format,
            width: self.width,
            height: self.height,
            present_mode: self.present_mode,
            desired_maximum_frame_latency: self.desired_maximum_frame_latency,
            alpha_mode: self.alpha_mode,
            view_formats: fun(self.view_formats.clone()),
        }
    }
}

/// Status of the received surface image.
#[repr(C)]
#[derive(Debug)]
pub enum SurfaceStatus {
    /// No issues.
    Good,
    /// The swap chain is operational, but it does no longer perfectly
    /// match the surface. A re-configuration is needed.
    Suboptimal,
    /// Unable to get the next frame, timed out.
    ///
    /// Try reconfiguring your surface.
    Timeout,
    /// The window is occluded (e.g. minimized or behind another window).
    ///
    /// Try again once the window is no longer occluded.
    Occluded,
    /// The surface under the swap chain has changed.
    ///
    /// Try reconfiguring your surface.
    Outdated,
    /// The surface under the swap chain is lost.
    Lost,
    /// `Surface::get_current_texture` has hit a validation error which was caught
    /// by a error scope.
    Validation,
}

/// Nanosecond timestamp used by the presentation engine.
///
/// The specific clock depends on the window system integration (WSI) API used.
///
/// <table>
/// <tr>
///     <td>WSI</td>
///     <td>Clock</td>
/// </tr>
/// <tr>
///     <td>IDXGISwapchain</td>
///     <td><a href="https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter">QueryPerformanceCounter</a></td>
/// </tr>
/// <tr>
///     <td>IPresentationManager</td>
///     <td><a href="https://docs.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-queryinterrupttimeprecise">QueryInterruptTimePrecise</a></td>
/// </tr>
/// <tr>
///     <td>CAMetalLayer</td>
///     <td><a href="https://developer.apple.com/documentation/kernel/1462446-mach_absolute_time">mach_absolute_time</a></td>
/// </tr>
/// <tr>
///     <td>VK_GOOGLE_display_timing</td>
///     <td><a href="https://linux.die.net/man/3/clock_gettime">clock_gettime(CLOCK_MONOTONIC)</a></td>
/// </tr>
/// </table>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PresentationTimestamp(
    /// Timestamp in nanoseconds.
    pub u128,
);

impl PresentationTimestamp {
    /// A timestamp that is invalid due to the platform not having a timestamp system.
    pub const INVALID_TIMESTAMP: Self = Self(u128::MAX);

    /// Returns true if this timestamp is the invalid timestamp.
    #[must_use]
    pub fn is_invalid(self) -> bool {
        self == Self::INVALID_TIMESTAMP
    }
}
