use crate::vk::bitflags::*;
use crate::vk::definitions::*;
use crate::vk::enums::*;
use core::ffi::*;
#[allow(non_camel_case_types)]
pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    p_name: *const c_char,
) -> PFN_vkVoidFunction;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateInstance = unsafe extern "system" fn(
    p_create_info: *const InstanceCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_instance: *mut crate::vk::Instance,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(
    p_property_count: *mut u32,
    p_properties: *mut LayerProperties,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyInstance = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    p_physical_device_count: *mut u32,
    p_physical_devices: *mut PhysicalDevice,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_features: *mut PhysicalDeviceFeatures,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    ty: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    p_image_format_properties: *mut ImageFormatProperties,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_properties: *mut PhysicalDeviceProperties,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceProcAddr = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_name: *const c_char,
) -> PFN_vkVoidFunction;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDevice = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_create_info: *const DeviceCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_device: *mut crate::vk::Device,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut LayerProperties,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    ty: ImageType,
    samples: SampleCountFlags,
    usage: ImageUsageFlags,
    tiling: ImageTiling,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties,
);
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDevice = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(
    device: crate::vk::Device,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut Queue,
);
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSubmit = unsafe extern "system" fn(
    queue: Queue,
    submit_count: u32,
    p_submits: *const SubmitInfo<'_>,
    fence: Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: Queue) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(device: crate::vk::Device) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_allocate_info: *const MemoryAllocateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_memory: *mut DeviceMemory,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkFreeMemory = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory: DeviceMemory,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkMapMemory = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory: DeviceMemory,
    offset: DeviceSize,
    size: DeviceSize,
    flags: MemoryMapFlags,
    pp_data: *mut *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkUnmapMemory =
    unsafe extern "system" fn(device: crate::vk::Device, memory: DeviceMemory);
#[allow(non_camel_case_types)]
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory: DeviceMemory,
    p_committed_memory_in_bytes: *mut DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(
    device: crate::vk::Device,
    buffer: Buffer,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(
    device: crate::vk::Device,
    buffer: Buffer,
    p_memory_requirements: *mut MemoryRequirements,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    p_memory_requirements: *mut MemoryRequirements,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
);
#[allow(non_camel_case_types)]
pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(
    queue: Queue,
    bind_info_count: u32,
    p_bind_info: *const BindSparseInfo<'_>,
    fence: Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateFence = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const FenceCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_fence: *mut Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyFence = unsafe extern "system" fn(
    device: crate::vk::Device,
    fence: Fence,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkResetFences = unsafe extern "system" fn(
    device: crate::vk::Device,
    fence_count: u32,
    p_fences: *const Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceStatus =
    unsafe extern "system" fn(device: crate::vk::Device, fence: Fence) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkWaitForFences = unsafe extern "system" fn(
    device: crate::vk::Device,
    fence_count: u32,
    p_fences: *const Fence,
    wait_all: Bool32,
    timeout: u64,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const SemaphoreCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_semaphore: *mut Semaphore,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(
    device: crate::vk::Device,
    semaphore: Semaphore,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateEvent = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const EventCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_event: *mut Event,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyEvent = unsafe extern "system" fn(
    device: crate::vk::Device,
    event: Event,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetEventStatus =
    unsafe extern "system" fn(device: crate::vk::Device, event: Event) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkSetEvent =
    unsafe extern "system" fn(device: crate::vk::Device, event: Event) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkResetEvent =
    unsafe extern "system" fn(device: crate::vk::Device, event: Event) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const QueryPoolCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_query_pool: *mut QueryPool,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    query_pool: QueryPool,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueryPoolResults = unsafe extern "system" fn(
    device: crate::vk::Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    data_size: usize,
    p_data: *mut c_void,
    stride: DeviceSize,
    flags: QueryResultFlags,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateBuffer = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const BufferCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_buffer: *mut Buffer,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(
    device: crate::vk::Device,
    buffer: Buffer,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateBufferView = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const BufferViewCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_view: *mut BufferView,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(
    device: crate::vk::Device,
    buffer_view: BufferView,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImage = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const ImageCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_image: *mut Image,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyImage = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    p_subresource: *const ImageSubresource,
    p_layout: *mut SubresourceLayout,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImageView = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const ImageViewCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_view: *mut ImageView,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyImageView = unsafe extern "system" fn(
    device: crate::vk::Device,
    image_view: ImageView,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const ShaderModuleCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_shader_module: *mut ShaderModule,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(
    device: crate::vk::Device,
    shader_module: ShaderModule,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const PipelineCacheCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipeline_cache: *mut PipelineCache,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_cache: PipelineCache,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_cache: PipelineCache,
    p_data_size: *mut usize,
    p_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(
    device: crate::vk::Device,
    dst_cache: PipelineCache,
    src_cache_count: u32,
    p_src_caches: *const PipelineCache,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const GraphicsPipelineCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipelines: *mut Pipeline,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const ComputePipelineCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipelines: *mut Pipeline,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline: Pipeline,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const PipelineLayoutCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipeline_layout: *mut PipelineLayout,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_layout: PipelineLayout,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSampler = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const SamplerCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_sampler: *mut Sampler,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySampler = unsafe extern "system" fn(
    device: crate::vk::Device,
    sampler: Sampler,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_set_layout: *mut DescriptorSetLayout,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_set_layout: DescriptorSetLayout,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const DescriptorPoolCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_descriptor_pool: *mut DescriptorPool,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_pool: DescriptorPool,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_pool: DescriptorPool,
    flags: DescriptorPoolResetFlags,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_allocate_info: *const DescriptorSetAllocateInfo<'_>,
    p_descriptor_sets: *mut DescriptorSet,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_pool: DescriptorPool,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet<'_>,
    descriptor_copy_count: u32,
    p_descriptor_copies: *const CopyDescriptorSet<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const FramebufferCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_framebuffer: *mut Framebuffer,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(
    device: crate::vk::Device,
    framebuffer: Framebuffer,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const RenderPassCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_render_pass: *mut RenderPass,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(
    device: crate::vk::Device,
    render_pass: RenderPass,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(
    device: crate::vk::Device,
    render_pass: RenderPass,
    p_granularity: *mut Extent2D,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_create_info: *const CommandPoolCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_command_pool: *mut CommandPool,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    command_pool: CommandPool,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkResetCommandPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    command_pool: CommandPool,
    flags: CommandPoolResetFlags,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_allocate_info: *const CommandBufferAllocateInfo<'_>,
    p_command_buffers: *mut CommandBuffer,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkFreeCommandBuffers = unsafe extern "system" fn(
    device: crate::vk::Device,
    command_pool: CommandPool,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
);
#[allow(non_camel_case_types)]
pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_begin_info: *const CommandBufferBeginInfo<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkEndCommandBuffer =
    unsafe extern "system" fn(command_buffer: CommandBuffer) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    flags: CommandBufferResetFlags,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewports: *const Viewport,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    p_scissors: *const Rect2D,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineWidth =
    unsafe extern "system" fn(command_buffer: CommandBuffer, line_width: f32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetBlendConstants =
    unsafe extern "system" fn(command_buffer: CommandBuffer, blend_constants: *const [f32; 4usize]);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    compare_mask: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    write_mask: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    reference: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
    dynamic_offset_count: u32,
    p_dynamic_offsets: *const u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    index_type: IndexType,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDraw = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatch = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchIndirect =
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_buffer: Buffer,
    region_count: u32,
    p_regions: *const BufferCopy,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageCopy,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageBlit,
    filter: Filter,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const BufferImageCopy,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_buffer: Buffer,
    region_count: u32,
    p_regions: *const BufferImageCopy,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    data_size: DeviceSize,
    p_data: *const c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdFillBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    size: DeviceSize,
    data: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_color: *const ClearColorValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_depth_stencil: *const ClearDepthStencilValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_attachments: *const ClearAttachment,
    rect_count: u32,
    p_rects: *const ClearRect,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageResolve,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event_count: u32,
    p_events: *const Event,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier<'_>,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier<'_>,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    dependency_flags: DependencyFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier<'_>,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier<'_>,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginQuery = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndQuery =
    unsafe extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetQueryPool = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteTimestamp = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlags,
    query_pool: QueryPool,
    query: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    stride: DeviceSize,
    flags: QueryResultFlags,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    layout: PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    p_values: *const c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo<'_>,
    contents: SubpassContents,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdNextSubpass =
    unsafe extern "system" fn(command_buffer: CommandBuffer, contents: SubpassContents);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
);
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceVersion =
    unsafe extern "system" fn(p_api_version: *mut u32) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(
    device: crate::vk::Device,
    p_queue_info: *const DeviceQueueInfo2<'_>,
    p_queue: *mut Queue,
);
