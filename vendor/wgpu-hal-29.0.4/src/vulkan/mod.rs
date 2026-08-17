/*!
# Vulkan API internals.

## Stack memory

Ash expects slices, which we don't generally have available.
We cope with this requirement by the combination of the following ways:
  - temporarily allocating `Vec` on heap, where overhead is permitted
  - growing temporary local storage

## Framebuffers and Render passes

Render passes are cached on the device and kept forever.

Framebuffers are also cached on the device, but they are removed when
any of the image views (they have) gets removed.
If Vulkan supports image-less framebuffers,
then the actual views are excluded from the framebuffer key.

## Fences

If timeline semaphores are available, they are used 1:1 with wgpu-hal fences.
Otherwise, we manage a pool of `VkFence` objects behind each `hal::Fence`.

!*/

mod adapter;
mod command;
pub mod conv;
mod device;
mod drm;
mod instance;
mod sampler;
mod semaphore_list;
mod swapchain;

pub use adapter::PhysicalDeviceFeatures;

use alloc::{boxed::Box, ffi::CString, sync::Arc, vec::Vec};
use core::{
    borrow::Borrow,
    ffi::CStr,
    fmt,
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    num::NonZeroU32,
};

use arrayvec::ArrayVec;
use ash::{ext, khr, vk};
use bytemuck::{Pod, Zeroable};
use hashbrown::HashSet;
use parking_lot::{Mutex, RwLock};

use naga::FastHashMap;
use wgt::InternalCounter;

use semaphore_list::SemaphoreList;

use crate::vulkan::semaphore_list::{SemaphoreListMode, SemaphoreType};

const MAX_TOTAL_ATTACHMENTS: usize = crate::MAX_COLOR_ATTACHMENTS * 2 + 1;

#[derive(Clone, Debug)]
pub struct Api;

impl crate::Api for Api {
    const VARIANT: wgt::Backend = wgt::Backend::Vulkan;

    type Instance = Instance;
    type Surface = Surface;
    type Adapter = Adapter;
    type Device = Device;

    type Queue = Queue;
    type CommandEncoder = CommandEncoder;
    type CommandBuffer = CommandBuffer;

    type Buffer = Buffer;
    type Texture = Texture;
    type SurfaceTexture = SurfaceTexture;
    type TextureView = TextureView;
    type Sampler = Sampler;
    type QuerySet = QuerySet;
    type Fence = Fence;
    type AccelerationStructure = AccelerationStructure;
    type PipelineCache = PipelineCache;

    type BindGroupLayout = BindGroupLayout;
    type BindGroup = BindGroup;
    type PipelineLayout = PipelineLayout;
    type ShaderModule = ShaderModule;
    type RenderPipeline = RenderPipeline;
    type ComputePipeline = ComputePipeline;
}

crate::impl_dyn_resource!(
    Adapter,
    AccelerationStructure,
    BindGroup,
    BindGroupLayout,
    Buffer,
    CommandBuffer,
    CommandEncoder,
    ComputePipeline,
    Device,
    Fence,
    Instance,
    PipelineCache,
    PipelineLayout,
    QuerySet,
    Queue,
    RenderPipeline,
    Sampler,
    ShaderModule,
    Surface,
    SurfaceTexture,
    Texture,
    TextureView
);

struct DebugUtils {
    extension: ext::debug_utils::Instance,
    messenger: vk::DebugUtilsMessengerEXT,

    /// Owning pointer to the debug messenger callback user data.
    ///
    /// `InstanceShared::drop` destroys the debug messenger before
    /// dropping this, so the callback should never receive a dangling
    /// user data pointer.
    #[allow(dead_code)]
    callback_data: Box<DebugUtilsMessengerUserData>,
}

pub struct DebugUtilsCreateInfo {
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: Box<DebugUtilsMessengerUserData>,
}

#[derive(Debug)]
/// The properties related to the validation layer needed for the
/// DebugUtilsMessenger for their workarounds
struct ValidationLayerProperties {
    /// Validation layer description, from `vk::LayerProperties`.
    layer_description: CString,

    /// Validation layer specification version, from `vk::LayerProperties`.
    layer_spec_version: u32,
}

/// User data needed by `instance::debug_utils_messenger_callback`.
///
/// When we create the [`vk::DebugUtilsMessengerEXT`], the `pUserData`
/// pointer refers to one of these values.
#[derive(Debug)]
pub struct DebugUtilsMessengerUserData {
    /// The properties related to the validation layer, if present
    validation_layer_properties: Option<ValidationLayerProperties>,

    /// If the OBS layer is present. OBS never increments the version of their layer,
    /// so there's no reason to have the version.
    has_obs_layer: bool,
}

pub struct InstanceShared {
    raw: ash::Instance,
    extensions: Vec<&'static CStr>,
    flags: wgt::InstanceFlags,
    memory_budget_thresholds: wgt::MemoryBudgetThresholds,
    debug_utils: Option<DebugUtils>,
    get_physical_device_properties: Option<khr::get_physical_device_properties2::Instance>,
    entry: ash::Entry,
    has_nv_optimus: bool,
    android_sdk_version: u32,
    /// The instance API version.
    ///
    /// Which is the version of Vulkan supported for instance-level functionality.
    ///
    /// It is associated with a `VkInstance` and its children,
    /// except for a `VkPhysicalDevice` and its children.
    instance_api_version: u32,

    // The `drop_guard` field must be the last field of this struct so it is dropped last.
    // Do not add new fields after it.
    drop_guard: Option<crate::DropGuard>,
}

pub struct Instance {
    shared: Arc<InstanceShared>,
}

pub struct Surface {
    inner: ManuallyDrop<Box<dyn swapchain::Surface>>,
    swapchain: RwLock<Option<Box<dyn swapchain::Swapchain>>>,
}

impl Surface {
    /// Returns the raw Vulkan surface handle.
    ///
    /// Returns `None` if the surface is a DXGI surface.
    pub unsafe fn raw_native_handle(&self) -> Option<vk::SurfaceKHR> {
        Some(
            self.inner
                .as_any()
                .downcast_ref::<swapchain::NativeSurface>()?
                .as_raw(),
        )
    }

    /// Get the raw Vulkan swapchain associated with this surface.
    ///
    /// Returns [`None`] if the surface is not configured or if the swapchain
    /// is a DXGI swapchain.
    pub fn raw_native_swapchain(&self) -> Option<vk::SwapchainKHR> {
        let read = self.swapchain.read();
        Some(
            read.as_ref()?
                .as_any()
                .downcast_ref::<swapchain::NativeSwapchain>()?
                .as_raw(),
        )
    }

    /// Set the present timing information which will be used for the next [presentation](crate::Queue::present()) of this surface,
    /// using [VK_GOOGLE_display_timing].
    ///
    /// This can be used to give an id to presentations, for future use of [`vk::PastPresentationTimingGOOGLE`].
    /// Note that `wgpu-hal` does *not* provide a way to use that API - you should manually access this through [`ash`].
    ///
    /// This can also be used to add a "not before" timestamp to the presentation.
    ///
    /// The exact semantics of the fields are also documented in the [specification](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentTimeGOOGLE.html) for the extension.
    ///
    /// # Panics
    ///
    /// - If the surface hasn't been configured.
    /// - If the surface has been configured for a DXGI swapchain.
    /// - If the device doesn't [support present timing](wgt::Features::VULKAN_GOOGLE_DISPLAY_TIMING).
    ///
    /// [VK_GOOGLE_display_timing]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_display_timing.html
    #[track_caller]
    pub fn set_next_present_time(&self, present_timing: vk::PresentTimeGOOGLE) {
        let mut swapchain = self.swapchain.write();
        swapchain
            .as_mut()
            .expect("Surface should have been configured")
            .as_any_mut()
            .downcast_mut::<swapchain::NativeSwapchain>()
            .expect("Surface should have a native Vulkan swapchain")
            .set_next_present_time(present_timing);
    }
}

#[derive(Debug)]
pub struct SurfaceTexture {
    index: u32,
    texture: Texture,
    metadata: Box<dyn swapchain::SurfaceTextureMetadata>,
}

impl crate::DynSurfaceTexture for SurfaceTexture {}

impl Borrow<Texture> for SurfaceTexture {
    fn borrow(&self) -> &Texture {
        &self.texture
    }
}

impl Borrow<dyn crate::DynTexture> for SurfaceTexture {
    fn borrow(&self) -> &dyn crate::DynTexture {
        &self.texture
    }
}

pub struct Adapter {
    raw: vk::PhysicalDevice,
    instance: Arc<InstanceShared>,
    //queue_families: Vec<vk::QueueFamilyProperties>,
    known_memory_flags: vk::MemoryPropertyFlags,
    phd_capabilities: adapter::PhysicalDeviceProperties,
    phd_features: PhysicalDeviceFeatures,
    downlevel_flags: wgt::DownlevelFlags,
    private_caps: PrivateCapabilities,
    workarounds: Workarounds,
}

// TODO there's no reason why this can't be unified--the function pointers should all be the same--it's not clear how to do this with `ash`.
enum ExtensionFn<T> {
    /// The loaded function pointer struct for an extension.
    Extension(T),
    /// The extension was promoted to a core version of Vulkan and the functions on `ash`'s `DeviceV1_x` traits should be used.
    Promoted,
}

struct DeviceExtensionFunctions {
    debug_utils: Option<ext::debug_utils::Device>,
    draw_indirect_count: Option<khr::draw_indirect_count::Device>,
    timeline_semaphore: Option<ExtensionFn<khr::timeline_semaphore::Device>>,
    ray_tracing: Option<RayTracingDeviceExtensionFunctions>,
    mesh_shading: Option<ext::mesh_shader::Device>,
}

struct RayTracingDeviceExtensionFunctions {
    acceleration_structure: khr::acceleration_structure::Device,
    buffer_device_address: khr::buffer_device_address::Device,
}

/// Set of internal capabilities, which don't show up in the exposed
/// device geometry, but affect the code paths taken internally.
#[derive(Clone, Debug)]
struct PrivateCapabilities {
    image_view_usage: bool,
    timeline_semaphores: bool,
    texture_d24: bool,
    texture_d24_s8: bool,
    texture_s8: bool,
    /// Ability to present contents to any screen. Only needed to work around broken platform configurations.
    can_present: bool,
    non_coherent_map_mask: wgt::BufferAddress,
    multi_draw_indirect: bool,
    max_draw_indirect_count: u32,

    /// True if this adapter advertises the [`robustBufferAccess`][vrba] feature.
    ///
    /// Note that Vulkan's `robustBufferAccess` is not sufficient to implement
    /// `wgpu_hal`'s guarantee that shaders will not access buffer contents via
    /// a given bindgroup binding outside that binding's [accessible
    /// region][ar]. Enabling `robustBufferAccess` does ensure that
    /// out-of-bounds reads and writes are not undefined behavior (that's good),
    /// but still permits out-of-bounds reads to return data from anywhere
    /// within the buffer, not just the accessible region.
    ///
    /// [ar]: ../struct.BufferBinding.html#accessible-region
    /// [vrba]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess
    robust_buffer_access: bool,

    robust_image_access: bool,

    /// True if this adapter supports the [`VK_EXT_robustness2`] extension's
    /// [`robustBufferAccess2`] feature.
    ///
    /// This is sufficient to implement `wgpu_hal`'s [required bounds-checking][ar] of
    /// shader accesses to buffer contents. If this feature is not available,
    /// this backend must have Naga inject bounds checks in the generated
    /// SPIR-V.
    ///
    /// [`VK_EXT_robustness2`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_robustness2.html
    /// [`robustBufferAccess2`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html#features-robustBufferAccess2
    /// [ar]: ../struct.BufferBinding.html#accessible-region
    robust_buffer_access2: bool,

    robust_image_access2: bool,
    zero_initialize_workgroup_memory: bool,
    image_format_list: bool,
    maximum_samplers: u32,

    /// True if this adapter supports the [`VK_KHR_shader_integer_dot_product`] extension
    /// (promoted to Vulkan 1.3).
    ///
    /// This is used to generate optimized code for WGSL's `dot4{I, U}8Packed`.
    ///
    /// [`VK_KHR_shader_integer_dot_product`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_integer_dot_product.html
    shader_integer_dot_product: bool,

    /// True if this adapter supports 8-bit integers provided by the
    /// [`VK_KHR_shader_float16_int8`] extension (promoted to Vulkan 1.2).
    ///
    /// Allows shaders to declare the "Int8" capability. Note, however, that this
    /// feature alone allows the use of 8-bit integers "only in the `Private`,
    /// `Workgroup` (for non-Block variables), and `Function` storage classes"
    /// ([see spec]). To use 8-bit integers in the interface storage classes (e.g.,
    /// `StorageBuffer`), you also need to enable the corresponding feature in
    /// `VkPhysicalDevice8BitStorageFeatures` and declare the corresponding SPIR-V
    /// capability (e.g., `StorageBuffer8BitAccess`).
    ///
    /// [`VK_KHR_shader_float16_int8`]: https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_shader_float16_int8.html
    /// [see spec]: https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderFloat16Int8Features.html#extension-features-shaderInt8
    shader_int8: bool,

    /// This is done to panic before undefined behavior, and is imperfect.
    /// Basically, to allow implementations to emulate mv using instancing, if you
    /// want to draw `n` instances to VR, you must draw `2n` instances, but you
    /// can never draw more than `u32::MAX` instances. Therefore, when drawing
    /// multiview on some vulkan implementations, it might restrict the instance
    /// count, which isn't usually a thing in webgpu. We don't expose this limit
    /// because its strange, i.e. only occurs on certain vulkan implementations
    /// if you are drawing more than 128 million instances. We still want to avoid
    /// undefined behavior in this situation, so we panic if the limit is violated.
    multiview_instance_index_limit: u32,

    /// BufferUsages::ACCELERATION_STRUCTURE_SCRATCH allows usage as a scratch buffer.
    /// Vulkan has no way to specify this as a usage, and it maps to other usages, but
    /// these usages do not have as high of an alignment requirement using the buffer as
    ///  a scratch buffer when building acceleration structures.
    scratch_buffer_alignment: u32,
}

bitflags::bitflags!(
    /// Workaround flags.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Workarounds: u32 {
        /// Only generate SPIR-V for one entry point at a time.
        const SEPARATE_ENTRY_POINTS = 0x1;
        /// Qualcomm OOMs when there are zero color attachments but a non-null pointer
        /// to a subpass resolve attachment array. This nulls out that pointer in that case.
        const EMPTY_RESOLVE_ATTACHMENT_LISTS = 0x2;
        /// If the following code returns false, then nvidia will end up filling the wrong range.
        ///
        /// ```skip
        /// fn nvidia_succeeds() -> bool {
        ///   # let (copy_length, start_offset) = (0, 0);
        ///     if copy_length >= 4096 {
        ///         if start_offset % 16 != 0 {
        ///             if copy_length == 4096 {
        ///                 return true;
        ///             }
        ///             if copy_length % 16 == 0 {
        ///                 return false;
        ///             }
        ///         }
        ///     }
        ///     true
        /// }
        /// ```
        ///
        /// As such, we need to make sure all calls to vkCmdFillBuffer are aligned to 16 bytes
        /// if they cover a range of 4096 bytes or more.
        const FORCE_FILL_BUFFER_WITH_SIZE_GREATER_4096_ALIGNED_OFFSET_16 = 0x4;
    }
);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct AttachmentKey {
    format: vk::Format,
    layout: vk::ImageLayout,
    ops: crate::AttachmentOps,
}

impl AttachmentKey {
    /// Returns an attachment key for a compatible attachment.
    fn compatible(format: vk::Format, layout: vk::ImageLayout) -> Self {
        Self {
            format,
            layout,
            ops: crate::AttachmentOps::all(),
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct ColorAttachmentKey {
    base: AttachmentKey,
    resolve: Option<AttachmentKey>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct DepthStencilAttachmentKey {
    base: AttachmentKey,
    stencil_ops: crate::AttachmentOps,
}

#[derive(Clone, Eq, Default, Hash, PartialEq)]
struct RenderPassKey {
    colors: ArrayVec<Option<ColorAttachmentKey>, { crate::MAX_COLOR_ATTACHMENTS }>,
    depth_stencil: Option<DepthStencilAttachmentKey>,
    sample_count: u32,
    multiview_mask: Option<NonZeroU32>,
}

struct DeviceShared {
    raw: ash::Device,
    family_index: u32,
    queue_index: u32,
    raw_queue: vk::Queue,
    instance: Arc<InstanceShared>,
    physical_device: vk::PhysicalDevice,
    enabled_extensions: Vec<&'static CStr>,
    extension_fns: DeviceExtensionFunctions,
    vendor_id: u32,
    pipeline_cache_validation_key: [u8; 16],
    timestamp_period: f32,
    private_caps: PrivateCapabilities,
    workarounds: Workarounds,
    features: wgt::Features,
    render_passes: Mutex<FastHashMap<RenderPassKey, vk::RenderPass>>,
    sampler_cache: Mutex<sampler::SamplerCache>,
    memory_allocations_counter: InternalCounter,

    /// Because we have cached framebuffers which are not deleted from until
    /// the device is destroyed, if the implementation of vulkan re-uses handles
    /// we need some way to differentiate between the old handle and the new handle.
    /// This factory allows us to have a dedicated identity value for each texture.
    texture_identity_factory: ResourceIdentityFactory<vk::Image>,
    /// As above, for texture views.
    texture_view_identity_factory: ResourceIdentityFactory<vk::ImageView>,

    empty_descriptor_set_layout: vk::DescriptorSetLayout,

    // The `drop_guard` field must be the last field of this struct so it is dropped last.
    // Do not add new fields after it.
    drop_guard: Option<crate::DropGuard>,
}

impl Drop for DeviceShared {
    fn drop(&mut self) {
        for &raw in self.render_passes.lock().values() {
            unsafe { self.raw.destroy_render_pass(raw, None) };
        }
        unsafe {
            self.raw
                .destroy_descriptor_set_layout(self.empty_descriptor_set_layout, None)
        };
        if self.drop_guard.is_none() {
            unsafe { self.raw.destroy_device(None) };
        }
    }
}

pub struct Device {
    mem_allocator: Mutex<gpu_allocator::vulkan::Allocator>,
    desc_allocator:
        Mutex<gpu_descriptor::DescriptorAllocator<vk::DescriptorPool, vk::DescriptorSet>>,
    valid_ash_memory_types: u32,
    naga_options: naga::back::spv::Options<'static>,
    #[cfg(feature = "renderdoc")]
    render_doc: crate::auxil::renderdoc::RenderDoc,
    counters: Arc<wgt::HalCounters>,
    // Struct members are dropped from first to last, put the Device last to ensure that
    // all resources that depends on it are destroyed before it like the mem_allocator
    shared: Arc<DeviceShared>,
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { self.desc_allocator.lock().cleanup(&*self.shared) };
    }
}

/// Semaphores for forcing queue submissions to run in order.
///
/// The [`wgpu_hal::Queue`] trait promises that if two calls to [`submit`] are
/// ordered, then the first submission will finish on the GPU before the second
/// submission begins. To get this behavior on Vulkan we need to pass semaphores
/// to [`vkQueueSubmit`] for the commands to wait on before beginning execution,
/// and to signal when their execution is done.
///
/// Normally this can be done with a single semaphore, waited on and then
/// signalled for each submission. At any given time there's exactly one
/// submission that would signal the semaphore, and exactly one waiting on it,
/// as Vulkan requires.
///
/// However, as of Oct 2021, bug [#5508] in the Mesa ANV drivers caused them to
/// hang if we use a single semaphore. The workaround is to alternate between
/// two semaphores. The bug has been fixed in Mesa, but we should probably keep
/// the workaround until, say, Oct 2026.
///
/// [`wgpu_hal::Queue`]: crate::Queue
/// [`submit`]: crate::Queue::submit
/// [`vkQueueSubmit`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#vkQueueSubmit
/// [#5508]: https://gitlab.freedesktop.org/mesa/mesa/-/issues/5508
#[derive(Clone)]
struct RelaySemaphores {
    /// The semaphore the next submission should wait on before beginning
    /// execution on the GPU. This is `None` for the first submission, which
    /// should not wait on anything at all.
    wait: Option<vk::Semaphore>,

    /// The semaphore the next submission should signal when it has finished
    /// execution on the GPU.
    signal: vk::Semaphore,
}

impl RelaySemaphores {
    fn new(device: &DeviceShared) -> Result<Self, crate::DeviceError> {
        Ok(Self {
            wait: None,
            signal: device.new_binary_semaphore("RelaySemaphores: 1")?,
        })
    }

    /// Advances the semaphores, returning the semaphores that should be used for a submission.
    fn advance(&mut self, device: &DeviceShared) -> Result<Self, crate::DeviceError> {
        let old = self.clone();

        // Build the state for the next submission.
        match self.wait {
            None => {
                // The `old` values describe the first submission to this queue.
                // The second submission should wait on `old.signal`, and then
                // signal a new semaphore which we'll create now.
                self.wait = Some(old.signal);
                self.signal = device.new_binary_semaphore("RelaySemaphores: 2")?;
            }
            Some(ref mut wait) => {
                // What this submission signals, the next should wait.
                mem::swap(wait, &mut self.signal);
            }
        };

        Ok(old)
    }

    /// Destroys the semaphores.
    unsafe fn destroy(&self, device: &ash::Device) {
        unsafe {
            if let Some(wait) = self.wait {
                device.destroy_semaphore(wait, None);
            }
            device.destroy_semaphore(self.signal, None);
        }
    }
}

pub struct Queue {
    raw: vk::Queue,
    device: Arc<DeviceShared>,
    family_index: u32,
    relay_semaphores: Mutex<RelaySemaphores>,
    signal_semaphores: Mutex<SemaphoreList>,
}

impl Queue {
    pub fn as_raw(&self) -> vk::Queue {
        self.raw
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe { self.relay_semaphores.lock().destroy(&self.device.raw) };
    }
}
#[derive(Debug)]
enum BufferMemoryBacking {
    Managed(gpu_allocator::vulkan::Allocation),
    VulkanMemory {
        memory: vk::DeviceMemory,
        offset: u64,
        size: u64,
    },
}
impl BufferMemoryBacking {
    fn memory(&self) -> vk::DeviceMemory {
        match self {
            Self::Managed(m) => unsafe { m.memory() },
            Self::VulkanMemory { memory, .. } => *memory,
        }
    }
    fn offset(&self) -> u64 {
        match self {
            Self::Managed(m) => m.offset(),
            Self::VulkanMemory { offset, .. } => *offset,
        }
    }
    fn size(&self) -> u64 {
        match self {
            Self::Managed(m) => m.size(),
            Self::VulkanMemory { size, .. } => *size,
        }
    }
}
#[derive(Debug)]
pub struct Buffer {
    raw: vk::Buffer,
    allocation: Option<Mutex<BufferMemoryBacking>>,
}
impl Buffer {
    /// # Safety
    ///
    /// - `vk_buffer`'s memory must be managed by the caller
    /// - Externally imported buffers can't be mapped by `wgpu`
    pub unsafe fn from_raw(vk_buffer: vk::Buffer) -> Self {
        Self {
            raw: vk_buffer,
            allocation: None,
        }
    }
    /// # Safety
    /// - We will use this buffer and the buffer's backing memory range as if we have exclusive ownership over it, until the wgpu resource is dropped and the wgpu-hal object is cleaned up
    /// - Externally imported buffers can't be mapped by `wgpu`
    /// - `offset` and `size` must be valid with the allocation of `memory`
    pub unsafe fn from_raw_managed(
        vk_buffer: vk::Buffer,
        memory: vk::DeviceMemory,
        offset: u64,
        size: u64,
    ) -> Self {
        Self {
            raw: vk_buffer,
            allocation: Some(Mutex::new(BufferMemoryBacking::VulkanMemory {
                memory,
                offset,
                size,
            })),
        }
    }
}

impl crate::DynBuffer for Buffer {}

#[derive(Debug)]
pub struct AccelerationStructure {
    raw: vk::AccelerationStructureKHR,
    buffer: vk::Buffer,
    allocation: gpu_allocator::vulkan::Allocation,
    compacted_size_query: Option<vk::QueryPool>,
}

impl crate::DynAccelerationStructure for AccelerationStructure {}

#[derive(Debug)]
pub enum TextureMemory {
    // shared memory in GPU allocator (owned by wgpu-hal)
    Allocation(gpu_allocator::vulkan::Allocation),

    // dedicated memory (owned by wgpu-hal)
    Dedicated(vk::DeviceMemory),

    // memory not owned by wgpu
    External,
}

#[derive(Debug)]
pub struct Texture {
    raw: vk::Image,
    memory: TextureMemory,
    format: wgt::TextureFormat,
    copy_size: crate::CopyExtent,
    identity: ResourceIdentity<vk::Image>,

    // The `drop_guard` field must be the last field of this struct so it is dropped last.
    // Do not add new fields after it.
    drop_guard: Option<crate::DropGuard>,
}

impl crate::DynTexture for Texture {}

impl Texture {
    /// # Safety
    ///
    /// - The image handle must not be manually destroyed
    pub unsafe fn raw_handle(&self) -> vk::Image {
        self.raw
    }

    /// # Safety
    ///
    /// - The caller must not free the `vk::DeviceMemory` or
    ///   `gpu_alloc::MemoryBlock` in the returned `TextureMemory`.
    pub unsafe fn memory(&self) -> &TextureMemory {
        &self.memory
    }
}

#[derive(Debug)]
pub struct TextureView {
    raw_texture: vk::Image,
    raw: vk::ImageView,
    _layers: NonZeroU32,
    format: wgt::TextureFormat,
    raw_format: vk::Format,
    base_mip_level: u32,
    dimension: wgt::TextureViewDimension,
    texture_identity: ResourceIdentity<vk::Image>,
    view_identity: ResourceIdentity<vk::ImageView>,
}

impl crate::DynTextureView for TextureView {}

impl TextureView {
    /// # Safety
    ///
    /// - The image view handle must not be manually destroyed
    pub unsafe fn raw_handle(&self) -> vk::ImageView {
        self.raw
    }

    /// Returns the raw texture view, along with its identity.
    fn identified_raw_view(&self) -> IdentifiedTextureView {
        IdentifiedTextureView {
            raw: self.raw,
            identity: self.view_identity,
        }
    }
}

#[derive(Debug)]
pub struct Sampler {
    raw: vk::Sampler,
    create_info: vk::SamplerCreateInfo<'static>,
}

impl crate::DynSampler for Sampler {}

/// Information about a binding within a specific BindGroupLayout / BindGroup.
/// This will be used to construct a [`naga::back::spv::BindingInfo`], where
/// the descriptor set value will be taken from the index of the group.
#[derive(Copy, Clone, Debug)]
struct BindingInfo {
    binding: u32,
    binding_array_size: Option<NonZeroU32>,
}

#[derive(Debug)]
pub struct BindGroupLayout {
    raw: vk::DescriptorSetLayout,
    desc_count: gpu_descriptor::DescriptorTotalCount,
    /// Sorted list of entries.
    entries: Box<[wgt::BindGroupLayoutEntry]>,
    /// Map of original binding index to remapped binding index and optional
    /// array size.
    binding_map: Vec<(u32, BindingInfo)>,
    contains_binding_arrays: bool,
}

impl crate::DynBindGroupLayout for BindGroupLayout {}

#[derive(Debug)]
pub struct PipelineLayout {
    raw: vk::PipelineLayout,
    binding_map: naga::back::spv::BindingMap,
}

impl crate::DynPipelineLayout for PipelineLayout {}

#[derive(Debug)]
pub struct BindGroup {
    set: gpu_descriptor::DescriptorSet<vk::DescriptorSet>,
}

impl crate::DynBindGroup for BindGroup {}

/// Miscellaneous allocation recycling pool for `CommandAllocator`.
#[derive(Default)]
struct Temp {
    marker: Vec<u8>,
    buffer_barriers: Vec<vk::BufferMemoryBarrier<'static>>,
    image_barriers: Vec<vk::ImageMemoryBarrier<'static>>,
}

impl Temp {
    fn clear(&mut self) {
        self.marker.clear();
        self.buffer_barriers.clear();
        self.image_barriers.clear();
    }

    fn make_c_str(&mut self, name: &str) -> &CStr {
        self.marker.clear();
        self.marker.extend_from_slice(name.as_bytes());
        self.marker.push(0);
        unsafe { CStr::from_bytes_with_nul_unchecked(&self.marker) }
    }
}

/// Generates unique IDs for each resource of type `T`.
///
/// Because vk handles are not permanently unique, this
/// provides a way to generate unique IDs for each resource.
struct ResourceIdentityFactory<T> {
    #[cfg(not(target_has_atomic = "64"))]
    next_id: Mutex<u64>,
    #[cfg(target_has_atomic = "64")]
    next_id: core::sync::atomic::AtomicU64,
    _phantom: PhantomData<T>,
}

impl<T> ResourceIdentityFactory<T> {
    fn new() -> Self {
        Self {
            #[cfg(not(target_has_atomic = "64"))]
            next_id: Mutex::new(0),
            #[cfg(target_has_atomic = "64")]
            next_id: core::sync::atomic::AtomicU64::new(0),
            _phantom: PhantomData,
        }
    }

    /// Returns a new unique ID for a resource of type `T`.
    fn next(&self) -> ResourceIdentity<T> {
        #[cfg(not(target_has_atomic = "64"))]
        {
            let mut next_id = self.next_id.lock();
            let id = *next_id;
            *next_id += 1;
            ResourceIdentity {
                id,
                _phantom: PhantomData,
            }
        }

        #[cfg(target_has_atomic = "64")]
        ResourceIdentity {
            id: self
                .next_id
                .fetch_add(1, core::sync::atomic::Ordering::Relaxed),
            _phantom: PhantomData,
        }
    }
}

/// A unique identifier for a resource of type `T`.
///
/// This is used as a hashable key for resources, which
/// is permanently unique through the lifetime of the program.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct ResourceIdentity<T> {
    id: u64,
    _phantom: PhantomData<T>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct FramebufferKey {
    raw_pass: vk::RenderPass,
    /// Because this is used as a key in a hash map, we need to include the identity
    /// so that this hashes differently, even if the ImageView handles are the same
    /// between different views.
    attachment_identities: ArrayVec<ResourceIdentity<vk::ImageView>, { MAX_TOTAL_ATTACHMENTS }>,
    /// While this is redundant for calculating the hash, we need access to an array
    /// of all the raw ImageViews when we are creating the actual framebuffer,
    /// so we store this here.
    attachment_views: ArrayVec<vk::ImageView, { MAX_TOTAL_ATTACHMENTS }>,
    extent: wgt::Extent3d,
}

impl FramebufferKey {
    fn push_view(&mut self, view: IdentifiedTextureView) {
        self.attachment_identities.push(view.identity);
        self.attachment_views.push(view.raw);
    }
}

/// A texture view paired with its identity.
#[derive(Copy, Clone)]
struct IdentifiedTextureView {
    raw: vk::ImageView,
    identity: ResourceIdentity<vk::ImageView>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct TempTextureViewKey {
    texture: vk::Image,
    /// As this is used in a hashmap, we need to
    /// include the identity so that this hashes differently,
    /// even if the Image handles are the same between different images.
    texture_identity: ResourceIdentity<vk::Image>,
    format: vk::Format,
    mip_level: u32,
    depth_slice: u32,
}

pub struct CommandEncoder {
    raw: vk::CommandPool,
    device: Arc<DeviceShared>,

    /// The current command buffer, if `self` is in the ["recording"]
    /// state.
    ///
    /// ["recording"]: crate::CommandEncoder
    ///
    /// If non-`null`, the buffer is in the Vulkan "recording" state.
    active: vk::CommandBuffer,

    /// What kind of pass we are currently within: compute or render.
    bind_point: vk::PipelineBindPoint,

    /// Allocation recycling pool for this encoder.
    temp: Temp,

    /// A pool of available command buffers.
    ///
    /// These are all in the Vulkan "initial" state.
    free: Vec<vk::CommandBuffer>,

    /// A pool of discarded command buffers.
    ///
    /// These could be in any Vulkan state except "pending".
    discarded: Vec<vk::CommandBuffer>,

    /// If this is true, the active renderpass enabled a debug span,
    /// and needs to be disabled on renderpass close.
    rpass_debug_marker_active: bool,

    /// If set, the end of the next render/compute pass will write a timestamp at
    /// the given pool & location.
    end_of_pass_timer_query: Option<(vk::QueryPool, u32)>,

    framebuffers: FastHashMap<FramebufferKey, vk::Framebuffer>,
    temp_texture_views: FastHashMap<TempTextureViewKey, IdentifiedTextureView>,

    counters: Arc<wgt::HalCounters>,

    current_pipeline_is_multiview: bool,
}

impl Drop for CommandEncoder {
    fn drop(&mut self) {
        // SAFETY:
        //
        // VUID-vkDestroyCommandPool-commandPool-00041: wgpu_hal requires that a
        // `CommandBuffer` must live until its execution is complete, and that a
        // `CommandBuffer` must not outlive the `CommandEncoder` that built it.
        // Thus, we know that none of our `CommandBuffers` are in the "pending"
        // state.
        //
        // The other VUIDs are pretty obvious.
        unsafe {
            // `vkDestroyCommandPool` also frees any command buffers allocated
            // from that pool, so there's no need to explicitly call
            // `vkFreeCommandBuffers` on `cmd_encoder`'s `free` and `discarded`
            // fields.
            self.device.raw.destroy_command_pool(self.raw, None);
        }

        for (_, fb) in self.framebuffers.drain() {
            unsafe { self.device.raw.destroy_framebuffer(fb, None) };
        }

        for (_, view) in self.temp_texture_views.drain() {
            unsafe { self.device.raw.destroy_image_view(view.raw, None) };
        }

        self.counters.command_encoders.sub(1);
    }
}

impl CommandEncoder {
    /// # Safety
    ///
    /// - The command buffer handle must not be manually destroyed
    pub unsafe fn raw_handle(&self) -> vk::CommandBuffer {
        self.active
    }
}

impl fmt::Debug for CommandEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandEncoder")
            .field("raw", &self.raw)
            .finish()
    }
}

#[derive(Debug)]
pub struct CommandBuffer {
    raw: vk::CommandBuffer,
}

impl crate::DynCommandBuffer for CommandBuffer {}

#[derive(Debug)]
pub enum ShaderModule {
    Raw(vk::ShaderModule),
    Intermediate {
        naga_shader: crate::NagaShader,
        runtime_checks: wgt::ShaderRuntimeChecks,
    },
}

impl crate::DynShaderModule for ShaderModule {}

#[derive(Debug)]
pub struct RenderPipeline {
    raw: vk::Pipeline,
    is_multiview: bool,
}

impl crate::DynRenderPipeline for RenderPipeline {}

#[derive(Debug)]
pub struct ComputePipeline {
    raw: vk::Pipeline,
}

impl crate::DynComputePipeline for ComputePipeline {}

#[derive(Debug)]
pub struct PipelineCache {
    raw: vk::PipelineCache,
}

impl crate::DynPipelineCache for PipelineCache {}

#[derive(Debug)]
pub struct QuerySet {
    raw: vk::QueryPool,
}

impl crate::DynQuerySet for QuerySet {}

/// The [`Api::Fence`] type for [`vulkan::Api`].
///
/// This is an `enum` because there are two possible implementations of
/// `wgpu-hal` fences on Vulkan: Vulkan fences, which work on any version of
/// Vulkan, and Vulkan timeline semaphores, which are easier and cheaper but
/// require non-1.0 features.
///
/// [`Device::create_fence`] returns a [`TimelineSemaphore`] if
/// [`VK_KHR_timeline_semaphore`] is available and enabled, and a [`FencePool`]
/// otherwise.
///
/// [`Api::Fence`]: crate::Api::Fence
/// [`vulkan::Api`]: Api
/// [`Device::create_fence`]: crate::Device::create_fence
/// [`TimelineSemaphore`]: Fence::TimelineSemaphore
/// [`VK_KHR_timeline_semaphore`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#VK_KHR_timeline_semaphore
/// [`FencePool`]: Fence::FencePool
#[derive(Debug)]
pub enum Fence {
    /// A Vulkan [timeline semaphore].
    ///
    /// These are simpler to use than Vulkan fences, since timeline semaphores
    /// work exactly the way [`wpgu_hal::Api::Fence`] is specified to work.
    ///
    /// [timeline semaphore]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores
    /// [`wpgu_hal::Api::Fence`]: crate::Api::Fence
    TimelineSemaphore(vk::Semaphore),

    /// A collection of Vulkan [fence]s, each associated with a [`FenceValue`].
    ///
    /// The effective [`FenceValue`] of this variant is the greater of
    /// `last_completed` and the maximum value associated with a signalled fence
    /// in `active`.
    ///
    /// Fences are available in all versions of Vulkan, but since they only have
    /// two states, "signaled" and "unsignaled", we need to use a separate fence
    /// for each queue submission we might want to wait for, and remember which
    /// [`FenceValue`] each one represents.
    ///
    /// [fence]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences
    /// [`FenceValue`]: crate::FenceValue
    FencePool {
        last_completed: crate::FenceValue,
        /// The pending fence values have to be ascending.
        active: Vec<(crate::FenceValue, vk::Fence)>,
        free: Vec<vk::Fence>,
    },
}

impl crate::DynFence for Fence {}

impl Fence {
    /// Return the highest [`FenceValue`] among the signalled fences in `active`.
    ///
    /// As an optimization, assume that we already know that the fence has
    /// reached `last_completed`, and don't bother checking fences whose values
    /// are less than that: those fences remain in the `active` array only
    /// because we haven't called `maintain` yet to clean them up.
    ///
    /// [`FenceValue`]: crate::FenceValue
    fn check_active(
        device: &ash::Device,
        mut last_completed: crate::FenceValue,
        active: &[(crate::FenceValue, vk::Fence)],
    ) -> Result<crate::FenceValue, crate::DeviceError> {
        for &(value, raw) in active.iter() {
            unsafe {
                if value > last_completed
                    && device
                        .get_fence_status(raw)
                        .map_err(map_host_device_oom_and_lost_err)?
                {
                    last_completed = value;
                }
            }
        }
        Ok(last_completed)
    }

    /// Return the highest signalled [`FenceValue`] for `self`.
    ///
    /// [`FenceValue`]: crate::FenceValue
    fn get_latest(
        &self,
        device: &ash::Device,
        extension: Option<&ExtensionFn<khr::timeline_semaphore::Device>>,
    ) -> Result<crate::FenceValue, crate::DeviceError> {
        match *self {
            Self::TimelineSemaphore(raw) => unsafe {
                Ok(match *extension.unwrap() {
                    ExtensionFn::Extension(ref ext) => ext
                        .get_semaphore_counter_value(raw)
                        .map_err(map_host_device_oom_and_lost_err)?,
                    ExtensionFn::Promoted => device
                        .get_semaphore_counter_value(raw)
                        .map_err(map_host_device_oom_and_lost_err)?,
                })
            },
            Self::FencePool {
                last_completed,
                ref active,
                free: _,
            } => Self::check_active(device, last_completed, active),
        }
    }

    /// Trim the internal state of this [`Fence`].
    ///
    /// This function has no externally visible effect, but you should call it
    /// periodically to keep this fence's resource consumption under control.
    ///
    /// For fences using the [`FencePool`] implementation, this function
    /// recycles fences that have been signaled. If you don't call this,
    /// [`Queue::submit`] will just keep allocating a new Vulkan fence every
    /// time it's called.
    ///
    /// [`FencePool`]: Fence::FencePool
    /// [`Queue::submit`]: crate::Queue::submit
    fn maintain(&mut self, device: &ash::Device) -> Result<(), crate::DeviceError> {
        match *self {
            Self::TimelineSemaphore(_) => {}
            Self::FencePool {
                ref mut last_completed,
                ref mut active,
                ref mut free,
            } => {
                let latest = Self::check_active(device, *last_completed, active)?;
                let base_free = free.len();
                for &(value, raw) in active.iter() {
                    if value <= latest {
                        free.push(raw);
                    }
                }
                if free.len() != base_free {
                    active.retain(|&(value, _)| value > latest);
                    unsafe { device.reset_fences(&free[base_free..]) }
                        .map_err(map_device_oom_err)?
                }
                *last_completed = latest;
            }
        }
        Ok(())
    }
}

impl crate::Queue for Queue {
    type A = Api;

    unsafe fn submit(
        &self,
        command_buffers: &[&CommandBuffer],
        surface_textures: &[&SurfaceTexture],
        (signal_fence, signal_value): (&mut Fence, crate::FenceValue),
    ) -> Result<(), crate::DeviceError> {
        let mut fence_raw = vk::Fence::null();

        let mut wait_semaphores = SemaphoreList::new(SemaphoreListMode::Wait);
        let mut signal_semaphores = SemaphoreList::new(SemaphoreListMode::Signal);

        // Double check that the same swapchain image isn't being given to us multiple times,
        // as that will deadlock when we try to lock them all.
        debug_assert!(
            {
                let mut check = HashSet::with_capacity(surface_textures.len());
                // We compare the Box by pointer, as Eq isn't well defined for SurfaceSemaphores.
                for st in surface_textures {
                    let ptr: *const () = <*const _>::cast(&*st.metadata);
                    check.insert(ptr as usize);
                }
                check.len() == surface_textures.len()
            },
            "More than one surface texture is being used from the same swapchain. This will cause a deadlock in release."
        );

        let locked_swapchain_semaphores = surface_textures
            .iter()
            .map(|st| st.metadata.get_semaphore_guard())
            .collect::<Vec<_>>();

        for mut semaphores in locked_swapchain_semaphores {
            semaphores.set_used_fence_value(signal_value);

            // If we're the first submission to operate on this image, wait on
            // its acquire semaphore, to make sure the presentation engine is
            // done with it.
            if let Some(sem) = semaphores.get_acquire_wait_semaphore() {
                wait_semaphores.push_wait(sem, vk::PipelineStageFlags::TOP_OF_PIPE);
            }

            // Get a semaphore to signal when we're done writing to this surface
            // image. Presentation of this image will wait for this.
            let signal_semaphore = semaphores.get_submit_signal_semaphore(&self.device)?;
            signal_semaphores.push_signal(signal_semaphore);
        }

        let mut guard = self.signal_semaphores.lock();
        if !guard.is_empty() {
            signal_semaphores.append(&mut guard);
        }

        // In order for submissions to be strictly ordered, we encode a dependency between each submission
        // using a pair of semaphores. This adds a wait if it is needed, and signals the next semaphore.
        let semaphore_state = self.relay_semaphores.lock().advance(&self.device)?;

        if let Some(sem) = semaphore_state.wait {
            wait_semaphores.push_wait(
                SemaphoreType::Binary(sem),
                vk::PipelineStageFlags::TOP_OF_PIPE,
            );
        }

        signal_semaphores.push_signal(SemaphoreType::Binary(semaphore_state.signal));

        // We need to signal our wgpu::Fence if we have one, this adds it to the signal list.
        signal_fence.maintain(&self.device.raw)?;
        match *signal_fence {
            Fence::TimelineSemaphore(raw) => {
                signal_semaphores.push_signal(SemaphoreType::Timeline(raw, signal_value));
            }
            Fence::FencePool {
                ref mut active,
                ref mut free,
                ..
            } => {
                fence_raw = match free.pop() {
                    Some(raw) => raw,
                    None => unsafe {
                        self.device
                            .raw
                            .create_fence(&vk::FenceCreateInfo::default(), None)
                            .map_err(map_host_device_oom_err)?
                    },
                };
                active.push((signal_value, fence_raw));
            }
        }

        let vk_cmd_buffers = command_buffers
            .iter()
            .map(|cmd| cmd.raw)
            .collect::<Vec<_>>();

        let mut vk_info = vk::SubmitInfo::default().command_buffers(&vk_cmd_buffers);
        let mut vk_timeline_info = mem::MaybeUninit::uninit();
        vk_info = SemaphoreList::add_to_submit(
            &mut wait_semaphores,
            &mut signal_semaphores,
            vk_info,
            &mut vk_timeline_info,
        );

        profiling::scope!("vkQueueSubmit");
        unsafe {
            self.device
                .raw
                .queue_submit(self.raw, &[vk_info], fence_raw)
                .map_err(map_host_device_oom_and_lost_err)?
        };
        Ok(())
    }

    unsafe fn present(
        &self,
        surface: &Surface,
        texture: SurfaceTexture,
    ) -> Result<(), crate::SurfaceError> {
        let mut swapchain = surface.swapchain.write();

        unsafe { swapchain.as_mut().unwrap().present(self, texture) }
    }

    unsafe fn get_timestamp_period(&self) -> f32 {
        self.device.timestamp_period
    }
}

impl Queue {
    pub fn raw_device(&self) -> &ash::Device {
        &self.device.raw
    }

    pub fn add_signal_semaphore(&self, semaphore: vk::Semaphore, semaphore_value: Option<u64>) {
        let mut guard = self.signal_semaphores.lock();
        if let Some(value) = semaphore_value {
            guard.push_signal(SemaphoreType::Timeline(semaphore, value));
        } else {
            guard.push_signal(SemaphoreType::Binary(semaphore));
        }
    }
}

/// Maps
///
/// - VK_ERROR_OUT_OF_HOST_MEMORY
/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
fn map_host_device_oom_err(err: vk::Result) -> crate::DeviceError {
    match err {
        vk::Result::ERROR_OUT_OF_HOST_MEMORY | vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => {
            get_oom_err(err)
        }
        e => get_unexpected_err(e),
    }
}

/// Maps
///
/// - VK_ERROR_OUT_OF_HOST_MEMORY
/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
/// - VK_ERROR_DEVICE_LOST
fn map_host_device_oom_and_lost_err(err: vk::Result) -> crate::DeviceError {
    match err {
        vk::Result::ERROR_DEVICE_LOST => get_lost_err(),
        other => map_host_device_oom_err(other),
    }
}

/// Maps
///
/// - VK_ERROR_OUT_OF_HOST_MEMORY
/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
/// - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
fn map_host_device_oom_and_ioca_err(err: vk::Result) -> crate::DeviceError {
    // We don't use VK_KHR_buffer_device_address
    // VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
    map_host_device_oom_err(err)
}

/// Maps
///
/// - VK_ERROR_OUT_OF_HOST_MEMORY
fn map_host_oom_err(err: vk::Result) -> crate::DeviceError {
    match err {
        vk::Result::ERROR_OUT_OF_HOST_MEMORY => get_oom_err(err),
        e => get_unexpected_err(e),
    }
}

/// Maps
///
/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
fn map_device_oom_err(err: vk::Result) -> crate::DeviceError {
    match err {
        vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => get_oom_err(err),
        e => get_unexpected_err(e),
    }
}

/// Maps
///
/// - VK_ERROR_OUT_OF_HOST_MEMORY
/// - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
fn map_host_oom_and_ioca_err(err: vk::Result) -> crate::DeviceError {
    // We don't use VK_KHR_buffer_device_address
    // VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
    map_host_oom_err(err)
}

/// Maps
///
/// - VK_ERROR_OUT_OF_HOST_MEMORY
/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
/// - VK_PIPELINE_COMPILE_REQUIRED_EXT
/// - VK_ERROR_INVALID_SHADER_NV
fn map_pipeline_err(err: vk::Result) -> crate::DeviceError {
    // We don't use VK_EXT_pipeline_creation_cache_control
    // VK_PIPELINE_COMPILE_REQUIRED_EXT
    // We don't use VK_NV_glsl_shader
    // VK_ERROR_INVALID_SHADER_NV
    map_host_device_oom_err(err)
}

/// Returns [`crate::DeviceError::Unexpected`] or panics if the `internal_error_panic`
/// feature flag is enabled.
fn get_unexpected_err(_err: vk::Result) -> crate::DeviceError {
    #[cfg(feature = "internal_error_panic")]
    panic!("Unexpected Vulkan error: {_err:?}");

    #[allow(unreachable_code)]
    crate::DeviceError::Unexpected
}

/// Returns [`crate::DeviceError::OutOfMemory`].
fn get_oom_err(_err: vk::Result) -> crate::DeviceError {
    crate::DeviceError::OutOfMemory
}

/// Returns [`crate::DeviceError::Lost`] or panics if the `device_lost_panic`
/// feature flag is enabled.
fn get_lost_err() -> crate::DeviceError {
    #[cfg(feature = "device_lost_panic")]
    panic!("Device lost");

    #[allow(unreachable_code)]
    crate::DeviceError::Lost
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct RawTlasInstance {
    transform: [f32; 12],
    custom_data_and_mask: u32,
    shader_binding_table_record_offset_and_flags: u32,
    acceleration_structure_reference: u64,
}

/// Arguments to the [`CreateDeviceCallback`].
pub struct CreateDeviceCallbackArgs<'arg, 'pnext, 'this>
where
    'this: 'pnext,
{
    /// The extensions to enable for the device. You must not remove anything from this list,
    /// but you may add to it.
    pub extensions: &'arg mut Vec<&'static CStr>,
    /// The physical device features to enable. You may enable features, but must not disable any.
    pub device_features: &'arg mut PhysicalDeviceFeatures,
    /// The queue create infos for the device. You may add or modify queue create infos as needed.
    pub queue_create_infos: &'arg mut Vec<vk::DeviceQueueCreateInfo<'pnext>>,
    /// The create info for the device. You may add or modify things in the pnext chain, but
    /// do not turn features off. Additionally, do not add things to the list of extensions,
    /// or to the feature set, as all changes to that member will be overwritten.
    pub create_info: &'arg mut vk::DeviceCreateInfo<'pnext>,
    /// We need to have `'this` in the struct, so we can declare that all lifetimes coming from
    /// captures in the closure will live longer (and hence satisfy) `'pnext`. However, we
    /// don't actually directly use `'this`
    _phantom: PhantomData<&'this ()>,
}

/// Callback to allow changing the vulkan device creation parameters.
///
/// # Safety:
/// - If you want to add extensions, add the to the `Vec<'static CStr>` not the create info,
///   as the create info value will be overwritten.
/// - Callback must not remove features.
/// - Callback must not change anything to what the instance does not support.
pub type CreateDeviceCallback<'this> =
    dyn for<'arg, 'pnext> FnOnce(CreateDeviceCallbackArgs<'arg, 'pnext, 'this>) + 'this;

/// Arguments to the [`CreateInstanceCallback`].
pub struct CreateInstanceCallbackArgs<'arg, 'pnext, 'this>
where
    'this: 'pnext,
{
    /// The extensions to enable for the instance. You must not remove anything from this list,
    /// but you may add to it.
    pub extensions: &'arg mut Vec<&'static CStr>,
    /// The create info for the instance. You may add or modify things in the pnext chain, but
    /// do not turn features off. Additionally, do not add things to the list of extensions,
    /// all changes to that member will be overwritten.
    pub create_info: &'arg mut vk::InstanceCreateInfo<'pnext>,
    /// Vulkan entry point.
    pub entry: &'arg ash::Entry,
    /// We need to have `'this` in the struct, so we can declare that all lifetimes coming from
    /// captures in the closure will live longer (and hence satisfy) `'pnext`. However, we
    /// don't actually directly use `'this`
    _phantom: PhantomData<&'this ()>,
}

/// Callback to allow changing the vulkan instance creation parameters.
///
/// # Safety:
/// - If you want to add extensions, add the to the `Vec<'static CStr>` not the create info,
///   as the create info value will be overwritten.
/// - Callback must not remove features.
/// - Callback must not change anything to what the instance does not support.
pub type CreateInstanceCallback<'this> =
    dyn for<'arg, 'pnext> FnOnce(CreateInstanceCallbackArgs<'arg, 'pnext, 'this>) + 'this;
