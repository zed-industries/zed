use crate::vk::bitflags::*;
use crate::vk::enums::*;
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl BufferCreateFlags {
    #[doc = "Buffer requires protected memory"]
    pub const PROTECTED: Self = Self(0b1000);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl CommandPoolCreateFlags {
    #[doc = "Command buffers allocated from pool are protected command buffers"]
    pub const PROTECTED: Self = Self(0b100);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl DependencyFlags {
    #[doc = "Dependency is across devices"]
    pub const DEVICE_GROUP: Self = Self(0b100);
    pub const VIEW_LOCAL: Self = Self(0b10);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl DeviceQueueCreateFlags {
    #[doc = "Queue is a protected-capable device queue"]
    pub const PROTECTED: Self = Self(0b1);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl Format {
    pub const G8B8G8R8_422_UNORM: Self = Self(1_000_156_000);
    pub const B8G8R8G8_422_UNORM: Self = Self(1_000_156_001);
    pub const G8_B8_R8_3PLANE_420_UNORM: Self = Self(1_000_156_002);
    pub const G8_B8R8_2PLANE_420_UNORM: Self = Self(1_000_156_003);
    pub const G8_B8_R8_3PLANE_422_UNORM: Self = Self(1_000_156_004);
    pub const G8_B8R8_2PLANE_422_UNORM: Self = Self(1_000_156_005);
    pub const G8_B8_R8_3PLANE_444_UNORM: Self = Self(1_000_156_006);
    pub const R10X6_UNORM_PACK16: Self = Self(1_000_156_007);
    pub const R10X6G10X6_UNORM_2PACK16: Self = Self(1_000_156_008);
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Self(1_000_156_009);
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Self(1_000_156_010);
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Self(1_000_156_011);
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Self(1_000_156_012);
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Self(1_000_156_013);
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Self(1_000_156_014);
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Self(1_000_156_015);
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Self(1_000_156_016);
    pub const R12X4_UNORM_PACK16: Self = Self(1_000_156_017);
    pub const R12X4G12X4_UNORM_2PACK16: Self = Self(1_000_156_018);
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Self(1_000_156_019);
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Self(1_000_156_020);
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Self(1_000_156_021);
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Self(1_000_156_022);
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Self(1_000_156_023);
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Self(1_000_156_024);
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Self(1_000_156_025);
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Self(1_000_156_026);
    pub const G16B16G16R16_422_UNORM: Self = Self(1_000_156_027);
    pub const B16G16R16G16_422_UNORM: Self = Self(1_000_156_028);
    pub const G16_B16_R16_3PLANE_420_UNORM: Self = Self(1_000_156_029);
    pub const G16_B16R16_2PLANE_420_UNORM: Self = Self(1_000_156_030);
    pub const G16_B16_R16_3PLANE_422_UNORM: Self = Self(1_000_156_031);
    pub const G16_B16R16_2PLANE_422_UNORM: Self = Self(1_000_156_032);
    pub const G16_B16_R16_3PLANE_444_UNORM: Self = Self(1_000_156_033);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl FormatFeatureFlags {
    #[doc = "Format can be used as the source image of image transfer commands"]
    pub const TRANSFER_SRC: Self = Self(0b100_0000_0000_0000);
    #[doc = "Format can be used as the destination image of image transfer commands"]
    pub const TRANSFER_DST: Self = Self(0b1000_0000_0000_0000);
    #[doc = "Format can have midpoint rather than cosited chroma samples"]
    pub const MIDPOINT_CHROMA_SAMPLES: Self = Self(0b10_0000_0000_0000_0000);
    #[doc = "Format can be used with linear filtering whilst color conversion is enabled"]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(0b100_0000_0000_0000_0000);
    #[doc = "Format can have different chroma, min and mag filters"]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self =
        Self(0b1000_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self =
        Self(0b1_0000_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
        Self(0b10_0000_0000_0000_0000_0000);
    #[doc = "Format supports disjoint planes"]
    pub const DISJOINT: Self = Self(0b100_0000_0000_0000_0000_0000);
    #[doc = "Format can have cosited rather than midpoint chroma samples"]
    pub const COSITED_CHROMA_SAMPLES: Self = Self(0b1000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl ImageAspectFlags {
    pub const PLANE_0: Self = Self(0b1_0000);
    pub const PLANE_1: Self = Self(0b10_0000);
    pub const PLANE_2: Self = Self(0b100_0000);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl ImageCreateFlags {
    pub const ALIAS: Self = Self(0b100_0000_0000);
    #[doc = "Allows using VkBindImageMemoryDeviceGroupInfo::pSplitInstanceBindRegions when binding memory to the image"]
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(0b100_0000);
    #[doc = "The 3D image can be viewed as a 2D or 2D array image"]
    pub const TYPE_2D_ARRAY_COMPATIBLE: Self = Self(0b10_0000);
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE: Self = Self(0b1000_0000);
    pub const EXTENDED_USAGE: Self = Self(0b1_0000_0000);
    #[doc = "Image requires protected memory"]
    pub const PROTECTED: Self = Self(0b1000_0000_0000);
    pub const DISJOINT: Self = Self(0b10_0000_0000);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl ImageLayout {
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1_000_117_000);
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: Self = Self(1_000_117_001);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl MemoryHeapFlags {
    #[doc = "If set, heap allocations allocate multiple instances by default"]
    pub const MULTI_INSTANCE: Self = Self(0b10);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl MemoryPropertyFlags {
    #[doc = "Memory is protected"]
    pub const PROTECTED: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl ObjectType {
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1_000_156_000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1_000_085_000);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl PipelineCreateFlags {
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(0b1000);
    pub const DISPATCH_BASE: Self = Self(0b1_0000);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl QueueFlags {
    #[doc = "Queues may support protected operations"]
    pub const PROTECTED: Self = Self(0b1_0000);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl Result {
    pub const ERROR_OUT_OF_POOL_MEMORY: Self = Self(-1_000_069_000);
    pub const ERROR_INVALID_EXTERNAL_HANDLE: Self = Self(-1_000_072_003);
}
#[doc = "Generated from 'VK_VERSION_1_1'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: Self = Self(1_000_094_000);
    pub const BIND_BUFFER_MEMORY_INFO: Self = Self(1_000_157_000);
    pub const BIND_IMAGE_MEMORY_INFO: Self = Self(1_000_157_001);
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: Self = Self(1_000_083_000);
    pub const MEMORY_DEDICATED_REQUIREMENTS: Self = Self(1_000_127_000);
    pub const MEMORY_DEDICATED_ALLOCATE_INFO: Self = Self(1_000_127_001);
    pub const MEMORY_ALLOCATE_FLAGS_INFO: Self = Self(1_000_060_000);
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: Self = Self(1_000_060_003);
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: Self = Self(1_000_060_004);
    pub const DEVICE_GROUP_SUBMIT_INFO: Self = Self(1_000_060_005);
    pub const DEVICE_GROUP_BIND_SPARSE_INFO: Self = Self(1_000_060_006);
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: Self = Self(1_000_060_013);
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: Self = Self(1_000_060_014);
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES: Self = Self(1_000_070_000);
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO: Self = Self(1_000_070_001);
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1_000_146_000);
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1_000_146_001);
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1_000_146_002);
    pub const MEMORY_REQUIREMENTS_2: Self = Self(1_000_146_003);
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: Self = Self(1_000_146_004);
    pub const PHYSICAL_DEVICE_FEATURES_2: Self = Self(1_000_059_000);
    pub const PHYSICAL_DEVICE_PROPERTIES_2: Self = Self(1_000_059_001);
    pub const FORMAT_PROPERTIES_2: Self = Self(1_000_059_002);
    pub const IMAGE_FORMAT_PROPERTIES_2: Self = Self(1_000_059_003);
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: Self = Self(1_000_059_004);
    pub const QUEUE_FAMILY_PROPERTIES_2: Self = Self(1_000_059_005);
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: Self = Self(1_000_059_006);
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2: Self = Self(1_000_059_007);
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: Self = Self(1_000_059_008);
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: Self = Self(1_000_117_000);
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: Self = Self(1_000_117_001);
    pub const IMAGE_VIEW_USAGE_CREATE_INFO: Self = Self(1_000_117_002);
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: Self = Self(1_000_117_003);
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO: Self = Self(1_000_053_000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES: Self = Self(1_000_053_001);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: Self = Self(1_000_053_002);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: Self = Self(1_000_120_000);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: Self =
        Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
    pub const PROTECTED_SUBMIT_INFO: Self = Self(1_000_145_000);
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: Self = Self(1_000_145_001);
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: Self = Self(1_000_145_002);
    pub const DEVICE_QUEUE_INFO_2: Self = Self(1_000_145_003);
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO: Self = Self(1_000_156_000);
    pub const SAMPLER_YCBCR_CONVERSION_INFO: Self = Self(1_000_156_001);
    pub const BIND_IMAGE_PLANE_MEMORY_INFO: Self = Self(1_000_156_002);
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: Self = Self(1_000_156_003);
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: Self = Self(1_000_156_004);
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: Self = Self(1_000_156_005);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: Self = Self(1_000_085_000);
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: Self = Self(1_000_071_000);
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES: Self = Self(1_000_071_001);
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: Self = Self(1_000_071_002);
    pub const EXTERNAL_BUFFER_PROPERTIES: Self = Self(1_000_071_003);
    pub const PHYSICAL_DEVICE_ID_PROPERTIES: Self = Self(1_000_071_004);
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO: Self = Self(1_000_072_000);
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO: Self = Self(1_000_072_001);
    pub const EXPORT_MEMORY_ALLOCATE_INFO: Self = Self(1_000_072_002);
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: Self = Self(1_000_112_000);
    pub const EXTERNAL_FENCE_PROPERTIES: Self = Self(1_000_112_001);
    pub const EXPORT_FENCE_CREATE_INFO: Self = Self(1_000_113_000);
    pub const EXPORT_SEMAPHORE_CREATE_INFO: Self = Self(1_000_077_000);
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: Self = Self(1_000_076_000);
    pub const EXTERNAL_SEMAPHORE_PROPERTIES: Self = Self(1_000_076_001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: Self = Self(1_000_168_000);
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT: Self = Self(1_000_168_001);
    pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES: Self = Self(1_000_063_000);
    pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: Self =
        Self::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES;
}
#[doc = "Generated from 'VK_VERSION_1_2'"]
impl BufferCreateFlags {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(0b1_0000);
}
#[doc = "Generated from 'VK_VERSION_1_2'"]
impl BufferUsageFlags {
    pub const SHADER_DEVICE_ADDRESS: Self = Self(0b10_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_VERSION_1_2'"]
impl DescriptorPoolCreateFlags {
    pub const UPDATE_AFTER_BIND: Self = Self(0b10);
}
#[doc = "Generated from 'VK_VERSION_1_2'"]
impl DescriptorSetLayoutCreateFlags {
    pub const UPDATE_AFTER_BIND_POOL: Self = Self(0b10);
}
#[doc = "Generated from 'VK_VERSION_1_2'"]
impl FormatFeatureFlags {
    #[doc = "Format can be used with min/max reduction filtering"]
    pub const SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(0b1_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_VERSION_1_2'"]
impl FramebufferCreateFlags {
    pub const IMAGELESS: Self = Self(0b1);
}
#[doc = "Generated from 'VK_VERSION_1_2'"]
impl ImageLayout {
    pub const DEPTH_ATTACHMENT_OPTIMAL: Self = Self(1_000_241_000);
    pub const DEPTH_READ_ONLY_OPTIMAL: Self = Self(1_000_241_001);
    pub const STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1_000_241_002);
    pub const STENCIL_READ_ONLY_OPTIMAL: Self = Self(1_000_241_003);
}
#[doc = "Generated from 'VK_VERSION_1_2'"]
impl MemoryAllocateFlags {
    pub const DEVICE_ADDRESS: Self = Self(0b10);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(0b100);
}
#[doc = "Generated from 'VK_VERSION_1_2'"]
impl Result {
    pub const ERROR_FRAGMENTATION: Self = Self(-1_000_161_000);
    pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: Self = Self(-1_000_257_000);
}
#[doc = "Generated from 'VK_VERSION_1_2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: Self = Self(49);
    pub const PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: Self = Self(50);
    pub const PHYSICAL_DEVICE_VULKAN_1_2_FEATURES: Self = Self(51);
    pub const PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES: Self = Self(52);
    pub const IMAGE_FORMAT_LIST_CREATE_INFO: Self = Self(1_000_147_000);
    pub const ATTACHMENT_DESCRIPTION_2: Self = Self(1_000_109_000);
    pub const ATTACHMENT_REFERENCE_2: Self = Self(1_000_109_001);
    pub const SUBPASS_DESCRIPTION_2: Self = Self(1_000_109_002);
    pub const SUBPASS_DEPENDENCY_2: Self = Self(1_000_109_003);
    pub const RENDER_PASS_CREATE_INFO_2: Self = Self(1_000_109_004);
    pub const SUBPASS_BEGIN_INFO: Self = Self(1_000_109_005);
    pub const SUBPASS_END_INFO: Self = Self(1_000_109_006);
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: Self = Self(1_000_177_000);
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES: Self = Self(1_000_196_000);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: Self = Self(1_000_180_000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: Self = Self(1_000_082_000);
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: Self = Self(1_000_197_000);
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: Self = Self(1_000_161_000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: Self = Self(1_000_161_001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: Self = Self(1_000_161_002);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: Self = Self(1_000_161_003);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: Self = Self(1_000_161_004);
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: Self = Self(1_000_199_000);
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: Self = Self(1_000_199_001);
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: Self = Self(1_000_221_000);
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO: Self = Self(1_000_246_000);
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: Self = Self(1_000_130_000);
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO: Self = Self(1_000_130_001);
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: Self = Self(1_000_211_000);
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: Self = Self(1_000_108_000);
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: Self = Self(1_000_108_001);
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: Self = Self(1_000_108_002);
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO: Self = Self(1_000_108_003);
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: Self = Self(1_000_253_000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: Self = Self(1_000_175_000);
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: Self = Self(1_000_241_000);
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT: Self = Self(1_000_241_001);
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: Self = Self(1_000_241_002);
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: Self = Self(1_000_261_000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: Self = Self(1_000_207_000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: Self = Self(1_000_207_001);
    pub const SEMAPHORE_TYPE_CREATE_INFO: Self = Self(1_000_207_002);
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO: Self = Self(1_000_207_003);
    pub const SEMAPHORE_WAIT_INFO: Self = Self(1_000_207_004);
    pub const SEMAPHORE_SIGNAL_INFO: Self = Self(1_000_207_005);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: Self = Self(1_000_257_000);
    pub const BUFFER_DEVICE_ADDRESS_INFO: Self = Self(1_000_244_001);
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: Self = Self(1_000_257_002);
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: Self = Self(1_000_257_003);
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: Self = Self(1_000_257_004);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl AccessFlags {
    pub const NONE: Self = Self(0);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl AttachmentStoreOp {
    pub const NONE: Self = Self(1_000_301_000);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl DescriptorType {
    pub const INLINE_UNIFORM_BLOCK: Self = Self(1_000_138_000);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl DynamicState {
    pub const CULL_MODE: Self = Self(1_000_267_000);
    pub const FRONT_FACE: Self = Self(1_000_267_001);
    pub const PRIMITIVE_TOPOLOGY: Self = Self(1_000_267_002);
    pub const VIEWPORT_WITH_COUNT: Self = Self(1_000_267_003);
    pub const SCISSOR_WITH_COUNT: Self = Self(1_000_267_004);
    pub const VERTEX_INPUT_BINDING_STRIDE: Self = Self(1_000_267_005);
    pub const DEPTH_TEST_ENABLE: Self = Self(1_000_267_006);
    pub const DEPTH_WRITE_ENABLE: Self = Self(1_000_267_007);
    pub const DEPTH_COMPARE_OP: Self = Self(1_000_267_008);
    pub const DEPTH_BOUNDS_TEST_ENABLE: Self = Self(1_000_267_009);
    pub const STENCIL_TEST_ENABLE: Self = Self(1_000_267_010);
    pub const STENCIL_OP: Self = Self(1_000_267_011);
    pub const RASTERIZER_DISCARD_ENABLE: Self = Self(1_000_377_001);
    pub const DEPTH_BIAS_ENABLE: Self = Self(1_000_377_002);
    pub const PRIMITIVE_RESTART_ENABLE: Self = Self(1_000_377_004);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl EventCreateFlags {
    pub const DEVICE_ONLY: Self = Self(0b1);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl Format {
    pub const G8_B8R8_2PLANE_444_UNORM: Self = Self(1_000_330_000);
    pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: Self = Self(1_000_330_001);
    pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: Self = Self(1_000_330_002);
    pub const G16_B16R16_2PLANE_444_UNORM: Self = Self(1_000_330_003);
    pub const A4R4G4B4_UNORM_PACK16: Self = Self(1_000_340_000);
    pub const A4B4G4R4_UNORM_PACK16: Self = Self(1_000_340_001);
    pub const ASTC_4X4_SFLOAT_BLOCK: Self = Self(1_000_066_000);
    pub const ASTC_5X4_SFLOAT_BLOCK: Self = Self(1_000_066_001);
    pub const ASTC_5X5_SFLOAT_BLOCK: Self = Self(1_000_066_002);
    pub const ASTC_6X5_SFLOAT_BLOCK: Self = Self(1_000_066_003);
    pub const ASTC_6X6_SFLOAT_BLOCK: Self = Self(1_000_066_004);
    pub const ASTC_8X5_SFLOAT_BLOCK: Self = Self(1_000_066_005);
    pub const ASTC_8X6_SFLOAT_BLOCK: Self = Self(1_000_066_006);
    pub const ASTC_8X8_SFLOAT_BLOCK: Self = Self(1_000_066_007);
    pub const ASTC_10X5_SFLOAT_BLOCK: Self = Self(1_000_066_008);
    pub const ASTC_10X6_SFLOAT_BLOCK: Self = Self(1_000_066_009);
    pub const ASTC_10X8_SFLOAT_BLOCK: Self = Self(1_000_066_010);
    pub const ASTC_10X10_SFLOAT_BLOCK: Self = Self(1_000_066_011);
    pub const ASTC_12X10_SFLOAT_BLOCK: Self = Self(1_000_066_012);
    pub const ASTC_12X12_SFLOAT_BLOCK: Self = Self(1_000_066_013);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl ImageAspectFlags {
    pub const NONE: Self = Self(0);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl ImageLayout {
    pub const READ_ONLY_OPTIMAL: Self = Self(1_000_314_000);
    pub const ATTACHMENT_OPTIMAL: Self = Self(1_000_314_001);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl ObjectType {
    pub const PRIVATE_DATA_SLOT: Self = Self(1_000_295_000);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl PipelineCacheCreateFlags {
    pub const EXTERNALLY_SYNCHRONIZED: Self = Self(0b1);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl PipelineCreateFlags {
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(0b1_0000_0000);
    pub const EARLY_RETURN_ON_FAILURE: Self = Self(0b10_0000_0000);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl PipelineShaderStageCreateFlags {
    pub const ALLOW_VARYING_SUBGROUP_SIZE: Self = Self(0b1);
    pub const REQUIRE_FULL_SUBGROUPS: Self = Self(0b10);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl PipelineStageFlags {
    pub const NONE: Self = Self(0);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl Result {
    pub const PIPELINE_COMPILE_REQUIRED: Self = Self(1_000_297_000);
}
#[doc = "Generated from 'VK_VERSION_1_3'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VULKAN_1_3_FEATURES: Self = Self(53);
    pub const PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES: Self = Self(54);
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO: Self = Self(1_000_192_000);
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES: Self = Self(1_000_215_000);
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES: Self = Self(1_000_245_000);
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: Self =
        Self(1_000_276_000);
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES: Self = Self(1_000_295_000);
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO: Self = Self(1_000_295_001);
    pub const PRIVATE_DATA_SLOT_CREATE_INFO: Self = Self(1_000_295_002);
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES: Self = Self(1_000_297_000);
    pub const MEMORY_BARRIER_2: Self = Self(1_000_314_000);
    pub const BUFFER_MEMORY_BARRIER_2: Self = Self(1_000_314_001);
    pub const IMAGE_MEMORY_BARRIER_2: Self = Self(1_000_314_002);
    pub const DEPENDENCY_INFO: Self = Self(1_000_314_003);
    pub const SUBMIT_INFO_2: Self = Self(1_000_314_004);
    pub const SEMAPHORE_SUBMIT_INFO: Self = Self(1_000_314_005);
    pub const COMMAND_BUFFER_SUBMIT_INFO: Self = Self(1_000_314_006);
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: Self = Self(1_000_314_007);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES: Self = Self(1_000_325_000);
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES: Self = Self(1_000_335_000);
    pub const COPY_BUFFER_INFO_2: Self = Self(1_000_337_000);
    pub const COPY_IMAGE_INFO_2: Self = Self(1_000_337_001);
    pub const COPY_BUFFER_TO_IMAGE_INFO_2: Self = Self(1_000_337_002);
    pub const COPY_IMAGE_TO_BUFFER_INFO_2: Self = Self(1_000_337_003);
    pub const BLIT_IMAGE_INFO_2: Self = Self(1_000_337_004);
    pub const RESOLVE_IMAGE_INFO_2: Self = Self(1_000_337_005);
    pub const BUFFER_COPY_2: Self = Self(1_000_337_006);
    pub const IMAGE_COPY_2: Self = Self(1_000_337_007);
    pub const IMAGE_BLIT_2: Self = Self(1_000_337_008);
    pub const BUFFER_IMAGE_COPY_2: Self = Self(1_000_337_009);
    pub const IMAGE_RESOLVE_2: Self = Self(1_000_337_010);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES: Self = Self(1_000_225_000);
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO: Self = Self(1_000_225_001);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES: Self = Self(1_000_225_002);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES: Self = Self(1_000_138_000);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES: Self = Self(1_000_138_001);
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: Self = Self(1_000_138_002);
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO: Self = Self(1_000_138_003);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES: Self = Self(1_000_066_000);
    pub const RENDERING_INFO: Self = Self(1_000_044_000);
    pub const RENDERING_ATTACHMENT_INFO: Self = Self(1_000_044_001);
    pub const PIPELINE_RENDERING_CREATE_INFO: Self = Self(1_000_044_002);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES: Self = Self(1_000_044_003);
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO: Self = Self(1_000_044_004);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES: Self = Self(1_000_280_000);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES: Self = Self(1_000_280_001);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES: Self = Self(1_000_281_001);
    pub const FORMAT_PROPERTIES_3: Self = Self(1_000_360_000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES: Self = Self(1_000_413_000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES: Self = Self(1_000_413_001);
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS: Self = Self(1_000_413_002);
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS: Self = Self(1_000_413_003);
}
