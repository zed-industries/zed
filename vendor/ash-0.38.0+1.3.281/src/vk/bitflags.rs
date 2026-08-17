use crate::vk::definitions::*;
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html>"]
pub struct PipelineCacheCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineCacheCreateFlags, Flags);
impl PipelineCacheCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html>"]
pub struct QueueFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueueFlags, Flags);
impl QueueFlags {
    #[doc = "Queue supports graphics operations"]
    pub const GRAPHICS: Self = Self(0b1);
    #[doc = "Queue supports compute operations"]
    pub const COMPUTE: Self = Self(0b10);
    #[doc = "Queue supports transfer operations"]
    pub const TRANSFER: Self = Self(0b100);
    #[doc = "Queue supports sparse resource memory management operations"]
    pub const SPARSE_BINDING: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html>"]
pub struct CullModeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CullModeFlags, Flags);
impl CullModeFlags {
    pub const NONE: Self = Self(0);
    pub const FRONT: Self = Self(0b1);
    pub const BACK: Self = Self(0b10);
    pub const FRONT_AND_BACK: Self = Self(0x0000_0003);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateFlagBits.html>"]
pub struct RenderPassCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(RenderPassCreateFlags, Flags);
impl RenderPassCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html>"]
pub struct DeviceQueueCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DeviceQueueCreateFlags, Flags);
impl DeviceQueueCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html>"]
pub struct MemoryPropertyFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryPropertyFlags, Flags);
impl MemoryPropertyFlags {
    #[doc = "If otherwise stated, then allocate memory on device"]
    pub const DEVICE_LOCAL: Self = Self(0b1);
    #[doc = "Memory is mappable by host"]
    pub const HOST_VISIBLE: Self = Self(0b10);
    #[doc = "Memory will have i/o coherency. If not set, application may need to use vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges to flush/invalidate host cache"]
    pub const HOST_COHERENT: Self = Self(0b100);
    #[doc = "Memory will be cached by the host"]
    pub const HOST_CACHED: Self = Self(0b1000);
    #[doc = "Memory may be allocated by the driver when it is required"]
    pub const LAZILY_ALLOCATED: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html>"]
pub struct MemoryHeapFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryHeapFlags, Flags);
impl MemoryHeapFlags {
    #[doc = "If set, heap represents device memory"]
    pub const DEVICE_LOCAL: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits.html>"]
pub struct AccessFlags(pub(crate) Flags);
vk_bitflags_wrapped!(AccessFlags, Flags);
impl AccessFlags {
    #[doc = "Controls coherency of indirect command reads"]
    pub const INDIRECT_COMMAND_READ: Self = Self(0b1);
    #[doc = "Controls coherency of index reads"]
    pub const INDEX_READ: Self = Self(0b10);
    #[doc = "Controls coherency of vertex attribute reads"]
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(0b100);
    #[doc = "Controls coherency of uniform buffer reads"]
    pub const UNIFORM_READ: Self = Self(0b1000);
    #[doc = "Controls coherency of input attachment reads"]
    pub const INPUT_ATTACHMENT_READ: Self = Self(0b1_0000);
    #[doc = "Controls coherency of shader reads"]
    pub const SHADER_READ: Self = Self(0b10_0000);
    #[doc = "Controls coherency of shader writes"]
    pub const SHADER_WRITE: Self = Self(0b100_0000);
    #[doc = "Controls coherency of color attachment reads"]
    pub const COLOR_ATTACHMENT_READ: Self = Self(0b1000_0000);
    #[doc = "Controls coherency of color attachment writes"]
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(0b1_0000_0000);
    #[doc = "Controls coherency of depth/stencil attachment reads"]
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(0b10_0000_0000);
    #[doc = "Controls coherency of depth/stencil attachment writes"]
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(0b100_0000_0000);
    #[doc = "Controls coherency of transfer reads"]
    pub const TRANSFER_READ: Self = Self(0b1000_0000_0000);
    #[doc = "Controls coherency of transfer writes"]
    pub const TRANSFER_WRITE: Self = Self(0b1_0000_0000_0000);
    #[doc = "Controls coherency of host reads"]
    pub const HOST_READ: Self = Self(0b10_0000_0000_0000);
    #[doc = "Controls coherency of host writes"]
    pub const HOST_WRITE: Self = Self(0b100_0000_0000_0000);
    #[doc = "Controls coherency of memory reads"]
    pub const MEMORY_READ: Self = Self(0b1000_0000_0000_0000);
    #[doc = "Controls coherency of memory writes"]
    pub const MEMORY_WRITE: Self = Self(0b1_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html>"]
pub struct BufferUsageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(BufferUsageFlags, Flags);
impl BufferUsageFlags {
    #[doc = "Can be used as a source of transfer operations"]
    pub const TRANSFER_SRC: Self = Self(0b1);
    #[doc = "Can be used as a destination of transfer operations"]
    pub const TRANSFER_DST: Self = Self(0b10);
    #[doc = "Can be used as TBO"]
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0b100);
    #[doc = "Can be used as IBO"]
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0b1000);
    #[doc = "Can be used as UBO"]
    pub const UNIFORM_BUFFER: Self = Self(0b1_0000);
    #[doc = "Can be used as SSBO"]
    pub const STORAGE_BUFFER: Self = Self(0b10_0000);
    #[doc = "Can be used as source of fixed-function index fetch (index buffer)"]
    pub const INDEX_BUFFER: Self = Self(0b100_0000);
    #[doc = "Can be used as source of fixed-function vertex fetch (VBO)"]
    pub const VERTEX_BUFFER: Self = Self(0b1000_0000);
    #[doc = "Can be the source of indirect parameters (e.g. indirect buffer, parameter buffer)"]
    pub const INDIRECT_BUFFER: Self = Self(0b1_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits2KHR.html>"]
pub struct BufferUsageFlags2KHR(pub(crate) Flags64);
vk_bitflags_wrapped!(BufferUsageFlags2KHR, Flags64);
impl BufferUsageFlags2KHR {
    pub const TRANSFER_SRC: Self = Self(0b1);
    pub const TRANSFER_DST: Self = Self(0b10);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0b100);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0b1000);
    pub const UNIFORM_BUFFER: Self = Self(0b1_0000);
    pub const STORAGE_BUFFER: Self = Self(0b10_0000);
    pub const INDEX_BUFFER: Self = Self(0b100_0000);
    pub const VERTEX_BUFFER: Self = Self(0b1000_0000);
    pub const INDIRECT_BUFFER: Self = Self(0b1_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html>"]
pub struct BufferCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(BufferCreateFlags, Flags);
impl BufferCreateFlags {
    #[doc = "Buffer should support sparse backing"]
    pub const SPARSE_BINDING: Self = Self(0b1);
    #[doc = "Buffer should support sparse backing with partial residency"]
    pub const SPARSE_RESIDENCY: Self = Self(0b10);
    #[doc = "Buffer should support constant data access to physical memory ranges mapped into multiple locations of sparse buffers"]
    pub const SPARSE_ALIASED: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html>"]
pub struct ShaderStageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ShaderStageFlags, Flags);
impl ShaderStageFlags {
    pub const VERTEX: Self = Self(0b1);
    pub const TESSELLATION_CONTROL: Self = Self(0b10);
    pub const TESSELLATION_EVALUATION: Self = Self(0b100);
    pub const GEOMETRY: Self = Self(0b1000);
    pub const FRAGMENT: Self = Self(0b1_0000);
    pub const COMPUTE: Self = Self(0b10_0000);
    pub const ALL_GRAPHICS: Self = Self(0x0000_001F);
    pub const ALL: Self = Self(0x7FFF_FFFF);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html>"]
pub struct ImageUsageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageUsageFlags, Flags);
impl ImageUsageFlags {
    #[doc = "Can be used as a source of transfer operations"]
    pub const TRANSFER_SRC: Self = Self(0b1);
    #[doc = "Can be used as a destination of transfer operations"]
    pub const TRANSFER_DST: Self = Self(0b10);
    #[doc = "Can be sampled from (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)"]
    pub const SAMPLED: Self = Self(0b100);
    #[doc = "Can be used as storage image (STORAGE_IMAGE descriptor type)"]
    pub const STORAGE: Self = Self(0b1000);
    #[doc = "Can be used as framebuffer color attachment"]
    pub const COLOR_ATTACHMENT: Self = Self(0b1_0000);
    #[doc = "Can be used as framebuffer depth/stencil attachment"]
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(0b10_0000);
    #[doc = "Image data not needed outside of rendering"]
    pub const TRANSIENT_ATTACHMENT: Self = Self(0b100_0000);
    #[doc = "Can be used as framebuffer input attachment"]
    pub const INPUT_ATTACHMENT: Self = Self(0b1000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html>"]
pub struct ImageCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageCreateFlags, Flags);
impl ImageCreateFlags {
    #[doc = "Image should support sparse backing"]
    pub const SPARSE_BINDING: Self = Self(0b1);
    #[doc = "Image should support sparse backing with partial residency"]
    pub const SPARSE_RESIDENCY: Self = Self(0b10);
    #[doc = "Image should support constant data access to physical memory ranges mapped into multiple locations of sparse images"]
    pub const SPARSE_ALIASED: Self = Self(0b100);
    #[doc = "Allows image views to have different format than the base image"]
    pub const MUTABLE_FORMAT: Self = Self(0b1000);
    #[doc = "Allows creating image views with cube type from the created image"]
    pub const CUBE_COMPATIBLE: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html>"]
pub struct ImageViewCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageViewCreateFlags, Flags);
impl ImageViewCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html>"]
pub struct SamplerCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SamplerCreateFlags, Flags);
impl SamplerCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreateFlagBits.html>"]
pub struct PipelineCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineCreateFlags, Flags);
impl PipelineCreateFlags {
    pub const DISABLE_OPTIMIZATION: Self = Self(0b1);
    pub const ALLOW_DERIVATIVES: Self = Self(0b10);
    pub const DERIVATIVE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreateFlagBits2KHR.html>"]
pub struct PipelineCreateFlags2KHR(pub(crate) Flags64);
vk_bitflags_wrapped!(PipelineCreateFlags2KHR, Flags64);
impl PipelineCreateFlags2KHR {
    pub const DISABLE_OPTIMIZATION: Self = Self(0b1);
    pub const ALLOW_DERIVATIVES: Self = Self(0b10);
    pub const DERIVATIVE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateFlagBits.html>"]
pub struct PipelineShaderStageCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineShaderStageCreateFlags, Flags);
impl PipelineShaderStageCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html>"]
pub struct ColorComponentFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ColorComponentFlags, Flags);
impl ColorComponentFlags {
    pub const R: Self = Self(0b1);
    pub const G: Self = Self(0b10);
    pub const B: Self = Self(0b100);
    pub const A: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlagBits.html>"]
pub struct FenceCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FenceCreateFlags, Flags);
impl FenceCreateFlags {
    pub const SIGNALED: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateFlagBits.html>"]
pub struct SemaphoreCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SemaphoreCreateFlags, Flags);
impl SemaphoreCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits.html>"]
pub struct FormatFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FormatFeatureFlags, Flags);
impl FormatFeatureFlags {
    #[doc = "Format can be used for sampled images (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)"]
    pub const SAMPLED_IMAGE: Self = Self(0b1);
    #[doc = "Format can be used for storage images (STORAGE_IMAGE descriptor type)"]
    pub const STORAGE_IMAGE: Self = Self(0b10);
    #[doc = "Format supports atomic operations in case it is used for storage images"]
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(0b100);
    #[doc = "Format can be used for uniform texel buffers (TBOs)"]
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0b1000);
    #[doc = "Format can be used for storage texel buffers (IBOs)"]
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0b1_0000);
    #[doc = "Format supports atomic operations in case it is used for storage texel buffers"]
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(0b10_0000);
    #[doc = "Format can be used for vertex buffers (VBOs)"]
    pub const VERTEX_BUFFER: Self = Self(0b100_0000);
    #[doc = "Format can be used for color attachment images"]
    pub const COLOR_ATTACHMENT: Self = Self(0b1000_0000);
    #[doc = "Format supports blending in case it is used for color attachment images"]
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(0b1_0000_0000);
    #[doc = "Format can be used for depth/stencil attachment images"]
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(0b10_0000_0000);
    #[doc = "Format can be used as the source image of blits with vkCmdBlitImage"]
    pub const BLIT_SRC: Self = Self(0b100_0000_0000);
    #[doc = "Format can be used as the destination image of blits with vkCmdBlitImage"]
    pub const BLIT_DST: Self = Self(0b1000_0000_0000);
    #[doc = "Format can be filtered with VK_FILTER_LINEAR when being sampled"]
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(0b1_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlagBits.html>"]
pub struct QueryControlFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryControlFlags, Flags);
impl QueryControlFlags {
    #[doc = "Require precise results to be collected by the query"]
    pub const PRECISE: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlagBits.html>"]
pub struct QueryResultFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryResultFlags, Flags);
impl QueryResultFlags {
    #[doc = "Results of the queries are written to the destination buffer as 64-bit values"]
    pub const TYPE_64: Self = Self(0b1);
    #[doc = "Results of the queries are waited on before proceeding with the result copy"]
    pub const WAIT: Self = Self(0b10);
    #[doc = "Besides the results of the query, the availability of the results is also written"]
    pub const WITH_AVAILABILITY: Self = Self(0b100);
    #[doc = "Copy the partial results of the query even if the final results are not available"]
    pub const PARTIAL: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html>"]
pub struct CommandBufferUsageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandBufferUsageFlags, Flags);
impl CommandBufferUsageFlags {
    pub const ONE_TIME_SUBMIT: Self = Self(0b1);
    pub const RENDER_PASS_CONTINUE: Self = Self(0b10);
    #[doc = "Command buffer may be submitted/executed more than once simultaneously"]
    pub const SIMULTANEOUS_USE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html>"]
pub struct QueryPipelineStatisticFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryPipelineStatisticFlags, Flags);
impl QueryPipelineStatisticFlags {
    #[doc = "Optional"]
    pub const INPUT_ASSEMBLY_VERTICES: Self = Self(0b1);
    #[doc = "Optional"]
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self = Self(0b10);
    #[doc = "Optional"]
    pub const VERTEX_SHADER_INVOCATIONS: Self = Self(0b100);
    #[doc = "Optional"]
    pub const GEOMETRY_SHADER_INVOCATIONS: Self = Self(0b1000);
    #[doc = "Optional"]
    pub const GEOMETRY_SHADER_PRIMITIVES: Self = Self(0b1_0000);
    #[doc = "Optional"]
    pub const CLIPPING_INVOCATIONS: Self = Self(0b10_0000);
    #[doc = "Optional"]
    pub const CLIPPING_PRIMITIVES: Self = Self(0b100_0000);
    #[doc = "Optional"]
    pub const FRAGMENT_SHADER_INVOCATIONS: Self = Self(0b1000_0000);
    #[doc = "Optional"]
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = Self(0b1_0000_0000);
    #[doc = "Optional"]
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = Self(0b10_0000_0000);
    #[doc = "Optional"]
    pub const COMPUTE_SHADER_INVOCATIONS: Self = Self(0b100_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryMapFlagBits.html>"]
pub struct MemoryMapFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryMapFlags, Flags);
impl MemoryMapFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html>"]
pub struct ImageAspectFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageAspectFlags, Flags);
impl ImageAspectFlags {
    pub const COLOR: Self = Self(0b1);
    pub const DEPTH: Self = Self(0b10);
    pub const STENCIL: Self = Self(0b100);
    pub const METADATA: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatFlagBits.html>"]
pub struct SparseImageFormatFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SparseImageFormatFlags, Flags);
impl SparseImageFormatFlags {
    #[doc = "Image uses a single mip tail region for all array layers"]
    pub const SINGLE_MIPTAIL: Self = Self(0b1);
    #[doc = "Image requires mip level dimensions to be an integer multiple of the sparse image block dimensions for non-tail mip levels."]
    pub const ALIGNED_MIP_SIZE: Self = Self(0b10);
    #[doc = "Image uses a non-standard sparse image block dimensions"]
    pub const NONSTANDARD_BLOCK_SIZE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBindFlagBits.html>"]
pub struct SparseMemoryBindFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SparseMemoryBindFlags, Flags);
impl SparseMemoryBindFlags {
    #[doc = "Operation binds resource metadata to memory"]
    pub const METADATA: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html>"]
pub struct PipelineStageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineStageFlags, Flags);
impl PipelineStageFlags {
    #[doc = "Before subsequent commands are processed"]
    pub const TOP_OF_PIPE: Self = Self(0b1);
    #[doc = "Draw/DispatchIndirect command fetch"]
    pub const DRAW_INDIRECT: Self = Self(0b10);
    #[doc = "Vertex/index fetch"]
    pub const VERTEX_INPUT: Self = Self(0b100);
    #[doc = "Vertex shading"]
    pub const VERTEX_SHADER: Self = Self(0b1000);
    #[doc = "Tessellation control shading"]
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(0b1_0000);
    #[doc = "Tessellation evaluation shading"]
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(0b10_0000);
    #[doc = "Geometry shading"]
    pub const GEOMETRY_SHADER: Self = Self(0b100_0000);
    #[doc = "Fragment shading"]
    pub const FRAGMENT_SHADER: Self = Self(0b1000_0000);
    #[doc = "Early fragment (depth and stencil) tests"]
    pub const EARLY_FRAGMENT_TESTS: Self = Self(0b1_0000_0000);
    #[doc = "Late fragment (depth and stencil) tests"]
    pub const LATE_FRAGMENT_TESTS: Self = Self(0b10_0000_0000);
    #[doc = "Color attachment writes"]
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(0b100_0000_0000);
    #[doc = "Compute shading"]
    pub const COMPUTE_SHADER: Self = Self(0b1000_0000_0000);
    #[doc = "Transfer/copy operations"]
    pub const TRANSFER: Self = Self(0b1_0000_0000_0000);
    #[doc = "After previous commands have completed"]
    pub const BOTTOM_OF_PIPE: Self = Self(0b10_0000_0000_0000);
    #[doc = "Indicates host (CPU) is a source/sink of the dependency"]
    pub const HOST: Self = Self(0b100_0000_0000_0000);
    #[doc = "All stages of the graphics pipeline"]
    pub const ALL_GRAPHICS: Self = Self(0b1000_0000_0000_0000);
    #[doc = "All stages supported on the queue"]
    pub const ALL_COMMANDS: Self = Self(0b1_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlagBits.html>"]
pub struct CommandPoolCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandPoolCreateFlags, Flags);
impl CommandPoolCreateFlags {
    #[doc = "Command buffers have a short lifetime"]
    pub const TRANSIENT: Self = Self(0b1);
    #[doc = "Command buffers may release their memory individually"]
    pub const RESET_COMMAND_BUFFER: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlagBits.html>"]
pub struct CommandPoolResetFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandPoolResetFlags, Flags);
impl CommandPoolResetFlags {
    #[doc = "Release resources owned by the pool"]
    pub const RELEASE_RESOURCES: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferResetFlagBits.html>"]
pub struct CommandBufferResetFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandBufferResetFlags, Flags);
impl CommandBufferResetFlags {
    #[doc = "Release resources owned by the buffer"]
    pub const RELEASE_RESOURCES: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html>"]
pub struct SampleCountFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SampleCountFlags, Flags);
impl SampleCountFlags {
    #[doc = "Sample count 1 supported"]
    pub const TYPE_1: Self = Self(0b1);
    #[doc = "Sample count 2 supported"]
    pub const TYPE_2: Self = Self(0b10);
    #[doc = "Sample count 4 supported"]
    pub const TYPE_4: Self = Self(0b100);
    #[doc = "Sample count 8 supported"]
    pub const TYPE_8: Self = Self(0b1000);
    #[doc = "Sample count 16 supported"]
    pub const TYPE_16: Self = Self(0b1_0000);
    #[doc = "Sample count 32 supported"]
    pub const TYPE_32: Self = Self(0b10_0000);
    #[doc = "Sample count 64 supported"]
    pub const TYPE_64: Self = Self(0b100_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescriptionFlagBits.html>"]
pub struct AttachmentDescriptionFlags(pub(crate) Flags);
vk_bitflags_wrapped!(AttachmentDescriptionFlags, Flags);
impl AttachmentDescriptionFlags {
    #[doc = "The attachment may alias physical memory of another attachment in the same render pass"]
    pub const MAY_ALIAS: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStencilFaceFlagBits.html>"]
pub struct StencilFaceFlags(pub(crate) Flags);
vk_bitflags_wrapped!(StencilFaceFlags, Flags);
impl StencilFaceFlags {
    #[doc = "Front face"]
    pub const FRONT: Self = Self(0b1);
    #[doc = "Back face"]
    pub const BACK: Self = Self(0b10);
    #[doc = "Front and back faces"]
    pub const FRONT_AND_BACK: Self = Self(0x0000_0003);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateFlagBits.html>"]
pub struct DescriptorPoolCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DescriptorPoolCreateFlags, Flags);
impl DescriptorPoolCreateFlags {
    #[doc = "Descriptor sets may be freed individually"]
    pub const FREE_DESCRIPTOR_SET: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html>"]
pub struct DependencyFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DependencyFlags, Flags);
impl DependencyFlags {
    #[doc = "Dependency is per pixel region "]
    pub const BY_REGION: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagBits.html>"]
pub struct SemaphoreWaitFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SemaphoreWaitFlags, Flags);
impl SemaphoreWaitFlags {
    pub const ANY: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html>"]
pub struct DisplayPlaneAlphaFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(DisplayPlaneAlphaFlagsKHR, Flags);
impl DisplayPlaneAlphaFlagsKHR {
    pub const OPAQUE: Self = Self(0b1);
    pub const GLOBAL: Self = Self(0b10);
    pub const PER_PIXEL: Self = Self(0b100);
    pub const PER_PIXEL_PREMULTIPLIED: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html>"]
pub struct CompositeAlphaFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(CompositeAlphaFlagsKHR, Flags);
impl CompositeAlphaFlagsKHR {
    pub const OPAQUE: Self = Self(0b1);
    pub const PRE_MULTIPLIED: Self = Self(0b10);
    pub const POST_MULTIPLIED: Self = Self(0b100);
    pub const INHERIT: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html>"]
pub struct SurfaceTransformFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(SurfaceTransformFlagsKHR, Flags);
impl SurfaceTransformFlagsKHR {
    pub const IDENTITY: Self = Self(0b1);
    pub const ROTATE_90: Self = Self(0b10);
    pub const ROTATE_180: Self = Self(0b100);
    pub const ROTATE_270: Self = Self(0b1000);
    pub const HORIZONTAL_MIRROR: Self = Self(0b1_0000);
    pub const HORIZONTAL_MIRROR_ROTATE_90: Self = Self(0b10_0000);
    pub const HORIZONTAL_MIRROR_ROTATE_180: Self = Self(0b100_0000);
    pub const HORIZONTAL_MIRROR_ROTATE_270: Self = Self(0b1000_0000);
    pub const INHERIT: Self = Self(0b1_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainImageUsageFlagBitsANDROID.html>"]
pub struct SwapchainImageUsageFlagsANDROID(pub(crate) Flags);
vk_bitflags_wrapped!(SwapchainImageUsageFlagsANDROID, Flags);
impl SwapchainImageUsageFlagsANDROID {
    pub const SHARED: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportFlagBitsEXT.html>"]
pub struct DebugReportFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DebugReportFlagsEXT, Flags);
impl DebugReportFlagsEXT {
    pub const INFORMATION: Self = Self(0b1);
    pub const WARNING: Self = Self(0b10);
    pub const PERFORMANCE_WARNING: Self = Self(0b100);
    pub const ERROR: Self = Self(0b1000);
    pub const DEBUG: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html>"]
pub struct ExternalMemoryHandleTypeFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryHandleTypeFlagsNV, Flags);
impl ExternalMemoryHandleTypeFlagsNV {
    pub const OPAQUE_WIN32: Self = Self(0b1);
    pub const OPAQUE_WIN32_KMT: Self = Self(0b10);
    pub const D3D11_IMAGE: Self = Self(0b100);
    pub const D3D11_IMAGE_KMT: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html>"]
pub struct ExternalMemoryFeatureFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryFeatureFlagsNV, Flags);
impl ExternalMemoryFeatureFlagsNV {
    pub const DEDICATED_ONLY: Self = Self(0b1);
    pub const EXPORTABLE: Self = Self(0b10);
    pub const IMPORTABLE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubgroupFeatureFlagBits.html>"]
pub struct SubgroupFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SubgroupFeatureFlags, Flags);
impl SubgroupFeatureFlags {
    #[doc = "Basic subgroup operations"]
    pub const BASIC: Self = Self(0b1);
    #[doc = "Vote subgroup operations"]
    pub const VOTE: Self = Self(0b10);
    #[doc = "Arithmetic subgroup operations"]
    pub const ARITHMETIC: Self = Self(0b100);
    #[doc = "Ballot subgroup operations"]
    pub const BALLOT: Self = Self(0b1000);
    #[doc = "Shuffle subgroup operations"]
    pub const SHUFFLE: Self = Self(0b1_0000);
    #[doc = "Shuffle relative subgroup operations"]
    pub const SHUFFLE_RELATIVE: Self = Self(0b10_0000);
    #[doc = "Clustered subgroup operations"]
    pub const CLUSTERED: Self = Self(0b100_0000);
    #[doc = "Quad subgroup operations"]
    pub const QUAD: Self = Self(0b1000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html>"]
pub struct IndirectCommandsLayoutUsageFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(IndirectCommandsLayoutUsageFlagsNV, Flags);
impl IndirectCommandsLayoutUsageFlagsNV {
    pub const EXPLICIT_PREPROCESS: Self = Self(0b1);
    pub const INDEXED_SEQUENCES: Self = Self(0b10);
    pub const UNORDERED_SEQUENCES: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectStateFlagBitsNV.html>"]
pub struct IndirectStateFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(IndirectStateFlagsNV, Flags);
impl IndirectStateFlagsNV {
    pub const FLAG_FRONTFACE: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateFlagBits.html>"]
pub struct PrivateDataSlotCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PrivateDataSlotCreateFlags, Flags);
impl PrivateDataSlotCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html>"]
pub struct DescriptorSetLayoutCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DescriptorSetLayoutCreateFlags, Flags);
impl DescriptorSetLayoutCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html>"]
pub struct ExternalMemoryHandleTypeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryHandleTypeFlags, Flags);
impl ExternalMemoryHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(0b1);
    pub const OPAQUE_WIN32: Self = Self(0b10);
    pub const OPAQUE_WIN32_KMT: Self = Self(0b100);
    pub const D3D11_TEXTURE: Self = Self(0b1000);
    pub const D3D11_TEXTURE_KMT: Self = Self(0b1_0000);
    pub const D3D12_HEAP: Self = Self(0b10_0000);
    pub const D3D12_RESOURCE: Self = Self(0b100_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBits.html>"]
pub struct ExternalMemoryFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryFeatureFlags, Flags);
impl ExternalMemoryFeatureFlags {
    pub const DEDICATED_ONLY: Self = Self(0b1);
    pub const EXPORTABLE: Self = Self(0b10);
    pub const IMPORTABLE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html>"]
pub struct ExternalSemaphoreHandleTypeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalSemaphoreHandleTypeFlags, Flags);
impl ExternalSemaphoreHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(0b1);
    pub const OPAQUE_WIN32: Self = Self(0b10);
    pub const OPAQUE_WIN32_KMT: Self = Self(0b100);
    pub const D3D12_FENCE: Self = Self(0b1000);
    pub const D3D11_FENCE: Self = Self::D3D12_FENCE;
    pub const SYNC_FD: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html>"]
pub struct ExternalSemaphoreFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalSemaphoreFeatureFlags, Flags);
impl ExternalSemaphoreFeatureFlags {
    pub const EXPORTABLE: Self = Self(0b1);
    pub const IMPORTABLE: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreImportFlagBits.html>"]
pub struct SemaphoreImportFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SemaphoreImportFlags, Flags);
impl SemaphoreImportFlags {
    pub const TEMPORARY: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html>"]
pub struct ExternalFenceHandleTypeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalFenceHandleTypeFlags, Flags);
impl ExternalFenceHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(0b1);
    pub const OPAQUE_WIN32: Self = Self(0b10);
    pub const OPAQUE_WIN32_KMT: Self = Self(0b100);
    pub const SYNC_FD: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagBits.html>"]
pub struct ExternalFenceFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalFenceFeatureFlags, Flags);
impl ExternalFenceFeatureFlags {
    pub const EXPORTABLE: Self = Self(0b1);
    pub const IMPORTABLE: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagBits.html>"]
pub struct FenceImportFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FenceImportFlags, Flags);
impl FenceImportFlags {
    pub const TEMPORARY: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html>"]
pub struct SurfaceCounterFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(SurfaceCounterFlagsEXT, Flags);
impl SurfaceCounterFlagsEXT {
    pub const VBLANK: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagBits.html>"]
pub struct PeerMemoryFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PeerMemoryFeatureFlags, Flags);
impl PeerMemoryFeatureFlags {
    #[doc = "Can read with vkCmdCopy commands"]
    pub const COPY_SRC: Self = Self(0b1);
    #[doc = "Can write with vkCmdCopy commands"]
    pub const COPY_DST: Self = Self(0b10);
    #[doc = "Can read with any access type/command"]
    pub const GENERIC_SRC: Self = Self(0b100);
    #[doc = "Can write with and access type/command"]
    pub const GENERIC_DST: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBits.html>"]
pub struct MemoryAllocateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryAllocateFlags, Flags);
impl MemoryAllocateFlags {
    #[doc = "Force allocation on specific devices"]
    pub const DEVICE_MASK: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html>"]
pub struct DeviceGroupPresentModeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(DeviceGroupPresentModeFlagsKHR, Flags);
impl DeviceGroupPresentModeFlagsKHR {
    #[doc = "Present from local memory"]
    pub const LOCAL: Self = Self(0b1);
    #[doc = "Present from remote memory"]
    pub const REMOTE: Self = Self(0b10);
    #[doc = "Present sum of local and/or remote memory"]
    pub const SUM: Self = Self(0b100);
    #[doc = "Each physical device presents from local memory"]
    pub const LOCAL_MULTI_DEVICE: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html>"]
pub struct SwapchainCreateFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(SwapchainCreateFlagsKHR, Flags);
impl SwapchainCreateFlagsKHR {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionFlagBits.html>"]
pub struct SubpassDescriptionFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SubpassDescriptionFlags, Flags);
impl SubpassDescriptionFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html>"]
pub struct DebugUtilsMessageSeverityFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DebugUtilsMessageSeverityFlagsEXT, Flags);
impl DebugUtilsMessageSeverityFlagsEXT {
    pub const VERBOSE: Self = Self(0b1);
    pub const INFO: Self = Self(0b1_0000);
    pub const WARNING: Self = Self(0b1_0000_0000);
    pub const ERROR: Self = Self(0b1_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html>"]
pub struct DebugUtilsMessageTypeFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DebugUtilsMessageTypeFlagsEXT, Flags);
impl DebugUtilsMessageTypeFlagsEXT {
    pub const GENERAL: Self = Self(0b1);
    pub const VALIDATION: Self = Self(0b10);
    pub const PERFORMANCE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlagBits.html>"]
pub struct DescriptorBindingFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DescriptorBindingFlags, Flags);
impl DescriptorBindingFlags {
    pub const UPDATE_AFTER_BIND: Self = Self(0b1);
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(0b10);
    pub const PARTIALLY_BOUND: Self = Self(0b100);
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html>"]
pub struct ConditionalRenderingFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(ConditionalRenderingFlagsEXT, Flags);
impl ConditionalRenderingFlagsEXT {
    pub const INVERTED: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBits.html>"]
pub struct ResolveModeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ResolveModeFlags, Flags);
impl ResolveModeFlags {
    pub const NONE: Self = Self(0);
    pub const SAMPLE_ZERO: Self = Self(0b1);
    pub const AVERAGE: Self = Self(0b10);
    pub const MIN: Self = Self(0b100);
    pub const MAX: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html>"]
pub struct GeometryInstanceFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(GeometryInstanceFlagsKHR, Flags);
impl GeometryInstanceFlagsKHR {
    pub const TRIANGLE_FACING_CULL_DISABLE: Self = Self(0b1);
    pub const TRIANGLE_FLIP_FACING: Self = Self(0b10);
    pub const FORCE_OPAQUE: Self = Self(0b100);
    pub const FORCE_NO_OPAQUE: Self = Self(0b1000);
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE: Self = Self::TRIANGLE_FLIP_FACING;
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagBitsKHR.html>"]
pub struct GeometryFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(GeometryFlagsKHR, Flags);
impl GeometryFlagsKHR {
    pub const OPAQUE: Self = Self(0b1);
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html>"]
pub struct BuildAccelerationStructureFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(BuildAccelerationStructureFlagsKHR, Flags);
impl BuildAccelerationStructureFlagsKHR {
    pub const ALLOW_UPDATE: Self = Self(0b1);
    pub const ALLOW_COMPACTION: Self = Self(0b10);
    pub const PREFER_FAST_TRACE: Self = Self(0b100);
    pub const PREFER_FAST_BUILD: Self = Self(0b1000);
    pub const LOW_MEMORY: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html>"]
pub struct AccelerationStructureCreateFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(AccelerationStructureCreateFlagsKHR, Flags);
impl AccelerationStructureCreateFlagsKHR {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateFlagBits.html>"]
pub struct FramebufferCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FramebufferCreateFlags, Flags);
impl FramebufferCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html>"]
pub struct DeviceDiagnosticsConfigFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(DeviceDiagnosticsConfigFlagsNV, Flags);
impl DeviceDiagnosticsConfigFlagsNV {
    pub const ENABLE_SHADER_DEBUG_INFO: Self = Self(0b1);
    pub const ENABLE_RESOURCE_TRACKING: Self = Self(0b10);
    pub const ENABLE_AUTOMATIC_CHECKPOINTS: Self = Self(0b100);
    pub const ENABLE_SHADER_ERROR_REPORTING: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackFlagBits.html>"]
pub struct PipelineCreationFeedbackFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineCreationFeedbackFlags, Flags);
impl PipelineCreationFeedbackFlags {
    pub const VALID: Self = Self(0b1);
    pub const VALID_EXT: Self = Self::VALID;
    pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(0b10);
    pub const APPLICATION_PIPELINE_CACHE_HIT_EXT: Self = Self::APPLICATION_PIPELINE_CACHE_HIT;
    pub const BASE_PIPELINE_ACCELERATION: Self = Self(0b100);
    pub const BASE_PIPELINE_ACCELERATION_EXT: Self = Self::BASE_PIPELINE_ACCELERATION;
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryDecompressionMethodFlagBitsNV.html>"]
pub struct MemoryDecompressionMethodFlagsNV(pub(crate) Flags64);
vk_bitflags_wrapped!(MemoryDecompressionMethodFlagsNV, Flags64);
impl MemoryDecompressionMethodFlagsNV {
    pub const GDEFLATE_1_0: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html>"]
pub struct PerformanceCounterDescriptionFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(PerformanceCounterDescriptionFlagsKHR, Flags);
impl PerformanceCounterDescriptionFlagsKHR {
    pub const PERFORMANCE_IMPACTING: Self = Self(0b1);
    pub const CONCURRENTLY_IMPACTED: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockFlagBitsKHR.html>"]
pub struct AcquireProfilingLockFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(AcquireProfilingLockFlagsKHR, Flags);
impl AcquireProfilingLockFlagsKHR {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html>"]
pub struct ShaderCorePropertiesFlagsAMD(pub(crate) Flags);
vk_bitflags_wrapped!(ShaderCorePropertiesFlagsAMD, Flags);
impl ShaderCorePropertiesFlagsAMD {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateFlagBits.html>"]
pub struct ShaderModuleCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ShaderModuleCreateFlags, Flags);
impl ShaderModuleCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlFlagBitsAMD.html>"]
pub struct PipelineCompilerControlFlagsAMD(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineCompilerControlFlagsAMD, Flags);
impl PipelineCompilerControlFlagsAMD {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkToolPurposeFlagBits.html>"]
pub struct ToolPurposeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ToolPurposeFlags, Flags);
impl ToolPurposeFlags {
    pub const VALIDATION: Self = Self(0b1);
    pub const VALIDATION_EXT: Self = Self::VALIDATION;
    pub const PROFILING: Self = Self(0b10);
    pub const PROFILING_EXT: Self = Self::PROFILING;
    pub const TRACING: Self = Self(0b100);
    pub const TRACING_EXT: Self = Self::TRACING;
    pub const ADDITIONAL_FEATURES: Self = Self(0b1000);
    pub const ADDITIONAL_FEATURES_EXT: Self = Self::ADDITIONAL_FEATURES;
    pub const MODIFYING_FEATURES: Self = Self(0b1_0000);
    pub const MODIFYING_FEATURES_EXT: Self = Self::MODIFYING_FEATURES;
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits2.html>"]
pub struct AccessFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(AccessFlags2, Flags64);
impl AccessFlags2 {
    pub const NONE: Self = Self(0);
    pub const NONE_KHR: Self = Self::NONE;
    pub const INDIRECT_COMMAND_READ: Self = Self(0b1);
    pub const INDIRECT_COMMAND_READ_KHR: Self = Self::INDIRECT_COMMAND_READ;
    pub const INDEX_READ: Self = Self(0b10);
    pub const INDEX_READ_KHR: Self = Self::INDEX_READ;
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(0b100);
    pub const VERTEX_ATTRIBUTE_READ_KHR: Self = Self::VERTEX_ATTRIBUTE_READ;
    pub const UNIFORM_READ: Self = Self(0b1000);
    pub const UNIFORM_READ_KHR: Self = Self::UNIFORM_READ;
    pub const INPUT_ATTACHMENT_READ: Self = Self(0b1_0000);
    pub const INPUT_ATTACHMENT_READ_KHR: Self = Self::INPUT_ATTACHMENT_READ;
    pub const SHADER_READ: Self = Self(0b10_0000);
    pub const SHADER_READ_KHR: Self = Self::SHADER_READ;
    pub const SHADER_WRITE: Self = Self(0b100_0000);
    pub const SHADER_WRITE_KHR: Self = Self::SHADER_WRITE;
    pub const COLOR_ATTACHMENT_READ: Self = Self(0b1000_0000);
    pub const COLOR_ATTACHMENT_READ_KHR: Self = Self::COLOR_ATTACHMENT_READ;
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(0b1_0000_0000);
    pub const COLOR_ATTACHMENT_WRITE_KHR: Self = Self::COLOR_ATTACHMENT_WRITE;
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(0b10_0000_0000);
    pub const DEPTH_STENCIL_ATTACHMENT_READ_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_READ;
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(0b100_0000_0000);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_WRITE;
    pub const TRANSFER_READ: Self = Self(0b1000_0000_0000);
    pub const TRANSFER_READ_KHR: Self = Self::TRANSFER_READ;
    pub const TRANSFER_WRITE: Self = Self(0b1_0000_0000_0000);
    pub const TRANSFER_WRITE_KHR: Self = Self::TRANSFER_WRITE;
    pub const HOST_READ: Self = Self(0b10_0000_0000_0000);
    pub const HOST_READ_KHR: Self = Self::HOST_READ;
    pub const HOST_WRITE: Self = Self(0b100_0000_0000_0000);
    pub const HOST_WRITE_KHR: Self = Self::HOST_WRITE;
    pub const MEMORY_READ: Self = Self(0b1000_0000_0000_0000);
    pub const MEMORY_READ_KHR: Self = Self::MEMORY_READ;
    pub const MEMORY_WRITE: Self = Self(0b1_0000_0000_0000_0000);
    pub const MEMORY_WRITE_KHR: Self = Self::MEMORY_WRITE;
    pub const SHADER_SAMPLED_READ: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const SHADER_SAMPLED_READ_KHR: Self = Self::SHADER_SAMPLED_READ;
    pub const SHADER_STORAGE_READ: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const SHADER_STORAGE_READ_KHR: Self = Self::SHADER_STORAGE_READ;
    pub const SHADER_STORAGE_WRITE: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const SHADER_STORAGE_WRITE_KHR: Self = Self::SHADER_STORAGE_WRITE;
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits2.html>"]
pub struct PipelineStageFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(PipelineStageFlags2, Flags64);
impl PipelineStageFlags2 {
    pub const NONE: Self = Self(0);
    pub const NONE_KHR: Self = Self::NONE;
    pub const TOP_OF_PIPE: Self = Self(0b1);
    pub const TOP_OF_PIPE_KHR: Self = Self::TOP_OF_PIPE;
    pub const DRAW_INDIRECT: Self = Self(0b10);
    pub const DRAW_INDIRECT_KHR: Self = Self::DRAW_INDIRECT;
    pub const VERTEX_INPUT: Self = Self(0b100);
    pub const VERTEX_INPUT_KHR: Self = Self::VERTEX_INPUT;
    pub const VERTEX_SHADER: Self = Self(0b1000);
    pub const VERTEX_SHADER_KHR: Self = Self::VERTEX_SHADER;
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(0b1_0000);
    pub const TESSELLATION_CONTROL_SHADER_KHR: Self = Self::TESSELLATION_CONTROL_SHADER;
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(0b10_0000);
    pub const TESSELLATION_EVALUATION_SHADER_KHR: Self = Self::TESSELLATION_EVALUATION_SHADER;
    pub const GEOMETRY_SHADER: Self = Self(0b100_0000);
    pub const GEOMETRY_SHADER_KHR: Self = Self::GEOMETRY_SHADER;
    pub const FRAGMENT_SHADER: Self = Self(0b1000_0000);
    pub const FRAGMENT_SHADER_KHR: Self = Self::FRAGMENT_SHADER;
    pub const EARLY_FRAGMENT_TESTS: Self = Self(0b1_0000_0000);
    pub const EARLY_FRAGMENT_TESTS_KHR: Self = Self::EARLY_FRAGMENT_TESTS;
    pub const LATE_FRAGMENT_TESTS: Self = Self(0b10_0000_0000);
    pub const LATE_FRAGMENT_TESTS_KHR: Self = Self::LATE_FRAGMENT_TESTS;
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(0b100_0000_0000);
    pub const COLOR_ATTACHMENT_OUTPUT_KHR: Self = Self::COLOR_ATTACHMENT_OUTPUT;
    pub const COMPUTE_SHADER: Self = Self(0b1000_0000_0000);
    pub const COMPUTE_SHADER_KHR: Self = Self::COMPUTE_SHADER;
    pub const ALL_TRANSFER: Self = Self(0b1_0000_0000_0000);
    pub const ALL_TRANSFER_KHR: Self = Self::ALL_TRANSFER;
    pub const TRANSFER: Self = Self::ALL_TRANSFER_KHR;
    pub const TRANSFER_KHR: Self = Self::ALL_TRANSFER;
    pub const BOTTOM_OF_PIPE: Self = Self(0b10_0000_0000_0000);
    pub const BOTTOM_OF_PIPE_KHR: Self = Self::BOTTOM_OF_PIPE;
    pub const HOST: Self = Self(0b100_0000_0000_0000);
    pub const HOST_KHR: Self = Self::HOST;
    pub const ALL_GRAPHICS: Self = Self(0b1000_0000_0000_0000);
    pub const ALL_GRAPHICS_KHR: Self = Self::ALL_GRAPHICS;
    pub const ALL_COMMANDS: Self = Self(0b1_0000_0000_0000_0000);
    pub const ALL_COMMANDS_KHR: Self = Self::ALL_COMMANDS;
    pub const COPY: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const COPY_KHR: Self = Self::COPY;
    pub const RESOLVE: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const RESOLVE_KHR: Self = Self::RESOLVE;
    pub const BLIT: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const BLIT_KHR: Self = Self::BLIT;
    pub const CLEAR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const CLEAR_KHR: Self = Self::CLEAR;
    pub const INDEX_INPUT: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const INDEX_INPUT_KHR: Self = Self::INDEX_INPUT;
    pub const VERTEX_ATTRIBUTE_INPUT: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const VERTEX_ATTRIBUTE_INPUT_KHR: Self = Self::VERTEX_ATTRIBUTE_INPUT;
    pub const PRE_RASTERIZATION_SHADERS: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const PRE_RASTERIZATION_SHADERS_KHR: Self = Self::PRE_RASTERIZATION_SHADERS;
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubmitFlagBits.html>"]
pub struct SubmitFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SubmitFlags, Flags);
impl SubmitFlags {
    pub const PROTECTED: Self = Self(0b1);
    pub const PROTECTED_KHR: Self = Self::PROTECTED;
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEventCreateFlagBits.html>"]
pub struct EventCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(EventCreateFlags, Flags);
impl EventCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateFlagBits.html>"]
pub struct PipelineLayoutCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineLayoutCreateFlags, Flags);
impl PipelineLayoutCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateFlagBits.html>"]
pub struct PipelineColorBlendStateCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineColorBlendStateCreateFlags, Flags);
impl PipelineColorBlendStateCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlagBits.html>"]
pub struct PipelineDepthStencilStateCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineDepthStencilStateCreateFlags, Flags);
impl PipelineDepthStencilStateCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryFlagBitsEXT.html>"]
pub struct GraphicsPipelineLibraryFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(GraphicsPipelineLibraryFlagsEXT, Flags);
impl GraphicsPipelineLibraryFlagsEXT {
    pub const VERTEX_INPUT_INTERFACE: Self = Self(0b1);
    pub const PRE_RASTERIZATION_SHADERS: Self = Self(0b10);
    pub const FRAGMENT_SHADER: Self = Self(0b100);
    pub const FRAGMENT_OUTPUT_INTERFACE: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceAddressBindingFlagBitsEXT.html>"]
pub struct DeviceAddressBindingFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DeviceAddressBindingFlagsEXT, Flags);
impl DeviceAddressBindingFlagsEXT {
    pub const INTERNAL_OBJECT: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFrameBoundaryFlagBitsEXT.html>"]
pub struct FrameBoundaryFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(FrameBoundaryFlagsEXT, Flags);
impl FrameBoundaryFlagsEXT {
    pub const FRAME_END: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentScalingFlagBitsEXT.html>"]
pub struct PresentScalingFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(PresentScalingFlagsEXT, Flags);
impl PresentScalingFlagsEXT {
    pub const ONE_TO_ONE: Self = Self(0b1);
    pub const ASPECT_RATIO_STRETCH: Self = Self(0b10);
    pub const STRETCH: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentGravityFlagBitsEXT.html>"]
pub struct PresentGravityFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(PresentGravityFlagsEXT, Flags);
impl PresentGravityFlagsEXT {
    pub const MIN: Self = Self(0b1);
    pub const MAX: Self = Self(0b10);
    pub const CENTERED: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSchedulingControlsFlagBitsARM.html>"]
pub struct PhysicalDeviceSchedulingControlsFlagsARM(pub(crate) Flags64);
vk_bitflags_wrapped!(PhysicalDeviceSchedulingControlsFlagsARM, Flags64);
impl PhysicalDeviceSchedulingControlsFlagsARM {
    pub const SHADER_CORE_COUNT: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCodecOperationFlagBitsKHR.html>"]
pub struct VideoCodecOperationFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoCodecOperationFlagsKHR, Flags);
impl VideoCodecOperationFlagsKHR {
    pub const NONE: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoChromaSubsamplingFlagBitsKHR.html>"]
pub struct VideoChromaSubsamplingFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoChromaSubsamplingFlagsKHR, Flags);
impl VideoChromaSubsamplingFlagsKHR {
    pub const INVALID: Self = Self(0);
    pub const MONOCHROME: Self = Self(0b1);
    pub const TYPE_420: Self = Self(0b10);
    pub const TYPE_422: Self = Self(0b100);
    pub const TYPE_444: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoComponentBitDepthFlagBitsKHR.html>"]
pub struct VideoComponentBitDepthFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoComponentBitDepthFlagsKHR, Flags);
impl VideoComponentBitDepthFlagsKHR {
    pub const INVALID: Self = Self(0);
    pub const TYPE_8: Self = Self(0b1);
    pub const TYPE_10: Self = Self(0b100);
    pub const TYPE_12: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilityFlagBitsKHR.html>"]
pub struct VideoCapabilityFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoCapabilityFlagsKHR, Flags);
impl VideoCapabilityFlagsKHR {
    pub const PROTECTED_CONTENT: Self = Self(0b1);
    pub const SEPARATE_REFERENCE_IMAGES: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateFlagBitsKHR.html>"]
pub struct VideoSessionCreateFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoSessionCreateFlagsKHR, Flags);
impl VideoSessionCreateFlagsKHR {
    pub const PROTECTED_CONTENT: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264PictureLayoutFlagBitsKHR.html>"]
pub struct VideoDecodeH264PictureLayoutFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoDecodeH264PictureLayoutFlagsKHR, Flags);
impl VideoDecodeH264PictureLayoutFlagsKHR {
    pub const PROGRESSIVE: Self = Self(0);
    pub const INTERLACED_INTERLEAVED_LINES: Self = Self(0b1);
    pub const INTERLACED_SEPARATE_PLANES: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlFlagBitsKHR.html>"]
pub struct VideoCodingControlFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoCodingControlFlagsKHR, Flags);
impl VideoCodingControlFlagsKHR {
    pub const RESET: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeUsageFlagBitsKHR.html>"]
pub struct VideoDecodeUsageFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoDecodeUsageFlagsKHR, Flags);
impl VideoDecodeUsageFlagsKHR {
    pub const DEFAULT: Self = Self(0);
    pub const TRANSCODING: Self = Self(0b1);
    pub const OFFLINE: Self = Self(0b10);
    pub const STREAMING: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilityFlagBitsKHR.html>"]
pub struct VideoDecodeCapabilityFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoDecodeCapabilityFlagsKHR, Flags);
impl VideoDecodeCapabilityFlagsKHR {
    pub const DPB_AND_OUTPUT_COINCIDE: Self = Self(0b1);
    pub const DPB_AND_OUTPUT_DISTINCT: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeFlagBitsKHR.html>"]
pub struct VideoEncodeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeFlagsKHR, Flags);
impl VideoEncodeFlagsKHR {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeUsageFlagBitsKHR.html>"]
pub struct VideoEncodeUsageFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeUsageFlagsKHR, Flags);
impl VideoEncodeUsageFlagsKHR {
    pub const DEFAULT: Self = Self(0);
    pub const TRANSCODING: Self = Self(0b1);
    pub const STREAMING: Self = Self(0b10);
    pub const RECORDING: Self = Self(0b100);
    pub const CONFERENCING: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeContentFlagBitsKHR.html>"]
pub struct VideoEncodeContentFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeContentFlagsKHR, Flags);
impl VideoEncodeContentFlagsKHR {
    pub const DEFAULT: Self = Self(0);
    pub const CAMERA: Self = Self(0b1);
    pub const DESKTOP: Self = Self(0b10);
    pub const RENDERED: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeCapabilityFlagBitsKHR.html>"]
pub struct VideoEncodeCapabilityFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeCapabilityFlagsKHR, Flags);
impl VideoEncodeCapabilityFlagsKHR {
    pub const PRECEDING_EXTERNALLY_ENCODED_BYTES: Self = Self(0b1);
    pub const INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeFeedbackFlagBitsKHR.html>"]
pub struct VideoEncodeFeedbackFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeFeedbackFlagsKHR, Flags);
impl VideoEncodeFeedbackFlagsKHR {
    pub const BITSTREAM_BUFFER_OFFSET: Self = Self(0b1);
    pub const BITSTREAM_BYTES_WRITTEN: Self = Self(0b10);
    pub const BITSTREAM_HAS_OVERRIDES: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlModeFlagBitsKHR.html>"]
pub struct VideoEncodeRateControlModeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeRateControlModeFlagsKHR, Flags);
impl VideoEncodeRateControlModeFlagsKHR {
    pub const DEFAULT: Self = Self(0);
    pub const DISABLED: Self = Self(0b1);
    pub const CBR: Self = Self(0b10);
    pub const VBR: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CapabilityFlagBitsKHR.html>"]
pub struct VideoEncodeH264CapabilityFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH264CapabilityFlagsKHR, Flags);
impl VideoEncodeH264CapabilityFlagsKHR {
    pub const HRD_COMPLIANCE: Self = Self(0b1);
    pub const PREDICTION_WEIGHT_TABLE_GENERATED: Self = Self(0b10);
    pub const ROW_UNALIGNED_SLICE: Self = Self(0b100);
    pub const DIFFERENT_SLICE_TYPE: Self = Self(0b1000);
    pub const B_FRAME_IN_L0_LIST: Self = Self(0b1_0000);
    pub const B_FRAME_IN_L1_LIST: Self = Self(0b10_0000);
    pub const PER_PICTURE_TYPE_MIN_MAX_QP: Self = Self(0b100_0000);
    pub const PER_SLICE_CONSTANT_QP: Self = Self(0b1000_0000);
    pub const GENERATE_PREFIX_NALU: Self = Self(0b1_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264StdFlagBitsKHR.html>"]
pub struct VideoEncodeH264StdFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH264StdFlagsKHR, Flags);
impl VideoEncodeH264StdFlagsKHR {
    pub const SEPARATE_COLOR_PLANE_FLAG_SET: Self = Self(0b1);
    pub const QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET: Self = Self(0b10);
    pub const SCALING_MATRIX_PRESENT_FLAG_SET: Self = Self(0b100);
    pub const CHROMA_QP_INDEX_OFFSET: Self = Self(0b1000);
    pub const SECOND_CHROMA_QP_INDEX_OFFSET: Self = Self(0b1_0000);
    pub const PIC_INIT_QP_MINUS26: Self = Self(0b10_0000);
    pub const WEIGHTED_PRED_FLAG_SET: Self = Self(0b100_0000);
    pub const WEIGHTED_BIPRED_IDC_EXPLICIT: Self = Self(0b1000_0000);
    pub const WEIGHTED_BIPRED_IDC_IMPLICIT: Self = Self(0b1_0000_0000);
    pub const TRANSFORM_8X8_MODE_FLAG_SET: Self = Self(0b10_0000_0000);
    pub const DIRECT_SPATIAL_MV_PRED_FLAG_UNSET: Self = Self(0b100_0000_0000);
    pub const ENTROPY_CODING_MODE_FLAG_UNSET: Self = Self(0b1000_0000_0000);
    pub const ENTROPY_CODING_MODE_FLAG_SET: Self = Self(0b1_0000_0000_0000);
    pub const DIRECT_8X8_INFERENCE_FLAG_UNSET: Self = Self(0b10_0000_0000_0000);
    pub const CONSTRAINED_INTRA_PRED_FLAG_SET: Self = Self(0b100_0000_0000_0000);
    pub const DEBLOCKING_FILTER_DISABLED: Self = Self(0b1000_0000_0000_0000);
    pub const DEBLOCKING_FILTER_ENABLED: Self = Self(0b1_0000_0000_0000_0000);
    pub const DEBLOCKING_FILTER_PARTIAL: Self = Self(0b10_0000_0000_0000_0000);
    pub const SLICE_QP_DELTA: Self = Self(0b1000_0000_0000_0000_0000);
    pub const DIFFERENT_SLICE_QP_DELTA: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264RateControlFlagBitsKHR.html>"]
pub struct VideoEncodeH264RateControlFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH264RateControlFlagsKHR, Flags);
impl VideoEncodeH264RateControlFlagsKHR {
    pub const ATTEMPT_HRD_COMPLIANCE: Self = Self(0b1);
    pub const REGULAR_GOP: Self = Self(0b10);
    pub const REFERENCE_PATTERN_FLAT: Self = Self(0b100);
    pub const REFERENCE_PATTERN_DYADIC: Self = Self(0b1000);
    pub const TEMPORAL_LAYER_PATTERN_DYADIC: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkHostImageCopyFlagBitsEXT.html>"]
pub struct HostImageCopyFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(HostImageCopyFlagsEXT, Flags);
impl HostImageCopyFlagsEXT {
    pub const MEMCPY: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsFlagBitsFUCHSIA.html>"]
pub struct ImageFormatConstraintsFlagsFUCHSIA(pub(crate) Flags);
vk_bitflags_wrapped!(ImageFormatConstraintsFlagsFUCHSIA, Flags);
impl ImageFormatConstraintsFlagsFUCHSIA {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFlagBitsFUCHSIA.html>"]
pub struct ImageConstraintsInfoFlagsFUCHSIA(pub(crate) Flags);
vk_bitflags_wrapped!(ImageConstraintsInfoFlagsFUCHSIA, Flags);
impl ImageConstraintsInfoFlagsFUCHSIA {
    pub const CPU_READ_RARELY: Self = Self(0b1);
    pub const CPU_READ_OFTEN: Self = Self(0b10);
    pub const CPU_WRITE_RARELY: Self = Self(0b100);
    pub const CPU_WRITE_OFTEN: Self = Self(0b1000);
    pub const PROTECTED_OPTIONAL: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits2.html>"]
pub struct FormatFeatureFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(FormatFeatureFlags2, Flags64);
impl FormatFeatureFlags2 {
    pub const SAMPLED_IMAGE: Self = Self(0b1);
    pub const SAMPLED_IMAGE_KHR: Self = Self::SAMPLED_IMAGE;
    pub const STORAGE_IMAGE: Self = Self(0b10);
    pub const STORAGE_IMAGE_KHR: Self = Self::STORAGE_IMAGE;
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(0b100);
    pub const STORAGE_IMAGE_ATOMIC_KHR: Self = Self::STORAGE_IMAGE_ATOMIC;
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0b1000);
    pub const UNIFORM_TEXEL_BUFFER_KHR: Self = Self::UNIFORM_TEXEL_BUFFER;
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0b1_0000);
    pub const STORAGE_TEXEL_BUFFER_KHR: Self = Self::STORAGE_TEXEL_BUFFER;
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(0b10_0000);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_KHR: Self = Self::STORAGE_TEXEL_BUFFER_ATOMIC;
    pub const VERTEX_BUFFER: Self = Self(0b100_0000);
    pub const VERTEX_BUFFER_KHR: Self = Self::VERTEX_BUFFER;
    pub const COLOR_ATTACHMENT: Self = Self(0b1000_0000);
    pub const COLOR_ATTACHMENT_KHR: Self = Self::COLOR_ATTACHMENT;
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(0b1_0000_0000);
    pub const COLOR_ATTACHMENT_BLEND_KHR: Self = Self::COLOR_ATTACHMENT_BLEND;
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(0b10_0000_0000);
    pub const DEPTH_STENCIL_ATTACHMENT_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT;
    pub const BLIT_SRC: Self = Self(0b100_0000_0000);
    pub const BLIT_SRC_KHR: Self = Self::BLIT_SRC;
    pub const BLIT_DST: Self = Self(0b1000_0000_0000);
    pub const BLIT_DST_KHR: Self = Self::BLIT_DST;
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(0b1_0000_0000_0000);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_KHR: Self = Self::SAMPLED_IMAGE_FILTER_LINEAR;
    pub const SAMPLED_IMAGE_FILTER_CUBIC: Self = Self(0b10_0000_0000_0000);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC;
    pub const TRANSFER_SRC: Self = Self(0b100_0000_0000_0000);
    pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
    pub const TRANSFER_DST: Self = Self(0b1000_0000_0000_0000);
    pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
    pub const SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(0b1_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_FILTER_MINMAX_KHR: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
    pub const MIDPOINT_CHROMA_SAMPLES: Self = Self(0b10_0000_0000_0000_0000);
    pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(0b100_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self =
        Self(0b1000_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self =
        Self(0b1_0000_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
        Self(0b10_0000_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
    pub const DISJOINT: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const DISJOINT_KHR: Self = Self::DISJOINT;
    pub const COSITED_CHROMA_SAMPLES: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self::COSITED_CHROMA_SAMPLES;
    pub const STORAGE_READ_WITHOUT_FORMAT: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000);
    pub const STORAGE_READ_WITHOUT_FORMAT_KHR: Self = Self::STORAGE_READ_WITHOUT_FORMAT;
    pub const STORAGE_WRITE_WITHOUT_FORMAT: Self =
        Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const STORAGE_WRITE_WITHOUT_FORMAT_KHR: Self = Self::STORAGE_WRITE_WITHOUT_FORMAT;
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON_KHR: Self = Self::SAMPLED_IMAGE_DEPTH_COMPARISON;
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagBits.html>"]
pub struct RenderingFlags(pub(crate) Flags);
vk_bitflags_wrapped!(RenderingFlags, Flags);
impl RenderingFlags {
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(0b1);
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR: Self =
        Self::CONTENTS_SECONDARY_COMMAND_BUFFERS;
    pub const SUSPENDING: Self = Self(0b10);
    pub const SUSPENDING_KHR: Self = Self::SUSPENDING;
    pub const RESUMING: Self = Self(0b100);
    pub const RESUMING_KHR: Self = Self::RESUMING;
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CapabilityFlagBitsKHR.html>"]
pub struct VideoEncodeH265CapabilityFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH265CapabilityFlagsKHR, Flags);
impl VideoEncodeH265CapabilityFlagsKHR {
    pub const HRD_COMPLIANCE: Self = Self(0b1);
    pub const PREDICTION_WEIGHT_TABLE_GENERATED: Self = Self(0b10);
    pub const ROW_UNALIGNED_SLICE_SEGMENT: Self = Self(0b100);
    pub const DIFFERENT_SLICE_SEGMENT_TYPE: Self = Self(0b1000);
    pub const B_FRAME_IN_L0_LIST: Self = Self(0b1_0000);
    pub const B_FRAME_IN_L1_LIST: Self = Self(0b10_0000);
    pub const PER_PICTURE_TYPE_MIN_MAX_QP: Self = Self(0b100_0000);
    pub const PER_SLICE_SEGMENT_CONSTANT_QP: Self = Self(0b1000_0000);
    pub const MULTIPLE_TILES_PER_SLICE_SEGMENT: Self = Self(0b1_0000_0000);
    pub const MULTIPLE_SLICE_SEGMENTS_PER_TILE: Self = Self(0b10_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265StdFlagBitsKHR.html>"]
pub struct VideoEncodeH265StdFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH265StdFlagsKHR, Flags);
impl VideoEncodeH265StdFlagsKHR {
    pub const SEPARATE_COLOR_PLANE_FLAG_SET: Self = Self(0b1);
    pub const SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET: Self = Self(0b10);
    pub const SCALING_LIST_DATA_PRESENT_FLAG_SET: Self = Self(0b100);
    pub const PCM_ENABLED_FLAG_SET: Self = Self(0b1000);
    pub const SPS_TEMPORAL_MVP_ENABLED_FLAG_SET: Self = Self(0b1_0000);
    pub const INIT_QP_MINUS26: Self = Self(0b10_0000);
    pub const WEIGHTED_PRED_FLAG_SET: Self = Self(0b100_0000);
    pub const WEIGHTED_BIPRED_FLAG_SET: Self = Self(0b1000_0000);
    pub const LOG2_PARALLEL_MERGE_LEVEL_MINUS2: Self = Self(0b1_0000_0000);
    pub const SIGN_DATA_HIDING_ENABLED_FLAG_SET: Self = Self(0b10_0000_0000);
    pub const TRANSFORM_SKIP_ENABLED_FLAG_SET: Self = Self(0b100_0000_0000);
    pub const TRANSFORM_SKIP_ENABLED_FLAG_UNSET: Self = Self(0b1000_0000_0000);
    pub const PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET: Self = Self(0b1_0000_0000_0000);
    pub const TRANSQUANT_BYPASS_ENABLED_FLAG_SET: Self = Self(0b10_0000_0000_0000);
    pub const CONSTRAINED_INTRA_PRED_FLAG_SET: Self = Self(0b100_0000_0000_0000);
    pub const ENTROPY_CODING_SYNC_ENABLED_FLAG_SET: Self = Self(0b1000_0000_0000_0000);
    pub const DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET: Self = Self(0b1_0000_0000_0000_0000);
    pub const DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET: Self = Self(0b10_0000_0000_0000_0000);
    pub const DEPENDENT_SLICE_SEGMENT_FLAG_SET: Self = Self(0b100_0000_0000_0000_0000);
    pub const SLICE_QP_DELTA: Self = Self(0b1000_0000_0000_0000_0000);
    pub const DIFFERENT_SLICE_QP_DELTA: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265RateControlFlagBitsKHR.html>"]
pub struct VideoEncodeH265RateControlFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH265RateControlFlagsKHR, Flags);
impl VideoEncodeH265RateControlFlagsKHR {
    pub const ATTEMPT_HRD_COMPLIANCE: Self = Self(0b1);
    pub const REGULAR_GOP: Self = Self(0b10);
    pub const REFERENCE_PATTERN_FLAT: Self = Self(0b100);
    pub const REFERENCE_PATTERN_DYADIC: Self = Self(0b1000);
    pub const TEMPORAL_SUB_LAYER_PATTERN_DYADIC: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CtbSizeFlagBitsKHR.html>"]
pub struct VideoEncodeH265CtbSizeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH265CtbSizeFlagsKHR, Flags);
impl VideoEncodeH265CtbSizeFlagsKHR {
    pub const TYPE_16: Self = Self(0b1);
    pub const TYPE_32: Self = Self(0b10);
    pub const TYPE_64: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265TransformBlockSizeFlagBitsKHR.html>"]
pub struct VideoEncodeH265TransformBlockSizeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH265TransformBlockSizeFlagsKHR, Flags);
impl VideoEncodeH265TransformBlockSizeFlagsKHR {
    pub const TYPE_4: Self = Self(0b1);
    pub const TYPE_8: Self = Self(0b10);
    pub const TYPE_16: Self = Self(0b100);
    pub const TYPE_32: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectTypeFlagBitsEXT.html>"]
pub struct ExportMetalObjectTypeFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(ExportMetalObjectTypeFlagsEXT, Flags);
impl ExportMetalObjectTypeFlagsEXT {
    pub const METAL_DEVICE: Self = Self(0b1);
    pub const METAL_COMMAND_QUEUE: Self = Self(0b10);
    pub const METAL_BUFFER: Self = Self(0b100);
    pub const METAL_TEXTURE: Self = Self(0b1000);
    pub const METAL_IOSURFACE: Self = Self(0b1_0000);
    pub const METAL_SHARED_EVENT: Self = Self(0b10_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html>"]
pub struct InstanceCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(InstanceCreateFlags, Flags);
impl InstanceCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFlagBitsEXT.html>"]
pub struct ImageCompressionFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(ImageCompressionFlagsEXT, Flags);
impl ImageCompressionFlagsEXT {
    pub const DEFAULT: Self = Self(0);
    pub const FIXED_RATE_DEFAULT: Self = Self(0b1);
    pub const FIXED_RATE_EXPLICIT: Self = Self(0b10);
    pub const DISABLED: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFixedRateFlagBitsEXT.html>"]
pub struct ImageCompressionFixedRateFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(ImageCompressionFixedRateFlagsEXT, Flags);
impl ImageCompressionFixedRateFlagsEXT {
    pub const NONE: Self = Self(0);
    pub const TYPE_1BPC: Self = Self(0b1);
    pub const TYPE_2BPC: Self = Self(0b10);
    pub const TYPE_3BPC: Self = Self(0b100);
    pub const TYPE_4BPC: Self = Self(0b1000);
    pub const TYPE_5BPC: Self = Self(0b1_0000);
    pub const TYPE_6BPC: Self = Self(0b10_0000);
    pub const TYPE_7BPC: Self = Self(0b100_0000);
    pub const TYPE_8BPC: Self = Self(0b1000_0000);
    pub const TYPE_9BPC: Self = Self(0b1_0000_0000);
    pub const TYPE_10BPC: Self = Self(0b10_0000_0000);
    pub const TYPE_11BPC: Self = Self(0b100_0000_0000);
    pub const TYPE_12BPC: Self = Self(0b1000_0000_0000);
    pub const TYPE_13BPC: Self = Self(0b1_0000_0000_0000);
    pub const TYPE_14BPC: Self = Self(0b10_0000_0000_0000);
    pub const TYPE_15BPC: Self = Self(0b100_0000_0000_0000);
    pub const TYPE_16BPC: Self = Self(0b1000_0000_0000_0000);
    pub const TYPE_17BPC: Self = Self(0b1_0000_0000_0000_0000);
    pub const TYPE_18BPC: Self = Self(0b10_0000_0000_0000_0000);
    pub const TYPE_19BPC: Self = Self(0b100_0000_0000_0000_0000);
    pub const TYPE_20BPC: Self = Self(0b1000_0000_0000_0000_0000);
    pub const TYPE_21BPC: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const TYPE_22BPC: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const TYPE_23BPC: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const TYPE_24BPC: Self = Self(0b1000_0000_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowGridSizeFlagBitsNV.html>"]
pub struct OpticalFlowGridSizeFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(OpticalFlowGridSizeFlagsNV, Flags);
impl OpticalFlowGridSizeFlagsNV {
    pub const UNKNOWN: Self = Self(0);
    pub const TYPE_1X1: Self = Self(0b1);
    pub const TYPE_2X2: Self = Self(0b10);
    pub const TYPE_4X4: Self = Self(0b100);
    pub const TYPE_8X8: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowUsageFlagBitsNV.html>"]
pub struct OpticalFlowUsageFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(OpticalFlowUsageFlagsNV, Flags);
impl OpticalFlowUsageFlagsNV {
    pub const UNKNOWN: Self = Self(0);
    pub const INPUT: Self = Self(0b1);
    pub const OUTPUT: Self = Self(0b10);
    pub const HINT: Self = Self(0b100);
    pub const COST: Self = Self(0b1000);
    pub const GLOBAL_FLOW: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreateFlagBitsNV.html>"]
pub struct OpticalFlowSessionCreateFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(OpticalFlowSessionCreateFlagsNV, Flags);
impl OpticalFlowSessionCreateFlagsNV {
    pub const ENABLE_HINT: Self = Self(0b1);
    pub const ENABLE_COST: Self = Self(0b10);
    pub const ENABLE_GLOBAL_FLOW: Self = Self(0b100);
    pub const ALLOW_REGIONS: Self = Self(0b1000);
    pub const BOTH_DIRECTIONS: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowExecuteFlagBitsNV.html>"]
pub struct OpticalFlowExecuteFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(OpticalFlowExecuteFlagsNV, Flags);
impl OpticalFlowExecuteFlagsNV {
    pub const DISABLE_TEMPORAL_HINTS: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildMicromapFlagBitsEXT.html>"]
pub struct BuildMicromapFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(BuildMicromapFlagsEXT, Flags);
impl BuildMicromapFlagsEXT {
    pub const PREFER_FAST_TRACE: Self = Self(0b1);
    pub const PREFER_FAST_BUILD: Self = Self(0b10);
    pub const ALLOW_COMPACTION: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapCreateFlagBitsEXT.html>"]
pub struct MicromapCreateFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(MicromapCreateFlagsEXT, Flags);
impl MicromapCreateFlagsEXT {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderCreateFlagBitsEXT.html>"]
pub struct ShaderCreateFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(ShaderCreateFlagsEXT, Flags);
impl ShaderCreateFlagsEXT {
    pub const LINK_STAGE: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryUnmapFlagBitsKHR.html>"]
pub struct MemoryUnmapFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryUnmapFlagsKHR, Flags);
impl MemoryUnmapFlagsKHR {}
