#![allow(unused_qualifications)]
use crate::vk::*;
use core::ffi::*;
#[derive(Clone)]
#[doc = "Raw Vulkan 1 static function pointers"]
pub struct StaticFn {
    pub get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
}
unsafe impl Send for StaticFn {}
unsafe impl Sync for StaticFn {}
impl StaticFn {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            get_instance_proc_addr: unsafe {
                unsafe extern "system" fn get_instance_proc_addr(
                    _instance: crate::vk::Instance,
                    _p_name: *const c_char,
                ) -> PFN_vkVoidFunction {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_instance_proc_addr)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetInstanceProcAddr\0");
                let val = _f(cname);
                if val.is_null() {
                    get_instance_proc_addr
                } else {
                    ::core::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
#[doc = "Raw Vulkan 1 entry point function pointers"]
pub struct EntryFnV1_0 {
    pub create_instance: PFN_vkCreateInstance,
    pub enumerate_instance_extension_properties: PFN_vkEnumerateInstanceExtensionProperties,
    pub enumerate_instance_layer_properties: PFN_vkEnumerateInstanceLayerProperties,
}
unsafe impl Send for EntryFnV1_0 {}
unsafe impl Sync for EntryFnV1_0 {}
impl EntryFnV1_0 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            create_instance: unsafe {
                unsafe extern "system" fn create_instance(
                    _p_create_info: *const InstanceCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_instance: *mut crate::vk::Instance,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_instance)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateInstance\0");
                let val = _f(cname);
                if val.is_null() {
                    create_instance
                } else {
                    ::core::mem::transmute(val)
                }
            },
            enumerate_instance_extension_properties: unsafe {
                unsafe extern "system" fn enumerate_instance_extension_properties(
                    _p_layer_name: *const c_char,
                    _p_property_count: *mut u32,
                    _p_properties: *mut ExtensionProperties,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(enumerate_instance_extension_properties)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumerateInstanceExtensionProperties\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    enumerate_instance_extension_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
            enumerate_instance_layer_properties: unsafe {
                unsafe extern "system" fn enumerate_instance_layer_properties(
                    _p_property_count: *mut u32,
                    _p_properties: *mut LayerProperties,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(enumerate_instance_layer_properties)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkEnumerateInstanceLayerProperties\0");
                let val = _f(cname);
                if val.is_null() {
                    enumerate_instance_layer_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
#[doc = "Raw Vulkan 1 instance-level function pointers"]
pub struct InstanceFnV1_0 {
    pub destroy_instance: PFN_vkDestroyInstance,
    pub enumerate_physical_devices: PFN_vkEnumeratePhysicalDevices,
    pub get_physical_device_features: PFN_vkGetPhysicalDeviceFeatures,
    pub get_physical_device_format_properties: PFN_vkGetPhysicalDeviceFormatProperties,
    pub get_physical_device_image_format_properties: PFN_vkGetPhysicalDeviceImageFormatProperties,
    pub get_physical_device_properties: PFN_vkGetPhysicalDeviceProperties,
    pub get_physical_device_queue_family_properties: PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    pub get_physical_device_memory_properties: PFN_vkGetPhysicalDeviceMemoryProperties,
    pub get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    pub create_device: PFN_vkCreateDevice,
    pub enumerate_device_extension_properties: PFN_vkEnumerateDeviceExtensionProperties,
    pub enumerate_device_layer_properties: PFN_vkEnumerateDeviceLayerProperties,
    pub get_physical_device_sparse_image_format_properties:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties,
}
unsafe impl Send for InstanceFnV1_0 {}
unsafe impl Sync for InstanceFnV1_0 {}
impl InstanceFnV1_0 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            destroy_instance: unsafe {
                unsafe extern "system" fn destroy_instance(
                    _instance: crate::vk::Instance,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_instance)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyInstance\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_instance
                } else {
                    ::core::mem::transmute(val)
                }
            },
            enumerate_physical_devices: unsafe {
                unsafe extern "system" fn enumerate_physical_devices(
                    _instance: crate::vk::Instance,
                    _p_physical_device_count: *mut u32,
                    _p_physical_devices: *mut PhysicalDevice,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(enumerate_physical_devices)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkEnumeratePhysicalDevices\0");
                let val = _f(cname);
                if val.is_null() {
                    enumerate_physical_devices
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_features: unsafe {
                unsafe extern "system" fn get_physical_device_features(
                    _physical_device: PhysicalDevice,
                    _p_features: *mut PhysicalDeviceFeatures,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_features)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceFeatures\0");
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_features
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_format_properties: unsafe {
                unsafe extern "system" fn get_physical_device_format_properties(
                    _physical_device: PhysicalDevice,
                    _format: Format,
                    _p_format_properties: *mut FormatProperties,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_format_properties)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceFormatProperties\0");
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_format_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_image_format_properties: unsafe {
                unsafe extern "system" fn get_physical_device_image_format_properties(
                    _physical_device: PhysicalDevice,
                    _format: Format,
                    _ty: ImageType,
                    _tiling: ImageTiling,
                    _usage: ImageUsageFlags,
                    _flags: ImageCreateFlags,
                    _p_image_format_properties: *mut ImageFormatProperties,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_image_format_properties)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceImageFormatProperties\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_image_format_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_properties: unsafe {
                unsafe extern "system" fn get_physical_device_properties(
                    _physical_device: PhysicalDevice,
                    _p_properties: *mut PhysicalDeviceProperties,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_properties)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceProperties\0");
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_queue_family_properties: unsafe {
                unsafe extern "system" fn get_physical_device_queue_family_properties(
                    _physical_device: PhysicalDevice,
                    _p_queue_family_property_count: *mut u32,
                    _p_queue_family_properties: *mut QueueFamilyProperties,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_queue_family_properties)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceQueueFamilyProperties\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_queue_family_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_memory_properties: unsafe {
                unsafe extern "system" fn get_physical_device_memory_properties(
                    _physical_device: PhysicalDevice,
                    _p_memory_properties: *mut PhysicalDeviceMemoryProperties,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_memory_properties)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceMemoryProperties\0");
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_memory_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_device_proc_addr: unsafe {
                unsafe extern "system" fn get_device_proc_addr(
                    _device: crate::vk::Device,
                    _p_name: *const c_char,
                ) -> PFN_vkVoidFunction {
                    panic!(concat!("Unable to load ", stringify!(get_device_proc_addr)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceProcAddr\0");
                let val = _f(cname);
                if val.is_null() {
                    get_device_proc_addr
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_device: unsafe {
                unsafe extern "system" fn create_device(
                    _physical_device: PhysicalDevice,
                    _p_create_info: *const DeviceCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_device: *mut crate::vk::Device,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_device)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateDevice\0");
                let val = _f(cname);
                if val.is_null() {
                    create_device
                } else {
                    ::core::mem::transmute(val)
                }
            },
            enumerate_device_extension_properties: unsafe {
                unsafe extern "system" fn enumerate_device_extension_properties(
                    _physical_device: PhysicalDevice,
                    _p_layer_name: *const c_char,
                    _p_property_count: *mut u32,
                    _p_properties: *mut ExtensionProperties,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(enumerate_device_extension_properties)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkEnumerateDeviceExtensionProperties\0");
                let val = _f(cname);
                if val.is_null() {
                    enumerate_device_extension_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
            enumerate_device_layer_properties: unsafe {
                unsafe extern "system" fn enumerate_device_layer_properties(
                    _physical_device: PhysicalDevice,
                    _p_property_count: *mut u32,
                    _p_properties: *mut LayerProperties,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(enumerate_device_layer_properties)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkEnumerateDeviceLayerProperties\0");
                let val = _f(cname);
                if val.is_null() {
                    enumerate_device_layer_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_sparse_image_format_properties: unsafe {
                unsafe extern "system" fn get_physical_device_sparse_image_format_properties(
                    _physical_device: PhysicalDevice,
                    _format: Format,
                    _ty: ImageType,
                    _samples: SampleCountFlags,
                    _usage: ImageUsageFlags,
                    _tiling: ImageTiling,
                    _p_property_count: *mut u32,
                    _p_properties: *mut SparseImageFormatProperties,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_sparse_image_format_properties)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSparseImageFormatProperties\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_sparse_image_format_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
#[doc = "Raw Vulkan 1 device-level function pointers"]
pub struct DeviceFnV1_0 {
    pub destroy_device: PFN_vkDestroyDevice,
    pub get_device_queue: PFN_vkGetDeviceQueue,
    pub queue_submit: PFN_vkQueueSubmit,
    pub queue_wait_idle: PFN_vkQueueWaitIdle,
    pub device_wait_idle: PFN_vkDeviceWaitIdle,
    pub allocate_memory: PFN_vkAllocateMemory,
    pub free_memory: PFN_vkFreeMemory,
    pub map_memory: PFN_vkMapMemory,
    pub unmap_memory: PFN_vkUnmapMemory,
    pub flush_mapped_memory_ranges: PFN_vkFlushMappedMemoryRanges,
    pub invalidate_mapped_memory_ranges: PFN_vkInvalidateMappedMemoryRanges,
    pub get_device_memory_commitment: PFN_vkGetDeviceMemoryCommitment,
    pub bind_buffer_memory: PFN_vkBindBufferMemory,
    pub bind_image_memory: PFN_vkBindImageMemory,
    pub get_buffer_memory_requirements: PFN_vkGetBufferMemoryRequirements,
    pub get_image_memory_requirements: PFN_vkGetImageMemoryRequirements,
    pub get_image_sparse_memory_requirements: PFN_vkGetImageSparseMemoryRequirements,
    pub queue_bind_sparse: PFN_vkQueueBindSparse,
    pub create_fence: PFN_vkCreateFence,
    pub destroy_fence: PFN_vkDestroyFence,
    pub reset_fences: PFN_vkResetFences,
    pub get_fence_status: PFN_vkGetFenceStatus,
    pub wait_for_fences: PFN_vkWaitForFences,
    pub create_semaphore: PFN_vkCreateSemaphore,
    pub destroy_semaphore: PFN_vkDestroySemaphore,
    pub create_event: PFN_vkCreateEvent,
    pub destroy_event: PFN_vkDestroyEvent,
    pub get_event_status: PFN_vkGetEventStatus,
    pub set_event: PFN_vkSetEvent,
    pub reset_event: PFN_vkResetEvent,
    pub create_query_pool: PFN_vkCreateQueryPool,
    pub destroy_query_pool: PFN_vkDestroyQueryPool,
    pub get_query_pool_results: PFN_vkGetQueryPoolResults,
    pub create_buffer: PFN_vkCreateBuffer,
    pub destroy_buffer: PFN_vkDestroyBuffer,
    pub create_buffer_view: PFN_vkCreateBufferView,
    pub destroy_buffer_view: PFN_vkDestroyBufferView,
    pub create_image: PFN_vkCreateImage,
    pub destroy_image: PFN_vkDestroyImage,
    pub get_image_subresource_layout: PFN_vkGetImageSubresourceLayout,
    pub create_image_view: PFN_vkCreateImageView,
    pub destroy_image_view: PFN_vkDestroyImageView,
    pub create_shader_module: PFN_vkCreateShaderModule,
    pub destroy_shader_module: PFN_vkDestroyShaderModule,
    pub create_pipeline_cache: PFN_vkCreatePipelineCache,
    pub destroy_pipeline_cache: PFN_vkDestroyPipelineCache,
    pub get_pipeline_cache_data: PFN_vkGetPipelineCacheData,
    pub merge_pipeline_caches: PFN_vkMergePipelineCaches,
    pub create_graphics_pipelines: PFN_vkCreateGraphicsPipelines,
    pub create_compute_pipelines: PFN_vkCreateComputePipelines,
    pub destroy_pipeline: PFN_vkDestroyPipeline,
    pub create_pipeline_layout: PFN_vkCreatePipelineLayout,
    pub destroy_pipeline_layout: PFN_vkDestroyPipelineLayout,
    pub create_sampler: PFN_vkCreateSampler,
    pub destroy_sampler: PFN_vkDestroySampler,
    pub create_descriptor_set_layout: PFN_vkCreateDescriptorSetLayout,
    pub destroy_descriptor_set_layout: PFN_vkDestroyDescriptorSetLayout,
    pub create_descriptor_pool: PFN_vkCreateDescriptorPool,
    pub destroy_descriptor_pool: PFN_vkDestroyDescriptorPool,
    pub reset_descriptor_pool: PFN_vkResetDescriptorPool,
    pub allocate_descriptor_sets: PFN_vkAllocateDescriptorSets,
    pub free_descriptor_sets: PFN_vkFreeDescriptorSets,
    pub update_descriptor_sets: PFN_vkUpdateDescriptorSets,
    pub create_framebuffer: PFN_vkCreateFramebuffer,
    pub destroy_framebuffer: PFN_vkDestroyFramebuffer,
    pub create_render_pass: PFN_vkCreateRenderPass,
    pub destroy_render_pass: PFN_vkDestroyRenderPass,
    pub get_render_area_granularity: PFN_vkGetRenderAreaGranularity,
    pub create_command_pool: PFN_vkCreateCommandPool,
    pub destroy_command_pool: PFN_vkDestroyCommandPool,
    pub reset_command_pool: PFN_vkResetCommandPool,
    pub allocate_command_buffers: PFN_vkAllocateCommandBuffers,
    pub free_command_buffers: PFN_vkFreeCommandBuffers,
    pub begin_command_buffer: PFN_vkBeginCommandBuffer,
    pub end_command_buffer: PFN_vkEndCommandBuffer,
    pub reset_command_buffer: PFN_vkResetCommandBuffer,
    pub cmd_bind_pipeline: PFN_vkCmdBindPipeline,
    pub cmd_set_viewport: PFN_vkCmdSetViewport,
    pub cmd_set_scissor: PFN_vkCmdSetScissor,
    pub cmd_set_line_width: PFN_vkCmdSetLineWidth,
    pub cmd_set_depth_bias: PFN_vkCmdSetDepthBias,
    pub cmd_set_blend_constants: PFN_vkCmdSetBlendConstants,
    pub cmd_set_depth_bounds: PFN_vkCmdSetDepthBounds,
    pub cmd_set_stencil_compare_mask: PFN_vkCmdSetStencilCompareMask,
    pub cmd_set_stencil_write_mask: PFN_vkCmdSetStencilWriteMask,
    pub cmd_set_stencil_reference: PFN_vkCmdSetStencilReference,
    pub cmd_bind_descriptor_sets: PFN_vkCmdBindDescriptorSets,
    pub cmd_bind_index_buffer: PFN_vkCmdBindIndexBuffer,
    pub cmd_bind_vertex_buffers: PFN_vkCmdBindVertexBuffers,
    pub cmd_draw: PFN_vkCmdDraw,
    pub cmd_draw_indexed: PFN_vkCmdDrawIndexed,
    pub cmd_draw_indirect: PFN_vkCmdDrawIndirect,
    pub cmd_draw_indexed_indirect: PFN_vkCmdDrawIndexedIndirect,
    pub cmd_dispatch: PFN_vkCmdDispatch,
    pub cmd_dispatch_indirect: PFN_vkCmdDispatchIndirect,
    pub cmd_copy_buffer: PFN_vkCmdCopyBuffer,
    pub cmd_copy_image: PFN_vkCmdCopyImage,
    pub cmd_blit_image: PFN_vkCmdBlitImage,
    pub cmd_copy_buffer_to_image: PFN_vkCmdCopyBufferToImage,
    pub cmd_copy_image_to_buffer: PFN_vkCmdCopyImageToBuffer,
    pub cmd_update_buffer: PFN_vkCmdUpdateBuffer,
    pub cmd_fill_buffer: PFN_vkCmdFillBuffer,
    pub cmd_clear_color_image: PFN_vkCmdClearColorImage,
    pub cmd_clear_depth_stencil_image: PFN_vkCmdClearDepthStencilImage,
    pub cmd_clear_attachments: PFN_vkCmdClearAttachments,
    pub cmd_resolve_image: PFN_vkCmdResolveImage,
    pub cmd_set_event: PFN_vkCmdSetEvent,
    pub cmd_reset_event: PFN_vkCmdResetEvent,
    pub cmd_wait_events: PFN_vkCmdWaitEvents,
    pub cmd_pipeline_barrier: PFN_vkCmdPipelineBarrier,
    pub cmd_begin_query: PFN_vkCmdBeginQuery,
    pub cmd_end_query: PFN_vkCmdEndQuery,
    pub cmd_reset_query_pool: PFN_vkCmdResetQueryPool,
    pub cmd_write_timestamp: PFN_vkCmdWriteTimestamp,
    pub cmd_copy_query_pool_results: PFN_vkCmdCopyQueryPoolResults,
    pub cmd_push_constants: PFN_vkCmdPushConstants,
    pub cmd_begin_render_pass: PFN_vkCmdBeginRenderPass,
    pub cmd_next_subpass: PFN_vkCmdNextSubpass,
    pub cmd_end_render_pass: PFN_vkCmdEndRenderPass,
    pub cmd_execute_commands: PFN_vkCmdExecuteCommands,
}
unsafe impl Send for DeviceFnV1_0 {}
unsafe impl Sync for DeviceFnV1_0 {}
impl DeviceFnV1_0 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            destroy_device: unsafe {
                unsafe extern "system" fn destroy_device(
                    _device: crate::vk::Device,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_device)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyDevice\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_device
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_device_queue: unsafe {
                unsafe extern "system" fn get_device_queue(
                    _device: crate::vk::Device,
                    _queue_family_index: u32,
                    _queue_index: u32,
                    _p_queue: *mut Queue,
                ) {
                    panic!(concat!("Unable to load ", stringify!(get_device_queue)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceQueue\0");
                let val = _f(cname);
                if val.is_null() {
                    get_device_queue
                } else {
                    ::core::mem::transmute(val)
                }
            },
            queue_submit: unsafe {
                unsafe extern "system" fn queue_submit(
                    _queue: Queue,
                    _submit_count: u32,
                    _p_submits: *const SubmitInfo<'_>,
                    _fence: Fence,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(queue_submit)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkQueueSubmit\0");
                let val = _f(cname);
                if val.is_null() {
                    queue_submit
                } else {
                    ::core::mem::transmute(val)
                }
            },
            queue_wait_idle: unsafe {
                unsafe extern "system" fn queue_wait_idle(_queue: Queue) -> Result {
                    panic!(concat!("Unable to load ", stringify!(queue_wait_idle)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkQueueWaitIdle\0");
                let val = _f(cname);
                if val.is_null() {
                    queue_wait_idle
                } else {
                    ::core::mem::transmute(val)
                }
            },
            device_wait_idle: unsafe {
                unsafe extern "system" fn device_wait_idle(_device: crate::vk::Device) -> Result {
                    panic!(concat!("Unable to load ", stringify!(device_wait_idle)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDeviceWaitIdle\0");
                let val = _f(cname);
                if val.is_null() {
                    device_wait_idle
                } else {
                    ::core::mem::transmute(val)
                }
            },
            allocate_memory: unsafe {
                unsafe extern "system" fn allocate_memory(
                    _device: crate::vk::Device,
                    _p_allocate_info: *const MemoryAllocateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_memory: *mut DeviceMemory,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(allocate_memory)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkAllocateMemory\0");
                let val = _f(cname);
                if val.is_null() {
                    allocate_memory
                } else {
                    ::core::mem::transmute(val)
                }
            },
            free_memory: unsafe {
                unsafe extern "system" fn free_memory(
                    _device: crate::vk::Device,
                    _memory: DeviceMemory,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(free_memory)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkFreeMemory\0");
                let val = _f(cname);
                if val.is_null() {
                    free_memory
                } else {
                    ::core::mem::transmute(val)
                }
            },
            map_memory: unsafe {
                unsafe extern "system" fn map_memory(
                    _device: crate::vk::Device,
                    _memory: DeviceMemory,
                    _offset: DeviceSize,
                    _size: DeviceSize,
                    _flags: MemoryMapFlags,
                    _pp_data: *mut *mut c_void,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(map_memory)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkMapMemory\0");
                let val = _f(cname);
                if val.is_null() {
                    map_memory
                } else {
                    ::core::mem::transmute(val)
                }
            },
            unmap_memory: unsafe {
                unsafe extern "system" fn unmap_memory(
                    _device: crate::vk::Device,
                    _memory: DeviceMemory,
                ) {
                    panic!(concat!("Unable to load ", stringify!(unmap_memory)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkUnmapMemory\0");
                let val = _f(cname);
                if val.is_null() {
                    unmap_memory
                } else {
                    ::core::mem::transmute(val)
                }
            },
            flush_mapped_memory_ranges: unsafe {
                unsafe extern "system" fn flush_mapped_memory_ranges(
                    _device: crate::vk::Device,
                    _memory_range_count: u32,
                    _p_memory_ranges: *const MappedMemoryRange<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(flush_mapped_memory_ranges)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkFlushMappedMemoryRanges\0");
                let val = _f(cname);
                if val.is_null() {
                    flush_mapped_memory_ranges
                } else {
                    ::core::mem::transmute(val)
                }
            },
            invalidate_mapped_memory_ranges: unsafe {
                unsafe extern "system" fn invalidate_mapped_memory_ranges(
                    _device: crate::vk::Device,
                    _memory_range_count: u32,
                    _p_memory_ranges: *const MappedMemoryRange<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(invalidate_mapped_memory_ranges)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkInvalidateMappedMemoryRanges\0");
                let val = _f(cname);
                if val.is_null() {
                    invalidate_mapped_memory_ranges
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_device_memory_commitment: unsafe {
                unsafe extern "system" fn get_device_memory_commitment(
                    _device: crate::vk::Device,
                    _memory: DeviceMemory,
                    _p_committed_memory_in_bytes: *mut DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_memory_commitment)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceMemoryCommitment\0");
                let val = _f(cname);
                if val.is_null() {
                    get_device_memory_commitment
                } else {
                    ::core::mem::transmute(val)
                }
            },
            bind_buffer_memory: unsafe {
                unsafe extern "system" fn bind_buffer_memory(
                    _device: crate::vk::Device,
                    _buffer: Buffer,
                    _memory: DeviceMemory,
                    _memory_offset: DeviceSize,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(bind_buffer_memory)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory\0");
                let val = _f(cname);
                if val.is_null() {
                    bind_buffer_memory
                } else {
                    ::core::mem::transmute(val)
                }
            },
            bind_image_memory: unsafe {
                unsafe extern "system" fn bind_image_memory(
                    _device: crate::vk::Device,
                    _image: Image,
                    _memory: DeviceMemory,
                    _memory_offset: DeviceSize,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(bind_image_memory)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory\0");
                let val = _f(cname);
                if val.is_null() {
                    bind_image_memory
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_buffer_memory_requirements: unsafe {
                unsafe extern "system" fn get_buffer_memory_requirements(
                    _device: crate::vk::Device,
                    _buffer: Buffer,
                    _p_memory_requirements: *mut MemoryRequirements,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_memory_requirements)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetBufferMemoryRequirements\0");
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_memory_requirements
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_image_memory_requirements: unsafe {
                unsafe extern "system" fn get_image_memory_requirements(
                    _device: crate::vk::Device,
                    _image: Image,
                    _p_memory_requirements: *mut MemoryRequirements,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_memory_requirements)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetImageMemoryRequirements\0");
                let val = _f(cname);
                if val.is_null() {
                    get_image_memory_requirements
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_image_sparse_memory_requirements: unsafe {
                unsafe extern "system" fn get_image_sparse_memory_requirements(
                    _device: crate::vk::Device,
                    _image: Image,
                    _p_sparse_memory_requirement_count: *mut u32,
                    _p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_sparse_memory_requirements)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetImageSparseMemoryRequirements\0");
                let val = _f(cname);
                if val.is_null() {
                    get_image_sparse_memory_requirements
                } else {
                    ::core::mem::transmute(val)
                }
            },
            queue_bind_sparse: unsafe {
                unsafe extern "system" fn queue_bind_sparse(
                    _queue: Queue,
                    _bind_info_count: u32,
                    _p_bind_info: *const BindSparseInfo<'_>,
                    _fence: Fence,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(queue_bind_sparse)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkQueueBindSparse\0");
                let val = _f(cname);
                if val.is_null() {
                    queue_bind_sparse
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_fence: unsafe {
                unsafe extern "system" fn create_fence(
                    _device: crate::vk::Device,
                    _p_create_info: *const FenceCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_fence: *mut Fence,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_fence)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateFence\0");
                let val = _f(cname);
                if val.is_null() {
                    create_fence
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_fence: unsafe {
                unsafe extern "system" fn destroy_fence(
                    _device: crate::vk::Device,
                    _fence: Fence,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_fence)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyFence\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_fence
                } else {
                    ::core::mem::transmute(val)
                }
            },
            reset_fences: unsafe {
                unsafe extern "system" fn reset_fences(
                    _device: crate::vk::Device,
                    _fence_count: u32,
                    _p_fences: *const Fence,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(reset_fences)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkResetFences\0");
                let val = _f(cname);
                if val.is_null() {
                    reset_fences
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_fence_status: unsafe {
                unsafe extern "system" fn get_fence_status(
                    _device: crate::vk::Device,
                    _fence: Fence,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_fence_status)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetFenceStatus\0");
                let val = _f(cname);
                if val.is_null() {
                    get_fence_status
                } else {
                    ::core::mem::transmute(val)
                }
            },
            wait_for_fences: unsafe {
                unsafe extern "system" fn wait_for_fences(
                    _device: crate::vk::Device,
                    _fence_count: u32,
                    _p_fences: *const Fence,
                    _wait_all: Bool32,
                    _timeout: u64,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(wait_for_fences)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkWaitForFences\0");
                let val = _f(cname);
                if val.is_null() {
                    wait_for_fences
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_semaphore: unsafe {
                unsafe extern "system" fn create_semaphore(
                    _device: crate::vk::Device,
                    _p_create_info: *const SemaphoreCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_semaphore: *mut Semaphore,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_semaphore)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateSemaphore\0");
                let val = _f(cname);
                if val.is_null() {
                    create_semaphore
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_semaphore: unsafe {
                unsafe extern "system" fn destroy_semaphore(
                    _device: crate::vk::Device,
                    _semaphore: Semaphore,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_semaphore)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroySemaphore\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_semaphore
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_event: unsafe {
                unsafe extern "system" fn create_event(
                    _device: crate::vk::Device,
                    _p_create_info: *const EventCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_event: *mut Event,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_event)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateEvent\0");
                let val = _f(cname);
                if val.is_null() {
                    create_event
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_event: unsafe {
                unsafe extern "system" fn destroy_event(
                    _device: crate::vk::Device,
                    _event: Event,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_event)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyEvent\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_event
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_event_status: unsafe {
                unsafe extern "system" fn get_event_status(
                    _device: crate::vk::Device,
                    _event: Event,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_event_status)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetEventStatus\0");
                let val = _f(cname);
                if val.is_null() {
                    get_event_status
                } else {
                    ::core::mem::transmute(val)
                }
            },
            set_event: unsafe {
                unsafe extern "system" fn set_event(
                    _device: crate::vk::Device,
                    _event: Event,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(set_event)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkSetEvent\0");
                let val = _f(cname);
                if val.is_null() {
                    set_event
                } else {
                    ::core::mem::transmute(val)
                }
            },
            reset_event: unsafe {
                unsafe extern "system" fn reset_event(
                    _device: crate::vk::Device,
                    _event: Event,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(reset_event)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkResetEvent\0");
                let val = _f(cname);
                if val.is_null() {
                    reset_event
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_query_pool: unsafe {
                unsafe extern "system" fn create_query_pool(
                    _device: crate::vk::Device,
                    _p_create_info: *const QueryPoolCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_query_pool: *mut QueryPool,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_query_pool)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateQueryPool\0");
                let val = _f(cname);
                if val.is_null() {
                    create_query_pool
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_query_pool: unsafe {
                unsafe extern "system" fn destroy_query_pool(
                    _device: crate::vk::Device,
                    _query_pool: QueryPool,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_query_pool)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyQueryPool\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_query_pool
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_query_pool_results: unsafe {
                unsafe extern "system" fn get_query_pool_results(
                    _device: crate::vk::Device,
                    _query_pool: QueryPool,
                    _first_query: u32,
                    _query_count: u32,
                    _data_size: usize,
                    _p_data: *mut c_void,
                    _stride: DeviceSize,
                    _flags: QueryResultFlags,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_query_pool_results)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetQueryPoolResults\0");
                let val = _f(cname);
                if val.is_null() {
                    get_query_pool_results
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_buffer: unsafe {
                unsafe extern "system" fn create_buffer(
                    _device: crate::vk::Device,
                    _p_create_info: *const BufferCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_buffer: *mut Buffer,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_buffer)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    create_buffer
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_buffer: unsafe {
                unsafe extern "system" fn destroy_buffer(
                    _device: crate::vk::Device,
                    _buffer: Buffer,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_buffer)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_buffer
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_buffer_view: unsafe {
                unsafe extern "system" fn create_buffer_view(
                    _device: crate::vk::Device,
                    _p_create_info: *const BufferViewCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_view: *mut BufferView,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_buffer_view)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateBufferView\0");
                let val = _f(cname);
                if val.is_null() {
                    create_buffer_view
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_buffer_view: unsafe {
                unsafe extern "system" fn destroy_buffer_view(
                    _device: crate::vk::Device,
                    _buffer_view: BufferView,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_buffer_view)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyBufferView\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_buffer_view
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_image: unsafe {
                unsafe extern "system" fn create_image(
                    _device: crate::vk::Device,
                    _p_create_info: *const ImageCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_image: *mut Image,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_image)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateImage\0");
                let val = _f(cname);
                if val.is_null() {
                    create_image
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_image: unsafe {
                unsafe extern "system" fn destroy_image(
                    _device: crate::vk::Device,
                    _image: Image,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_image)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyImage\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_image
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_image_subresource_layout: unsafe {
                unsafe extern "system" fn get_image_subresource_layout(
                    _device: crate::vk::Device,
                    _image: Image,
                    _p_subresource: *const ImageSubresource,
                    _p_layout: *mut SubresourceLayout,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_subresource_layout)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetImageSubresourceLayout\0");
                let val = _f(cname);
                if val.is_null() {
                    get_image_subresource_layout
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_image_view: unsafe {
                unsafe extern "system" fn create_image_view(
                    _device: crate::vk::Device,
                    _p_create_info: *const ImageViewCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_view: *mut ImageView,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_image_view)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateImageView\0");
                let val = _f(cname);
                if val.is_null() {
                    create_image_view
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_image_view: unsafe {
                unsafe extern "system" fn destroy_image_view(
                    _device: crate::vk::Device,
                    _image_view: ImageView,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_image_view)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyImageView\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_image_view
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_shader_module: unsafe {
                unsafe extern "system" fn create_shader_module(
                    _device: crate::vk::Device,
                    _p_create_info: *const ShaderModuleCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_shader_module: *mut ShaderModule,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_shader_module)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateShaderModule\0");
                let val = _f(cname);
                if val.is_null() {
                    create_shader_module
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_shader_module: unsafe {
                unsafe extern "system" fn destroy_shader_module(
                    _device: crate::vk::Device,
                    _shader_module: ShaderModule,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_shader_module)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyShaderModule\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_shader_module
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_pipeline_cache: unsafe {
                unsafe extern "system" fn create_pipeline_cache(
                    _device: crate::vk::Device,
                    _p_create_info: *const PipelineCacheCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_pipeline_cache: *mut PipelineCache,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_pipeline_cache)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreatePipelineCache\0");
                let val = _f(cname);
                if val.is_null() {
                    create_pipeline_cache
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_pipeline_cache: unsafe {
                unsafe extern "system" fn destroy_pipeline_cache(
                    _device: crate::vk::Device,
                    _pipeline_cache: PipelineCache,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_pipeline_cache)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyPipelineCache\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_pipeline_cache
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_pipeline_cache_data: unsafe {
                unsafe extern "system" fn get_pipeline_cache_data(
                    _device: crate::vk::Device,
                    _pipeline_cache: PipelineCache,
                    _p_data_size: *mut usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_pipeline_cache_data)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetPipelineCacheData\0");
                let val = _f(cname);
                if val.is_null() {
                    get_pipeline_cache_data
                } else {
                    ::core::mem::transmute(val)
                }
            },
            merge_pipeline_caches: unsafe {
                unsafe extern "system" fn merge_pipeline_caches(
                    _device: crate::vk::Device,
                    _dst_cache: PipelineCache,
                    _src_cache_count: u32,
                    _p_src_caches: *const PipelineCache,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(merge_pipeline_caches)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkMergePipelineCaches\0");
                let val = _f(cname);
                if val.is_null() {
                    merge_pipeline_caches
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_graphics_pipelines: unsafe {
                unsafe extern "system" fn create_graphics_pipelines(
                    _device: crate::vk::Device,
                    _pipeline_cache: PipelineCache,
                    _create_info_count: u32,
                    _p_create_infos: *const GraphicsPipelineCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_pipelines: *mut Pipeline,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_graphics_pipelines)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateGraphicsPipelines\0");
                let val = _f(cname);
                if val.is_null() {
                    create_graphics_pipelines
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_compute_pipelines: unsafe {
                unsafe extern "system" fn create_compute_pipelines(
                    _device: crate::vk::Device,
                    _pipeline_cache: PipelineCache,
                    _create_info_count: u32,
                    _p_create_infos: *const ComputePipelineCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_pipelines: *mut Pipeline,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_compute_pipelines)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateComputePipelines\0");
                let val = _f(cname);
                if val.is_null() {
                    create_compute_pipelines
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_pipeline: unsafe {
                unsafe extern "system" fn destroy_pipeline(
                    _device: crate::vk::Device,
                    _pipeline: Pipeline,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_pipeline)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyPipeline\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_pipeline
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_pipeline_layout: unsafe {
                unsafe extern "system" fn create_pipeline_layout(
                    _device: crate::vk::Device,
                    _p_create_info: *const PipelineLayoutCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_pipeline_layout: *mut PipelineLayout,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_pipeline_layout)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreatePipelineLayout\0");
                let val = _f(cname);
                if val.is_null() {
                    create_pipeline_layout
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_pipeline_layout: unsafe {
                unsafe extern "system" fn destroy_pipeline_layout(
                    _device: crate::vk::Device,
                    _pipeline_layout: PipelineLayout,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_pipeline_layout)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyPipelineLayout\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_pipeline_layout
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_sampler: unsafe {
                unsafe extern "system" fn create_sampler(
                    _device: crate::vk::Device,
                    _p_create_info: *const SamplerCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_sampler: *mut Sampler,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_sampler)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateSampler\0");
                let val = _f(cname);
                if val.is_null() {
                    create_sampler
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_sampler: unsafe {
                unsafe extern "system" fn destroy_sampler(
                    _device: crate::vk::Device,
                    _sampler: Sampler,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_sampler)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroySampler\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_sampler
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_descriptor_set_layout: unsafe {
                unsafe extern "system" fn create_descriptor_set_layout(
                    _device: crate::vk::Device,
                    _p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_set_layout: *mut DescriptorSetLayout,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_descriptor_set_layout)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateDescriptorSetLayout\0");
                let val = _f(cname);
                if val.is_null() {
                    create_descriptor_set_layout
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_descriptor_set_layout: unsafe {
                unsafe extern "system" fn destroy_descriptor_set_layout(
                    _device: crate::vk::Device,
                    _descriptor_set_layout: DescriptorSetLayout,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_descriptor_set_layout)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyDescriptorSetLayout\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_descriptor_set_layout
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_descriptor_pool: unsafe {
                unsafe extern "system" fn create_descriptor_pool(
                    _device: crate::vk::Device,
                    _p_create_info: *const DescriptorPoolCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_descriptor_pool: *mut DescriptorPool,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_descriptor_pool)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateDescriptorPool\0");
                let val = _f(cname);
                if val.is_null() {
                    create_descriptor_pool
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_descriptor_pool: unsafe {
                unsafe extern "system" fn destroy_descriptor_pool(
                    _device: crate::vk::Device,
                    _descriptor_pool: DescriptorPool,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_descriptor_pool)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyDescriptorPool\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_descriptor_pool
                } else {
                    ::core::mem::transmute(val)
                }
            },
            reset_descriptor_pool: unsafe {
                unsafe extern "system" fn reset_descriptor_pool(
                    _device: crate::vk::Device,
                    _descriptor_pool: DescriptorPool,
                    _flags: DescriptorPoolResetFlags,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(reset_descriptor_pool)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkResetDescriptorPool\0");
                let val = _f(cname);
                if val.is_null() {
                    reset_descriptor_pool
                } else {
                    ::core::mem::transmute(val)
                }
            },
            allocate_descriptor_sets: unsafe {
                unsafe extern "system" fn allocate_descriptor_sets(
                    _device: crate::vk::Device,
                    _p_allocate_info: *const DescriptorSetAllocateInfo<'_>,
                    _p_descriptor_sets: *mut DescriptorSet,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(allocate_descriptor_sets)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkAllocateDescriptorSets\0");
                let val = _f(cname);
                if val.is_null() {
                    allocate_descriptor_sets
                } else {
                    ::core::mem::transmute(val)
                }
            },
            free_descriptor_sets: unsafe {
                unsafe extern "system" fn free_descriptor_sets(
                    _device: crate::vk::Device,
                    _descriptor_pool: DescriptorPool,
                    _descriptor_set_count: u32,
                    _p_descriptor_sets: *const DescriptorSet,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(free_descriptor_sets)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkFreeDescriptorSets\0");
                let val = _f(cname);
                if val.is_null() {
                    free_descriptor_sets
                } else {
                    ::core::mem::transmute(val)
                }
            },
            update_descriptor_sets: unsafe {
                unsafe extern "system" fn update_descriptor_sets(
                    _device: crate::vk::Device,
                    _descriptor_write_count: u32,
                    _p_descriptor_writes: *const WriteDescriptorSet<'_>,
                    _descriptor_copy_count: u32,
                    _p_descriptor_copies: *const CopyDescriptorSet<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(update_descriptor_sets)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkUpdateDescriptorSets\0");
                let val = _f(cname);
                if val.is_null() {
                    update_descriptor_sets
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_framebuffer: unsafe {
                unsafe extern "system" fn create_framebuffer(
                    _device: crate::vk::Device,
                    _p_create_info: *const FramebufferCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_framebuffer: *mut Framebuffer,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_framebuffer)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateFramebuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    create_framebuffer
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_framebuffer: unsafe {
                unsafe extern "system" fn destroy_framebuffer(
                    _device: crate::vk::Device,
                    _framebuffer: Framebuffer,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_framebuffer)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyFramebuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_framebuffer
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_render_pass: unsafe {
                unsafe extern "system" fn create_render_pass(
                    _device: crate::vk::Device,
                    _p_create_info: *const RenderPassCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_render_pass: *mut RenderPass,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_render_pass)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateRenderPass\0");
                let val = _f(cname);
                if val.is_null() {
                    create_render_pass
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_render_pass: unsafe {
                unsafe extern "system" fn destroy_render_pass(
                    _device: crate::vk::Device,
                    _render_pass: RenderPass,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_render_pass)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyRenderPass\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_render_pass
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_render_area_granularity: unsafe {
                unsafe extern "system" fn get_render_area_granularity(
                    _device: crate::vk::Device,
                    _render_pass: RenderPass,
                    _p_granularity: *mut Extent2D,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_render_area_granularity)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetRenderAreaGranularity\0");
                let val = _f(cname);
                if val.is_null() {
                    get_render_area_granularity
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_command_pool: unsafe {
                unsafe extern "system" fn create_command_pool(
                    _device: crate::vk::Device,
                    _p_create_info: *const CommandPoolCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_command_pool: *mut CommandPool,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_command_pool)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateCommandPool\0");
                let val = _f(cname);
                if val.is_null() {
                    create_command_pool
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_command_pool: unsafe {
                unsafe extern "system" fn destroy_command_pool(
                    _device: crate::vk::Device,
                    _command_pool: CommandPool,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_command_pool)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyCommandPool\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_command_pool
                } else {
                    ::core::mem::transmute(val)
                }
            },
            reset_command_pool: unsafe {
                unsafe extern "system" fn reset_command_pool(
                    _device: crate::vk::Device,
                    _command_pool: CommandPool,
                    _flags: CommandPoolResetFlags,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(reset_command_pool)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkResetCommandPool\0");
                let val = _f(cname);
                if val.is_null() {
                    reset_command_pool
                } else {
                    ::core::mem::transmute(val)
                }
            },
            allocate_command_buffers: unsafe {
                unsafe extern "system" fn allocate_command_buffers(
                    _device: crate::vk::Device,
                    _p_allocate_info: *const CommandBufferAllocateInfo<'_>,
                    _p_command_buffers: *mut CommandBuffer,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(allocate_command_buffers)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkAllocateCommandBuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    allocate_command_buffers
                } else {
                    ::core::mem::transmute(val)
                }
            },
            free_command_buffers: unsafe {
                unsafe extern "system" fn free_command_buffers(
                    _device: crate::vk::Device,
                    _command_pool: CommandPool,
                    _command_buffer_count: u32,
                    _p_command_buffers: *const CommandBuffer,
                ) {
                    panic!(concat!("Unable to load ", stringify!(free_command_buffers)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkFreeCommandBuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    free_command_buffers
                } else {
                    ::core::mem::transmute(val)
                }
            },
            begin_command_buffer: unsafe {
                unsafe extern "system" fn begin_command_buffer(
                    _command_buffer: CommandBuffer,
                    _p_begin_info: *const CommandBufferBeginInfo<'_>,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(begin_command_buffer)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkBeginCommandBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    begin_command_buffer
                } else {
                    ::core::mem::transmute(val)
                }
            },
            end_command_buffer: unsafe {
                unsafe extern "system" fn end_command_buffer(
                    _command_buffer: CommandBuffer,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(end_command_buffer)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkEndCommandBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    end_command_buffer
                } else {
                    ::core::mem::transmute(val)
                }
            },
            reset_command_buffer: unsafe {
                unsafe extern "system" fn reset_command_buffer(
                    _command_buffer: CommandBuffer,
                    _flags: CommandBufferResetFlags,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(reset_command_buffer)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkResetCommandBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    reset_command_buffer
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_bind_pipeline: unsafe {
                unsafe extern "system" fn cmd_bind_pipeline(
                    _command_buffer: CommandBuffer,
                    _pipeline_bind_point: PipelineBindPoint,
                    _pipeline: Pipeline,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_bind_pipeline)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBindPipeline\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_pipeline
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_viewport: unsafe {
                unsafe extern "system" fn cmd_set_viewport(
                    _command_buffer: CommandBuffer,
                    _first_viewport: u32,
                    _viewport_count: u32,
                    _p_viewports: *const Viewport,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_viewport)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewport\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_scissor: unsafe {
                unsafe extern "system" fn cmd_set_scissor(
                    _command_buffer: CommandBuffer,
                    _first_scissor: u32,
                    _scissor_count: u32,
                    _p_scissors: *const Rect2D,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_scissor)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetScissor\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_scissor
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_line_width: unsafe {
                unsafe extern "system" fn cmd_set_line_width(
                    _command_buffer: CommandBuffer,
                    _line_width: f32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_line_width)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineWidth\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_line_width
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_depth_bias: unsafe {
                unsafe extern "system" fn cmd_set_depth_bias(
                    _command_buffer: CommandBuffer,
                    _depth_bias_constant_factor: f32,
                    _depth_bias_clamp: f32,
                    _depth_bias_slope_factor: f32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_depth_bias)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBias\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_bias
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_blend_constants: unsafe {
                unsafe extern "system" fn cmd_set_blend_constants(
                    _command_buffer: CommandBuffer,
                    _blend_constants: *const [f32; 4usize],
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_blend_constants)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetBlendConstants\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_blend_constants
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_depth_bounds: unsafe {
                unsafe extern "system" fn cmd_set_depth_bounds(
                    _command_buffer: CommandBuffer,
                    _min_depth_bounds: f32,
                    _max_depth_bounds: f32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_depth_bounds)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBounds\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_bounds
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_stencil_compare_mask: unsafe {
                unsafe extern "system" fn cmd_set_stencil_compare_mask(
                    _command_buffer: CommandBuffer,
                    _face_mask: StencilFaceFlags,
                    _compare_mask: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_stencil_compare_mask)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilCompareMask\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_stencil_compare_mask
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_stencil_write_mask: unsafe {
                unsafe extern "system" fn cmd_set_stencil_write_mask(
                    _command_buffer: CommandBuffer,
                    _face_mask: StencilFaceFlags,
                    _write_mask: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_stencil_write_mask)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilWriteMask\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_stencil_write_mask
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_stencil_reference: unsafe {
                unsafe extern "system" fn cmd_set_stencil_reference(
                    _command_buffer: CommandBuffer,
                    _face_mask: StencilFaceFlags,
                    _reference: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_stencil_reference)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilReference\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_stencil_reference
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_bind_descriptor_sets: unsafe {
                unsafe extern "system" fn cmd_bind_descriptor_sets(
                    _command_buffer: CommandBuffer,
                    _pipeline_bind_point: PipelineBindPoint,
                    _layout: PipelineLayout,
                    _first_set: u32,
                    _descriptor_set_count: u32,
                    _p_descriptor_sets: *const DescriptorSet,
                    _dynamic_offset_count: u32,
                    _p_dynamic_offsets: *const u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_descriptor_sets)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBindDescriptorSets\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_descriptor_sets
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_bind_index_buffer: unsafe {
                unsafe extern "system" fn cmd_bind_index_buffer(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _index_type: IndexType,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_index_buffer)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBindIndexBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_index_buffer
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_bind_vertex_buffers: unsafe {
                unsafe extern "system" fn cmd_bind_vertex_buffers(
                    _command_buffer: CommandBuffer,
                    _first_binding: u32,
                    _binding_count: u32,
                    _p_buffers: *const Buffer,
                    _p_offsets: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_vertex_buffers)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBindVertexBuffers\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_vertex_buffers
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_draw: unsafe {
                unsafe extern "system" fn cmd_draw(
                    _command_buffer: CommandBuffer,
                    _vertex_count: u32,
                    _instance_count: u32,
                    _first_vertex: u32,
                    _first_instance: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_draw)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDraw\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_draw_indexed: unsafe {
                unsafe extern "system" fn cmd_draw_indexed(
                    _command_buffer: CommandBuffer,
                    _index_count: u32,
                    _instance_count: u32,
                    _first_index: u32,
                    _vertex_offset: i32,
                    _first_instance: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_draw_indexed)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndexed\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indexed
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_draw_indirect: unsafe {
                unsafe extern "system" fn cmd_draw_indirect(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_draw_indirect)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirect\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indirect
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_draw_indexed_indirect: unsafe {
                unsafe extern "system" fn cmd_draw_indexed_indirect(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indexed_indirect)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndexedIndirect\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indexed_indirect
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_dispatch: unsafe {
                unsafe extern "system" fn cmd_dispatch(
                    _command_buffer: CommandBuffer,
                    _group_count_x: u32,
                    _group_count_y: u32,
                    _group_count_z: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_dispatch)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatch\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_dispatch
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_dispatch_indirect: unsafe {
                unsafe extern "system" fn cmd_dispatch_indirect(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_dispatch_indirect)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchIndirect\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_dispatch_indirect
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_copy_buffer: unsafe {
                unsafe extern "system" fn cmd_copy_buffer(
                    _command_buffer: CommandBuffer,
                    _src_buffer: Buffer,
                    _dst_buffer: Buffer,
                    _region_count: u32,
                    _p_regions: *const BufferCopy,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_copy_buffer)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_buffer
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_copy_image: unsafe {
                unsafe extern "system" fn cmd_copy_image(
                    _command_buffer: CommandBuffer,
                    _src_image: Image,
                    _src_image_layout: ImageLayout,
                    _dst_image: Image,
                    _dst_image_layout: ImageLayout,
                    _region_count: u32,
                    _p_regions: *const ImageCopy,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_copy_image)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImage\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_image
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_blit_image: unsafe {
                unsafe extern "system" fn cmd_blit_image(
                    _command_buffer: CommandBuffer,
                    _src_image: Image,
                    _src_image_layout: ImageLayout,
                    _dst_image: Image,
                    _dst_image_layout: ImageLayout,
                    _region_count: u32,
                    _p_regions: *const ImageBlit,
                    _filter: Filter,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_blit_image)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBlitImage\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_blit_image
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_copy_buffer_to_image: unsafe {
                unsafe extern "system" fn cmd_copy_buffer_to_image(
                    _command_buffer: CommandBuffer,
                    _src_buffer: Buffer,
                    _dst_image: Image,
                    _dst_image_layout: ImageLayout,
                    _region_count: u32,
                    _p_regions: *const BufferImageCopy,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_buffer_to_image)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBufferToImage\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_buffer_to_image
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_copy_image_to_buffer: unsafe {
                unsafe extern "system" fn cmd_copy_image_to_buffer(
                    _command_buffer: CommandBuffer,
                    _src_image: Image,
                    _src_image_layout: ImageLayout,
                    _dst_buffer: Buffer,
                    _region_count: u32,
                    _p_regions: *const BufferImageCopy,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_image_to_buffer)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImageToBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_image_to_buffer
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_update_buffer: unsafe {
                unsafe extern "system" fn cmd_update_buffer(
                    _command_buffer: CommandBuffer,
                    _dst_buffer: Buffer,
                    _dst_offset: DeviceSize,
                    _data_size: DeviceSize,
                    _p_data: *const c_void,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_update_buffer)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdUpdateBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_update_buffer
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_fill_buffer: unsafe {
                unsafe extern "system" fn cmd_fill_buffer(
                    _command_buffer: CommandBuffer,
                    _dst_buffer: Buffer,
                    _dst_offset: DeviceSize,
                    _size: DeviceSize,
                    _data: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_fill_buffer)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdFillBuffer\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_fill_buffer
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_clear_color_image: unsafe {
                unsafe extern "system" fn cmd_clear_color_image(
                    _command_buffer: CommandBuffer,
                    _image: Image,
                    _image_layout: ImageLayout,
                    _p_color: *const ClearColorValue,
                    _range_count: u32,
                    _p_ranges: *const ImageSubresourceRange,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_clear_color_image)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdClearColorImage\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_clear_color_image
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_clear_depth_stencil_image: unsafe {
                unsafe extern "system" fn cmd_clear_depth_stencil_image(
                    _command_buffer: CommandBuffer,
                    _image: Image,
                    _image_layout: ImageLayout,
                    _p_depth_stencil: *const ClearDepthStencilValue,
                    _range_count: u32,
                    _p_ranges: *const ImageSubresourceRange,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_clear_depth_stencil_image)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdClearDepthStencilImage\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_clear_depth_stencil_image
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_clear_attachments: unsafe {
                unsafe extern "system" fn cmd_clear_attachments(
                    _command_buffer: CommandBuffer,
                    _attachment_count: u32,
                    _p_attachments: *const ClearAttachment,
                    _rect_count: u32,
                    _p_rects: *const ClearRect,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_clear_attachments)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdClearAttachments\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_clear_attachments
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_resolve_image: unsafe {
                unsafe extern "system" fn cmd_resolve_image(
                    _command_buffer: CommandBuffer,
                    _src_image: Image,
                    _src_image_layout: ImageLayout,
                    _dst_image: Image,
                    _dst_image_layout: ImageLayout,
                    _region_count: u32,
                    _p_regions: *const ImageResolve,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_resolve_image)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdResolveImage\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_resolve_image
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_event: unsafe {
                unsafe extern "system" fn cmd_set_event(
                    _command_buffer: CommandBuffer,
                    _event: Event,
                    _stage_mask: PipelineStageFlags,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_event)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetEvent\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_event
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_reset_event: unsafe {
                unsafe extern "system" fn cmd_reset_event(
                    _command_buffer: CommandBuffer,
                    _event: Event,
                    _stage_mask: PipelineStageFlags,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_reset_event)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdResetEvent\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_reset_event
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_wait_events: unsafe {
                unsafe extern "system" fn cmd_wait_events(
                    _command_buffer: CommandBuffer,
                    _event_count: u32,
                    _p_events: *const Event,
                    _src_stage_mask: PipelineStageFlags,
                    _dst_stage_mask: PipelineStageFlags,
                    _memory_barrier_count: u32,
                    _p_memory_barriers: *const MemoryBarrier<'_>,
                    _buffer_memory_barrier_count: u32,
                    _p_buffer_memory_barriers: *const BufferMemoryBarrier<'_>,
                    _image_memory_barrier_count: u32,
                    _p_image_memory_barriers: *const ImageMemoryBarrier<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_wait_events)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdWaitEvents\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_wait_events
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_pipeline_barrier: unsafe {
                unsafe extern "system" fn cmd_pipeline_barrier(
                    _command_buffer: CommandBuffer,
                    _src_stage_mask: PipelineStageFlags,
                    _dst_stage_mask: PipelineStageFlags,
                    _dependency_flags: DependencyFlags,
                    _memory_barrier_count: u32,
                    _p_memory_barriers: *const MemoryBarrier<'_>,
                    _buffer_memory_barrier_count: u32,
                    _p_buffer_memory_barriers: *const BufferMemoryBarrier<'_>,
                    _image_memory_barrier_count: u32,
                    _p_image_memory_barriers: *const ImageMemoryBarrier<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_pipeline_barrier)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdPipelineBarrier\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_pipeline_barrier
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_begin_query: unsafe {
                unsafe extern "system" fn cmd_begin_query(
                    _command_buffer: CommandBuffer,
                    _query_pool: QueryPool,
                    _query: u32,
                    _flags: QueryControlFlags,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_begin_query)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginQuery\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_query
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_end_query: unsafe {
                unsafe extern "system" fn cmd_end_query(
                    _command_buffer: CommandBuffer,
                    _query_pool: QueryPool,
                    _query: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_end_query)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdEndQuery\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_query
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_reset_query_pool: unsafe {
                unsafe extern "system" fn cmd_reset_query_pool(
                    _command_buffer: CommandBuffer,
                    _query_pool: QueryPool,
                    _first_query: u32,
                    _query_count: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_reset_query_pool)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdResetQueryPool\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_reset_query_pool
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_write_timestamp: unsafe {
                unsafe extern "system" fn cmd_write_timestamp(
                    _command_buffer: CommandBuffer,
                    _pipeline_stage: PipelineStageFlags,
                    _query_pool: QueryPool,
                    _query: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_write_timestamp)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteTimestamp\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_timestamp
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_copy_query_pool_results: unsafe {
                unsafe extern "system" fn cmd_copy_query_pool_results(
                    _command_buffer: CommandBuffer,
                    _query_pool: QueryPool,
                    _first_query: u32,
                    _query_count: u32,
                    _dst_buffer: Buffer,
                    _dst_offset: DeviceSize,
                    _stride: DeviceSize,
                    _flags: QueryResultFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_query_pool_results)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyQueryPoolResults\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_query_pool_results
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_push_constants: unsafe {
                unsafe extern "system" fn cmd_push_constants(
                    _command_buffer: CommandBuffer,
                    _layout: PipelineLayout,
                    _stage_flags: ShaderStageFlags,
                    _offset: u32,
                    _size: u32,
                    _p_values: *const c_void,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_push_constants)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdPushConstants\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_push_constants
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_begin_render_pass: unsafe {
                unsafe extern "system" fn cmd_begin_render_pass(
                    _command_buffer: CommandBuffer,
                    _p_render_pass_begin: *const RenderPassBeginInfo<'_>,
                    _contents: SubpassContents,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_render_pass)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderPass\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_render_pass
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_next_subpass: unsafe {
                unsafe extern "system" fn cmd_next_subpass(
                    _command_buffer: CommandBuffer,
                    _contents: SubpassContents,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_next_subpass)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdNextSubpass\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_next_subpass
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_end_render_pass: unsafe {
                unsafe extern "system" fn cmd_end_render_pass(_command_buffer: CommandBuffer) {
                    panic!(concat!("Unable to load ", stringify!(cmd_end_render_pass)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderPass\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_render_pass
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_execute_commands: unsafe {
                unsafe extern "system" fn cmd_execute_commands(
                    _command_buffer: CommandBuffer,
                    _command_buffer_count: u32,
                    _p_command_buffers: *const CommandBuffer,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_execute_commands)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdExecuteCommands\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_execute_commands
                } else {
                    ::core::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
#[doc = "Raw Vulkan 1.1 entry point function pointers"]
pub struct EntryFnV1_1 {
    pub enumerate_instance_version: PFN_vkEnumerateInstanceVersion,
}
unsafe impl Send for EntryFnV1_1 {}
unsafe impl Sync for EntryFnV1_1 {}
impl EntryFnV1_1 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            enumerate_instance_version: unsafe {
                unsafe extern "system" fn enumerate_instance_version(
                    _p_api_version: *mut u32,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(enumerate_instance_version)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkEnumerateInstanceVersion\0");
                let val = _f(cname);
                if val.is_null() {
                    enumerate_instance_version
                } else {
                    ::core::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
#[doc = "Raw Vulkan 1.1 instance-level function pointers"]
pub struct InstanceFnV1_1 {
    pub enumerate_physical_device_groups: PFN_vkEnumeratePhysicalDeviceGroups,
    pub get_physical_device_features2: PFN_vkGetPhysicalDeviceFeatures2,
    pub get_physical_device_properties2: PFN_vkGetPhysicalDeviceProperties2,
    pub get_physical_device_format_properties2: PFN_vkGetPhysicalDeviceFormatProperties2,
    pub get_physical_device_image_format_properties2: PFN_vkGetPhysicalDeviceImageFormatProperties2,
    pub get_physical_device_queue_family_properties2: PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    pub get_physical_device_memory_properties2: PFN_vkGetPhysicalDeviceMemoryProperties2,
    pub get_physical_device_sparse_image_format_properties2:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
    pub get_physical_device_external_buffer_properties:
        PFN_vkGetPhysicalDeviceExternalBufferProperties,
    pub get_physical_device_external_fence_properties:
        PFN_vkGetPhysicalDeviceExternalFenceProperties,
    pub get_physical_device_external_semaphore_properties:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
}
unsafe impl Send for InstanceFnV1_1 {}
unsafe impl Sync for InstanceFnV1_1 {}
impl InstanceFnV1_1 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            enumerate_physical_device_groups: unsafe {
                unsafe extern "system" fn enumerate_physical_device_groups(
                    _instance: crate::vk::Instance,
                    _p_physical_device_group_count: *mut u32,
                    _p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(enumerate_physical_device_groups)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkEnumeratePhysicalDeviceGroups\0");
                let val = _f(cname);
                if val.is_null() {
                    enumerate_physical_device_groups
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_features2: unsafe {
                unsafe extern "system" fn get_physical_device_features2(
                    _physical_device: PhysicalDevice,
                    _p_features: *mut PhysicalDeviceFeatures2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_features2)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceFeatures2\0");
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_features2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_properties2: unsafe {
                unsafe extern "system" fn get_physical_device_properties2(
                    _physical_device: PhysicalDevice,
                    _p_properties: *mut PhysicalDeviceProperties2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_properties2)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceProperties2\0");
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_properties2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_format_properties2: unsafe {
                unsafe extern "system" fn get_physical_device_format_properties2(
                    _physical_device: PhysicalDevice,
                    _format: Format,
                    _p_format_properties: *mut FormatProperties2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_format_properties2)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceFormatProperties2\0");
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_format_properties2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_image_format_properties2: unsafe {
                unsafe extern "system" fn get_physical_device_image_format_properties2(
                    _physical_device: PhysicalDevice,
                    _p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
                    _p_image_format_properties: *mut ImageFormatProperties2<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_image_format_properties2)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceImageFormatProperties2\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_image_format_properties2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_queue_family_properties2: unsafe {
                unsafe extern "system" fn get_physical_device_queue_family_properties2(
                    _physical_device: PhysicalDevice,
                    _p_queue_family_property_count: *mut u32,
                    _p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_queue_family_properties2)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceQueueFamilyProperties2\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_queue_family_properties2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_memory_properties2: unsafe {
                unsafe extern "system" fn get_physical_device_memory_properties2(
                    _physical_device: PhysicalDevice,
                    _p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_memory_properties2)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceMemoryProperties2\0");
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_memory_properties2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_sparse_image_format_properties2: unsafe {
                unsafe extern "system" fn get_physical_device_sparse_image_format_properties2(
                    _physical_device: PhysicalDevice,
                    _p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
                    _p_property_count: *mut u32,
                    _p_properties: *mut SparseImageFormatProperties2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_sparse_image_format_properties2)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSparseImageFormatProperties2\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_sparse_image_format_properties2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_external_buffer_properties: unsafe {
                unsafe extern "system" fn get_physical_device_external_buffer_properties(
                    _physical_device: PhysicalDevice,
                    _p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo<'_>,
                    _p_external_buffer_properties: *mut ExternalBufferProperties<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_external_buffer_properties)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalBufferProperties\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_external_buffer_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_external_fence_properties: unsafe {
                unsafe extern "system" fn get_physical_device_external_fence_properties(
                    _physical_device: PhysicalDevice,
                    _p_external_fence_info: *const PhysicalDeviceExternalFenceInfo<'_>,
                    _p_external_fence_properties: *mut ExternalFenceProperties<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_external_fence_properties)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalFenceProperties\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_external_fence_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_physical_device_external_semaphore_properties: unsafe {
                unsafe extern "system" fn get_physical_device_external_semaphore_properties(
                    _physical_device: PhysicalDevice,
                    _p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo<'_>,
                    _p_external_semaphore_properties: *mut ExternalSemaphoreProperties<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_external_semaphore_properties)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalSemaphoreProperties\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_external_semaphore_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
#[doc = "Raw Vulkan 1.1 device-level function pointers"]
pub struct DeviceFnV1_1 {
    pub bind_buffer_memory2: PFN_vkBindBufferMemory2,
    pub bind_image_memory2: PFN_vkBindImageMemory2,
    pub get_device_group_peer_memory_features: PFN_vkGetDeviceGroupPeerMemoryFeatures,
    pub cmd_set_device_mask: PFN_vkCmdSetDeviceMask,
    pub cmd_dispatch_base: PFN_vkCmdDispatchBase,
    pub get_image_memory_requirements2: PFN_vkGetImageMemoryRequirements2,
    pub get_buffer_memory_requirements2: PFN_vkGetBufferMemoryRequirements2,
    pub get_image_sparse_memory_requirements2: PFN_vkGetImageSparseMemoryRequirements2,
    pub trim_command_pool: PFN_vkTrimCommandPool,
    pub get_device_queue2: PFN_vkGetDeviceQueue2,
    pub create_sampler_ycbcr_conversion: PFN_vkCreateSamplerYcbcrConversion,
    pub destroy_sampler_ycbcr_conversion: PFN_vkDestroySamplerYcbcrConversion,
    pub create_descriptor_update_template: PFN_vkCreateDescriptorUpdateTemplate,
    pub destroy_descriptor_update_template: PFN_vkDestroyDescriptorUpdateTemplate,
    pub update_descriptor_set_with_template: PFN_vkUpdateDescriptorSetWithTemplate,
    pub get_descriptor_set_layout_support: PFN_vkGetDescriptorSetLayoutSupport,
}
unsafe impl Send for DeviceFnV1_1 {}
unsafe impl Sync for DeviceFnV1_1 {}
impl DeviceFnV1_1 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            bind_buffer_memory2: unsafe {
                unsafe extern "system" fn bind_buffer_memory2(
                    _device: crate::vk::Device,
                    _bind_info_count: u32,
                    _p_bind_infos: *const BindBufferMemoryInfo<'_>,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(bind_buffer_memory2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory2\0");
                let val = _f(cname);
                if val.is_null() {
                    bind_buffer_memory2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            bind_image_memory2: unsafe {
                unsafe extern "system" fn bind_image_memory2(
                    _device: crate::vk::Device,
                    _bind_info_count: u32,
                    _p_bind_infos: *const BindImageMemoryInfo<'_>,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(bind_image_memory2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory2\0");
                let val = _f(cname);
                if val.is_null() {
                    bind_image_memory2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_device_group_peer_memory_features: unsafe {
                unsafe extern "system" fn get_device_group_peer_memory_features(
                    _device: crate::vk::Device,
                    _heap_index: u32,
                    _local_device_index: u32,
                    _remote_device_index: u32,
                    _p_peer_memory_features: *mut PeerMemoryFeatureFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_peer_memory_features)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceGroupPeerMemoryFeatures\0");
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_peer_memory_features
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_device_mask: unsafe {
                unsafe extern "system" fn cmd_set_device_mask(
                    _command_buffer: CommandBuffer,
                    _device_mask: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_device_mask)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDeviceMask\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_device_mask
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_dispatch_base: unsafe {
                unsafe extern "system" fn cmd_dispatch_base(
                    _command_buffer: CommandBuffer,
                    _base_group_x: u32,
                    _base_group_y: u32,
                    _base_group_z: u32,
                    _group_count_x: u32,
                    _group_count_y: u32,
                    _group_count_z: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_dispatch_base)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchBase\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_dispatch_base
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_image_memory_requirements2: unsafe {
                unsafe extern "system" fn get_image_memory_requirements2(
                    _device: crate::vk::Device,
                    _p_info: *const ImageMemoryRequirementsInfo2<'_>,
                    _p_memory_requirements: *mut MemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_memory_requirements2)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetImageMemoryRequirements2\0");
                let val = _f(cname);
                if val.is_null() {
                    get_image_memory_requirements2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_buffer_memory_requirements2: unsafe {
                unsafe extern "system" fn get_buffer_memory_requirements2(
                    _device: crate::vk::Device,
                    _p_info: *const BufferMemoryRequirementsInfo2<'_>,
                    _p_memory_requirements: *mut MemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_memory_requirements2)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetBufferMemoryRequirements2\0");
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_memory_requirements2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_image_sparse_memory_requirements2: unsafe {
                unsafe extern "system" fn get_image_sparse_memory_requirements2(
                    _device: crate::vk::Device,
                    _p_info: *const ImageSparseMemoryRequirementsInfo2<'_>,
                    _p_sparse_memory_requirement_count: *mut u32,
                    _p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_sparse_memory_requirements2)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetImageSparseMemoryRequirements2\0");
                let val = _f(cname);
                if val.is_null() {
                    get_image_sparse_memory_requirements2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            trim_command_pool: unsafe {
                unsafe extern "system" fn trim_command_pool(
                    _device: crate::vk::Device,
                    _command_pool: CommandPool,
                    _flags: CommandPoolTrimFlags,
                ) {
                    panic!(concat!("Unable to load ", stringify!(trim_command_pool)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkTrimCommandPool\0");
                let val = _f(cname);
                if val.is_null() {
                    trim_command_pool
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_device_queue2: unsafe {
                unsafe extern "system" fn get_device_queue2(
                    _device: crate::vk::Device,
                    _p_queue_info: *const DeviceQueueInfo2<'_>,
                    _p_queue: *mut Queue,
                ) {
                    panic!(concat!("Unable to load ", stringify!(get_device_queue2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceQueue2\0");
                let val = _f(cname);
                if val.is_null() {
                    get_device_queue2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_sampler_ycbcr_conversion: unsafe {
                unsafe extern "system" fn create_sampler_ycbcr_conversion(
                    _device: crate::vk::Device,
                    _p_create_info: *const SamplerYcbcrConversionCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_ycbcr_conversion: *mut SamplerYcbcrConversion,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_sampler_ycbcr_conversion)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkCreateSamplerYcbcrConversion\0");
                let val = _f(cname);
                if val.is_null() {
                    create_sampler_ycbcr_conversion
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_sampler_ycbcr_conversion: unsafe {
                unsafe extern "system" fn destroy_sampler_ycbcr_conversion(
                    _device: crate::vk::Device,
                    _ycbcr_conversion: SamplerYcbcrConversion,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_sampler_ycbcr_conversion)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkDestroySamplerYcbcrConversion\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_sampler_ycbcr_conversion
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_descriptor_update_template: unsafe {
                unsafe extern "system" fn create_descriptor_update_template(
                    _device: crate::vk::Device,
                    _p_create_info: *const DescriptorUpdateTemplateCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_descriptor_update_template: *mut DescriptorUpdateTemplate,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_descriptor_update_template)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkCreateDescriptorUpdateTemplate\0");
                let val = _f(cname);
                if val.is_null() {
                    create_descriptor_update_template
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_descriptor_update_template: unsafe {
                unsafe extern "system" fn destroy_descriptor_update_template(
                    _device: crate::vk::Device,
                    _descriptor_update_template: DescriptorUpdateTemplate,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_descriptor_update_template)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkDestroyDescriptorUpdateTemplate\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_descriptor_update_template
                } else {
                    ::core::mem::transmute(val)
                }
            },
            update_descriptor_set_with_template: unsafe {
                unsafe extern "system" fn update_descriptor_set_with_template(
                    _device: crate::vk::Device,
                    _descriptor_set: DescriptorSet,
                    _descriptor_update_template: DescriptorUpdateTemplate,
                    _p_data: *const c_void,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(update_descriptor_set_with_template)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkUpdateDescriptorSetWithTemplate\0");
                let val = _f(cname);
                if val.is_null() {
                    update_descriptor_set_with_template
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_descriptor_set_layout_support: unsafe {
                unsafe extern "system" fn get_descriptor_set_layout_support(
                    _device: crate::vk::Device,
                    _p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
                    _p_support: *mut DescriptorSetLayoutSupport<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_descriptor_set_layout_support)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetDescriptorSetLayoutSupport\0");
                let val = _f(cname);
                if val.is_null() {
                    get_descriptor_set_layout_support
                } else {
                    ::core::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
#[doc = "Raw Vulkan 1.2 entry point function pointers"]
pub struct EntryFnV1_2;
#[derive(Clone)]
#[doc = "Raw Vulkan 1.2 instance-level function pointers"]
pub struct InstanceFnV1_2;
#[derive(Clone)]
#[doc = "Raw Vulkan 1.2 device-level function pointers"]
pub struct DeviceFnV1_2 {
    pub cmd_draw_indirect_count: PFN_vkCmdDrawIndirectCount,
    pub cmd_draw_indexed_indirect_count: PFN_vkCmdDrawIndexedIndirectCount,
    pub create_render_pass2: PFN_vkCreateRenderPass2,
    pub cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2,
    pub cmd_next_subpass2: PFN_vkCmdNextSubpass2,
    pub cmd_end_render_pass2: PFN_vkCmdEndRenderPass2,
    pub reset_query_pool: PFN_vkResetQueryPool,
    pub get_semaphore_counter_value: PFN_vkGetSemaphoreCounterValue,
    pub wait_semaphores: PFN_vkWaitSemaphores,
    pub signal_semaphore: PFN_vkSignalSemaphore,
    pub get_buffer_device_address: PFN_vkGetBufferDeviceAddress,
    pub get_buffer_opaque_capture_address: PFN_vkGetBufferOpaqueCaptureAddress,
    pub get_device_memory_opaque_capture_address: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
}
unsafe impl Send for DeviceFnV1_2 {}
unsafe impl Sync for DeviceFnV1_2 {}
impl DeviceFnV1_2 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            cmd_draw_indirect_count: unsafe {
                unsafe extern "system" fn cmd_draw_indirect_count(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indirect_count)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCount\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indirect_count
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_draw_indexed_indirect_count: unsafe {
                unsafe extern "system" fn cmd_draw_indexed_indirect_count(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indexed_indirect_count)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndexedIndirectCount\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indexed_indirect_count
                } else {
                    ::core::mem::transmute(val)
                }
            },
            create_render_pass2: unsafe {
                unsafe extern "system" fn create_render_pass2(
                    _device: crate::vk::Device,
                    _p_create_info: *const RenderPassCreateInfo2<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_render_pass: *mut RenderPass,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_render_pass2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateRenderPass2\0");
                let val = _f(cname);
                if val.is_null() {
                    create_render_pass2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_begin_render_pass2: unsafe {
                unsafe extern "system" fn cmd_begin_render_pass2(
                    _command_buffer: CommandBuffer,
                    _p_render_pass_begin: *const RenderPassBeginInfo<'_>,
                    _p_subpass_begin_info: *const SubpassBeginInfo<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_render_pass2)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderPass2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_render_pass2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_next_subpass2: unsafe {
                unsafe extern "system" fn cmd_next_subpass2(
                    _command_buffer: CommandBuffer,
                    _p_subpass_begin_info: *const SubpassBeginInfo<'_>,
                    _p_subpass_end_info: *const SubpassEndInfo<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_next_subpass2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdNextSubpass2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_next_subpass2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_end_render_pass2: unsafe {
                unsafe extern "system" fn cmd_end_render_pass2(
                    _command_buffer: CommandBuffer,
                    _p_subpass_end_info: *const SubpassEndInfo<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_end_render_pass2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderPass2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_render_pass2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            reset_query_pool: unsafe {
                unsafe extern "system" fn reset_query_pool(
                    _device: crate::vk::Device,
                    _query_pool: QueryPool,
                    _first_query: u32,
                    _query_count: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(reset_query_pool)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkResetQueryPool\0");
                let val = _f(cname);
                if val.is_null() {
                    reset_query_pool
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_semaphore_counter_value: unsafe {
                unsafe extern "system" fn get_semaphore_counter_value(
                    _device: crate::vk::Device,
                    _semaphore: Semaphore,
                    _p_value: *mut u64,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_semaphore_counter_value)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetSemaphoreCounterValue\0");
                let val = _f(cname);
                if val.is_null() {
                    get_semaphore_counter_value
                } else {
                    ::core::mem::transmute(val)
                }
            },
            wait_semaphores: unsafe {
                unsafe extern "system" fn wait_semaphores(
                    _device: crate::vk::Device,
                    _p_wait_info: *const SemaphoreWaitInfo<'_>,
                    _timeout: u64,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(wait_semaphores)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkWaitSemaphores\0");
                let val = _f(cname);
                if val.is_null() {
                    wait_semaphores
                } else {
                    ::core::mem::transmute(val)
                }
            },
            signal_semaphore: unsafe {
                unsafe extern "system" fn signal_semaphore(
                    _device: crate::vk::Device,
                    _p_signal_info: *const SemaphoreSignalInfo<'_>,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(signal_semaphore)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkSignalSemaphore\0");
                let val = _f(cname);
                if val.is_null() {
                    signal_semaphore
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_buffer_device_address: unsafe {
                unsafe extern "system" fn get_buffer_device_address(
                    _device: crate::vk::Device,
                    _p_info: *const BufferDeviceAddressInfo<'_>,
                ) -> DeviceAddress {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_device_address)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetBufferDeviceAddress\0");
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_device_address
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_buffer_opaque_capture_address: unsafe {
                unsafe extern "system" fn get_buffer_opaque_capture_address(
                    _device: crate::vk::Device,
                    _p_info: *const BufferDeviceAddressInfo<'_>,
                ) -> u64 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_opaque_capture_address)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetBufferOpaqueCaptureAddress\0");
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_opaque_capture_address
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_device_memory_opaque_capture_address: unsafe {
                unsafe extern "system" fn get_device_memory_opaque_capture_address(
                    _device: crate::vk::Device,
                    _p_info: *const DeviceMemoryOpaqueCaptureAddressInfo<'_>,
                ) -> u64 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_memory_opaque_capture_address)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceMemoryOpaqueCaptureAddress\0");
                let val = _f(cname);
                if val.is_null() {
                    get_device_memory_opaque_capture_address
                } else {
                    ::core::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
#[doc = "Raw Vulkan 1.3 entry point function pointers"]
pub struct EntryFnV1_3;
#[derive(Clone)]
#[doc = "Raw Vulkan 1.3 instance-level function pointers"]
pub struct InstanceFnV1_3 {
    pub get_physical_device_tool_properties: PFN_vkGetPhysicalDeviceToolProperties,
}
unsafe impl Send for InstanceFnV1_3 {}
unsafe impl Sync for InstanceFnV1_3 {}
impl InstanceFnV1_3 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            get_physical_device_tool_properties: unsafe {
                unsafe extern "system" fn get_physical_device_tool_properties(
                    _physical_device: PhysicalDevice,
                    _p_tool_count: *mut u32,
                    _p_tool_properties: *mut PhysicalDeviceToolProperties<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_tool_properties)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceToolProperties\0");
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_tool_properties
                } else {
                    ::core::mem::transmute(val)
                }
            },
        }
    }
}
#[derive(Clone)]
#[doc = "Raw Vulkan 1.3 device-level function pointers"]
pub struct DeviceFnV1_3 {
    pub create_private_data_slot: PFN_vkCreatePrivateDataSlot,
    pub destroy_private_data_slot: PFN_vkDestroyPrivateDataSlot,
    pub set_private_data: PFN_vkSetPrivateData,
    pub get_private_data: PFN_vkGetPrivateData,
    pub cmd_set_event2: PFN_vkCmdSetEvent2,
    pub cmd_reset_event2: PFN_vkCmdResetEvent2,
    pub cmd_wait_events2: PFN_vkCmdWaitEvents2,
    pub cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2,
    pub cmd_write_timestamp2: PFN_vkCmdWriteTimestamp2,
    pub queue_submit2: PFN_vkQueueSubmit2,
    pub cmd_copy_buffer2: PFN_vkCmdCopyBuffer2,
    pub cmd_copy_image2: PFN_vkCmdCopyImage2,
    pub cmd_copy_buffer_to_image2: PFN_vkCmdCopyBufferToImage2,
    pub cmd_copy_image_to_buffer2: PFN_vkCmdCopyImageToBuffer2,
    pub cmd_blit_image2: PFN_vkCmdBlitImage2,
    pub cmd_resolve_image2: PFN_vkCmdResolveImage2,
    pub cmd_begin_rendering: PFN_vkCmdBeginRendering,
    pub cmd_end_rendering: PFN_vkCmdEndRendering,
    pub cmd_set_cull_mode: PFN_vkCmdSetCullMode,
    pub cmd_set_front_face: PFN_vkCmdSetFrontFace,
    pub cmd_set_primitive_topology: PFN_vkCmdSetPrimitiveTopology,
    pub cmd_set_viewport_with_count: PFN_vkCmdSetViewportWithCount,
    pub cmd_set_scissor_with_count: PFN_vkCmdSetScissorWithCount,
    pub cmd_bind_vertex_buffers2: PFN_vkCmdBindVertexBuffers2,
    pub cmd_set_depth_test_enable: PFN_vkCmdSetDepthTestEnable,
    pub cmd_set_depth_write_enable: PFN_vkCmdSetDepthWriteEnable,
    pub cmd_set_depth_compare_op: PFN_vkCmdSetDepthCompareOp,
    pub cmd_set_depth_bounds_test_enable: PFN_vkCmdSetDepthBoundsTestEnable,
    pub cmd_set_stencil_test_enable: PFN_vkCmdSetStencilTestEnable,
    pub cmd_set_stencil_op: PFN_vkCmdSetStencilOp,
    pub cmd_set_rasterizer_discard_enable: PFN_vkCmdSetRasterizerDiscardEnable,
    pub cmd_set_depth_bias_enable: PFN_vkCmdSetDepthBiasEnable,
    pub cmd_set_primitive_restart_enable: PFN_vkCmdSetPrimitiveRestartEnable,
    pub get_device_buffer_memory_requirements: PFN_vkGetDeviceBufferMemoryRequirements,
    pub get_device_image_memory_requirements: PFN_vkGetDeviceImageMemoryRequirements,
    pub get_device_image_sparse_memory_requirements: PFN_vkGetDeviceImageSparseMemoryRequirements,
}
unsafe impl Send for DeviceFnV1_3 {}
unsafe impl Sync for DeviceFnV1_3 {}
impl DeviceFnV1_3 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            create_private_data_slot: unsafe {
                unsafe extern "system" fn create_private_data_slot(
                    _device: crate::vk::Device,
                    _p_create_info: *const PrivateDataSlotCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_private_data_slot: *mut PrivateDataSlot,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_private_data_slot)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreatePrivateDataSlot\0");
                let val = _f(cname);
                if val.is_null() {
                    create_private_data_slot
                } else {
                    ::core::mem::transmute(val)
                }
            },
            destroy_private_data_slot: unsafe {
                unsafe extern "system" fn destroy_private_data_slot(
                    _device: crate::vk::Device,
                    _private_data_slot: PrivateDataSlot,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_private_data_slot)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyPrivateDataSlot\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_private_data_slot
                } else {
                    ::core::mem::transmute(val)
                }
            },
            set_private_data: unsafe {
                unsafe extern "system" fn set_private_data(
                    _device: crate::vk::Device,
                    _object_type: ObjectType,
                    _object_handle: u64,
                    _private_data_slot: PrivateDataSlot,
                    _data: u64,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(set_private_data)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkSetPrivateData\0");
                let val = _f(cname);
                if val.is_null() {
                    set_private_data
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_private_data: unsafe {
                unsafe extern "system" fn get_private_data(
                    _device: crate::vk::Device,
                    _object_type: ObjectType,
                    _object_handle: u64,
                    _private_data_slot: PrivateDataSlot,
                    _p_data: *mut u64,
                ) {
                    panic!(concat!("Unable to load ", stringify!(get_private_data)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkGetPrivateData\0");
                let val = _f(cname);
                if val.is_null() {
                    get_private_data
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_event2: unsafe {
                unsafe extern "system" fn cmd_set_event2(
                    _command_buffer: CommandBuffer,
                    _event: Event,
                    _p_dependency_info: *const DependencyInfo<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_event2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetEvent2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_event2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_reset_event2: unsafe {
                unsafe extern "system" fn cmd_reset_event2(
                    _command_buffer: CommandBuffer,
                    _event: Event,
                    _stage_mask: PipelineStageFlags2,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_reset_event2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdResetEvent2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_reset_event2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_wait_events2: unsafe {
                unsafe extern "system" fn cmd_wait_events2(
                    _command_buffer: CommandBuffer,
                    _event_count: u32,
                    _p_events: *const Event,
                    _p_dependency_infos: *const DependencyInfo<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_wait_events2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdWaitEvents2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_wait_events2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_pipeline_barrier2: unsafe {
                unsafe extern "system" fn cmd_pipeline_barrier2(
                    _command_buffer: CommandBuffer,
                    _p_dependency_info: *const DependencyInfo<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_pipeline_barrier2)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdPipelineBarrier2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_pipeline_barrier2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_write_timestamp2: unsafe {
                unsafe extern "system" fn cmd_write_timestamp2(
                    _command_buffer: CommandBuffer,
                    _stage: PipelineStageFlags2,
                    _query_pool: QueryPool,
                    _query: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_write_timestamp2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteTimestamp2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_timestamp2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            queue_submit2: unsafe {
                unsafe extern "system" fn queue_submit2(
                    _queue: Queue,
                    _submit_count: u32,
                    _p_submits: *const SubmitInfo2<'_>,
                    _fence: Fence,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(queue_submit2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkQueueSubmit2\0");
                let val = _f(cname);
                if val.is_null() {
                    queue_submit2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_copy_buffer2: unsafe {
                unsafe extern "system" fn cmd_copy_buffer2(
                    _command_buffer: CommandBuffer,
                    _p_copy_buffer_info: *const CopyBufferInfo2<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_copy_buffer2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBuffer2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_buffer2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_copy_image2: unsafe {
                unsafe extern "system" fn cmd_copy_image2(
                    _command_buffer: CommandBuffer,
                    _p_copy_image_info: *const CopyImageInfo2<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_copy_image2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImage2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_image2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_copy_buffer_to_image2: unsafe {
                unsafe extern "system" fn cmd_copy_buffer_to_image2(
                    _command_buffer: CommandBuffer,
                    _p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_buffer_to_image2)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBufferToImage2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_buffer_to_image2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_copy_image_to_buffer2: unsafe {
                unsafe extern "system" fn cmd_copy_image_to_buffer2(
                    _command_buffer: CommandBuffer,
                    _p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_image_to_buffer2)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImageToBuffer2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_image_to_buffer2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_blit_image2: unsafe {
                unsafe extern "system" fn cmd_blit_image2(
                    _command_buffer: CommandBuffer,
                    _p_blit_image_info: *const BlitImageInfo2<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_blit_image2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBlitImage2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_blit_image2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_resolve_image2: unsafe {
                unsafe extern "system" fn cmd_resolve_image2(
                    _command_buffer: CommandBuffer,
                    _p_resolve_image_info: *const ResolveImageInfo2<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_resolve_image2)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdResolveImage2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_resolve_image2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_begin_rendering: unsafe {
                unsafe extern "system" fn cmd_begin_rendering(
                    _command_buffer: CommandBuffer,
                    _p_rendering_info: *const RenderingInfo<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_begin_rendering)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRendering\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_rendering
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_end_rendering: unsafe {
                unsafe extern "system" fn cmd_end_rendering(_command_buffer: CommandBuffer) {
                    panic!(concat!("Unable to load ", stringify!(cmd_end_rendering)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRendering\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_rendering
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_cull_mode: unsafe {
                unsafe extern "system" fn cmd_set_cull_mode(
                    _command_buffer: CommandBuffer,
                    _cull_mode: CullModeFlags,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_cull_mode)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCullMode\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_cull_mode
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_front_face: unsafe {
                unsafe extern "system" fn cmd_set_front_face(
                    _command_buffer: CommandBuffer,
                    _front_face: FrontFace,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_front_face)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetFrontFace\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_front_face
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_primitive_topology: unsafe {
                unsafe extern "system" fn cmd_set_primitive_topology(
                    _command_buffer: CommandBuffer,
                    _primitive_topology: PrimitiveTopology,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_primitive_topology)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPrimitiveTopology\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_primitive_topology
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_viewport_with_count: unsafe {
                unsafe extern "system" fn cmd_set_viewport_with_count(
                    _command_buffer: CommandBuffer,
                    _viewport_count: u32,
                    _p_viewports: *const Viewport,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_viewport_with_count)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportWithCount\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport_with_count
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_scissor_with_count: unsafe {
                unsafe extern "system" fn cmd_set_scissor_with_count(
                    _command_buffer: CommandBuffer,
                    _scissor_count: u32,
                    _p_scissors: *const Rect2D,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_scissor_with_count)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetScissorWithCount\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_scissor_with_count
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_bind_vertex_buffers2: unsafe {
                unsafe extern "system" fn cmd_bind_vertex_buffers2(
                    _command_buffer: CommandBuffer,
                    _first_binding: u32,
                    _binding_count: u32,
                    _p_buffers: *const Buffer,
                    _p_offsets: *const DeviceSize,
                    _p_sizes: *const DeviceSize,
                    _p_strides: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_vertex_buffers2)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBindVertexBuffers2\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_vertex_buffers2
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_depth_test_enable: unsafe {
                unsafe extern "system" fn cmd_set_depth_test_enable(
                    _command_buffer: CommandBuffer,
                    _depth_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_test_enable)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthTestEnable\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_test_enable
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_depth_write_enable: unsafe {
                unsafe extern "system" fn cmd_set_depth_write_enable(
                    _command_buffer: CommandBuffer,
                    _depth_write_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_write_enable)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthWriteEnable\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_write_enable
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_depth_compare_op: unsafe {
                unsafe extern "system" fn cmd_set_depth_compare_op(
                    _command_buffer: CommandBuffer,
                    _depth_compare_op: CompareOp,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_compare_op)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthCompareOp\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_compare_op
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_depth_bounds_test_enable: unsafe {
                unsafe extern "system" fn cmd_set_depth_bounds_test_enable(
                    _command_buffer: CommandBuffer,
                    _depth_bounds_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_bounds_test_enable)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBoundsTestEnable\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_bounds_test_enable
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_stencil_test_enable: unsafe {
                unsafe extern "system" fn cmd_set_stencil_test_enable(
                    _command_buffer: CommandBuffer,
                    _stencil_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_stencil_test_enable)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilTestEnable\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_stencil_test_enable
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_stencil_op: unsafe {
                unsafe extern "system" fn cmd_set_stencil_op(
                    _command_buffer: CommandBuffer,
                    _face_mask: StencilFaceFlags,
                    _fail_op: StencilOp,
                    _pass_op: StencilOp,
                    _depth_fail_op: StencilOp,
                    _compare_op: CompareOp,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_stencil_op)))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilOp\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_stencil_op
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_rasterizer_discard_enable: unsafe {
                unsafe extern "system" fn cmd_set_rasterizer_discard_enable(
                    _command_buffer: CommandBuffer,
                    _rasterizer_discard_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_rasterizer_discard_enable)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkCmdSetRasterizerDiscardEnable\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_rasterizer_discard_enable
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_depth_bias_enable: unsafe {
                unsafe extern "system" fn cmd_set_depth_bias_enable(
                    _command_buffer: CommandBuffer,
                    _depth_bias_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_bias_enable)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBiasEnable\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_bias_enable
                } else {
                    ::core::mem::transmute(val)
                }
            },
            cmd_set_primitive_restart_enable: unsafe {
                unsafe extern "system" fn cmd_set_primitive_restart_enable(
                    _command_buffer: CommandBuffer,
                    _primitive_restart_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_primitive_restart_enable)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPrimitiveRestartEnable\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_primitive_restart_enable
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_device_buffer_memory_requirements: unsafe {
                unsafe extern "system" fn get_device_buffer_memory_requirements(
                    _device: crate::vk::Device,
                    _p_info: *const DeviceBufferMemoryRequirements<'_>,
                    _p_memory_requirements: *mut MemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_buffer_memory_requirements)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceBufferMemoryRequirements\0");
                let val = _f(cname);
                if val.is_null() {
                    get_device_buffer_memory_requirements
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_device_image_memory_requirements: unsafe {
                unsafe extern "system" fn get_device_image_memory_requirements(
                    _device: crate::vk::Device,
                    _p_info: *const DeviceImageMemoryRequirements<'_>,
                    _p_memory_requirements: *mut MemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_image_memory_requirements)
                    ))
                }
                let cname =
                    CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceImageMemoryRequirements\0");
                let val = _f(cname);
                if val.is_null() {
                    get_device_image_memory_requirements
                } else {
                    ::core::mem::transmute(val)
                }
            },
            get_device_image_sparse_memory_requirements: unsafe {
                unsafe extern "system" fn get_device_image_sparse_memory_requirements(
                    _device: crate::vk::Device,
                    _p_info: *const DeviceImageMemoryRequirements<'_>,
                    _p_sparse_memory_requirement_count: *mut u32,
                    _p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_image_sparse_memory_requirements)
                    ))
                }
                let cname = CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceImageSparseMemoryRequirements\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_image_sparse_memory_requirements
                } else {
                    ::core::mem::transmute(val)
                }
            },
        }
    }
}
